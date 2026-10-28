//! Join reader contracts over the current pressure-source-1 producer bytes
//! (0.8.0/3.0.0/exact_pressure_v3). B7b deleted the v2 contract, its reader and
//! the directional cross-read codes; no relabelled fixture is treated as a valid
//! pressure-source publication.
use open_pipe_stress_result_export::{derivative, physics_source as ps, reader_verdict, semantic_contract as sc};
use serde_json::{json, Value};
const SOURCE: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06-sparse_interactive.raw.json");
const REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json");
const LOAD_STATE: &str = include_str!("../../../../fixtures/product_preview/pressure_source/load_reference_source/n06-sparse_interactive.raw.json");
const LOAD_STATE_REQUEST: &str = include_str!("../../../../fixtures/product_preview/pressure_source/load_reference_source/n06.request.json");
fn source() -> Value { serde_json::from_str(SOURCE).unwrap() }
fn invocation() -> Value { json!({"request":serde_json::from_str::<Value>(REQUEST).unwrap(),"solver_mode":"sparse_interactive"}) }
fn load_state() -> (Value, Value) {
    (serde_json::from_str(LOAD_STATE).unwrap(),
     json!({"request":serde_json::from_str::<Value>(LOAD_STATE_REQUEST).unwrap(),"solver_mode":"sparse_interactive"}))
}

#[test]
fn selected_source_admission_and_standing() {
    let source = source();
    let captured = invocation();
    assert_eq!(ps::validate(&source, Some(&captured)), Ok(true));
    assert_eq!(ps::validate(&source, None), Ok(false));
    assert_eq!(ps::validate_transport_metadata(&source), Ok(()));
    let refs = vec![json!({"ref_type":"load_case", "ref_id":"case"})];
    assert_eq!(sc::numerical_use_standing_with_context(&source, &refs, Some(&captured)), "numerically_eligible");
    // A valid load/reference-state reading is never numerically eligible, with
    // or without its captured invocation (join standing ruling).
    let (resolved, captured) = load_state();
    assert_eq!(ps::validate(&resolved, Some(&captured)), Ok(false));
    assert_eq!(ps::validate(&resolved, None), Ok(false));
    assert_eq!(ps::validate_transport_metadata(&resolved), Ok(()));
    assert_ne!(sc::numerical_use_standing_with_context(&resolved, &refs, Some(&captured)), "numerically_eligible");
}

