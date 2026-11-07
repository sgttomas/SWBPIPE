//! RT-A6 H1 (DEL-05-01 Work `hydrostatic-test.md` §4.1, §6 item 3): the
//! connected native transport. An explicit contents source case, through this
//! crate's JSON request (the same one the desktop's Wasm export and native
//! command forward), generates a plan into a different new case; the real
//! operation applier applies it atomically; JSON save/reopen keeps it; the
//! refresh inspection finds it fresh; a selected dependency edit stales it and
//! the refresh resolves the target's own persisted selection; the solve
//! consumes one mass. The reference-system capture pins legacy bytes.
use open_pipe_stress_canonical_json::canonical_json;
use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch};
use open_pipe_stress_product_physics::{run_linear_static_preview, MechanicsEnvelope};
use open_pipe_stress_self_weight_wasm::{generate_plan, generate_self_weight_plan_json};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::f64::consts::PI;

const PROV: &str = "invented_rt_a6_h1_contents_transport_test";
const LENGTH: f64 = 2.0;
const GRAVITY: f64 = -9.80665;

fn q(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn hash(model: &Value) -> String {
    format!("sha256:{:x}", Sha256::digest(canonical_json(model).as_bytes()))
}
fn claim(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","value":hash(model)})
}
/// The public operation envelope the maintained applier tests use.
fn operation(draft: &Value, id: &str) -> Value {
    json!({"operation_id":format!("operation:{id}"),"operation_kind":draft["operation_kind"],"operation_status":"proposed","author_type":"user",
        "target":{"object_type":draft["object_type"],"ref":draft["target_ref"]},
        "change":{"change_id":format!("change:{id}"),"change_kind":draft["change_kind"],"field_label":draft["field_label"],"field_path":draft["field_path"],"before":draft["before"],"after":draft["after"],"unit":draft["unit"],"dimension":draft["dimension"],"source_note":draft["source_note"]},
        "validation":{"schema_validation":"not_run","constraint_validation":"not_run","unit_validation":"not_run","diff_preview_status":"not_generated","application_status":"not_applied"},
        "audit_boundary":{"mutation_route":"structured_operations_only","direct_model_mutation_allowed":false,"requires_user_acceptance":true,"mutates_accepted_model_state":false},
        "professional_boundary":{"human_review_required":true,"software_makes_compliance_claim":false,"software_makes_certification_claim":false,"software_makes_sealing_claim":false,"software_makes_approval_claim":false,"software_makes_authentication_claim":false},"rationale":"Invented H1 connected regression"})
}
/// One explicit authoring step through the real applier.
fn step(model: &Value, kind: &str, change_kind: &str, object: &str, target: &str, path: &str, before: &str, after: &str) -> Value {
    let draft = json!({"object_type":object,"target_ref":target,"operation_kind":kind,"change_kind":change_kind,"field_label":change_kind,
        "field_path":path,"before":before,"after":after,"unit":"none","dimension":"dimensionless","source_note":PROV});
    let out = apply_operation(model, &operation(&draft, change_kind), Some(&claim(model)));
    out.applied_model.unwrap_or_else(|| panic!("{change_kind}: {:?}", out.diagnostics))
}

/// A pressure-free 2 m cantilever, r_o = 0.1 m, t = 0.01 m, steel 7,850 kg/m^3,
/// no section contents; set `cs:hydro` states `HYDRO` = 1,000 kg/m^3 and
/// `ALT` = 1,000 kg/m^3; case `case:hydro` selects `HYDRO`.
fn model() -> Value {
    json!({"schema_version":"0.9.0","document_kind":"openpipestress.product_preview.model",
        "project":{"id":"project:h1-transport","units":{"length":"m","force":"N","angle":"rad","pressure":"Pa","temperature":"degC","stress":"Pa"}},
        "analysis_status":{"mechanics":"draft","rule_check":"not_run","professional_acceptance":"not_assessed"},
        "nodes":[{"id":"node:root","position":{"x":0,"y":0,"z":0},"provenance":PROV},{"id":"node:tip","position":{"x":LENGTH,"y":0,"z":0},"provenance":PROV}],
        "pipe_segments":[{"id":"pipe:beam","from":"node:root","to":"node:tip","material":"material:m","y_reference":{"x":0,"y":1,"z":0},"provenance":PROV,
            "condition_set_ref":"cs:hydro",
            "section":{"outside_diameter":q(0.2,"m"),"wall_thickness":q(0.01,"m"),"material_density":q(7850.0,"kg/m^3")}}],
        "materials":[{"id":"material:m","elastic_modulus":q(200e9,"Pa"),"shear_modulus":q(80e9,"Pa"),"provenance":PROV}],
        "supports":[{"id":"support:root","node":"node:root","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":PROV}],
        "condition_sets":[{"id":"cs:hydro","provenance":PROV,"conditions":[{"key":"HYDRO","contents_density":q(1000.0,"kg/m^3")},{"key":"ALT","contents_density":q(1000.0,"kg/m^3")}]}],
        "sections":[],"components":[],
        "load_cases":[{"id":"case:hydro","label":"Hydrotest source","primitive_loads":[],"provenance":PROV,"condition_use":{"contents_density":"HYDRO"}}],
        "combinations":[]})
}
fn request(model: &Value, source: Value) -> Value {
    json!({"case_id":"case:weight","label":"Hydrotest weight","pipe_refs":["pipe:beam"],"gravity":{"value":GRAVITY,"unit":"m/s^2","axis":"global_y"},
        "provenance":PROV,"source_model_hash":hash(model),"contents_source_case_ref":source})
}
fn transport(model: &Value, request: &Value) -> Value {
    let before = model.clone();
    let outcome = generate_plan(model, request);
    assert_eq!(outcome, serde_json::from_str::<Value>(&generate_self_weight_plan_json(&model.to_string(), &request.to_string())).unwrap());
    assert_eq!(*model, before, "the input model is never mutated");
    outcome
}
fn batch(outcome: &Value, id: &str) -> Value {
    let operations: Vec<_> = outcome["plan"]["changes"].as_array().unwrap().iter().enumerate()
        .map(|(i, draft)| operation(draft, &format!("{id}-{i}"))).collect();
    json!({"batch_id":format!("batch:{id}"),"source_model_hash":outcome["plan"]["source_model_hash"],"operations":operations})
}
fn apply_batch(model: &Value, batch: &Value) -> Value {
    let outcome = apply_operation_batch(model, batch, Some(&claim(model)));
    assert_eq!(outcome["validation"]["application_status"], "applied_to_session_model", "{outcome}");
    outcome["applied_model"].clone()
}
fn reopen(model: &Value) -> Value {
    let reopened: Value = serde_json::from_slice(&serde_json::to_vec_pretty(model).unwrap()).unwrap();
    assert_eq!(*model, reopened);
    reopened
}
fn refresh(model: &Value) -> Value {
    transport(model, &json!({"mode":"refresh","case_id":"case:weight","source_model_hash":hash(model),"manual_overrides":"block"}))
}
fn case<'a>(model: &'a Value, id: &str) -> &'a Value {
    model["load_cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap()
}
fn solve(model: &Value) -> MechanicsEnvelope {
    run_linear_static_preview(serde_json::from_value(json!({"model":model,"materials":[]})).unwrap())
}

