//! DEL-10-05 REQ-017 / AC-013: the run evidence of the actual
//! `openpipestress-runner` process, for each verb and for `size-hangers`,
//! admitted and refused; the blocked case and exit 2 carry none; each
//! substitution (another request, a model changed in one member, the result
//! edited, the evidence edited or re-paired) fails by the named digest; the same
//! request run twice gives the same result digest; and a request outside the
//! canonical profile is stated as unavailable. The verifier is the crate's
//! `run_evidence::verify`; `tests/test_run_evidence.py` checks the same runs with
//! a verifier that imports no product code. The inputs are the committed,
//! invented runner and sizing inputs, read at compile time.

use open_pipe_stress_headless_runner::run_evidence::{self, verify};
use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

const SOLVE_INPUT: &str =
    include_str!("../../../../validation/witness/inputs/tp_runner_015_final_cli_solve_input.json");
const SOLVED_MODEL: &str =
    include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const VALIDATION_BLOCKING_INPUT: &str = include_str!(
    "../../../../validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json"
);
const EXPORT_SUCCESS_INPUT: &str =
    include_str!("../../../../validation/witness/inputs/del1005_export_results_success_input.json");
const EXPORT_MISSING_INPUT: &str = include_str!(
    "../../../../validation/witness/inputs/del1005_export_results_missing_payload_input.json"
);
const BENCHMARK_INPUT: &str = include_str!(
    "../../../../validation/witness/inputs/del1005_payload_binding_benchmark_single_case_input.json"
);
const BENCHMARK_MISSING_INPUT: &str = include_str!(
    "../../../../validation/witness/inputs/del1005_payload_binding_benchmark_payload_missing_input.json"
);
const REGRESSION_MISSING_INPUT: &str = include_str!(
    "../../../../validation/witness/inputs/del1005_payload_binding_regression_payload_missing_input.json"
);
const SIZING_INPUT: &str = include_str!("../../../../fixtures/results/hanger_sizing_input_two_span.json");
const SIZING_REFUSED_INPUT: &str =
    include_str!("../../../../fixtures/results/hanger_sizing_input_refused.json");

struct Run {
    code: i32,
    stdout: String,
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
        let _ = pipe.write_all(&input);
    });
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    Run { code: output.status.code().unwrap(), stdout: String::from_utf8(output.stdout).unwrap() }
}

fn emitted(run: &Run) -> Value {
    serde_json::from_str(&run.stdout).unwrap_or_else(|e| panic!("{e}: {}", run.stdout))
}

/// The admitted solve: the committed solve input with a model the product solves.
fn solve_admitted() -> String {
    let mut input: Value = serde_json::from_str(SOLVE_INPUT).unwrap();
    input["solve"]["preview_model"]["model"] = serde_json::from_str(SOLVED_MODEL).unwrap();
    serde_json::to_string_pretty(&input).unwrap()
}

/// `validate-input` over the solve input's request (it carries the solve
/// payload too, which `validate-input` does not read).
fn validate_admitted() -> String {
    let mut input: Value = serde_json::from_str(SOLVE_INPUT).unwrap();
    input["request"]["operation"] = json!("validate_input");
    input["request"]["operation_ref"]["ref_id"] = json!("validate_input");
    input.as_object_mut().unwrap().remove("solve");
    serde_json::to_string(&input).unwrap()
}

fn regression_admitted() -> String {
    let mut input: Value = serde_json::from_str(BENCHMARK_INPUT).unwrap();
    input["request"]["operation"] = json!("run_regression");
    input["request"]["operation_ref"]["ref_id"] = json!("run_regression");
    let object = input.as_object_mut().unwrap();
    object.remove("benchmark");
    object.insert(
        "regression".into(),
        json!({"suite": "nonlinear", "cases": ["NL-NONCONVERGENCE-LIMIT-ORIGINAL"]}),
    );
    serde_json::to_string(&input).unwrap()
}

/// (command, input, expected exit) for each verb and `size-hangers`, admitted and refused.
fn cases() -> Vec<(&'static str, String, i32)> {
    vec![
        ("solve", solve_admitted(), 0),
        ("solve", SOLVE_INPUT.to_string(), 1),
        ("validate-input", validate_admitted(), 0),
        ("validate-input", VALIDATION_BLOCKING_INPUT.to_string(), 1),
        ("export-results", EXPORT_SUCCESS_INPUT.to_string(), 0),
        ("export-results", EXPORT_MISSING_INPUT.to_string(), 1),
        ("run-benchmark", BENCHMARK_INPUT.to_string(), 0),
        ("run-benchmark", BENCHMARK_MISSING_INPUT.to_string(), 1),
        ("run-regression", regression_admitted(), 0),
        ("run-regression", REGRESSION_MISSING_INPUT.to_string(), 1),
        ("size-hangers", SIZING_INPUT.to_string(), 0),
        ("size-hangers", SIZING_REFUSED_INPUT.to_string(), 1),
    ]
}

fn intent_run(command: &str, input: &str) -> Run {
    run(&[command, "--explicit-local-private-intent"], input.as_bytes())
}