/// MJ5 on current inputs: the expected contract is fixed by the caller (and on the
/// invocation-bound path by the captured contract), never inferred from received
/// labels. A statement relabelled with the retired v2 profile or identity is
/// refused by the current reader's identity check before any shape check; a
/// damaged receipt under the current header is refused by its receipt shape.
#[test]
fn received_labels_never_choose_the_source_reader() {
    let source = source();
    let mut relabelled = source.clone();
    relabelled["formulation_basis"]["profile_id"] = json!("exact_straight_pressure_v2");
    for case in relabelled["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        case["profile_mode"] = json!("exact_straight_pressure_v2");
    }
    relabelled["source_block_recovery"] = json!({});
    let mut retired = source.clone();
    retired["producer"]["semantic_contract_id"] = json!("openpipestress.result_semantics/0.3.0/physics-source-1");
    for statement in [&relabelled, &retired] {
        assert_eq!(ps::validate_in(statement, None, ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_IDENTITY".into()));
        assert_eq!(ps::validate_transport_metadata_in(statement, ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_IDENTITY".into()));
        assert_eq!(reader_verdict::validate("pressure-source", statement, None, None)["error"]["code"], "PHYSICS_SOURCE_IDENTITY");
        assert_eq!(reader_verdict::transport("pressure-source", statement)["error"]["code"], "PHYSICS_SOURCE_IDENTITY");
    }
    assert!(sc::for_source(&retired).unwrap_err().starts_with(sc::SOURCE_EXACT_PRESSURE_V2_RETIRED));
    let mut damaged = source.clone();
    damaged["source_block_recovery"] = json!({});
    assert_eq!(ps::validate_in(&damaged, None, ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_RECEIPT_SHAPE".into()));
    assert_eq!(ps::validate_transport_metadata_in(&damaged, ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_RECEIPT_SHAPE".into()));
}

#[test]
fn captured_contract_fixes_the_expected_source_reader() {
    let captured = invocation();
    assert_eq!(ps::SourceContract::from_invocation(&captured), Ok(ps::SourceContract::PressureV3));
    // A captured retired v2 contract, or the current contract without its shear
    // model, or an older model version, fixes no reader: refused before the
    // received statement is read.
    for (pointer, value) in [
        ("/request/model/pressure_contract", json!({"version":"2.0.0", "mode":"exact_straight_pressure_v2"})),
        ("/request/model/pressure_contract", json!({"version":"3.0.0", "mode":ps::PRESSURE_PROFILE})),
        ("/request/model/schema_version", json!("0.4.0")),
        // O4 (§3.4): an unknown, null or non-string shear model, and an extra contract member.
        ("/request/model/pressure_contract/shear_model", json!("cowper")),
        ("/request/model/pressure_contract/shear_model", Value::Null),
        ("/request/model/pressure_contract/shear_model", json!(5)),
        ("/request/model/pressure_contract", json!({"version":"3.0.0", "mode":ps::PRESSURE_PROFILE, "shear_model":"euler_bernoulli", "shear_factor":0.5})),
    ] {
        let mut other = captured.clone();
        *other.pointer_mut(pointer).unwrap() = value;
        assert_eq!(ps::SourceContract::from_invocation(&other), Err("PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED".into()), "{pointer}");
        assert_eq!(ps::validate(&source(), Some(&other)), Err("PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED".into()), "{pointer}");
    }
    assert_eq!(ps::validate_in(&json!({}), Some(&captured), ps::SourceContract::PressureV3), Err("PHYSICS_SOURCE_IDENTITY".into()));
    // O4 (§3.4): either admitted shear model is the exact contract; a top-level
    // shear model is still refused.
    let mut timoshenko = captured.clone();
    *timoshenko.pointer_mut("/request/model/pressure_contract/shear_model").unwrap() = json!("timoshenko_annulus_energy_v1");
    assert_eq!(ps::SourceContract::from_invocation(&timoshenko), Ok(ps::SourceContract::PressureV3));
    timoshenko["request"]["model"]["shear_model"] = json!("timoshenko_annulus_energy_v1");
    assert_eq!(ps::SourceContract::from_invocation(&timoshenko), Err("PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED".into()));
}

#[test]
fn captured_model_group_requires_its_received_load_state_namespace() {
    // Contradictory context controls: neither is a valid joined publication. The
    // captured model's load/reference-state group (by presence) must match the
    // received namespace, in both directions.
    let mut captured = invocation();
    captured["request"]["model"]["reference_configurations"] = json!([]);
    assert_eq!(ps::validate(&source(), Some(&captured)), Err("PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT".into()));
    let (resolved, _) = load_state();
    assert_eq!(ps::validate(&resolved, Some(&invocation())), Err("PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT".into()));
}

#[test]
fn pressure_source_receipt_cannot_downgrade_to_pressure_1() {
    let mut source = source();
    source["producer"]["semantic_contract_id"] = json!(sc::PRESSURE_ID);
    source["formulation_basis"]["profile_id"] = json!(ps::PRESSURE_PROFILE);
    assert_eq!(sc::for_source_metadata(&source).unwrap_err(), "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN");
    assert_eq!(sc::for_source(&source).unwrap_err(), "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN");
    assert_eq!(ps::validate_transport_metadata_in(&source, ps::SourceContract::PressureV3).unwrap_err(), "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN");
    assert_eq!(reader_verdict::dispatch(&source, None, None)["error"]["code"], "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN");
}

/// The committed joined pressure-source-1 producer raw (both modes) and its request's model.
fn joined(name: &str, mode: &str) -> (Value, Value) {
    let (raw, request) = match (name, mode) {
        ("n05", "sparse_interactive") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json")),
        ("n05", "dense_scrutiny") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json")),
        ("n06", "sparse_interactive") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06-sparse_interactive.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json")),
        ("n06", "dense_scrutiny") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06-dense_scrutiny.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json")),
        ("mixed", "sparse_interactive") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json")),
        ("mixed", "dense_scrutiny") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/mixed-dense_scrutiny.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/mixed.request.json")),
        ("fields", "sparse_interactive") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/fields-sparse_interactive.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/fields.request.json")),
        ("fields", "dense_scrutiny") => (include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/fields-dense_scrutiny.raw.json"), include_str!("../../../../fixtures/product_preview/pressure_source/physics_source/fields.request.json")),
        other => panic!("no committed joined fixture {other:?}"),
    };
    (serde_json::from_str(raw).unwrap(), serde_json::from_str::<Value>(request).unwrap()["model"].clone())
}
fn derive(raw: &Value, model: &Value) -> Result<Value, String> {
    let base: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"
    ))
    .unwrap();
    let carrier = derivative::checksum(raw, "attested_headless_producer_carrier", derivative::reference("test_carrier", "committed-pressure-source-producer-output"))?;
    let origin = json!({"origin_id":"pressure-source-export-test","origin_class":"attested_headless_producer","qualification_ref":derivative::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Committed producer bytes; consumer contract test only","actual_model_ref":derivative::reference("model_payload",raw["model_ref"].as_str().unwrap_or("")),"mechanics_run_ref":derivative::reference("mechanics_run",raw["run_id"].as_str().unwrap_or("")),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    derivative::derive_document(base, model, raw, origin, None)
}
/// Changes the receipt's invocation commitment (no invocation is supplied, so the
/// reader cannot recompute it) and reseals the receipt hash: a self-consistent
/// receipt that is not the one received.
fn resealed_change(receipt: &mut Value) {
    receipt["body"]["invocation"]["value"] = json!("0".repeat(64));
    receipt["receipt_sha256"] = json!(open_pipe_stress_result_export::source_blocks::domain_hash("source_blocks_receipt_v1", &receipt["body"]).unwrap());
}

#[test]
fn pressure_source_export_binds_the_v3_receipt_after_u2() {
    // b7-plan MJ8 (D6 lapsed with T4-U2's export, #1314): the admitted export
    // carries the received v3 evidence and receipt whole, and binds them.
    let mut checked = 0;
    for name in ["n05", "n06", "mixed", "fields"] {
        for mode in ["sparse_interactive", "dense_scrutiny"] {
            let (raw, model) = joined(name, mode);
            assert_eq!(raw["producer"]["semantic_contract_id"], ps::PRESSURE_CONTRACT_ID, "{name}-{mode}");
            let doc = derive(&raw, &model).unwrap_or_else(|e| panic!("{name}-{mode}: {e}"));
            derivative::validate_document(&doc, &raw).unwrap_or_else(|e| panic!("{name}-{mode}: {e}"));
            let envelope = &doc["result_envelope"];
            assert_eq!(envelope["semantic_contract_ref"]["ref_id"], ps::PRESSURE_CONTRACT_ID);
            assert_eq!(envelope["source_block_recovery"], raw["source_block_recovery"], "{name}-{mode}");
            assert_eq!(envelope["contract_evidence"], raw["contract_evidence"], "{name}-{mode}");
            // A changed, resealed receipt in the document is refused by the binding.
            let mut changed = doc.clone();
            resealed_change(&mut changed["result_envelope"]["source_block_recovery"]);
            assert_eq!(derivative::validate_document(&changed, &raw).unwrap_err(), "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH", "{name}-{mode}");
            // A source whose receipt was changed and resealed still reads without its
            // invocation, but the document exported from the received source does not
            // bind it, and its own export carries the changed receipt.
            let mut other = raw.clone();
            resealed_change(&mut other["source_block_recovery"]);
            assert_eq!(derivative::validate_document(&doc, &other).unwrap_err(), "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH", "{name}-{mode}");
            let other_doc = derive(&other, &model).unwrap_or_else(|e| panic!("{name}-{mode}: {e}"));
            assert_eq!(other_doc["result_envelope"]["source_block_recovery"], other["source_block_recovery"]);
            assert_eq!(derivative::validate_document(&other_doc, &raw).unwrap_err(), "SOURCE_BLOCK_RECEIPT_BINDING_MISMATCH", "{name}-{mode}");
            checked += 1;
        }
    }
    assert_eq!(checked, 8);
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
fn pressure_source_binding_uses_the_reviewed_resident_table() {
    // B7b (O-10): the pressure-source table is the independent resident source
    // table; the deleted physics-source-1 table no longer supplies its rows.
    let bytes = include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_pressure_source_1.json");
    let table = sc::verify_pressure_source_table(bytes).unwrap();
    assert_eq!(&table, sc::pressure_source_contract());
    // T4-U6 (D-5, §2.4): one stress signature per (recovery token x section basis), 69 -> 88;
    // T4-U4b §4.3: + the straight derived signatures on both bases, 88 -> 94.
    assert_eq!(table["rows"].as_array().unwrap().len(), 94);
    assert_eq!(reader_verdict::binding("pressure_source"), json!({"id":ps::PRESSURE_CONTRACT_ID, "sha256":sc::PRESSURE_SOURCE_TABLE_SHA256}));
    let mut edited = bytes.to_vec(); edited.push(b'\n');
    assert_eq!(sc::verify_pressure_source_table(&edited).unwrap_err(), "SOURCE_PRESSURE_SOURCE_TABLE_HASH");
}

/// The statement with its receipt's invocation digest rebound to `captured` and the
/// receipt resealed (the receipt is `H(source_blocks_receipt_v1, body)`).
fn rebound(statement: &Value, captured: &Value) -> Value {
    use open_pipe_stress_result_export::source_blocks::domain_hash;
    let mut out = statement.clone();
    let receipt = &mut out["source_block_recovery"];
    receipt["body"]["invocation"]["value"] = json!(domain_hash("source_blocks_invocation_v1", captured).unwrap());
    receipt["receipt_sha256"] = json!(domain_hash("source_blocks_receipt_v1", &receipt["body"]).unwrap());
    out
}

/// B7b (ccb19ba0bf; b7-plan "Exact-contract fields") and O4 (DEL-04-08 Design
/// `o4-source-extension.md` §3.4): the captured exact document declares an explicit
/// length-valued, finite corrosion allowance c_a >= 0 with c_m + c_a < t on every pipe
/// section; zero is no longer required. On the 0.3-style statement and on its
/// load/reference-state twin, an absent or a null allowance, one of the wrong dimension, a
/// negative one, one equal to or above the wall, or one that with the mill tolerance
/// reaches the wall is refused (PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE), with or without
/// the receipt's invocation digest rebound to it; the unbound and transport readings of
/// the rebound statement are unchanged. An explicit zero in another length unit, rebound,
/// reads as the genuine statement does. An admitted nonzero allowance passes this check and
/// is then refused by the evidence's by-value binding (the statement's sections carry
/// c_a = 0), never by this code.
#[test]
fn captured_exact_document_requires_an_admitted_corrosion_allowance() {
    const CA: &str = "/request/model/pipe_segments/0/section/corrosion_allowance";
    const CODE: &str = "PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE";
    let (resolved, resolved_invocation) = load_state();
    for (label, statement, captured, genuine) in [
        ("0.3-style n06", source(), invocation(), Ok(true)),
        ("load/reference-state n06", resolved, resolved_invocation, Ok(false)),
    ] {
        assert_eq!(captured.pointer(CA), Some(&json!({"unit": "m", "value": 0.0})), "{label}");
        assert_eq!(ps::validate(&statement, Some(&captured)), genuine, "{label}");
        let mut millimetres = captured.clone();
        *millimetres.pointer_mut(CA).unwrap() = json!({"value": 0.0, "unit": "mm"});
        assert_eq!(ps::validate(&rebound(&statement, &millimetres), Some(&millimetres)), genuine, "{label}: 0 mm");
        let wall = captured.pointer("/request/model/pipe_segments/0/section/wall_thickness").unwrap().clone();
        assert_eq!(wall, json!({"unit": "m", "value": 0.01}), "{label}: the wall the over-wall values reach");
        for (what, value, mill) in [
            ("absent", None, None),
            ("null", Some(Value::Null), None),
            ("wrong dimension", Some(json!({"value": 0.0, "unit": "Pa"})), None),
            ("negative", Some(json!({"value": -0.001, "unit": "m"})), None),
            ("the wall", Some(json!({"value": 0.01, "unit": "m"})), None),
            ("above the wall", Some(json!({"value": 20.0, "unit": "mm"})), None),
            ("with the mill tolerance, the wall", Some(json!({"value": 0.006, "unit": "m"})), Some(json!({"value": 4.0, "unit": "mm"}))),
            // c_m + c_a rounds to t while (t - c_m) - c_a stays positive: the section sum decides.
            ("the section sum at the rounding boundary", Some(json!({"value": 0.0029999999999999996, "unit": "m"})), Some(json!({"value": 0.007, "unit": "m"}))),
            ("beside a non-length mill tolerance", Some(json!({"value": 0.0, "unit": "m"})), Some(json!({"value": 1.0, "unit": "N"}))),
            ("beside a negative mill tolerance", Some(json!({"value": 0.0, "unit": "m"})), Some(json!({"value": -0.001, "unit": "m"}))),
        ] {
            let mut other = captured.clone();
            match value {
                Some(v) => *other.pointer_mut(CA).unwrap() = v,
                None => {
                    other.pointer_mut("/request/model/pipe_segments/0/section").unwrap().as_object_mut().unwrap().remove("corrosion_allowance");
                }
            }
            if let Some(mill) = mill {
                other.pointer_mut("/request/model/pipe_segments/0/section").unwrap()["mill_tolerance"] = mill;
            }
            let ctx = format!("{label}: {what}");
            assert_eq!(ps::validate(&statement, Some(&other)), Err(CODE.into()), "{ctx}");
            let resealed = rebound(&statement, &other);
            assert_eq!(ps::validate(&resealed, Some(&other)), Err(CODE.into()), "{ctx}: rebound");
            assert_eq!(reader_verdict::validate("pressure-source", &resealed, None, Some(&other))["error"]["code"], CODE, "{ctx}");
            assert_eq!(ps::validate(&resealed, None), Ok(false), "{ctx}: unbound");
            assert_eq!(ps::validate_transport_metadata(&resealed), Ok(()), "{ctx}: transport");
            let refs = vec![json!({"ref_type": "load_case", "ref_id": "case"})];
            assert_ne!(sc::numerical_use_standing_with_context(&resealed, &refs, Some(&other)), "numerically_eligible", "{ctx}");
        }
        // Admitted values (with and without a mill tolerance) pass the allowance check. The
        // statement's sections carry c_a = 0, so its by-value section binding refuses it
        // through the reader verdict (G2's model checks); the raw reading of the 0.3-style
        // statement refuses it too, and the resolved one is never eligible (T1).
        for (what, value, mill) in [
            ("0.001 m", json!({"value": 0.001, "unit": "m"}), None),
            ("1 mm beside a 0.5 mm mill tolerance", json!({"value": 1.0, "unit": "mm"}), Some(json!({"value": 0.5, "unit": "mm"}))),
        ] {
            let mut other = captured.clone();
            *other.pointer_mut(CA).unwrap() = value;
            if let Some(mill) = mill {
                other.pointer_mut("/request/model/pipe_segments/0/section").unwrap()["mill_tolerance"] = mill;
            }
            let resealed = rebound(&statement, &other);
            let (raw, verdict) = if genuine == Ok(true) {
                (Err("PHYSICS_SOURCE_ACTUAL_SOURCE_GEOMETRY".to_string()), "PHYSICS_SOURCE_ACTUAL_SOURCE_GEOMETRY")
            } else {
                (Ok(false), "SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY")
            };
            assert_eq!(ps::validate(&resealed, Some(&other)), raw, "{label}: {what}");
            assert_eq!(reader_verdict::validate("pressure-source", &resealed, None, Some(&other))["error"]["code"], verdict, "{label}: {what}");
        }
    }
}

/// `captured` with a second pipe segment appended to its model: a copy of the first
/// (same nodes, material and section) under a new id, whose section allowance is
/// `allowance` (`None` removes the key). The first pipe keeps its explicit 0 m.
fn with_second_pipe(captured: &Value, allowance: Option<Value>) -> Value {
    let mut other = captured.clone();
    let pipes = other.pointer_mut("/request/model/pipe_segments").unwrap().as_array_mut().unwrap();
    assert_eq!(pipes.len(), 1, "the committed request has one pipe segment");
    let mut second = pipes[0].clone();
    second["id"] = json!(format!("{}-second", second["id"].as_str().unwrap()));
    let section = second["section"].as_object_mut().unwrap();
    match allowance {
        Some(v) => {
            section.insert("corrosion_allowance".into(), v);
        }
        None => {
            section.remove("corrosion_allowance");
        }
    }
    pipes.push(second);
    other
}

/// B7b review N-1: the allowance check covers every pipe section, not the first. Every
/// committed pressure-source-1 request has one pipe, so on the 0.3-style n06 statement and
/// its load/reference-state twin a second pipe is appended to the captured model and the
/// receipt's invocation digest rebound to it (`rebound`). `check_invocation` is the first
/// step of `validate_in` (after `from_invocation`, which reads no pipe), so a second pipe
/// carrying no allowance, a null one, one of the wrong dimension, a negative one or one
/// reaching the wall is refused PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE, deterministically,
/// bound and through the reader verdict, while the first pipe keeps its explicit 0 m. With
/// an explicit 0 m or (O4 §3.4) an admitted 0.001 m on the second pipe the allowance check
/// passes and the statement is refused later instead, by the receipt's coverage of the
/// captured model (SOURCE_BLOCKS_CURRENT_MODEL_SOURCE_COVERAGE: its sources name one member,
/// the model two pipes), never for the allowance; a first-section-only check would give
/// that code for every refused value above.
#[test]
fn captured_allowance_is_required_on_every_pipe_section() {
    const CODE: &str = "PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE";
    const LATER: &str = "SOURCE_BLOCKS_CURRENT_MODEL_SOURCE_COVERAGE";
    let (resolved, resolved_invocation) = load_state();
    for (label, statement, captured, genuine) in [
        ("0.3-style n06", source(), invocation(), Ok(true)),
        ("load/reference-state n06", resolved, resolved_invocation, Ok(false)),
    ] {
        assert_eq!(ps::validate(&statement, Some(&captured)), genuine, "{label}: control");
        for (what, admitted) in [("0 m", json!({"value": 0.0, "unit": "m"})), ("0.001 m", json!({"value": 0.001, "unit": "m"}))] {
            let second = with_second_pipe(&captured, Some(admitted));
            let resealed = rebound(&statement, &second);
            assert_eq!(ps::validate(&resealed, Some(&second)), Err(LATER.into()), "{label}: second pipe {what}");
            assert_eq!(reader_verdict::validate("pressure-source", &resealed, None, Some(&second))["error"]["code"], LATER, "{label}: second pipe {what}");
        }
        for (what, value) in [
            ("absent", None),
            ("null", Some(Value::Null)),
            ("wrong dimension", Some(json!({"value": 0.0, "unit": "Pa"}))),
            ("negative", Some(json!({"value": -0.001, "unit": "m"}))),
            ("the wall", Some(json!({"value": 0.01, "unit": "m"}))),
        ] {
            let other = with_second_pipe(&captured, value);
            let ctx = format!("{label}: second pipe {what}");
            assert_eq!(other.pointer("/request/model/pipe_segments/0/section/corrosion_allowance"), Some(&json!({"unit": "m", "value": 0.0})), "{ctx}");
            assert_eq!(ps::validate(&statement, Some(&other)), Err(CODE.into()), "{ctx}");
            let resealed = rebound(&statement, &other);
            assert_eq!(ps::validate(&resealed, Some(&other)), Err(CODE.into()), "{ctx}: rebound");
            assert_eq!(reader_verdict::validate("pressure-source", &resealed, None, Some(&other))["error"]["code"], CODE, "{ctx}");
            assert_eq!(ps::validate(&resealed, None), Ok(false), "{ctx}: unbound");
            let refs = vec![json!({"ref_type": "load_case", "ref_id": "case"})];
            assert_ne!(sc::numerical_use_standing_with_context(&resealed, &refs, Some(&other)), "numerically_eligible", "{ctx}");
        }
    }
}

/// B7b (ccb19ba0bf), header parity with the retired load-reference routes:
/// (1) the load/reference-state group owns no carrier namespace, so a pressure-2 or
/// pressure-source-1 statement carrying the group and `carrier_evidence` is refused at
/// the header (SOURCE_PRODUCER_CONTRACT_UNSUPPORTED; the reader-verdict dispatch's own
/// carrier guard gives SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED); (2) the pressure-source
/// route requires its evidence object before its receipt is read.
#[test]
fn header_guards_refuse_a_carrier_on_the_group_and_a_source_without_evidence() {
    const UNSUPPORTED: &str = "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED";
    let pressure_group: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json"
    ))
    .unwrap();
    let (resolved, _) = load_state();
    for (label, statement) in [("pressure-2 with the group", pressure_group), ("pressure-source-1 with the group", resolved.clone())] {
        assert!(sc::for_source(&statement).is_ok(), "{label}: control");
        let mut carrier = statement.clone();
        carrier["carrier_evidence"] = json!({});
        assert_eq!(sc::for_source(&carrier).unwrap_err(), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "{label}");
        assert_eq!(sc::for_source_metadata(&carrier).unwrap_err(), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "{label}");
        assert_eq!(reader_verdict::dispatch(&carrier, None, None)["error"]["code"], UNSUPPORTED, "{label}");
    }
    for (label, statement) in [("0.3-style n06", source()), ("load/reference-state n06", resolved)] {
        assert_eq!(reader_verdict::dispatch(&statement, None, None)["route"], "pressure_source", "{label}: control");
        for (what, evidence) in [("removed", None), ("null", Some(Value::Null)), ("an array", Some(json!([])))] {
            let mut other = statement.clone();
            match evidence {
                Some(v) => other["contract_evidence"] = v,
                None => {
                    other.as_object_mut().unwrap().remove("contract_evidence");
                }
            }
            let dispatched = reader_verdict::dispatch(&other, None, None);
            assert_eq!((dispatched["route"].as_str(), dispatched["error"]["code"].as_str()), (Some("unsupported"), Some(UNSUPPORTED)), "{label}: evidence {what}");
        }
    }
}
