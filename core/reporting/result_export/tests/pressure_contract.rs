//! T4-U2a: reader dispatch on the exact pressure contract identity, over the
//! shared generated pressure-1 corpus (also read by the Python and TypeScript
//! readers). v2 is unchanged and still refuses p_pa < 0; v3 admits straight
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
        assert_eq!((table["semantic_contract_id"].as_str(), version), (Some(s::PRESSURE_ID), "0.3.0"));
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
    let (pressure, physics) = (s::pressure_contract(), s::physics_contract());
    assert_eq!(pressure["rows"], physics["rows"]);
    assert_eq!(pressure["semantic_contract_id"], s::PRESSURE_ID);
    assert_eq!(pressure["formulation_profile_id"], s::PRESSURE_PROFILE);
    assert!(!pressure["reserved_inactive_successors"].as_array().unwrap().contains(&json!(s::PRESSURE_ID)));
}
