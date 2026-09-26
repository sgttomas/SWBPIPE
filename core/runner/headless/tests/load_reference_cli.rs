//! T1 WP4: the 0.4.0 route through the actual `openpipestress-runner solve`
//! process (stdin JSON in, the `ControlledExport`-wrapped CLI output on stdout),
//! in both solver modes. Tests only; no native UI qualification is claimed.
//! When `HEADLESS_LOAD_REFERENCE_OUTPUT_DIR` / `HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR`
//! is set, the actual CLI input and stdout are written for the Python consumer.
//! All inputs are invented, committed fixtures or declared transforms of them.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, PrivacyContext, ProfessionalBoundary, Provenance,
    RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::PreviewSolverMode;
use open_pipe_stress_result_export::source_blocks;
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const LR_ID: &str = "openpipestress.result_semantics/0.3.0/load-reference-1";
const LRS_ID: &str = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
const PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/physics-1";
const PREVIEW_PHYSICS_ID: &str = "openpipestress.result_semantics/0.3.0/preview-physics-1";
const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive,
    PreviewSolverMode::DenseScrutiny,
];

fn project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn read(path: &str) -> Value {
    serde_json::from_slice(&std::fs::read(project().join(path)).unwrap()).unwrap()
}

fn lr_request(stem: &str) -> Value {
    read(&format!(
        "fixtures/product_preview/load_reference/{stem}.request.json"
    ))
}

fn lrs_request(stem: &str) -> Value {
    read(&format!(
        "fixtures/product_preview/load_reference_source/{stem}.request.json"
    ))
}

fn metadata(payload: &Value) -> RunnerRequest {
    let model = &payload["model"];
    let id = model["project"]["id"].as_str().unwrap();
    RunnerRequest {
        request_id: "t1-wp4-load-reference-cli".into(),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "solve"),
        project_ref: Reference::new("project", id),
        model_ref: Reference::new("model", id),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap()))
            .collect(),
        input_manifest_ref: Reference::new("audit_manifest", "t1-wp4-cli-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented T1 WP4 CLI fixture".into(),
            source_location: "core/runner/headless/tests/load_reference_cli.rs".into(),
            source_license: "project invented".into(),
            contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering example".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "pending".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

fn cli_input(payload: &Value) -> String {
    json!({"request": metadata(payload), "solve": {"preview_model": payload}}).to_string()
}

struct Run {
    code: i32,
    stdout: String,
    controlled: Value,
}

fn run_cli(args: &[&str], input: &str) -> Run {
    let mut child = Command::new(env!("CARGO_BIN_EXE_openpipestress-runner"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(out.stdout).unwrap();
    let controlled: Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!(
            "{args:?}: stdout is not JSON: {e}; stderr={}",
            String::from_utf8_lossy(&out.stderr)
        )
    });
    Run {
        code: out.status.code().unwrap(),
        stdout,
        controlled,
    }
}

fn solve_cli(payload: &Value, mode: PreviewSolverMode) -> Run {
    run_cli(
        &[
            "solve",
            "--explicit-local-private-intent",
            "--solver-mode",
            mode.as_str(),
        ],
        &cli_input(payload),
    )
}

/// The ControlledExport wrapper around one CLI output: nothing withheld.
fn payload_of<'a>(run: &'a Run, ctx: &str) -> &'a Value {
    let c = &run.controlled;
    assert_eq!(c["blocked"], false, "{ctx}: {:?}", c["findings"]);
    assert_eq!(c["summary"]["blocking_count"], 0, "{ctx}");
    assert_eq!(c["summary"]["cloud_transmission_attempted"], false, "{ctx}");
    assert_eq!(c["summary"]["professional_claims_made"], false, "{ctx}");
    let p = &c["payload"];
    assert_eq!(
        p["artifact"], "openpipestress.headless_runner_cli_output",
        "{ctx}"
    );
    assert_eq!(p["command"], "solve", "{ctx}");
    assert_eq!(p["operation"], "solve", "{ctx}");
    for key in [
        "result_envelope_document",
        "qualified_preview_evidence",
        "canonical_export_unavailability",
        "actual_invocation",
    ] {
        assert!(
            p.get(key).is_none(),
            "{ctx}: {key} leaked onto the CLI surface"
        );
        assert!(p["mechanics_envelope"].get(key).is_none(), "{ctx}: {key}");
    }
    p
}

fn blocking_codes(raw: &Value) -> Vec<String> {
    raw["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"] == "blocking")
        .map(|d| d["code"].as_str().unwrap().to_string())
        .collect()
}

