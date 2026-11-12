//! An invented pressure-free solve request builder shared by the crate's
//! integration tests (`derived_solves.rs`, `publication.rs`).
#![allow(dead_code)]

use serde_json::{json, Value};

pub struct Member<'a> {
    pub id: &'a str,
    pub from: &'a str,
    pub to: &'a str,
    pub outside_diameter_m: f64,
    pub wall_m: f64,
    pub elastic_modulus_pa: f64,
    pub expansion_per_k: f64,
}

pub fn material_id(member: &str) -> String {
    format!("material:{member}")
}

/// An invented pressure-free solve request: `{model, materials}`.
pub fn request(nodes: &[(&str, [f64; 3])], members: &[Member], supports: Vec<Value>, load_cases: Vec<Value>) -> Value {
    let materials: Vec<Value> = members
        .iter()
        .map(|m| json!({
            "id": material_id(m.id),
            "label": "Invented test material",
            "elastic_modulus": {"value": m.elastic_modulus_pa, "unit": "Pa"},
            "shear_modulus": {"value": m.elastic_modulus_pa / 2.6, "unit": "Pa"},
            "thermal_expansion_coefficient": {"value": m.expansion_per_k, "unit": "1/degC"},
            "provenance": "invented_test_value"
        }))
        .collect();
    let pipes: Vec<Value> = members
        .iter()
        .map(|m| {
            let a = nodes.iter().find(|(id, _)| *id == m.from).unwrap().1;
            let b = nodes.iter().find(|(id, _)| *id == m.to).unwrap().1;
            let vertical = (a[0] - b[0]).abs() < 1e-12 && (a[1] - b[1]).abs() < 1e-12;
            let y_reference = if vertical { json!({"x": 1, "y": 0, "z": 0}) } else { json!({"x": 0, "y": 0, "z": 1}) };
            json!({
                "id": m.id, "from": m.from, "to": m.to,
                "section": {"outside_diameter": {"value": m.outside_diameter_m, "unit": "m"}, "wall_thickness": {"value": m.wall_m, "unit": "m"}},
                "material": material_id(m.id), "y_reference": y_reference, "provenance": "invented_test_value"
            })
        })
        .collect();
    let nodes: Vec<Value> = nodes
        .iter()
        .map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": "invented_test_value"}))
        .collect();
    json!({
        "model": {
            "schema_version": "0.9.0",
            "document_kind": "openpipestress.product_preview.model",
            "data_boundary": {
                "public_examples_policy": "invented_or_cleared_data_only",
                "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
                "private_data_policy": "no_private_project_data",
                "professional_boundary": "technical_preview_requires_human_engineering_review"
            },
            "project": {"id": "project:hanger-sizing-test", "name": "Invented hanger sizing test",
                        "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC"}},
            "materials": materials,
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                "rule_check": "not_performed_user_rule_inputs_missing",
                                "professional_acceptance": "not_provided"},
            "nodes": nodes,
            "pipe_segments": pipes,
            "supports": supports,
            "components": [],
            "load_cases": load_cases,
            "combinations": []
        },
        "materials": []
    })
}

pub fn support(id: &str, node: &str, family: &str, restraints: &[&str]) -> Value {
    json!({"id": id, "node": node, "family": family, "restraints": restraints, "provenance": "invented_test_value"})
}

/// A stiff rotational spring that removes a beam's torsional mechanism.
pub fn torsion_spring(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "spring", "restraints": ["RX"],
           "stiffness": {"dof": "RX", "value": {"value": 1.0e9, "unit": "N*m/rad"}}, "provenance": "invented_test_value"})
}

pub fn lift_off(id: &str, node: &str) -> Value {
    json!({"id": id, "node": node, "family": "nonlinear", "restraints": [],
           "nonlinear": {"behavior": "lift_off", "dof": "UZ", "initial_state": "active", "contact_when": "positive_reaction"},
           "provenance": "invented_test_value"})
}

