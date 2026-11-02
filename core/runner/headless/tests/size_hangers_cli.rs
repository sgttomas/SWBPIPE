//! DEL-04-11 slice 1d-α (`Work/hanger-sizing.md` §8.3.2): the actual
//! `openpipestress-runner size-hangers` process. Stdin or `--input` in, the
//! `ControlledExport`-wrapped document on stdout, `--output` written as the
//! other verbs write it; exit 0 sized, 1 refused, 2 when no document is formed.
//! The inputs are the committed, invented corpus inputs
//! (`fixtures/results/hanger_sizing_input_*.json`).

use open_pipe_stress_hanger_sizing::reader::{self, Status, Supplied};
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn project() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(project().join("fixtures/results").join(name)).unwrap()
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn run(args: &[&str], stdin: &[u8]) -> Run {
    let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut pipe = child.stdin.take().unwrap();
    let input = stdin.to_vec();
    let writer = std::thread::spawn(move || {
        // The runner may exit before reading everything; a broken pipe is fine.
        let _ = pipe.write_all(&input);
    });
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    Run {
        code: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

fn controlled(run: &Run) -> Value {
    serde_json::from_str(&run.stdout).unwrap_or_else(|e| panic!("{e}: {}", run.stdout))
}

/// The number-aware equality the reader's reproduction uses: −0 equals 0.
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Object(x), Value::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w))),
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(v, w)| same(v, w)),
        _ => a == b,
    }
}

#[test]
fn a_sized_request_publishes_the_document_with_exit_zero() {
    for stem in ["two_span", "four_span"] {
        let input = fixture(&format!("hanger_sizing_input_{stem}.json"));
        let out = run(&["size-hangers", "--explicit-local-private-intent", "--input", "-"], input.as_bytes());
        assert_eq!(out.code, 0, "{stem}: {}", out.stderr);
        let c = controlled(&out);
        assert_eq!(c["blocked"], false, "{stem}");
        let document = &c["payload"];
        let committed: Value = serde_json::from_str(&fixture(&format!("hanger_sizing_result_{stem}.json"))).unwrap();
        assert!(same(document, &committed), "{stem}: the runner's document is not the committed one");
        // The product reader reads the runner's own bytes, every standing passing.
        let bound: Value = serde_json::from_str(&input).unwrap();
        let verdict = reader::read(
            &serde_json::to_string(document).unwrap(),
            Supplied { solve_request: Some(&bound["request"]), library: Some(&bound["library"]) },
        );
        for standing in [&verdict.closure, &verdict.binding, &verdict.arithmetic, &verdict.derivation, &verdict.reproduction] {
            assert_eq!(standing.status, Status::Passed, "{stem}: {standing:?}");
        }
    }
}

#[test]
fn input_from_a_file_and_output_to_a_file_follow_the_other_verbs() {
    let dir = std::env::temp_dir().join(format!("size-hangers-cli-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("input.json");
    let output = dir.join("result.json");
    std::fs::write(&input, fixture("hanger_sizing_input_refused.json")).unwrap();
    let out = run(
        &["size-hangers", "--input", input.to_str().unwrap(), "--output", output.to_str().unwrap(), "--explicit-local-private-intent"],
        b"",
    );
    assert_eq!(out.code, 1, "a refused request exits 1: {}", out.stderr);
    let written = std::fs::read_to_string(&output).unwrap();
    assert_eq!(written, format!("{}\n", out.stdout.trim_end()));
    let c = controlled(&out);
    assert_eq!(c["payload"]["status"], "refused");
    assert_eq!(c["payload"]["refusals"][0]["code"], "HANGER_SIZING_CANDIDATE_UNKNOWN");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn an_input_outside_the_profile_is_a_refused_document_not_an_unusable_input() {
    let input = fixture("hanger_sizing_input_two_span.json").replacen("\"weight_case\"", "\"weight_case\": \"x\", \"weight_case\"", 1);
    let out = run(&["size-hangers", "--explicit-local-private-intent"], input.as_bytes());
    assert_eq!(out.code, 1);
    let doc = &controlled(&out)["payload"];
    assert_eq!(doc["refusals"][0]["code"], "HANGER_SIZING_INPUT_NOT_ADMITTED");
    assert_eq!(doc["refusals"][0]["names"], json!(["duplicate_key", "$.sizing.weight_case"]));
    assert_eq!(doc["inputs"]["sizing_request"], Value::Null);
    let out = run(&["size-hangers", "--explicit-local-private-intent"], b"{\"request\": ");
    assert_eq!(out.code, 1);
    assert_eq!(controlled(&out)["payload"]["refusals"][0]["names"], json!(["invalid_json", "$.request"]));
}

#[test]
fn an_unusable_invocation_or_input_exits_two_without_a_document() {
    let input = fixture("hanger_sizing_input_two_span.json");
    // --solver-mode is the solve verb's only; the request carries the mode.
    let out = run(&["size-hangers", "--solver-mode", "dense_scrutiny"], input.as_bytes());
    assert_eq!((out.code, out.stdout.as_str()), (2, ""));
    assert!(out.stderr.contains("--solver-mode is accepted once for solve only"), "{}", out.stderr);
    // The verb's own 64 MiB limit, before any parse: valid JSON padded past it.
    let mut large = input.into_bytes();
    large.resize(64 * 1024 * 1024 + 1, b' ');
    let out = run(&["size-hangers", "--explicit-local-private-intent"], &large);
    assert_eq!((out.code, out.stdout.as_str()), (2, ""));
    assert!(out.stderr.contains("HANGER_SIZING_INPUT_TOO_LARGE"), "{}", out.stderr);
    // At the limit the input is read and sized.
    let mut limit = fixture("hanger_sizing_input_refused.json").into_bytes();
    limit.resize(64 * 1024 * 1024, b' ');
    assert_eq!(run(&["size-hangers", "--explicit-local-private-intent"], &limit).code, 1);
    // An input that is not UTF-8 is unusable under the runner's reading.
    let out = run(&["size-hangers"], &[0xff, 0xfe]);
    assert_eq!((out.code, out.stdout.as_str()), (2, ""));
}

/// The local-private control is reused unchanged (§8.3.2). Its rule classifies
/// by path token, and no member path of the hanger-sizing document carries one of
/// its private tokens, so without `--explicit-local-private-intent` the document
/// is not blocked: only its null members draw an unknown-provenance warning. This
/// records the control's actual behaviour; it was reported to Agent 0 with the
/// slice, because the design states the result is local-private output.
#[test]
fn the_unchanged_local_private_control_does_not_block_the_document_without_intent() {
    let input = fixture("hanger_sizing_input_two_span.json");
    let out = run(&["size-hangers"], input.as_bytes());
    let c = controlled(&out);
    assert_eq!(out.code, 0);
    assert_eq!(c["blocked"], false);
    assert_eq!(c["summary"]["blocking_count"], 0);
    assert!(c["findings"].as_array().unwrap().iter().all(|f| f["code"] == "UNKNOWN_PROVENANCE_WARNING"));
    let with_intent = controlled(&run(&["size-hangers", "--explicit-local-private-intent"], input.as_bytes()));
    assert_eq!(with_intent["payload"], c["payload"]);
}