fn has_blocking(validation: &Value) -> bool {
    validation["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["severity"] == "blocking")
}

fn write_cli_artifacts(var: &str, name: &str, input: &str, run: &Run) {
    let Ok(dir) = std::env::var(var) else { return };
    let dir = Path::new(&dir);
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join(format!("cli-{name}.input.json")), input).unwrap();
    std::fs::write(dir.join(format!("cli-{name}.output.json")), &run.stdout).unwrap();
}

/// A solved CLI run: clean exit and the library's own raw and runner result.
fn assert_solved_matches_library(
    payload: &Value,
    mode: PreviewSolverMode,
    run: &Run,
    ctx: &str,
) -> Value {
    assert_eq!(
        run.code,
        0,
        "{ctx}: {}",
        run.stdout.chars().take(2000).collect::<String>()
    );
    let p = payload_of(run, ctx);
    assert!(p["diagnostics"].as_array().unwrap().is_empty(), "{ctx}");
    assert!(!has_blocking(&p["request_validation"]), "{ctx}");
    assert!(!has_blocking(&p["result_validation"]), "{ctx}");
    let library =
        run_preview_model_value_with_mode(metadata(payload), payload.clone(), mode).unwrap();
    let raw = serde_json::to_value(library.mechanics_envelope.as_ref().unwrap()).unwrap();
    assert_eq!(
        p["mechanics_envelope"], raw,
        "{ctx}: CLI raw is the library raw"
    );
    assert_eq!(
        p["runner_result"],
        serde_json::to_value(&library.runner_result).unwrap(),
        "{ctx}"
    );
    assert_eq!(p["runner_result"]["job"]["state"], "COMPLETED", "{ctx}");
    assert!(
        p["runner_result"]["analysis_status"]
            .as_array()
            .unwrap()
            .contains(&json!("MECHANICS_SOLVED")),
        "{ctx}"
    );
    raw
}

/// A refused or blocked CLI run: exit 1 and nothing published as solved.
fn assert_blocked_run(run: &Run, ctx: &str) -> Value {
    assert_eq!(run.code, 1, "{ctx}");
    let p = payload_of(run, ctx);
    let raw = p["mechanics_envelope"].clone();
    assert_eq!(raw["status"]["mechanics"], "MODEL_INCOMPLETE", "{ctx}");
    assert!(raw["results"].as_array().unwrap().is_empty(), "{ctx}");
    assert!(raw.get("source_block_recovery").is_none(), "{ctx}");
    assert!(has_blocking(&p["result_validation"]), "{ctx}");
    let statuses = p["runner_result"]["analysis_status"].as_array().unwrap();
    assert!(!statuses.contains(&json!("MECHANICS_SOLVED")), "{ctx}");
    assert!(statuses.contains(&json!("MODEL_INCOMPLETE")), "{ctx}");
    raw
}

#[test]
fn cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route() {
    for stem in ["connected", "pressure"] {
        let payload = lr_request(stem);
        for mode in MODES {
            let ctx = format!("cli {stem}/{}", mode.as_str());
            let input = cli_input(&payload);
            let run = solve_cli(&payload, mode);
            let raw = assert_solved_matches_library(&payload, mode, &run, &ctx);
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert!(raw.get("source_block_recovery").is_none(), "{ctx}");
            assert_eq!(raw["numerical_quality"]["status"], "checks_passed", "{ctx}");
            let frozen = read(&format!(
                "fixtures/product_preview/load_reference/{stem}-{}.raw.json",
                mode.as_str()
            ));
            assert_eq!(raw, frozen, "{ctx}");
            if mode == PreviewSolverMode::SparseInteractive {
                let default = run_cli(&["solve", "--explicit-local-private-intent"], &input);
                assert_eq!(default.code, run.code, "{ctx}");
                assert_eq!(
                    default.stdout, run.stdout,
                    "{ctx}: sparse is the default mode"
                );
            }
            write_cli_artifacts(
                "HEADLESS_LOAD_REFERENCE_OUTPUT_DIR",
                &format!("{stem}-{}", mode.as_str()),
                &input,
                &run,
            );
        }
    }
}

#[test]
fn cli_explicit_output_path_receives_the_controlled_stdout() {
    let payload = lr_request("connected");
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("t1-wp4-load-reference-cli-output.json");
    let _ = std::fs::remove_file(&path);
    let run = run_cli(
        &[
            "solve",
            "--explicit-local-private-intent",
            "--solver-mode",
            "dense_scrutiny",
            "--output",
            path.to_str().unwrap(),
        ],
        &cli_input(&payload),
    );
    assert_eq!(run.code, 0);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), run.stdout);
    std::fs::remove_file(&path).unwrap();
}

