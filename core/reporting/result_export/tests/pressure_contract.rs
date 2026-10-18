//! T4-U2a: reader dispatch on the exact pressure contract identity, over the
//! shared generated pressure-2 corpus (also read by the Python
//! reader). v3 admits straight families with signed p_pa; B7b retired the v2
//! contract, so a statement labelled with a retired v2 identity is refused by
//! name and v2 profile labels are refused by the current reader; unknown
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

#[test]
fn the_shared_corpus_is_admitted_by_the_pressure_1_reader_and_dispatch() {
    let cases = corpus();
    assert_eq!(cases.len(), 4);
    assert!(cases.iter().any(|(_, e)| load_state(e)) && cases.iter().any(|(_, e)| !load_state(e)));
    for (label, envelope) in &cases {
        assert_eq!(envelope["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{label}");
        s::validate_pressure_evidence(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        let (table, version) = s::for_source(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        // pressure-2 rows resolve against its own reviewed resident table (B7b, O-10).
        assert!(std::ptr::eq(table, s::pressure_rows_contract()) && version == "0.3.0", "{label}");
        let (meta, meta_version) = s::for_source_metadata(envelope).unwrap();
        assert!(std::ptr::eq(meta, table) && meta_version == "0.3.0", "{label}");
    }
    // Signed pressure is exercised: the cases without the group carry p_pa < 0.
    assert!(cases.iter().filter(|(_, e)| !load_state(e)).all(|(_, e)| e["contract_evidence"]["pressure"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["p_pa"].as_f64().unwrap() < 0.0)));
}

/// B7b: no v2 reader remains. Evidence relabelled with the retired v2 profile is
/// refused by the current reader at its profile check, and a statement whose
/// header names a retired v2 identity is refused by name at the header.
#[test]
fn retired_v2_labels_are_refused() {
    for (label, envelope) in corpus() {
        let as_v2 = relabel(&envelope, "2.0.0", "exact_straight_pressure_v2");
        let error = s::validate_pressure_evidence(&as_v2).unwrap_err();
        assert!(error.ends_with("SOURCE_PHYSICS_CASE_PROFILE"), "{label}: {error}");
        for id in s::RETIRED_EXACT_V2_IDS {
            let mut relabelled = envelope.clone();
            relabelled["producer"]["semantic_contract_id"] = json!(id);
            for refusal in [s::for_source(&relabelled).unwrap_err(), s::for_source_metadata(&relabelled).unwrap_err()] {
                assert!(refusal.starts_with(&format!("{}: ", s::SOURCE_EXACT_PRESSURE_V2_RETIRED)), "{label} {id}: {refusal}");
                assert!(refusal.contains("re-author the model as 0.6.0"), "{label} {id}: {refusal}");
            }
        }
    }
}

#[test]
fn unknown_identities_and_profiles_are_refused() {
    for (label, envelope) in corpus() {
        let mut unknown = envelope.clone();
        unknown["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/pressure-3");
        assert_eq!(s::for_source(&unknown).unwrap_err(), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "{label}");
        // Model 0.6.0: the retired pressure-1 identity is refused by name (re-run).
        let mut retired = envelope.clone();
        retired["producer"]["semantic_contract_id"] = json!(s::RETIRED_PRESSURE_1_ID);
        let refusal = s::for_source(&retired).unwrap_err();
        assert!(refusal.starts_with(&format!("{}: ", s::SOURCE_PRESSURE_1_RETIRED)) && refusal.contains("re-run"), "{label}: {refusal}");
        let mut profile = envelope.clone();
        profile["formulation_basis"]["profile_id"] = json!("exact_straight_pressure_v2");
        assert_eq!(s::for_source(&profile).unwrap_err(), "SOURCE_FORMULATION_BASIS_UNSUPPORTED", "{label}");
        let error = s::validate_pressure_evidence(&relabel(&envelope, "3.0.0", "exact_pressure_v4")).unwrap_err();
        assert!(error.ends_with("SOURCE_PHYSICS_CASE_PROFILE"), "{label}: {error}");
        let error = s::validate_pressure_evidence(&relabel(&envelope, "4.0.0", "exact_pressure_v3")).unwrap_err();
        assert!(error.ends_with("SOURCE_PHYSICS_REGION_PROFILE"), "{label}: {error}");
    }
}

/// B7b (O-10): pressure-2 has its own reviewed resident table. Its physical rows
/// are the exact source family's (the independent pressure-source-1 table, less
/// its source-basis signatures), followed by one signature per legacy connector
/// (kind, unit, component) and exactly the two U5 scalar signatures.
#[test]
fn the_pressure_1_table_is_the_physical_family_plus_the_connector_rows() {
    let pressure: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/semantic_contract_v0_3_pressure_2.json"
    ))
    .unwrap();
    let source = s::pressure_source_contract();
    assert_eq!(&pressure, s::pressure_rows_contract());
    assert_eq!(pressure["semantic_contract_id"], s::PRESSURE_ID);
    assert_eq!(pressure["formulation_profile_id"], s::PRESSURE_PROFILE);
    assert_eq!(pressure["reserved_inactive_successors"], json!([]));
    assert!(pressure.get("inherited_semantic_contract_sha256").is_none());
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
    // The vocabulary is the source family's (without its source-only maximum
    // bases) plus the connector components, frame, location and basis. T4-U6
    // (D-5, §2.4): the connector basis takes the source-only maximum basis's
    // place, ahead of the per-section-basis tokens both tables append.
    let max_basis = open_pipe_stress_result_export::physics_source::MAX_BASIS;
    let mut vocabulary = source["canonical_metadata_vocabulary"].clone();
    {
        let bases = vocabulary["basis"]["enum"].as_array_mut().unwrap();
        let slot = bases.iter().position(|b| b == max_basis).unwrap();
        bases[slot] = json!(BASIS);
        bases.retain(|b| !b.as_str().unwrap().starts_with(max_basis));
    }
    let extend = |v: &mut Value, key: &str, extra: &[&str]| {
        let list = v[key]["enum"].as_array_mut().unwrap();
        list.extend(extra.iter().map(|x| json!(x)));
    };
    extend(&mut vocabulary, "component", &current_components);
    extend(&mut vocabulary, "coordinate_system", &[LOCAL_FRAME]);
    extend(&mut vocabulary, "location", &[LOCAL]);
    assert_eq!(pressure["canonical_metadata_vocabulary"], vocabulary);
    let schema: Value = serde_json::from_str(include_str!(
        "../../../../schemas/results_pressure_2.v0.3.schema.json"
    ))
    .unwrap();
    assert_eq!(schema["$defs"]["PressureResultMetadata"]["properties"], vocabulary);
    assert_eq!(schema["$defs"]["PressureResultMetadata"]["required"],
        json!(["component", "coordinate_system", "location", "basis", "sign_convention"]));
    assert_eq!(schema["$defs"]["PressureResultMetadata"]["additionalProperties"], false);
    // The two tables share their header members; the source table adds only its
    // source-block policy. T4-U6 (§2.4): pressure-2 now also declares the
    // source-basis signature policy, with the same text.
    let (p, q) = (pressure.as_object().unwrap(), source.as_object().unwrap());
    let mut source_keys: Vec<&String> = q.keys().collect();
    source_keys.retain(|k| k.as_str() != "source_block_policy");
    assert_eq!(p.keys().collect::<Vec<_>>(), source_keys);
    assert_eq!(p["source_basis_signature_policy"], q["source_basis_signature_policy"]);
    for key in ["schema_version", "hash_vectors", "source_schema_version", "source_producer", "reserved_inactive_successors", "formulation_profile_id"] {
        assert_eq!(p[key], q[key], "{key}");
    }
    // Rows: the physical family's signatures, then exactly one per legacy
    // connector (kind, unit, component), then the two U5 signatures.
    let rows = p["rows"].as_array().unwrap();
    let signature = |r: &Value| (r["kind"].as_str().unwrap().to_string(), r["unit"].as_str().unwrap().to_string(), r["component"].as_str().map(String::from));
    // T4-U6 (§2.4): a stress-family row declares its `source_basis` (recovery
    // token and section basis), as the source table's same signature does; every
    // other ordinary row has none. Each ordinary row is a source row but for its id.
    let without_id = |r: &Value| {
        let mut r = r.clone();
        r.as_object_mut().unwrap().remove("signature_id");
        r
    };
    let split = rows.iter().position(|r| r["kind"].as_str().unwrap().starts_with("connector_")).unwrap();
    let (physical, connector) = rows.split_at(split);
    for row in physical {
        let declared = row.get("source_basis").map(|b| b.as_str().unwrap());
        let stress = open_pipe_stress_result_export::section_basis::is_stress_kind(row["kind"].as_str().unwrap());
        assert_eq!(declared.is_some(), stress, "{row}");
        if let Some(basis) = declared {
            assert!(open_pipe_stress_result_export::section_basis::split_token(basis).is_some(), "{row}");
        }
    }
    let source_rows = q["rows"].as_array().unwrap();
    let mut a: Vec<_> = physical.iter().map(signature).collect();
    let mut b: Vec<_> = source_rows.iter().map(signature).collect();
    a.sort();
    a.dedup();
    b.sort();
    b.dedup();
    assert_eq!(a, b, "the ordinary and source tables cover one physical family");
    for row in physical {
        assert!(source_rows.iter().any(|s| without_id(s) == without_id(row)), "{row}");
    }
    let connector: Vec<(String, String, String)> = connector
        .iter()
        .map(|r| {
            assert!(r["kind"].as_str().unwrap().starts_with("connector_"));
            assert_eq!(r["category"], "physical_quantity");
            (r["kind"].as_str().unwrap().into(), r["unit"].as_str().unwrap().into(), r["component"].as_str().unwrap().into())
        })
        .collect();
    let mut expected: Vec<(String, String, String)> = LEGACY_KINDS
        .iter()
        .flat_map(|(kind, unit, components, _)| {
            components.iter().map(move |c| (kind.to_string(), unit.to_string(), c.to_string()))
        })
        .collect();
    expected.extend(U5_SIGNATURES.iter().map(|(k, u, c)| (k.to_string(), u.to_string(), c.to_string())));
    assert_eq!(connector, expected);
    assert_eq!(p["source_signature_count"], rows.len());
    // T4-U6 (§2.4): 88 + 18 (015-018 become four signatures each, 068 and
    // 072-076 two each); the kinds are unchanged.
    assert_eq!(rows.len(), 106);
    assert_eq!(p["source_kind_count"], 59);
    // Signature ids are identities, not positions: stable and ascending, and
    // removed rows are not renumbered: B3's three retired rows (008, 020, 021),
    // E14's retired parity signature (037) and H-4's joint and thrust rows (038-042).
    // T4-U6: a stress signature split per (recovery token x section basis) keeps
    // its number and adds `-<token>-<basis>` (its source_basis, hyphenated).
    let mut numbers: Vec<u32> = Vec::new();
    for row in rows {
        let id = row["signature_id"].as_str().unwrap().strip_prefix("supported-source-").unwrap();
        let (number, suffix) = id.split_at(3);
        match row.get("source_basis").and_then(|b| b.as_str()) {
            Some(basis) => {
                let (token, section) = open_pipe_stress_result_export::section_basis::split_token(basis).unwrap();
                let token = token.strip_prefix("recovered_from_").unwrap_or(token);
                assert_eq!(suffix, format!("-{}-{}", token, section).replace('_', "-"), "{id}");
            }
            None => assert_eq!(suffix, "", "{id}"),
        }
        let number: u32 = number.parse().unwrap();
        if numbers.last() != Some(&number) {
            numbers.push(number);
        }
    }
    let removed = [8, 20, 21, 37, 38, 39, 40, 41, 42];
    let expected: Vec<u32> =
        (0..numbers.len() as u32 + removed.len() as u32).filter(|n| !removed.contains(n)).collect();
    assert_eq!(numbers, expected);
}