#[test]
fn explicit_contents_generate_apply_reopen_inspect_and_solve() {
    let original = model();
    let outcome = transport(&original, &request(&original, json!("case:hydro")));
    assert_eq!(outcome["status"], "ready", "{outcome}");
    let created: Value = serde_json::from_str(outcome["plan"]["changes"][0]["after"].as_str().unwrap()).unwrap();
    assert_eq!(created["condition_use"], json!({"contents_density":"HYDRO"}), "the target states the selection as its own");
    let evidence = &outcome["plan"]["source_evidence"][0];
    assert_eq!(evidence["contents_source"]["origin_case"], "case:hydro");
    // Independent weight: pi (0.0019 * 7850 + 0.0081 * 1000) * 9.80665 N/m.
    let reference = 902800199.0 / 4000000.0 * PI;
    let intensity = evidence["generated_payload"]["magnitude"]["value"].as_f64().unwrap();
    assert!((intensity + reference).abs() <= 1e-14 * reference, "{intensity} vs -{reference}");

    // The real applier applies the plan atomically; JSON save/reopen keeps it fresh.
    let adopted = reopen(&apply_batch(&original, &batch(&outcome, "weight")));
    assert_eq!(case(&adopted, "case:weight")["condition_use"], json!({"contents_density":"HYDRO"}));
    let fresh = refresh(&adopted);
    assert_eq!(fresh["diagnostics"][0]["code"], "SELF-WEIGHT-REFRESH-NOT-NEEDED", "{fresh}");

    // Unrelated to the target: the origin's key, through the real applier.
    let origin_edited = step(&adopted, "modify", "set_load_case_condition_use", "Load", "case:hydro", "condition_use",
        &canonical_json(&json!({"contents_density":"HYDRO"})), &canonical_json(&json!({"contents_density":"ALT"})));
    assert_eq!(refresh(&origin_edited)["diagnostics"][0]["code"], "SELF-WEIGHT-REFRESH-NOT-NEEDED");

    // The target's own selection, equal density from another key: stale; the
    // refresh resolves the target's persisted selection, keeps identity and origin.
    let rekeyed = step(&adopted, "modify", "set_load_case_condition_use", "Load", "case:weight", "condition_use",
        &canonical_json(&json!({"contents_density":"HYDRO"})), &canonical_json(&json!({"contents_density":"ALT"})));
    let stale = refresh(&rekeyed);
    assert_eq!(stale["status"], "ready", "{stale}");
    assert_eq!(stale["plan"]["source_evidence"][0]["state"], "stale");
    let refreshed = reopen(&apply_batch(&rekeyed, &batch(&stale, "refresh")));
    let record: Value = serde_json::from_str(case(&refreshed, "case:weight")["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
    assert_eq!(record["contents_source"]["condition_key"], "ALT");
    assert_eq!(record["contents_source"]["origin_case"], "case:hydro");
    assert_eq!(record["generated_payload"]["magnitude"]["value"].as_f64().unwrap().to_bits(), intensity.to_bits());
    assert_eq!(refresh(&refreshed)["diagnostics"][0]["code"], "SELF-WEIGHT-REFRESH-NOT-NEEDED");

    // Solve the complete applied/reopened model: keep the source selection,
    // apply exactly one target weight, and report a nonblocking source notice.
    let solved = solve(&reopen(&adopted));
    assert_eq!(solved.status.mechanics, "MECHANICS_SOLVED", "{:?}", solved.diagnostics);
    assert!(solved.diagnostics.iter().any(|d| d.code == "CASE_CONTENTS_WEIGHT_NOT_APPLIED" && d.severity == "warning" && d.affected_refs.contains(&"case:hydro".to_string())));
    let fy = solved.results.iter().find(|r| r.kind == "support_reaction_component_v2" && r.entity_ref == "support:root"
        && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == "case:weight") && r.metadata.as_ref().is_some_and(|m| m.component == "Fy")).unwrap().value;
    assert!((fy - reference * LENGTH).abs() <= 1e-9 * reference * LENGTH, "{fy} vs {}", reference * LENGTH);
}

#[test]
fn malformed_or_unresolvable_sources_refuse_by_name_and_never_fall_back() {
    let original = model();
    for (source, code) in [
        (Value::Null, "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"),
        (json!(""), "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"),
        (json!("   "), "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"),
        (json!(7), "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"),
        (json!({"id":"case:hydro"}), "SELF_WEIGHT_CONTENTS_SOURCE_INVALID"),
        (json!("case:missing"), "CASE_CONTENTS_CONTEXT_MISSING"),
        (json!("case:hydro "), "CASE_CONTENTS_CONTEXT_MISSING"),
        (json!("case:weight"), "CASE_CONTENTS_CONTEXT_CONFLICT"),
    ] {
        let outcome = transport(&original, &request(&original, source.clone()));
        assert_eq!(outcome["status"], "blocked", "{source}");
        assert!(outcome["plan"].is_null());
        assert_eq!(outcome["diagnostics"][0]["code"], code, "{source}: {outcome}");
    }
    // An unknown member beside a valid source is a shape refusal, as before.
    let mut unknown = request(&original, json!("case:hydro"));
    unknown["contents_source"] = json!("case:hydro");
    assert_eq!(transport(&original, &unknown)["diagnostics"][0]["code"], "SELF-WEIGHT-REQUEST-SHAPE-INVALID");
    // The source-model hash guard applies to a contents request exactly as before.
    let mut stale = request(&original, json!("case:hydro"));
    stale["source_model_hash"] = json!(format!("sha256:{}", "0".repeat(64)));
    assert_eq!(transport(&original, &stale)["diagnostics"][0]["code"], "SELF-WEIGHT-SOURCE-HASH-MISMATCH");
    // A source with no contents use, and a selection that cannot resolve.
    let mut no_use = original.clone();
    no_use["load_cases"][0].as_object_mut().unwrap().remove("condition_use");
    assert_eq!(transport(&no_use, &request(&no_use, json!("case:hydro")))["diagnostics"][0]["code"], "CASE_CONTENTS_CONTEXT_MISSING");
    let mut unassigned = original.clone();
    unassigned["pipe_segments"][0].as_object_mut().unwrap().remove("condition_set_ref");
    assert_eq!(transport(&unassigned, &request(&unassigned, json!("case:hydro")))["diagnostics"][0]["code"], "OPERATING_CONDITION_SET_MISSING");
}

/// Legacy bytes: the reference system's committed capture (DEL-09-01) was made
/// by this crate's generator before RT-A6 H1. With no contents source, every
/// captured case record and primitive regenerates byte for byte, except the
/// source-model hash, which the later 0.9.0 version token changed.
#[test]
fn no_use_generation_regenerates_the_reference_system_capture_byte_for_byte() {
    let mut base: Value = serde_json::from_str(include_str!("../../../../fixtures/reference_system/reference_system.json")).unwrap();
    base["load_cases"] = json!([]);
    base["combinations"] = json!([]);
    let capture: Value = serde_json::from_str(include_str!("../../../../fixtures/reference_system/self_weight_capture.json")).unwrap();
    let current = hash(&base);
    let cases = capture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 3);
    for captured in cases {
        let mut request = captured["request"].clone();
        let old = request["source_model_hash"].as_str().unwrap().to_string();
        request["source_model_hash"] = json!(current);
        assert!(request.get("contents_source_case_ref").is_none());
        let outcome = generate_plan(&base, &request);
        assert_eq!(outcome["status"], "ready", "{outcome}");
        let changes = outcome["plan"]["changes"].as_array().unwrap();
        assert_eq!(outcome["plan"]["scope_label"], captured["scope_label"]);
        let record: Value = serde_json::from_str(changes[0]["after"].as_str().unwrap()).unwrap();
        assert_eq!(record, captured["case_record"]);
        let loads = captured["primitive_loads"].as_array().unwrap();
        assert_eq!(changes.len(), loads.len() + 1);
        for (change, load) in changes[1..].iter().zip(loads) {
            let mut expected = load.clone();
            expected["provenance"] = json!(load["provenance"].as_str().unwrap().replace(&old, &current));
            assert_eq!(change["after"].as_str().unwrap(), expected.to_string(), "{}", load["id"]);
            assert!(!change["after"].as_str().unwrap().contains("contents_source"));
        }
    }
}

