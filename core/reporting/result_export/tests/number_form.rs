//! b7-plan R2 A2: the readers read numbers by value, not by their JSON form.
//! The source reader's section parameters were the one comparison that read the
//! form (`pa_per_mpa` against the float `1e6`); a statement re-serialized in JCS
//! or `JSON.stringify` form writes it `1000000`. A wrong value is still refused.
//! B7b: read over the current pressure-source-1 and pressure-2 statements.
use open_pipe_stress_result_export::{physics_source as ps, reader_verdict as rv, source_blocks::domain_hash};
use serde_json::{json, Value};

fn read(path: &str) -> Value {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../");
    serde_json::from_str(&std::fs::read_to_string(format!("{root}{path}")).unwrap()).unwrap()
}
fn genuine() -> (Value, Value) {
    let request = read("fixtures/product_preview/pressure_source/physics_source/n05.request.json");
    let source = read("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json");
    (source, json!({"request": request, "solver_mode": "sparse_interactive"}))
}
/// The first section-stress check's parameters set to `value`, the receipt resealed
/// so that the check is reached.
fn with_parameter(source: &Value, key: &str, value: Value) -> Value {
    with_parameter_on(source, &[0], key, value)
}
/// The parameters of the listed section-stress checks set to `value`, resealed.
fn with_parameter_on(source: &Value, checks: &[usize], key: &str, value: Value) -> Value {
    let mut doc = source.clone();
    for &index in checks {
        doc["source_block_recovery"]["body"]["cases"][0]["section_stress_checks"][index]["parameters"][key] = value.clone();
    }
    let mut publication = doc.clone();
    publication.as_object_mut().unwrap().remove("source_block_recovery");
    doc["source_block_recovery"]["body"]["publication_sha256"] =
        json!(domain_hash("source_blocks_publication_v1", &publication).unwrap());
    doc["source_block_recovery"]["receipt_sha256"] =
        json!(domain_hash("source_blocks_receipt_v1", &doc["source_block_recovery"]["body"]).unwrap());
    doc
}

#[test]
fn section_parameters_compare_by_value() {
    let (source, invocation) = genuine();
    let parameters = &source["source_block_recovery"]["body"]["cases"][0]["section_stress_checks"][0]["parameters"];
    assert!(parameters["pa_per_mpa"].is_f64(), "published as 1000000.0");
    assert_eq!(ps::validate(&source, Some(&invocation)), Ok(true));
    // The JCS form of the whole statement, and the integer form of this member alone.
    let jcs: Value = serde_json::from_str(&open_pipe_stress_canonical_json::canonical_json(&source)).unwrap();
    assert!(jcs["source_block_recovery"]["body"]["cases"][0]["section_stress_checks"][0]["parameters"]["pa_per_mpa"].is_u64());
    assert_eq!(ps::validate(&jcs, Some(&invocation)), Ok(true));
    // T4-U6 (D-5 §2.5, §5): the member's check is followed by its reduced
    // twin's, which binds the same parameters; the member is the pair, so its
    // integer form is written on both, as a value-preserving re-serializer does.
    let checks = &source["source_block_recovery"]["body"]["cases"][0]["section_stress_checks"];
    assert_eq!(
        checks[1]["result_id"].as_str().unwrap(),
        checks[0]["result_id"].as_str().unwrap().replace(":nominal-section", ":reduced-section"),
        "the second check is the first one's reduced twin"
    );
    let integer = with_parameter_on(&source, &[0, 1], "pa_per_mpa", json!(1_000_000));
    assert_eq!(ps::validate(&integer, Some(&invocation)), Ok(true));
    assert_eq!(rv::validate("pressure-source", &integer, None, Some(&invocation))["ok"], true);
    // Wrong values are refused as before.
    // Wrong values are refused as before: the receipt schema holds `pa_per_mpa` and
    // the members, and the section check the copied section values.
    for (key, value, code) in [
        ("pa_per_mpa", json!(100_000), "PHYSICS_SOURCE_RECEIPT_SHAPE"),
        ("pa_per_mpa", json!(1e6 + 1.0), "PHYSICS_SOURCE_RECEIPT_SHAPE"),
        ("pa_per_mpa", json!("1000000"), "PHYSICS_SOURCE_RECEIPT_SHAPE"),
        ("pa_per_mpa", json!(null), "PHYSICS_SOURCE_RECEIPT_SHAPE"),
        ("invented", json!(1), "PHYSICS_SOURCE_RECEIPT_SHAPE"),
        ("area_m2", json!(f64::from_bits(parameters["area_m2"].as_f64().unwrap().to_bits() + 1)), "PHYSICS_SOURCE_STRESS_SECTION_PARAMETERS"),
        ("torsion_radius_m", json!(0.1000000001), "PHYSICS_SOURCE_STRESS_SECTION_PARAMETERS"),
    ] {
        let wrong = with_parameter(&source, key, value.clone());
        assert_eq!(ps::validate(&wrong, Some(&invocation)), Err(code.to_string()), "{key} = {value}");
    }
    let mut missing = with_parameter(&source, "pa_per_mpa", json!(1e6));
    missing["source_block_recovery"]["body"]["cases"][0]["section_stress_checks"][0]["parameters"]
        .as_object_mut()
        .unwrap()
        .remove("pa_per_mpa");
    let missing = with_parameter(&missing, "area_m2", parameters["area_m2"].clone());
    assert!(ps::validate(&missing, Some(&invocation)).is_err());
}

/// A result document derived from a statement, written in `form`.
fn derived(raw: &Value, request: &Value) -> Value {
    use open_pipe_stress_result_export::derivative as d;
    let base = read("fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json");
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "a2-number-form")).unwrap();
    let origin = json!({"origin_id":"a2-number-form", "origin_class":"attested_headless_producer", "qualification_ref":d::reference("test_fixture","not-authentication"), "authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Committed producer bytes; consumer contract test only", "actual_model_ref":d::reference("model_payload",raw["model_ref"].as_str().unwrap()),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, &request["model"], raw, origin, Some(request)).unwrap()
}

/// A result document's copied members bind the source by value (owner,
/// 2026-10-11: "Keep envelope, hash by value."): the document written in JCS form
/// still validates against the statement as published, and a changed value is
/// still refused.
#[test]
fn copied_members_bind_by_value() {
    use open_pipe_stress_result_export::derivative as d;
    let jcs = |v: &Value| -> Value { serde_json::from_str(&open_pipe_stress_canonical_json::canonical_json(v)).unwrap() };
    for (raw, request) in [
        (
            read("fixtures/results/pressure_connected_mechanics_sparse.json"),
            read("core/product_physics/tests/fixtures/exact_pressure_connected_request.json"),
        ),
        (
            read("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"),
            read("fixtures/product_preview/pressure_source/physics_source/n05.request.json"),
        ),
    ] {
        let doc = derived(&raw, &request);
        let copied = jcs(&doc);
        assert_ne!(copied, doc, "the JCS form writes some number differently");
        assert_eq!(d::validate_document(&copied, &raw), Ok(()));
        // The statement in JCS form against the document as derived.
        assert_eq!(d::validate_document(&doc, &jcs(&raw)), Ok(()));
        // A changed copied value is refused.
        let mut wrong = copied.clone();
        let evidence = &mut wrong["result_envelope"]["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["ro_m"];
        *evidence = json!(evidence.as_f64().unwrap() * (1.0 + f64::EPSILON));
        assert!(d::validate_document(&wrong, &raw).is_err());
    }
}
