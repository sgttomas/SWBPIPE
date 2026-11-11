//! The native size-and-read entry, increment 3c part 1 (DEL-04-11
//! design §8.3, §13): its reply on the committed corpus inputs
//! (`fixtures/results/hanger_sizing_input_*.json`), the reply's agreement with
//! the desktop adapter's evidence shape and hash
//! (`apps/desktop/src/features/results/hangerSizingView.ts`), the refusals that
//! form no document, and the reader option leaving `read_document` unchanged.
//! All values are invented.

use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_hanger_sizing::desktop::{size_and_read, SizeAndReadError, INPUT_NOT_UTF8};
use open_pipe_stress_hanger_sizing::publish::{checked_sha256, size_hangers, MAX_INPUT_BYTES, REFUSAL_CODES};
use open_pipe_stress_hanger_sizing::reader::{self, ReadOptions, Status, Supplied};
use serde_json::{json, Value};
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results").join(name)
}

fn read_bytes(name: &str) -> Vec<u8> {
    std::fs::read(fixture_path(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn read_json(name: &str) -> Value {
    serde_json::from_slice(&read_bytes(name)).unwrap()
}

const INPUTS: [(&str, &str, bool); 3] = [
    ("hanger_sizing_input_two_span.json", "hanger_sizing_result_two_span.json", true),
    ("hanger_sizing_input_four_span.json", "hanger_sizing_result_four_span.json", true),
    ("hanger_sizing_input_refused.json", "hanger_sizing_result_refused.json", false),
];

const STANDINGS: [&str; 5] = ["closure", "binding", "arithmetic_consistent_with_published_positions", "derivation", "reproduction"];

fn keys(value: &Value) -> Vec<&str> {
    let mut keys: Vec<&str> = value.as_object().expect("an object").keys().map(String::as_str).collect();
    keys.sort_unstable();
    keys
}

fn lowercase_sha256(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// `wellFormedEvidence` of `hangerSizingView.ts`, held more strictly: the same
/// tests (a plain object; `document_sha256` matching `^[0-9a-f]{64}$`; each of
/// the five standings a plain object with `status` passed, failed or
/// not_checked and `difference` null or a string; `binding.not_checked` an
/// array of strings), with each object's members exactly those of
/// `HangerSizingReaderEvidence`.
fn assert_desktop_evidence(evidence: &Value) {
    assert_eq!(keys(evidence), ["document_sha256", "verdict"]);
    assert!(lowercase_sha256(evidence["document_sha256"].as_str().expect("a string")), "{evidence}");
    let verdict = &evidence["verdict"];
    assert_eq!(keys(verdict), {
        let mut names = STANDINGS.to_vec();
        names.sort_unstable();
        names
    });
    for name in STANDINGS {
        let standing = &verdict[name];
        let members: &[&str] = if name == "binding" { &["difference", "not_checked", "status"] } else { &["difference", "status"] };
        assert_eq!(keys(standing), members, "{name}");
        assert!(["passed", "failed", "not_checked"].contains(&standing["status"].as_str().expect("a string")), "{name}");
        assert!(standing["difference"].is_null() || standing["difference"].is_string(), "{name}");
    }
    let not_checked = verdict["binding"]["not_checked"].as_array().expect("an array");
    assert!(not_checked.iter().all(Value::is_string));
}

fn statuses(verdict: &Value) -> Vec<&str> {
    STANDINGS.iter().map(|name| verdict[*name]["status"].as_str().unwrap()).collect()
}

#[test]
fn the_reply_carries_the_published_document_its_checked_hash_and_the_verdict() {
    for (input_name, result_name, sized) in INPUTS {
        let raw = read_bytes(input_name);
        let out = size_and_read(&raw).unwrap_or_else(|e| panic!("{input_name}: {e:?}"));
        let published = size_hangers(std::str::from_utf8(&raw).unwrap());
        assert_eq!(out.published.is_sized(), sized, "{input_name}");
        assert_eq!(out.published.document(), published.document(), "{input_name}");

        // The reply is one JSON text of exactly {document, evidence}, its size
        // the bytes a reply bound applies to.
        assert_eq!(out.reply_len(), out.reply().len());
        let reply: Value = serde_json::from_str(out.reply()).unwrap();
        assert_eq!(keys(&reply), ["document", "evidence"], "{input_name}");
        // Every number round-trips: the parsed document is the published one.
        assert_eq!(&reply["document"], published.document(), "{input_name}");
        assert_desktop_evidence(&reply["evidence"]);

        // The evidence's hash is the checked canonical hash of the document,
        // in-process and as parsed back from the reply.
        let sha = reply["evidence"]["document_sha256"].as_str().unwrap();
        assert_eq!(sha, out.document_sha256);
        assert_eq!(Some(sha), checked_sha256(published.document()).as_deref(), "{input_name}");
        assert_eq!(Some(sha), checked_sha256(&reply["document"]).as_deref(), "{input_name}");
        // `canonicalSha256HexCheckedV1` hashes the checked canonical text of the
        // parsed value's JSON text; the same function's text on the reply's own
        // `document` bytes hashes to the same digest.
        let document_text = serde_json::to_string(&reply["document"]).unwrap();
        let canonical = canonical_json_checked_v1_text(&document_text).unwrap();
        assert_eq!(canonical, canonical_json_checked_v1_text(&serde_json::to_string(published.document()).unwrap()).unwrap());

        // The committed result: the hash agrees exactly when the document does.
        // The committed sized documents were formed on one build and platform
        // (§8.3.7), so their equality to this build's is not asserted; the
        // refused document carries no solved number and equals it everywhere.
        let committed = read_json(result_name);
        let same_document = reply["document"] == committed;
        assert_eq!(same_document, checked_sha256(&committed).as_deref() == Some(sha), "{input_name}");
        if !sized {
            assert!(same_document, "{input_name}: the refused document is the committed one");
        }

        // The verdict: the reader on the published document, given the same
        // input's request and library, reproduction not run.
        let input = read_json(input_name);
        let supplied = Supplied { solve_request: Some(&input["request"]), library: Some(&input["library"]) };
        let expected = reader::read_document_with(published.document(), supplied, ReadOptions::WITHOUT_REPRODUCTION);
        assert_eq!(out.verdict, expected, "{input_name}");
        let verdict = &reply["evidence"]["verdict"];
        assert_eq!(verdict, &expected.to_json(), "{input_name}");
        if sized {
            assert_eq!(statuses(verdict), ["passed", "passed", "passed", "passed", "not_checked"], "{input_name}: {verdict}");
        } else {
            // A refused document: closure and binding only.
            assert_eq!(statuses(verdict), ["passed", "passed", "not_checked", "not_checked", "not_checked"], "{input_name}: {verdict}");
        }
        assert_eq!(verdict["binding"]["not_checked"], json!([]), "{input_name}");
        for name in STANDINGS {
            assert!(verdict[name]["difference"].is_null(), "{input_name}: {name}");
        }
    }
}

/// `hangerSizingCsv.test.ts` asserts that the desktop's
/// `canonicalSha256HexCheckedV1` of the committed two-span document is this
/// digest; the crate's `checked_sha256` forms the same one.
#[test]
fn the_crate_hash_of_the_committed_two_span_document_is_the_desktops() {
    let committed = read_json("hanger_sizing_result_two_span.json");
    assert_eq!(
        checked_sha256(&committed).as_deref(),
        Some("a0d3764ae80d968cebd91c953412f287d869fba96dd631e671a29a3b4b653338")
    );
}

#[test]
fn an_input_over_the_cap_or_not_utf8_forms_no_document() {
    let over = vec![b' '; MAX_INPUT_BYTES + 1];
    let error = size_and_read(&over).unwrap_err();
    assert_eq!(error, SizeAndReadError::TooLarge { bytes: MAX_INPUT_BYTES + 1 });
    assert_eq!(error.code(), "HANGER_SIZING_INPUT_TOO_LARGE");
    assert!(!error.message().is_empty());

    // At the cap the input is admitted and published, as the headless verb's.
    let mut at_cap = read_bytes("hanger_sizing_input_refused.json");
    at_cap.resize(MAX_INPUT_BYTES, b' ');
    let out = size_and_read(&at_cap).unwrap();
    assert_eq!(out.published.document(), size_hangers(std::str::from_utf8(&at_cap).unwrap()).document());
    assert_eq!(out.published.document()["refusals"][0]["code"], "HANGER_SIZING_CANDIDATE_UNKNOWN");

    let text = read_bytes("hanger_sizing_input_two_span.json");
    let at = text.windows(8).position(|w| w == b"load:O1\"").unwrap();
    for bad in [&[0xFF][..], &[0xC3][..], &[0xED, 0xA0, 0x80][..]] {
        let mut raw = text.clone();
        raw.splice(at..at, bad.iter().copied());
        let error = size_and_read(&raw).unwrap_err();
        assert_eq!(error, SizeAndReadError::NotUtf8 { valid_up_to: at }, "{bad:?}");
        assert_eq!(error.code(), INPUT_NOT_UTF8);
        assert!(!error.message().contains("load:"));
    }
    // A truncated sequence at the end is not UTF-8 either.
    let mut raw = text.clone();
    raw.push(0xE2);
    assert_eq!(size_and_read(&raw).unwrap_err(), SizeAndReadError::NotUtf8 { valid_up_to: text.len() });
    // Neither code is a refused document's code: no document is formed.
    assert!(!REFUSAL_CODES.contains(&INPUT_NOT_UTF8));
}

#[test]
fn an_input_outside_the_profile_is_the_headless_refusal_read_with_nothing_supplied() {
    let mut raw = read_bytes("hanger_sizing_input_two_span.json");
    raw.extend_from_slice(b" x");
    let out = size_and_read(&raw).unwrap();
    assert_eq!(out.published.document(), size_hangers(std::str::from_utf8(&raw).unwrap()).document());
    assert_eq!(out.published.document()["refusals"][0]["code"], "HANGER_SIZING_INPUT_NOT_ADMITTED");
    let reply: Value = serde_json::from_str(out.reply()).unwrap();
    assert_desktop_evidence(&reply["evidence"]);
    let verdict = &reply["evidence"]["verdict"];
    // No member of the input was admitted, so none is supplied and the
    // document binds no hash: binding is not checked.
    assert_eq!(statuses(verdict), ["passed", "not_checked", "not_checked", "not_checked", "not_checked"], "{verdict}");
    assert_eq!(verdict["binding"]["not_checked"], json!(["library", "solve_request"]));

    // An admitted input of the wrong shape supplies the members it has.
    let mut input = read_json("hanger_sizing_input_two_span.json");
    input["extra"] = json!(1);
    let out = size_and_read(input.to_string().as_bytes()).unwrap();
    assert_eq!(out.published.document()["refusals"][0]["code"], "HANGER_SIZING_INPUT_SHAPE_INVALID");
    assert_eq!(out.verdict.binding.status, Status::Passed);
    assert!(out.verdict.binding_not_checked.is_empty());
}

/// The corpus mutations (`replace` and `add`), as `tests/publication.rs` applies them.
fn mutate(doc: &mut Value, ops: &[Value]) {
    for op in ops {
        let path = op["path"].as_str().unwrap();
        match op["op"].as_str().unwrap() {
            "replace" => *doc.pointer_mut(path).unwrap() = op["value"].clone(),
            "add" => {
                let (parent, key) = path.rsplit_once('/').unwrap();
                doc.pointer_mut(parent).unwrap()[key] = op["value"].clone();
            }
            other => panic!("unknown op {other}"),
        }
    }
}

/// `read_document` runs every standing, as `ReadOptions::FULL`; without
/// reproduction every other standing is the same and reproduction is
/// `not_checked`. Checked over the corpus entries and a fresh document whose
/// reproduction passes.
#[test]
fn the_reader_option_leaves_read_document_unchanged() {
    let corpus = read_json("hanger_sizing_corpus.json");
    let mut cases = Vec::new();
    for entry in corpus["entries"].as_array().unwrap() {
        let mut doc = read_json(entry["document"].as_str().unwrap());
        mutate(&mut doc, entry["mutations"].as_array().unwrap());
        cases.push((entry["id"].as_str().unwrap().to_string(), doc, entry["input"].as_str().unwrap().to_string(), entry["supply"].clone()));
    }
    let fresh = size_hangers(std::str::from_utf8(&read_bytes("hanger_sizing_input_two_span.json")).unwrap()).into_document();
    cases.push(("fresh-two-span".into(), fresh, "hanger_sizing_input_two_span.json".into(), json!(["solve_request", "library"])));
    let mut reproduction_ran = 0;
    for (id, doc, input_name, supply) in &cases {
        let input = read_json(input_name);
        let supply = supply.as_array().unwrap();
        let supplied = Supplied {
            solve_request: supply.contains(&json!("solve_request")).then_some(&input["request"]),
            library: supply.contains(&json!("library")).then_some(&input["library"]),
        };
        let full = reader::read_document(doc, supplied);
        assert_eq!(full, reader::read_document_with(doc, supplied, ReadOptions::FULL), "{id}");
        assert_eq!(full, reader::read(&serde_json::to_string(doc).unwrap(), supplied), "{id}");
        let without = reader::read_document_with(doc, supplied, ReadOptions::WITHOUT_REPRODUCTION);
        assert_eq!(without.reproduction.status, Status::NotChecked, "{id}");
        assert_eq!(without.reproduction.difference, None, "{id}");
        assert_eq!(
            (&without.closure, &without.binding, &without.binding_not_checked, &without.arithmetic, &without.derivation),
            (&full.closure, &full.binding, &full.binding_not_checked, &full.arithmetic, &full.derivation),
            "{id}"
        );
        if full.reproduction.status != Status::NotChecked {
            reproduction_ran += 1;
        }
    }
    // The option is exercised where reproduction runs: passed on the fresh
    // document, failed on the forgery and the mutations.
    assert!(reproduction_ran >= 8, "{reproduction_ran}");
}
