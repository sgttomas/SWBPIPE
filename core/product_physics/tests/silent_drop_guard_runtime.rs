//! No silent wrong results (Agent 0, 2026-10-23): authored input or combination
//! rows are never dropped without a name. Every request goes through the captured
//! value entry the desktop uses, in both solver modes.
//!
//! - Closed model records: an unknown key on the model root, a load case (its
//!   equivalent_static inputs and solve temperature), a primitive load (its
//!   target and magnitude), a combination or a combination term refuses by name
//!   (`UNKNOWN_MODEL_FIELD`) before anything is solved. So does one on a node, a
//!   pipe segment (its section and `y_reference`), a section, a support (its
//!   stiffness, hanger and nonlinear inputs), a component (its geometry,
//!   modifiers, mechanics interface and objective connector), a model or request
//!   material (its temperature points), or any quantity or vector they carry.
//! - A `mechanics` combination naming one load case twice blocks by name
//!   (`LOAD_COMBINATION_DUPLICATE_TERM`) instead of losing every combined row.
//! - Controls: well-formed documents (including the declared, solve-ignored
//!   `data_boundary`, `diagnostics` and load-case `kind`/`label`/`status`, and
//!   model document 0.7.0's root `lines`, `unresolved_assumptions`,
//!   `constraints`, `equipment_interfaces` and `rule_pack_refs`, the load-case,
//!   combination and nested records' `tags`/`notes`, and the nested records'
//!   other declared members) publish, byte-identically whatever their values or
//!   presence; a combination that publishes keeps its rows and diagnostics.
//!   Setting `SILENT_DROP_GUARD_DUMP=<dir>` writes the control envelopes' exact
//!   bytes there, for a byte comparison against another tree.
//! - A record that is not an object (a positional array, null or a scalar)
//!   refuses by its path (`MODEL_RECORD_NOT_OBJECT`) before anything is solved.
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, LinearStaticPreviewRequest, MechanicsEnvelope,
    PreviewSolverMode,
};
use serde_json::{json, Value};

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const DEMO: &str = include_str!("../../../fixtures/product_preview/invented_demo_model.json");
const COMBINATION: &str = "combination:C-OPER-ALT";

/// The bundled demo: the desktop's default session model (nonlinear supports,
/// so its combination is gated).
fn demo() -> Value {
    serde_json::from_str(DEMO).unwrap()
}

/// The demo with only its linear (anchor and guide) supports, so its
/// combination publishes combined rows.
fn linear() -> Value {
    let mut model = demo();
    model["supports"]
        .as_array_mut()
        .unwrap()
        .retain(|support| matches!(support["family"].as_str(), Some("anchor" | "guide")));
    model
}

fn request(model: &Value) -> Value {
    json!({"model": model, "materials": []})
}

fn solve(model: &Value, mode: PreviewSolverMode) -> Result<MechanicsEnvelope, String> {
    run_linear_static_preview_value_with_mode(request(model), mode)
}

fn refusal(model: &Value, mode: PreviewSolverMode, what: &str) -> String {
    match solve(model, mode) {
        Ok(envelope) => panic!(
            "{what}/{mode:?}: an unknown key must refuse, not publish (mechanics {}, {} rows)",
            envelope.status.mechanics,
            envelope.results.len()
        ),
        Err(error) => error,
    }
}

fn combination_rows(envelope: &MechanicsEnvelope) -> usize {
    envelope
        .results
        .iter()
        .filter(|row| row.basis_ref.as_ref().is_some_and(|basis| basis.ref_type == "combination"))
        .count()
}

fn codes(envelope: &MechanicsEnvelope) -> Vec<&str> {
    envelope.diagnostics.iter().map(|d| d.code.as_str()).collect()
}