pub fn line_load(id: &str, pipe: &str, w_down_n_per_m: f64) -> Value {
    json!({"id": id, "category": "weight", "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
           "magnitude": {"value": -w_down_n_per_m, "unit": "N/m"}, "dimension": "force_per_length", "provenance": "invented_test_value"})
}

pub fn thermal(id: &str, pipe: &str, delta_t: f64) -> Value {
    json!({"id": id, "category": "thermal", "target": {"type": "element", "pipe": pipe}, "direction": "global_z",
           "magnitude": {"value": delta_t, "unit": "degC"}, "dimension": "temperature_interval", "provenance": "invented_test_value"})
}

pub fn nodal_fz(id: &str, node: &str, fz_n: f64) -> Value {
    json!({"id": id, "category": "concentrated_force", "target": {"type": "node", "node": node}, "direction": "global_z",
           "magnitude": {"value": fz_n, "unit": "N"}, "dimension": "force", "provenance": "invented_test_value"})
}

pub fn case(id: &str, loads: Vec<Value>) -> Value {
    json!({"id": id, "kind": "primitive_user_load", "primitive_loads": loads, "provenance": "invented_test_value"})
}

// --- the exact route (EX) -------------------------------------------------------

/// The exact pressure contract, `3.0.0/exact_pressure_v3`, Euler-Bernoulli.
pub fn exact_contract() -> Value {
    json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"})
}

/// The EX twin of a request built by [`request`]: the same invented model on the
/// exact pressure route. Each section carries an explicit zero corrosion allowance,
/// each material is stated by E and ν (ν = 0.3, which is what [`request`]'s
/// G = E/2.6 gives), and each case has an explicit empty set of pressure regions.
/// Its cases carry no analysis state, so their stored primitives act as on PF.
pub fn exact(mut doc: Value) -> Value {
    let model = &mut doc["model"];
    model["pressure_contract"] = exact_contract();
    for pipe in model["pipe_segments"].as_array_mut().unwrap() {
        pipe["section"]["corrosion_allowance"] = json!({"value": 0.0, "unit": "m"});
    }
    for material in model["materials"].as_array_mut().unwrap() {
        let map = material.as_object_mut().unwrap();
        map.remove("shear_modulus");
        map.insert("constitutive_basis".into(), json!("homogeneous_isotropic_E_nu_v1"));
        map.insert("poisson_ratio".into(), json!({"value": 0.3, "unit": "1"}));
    }
    for case in model["load_cases"].as_array_mut().unwrap() {
        case["pressure_regions"] = json!([]);
    }
    doc
}

