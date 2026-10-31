//! T4-U4b frozen references (`validation/references/t4_u4b/`): the builder
//! that authors the eight model 0.8.0 requests from `cases.json` by the
//! README's "Binding" rules, and the PP reference test over both modes and
//! both entries (DEL-04-07 Design `u4b-u5b-implementation.md` §7.1).
//!
//! The requests are formed in memory from the SHA-bound `cases.json`; their
//! own SHA-256 values are pinned here ([`REQUEST_SHA256`]). The frozen tree
//! stays unchanged: no request or request SHA-256 is written into
//! `oracle.json`, and the binding stays pinned in this test (design §1.4, as
//! for t4_u5b).
use super::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const CASES: &str = include_str!("../../../validation/references/t4_u4b/cases.json");
const ORACLE: &str = include_str!("../../../validation/references/t4_u4b/oracle.json");
const PROVENANCE: &str = "invented T4-U4b arc stress reference input; not library, component or code-rule data";
const STATIONS: [&str; 5] = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];

/// The SHA-256 of each authored request's compact JSON (serde_json order).
const REQUEST_SHA256: [(&str, &str); 8] = [
    ("U4B-L90-P", "c6614d1cbfaad12bf1c4141d5260bd13d1fd64512e0c784aa0df6a3a686bde9d"),
    ("U4B-L90-P-COINCIDENT", "440d62bf8d14b21513fecac0df4a05700ca56be1831a476e32ba27de8ac12f6e"),
    ("U4B-L90-P-MOVED", "27a7c117f5dde9ef6a6cd69fa84cdafd99c3e06067643d8849375a59d9de5d3a"),
    ("U4B-ARC106-RADIAL-NEGP", "1d40eea209385275853b2544fc391d4c494a54e225498764a27e7e1775c5ab0b"),
    ("U4B-ARC90-TIGHT-NSIGN-P", "14bb0a1a5be37770e6328c712f96380f8054dc7981b00fc90d5d0e5395ba8b99"),
    ("U4B-ARC90-OOP-PREVIEW", "858b5bcdc48e87c700e7ca2c9f10f5493ff0ef3c39d03e553d49ee3a75ff9841"),
    ("U4B-Z-PREVIEW-REV", "59befe01543ab8bd5c3e207297917d05f8c6344403075bd75771ba337c893331"),
    ("U4B-RISE74-P-REV", "e4b37e056f6f1f09f70414cab6af482c7e5acfd889e8b7cb4db255f79e70d10b"),
];

fn n(v: &Value) -> f64 {
    if v.is_array() {
        return n(&v[1]);
    }
    v.as_f64().or_else(|| v.as_str().map(|s| s.parse().unwrap())).unwrap_or_else(|| panic!("number {v}"))
}
fn quantity(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn vector(v: &Value) -> Value {
    json!({"x": n(&v[0]), "y": n(&v[1]), "z": n(&v[2])})
}

pub(crate) fn inputs() -> (Value, Value) {
    let cases: Value = serde_json::from_str(CASES).unwrap();
    let oracle: Value = serde_json::from_str(ORACLE).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(CASES.as_bytes())), oracle["cases_sha256"].as_str().unwrap());
    (cases, oracle)
}

/// One model 0.8.0 request from a case description (README "Binding"): bend
/// components at the arc's authored from node with R, the binary64 bend angle,
/// k and the directional pair; primitive loads per nonzero global component;
/// the pressure region and its terminals; the shear model. The two preview
/// cases give E and nu but no G: G = RN(E/RN(2·RN(1+nu))), so the route's
/// derived nu matches the oracle's within a few ulps.
pub(crate) fn build_request(case: &Value) -> Value {
    let exact = case["route"] == "exact";
    let nodes: Vec<Value> = case["nodes"].as_array().unwrap().iter()
        .map(|node| json!({"id": node["id"], "position": vector(&node["position"]), "provenance": PROVENANCE}))
        .collect();
    let sections: HashMap<&str, &Value> = case["sections"].as_array().unwrap().iter().map(|s| (s["id"].as_str().unwrap(), s)).collect();
    let mut pipes = Vec::new();
    let mut components = Vec::new();
    for member in case["members"].as_array().unwrap() {
        let s = sections[member["section"].as_str().unwrap()];
        let mut section = json!({"outside_diameter": quantity(n(&s["outside_diameter"]), "m"),
            "wall_thickness": quantity(n(&s["wall_thickness"]), "m")});
        if s.get("mill_tolerance").is_some() {
            section["mill_tolerance"] = quantity(n(&s["mill_tolerance"]), "m");
        }
        if exact {
            section["corrosion_allowance"] = quantity(s.get("corrosion_allowance").map_or(0.0, n), "m");
        }
        pipes.push(json!({"id": member["id"], "from": member["from"], "to": member["to"], "section": section,
            "material": member["material"], "y_reference": vector(&member["y_reference"]), "provenance": PROVENANCE}));
        if member["kind"] == "realized_arc" {
            let arc = &member["arc"];
            let sif = &member["directional_sif_user_values"];
            let mut modifiers = json!({"flexibility_factor_user_value": quantity(n(&arc["flexibility_factor_user_value"]), "none"),
                "source_reference": "invented test value"});
            if sif.is_object() {
                modifiers["sif_in_plane_user_value"] = quantity(n(&sif["i_ip"]), "1");
                modifiers["sif_out_of_plane_user_value"] = quantity(n(&sif["i_op"]), "1");
            }
            components.push(json!({"id": format!("component:bend:{}", member["id"].as_str().unwrap()), "kind": "bend",
                "node": arc["bend_component_attachment_node"],
                "geometry": {"bend_pipe_ref": member["id"], "bend_radius": quantity(n(&arc["bend_radius"]), "m"),
                    "bend_angle": quantity(n(&arc["bend_angle_binary64_for_authoring"]), "rad"),
                    "bend_plane_orientation": "plane of the span chord and the pipe y_reference; the arc bows toward +y_reference"},
                "modifiers": modifiers,
                "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
                "provenance": format!("{PROVENANCE}; user flexibility factor k and directional SIFs are invented test values")}));
        }
    }
    let materials: Vec<Value> = case["materials"].as_array().unwrap().iter().map(|m| {
        let (e, nu) = (n(&m["elastic_modulus"]), n(&m["poisson_ratio"]));
        if exact {
            json!({"id": m["id"], "constitutive_basis": m["constitutive_basis"], "elastic_modulus": quantity(e, "Pa"),
                "poisson_ratio": quantity(nu, "1"), "provenance": PROVENANCE})
        } else {
            json!({"id": m["id"], "elastic_modulus": quantity(e, "Pa"), "shear_modulus": quantity(e / (2.0 * (1.0 + nu)), "Pa"),
                "thermal_expansion_coefficient": quantity(n(&m["thermal_expansion_coefficient"]), "1/degC"), "provenance": PROVENANCE})
        }
    }).collect();
    let supports: Vec<Value> = case["supports"].as_array().unwrap().iter()
        .map(|s| json!({"id": s["id"], "node": s["node"], "family": s["family"], "restraints": s["restraints"], "provenance": PROVENANCE}))
        .collect();
    let lc = &case["load_case"];
    let mut loads = Vec::new();
    let axes = ["x", "y", "z"];
    for f in lc["nodal_forces"].as_array().into_iter().flatten() {
        for (k, v) in f["global_components"].as_array().unwrap().iter().enumerate() {
            if n(v) != 0.0 {
                loads.push(json!({"id": format!("{}:global_{}", f["id"].as_str().unwrap(), axes[k]), "category": "concentrated_force",
                    "target": {"type": "node", "node": f["node"]}, "direction": format!("global_{}", axes[k]),
                    "magnitude": quantity(n(v), "N"), "dimension": "force", "provenance": PROVENANCE}));
            }
        }
    }
    for f in lc["nodal_moments"].as_array().into_iter().flatten() {
        for (k, v) in f["global_components"].as_array().unwrap().iter().enumerate() {
            if n(v) != 0.0 {
                loads.push(json!({"id": format!("{}:rotation_{}", f["id"].as_str().unwrap(), axes[k]), "category": "concentrated_moment",
                    "target": {"type": "node", "node": f["node"]}, "direction": format!("rotation_{}", axes[k]),
                    "magnitude": quantity(n(v), "N*m"), "dimension": "moment", "provenance": PROVENANCE}));
            }
        }
    }
    for f in lc["member_uniform_forces"].as_array().into_iter().flatten() {
        for (k, v) in f["global_components_per_length"].as_array().unwrap().iter().enumerate() {
            if n(v) != 0.0 {
                loads.push(json!({"id": format!("{}:global_{}", f["id"].as_str().unwrap(), axes[k]), "category": "distributed_force",
                    "target": {"type": "element", "pipe": f["member"]}, "direction": format!("global_{}", axes[k]),
                    "magnitude": quantity(n(v), "N/m"), "dimension": "force_per_length", "provenance": PROVENANCE}));
            }
        }
    }
    let mut load_case = json!({"id": lc["id"], "primitive_loads": loads, "provenance": PROVENANCE});
    let mut model = json!({"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "project": {"id": format!("project:{}", case["case_id"].as_str().unwrap().to_lowercase()),
            "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components, "materials": materials,
        "combinations": []});
    if exact {
        let contract = &case["pressure_contract"];
        model["pressure_contract"] = json!({"version": contract["version"], "mode": contract["mode"], "shear_model": contract["shear_model"]});
        let regions: Vec<Value> = lc["pressure_regions"].as_array().unwrap().iter().map(|r| json!({"id": r["id"], "member_pipe_ids": r["member_ids"],
            "pressure_basis": r["pressure_basis"], "pressure": quantity(n(&r["pressure"]), "Pa"),
            "terminals": r["terminals"].as_array().unwrap().iter().map(|t| json!({"node_ref": t["node"], "closure_transfer": t["closure_transfer"], "provenance": PROVENANCE})).collect::<Vec<_>>(),
            "provenance": PROVENANCE})).collect();
        load_case["pressure_regions"] = json!(regions);
        add_load_state(&mut model, &mut load_case, case);
    }
    model["load_cases"] = json!([load_case]);
    json!({"model": model, "materials": []})
}

