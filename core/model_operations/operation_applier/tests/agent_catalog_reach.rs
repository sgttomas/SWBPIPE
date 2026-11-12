//! swbpipe-catalog/5 (DEL-16-06 CLM-005, REQ-007, AC-006): the operation kinds
//! the embedded agent's catalog newly admits, examined at the engine with
//! scripted agent-authored inputs.
//!
//! The route: the committed reference system (`fixtures/reference_system/`,
//! read, never edited) without the families these kinds enter (the project's
//! non-length units, the material's temperature points and expansion law, the
//! sections and every pipe's binding, the reference configuration, the three
//! cases' pressure regions and analysis states, the two combinations) is the
//! starting document. Agent-form operations carrying the fixture's own values
//! enter them, each proposal stamped as the host stamps it (`author_type:
//! agent`), held to the catalog's admitted rows and argument bound, and applied
//! as one atomic batch. The result is the reference document byte for byte, and
//! no component (its modifiers, realization and geometry) changes on the way.
//!
//! The rule that does not move: every route by which an agent-authored
//! operation could write a person-entered value (k, the SIFs, the other
//! modifiers and their source, bend realization or insertion, a library record
//! copying modifier members) is refused by the engine with its code, and the
//! catalog admits none of them. The person's entries in `authoring.json` are
//! the values attempted.
//!
//! This is a route check with scripted inputs. It does not show that the agent
//! builds the reference system or completes a use case: the geometry,
//! components, supports and generated loads are the fixture's, and k and the
//! SIFs are the person's. All data are invented.
use open_pipe_stress_operation_applier::{apply_operation, apply_operation_batch, canonical_json, sha256_hex, validate_operation_batch};
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const REFERENCE: &str = include_str!("../../../../fixtures/reference_system/reference_system.json");
const AUTHORING: &str = include_str!("../../../../fixtures/reference_system/authoring.json");
const CATALOG: &str = include_str!("../../../../apps/desktop/src-tauri/resources/agent-capability-catalog.json");
// The host stamp the desktop applies (its vitest binds embeddedAgentSession.ts to it).
const HOST: &str = include_str!("../../../../fixtures/model_operations/agent_w9a_journey.json");
const K: &str = "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED";
const MODIFIER: &str = "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED";

fn reference() -> Value {
    serde_json::from_str(REFERENCE).unwrap()
}
fn catalog() -> Value {
    serde_json::from_str(CATALOG).unwrap()
}
fn hash(model: &Value) -> Value {
    json!({"algorithm":"sha256","canonicalization":"rfc8785_jcs","payload_scope":"model_payload","payload_ref":"model","value":format!("sha256:{}",sha256_hex(&canonical_json(model))),"hash_status":"computed_local_preview"})
}
const STARTING_UNITS: [(&str, &str); 6] = [("force", "kN"), ("moment", "kN*m"), ("pressure", "kPa"), ("stress", "MPa"), ("temperature", "degF"), ("angle", "deg")];

/// The reference document without the families this journey enters.
fn starting_document() -> Value {
    let mut model = reference();
    for (key, symbol) in STARTING_UNITS {
        model["project"]["units"][key] = json!(symbol);
    }
    model["sections"] = json!([]);
    model["combinations"] = json!([]);
    model.as_object_mut().unwrap().remove("reference_configurations");
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe.as_object_mut().unwrap().remove("section_ref");
    }
    for material in model["materials"].as_array_mut().unwrap() {
        let material = material.as_object_mut().unwrap();
        material.remove("temperature_points");
        material.remove("expansion_laws");
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        let case = case.as_object_mut().unwrap();
        case.remove("analysis_state");
        case.remove("pressure_regions");
    }
    model
}

