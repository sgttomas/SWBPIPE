//! Operating-case slice 1a (DEL-05-02 Design `pf-nonlinear-operand-standing.md` §3):
//! the preview-physics-2 reader's `combination_operands` checks over the shared
//! corpus (`fixtures/results/preview_physics_nonlinear_operands_corpus.json`,
//! written by PP `op1a_operand_standing.rs`). The actual producer documents are
//! admitted with and without their models; every forgery is an in-memory copy and
//! is refused, by the evidence pass or by the model binding.
use open_pipe_stress_result_export::{reader_verdict, semantic_contract as s};
use serde_json::{json, Value};

const CORPUS: &str = include_str!("../../../../fixtures/results/preview_physics_nonlinear_operands_corpus.json");

fn entry(name: &str) -> (Value, Value) {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let e = corpus["documents"].as_array().unwrap().iter().find(|e| e["name"] == name).unwrap().clone();
    (e["document"].clone(), e["model"].clone())
}
fn refused(source: &Value, code: &str, what: &str) {
    assert_eq!(s::validate_preview_physics_evidence(source), Err(format!("SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_{code}")), "{what}");
}
fn with_model(source: &Value, model: &Value) -> Value {
    reader_verdict::validate("preview-physics", source, Some(model), None)
}
fn diagnostics(v: &mut Value) -> &mut Vec<Value> {
    v["diagnostics"].as_array_mut().unwrap()
}
fn position(v: &Value, code: &str) -> usize {
    v["diagnostics"].as_array().unwrap().iter().position(|d| d["code"] == code).unwrap()
}

#[test]
fn producer_documents_are_admitted_with_and_without_their_models() {
    for name in ["v1_gap_closes", "v3_friction"] {
        let (source, model) = entry(name);
        assert_eq!(s::validate_preview_physics_evidence(&source), Ok(()), "{name}");
        assert_eq!(with_model(&source, &model)["ok"], true, "{name}: {}", with_model(&source, &model));
    }
    // The corpus covers both notices and their absence.
    let (v1, _) = entry("v1_gap_closes");
    assert_eq!(v1["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"].as_str().unwrap().starts_with("COMBINATION_OPERAND_")).count(), 4);
    let (v3, _) = entry("v3_friction");
    assert!(v3["diagnostics"].as_array().unwrap().iter().all(|d| !d["code"].as_str().unwrap().starts_with("COMBINATION_OPERAND_")));
}

#[test]
fn forged_standing_and_records_are_refused() {
    let (v1, _) = entry("v1_gap_closes");
    let (v3, _) = entry("v3_friction");
    let forge = |base: &Value, edit: &dyn Fn(&mut Value)| {
        let mut v = base.clone();
        edit(&mut v);
        v
    };
    // Standing: a passed copy of a non-passing case, and the other case's record.
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["operands"][0]["numerical_quality"]["solve_quality"] = json!("checks_passed")), "STANDING", "forged passing standing");
    refused(&forge(&v3, &|v| v["contract_evidence"]["combination_operands"][0]["operands"][0]["numerical_quality"]["solve_quality"] = json!("sensitive")), "STANDING", "forged failing standing");
    refused(&forge(&v1, &|v| {
        let other = v["contract_evidence"]["combination_operands"][0]["operands"][1]["numerical_quality"].clone();
        v["contract_evidence"]["combination_operands"][0]["operands"][0]["numerical_quality"] = other;
    }), "STANDING", "another case's record");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["operands"][0]["numerical_quality"] = Value::Null), "STANDING", "null standing");
    // Presence: dropped, emptied, one record removed, a record on a linear-looking document.
    refused(&forge(&v1, &|v| { v["contract_evidence"].as_object_mut().unwrap().remove("combination_operands"); }), "PRESENCE", "member dropped");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"] = json!([])), "PRESENCE", "member emptied");
    refused(&forge(&v1, &|v| { v["contract_evidence"]["combination_operands"].as_array_mut().unwrap().pop(); }), "PRESENCE", "a record removed");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"].as_array_mut().unwrap().reverse()), "PRESENCE", "records out of gate order");
    refused(&forge(&v3, &|v| {
        v["results"].as_array_mut().unwrap().retain(|r| r["kind"] != "nonlinear_support_active_set_state_code");
    }), "PRESENCE", "the member without a nonlinear state row");
    // Shape: an extra key, the standing token, the basis, swapped operands and expression.
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["certified"] = json!(true)), "SHAPE", "extra record key");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["standing"] = json!("established")), "SHAPE", "standing token");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["basis"] = json!("mechanics")), "SHAPE", "basis");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["operands"].as_array_mut().unwrap().reverse()), "SHAPE", "operands swapped");
    refused(&forge(&v1, &|v| v["contract_evidence"]["combination_operands"][0]["operands"][0]["extra"] = json!(1)), "SHAPE", "extra operand key");
    // Swapping the stated minuend and subtrahend contradicts the rows' source order.
    refused(&forge(&v1, &|v| {
        let record = &mut v["contract_evidence"]["combination_operands"][0];
        record["expression"] = json!({"minuend_id": "case:S", "subtrahend_id": "case:O"});
        record["operands"].as_array_mut().unwrap().reverse();
    }), "ROWS", "expression swapped with its operands");
    // On a blocked envelope the member is refused.
    let mut blocked = v3.clone();
    blocked["status"]["mechanics"] = json!("MECHANICS_BLOCKED");
    refused(&blocked, "PRESENCE", "blocked envelope");
}

