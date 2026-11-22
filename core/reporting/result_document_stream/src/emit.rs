//! One emission of the document: the canonical bytes, in order, to a stage, as a content
//! source a visitor walks.
//!
//! The root, `result_envelope` and the one set object under `result_sets` are source
//! objects; `result_sets` and the six row lists are source lists; every row entry and every
//! other member is handed over as a whole value. Walking a member hands it to the visitor and
//! stages exactly that member's canonical bytes, with its punctuation, in the same call.
//!
//! The canonical text is staged piece by piece as it is written, a long string included:
//! no member's or row entry's text is held whole, and every stage write and every verifier
//! read is at most [`CHUNK`] bytes. A row entry is held as one value only while it is
//! handed over; the six row lists never form one value.
//!
//! A member handed over as a value is a `serde_json::Value` whose objects iterate their
//! members in sorted order, which a rule walking it relies on: that holds because serde_json
//! is built without its `preserve_order` feature (no manifest in the repository enables it;
//! `tests/map_order.rs` fails if a dependency ever does).
//!
//! Whatever the visitor does, a walk returns `Ok` only when every member was handed over, no
//! visitor call returned `Err`, and every source object or list handed over had been walked
//! to its end exactly once when the visitor's call for it returned; otherwise the emission
//! has failed, and [`Emission::finish`] says why. Nothing is staged after a failure.

use crate::canonical;
use open_pipe_stress_export_decision::{Member, Peek, SourceList, SourceObject, WalkStopped};
use crate::{DocumentSource, RowList, Slot};
use sha2::{Digest, Sha256};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::io::{ErrorKind, Read, Write};

/// The bytes are passed to the stage in chunks of at most this size: every write to a
/// caller's stage, and every read the verifier asks of a written file, is at most `CHUNK`
/// bytes (65,536).
const CHUNK: usize = 1 << 16;

/// Where an emission's bytes go.
pub(crate) enum Stage<'s> {
    /// Nowhere: only digested (the first emission).
    Digest,
    /// To a caller's writer.
    Write(&'s mut dyn Write),
    /// Compared with what a reader gives (the verifier).
    Compare(&'s mut dyn Read),
}

/// What a successful emission wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emitted {
    /// The number of bytes of the document.
    pub bytes: u64,
    /// The SHA-256 of those bytes, as lowercase hexadecimal.
    pub sha256: String,
}

struct Out<'s> {
    stage: Stage<'s>,
    buffer: Vec<u8>,
    theirs: Vec<u8>,
    /// Every byte.
    full: Sha256,
    /// Every byte but the hash member's (the first emission's bytes).
    payload: Sha256,
    bytes: u64,
    staged: u64,
    failure: Option<String>,
}

impl Out<'_> {
    fn put(&mut self, piece: &[u8], in_payload: bool) {
        if self.failure.is_some() {
            return;
        }
        self.full.update(piece);
        if in_payload {
            self.payload.update(piece);
        }
        self.bytes += piece.len() as u64;
        if matches!(self.stage, Stage::Digest) {
            return;
        }
        // Filled to exactly one chunk at a time, so the buffer never holds more than CHUNK.
        let mut rest = piece;
        while !rest.is_empty() && self.failure.is_none() {
            let (now, later) = rest.split_at(rest.len().min(CHUNK - self.buffer.len()));
            self.buffer.extend_from_slice(now);
            rest = later;
            if self.buffer.len() == CHUNK {
                self.flush();
            }
        }
    }

    fn flush(&mut self) {
        if self.failure.is_some() || self.buffer.is_empty() {
            self.buffer.clear();
            return;
        }
        let failure = match &mut self.stage {
            Stage::Digest => None,
            Stage::Write(stage) => stage.write_all(&self.buffer).err().map(|e| format!("DOCUMENT_STREAM_STAGE_WRITE_FAILED: {e}")),
            Stage::Compare(written) => {
                self.theirs.resize(self.buffer.len(), 0);
                match written.read_exact(&mut self.theirs) {
                    Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                        Some(format!("DOCUMENT_STREAM_VERIFY_SHORTER: the written document ends before byte {}", self.staged + self.buffer.len() as u64))
                    }
                    Err(e) => Some(format!("DOCUMENT_STREAM_VERIFY_READ_FAILED: {e}")),
                    Ok(()) => self.theirs.iter().zip(&self.buffer).position(|(a, b)| a != b).map(|at| {
                        format!("DOCUMENT_STREAM_VERIFY_MISMATCH: the written document differs at byte {}", self.staged + at as u64)
                    }),
                }
            }
        };
        self.staged += self.buffer.len() as u64;
        self.buffer.clear();
        if let Some(failure) = failure {
            self.fail(failure);
        }
    }

    fn fail(&mut self, reason: String) {
        if self.failure.is_none() {
            self.failure = Some(reason);
        }
        self.buffer.clear();
    }
}