/// The binding rules' load/reference state: an installed reference of every
/// member and each member's explicit base properties.
fn add_load_state(model: &mut Value, load_case: &mut Value, case: &Value) {
    let member_ids: Vec<&Value> = case["members"].as_array().unwrap().iter().map(|m| &m["id"]).collect();
    model["reference_configurations"] = json!([{"id": "reference:installed", "label": "Installed reference",
        "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": member_ids.iter().map(|id| json!({"pipe_ref": id, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROVENANCE})).collect::<Vec<_>>(),
        "provenance": PROVENANCE}]);
    let material_of = |id: &Value| case["members"].as_array().unwrap().iter().find(|m| &m["id"] == id).unwrap()["material"].clone();
    load_case["analysis_state"] = json!({"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
        "element_states": member_ids.iter().map(|id| json!({"pipe_ref": id,
            "material_selection": {"kind": "explicit_base_properties", "material_ref": material_of(id), "applicability_reference": "invented analytical basis declared applicable"},
            "thermal_state": {"kind": "unchanged_reference", "provenance": PROVENANCE}})).collect::<Vec<_>>(),
        "support_states": case["supports"].as_array().unwrap().iter().map(|s| json!({"support_ref": s["id"], "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>(),
        "load_sources": load_case["primitive_loads"].as_array().unwrap().iter().map(|l| json!({"source_ref": l["id"], "factor": 1.0})).collect::<Vec<_>>(),
        "history": {"kind": "independent_equilibrium"}, "provenance": PROVENANCE});
}

/// The authored requests with their pinned hashes checked (`U4B_PRINT_REQUEST_SHA256`
/// prints them instead, for re-pinning after a deliberate builder change).
pub(crate) fn requests() -> Vec<(String, Value)> {
    let (cases, _) = inputs();
    let print = std::env::var("U4B_PRINT_REQUEST_SHA256").is_ok();
    cases["cases"].as_array().unwrap().iter().map(|case| {
        let id = case["case_id"].as_str().unwrap().to_string();
        let request = build_request(case);
        let sha = format!("{:x}", Sha256::digest(serde_json::to_string(&request).unwrap().as_bytes()));
        if print {
            println!("(\"{id}\", \"{sha}\"),");
        } else {
            let pinned = REQUEST_SHA256.iter().find(|(case, _)| *case == id).expect("a pinned case").1;
            assert_eq!(sha, pinned, "{id}: the authored request's SHA-256");
        }
        (id, request)
    }).collect()
}

struct Scales<'a> {
    case: &'a Value,
}
impl Scales<'_> {
    fn member(&self, pipe: &str, family: &str) -> f64 {
        n(&self.case["zero_scales"][pipe][family])
    }
    fn basis(&self, pipe: &str, basis: &str, family: &str) -> f64 {
        let own = n(&self.case["zero_scales"][pipe][basis][family]);
        if own != 0.0 { own } else { self.case_scale(basis, family) }
    }
    fn case_scale(&self, basis: &str, family: &str) -> f64 {
        self.case["zero_scales"].as_object().unwrap().values()
            .filter_map(|m| m[basis][family].as_array().map(|_| n(&m[basis][family])))
            .fold(0.0, f64::max)
    }
}

fn product_basis(basis: &str) -> &str {
    if basis == "preview_single_section" { "" } else { basis }
}
fn rows_on<'a>(out: &'a MechanicsEnvelope, kind: &str, entity: &str, location: &str, component: &str, basis: &str) -> Vec<&'a ResultItem> {
    out.results.iter().filter(|r| r.kind == kind && r.entity_ref == entity && r.metadata.as_ref().is_some_and(|m|
        (location.is_empty() || m.location == location) && (component.is_empty() || m.component == component)
            && m.basis.rsplit_once("; section_basis=").map_or("", |(_, b)| b) == product_basis(basis))).collect()
}
fn value_on(out: &MechanicsEnvelope, kind: &str, entity: &str, location: &str, component: &str, basis: &str) -> f64 {
    let found = rows_on(out, kind, entity, location, component, basis);
    assert_eq!(found.len(), 1, "{entity} {kind} {location} {component} [{basis}]: {:?}", out.diagnostics);
    found[0].value
}
fn tolerance(expected: f64, scale: f64) -> f64 {
    1e-9 * expected.abs().max(scale)
}

/// Worst observed fraction of the criterion per quantity family.
#[derive(Default)]
pub(crate) struct Agreement(BTreeMap<&'static str, f64>);
impl Agreement {
    fn close(&mut self, family: &'static str, label: &str, got: f64, expected: f64, scale: f64) {
        let t = tolerance(expected, scale);
        let fraction = (got - expected).abs() / t;
        assert!(fraction <= 1.0, "{label}: {got:.17e} != {expected:.17e}, {fraction} tolerances");
        let worst = self.0.entry(family).or_insert(0.0);
        *worst = worst.max(fraction);
    }
}

fn evidence_case(out: &MechanicsEnvelope, exact: bool) -> &Value {
    let evidence = out.contract_evidence.as_ref().expect("contract evidence");
    if exact { &evidence["exact_cases"][0] } else { &evidence["preview_cases"][0] }
}
fn maximum_record<'a>(out: &'a MechanicsEnvelope, exact: bool, key: &str, pipe: &str, basis: &str) -> &'a Value {
    let case = evidence_case(out, exact);
    case[key].as_array().unwrap().iter().find(|r| r["pipe_id"] == pipe
        && (key == "pipe_stress_extrema" || r.get("approximation").is_some())
        && (!exact || r["section_basis"] == basis)).unwrap_or_else(|| panic!("{key} {pipe} {basis}"))
}

/// The location of a member or intensified maximum: an end maximum exactly at
/// its end, an interior one within its `location_band` (fraction).
fn location(label: &str, published: f64, expected: &Value) {
    let loc = &expected["location"];
    match loc["kind"].as_str().unwrap() {
        "end_i" => assert_eq!(published, 0.0, "{label}: an end_i maximum is exactly at 0"),
        "end_j" => assert_eq!(published, 1.0, "{label}: an end_j maximum is exactly at 1"),
        _ => {
            let band = n(&expected["location_band"]["fraction"]);
            assert!((published - n(&loc["fraction"])).abs() <= band, "{label}: fraction {published} outside {} ± {band}", n(&loc["fraction"]));
        }
    }
}

/// §7.1: every mapped row of one case's envelope against the oracle.
fn check_case(out: &MechanicsEnvelope, case: &Value, label: &str, agreement: &mut Agreement) {
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: {:?}", out.diagnostics);
    let exact = case["route"] == "exact";
    let bases: &[&str] = if exact { &["nominal_section", "reduced_section"] } else { &["preview_single_section"] };
    let scales = Scales { case };
    let q = &case["quantities"];
    for (pipe, derived) in case["derived"].as_object().unwrap() {
        let arc = derived["kind"] != "straight";
        let ro = n(&derived["section_bases"][if exact { "nominal_section" } else { "preview_single_section" }]["ro"]);
        for station in STATIONS {
            // §7.1 (review 7(a)): the j-side section resultants against the
            // published force and moment rows (no section basis): the section rows
            // inside and at end_j, and at end_i the node-on-element end rows, whose
            // sign is the opposite of the j-side cut. On a pressurized exact member
            // N_w is the wall force row and S the effective axial force row.
            let resultants = &q["resultants_tangent_frame_j_side"][pipe][station];
            let axial: &[(&str, &str)] = if resultants.get("N_w").is_some() {
                &[("N_w", "pipe_wall_axial_force_v2"), ("S", "pipe_effective_axial_force_v2")]
            } else {
                &[("N", "element_local_axial_force")]
            };
            let mut compared = 0;
            for &(key, kind) in axial.iter().chain(&[
                ("V_y", "element_local_shear_force_y"), ("V_z", "element_local_shear_force_z"), ("T", "element_local_torsional_moment"),
                ("M_y", "element_local_bending_moment_y"), ("M_z", "element_local_bending_moment_z"),
            ]) {
                let scale = scales.member(pipe, if key.starts_with('V') || key.starts_with('N') || key == "S" { "force" } else { "moment" });
                let sign = if station == "end_i" && kind.starts_with("element_local_") { -1.0 } else { 1.0 };
                let published = sign * value_on(out, kind, pipe, station, "", "preview_single_section");
                agreement.close("resultants_tangent_frame_j_side", &format!("{label} {pipe} {station} {key}"), published, n(&resultants[key]), scale);
                compared += 1;
            }
            assert_eq!(compared, resultants.as_object().unwrap().len(), "{label} {pipe} {station}: every resultant compared");
            for basis in bases {
                let ne = &q["normal_extremes"][pipe][station][basis];
                let normal_scale = scales.basis(pipe, basis, "normal_stress");
                let plus = value_on(out, u4b_rows::EXTREME_KIND, pipe, station, "sigma_plus", basis) * 1e6;
                let minus = value_on(out, u4b_rows::EXTREME_KIND, pipe, station, "sigma_minus", basis) * 1e6;
                agreement.close("sigma_plus_minus", &format!("{label} {pipe} {station} {basis} sigma+"), plus, n(&ne["sigma_plus"]), normal_scale);
                agreement.close("sigma_plus_minus", &format!("{label} {pipe} {station} {basis} sigma-"), minus, n(&ne["sigma_minus"]), normal_scale);
                agreement.close("max_abs_normal", &format!("{label} {pipe} {station} {basis} max_abs"), plus.abs().max(minus.abs()), n(&ne["max_abs_normal"]), normal_scale);
                // §2.3: the fibres from the published bending rows, at zero scale r_o.
                let sy = value_on(out, "element_local_bending_normal_stress_y", pipe, station, "", basis);
                let sz = value_on(out, "element_local_bending_normal_stress_z", pipe, station, "", basis);
                let moment_scale = scales.member(pipe, "moment");
                let modulus = n(&derived["section_bases"][*basis]["Z"]);
                let hypot_moment = norm2(sy, sz) * 1e6 * modulus;
                if !ne["sigma_plus_fibre"].is_object() {
                    // §7.1 (review 7(b)): the oracle ties the fibre (zero bending); the
                    // published hypot is zero within the criterion before the fibre is skipped.
                    agreement.close("tied_fibre_hypot", &format!("{label} {pipe} {station} {basis} tied"), hypot_moment, 0.0, moment_scale);
                } else if hypot_moment > 1e-9 * moment_scale {
                    let fibres = open_pipe_stress_stress_recovery::section_stress_quantities::normal_extreme_fibres(ro, sy, sz).expect("bending is nonzero");
                    for (k, key) in [(0, "sigma_plus_fibre"), (1, "sigma_minus_fibre")] {
                        agreement.close("fibre", &format!("{label} {pipe} {station} {basis} {key} y"), fibres[k][0], n(&ne[key]["y"]), ro);
                        agreement.close("fibre", &format!("{label} {pipe} {station} {basis} {key} z"), fibres[k][1], n(&ne[key]["z"]), ro);
                        // The mirrored convention (σ_z, −σ_y) must fail against the oracle.
                        let h = norm2(sy, sz);
                        let mirrored = [ro * (sz / h), ro * (-sy / h)].map(|v| if k == 0 { v } else { -v });
                        assert!((mirrored[0] - n(&ne[key]["y"])).abs() > tolerance(n(&ne[key]["y"]), ro)
                            || (mirrored[1] - n(&ne[key]["z"])).abs() > tolerance(n(&ne[key]["z"]), ro), "{label} {pipe} {station}: the mirrored fibre convention is discriminated");
                    }
                }
                let shear_scale = scales.basis(pipe, basis, "shear_stress");
                agreement.close("sv_transverse_shear_max", &format!("{label} {pipe} {station} {basis} tau"),
                    value_on(out, u4b_rows::SHEAR_MAX_KIND, pipe, station, "tau_v_max", basis) * 1e6,
                    n(&q["sv_transverse_shear_max"][pipe][station][basis]["value"]), shear_scale);
                agreement.close("torsional_shear", &format!("{label} {pipe} {station} {basis} torsion"),
                    value_on(out, "element_local_torsional_shear_stress", pipe, station, "", basis) * 1e6,
                    n(&q["torsional_shear"][pipe][station][basis]), shear_scale);
                if arc && q["intensified_bending"][pipe].is_object() {
                    agreement.close("intensified_bending", &format!("{label} {pipe} {station} {basis} intensified"),
                        value_on(out, u4b_rows::DIRECTIONAL_KIND, pipe, station, "intensified_bending", basis),
                        n(&q["intensified_bending"][pipe][station][basis]), normal_scale);
                }
                if arc && q["toroidal_membrane_hoop"][pipe].is_object() {
                    let hoop = &q["toroidal_membrane_hoop"][pipe][basis]["fibres"];
                    let pressure_scale = scales.basis(pipe, basis, "pressure_stress");
                    for (component, fibre) in [("toroidal_intrados_hoop_stress", "y_pos"), ("toroidal_crown_hoop_stress", "z_pos"), ("toroidal_extrados_hoop_stress", "y_neg")] {
                        agreement.close("toroidal_hoop", &format!("{label} {pipe} {station} {basis} {component}"),
                            value_on(out, u4b_rows::HOOP_KIND, pipe, station, component, basis), n(&hoop[fibre]), pressure_scale);
                    }
                    let radial = &q["radial_surface"][pipe][basis];
                    for (component, key) in [("toroidal_inner_radial_stress", "inner"), ("toroidal_outer_radial_stress", "outer")] {
                        agreement.close("radial_surface", &format!("{label} {pipe} {station} {basis} {component}"),
                            value_on(out, u4b_rows::RADIAL_KIND, pipe, station, component, basis), n(&radial[key]), pressure_scale);
                    }
                } else {
                    assert!(rows_on(out, u4b_rows::HOOP_KIND, pipe, "", "", basis).is_empty(), "{label} {pipe}: no hoop row");
                }
            }
        }
        for basis in bases {
            let normal_scale = scales.basis(pipe, basis, "normal_stress");
            let expected = &q["member_maximum"][pipe][basis];
            let value = value_on(out, "pipe_elastic_normal_stress_maximum_v2", pipe, "governing_station", "", basis);
            agreement.close("member_maximum", &format!("{label} {pipe} {basis} maximum"), value, n(&expected["value"]), normal_scale);
            let record = maximum_record(out, exact, "pipe_stress_extrema", pipe, basis);
            if arc {
                assert_eq!(record["approximation"], u4b_rows::ARC_APPROXIMATION);
                assert_eq!(record["span_index"], 0);
            }
            location(&format!("{label} {pipe} {basis} maximum"), n(&record["station_fraction"]), expected);
            if arc && q["intensified_bending_maximum"][pipe].is_object() {
                let expected = &q["intensified_bending_maximum"][pipe][basis];
                agreement.close("intensified_bending_maximum", &format!("{label} {pipe} {basis} intensified maximum"),
                    value_on(out, u4b_rows::DIRECTIONAL_MAX_KIND, pipe, "governing_station", "", basis), n(&expected["value"]), normal_scale);
                let record = maximum_record(out, exact, "intensified_measures", pipe, basis);
                assert_eq!(record["approximation"], u4b_rows::ARC_DIRECTIONAL_APPROXIMATION);
                location(&format!("{label} {pipe} {basis} intensified maximum"), n(&record["station_fraction"]), expected);
            }
        }
    }
    // Headlines per basis.
    for basis in bases {
        let expected = &q["headline"][*basis];
        let headline = match *basis {
            "nominal_section" => out.summary.max_open_formula_stress_nominal_section.clone().flatten(),
            "reduced_section" => out.summary.max_open_formula_stress_reduced_section.clone().flatten(),
            _ => out.summary.max_open_formula_stress.clone(),
        }.unwrap_or_else(|| panic!("{label} {basis}: a headline over arcs and straights"));
        agreement.close("headline", &format!("{label} {basis} headline"), headline.value, n(&expected["value"]), scales.case_scale(basis, "normal_stress"));
        let members: Vec<&str> = expected["co_governing_members"].as_array().unwrap().iter().map(|m| m.as_str().unwrap()).collect();
        assert!(members.contains(&headline.location_ref.as_str()), "{label} {basis}: headline member {}", headline.location_ref);
    }
    discriminators(out, case, label);
}