/// One agent-form operation: what a swbpipe_propose_operations call carries.
#[allow(clippy::too_many_arguments)]
fn agent(kind: &str, object: &str, target: &str, change_kind: &str, path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Value {
    let id = format!("{change_kind}:{target}:{path}");
    json!({"operation_id":format!("op:{id}"),"operation_kind":kind,"target":{"object_type":object,"ref":target},
        "change":{"change_id":format!("change:{id}"),"change_kind":change_kind,"field_label":path,"field_path":path,"before":before,"after":after,"unit":unit,"dimension":dimension,
            "source_note":"the reference system's run sheet, as the person stated it"},
        "rationale":"enter the stated value"})
}
fn none(kind: &str, object: &str, target: &str, change_kind: &str, path: &str, before: &str, after: &str) -> Value {
    agent(kind, object, target, change_kind, path, before, after, "none", "dimensionless")
}

/// The catalog's admission of an agent-form operation: its discriminators match
/// one admitted row and its texts lie within the declared bounds.
fn catalog_admits(op: &Value) -> bool {
    let catalog = catalog();
    let change = &catalog["definitions"]["operationIntent"]["properties"]["change"]["properties"];
    let chars = |key: &str| op["change"][key].as_str().unwrap().chars().count() as u64;
    chars("before") <= change["before"]["maxLength"].as_u64().unwrap()
        && chars("after") <= change["after"]["maxLength"].as_u64().unwrap()
        && catalog["admittedOperations"].as_array().unwrap().iter().any(|row| {
            row["operationKind"] == op["operation_kind"]
                && row["objectType"] == op["target"]["object_type"]
                && row["changeKind"] == op["change"]["change_kind"]
                && row["fieldPaths"].as_array().unwrap().contains(&op["change"]["field_path"])
        })
}

/// The host stamp (`stampAgentOperations`): status, attribution, queue, source and constant records.
fn stamped(op: &Value) -> Value {
    let host: Value = serde_json::from_str(HOST).unwrap();
    let mut native = op.clone();
    for (key, value) in host["host"]["stamp"].as_object().unwrap() {
        assert!(native.as_object_mut().unwrap().insert(key.clone(), value.clone()).is_none(), "an agent-form operation carries no host field `{key}`");
    }
    native["queue_id"] = json!("proposal-reach");
    native["source"] = json!({"source_ref":"request-reach","source_channel":"embedded_agent","source_role":"agent"});
    assert_eq!(native["author_type"], "agent");
    native
}

/// One proposal: within the catalog's admitted rows, operation count and argument
/// bytes, applied as one atomic agent batch. Components never change.
fn propose(model: &Value, operations: Vec<Value>) -> Value {
    let catalog = catalog();
    assert!(operations.len() as u64 <= catalog["limits"]["proposalOperations"].as_u64().unwrap());
    for op in &operations {
        assert!(catalog_admits(op), "not admitted: {}", op["change"]["field_path"]);
    }
    let claim = hash(model);
    let mut basis_hash = claim.clone();
    basis_hash["value"] = json!(claim["value"].as_str().unwrap().trim_start_matches("sha256:"));
    let arguments = json!({"workspaceHandle":"w".repeat(48),"basis":{"workspaceHandle":"w".repeat(48),"applicationGeneration":1,"modelRevision":1,"modelHash":basis_hash,"stateId":null,"runId":null},
        "claimedModelHash":basis_hash,"operations":operations,"assumptions":[]});
    let bytes = serde_json::to_vec(&arguments).unwrap().len() as u64;
    assert!(bytes <= catalog["limits"]["argumentBytes"].as_u64().unwrap(), "{bytes} argument bytes");
    let batch = json!({"batch_id":"agent-proposal-reach","operations":operations.iter().map(stamped).collect::<Vec<_>>(),"source_model_hash":claim["value"]});
    let outcome = apply_operation_batch(model, &batch, Some(&claim));
    assert_eq!(outcome["validation"]["batch_validation_status"], "passed", "{:#}", outcome["diagnostics"]);
    let applied = outcome["applied_model"].clone();
    assert_eq!(person_entered(&applied), person_entered(model), "no person-entered member changes");
    applied
}

/// Every component's person-entered members (modifiers and their source), its
/// realization and its connector, keyed by id.
fn person_entered(model: &Value) -> String {
    canonical_json(&Value::Array(model["components"].as_array().unwrap().iter().map(|c| json!({"id":c["id"],"modifiers":c.get("modifiers"),
        "mechanics_interface":c.get("mechanics_interface"),"objective_connector":c.get("objective_connector")})).collect()))
}

/// The paths at which two documents differ (at most a few, for the message).
fn differ(at: &str, left: &Value, right: &Value, out: &mut Vec<String>) {
    if out.len() >= 8 || canonical_json(left) == canonical_json(right) {
        return;
    }
    match (left, right) {
        (Value::Object(a), Value::Object(b)) => {
            for key in a.keys().chain(b.keys().filter(|k| !a.contains_key(*k))) {
                differ(&format!("{at}.{key}"), a.get(key).unwrap_or(&Value::Null), b.get(key).unwrap_or(&Value::Null), out);
            }
        }
        (Value::Array(a), Value::Array(b)) if a.len() == b.len() => {
            for (i, (x, y)) in a.iter().zip(b).enumerate() {
                differ(&format!("{at}[{i}]"), x, y, out);
            }
        }
        _ => out.push(format!("{at}: {} != {}", &canonical_json(left).chars().take(120).collect::<String>(), &canonical_json(right).chars().take(120).collect::<String>())),
    }
}

fn find<'a>(model: &'a Value, collection: &str, id: &str) -> &'a Value {
    model[collection].as_array().unwrap().iter().find(|r| r["id"] == id).unwrap_or_else(|| panic!("{collection} {id}"))
}

