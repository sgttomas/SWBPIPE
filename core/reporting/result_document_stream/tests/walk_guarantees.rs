//! What the producer guarantees whatever its visitor does, and what the recording-visitor
//! comparison of `committed_parity.rs` can detect. Invented data only.

mod common;

use common::{compare, parse, record_object, recorded_is_staged, Input, Outcome};
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_result_document_stream::{walk_whole, DocumentSource, Member, SourceObject, WalkStopped};
use serde_json::json;
use std::io::{Read, Write};

const SOURCE: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");
const PRESSURE: &str = include_str!("../../../../fixtures/results/pressure_connected_mechanics_sparse.json");

fn input() -> Input {
    Input::attested("connected sparse", parse(SOURCE), None)
}

fn stream(input: &Input) -> DocumentSource<'_> {
    DocumentSource::new(input.base.clone(), &input.model, &input.source, input.origin.clone(), None).unwrap()
}

/// A stage that writes one byte other than it is given: the first lowercase letter after
/// `from` bytes becomes the next letter.
struct OneByteOff {
    inner: Vec<u8>,
    from: usize,
    done: bool,
}

impl Write for OneByteOff {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let start = self.inner.len();
        self.inner.extend_from_slice(bytes);
        if !self.done {
            for at in start.max(self.from)..self.inner.len() {
                if self.inner[at].is_ascii_lowercase() && self.inner[at] != b'z' {
                    self.inner[at] += 1;
                    self.done = true;
                    break;
                }
            }
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn the_recording_comparison_fails_on_a_stage_one_byte_off() {
    let input = input();
    let stream = stream(&input);
    // The honest stage: what the visitor rebuilt is what was staged.
    let mut staged = Vec::new();
    let emission = stream.emission(&mut staged);
    let recorded = record_object(&emission).unwrap();
    emission.finish().unwrap();
    assert!(recorded_is_staged(&recorded, &staged));
    // A stage one byte off, past the header, in the row lists: the comparison sees it.
    let mut off = OneByteOff { inner: Vec::new(), from: staged.len() / 2, done: false };
    let emission = stream.emission(&mut off);
    let recorded = record_object(&emission).unwrap();
    emission.finish().unwrap();
    assert!(off.done);
    assert_ne!(off.inner, staged);
    assert!(!recorded_is_staged(&recorded, &off.inner), "the comparison missed a staged byte other than shown");
    // And the verifier refuses the altered file.
    let refusal = stream.verify(&mut off.inner.as_slice()).unwrap_err();
    assert!(refusal.starts_with("DOCUMENT_STREAM_VERIFY_MISMATCH"), "{refusal}");
}

#[test]
fn the_verifier_holds_the_written_file_to_the_last_byte() {
    let input = input();
    let stream = stream(&input);
    let mut written = Vec::new();
    let emitted = stream.write_to(&mut written).unwrap();
    assert_eq!(stream.verify(&mut written.as_slice()).unwrap(), emitted);
    let shorter = &written[..written.len() - 1];
    assert!(stream.verify(&mut &shorter[..]).unwrap_err().starts_with("DOCUMENT_STREAM_VERIFY_SHORTER"));
    let mut longer = written.clone();
    longer.push(b'\n');
    assert!(stream.verify(&mut longer.as_slice()).unwrap_err().starts_with("DOCUMENT_STREAM_VERIFY_LONGER"));
    let mut last = written.clone();
    *last.last_mut().unwrap() = b']';
    assert!(stream.verify(&mut last.as_slice()).unwrap_err().starts_with("DOCUMENT_STREAM_VERIFY_MISMATCH"));
}

/// Walks the root, misbehaving as `act` says when it meets the member named `at` (and
/// walking everything else whole), and returns the walk's result and the emission's.
fn misbehave(at: &str, act: &dyn Fn(Member<'_>) -> Result<(), WalkStopped>) -> (Result<(), WalkStopped>, Result<(), String>) {
    let input = input();
    let stream = stream(&input);
    let mut staged = Vec::new();
    let emission = stream.emission(&mut staged);
    let walked = walk_until(&emission, at, act);
    let finished = emission.finish().map(|_| ());
    (walked, finished)
}

fn walk_until(object: &dyn SourceObject, at: &str, act: &dyn Fn(Member<'_>) -> Result<(), WalkStopped>) -> Result<(), WalkStopped> {
    object.walk(&mut |name, member| {
        if name == at {
            return act(member);
        }
        match member {
            Member::Value(_) => Ok(()),
            Member::Object(child) => walk_until(child, at, act),
            Member::List(list) => list.walk(&mut |entry| match entry {
                Member::Object(child) => walk_until(child, at, act),
                _ => Ok(()),
            }),
        }
    })
}

fn refused(outcome: (Result<(), WalkStopped>, Result<(), String>), code: &str) {
    let (walked, finished) = outcome;
    assert_eq!(walked, Err(WalkStopped));
    let reason = finished.unwrap_err();
    assert!(reason.starts_with(code), "{reason}");
}

#[test]
fn a_visitor_that_skips_a_source_member_gets_an_error() {
    refused(misbehave("result_envelope", &|_| Ok(())), "DOCUMENT_STREAM_MEMBER_NOT_WALKED");
    refused(misbehave("row_accounting", &|_| Ok(())), "DOCUMENT_STREAM_MEMBER_NOT_WALKED");
    refused(misbehave("result_sets", &|_| Ok(())), "DOCUMENT_STREAM_MEMBER_NOT_WALKED");
}

#[test]
fn a_visitor_that_walks_a_member_twice_gets_an_error() {
    let twice = |member: Member<'_>| match member {
        Member::List(list) => {
            list.walk(&mut |_| Ok(()))?;
            list.walk(&mut |_| Ok(()))
        }
        Member::Object(object) => {
            walk_whole(object)?;
            walk_whole(object)
        }
        Member::Value(_) => Ok(()),
    };
    refused(misbehave("source_annotations", &twice), "DOCUMENT_STREAM_WALKED_TWICE");
    refused(misbehave("result_envelope", &twice), "DOCUMENT_STREAM_WALKED_TWICE");
}

#[test]
fn a_visitor_that_stops_gets_an_error() {
    refused(misbehave("diagnostics", &|_| Err(WalkStopped)), "DOCUMENT_STREAM_VISITOR_STOPPED");
    // Stopping at the third entry of a row list.
    let third = |member: Member<'_>| match member {
        Member::List(list) => {
            let mut seen = 0;
            list.walk(&mut |_| {
                seen += 1;
                if seen == 3 {
                    Err(WalkStopped)
                } else {
                    Ok(())
                }
            })
        }
        _ => Ok(()),
    };
    refused(misbehave("unit_preservation_witnesses", &third), "DOCUMENT_STREAM_VISITOR_STOPPED");
}

#[test]
fn a_visitor_that_swallows_a_childs_stop_gets_an_error() {
    let swallow = |member: Member<'_>| match member {
        Member::List(list) => {
            let _ = list.walk(&mut |_| Err(WalkStopped));
            Ok(())
        }
        Member::Object(object) => {
            let _ = object.walk(&mut |_, _| Err(WalkStopped));
            Ok(())
        }
        Member::Value(_) => Ok(()),
    };
    refused(misbehave("row_accounting", &swallow), "DOCUMENT_STREAM_VISITOR_STOPPED");
    refused(misbehave("result_envelope", &swallow), "DOCUMENT_STREAM_VISITOR_STOPPED");
}

#[test]
fn the_root_is_walked_once() {
    let input = input();
    let stream = stream(&input);
    let mut staged = Vec::new();
    let emission = stream.emission(&mut staged);
    assert!(emission.finish().unwrap_err().starts_with("DOCUMENT_STREAM_NOT_WALKED"));
    let mut staged = Vec::new();
    let emission = stream.emission(&mut staged);
    walk_whole(&emission).unwrap();
    assert_eq!(walk_whole(&emission), Err(WalkStopped));
    assert!(emission.finish().unwrap_err().starts_with("DOCUMENT_STREAM_WALKED_TWICE"));
}

/// A stage whose writes fail after `room` bytes.
struct Full {
    room: usize,
}

impl Write for Full {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.room {
            return Err(std::io::Error::other("stage full"));
        }
        self.room -= bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The chunk every stage write and every verifier read stays within.
const CHUNK: usize = 65_536;

/// About `bytes` bytes of text with every kind of JSON escape scattered through it.
fn long_text(bytes: usize, seed: &str) -> String {
    const SCATTERED: [&str; 12] = ["\"", "\\", "\n", "\t", "\r", "\u{8}", "\u{c}", "\u{1}", "\u{1f}", "\u{7f}", "\u{2028}", "\u{1f600}"];
    let mut text = String::with_capacity(bytes + 64);
    let mut at = 0;
    while text.len() < bytes {
        text.push_str(seed);
        text.push_str(" é ");
        text.push_str(SCATTERED[at % SCATTERED.len()]);
        at += 1;
    }
    text
}

/// A stage that keeps what it is given and the largest slice one write gave it.
struct Widest {
    inner: Vec<u8>,
    widest: usize,
}

impl Write for Widest {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.widest = self.widest.max(bytes.len());
        self.inner.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// A written file that notes the largest buffer one read asked it to fill.
struct WidestRead<'w> {
    written: &'w [u8],
    widest: usize,
}

impl Read for WidestRead<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.widest = self.widest.max(buffer.len());
        self.written.read(buffer)
    }
}