/// The nodes a walk passes through: the root, the envelope, `result_sets`, its one set
/// object, and the six row lists.
#[derive(Clone, Copy)]
enum Node {
    Root,
    Envelope,
    Sets,
    Set,
    List(RowList),
}

impl Node {
    fn index(self) -> usize {
        match self {
            Node::Root => 0,
            Node::Envelope => 1,
            Node::Sets => 2,
            Node::Set => 3,
            Node::List(list) => 4 + list as usize,
        }
    }
}

/// One emission of a [`DocumentSource`]: the root [`SourceObject`] of the document. Walk it
/// once, then [`finish`](Emission::finish).
pub struct Emission<'s, 'a> {
    document: &'s DocumentSource<'a>,
    out: RefCell<Out<'s>>,
    walks: [Cell<u32>; 10],
    ended: [Cell<bool>; 10],
}

impl<'s, 'a> Emission<'s, 'a> {
    pub(crate) fn new(document: &'s DocumentSource<'a>, stage: Stage<'s>) -> Self {
        Emission {
            document,
            out: RefCell::new(Out {
                stage,
                buffer: Vec::with_capacity(CHUNK),
                theirs: Vec::new(),
                full: Sha256::new(),
                payload: Sha256::new(),
                bytes: 0,
                staged: 0,
                failure: None,
            }),
            walks: Default::default(),
            ended: Default::default(),
        }
    }

    /// Whether the emission succeeded: the root was walked once to its end, nothing failed,
    /// every byte reached the stage, and the bytes without the hash member digest to the
    /// derivative hash. On `Err` the staged bytes are not the document and are discarded by
    /// the caller. This is the producer's outcome only: where a rule walked the emission,
    /// whether the bytes may be exposed is the rule's outcome, whatever the walk returned.
    pub fn finish(self) -> Result<Emitted, String> {
        let document = self.document;
        let (emitted, payload) = self.finish_payload(Ok(()))?;
        if payload != document.derivative {
            return Err("DOCUMENT_STREAM_HASH_MISMATCH: the bytes without the hash member do not digest to the derivative hash".into());
        }
        Ok(emitted)
    }

    pub(crate) fn finish_walk(self, walked: Result<(), WalkStopped>) -> Result<Emitted, String> {
        if walked.is_err() {
            let reason = self.out.borrow().failure.clone();
            return Err(reason.unwrap_or_else(|| "DOCUMENT_STREAM_WALK_STOPPED".into()));
        }
        self.finish()
    }

    /// The emission's outcome and the digest of its bytes without the hash member.
    pub(crate) fn finish_payload(self, walked: Result<(), WalkStopped>) -> Result<(Emitted, String), String> {
        let root = Node::Root.index();
        let complete = self.walks[root].get() == 1 && self.ended[root].get();
        let mut out = self.out.into_inner();
        out.flush();
        if let Some(failure) = out.failure {
            return Err(failure);
        }
        if walked.is_err() {
            return Err("DOCUMENT_STREAM_WALK_STOPPED".into());
        }
        if !complete {
            return Err("DOCUMENT_STREAM_NOT_WALKED: the root was not walked once to its end".into());
        }
        match &mut out.stage {
            Stage::Digest => {}
            Stage::Write(stage) => stage.flush().map_err(|e| format!("DOCUMENT_STREAM_STAGE_WRITE_FAILED: {e}"))?,
            Stage::Compare(written) => {
                let mut more = [0u8; 1];
                loop {
                    match written.read(&mut more) {
                        Ok(0) => break,
                        Ok(_) => return Err(format!("DOCUMENT_STREAM_VERIFY_LONGER: the written document continues after byte {}", out.bytes)),
                        Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                        Err(e) => return Err(format!("DOCUMENT_STREAM_VERIFY_READ_FAILED: {e}")),
                    }
                }
            }
        }
        let emitted = Emitted { bytes: out.bytes, sha256: canonical::hex(&out.full.finalize().into()) };
        Ok((emitted, canonical::hex(&out.payload.finalize().into())))
    }