/// §7.1: every listed mutant row the product publishes lies at least 1e3
/// tolerances from the product's row (field-function and not-formed rows are
/// checked in stress_recovery or not formed).
fn discriminators(out: &MechanicsEnvelope, case: &Value, label: &str) {
    let scales = Scales { case };
    for (mutant, control) in case["discriminators"].as_object().unwrap() {
        for listed in control["listed"].as_array().into_iter().flatten() {
            let path: Vec<&str> = listed["row"].as_str().unwrap().split('/').collect();
            let wrong = n(&listed["wrong"]);
            let expected = n(&listed["expected"]);
            let (got, scale) = match path[0] {
                "normal_extremes" => {
                    let (pipe, station, basis) = (path[1], path[2], path[3]);
                    let p = value_on(out, u4b_rows::EXTREME_KIND, pipe, station, "sigma_plus", basis) * 1e6;
                    let m = value_on(out, u4b_rows::EXTREME_KIND, pipe, station, "sigma_minus", basis) * 1e6;
                    (match path[4] { "sigma_plus" => p, "sigma_minus" => m, _ => p.abs().max(m.abs()) }, scales.basis(pipe, basis, "normal_stress"))
                }
                "sv_transverse_shear_max" => (value_on(out, u4b_rows::SHEAR_MAX_KIND, path[1], path[2], "tau_v_max", path[3]) * 1e6, scales.basis(path[1], path[3], "shear_stress")),
                "torsional_shear" => (value_on(out, "element_local_torsional_shear_stress", path[1], path[2], "", path[3]) * 1e6, scales.basis(path[1], path[3], "shear_stress")),
                "member_maximum" => (value_on(out, "pipe_elastic_normal_stress_maximum_v2", path[1], "governing_station", "", path[2]), scales.basis(path[1], path[2], "normal_stress")),
                "intensified_bending" => (value_on(out, u4b_rows::DIRECTIONAL_KIND, path[1], path[2], "", path[3]), scales.basis(path[1], path[3], "normal_stress")),
                "intensified_bending_maximum" => (value_on(out, u4b_rows::DIRECTIONAL_MAX_KIND, path[1], "governing_station", "", path[2]), scales.basis(path[1], path[2], "normal_stress")),
                "toroidal_membrane_hoop" => {
                    let component = match path[3] { "y_pos" => "toroidal_intrados_hoop_stress", "z_pos" => "toroidal_crown_hoop_stress", "y_neg" => "toroidal_extrados_hoop_stress", other => panic!("{other}") };
                    (value_on(out, u4b_rows::HOOP_KIND, path[1], "midspan", component, path[2]), scales.basis(path[1], path[2], "pressure_stress"))
                }
                "radial_surface" => (value_on(out, u4b_rows::RADIAL_KIND, path[1], "midspan", "toroidal_inner_radial_stress", path[2]), scales.basis(path[1], path[2], "pressure_stress")),
                "headline" => {
                    let headline = match path[1] {
                        "nominal_section" => out.summary.max_open_formula_stress_nominal_section.clone().flatten(),
                        "reduced_section" => out.summary.max_open_formula_stress_reduced_section.clone().flatten(),
                        _ => out.summary.max_open_formula_stress.clone(),
                    };
                    (headline.unwrap().value, scales.case_scale(path[1], "normal_stress"))
                }
                // Not published (§2.2): field functions (stress_recovery tests) and not-formed measures.
                "fibre_normal" | "sv_transverse_shear" | "flexure_plus_torsion_check" | "axial_plus_intensified_bending" => continue,
                other => panic!("{label} {mutant}: unmapped discriminator row {other}"),
            };
            let separation = (got - wrong).abs() / tolerance(expected, scale);
            assert!(separation >= 1e3, "{label} {mutant} {}: {separation} tolerances from the mutant", listed["row"]);
        }
    }
}

/// Review item 5: the two arc models of the shared base reader corpus, built
/// from `cases.json` by the binding rules (the frozen tree stays unchanged;
/// the requests' SHA-256 stay pinned in [`REQUEST_SHA256`]): a pressurized
/// pressure-2 L line with a directional arc and a preview line with two
/// directional arcs. `U4B_WRITE_CORPUS_MODELS=1` rewrites them; the producer's
/// envelopes of them are written by the `exact_pressure_connected` and
/// `preview_physics_capture` examples (`fixtures/results/*_u4b_arc_sparse.json`).
pub(crate) const CORPUS_MODELS: [(&str, &str); 2] = [
    ("U4B-L90-P", "tests/fixtures/u4b_l90_p_model.json"),
    ("U4B-Z-PREVIEW-REV", "tests/fixtures/u4b_z_preview_rev_model.json"),
];
#[test]
fn u4b_corpus_models_are_current() {
    let requests = requests();
    for (case_id, path) in CORPUS_MODELS {
        let request = &requests.iter().find(|(id, _)| id == case_id).expect("a reference case").1;
        let bytes = serde_json::to_string_pretty(&request["model"]).unwrap() + "\n";
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        if std::env::var_os("U4B_WRITE_CORPUS_MODELS").is_some() {
            std::fs::write(&path, &bytes).unwrap();
        }
        assert_eq!(std::fs::read_to_string(&path).unwrap(), bytes, "{}: regenerate with U4B_WRITE_CORPUS_MODELS=1", path.display());
    }
}

/// The two arc envelopes are their writers' current output (the
/// `exact_pressure_connected` and `preview_physics_capture` examples, sparse,
/// `--model` on [`CORPUS_MODELS`]); nothing else re-runs the producer on them.
#[test]
fn u4b_corpus_envelopes_are_current() {
    let exact = LinearStaticPreviewRequest {
        model: serde_json::from_str(include_str!("../tests/fixtures/u4b_l90_p_model.json")).unwrap(),
        materials: Vec::new(),
    };
    let exact = serde_json::to_string_pretty(&run_linear_static_preview_with_mode(exact, PreviewSolverMode::SparseInteractive)).unwrap() + "\n";
    assert!(exact == include_str!("../../../fixtures/results/pressure_u4b_arc_sparse.json"),
        "fixtures/results/pressure_u4b_arc_sparse.json: regenerate with `cargo run --example exact_pressure_connected -- sparse --model tests/fixtures/u4b_l90_p_model.json`");
    let model: Value = serde_json::from_str(include_str!("../tests/fixtures/u4b_z_preview_rev_model.json")).unwrap();
    let preview = run_linear_static_preview_value_with_mode(json!({"model": model, "materials": []}), PreviewSolverMode::SparseInteractive).unwrap();
    let preview = serde_json::to_string_pretty(&preview).unwrap() + "\n";
    assert!(preview == include_str!("../../../fixtures/results/preview_physics_u4b_arc_sparse.json"),
        "fixtures/results/preview_physics_u4b_arc_sparse.json: regenerate with `cargo run --example preview_physics_capture -- sparse --model tests/fixtures/u4b_z_preview_rev_model.json`");
}

#[test]
fn u4b_requests_follow_the_binding_rules() {
    let (cases, _) = inputs();
    for case in cases["cases"].as_array().unwrap() {
        let request = build_request(case);
        let model = &request["model"];
        assert_eq!(model["schema_version"], MODEL_SCHEMA_VERSION);
        for member in case["members"].as_array().unwrap().iter().filter(|m| m["kind"] == "realized_arc") {
            let bend = model["components"].as_array().unwrap().iter().find(|c| c["geometry"]["bend_pipe_ref"] == member["id"]).unwrap();
            assert_eq!(bend["node"], member["from"], "attached at the arc's authored from node");
            assert_eq!(bend["geometry"]["bend_angle"]["value"], member["arc"]["bend_angle_binary64_for_authoring"]);
        }
        if case["route"] != "exact" {
            let m = &model["materials"][0];
            let (e, nu) = (n(&case["materials"][0]["elastic_modulus"]), n(&case["materials"][0]["poisson_ratio"]));
            let derived = u4b_rows::preview_poisson_ratio(e, n(&m["shear_modulus"]["value"]));
            assert!((derived - nu).abs() <= 8.0 * f64::EPSILON, "derived nu {derived} vs {nu}");
        }
    }
    requests();
}

#[test]
fn u4b_frozen_references_both_entries_and_modes() {
    let (cases, oracle) = inputs();
    let mut agreement = Agreement::default();
    for (id, request) in requests() {
        let case = &oracle["cases"][&id];
        let _ = &cases;
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            for captured in [true, false] {
                let out = if captured {
                    run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()
                } else {
                    run_linear_static_preview_with_mode(serde_json::from_value(request.clone()).unwrap(), mode)
                };
                check_case(&out, case, &format!("{id} {mode:?} captured={captured}"), &mut agreement);
                // §5.1: the product reader (with the captured model) admits it.
                let reader = if case["route"] == "exact" { "pressure" } else { "preview-physics" };
                let verdict = crate::result_export::reader_verdict::validate(reader, &serde_json::to_value(&out).unwrap(), Some(&request["model"]), None);
                assert_eq!(verdict["ok"], true, "{id} {mode:?}: {verdict}");
            }
        }
    }
    for (family, worst) in &agreement.0 {
        println!("t4_u4b worst fraction of criterion: {family} {worst:.3e}");
    }
}