fn dump(name: &str, mode: PreviewSolverMode, envelope: &MechanicsEnvelope) {
    if let Some(dir) = std::env::var_os("SILENT_DROP_GUARD_DUMP") {
        let path = std::path::Path::new(&dir).join(format!("{name}.{}.json", mode.as_str()));
        std::fs::write(path, serde_json::to_vec(envelope).unwrap()).unwrap();
    }
}

/// (what, edit, named field, named record, path)
type Probe = (&'static str, fn(&mut Value), &'static str, &'static str, &'static str);

fn probes() -> Vec<Probe> {
    vec![
        (
            "misspelled primitive_loads empties a case",
            |m| {
                let case = m["load_cases"][1].as_object_mut().unwrap();
                let loads = case.remove("primitive_loads").unwrap();
                case.insert("primitive_load".into(), loads);
            },
            "primitive_load",
            "load case `load:L-200`",
            "model.load_cases[1]",
        ),
        (
            "misspelled combinations drops every combination",
            |m| {
                let root = m.as_object_mut().unwrap();
                let combinations = root.remove("combinations").unwrap();
                root.insert("combination".into(), combinations);
            },
            "combination",
            "the model",
            "model",
        ),
        (
            "unknown key on a primitive load",
            |m| m["load_cases"][0]["primitive_loads"][1]["scale"] = json!(2.0),
            "scale",
            "primitive load `load:L-100-Y` in load case `load:L-100`",
            "model.load_cases[0].primitive_loads[1]",
        ),
        (
            "unknown key on a primitive-load target",
            |m| m["load_cases"][0]["primitive_loads"][1]["target"]["pipe"] = json!("pipe:P-120"),
            "pipe",
            "the target of primitive load `load:L-100-Y` in load case `load:L-100`",
            "model.load_cases[0].primitive_loads[1].target",
        ),
        (
            "unknown key on a primitive-load magnitude",
            |m| m["load_cases"][0]["primitive_loads"][1]["magnitude"]["factor"] = json!(0.5),
            "factor",
            "the magnitude of primitive load `load:L-100-Y` in load case `load:L-100`",
            "model.load_cases[0].primitive_loads[1].magnitude",
        ),
        (
            "misspelled exposed_pipe_refs drops the wind span",
            |m| m["load_cases"][0]["equivalent_static"] = json!({"wind": {"pressure": {"value": 500.0, "unit": "Pa"}, "shape_factor": {"value": 1.0, "unit": "1"}, "direction": "global_y", "exposed_pipe_ref": ["pipe:P-120"]}, "provenance": "invented"}),
            "exposed_pipe_ref",
            "the wind input of load case `load:L-100`",
            "model.load_cases[0].equivalent_static.wind",
        ),
        (
            "unknown key on a solve temperature",
            |m| m["load_cases"][0]["modulus_basis_temperature"] = json!({"value": 150.0, "unit": "degC", "basis": "guessed"}),
            "basis",
            "the modulus_basis_temperature of load case `load:L-100`",
            "model.load_cases[0].modulus_basis_temperature",
        ),
        (
            "unknown key on a combination",
            |m| m["combinations"][0]["factors"] = json!([1.0]),
            "factors",
            "combination `combination:C-OPER-ALT`",
            "model.combinations[0]",
        ),
        (
            "unknown key on a combination term",
            |m| m["combinations"][0]["terms"][1]["load_case_ref"] = json!("load:L-100"),
            "load_case_ref",
            "term 1 of combination `combination:C-OPER-ALT`",
            "model.combinations[0].terms[1]",
        ),
    ]
}

#[test]
fn an_unknown_key_on_a_closed_record_refuses_by_field_and_record() {
    for base in [demo(), linear()] {
        for (what, edit, field, record, path) in probes() {
            let mut model = base.clone();
            edit(&mut model);
            for mode in MODES {
                let error = refusal(&model, mode, what);
                assert!(error.starts_with("UNKNOWN_MODEL_FIELD: "), "{what}/{mode:?}: {error}");
                let named = format!("unknown field `{field}` in {record}");
                assert!(error.contains(&named), "{what}/{mode:?}: expected `{named}` in {error}");
                assert!(error.contains(&format!("{named} ({path})")), "{what}/{mode:?}: {error}");
            }
            // Every typed reader refuses the same key (the records deny unknown fields).
            let typed = serde_json::from_value::<LinearStaticPreviewRequest>(request(&model)).unwrap_err().to_string();
            assert!(typed.contains(&format!("unknown field `{field}`")), "{what}: {typed}");
        }
    }
}