fn sorted(mut names: Vec<String>) -> Vec<String> {
    names.sort();
    names
}

#[test]
fn every_verb_and_size_hangers_admitted_and_refused_carries_verifying_run_evidence() {
    for (command, input, code) in cases() {
        let out = intent_run(command, &input);
        assert_eq!(out.code, code, "{command}: {}", out.stdout);
        let document = emitted(&out);
        let keys: Vec<&str> = document.as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(
            sorted(keys.iter().map(|k| k.to_string()).collect()),
            sorted(["payload", "decisions", "findings", "blocked", "summary", "run_evidence"].map(String::from).to_vec()),
            "{command}"
        );
        // The wrapper's members keep their order, with the evidence last.
        let positions: Vec<usize> = ["\"payload\"", "\"decisions\"", "\"findings\"", "\"blocked\"", "\"summary\"", "\"run_evidence\""]
            .iter()
            .map(|key| out.stdout.find(&format!("\n  {key}:")).unwrap_or_else(|| panic!("{command}: {key}")))
            .collect();
        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{command}: {positions:?}");
        assert_eq!(document["blocked"], false, "{command}");
        assert_eq!(verify(input.as_bytes(), &document), Ok(()), "{command}");
        let evidence = &document["run_evidence"];
        assert_eq!(evidence["evidence_version"], run_evidence::EVIDENCE_VERSION);
        assert_eq!(evidence["canonical_profile"], "openpipestress_jcs_ijson_v1");
        assert_eq!(
            evidence["invocation"],
            json!({
                "command": command,
                "solver_mode": if command == "solve" { json!("sparse_interactive") } else { Value::Null },
                "explicit_local_private_intent": true,
            })
        );
        match run_evidence::model_pointer(command) {
            Some(pointer) => assert_eq!(evidence["model"]["pointer"], pointer, "{command}"),
            None => assert_eq!(evidence["model"], json!({"absent": run_evidence::ABSENT_NOT_CARRIED})),
        }
        assert_eq!(evidence["producer"], run_evidence::producer());
        for digest in [&evidence["request"]["raw_sha256"], &evidence["request"]["canonical_sha256"], &evidence["result"]["canonical_sha256"], &evidence["binding_sha256"]] {
            let text = digest.as_str().unwrap_or_else(|| panic!("{command}: {digest}"));
            assert!(text.len() == 64 && text.bytes().all(|b| b.is_ascii_hexdigit()), "{command}");
        }
    }
}

#[test]
fn the_solver_mode_is_the_one_the_solve_ran_with() {
    let input = solve_admitted();
    let out = run(&["solve", "--explicit-local-private-intent", "--solver-mode", "dense_scrutiny"], input.as_bytes());
    assert_eq!(out.code, 0);
    let document = emitted(&out);
    assert_eq!(document["run_evidence"]["invocation"]["solver_mode"], "dense_scrutiny");
    assert_eq!(verify(input.as_bytes(), &document), Ok(()));
    // The sparse result of the same request is another result: its evidence does
    // not verify against the dense one.
    let mut swapped = emitted(&intent_run("solve", &input));
    swapped["run_evidence"] = document["run_evidence"].clone();
    assert_eq!(verify(input.as_bytes(), &swapped), Err(vec!["result.canonical_sha256".to_string()]));
}

#[test]
fn a_blocked_result_and_a_run_that_exits_2_carry_no_run_evidence() {
    for (command, input) in [("solve", solve_admitted()), ("size-hangers", SIZING_INPUT.to_string())] {
        let out = run(&[command], input.as_bytes());
        assert_eq!(out.code, 1, "{command}");
        let document = emitted(&out);
        assert_eq!(document["blocked"], true, "{command}");
        assert_eq!(document["payload"], Value::Null, "{command}");
        assert!(document.get("run_evidence").is_none(), "{command}");
        assert!(!out.stdout.contains("run_evidence"), "{command}");
    }
    for args in [vec!["solve", "--explicit-local-private-intent"], vec!["preview", "--explicit-local-private-intent"]] {
        let out = run(&args, b"{");
        assert_eq!(out.code, 2, "{args:?}");
        assert!(!out.stdout.contains("run_evidence"), "{args:?}: {}", out.stdout);
    }
    let out = run(&["solve", "--explicit-local-private-intent", "--solver-mode"], b"{}");
    assert_eq!(out.code, 2);
    assert!(out.stdout.is_empty());
}