/// PP's binary64 station function of an arc (the same chain as
/// `curved_bend_section_resultants`, which formed the station rows): the
/// published end-j force through the end-j frame's transpose, then the
/// arc closed form at `fraction`.
fn station_function(probe: &u4b_rows::probe::ArcProbe, fraction: f64) -> [f64; 6] {
    let axes = curved_bend_end_axes(&probe.bend, true).unwrap();
    let mut node_j = [0.0; DOF_PER_NODE];
    for block in 0..2 {
        for component in 0..3 {
            let mut value = 0.0;
            for (axis, row) in axes.iter().enumerate() {
                value += row[component] * probe.end_forces[DOF_PER_NODE + 3 * block + axis];
            }
            node_j[3 * block + component] = value;
        }
    }
    probe.bend.macro_element.arc_section_resultant_terms(fraction, node_j, &probe.intensities).unwrap()
}
fn measure_at(probe: &u4b_rows::probe::ArcProbe, fraction: f64, pressure_area_product: Option<f64>) -> f64 {
    let r = station_function(probe, fraction);
    let b = &probe.basis;
    match probe.measure {
        open_pipe_stress_frame_kernel::structural::ArcStressMeasure::ElasticNormal => {
            (r[0] + pressure_area_product.unwrap_or(0.0)).abs() / b.area + norm2(r[4], r[5]) / b.modulus
        }
        open_pipe_stress_frame_kernel::structural::ArcStressMeasure::DirectionalIntensified { in_plane, out_of_plane } => {
            norm2(in_plane * r[5], out_of_plane * r[4]) / b.modulus
        }
    }
}
/// The dense maximum of the station function: 4,097 samples, then a
/// golden-section refinement around the best sample.
fn dense_maximum(probe: &u4b_rows::probe::ArcProbe, pressure_area_product: Option<f64>) -> f64 {
    let f = |x: f64| measure_at(probe, x, pressure_area_product);
    let samples = 1024;
    let (mut best, mut at) = (f64::NEG_INFINITY, 0usize);
    for k in 0..=samples {
        let v = f(k as f64 / samples as f64);
        if v > best {
            best = v;
            at = k;
        }
    }
    let (mut lo, mut hi) = ((at.max(1) - 1) as f64 / samples as f64, ((at + 1).min(samples)) as f64 / samples as f64);
    let g = 0.5 * (5f64.sqrt() - 1.0);
    for _ in 0..80 {
        let (a, b) = (hi - g * (hi - lo), lo + g * (hi - lo));
        if f(a) >= f(b) { hi = b } else { lo = a }
    }
    best.max(f(0.5 * (lo + hi)))
}

/// Manager's K2 integration control: each arc maximum (elastic and
/// directional, every basis, the t4_u4b and t4_u4a corpora) is certified, its
/// station cover equals PP's own station function at the station fractions
/// (with pA_i once), and its enclosure contains the dense maximum of that
/// function. The pressure double count (N_w passed with pA_i) moves the
/// result off the dense maximum or is refused, so the binding catches it.
#[test]
fn u4b_arc_maximum_binds_its_station_rows() {
    let mut documents: Vec<(String, Value)> = requests();
    let u4a: Value = serde_json::from_str(include_str!("../../../validation/references/t4_u4a/requests.json")).unwrap();
    for (id, request) in u4a["cases"].as_object().unwrap() {
        documents.push((id.clone(), crate::b7b_admission_tests::reauthor_current_request(request.clone())));
    }
    let (mut checked, mut double_counts) = (0usize, 0usize);
    for (id, request) in documents {
        u4b_rows::probe::PROBE.with(|p| *p.borrow_mut() = Some(Vec::new()));
        let out = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive).unwrap();
        let probes = u4b_rows::probe::PROBE.with(|p| p.borrow_mut().take().unwrap());
        assert!(!out.diagnostics.iter().any(|d| matches!(d.code.as_str(), "EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE" | "PREVIEW_STRESS_MAXIMUM_UNAVAILABLE" | "DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE")),
            "{id}: no arc maximum is refused: {:?}", out.diagnostics);
        for probe in &probes {
            let label = format!("{id} {} {:?} {:?}", probe.bend.pipe_id, probe.basis.basis, probe.measure);
            let maximum = probe.outcome.as_ref().unwrap_or_else(|e| panic!("{label}: refused {e}"));
            let scale = probe.witnesses.iter().map(|w| w.1).fold(0.0, f64::max).max(f64::MIN_POSITIVE);
            for &(fraction, witness) in &probe.witnesses {
                let at = measure_at(probe, fraction, probe.pressure_area_product);
                assert!((at - witness).abs() <= 1e-12 * scale, "{label}: station {fraction} cover {witness} vs station function {at}");
            }
            let dense = dense_maximum(probe, probe.pressure_area_product);
            let slack = 1e-12 * dense.abs() + 1e-9;
            assert!(dense <= maximum.upper_bound + slack, "{label}: dense {dense} above the certified upper {}", maximum.upper_bound);
            let published = u4b_rows::arc_published_value(maximum);
            assert!((published - dense).abs() <= 1e-10 * dense.abs() + 1e-6, "{label}: published {published} vs dense {dense}");
            checked += 1;
            // The double count: N_w (= S + pA_i) as the end-j axial, with pA_i again.
            if let (Some(pai), open_pipe_stress_frame_kernel::structural::ArcStressMeasure::ElasticNormal) = (probe.pressure_area_product, probe.measure) {
                let mut forces = probe.end_forces.clone();
                forces[DOF_PER_NODE] += pai;
                let wrong = u4b_rows::arc_maximum(&u4b_rows::ArcOperands { bend: &probe.bend, end_forces: &forces, intensities: &probe.intensities, pressure_area_product: Some(pai) },
                    &probe.basis, probe.measure, &probe.witnesses);
                if let Ok(wrong) = wrong {
                    let value = u4b_rows::arc_published_value(&wrong);
                    assert!((value - dense).abs() > 1e-9 * dense.abs(), "{label}: the double count is not discriminated ({value} vs {dense})");
                }
                double_counts += 1;
            }
        }
    }
    assert!(checked >= 60 && double_counts >= 20, "{checked} maxima, {double_counts} double-count controls");
    println!("arc maxima bound: {checked}; double-count controls: {double_counts}");
}

fn verdict(out: &MechanicsEnvelope, reader: &str, model: &Value) -> Value {
    crate::result_export::reader_verdict::validate(reader, &serde_json::to_value(out).unwrap(), Some(model), None)
}
fn model_with_materials(request: &Value) -> Value {
    let mut model = request["model"].clone();
    if model.get("materials").is_none_or(|m| m.as_array().is_none_or(|a| a.is_empty())) {
        model["materials"] = request["materials"].clone();
    }
    model
}
fn shear_rows<'a>(out: &'a MechanicsEnvelope, case: &str) -> Vec<&'a ResultItem> {
    out.results.iter().filter(|r| r.kind == u4b_rows::SHEAR_MAX_KIND && r.basis_ref.as_ref().is_some_and(|b| b.ref_id == case)).collect()
}
/// τ of one row recomputed with ν = E/(2G) − 1 and the member's preview annulus.
fn expected_tau(out: &MechanicsEnvelope, row: &ResultItem, e: f64, g: f64, od: f64, t: f64) -> f64 {
    let case = &row.basis_ref.as_ref().unwrap().ref_id;
    let location = &row.metadata.as_ref().unwrap().location;
    let shear = |kind: &str| out.results.iter().find(|r| r.kind == kind && r.entity_ref == row.entity_ref
        && r.basis_ref.as_ref().is_some_and(|b| &b.ref_id == case) && r.metadata.as_ref().is_some_and(|m| &m.location == location)).unwrap().value;
    let id = od - 2.0 * t;
    let factor = open_pipe_stress_stress_recovery::section_stress_quantities::saint_venant_shear_factor(
        u4b_rows::preview_poisson_ratio(e, g), od / 2.0, id / 2.0, PI * (od.powi(4) - id.powi(4)) / 64.0).unwrap();
    open_pipe_stress_stress_recovery::section_stress_quantities::transverse_shear_maximum_mpa(shear("element_local_shear_force_y"), shear("element_local_shear_force_z"), factor)
}

/// §2.4 / §7.4: ν by route. preview-physics-2 derives ν = E/(2G) − 1 from the
/// moduli the solve uses (base, named point, interpolated); a derived ν
/// outside −1 < ν < 1/2 removes the member's τ rows with the named warning;
/// pressure-2 uses the authored ν (the t4_u4b exact cases and the exact
/// reader's bit-exact recompute from the evidence ν).
#[test]
fn u4b_poisson_controls_by_route() {
    let raw: Value = serde_json::from_str(include_str!("../../../fixtures/product_preview/invented_dec092_temperature_g_request.json")).unwrap();
    let mut request = raw.clone();
    request["model"]["schema_version"] = json!(MODEL_SCHEMA_VERSION);
    // A transverse tip force beside the fixture's torsion, so V is nonzero.
    for case in request["model"]["load_cases"].as_array_mut().unwrap() {
        let mut force = case["primitive_loads"][0].clone();
        force["id"] = json!(format!("{}:transverse", force["id"].as_str().unwrap()));
        force["category"] = json!("concentrated_force");
        force["direction"] = json!("UZ");
        force["dimension"] = json!("force");
        force["magnitude"] = json!({"value": 1500.0, "unit": "N"});
        case["primitive_loads"].as_array_mut().unwrap().push(force);
    }
    let section = &request["model"]["pipe_segments"][0]["section"];
    let mm = |q: &Value| n(&q["value"]) * if q["unit"] == "mm" { 1e-3 } else { 1.0 };
    let od = mm(&section["outside_diameter"]);
    let t = mm(&section["wall_thickness"]) - section.get("mill_tolerance").filter(|m| !m.is_null()).map_or(0.0, mm);
    let out = run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive).unwrap();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{:?}", out.diagnostics);
    // The point (G = 50 GPa, ν = 0.9) and interpolated (ν ≈ 1.06) cases are out of domain.
    for case in ["load:L-DEC092-EXACT", "load:L-DEC092-INTERPOLATED"] {
        assert!(shear_rows(&out, case).is_empty(), "{case}: no τ outside the domain");
        let pipes: Vec<&str> = request["model"]["pipe_segments"].as_array().unwrap().iter().map(|p| p["id"].as_str().unwrap()).collect();
        for pipe in pipes {
            assert!(out.diagnostics.iter().any(|d| d.code == u4b_rows::POISSON_OUT_OF_DOMAIN && d.severity == "warning"
                && d.affected_refs == vec![case.to_string(), pipe.to_string()]), "{case} {pipe}: the named warning");
        }
    }
    let base = shear_rows(&out, "load:L-DEC092-BASE");
    assert!(!base.is_empty());
    for row in base {
        assert_eq!(row.value.to_bits(), expected_tau(&out, row, 200000e6, 80000e6, od, t).to_bits(), "base moduli");
    }
    assert_eq!(verdict(&out, "preview-physics", &model_with_materials(&request))["ok"], true);
    // In domain: the named point's own moduli, and an interpolated case (model-free reader checks).
    request["materials"][0]["temperature_points"][0]["shear_modulus"]["value"] = json!(73000.0);
    request["materials"][0]["temperature_points"][1]["shear_modulus"]["value"] = json!(80000.0);
    request["materials"][0]["temperature_points"][2]["shear_modulus"]["value"] = json!(72000.0);
    let out = run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::DenseScrutiny).unwrap();
    assert!(!out.diagnostics.iter().any(|d| d.code == u4b_rows::POISSON_OUT_OF_DOMAIN));
    assert!(shear_rows(&out, "load:L-DEC092-EXACT").iter().any(|r| r.value > 0.0));
    for row in shear_rows(&out, "load:L-DEC092-EXACT").into_iter().filter(|r| r.value > 0.0) {
        assert_eq!(row.value.to_bits(), expected_tau(&out, row, 190000e6, 73000e6, od, t).to_bits(), "named point moduli, not the base");
        assert_ne!(row.value.to_bits(), expected_tau(&out, row, 200000e6, 80000e6, od, t).to_bits());
    }
    assert!(!shear_rows(&out, "load:L-DEC092-INTERPOLATED").is_empty());
    assert_eq!(verdict(&out, "preview-physics", &model_with_materials(&request))["ok"], true);
    // A tampered τ is refused with the model (bit for bit), but passes the
    // model-free read, which checks custody, coverage and sign only.
    let mut tampered = serde_json::to_value(&out).unwrap();
    let row = tampered["results"].as_array_mut().unwrap().iter_mut()
        .find(|r| r["kind"] == u4b_rows::SHEAR_MAX_KIND && r["basis_ref"]["ref_id"] == "load:L-DEC092-BASE").unwrap();
    row["value"] = json!(f64::from_bits(row["value"].as_f64().unwrap().to_bits() + 1));
    let with_model = crate::result_export::reader_verdict::validate("preview-physics", &tampered, Some(&model_with_materials(&request)), None);
    assert_eq!(with_model["error"]["code"], "SOURCE_PREVIEW_PHYSICS_SHEAR_VALUE");
    assert_eq!(crate::result_export::reader_verdict::validate("preview-physics", &tampered, None, None)["ok"], true);
}

