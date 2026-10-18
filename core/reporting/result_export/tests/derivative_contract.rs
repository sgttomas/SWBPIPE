//! Derivative assembly and validation over a current producer carrier, not
//! origin authentication tests. b7-plan E2 retired the 0.1.0 carrier and its
//! v0_2 table (with their 60/28 projection fixtures), and B1 retired
//! precision-1, the last header whose table admitted synthetic rows without
//! route evidence. Every current header now binds its rows to its own closed
//! table and evidence, so a synthetic unknown-kind or integer-valued row has
//! no current carrier; the carrier here is the committed preview-physics-2
//! producer statement, read as received.
use open_pipe_stress_result_export::derivative as d;
use serde_json::{json, Value};

fn project() -> Result<(Value, Value), String> {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let source: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/preview_physics_invented_sparse.json"
    ))
    .unwrap();
    let model = source["model_ref"].as_str().unwrap().to_owned();
    let carrier = d::checksum(
        &source,
        "attested_headless_producer_carrier",
        d::reference("test_carrier", "derivative-contract"),
    )?;
    let origin = json!({"origin_id":"derivative-contract-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Retained producer bytes; derivative contract test only","actual_model_ref":d::reference("model_payload",&model),"mechanics_run_ref":d::reference("mechanics_run",source["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    let doc = d::derive_document(base, &json!({"project":{"id":model}}), &source, origin, None)?;
    Ok((doc, source))
}

#[test]
fn accounting_and_value_tamper_are_refused() {
    let (doc, source) = project().unwrap();
    assert_eq!(d::validate_document(&doc, &source), Ok(()));
    let mut t = doc.clone();
    t["result_envelope"]["row_accounting"] = json!([]);
    assert!(d::validate_document(&t, &source).is_err());
    let mut t = doc;
    let magnitude = t["result_envelope"]["result_sets"][0]["values"][0]["magnitude"].as_f64().unwrap();
    t["result_envelope"]["result_sets"][0]["values"][0]["magnitude"] = json!(magnitude * 2.0 + 1.0);
    assert!(d::validate_document(&t, &source).is_err());
}

#[test]
fn guarded_serializer_numeric_keys_unicode_negative_zero_and_unsafe_int() {
    let x = json!({"10":10,"2":2,"é":"accent","𐀀":"supplementary","\u{e000}":"bmp","zero":-0.0});
    assert_eq!(d::digest(&x).unwrap().len(), 64);
    assert!(open_pipe_stress_canonical_json::canonical_json(&x).starts_with("{\"10\":10,\"2\":2,"));
    assert!(d::digest(&json!({"unsafe":9007199254740992u64})).is_err());
}

#[test]
fn validator_rejects_rescoped_ghost_orphan_and_semantic_tampering_even_when_rehashed() {
    let (doc, source) = project().unwrap();
    let paths = [
        "/result_envelope/row_accounting/0/target_ref/ref_id",
        "/result_envelope/row_accounting/0/received_carrier_row_checksum/payload_scope",
        "/result_envelope/source_annotations/0/source_origin_ref/ref_id",
        "/result_envelope/unit_preservation_witnesses/0/source_quantity/unit",
        "/result_envelope/unit_preservation_witnesses/0/target_row_checksum/algorithm",
        "/result_envelope/result_sets/0/values/0/dimension",
    ];
    let mut tampered = 0;
    for path in paths {
        let mut bad = doc.clone();
        let Some(slot) = bad.pointer_mut(path) else { continue };
        *slot = json!("tampered");
        tampered += 1;
        bad["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");
        let hash = d::checksum(
            &bad,
            "derivative_document_excludes_own_hash",
            d::reference("derivative_document", bad["result_envelope"]["envelope_id"].as_str().unwrap()),
        )
        .unwrap();
        bad["result_envelope"]["reproducibility"]["derivative_hash"] = hash;
        assert!(d::validate_document(&bad, &source).is_err(), "{path}");
    }
    // The row accounting and the value dimension are always present.
    assert!(tampered >= 3, "{tampered}");
    for value in [json!(null), json!({}), json!([])] {
        let mut bad = doc.clone();
        bad["result_envelope"]["result_sets"] = value;
        assert!(d::validate_document(&bad, &source).is_err());
    }
}

#[test]
fn derived_documents_bind_by_value_across_json_forms() {
    let (doc, source) = project().unwrap();
    let jcs = |v: &Value| -> Value {
        serde_json::from_str(&open_pipe_stress_canonical_json::canonical_json(v)).unwrap()
    };
    let copied = jcs(&doc);
    assert_ne!(copied, doc, "JCS changes numeric JSON forms");
    assert_eq!(
        open_pipe_stress_canonical_json::canonical_json(&copied),
        open_pipe_stress_canonical_json::canonical_json(&doc)
    );
    assert_eq!(d::validate_document(&copied, &source), Ok(()));
    assert_eq!(d::validate_document(&doc, &jcs(&source)), Ok(()));
    let mut wrong = copied;
    let magnitude = wrong["result_envelope"]["result_sets"][0]["values"][0]["magnitude"].as_f64().unwrap();
    wrong["result_envelope"]["result_sets"][0]["values"][0]["magnitude"] = json!(magnitude * 2.0 + 1.0);
    assert!(d::validate_document(&wrong, &source).is_err());
}