fn nested_probes() -> Vec<Probe> {
    vec![
        ("misspelled node label", |m| m["nodes"][2]["lable"] = json!("Elbow"), "lable", "node `node:N-120`", "model.nodes[2]"),
        ("unknown key on a node position", |m| m["nodes"][1]["position"]["w"] = json!(0.0), "w", "the position of node `node:N-110`", "model.nodes[1].position"),
        ("unknown key on a pipe segment", |m| m["pipe_segments"][0]["sectionref"] = json!("section:S"), "sectionref", "pipe segment `pipe:P-100`", "model.pipe_segments[0]"),
        (
            "misspelled mill_tolerance solves without it",
            |m| m["pipe_segments"][1]["section"]["mill_tolerence"] = json!({"value": 0.001, "unit": "m"}),
            "mill_tolerence",
            "the section of pipe segment `pipe:P-110`",
            "model.pipe_segments[1].section",
        ),
        (
            "unknown key on a section quantity",
            |m| m["pipe_segments"][0]["section"]["wall_thickness"]["tolerance"] = json!(0.125),
            "tolerance",
            "the wall_thickness of the section of pipe segment `pipe:P-100`",
            "model.pipe_segments[0].section.wall_thickness",
        ),
        ("unknown key on a y_reference", |m| m["pipe_segments"][3]["y_reference"]["unit"] = json!("m"), "unit", "the y_reference of pipe segment `pipe:P-130`", "model.pipe_segments[3].y_reference"),
        (
            "unknown key on a section of the library",
            |m| m["sections"] = json!([{"id": "section:S", "name": "s", "section_type": "pipe", "properties": {}, "provenance": "p", "lable": "s"}]),
            "lable",
            "section `section:S`",
            "model.sections[0]",
        ),
        (
            "unknown key on a section property",
            |m| m["sections"] = json!([{"id": "section:S", "name": "s", "section_type": "pipe", "properties": {"outside_diameter": {"value": 0.168, "unit": "m", "grade": "x"}}, "provenance": "p"}]),
            "grade",
            "the outside_diameter of the properties of section `section:S`",
            "model.sections[0].properties.outside_diameter",
        ),
        ("misspelled restraints", |m| m["supports"][0]["restraint"] = json!(["UX"]), "restraint", "support `support:S-100`", "model.supports[0]"),
        (
            "unknown key on a support stiffness",
            |m| m["supports"][1]["stiffness"] = json!({"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}, "direction": "UY"}),
            "direction",
            "the stiffness of support `support:S-120`",
            "model.supports[1].stiffness",
        ),
        ("misspelled hanger load", |m| m["supports"][5]["hanger"]["hot_lode"] = json!({"value": 390.0, "unit": "N"}), "hot_lode", "the hanger of support `support:SH-140`", "model.supports[5].hanger"),
        (
            "unknown key on a hanger stiffness",
            |m| m["supports"][5]["hanger"]["stiffness"]["direction"] = json!("UZ"),
            "direction",
            "the stiffness of the hanger of support `support:SH-140`",
            "model.supports[5].hanger.stiffness",
        ),
        (
            "unknown key on a hanger stiffness value",
            |m| m["supports"][5]["hanger"]["stiffness"]["value"]["scale"] = json!(2.0),
            "scale",
            "the value of the stiffness of the hanger of support `support:SH-140`",
            "model.supports[5].hanger.stiffness.value",
        ),
        (
            "unknown key on a hanger quantity",
            |m| m["supports"][6]["hanger"]["constant_load"]["basis"] = json!("cold"),
            "basis",
            "the constant_load of the hanger of support `support:CE-120`",
            "model.supports[6].hanger.constant_load",
        ),
        ("misspelled nonlinear gap", |m| m["supports"][3]["nonlinear"]["gap_size"] = json!({"value": 0.0, "unit": "m"}), "gap_size", "the nonlinear of support `support:NL-140`", "model.supports[3].nonlinear"),
        (
            "unknown key on a normal reaction source",
            |m| m["supports"][4]["nonlinear"]["normal_reaction_source"]["direction"] = json!("UY"),
            "direction",
            "the normal_reaction_source of the nonlinear of support `support:NL-130-FRIC`",
            "model.supports[4].nonlinear.normal_reaction_source",
        ),
        (
            "unknown key on a friction coefficient",
            |m| m["supports"][4]["nonlinear"]["friction_coefficient"]["static"] = json!(true),
            "static",
            "the friction_coefficient of the nonlinear of support `support:NL-130-FRIC`",
            "model.supports[4].nonlinear.friction_coefficient",
        ),
        ("misspelled component kind", |m| m["components"][0]["kindd"] = json!("bend"), "kindd", "component `component:C-110`", "model.components[0]"),
        ("misspelled bend radius", |m| m["components"][0]["geometry"]["bend_raduis"] = json!({"value": 0.45, "unit": "m"}), "bend_raduis", "the geometry of component `component:C-110`", "model.components[0].geometry"),
        (
            "unknown key on a bend radius",
            |m| m["components"][0]["geometry"]["bend_radius"]["basis"] = json!("long"),
            "basis",
            "the bend_radius of the geometry of component `component:C-110`",
            "model.components[0].geometry.bend_radius",
        ),
        (
            "unknown key on a center of gravity",
            |m| m["components"][1]["geometry"]["center_of_gravity"]["w"] = json!(0.0),
            "w",
            "the center_of_gravity of the geometry of component `component:C-130`",
            "model.components[1].geometry.center_of_gravity",
        ),
        ("misspelled user SIF", |m| m["components"][0]["modifiers"]["sif_user_valu"] = json!({"value": 1.3, "unit": "none"}), "sif_user_valu", "the modifiers of component `component:C-110`", "model.components[0].modifiers"),
        (
            "unknown key on a modifier quantity",
            |m| m["components"][1]["modifiers"]["linear_stiffness_user_value"]["axis"] = json!("x"),
            "axis",
            "the linear_stiffness_user_value of the modifiers of component `component:C-130`",
            "model.components[1].modifiers.linear_stiffness_user_value",
        ),
        (
            "unknown key on a mechanics interface",
            |m| m["components"][0]["mechanics_interface"]["solver"] = json!("x"),
            "solver",
            "the mechanics_interface of component `component:C-110`",
            "model.components[0].mechanics_interface",
        ),
        (
            "unknown key on an objective connector",
            |m| m["components"][2]["objective_connector"] = json!({"version": "1.0.0", "stifness": {}}),
            "stifness",
            "the objective_connector of component `component:C-140`",
            "model.components[2].objective_connector",
        ),
        ("misspelled elastic modulus", |m| m["materials"][0]["elastic_modulos"] = json!({"value": 1.9e11, "unit": "Pa"}), "elastic_modulos", "material `material:invented-carbon-steel`", "model.materials[0]"),
        (
            "unknown key on a material quantity",
            |m| m["materials"][0]["elastic_modulus"]["temperature"] = json!(20.0),
            "temperature",
            "the elastic_modulus of material `material:invented-carbon-steel`",
            "model.materials[0].elastic_modulus",
        ),
        (
            "misspelled temperature point temperature",
            |m| m["materials"][0]["temperature_points"] = json!([{"id": "point:hot", "tempreature": {"value": 120.0, "unit": "degC"}}]),
            "tempreature",
            "temperature point `point:hot` of material `material:invented-carbon-steel`",
            "model.materials[0].temperature_points[0]",
        ),
        (
            "unknown key on a temperature point quantity",
            |m| m["materials"][0]["temperature_points"] = json!([{"id": "point:hot", "elastic_modulus": {"value": 1.9e11, "unit": "Pa", "at": "hot"}}]),
            "at",
            "the elastic_modulus of temperature point `point:hot` of material `material:invented-carbon-steel`",
            "model.materials[0].temperature_points[0].elastic_modulus",
        ),
        (
            "unknown key on an optional section quantity",
            |m| m["pipe_segments"][2]["section"]["insulation_density"] = json!({"value": 100.0, "unit": "kg/m^3", "vaule": 1.0}),
            "vaule",
            "the insulation_density of the section of pipe segment `pipe:P-120`",
            "model.pipe_segments[2].section.insulation_density",
        ),
    ]
}

#[test]
fn an_unknown_key_on_a_nested_record_refuses_by_field_record_and_path() {
    let base = demo();
    for (what, edit, field, record, path) in nested_probes() {
        let mut model = base.clone();
        edit(&mut model);
        for mode in MODES {
            let error = refusal(&model, mode, what);
            let named = format!("unknown field `{field}` in {record} ({path})");
            assert!(error.starts_with("UNKNOWN_MODEL_FIELD: request: "), "{what}/{mode:?}: {error}");
            assert!(error.contains(&named), "{what}/{mode:?}: expected `{named}` in {error}");
            assert_eq!(error.matches("unknown field").count(), 1, "{what}/{mode:?}: {error}");
        }
        let typed = serde_json::from_value::<LinearStaticPreviewRequest>(request(&model)).unwrap_err().to_string();
        assert!(typed.contains(&format!("unknown field `{field}`")), "{what}: {typed}");
    }
}

#[test]
fn an_unknown_key_on_a_request_material_refuses_by_field_record_and_path() {
    let material = json!({"id": "material:R", "elastic_modulus": {"value": 2.0e11, "unit": "Pa"}, "poison_ratio": {"value": 0.3, "unit": "1"}});
    let request = json!({"model": demo(), "materials": [material]});
    for mode in MODES {
        let error = run_linear_static_preview_value_with_mode(request.clone(), mode).map(drop).unwrap_err();
        assert!(error.contains("unknown field `poison_ratio` in request material `material:R` (materials[0])"), "{mode:?}: {error}");
    }
    let typed = serde_json::from_value::<LinearStaticPreviewRequest>(request).unwrap_err().to_string();
    assert!(typed.contains("unknown field `poison_ratio`"), "{typed}");
}

/// The desktop's hand-edit refusal (`apps/desktop/src/features/workspace/solveJobResource.session.test.tsx`,
/// `HAND_EDIT_REFUSAL`): that session's model with one misspelled section key is refused with exactly this text.
#[test]
fn the_desktop_hand_edit_is_refused_with_this_text() {
    const HAND_EDIT_REFUSAL: &str = "UNKNOWN_MODEL_FIELD: request: unknown field `mill_tolerence` in the section of pipe segment `pipe:P-110` (model.pipe_segments[1].section). The model document does not define these keys, so they would be ignored; nothing is solved. Correct or remove each key.";
    let mut model: Value = serde_json::from_str(include_str!("fixtures/preview_physics_invented_model.json")).unwrap();
    model["pipe_segments"][1]["section"]["mill_tolerence"] = json!({"value": 0.0005, "unit": "m"});
    for mode in MODES {
        assert_eq!(refusal(&model, mode, "the desktop's hand edit"), HAND_EDIT_REFUSAL, "{mode:?}");
    }
}

#[test]
fn a_positional_record_is_refused_by_its_path() {
    let mut model = demo();
    model["nodes"][2] = json!(["node:N-120", {"x": 0, "y": 0, "z": 0, "extra": 1}, null, null, null, null]);
    for mode in MODES {
        let error = refusal(&model, mode, "a positional node");
        assert!(error.starts_with("MODEL_RECORD_NOT_OBJECT: request: model.nodes[2] is an array (a positional record). "), "{mode:?}: {error}");
    }
    // A positional record that serde would read: refused by the scan all the same.
    let mut short = demo();
    short["nodes"][2] = json!(["node:N-120", {"x": 0, "y": 0, "z": 0}, null]);
    assert!(serde_json::from_value::<LinearStaticPreviewRequest>(request(&short)).is_ok(), "serde reads the positional form");
    for mode in MODES {
        assert!(refusal(&short, mode, "a readable positional node").starts_with("MODEL_RECORD_NOT_OBJECT: "), "{mode:?}");
    }
}

#[test]
fn every_unknown_key_is_named_with_a_bounded_list() {
    let mut model = linear();
    for index in 0..12 {
        model["combinations"][0][format!("extra_{index:02}")] = json!(index);
    }
    for mode in MODES {
        let error = refusal(&model, mode, "twelve unknown keys");
        assert_eq!(error.matches("unknown field `extra_").count(), 8, "{error}");
        assert!(error.contains("; and 4 more."), "{error}");
    }
}

#[test]
fn a_duplicate_mechanics_term_blocks_by_name() {
    let mut model = linear();
    model["combinations"][0]["terms"][1]["load_case"] = json!("load:L-100");
    for mode in MODES {
        let envelope = solve(&model, mode).unwrap();
        let duplicate = envelope
            .diagnostics
            .iter()
            .find(|d| d.code == "LOAD_COMBINATION_DUPLICATE_TERM")
            .unwrap_or_else(|| panic!("{mode:?}: no duplicate-term refusal; diagnostics {:?}", codes(&envelope)));
        assert_eq!(duplicate.severity, "blocking", "{mode:?}");
        assert_eq!(duplicate.affected_refs, vec![COMBINATION.to_string(), "load:L-100".to_string()], "{mode:?}");
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE", "{mode:?}");
        assert!(envelope.results.is_empty(), "{mode:?}: a blocked request publishes no rows");
    }
}

#[test]
fn three_repeated_terms_emit_unique_diagnostic_ids() {
    let mut model = linear();
    let term = model["combinations"][0]["terms"][0].clone();
    model["combinations"][0]["terms"] = json!([term.clone(), term.clone(), term]);
    for mode in MODES {
        let envelope = solve(&model, mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        assert!(codes(&envelope).contains(&"LOAD_COMBINATION_DUPLICATE_TERM"));
        let ids: Vec<&str> = envelope.diagnostics.iter().map(|d| d.id.as_str()).collect();
        let unique: std::collections::HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "{mode:?}: emitted diagnostic ids must be unambiguous");
    }
}

#[test]
fn three_repeated_range_operands_emit_unique_diagnostic_ids() {
    let mut model = linear();
    let case = model["combinations"][0]["terms"][0]["load_case"].clone();
    model["combinations"] = json!([{"id": "combination:range", "basis": "range_envelope",
        "operand_ids": [case.clone(), case.clone(), case], "mode": "max", "provenance": "silent-drop"}]);
    for mode in MODES {
        let envelope = solve(&model, mode).unwrap();
        assert_eq!(envelope.status.mechanics, "MODEL_INCOMPLETE");
        let duplicates = envelope.diagnostics.iter().filter(|d| d.code == "LOAD_COMBINATION_DUPLICATE_TERM").count();
        assert_eq!(duplicates, 1, "{mode:?}: one diagnostic per repeated operand");
        let blocking: Vec<&str> = envelope.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| d.code.as_str()).collect();
        assert_eq!(blocking, ["LOAD_COMBINATION_DUPLICATE_TERM"], "{mode:?}: the repeat is the only refusal");
        let ids: Vec<&str> = envelope.diagnostics.iter().map(|d| d.id.as_str()).collect();
        let unique: std::collections::HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "{mode:?}: emitted diagnostic ids must be unambiguous");
    }
}