/// §2.5: the torus domain guard. R below, at and one ulp above r_o, and at
/// (r_i + r_o)/2, on a pressurized arc of pressure-2.
#[test]
fn u4b_torus_domain_guard() {
    let (cases, _) = inputs();
    let base = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "U4B-ARC90-TIGHT-NSIGN-P").unwrap();
    let ro = n(&base["sections"][0]["outside_diameter"]) / 2.0;
    let wall = n(&base["sections"][0]["wall_thickness"]);
    let ri = ro - wall;
    for (label, r, published) in [("below", ro.next_down(), false), ("at", ro, false), ("one ulp above", ro.next_up(), true), ("mid-wall", (ri + ro) / 2.0, false)] {
        let mut case = base.clone();
        // A short chord so that the bend radius admits the arc.
        let chord = 0.5 * r;
        let a = &case["nodes"][0]["position"].clone();
        let b = &case["nodes"][1]["position"].clone();
        let d: Vec<f64> = (0..3).map(|k| n(&b[k]) - n(&a[k])).collect();
        let length = norm3(d[0], d[1], d[2]);
        case["nodes"][1]["position"] = json!((0..3).map(|k| n(&a[k]) + d[k] / length * chord).collect::<Vec<_>>());
        case["members"][0]["arc"]["bend_radius"] = json!(r);
        case["members"][0]["arc"]["bend_angle_binary64_for_authoring"] = json!(2.0 * (chord / (2.0 * r)).asin());
        let request = build_request(&case);
        let out = run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive).unwrap();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{label}: {:?}", out.diagnostics);
        let hoop = out.results.iter().filter(|r| r.kind == u4b_rows::HOOP_KIND || r.kind == u4b_rows::RADIAL_KIND).count();
        let warned = out.diagnostics.iter().any(|d| d.code == u4b_rows::TOROIDAL_DOMAIN);
        assert_eq!(hoop, if published { 5 * 5 * 2 } else { 0 }, "{label}: hoop and radial rows exactly when R > r_o");
        assert_eq!(warned, !published, "{label}: ARC_TOROIDAL_HOOP_DOMAIN");
        assert_eq!(verdict(&out, "pressure", &request["model"])["ok"], true, "{label}");
        // The other quantities stay published either way.
        assert!(out.results.iter().any(|r| r.kind == u4b_rows::MAXIMUM_KIND));
        if !published {
            // The reader requires the warning where the rows are absent.
            let mut stripped = serde_json::to_value(&out).unwrap();
            stripped["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != u4b_rows::TOROIDAL_DOMAIN);
            let refused = crate::result_export::reader_verdict::validate("pressure", &stripped, None, None);
            assert!(refused["error"]["detail"].as_str().is_some_and(|d| d.contains("SOURCE_PHYSICS_TOROIDAL_DOMAIN_WARNING")), "{label}: {refused}");
        }
    }
}

/// Review item 1 (§2.6, §5.1): the directional rows and records are bound to the
/// captured model on both routes. Model-free reads cannot see a whole dropped
/// family or consistently changed factors; the model-holding readers refuse
/// both. One dropped basis is refused either way (the twin rule).
#[test]
fn u4b_directional_rows_bind_to_the_captured_model() {
    let (cases, _) = inputs();
    for (id, reader, exact) in [("U4B-L90-P", "pressure", true), ("U4B-ARC90-OOP-PREVIEW", "preview-physics", false)] {
        let case = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == id).unwrap();
        let request = build_request(case);
        let model = model_with_materials(&request);
        let out = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive).unwrap()).unwrap();
        let read = |doc: &Value, model: Option<&Value>| crate::result_export::reader_verdict::validate(reader, doc, model, None);
        assert_eq!(read(&out, Some(&model))["ok"], true, "{id}: {}", read(&out, Some(&model)));
        let cases_key = if exact { "exact_cases" } else { "preview_cases" };
        let directional = |r: &Value| matches!(r["kind"].as_str(), Some(u4b_rows::DIRECTIONAL_KIND | u4b_rows::DIRECTIONAL_MAX_KIND));
        let code = |v: &Value| v["error"]["detail"].as_str().or(v["error"]["code"].as_str()).unwrap_or("").to_string();
        // (a) The whole family dropped: rows, records and unavailable outcomes.
        let mut dropped = out.clone();
        dropped["results"].as_array_mut().unwrap().retain(|r| !directional(r));
        dropped["diagnostics"].as_array_mut().unwrap().retain(|d| d["code"] != "DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE");
        for c in dropped["contract_evidence"][cases_key].as_array_mut().unwrap() {
            c["intensified_measures"].as_array_mut().unwrap().retain(|r| r.get("i_in_plane").is_none());
            // A pressure-2 case carries the list only when it has a record.
            if exact && c["intensified_measures"].as_array().unwrap().is_empty() {
                c.as_object_mut().unwrap().remove("intensified_measures");
            }
        }
        assert_eq!(read(&dropped, None)["ok"], true, "{id}: a model-free read cannot see the dropped family: {}", read(&dropped, None));
        assert!(code(&read(&dropped, Some(&model))).contains("INTENSIFIED_MODEL_COVERAGE"), "{id}: {}", read(&dropped, Some(&model)));
        // (b) One basis dropped (pressure-2 has two).
        if exact {
            let mut one = out.clone();
            let reduced = |v: &Value| v["section_basis"] == "reduced_section"
                || v["metadata"]["basis"].as_str().is_some_and(|b| b.ends_with("; section_basis=reduced_section"));
            one["results"].as_array_mut().unwrap().retain(|r| !(directional(r) && reduced(r)));
            for c in one["contract_evidence"][cases_key].as_array_mut().unwrap() {
                c["intensified_measures"].as_array_mut().unwrap().retain(|r| !(r.get("i_in_plane").is_some() && reduced(r)));
            }
            assert_eq!(read(&one, None)["ok"], false, "{id}: one basis dropped breaks the twin rule");
            assert_eq!(read(&one, Some(&model))["ok"], false, "{id}");
        }
        // (c) The factors halved in every record, the station rows recomputed from
        // them (the maxima's upper bounds still cover the smaller stations).
        let mut halved = out.clone();
        let mut factors = HashMap::new();
        for c in halved["contract_evidence"][cases_key].as_array_mut().unwrap() {
            for r in c["intensified_measures"].as_array_mut().unwrap().iter_mut().filter(|r| r.get("i_in_plane").is_some()) {
                for k in ["i_in_plane", "i_out_of_plane"] {
                    r[k] = json!(r[k].as_f64().unwrap() / 2.0);
                }
                factors.insert(r["result_id"].as_str().unwrap().to_string(), (n(&r["i_in_plane"]), n(&r["i_out_of_plane"])));
            }
        }
        let rows = halved["results"].as_array().unwrap().clone();
        let bending = |row: &Value, axis: &str| rows.iter().find(|r| r["kind"] == format!("element_local_bending_normal_stress_{axis}") && r["entity_ref"] == row["entity_ref"]
            && r["basis_ref"] == row["basis_ref"] && r["metadata"]["location"] == row["metadata"]["location"]
            && r["metadata"]["basis"].as_str().unwrap().rsplit_once("; section_basis=").map(|x| x.1) == row["metadata"]["basis"].as_str().unwrap().rsplit_once("; section_basis=").map(|x| x.1)).unwrap()["value"].as_f64().unwrap();
        let mut recomputed = 0;
        for row in halved["results"].as_array_mut().unwrap().iter_mut().filter(|r| r["kind"] == u4b_rows::DIRECTIONAL_KIND) {
            let (i_ip, i_op) = factors[row["id"].as_str().unwrap()];
            row["value"] = json!(1e6 * norm2(i_ip * bending(row, "z"), i_op * bending(row, "y")));
            recomputed += 1;
        }
        assert_eq!(recomputed, 5 * if exact { 2 } else { 1 } * case["members"].as_array().unwrap().iter().filter(|m| m["directional_sif_user_values"].is_object()).count());
        assert_eq!(read(&halved, None)["ok"], true, "{id}: {}", read(&halved, None));
        assert!(code(&read(&halved, Some(&model))).contains("INTENSIFIED_MODEL_FACTORS"), "{id}: {}", read(&halved, Some(&model)));
        // A directional row on a member without the model's pair.
        let mut stripped = model.clone();
        for component in stripped["components"].as_array_mut().unwrap() {
            let m = component["modifiers"].as_object_mut().unwrap();
            m.remove("sif_in_plane_user_value");
            m.remove("sif_out_of_plane_user_value");
        }
        assert!(code(&read(&out, Some(&stripped))).contains("INTENSIFIED_MODEL"), "{id}");
    }
}

/// Review item 3 (§2.4): the Poisson exemption is bound. pressure-2 admits only
/// −1 < ν < 1/2, so a forged warning with the member's τ rows deleted is
/// refused by the envelope reader; on preview-physics-2 in the domain the
/// same tamper passes a model-free read (custody, as designed) and is refused
/// with the model, which forms the member's ν.
#[test]
fn u4b_forged_poisson_warnings_are_refused() {
    let (cases, _) = inputs();
    for (id, reader, model_free_refuses) in [("U4B-L90-P", "pressure", true), ("U4B-Z-PREVIEW-REV", "preview-physics", false)] {
        let case = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == id).unwrap();
        let request = build_request(case);
        let model = model_with_materials(&request);
        let out = serde_json::to_value(run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive).unwrap()).unwrap();
        let read = |doc: &Value, model: Option<&Value>| crate::result_export::reader_verdict::validate(reader, doc, model, None);
        let detail = |v: &Value| v["error"]["detail"].as_str().or(v["error"]["code"].as_str()).unwrap_or("").to_string();
        assert_eq!(read(&out, Some(&model))["ok"], true, "{id}");
        let case_id = case["load_case"]["id"].as_str().unwrap();
        let pipe = "pipe:S1";
        let mut forged = out.clone();
        forged["results"].as_array_mut().unwrap().retain(|r| !(r["kind"] == u4b_rows::SHEAR_MAX_KIND && r["entity_ref"] == pipe));
        forged["diagnostics"].as_array_mut().unwrap().push(json!({"id": "diagnostic:transverse-shear:forged:poisson", "code": u4b_rows::POISSON_OUT_OF_DOMAIN,
            "severity": "warning", "message": "forged", "affected_refs": [case_id, pipe]}));
        let free = read(&forged, None);
        if model_free_refuses {
            assert!(detail(&free).contains("SHEAR_POISSON_EXEMPTION"), "{id}: {free}");
        } else {
            assert_eq!(free["ok"], true, "{id}: the model-free read keeps custody only: {free}");
            assert!(detail(&read(&forged, Some(&model))).contains("SHEAR_POISSON_EXEMPTION"), "{id}: {}", read(&forged, Some(&model)));
        }
    }
}