/// A member of the header and a row entry each far larger than one chunk: the bytes and the
/// hash are still derive_document's, and no stage write or verifier read exceeds one chunk.
#[test]
fn stage_writes_and_verifier_reads_stay_within_one_chunk() {
    // The pressure reader discloses a row of a kind it does not know (the preview reader
    // refuses one), so the long kind reaches a row entry.
    let mut input = Input::attested("a long diagnostic message and a long row kind", parse(PRESSURE), None);
    let message = long_text(300 * 1024, "a long diagnostic message");
    input.base["result_envelope"]["diagnostics"][0]["message"] = json!(message);
    let kind = long_text(300 * 1024, "document_stream_long_kind");
    input.source["results"][0]["kind"] = json!(kind);
    let input = input.rebound();
    assert!(canonical_json(&input.source["results"][0]).len() > 256 * 1024);

    let document = match compare(&input) {
        Outcome::Derived(document) => document,
        Outcome::Refused(text) => panic!("both refuse: {text}"),
    };
    let disclosure = document["result_envelope"]["row_disclosures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["source_kind"] == json!(kind))
        .expect("the long row is disclosed");
    assert!(canonical_json(disclosure).len() > CHUNK);

    let stream = stream(&input);
    let mut stage = Widest { inner: Vec::new(), widest: 0 };
    let emitted = stream.write_to(&mut stage).unwrap();
    assert_eq!(stage.inner, canonical_json(&document).as_bytes());
    let mut written = WidestRead { written: &stage.inner, widest: 0 };
    let verified = stream.verify(&mut written);
    let (wrote, read) = (stage.widest, written.widest);
    assert!(
        (1..=CHUNK).contains(&wrote) && (1..=CHUNK).contains(&read),
        "the largest stage write was {wrote} bytes and the largest verifier read {read} bytes; each must be within 1..={CHUNK}"
    );
    assert_eq!(verified.unwrap(), emitted);
}

#[test]
fn a_failed_write_ends_the_emission_with_an_error() {
    let input = input();
    let stream = stream(&input);
    let mut full = Full { room: 100_000 };
    let refusal = stream.write_to(&mut full).unwrap_err();
    assert!(refusal.starts_with("DOCUMENT_STREAM_STAGE_WRITE_FAILED"), "{refusal}");
}