#[test]
fn well_formed_documents_with_declared_ignored_members_publish() {
    // The demo carries data_boundary, diagnostics and load-case kind/label/status.
    let demo = demo();
    assert!(demo.get("data_boundary").is_some() && demo.get("diagnostics").is_some());
    for key in ["kind", "label", "status"] {
        assert!(demo["load_cases"][0].get(key).is_some(), "{key}");
    }
    for (name, model, combined) in [("demo", demo.clone(), false), ("linear", linear(), true)] {
        for mode in MODES {
            let envelope = solve(&model, mode).unwrap_or_else(|e| panic!("{name}/{mode:?}: {e}"));
            assert_eq!(envelope.status.mechanics, "MECHANICS_SOLVED", "{name}/{mode:?}");
            let diagnostics = codes(&envelope);
            for code in ["LOAD_COMBINATION_ROW_WITHHELD", "LOAD_COMBINATION_SOURCE_RESULT_MISSING", "LOAD_COMBINATION_VECTOR_INVALID"] {
                assert!(!diagnostics.contains(&code), "{name}/{mode:?}: {code}");
            }
            assert_eq!(combination_rows(&envelope) > 0, combined, "{name}/{mode:?}");
            dump(name, mode, &envelope);
        }
    }
    // The declared members are ignored by the solve: their values do not move a byte.
    let mut varied = linear();
    varied["data_boundary"] = json!({"public_examples_policy": "varied"});
    varied["diagnostics"] = json!([]);
    for case in varied["load_cases"].as_array_mut().unwrap() {
        case["kind"] = json!("varied");
        case["label"] = json!("varied");
        case["status"] = json!("varied");
        case["tags"] = json!(["varied"]);
        case["notes"] = json!("varied");
    }
    // Model document 0.7.0's declared members (accepted and ignored).
    for member in ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"] {
        varied[member] = json!([{"id": format!("{member}:varied")}]);
    }
    for combination in varied["combinations"].as_array_mut().unwrap() {
        combination["tags"] = json!(["varied"]);
        combination["notes"] = json!("varied");
    }
    // The nested records' declared members, varied and removed.
    let mut stripped = linear();
    for (collection, members) in NESTED_DECLARED {
        for (model, value) in [(&mut varied, Some(json!({"varied": [1, "two"]}))), (&mut stripped, None)] {
            for record in model[collection].as_array_mut().unwrap() {
                let record = record.as_object_mut().unwrap();
                for member in members {
                    match &value {
                        Some(value) => drop(record.insert(member.to_string(), value.clone())),
                        None => drop(record.remove(*member)),
                    }
                }
            }
        }
    }
    assert!(linear()["nodes"][0].get("label").is_some() && stripped["nodes"][0].get("label").is_none());
    for mode in MODES {
        let base = serde_json::to_vec(&solve(&linear(), mode).unwrap()).unwrap();
        let other = serde_json::to_vec(&solve(&varied, mode).unwrap()).unwrap();
        let without = serde_json::to_vec(&solve(&stripped, mode).unwrap()).unwrap();
        assert_eq!(base, other, "{mode:?}");
        assert_eq!(base, without, "{mode:?}");
    }
}

/// The nested records' declared members the solve never reads (model document
/// 0.7.0), by collection of the demo.
const NESTED_DECLARED: [(&str, &[&str]); 5] = [
    ("nodes", &["label", "tags", "notes"]),
    ("pipe_segments", &["label", "tags", "notes"]),
    ("supports", &["label", "boundary_association", "properties", "tags", "notes", "provenance_record"]),
    ("components", &["completeness", "tags", "notes", "provenance_record"]),
    ("materials", &["label", "tags", "notes", "provenance_record"]),
];
