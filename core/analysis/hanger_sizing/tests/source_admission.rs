//! The source model's admission is not hidden by projection (DEL-04-11 REQ-003,
//! REQ-009). A derived document keeps one case and no combinations, so what the
//! model's own solve refuses in the complete source must refuse the request with
//! the solve's code before any derived solve.
//!
//! - Every committed sized input is a complete source that an ordinary solve of the
//!   product admits, with every case and combination it carries.
//! - The authored pressure-elongation setting is judged on the complete source by
//!   the product's own `pressure_elongation_refusals`: an empty, duplicated or
//!   unknown selection, a selection on a document without a pressure contract and a
//!   malformed setting each refuse the request through the real sizing entry, and an
//!   ordinary solve of the same source refuses it with the same code.
//! - A valid selection is admitted, and each derived document keeps it only for a
//!   case it selects; unselected cases carry no setting, and other settings are kept.
//!
//! These are admission and document checks. They set no numerical expectation: the
//! numerical evidence with pressure elongation selected is EXS-A's, in
//! `ex_references.rs`, for its pressure-independent vertical quantities only.

#[path = "common/model_builder.rs"]
mod model_builder;

use open_pipe_stress_hanger_sizing::derived::{derived_document, Role, Site};
use open_pipe_stress_hanger_sizing::publish::{checked_sha256, size_hangers};
use open_pipe_stress_hanger_sizing::Dof;
use open_pipe_stress_product_physics::model_settings::{
    MODEL_SETTING_PRESSURE_ELONGATION_CASES_INVALID, MODEL_SETTING_PRESSURE_ELONGATION_CASE_UNKNOWN,
    MODEL_SETTING_PRESSURE_ELONGATION_WITHOUT_PRESSURE_CONTRACT,
};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use std::path::PathBuf;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];

fn results_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/results")
}

fn read_json(name: &str) -> Value {
    let path = results_dir().join(name);
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{name}: {e}"))).unwrap()
}

/// The ordinary solve's blocking codes for a complete solve request, or its refusal text.
fn ordinary_blocking(request: &Value, mode: PreviewSolverMode) -> Result<Vec<String>, String> {
    let envelope = run_linear_static_preview_value_with_mode(request.clone(), mode)?;
    Ok(envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.clone()).collect())
}

/// Every committed input whose committed result is sized.
fn sized_inputs() -> Vec<(String, Value)> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(results_dir()).unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let Some(stem) = name.strip_prefix("hanger_sizing_input_").and_then(|n| n.strip_suffix(".json")) else { continue };
        let result = format!("hanger_sizing_result_{stem}.json");
        if results_dir().join(&result).exists() && read_json(&result)["status"] == "sized" {
            found.push((stem.to_string(), read_json(&name)));
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    found
}

#[test]
fn every_committed_sized_input_is_a_complete_source_the_ordinary_solve_admits() {
    let inputs = sized_inputs();
    let names: Vec<&str> = inputs.iter().map(|(n, _)| n.as_str()).collect();
    // Both EX sources are among them.
    for ex in ["ex_pressurised", "ex_riser"] {
        assert!(names.contains(&ex), "{names:?}");
    }
    for (name, input) in &inputs {
        let request = &input["request"];
        assert!(request["model"]["load_cases"].as_array().unwrap().len() >= 2, "{name}: not a complete source");
        for mode in MODES {
            let blocking = ordinary_blocking(request, mode).unwrap_or_else(|e| panic!("{name} {mode:?}: refused: {e}"));
            assert!(blocking.is_empty(), "{name} {mode:?}: the complete source is not admitted: {blocking:?}");
        }
    }
}

fn with_setting(mut input: Value, setting: Value) -> Value {
    input["request"]["model"]["model_settings"] = json!({"pressure_elongation": setting});
    input
}

/// The real entry refuses the request naming the source, with the product's code,
/// before any derived solve; an ordinary solve of the same complete source refuses
/// it with that code too.
fn assert_source_refused(input: &Value, code: &str, what: &str) {
    let sized = size_hangers(&serde_json::to_string(input).unwrap());
    let document = sized.document();
    assert!(!sized.is_sized(), "{what}: sized: {document}");
    let refusals = document["refusals"].as_array().unwrap();
    assert_eq!(refusals.len(), 1, "{what}: {refusals:?}");
    assert_eq!(refusals[0]["code"], "HANGER_SIZING_DERIVED_SOLVE_FAILED", "{what}");
    assert_eq!(refusals[0]["names"], json!(["source"]), "{what}");
    assert!(refusals[0]["message"].as_str().unwrap().contains(code), "{what}: {}", refusals[0]["message"]);
    match ordinary_blocking(&input["request"], PreviewSolverMode::DenseScrutiny) {
        Ok(blocking) => assert!(blocking.iter().any(|c| c == code), "{what}: the ordinary solve does not refuse with {code}: {blocking:?}"),
        Err(text) => assert!(text.contains(code), "{what}: {text}"),
    }
}

