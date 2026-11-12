//! DEL-10-05 REQ-017 / AC-013: the run evidence of the actual
//! `openpipestress-runner` process, for each verb and for `size-hangers`,
//! admitted and refused, with and without local-private intent.
//!
//! - With intent the control passes every request: the evidence is complete and
//!   verifies; each substitution (another request, a model changed in one
//!   member, the result edited, the evidence edited or re-paired) fails by the
//!   named digest; the same request run twice gives the same result digest; a
//!   request outside the canonical profile is stated as unavailable.
//! - Without intent the control blocks the output, or passes it and blocks the
//!   request itself: then the request and model digests are withheld and stated
//!   as withheld, and no digest of the request leaves the process.
//! - A request the control cannot judge in full is withheld whatever the intent.
//! - A blocked output, a run that exits 2 and `read` carry no evidence.
//!
//! The verifier is the crate's `run_evidence::verify`; `tests/test_run_evidence.py`
//! checks the same runs with a verifier that imports no product code. The
//! inputs are the committed, invented runner and sizing inputs, read at compile
//! time.

use open_pipe_stress_canonical_json::{canonical_json, canonical_json_checked_v1_text};
use open_pipe_stress_headless_runner::redaction_binding::control_local_private;
use open_pipe_stress_headless_runner::run_evidence::{self, verify, Checked, RequestJudgment};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
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
        let _ = pipe.write_all(&input);
    });
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    Run {
        code: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn emitted(run: &Run) -> Value {
    serde_json::from_str(&run.stdout).unwrap_or_else(|e| panic!("{e}: {}", run.stdout))
}

fn sha(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// The admitted solve: the committed solve input with a model the product solves.
fn solve_admitted() -> String {
    let mut input: Value = serde_json::from_str(SOLVE_INPUT).unwrap();
    input["solve"]["preview_model"]["model"] = serde_json::from_str(SOLVED_MODEL).unwrap();
    serde_json::to_string_pretty(&input).unwrap()
}

/// `validate-input` over the solve input's request.
fn validate_admitted() -> String {
    let mut input: Value = serde_json::from_str(SOLVE_INPUT).unwrap();
    input["request"]["operation"] = json!("validate_input");
    input["request"]["operation_ref"]["ref_id"] = json!("validate_input");
    input.as_object_mut().unwrap().remove("solve");
    serde_json::to_string(&input).unwrap()
}

/// `validate-input` carrying a model document beside its request (which it
/// does not read): the case where the control passes the output without intent.
fn validate_carrying_a_model() -> String {
    let mut input: Value = serde_json::from_str(&validate_admitted()).unwrap();
    input["solve"] = json!({"preview_model": {"model": serde_json::from_str::<Value>(SOLVED_MODEL).unwrap()}});
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

/// (command, input, expected exit with intent) for each verb and `size-hangers`, admitted and refused.
fn cases() -> Vec<(&'static str, String, i32)> {
    vec![
        ("solve", solve_admitted(), 0),
        ("solve", SOLVE_INPUT.to_string(), 1),
        ("validate-input", validate_admitted(), 0),
        ("validate-input", validate_carrying_a_model(), 0),
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

/// Every digest of the request a complete evidence would carry: raw bytes,
/// canonical request and (where the command carries one) the model.
fn request_digests(command: &str, input: &str) -> Vec<String> {
    let mut digests = vec![sha(input.as_bytes())];
    if let Ok(canonical) = canonical_json_checked_v1_text(input) {
        digests.push(sha(canonical.as_bytes()));
        let value: Value = serde_json::from_str(&canonical).unwrap();
        if let Some(model) = run_evidence::model_pointer(command).and_then(|p| value.pointer(p)) {
            digests.push(sha(canonical_json(model).as_bytes()));
        }
    }
    digests
}

#[test]
fn with_intent_every_verb_and_size_hangers_admitted_and_refused_carries_complete_verifying_evidence() {
    for (command, input, code) in cases() {
        let out = intent_run(command, &input);
        assert_eq!(out.code, code, "{command}: {}", out.stdout);
        let document = emitted(&out);
        let keys: Vec<String> = document.as_object().unwrap().keys().cloned().collect();
        assert_eq!(
            sorted(keys),
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
        assert_eq!(verify(input.as_bytes(), &document), Ok(Checked::Verified), "{command}");
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
        assert_eq!(evidence["request"]["raw_sha256"], sha(input.as_bytes()), "{command}");
        for digest in [&evidence["request"]["canonical_sha256"], &evidence["result"]["canonical_sha256"], &evidence["binding_sha256"]] {
            let text = digest.as_str().unwrap_or_else(|| panic!("{command}: {digest}"));
            assert!(text.len() == 64 && text.bytes().all(|b| b.is_ascii_hexdigit()), "{command}");
        }
    }
}

/// Path: every verb and `size-hangers`, admitted and refused, without intent.
/// Either the control blocks the output (no evidence), or it passes the output
/// and, judging the request itself, blocks it: the request and model are stated
/// withheld, the result digest is kept, and no digest of the request appears on
/// standard output or standard error.
#[test]
fn without_intent_no_digest_of_the_request_leaves_the_process() {
    let mut passed = Vec::new();
    for (command, input, _) in cases() {
        let out = run(&[command], input.as_bytes());
        let document = emitted(&out);
        for digest in request_digests(command, &input) {
            assert!(!out.stdout.contains(&digest) && !out.stderr.contains(&digest), "{command}: a request digest left");
        }
        if document["blocked"] == true {
            assert_eq!(out.code, 1, "{command}");
            assert!(document.get("run_evidence").is_none(), "{command}");
            continue;
        }
        passed.push(command);
        let evidence = &document["run_evidence"];
        let statement = json!({"withheld": run_evidence::WITHHELD_BLOCKED});
        assert_eq!(evidence["request"], statement, "{command}");
        assert_eq!(evidence["model"], statement, "{command}");
        assert_eq!(evidence["invocation"]["explicit_local_private_intent"], false);
        assert_eq!(
            verify(input.as_bytes(), &document),
            Ok(Checked::Withheld(vec!["model".into(), "request".into()])),
            "{command}"
        );
        // A withheld request cannot be compared: another request is not detected,
        // and the outcome is still withheld, never verified.
        assert_eq!(
            verify(SOLVE_INPUT.as_bytes(), &document),
            Ok(Checked::Withheld(vec!["model".into(), "request".into()]))
        );
        // The result digest is kept and still binds the result.
        let mut edited = document.clone();
        edited["summary"]["redacted_count"] = json!(7);
        assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["result.canonical_sha256".to_string()]));
        // A withheld statement replaced by digests fails the binding.
        let mut unwithheld = document.clone();
        unwithheld["run_evidence"]["request"] = json!({"raw_sha256": sha(input.as_bytes()), "canonical_sha256": "0".repeat(64)});
        assert!(verify(input.as_bytes(), &unwithheld).unwrap_err().contains(&"binding_sha256".to_string()));
    }
    // The paths that do pass without intent today: validate-input, whose output
    // names no private value, whatever its request carries.
    assert_eq!(passed, ["validate-input", "validate-input"]);
}

/// Path: a request the control passes without intent (every value it classes
/// private stated unknown) carries complete evidence without intent.
#[test]
fn a_request_the_control_passes_carries_complete_evidence_without_intent() {
    let mut input: Value = serde_json::from_str(&validate_admitted()).unwrap();
    for pointer in ["/request/project_ref/ref_type", "/request/project_ref/ref_id", "/request/model_ref/ref_type",
                    "/request/model_ref/ref_id", "/request/provenance/source_location"] {
        *input.pointer_mut(pointer).unwrap() = json!("unknown");
    }
    let input = serde_json::to_string(&input).unwrap();
    let judged: Value = serde_json::from_str(&input).unwrap();
    let judgment = control_local_private(judged, false);
    let blocking: Vec<&str> = judgment.decisions.iter().filter(|d| d.action == "block_export").map(|d| d.path.as_str()).collect();
    assert!(!judgment.blocked, "the control passes this request: {blocking:?}");
    let out = run(&["validate-input"], input.as_bytes());
    let document = emitted(&out);
    assert_eq!(document["blocked"], false, "{}", out.stdout);
    assert_eq!(verify(input.as_bytes(), &document), Ok(Checked::Verified));
    assert_eq!(document["run_evidence"]["request"]["raw_sha256"], sha(input.as_bytes()));
}

/// Path: a request the control cannot judge in full (a duplicate member, whose
/// parsed value would hide the first occurrence from the control; a lone
/// surrogate; a number out of range) is withheld whatever the intent.
#[test]
fn a_request_the_control_cannot_judge_is_withheld_whatever_the_intent() {
    let base = validate_admitted();
    let trimmed = base.trim_end().strip_suffix('}').unwrap();
    for member in [
        // The control would pass "unknown" and never see the first value.
        "\"source_location\":\"/an/invented/private/path\",\"source_location\":\"unknown\"",
        "\"note\":\"\\ud800\"",
        "\"note\":1e400",
    ] {
        let input = format!("{trimmed},{member}}}");
        for args in [vec!["validate-input", "--explicit-local-private-intent"], vec!["validate-input"]] {
            let out = run(&args, input.as_bytes());
            let document = emitted(&out);
            if document["blocked"] == true {
                assert!(document.get("run_evidence").is_none());
                continue;
            }
            let statement = json!({"withheld": run_evidence::WITHHELD_UNJUDGED});
            assert_eq!(document["run_evidence"]["request"], statement, "{member} {args:?}");
            assert_eq!(document["run_evidence"]["model"], statement, "{member} {args:?}");
            assert!(!out.stdout.contains(&sha(input.as_bytes())), "{member}");
            assert_eq!(
                verify(input.as_bytes(), &document),
                Ok(Checked::Withheld(vec!["model".into(), "request".into()]))
            );
        }
    }
}

/// Path: the result digest of an output the control passed with members
/// stripped covers the output as emitted, never what the control removed.
#[test]
fn the_result_digest_covers_the_output_as_emitted_not_what_the_control_stripped() {
    let output = json!({"artifact": "invented", "explicit_local_private_intent": true,
                        "export_policy": {"user_intent": "invented private statement", "kept": 1}});
    let controlled = control_local_private(output.clone(), true);
    let emitted = serde_json::to_value(&controlled).unwrap();
    assert!(!emitted.to_string().contains("invented private statement"));
    let raw = br#"{"invented":"request"}"#;
    let invocation = run_evidence::Invocation { command: "validate-input", solver_mode: None, explicit_local_private_intent: true };
    let evidence = run_evidence::form(raw, invocation, RequestJudgment::Passed, &emitted);
    let stated = evidence["result"]["canonical_sha256"].as_str().unwrap();
    assert_eq!(stated, sha(canonical_json(&emitted).as_bytes()));
    let mut unstripped = emitted.clone();
    unstripped["payload"] = output;
    assert_ne!(stated, sha(canonical_json(&unstripped).as_bytes()));
}

/// Path: the producer block names only components, versions and a build
/// reference; it carries no path, host or request value.
#[test]
fn the_producer_block_carries_no_path_host_or_request_value() {
    let document = emitted(&intent_run("solve", &solve_admitted()));
    let producer = &document["run_evidence"]["producer"];
    assert_eq!(producer, &run_evidence::producer());
    assert_eq!(
        producer.as_object().unwrap().keys().cloned().collect::<Vec<_>>(),
        ["build_ref", "product", "retained_build_identity", "runner"].map(String::from).to_vec()
    );
    let text = producer.to_string();
    for forbidden in ["/", "\\", "precision-connected", "invented"] {
        if producer["retained_build_identity"].is_string() {
            continue; // a registered build's identity is its committed registration text
        }
        assert!(!text.contains(forbidden), "{forbidden}: {text}");
    }
}

/// A withheld-evidence result is told apart from a complete one and from one
/// without run evidence, and a check that requires the binding fails closed on
/// the withheld form and on the absent one.
#[test]
fn withheld_absent_and_complete_evidence_are_told_apart_and_only_complete_binds() {
    use run_evidence::{bound, Binding};
    let input = validate_carrying_a_model();
    let withheld = emitted(&run(&["validate-input"], input.as_bytes()));
    let complete = emitted(&intent_run("validate-input", &input));
    let mut absent = complete.clone();
    absent.as_object_mut().unwrap().remove("run_evidence");
    // Stated member by member.
    for member in ["request", "model"] {
        assert_eq!(withheld["run_evidence"][member], json!({"withheld": run_evidence::WITHHELD_BLOCKED}));
    }
    assert_eq!(verify(input.as_bytes(), &complete), Ok(Checked::Verified));
    assert_eq!(verify(input.as_bytes(), &withheld), Ok(Checked::Withheld(vec!["model".into(), "request".into()])));
    assert_eq!(verify(input.as_bytes(), &absent), Err(vec!["run_evidence".to_string()]));
    assert_eq!(bound(input.as_bytes(), &complete), Binding::Bound);
    assert_eq!(bound(input.as_bytes(), &withheld), Binding::NotBoundWithheld);
    assert_eq!(bound(input.as_bytes(), &absent), Binding::NotBoundNoRunEvidence);
    assert!(!bound(input.as_bytes(), &withheld).is_bound() && !bound(input.as_bytes(), &absent).is_bound());
    // Withholding only one member is not the withheld form: it is compared, and fails.
    let mut half = withheld.clone();
    half["run_evidence"]["model"] = complete["run_evidence"]["model"].clone();
    assert!(matches!(bound(input.as_bytes(), &half), Binding::NotBoundMismatch(_)));
}

#[test]
fn the_solver_mode_is_the_one_the_solve_ran_with() {
    let input = solve_admitted();
    let out = run(&["solve", "--explicit-local-private-intent", "--solver-mode", "dense_scrutiny"], input.as_bytes());
    assert_eq!(out.code, 0);
    let document = emitted(&out);
    assert_eq!(document["run_evidence"]["invocation"]["solver_mode"], "dense_scrutiny");
    assert_eq!(verify(input.as_bytes(), &document), Ok(Checked::Verified));
    let mut swapped = emitted(&intent_run("solve", &input));
    swapped["run_evidence"] = document["run_evidence"].clone();
    assert_eq!(verify(input.as_bytes(), &swapped), Err(vec!["result.canonical_sha256".to_string()]));
}

#[test]
fn a_blocked_result_a_run_that_exits_2_and_read_carry_no_run_evidence() {
    for (command, input) in [("solve", solve_admitted()), ("size-hangers", SIZING_INPUT.to_string())] {
        let out = run(&[command], input.as_bytes());
        assert_eq!(out.code, 1, "{command}");
        let document = emitted(&out);
        assert_eq!(document["blocked"], true, "{command}");
        assert_eq!(document["payload"], Value::Null, "{command}");
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
    let out = run(&["read", "retained-precision"], b"{}");
    assert!(!out.stdout.contains("run_evidence"), "{}", out.stdout);
}

#[test]
fn each_substitution_fails_by_the_named_digest() {
    let input = solve_admitted();
    let document = emitted(&intent_run("solve", &input));
    assert_eq!(verify(input.as_bytes(), &document), Ok(Checked::Verified));
    let request_and_model =
        sorted(vec!["model.canonical_sha256".into(), "request.canonical_sha256".into(), "request.raw_sha256".into()]);

    assert_eq!(verify(SOLVE_INPUT.as_bytes(), &document), Err(request_and_model.clone()));
    let mut changed: Value = serde_json::from_str(&input).unwrap();
    let model = changed.pointer_mut(run_evidence::SOLVE_MODEL_POINTER).unwrap();
    model["project"]["name"] = json!(format!("{} (edited)", model["project"]["name"].as_str().unwrap()));
    let changed = serde_json::to_string_pretty(&changed).unwrap();
    assert_eq!(verify(changed.as_bytes(), &document), Err(request_and_model.clone()));
    let reencoded = serde_json::to_string(&serde_json::from_str::<Value>(&input).unwrap()).unwrap();
    assert_eq!(verify(reencoded.as_bytes(), &document), Err(vec!["request.raw_sha256".to_string()]));
    let mut edited = document.clone();
    edited["payload"]["runner_result"]["run_id"] = json!("run:headless-preview:another");
    assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["result.canonical_sha256".to_string()]));
    for (pointer, value) in [("/run_evidence/invocation/solver_mode", json!("dense_scrutiny")),
                             ("/run_evidence/producer/build_ref", json!("another@0.0.0"))] {
        let mut edited = document.clone();
        *edited.pointer_mut(pointer).unwrap() = value;
        assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["binding_sha256".to_string()]), "{pointer}");
    }
    // Stating the evidence withheld after the fact fails the binding.
    let mut edited = document.clone();
    edited["run_evidence"]["request"] = json!({"withheld": run_evidence::WITHHELD_BLOCKED});
    edited["run_evidence"]["model"] = json!({"withheld": run_evidence::WITHHELD_BLOCKED});
    assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["binding_sha256".to_string()]));
    for binding in [Value::Null, json!({"unavailable": "CHECKED-JSON-UNSAFE-INTEGER"})] {
        let mut edited = document.clone();
        edited["run_evidence"]["producer"]["build_ref"] = json!(9007199254740993u64);
        edited["run_evidence"]["binding_sha256"] = binding;
        assert_eq!(verify(input.as_bytes(), &edited), Err(vec!["binding_sha256".to_string()]));
    }
    let refused = emitted(&intent_run("solve", SOLVE_INPUT));
    let mut repaired = document.clone();
    repaired["run_evidence"] = refused["run_evidence"].clone();
    let mut expected = request_and_model.clone();
    expected.push("result.canonical_sha256".into());
    assert_eq!(verify(input.as_bytes(), &repaired), Err(expected));
    assert_eq!(verify(SOLVE_INPUT.as_bytes(), &repaired), Err(vec!["result.canonical_sha256".to_string()]));
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
    for (member, code) in [("9007199254740993", "CHECKED-JSON-UNSAFE-INTEGER"), ("1e16", "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT")] {
        let input = format!("{trimmed},\"note\":{member}}}");
        let out = intent_run("validate-input", &input);
        assert_eq!(out.code, 0, "{member}: {}", out.stdout);
        let document = emitted(&out);
        let evidence = &document["run_evidence"];
        assert_eq!(evidence["request"]["canonical_sha256"], json!({"unavailable": code}), "{member}");
        assert_eq!(evidence["request"]["raw_sha256"], sha(input.as_bytes()));
        // The refusal is stated by code; the evidence quotes no value of the request.
        assert!(!evidence.to_string().contains("9007199254740993") && !evidence.to_string().contains("note"));
        assert_eq!(verify(input.as_bytes(), &document), Ok(Checked::Verified), "{member}");
    }
    let trimmed = SIZING_INPUT.trim_end().strip_suffix('}').unwrap();
    let input = format!("{trimmed},\"note\":9007199254740993}}");
    let document = emitted(&intent_run("size-hangers", &input));
    assert_eq!(
        document["run_evidence"]["model"],
        json!({"pointer": run_evidence::SIZE_HANGERS_MODEL_POINTER, "canonical_sha256": {"unavailable": "CHECKED-JSON-UNSAFE-INTEGER"}})
    );
    assert_eq!(verify(input.as_bytes(), &document), Ok(Checked::Verified));
}