/// Review item 4 (§2.4): the preview readers no longer exempt load-state cases
/// from the bit-for-bit τ check. No such case exists on preview-physics-2: the
/// producer refuses `analysis_state` without the exact pressure contract, by
/// name and with no row, so only interpolated moduli keep the model-free checks.
#[test]
fn u4b_load_state_cases_never_reach_preview() {
    let (cases, _) = inputs();
    let case = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "U4B-ARC90-OOP-PREVIEW").unwrap();
    let mut request = build_request(case);
    for material in request["model"]["materials"].as_array_mut().unwrap() {
        material["constitutive_basis"] = json!("homogeneous_isotropic_E_nu_v1");
        material["poisson_ratio"] = quantity(n(&case["materials"][0]["poisson_ratio"]), "1");
    }
    let mut load_case = request["model"]["load_cases"][0].clone();
    add_load_state(&mut request["model"], &mut load_case, case);
    request["model"]["load_cases"] = json!([load_case]);
    let out = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive).unwrap();
    assert_eq!(out.status.mechanics, "MODEL_INCOMPLETE");
    assert!(out.results.is_empty());
    assert!(out.diagnostics.iter().any(|d| d.code == "LOAD_STATE_CONTRACT_VERSION_MISMATCH" && d.severity == "blocking"), "{:?}", out.diagnostics);
}

/// §2.5 (review item 9): only R <= r_o is the named domain warning, with no
/// hoop or radial row. Any other failure from finite inputs, here an
/// overflowing hoop (p = f64::MAX), takes its own blocking path,
/// `ARC_TOROIDAL_HOOP_EVALUATION_FAILED`, which withholds the whole run.
#[test]
fn u4b_toroidal_evaluation_failure_blocks_by_name() {
    let (cases, _) = inputs();
    let case = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "U4B-ARC90-TIGHT-NSIGN-P").unwrap();
    let model: PreviewModel = serde_json::from_value(build_request(case)["model"].clone()).unwrap();
    let basis = |basis| u4b_rows::BasisSection { basis: Some(basis), outer_radius: 0.05, inner_radius: 0.045, wall: 0.005, inertia: 1e-6, area: 1e-3, modulus: 2e-5 };
    let bases = vec![basis(pressure_exact::SectionBasis::Nominal), basis(pressure_exact::SectionBasis::Reduced)];
    for (pressure, bend_radius, code, severity) in [
        (f64::MAX, 1.0, u4b_rows::TOROIDAL_EVALUATION_FAILED, "blocking"),
        (1e6, 0.04, u4b_rows::TOROIDAL_DOMAIN, "warning"),
    ] {
        let (mut results, mut diagnostics) = (Vec::new(), Vec::new());
        u4b_rows::append_station_quantities(&model, &mut results, &mut u4b_rows::MemberRows::new("pipe:BEND", Some(0)), &mut diagnostics, &u4b_rows::MemberQuantities {
            case_id: "case:x", pipe_id: "pipe:BEND", arc: true, exact: true, bases: bases.clone(), poisson: u4b_rows::ShearPoisson::Authored(0.3),
            directional: None, toroidal: Some((pressure, bend_radius)),
        });
        assert!(!results.iter().any(|r| r.kind == u4b_rows::HOOP_KIND || r.kind == u4b_rows::RADIAL_KIND), "{code}");
        assert_eq!(diagnostics.len(), 1, "{code}: {diagnostics:?}");
        assert_eq!((diagnostics[0].code.as_str(), diagnostics[0].severity.as_str()), (code, severity));
        assert_eq!(diagnostics[0].affected_refs, vec!["case:x".to_string(), "pipe:BEND".to_string()]);
        assert_eq!(has_blocking(&diagnostics), severity == "blocking", "a blocking diagnostic withholds the run (`blocked_envelope`)");
    }
}

/// §2.6: the directional SIF pair's named refusals (MODEL_INCOMPLETE, no rows).
#[test]
fn u4b_directional_sif_refusals() {
    let (cases, _) = inputs();
    let case = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "U4B-ARC90-OOP-PREVIEW").unwrap();
    let request = build_request(case);
    let refused = |request: Value, code: &str| {
        let out = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive).unwrap();
        assert_eq!(out.status.mechanics, "MODEL_INCOMPLETE", "{code}: {:?}", out.diagnostics);
        assert!(out.results.is_empty());
        assert!(out.diagnostics.iter().any(|d| d.code == code && d.severity == "blocking"), "{code}: {:?}", out.diagnostics);
    };
    fn modifiers(request: &mut Value) -> &mut Value { &mut request["model"]["components"][0]["modifiers"] }
    let mut one = request.clone();
    modifiers(&mut one).as_object_mut().unwrap().remove("sif_out_of_plane_user_value");
    refused(one, validation::BEND_DIRECTIONAL_SIF_INCOMPLETE);
    for bad in [json!({"value": 0.0, "unit": "1"}), json!({"value": -1.5, "unit": "1"}), json!({"value": 1.5, "unit": "m"})] {
        let mut wrong = request.clone();
        modifiers(&mut wrong)["sif_in_plane_user_value"] = bad;
        refused(wrong, validation::BEND_DIRECTIONAL_SIF_INCOMPLETE);
    }
    let mut unsourced = request.clone();
    modifiers(&mut unsourced).as_object_mut().unwrap().remove("source_reference");
    refused(unsourced, validation::BEND_DIRECTIONAL_SIF_INCOMPLETE);
    // On a geometry-only marker: not a realized arc.
    let mut marker = request.clone();
    marker["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only");
    refused(marker, validation::BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC);
    // The scalar SIF on a realized arc is never used as both directional factors.
    let mut scalar = request.clone();
    let m = modifiers(&mut scalar).as_object_mut().unwrap();
    m.remove("sif_in_plane_user_value");
    m.remove("sif_out_of_plane_user_value");
    m.insert("sif_user_value".into(), json!({"value": 2.0, "unit": "none"}));
    let out = run_linear_static_preview_value_with_mode(scalar, PreviewSolverMode::SparseInteractive).unwrap();
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED");
    assert!(!out.results.iter().any(|r| r.kind == u4b_rows::DIRECTIONAL_KIND || r.kind == u4b_rows::DIRECTIONAL_MAX_KIND));
    assert!(out.diagnostics.iter().any(|d| d.code == "COMPONENT_STRESS_INTENSIFICATION_NOT_APPLIED"));
}

/// DEL-03-03: `BEND_USER_MODIFIER_INPUT_MISSING` is satisfied by the user SIF
/// the bend's consumer uses: on a realized bend only the whole directional
/// pair (scalar only, a half pair or neither leave it; the scalar-only message
/// says the scalar is not applied to realized arcs), on a geometry-only marker
/// the scalar (a pair there stays refused).
#[test]
fn u4b_bend_modifier_warning_follows_the_sif_consumer() {
    let (cases, _) = inputs();
    let case = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "U4B-ARC90-OOP-PREVIEW").unwrap();
    let request = build_request(case);
    let warning = |request: Value| {
        let out = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive).unwrap();
        let message = out.diagnostics.iter().find(|d| d.code == "BEND_USER_MODIFIER_INPUT_MISSING").map(|d| d.message.clone());
        (message, out)
    };
    let scalar_not_applied = |message: &Option<String>| {
        message.as_deref().is_some_and(|m| m.contains("sif_user_value is not applied to realized arcs") && m.contains("sif_in_plane_user_value, sif_out_of_plane_user_value"))
    };
    fn modifiers(request: &mut Value) -> &mut serde_json::Map<String, Value> {
        request["model"]["components"][0]["modifiers"].as_object_mut().unwrap()
    }
    fn without_pair(request: &Value) -> Value {
        let mut request = request.clone();
        modifiers(&mut request).remove("sif_in_plane_user_value");
        modifiers(&mut request).remove("sif_out_of_plane_user_value");
        request
    }
    let scalar = json!({"value": 2.0, "unit": "none"});
    // Realized bend.
    let (message, out) = warning(request.clone());
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED");
    assert!(out.results.iter().any(|r| r.kind == u4b_rows::DIRECTIONAL_KIND), "the pair is consumed");
    assert!(message.is_none(), "pair: {:?}", out.diagnostics);
    let mut both = request.clone();
    modifiers(&mut both).insert("sif_user_value".into(), scalar.clone());
    let (message, out) = warning(both);
    assert!(message.is_none(), "pair and scalar: {:?}", out.diagnostics);
    let mut scalar_only = without_pair(&request);
    modifiers(&mut scalar_only).insert("sif_user_value".into(), scalar.clone());
    let (message, out) = warning(scalar_only);
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED");
    assert!(!out.results.iter().any(|r| r.kind == u4b_rows::DIRECTIONAL_KIND || r.kind == preview_physics::INTENSIFIED_KIND), "the scalar is not applied");
    assert!(scalar_not_applied(&message), "scalar only: {message:?}");
    let (message, out) = warning(without_pair(&request));
    assert_eq!(out.status.mechanics, "MECHANICS_SOLVED");
    assert!(message.is_some() && !scalar_not_applied(&message), "neither: {message:?}");
    for half in ["sif_in_plane_user_value", "sif_out_of_plane_user_value"] {
        let mut one = request.clone();
        modifiers(&mut one).remove(half);
        let (message, out) = warning(one.clone());
        assert!(out.diagnostics.iter().any(|d| d.code == validation::BEND_DIRECTIONAL_SIF_INCOMPLETE), "{half}");
        assert!(message.is_some() && !scalar_not_applied(&message), "without {half}: {message:?}");
        modifiers(&mut one).insert("sif_user_value".into(), scalar.clone());
        let (message, _) = warning(one);
        assert!(scalar_not_applied(&message), "scalar without {half}: {message:?}");
    }
    // Geometry-only marker.
    let marker = |request: &Value| {
        let mut request = request.clone();
        request["model"]["components"][0]["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only");
        request
    };
    let (message, out) = warning(marker(&request));
    assert!(out.diagnostics.iter().any(|d| d.code == validation::BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC));
    assert!(message.is_some() && !scalar_not_applied(&message), "pair on a marker: {message:?}");
    let mut marker_scalar = marker(&without_pair(&request));
    modifiers(&mut marker_scalar).insert("sif_user_value".into(), scalar);
    let (message, out) = warning(marker_scalar);
    assert!(message.is_none(), "scalar on a marker: {:?}", out.diagnostics);
    let (message, _) = warning(marker(&without_pair(&request)));
    assert!(message.is_some() && !scalar_not_applied(&message), "neither on a marker: {message:?}");
}

