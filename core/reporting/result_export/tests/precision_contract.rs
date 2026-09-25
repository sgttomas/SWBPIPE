//! Transport vectors are independent binary64 bit patterns, not solver oracles.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};
fn source(value: f64) -> Value {
    json!({"schema_version":"0.2.0", "run_id":"precision-transport", "model_ref":"precision-model", "status":{"mechanics":"MECHANICS_SOLVED"},
      "producer":{"component_name":"open_pipe_stress_product_physics","component_version":"0.2.0","semantic_contract_id":s::PRECISION_ID},
      "numerical_quality":{"value_representation":"finite_binary64","publication_quantization":"none","integrity_policy":"M03-INTEGRITY-v1","status":"not_assessed","cases":[]},
      "formulation_basis":{"profile_id":"product_preview_mechanics_v1","limitations":["Transport test; physics unqualified"]},
      "diagnostics":[],"results":[{"id":"rotation","kind":"global_nodal_rotation_x","value":value,"unit":"rad","entity_ref":"node:transport"}]})
}
fn derive(source: &Value) -> Result<Value,String> {
    let base:Value=serde_json::from_str(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json")).unwrap();
    let carrier=d::checksum(source,"attested_headless_producer_carrier",d::reference("test_carrier","transport"))?;
    let origin=json!({"origin_id":"transport-fixture", "origin_class":"attested_headless_producer", "qualification_ref":d::reference("test_fixture","not-authentication"), "authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Synthetic transport only", "actual_model_ref":d::reference("model_payload","precision-model"),"mechanics_run_ref":d::reference("mechanics_run","precision-transport"),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base,&json!({"project":{"id":"precision-model"}}),source,origin,None)
}
#[test]
fn same_unit_bits_survive_json_derive_read_and_hash() {
    let fixture:Value=serde_json::from_str(include_str!("../../../../fixtures/results/precision_transport_v0_3.json")).unwrap();
    for vector in fixture["vectors"].as_array().unwrap() {
        let bits=u64::from_str_radix(vector["bits_hex"].as_str().unwrap(),16).unwrap();
        let value:f64=vector["decimal"].as_str().unwrap().parse().unwrap();
        assert_eq!(value.to_bits(),bits);let raw=source(value);
        let parsed:Value=serde_json::from_slice(&serde_json::to_vec(&raw).unwrap()).unwrap();
        assert_eq!(parsed["results"][0]["value"].as_f64().unwrap().to_bits(),value.to_bits());
        let doc=derive(&parsed).unwrap();assert_eq!(doc["schema_version"],"0.3.0");
        if let Ok(dir)=std::env::var("RESULTS_RUST_CONTRACT_OUTPUT_DIR") {
            let dir=std::path::Path::new(&dir);std::fs::create_dir_all(dir).unwrap();
            std::fs::write(dir.join(format!("precision-{bits:016x}.document.json")),serde_json::to_vec_pretty(&doc).unwrap()).unwrap();
        }
        let reread:Value=serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reread,&parsed).unwrap();assert_eq!(d::digest(&doc).unwrap(),d::digest(&reread).unwrap());
        let account=&doc["result_envelope"]["row_accounting"][0];let target=doc.pointer(account["target_field_path"].as_str().unwrap()).unwrap();
        let number=if account["disposition"]=="disclosed"{&target["source_value"]}else{&target["magnitude"]};
        assert_eq!(number.as_f64().unwrap().to_bits(),value.to_bits());
    }
}
#[test]
fn explicit_dispatch_metadata_binding_and_unchanged_ijson_limit() {
    let raw=source(0.00000012345678901234567);let doc=derive(&raw).unwrap();
    for key in ["producer","numerical_quality","formulation_basis"] {assert_eq!(doc["result_envelope"][key],raw[key]);}
    for id in ["reactions-1","pressure-1","stress-1","future-1"] { let mut wrong=raw.clone(); wrong["producer"]["semantic_contract_id"]=json!(format!("openpipestress.result_semantics/0.3.0/{id}"));assert!(derive(&wrong).is_err()); }
    let mut wrong=raw.clone();wrong["schema_version"]=json!("0.1.0");assert!(derive(&wrong).is_err());
    let mut wrong=raw.clone();wrong["schema_version"]=json!("9.0.0");assert!(derive(&wrong).is_err());
    for path in ["/result_envelope/producer/component_version","/result_envelope/numerical_quality/status","/result_envelope/formulation_basis/profile_id","/result_envelope/semantic_contract_ref/ref_id"] {
        let mut altered=doc.clone();*altered.pointer_mut(path).unwrap()=json!("tampered");assert!(d::validate_document(&altered,&raw).is_err());
    }
    let mut unknown=raw.clone();unknown["results"][0]["kind"]=json!("unknown_future_quantity");assert_eq!(derive(&unknown).unwrap()["result_envelope"]["row_disclosures"][0]["reason_code"],"unsupported_source_kind");
    assert!(derive(&source(9007199254740992.0)).unwrap_err().contains("UNSAFE_JSON_NUMBER"));
    let mut legacy=raw;legacy["schema_version"]=json!("0.1.0");for key in ["producer","numerical_quality","formulation_basis"]{legacy.as_object_mut().unwrap().remove(key);}let old=derive(&legacy).unwrap();assert_eq!(old["schema_version"],"0.2.0");assert!(old["result_envelope"].get("producer").is_none());
}
#[test]
fn historical_hash_vectors_and_rows_are_unchanged() {
    assert_eq!(s::contract()["rows"],s::precision_contract()["rows"]);
    for vector in s::contract()["hash_vectors"].as_array().unwrap(){assert_eq!(d::digest(&vector["input"]).unwrap(),vector["expected_sha256"]);}
}
#[test]
fn rule_eligibility_requires_actual_case_coverage_and_resolving_evidence() {
    let mut raw=source(1e-7);let basis=json!({"ref_type":"load_case","ref_id":"load:one"});
    assert_eq!(s::numerical_use_standing(&raw,&[basis.clone()]),"needs_recompute");
    raw["numerical_quality"]["status"]=json!("checks_passed");
    assert_eq!(s::numerical_use_standing(&raw,&[basis.clone()]),"needs_recompute");
    raw["numerical_quality"]["cases"]=json!([{"basis_ref":basis,"structural_status":"passive_model_basis","solve_quality":"checks_passed","model_matrix_fidelity":"represented_equations_retained","accuracy_evidence":"not_claimed","evidence_refs":["rotation"]}]);
    assert_eq!(s::numerical_use_standing(&raw,&[basis.clone()]),"numerically_eligible");
    assert_eq!(s::numerical_use_standing(&raw,&[]),"needs_recompute");
    assert_eq!(s::numerical_use_standing(&raw,&[basis.clone(),basis.clone()]),"needs_recompute");
    raw["numerical_quality"]["cases"][0]["solve_quality"]=json!("sensitive");
    assert_eq!(s::numerical_use_standing(&raw,&[basis.clone()]),"needs_recompute");
    raw["numerical_quality"]["status"]=json!("sensitive");
    for accuracy in ["not_claimed", "reference_verified"] {
        raw["numerical_quality"]["cases"][0]["accuracy_evidence"]=json!(accuracy);
        let before=raw.clone();
        assert!(s::for_source(&raw).is_ok());
        assert_eq!(s::numerical_use_standing(&raw,&[basis.clone()]),"needs_recompute");
        assert_eq!(raw,before);
    }
    raw["numerical_quality"]["cases"][0]["evidence_refs"]=json!(["missing"]);
    assert_eq!(s::numerical_use_standing(&raw,&[basis]),"needs_recompute");
}
