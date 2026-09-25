//! Frozen semantic projection expectations, not origin authentication tests.
use open_pipe_stress_result_export::{derivative as d,semantic_contract};
use serde_json::{json,Value};
fn project(row:Value)->Result<(Value,Value),String>{
 let base:Value=serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
 let model=json!({"project":{"id":"project:synthetic-semantic-contract"}});
 let source=json!({"schema_version":"0.1.0","run_id":"run:synthetic-semantic-contract","model_ref":model["project"]["id"],"status":{"mechanics":"MECHANICS_SOLVED"},"results":[row]});
 let carrier=d::checksum(&source,"received_current_dimension_absent_carrier",d::reference("synthetic_received_carrier","fixture-only"))?;
 let origin=json!({"origin_id":"synthetic-fixture-unattested","origin_class":"received_current_dimension_absent","qualification_ref":d::reference("synthetic_fixture","not-authentic-Current"),"authentic_producer_available":false,"received_carrier_checksum":carrier,"original_producer_checksum":null,"origin_limit":"Unattested synthetic test projection","actual_model_ref":d::reference("model_payload","project:synthetic-semantic-contract"),"mechanics_run_ref":d::reference("mechanics_run","run:synthetic-semantic-contract"),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
 let doc=d::derive_document(base,&model,&source,origin,None)?;Ok((doc,source))
}
fn capture(name:&str,doc:&Value,source:&Value){if let Ok(dir)=std::env::var("RESULTS_RUST_CONTRACT_OUTPUT_DIR"){let dir=std::path::Path::new(&dir);std::fs::create_dir_all(dir).unwrap();std::fs::write(dir.join(format!("{name}.document.json")),serde_json::to_vec_pretty(doc).unwrap()).unwrap();std::fs::write(dir.join(format!("{name}.received.json")),serde_json::to_vec_pretty(source).unwrap()).unwrap();}}

#[test]fn all60_independent_semantics_metadata_and_missing_cases(){
 let cases:Value=serde_json::from_str(include_str!("../../../../fixtures/results/invented/result_export_v0_2.json")).unwrap();assert_eq!(cases["fixtures"].as_array().unwrap().len(),60);
 for f in cases["fixtures"].as_array().unwrap(){
  let row=f["input_row"].clone();let(doc,source)=project(row.clone()).unwrap_or_else(|e|panic!("{} {e}",f["signature_id"]));capture(f["signature_id"].as_str().unwrap(),&doc,&source);let e=&doc["result_envelope"];let expected=&f["expected"];let account=&e["row_accounting"][0];assert_eq!(account["disposition"],expected["disposition"],"{}",f["signature_id"]);
  let target=doc.pointer(account["target_field_path"].as_str().unwrap()).unwrap();let actual_value=if account["disposition"]=="disclosed"{&target["source_value"]}else{&target["magnitude"]};assert_eq!(actual_value,&row["value"]);assert_eq!(e["source_annotations"][0]["metadata"],row.get("metadata").cloned().unwrap_or(Value::Null));assert_eq!(e["source_annotations"][0]["source_physical_semantic_dimension"],expected["source_physical_semantic_dimension"]);assert_eq!(e["source_annotations"][0]["derivative_target_dimension"],expected["derivative_target_dimension"]);
  if !expected["canonical_metadata"].is_null(){assert_eq!(target["metadata"],expected["canonical_metadata"]);}
  d::validate_document(&doc,&source).unwrap();
  let mut wrong=row.clone();wrong["unit"]=json!("Pa");assert!(project(wrong).unwrap_err().contains("UNIT_CONTRADICTION"));
  if !f["negative_component"].is_null(){let mut wrong=row.clone();wrong["metadata"]["component"]=json!("contradictory_source_component");assert!(project(wrong).unwrap_err().contains("COMPONENT_CONTRADICTION"));}
  let s=semantic_contract::signature(&row).unwrap().unwrap();if s["canonical_disposition"]=="exported_review"||matches!(s["family"].as_str(),Some("force"|"moment")){let mut missing=row;missing.as_object_mut().unwrap().remove("metadata");let (missing,_)=project(missing).unwrap();assert_eq!(missing["result_envelope"]["row_accounting"][0]["disposition"],"disclosed");}
 }
}
#[test]fn all28_raw_annotations_are_exact_and_canonical_projection_is_strict(){
 let cases:Value=serde_json::from_str(include_str!("../../../../fixtures/results/invented/result_export_v0_2_rejections.json")).unwrap();assert_eq!(cases["cases"].as_array().unwrap().len(),28);
 for f in cases["cases"].as_array().unwrap(){let(doc,source)=project(f["received_row"].clone()).unwrap_or_else(|e|panic!("{} {e}",f["case_id"]));capture(f["case_id"].as_str().unwrap(),&doc,&source);let e=&doc["result_envelope"];assert_eq!(e["source_annotations"][0]["metadata"],f["expected"]["annotation_metadata"]);assert_eq!(e["row_accounting"][0]["disposition"],f["expected"]["row_disposition"],"{}",f["case_id"]);let target=doc.pointer(e["row_accounting"][0]["target_field_path"].as_str().unwrap()).unwrap();if f["expected"]["row_disposition"]=="disclosed"{assert_eq!(target["reason_code"],f["expected"]["reason_code"]);}else if !f["expected"]["canonical_metadata"].is_null(){assert_eq!(target["metadata"],f["expected"]["canonical_metadata"]);}}
}
#[test]fn integer_values_unknown_disclosures_and_accounting_tamper(){
 let row=json!({"id":"integer-quantity","kind":"displacement_magnitude","value":10,"unit":"mm","entity_ref":"node:fixture"});let(doc,source)=project(row).unwrap();assert_eq!(doc["result_envelope"]["result_sets"][0]["values"][0]["magnitude"],json!(10));let mut t=doc.clone();t["result_envelope"]["row_accounting"]=json!([]);assert!(d::validate_document(&t,&source).is_err());let mut t=doc;t["result_envelope"]["result_sets"][0]["values"][0]["magnitude"]=json!(11);assert!(d::validate_document(&t,&source).is_err());
 let row=json!({"id":"integer-unknown","kind":"future_unknown_force_ratio","value":12,"unit":"unknown-count","entity_ref":"fixture","metadata":{"extra":{"é":[1,"𐀀"]}}});let(doc,_)=project(row).unwrap();assert_eq!(doc["result_envelope"]["row_disclosures"][0]["source_value"],json!(12));assert!(doc["result_envelope"]["row_disclosures"][0]["declared_semantic_dimension"].is_null());
}
#[test]fn guarded_serializer_numeric_keys_unicode_negative_zero_and_unsafe_int(){
 let x=json!({"10":10,"2":2,"é":"accent","𐀀":"supplementary","\u{e000}":"bmp","zero":-0.0});assert_eq!(d::digest(&x).unwrap().len(),64);assert!(open_pipe_stress_canonical_json::canonical_json(&x).starts_with("{\"10\":10,\"2\":2,"));assert!(d::digest(&json!({"unsafe":9007199254740992u64})).is_err());
}

