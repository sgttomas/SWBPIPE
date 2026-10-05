//! RV80 reviewer-only probes (not part of the candidate); reader_confirm_02 adds PR8-PR15. Each probe edits a
//! shared base, rehashes every affected digest exactly as the candidate's own
//! harness does, and records the Rust reader's first failure with and without
//! the actual invocation. Expectations are RV80's contract readings.
use open_pipe_stress_result_export::retained_precision as rp;
use open_pipe_stress_result_export::source_blocks::domain_hash;
use serde_json::{json, Value};
fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/results/retained_precision_cases.json"
    ))
    .unwrap()
}
fn rehash(source: &mut Value) {
    let b = &mut source["retained_precision"]["body"];
    let attempts = b["product_attempts"].clone();
    for s in b["sources"].as_array_mut().unwrap() {
        if let Some(ai) = s["preparation"]["attempt_ref"].as_u64() {
            let a = &attempts[ai as usize];
            if a["preparation"]["members"].as_array().unwrap().iter().all(|m| m["result"]["kind"] == "prepared") {
                let members: Vec<_> = a["preparation"]["members"].as_array().unwrap().iter().map(|m| json!({"member":m["member"],"old_source":m["old_source"],"old_facts":m["old_facts"],"section":m["result"]["section"]})).collect();
                let payload = json!({"definition_id":a["definition_id"],"definition_sha256":rp::DEFINITION_HASH,"owner_ref":a["owner_ref"],"ordinary_attempt_ref":a["ordinary_attempt_ref"],"material_basis_ref":a["material_basis_ref"],"members":members});
                s["preparation"]["sha256"] = domain_hash("retained_precision_preparation_v1", &payload).unwrap().into();
            }
        }
    }
    let sources = b["sources"].clone();
    for c in b["cases"].as_array_mut().unwrap() {
        if c.get("source_identity_sha256").is_some() {
            let mut s = sources[c["source_ref"].as_u64().unwrap() as usize].clone();
            s.as_object_mut().unwrap().remove("index");
            c["source_identity_sha256"] = domain_hash("retained_precision_source_mp_v2", &s).unwrap().into();
        }
    }
    let mut public = source.clone();
    public.as_object_mut().unwrap().remove("retained_precision");
    source["retained_precision"]["body"]["publication_sha256"] = domain_hash("retained_precision_publication_mp_v2", &public).unwrap().into();
    source["retained_precision"]["receipt_sha256"] = domain_hash("retained_precision_receipt_mp_v2", &source["retained_precision"]["body"]).unwrap().into();
}
fn set(v: &mut Value, path: &[Value], value: Value) {
    let mut p = v;
    for k in path {
        p = if let Some(i) = k.as_u64() { &mut p[i as usize] } else { &mut p[k.as_str().unwrap()] };
    }
    if let Some(d) = value.get("$add").and_then(Value::as_u64) {
        *p = json!(p.as_u64().unwrap() + d);
    } else {
        *p = value;
    }
}
fn outcome(r: Result<rp::Validation, rp::ValidationError>) -> Value {
    match r {
        Ok(v) => json!({"ok":true,"eligible":v.numerical_eligible}),
        Err(e) => json!({"gate":e.gate,"code":e.code}),
    }
}
#[test]
fn rv80_probes() {
    let shared = corpus();
    let probes = json!([
      {"id":"PR1_native_error_with_selected_run","base":"two_case_facade_after_certificate_synthetic",
       "edits":[[["retained_precision","body","product_attempts",1,"result","error"],{"kind":"native","run_ref":1}]],
       "contract":"S06:38 native{run_ref}: 'native selected is invalid for this error' -> G5 PRODUCT_ATTEMPT_MISMATCH"},
      {"id":"PR2_unsourced_complete_old_member_gap","base":"two_case_preparation_failure_synthetic",
       "edits":[[["retained_precision","body","product_attempts",1,"operational","old",0,"member"],1]],
       "contract":"F1:78-84,101,130 old ids are the full inventory in native member order; G3 checks the declared inventory -> G3 COVERAGE_MISMATCH"},
      {"id":"PR3_unavailable_record_bound_null_with_data","base":"two_case_facade_after_certificate_synthetic",
       "edits":[[["retained_precision","body","cases",1,"run","records",1,"verification","bound",0,"value"],null]],
       "contract":"I57 s4 item 4 + ROOT parity rule 3: record bound non-null iff has_data -> G5a SCALE_MISMATCH"},
      {"id":"PR4_unavailable_record_bound_extra_on_data_free_flag","base":"two_case_facade_after_certificate_synthetic",
       "edits":[[["retained_precision","body","product_attempts",1,"proof","summary_coverage",0,"has_data"],false]],
       "contract":"I57 s4 item 4 / data facts: nonzero free nodal term implies has_data -> G5a SCALE_MISMATCH"},
      {"id":"PR5_native_error_foreign_run_ref","base":"two_case_facade_after_certificate_synthetic",
       "edits":[[["retained_precision","body","product_attempts",1,"result","error"],{"kind":"native","run_ref":0}]],
       "contract":"S06:33,38 same Run; native selected invalid -> G5 PRODUCT_ATTEMPT_MISMATCH"},
      {"id":"PR6_unavailable_source_preparation_backref_foreign","base":"two_case_facade_after_certificate_synthetic",
       "edits":[[["retained_precision","body","sources",1,"preparation","attempt_ref"],0]],
       "contract":"C3:146-148 CaseSource.preparation.attempt_ref = attempt.id for every constructed source -> G5 PRODUCT_ATTEMPT_MISMATCH (or earlier G1 if a hash moves)"},
      {"id":"PR7_ordinary_material_basis_disagrees","base":"two_case_facade_after_certificate_synthetic",
       "edits":[[["retained_precision","body","ordinary_attempts",1,"material_basis_ref"],1]],
       "contract":"C3:165-166 product attempt agrees with ordinary attempt and material basis -> G5 PRODUCT_ATTEMPT_MISMATCH"},
      {"id":"PR8_captured_prefix_with_run_ref","base":"two_case_preparation_failure_synthetic",
       "edits":[[["retained_precision","body","product_attempts",1,"operational","old_coverage"],"captured_prefix"],[["retained_precision","body","product_attempts",1,"run_ref"],0]],
       "contract":"D1 (F1:97,130-131; C3:304): captured_prefix member part at G3; null run_ref is G5 PRODUCT_ATTEMPT_MISMATCH"},
      {"id":"PR9_native_work_then_attempt_dual","base":"ordinary_prepared_synthetic",
       "edits":[[["retained_precision","body","cases",0,"run","records",0,"work","own_lme"],{"$add":1}],[["retained_precision","body","cases",0,"selection","pivot_margin_min"],"3ff0000000000001"]],
       "contract":"D3 class-1 convention: a native class with an ATTEMPT defect reports ATTEMPT -> G5 ATTEMPT_MISMATCH"},
      {"id":"PR10_native_work_only","base":"ordinary_prepared_synthetic",
       "edits":[[["retained_precision","body","cases",0,"run","records",0,"work","own_lme"],{"$add":1}]],
       "contract":"D3: WORK alone reports WORK -> G5 WORK_MISMATCH"},
      {"id":"PR11_empty_body_inventory_empty_coverage","base":"ordinary_prepared_synthetic",
       "edits":[[["retained_precision","body","sources",0,"body_membership"],[]],[["retained_precision","body","product_attempts",0,"proof","summary_coverage"],[]]],
       "contract":"I57 s1 (no empty complete vector; FK/source.rs:497-498 NoNodes) and ROOT parity rule 2 -> G3 COVERAGE_MISMATCH"},
      {"id":"PR12_estimate_reason_quantity_not_in_layout","base":"p512_ladder_synthetic",
       "edits":[[["retained_precision","body","cases",0,"run","records",0,"outcome","reason"],{"space":"attempt","tag":"verification_estimate","quantity":{"tag":"displacement","dof":{"node":99,"component":"UX"}},"body":0,"kind":"translation"}],
                [["retained_precision","body","cases",0,"run","attempts",0,"outcome","reason"],{"space":"attempt","tag":"verification_estimate","quantity":{"tag":"displacement","dof":{"node":99,"component":"UX"}},"body":0,"kind":"translation"}]],
       "contract":"C2:22,54 every quantity reference resolves against the Run's layout (D5d names only stop_rule) -> RV80 reading G5 ATTEMPT_MISMATCH"},
      {"id":"PR13_stop_rule_reason_quantity_not_in_layout","base":"p512_ladder_synthetic",
       "edits":[[["retained_precision","body","cases",0,"run","records",0,"outcome","reason"],{"space":"attempt","tag":"stop_rule","quantity":{"tag":"displacement","dof":{"node":99,"component":"UX"}},"body":0,"kind":"translation"}],
                [["retained_precision","body","cases",0,"run","attempts",0,"outcome","reason"],{"space":"attempt","tag":"stop_rule","quantity":{"tag":"displacement","dof":{"node":99,"component":"UX"}},"body":0,"kind":"translation"}]],
       "contract":"D5d -> G5 ATTEMPT_MISMATCH"},
      {"id": "PR14_idle_run_records_exhausted_meter_chain_not", "base": "two_case_facade_after_certificate_synthetic", "edits": [[["retained_precision", "body", "cases", 1, "run", "records"], []], [["retained_precision", "body", "cases", 1, "run", "attempts"], []], [["retained_precision", "body", "cases", 1, "run", "case_charge"], 0], [["retained_precision", "body", "cases", 1, "run", "invocation_increment"], 0], [["retained_precision", "body", "cases", 1, "run", "origin", "group"], null], [["retained_precision", "body", "cases", 1, "run", "kernel_terminal"], {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "budget", "scope": "invocation"}}], [["retained_precision", "body", "groups", 0, "source_refs"], [0]], [["retained_precision", "body", "cases", 1, "run", "cache_before"], []], [["retained_precision", "body", "cases", 1, "run", "cache_after"], []], [["retained_precision", "body", "cases", 1, "run", "invocation_before"], 60000000000u64], [["retained_precision", "body", "cases", 1, "run", "invocation_after"], 60000000000u64], [["retained_precision", "body", "calls", 0, "invocation_after"], 60000000000u64], [["retained_precision", "body", "work", "charged"], 60000000000u64]], "contract": "D3 class-1 convention: the run's recorded invocation_before >= Li satisfies the N10 idle shape (g5_schedule; PY:439,675); the broken meter chain is native WORK -> RV80 reading G5 WORK_MISMATCH (Rust's extra running-meter conjunct gives ATTEMPT)"},
      {"id": "PR15_idle_run_not_exhausted_control", "base": "two_case_facade_after_certificate_synthetic", "edits": [[["retained_precision", "body", "cases", 1, "run", "records"], []], [["retained_precision", "body", "cases", 1, "run", "attempts"], []], [["retained_precision", "body", "cases", 1, "run", "case_charge"], 0], [["retained_precision", "body", "cases", 1, "run", "invocation_increment"], 0], [["retained_precision", "body", "cases", 1, "run", "origin", "group"], null], [["retained_precision", "body", "cases", 1, "run", "kernel_terminal"], {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "budget", "scope": "invocation"}}], [["retained_precision", "body", "groups", 0, "source_refs"], [0]], [["retained_precision", "body", "cases", 1, "run", "cache_before"], []], [["retained_precision", "body", "cases", 1, "run", "cache_after"], []], [["retained_precision", "body", "cases", 1, "run", "invocation_after"], 17], [["retained_precision", "body", "calls", 0, "invocation_after"], 17], [["retained_precision", "body", "work", "charged"], 17]], "contract": "control = shared idle_budget_not_exhausted_no_group -> G5 ATTEMPT_MISMATCH"}
    ]);
    let mut out = Vec::new();
    for p in probes.as_array().unwrap() {
        let case = shared["cases"].as_array().unwrap().iter().find(|c| c["id"] == p["base"]).unwrap();
        let mut source = case["source"].clone();
        for e in p["edits"].as_array().unwrap() {
            set(&mut source, e[0].as_array().unwrap(), e[1].clone());
        }
        rehash(&mut source);
        let bound = outcome(rp::validate(&source, Some(&case["invocation"])));
        let unbound = outcome(rp::validate(&source, None));
        let row = json!({"id":p["id"],"contract":p["contract"],"rust_with_invocation":bound,"rust_without_invocation":unbound});
        println!("RV80PROBE {}", row);
        out.push(row);
    }
    if let Ok(path) = std::env::var("RV80_PROBES_OUT") {
        std::fs::write(path, serde_json::to_string_pretty(&out).unwrap()).unwrap();
    }
}