/// Without the wrapper-owned local-private intent, the ControlledExport wrapper
/// withholds the whole solved 0.4.0 output (existing DEC-065 policy, not T1's):
/// no payload on stdout, exit 1, and no file at an explicit output path.
#[test]
fn cli_without_local_private_intent_withholds_the_solved_output() {
    for (label, payload) in [
        ("load-reference-1", lr_request("connected")),
        ("joined", lrs_request("n05")),
    ] {
        for mode in MODES {
            let ctx = format!("{label}/{}", mode.as_str());
            let path = Path::new(env!("CARGO_TARGET_TMPDIR"))
                .join(format!("t1-wp4-withheld-{label}-{}.json", mode.as_str()));
            let _ = std::fs::remove_file(&path);
            let run = run_cli(
                &[
                    "solve",
                    "--solver-mode",
                    mode.as_str(),
                    "--output",
                    path.to_str().unwrap(),
                ],
                &cli_input(&payload),
            );
            assert_eq!(run.code, 1, "{ctx}");
            let c = &run.controlled;
            assert_eq!(c["blocked"], true, "{ctx}");
            assert!(c["payload"].is_null(), "{ctx}");
            assert!(
                c["summary"]["blocking_count"].as_u64().unwrap() >= 1,
                "{ctx}"
            );
            assert!(
                c["findings"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|f| f["code"] == "LOCAL_PRIVATE_INTENT_REQUIRED"
                        && f["action"] == "block_export"),
                "{ctx}"
            );
            // Findings name paths only; no withheld value reaches stdout.
            let library =
                run_preview_model_value_with_mode(metadata(&payload), payload.clone(), mode)
                    .unwrap();
            let raw = serde_json::to_value(library.mechanics_envelope.as_ref().unwrap()).unwrap();
            let run_id = raw["run_id"].as_str().unwrap();
            assert!(
                !run.stdout.contains(run_id),
                "{ctx}: withheld output leaked"
            );
            if let Some(receipt) = raw["source_block_recovery"]["receipt_sha256"].as_str() {
                assert!(
                    !run.stdout.contains(receipt),
                    "{ctx}: withheld receipt leaked"
                );
            }
            assert!(!path.exists(), "{ctx}: a withheld output was written");
        }
    }
}

#[test]
fn cli_joined_witnesses_both_modes_retain_the_invocation_bound_receipt() {
    for stem in ["n05", "n06", "fields", "mixed", "eigen_motion"] {
        let payload = lrs_request(stem);
        for mode in MODES {
            let ctx = format!("cli joined {stem}/{}", mode.as_str());
            let input = cli_input(&payload);
            let run = solve_cli(&payload, mode);
            let raw = assert_solved_matches_library(&payload, mode, &run, &ctx);
            assert_eq!(raw["producer"]["semantic_contract_id"], LRS_ID, "{ctx}");
            let frozen = read(&format!(
                "fixtures/product_preview/load_reference_source/{stem}-{}.raw.json",
                mode.as_str()
            ));
            assert_eq!(raw, frozen, "{ctx}");
            let actual = json!({"request": payload, "solver_mode": mode.as_str()});
            assert_eq!(
                raw["source_block_recovery"]["body"]["invocation"]["value"],
                source_blocks::domain_hash("source_blocks_invocation_v1", &actual).unwrap(),
                "{ctx}"
            );
            write_cli_artifacts(
                "HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR",
                &format!("{stem}-{}", mode.as_str()),
                &input,
                &run,
            );
        }
    }
}