#[test]
fn newly_admitted_kinds_enter_the_reference_families_from_the_fixture_values() {
    let fixture = reference();
    let mut model = starting_document();
    let project = fixture["project"]["id"].as_str().unwrap().to_string();
    // Units: each key from its stored symbol to the fixture's; the length unit is already the fixture's.
    model = propose(&model, STARTING_UNITS.iter().map(|(key, symbol)| {
        let after = fixture["project"]["units"][*key].as_str().unwrap();
        agent("modify", "Project", &project, "set_project_unit", &format!("project.units.{key}"), symbol, after, after, key)
    }).collect());
    // The material's temperature points, then its expansion law.
    let material = find(&fixture, "materials", "material:RS-M1");
    model = propose(&model, vec![
        none("modify", "Material", "material:RS-M1", "set_field", "temperature_points", "not_present", &canonical_json(&material["temperature_points"])),
        none("modify", "Material", "material:RS-M1", "set_field", "expansion_laws", "not_present", &canonical_json(&material["expansion_laws"])),
    ]);
    // The three sections, then each pipe's binding (before: the pipe's canonical record).
    model = propose(&model, fixture["sections"].as_array().unwrap().iter().map(|section| {
        let unit = section["properties"]["outside_diameter"]["unit"].as_str().unwrap();
        agent("create", "Section", section["id"].as_str().unwrap(), "create_section", "sections", "not_present", &canonical_json(section), unit, "length")
    }).collect());
    let pipes: Vec<Value> = fixture["pipe_segments"].as_array().unwrap().clone();
    // Sixteen bindings exceed the argument bound (each before is a whole pipe record); ten fit.
    for chunk in pipes.chunks(10) {
        let ops = chunk.iter().map(|pipe| {
            let id = pipe["id"].as_str().unwrap();
            none("modify", "Element", id, "assign_section", "section_ref", &canonical_json(find(&model, "pipe_segments", id)), &canonical_json(&json!({"section_ref": pipe["section_ref"]})))
        }).collect();
        model = propose(&model, ops);
    }
    // The reference configuration.
    model = propose(&model, vec![none("modify", "Model", &project, "set_field", "reference_configurations", "not_present", &canonical_json(&fixture["reference_configurations"]))]);
    // Each case's pressure regions, then each analysis state alone in its proposal.
    let cases: Vec<Value> = fixture["load_cases"].as_array().unwrap().clone();
    model = propose(&model, cases.iter().map(|case| none("modify", "Load", case["id"].as_str().unwrap(), "update_load", "pressure_regions", "not_present", &canonical_json(&case["pressure_regions"]))).collect());
    for case in &cases {
        let state = canonical_json(&case["analysis_state"]);
        assert!(state.chars().count() > 8192, "the reason the after bound moved");
        model = propose(&model, vec![none("modify", "Load", case["id"].as_str().unwrap(), "update_load", "analysis_state", "not_present", &state)]);
    }
    // The two combinations.
    model = propose(&model, fixture["combinations"].as_array().unwrap().iter().map(|combination| {
        none("create", "Combination", combination["id"].as_str().unwrap(), "create_combination", "combinations", "not_present", &canonical_json(combination))
    }).collect());
    // The run sheet's §3.14 difference the person's route has too: the engine stores
    // `terms: []` on a subtraction. It is checked to be exactly that, then set aside.
    for combination in model["combinations"].as_array_mut().unwrap() {
        assert_eq!(combination["basis"], "result_state_subtraction");
        assert_eq!(combination.as_object_mut().unwrap().remove("terms"), Some(json!([])));
    }
    let mut differences = Vec::new();
    differ("$", &model, &fixture, &mut differences);
    assert!(differences.is_empty(), "the entered document differs from the reference at {differences:?}");
    assert_eq!(canonical_json(&model), canonical_json(&fixture), "the entered families equal the reference document");
    assert_eq!(canonical_json(&model["components"]), canonical_json(&starting_document()["components"]), "no component changed on the way");
}