#[test]
fn forged_rows_are_refused() {
    let (v1, _) = entry("v1_gap_closes");
    let mut v = v1.clone();
    let index = v["results"].as_array().unwrap().iter()
        .position(|r| r["basis_ref"]["ref_id"] == "comb:o-s" && r["kind"] == "global_nodal_displacement_x").unwrap();
    let refs = v["results"][index]["source_result_refs"].as_array_mut().unwrap();
    refs.reverse();
    refused(&v, "ROWS", "source rows out of operand order");
}

#[test]
fn forged_missing_and_extra_notices_are_refused() {
    let (v1, _) = entry("v1_gap_closes");
    let (v3, _) = entry("v3_friction");
    for code in ["COMBINATION_OPERAND_SUPPORT_STATES_DIFFER", "COMBINATION_OPERAND_CHECKS_NOT_PASSED"] {
        let i = position(&v1, code);
        let mut missing = v1.clone();
        diagnostics(&mut missing).remove(i);
        refused(&missing, "NOTICE", &format!("{code} missing"));
        for field in ["id", "severity", "message", "source"] {
            let mut altered = v1.clone();
            diagnostics(&mut altered)[i][field] = json!("forged");
            refused(&altered, "NOTICE", &format!("{code} {field}"));
        }
        let mut refs = v1.clone();
        diagnostics(&mut refs)[i]["affected_refs"].as_array_mut().unwrap().pop();
        refused(&refs, "NOTICE", &format!("{code} affected refs"));
        let mut duplicate = v1.clone();
        let copy = duplicate["diagnostics"][i].clone();
        diagnostics(&mut duplicate).push(copy);
        refused(&duplicate, "NOTICE", &format!("{code} duplicated"));
        // A notice the document's evidence does not support.
        let mut extra = v3.clone();
        let notice = v1["diagnostics"][i].clone();
        diagnostics(&mut extra).push(notice);
        refused(&extra, "NOTICE", &format!("{code} without its condition"));
    }
    // A forged support state: the notice no longer matches the rows.
    let mut state = v1.clone();
    for row in state["results"].as_array_mut().unwrap() {
        if row["kind"] == "nonlinear_support_active_set_state_code" && row["basis_ref"]["ref_id"] == "case:O" {
            row["value"] = json!(0.0);
        }
    }
    refused(&state, "NOTICE", "states made equal");
    let mut code = v1.clone();
    for row in code["results"].as_array_mut().unwrap() {
        if row["kind"] == "nonlinear_support_active_set_state_code" {
            row["value"] = json!(7.0);
        }
    }
    refused(&code, "STATE_CODE", "state code outside 0-3");
}

#[test]
fn the_model_binding_refuses_forged_expressions_and_selections() {
    let (v1, model) = entry("v1_gap_closes");
    let refused_with = |source: &Value, model: &Value, what: &str| {
        let verdict = with_model(source, model);
        assert_eq!(verdict["ok"], false, "{what}");
        assert!(verdict.to_string().contains("COMBINATION_OPERANDS_MODEL"), "{what}: {verdict}");
    };
    let mut m = model.clone();
    m["combinations"][1]["mode"] = json!("min");
    refused_with(&v1, &m, "range mode");
    // A forged selection passes the evidence pass (its shape is valid) and is refused by the binding.
    let mut forged = v1.clone();
    forged["contract_evidence"]["combination_operands"][0]["operands"][0]["modulus_basis_ref"] = json!("point:hot");
    assert_eq!(s::validate_preview_physics_evidence(&forged), Ok(()));
    refused_with(&forged, &model, "modulus selection");
    let mut forged = v1.clone();
    forged["contract_evidence"]["combination_operands"][1]["expression"]["mode"] = json!("min");
    assert_eq!(s::validate_preview_physics_evidence(&forged), Ok(()));
    refused_with(&forged, &model, "stated range mode");
    let mut m = model.clone();
    m["supports"].as_array_mut().unwrap().retain(|s| s.get("nonlinear").is_none());
    refused_with(&v1, &m, "a linear model states no record");
    let mut m = model.clone();
    m["combinations"].as_array_mut().unwrap().remove(1);
    refused_with(&v1, &m, "a record without its combination");
}