    fn failed(&self) -> bool {
        self.out.borrow().failure.is_some()
    }

    fn fail(&self, reason: &str) -> WalkStopped {
        self.out.borrow_mut().fail(reason.to_owned());
        WalkStopped
    }

    fn put(&self, piece: &[u8]) {
        self.out.borrow_mut().put(piece, true);
    }

    fn write_value(&self, value: &serde_json::Value, in_payload: bool) {
        let mut out = self.out.borrow_mut();
        canonical::write(value, &mut |piece| out.put(piece, in_payload));
    }

    fn write_name(&self, name: &str, in_payload: bool) {
        let mut out = self.out.borrow_mut();
        canonical::write_str(name, &mut |piece| out.put(piece, in_payload));
        out.put(b":", in_payload);
    }

    /// `reproducibility`, member by member: its `derivative_hash` member, and the one comma
    /// that separates it from a neighbour, are left out of the payload digest, so that the
    /// payload is exactly what the first emission wrote.
    fn write_reproducibility(&self, value: &serde_json::Value) {
        let Some(map) = value.as_object() else {
            return self.write_value(value, true);
        };
        let keys = canonical::sorted_keys(map);
        let hash_at = keys.iter().position(|key| *key == "derivative_hash");
        self.put(b"{");
        for (at, key) in keys.into_iter().enumerate() {
            let hidden = hash_at == Some(at);
            if at > 0 {
                let separator_hidden = hidden || (hash_at == Some(0) && at == 1);
                self.out.borrow_mut().put(b",", !separator_hidden);
            }
            self.write_name(key, !hidden);
            self.write_value(&map[key.as_str()], !hidden);
        }
        self.put(b"}");
    }

    fn begin(&self, node: Node) -> Result<(), WalkStopped> {
        if self.failed() {
            return Err(WalkStopped);
        }
        let walks = &self.walks[node.index()];
        walks.set(walks.get() + 1);
        if walks.get() > 1 {
            return Err(self.fail("DOCUMENT_STREAM_WALKED_TWICE: a source object or list was walked more than once"));
        }
        Ok(())
    }

    fn end(&self, node: Node) -> Result<(), WalkStopped> {
        if self.failed() {
            return Err(WalkStopped);
        }
        self.ended[node.index()].set(true);
        Ok(())
    }

    /// After the visitor's call for a member returned: the walk goes on only if the call
    /// returned `Ok`, nothing failed, and a source object or list handed over (`child`) was
    /// walked to its end exactly once.
    fn handed(&self, visited: Result<(), WalkStopped>, child: Option<Node>) -> Result<(), WalkStopped> {
        if visited.is_err() {
            return Err(self.fail("DOCUMENT_STREAM_VISITOR_STOPPED: a visitor call returned Err"));
        }
        if self.failed() {
            return Err(WalkStopped);
        }
        if let Some(child) = child {
            let index = child.index();
            if self.walks[index].get() != 1 || !self.ended[index].get() {
                return Err(self.fail("DOCUMENT_STREAM_MEMBER_NOT_WALKED: a source object or list handed over was not walked to its end once"));
            }
        }
        Ok(())
    }