#[test]fn independent_qualified_hash_vectors_match_exact_utf8_and_digest(){
 let contract:Value=serde_json::from_str(include_str!("../../../../fixtures/results/semantic_contract_v0_2.json")).unwrap();
 for v in contract["hash_vectors"].as_array().unwrap(){assert_eq!(open_pipe_stress_canonical_json::canonical_json(&v["input"]),v["expected_canonical_json"]);assert_eq!(d::digest(&v["input"]).unwrap(),v["expected_sha256"]);}
}

#[test]fn validator_rejects_rescoped_ghost_orphan_and_semantic_tampering_even_when_rehashed(){
 let (doc,source)=project(json!({"id":"binding","kind":"displacement_magnitude","value":10,"unit":"mm","entity_ref":"node:fixture"})).unwrap();
 let paths=["/result_envelope/row_accounting/0/target_ref/ref_id","/result_envelope/row_accounting/0/received_carrier_row_checksum/payload_scope","/result_envelope/source_annotations/0/source_origin_ref/ref_id","/result_envelope/unit_preservation_witnesses/0/source_quantity/unit","/result_envelope/unit_preservation_witnesses/0/target_row_checksum/algorithm","/result_envelope/result_sets/0/values/0/dimension"];
 for path in paths{let mut bad=doc.clone();*bad.pointer_mut(path).unwrap()=json!("tampered");bad["result_envelope"]["reproducibility"].as_object_mut().unwrap().remove("derivative_hash");let hash=d::checksum(&bad,"derivative_document_excludes_own_hash",d::reference("derivative_document",bad["result_envelope"]["envelope_id"].as_str().unwrap())).unwrap();bad["result_envelope"]["reproducibility"]["derivative_hash"]=hash;assert!(d::validate_document(&bad,&source).is_err(),"{path}");}
 for value in [json!(null),json!({}),json!([])]{let mut bad=doc.clone();bad["result_envelope"]["result_sets"]=value;assert!(d::validate_document(&bad,&source).is_err());}
}
