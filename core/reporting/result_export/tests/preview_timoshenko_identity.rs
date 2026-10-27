//! DEL-05-03 Design `pf-timoshenko.md` §4: the pressure-free Timoshenko identity
//! (`preview-physics-timoshenko-1`) beside preview-physics-2 on the preview-physics
//! route. Model-free readers admit either complete identity (table, profile and
//! limitation array together) and refuse a mixed array; model-bound readers refuse a
//! crossed pair; bindings are keyed by `semantic_contract_id`.
//!
//! Apart from the product's own envelopes (the last test), the Timoshenko sources here are
//! **transformed stand-ins**: committed Euler–Bernoulli producer envelopes with the
//! identity, the profile and limitation 0 swapped (`timoshenko_of`). Their rows are
//! Euler–Bernoulli values; these tests read identity, not mechanics.
use open_pipe_stress_result_export::{
    derivative as d, reader_verdict as rv, retained_precision as rp, semantic_contract as s,
};
use serde_json::{json, Value};

const SOURCES: [(&str, &str); 4] = [
    ("connected_sparse", include_str!("../../../../fixtures/results/preview_physics_connected_sparse.json")),
    ("connected_dense", include_str!("../../../../fixtures/results/preview_physics_connected_dense.json")),
    ("invented_sparse", include_str!("../../../../fixtures/results/preview_physics_invented_sparse.json")),
    ("invented_dense", include_str!("../../../../fixtures/results/preview_physics_invented_dense.json")),
];
const CONNECTED_MODEL: &str = include_str!("../../../../fixtures/model_operations/precision_connected_ui_model.json");
const INVENTED_MODEL: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");
const TIMOSHENKO_TABLE: &[u8] =
    include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json");
const PREVIEW_TABLE: &[u8] = include_bytes!("../../../../fixtures/results/semantic_contract_v0_3_preview_physics_2.json");

fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}
fn model_for(name: &str, shear_model: Option<&str>) -> Value {
    let mut model = parse(if name.starts_with("connected") { CONNECTED_MODEL } else { INVENTED_MODEL });
    if let Some(value) = shear_model {
        model["model_settings"] = json!({"shear_model": value});
    }
    model
}
/// A transformed stand-in: the Euler–Bernoulli envelope under the Timoshenko identity.
fn timoshenko_of(eb: &Value) -> Value {
    let mut source = eb.clone();
    let table = s::preview_physics_timoshenko_contract();
    source["producer"]["semantic_contract_id"] = json!(s::PREVIEW_PHYSICS_TIMOSHENKO_ID);
    source["formulation_basis"]["profile_id"] = json!(s::PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    source["formulation_basis"]["limitations"] = table["supported_profile_limitations"].clone();
    source
}
fn bases(raw: &Value) -> Vec<Value> {
    raw["numerical_quality"]["cases"].as_array().unwrap().iter().map(|c| c["basis_ref"].clone()).collect()
}
fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
fn derive(raw: &Value) -> Result<Value, String> {
    let base = parse(include_str!("../../../../fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json"));
    let model = raw["model_ref"].as_str().unwrap();
    let carrier = d::checksum(raw, "attested_headless_producer_carrier", d::reference("test_carrier", "preview-physics"))?;
    let origin = json!({"origin_id":"preview-timoshenko-consumer-test","origin_class":"attested_headless_producer","qualification_ref":d::reference("test_fixture","not-authentication"),"authentic_producer_available":true,"received_carrier_checksum":carrier,"original_producer_checksum":carrier,"origin_limit":"Transformed stand-in; consumer contract test only","actual_model_ref":d::reference("model_payload",model),"mechanics_run_ref":d::reference("mechanics_run",raw["run_id"].as_str().unwrap()),"request_model_ref":null,"request_run_ref":null,"request_alias_disclosure":null});
    d::derive_document(base, &json!({"project":{"id":model}}), raw, origin, None)
}

#[test]
fn the_timoshenko_table_is_pinned_and_preview_physics_2_is_unchanged() {
    assert_eq!(sha256(TIMOSHENKO_TABLE), s::PREVIEW_PHYSICS_TIMOSHENKO_SHA256);
    assert_eq!(sha256(PREVIEW_TABLE), s::PREVIEW_PHYSICS_SHA256);
    assert_eq!(s::PREVIEW_PHYSICS_SHA256, "5a6630d9157ad1f4232c558d0cd57414677add8d1c95cc0a5d19a2579fb3a05b");
    let (eb, ti) = (s::preview_physics_contract(), s::preview_physics_timoshenko_contract());
    let differing: Vec<&String> =
        eb.as_object().unwrap().keys().filter(|k| eb[k.as_str()] != ti[k.as_str()]).collect();
    assert_eq!(differing, ["formulation_profile_id", "semantic_contract_id", "supported_profile_limitations"]);
    let (el, tl) = (eb["supported_profile_limitations"].as_array().unwrap(), ti["supported_profile_limitations"].as_array().unwrap());
    assert_eq!(el[1..], tl[1..]);
    assert!(tl[0].as_str().unwrap().contains("timoshenko_annulus_energy_v1"));
    let mut tampered = TIMOSHENKO_TABLE.to_vec();
    tampered.push(b'\n');
    assert_eq!(s::verify_preview_physics_timoshenko_table(&tampered).unwrap_err(), "SOURCE_PREVIEW_PHYSICS_TIMOSHENKO_TABLE_HASH");
    assert!(s::is_fresh_identity(s::PREVIEW_PHYSICS_TIMOSHENKO_ID) && s::is_fresh_identity(s::PREVIEW_PHYSICS_ID));
}

/// E14's retired parity kind, for the Timoshenko table. This sits here rather than in
/// `semantic_contract.rs`'s closed `e14_tests` module, whose bytes the retained-memory law
/// pins (pf-timoshenko.md §4).
#[test]
fn e14_the_timoshenko_table_does_not_carry_the_retired_parity_kind() {
    let row = json!({"kind":"sparse_live_path_dense_parity_relative_delta","unit":"unitless",
        "entity_ref":"solver:sparse_direct","value":0.0,
        "metadata":{"component":"sparse_live_path","coordinate_system":"reduced_system","location":"load_case",
            "basis":"retired","sign_convention":"retired"}});
    let table = s::preview_physics_timoshenko_contract();
    assert!(table["rows"].as_array().unwrap().iter().all(|r| r["kind"] != row["kind"]));
    assert_eq!(s::signature_in(table, &row).ok().flatten(), None);
}

#[test]
fn model_free_readers_admit_either_complete_identity() {
    for (name, text) in SOURCES {
        let eb = parse(text);
        for (source, id, table_id) in [
            (eb.clone(), s::PREVIEW_PHYSICS_ID, s::PREVIEW_PHYSICS_ID),
            (timoshenko_of(&eb), s::PREVIEW_PHYSICS_TIMOSHENKO_ID, s::PREVIEW_PHYSICS_TIMOSHENKO_ID),
        ] {
            let (table, version) = s::for_source(&source).unwrap_or_else(|e| panic!("{name} {id}: {e}"));
            assert_eq!((table["semantic_contract_id"].as_str(), version), (Some(table_id), "0.3.0"), "{name}");
            s::validate_preview_physics_evidence(&source).unwrap_or_else(|e| panic!("{name} {id}: {e}"));
            rp::validate_preview_physics_transport_metadata(&source).unwrap_or_else(|e| panic!("{name} {id}: {e}"));
            assert_eq!(rv::transport("preview-physics", &source), json!({"ok": true}), "{name}");
            assert_eq!(rv::validate("preview-physics", &source, None, None), json!({"ok": true}), "{name}");
            assert_eq!(s::numerical_use_standing(&source, &bases(&source)), "numerically_eligible", "{name} {id}");
            let d = rv::dispatch(&source, None, None);
            assert_eq!(d["route"], "preview_physics", "{name}");
            let sha = if id == s::PREVIEW_PHYSICS_ID { s::PREVIEW_PHYSICS_SHA256 } else { s::PREVIEW_PHYSICS_TIMOSHENKO_SHA256 };
            assert_eq!(d["binding"], json!({"id": id, "sha256": sha}), "{name}");
        }
    }
}

#[test]
fn a_mixed_identity_is_refused_by_every_model_free_reader() {
    let eb = parse(SOURCES[0].1);
    let ti = timoshenko_of(&eb);
    let eb_lim = s::preview_physics_contract()["supported_profile_limitations"].clone();
    let ti_lim = s::preview_physics_timoshenko_contract()["supported_profile_limitations"].clone();
    let mut mixed = Vec::new();
    // The Timoshenko identity with Euler–Bernoulli's profile, limitations, or limitation 0.
    let mut m = ti.clone();
    m["formulation_basis"]["profile_id"] = json!(s::PREVIEW_PHYSICS_PROFILE);
    mixed.push(("ti id, eb profile", m));
    let mut m = ti.clone();
    m["formulation_basis"]["limitations"] = eb_lim.clone();
    mixed.push(("ti id, eb limitations", m));
    let mut m = ti.clone();
    m["formulation_basis"]["limitations"][0] = eb_lim[0].clone();
    mixed.push(("ti id, eb limitation 0", m));
    // preview-physics-2 with the Timoshenko profile, limitations, or limitation 0.
    let mut m = eb.clone();
    m["formulation_basis"]["profile_id"] = json!(s::PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    mixed.push(("eb id, ti profile", m));
    let mut m = eb.clone();
    m["formulation_basis"]["limitations"] = ti_lim.clone();
    mixed.push(("eb id, ti limitations", m));
    let mut m = eb.clone();
    m["formulation_basis"]["profile_id"] = json!(s::PREVIEW_PHYSICS_TIMOSHENKO_PROFILE);
    m["formulation_basis"]["limitations"] = ti_lim.clone();
    mixed.push(("eb id, ti profile and limitations", m));
    for (label, source) in mixed {
        let raw = s::validate_preview_physics_evidence(&source).unwrap_err();
        assert_eq!(raw, "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS", "{label}");
        let transport = rp::validate_preview_physics_transport_metadata(&source).unwrap_err();
        assert_eq!(transport, "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: formulation profile or limitations", "{label}");
        assert!(s::for_source(&source).is_err(), "{label}");
        assert_eq!(s::numerical_use_standing(&source, &bases(&source)), "unsupported", "{label}");
        assert_eq!(rv::validate("preview-physics", &source, None, None)["ok"], false, "{label}");
    }
}

#[test]
fn model_bound_readers_refuse_a_crossed_pair() {
    for (name, text) in SOURCES {
        let eb = parse(text);
        let ti = timoshenko_of(&eb);
        let (absent, explicit_eb, timoshenko) =
            (model_for(name, None), model_for(name, Some("euler_bernoulli")), model_for(name, Some("timoshenko_annulus_energy_v1")));
        let ok = json!({"ok": true});
        assert_eq!(rv::validate("preview-physics", &eb, Some(&absent), None), ok, "{name}");
        assert_eq!(rv::validate("preview-physics", &eb, Some(&explicit_eb), None), ok, "{name}");
        assert_eq!(rv::validate("preview-physics", &ti, Some(&timoshenko), None), ok, "{name}");
        for (source, model) in [(&ti, &absent), (&ti, &explicit_eb), (&eb, &timoshenko)] {
            let out = rv::validate("preview-physics", source, Some(model), None);
            assert_eq!(out["error"]["code"], rv::SHEAR_MODEL_IDENTITY, "{name}: {out}");
            let standing = rv::standing(source, Some(model), None);
            assert_eq!(standing["eligible"], false, "{name}");
            assert!(standing["findings"][0].as_str().unwrap().starts_with(rv::SHEAR_MODEL_IDENTITY), "{name}: {standing}");
        }
        assert_eq!(rv::standing(&ti, Some(&timoshenko), None)["findings"], json!([]), "{name}");
        // A model value the producer refuses binds neither identity.
        let unknown = model_for(name, Some("cowper"));
        for source in [&eb, &ti] {
            assert_eq!(rv::validate("preview-physics", source, Some(&unknown), None)["error"]["code"], rv::SHEAR_MODEL_IDENTITY);
        }
    }
}

#[test]
fn the_derivative_carries_the_timoshenko_identity_and_its_evidence() {
    for (name, text) in SOURCES {
        let ti = timoshenko_of(&parse(text));
        let doc = derive(&ti).unwrap_or_else(|e| panic!("{name}: {e}"));
        let envelope = &doc["result_envelope"];
        assert_eq!(envelope["producer"]["semantic_contract_id"], s::PREVIEW_PHYSICS_TIMOSHENKO_ID);
        assert_eq!(envelope["semantic_contract_ref"], json!({"ref_type": "semantic_contract", "ref_id": s::PREVIEW_PHYSICS_TIMOSHENKO_ID}));
        assert_eq!(envelope["formulation_basis"], ti["formulation_basis"]);
        assert_eq!(envelope["contract_evidence"], ti["contract_evidence"], "{name}");
        let (table, _) = s::for_export_metadata(envelope).unwrap();
        assert_eq!(table["semantic_contract_id"], s::PREVIEW_PHYSICS_TIMOSHENKO_ID);
        let reloaded: Value = serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reloaded, &ti).unwrap_or_else(|e| panic!("{name}: {e}"));
        // The document cannot be relabelled under the other identity.
        let mut relabelled = reloaded.clone();
        relabelled["result_envelope"]["semantic_contract_ref"]["ref_id"] = json!(s::PREVIEW_PHYSICS_ID);
        assert!(d::validate_document(&relabelled, &ti).is_err(), "{name}");
    }
}

#[test]
fn a_blocked_timoshenko_envelope_is_read_under_its_identity() {
    let mut blocked = timoshenko_of(&parse(SOURCES[2].1));
    blocked["status"]["mechanics"] = json!("MODEL_INCOMPLETE");
    blocked["results"] = json!([]);
    blocked["summary"]["max_open_formula_stress"] = Value::Null;
    blocked["summary"]["max_displacement"] = Value::Null;
    blocked["summary"]["component_stress_modifier_count"] = json!(0);
    blocked["contract_evidence"] = json!({"preview_cases": [], "combination_gates": []});
    blocked["diagnostics"] = json!([{"id": "diagnostic:preview-physics:pf-timoshenko-factor:6:pipe:P1", "code": "PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE",
        "severity": "blocking", "message": "hand-built blocked control", "source": "core/product_physics", "affected_refs": ["pipe:P1"]}]);
    let (table, _) = s::for_source(&blocked).unwrap();
    assert_eq!(table["semantic_contract_id"], s::PREVIEW_PHYSICS_TIMOSHENKO_ID);
    let d = rv::dispatch(&blocked, None, None);
    assert_eq!(d["binding"]["id"], s::PREVIEW_PHYSICS_TIMOSHENKO_ID);
    assert!(d["standing"]["findings"].as_array().unwrap().iter().any(|f| f == "MECHANICS_NOT_SOLVED"));
}

/// The product's own Timoshenko envelopes (`pf_timoshenko_envelopes.rs` keeps them
/// current) through every reader above, bound to the frozen requests' models: a
/// straight solve, a solve with a realized arc and its disclosure, and a blocked
/// partial-span refusal.
const PRODUCT: [(&str, &str, &str); 3] = [
    ("straight", include_str!("../../../../fixtures/results/preview_physics_timoshenko_straight_sparse.json"),
        include_str!("../../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json")),
    ("arc", include_str!("../../../../fixtures/results/preview_physics_timoshenko_arc_sparse.json"),
        include_str!("../../../../validation/references/pf_timoshenko/requests/PFT-ARC-TIP-TIMOSHENKO.request.json")),
    ("partial_span_blocked", include_str!("../../../../fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json"),
        include_str!("../../../../validation/references/pf_timoshenko/requests/PFT-SB-L1-CM0-TIMOSHENKO.request.json")),
];

#[test]
fn the_products_timoshenko_envelopes_are_read_and_exported_under_their_identity() {
    for (name, text, request) in PRODUCT {
        let source = parse(text);
        let model = parse(request)["model"].clone();
        assert_eq!(model["model_settings"]["shear_model"], "timoshenko_annulus_energy_v1", "{name}");
        assert_eq!(source["producer"]["semantic_contract_id"], s::PREVIEW_PHYSICS_TIMOSHENKO_ID, "{name}");
        let (table, _) = s::for_source(&source).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(table["semantic_contract_id"], s::PREVIEW_PHYSICS_TIMOSHENKO_ID, "{name}");
        s::validate_preview_physics_evidence(&source).unwrap_or_else(|e| panic!("{name}: {e}"));
        rp::validate_preview_physics_transport_metadata(&source).unwrap_or_else(|e| panic!("{name}: {e}"));
        let d = rv::dispatch(&source, Some(&model), None);
        assert_eq!(d["binding"], json!({"id": s::PREVIEW_PHYSICS_TIMOSHENKO_ID, "sha256": s::PREVIEW_PHYSICS_TIMOSHENKO_SHA256}), "{name}");
        // The model-bound crossed pair: the same envelope on the model without the selection.
        let mut absent = model.clone();
        absent.as_object_mut().unwrap().remove("model_settings");
        assert_eq!(rv::validate("preview-physics", &source, Some(&absent), None)["error"]["code"], rv::SHEAR_MODEL_IDENTITY, "{name}");
        if name == "partial_span_blocked" {
            assert_eq!(source["status"]["mechanics"], "MODEL_INCOMPLETE");
            assert!(d["standing"]["findings"].as_array().unwrap().iter().any(|f| f == "MECHANICS_NOT_SOLVED"), "{d}");
            assert!(source["diagnostics"].as_array().unwrap().iter().any(|x| x["code"] == "PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED"));
            continue;
        }
        assert_eq!(rv::validate("preview-physics", &source, Some(&model), None), json!({"ok": true}), "{name}");
        assert_eq!(rv::standing(&source, Some(&model), None)["findings"], json!([]), "{name}");
        assert_eq!(s::numerical_use_standing(&source, &bases(&source)), "numerically_eligible", "{name}");
        let arcs = source["diagnostics"].as_array().unwrap().iter().filter(|x| x["code"] == "PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI").count();
        assert_eq!(arcs, usize::from(name == "arc"), "{name}");
        // The result export: the derivative keeps the identity, the evidence and the formulation basis,
        // whose limitation 0 states the realized-arc rule. It carries none of the product's diagnostics
        // (for either identity); the named arc disclosure reaches the report (`pf_timoshenko_package.rs`).
        let doc = derive(&source).unwrap_or_else(|e| panic!("{name}: {e}"));
        let envelope = &doc["result_envelope"];
        assert_eq!(envelope["semantic_contract_ref"], json!({"ref_type": "semantic_contract", "ref_id": s::PREVIEW_PHYSICS_TIMOSHENKO_ID}));
        assert_eq!(envelope["formulation_basis"], source["formulation_basis"], "{name}");
        assert_eq!(envelope["contract_evidence"], source["contract_evidence"], "{name}");
        let reloaded: Value = serde_json::from_slice(&serde_json::to_vec(&doc).unwrap()).unwrap();
        d::validate_document(&reloaded, &source).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
}
