//! Actual joined sparse/dense producer bytes, retained under fixtures/results.
//! This proves consumer admission and source binding, not producer authentication.
use open_pipe_stress_result_export::{derivative as d, semantic_contract as s};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

fn source(dense: bool) -> Value {
    serde_json::from_str(if dense {
        include_str!("../../../../fixtures/results/physics_connected_mechanics_dense.json")
    } else {
        include_str!("../../../../fixtures/results/physics_connected_mechanics_sparse.json")
    })
    .unwrap()
}
fn bases(raw: &Value) -> Vec<Value> {
    raw["numerical_quality"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["basis_ref"].clone())
        .collect()
}
fn derive(raw: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let request: Value = serde_json::from_str(include_str!(
        "../../../product_physics/tests/fixtures/exact_pressure_connected_request.json"
    ))
    .unwrap();
    let carrier = d::checksum(
        raw,
        "attested_headless_producer_carrier",
        d::reference("test_carrier", "retained-real-joined-output"),
    )?;
    let origin = json!({"origin_id":"physics-consumer-test", "origin_class":"attested_headless_producer", "qualification_ref":d::reference("test_fixture","not-authentication"), "authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Retained producer bytes; consumer contract test only", "actual_model_ref":d::reference("model_payload",raw["model_ref"].as_str().unwrap()),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, &request["model"], raw, origin, Some(&request))
}
#[test]
fn actual_joined_outputs_admit_preserve_and_validate_physics_derivatives() {
    for dense in [false, true] {
        let raw = source(dense);
        assert_eq!(
            s::for_source(&raw).unwrap().0["semantic_contract_id"],
            s::PHYSICS_ID
        );
        assert_eq!(
            s::numerical_use_standing(&raw, &bases(&raw)),
            "numerically_eligible"
        );
        let doc = derive(&raw).unwrap();
        assert_eq!(doc["schema_version"], "0.3.0");
        assert_eq!(
            doc["result_envelope"]["contract_evidence"],
            raw["contract_evidence"]
        );
        let reloaded: Value = serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reloaded, &raw).unwrap();
        for account in doc["result_envelope"]["row_accounting"].as_array().unwrap() {
            let row = &raw["results"][account["source_row_index"].as_u64().unwrap() as usize];
            let target = doc
                .pointer(account["target_field_path"].as_str().unwrap())
                .unwrap();
            if row["kind"].as_str().unwrap().ends_with("_v2") {
                assert_eq!(account["disposition"], "exported_quantity");
                assert_eq!(
                    target["magnitude"].as_f64().unwrap().to_bits(),
                    row["value"].as_f64().unwrap().to_bits()
                );
                assert_eq!(target["basis_ref"], row["basis_ref"]);
                assert_eq!(target["metadata"], row["metadata"]);
            }
        }
        if let Ok(dir) = std::env::var("RESULTS_RUST_CONTRACT_OUTPUT_DIR") {
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(
                std::path::Path::new(&dir).join(if dense {
                    "physics-dense.document.json"
                } else {
                    "physics-sparse.document.json"
                }),
                serde_json::to_vec_pretty(&doc).unwrap(),
            )
            .unwrap();
        }
    }
}
#[test]
fn physical_case_member_material_region_and_extrema_tampering_is_rejected() {
    let raw = source(false);
    for (path, value) in [
        ("/contract_evidence/extra", json!([])),
        ("/contract_evidence/connector", json!([{}])),
        (
            "/contract_evidence/exact_cases/0/profile_mode",
            json!("legacy_pressure_v1"),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_materials/0/E_pa",
            json!(0),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_materials/0/G_pa",
            json!(1),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_materials/0/nu",
            json!(0.5),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_materials/0/alpha_per_kelvin",
            json!(0),
        ),
        ("/contract_evidence/exact_cases/1/pipe_materials", json!([])),
        (
            "/contract_evidence/exact_cases/0/pipe_sections/0/pipe_id",
            json!("unknown"),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_sections/0/outside_diameter_m",
            json!(0.2),
        ),
        (
            "/contract_evidence/pressure/0/load_case_id",
            json!("case:six-component-load"),
        ),
        ("/contract_evidence/pressure/0/result_ids", json!([])),
        (
            "/contract_evidence/pressure/0/member_pipe_ids",
            json!(["unknown"]),
        ),
        ("/contract_evidence/pressure/0/materials/0/nu", json!(0.2)),
        ("/contract_evidence/pressure/0/geometry/0/As_m2", json!(1)),
        (
            "/contract_evidence/pressure/0/terminals/0/closure_transfer",
            json!("unknown"),
        ),
        (
            "/contract_evidence/pressure/0/terminals/0/remote_closure_excluded_from_pipe_solve",
            json!(true),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_stress_extrema/0/result_id",
            json!("missing"),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_stress_extrema/0/value_lower_pa",
            json!(1),
        ),
        (
            "/contract_evidence/exact_cases/0/pipe_stress_extrema/0/certified_gap_pa",
            json!(1e9),
        ),
        (
            "/contract_evidence/exact_cases/0/stress_maximum_coverage/complete",
            json!(false),
        ),
        (
            "/contract_evidence/exact_cases/0/pressure_rhs_assembly/method",
            json!("source-blocks-1"),
        ),
        (
            "/contract_evidence/exact_cases/0/pressure_rhs_assembly/load_case_id",
            json!("case:six-component-load"),
        ),
        (
            "/contract_evidence/exact_cases/0/pressure_rhs_assembly/cancellation_screen",
            json!(1),
        ),
        (
            "/contract_evidence/exact_cases/0/pressure_rhs_assembly/groups/0/terms/0/pipe_id",
            json!("missing"),
        ),
        (
            "/contract_evidence/exact_cases/0/pressure_rhs_assembly/groups/0/pressure_bits",
            json!("0000000000000000"),
        ),
        (
            "/numerical_quality/cases/0/basis_ref/ref_id",
            json!("missing"),
        ),
        ("/summary/max_open_formula_stress/value", json!(1)),
    ] {
        let mut bad = raw.clone();
        if path == "/contract_evidence/extra" {
            bad["contract_evidence"]["extra"] = value;
        } else {
            *bad.pointer_mut(path).unwrap() = value;
        }
        assert!(s::for_source(&bad).is_err(), "accepted mutation {path}");
        assert!(derive(&bad).is_err(), "derived mutation {path}");
    }
    for key in ["pressure", "exact_cases"] {
        let mut bad = raw.clone();
        let item = bad["contract_evidence"][key][0].clone();
        bad["contract_evidence"][key]
            .as_array_mut()
            .unwrap()
            .push(item);
        assert!(s::for_source(&bad).is_err());
    }
    let pressure_index = raw["results"]
        .as_array()
        .unwrap()
        .iter()
        .position(|r| r["kind"] == "pipe_wall_axial_force_v2")
        .unwrap();
    for (key, value) in [
        (
            "basis_ref",
            json!({"ref_type":"combination","ref_id":"not-supported"}),
        ),
        ("entity_ref", json!("missing")),
        (
            "source_result_refs",
            json!([{"ref_type":"result_value","ref_id":"cyclic"}]),
        ),
    ] {
        let mut bad = raw.clone();
        bad["results"][pressure_index][key] = value;
        assert!(s::for_source(&bad).is_err());
    }
    for key in [
        "component",
        "coordinate_system",
        "basis",
        "location",
        "sign_convention",
    ] {
        let mut bad = raw.clone();
        bad["results"][pressure_index]["metadata"][key] = json!("contradiction");
        assert!(s::for_source(&bad).is_err());
    }
    let mut missing = raw.clone();
    let id = missing["results"]
        .as_array_mut()
        .unwrap()
        .remove(pressure_index)["id"]
        .clone();
    missing["contract_evidence"]["pressure"][0]["result_ids"]
        .as_array_mut()
        .unwrap()
        .retain(|x| *x != id);
    assert!(s::for_source(&missing)
        .unwrap_err()
        .contains("PRESSURE_ROW_COVERAGE"));
}
#[test]
fn metadata_only_dispatch_cannot_qualify_missing_raw_evidence_or_reserved_methods() {
    let raw = source(false);
    let mut header = raw.clone();
    header.as_object_mut().unwrap().remove("results");
    assert!(s::for_source_metadata(&header).is_ok());
    assert!(s::for_source(&header).is_err());
    for id in [
        s::PRECISION_ID,
        "openpipestress.result_semantics/0.3.0/precision-2",
        "openpipestress.result_semantics/0.3.0/source-blocks-1",
        "openpipestress.result_semantics/0.3.0/unknown",
    ] {
        let mut bad = raw.clone();
        bad["producer"]["semantic_contract_id"] = json!(id);
        assert!(s::for_source(&bad).is_err());
    }
    for key in ["source_block_recovery", "carrier_evidence"] {
        let mut bad = raw.clone();
        bad[key] = json!({});
        assert!(s::for_source(&bad).is_err());
    }
    for status in ["sensitive", "failed", "unresolved", "not_assessed"] {
        let mut inspectable = raw.clone();
        inspectable["numerical_quality"]["status"] = json!(status);
        assert!(s::for_source(&inspectable).is_ok());
        assert_eq!(
            s::numerical_use_standing(&inspectable, &bases(&raw)),
            "needs_recompute"
        );
    }
    let mut cross_case = raw.clone();
    cross_case["numerical_quality"]["cases"][0]["evidence_refs"] =
        raw["numerical_quality"]["cases"][1]["evidence_refs"].clone();
    assert!(s::for_source(&cross_case)
        .unwrap_err()
        .contains("NUMERICAL_EVIDENCE_CASE_BINDING"));
    let mut bad = raw.clone();
    bad["numerical_quality"]["cases"][0]["evidence_refs"] = json!(["absent"]);
    assert_ne!(
        s::numerical_use_standing(&bad, &bases(&raw)),
        "numerically_eligible"
    );
}
#[test]
fn canonical_physical_evidence_is_source_bound_even_after_rehash() {
    let raw = source(false);
    let doc = derive(&raw).unwrap();
    let mut bad = doc;
    bad["result_envelope"]["contract_evidence"]["pressure"][0]["p_pa"] = json!(1);
    bad["result_envelope"]["reproducibility"]
        .as_object_mut()
        .unwrap()
        .remove("derivative_hash");
    bad["result_envelope"]["reproducibility"]["derivative_hash"] = d::checksum(
        &bad,
        "derivative_document_excludes_own_hash",
        d::reference(
            "derivative_document",
            bad["result_envelope"]["envelope_id"].as_str().unwrap(),
        ),
    )
    .unwrap();
    assert_eq!(
        d::validate_document(&bad, &raw).unwrap_err(),
        "SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH"
    );
}
#[test]
fn additive_table_retains_frozen_precision_bytes_and_semantics() {
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(include_bytes!(
                "../../../../fixtures/results/semantic_contract_v0_3_precision_1.json"
            ))
        ),
        "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e"
    );
    assert_eq!(
        &s::physics_contract()["rows"].as_array().unwrap()[..60],
        s::precision_contract()["rows"].as_array().unwrap()
    );
    assert_eq!(s::physics_contract()["source_signature_count"], 77);
    assert_eq!(s::physics_contract()["source_kind_count"], 57);
}

