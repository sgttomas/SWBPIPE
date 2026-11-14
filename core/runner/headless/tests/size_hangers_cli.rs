//! DEL-04-11 slice 1d-α (design §8.3.2): the actual
//! `openpipestress-runner size-hangers` process. Stdin or `--input` in, the
//! `ControlledExport`-wrapped document on stdout, `--output` written as the
//! other verbs write it; exit 0 sized, 1 refused, 2 when no document is formed.
//! The inputs are the committed, invented corpus inputs
//! (`fixtures/results/hanger_sizing_input_*.json`).

use open_pipe_stress_hanger_sizing::publish::size_hangers;
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

/// With the intent flag, the document is the payload, unchanged: the runner's
/// document is the one this build forms in-process from the same input (both
/// are this build on this platform, so no committed bytes are compared), and
/// the product reader passes every standing on the runner's own bytes. The two
/// `ex_` inputs are on the exact pressure route.
#[test]
fn a_sized_request_with_intent_publishes_the_document_with_exit_zero() {
    for (stem, route) in [("two_span", "pressure_free"), ("four_span", "pressure_free"), ("ex_pressurised", "exact_pressure"),
                          ("ex_riser", "exact_pressure")] {
        let input = fixture(&format!("hanger_sizing_input_{stem}.json"));
        let out = run(&["size-hangers", "--explicit-local-private-intent", "--input", "-"], input.as_bytes());
        assert_eq!(out.code, 0, "{stem}: {}", out.stderr);
        let c = controlled(&out);
        assert_eq!(c["blocked"], false, "{stem}");
        let document = &c["payload"];
        assert_eq!(document, &size_hangers(&input).into_document(), "{stem}: the payload is not the document");
        assert_eq!(document["route"], route, "{stem}");
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

fn assert_gate(c: &Value, blocked: bool, ctx: &str) {
    let (action, reason, severity) = if blocked {
        ("block_export", "LOCAL_PRIVATE_INTENT_REQUIRED", "BLOCKING")
    } else {
        ("warning_only", "PRIVATE_LOCAL_ALLOWED_WITH_WARNING", "WARNING")
    };
    assert_eq!(c["blocked"], blocked, "{ctx}");
    assert_eq!(c["decisions"], json!([{
        "decision_id": "REDC-DEC-0001", "path": "$", "field_class": "hanger_sizing_result",
        "privacy_classification": "private_project_data", "redistribution_status": "private_only",
        "review_status": "accepted", "export_context": "local_private", "action": action,
        "reason_code": reason, "source_metadata_present": true
    }]), "{ctx}");
    assert_eq!(c["findings"], json!([{
        "finding_id": "REDC-FND-0001", "code": reason, "class": "PRIVATE_DATA_WARNING", "severity": severity,
        "path": "$", "action": action,
        "message": if blocked { "Export is blocked until wrapper-owned local-private intent is explicit." }
                   else { "Export retained the value in a local/private context with an explicit warning." },
        "remediation": "Use the runner's explicit local-private intent flag for known private values."
    }]), "{ctx}");
    assert_eq!(c["summary"], json!({
        "decision_count": 1, "finding_count": 1, "redacted_count": 0, "omitted_count": 0,
        "warning_count": usize::from(!blocked), "blocking_count": usize::from(blocked),
        "cloud_transmission_attempted": false, "professional_claims_made": false
    }), "{ctx}");
}

/// The whole document is local-private output (§8.3.2), sized or refused (a
/// refused document can name the library's id): without
/// `--explicit-local-private-intent` it is blocked by one decision at `$`, with
/// no payload, exit 1 and no `--output` file.
#[test]
fn without_intent_the_whole_document_is_blocked_and_nothing_is_written() {
    let dir = std::env::temp_dir().join(format!("size-hangers-gate-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    for stem in ["two_span", "four_span", "refused"] {
        let output = dir.join(format!("{stem}.json"));
        let input = fixture(&format!("hanger_sizing_input_{stem}.json"));
        let out = run(&["size-hangers", "--output", output.to_str().unwrap()], input.as_bytes());
        assert_eq!(out.code, 1, "{stem}: {}", out.stderr);
        let c = controlled(&out);
        assert_eq!(c["payload"], Value::Null, "{stem}");
        assert_gate(&c, true, stem);
        assert!(!output.exists(), "{stem}: a blocked document was written");
    }
    // An input that is not admitted forms a refused document, gated alike.
    let out = run(&["size-hangers"], b"{\"request\": ");
    assert_eq!(out.code, 1);
    assert_gate(&controlled(&out), true, "not admitted");
    std::fs::remove_dir_all(&dir).unwrap();
}

/// With the intent flag the same decision warns, the document is the payload
/// unchanged, and the exit status is the document's: 0 sized, 1 refused.
#[test]
fn with_intent_the_whole_document_is_kept_with_one_warning() {
    for (stem, code) in [("two_span", 0), ("four_span", 0), ("refused", 1)] {
        let input = fixture(&format!("hanger_sizing_input_{stem}.json"));
        let out = run(&["size-hangers", "--explicit-local-private-intent"], input.as_bytes());
        assert_eq!(out.code, code, "{stem}: {}", out.stderr);
        let c = controlled(&out);
        assert_gate(&c, false, stem);
        assert_eq!(c["payload"], size_hangers(&input).into_document(), "{stem}");
    }
}
