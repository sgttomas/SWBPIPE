//! The release timing driver's functional tests: its JSON line, its entry-error
//! exit, and that the bytes it writes are the product's publication unchanged.
//! Nothing here measures or asserts a time or memory bound.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_release_timing");

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/product_preview").join(name)
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("release_timing_test_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn run(args: &[&str]) -> (i32, Value) {
    let output = Command::new(BIN).args(args).output().expect("run release_timing");
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "exactly one JSON line: {stdout}");
    (output.status.code().expect("exit code"), serde_json::from_str(lines[0]).expect("JSON line"))
}

/// The worker's exact bytes, computed independently of the driver.
fn product_bytes(document: &Value, mode: PreviewSolverMode) -> String {
    let request = json!({"model": document.clone(), "materials": []});
    serde_json::to_string(&run_linear_static_preview_value_with_mode(request, mode).expect("entry")).expect("serialize")
}

const REQUIRED: [&str; 28] = [
    "schema", "driver", "invocation", "document", "route", "mode", "nodes", "members", "supports", "cases",
    "combinations", "status", "blocking_codes", "first_blocking_messages", "diagnostic_counts", "rows", "output_bytes",
    "output_sha256", "rows_per_basis", "row_kinds", "row_classes", "seconds", "peak_memory",
    "driver_peak_footprint_bytes", "repeats", "limits", "parse_back_ok", "outputs_identical_across_repeats",
];
const SOLVE_STAGES: [&str; 7] = ["read", "parse", "request", "entry", "serialize", "release_envelope", "parse_back"];
const MODEL_CHECK_STAGES: [&str; 4] = ["read", "parse", "model_checks", "serialize"];

#[test]
fn prints_one_release_timing_line_with_the_required_keys() {
    let model = fixture("invented_demo_model.json");
    let (code, out) = run(&[model.to_str().unwrap()]);
    assert_eq!(code, 0, "{out}");
    for key in REQUIRED {
        assert!(out.get(key).is_some(), "missing {key}: {out}");
    }
    assert_eq!(out["schema"], "release-timing-v2");
    assert_eq!(out["invocation"], "solve");
    assert_eq!(out["mode"], "sparse_interactive");
    assert_eq!(out["document"]["input_kind"], "model_document");
    let bytes = std::fs::read(&model).unwrap();
    assert_eq!(out["document"]["bytes"], bytes.len());
    let document: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(out["nodes"], document["nodes"].as_array().unwrap().len());
    assert_eq!(out["route"], if document.get("pressure_contract").is_some() { "EX" } else { "PF" });
    assert_eq!(out["parse_back_ok"], true);
    for stage in ["read", "parse", "request", "entry", "serialize", "release_envelope", "parse_back", "total"] {
        assert!(out["seconds"][stage].as_f64().is_some(), "stage {stage}");
    }
    for stage in ["read", "parse", "request", "entry", "serialize", "release_envelope", "parse_back"] {
        assert!(out["peak_memory"][stage].get("ru_maxrss_bytes").is_some(), "stage {stage}");
    }
    // The solve runs the worker's stages and nothing else: no model checks, no side stage.
    assert_eq!(out["stages"], json!(SOLVE_STAGES));
    assert!(out.get("side_stages").is_none() && out.get("model_checks").is_none(), "{out}");
    let stages: Vec<&String> = out["seconds"].as_object().unwrap().keys().filter(|k| *k != "total").collect();
    assert_eq!(stages.len(), SOLVE_STAGES.len(), "only the worker's stages are timed: {stages:?}");
    let worker: f64 = SOLVE_STAGES.iter().map(|stage| out["seconds"][*stage].as_f64().unwrap()).sum();
    assert!((out["seconds"]["total"].as_f64().unwrap() - worker).abs() < 1e-9, "total is the worker stages");
    // parse_back is described as a cheaper stand-in and a lower bound, never as the worker's peak.
    let limits = out["limits"].to_string();
    assert!(limits.contains("cheaper stand-in") && limits.contains("lower bound"), "{limits}");
    assert!(limits.contains("read-back peak is not reproduced"), "{limits}");
    assert_eq!(
        out["driver_peak_footprint_bytes"],
        out["peak_memory"]["parse_back"]["lifetime_max_phys_footprint_bytes"]
    );
    let rows = out["rows"].as_u64().unwrap();
    let by_kind: u64 = out["row_kinds"].as_object().unwrap().values().map(|t| t["rows"].as_u64().unwrap()).sum();
    let by_class: u64 = out["row_classes"].as_object().unwrap().values().map(|t| t["rows"].as_u64().unwrap()).sum();
    assert_eq!((by_kind, by_class), (rows, rows));
    let class_bytes: u64 = out["row_classes"].as_object().unwrap().values().map(|t| t["bytes"].as_u64().unwrap()).sum();
    assert!(class_bytes > 0 && class_bytes < out["output_bytes"].as_u64().unwrap());
    let expected = product_bytes(&document, PreviewSolverMode::SparseInteractive);
    assert_eq!(out["output_bytes"], expected.len());
    // No absolute path in the line (the runner keeps its records path-free).
    assert!(!out.to_string().contains(model.to_str().unwrap()));
}

