//! Derivative assembly and validation over a synthetic current carrier, not
//! origin authentication tests. b7-plan E2: the retired 0.1.0 carrier and its
//! v0_2 table (with their 60/28 projection fixtures) are gone; the carrier here
//! is a current precision-1 header, whose table admits synthetic rows without
//! route evidence.
use open_pipe_stress_result_export::derivative as d;
use serde_json::{json,Value};
fn project(row:Value)->Result<(Value,Value),String>{
 let base:Value=serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
 let model=json!({"project":{"id":"project:synthetic-semantic-contract"}});
 let source=json!({"schema_version":"0.2.0","producer":{"component_name":"open_pipe_stress_product_physics","component_version":"0.2.0","semantic_contract_id":"openpipestress.result_semantics/0.3.0/precision-1"},
  "numerical_quality":{"value_representation":"finite_binary64","publication_quantization":"none","integrity_policy":"M03-INTEGRITY-v1","status":"checks_passed","cases":[]},
  "formulation_basis":{"profile_id":"product_preview_mechanics_v1","limitations":["synthetic derivative test carrier"]},
  "run_id":"run:synthetic-semantic-contract","model_ref":model["project"]["id"],"status":{"mechanics":"MECHANICS_SOLVED"},"results":[row]});
 let carrier=d::checksum(&source,"received_current_dimension_absent_carrier",d::reference("synthetic_received_carrier","fixture-only"))?;
 let origin=json!({"origin_id":"synthetic-fixture-unattested","origin_class":"received_current_dimension_absent","qualification_ref":d::reference("synthetic_fixture","not-authentic-Current"),"authentic_producer_available":false,"received_carrier_checksum":carrier,"original_producer_checksum":null,"origin_limit":"Unattested synthetic test projection","actual_model_ref":d::reference("model_payload","project:synthetic-semantic-contract"),"mechanics_run_ref":d::reference("mechanics_run","run:synthetic-semantic-contract"),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
 let doc=d::derive_document(base,&model,&source,origin,None)?;Ok((doc,source))
}

#[test]fn integer_values_unknown_disclosures_and_accounting_tamper(){
 let row=json!({"id":"integer-quantity","kind":"displacement_magnitude","value":10,"unit":"mm","entity_ref":"node:fixture"});let(doc,source)=project(row).unwrap();assert_eq!(doc["result_envelope"]["result_sets"][0]["values"][0]["magnitude"],json!(10));let mut t=doc.clone();t["result_envelope"]["row_accounting"]=json!([]);assert!(d::validate_document(&t,&source).is_err());let mut t=doc;t["result_envelope"]["result_sets"][0]["values"][0]["magnitude"]=json!(11);assert!(d::validate_document(&t,&source).is_err());
 let row=json!({"id":"integer-unknown","kind":"future_unknown_force_ratio","value":12,"unit":"unknown-count","entity_ref":"fixture","metadata":{"extra":{"é":[1,"𐀀"]}}});let(doc,_)=project(row).unwrap();assert_eq!(doc["result_envelope"]["row_disclosures"][0]["source_value"],json!(12));assert!(doc["result_envelope"]["row_disclosures"][0]["declared_semantic_dimension"].is_null());
}
#[test]fn guarded_serializer_numeric_keys_unicode_negative_zero_and_unsafe_int(){
 let x=json!({"10":10,"2":2,"é":"accent","𐀀":"supplementary","\u{e000}":"bmp","zero":-0.0});assert_eq!(d::digest(&x).unwrap().len(),64);assert!(open_pipe_stress_canonical_json::canonical_json(&x).starts_with("{\"10\":10,\"2\":2,"));assert!(d::digest(&json!({"unsafe":9007199254740992u64})).is_err());
}


#[test]fn validator_rejects_rescoped_ghost_orphan_and_semantic_tampering_even_when_rehashed(){
 let (doc,source)=project(json!({"id":"binding","kind":"displacement_magnitude","value":10,"unit":"mm","entity_ref":"node:fixture"})).unwrap();
 let paths=["/result_envelope/row_accounting/0/target_ref/ref_id","/result_envelope/row_accounting/0/received_carrier_row_checksum/payload_scope","/result_envelope/source_annotations/0/source_origin_ref/ref_id","/result_envelope/unit_preservation_witnesses/0/source_quantity/unit","/result_envelope/unit_preservation_witnesses/0/target_row_checksum/algorithm","/result_envelope/result_sets/0/values/0/dimension"];
 for path in paths{let mut bad=doc.clone();*bad.pointer_mut(path).unwrap()=json!("tampered");bad["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");let hash=d::checksum(&bad,"derivative_document_excludes_own_hash",d::reference("derivative_document",bad["result_envelope"]["envelope_id"].as_str().unwrap())).unwrap();bad["result_envelope"]["reproducibility"]["derivative_hash"]=hash;assert!(d::validate_document(&bad,&source).is_err(),"{path}");}
 for value in [json!(null),json!({}),json!([])]{let mut bad=doc.clone();bad["result_envelope"]["result_sets"]=value;assert!(d::validate_document(&bad,&source).is_err());}
}

#[test]
fn numeric_raw_annotations_bind_by_value_across_json_forms() {
    let (doc, source) = project(json!({
        "id": "unknown-numeric-annotation", "kind": "invented_unknown_kind",
        "value": 1.0, "unit": "m", "entity_ref": "node:fixture",
        "metadata": {"extra": {"é": [1.0, {"value": 2.0}]}}
    })).unwrap();
    let jcs = |v: &Value| -> Value {
        serde_json::from_str(&open_pipe_stress_canonical_json::canonical_json(v)).unwrap()
    };
    let copied = jcs(&doc);
    assert_ne!(copied, doc, "JCS changes numeric JSON forms");
    assert_eq!(open_pipe_stress_canonical_json::canonical_json(&copied),
               open_pipe_stress_canonical_json::canonical_json(&doc));
    assert_eq!(d::validate_document(&copied, &source), Ok(()));
    assert_eq!(d::validate_document(&doc, &jcs(&source)), Ok(()));
    let mut wrong = copied;
    wrong["result_envelope"]["source_annotations"][0]["metadata"]["extra"]["é"][0] = json!(2);
    assert_eq!(d::validate_document(&wrong, &source), Err("SOURCE_ANNOTATION_MISMATCH".into()));
}
