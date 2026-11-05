//! RT-A5 `minuend_final` readers over the shared corpus
//! (`fixtures/results/preview_physics_held_status_corpus.json`, written by
//! `core/product_physics/tests/op_mf_held_status.rs`): with the model and without it.
//! Each forgery edits what a reading checks and must be refused by name; a forgery of
//! solved values coherent with its decisions is outside both readings (§7.6).
use super::*;
use crate::reader_verdict;
use serde_json::json;

const CORPUS: &str = include_str!("../../../../fixtures/results/preview_physics_held_status_corpus.json");

fn corpus() -> Vec<(String, Value, Value)> {
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    corpus["documents"].as_array().unwrap().iter().map(|e| (e["name"].as_str().unwrap().to_string(), e["model"].clone(), e["document"].clone())).collect()
}
fn document(name: &str) -> (Value, Value) {
    let (_, model, document) = corpus().into_iter().find(|(n, _, _)| n == name).unwrap_or_else(|| panic!("{name}"));
    (model, document)
}
fn code(verdict: &Value) -> String {
    if verdict["ok"] == true { "ok".into() } else { verdict["error"]["code"].as_str().unwrap_or_default().to_string() }
}
fn read(source: &Value, model: Option<&Value>) -> String {
    code(&reader_verdict::validate("preview-physics", source, model, None))
}
fn record(v: &mut Value) -> &mut Value {
    &mut v["contract_evidence"][MEMBER][0]
}
fn entry(v: &mut Value) -> &mut Value {
    &mut record(v)["held"][0]
}
fn hex(x: f64) -> Value {
    json!(format!("{:016x}", x.to_bits()))
}