#[test]
fn each_substitution_fails_by_the_named_digest() {
    let input = solve_admitted();
    let document = emitted(&intent_run("solve", &input));
    assert_eq!(verify(input.as_bytes(), &document), Ok(()));

    // Another request.
    let other = SOLVE_INPUT.as_bytes();
    assert_eq!(
        verify(other, &document),
        Err(sorted(vec!["model.canonical_sha256".into(), "request.canonical_sha256".into(), "request.raw_sha256".into()]))
    );
    // The same request with its model changed in one member.
    let mut changed: Value = serde_json::from_str(&input).unwrap();
    let model = changed.pointer_mut(run_evidence::SOLVE_MODEL_POINTER).unwrap();
    model["project"]["name"] = json!(format!("{} (edited)", model["project"]["name"].as_str().unwrap_or("model")));
    let changed = serde_json::to_string_pretty(&changed).unwrap();
    assert_eq!(
        verify(changed.as_bytes(), &document),
        Err(sorted(vec!["model.canonical_sha256".into(), "request.canonical_sha256".into(), "request.raw_sha256".into()]))
    );
    // The same request re-encoded (other whitespace): only the raw bytes differ.
    let reencoded = serde_json::to_string(&serde_json::from_str::<Value>(&input).unwrap()).unwrap();
    assert_eq!(verify(reencoded.as_bytes(), &document), Err(vec!["request.raw_sha256".to_string()]));
    // The result edited.
    let mut edited = document.clone();
    edited["payload"]["runner_result"]["run_id"] = json!("run:headless-preview:another");
    assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["result.canonical_sha256".to_string()]));
    // The evidence edited: a stated member changed without its binding.
    let mut edited = document.clone();
    edited["run_evidence"]["invocation"]["solver_mode"] = json!("dense_scrutiny");
    assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["binding_sha256".to_string()]));
    let mut edited = document.clone();
    edited["run_evidence"]["producer"]["build_ref"] = json!("another@0.0.0");
    assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["binding_sha256".to_string()]));
    // Evidence holding a value the profile refuses binds nothing, whatever its binding states.
    for binding in [Value::Null, json!({"unavailable": "CHECKED-JSON-UNSAFE-INTEGER"})] {
        let mut edited = document.clone();
        edited["run_evidence"]["producer"]["build_ref"] = json!(9007199254740993u64);
        edited["run_evidence"]["binding_sha256"] = binding;
        assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["binding_sha256".to_string()]));
    }
    // The evidence re-paired: another run's evidence on this result.
    let refused = emitted(&intent_run("solve", SOLVE_INPUT));
    let mut repaired = document.clone();
    repaired["run_evidence"] = refused["run_evidence"].clone();
    assert_eq!(
        verify(input.as_bytes(), &repaired),
        Err(sorted(vec![
            "model.canonical_sha256".into(),
            "request.canonical_sha256".into(),
            "request.raw_sha256".into(),
            "result.canonical_sha256".into()
        ]))
    );
    assert_eq!(verify(SOLVE_INPUT.as_bytes(), &repaired), Err(vec!["result.canonical_sha256".to_string()]));
    // The evidence removed.
    let mut stripped = document.clone();
    stripped.as_object_mut().unwrap().remove("run_evidence");
    assert_eq!(verify(input.as_bytes(), &stripped), Err(vec!["run_evidence".to_string()]));
}

#[test]
fn the_same_request_run_twice_gives_the_same_result_digest() {
    for (command, input, _) in cases() {
        let first = intent_run(command, &input);
        let second = intent_run(command, &input);
        assert_eq!(first.stdout, second.stdout, "{command}");
        assert_eq!(
            emitted(&first)["run_evidence"]["result"]["canonical_sha256"],
            emitted(&second)["run_evidence"]["result"]["canonical_sha256"],
            "{command}"
        );
    }
}

#[test]
fn a_request_outside_the_canonical_profile_is_stated_as_unavailable() {
    let base = validate_admitted();
    let trimmed = base.trim_end().strip_suffix('}').unwrap();
    for (member, code) in [
        ("9007199254740993", "CHECKED-JSON-UNSAFE-INTEGER"),
        ("1e16", "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT"),
        ("{\"a\":1,\"a\":2}", "CHECKED-JSON-INVALID"),
        ("\"\\ud800\"", "CHECKED-JSON-INVALID"),
    ] {
        let input = format!("{trimmed},\"note\":{member}}}");
        let out = intent_run("validate-input", &input);
        assert_eq!(out.code, 0, "{member}: {}", out.stdout);
        let document = emitted(&out);
        let evidence = &document["run_evidence"];
        assert_eq!(evidence["request"]["canonical_sha256"], json!({"unavailable": code}), "{member}");
        assert_eq!(evidence["request"]["raw_sha256"].as_str().unwrap().len(), 64);
        // The refusal is stated by code; the evidence quotes no value of the request.
        assert!(!evidence.to_string().contains("9007199254740993") && !evidence.to_string().contains("note"));
        assert_eq!(verify(input.as_bytes(), &document), Ok(()), "{member}");
    }
    // A model document outside the profile: its digest is unavailable too.
    let trimmed = SIZING_INPUT.trim_end().strip_suffix('}').unwrap();
    let input = format!("{trimmed},\"note\":9007199254740993}}");
    let out = intent_run("size-hangers", &input);
    let document = emitted(&out);
    assert_eq!(
        document["run_evidence"]["model"],
        json!({"pointer": run_evidence::SIZE_HANGERS_MODEL_POINTER, "canonical_sha256": {"unavailable": "CHECKED-JSON-UNSAFE-INTEGER"}})
    );
    assert_eq!(verify(input.as_bytes(), &document), Ok(()));
}