/// §7.4: end-i tension and compression with nonzero bending: σ± follow the
/// j-side stress rows, not the node-on-element end-force rows.
#[test]
fn u4b_end_i_extremes_follow_the_j_side_rows() {
    let (cases, _) = inputs();
    let z = cases["cases"].as_array().unwrap().iter().find(|c| c["case_id"] == "U4B-Z-PREVIEW-REV").unwrap();
    for sign in [1.0, -1.0] {
        let mut case = z.clone();
        // A cantilever: the first straight member alone, tip force along its axis and across it.
        let members: Vec<Value> = case["members"].as_array().unwrap().iter().take(1).cloned().collect();
        let (from, to) = (members[0]["from"].clone(), members[0]["to"].clone());
        case["members"] = json!(members);
        case["nodes"] = json!(case["nodes"].as_array().unwrap().iter().filter(|n| n["id"] == from || n["id"] == to).cloned().collect::<Vec<_>>());
        let a = case["nodes"].as_array().unwrap().iter().find(|n| n["id"] == from).unwrap()["position"].clone();
        let b = case["nodes"].as_array().unwrap().iter().find(|n| n["id"] == to).unwrap()["position"].clone();
        let axis: Vec<f64> = (0..3).map(|k| n(&b[k]) - n(&a[k])).collect();
        let length = norm3(axis[0], axis[1], axis[2]);
        let force: Vec<f64> = (0..3).map(|k| sign * 5000.0 * axis[k] / length + if k == 2 { 800.0 } else { 0.0 }).collect();
        case["supports"] = json!([{"id": "support:anchor", "node": from, "family": "anchor", "restraints": ["UX","UY","UZ","RX","RY","RZ"]}]);
        case["load_case"] = json!({"id": "case:end-i", "nodal_forces": [{"id": "load:tip", "node": to, "global_components": force}]});
        let out = run_linear_static_preview_value_with_mode(build_request(&case), PreviewSolverMode::SparseInteractive).unwrap();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{:?}", out.diagnostics);
        let pipe = members[0]["id"].as_str().unwrap();
        let v = |kind: &str, component: &str| value_on(&out, kind, pipe, "end_i", component, "preview_single_section");
        let (plus, minus) = (v(u4b_rows::EXTREME_KIND, "sigma_plus"), v(u4b_rows::EXTREME_KIND, "sigma_minus"));
        let axial_stress = v("element_local_axial_normal_stress", "");
        let end_force = v("element_local_axial_force", "");
        let h = norm2(v("element_local_bending_normal_stress_y", ""), v("element_local_bending_normal_stress_z", ""));
        assert!(h > 0.0);
        assert_eq!(plus.to_bits(), (axial_stress + h).to_bits());
        assert_eq!(minus.to_bits(), (axial_stress - h).to_bits());
        // Tension at the tip is a positive j-side axial stress at end_i, and the
        // node-on-element end-force row there has the opposite sign.
        assert_eq!(axial_stress > 0.0, sign > 0.0);
        assert_eq!(end_force < 0.0, sign > 0.0);
    }
}

/// The error of `value` against the exact c* = hi + lo (hi = RN(c*), lo = RN(c* − hi)), in u = 2^-53.
fn error_in_u(value: f64, hi: f64, lo: f64) -> f64 {
    ((value - hi) - lo) / hi / (f64::EPSILON / 2.0)
}

/// The two annuli (r_o, r_i,b, I_b) of [`PREVIEW_SHEAR_FAMILY`], binary64 operands taken as
/// exact: a thin wall (r_o − r_i exact by Sterbenz) and a thick one (r_i < r_o/2).
const SHEAR_FAMILY_SECTIONS: [(f64, f64, f64); 2] = [(0.1, 0.09, 2.6e-5), (0.05, 0.01, 4.9e-6)];

/// (section, E bits, G bits, hi bits, lo bits) with c* = ((3+2ν)r_o² + (1+2ν)r_i²)/(4(1+ν)I),
/// ν = E/(2G) − 1, evaluated in exact rational arithmetic (Python `fractions`) from the binary64
/// operands, with the Saint-Venant ν form rather than the x identity. E/(2G) runs from 1.4 down
/// to 1e-300: the Sterbenz edge 1/2 and its neighbours, ν = 0, ν near −1, the admission edge
/// (q = RN(E/(2G)) > 2^-54) and beyond it, and Codex's pair (E = 1 GPa, G = 6755399441055744 GPa).
#[rustfmt::skip]
const PREVIEW_SHEAR_FAMILY: [(usize, u64, u64, u64, u64); 38] = [
        (0, 0x42474876e8000000, 0x4230a17981249249, 0x40769205a05a05a1, 0x3cfd6ea388020134), // E/(2G) = 1.4000000000000001, q = 1.4000000000000001, nu = 0.40000000000000013
        (0, 0x42474876e8000000, 0x4232a05f20000000, 0x4076ab13b13b13b2, 0x3cdc3e4060f25dfe), // E/(2G) = 1.25, q = 1.25, nu = 0.25
        (0, 0x42474876e8000000, 0x42374876e8000000, 0x4076e589d89d89d9, 0x3d1e55eeef5657dd), // E/(2G) = 1, q = 1.0, nu = 0.0
        (0, 0x42474876e8000000, 0x423f0b4935555555, 0x407746f96f96f971, 0xbd1dc27df6b8a52c), // E/(2G) = 0.75, q = 0.75, nu = -0.25
        (0, 0x42474876e8000000, 0x42474876e7ffffff, 0x407809d89d89d89f, 0xbd1916d50367904c), // E/(2G) = 0.50000000000000011, q = 0.5000000000000001, nu = -0.4999999999999999
        (0, 0x42474876e8000000, 0x42474876e8000000, 0x407809d89d89d89f, 0xbd12cfda8245ae32), // E/(2G) = 0.5, q = 0.5, nu = -0.5
        (0, 0x42474876e8000000, 0x42474876e8000001, 0x407809d89d89d89f, 0xbd0911c00247982f), // E/(2G) = 0.49999999999999994, q = 0.49999999999999994, nu = -0.5
        (0, 0x42474876e8000000, 0x424d1a94a2000000, 0x40789c0000000001, 0x3d149d40c4ec4ec7), // E/(2G) = 0.40000000000000002, q = 0.4, nu = -0.6
        (0, 0x42474876e8000000, 0x42574876e8000000, 0x407a527627627629, 0x3d05c92535048b61), // E/(2G) = 0.25, q = 0.25, nu = -0.75
        (0, 0x42474876e8000000, 0x426d1a94a2000000, 0x4080962762762764, 0x3d2200ecf86d10ac), // E/(2G) = 0.10000000000000001, q = 0.1, nu = -0.9
        (0, 0x42474876e8000000, 0x42a2309ce5400000, 0x40a0fe0000000004, 0xbd21a37bffffffeb), // E/(2G) = 0.01, q = 0.01, nu = -0.99
        (0, 0x42474876e8000000, 0x430c6bf526340000, 0x4106580313b13b1a, 0xbda1624896bedd21), // E/(2G) = 0.0001, q = 0.0001, nu = -0.9999
        (0, 0x42474876e8000000, 0x43e158e460913d00, 0x41db3928e8400007, 0x3e777be75ea1b8a7), // E/(2G) = 1e-08, q = 1e-08, nu = -0.99999999
        (0, 0x22de7c5f127bd87e, 0x254bba08cf8c979d, 0x42b09da302a5aadd, 0x3f5c563732274216), // E/(2G) = 9.9999999999999998e-13, q = 1e-12, nu = -0.999999999999
        (0, 0x22de7c5f127bd87e, 0x261452ea0c529054, 0x43785be5be5be5db, 0xc012d759e0b1bc2b), // E/(2G) = 1.6653345369377348e-16, q = 1.6653345369377348e-16, nu = -0.9999999999999998
        (0, 0x22de7c5f127bd87e, 0x2655275ed8d8f36b, 0x43b95a88dbe7f6b9, 0xc04a3b7358cb3806), // E/(2G) = 1.0000000000000001e-17, q = 1e-17, nu = -1.0
        (0, 0x22de7c5f127bd87e, 0x37916c262777579c, 0x54f4e1aa02e93f3c, 0xd1601cc24d414d92), // E/(2G) = 1e-100, q = 1e-100, nu = -1.0
        (0, 0x22de7c5f127bd87e, 0x6116c2d4256ffcc3, 0x7e7b47aef65e102a, 0xfafa0c0fd21ef11f), // E/(2G) = 1e-300, q = 1e-300, nu = -1.0
        (0, 0x41cdcd6500000000, 0x45165a0bc0000000, 0x438b676276276288, 0x402c9c7fcf86d126), // E/(2G) = 7.4014868308343765e-17, q = 7.401486830834377e-17, nu = -0.9999999999999999
        (1, 0x42474876e8000000, 0x4230a17981249249, 0x40760c509b3df292, 0xbd043a3498523104), // E/(2G) = 1.4000000000000001, q = 1.4000000000000001, nu = 0.40000000000000013
        (1, 0x42474876e8000000, 0x4232a05f20000000, 0x4076b43eb1a1f58e, 0x3d08634a5e77a155), // E/(2G) = 1.25, q = 1.25, nu = 0.25
        (1, 0x42474876e8000000, 0x42374876e8000000, 0x40783c14e5e0a730, 0x3d0d21df51fe14b3), // E/(2G) = 1, q = 1.0, nu = 0.0
        (1, 0x42474876e8000000, 0x423f0b4935555555, 0x407ac92492492494, 0xbd1f28b332392691), // E/(2G) = 0.75, q = 0.75, nu = -0.25
        (1, 0x42474876e8000000, 0x42474876e7ffffff, 0x407fe343eb1a1f5a, 0xbd0f4aabb6447721), // E/(2G) = 0.50000000000000011, q = 0.5000000000000001, nu = -0.4999999999999999
        (1, 0x42474876e8000000, 0x42474876e8000000, 0x407fe343eb1a1f5a, 0x3d1a6d6409cf2ac4), // E/(2G) = 0.5, q = 0.5, nu = -0.5
        (1, 0x42474876e8000000, 0x42474876e8000001, 0x407fe343eb1a1f5b, 0x3cf20077bb024460), // E/(2G) = 0.49999999999999994, q = 0.49999999999999994, nu = -0.5
        (1, 0x42474876e8000000, 0x424d1a94a2000000, 0x4081db6db6db6db8, 0xbd1fa461c5c8c507), // E/(2G) = 0.40000000000000002, q = 0.4, nu = -0.6
        (1, 0x42474876e8000000, 0x42574876e8000000, 0x408798d0fac687d7, 0x3d29132665b7b5cc), // E/(2G) = 0.25, q = 0.25, nu = -0.75
        (1, 0x42474876e8000000, 0x426d1a94a2000000, 0x4097472f0539782b, 0xbcfabbe3bebf47a5), // E/(2G) = 0.10000000000000001, q = 0.1, nu = -0.9
        (1, 0x42474876e8000000, 0x42a2309ce5400000, 0x40c86f1a1f58d0fc, 0x3d2268f88cbdc9be), // E/(2G) = 0.01, q = 0.01, nu = -0.99
        (1, 0x42474876e8000000, 0x430c6bf526340000, 0x4132b0331a1f58d2, 0xbdb1ca725fbd77b7), // E/(2G) = 0.0001, q = 0.0001, nu = -0.9999
        (1, 0x42474876e8000000, 0x43e158e460913d00, 0x4206ced28d83eb1b, 0x3ea21c77c4dab1a7), // E/(2G) = 1e-08, q = 1e-08, nu = -0.99999999
        (1, 0x22de7c5f127bd87e, 0x254bba08cf8c979d, 0x42dbd777fba0e985, 0xbf7aa85f616152c4), // E/(2G) = 9.9999999999999998e-13, q = 1e-12, nu = -0.999999999999
        (1, 0x22de7c5f127bd87e, 0x261452ea0c529054, 0x43a4687d6343eb1d, 0x403de6200ec8f298), // E/(2G) = 1.6653345369377348e-16, q = 1.6653345369377348e-16, nu = -0.9999999999999998
        (1, 0x22de7c5f127bd87e, 0x2655275ed8d8f36b, 0x43e53dd3ba2a018e, 0xc081cfed9df7f8a7), // E/(2G) = 1.0000000000000001e-17, q = 1e-17, nu = -1.0
        (1, 0x22de7c5f127bd87e, 0x37916c262777579c, 0x55217ea4fc1d5523, 0xd1cb3fc9139232ab), // E/(2G) = 1e-100, q = 1e-100, nu = -1.0
        (1, 0x22de7c5f127bd87e, 0x6116c2d4256ffcc3, 0x7ea6dafdd3155664, 0xfb237a64094bc44f), // E/(2G) = 1e-300, q = 1e-300, nu = -1.0
        (1, 0x41cdcd6500000000, 0x45165a0bc0000000, 0x43b6f58d0fac6880, 0xc05a170b5af2e2a9), // E/(2G) = 7.4014868308343765e-17, q = 7.401486830834377e-17, nu = -0.9999999999999999
];