/// The load/reference-state twin of an [`exact`] request: every case gets an
/// analysis state whose ledger includes each stored non-thermal primitive at
/// factor 1, with every support active and unmoved. A thermal primitive on a pipe
/// becomes that pipe's constant-α interval state (the ledger cannot carry it);
/// every other pipe keeps its reference state.
pub fn load_reference(mut doc: Value) -> Value {
    let model = &mut doc["model"];
    let pipes: Vec<String> = model["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].as_str().unwrap().to_string()).collect();
    let supports: Vec<String> = model["supports"].as_array().unwrap().iter().map(|s| s["id"].as_str().unwrap().to_string()).collect();
    let material_of = |pipe: &str, model: &Value| -> String {
        model["pipe_segments"].as_array().unwrap().iter().find(|p| p["id"] == pipe).unwrap()["material"].as_str().unwrap().to_string()
    };
    let alpha_of = |material: &str, model: &Value| -> f64 {
        model["materials"].as_array().unwrap().iter().find(|m| m["id"] == material).unwrap()["thermal_expansion_coefficient"]["value"]
            .as_f64().unwrap()
    };
    model["reference_configurations"] = json!([{
        "id": "reference:installed",
        "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": pipes.iter().map(|p| json!({"pipe_ref": p, "basis": {"kind": "direct_strain_reference"},
            "fit": {"kind": "none"}, "provenance": "invented_test_value"})).collect::<Vec<_>>(),
        "provenance": "invented_test_value"
    }]);
    let snapshot = model.clone();
    for case in model["load_cases"].as_array_mut().unwrap() {
        let loads = case["primitive_loads"].as_array().cloned().unwrap_or_default();
        let (thermal, kept): (Vec<Value>, Vec<Value>) = loads.into_iter().partition(|l| l["category"] == "thermal");
        let elements: Vec<Value> = pipes
            .iter()
            .map(|pipe| {
                let material = material_of(pipe, &snapshot);
                let state = match thermal.iter().find(|t| t["target"]["pipe"] == pipe.as_str()) {
                    Some(t) => json!({"kind": "constant_alpha_interval",
                        "coefficient": {"value": alpha_of(&material, &snapshot), "unit": "1/K"},
                        "temperature_change": {"value": t["magnitude"]["value"], "unit": "K"},
                        "coefficient_meaning": "engineering_interval", "provenance": "invented_test_value"}),
                    None => json!({"kind": "unchanged_reference", "provenance": "invented_test_value"}),
                };
                json!({"pipe_ref": pipe,
                       "material_selection": {"kind": "explicit_base_properties", "material_ref": material,
                                              "applicability_reference": "invented_test_value"},
                       "thermal_state": state})
            })
            .collect();
        let sources: Vec<Value> = kept.iter().map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect();
        case["primitive_loads"] = Value::Array(kept);
        case["analysis_state"] = json!({
            "contract": "openpipestress.load_reference_state/1.0.0",
            "reference_configuration_ref": "reference:installed",
            "element_states": elements,
            "support_states": supports.iter().map(|s| json!({"support_ref": s, "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>(),
            "load_sources": sources,
            "history": {"kind": "independent_equilibrium"},
            "provenance": "invented_test_value"
        });
    }
    doc
}

/// The three routes every refusal and sizing test runs on: PF, EX without an
/// analysis state, and EX as a load/reference-state document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    PressureFree,
    Exact,
    ExactLoadReference,
}

pub const ROUTES: [Route; 3] = [Route::PressureFree, Route::Exact, Route::ExactLoadReference];

/// A request built by [`request`] on `route`.
pub fn on(route: Route, doc: Value) -> Value {
    match route {
        Route::PressureFree => doc,
        Route::Exact => exact(doc),
        Route::ExactLoadReference => load_reference(exact(doc)),
    }
}

/// A support's state in one case of a load/reference-state document.
pub fn support_state_mut<'a>(doc: &'a mut Value, case: &str, support: &str) -> &'a mut Value {
    doc["model"]["load_cases"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|c| c["id"] == case)
        .unwrap()["analysis_state"]["support_states"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|s| s["support_ref"] == support)
        .unwrap()
}

/// Adds a support and, on a load/reference-state document, its active state in every case.
pub fn add_support(doc: &mut Value, support: Value) {
    let id = support["id"].clone();
    doc["model"]["supports"].as_array_mut().unwrap().push(support);
    for case in doc["model"]["load_cases"].as_array_mut().unwrap() {
        if let Some(states) = case.pointer_mut("/analysis_state/support_states").and_then(Value::as_array_mut) {
            states.push(json!({"support_ref": id, "participation": {"kind": "active_model_device"}}));
        }
    }
}

/// Adds a stored primitive to a case and, on a load/reference-state document, enters it in the ledger.
pub fn add_load(doc: &mut Value, case: &str, load: Value) {
    let id = load["id"].clone();
    let case = doc["model"]["load_cases"].as_array_mut().unwrap().iter_mut().find(|c| c["id"] == case).unwrap();
    case["primitive_loads"].as_array_mut().unwrap().push(load);
    if let Some(sources) = case.pointer_mut("/analysis_state/load_sources").and_then(Value::as_array_mut) {
        sources.push(json!({"source_ref": id, "factor": 1.0}));
    }
}

// --- the exact-route sizing reference's models (EXS-A, EXS-B) -------------------

/// The product document of one `ex_sizing.json` model, built from its physical
/// description: on EX, as a load/reference-state document. Each case stores every
/// stored load and its ledger includes those the reference includes, at their
/// factors; the moved non-candidate supports are boundary motions of their states;
/// each member's operating temperature is the case's, against the installation
/// temperature of the reference configuration, with a constant secant coefficient
/// whose datum is that temperature. The section is the stated tube with no
/// corrosion allowance, mill tolerance or density, so no weight is generated.
/// Candidates are placeholder rigid UZ restraints. A guide's RX becomes a stiff
/// torsion spring, as in the R-1 reference requests, which carries no load here.
pub fn exs_request(reference: &Value) -> Value {
    let section = &reference["section"];
    let t_install = reference["installation_temperature_c"].as_f64().unwrap();
    let nodes: Vec<Value> = reference["nodes"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": "invented_test_value"}))
        .collect();
    let vertical = |m: &Value| {
        let (a, b) = (&reference["nodes"][m["end_nodes"][0].as_str().unwrap()], &reference["nodes"][m["end_nodes"][1].as_str().unwrap()]);
        a[0] == b[0] && a[1] == b[1]
    };
    let members = reference["members"].as_array().unwrap();
    let pipes: Vec<Value> = members
        .iter()
        .map(|m| json!({
            "id": m["id"], "from": m["end_nodes"][0], "to": m["end_nodes"][1], "material": "material:EXS",
            "section": {"outside_diameter": {"value": section["outside_diameter_m"], "unit": "m"},
                        "wall_thickness": {"value": section["wall_thickness_m"], "unit": "m"},
                        "corrosion_allowance": {"value": 0.0, "unit": "m"}},
            "y_reference": if vertical(m) { json!({"x": 1, "y": 0, "z": 0}) } else { json!({"x": 0, "y": 0, "z": 1}) },
            "provenance": "invented_test_value"}))
        .collect();
    let material = json!({
        "id": "material:EXS", "label": "Invented EXS material", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": {"value": section["elastic_modulus_pa"], "unit": "Pa"},
        "poisson_ratio": {"value": section["poisson_ratio"], "unit": "1"},
        "expansion_laws": [{"id": "law:EXS", "definition": "engineering_secant", "datum_temperature": {"value": t_install, "unit": "degC"},
                            "data": {"kind": "constant", "coefficient": {"value": section["thermal_expansion_per_k"], "unit": "1/K"}},
                            "provenance": "invented_test_value"}],
        "provenance": "invented_test_value"});
    let mut supports = Vec::new();
    for s in reference["supports"].as_array().unwrap() {
        let (id, node) = (s["id"].as_str().unwrap(), s["node"].as_str().unwrap());
        let dofs: Vec<&str> = s["dofs"].as_array().unwrap().iter().map(|d| d.as_str().unwrap()).collect();
        match (s["kind"].as_str().unwrap(), dofs.as_slice()) {
            ("anchor", _) => supports.push(support(id, node, "anchor", &dofs)),
            ("rigid" | "candidate", ["UZ"]) => supports.push(support(id, node, "vertical_support", &["UZ"])),
            ("rigid", _) => {
                let translations: Vec<&str> = dofs.iter().copied().filter(|d| d.starts_with('U')).collect();
                supports.push(support(id, node, "guide", &translations));
                if dofs.contains(&"RX") {
                    supports.push(torsion_spring(&format!("{id}-torsion"), node));
                }
            }
            (kind, _) => panic!("unknown support kind {kind}"),
        }
    }
    let support_ids: Vec<String> = supports.iter().map(|s| s["id"].as_str().unwrap().to_string()).collect();
    let stored: Vec<Value> = reference["stored_loads"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|l| l["kind"] == "nodal_force")
        .map(|l| nodal_fz(l["id"].as_str().unwrap(), l["node"].as_str().unwrap(), l["value_n"].as_f64().unwrap()))
        .collect();
    let pressure = reference["stored_loads"].as_array().unwrap().iter().find(|l| l["kind"] == "internal_pressure");
    let cases: Vec<Value> = reference["load_cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|lc| {
            let sources: Vec<Value> = lc["includes"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|i| stored.iter().any(|l| l["id"] == i["load"]))
                .map(|i| json!({"source_ref": i["load"], "factor": i["factor"]}))
                .collect();
            let included_pressure = pressure.filter(|p| lc["includes"].as_array().unwrap().iter().any(|i| i["load"] == p["id"]));
            let regions: Vec<Value> = included_pressure
                .map(|p| json!({
                    "id": p["id"], "member_pipe_ids": p["members"], "pressure_basis": "internal_differential_zero_external_v1",
                    "pressure": {"value": p["value_pa"], "unit": "Pa"},
                    "terminals": p["closed_ends"].as_array().unwrap().iter().map(|n| json!({"node_ref": n,
                        "closure_transfer": "transfers_to_wall", "provenance": "invented_test_value"})).collect::<Vec<_>>(),
                    "provenance": "invented_test_value"}))
                .into_iter()
                .collect();
            let states: Vec<Value> = support_ids
                .iter()
                .map(|id| {
                    let motions: Vec<Value> = lc["imposed_displacements"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter(|m| m["support"] == id.as_str())
                        .map(|m| json!({"dof": m["dof"], "value": {"value": m["value_m"], "unit": "m"}, "meaning": "absolute_reference_displacement"}))
                        .collect();
                    let mut state = json!({"support_ref": id, "participation": {"kind": "active_model_device"}});
                    if !motions.is_empty() {
                        state["boundary_motion"] = Value::Array(motions);
                    }
                    state
                })
                .collect();
            json!({
                "id": lc["id"], "kind": "primitive_user_load", "primitive_loads": stored, "pressure_regions": regions,
                "analysis_state": {
                    "contract": "openpipestress.load_reference_state/1.0.0",
                    "reference_configuration_ref": "reference:installed",
                    "element_states": members.iter().map(|m| json!({
                        "pipe_ref": m["id"],
                        "operating_temperature": {"value": lc["temperature_c"], "unit": "degC"},
                        "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:EXS",
                                               "applicability_reference": "invented_test_value"},
                        "thermal_state": {"kind": "free_length_state", "expansion_law_ref": "law:EXS"}})).collect::<Vec<_>>(),
                    "support_states": states,
                    "load_sources": sources,
                    "history": {"kind": "independent_equilibrium"},
                    "provenance": "invented_test_value"
                },
                "provenance": "invented_test_value"
            })
        })
        .collect();
    json!({
        "model": {
            "schema_version": "0.9.0",
            "document_kind": "openpipestress.product_preview.model",
            "pressure_contract": exact_contract(),
            "data_boundary": {
                "public_examples_policy": "invented_or_cleared_data_only",
                "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
                "private_data_policy": "no_private_project_data",
                "professional_boundary": "technical_preview_requires_human_engineering_review"
            },
            "project": {"id": format!("project:hanger-sizing-{}", reference["id"].as_str().unwrap().to_lowercase()),
                        "name": "Invented exact-route hanger sizing reference",
                        "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "temperature": "degC"}},
            "materials": [material],
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                "rule_check": "not_performed_user_rule_inputs_missing",
                                "professional_acceptance": "not_provided"},
            "nodes": nodes,
            "pipe_segments": pipes,
            "supports": supports,
            "components": [],
            "reference_configurations": [{
                "id": "reference:installed",
                "geometry_ref": {"kind": "authored_model_geometry"},
                "member_references": members.iter().map(|m| json!({"pipe_ref": m["id"],
                    "basis": {"kind": "temperature_reference", "installation_temperature": {"value": t_install, "unit": "degC"}},
                    "fit": {"kind": "none"}, "provenance": "invented_test_value"})).collect::<Vec<_>>(),
                "provenance": "invented_test_value"
            }],
            "load_cases": cases,
            "combinations": []
        },
        "materials": []
    })
}
