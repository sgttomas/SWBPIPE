//! What the producer guarantees whatever its visitor does, and what the recording-visitor
//! comparison of `committed_parity.rs` can detect. Invented data only.

mod common;

use common::{parse, record_object, recorded_is_staged, Input};
use open_pipe_stress_result_document_stream::{walk_whole, DocumentSource, Member, SourceObject, WalkStopped};
use std::io::Write;

const SOURCE: &str = include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json");

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

#[test]
fn a_failed_write_ends_the_emission_with_an_error() {
    let input = input();
    let stream = stream(&input);
    let mut full = Full { room: 100_000 };
    let refusal = stream.write_to(&mut full).unwrap_err();
    assert!(refusal.starts_with("DOCUMENT_STREAM_STAGE_WRITE_FAILED"), "{refusal}");
}
