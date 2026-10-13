//! T4-U2a: reader dispatch on the exact pressure contract identity, over the
//! shared generated pressure-1 corpus (also read by the Python
//! reader). v2 is unchanged and still refuses p_pa < 0; v3 admits straight
//! families with signed p_pa; neither is read as the other; unknown
//! identities are refused.
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

fn corpus() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"
    ))
    .unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| (case["label"].as_str().unwrap().to_string(), case["envelope"].clone()))
        .collect()
}

fn load_state(envelope: &Value) -> bool {
    envelope["contract_evidence"].get("load_reference_states").is_some()
}

/// The contract strings rewritten to `(version, mode)`; nothing else changes.
fn relabel(envelope: &Value, version: &str, mode: &str) -> Value {
    let mut out = envelope.clone();
    for case in out["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        case["profile_mode"] = json!(mode);
    }
    for region in out["contract_evidence"]["pressure"].as_array_mut().unwrap() {
        region["profile_version"] = json!(version);
        region["profile_mode"] = json!(mode);
    }
    out
}

fn v2_reader(envelope: &Value) -> Result<(), String> {
    if load_state(envelope) {
        s::validate_load_reference_evidence(envelope)
    } else {
        s::validate_physics_evidence(envelope)
    }
}

#[test]
fn the_shared_corpus_is_admitted_by_the_pressure_1_reader_and_dispatch() {
    let cases = corpus();
    assert_eq!(cases.len(), 4);
    assert!(cases.iter().any(|(_, e)| load_state(e)) && cases.iter().any(|(_, e)| !load_state(e)));
    for (label, envelope) in &cases {
        assert_eq!(envelope["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{label}");
        s::validate_pressure_evidence(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        let (table, version) = s::for_source(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        // pressure-1 rows resolve against physics-1's resident table (no
        // pressure-1 static in the reader; T4-RV14 F1).
        assert!(std::ptr::eq(table, s::pressure_rows_contract()) && version == "0.3.0", "{label}");
        let (meta, meta_version) = s::for_source_metadata(envelope).unwrap();
        assert!(std::ptr::eq(meta, table) && meta_version == "0.3.0", "{label}");
    }
    // Signed pressure is exercised: the 0.3.0 cases carry p_pa < 0.
    assert!(cases.iter().filter(|(_, e)| !load_state(e)).all(|(_, e)| e["contract_evidence"]["pressure"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["p_pa"].as_f64().unwrap() < 0.0)));
}

#[test]
fn neither_contract_is_read_as_the_other() {
    for (label, envelope) in corpus() {
        let error = v2_reader(&envelope).unwrap_err();
        assert!(error.ends_with(s::V3_READ_AS_V2), "{label}: {error}");
        let as_v2 = relabel(&envelope, "2.0.0", "exact_straight_pressure_v2");
        let error = s::validate_pressure_evidence(&as_v2).unwrap_err();
        assert!(error.ends_with(s::V2_READ_AS_V3), "{label}: {error}");
        // Relabelled v2 evidence read by the v2 reader: v2 is unchanged.
        let v2 = v2_reader(&as_v2);
        if load_state(&envelope) {
            v2.unwrap_or_else(|e| panic!("{label}: {e}"));
        } else {
            // p_pa < 0 is still refused under v2.
            assert!(v2.unwrap_err().ends_with("SOURCE_PHYSICS_PRESSURE_RANGE"), "{label}");
        }
    }
}

#[test]
fn unknown_identities_and_profiles_are_refused() {
    for (label, envelope) in corpus() {
        let mut unknown = envelope.clone();
        unknown["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/pressure-2");
        assert_eq!(s::for_source(&unknown).unwrap_err(), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "{label}");
        let mut profile = envelope.clone();
        profile["formulation_basis"]["profile_id"] = json!("exact_straight_pressure_v2");
        assert_eq!(s::for_source(&profile).unwrap_err(), "SOURCE_FORMULATION_BASIS_UNSUPPORTED", "{label}");
        let error = s::validate_pressure_evidence(&relabel(&envelope, "3.0.0", "exact_pressure_v4")).unwrap_err();
        assert!(error.ends_with("SOURCE_PHYSICS_CASE_PROFILE"), "{label}: {error}");
        let error = s::validate_pressure_evidence(&relabel(&envelope, "4.0.0", "exact_pressure_v3")).unwrap_err();
        assert!(error.ends_with("SOURCE_PHYSICS_REGION_PROFILE"), "{label}: {error}");
        // pressure-1 relabelled as physics-1 is refused by the v2 dispatch.
        let mut relabelled = envelope.clone();
        relabelled["producer"]["semantic_contract_id"] = json!(if load_state(&envelope) { s::LOAD_REFERENCE_ID } else { s::PHYSICS_ID });
        assert!(s::for_source(&relabelled).is_err(), "{label}");
    }
}

#[test]
fn the_pressure_1_skeleton_keeps_physics_1_rows_under_its_own_identity() {
    let pressure: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/semantic_contract_v0_3_pressure_1.json"
    ))
    .unwrap();
    let physics = s::physics_contract();
    assert!(std::ptr::eq(s::pressure_rows_contract(), physics));
    assert_eq!(pressure["semantic_contract_id"], s::PRESSURE_ID);
    assert_eq!(pressure["formulation_profile_id"], s::PRESSURE_PROFILE);
    assert!(!pressure["reserved_inactive_successors"].as_array().unwrap().contains(&json!(s::PRESSURE_ID)));
    // The ordinary table stays staged, nonresident and unreviewed through
    // batch 5, the join and B7a; B7b D4 owns its synchronization and binding.
    // Preserve its complete pre-U5 extension, independently of the current
    // reader/schema's exact two-family delta. No current row is filtered out.
    let own = [
        "semantic_contract_id",
        "formulation_profile_id",
        "inherited_semantic_contract_sha256",
        "reserved_inactive_successors",
        "contract_evidence_policy",
        "supported_profile_limitations",
        "rows",
        "source_signature_count",
        "source_kind_count",
        "canonical_metadata_vocabulary",
    ];
    let (p, q) = (pressure.as_object().unwrap(), physics.as_object().unwrap());
    // Immutable pre-U5 constants from connector_evidence.rs at
    // f79493a7215f798e5d0328d52e5f461ee9065eff, not inferred from today's reader.
    const LEGACY_KINDS: [(&str, &str, [&str; 3], &[&str]); 6] = [
        ("connector_generalized_translation_v1", "m", ["qt_x", "qt_y", "qt_z"], &["connector_local"]),
        ("connector_generalized_rotation_v1", "rad", ["qr_x", "qr_y", "qr_z"], &["connector_local"]),
        ("connector_generalized_force_v1", "N", ["gt_x", "gt_y", "gt_z"], &["connector_local"]),
        ("connector_generalized_moment_v1", "N*m", ["gr_x", "gr_y", "gr_z"], &["connector_local"]),
        ("connector_endpoint_force_v1", "N", ["Fx", "Fy", "Fz"], &["end_i", "end_j"]),
        ("connector_endpoint_moment_v1", "N*m", ["Mx", "My", "Mz"], &["end_i", "end_j"]),
    ];
    const LEGACY_COMPONENTS: [&str; 12] = [
        "qt_x", "qt_y", "qt_z", "qr_x", "qr_y", "qr_z", "gt_x", "gt_y", "gt_z", "gr_x", "gr_y", "gr_z",
    ];
    const LOCAL_FRAME: &str = "connector_axes_q";
    const LOCAL: &str = "connector_local";
    const BASIS: &str = "objective_connector_v1;symmetric_midpoint_small_rotation_v1";
    // U5a Design sections 4-5: exactly two scalar signatures. The reader's
    // fixed-width array padding is membership storage, not three table rows.
    const U5_SIGNATURES: [(&str, &str, &str); 2] = [
        ("connector_pressure_force_v1", "N", "pe_x"),
        ("connector_effective_axial_force_v1", "N", "s_x"),
    ];
    let mut current_kinds = LEGACY_KINDS.to_vec();
    for (kind, unit, component) in U5_SIGNATURES {
        current_kinds.push((kind, unit, [component; 3], &[LOCAL]));
    }
    assert_eq!(s::CONNECTOR_ROW_KINDS.as_slice(), current_kinds.as_slice());
    let mut current_components = LEGACY_COMPONENTS.to_vec();
    current_components.extend(["pe_x", "s_x"]);
    assert_eq!(s::CONNECTOR_VOCABULARY_COMPONENTS.as_slice(), current_components.as_slice());
    assert_eq!(s::CONNECTOR_VOCABULARY_COORDINATE_SYSTEMS, [LOCAL_FRAME]);
    assert_eq!(s::CONNECTOR_VOCABULARY_LOCATIONS, [LOCAL]);
    assert_eq!(s::CONNECTOR_ROW_BASIS, BASIS);
    // The staged table's entire vocabulary is physics-1's plus the legacy
    // components, frame, location and basis, with no extra or missing member.
    let mut vocabulary = q["canonical_metadata_vocabulary"].clone();
    let extend = |v: &mut Value, key: &str, extra: &[&str]| {
        let list = v[key]["enum"].as_array_mut().unwrap();
        list.extend(extra.iter().map(|x| json!(x)));
    };
    extend(&mut vocabulary, "component", &LEGACY_COMPONENTS);
    extend(&mut vocabulary, "coordinate_system", &[LOCAL_FRAME]);
    extend(&mut vocabulary, "location", &[LOCAL]);
    extend(&mut vocabulary, "basis", &[BASIS]);
    assert_eq!(p["canonical_metadata_vocabulary"], vocabulary);
    let mut current_vocabulary = vocabulary.clone();
    extend(&mut current_vocabulary, "component", &["pe_x", "s_x"]);
    let schema: Value = serde_json::from_str(include_str!(
        "../../../../schemas/results_pressure_1.v0.3.schema.json"
    ))
    .unwrap();
    assert_eq!(schema["$defs"]["PressureResultMetadata"]["properties"], current_vocabulary);
    assert_eq!(schema["$defs"]["PressureResultMetadata"]["required"],
        json!(["component", "coordinate_system", "location", "basis", "sign_convention"]));
    assert_eq!(schema["$defs"]["PressureResultMetadata"]["additionalProperties"], false);
    assert_eq!(p.keys().collect::<Vec<_>>(), q.keys().collect::<Vec<_>>());
    for (key, value) in p {
        if !own.contains(&key.as_str()) {
            assert_eq!(value, &q[key], "{key}");
        }
    }
    // T4-U3: the rows are physics-1's, in order, then exactly one signature
    // per legacy connector (kind, unit, component), not the reader's padded
    // scalar entries. B7b will add exactly the two U5_SIGNATURES above.
    let (rows, inherited) = (p["rows"].as_array().unwrap(), q["rows"].as_array().unwrap());
    assert_eq!(&rows[..inherited.len()], &inherited[..]);
    let connector: Vec<(String, String, String)> = rows[inherited.len()..]
        .iter()
        .map(|r| {
            assert!(r["kind"].as_str().unwrap().starts_with("connector_"));
            assert_eq!(r["category"], "physical_quantity");
            (r["kind"].as_str().unwrap().into(), r["unit"].as_str().unwrap().into(), r["component"].as_str().unwrap().into())
        })
        .collect();
    let expected: Vec<(String, String, String)> = LEGACY_KINDS
        .iter()
        .flat_map(|(kind, unit, components, _)| {
            components.iter().map(move |c| (kind.to_string(), unit.to_string(), c.to_string()))
        })
        .collect();
    assert_eq!(connector, expected);
    assert_eq!(p["source_signature_count"], rows.len());
    assert_eq!(p["source_kind_count"], q["source_kind_count"].as_u64().unwrap() + 6);
    for (i, r) in rows.iter().enumerate() {
        assert_eq!(r["signature_id"], format!("supported-source-{i:03}"));
    }
}