#[test]
fn each_stage_writes_one_progress_line_to_stderr() {
    let model = fixture("invented_demo_model.json");
    let output = Command::new(BIN).arg(&model).args(["--repeats", "2"]).output().expect("run release_timing");
    assert!(output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("utf-8 stderr");
    let lines: Vec<Value> = stderr
        .lines()
        .filter_map(|line| line.strip_prefix("release-timing-stage "))
        .map(|json| serde_json::from_str(json).expect("progress JSON"))
        .collect();
    let stages = SOLVE_STAGES;
    assert_eq!(lines.len(), 2 * stages.len(), "{stderr}");
    for (index, line) in lines.iter().enumerate() {
        assert_eq!(line["repeat"], index / stages.len());
        assert_eq!(line["stage"], stages[index % stages.len()]);
        assert!(line["seconds"].as_f64().is_some());
        for key in ["ru_maxrss_bytes", "phys_footprint_bytes", "lifetime_max_phys_footprint_bytes"] {
            assert!(line.get(key).is_some(), "{key}: {line}");
        }
    }
    // Stdout keeps its one JSON line, with the same stage seconds as the progress lines.
    let stdout = String::from_utf8(output.stdout).unwrap();
    let out: Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(stdout.lines().count(), 1);
    assert_eq!(out["seconds"]["entry"], lines[3]["seconds"]);
}

#[test]
fn written_bytes_are_the_products_publication_in_both_modes() {
    let model = fixture("invented_demo_model.json");
    let document: Value = serde_json::from_slice(&std::fs::read(&model).unwrap()).unwrap();
    for (flag, mode) in [("sparse", PreviewSolverMode::SparseInteractive), ("dense", PreviewSolverMode::DenseScrutiny)] {
        let target = scratch(&format!("written_{flag}.json"));
        let (code, out) = run(&[model.to_str().unwrap(), "--mode", flag, "--write", target.to_str().unwrap()]);
        assert_eq!(code, 0, "{out}");
        assert_eq!(out["mode"], mode.as_str());
        assert_eq!(out["written"], true);
        let written = std::fs::read_to_string(&target).unwrap();
        assert_eq!(written, product_bytes(&document, mode), "the driver altered publication ({flag})");
        std::fs::remove_file(target).unwrap();
    }
}

#[test]
fn a_request_document_solves_its_model() {
    let request = fixture("rf_skew_t_cant_off_122_r1e-04.request.json");
    let value: Value = serde_json::from_slice(&std::fs::read(&request).unwrap()).unwrap();
    let target = scratch("written_request.json");
    let (code, out) = run(&[request.to_str().unwrap(), "--write", target.to_str().unwrap()]);
    assert_eq!(code, 0, "{out}");
    assert_eq!(out["document"]["input_kind"], "request");
    let written = std::fs::read_to_string(&target).unwrap();
    assert_eq!(written, product_bytes(&value["model"], PreviewSolverMode::SparseInteractive));
    std::fs::remove_file(target).unwrap();
}

#[test]
fn repeats_rerun_the_whole_sequence() {
    let model = fixture("invented_demo_model.json");
    let (code, out) = run(&[model.to_str().unwrap(), "--repeats", "2"]);
    assert_eq!(code, 0, "{out}");
    let repeats = out["repeats"].as_array().unwrap();
    assert_eq!(repeats.len(), 2);
    assert_eq!(out["outputs_identical_across_repeats"], true);
    assert_eq!(repeats[0]["output_sha256"], repeats[1]["output_sha256"]);
    assert!(repeats[1]["seconds"]["entry"].as_f64().is_some());
}

#[test]
fn an_invalid_document_is_an_entry_error_with_exit_3() {
    let target = scratch("invalid.json");
    std::fs::write(&target, b"{\"nodes\": [").unwrap();
    let (code, out) = run(&[target.to_str().unwrap()]);
    assert_eq!(code, 3, "{out}");
    assert_eq!(out["schema"], "release-timing-v2");
    assert_eq!(out["status"], "entry_error");
    assert_eq!(out["entry_error"]["stage"], "parse");
    assert!(out["entry_error"]["message"].as_str().unwrap().starts_with("PREVIEW-SOLVE-MODEL-INVALID: "));
    std::fs::remove_file(target).unwrap();
}

#[test]
fn an_entry_refusal_is_an_entry_error_with_exit_3() {
    // A JSON value the entry cannot read as a model (not a parse error).
    let target = scratch("not_a_model.json");
    std::fs::write(&target, b"[1, 2, 3]").unwrap();
    let (code, out) = run(&[target.to_str().unwrap()]);
    assert_eq!(code, 3, "{out}");
    assert_eq!(out["status"], "entry_error");
    assert_eq!(out["entry_error"]["stage"], "entry");
    let request = json!({"model": [1, 2, 3], "materials": []});
    let expected = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive).err();
    assert_eq!(out["entry_error"]["message"].as_str(), expected.as_deref());
    std::fs::remove_file(target).unwrap();
}