#[test]
fn an_invalid_pressure_elongation_selection_refuses_the_request_with_the_solves_code() {
    let ex = read_json("hanger_sizing_input_ex_pressurised.json");
    let design = ex["sizing"]["design_operating_case"].as_str().unwrap().to_string();
    assert_source_refused(&with_setting(ex.clone(), json!({"operating_load_cases": []})),
        MODEL_SETTING_PRESSURE_ELONGATION_CASES_INVALID, "empty");
    assert_source_refused(&with_setting(ex.clone(), json!({"operating_load_cases": [design, design]})),
        MODEL_SETTING_PRESSURE_ELONGATION_CASES_INVALID, "duplicated");
    assert_source_refused(&with_setting(ex.clone(), json!({"operating_load_cases": [design, "no-such-case"]})),
        MODEL_SETTING_PRESSURE_ELONGATION_CASE_UNKNOWN, "unknown");
    // A malformed setting is refused by the solve's own request parse.
    let malformed = with_setting(ex.clone(), json!({"operating_load_cases": design}));
    let sized = size_hangers(&serde_json::to_string(&malformed).unwrap());
    assert!(!sized.is_sized());
    assert_eq!(sized.document()["refusals"][0]["names"], json!(["source"]), "{}", sized.document());
    assert!(run_linear_static_preview_value_with_mode(malformed["request"].clone(), PreviewSolverMode::DenseScrutiny).is_err());
    // Without a pressure contract.
    let pf = read_json("hanger_sizing_input_two_span.json");
    assert!(pf["request"]["model"].get("pressure_contract").is_none());
    let case = pf["sizing"]["design_operating_case"].clone();
    assert_source_refused(&with_setting(pf, json!({"operating_load_cases": [case]})),
        MODEL_SETTING_PRESSURE_ELONGATION_WITHOUT_PRESSURE_CONTRACT, "without a pressure contract");
}

#[test]
fn a_valid_selection_is_kept_only_in_the_derived_documents_of_the_cases_it_selects() {
    let ex = read_json("hanger_sizing_input_ex_pressurised.json");
    let cases: Vec<String> =
        ex["request"]["model"]["load_cases"].as_array().unwrap().iter().map(|c| c["id"].as_str().unwrap().to_string()).collect();
    let sites: Vec<Site> = ex["sizing"]["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            let id = c["support"].as_str().unwrap();
            let support = ex["request"]["model"]["supports"].as_array().unwrap().iter().find(|s| s["id"] == id).unwrap();
            Site { support: id.into(), node: support["node"].as_str().unwrap().into(), dof: Dof::Uz }
        })
        .collect();
    let design = ex["sizing"]["design_operating_case"].as_str().unwrap();
    let check = ex["sizing"]["check_operating_cases"][0].as_str().unwrap();
    let tolerance = json!({"value": 0.002, "unit": "m"});
    for selected in [vec![design], vec![check], vec![design, check]] {
        let mut input = ex.clone();
        input["request"]["model"]["model_settings"] = json!({"node_proximity_tolerance": tolerance,
            "pressure_elongation": {"operating_load_cases": selected}});
        let request = &input["request"];
        for case in &cases {
            for roles in [vec![Role::Rigid; sites.len()], vec![Role::Force(1000.0); sites.len()]] {
                let projected = derived_document(request, case, &sites, &roles);
                let settings = &projected["model"]["model_settings"];
                assert_eq!(settings["node_proximity_tolerance"], tolerance, "{selected:?} {case}: an unrelated setting was not kept");
                if selected.contains(&case.as_str()) {
                    assert_eq!(settings["pressure_elongation"], json!({"operating_load_cases": [case]}), "{selected:?} {case}");
                } else {
                    assert!(settings.get("pressure_elongation").is_none(), "{selected:?} {case}: {settings}");
                }
            }
        }
        // The real entry admits it, runs 2n + 3 solves and binds the complete
        // source as given, the setting included.
        let sized = size_hangers(&serde_json::to_string(&input).unwrap());
        let document = sized.document();
        assert!(sized.is_sized(), "{selected:?}: {document}");
        let n = 1 + ex["sizing"]["check_operating_cases"].as_array().unwrap().len();
        assert_eq!(document["result"]["solves"].as_array().unwrap().len(), 2 * n + 3, "{selected:?}");
        assert_eq!(document["inputs"]["solve_request"]["sha256"].as_str(), checked_sha256(request).as_deref(), "{selected:?}");
        // The stored model is unchanged by the run.
        assert_eq!(input["request"]["model"]["model_settings"]["pressure_elongation"]["operating_load_cases"], json!(selected));
    }
}
