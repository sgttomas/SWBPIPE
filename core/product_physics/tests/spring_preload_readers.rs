//! DEL-04-11 REQ-014: the readers re-derive the variable spring preload from the
//! model and do not take the producer's word for it. On the exact route the
//! reader binds every case's spring applied-load rows to the model's decision,
//! as it binds constant-effort rows. On the preview route the producer states
//! the fixed-load fact `applying_variable_spring_cold_load` beside
//! `consuming_constant_effort`, and the reader compares it with its own
//! re-derivation and recomputes the gates from both facts. Models are the
//! preload reference's P-A (`validation/references/hanger_sizing/preload.json`),
//! invented data.
use open_pipe_stress_product_physics::{result_export::reader_verdict, run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

#[path = "common/spring_preload_model.rs"]
mod model;
use model::*;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const FACT: &str = "applying_variable_spring_cold_load";

fn solved(document: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let envelope = serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap();
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
    envelope
}

/// P-A with PA-WT and PA-B as two cases of one document and the given combinations.
fn pa_two_cases(route: Route, combinations: Value) -> Value {
    let mut wt = document("PA-WT", route, MODEL_SCHEMA_VERSION);
    let b = document("PA-B", route, MODEL_SCHEMA_VERSION);
    wt["model"]["load_cases"].as_array_mut().unwrap().push(b["model"]["load_cases"][0].clone());
    wt["model"]["combinations"] = combinations;
    wt
}

fn verdict(family: &str, envelope: &Value, model: Option<&Value>) -> Value {
    reader_verdict::validate(family, envelope, model, None)
}

fn refused_with(v: &Value, fragment: &str, what: &str) {
    assert_eq!(v["ok"], false, "{what}: {v:#}");
    assert!(v["error"]["code"].as_str().is_some_and(|c| c.contains(fragment)), "{what}: {v:#}");
}

/// The exact reader: a document whose only combination is withheld by the
/// cold load's fixed-load count reads clean with its model; a changed spring
/// row, or a model whose spring applies nothing, is refused by name.
#[test]
fn the_exact_reader_binds_the_spring_rows_to_the_model() {
    let combinations = json!([{"id": "comb:twice", "basis": "mechanics",
        "terms": [{"load_case": "PA-WT", "factor": 1.0}, {"load_case": "PA-B", "factor": 1.0}], "provenance": "invented"}]);
    let document = pa_two_cases(Route::Exact, combinations);
    for mode in MODES {
        let what = format!("exact {mode:?}");
        let envelope = solved(&document, mode, &what);
        let clean = verdict("pressure", &envelope, Some(&document["model"]));
        assert_eq!(clean["ok"], true, "{what}: {clean:#}");
        // A changed applied force in the second case.
        let mut changed = envelope.clone();
        let row = changed["results"].as_array_mut().unwrap().iter_mut()
            .find(|r| r["kind"] == APPLIED && r["basis_ref"]["ref_id"] == "PA-B" && r["entity_ref"] == "SH-3").unwrap();
        row["value"] = json!(row["value"].as_f64().unwrap() + 1.0);
        refused_with(&verdict("pressure", &changed, Some(&document["model"])), "MODEL_SPRING_PRELOAD", &format!("{what}: changed row"));
        // A model in which SH-3 has no cold load: its rows are not the model's.
        let mut model = document["model"].clone();
        let support = model["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "SH-3").unwrap();
        support["hanger"].as_object_mut().unwrap().remove("cold_load");
        refused_with(&verdict("pressure", &envelope, Some(&model)), "MODEL_SPRING_PRELOAD", &format!("{what}: model without the cold load"));
    }
}

/// The preview reader: an absolute sum of the two cases is withheld by the
/// cold load (no constant-effort support in P-A), the derivation states the
/// spring fact, and the document reads clean with and without its model.
/// Removing the fact is refused with the model (it is not the model's
/// derivation) and without it (the stated gate is then not the derived one);
/// stating it for a model whose springs apply nothing is refused.
#[test]
#[ignore = "the preview reader refuses the applied row's kind (SOURCE_PREVIEW_PHYSICS_ROW_SIGNATURE) until preview-physics-2 lists it: stage B1 part 2, held on the reviewed-inputs question"]
fn the_preview_reader_re_derives_the_spring_fact() {
    let combinations = json!([{"id": "comb:abs", "basis": "absolute_sum",
        "terms": [{"load_case": "PA-WT", "factor": 1.0}, {"load_case": "PA-B", "factor": 1.0}], "provenance": "invented"}]);
    let document = pa_two_cases(Route::PressureFree, combinations);
    for mode in MODES {
        let what = format!("preview {mode:?}");
        let envelope = solved(&document, mode, &what);
        let derivation = &envelope["contract_evidence"]["combination_derivation"];
        assert_eq!(derivation[FACT], true, "{what}: {derivation:#}");
        assert_eq!(derivation["consuming_constant_effort"], false, "{what}");
        let gate = envelope["contract_evidence"]["combination_gates"].as_array().unwrap().iter().find(|g| g["combination_id"] == "comb:abs").unwrap();
        assert_eq!(gate["reason"], "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND", "{what}: {gate:#}");
        for model in [Some(&document["model"]), None] {
            let v = verdict("preview-physics", &envelope, model);
            assert_eq!(v["ok"], true, "{what} model={}: {v:#}", model.is_some());
        }
        let mut without = envelope.clone();
        without["contract_evidence"]["combination_derivation"].as_object_mut().unwrap().remove(FACT);
        refused_with(&verdict("preview-physics", &without, Some(&document["model"])), "DERIVATION_EVIDENCE", &format!("{what}: fact removed, with the model"));
        refused_with(&verdict("preview-physics", &without, None), "GATE", &format!("{what}: fact removed, model-free"));
        let mut model = document["model"].clone();
        for support in model["supports"].as_array_mut().unwrap() {
            if let Some(hanger) = support["hanger"].as_object_mut() {
                hanger.remove("cold_load");
            }
        }
        assert_eq!(verdict("preview-physics", &envelope, Some(&model))["ok"], false, "{what}: a model whose springs apply nothing");
    }
}