#[test]
fn two_analysis_states_exceed_one_proposal_and_a_replacement_needs_two() {
    // The argument bound is unchanged: a state travels alone, and replacing a
    // stored state (its before text is the state) is a removal, then a write.
    let fixture = reference();
    let cases = fixture["load_cases"].as_array().unwrap();
    let ops: Vec<Value> = cases.iter().take(2).map(|case| none("modify", "Load", case["id"].as_str().unwrap(), "update_load", "analysis_state", "not_present", &canonical_json(&case["analysis_state"]))).collect();
    let arguments = json!({"operations": ops});
    assert!(serde_json::to_vec(&arguments).unwrap().len() as u64 > catalog()["limits"]["argumentBytes"].as_u64().unwrap());
    let state = canonical_json(&cases[1]["analysis_state"]);
    let replace = json!({"operations":[none("modify", "Load", "load:RS-OPE", "update_load", "analysis_state", &state, &state)]});
    assert!(serde_json::to_vec(&replace).unwrap().len() as u64 > catalog()["limits"]["argumentBytes"].as_u64().unwrap());
    let removal = none("modify", "Load", "load:RS-OPE", "update_load", "analysis_state", &state, "not_present");
    assert!(catalog_admits(&removal));
    let applied = propose(&fixture, vec![removal]);
    assert!(find(&applied, "load_cases", "load:RS-OPE").get("analysis_state").is_none());
}

fn refusal_codes(model: &Value, op: &Value) -> Vec<String> {
    let outcome = apply_operation(model, &stamped(op), Some(&hash(model)));
    assert!(outcome.applied_model.is_none(), "applied: {}", op["change"]["field_path"]);
    outcome.diagnostics.iter().map(|d| d.code.clone()).collect()
}