#[test]
fn contents_aware_ex_target_is_complete_through_the_existing_operation_route() {
    let mut original = model();
    original["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"});
    original["materials"][0]["constitutive_basis"] = json!("homogeneous_isotropic_E_nu_v1");
    original["materials"][0]["poisson_ratio"] = q(0.25, "1");
    original["load_cases"][0]["pressure_regions"] = json!([]);
    original["pipe_segments"][0]["section"]["corrosion_allowance"] = q(0.001, "m");
    let outcome = transport(&original, &request(&original, json!("case:hydro")));
    assert_eq!(outcome["status"], "ready", "{outcome}");
    let created: Value = serde_json::from_str(outcome["plan"]["changes"][0]["after"].as_str().unwrap()).unwrap();
    assert_eq!(created["pressure_regions"], json!([]));
    assert!(created.get("test_condition").is_none());
    let adopted = reopen(&apply_batch(&original, &batch(&outcome, "ex-weight")));
    assert_eq!(case(&adopted, "case:weight")["pressure_regions"], json!([]));
    assert_eq!(refresh(&adopted)["diagnostics"][0]["code"], "SELF-WEIGHT-REFRESH-NOT-NEEDED");
    let solved = solve(&adopted);
    assert_eq!(solved.status.mechanics, "MECHANICS_SOLVED", "{:?}", solved.diagnostics);
    assert!(solved.diagnostics.iter().any(|d| d.code == "CASE_CONTENTS_WEIGHT_NOT_APPLIED" && d.severity == "warning"));
    let reference = (5686876335.0 / 25000000.0) * PI;
    let fy = solved.results.iter().find(|r| r.kind == "support_reaction_component_v2" && r.entity_ref == "support:root"
        && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == "case:weight") && r.metadata.as_ref().is_some_and(|m| m.component == "Fy")).unwrap().value;
    assert!((fy - reference * LENGTH).abs() <= 1e-9 * reference * LENGTH);
    // A nonempty pressure payload is not copied through this weight-only path.
    let mut invalid = batch(&outcome, "ex-invalid-pressure");
    let mut payload: Value = serde_json::from_str(invalid["operations"][0]["change"]["after"].as_str().unwrap()).unwrap();
    payload["pressure_regions"] = json!([{"id":"not-a-weight-region"}]);
    invalid["operations"][0]["change"]["after"] = json!(canonical_json(&payload));
    let refused = apply_operation_batch(&original, &invalid, Some(&claim(&original)));
    assert_ne!(refused["validation"]["application_status"], "applied_to_session_model");
    assert!(refused["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "OP-CREATE-LOAD-CASE-PAYLOAD-INVALID"));
}

fn managed_replacement(model: &Value, loads: &Value, id: &str) -> Value {
    let draft = json!({"object_type":"Load","target_ref":"case:weight","operation_kind":"modify",
        "change_kind":"update_load","field_label":"Refresh generated self-weight","field_path":"generated_self_weight",
        "before":canonical_json(&case(model,"case:weight")["primitive_loads"]),"after":canonical_json(loads),
        "unit":"none","dimension":"dimensionless","source_note":PROV});
    serde_json::to_value(apply_operation(model, &operation(&draft,id), Some(&claim(model)))).unwrap()
}

#[test]
fn managed_refresh_preserves_contents_history_and_validates_current_source_evidence() {
    let original = model();
    let generation = transport(&original, &request(&original,json!("case:hydro")));
    let fresh = apply_batch(&original,&batch(&generation,"history-base"));
    for stale in [false,true] {
        let mut before = fresh.clone();
        if stale {
            before = step(&before,"modify","set_load_case_condition_use","Load","case:weight","condition_use",
                &canonical_json(&json!({"contents_density":"HYDRO"})),&canonical_json(&json!({"contents_density":"ALT"})));
        }
        let mut valid = if stale {
            let plan = refresh(&before);
            assert_eq!(plan["status"],"ready","{plan}");
            serde_json::from_str::<Value>(plan["plan"]["changes"][0]["after"].as_str().unwrap()).unwrap()
        } else {
            let mut values = case(&before,"case:weight")["primitive_loads"].clone();
            let mut record: Value = serde_json::from_str(values[0]["provenance"].as_str().unwrap()).unwrap();
            record["source_model_hash"] = json!(hash(&before));
            record["contents_source"]["selected_by_case"] = json!("case:weight");
            values[0]["provenance"] = json!(canonical_json(&record));
            values
        };
        for attack in ["origin","selector","both","superseded-shape","superseded-value","superseded-extra","drop"] {
            let mut proposed = valid.clone();
            let mut record: Value = serde_json::from_str(proposed[0]["provenance"].as_str().unwrap()).unwrap();
            match attack {
                "origin" => record["contents_source"]["origin_case"] = json!("fabricated:origin"),
                "selector" => record["contents_source"]["selected_by_case"] = json!("fabricated:selector"),
                "both" => {
                    record["contents_source"]["origin_case"] = json!("fabricated:origin");
                    record["contents_source"]["selected_by_case"] = json!("fabricated:origin");
                },
                "superseded-shape" => record["contents_source"]["section_contents_density_superseded"] = json!({"garbage":true}),
                "superseded-value" => record["contents_source"]["section_contents_density_superseded"] = q(99.0,"kg/m^3"),
                "superseded-extra" => record["contents_source"]["section_contents_density_superseded"] = json!({"value":0,"unit":"kg/m^3","extra":true}),
                "drop" => { record.as_object_mut().unwrap().remove("contents_source"); },
                _ => unreachable!(),
            }
            proposed[0]["provenance"] = json!(canonical_json(&record));
            let outcome = managed_replacement(&before,&proposed,attack);
            assert!(outcome["applied_model"].is_null(),"stale={stale} attack={attack}: {outcome}");
            assert!(outcome["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "OP-RICH-PAYLOAD-INVALID"),"{outcome}");
        }
        // The actual current-source refresh remains valid after the origin is
        // removed: its recorded identity is history, not a live lookup.
        if stale {
            before["load_cases"].as_array_mut().unwrap().retain(|case| case["id"] != "case:hydro");
            let plan = refresh(&before);
            assert_eq!(plan["status"],"ready","{plan}");
            valid = serde_json::from_str(plan["plan"]["changes"][0]["after"].as_str().unwrap()).unwrap();
            let accepted = managed_replacement(&before,&valid,"valid-orphan-origin");
            assert_eq!(accepted["validation"]["application_status"],"applied_to_session_model","{accepted}");
            assert_eq!(refresh(&accepted["applied_model"])["diagnostics"][0]["code"],"SELF-WEIGHT-REFRESH-NOT-NEEDED");
        }
    }
}

#[test]
fn removed_contents_context_refuses_refresh_without_erasing_origin_or_using_sections() {
    let original = model();
    let generated = transport(&original,&request(&original,json!("case:hydro")));
    let adopted = apply_batch(&original,&batch(&generated,"removed-context"));
    let cleared = step(&adopted,"modify","clear_load_case_condition_use","Load","case:weight","condition_use",
        &canonical_json(&json!({"contents_density":"HYDRO"})),"not_present");
    let refused = refresh(&cleared);
    assert_eq!(refused["status"],"blocked","{refused}");
    assert!(refused["plan"].is_null());
    assert_eq!(refused["diagnostics"][0]["code"],"CASE_CONTENTS_CONTEXT_MISSING");
}

#[test]
fn unsupported_contents_seismic_never_carries_as_info_under_entity_key_collision() {
    let mut original = model();
    original["load_cases"][0]["id"] = json!("SUS");
    original["load_cases"][0]["condition_use"]["contents_density"] = json!("SUS");
    original["condition_sets"][0]["conditions"][0]["key"] = json!("SUS");
    original["load_cases"][0]["equivalent_static"] = json!({"seismic":{
        "gravity_acceleration":q(9.80665,"m/s^2"),"g_factor_y":q(0.1,"1")}});
    let draft = json!({"object_type":"Load","target_ref":"case:other","operation_kind":"create","change_kind":"create_load_case",
        "field_label":"Create another contents case","field_path":"load_cases","before":"not_present",
        "after":canonical_json(&json!({"id":"case:other","label":"Other","kind":"primitive_user_load","status":"draft",
            "provenance":PROV,"primitive_loads":[],"condition_use":{"contents_density":"SUS"}})),
        "unit":"none","dimension":"dimensionless","source_note":PROV});
    let outcome = apply_operation(&original,&operation(&draft,"never-carried"),Some(&claim(&original)));
    assert!(outcome.applied_model.is_none(),"{:?}",outcome.diagnostics);
    assert!(outcome.diagnostics.iter().any(|d| d.code == "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED" && d.severity == "blocking"));
    assert!(!outcome.diagnostics.iter().any(|d| d.code == "LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED" && d.severity == "info"));
}

#[test]
fn a_legacy_target_can_adopt_its_own_contents_selection_without_fabricating_an_origin() {
    let mut original = model();
    original["load_cases"] = json!([]);
    let mut wire = request(&original,Value::Null);
    wire.as_object_mut().unwrap().remove("contents_source_case_ref");
    let generated = transport(&original,&wire);
    let legacy = apply_batch(&original,&batch(&generated,"legacy-before-contents"));
    let selected = step(&legacy,"modify","set_load_case_condition_use","Load","case:weight","condition_use",
        "not_present",&canonical_json(&json!({"contents_density":"HYDRO"})));
    let plan = refresh(&selected);
    assert_eq!(plan["status"],"ready","{plan}");
    let adopted = reopen(&apply_batch(&selected,&batch(&plan,"legacy-adopts-contents")));
    let record: Value = serde_json::from_str(case(&adopted,"case:weight")["primitive_loads"][0]["provenance"].as_str().unwrap()).unwrap();
    assert_eq!(record["contents_source"]["origin_case"],"case:weight");
    assert_eq!(record["contents_source"]["selected_by_case"],"case:weight");
    assert_eq!(refresh(&adopted)["diagnostics"][0]["code"],"SELF-WEIGHT-REFRESH-NOT-NEEDED");
}
