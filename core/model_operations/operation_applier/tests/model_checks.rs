//! The desktop engine's on-demand model checks (DEL-04-06 Design
//! `on-demand-model-checks.md`): the report carries the solve's own
//! pre-formation diagnostics for the model, as the desktop's solve request
//! carries it, and the solve's refusal text when the model cannot be read.
//! Models are invented.
use open_pipe_stress_operation_applier::{model_checks, MODEL_CHECKS_DOCUMENT_KIND};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const BUNDLED_DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");

fn solve_diagnostics(model: &Value) -> Vec<Value> {
    let envelope = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), PreviewSolverMode::SparseInteractive).unwrap();
    serde_json::to_value(envelope).unwrap()["diagnostics"].as_array().cloned().unwrap()
}

#[test]
fn the_report_carries_the_solves_leading_diagnostics_and_leaves_the_model_alone() {
    let mut model: Value = serde_json::from_str(BUNDLED_DEMO).unwrap();
    // A second, anchored line that no element joins to the first: one topology warning.
    let nodes = model["nodes"].as_array_mut().unwrap();
    nodes.push(json!({"id":"node:checks-c","position":{"x":40.0,"y":40.0,"z":0.0},"provenance":"checks_invented"}));
    nodes.push(json!({"id":"node:checks-d","position":{"x":41.0,"y":40.0,"z":0.0},"provenance":"checks_invented"}));
    let mut second = model["pipe_segments"][0].clone();
    second["id"] = json!("pipe:checks-c-d");
    second["from"] = json!("node:checks-c");
    second["to"] = json!("node:checks-d");
    model["pipe_segments"].as_array_mut().unwrap().push(second);
    let mut anchor = model["supports"][0].clone();
    anchor["id"] = json!("support:checks-c");
    anchor["node"] = json!("node:checks-c");
    model["supports"].as_array_mut().unwrap().push(anchor);
    let before = model.to_string();

    let report = model_checks(&model);
    assert_eq!(model.to_string(), before, "the checks leave the model unchanged");
    assert_eq!(report["document_kind"], MODEL_CHECKS_DOCUMENT_KIND);
    assert_eq!(report["checked"], true);
    let findings = report["diagnostics"].as_array().unwrap();
    assert!(findings.iter().any(|d| d["code"] == "MODEL_TOPOLOGY_DISCONNECTED" && d["severity"] == "warning"), "{findings:#?}");
    let solved = solve_diagnostics(&model);
    assert_eq!(&solved[..findings.len()], &findings[..], "the findings are the solve's leading diagnostics");
}

#[test]
fn a_model_the_solve_cannot_read_is_reported_unchecked_with_its_refusal() {
    let mut model: Value = serde_json::from_str(BUNDLED_DEMO).unwrap();
    model["nodes"][0]["undeclared_member"] = json!(true);
    let report = model_checks(&model);
    assert_eq!(report["checked"], false);
    let solve = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), PreviewSolverMode::SparseInteractive).err().unwrap();
    assert_eq!(report["refusal"], json!(solve));
    assert!(report.get("diagnostics").is_none());
}

// Codex review r1 of #1665: the report is bounded whatever the model, and the
// browser engine refuses a model text above its bound before parsing it.
#[test]
fn the_report_is_bounded_whatever_the_model() {
    use open_pipe_stress_operation_applier::{
        model_checks_text, MODEL_CHECKS_INPUT_TOO_LARGE, MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES, MODEL_CHECKS_MAX_REFS,
        MODEL_CHECKS_MAX_RESPONSE_BYTES, MODEL_CHECKS_MAX_TEXT_BYTES,
    };
    // 4,000 supports with long ids and an unknown family: one blocking finding each,
    // whose references quote the long id (about 11.3 MB of findings unbounded).
    let mut model: Value = serde_json::from_str(BUNDLED_DEMO).unwrap();
    let node = model["nodes"][0]["id"].clone();
    model["supports"] = Value::Array((0..4_000).map(|i| json!({"id": format!("support:review-{i}-{}", "é".repeat(400)), "node": node, "family": "review_unknown", "restraints": ["UX"], "provenance": "review_invented"})).collect());
    let report = model_checks(&model);
    let text = report.to_string();
    assert!(text.len() <= MODEL_CHECKS_MAX_RESPONSE_BYTES + 512, "report {} bytes", text.len());
    for finding in report["diagnostics"].as_array().unwrap() {
        for key in ["id", "code", "severity", "message", "source"] {
            assert!(finding[key].as_str().map_or(true, |text| text.len() <= MODEL_CHECKS_MAX_TEXT_BYTES), "{key}");
        }
        assert!(finding["affected_refs"].as_array().map_or(true, |refs| refs.len() <= MODEL_CHECKS_MAX_REFS));
    }
    let shown = report["diagnostics"].as_array().unwrap().len() as u64;
    let solved = solve_diagnostics(&model);
    assert_eq!(shown + report["diagnostics_not_shown"].as_u64().unwrap(), solved.len() as u64, "every finding is shown or counted");
    let unbounded = serde_json::to_string(&solved).unwrap().len();
    eprintln!("bounded report {} bytes, {shown} shown, {} not shown; unbounded findings {unbounded} bytes", text.len(), report["diagnostics_not_shown"]);
    assert!(unbounded > MODEL_CHECKS_MAX_RESPONSE_BYTES, "the probe exceeds the bound unbounded");
    assert!(report["diagnostics_not_shown"].as_u64().unwrap() > 0);

    // A long text the solve quotes in its refusal is cut on a character boundary.
    let long = json!("révïew".repeat(200_000));
    let report = model_checks(&long);
    assert_eq!(report["checked"], false);
    assert!(report["refusal"].as_str().unwrap().len() <= MODEL_CHECKS_MAX_TEXT_BYTES);

    // The browser bound: refused by name before any parse, at one byte over.
    let over = format!("{}{}", " ".repeat(MODEL_CHECKS_MAX_BROWSER_INPUT_BYTES), "{}");
    let report = model_checks_text(&over).unwrap();
    assert_eq!((report["checked"].clone(), report["refusal_code"].clone()), (json!(false), json!(MODEL_CHECKS_INPUT_TOO_LARGE)));
    assert!(model_checks_text(BUNDLED_DEMO).unwrap()["checked"] == true);
    assert!(model_checks_text("{").is_err());
}