#[test]
fn every_person_entered_route_is_refused_by_the_engine_and_not_admitted_by_the_catalog() {
    let model = reference();
    let authoring: Value = serde_json::from_str(AUTHORING).unwrap();
    let bend = &authoring["bends"][0];
    let inputs = &bend["person_inputs"];
    let component = bend["component"].as_str().unwrap();
    let q = |v: &Value| canonical_json(v);
    // The person's own entries, attempted by the agent on the admitted Component set_field kind.
    let mut attempts = vec![
        (K, none("modify", "Component", component, "set_field", "modifiers.flexibility_factor_user_value.value", "TBD", &q(&inputs["flexibility_factor"]))),
        (MODIFIER, none("modify", "Component", component, "set_field", "modifiers.sif_in_plane_user_value.value", "TBD", &q(&inputs["sif_in_plane_user_value"]))),
        (MODIFIER, none("modify", "Component", component, "set_field", "modifiers.sif_out_of_plane_user_value.value", "TBD", &q(&inputs["sif_out_of_plane_user_value"]))),
        (MODIFIER, none("modify", "Component", component, "set_field", "modifiers.source_reference", "TBD", inputs["sif_source_reference"].as_str().unwrap())),
        (K, none("modify", "Component", component, "set_field", "bend_realization", "TBD", "{}")),
        (K, none("modify", "Node", "node:RS-060", "insert_bend_at_corner", "bend_at_corner", "not_present", "{}")),
    ];
    for path in ["sif_user_value", "branch_header_sif_user_value", "branch_branch_sif_user_value", "stiffness_scaling_user_value", "linear_stiffness_user_value",
        "rotational_stiffness_user_value", "axial_stiffness_user_value", "lateral_stiffness_user_value", "angular_stiffness_user_value", "torsional_stiffness_user_value"] {
        attempts.push((MODIFIER, none("modify", "Component", "component:RS-T1", "set_field", &format!("modifiers.{path}.value"), "TBD", "{\"value\":2,\"unit\":\"none\"}")));
    }
    // A component created with modifiers, and a library record copying modifier members.
    let mut tee = find(&model, "components", "component:RS-T1").clone();
    tee["id"] = json!("component:RS-T2");
    attempts.push((MODIFIER, none("insert", "Component", "component:RS-T2", "insert_component_symbol", "components", "not_present", &canonical_json(&tee))));
    attempts.push((MODIFIER, none("modify", "Component", "component:RS-T1", "apply_library_record", "components", "TBD", &canonical_json(&json!({"modifiers.branch_header_sif_user_value": {"value": 2.4, "unit": "1"}})))));
    attempts.push((K, none("modify", "Component", component, "apply_library_record", "components", "TBD", &canonical_json(&json!({"modifiers.flexibility_factor_user_value": inputs["flexibility_factor"]})))));
    for (code, op) in &attempts {
        let found = refusal_codes(&model, op);
        assert!(found.contains(&code.to_string()), "{}: {found:?}", op["change"]["field_path"]);
        // insert_component_symbol is admitted by kind (since swbpipe-catalog/4); the
        // catalog cannot see a modifiers member inside the after text, so for that
        // route the engine's refusal is the gate. Every other route is not admitted.
        let by_kind = op["change"]["change_kind"] == "insert_component_symbol";
        assert_eq!(catalog_admits(op), by_kind, "catalog admission of {} {}", op["change"]["change_kind"], op["change"]["field_path"]);
    }
    // No admitted row names a person-entered path or route.
    for row in catalog()["admittedOperations"].as_array().unwrap() {
        assert!(!matches!(row["changeKind"].as_str().unwrap(), "insert_bend_at_corner" | "apply_library_record"), "{row}");
        for path in row["fieldPaths"].as_array().unwrap() {
            let path = path.as_str().unwrap();
            assert!(!path.starts_with("modifiers") && path != "bend_realization" && path != "generated_self_weight" && !path.starts_with("mechanics_interface"), "{path}");
        }
    }
}