#[test]
fn the_model_checks_are_an_invocation_of_their_own() {
    // `--model-checks`: the desktop check worker's stages on the document, its report as its output,
    // in a process that runs no solve.
    let model = fixture("invented_demo_model.json");
    let target = scratch("model_checks_report.json");
    let output = Command::new(BIN)
        .arg(&model)
        .args(["--model-checks", "--repeats", "2", "--write", target.to_str().unwrap()])
        .output()
        .expect("run release_timing");
    assert_eq!(output.status.code(), Some(0));
    let out: Value = serde_json::from_str(String::from_utf8(output.stdout).unwrap().trim()).unwrap();
    assert_eq!(out["schema"], "release-timing-v2");
    assert_eq!(out["invocation"], "model_checks");
    assert_eq!(out["stages"], json!(MODEL_CHECK_STAGES));
    assert_eq!(out["status"], "MODEL_CHECKS_REPORTED");
    assert_eq!(out["blocking_codes"], json!([]));
    assert!(out.get("mode").is_none() && out.get("rows").is_none() && out.get("parse_back_ok").is_none(), "{out}");
    assert_eq!(out["model_checks"]["checked"], true);
    for stage in MODEL_CHECK_STAGES {
        assert!(out["seconds"][stage].as_f64().is_some(), "stage {stage}");
        assert!(out["peak_memory"][stage].get("lifetime_max_phys_footprint_bytes").is_some(), "stage {stage}");
    }
    assert_eq!(out["driver_peak_footprint_bytes"], out["peak_memory"]["serialize"]["lifetime_max_phys_footprint_bytes"]);
    // The written report is the public model-check entry's report on the same model.
    let document: Value = serde_json::from_slice(&std::fs::read(&model).unwrap()).unwrap();
    let expected = serde_json::to_string(&open_pipe_stress_operation_applier::model_checks_owned(document)).unwrap();
    assert_eq!(std::fs::read_to_string(&target).unwrap(), expected);
    assert_eq!(out["output_bytes"], expected.len());
    assert_eq!(out["outputs_identical_across_repeats"], true);
    std::fs::remove_file(target).unwrap();
    // Its progress lines are its own stages only.
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stages: Vec<String> = stderr
        .lines()
        .filter_map(|line| line.strip_prefix("release-timing-stage "))
        .map(|json| serde_json::from_str::<Value>(json).unwrap()["stage"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(stages, [MODEL_CHECK_STAGES, MODEL_CHECK_STAGES].concat());
}

#[test]
fn the_model_checks_take_no_solver_mode_and_report_an_unreadable_document() {
    let model = fixture("invented_demo_model.json");
    let output = Command::new(BIN).arg(&model).args(["--model-checks", "--mode", "sparse"]).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("no solver mode"));
    let target = scratch("invalid_checks.json");
    std::fs::write(&target, b"{\"nodes\": [").unwrap();
    let (code, out) = run(&[target.to_str().unwrap(), "--model-checks"]);
    assert_eq!(code, 3, "{out}");
    assert_eq!((out["status"].as_str(), out["entry_error"]["stage"].as_str()), (Some("entry_error"), Some("parse")));
    assert!(out["entry_error"]["message"].as_str().unwrap().starts_with("MODEL-CHECKS-ARGS-INVALID: "));
    std::fs::remove_file(target).unwrap();
}

#[test]
fn the_build_identity_is_printed_with_a_three_state_dirty_flag() {
    let output = Command::new(BIN).arg("--build-identity").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    let identity: Value = serde_json::from_str(String::from_utf8(output.stdout).unwrap().trim()).unwrap();
    for key in ["source_commit", "source_tree", "source_dirty", "build_profile", "rustc", "cargo"] {
        assert!(identity.get(key).is_some(), "{key}: {identity}");
    }
    let dirty = &identity["source_dirty"];
    assert!(dirty.is_boolean() || dirty == "unknown", "true, false or \"unknown\": {dirty}");
    if identity["source_commit"] == "unknown" {
        assert_eq!(dirty, "unknown", "no commit: the flag cannot be false");
    }
}