/// DEL-04-07 §2.4: on preview-physics-2, c_b from the moduli is within 8.5u of the exact
/// Saint-Venant value for every E/(2G) in (0, 3/2), down to E/(2G) → 0. Forming 1 + ν as
/// RN(RN(E/(2G)) − 1) + 1 recovered 1+ν* only to within u + 2^-54/q* relative to the exact
/// q* = E/(2G), below E/(2G) = 1/2 (33% at
/// Codex's pair). Admission is unchanged: a member whose ν = RN(q − 1) is −1 is out of the
/// domain, and its x-form value is still checked (7u) through the shared moduli terms.
#[test]
fn u4b_preview_shear_factor_from_moduli_is_within_its_bound() {
    let (mut admitted, mut below_half, mut refused) = (0, 0, 0);
    for (section, e, g, hi, lo) in PREVIEW_SHEAR_FAMILY {
        let (e, g, hi, lo) = (f64::from_bits(e), f64::from_bits(g), f64::from_bits(hi), f64::from_bits(lo));
        let (ro, ri, inertia) = SHEAR_FAMILY_SECTIONS[section];
        let q = e / (2.0 * g);
        let nu = u4b_rows::preview_poisson_ratio(e, g);
        match u4b_rows::preview_shear_factor(e, g, ro, ri, inertia) {
            Ok(c) => {
                let err = error_in_u(c, hi, lo);
                assert!(err.abs() <= 8.5, "E/(2G) = {q:e}, section {section}: c_b {c:e} is {err:.3}u from exact");
                if q >= 0.5 {
                    // The ν sequence's bits stand where its 1 + ν is exact.
                    assert_eq!(c.to_bits(), open_pipe_stress_stress_recovery::section_stress_quantities::saint_venant_shear_factor(nu, ro, ri, inertia).unwrap().to_bits());
                } else {
                    below_half += 1;
                }
                admitted += 1;
            }
            Err(error) => {
                assert_eq!(nu, -1.0, "E/(2G) = {q:e}: only ν = −1 is refused here");
                assert_eq!(error, open_pipe_stress_stress_recovery::section_stress_quantities::SectionQuantityError::PoissonOutOfDomain { nu });
                refused += 1;
            }
        }
        let (x, _) = open_pipe_stress_frame_kernel::shear::moduli_ratio_terms(e, g).unwrap();
        let c = open_pipe_stress_stress_recovery::section_stress_quantities::saint_venant_shear_factor_of_ratio(x, ro, ri, inertia).unwrap();
        let err = error_in_u(c, hi, lo);
        assert!(err.abs() <= 7.0, "E/(2G) = {q:e}, section {section}: x form {c:e} is {err:.3}u from exact");
    }
    assert_eq!((admitted, below_half, refused), (32, 20, 6));
}

/// A G > E preview cantilever (OD 0.2 m, wall 0.01 m, L 1 m, fixed root, tip restrained except UY,
/// a tip force in global Y), the moduli authored in GPa so G stays a safe integer.
fn g_above_e_request(e_gpa: f64, g_gpa: f64, force_n: f64) -> Value {
    let provenance = "invented regression model; not library, catalog, manufacturer or code-rule data";
    let material = json!({"id": "mat:m", "provenance": provenance,
        "elastic_modulus": {"unit": "GPa", "value": e_gpa}, "shear_modulus": {"unit": "GPa", "value": g_gpa},
        "thermal_expansion_coefficient": {"unit": "1/degC", "value": 1.2e-05}});
    let model = json!({
        "schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided", "rule_check": "not_performed_user_rule_inputs_missing"},
        "project": {"id": "invented:u4b:g-above-e", "units": {"angle": "rad", "force": "N", "length": "m", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "nodes": [{"id": "node:N0", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": provenance},
                  {"id": "node:N1", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": provenance}],
        "materials": [material.clone()],
        "pipe_segments": [{"id": "pipe:N01", "from": "node:N0", "to": "node:N1", "material": "mat:m", "provenance": provenance,
            "section": {"outside_diameter": {"unit": "m", "value": 0.2}, "wall_thickness": {"unit": "m", "value": 0.01}},
            "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}}],
        "supports": [{"id": "support:root", "family": "anchor", "node": "node:N0", "provenance": provenance, "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
                     {"id": "support:tip", "family": "anchor", "node": "node:N1", "provenance": provenance, "restraints": ["UX", "UZ", "RX", "RY", "RZ"]}],
        "components": [], "combinations": [],
        "load_cases": [{"id": "case:1", "kind": "primitive_user_load", "provenance": provenance, "primitive_loads": [{"id": "load:N1:fy",
            "category": "concentrated_force", "dimension": "force", "direction": "global_y", "provenance": provenance,
            "magnitude": {"unit": "N", "value": force_n}, "target": {"node": "node:N1", "type": "node"}}]}],
    });
    json!({"model": model, "materials": [material]})
}

/// The two G > E documents: (name, E GPa, G GPa, tip force N, exact c* hi bits, lo bits), c* = exact rational
/// Saint-Venant c_b (ν form, Python `fractions`) from the preview annulus's binary64 operands.
/// Codex's: E/(2G) = 1/(3·2^52), ν = RN(q − 1) = −1 + 2^-53. Moderate: E/(2G) = 0.1.
const G_ABOVE_E_DOCUMENTS: [(&str, f64, f64, f64, u64, u64); 2] = [
    ("codex_q_1_over_3_2p52", 1.0, 6755399441055744.0, 1e-18, 0x438a6117fde5fe85, 0xc0043768a891bdcb),
    ("moderate_q_0_1", 200.0, 1000.0, 1500.0, 0x407feecc3359c847, 0xbd07ffe48fdbe857),
];

/// DEL-04-07 §2.4 on documents: each τ_v,max row of a G > E preview member is within 11.5u of |V|·c*/1e6; forming
/// 1 + ν from the rounded ν published Codex's 1.584022414149910e-7 MPa, 33.3% low (Codex's independent value
/// 2.376033621224861803e-7 MPa at |V| = 1e-18 N, with an exact I, agrees to 1e-14). Euler–Bernoulli only: this base
/// refuses `timoshenko_annulus_energy_v1` at solve. The Rust preview-physics reader recomputes the same bits; the
/// Python reader reads the same envelopes from [`PREVIEW_SHEAR_MODULI_FIXTURE`].
#[test]
fn u4b_preview_shear_maximum_with_g_above_e() {
    let (od, t) = (0.2_f64, 0.01_f64);
    let id = od - 2.0 * t;
    // The product forms x^4 at run time as (x·x)·(x·x). `powi` on these constants is not reproducible: the hosted
    // x86_64 Linux build gives the correctly rounded x^4 (compile-time folding), whose I is 9 ulps away.
    let (od2, id2) = (od * od, id * id);
    let inertia = PI * (od2 * od2 - id2 * id2) / 64.0;
    assert_eq!(((od / 2.0).to_bits(), (id / 2.0).to_bits(), inertia.to_bits()), (0x3fb999999999999a, 0x3fb70a3d70a3d70b, 0x3efc52664442210e));
    for (name, e, g, force, hi, lo) in G_ABOVE_E_DOCUMENTS {
        let (hi, lo) = (f64::from_bits(hi), f64::from_bits(lo));
        let request = g_above_e_request(e, g, force);
        let out = run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive).unwrap();
        assert_eq!(out.status.mechanics, "MECHANICS_SOLVED", "{name}: {:?}", out.diagnostics);
        assert!(!out.diagnostics.iter().any(|d| d.code == u4b_rows::POISSON_OUT_OF_DOMAIN), "{name}: admitted");
        let rows = shear_rows(&out, "case:1");
        assert_eq!(rows.len(), u4b_rows::STATIONS.len(), "{name}");
        for row in rows {
            let location = &row.metadata.as_ref().unwrap().location;
            let shear = |kind: &str| out.results.iter().find(|r| r.kind == kind && r.entity_ref == row.entity_ref
                && r.metadata.as_ref().is_some_and(|m| &m.location == location)).unwrap().value;
            // One local shear carries the tip force; |V| = norm2 is then exact.
            let (vy, vz) = (shear("element_local_shear_force_y").abs(), shear("element_local_shear_force_z").abs());
            assert_eq!(vy.min(vz), 0.0, "{name} {location}");
            let v = vy.max(vz);
            // τ* = v·(hi + lo)/1e6 in double-double: v·hi = p + p_err exactly, then the quotient's residual.
            let p = v * hi;
            let p_err = v.mul_add(hi, -p);
            let tau_hi = p / 1e6;
            let tau_lo = ((-tau_hi).mul_add(1e6, p) + p_err + v * lo) / 1e6;
            let err = error_in_u(row.value, tau_hi, tau_lo);
            assert!(err.abs() <= 11.5, "{name} {location}: τ_v,max {:e} is {err:.3}u from exact {tau_hi:e}", row.value);
            if force == 1e-18 {
                let codex = 2.376033621224861803e-7 * (v / 1e-18);
                assert!((row.value / codex - 1.0).abs() <= 1e-14, "{location}: {:e} vs Codex's {codex:e}", row.value);
            }
        }
        assert_eq!(verdict(&out, "preview-physics", &model_with_materials(&request))["ok"], true, "{name}");
    }
}

/// The cross-reader fixture (`tests/fixtures/u4b_preview_shear_moduli.json`): the family's operands, exact values and
/// the product's c_b bits, and the two G > E documents with their sparse envelopes. `tests/test_u4b_preview_shear_moduli.py`
/// requires the Python reader to reproduce every c_b and to accept both envelopes with their models (bit for bit τ).
const PREVIEW_SHEAR_MODULI_FIXTURE: &str = "tests/fixtures/u4b_preview_shear_moduli.json";

#[test]
fn u4b_preview_shear_cross_reader_fixture_is_current() {
    let hex = |v: u64| format!("0x{v:016x}");
    let family: Vec<Value> = PREVIEW_SHEAR_FAMILY.iter().map(|&(section, e, g, hi, lo)| {
        let (ro, ri, inertia) = SHEAR_FAMILY_SECTIONS[section];
        let c = u4b_rows::preview_shear_factor(f64::from_bits(e), f64::from_bits(g), ro, ri, inertia).ok();
        json!({"section": section, "e": hex(e), "g": hex(g), "exact_hi": hex(hi), "exact_lo": hex(lo), "c_b": c.map(|c| hex(c.to_bits()))})
    }).collect();
    let documents: Vec<Value> = G_ABOVE_E_DOCUMENTS.iter().map(|&(name, e, g, force, _, _)| {
        let request = g_above_e_request(e, g, force);
        let out = run_linear_static_preview_value_with_mode(request.clone(), PreviewSolverMode::SparseInteractive).unwrap();
        json!({"name": name, "model": model_with_materials(&request), "envelope": serde_json::to_value(&out).unwrap()})
    }).collect();
    let sections: Vec<Value> = SHEAR_FAMILY_SECTIONS.iter().map(|&(ro, ri, inertia)| json!([hex(ro.to_bits()), hex(ri.to_bits()), hex(inertia.to_bits())])).collect();
    let fixture = json!({"sections": sections, "family": family, "documents": documents});
    let bytes = serde_json::to_string_pretty(&fixture).unwrap() + "\n";
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(PREVIEW_SHEAR_MODULI_FIXTURE);
    if std::env::var_os("U4B_WRITE_PREVIEW_SHEAR_MODULI").is_some() {
        std::fs::write(&path, &bytes).unwrap();
    }
    assert_eq!(std::fs::read_to_string(&path).unwrap(), bytes, "{}: regenerate with U4B_WRITE_PREVIEW_SHEAR_MODULI=1", path.display());
}