    fn walk_object(
        &self,
        node: Node,
        members: &[(String, Slot<'a>)],
        visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>,
    ) -> Result<(), WalkStopped> {
        self.begin(node)?;
        self.put(b"{");
        for (at, (name, slot)) in members.iter().enumerate() {
            if at > 0 {
                self.put(b",");
            }
            self.write_name(name, true);
            let (visited, child) = match slot {
                Slot::Owned(value) => {
                    self.write_value(value, true);
                    (visit(name, Member::Value(value)), None)
                }
                Slot::Borrowed(value) => {
                    self.write_value(value, true);
                    (visit(name, Member::Value(value)), None)
                }
                Slot::Reproducibility(value) => {
                    self.write_reproducibility(value);
                    (visit(name, Member::Value(value)), None)
                }
                Slot::Envelope => (visit(name, Member::Object(&EnvelopeSource(self))), Some(Node::Envelope)),
                Slot::ResultSets => (visit(name, Member::List(&SetsSource(self))), Some(Node::Sets)),
                Slot::List(list) => (visit(name, Member::List(&RowListSource(self, *list))), Some(Node::List(*list))),
            };
            self.handed(visited, child)?;
        }
        self.put(b"}");
        self.end(node)
    }

    fn walk_rows(&self, list: RowList, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        let node = Node::List(list);
        self.begin(node)?;
        self.put(b"[");
        let mut checked = Sha256::new();
        let mut first = true;
        for (index, fact) in self.document.facts.iter().enumerate() {
            if !list.holds(fact.disposition) {
                continue;
            }
            let entry = match self.document.entry(list, index, fact) {
                Ok(entry) => entry,
                Err(e) => return Err(self.fail(&format!("DOCUMENT_STREAM_ROW_FAILED: source row {index}: {e}"))),
            };
            if !first {
                self.put(b",");
            }
            first = false;
            // The entry's canonical text, streamed: digested and staged piece by piece.
            {
                let mut out = self.out.borrow_mut();
                canonical::write(&entry, &mut |piece| {
                    checked.update(piece);
                    out.put(piece, true);
                });
            }
            self.handed(visit(Member::Value(&entry)), None)?;
        }
        self.put(b"]");
        let digest: [u8; 32] = checked.finalize().into();
        if digest != self.document.list_digests[list as usize] {
            return Err(self.fail(&format!("DOCUMENT_STREAM_ROW_LIST_MISMATCH: {} is not the list checked in the row sweep", list.name())));
        }
        self.end(node)
    }
}

fn names(members: &[(String, Slot)]) -> Vec<String> {
    members.iter().map(|(name, _)| name.clone()).collect()
}

fn peek<'m>(members: &'m [(String, Slot)], name: &str) -> Option<Peek<'m>> {
    let (_, slot) = members.iter().find(|(member, _)| member == name)?;
    Some(match slot {
        Slot::Owned(value) | Slot::Reproducibility(value) => Peek::Value(Cow::Borrowed(value)),
        Slot::Borrowed(value) => Peek::Value(Cow::Borrowed(*value)),
        Slot::Envelope => Peek::Object,
        Slot::ResultSets | Slot::List(_) => Peek::List,
    })
}

impl SourceObject for Emission<'_, '_> {
    fn member_names(&self) -> Vec<String> {
        names(&self.document.root)
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        peek(&self.document.root, name)
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.walk_object(Node::Root, &self.document.root, visit)
    }
}

struct EnvelopeSource<'e, 's, 'a>(&'e Emission<'s, 'a>);

impl SourceObject for EnvelopeSource<'_, '_, '_> {
    fn member_names(&self) -> Vec<String> {
        names(&self.0.document.envelope)
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        peek(&self.0.document.envelope, name)
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.0.walk_object(Node::Envelope, &self.0.document.envelope, visit)
    }
}

struct SetSource<'e, 's, 'a>(&'e Emission<'s, 'a>);

impl SourceObject for SetSource<'_, '_, '_> {
    fn member_names(&self) -> Vec<String> {
        names(&self.0.document.set)
    }

    fn peek(&self, name: &str) -> Option<Peek<'_>> {
        peek(&self.0.document.set, name)
    }

    fn walk(&self, visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.0.walk_object(Node::Set, &self.0.document.set, visit)
    }
}

struct SetsSource<'e, 's, 'a>(&'e Emission<'s, 'a>);

impl SourceList for SetsSource<'_, '_, '_> {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        let emission = self.0;
        emission.begin(Node::Sets)?;
        emission.put(b"[");
        emission.handed(visit(Member::Object(&SetSource(emission))), Some(Node::Set))?;
        emission.put(b"]");
        emission.end(Node::Sets)
    }
}

struct RowListSource<'e, 's, 'a>(&'e Emission<'s, 'a>, RowList);

impl SourceList for RowListSource<'_, '_, '_> {
    fn walk(&self, visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
        self.0.walk_rows(self.1, visit)
    }
}

/// Walks a source object whole: every source object and list it hands over, to its end,
/// once, passing on every `Err`. The visitor that writes or digests an emission without
/// looking at it.
pub fn walk_whole(object: &dyn SourceObject) -> Result<(), WalkStopped> {
    object.walk(&mut |_, member| walk_member(member))
}

fn walk_member(member: Member<'_>) -> Result<(), WalkStopped> {
    match member {
        Member::Value(_) => Ok(()),
        Member::Object(object) => walk_whole(object),
        Member::List(list) => list.walk(&mut |entry| walk_member(entry)),
    }
}