#[test]
fn cli_refusals_have_targeted_codes_and_publish_nothing_as_solved() {
    let mut explicit_null = lr_request("connected");
    explicit_null["model"]["materials"][0]["expansion_laws"] = Value::Null;
    let mut unknown = lr_request("connected");
    unknown["model"]["load_cases"][0]["analysis_state"]["unreviewed_field"] = json!(true);
    let mut pre_exact = lr_request("connected");
    pre_exact["model"]["schema_version"] = json!("0.3.0");
    let mut pre_ordinary = pre_exact.clone();
    pre_ordinary["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    for mode in MODES {
        let m = mode.as_str();
        let raw = assert_blocked_run(
            &solve_cli(&explicit_null, mode),
            &format!("explicit null/{m}"),
        );
        assert!(
            blocking_codes(&raw).contains(&"LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED".to_string()),
            "{m}"
        );
        assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{m}");

        // An unknown field never reaches the solver: the typed input gate refuses it.
        let run = solve_cli(&unknown, mode);
        assert_eq!(run.code, 1, "unknown/{m}");
        let p = payload_of(&run, &format!("unknown/{m}"));
        assert!(
            p["runner_result"].is_null() && p["mechanics_envelope"].is_null(),
            "unknown/{m}"
        );
        let diagnostics = p["diagnostics"].as_array().unwrap();
        assert_eq!(diagnostics.len(), 1, "unknown/{m}: {diagnostics:?}");
        assert_eq!(
            diagnostics[0]["code"], "HEADLESS_RUNNER_SOLVE_PAYLOAD_INVALID",
            "unknown/{m}"
        );
        assert_eq!(diagnostics[0]["severity"], "blocking", "unknown/{m}");
        assert!(
            diagnostics[0]["message"]
                .as_str()
                .unwrap()
                .contains("unknown field `unreviewed_field`"),
            "unknown/{m}"
        );

        for (label, payload, identity) in [
            ("exact", &pre_exact, PHYSICS_ID),
            ("ordinary", &pre_ordinary, PREVIEW_PHYSICS_ID),
        ] {
            let raw = assert_blocked_run(&solve_cli(payload, mode), &format!("0.3.0 {label}/{m}"));
            assert!(
                blocking_codes(&raw).contains(&"LOAD_STATE_CONTRACT_VERSION_MISMATCH".to_string()),
                "{label}/{m}"
            );
            assert_eq!(
                raw["producer"]["semantic_contract_id"], identity,
                "{label}/{m}"
            );
        }
    }
}

#[test]
fn cli_blocked_0_4_0_documents_stay_on_load_reference_one() {
    let mut missing = lr_request("connected");
    missing["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    let mut legacy = lr_request("connected");
    legacy["model"]["pressure_contract"] = json!({"version": "1.0.0", "mode": "legacy"});
    for (label, payload) in [("missing", missing), ("legacy", legacy)] {
        for mode in MODES {
            let ctx = format!("blocked {label}/{}", mode.as_str());
            let raw = assert_blocked_run(&solve_cli(&payload, mode), &ctx);
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert_ne!(
                raw["producer"]["semantic_contract_id"], PREVIEW_PHYSICS_ID,
                "{ctx}"
            );
            assert_eq!(
                raw["contract_evidence"],
                json!({"pressure": [], "connector": [], "exact_cases": [], "load_reference_states": []}),
                "{ctx}"
            );
        }
    }
}

/// The CP3 reviewer's budget-cliff probe P9 (see
/// `core/product_physics/src/source_receipt/load_state_fallback_tests.rs`).
fn with_extra_loads(mut request: Value, extra: usize) -> Value {
    let case = &mut request["model"]["load_cases"][0];
    for (i, dir) in ["UZ", "UY", "RY"].iter().take(extra).enumerate() {
        let mut load = case["primitive_loads"][0].clone();
        let id = format!("extra:{i}");
        load["id"] = json!(id);
        load["direction"] = json!(dir);
        if !dir.starts_with('R') {
            load["category"] = json!("concentrated_force");
            load["dimension"] = json!("force");
            load["magnitude"] = json!({"value": 1.0e-6, "unit": "N"});
        }
        case["primitive_loads"].as_array_mut().unwrap().push(load);
        case["analysis_state"]["load_sources"]
            .as_array_mut()
            .unwrap()
            .push(json!({"source_ref": id, "factor": 1.0}));
    }
    request
}

#[test]
fn cli_sf1_budget_cliff_publishes_the_ordinary_route_with_its_diagnostic() {
    for extra in 1..=3 {
        let payload = with_extra_loads(lrs_request("eigen_motion"), extra);
        for mode in MODES {
            let ctx = format!("cli SF-1 extra={extra}/{}", mode.as_str());
            let run = solve_cli(&payload, mode);
            let raw = assert_solved_matches_library(&payload, mode, &run, &ctx);
            assert_eq!(raw["producer"]["semantic_contract_id"], LR_ID, "{ctx}");
            assert!(raw.get("source_block_recovery").is_none(), "{ctx}");
            assert!(blocking_codes(&raw).is_empty(), "{ctx}");
            let unavailable: Vec<&Value> = raw["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|d| d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")
                .collect();
            assert_eq!(unavailable.len(), 1, "{ctx}");
            assert!(
                unavailable[0]["message"]
                    .as_str()
                    .unwrap()
                    .contains("captured replay reservation"),
                "{ctx}"
            );
        }
    }
}