#[test]
fn explicit_unavailable_extrema_preserve_signed_rows_and_withhold_headline() {
    // A disclosure-path mutation, not a newly produced or qualified solve.
    let mut raw = source(false);
    let removed_id =
        raw["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["result_id"].clone();
    let pipe =
        raw["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["pipe_id"].clone();
    raw["results"]
        .as_array_mut()
        .unwrap()
        .retain(|r| r["id"] != removed_id);
    raw["contract_evidence"]["pressure"][0]["result_ids"]
        .as_array_mut()
        .unwrap()
        .retain(|id| *id != removed_id);
    raw["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"] = json!([]);
    raw["contract_evidence"]["exact_cases"][0]["stress_maximum_coverage"] =
        json!({"complete":false,"unavailable_pipe_ids":[pipe]});
    let headline = raw["summary"]["max_open_formula_stress"].take();
    s::for_source(&raw).unwrap();
    let doc = derive(&raw).unwrap();
    assert!(doc["result_envelope"]["result_sets"][0]["values"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r["source_kind"] == "pipe_axial_membrane_stress_v2"));
    raw["summary"]["max_open_formula_stress"] = headline;
    assert!(s::for_source(&raw)
        .unwrap_err()
        .contains("INCOMPLETE_STRESS_HEADLINE"));
}

#[test]
fn actual_pressure_thermal_cases_retain_distinct_pressure_only_ledgers() {
    for bytes in [
        include_str!("../../../../fixtures/results/physics_thermal_ui_mechanics_sparse.json"),
        include_str!("../../../../fixtures/results/physics_thermal_ui_mechanics_dense.json"),
    ] {
        let raw: Value = serde_json::from_str(bytes).unwrap();
        assert_eq!(
            raw["contract_evidence"]["pressure"][0]["materials"][0]["thermal_consumed"],
            true
        );
        assert_eq!(
            raw["contract_evidence"]["pressure"][0]["applied_loads"][0]["thermal_included"],
            false
        );
        s::for_source(&raw).unwrap();
        assert_eq!(
            s::numerical_use_standing(&raw, &bases(&raw)),
            "numerically_eligible"
        );
    }
}

#[test]
fn temperature_selection_must_bind_the_exact_case_material_basis() {
    let mut raw = source(false);
    raw["contract_evidence"]["pressure"][0]["materials"][0]["temperature_basis"] =
        json!({"selection":"exact_point","point_id":"missing"});
    assert!(s::for_source(&raw)
        .unwrap_err()
        .contains("TEMPERATURE_CASE_BINDING"));
}

#[test]
fn remote_closure_reaction_must_oppose_the_closure_load() {
    // A closure-shape probe only; these mutations are not new solved evidence.
    let mut raw = source(false);
    let terminal = &mut raw["contract_evidence"]["pressure"][0]["terminals"][0];
    terminal["closure_transfer"] = json!("separately_supported_or_compensated");
    terminal["remote_closure_excluded_from_pipe_solve"] = json!(true);
    terminal["pipe_cap_transfer_global_n"] = json!([0, 0, 0]);
    terminal["remote_closure_support_reaction_global_n"] = json!(terminal
        ["closure_pressure_load_global_n"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| -v.as_f64().unwrap())
        .collect::<Vec<_>>());
    s::for_source(&raw).unwrap();
    raw["contract_evidence"]["pressure"][0]["terminals"][0]
        ["remote_closure_support_reaction_global_n"][0] = json!(999);
    assert!(s::for_source(&raw)
        .unwrap_err()
        .contains("TERMINAL_REMOTE_REACTION"));
}

#[test]
fn pressure_ledger_cannot_claim_to_include_thermal_actions() {
    let mut raw = source(false);
    raw["contract_evidence"]["pressure"][0]["applied_loads"][0]["thermal_included"] = json!(true);
    assert!(s::for_source(&raw)
        .unwrap_err()
        .contains("APPLIED_THERMAL_FLAG"));
}