#[test]
fn every_corpus_document_reads_with_its_model_and_without_it() {
    let documents = corpus();
    assert!(documents.len() >= 11);
    let mut failures = Vec::new();
    for (name, model, document) in documents {
        let (with, without) = (read(&document, Some(&model)), read(&document, None));
        if with != "ok" || without != "ok" {
            failures.push(format!("{name}: with {with}, without {without}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Both readings refuse by name what the document itself shows forged.
#[test]
fn both_readings_refuse_forged_evidence_by_name() {
    let mut cases: Vec<(&str, &str, Value, &str)> = Vec::new();
    let mut tamper = |what: &'static str, base: &'static str, code: &'static str, edit: &dyn Fn(&mut Value)| {
        let (_, mut forged) = document(base);
        edit(&mut forged);
        cases.push((what, base, forged, code));
    };
    // The operand record binds its held standing to the record, and its rows to the held
    // rows, and is read first: a forgery it sees is refused by its name.
    const OPERANDS_SHAPE: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE";
    const OPERANDS_ROWS: &str = "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_ROWS";
    // Shape.
    tamper("empty member", "h2_released", OPERANDS_SHAPE, &|v| v["contract_evidence"][MEMBER] = json!([]));
    tamper("extra record member", "h2_released", EVIDENCE, &|v| record(v)["note"] = json!("x"));
    tamper("extra entry member", "h2_released", EVIDENCE, &|v| entry(v)["note"] = json!("x"));
    tamper("missing input", "h4_kept_explicit", EVIDENCE, &|v| { entry(v)["inputs"].as_object_mut().unwrap().remove("coefficient_bits"); });
    tamper("upper-case bits", "h2_released", EVIDENCE, &|v| entry(v)["inputs"]["reaction_bits"] = json!(entry(v)["inputs"]["reaction_bits"].as_str().unwrap().to_ascii_uppercase().replace('0', "A")));
    tamper("an unknown state", "h2_released", EVIDENCE, &|v| entry(v)["held_state"] = json!("closed"));
    tamper("a withheld record with held entries", "source_unavailable", EVIDENCE, &|v| record(v)["held"] = json!([{"support_id": "x"}]));
    // Bindings.
    tamper("unknown combination", "h2_released", OPERANDS_SHAPE, &|v| record(v)["combination_id"] = json!("comb:none"));
    tamper("source and target swapped", "h2_released", BINDING, &|v| {
        let r = record(v).clone();
        record(v)["source"]["load_case_id"] = r["target"]["load_case_id"].clone();
        record(v)["target"]["load_case_id"] = r["source"]["load_case_id"].clone();
    });
    tamper("another mode", "h2_released", BINDING, &|v| record(v)["source"]["solver_mode"] = json!("dense_scrutiny"));
    tamper("an outcome the gate does not state", "h2_released", OPERANDS_SHAPE, &|v| record(v)["outcome"] = json!("HELD_STATUS_BINDING"));
    tamper("the minuend's published code against the held state", "h2_released", BINDING, &|v| {
        let code = v["results"].as_array_mut().unwrap().iter_mut()
            .find(|r| r["kind"] == "nonlinear_support_active_set_state_code" && r["basis_ref"]["ref_id"] == "case:O").unwrap();
        code["value"] = json!(0.0);
    });
    tamper("a held entry missing", "h2_released", EVIDENCE, &|v| record(v)["held"] = json!([]));
    tamper("an extra held entry", "h2_released", BINDING, &|v| {
        let mut extra = entry(v).clone();
        extra["support_id"] = json!("support:u");
        record(v)["held"].as_array_mut().unwrap().push(extra);
    });
    tamper("a DOF forged", "h2_released", BINDING, &|v| entry(v)["dof"] = json!("UY"));
    tamper("no held-operand rows", "h2_released", OPERANDS_ROWS, &|v| {
        v["results"].as_array_mut().unwrap().retain(|r| !r["id"].as_str().unwrap().contains(":held:"));
    });
    // Decisions against their inputs.
    tamper("classified state forged", "h2_released", DERIVATION, &|v| entry(v)["classified_state"] = json!("active"));
    tamper("a sense forged", "h2_released", DERIVATION, &|v| entry(v)["sense"] = json!("negative_reaction"));
    tamper("released forged", "h2_released", DERIVATION, &|v| entry(v)["released"] = json!("none"));
    tamper("a stick decision without its inputs", "h4_kept_explicit", DERIVATION, &|v| entry(v)["inputs"]["reaction_bits"] = hex(1.0e6));
    tamper("a branch verdict forged", "h13_branch", DERIVATION, &|v| entry(v)["inputs"]["branch_admissible"] = json!(true));
    tamper("a zero limit read by mu > 0 (H-15c)", "h15c_zero_limit", DERIVATION, &|v| {
        // A reader branching on mu > 0 would classify this sticking; the classifier keeps it sliding.
        entry(v)["classified_state"] = json!("sticking");
    });
    tamper("a strict-gap decision against its signs", "h7g_gap", DERIVATION, &|v| entry(v)["strict_gap"]["state"] = json!("active"));
    tamper("a strict-gap prior forged", "h7g_gap", EVIDENCE, &|v| entry(v)["strict_gap"]["prior"] = json!("inactive"));
    // Standing and disclosure.
    tamper("a release promoted", "h2_released", OPERANDS_SHAPE, &|v| {
        record(v)["standing"] = json!({"kind": "established", "reason": ESTABLISHED});
    });
    tamper("a gap standing forged", "h7g_gap", OPERANDS_SHAPE, &|v| record(v)["standing"]["kind"] = json!("not_established"));
    tamper("E without its conditions", "h4_kept_explicit", DERIVATION, &|v| { record(v).as_object_mut().unwrap().remove("conditions"); });
    tamper("a condition's class forged", "h16a_input_decision", DERIVATION, &|v| record(v)["conditions"][0]["class"] = json!("strict_ball"));
    tamper("E's sentence edited", "h4_kept_explicit", OPERANDS_SHAPE, &|v| record(v)["standing"]["reason"] = json!("Established."));
    tamper("the release notice removed", "h2_released", DERIVATION, &|v| {
        v["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != RELEASED);
    });
    tamper("an unconverged held result claimed kept", "h4_kept_explicit", DERIVATION, &|v| record(v)["held_solve"]["converged"] = json!(false));
    // The derived normal's source (review P2-1), against the minuend's derived-normal row.
    tamper("a derived normal's source support forged", "h6b_kept_derived", BINDING, &|v| entry(v)["inputs"]["normal_source_ref"]["support"] = json!("support:a"));
    tamper("a derived normal's source DOF forged", "h6b_kept_derived", BINDING, &|v| entry(v)["inputs"]["normal_source_ref"]["dof"] = json!("UX"));
    tamper("a derived normal's source removed", "h6b_kept_derived", EVIDENCE, &|v| { entry(v)["inputs"].as_object_mut().unwrap().remove("normal_source_ref"); });
    tamper("a derived normal's source opened", "h6b_kept_derived", EVIDENCE, &|v| entry(v)["inputs"]["normal_source_ref"]["note"] = json!("x"));
    tamper("an explicit normal given a source", "h4_kept_explicit", EVIDENCE, &|v| entry(v)["inputs"]["normal_source_ref"] = json!({"support": "support:z", "node": "node:b", "dof": "UZ"}));
    // The source's provenance and the target's modulus key (review P2-2).
    tamper("an unqualified source admitted", "h4_kept_explicit", BINDING, &|v| record(v)["source"]["qualification"] = json!("inspectable_unqualified_gap"));
    tamper("an admitted source's standing removed", "h4_kept_explicit", BINDING, &|v| record(v)["source"]["standing"] = Value::Null);
    tamper("a confirmed source stated not established", "h4_kept_explicit", BINDING, &|v| record(v)["source"]["standing"] = json!("not_established"));
    tamper("an unconfirmed source stated confirmed", "h15c_zero_limit", BINDING, &|v| record(v)["source"]["standing"] = json!("confirmed"));
    tamper("an absent source stated qualified", "source_unavailable", BINDING, &|v| {
        record(v)["source"]["qualification"] = json!("qualified");
        record(v)["source"]["standing"] = json!("not_established");
    });
    tamper("a target modulus key forged", "h4_kept_explicit", BINDING, &|v| record(v)["target"]["modulus_basis_key"] = json!("bogus"));
    tamper("a target modulus key stated exact", "h4_kept_explicit", BINDING, &|v| record(v)["target"]["modulus_basis_key"] = json!("exact:point-a"));
    // The held solve's convergence against its decisions (review P2-3).
    tamper("a released held result claimed converged", "h2_released", DERIVATION, &|v| {
        record(v)["held_solve"]["converged"] = json!(true);
        record(v)["held_solve"]["exit_diagnostic"] = Value::Null;
    });
    tamper("a released held result's convergence alone forged", "h2_released", DERIVATION, &|v| record(v)["held_solve"]["converged"] = json!(true));
    tamper("a non-converged exit diagnostic removed", "h2_released", DERIVATION, &|v| record(v)["held_solve"]["exit_diagnostic"] = Value::Null);
    tamper("a non-converged exit diagnostic renamed", "h2_released", DERIVATION, &|v| record(v)["held_solve"]["exit_diagnostic"]["code"] = json!("NONLINEAR_SUPPORT_STATE_REVIEW"));
    tamper("a non-converged exit diagnostic emptied", "h2_released", DERIVATION, &|v| record(v)["held_solve"]["exit_diagnostic"]["message"] = json!(" "));
    tamper("a converged held result given an exit diagnostic", "h4_kept_explicit", DERIVATION, &|v| {
        record(v)["held_solve"]["exit_diagnostic"] = json!({"code": "NONLINEAR_SUPPORT_NONCONVERGENCE", "message": "x"});
    });
    // Rows.
    tamper("the held sign on the minuend's case row", "h2_released", ROW, &|v| {
        let row = v["results"].as_array_mut().unwrap().iter_mut().find(|r| r["basis_ref"]["ref_type"] == "load_case" && !r["metadata"].is_null()).unwrap();
        row["metadata"]["sign_convention"] = json!(SIGN);
    });
    tamper("the typed basis on a difference row", "h2_released", OPERANDS_ROWS, &|v| {
        let row = v["results"].as_array_mut().unwrap().iter_mut().find(|r| r["basis_ref"]["ref_type"] == "combination" && !r["id"].as_str().unwrap().contains(":held:") && !r["metadata"].is_null() && r["kind"] != "combination_modulus_basis_record").unwrap();
        row["metadata"]["basis"] = json!(OPERAND_BASIS);
    });
    tamper("a held row's clause removed", "h2_released", ROW, &|v| {
        let row = v["results"].as_array_mut().unwrap().iter_mut().find(|r| r["id"].as_str().unwrap().contains(":held:")).unwrap();
        row["metadata"]["sign_convention"] = json!("positive along global X");
    });
    let mut failures = Vec::new();
    for (what, base, forged, expected) in cases {
        let (model, _) = document(base);
        let (without, with) = (read(&forged, None), read(&forged, Some(&model)));
        if without != expected || with == "ok" {
            failures.push(format!("{what}: without {without} (expected {expected}), with {with}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// With the model the reader also binds each support's inputs and the grounds.
#[test]
fn the_model_bound_reading_refuses_inputs_the_model_contradicts() {
    let mut cases: Vec<(&str, &str, Value)> = Vec::new();
    let mut tamper = |what: &'static str, base: &'static str, edit: &dyn Fn(&mut Value)| {
        let (_, mut forged) = document(base);
        edit(&mut forged);
        cases.push((what, base, forged));
    };
    tamper("a coefficient forged", "h4_kept_explicit", &|v| entry(v)["inputs"]["coefficient_bits"] = hex(0.5));
    tamper("an explicit normal forged consistently", "h4_kept_explicit", &|v| {
        entry(v)["inputs"]["normal_signed_bits"] = hex(8192.0);
        entry(v)["inputs"]["normal_trial_bits"] = hex(8192.0);
    });
    tamper("a node forged", "h2_released", &|v| entry(v)["node"] = json!("node:a"));
    tamper("the trial's ground forged", "h17c_moved_ground", &|v| entry(v)["inputs"]["ground_bits"] = hex(1.0 / 1024.0));
    tamper("a derived normal's source node forged", "h6b_kept_derived", &|v| entry(v)["inputs"]["normal_source_ref"]["node"] = json!("node:a"));
    let mut failures = Vec::new();
    for (what, base, forged) in cases {
        let (model, _) = document(base);
        let with = read(&forged, Some(&model));
        if with != MODEL {
            failures.push(format!("{what}: with {with}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
    // The sliding-ground withholding is re-derived from both cases' authored motion: a
    // minuend that publishes no sliding support cannot be withheld by it.
    let (model, mut forged) = document("sliding_ground_differs");
    for code in forged["results"].as_array_mut().unwrap().iter_mut().filter(|r| r["kind"] == "nonlinear_support_active_set_state_code") {
        code["value"] = json!(2.0);
    }
    assert_eq!(read(&forged, Some(&model)), MODEL);
    // The model's own derived-normal assignment (review P2-1): the document unchanged, the
    // model's source moved to another support and DOF.
    let (mut model, document) = super::tests::document("h6b_kept_derived");
    let support = model["supports"].as_array_mut().unwrap().iter_mut().find(|s| s["id"] == "support:t").unwrap();
    support["nonlinear"]["normal_reaction_source"] = json!({"support_ref": "support:a", "dof": "UX"});
    assert_ne!(read(&document, Some(&model)), "ok");
    assert_eq!(read(&document, None), "ok");
    assert_eq!(super::model_check(&document, &model, &super::read(&document).unwrap()), Err(MODEL.to_string()));
    // The target's modulus key against the model (review P2-2): a key and label forged
    // coherently read without the model, and are refused with it.
    let (model, mut forged) = super::tests::document("h4_kept_explicit");
    record(&mut forged)["target"]["modulus_basis_key"] = json!("exact:point-a");
    let label = forged["results"].as_array_mut().unwrap().iter_mut()
        .find(|r| r["kind"] == MODULUS_RECORD && r["entity_ref"] == "case:S" && r["basis_ref"]["ref_type"] == "combination").unwrap();
    label["metadata"]["basis"] = json!("temperature_point:point-a; selection=exact_user_entered_point; interpolation=not_performed; x");
    assert_eq!(super::read(&forged).map(|_| ()), Ok(()));
    assert_eq!(super::model_check(&forged, &model, &super::read(&forged).unwrap()), Err(MODEL.to_string()));
    // A case with conditions cannot be read with the model.
    let (mut model, document) = super::tests::document("h2_released");
    model["load_cases"][0]["condition_use"] = json!({"condition_set": "x"});
    assert_eq!(read(&document, Some(&model)), UNVERIFIED);
}