#[test]
fn admitted_edits_next_to_a_component_leave_its_person_entered_values_unchanged() {
    // A geometry, section, material and setting edit an agent may propose: each
    // writes its own member, and no component's modifiers, realization or source
    // change as a side effect.
    let model = reference();
    let bend = find(&model, "components", "component:RS-B1");
    let radius = &bend["geometry"]["bend_radius"];
    let tee = find(&model, "components", "component:RS-T1");
    let pipe = find(&model, "pipe_segments", "pipe:RS-P060");
    let material = find(&model, "materials", "material:RS-M1");
    let section = find(&model, "sections", "section:RS-S270");
    let project = model["project"]["id"].as_str().unwrap();
    let edits = vec![
        agent("modify", "Component", "component:RS-T1", "set_field", "geometry.branch_connection_type", tee["geometry"]["branch_connection_type"].as_str().unwrap(), "agent-stated connection type", "none", "dimensionless"),
        agent("modify", "Component", "component:RS-B1", "set_field", "geometry.bend_radius.value", &radius["value"].to_string(), &canonical_json(&json!({"value":0.45,"unit":radius["unit"]})), radius["unit"].as_str().unwrap(), "length"),
        agent("modify", "Element", "pipe:RS-P060", "set_field", "section.corrosion_allowance.value", &pipe["section"]["corrosion_allowance"]["value"].to_string(), "{\"value\":2,\"unit\":\"mm\"}", "mm", "length"),
        agent("modify", "Material", "material:RS-M1", "set_field", "elastic_modulus.value", &material["elastic_modulus"]["value"].to_string(), "{\"value\":201,\"unit\":\"GPa\"}", "GPa", "stress"),
        none("modify", "Material", "material:RS-M1", "set_field", "constitutive_properties", &canonical_json(&json!({"constitutive_basis":material["constitutive_basis"],"elastic_modulus":material["elastic_modulus"],"poisson_ratio":material["poisson_ratio"],"thermal_expansion_coefficient":material["thermal_expansion_coefficient"],"provenance":material["provenance"]})),
            &canonical_json(&json!({"constitutive_basis":material["constitutive_basis"],"elastic_modulus":material["elastic_modulus"],"poisson_ratio":material["poisson_ratio"],"thermal_expansion_coefficient":material["thermal_expansion_coefficient"],"provenance":"agent-stated source"}))),
        agent("create", "Section", "section:agent-S270", "create_section", "sections", "not_present", &canonical_json(&json!({"id":"section:agent-S270","name":"agent-S270","section_type":"pipe","properties":section["properties"],"provenance":"agent-stated source"})), "mm", "length"),
        agent("modify", "Project", project, "set_project_unit", "project.units.force", "N", "kN", "kN", "force"),
    ];
    for op in edits {
        let applied = propose(&model, vec![op.clone()]);
        assert_ne!(canonical_json(&applied), canonical_json(&model), "{}", op["change"]["field_path"]);
    }
}

const M2: &str = include_str!("../../../../fixtures/model_operations/agent_m2_component_journey.json");
const DEMO: &str = include_str!("../../../../fixtures/product_preview/invented_demo_model.json");

/// A valid agent-authored create of `change_kind` and the document it applies to,
/// from committed fixtures: the M2 component journey's own operations (applied in
/// order up to the first of that kind), the reference system, or the pressure-free
/// demo model. A create kind the catalog admits without an entry here fails the test.
fn valid_creates(change_kind: &str) -> Vec<(Value, Value)> {
    let m2: Value = serde_json::from_str(M2).unwrap();
    let journey: Vec<Value> = m2["proposals"].as_array().unwrap().iter()
        .filter(|p| p["phase"] != "corrections")
        .flat_map(|p| p["operations"].as_array().unwrap().clone()).collect();
    if journey.iter().any(|op| op["change"]["change_kind"] == change_kind) {
        let mut model = m2["blank"]["model"].clone();
        let mut found = Vec::new();
        for op in &journey {
            if op["change"]["change_kind"] == change_kind {
                found.push((model.clone(), op.clone()));
            }
            model = applies(&model, op).unwrap_or_else(|| panic!("the journey's {} applies", op["operation_id"]));
        }
        return found;
    }
    let reference = reference();
    let demo: Value = serde_json::from_str(DEMO).unwrap();
    let record = |kind: &str, object: &str, id: &str, path: &str, after: Value| none(kind, object, id, change_kind, path, "not_present", &canonical_json(&after));
    match change_kind {
        "create_section" => vec![(reference.clone(), agent("create", "Section", "section:agent-S1", "create_section", "sections", "not_present",
            &canonical_json(&json!({"id":"section:agent-S1","name":"agent-S1","section_type":"pipe","properties":find(&reference, "sections", "section:RS-S270")["properties"],"provenance":"agent-stated source"})), "mm", "length"))],
        "create_combination" => vec![(reference.clone(), record("create", "Combination", "combination:agent-M", "combinations",
            json!({"id":"combination:agent-M","label":"agent M","basis":"mechanics","terms":[{"load_case":"load:RS-SUS","factor":1}],"provenance":"agent-stated source"})))],
        "create_combination_term" => {
            let with = propose(&reference, vec![none("create", "Combination", "combination:agent-M", "create_combination", "combinations", "not_present",
                &canonical_json(&json!({"id":"combination:agent-M","label":"agent M","basis":"mechanics","terms":[{"load_case":"load:RS-SUS","factor":1}],"provenance":"agent-stated source"})))]);
            vec![(with, record("create", "Combination", "combination:agent-M", "terms", json!({"load_case":"load:RS-OPE","factor":1.5})))]
        }
        "create_condition_set" => vec![(demo.clone(), record("create", "ConditionSet", "cs:agent", "condition_sets",
            json!({"id":"cs:agent","label":"agent set","provenance":"agent-stated source","installation_temperature":{"value":20,"unit":"degC"},"conditions":[{"key":"OPE1","temperature":{"value":150,"unit":"degC"}}],"exposure":{"wind":false}})))],
        "add_rule_pack_ref" => vec![(demo.clone(), record("create", "RulePackRef", "invented_private_pack", "rule_pack_refs",
            json!({"rule_pack_id":"invented_private_pack","name":"Invented pack","rule_pack_version":"0.1.0","checksum":{"algorithm":"sha256","canonicalization":"JCS","payload_scope":"rule_pack_payload","payload_excludes":["checksums"],"value":"a".repeat(64)},"redistribution_status":"private_only"})))],
        other => panic!("the catalog admits create kind {other}; give it a case here"),
    }
}

