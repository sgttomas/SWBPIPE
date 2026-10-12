//! Join reader contracts. Existing producer bytes supply cross-read negatives;
//! no relabelled fixture is treated as a valid pressure-source publication.
use open_pipe_stress_result_export::{derivative, physics_source as ps, reader_verdict, semantic_contract as sc};
use serde_json::{json, Value};
const V2: &str = include_str!("../../../../fixtures/product_preview/physics_source/n06-sparse_interactive.raw.json");
const REQUEST: &str = include_str!("../../../../fixtures/product_preview/physics_source/n06.request.json");
fn v2() -> Value { serde_json::from_str(V2).unwrap() }
fn invocation() -> Value { json!({"request":serde_json::from_str::<Value>(REQUEST).unwrap(),"solver_mode":"sparse_interactive"}) }

#[test]
fn v2_selected_source_admission_and_standing_are_unchanged() {
    let source = v2();
    let captured = invocation();
    assert_eq!(ps::validate(&source, Some(&captured)), Ok(true));
    assert_eq!(ps::validate(&source, None), Ok(false));
    assert_eq!(ps::validate_transport_metadata(&source), Ok(()));
    let refs = vec![json!({"ref_type":"load_case", "ref_id":"case"})];
    assert_eq!(sc::numerical_use_standing_with_context(&source, &refs, Some(&captured)), "numerically_eligible");
}

#[test]
fn source_contract_cross_reads_are_refused_by_name() {
    let source = v2();
    assert_eq!(ps::validate_in(&source, None, ps::SourceContract::PressureV3), Err(sc::V2_READ_AS_V3.into()));
    assert_eq!(ps::validate_transport_metadata_in(&source, ps::SourceContract::PressureV3), Err(sc::V2_READ_AS_V3.into()));
    // Contradictory headers and damaged receipt shapes never choose their reader.
    let mut relabelled = source.clone();
    relabelled["producer"]["semantic_contract_id"] = json!(ps::PRESSURE_CONTRACT_ID);
    relabelled["formulation_basis"]["profile_id"] = json!(ps::PRESSURE_PROFILE);
    relabelled["source_block_recovery"] = json!({});
    for contract in [ps::SourceContract::PhysicsV2, ps::SourceContract::PressureV3] {
        let code = if contract == ps::SourceContract::PhysicsV2 { sc::V3_READ_AS_V2 } else { sc::V2_READ_AS_V3 };
        assert_eq!(ps::validate_in(&relabelled, None, contract), Err(code.into()));
        assert_eq!(ps::validate_transport_metadata_in(&relabelled, contract), Err(code.into()));
    }
    assert_eq!(reader_verdict::validate("pressure-source", &source, None, None)["error"]["code"], sc::V2_READ_AS_V3);
    assert_eq!(reader_verdict::transport("pressure-source", &source)["error"]["code"], sc::V2_READ_AS_V3);
}

#[test]
fn captured_contract_fixes_the_expected_source_reader() {
    let mut captured = invocation();
    captured["request"]["model"]["pressure_contract"] = json!({"version":"3.0.0", "mode":ps::PRESSURE_PROFILE});
    assert_eq!(ps::SourceContract::from_invocation(&captured), Ok(ps::SourceContract::PressureV3));
    assert_eq!(ps::validate_in(&v2(), Some(&captured), ps::SourceContract::PhysicsV2), Err(sc::V3_READ_AS_V2.into()));
    assert_eq!(ps::validate_in(&json!({}), Some(&invocation()), ps::SourceContract::PressureV3), Err(sc::V2_READ_AS_V3.into()));
}

#[test]
fn captured_v3_model_version_requires_its_received_load_state_namespace() {
    // Contradictory context control: this is not a valid joined publication.
    let mut source = v2();
    source["producer"]["semantic_contract_id"] = json!(ps::PRESSURE_CONTRACT_ID);
    source["formulation_basis"]["profile_id"] = json!(ps::PRESSURE_PROFILE);
    for case in source["contract_evidence"]["exact_cases"].as_array_mut().unwrap() { case["profile_mode"] = json!(ps::PRESSURE_PROFILE); }
    let mut captured = invocation();
    captured["request"]["model"]["schema_version"] = json!("0.4.0");
    captured["request"]["model"]["pressure_contract"] = json!({"version":"3.0.0", "mode":ps::PRESSURE_PROFILE});
    assert_eq!(ps::validate_in(&source, Some(&captured), ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT".into()));
}

#[test]
fn pressure_source_receipt_cannot_downgrade_to_pressure_1() {
    let mut source = v2();
    source["producer"]["semantic_contract_id"] = json!(sc::PRESSURE_ID);
    source["formulation_basis"]["profile_id"] = json!(ps::PRESSURE_PROFILE);
    assert_eq!(sc::for_source_metadata(&source).unwrap_err(), "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN");
    assert_eq!(sc::for_source(&source).unwrap_err(), "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN");
    assert_eq!(ps::validate_transport_metadata_in(&source, ps::SourceContract::PressureV3).unwrap_err(), "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN");
    assert_eq!(reader_verdict::dispatch(&source, None, None)["error"]["code"], "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN");
}

#[test]
fn pressure_source_export_is_refused_until_u2() {
    // This is only an export refusal control, never a fabricated admission.
    let source = json!({"producer":{"semantic_contract_id":ps::PRESSURE_CONTRACT_ID}});
    assert_eq!(derivative::derive_document(json!({}), &json!({}), &source, json!({}), None).unwrap_err(), derivative::PRESSURE_1_EXPORT_NOT_AVAILABLE);
    assert_eq!(derivative::validate_document(&json!({}), &source).unwrap_err(), derivative::PRESSURE_1_EXPORT_NOT_AVAILABLE);
}

#[test]
fn pressure_joined_dispatch_has_no_v2_projection_or_reseal() {
    let join = include_str!("../src/load_reference_source.rs");
    let direct = join.split("fn validate_pressure_source_evidence").nth(1).unwrap().split("fn case_pressure").next().unwrap();
    assert!(direct.contains("Method::PressureJoined"));
    for forbidden in ["project(", "project_load_reference(", "domain_hash(", "receipt_sha256"] {
        assert!(!direct.contains(forbidden), "direct joined path contains {forbidden}");
    }
    let reader = include_str!("../src/physics_source.rs");
    let direct = reader.split("pub fn validate_in(").nth(1).unwrap().split("#[cfg(test)]").next().unwrap();
    assert!(direct.contains("validate_physics_evidence_in(source, true, contract.pressure())"));
    for forbidden in ["project(", "project_load_reference(", "domain_hash(", "receipt_sha256"] {
        assert!(!direct.contains(forbidden), "raw reader contains {forbidden}");
    }
}

#[test]
fn pressure_source_binding_uses_the_reviewed_nonresident_table() {
    let bytes = include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json");
    let table = sc::verify_pressure_source_table(bytes).unwrap();
    assert_eq!(table["rows"], sc::physics_source_contract()["rows"]);
    assert_eq!(reader_verdict::binding("pressure_source"), json!({"id":ps::PRESSURE_CONTRACT_ID, "sha256":sc::PRESSURE_SOURCE_TABLE_SHA256}));
    let mut edited = bytes.to_vec(); edited.push(b'\n');
    assert_eq!(sc::verify_pressure_source_table(&edited).unwrap_err(), "SOURCE_PRESSURE_SOURCE_TABLE_HASH");
}