fn applies(model: &Value, op: &Value) -> Option<Value> {
    let outcome = apply_operation(model, &stamped(op), Some(&hash(model)));
    outcome.applied_model
}

/// What the engine does, for an operation the person did not author, with a person-entered
/// member in the record each create kind writes (DEL-16-06 AC-006): refuses the operation by the
/// person-entry code, refuses it as an invalid payload, refuses it as a slot-shape error, or
/// writes the record without the member. Stated per kind and member; an unlisted kind fails.
#[derive(Debug, PartialEq)]
enum Behaviour {
    RefusedByName(&'static str),
    RefusedPayload(&'static str),
    RefusedSlotShape(&'static str),
    WrittenWithout,
}
fn expected(change_kind: &str, component_kind: &str, member: &str) -> Behaviour {
    use Behaviour::*;
    match (change_kind, member) {
        ("insert_component_symbol", "modifiers") => RefusedByName(MODIFIER),
        ("insert_component_symbol", "curved interface") => match component_kind {
            "reducer" | "valve" | "flange" => RefusedPayload("OP-CREATE-RIGID-COMPONENT-CONSUMPTION-INVALID"),
            "expansion_joint" => RefusedPayload("OP-CREATE-EXPANSION-JOINT-ANNOTATION-ONLY"),
            "bend" | "tee" => WrittenWithout,
            other => panic!("no stated behaviour for a {other} with a curved interface"),
        },
        ("insert_component_symbol", "bend_realization") => WrittenWithout,
        ("create_support", _) => RefusedPayload("OP-CREATE-SUPPORT-PAYLOAD-INVALID"),
        ("create_condition_set" | "add_rule_pack_ref", _) => RefusedSlotShape("MODEL-SLOT-SHAPE"),
        ("create_material" | "create_node" | "connect_pipe_run" | "create_load_case" | "create_primitive_load" | "create_section"
            | "create_combination" | "create_combination_term", "modifiers") => WrittenWithout,
        (other, member) => panic!("no stated behaviour for {other} with {member}; state it here"),
    }
}

#[test]
fn every_admitted_create_kind_refuses_or_drops_a_person_entered_member() {
    // The create kinds are read from the catalog's rows, so a newly admitted one is covered or fails here.
    let creates: Vec<String> = catalog()["admittedOperations"].as_array().unwrap().iter()
        .filter(|row| matches!(row["operationKind"].as_str().unwrap(), "create" | "insert" | "connect"))
        .map(|row| row["changeKind"].as_str().unwrap().to_string()).collect();
    assert_eq!(creates.len(), 12, "{creates:?}");
    let authoring: Value = serde_json::from_str(AUTHORING).unwrap();
    let person = &authoring["bends"][0]["person_inputs"];
    let modifiers = json!({"flexibility_factor_user_value": person["flexibility_factor"], "sif_user_value": person["sif_in_plane_user_value"],
        "sif_in_plane_user_value": person["sif_in_plane_user_value"], "source_reference": person["sif_source_reference"]});
    let mut covered = Vec::new();
    for change_kind in &creates {
        for (model, op) in valid_creates(change_kind) {
            let clean = applies(&model, &op).unwrap_or_else(|| panic!("{change_kind}: the clean create must apply"));
            let after: Value = serde_json::from_str(op["change"]["after"].as_str().unwrap()).unwrap();
            let component_kind = after["kind"].as_str().unwrap_or("").to_string();
            let mut attempts = vec![("modifiers", { let mut a = after.clone(); a["modifiers"] = modifiers.clone(); a })];
            if change_kind == "insert_component_symbol" {
                // A component may also carry its realization: a curved macro-element interface, or a bend_realization member.
                attempts.push(("curved interface", { let mut a = after.clone(); a["mechanics_interface"] = json!({"solver_consumption":"curved_bend_macro_element"}); a }));
                attempts.push(("bend_realization", { let mut a = after.clone(); a["bend_realization"] = json!({"radius": person["flexibility_factor"]}); a }));
            }
            for (member, carried) in attempts {
                let mut attempt = op.clone();
                attempt["change"]["after"] = json!(canonical_json(&carried));
                let outcome = apply_operation(&model, &stamped(&attempt), Some(&hash(&model)));
                let codes: Vec<String> = outcome.diagnostics.iter().map(|d| d.code.clone()).collect();
                let label = format!("{change_kind} {} ({component_kind}) with {member}: {codes:?}", op["target"]["ref"]);
                match expected(change_kind, &component_kind, member) {
                    Behaviour::RefusedByName(code) | Behaviour::RefusedPayload(code) | Behaviour::RefusedSlotShape(code) => {
                        assert!(outcome.applied_model.is_none(), "{label}: expected a refusal");
                        assert!(codes.iter().any(|c| c == code), "{label}: expected {code}");
                    }
                    Behaviour::WrittenWithout => {
                        let applied = outcome.applied_model.unwrap_or_else(|| panic!("{label}: expected the record written without the member"));
                        assert_eq!(canonical_json(&applied), canonical_json(&clean), "{label}: the record must be exactly the clean create's");
                    }
                }
            }
            covered.push(change_kind.clone());
        }
    }
    covered.dedup();
    assert_eq!(covered, creates, "every admitted create kind has a case");
}

#[test]
fn a_removal_applied_alone_leaves_a_case_the_solve_refuses_by_name() {
    // Replacing a stored analysis state is two proposals: its removal, then the
    // new state. If the person applies only the removal, the preview shows the
    // state going to not_present, and the solve refuses the case by name.
    let fixture = reference();
    let state = canonical_json(&find(&fixture, "load_cases", "load:RS-OPE")["analysis_state"]);
    let removal = none("modify", "Load", "load:RS-OPE", "update_load", "analysis_state", &state, "not_present");
    let claim = hash(&fixture);
    let batch = json!({"batch_id":"agent-removal","operations":[stamped(&removal)],"source_model_hash":claim["value"]});
    let preview = validate_operation_batch(&fixture, &batch, Some(&claim));
    assert_eq!(preview["validation"]["batch_validation_status"], "passed", "{:#}", preview["diagnostics"]);
    let rows = preview["diff_preview"].as_array().or_else(|| preview["operation_outcomes"][0]["diff_preview"].as_array()).cloned().unwrap_or_default();
    assert!(rows.iter().any(|r| r["field_path"] == "analysis_state" && r["after"] == "not_present"), "{preview:#}");
    let removed = propose(&fixture, vec![removal]);
    let out = run_linear_static_preview_value_with_mode(json!({"model": removed, "materials": []}), PreviewSolverMode::SparseInteractive)
        .expect("the typed boundary admits the document");
    let refusal = out.diagnostics.iter().find(|d| d.code == "LOAD_STATE_ANALYSIS_STATE_REQUIRED").unwrap_or_else(|| panic!("{:?}", out.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>()));
    assert_eq!(refusal.severity, "blocking");
    assert!(refusal.affected_refs.iter().any(|r| r == "load:RS-OPE"), "{:?}", refusal.affected_refs);
    assert_ne!(out.status.mechanics, "MECHANICS_SOLVED");
}
