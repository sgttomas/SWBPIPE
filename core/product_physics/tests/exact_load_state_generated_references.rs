//! Generated static-equivalent loads on a load/reference-state document, on
//! the exact route (`3.0.0/exact_pressure_v3`), against the two frozen
//! independent references (DEL-05-01 REQ-008, AC-007 and VER-005; DEL-04-07
//! REQ-011; issue 452):
//!
//! - `validation/references/exact_load_state_equivalent_static`: wind and
//!   seismic beside a temperature state and internal pressure, on a
//!   Timoshenko straight run (STRAIGHT) and an Euler-Bernoulli frame (FRAME),
//!   at both material points, with pressure elongation off and on;
//! - `validation/references/exact_load_state_generated_interactions`:
//!   generated loads beside a constant-effort support (CEF), a lift-off
//!   support that seismic uplift lifts (LO), a connector-replaced span (JR),
//!   the seismic terms formed by plain subtraction and their SRSS and
//!   absolute-sum combinations, and the refusals.
//!
//! Each package is pinned to the hash in its SHA256SUMS. Nothing here derives
//! an expectation from product code; the documents are written from the
//! packages' inputs. Both solver modes are run.
//!
//! Comparison (the packages' rule, DEC-024/DEC-026): a nonzero expected row
//! passes when |observed - expected| <= 1e-9 |expected|; an exactly zero row
//! passes when |observed| <= floor. The interactions package freezes each
//! item's floors (`comparison_floor`). The equivalent-static package sets no
//! tolerance of its own; its floors here follow the same rule: 1e-9 times the
//! largest expected magnitude of the row's family in the same case and
//! setting, or 1e-9 N, N*m and 1e-12 m, rad where the family is all zero.
//!
//! Every comparison counts its rows and asserts the count, so that no filter
//! can pass by selecting nothing. The equivalent-static package's
//! `components` (16 load components, 864 rows) are not bound here. Each is
//! the response to one ingredient of a combined case (thermal, wind, seismic
//! or pressure elongation) that is not itself a case of the package. The
//! package states that "`components` gives every component's full rows, and a
//! cross-check confirms that one direct solve equals the sum" (README,
//! "Superposition"), and the direct cases, whose 864 rows are bound, are what
//! a document solves.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const EQS_REFERENCES: &str = include_str!("../../../validation/references/exact_load_state_equivalent_static/references.json");
const EQS_SUMS: &str = include_str!("../../../validation/references/exact_load_state_equivalent_static/SHA256SUMS");
const GI_REFERENCES: &str = include_str!("../../../validation/references/exact_load_state_generated_interactions/references.json");
const GI_SUMS: &str = include_str!("../../../validation/references/exact_load_state_generated_interactions/SHA256SUMS");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const RELATIVE: f64 = 1.0e-9;
const PROV: &str = "invented exact load-state generated-load reference input; not library, catalogue or code data";
const CONTRACT: &str = "openpipestress.load_reference_state/1.0.0";
const MATERIAL: &str = "material:ref";
const GENERATED_KIND: &str = "generated_equivalent_static";

// ------------------------------------------------------------------ common
fn frozen(references: &str, sums: &str) -> Value {
    let digest: String = Sha256::digest(references.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = sums.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    let doc: Value = serde_json::from_str(references).unwrap();
    assert!(doc["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
    doc
}
fn eqs() -> Value {
    serde_json::from_str(EQS_REFERENCES).unwrap()
}
fn gi() -> Value {
    serde_json::from_str(GI_REFERENCES).unwrap()
}
fn ratio(text: &Value) -> f64 {
    let s = text.as_str().unwrap_or_else(|| panic!("rational text {text}"));
    match s.split_once('/') {
        Some((p, q)) => p.parse::<f64>().unwrap() / q.parse::<f64>().unwrap(),
        None => s.parse().unwrap(),
    }
}
fn input(doc: &Value, key: &str) -> f64 {
    ratio(&doc["inputs"][key]["exact"])
}
fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}
fn value(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("not a value record: {v}"))
}
fn run(wire: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(wire.clone(), mode).unwrap_or_else(|e| panic!("typed boundary: {e}"))).unwrap()
}
fn codes(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap_or(""))).collect()
}
fn blocking(out: &Value) -> Vec<String> {
    out["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking")
        .map(|d| format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap_or(""))).collect()
}
fn solved(wire: &Value, mode: PreviewSolverMode, what: &str) -> Value {
    let out = run(wire, mode);
    assert_eq!(out["status"]["mechanics"], "MECHANICS_SOLVED", "{what} {mode:?}: blocking {:#?}", blocking(&out));
    out
}
fn refused(wire: &Value, code: &str, what: &str) -> Vec<Value> {
    let mut found = Vec::new();
    for mode in MODES {
        let out = run(wire, mode);
        assert_eq!(out["status"]["mechanics"], "MODEL_INCOMPLETE", "{what} {mode:?}: {:?}", codes(&out));
        assert!(out["results"].as_array().unwrap().is_empty(), "{what}: a refused document publishes no rows");
        assert!(out["contract_evidence"]["load_reference_states"].as_array().is_none_or(Vec::is_empty), "{what}: no record");
        found = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code && d["severity"] == "blocking").cloned().collect();
        assert!(!found.is_empty(), "{what} {mode:?}: expected {code}, got {:?}", codes(&out));
    }
    found
}

/// Where a reference item's rows are published.
#[derive(Clone, Copy)]
struct At<'a> {
    ref_type: &'a str,
    ref_id: &'a str,
}
fn case_at(id: &str) -> At<'_> {
    At { ref_type: "load_case", ref_id: id }
}
fn combination_at(id: &str) -> At<'_> {
    At { ref_type: "combination", ref_id: id }
}

/// The one published row matching, as a value.
fn row(out: &Value, at: At<'_>, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>) -> f64 {
    let found = out["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity
            && location.is_none_or(|l| r["metadata"]["location"] == l)
            && component.is_none_or(|c| r["metadata"]["component"] == c)
            && r["basis_ref"]["ref_type"] == at.ref_type && r["basis_ref"]["ref_id"] == at.ref_id
    }).collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity} {location:?} {component:?} @{}: {} rows", at.ref_id, found.len());
    found[0]["value"].as_f64().unwrap()
}

/// The unit axis, end i to end j, of a member of a model of the
/// equivalent-static package, from the frozen coordinates.
fn member_axis(doc: &Value, name: &str, pipe: &str) -> [f64; 3] {
    let model = &doc["models"][name];
    {
        if let Some(member) = model["members"].as_array().unwrap().iter().find(|m| m["id"] == pipe) {
            let point = |node: &Value| -> [f64; 3] {
                let c = model["nodes"][node.as_str().unwrap()].as_array().unwrap();
                [ratio(&c[0]), ratio(&c[1]), ratio(&c[2])]
            };
            let (i, j) = (point(&member["end_i"]), point(&member["end_j"]));
            let d = [j[0] - i[0], j[1] - i[1], j[2] - i[2]];
            let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            return [d[0] / length, d[1] / length, d[2] / length];
        }
    }
    panic!("{name}: no member {pipe}")
}

/// One reference row id read from the envelope, in the reference's SI unit.
fn observed(out: &Value, at: At<'_>, id: &str) -> f64 {
    let parts = id.split('/').collect::<Vec<_>>();
    match parts.as_slice() {
        ["motion", node, dof] => {
            let axis = &dof[1..];
            if dof.starts_with('u') {
                row(out, at, &format!("global_nodal_displacement_{axis}"), node, None, None) / 1000.0
            } else {
                row(out, at, &format!("global_nodal_rotation_{axis}"), node, None, None)
            }
        }
        ["reaction", support, component] => row(out, at, "support_reaction_component_v2", support, None, Some(component)),
        ["member_end", pipe, end, component] => {
            let kind = if component.starts_with('F') { "element_global_end_force_v1" } else { "element_global_end_moment_v1" };
            row(out, at, kind, pipe, Some(&format!("end_{end}")), Some(component))
        }
        ["constant_effort", support, "Fz"] => row(out, at, "constant_effort_support_applied_load", support, None, None),
        ["nonlinear_support", support, "reaction_Fz"] => row(out, at, "nonlinear_support_final_reaction", support, None, None),
        ["nonlinear_support", support, "relative_uz"] => row(out, at, "nonlinear_support_final_displacement", support, None, None) / 1000.0,
        _ => panic!("row id {id}"),
    }
}

/// The comparison of one item's rows: (expected, floor by family) per row
/// id; returns the number compared and the largest relative error.
fn compare(out: &Value, at: At<'_>, rows: &[(String, String, f64)], floors: &BTreeMap<String, f64>, what: &str) -> (usize, f64) {
    let mut worst = 0.0f64;
    for (id, family, expected) in rows {
        let got = observed(out, at, id);
        if *expected == 0.0 {
            let floor = floors[family];
            assert!(got.abs() <= floor, "{what} {id}: observed {got:.17e}, expected exact zero (floor {floor:.3e})");
        } else {
            let error = (got - expected).abs() / expected.abs();
            assert!(error <= RELATIVE, "{what} {id}: observed {got:.17e}, expected {expected:.17e}, relative {error:.3e}");
            worst = worst.max(error);
        }
    }
    (rows.len(), worst)
}

/// The rule's floors from expected rows alone: 1e-9 times the family's
/// largest magnitude, or the all-zero floors.
fn rule_floors(rows: &[(String, String, f64)]) -> BTreeMap<String, f64> {
    let mut floors = BTreeMap::new();
    for (family, zero) in [("force", 1e-9), ("moment", 1e-9), ("translation", 1e-12), ("rotation", 1e-12)] {
        let largest = rows.iter().filter(|(_, f, _)| f == family).map(|(_, _, v)| v.abs()).fold(0.0, f64::max);
        floors.insert(family.to_string(), if largest == 0.0 { zero } else { RELATIVE * largest });
    }
    floors
}
fn frozen_floors(item: &Value) -> BTreeMap<String, f64> {
    item["comparison_floor"].as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.as_f64().unwrap())).collect()
}

/// The case's published record of load sources (`contributions`).
fn record<'a>(out: &'a Value, case: &str) -> &'a Value {
    out["contract_evidence"]["load_reference_states"].as_array().unwrap().iter().find(|r| r["load_case_id"] == case)
        .unwrap_or_else(|| panic!("no load/reference-state record for {case}"))
}
fn generated_sources(out: &Value, case: &str) -> Vec<Value> {
    record(out, case)["contributions"].as_array().unwrap().iter().filter(|c| c["owner_kind"] == GENERATED_KIND).cloned().collect()
}
/// DEL-04-07 REQ-011: each expected generated source appears exactly once in
/// the case's record of load sources, with its bound identity and factor 1,
/// and no other generated source does; the record equals the exact case's
/// generation record. `expected` is (member index, kind, axis, part span).
fn assert_generated_record(out: &Value, case: &str, expected: &[(usize, &str, &str, bool)], what: &str) {
    let published = generated_sources(out, case);
    assert_eq!(published.len(), expected.len(), "{what}: one record entry per generated load; {published:#?}");
    for (index, kind, axis, part) in expected {
        let id = format!("{case}:generated:{kind}:{axis}:element-{index}");
        let matching = published.iter().filter(|c| {
            let source = c["source_id"].as_str().unwrap();
            if *part { source.starts_with(&format!("{id}:extent-")) } else { source == id }
        }).collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{what}: {id} (part {part}) exactly once in {published:#?}");
        assert_eq!(matching[0]["factor"], json!(1.0), "{what}: factor 1");
        assert_eq!(matching[0]["generation_kind"], json!(kind), "{what}");
        assert_eq!(matching[0]["classification"], "generated_applied", "{what}");
    }
    let exact = out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case).unwrap();
    let ids = published.iter().map(|c| c["source_id"].clone()).collect::<Vec<_>>();
    if expected.is_empty() {
        assert!(exact.get("equivalent_static").is_none(), "{what}: no generation record");
    } else {
        assert_eq!(exact["equivalent_static"]["generated_load_ids"], json!(ids), "{what}: the record equals the generation record");
    }
}

fn axis_token(axis: usize) -> &'static str {
    ["global-x", "global-y", "global-z"][axis]
}

// ------------------------------------------------------------------ equivalent-static package
/// One model of the equivalent-static package: nodes, members, anchors, its
/// shear model and its pressure regions.
fn eqs_model(doc: &Value, name: &str) -> (Value, Vec<String>) {
    let m = &doc["models"][name];
    let nodes = m["nodes"].as_object().unwrap().iter().map(|(id, xyz)| {
        json!({"id": id, "position": {"x": ratio(&xyz[0]), "y": ratio(&xyz[1]), "z": ratio(&xyz[2])}, "provenance": PROV})
    }).collect::<Vec<_>>();
    let section = |s: &str| json!({
        "outside_diameter": qv(input(doc, &format!("{s}_D")), "m"), "wall_thickness": qv(input(doc, &format!("{s}_t")), "m"),
        "mill_tolerance": qv(input(doc, &format!("{s}_cm")), "m"), "corrosion_allowance": qv(input(doc, &format!("{s}_ca")), "m"),
        "material_density": qv(input(doc, &format!("{s}_rho_metal")), "kg/m^3"),
        "contents_density": qv(input(doc, &format!("{s}_rho_contents")), "kg/m^3"),
        "insulation_thickness": qv(input(doc, &format!("{s}_tins")), "m"),
        "insulation_density": qv(input(doc, &format!("{s}_rho_insulation")), "kg/m^3")});
    let pipes = m["members"].as_array().unwrap().iter().map(|p| {
        json!({"id": p["id"], "from": p["end_i"], "to": p["end_j"], "section": section(p["section"].as_str().unwrap()),
            "material": MATERIAL, "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV})
    }).collect::<Vec<_>>();
    let ids = pipes.iter().map(|p| p["id"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    let supports = m["anchors"].as_array().unwrap().iter().map(|a| json!({"id": a["id"], "node": a["node"], "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})).collect::<Vec<_>>();
    let t_inst = input(doc, "T_installation");
    let t_op = input(doc, "T_operating");
    let nu = input(doc, "nu");
    let material = json!({"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": qv(input(doc, "E_installation"), "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV,
        "temperature_points": [
            {"id": "point:installation", "temperature": qv(t_inst, "K"), "elastic_modulus": qv(input(doc, "E_installation"), "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV},
            {"id": "point:operating", "temperature": qv(t_op, "K"), "elastic_modulus": qv(input(doc, "E_operating"), "Pa"), "poisson_ratio": qv(nu, "1"), "provenance": PROV}],
        "expansion_laws": [{"id": "law:secant", "definition": "engineering_secant", "datum_temperature": qv(t_inst, "K"),
            "data": {"kind": "constant", "coefficient": qv(input(doc, "alpha"), "1/K")}, "provenance": PROV}]});
    let shear = if m["shear_model"] == "timoshenko_annulus_energy_v1" { "timoshenko_annulus_energy_v1" } else { "euler_bernoulli" };
    let model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear},
        "project": {"id": format!("project:{name}"), "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "K"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [], "materials": [material],
        "reference_configurations": [{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": ids.iter().map(|p| json!({"pipe_ref": p, "basis": {"kind": "temperature_reference", "installation_temperature": qv(t_inst, "K")},
                "fit": {"kind": "none"}, "provenance": PROV})).collect::<Vec<_>>(), "provenance": PROV}],
        "load_cases": [], "combinations": []});
    (model, ids)
}

fn eqs_regions(doc: &Value, name: &str) -> Value {
    json!(doc["models"][name]["pressure_regions"].as_array().unwrap().iter().map(|r| json!({"id": r["id"], "member_pipe_ids": r["member_pipe_ids"],
        "pressure_basis": r["pressure_basis"], "pressure": qv(ratio(&r["pressure_Pa"]), "Pa"),
        "terminals": r["terminals"].as_array().unwrap().iter().map(|t| json!({"node_ref": t["node_ref"], "closure_transfer": t["closure_transfer"], "provenance": PROV})).collect::<Vec<_>>(),
        "provenance": PROV})).collect::<Vec<_>>())
}

/// The case's generation inputs, from its generated loads' sources.
fn eqs_generation(doc: &Value, name: &str, case: &Value) -> Option<Value> {
    let sources = case["generated_loads"].as_array().unwrap().iter().map(|g| g["source"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    let first = sources.first()?;
    if first.ends_with("/wind") {
        let w = &doc["models"][name]["wind"];
        Some(json!({"wind": {"pressure": qv(input(doc, "wind_pressure"), "Pa"), "shape_factor": qv(input(doc, "wind_shape_factor"), "1"),
            "direction": w["direction"], "exposed_pipe_refs": w["exposed_pipe_refs"],
            "exposed_spans": w["exposed_spans"].as_array().unwrap().iter().map(|s| json!({"pipe_ref": s["pipe_ref"],
                "start_fraction": qv(ratio(&s["start_fraction"]), "1"), "end_fraction": qv(ratio(&s["end_fraction"]), "1")})).collect::<Vec<_>>()},
            "provenance": PROV}))
    } else {
        let axis = first.rsplit('_').next().unwrap();
        let mut seismic = json!({"gravity_acceleration": qv(input(doc, "gravity"), "m/s^2")});
        seismic[format!("g_factor_{axis}")] = qv(input(doc, &format!("g_factor_{axis}")), "1");
        Some(json!({"seismic": seismic, "provenance": PROV}))
    }
}

fn eqs_case(doc: &Value, name: &str, ids: &[String], case: &Value) -> Value {
    let t_op = input(doc, "T_operating");
    let t_inst = input(doc, "T_installation");
    let state = &case["load_reference_state"];
    let element_states = state["element_states"].as_array().unwrap().iter().map(|e| {
        let operating = e["material_selection"] == "point at the operating temperature";
        let (point, temperature) = if operating { ("point:operating", t_op) } else { ("point:installation", t_inst) };
        let thermal = if e["thermal_state"] == "none" {
            json!({"kind": "unchanged_reference", "provenance": PROV})
        } else {
            json!({"kind": "free_length_state", "expansion_law_ref": "law:secant"})
        };
        json!({"pipe_ref": e["pipe_ref"], "operating_temperature": qv(temperature, "K"),
            "material_selection": {"kind": "exact_point", "material_ref": MATERIAL, "point_ref": point}, "thermal_state": thermal})
    }).collect::<Vec<_>>();
    assert_eq!(element_states.len(), ids.len());
    let mut out = json!({"id": case["id"], "provenance": PROV, "primitive_loads": [],
        "pressure_regions": if case["pressure_regions"] == "none" { json!([]) } else { eqs_regions(doc, name) },
        "analysis_state": {"contract": CONTRACT, "reference_configuration_ref": "reference:installed", "element_states": element_states,
            "support_states": doc["models"][name]["anchors"].as_array().unwrap().iter().map(|a| json!({"support_ref": a["id"], "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>(),
            "load_sources": [], "history": {"kind": "independent_equilibrium"}, "provenance": PROV}});
    if let Some(generation) = eqs_generation(doc, name, case) {
        out["equivalent_static"] = generation;
    }
    out
}

/// One document per model with all its cases; pressure elongation on lists
/// every pressurised case.
fn eqs_document(doc: &Value, name: &str, elongation: bool) -> Value {
    let (mut model, ids) = eqs_model(doc, name);
    let cases = doc["cases"].as_array().unwrap().iter().filter(|c| c["model"] == name).map(|c| eqs_case(doc, name, &ids, c)).collect::<Vec<_>>();
    if elongation {
        let pressurised = cases.iter().filter(|c| !c["pressure_regions"].as_array().unwrap().is_empty()).map(|c| c["id"].clone()).collect::<Vec<_>>();
        model["model_settings"] = json!({"pressure_elongation": {"operating_load_cases": pressurised}});
    }
    model["load_cases"] = json!(cases);
    json!({"model": model, "materials": []})
}

fn eqs_rows(case: &Value, elongation: bool) -> Vec<(String, String, f64)> {
    case["rows"].as_array().unwrap().iter().map(|r| {
        let expected = match r["pressure_elongation"].as_str() {
            Some("dependent") => value(&r[if elongation { "expected_pressure_elongation_on" } else { "expected_pressure_elongation_off" }]),
            _ => value(&r["expected"]),
        };
        (r["id"].as_str().unwrap().to_string(), r["family"].as_str().unwrap().to_string(), expected)
    }).collect()
}

/// The package's member-end rows give the element (effective) force N_el;
/// the product's `element_global_end_force_v1` is the wall force N_w. The
/// package states the relation N_w = N_el + p·A_i under either pressure
/// setting (README, "Pressure-setting dependence", "Forces in both
/// settings"), with A_i the pressure-bore area. So on each member of a
/// pressurised case's regions the expected wall end force is formed from
/// frozen values alone: the axial force of a member is −F·a at end i and
/// +F·a at end j (README, "Member end"), so the wall end force is the
/// package's end force − p·A_i·a at end i and + p·A_i·a at end j, with a the
/// member's unit axis from the frozen coordinates. p is `inputs.p`; A_i is
/// `derived.sections.<section>.A_i_m2`, the section the model's member
/// names; the members are those of `models.<name>.pressure_regions[].member_pipe_ids`;
/// a case is pressurised when its `pressure_regions` is "as the model". No
/// product row enters an expected value. Returns the rows and the count
/// of rows the relation moved.
fn eqs_wall_rows(doc: &Value, name: &str, case: &Value, rows: Vec<(String, String, f64)>) -> (Vec<(String, String, f64)>, usize) {
    if case["pressure_regions"] != "as the model" {
        return (rows, 0);
    }
    let model = &doc["models"][name];
    let p = ratio(&doc["inputs"]["p"]["exact"]);
    let pressurised = model["pressure_regions"].as_array().unwrap().iter()
        .flat_map(|r| r["member_pipe_ids"].as_array().unwrap().iter().map(|m| m.as_str().unwrap().to_string()))
        .collect::<Vec<_>>();
    let mut moved = 0;
    let rows = rows.into_iter().map(|(id, family, expected)| {
        let parts = id.split('/').collect::<Vec<_>>();
        if let ["member_end", pipe, end, component] = parts.as_slice() {
            if component.starts_with('F') && pressurised.iter().any(|m| m == pipe) {
                let member = model["members"].as_array().unwrap().iter().find(|m| m["id"] == *pipe).unwrap();
                let area = value(&doc["derived"]["sections"][member["section"].as_str().unwrap()]["A_i_m2"]);
                let axis = member_axis(doc, name, pipe)[match *component { "Fx" => 0, "Fy" => 1, _ => 2 }];
                let sign = if *end == "i" { -1.0 } else { 1.0 };
                if axis != 0.0 {
                    moved += 1;
                }
                return (id.clone(), family, expected + sign * p * area * axis);
            }
        }
        (id, family, expected)
    }).collect();
    (rows, moved)
}

/// (member index, kind, axis, part span) of a case's generated loads.
fn eqs_expected_generated(doc: &Value, name: &str, ids: &[String], case: &Value) -> Vec<(usize, &'static str, &'static str, bool)> {
    let wind_axis = match doc["models"][name]["wind"]["direction"].as_str().unwrap() { "global_x" => 0, "global_y" => 1, _ => 2 };
    case["generated_loads"].as_array().unwrap().iter().map(|g| {
        let index = ids.iter().position(|p| *p == g["member"]).unwrap();
        let source = g["source"].as_str().unwrap();
        if source.contains("/wind") {
            (index, "wind", axis_token(wind_axis), source.contains('['))
        } else {
            let axis = match source.rsplit('_').next().unwrap() { "x" => 0, "y" => 1, _ => 2 };
            (index, "seismic", axis_token(axis), false)
        }
    }).collect()
}

#[test]
fn equivalent_static_references_are_the_frozen_bytes() {
    let doc = frozen(EQS_REFERENCES, EQS_SUMS);
    assert_eq!(doc["cases"].as_array().unwrap().len(), 16);
}

/// Every row of every case of STRAIGHT and FRAME, with pressure elongation
/// off and on, in both solver modes; every generating case records each
/// generated source once at factor 1; the reference cases publish zero
/// without a finding.
#[test]
fn equivalent_static_cases_reproduce_the_reference() {
    let doc = eqs();
    let mut compared = 0;
    let mut wall_moved = 0;
    let mut worst = 0.0f64;
    for name in ["STRAIGHT", "FRAME"] {
        let (_, ids) = eqs_model(&doc, name);
        for elongation in [false, true] {
            let wire = eqs_document(&doc, name, elongation);
            for mode in MODES {
                let what = format!("{name} elongation {elongation} {mode:?}");
                let out = solved(&wire, mode, &what);
                for case in doc["cases"].as_array().unwrap().iter().filter(|c| c["model"] == name) {
                    let id = case["id"].as_str().unwrap();
                    let frozen_rows = eqs_rows(case, elongation);
                    // The floors are the rule's, from the frozen rows.
                    let floors = rule_floors(&frozen_rows);
                    let (rows, n_wall) = eqs_wall_rows(&doc, name, case, frozen_rows.clone());
                    wall_moved += n_wall;
                    let (n, w) = compare(&out, case_at(id), &rows, &floors, &format!("{what} {id}"));
                    compared += n;
                    worst = worst.max(w);
                    assert_generated_record(&out, id, &eqs_expected_generated(&doc, name, &ids, case), &format!("{what} {id}"));
                }
            }
        }
    }
    assert_eq!(compared, 16 * 54 * 2 * 2, "every row in both settings and both modes");
    assert_eq!(wall_moved, 224, "the wall end forces of the pressurised members, in both settings and both modes");
    assert!(worst <= RELATIVE);
}

/// The package's refusals on the load/reference-state document: a missing
/// wind or seismic input, a span on a member that is not a pipe and an
/// inverted span are refused by name, with no rows and no record.
#[test]
fn equivalent_static_refusals_are_refused_by_name() {
    let doc = eqs();
    for name in ["STRAIGHT", "FRAME"] {
        let base = eqs_document(&doc, name, false);
        let case_index = |wire: &Value, id: &str| wire["model"]["load_cases"].as_array().unwrap().iter().position(|c| c["id"] == id).unwrap();
        let hot = format!("{name}-WIND-HOT");
        let seismic = format!("{name}-SEISMIC-Y");
        let mutations: Vec<(&str, &str, Box<dyn Fn(&mut Value)>)> = vec![
            ("REFUSE-WIND-NO-SHAPE-FACTOR", "EQUIVALENT_STATIC_INPUT_MISSING", Box::new(|c: &mut Value| { c["equivalent_static"]["wind"].as_object_mut().unwrap().remove("shape_factor"); })),
            ("REFUSE-WIND-NO-PRESSURE", "EQUIVALENT_STATIC_INPUT_MISSING", Box::new(|c: &mut Value| { c["equivalent_static"]["wind"].as_object_mut().unwrap().remove("pressure"); })),
            ("REFUSE-WIND-NO-DIRECTION", "EQUIVALENT_STATIC_INPUT_MISSING", Box::new(|c: &mut Value| { c["equivalent_static"]["wind"].as_object_mut().unwrap().remove("direction"); })),
            ("REFUSE-WIND-SPAN-NOT-A-PIPE", "EQUIVALENT_STATIC_INPUT_INVALID", Box::new(|c: &mut Value| { c["equivalent_static"]["wind"]["exposed_spans"][0]["pipe_ref"] = json!("support:a"); })),
            ("REFUSE-WIND-SPAN-INVERTED", "EQUIVALENT_STATIC_INPUT_INVALID", Box::new(|c: &mut Value| {
                let span = &mut c["equivalent_static"]["wind"]["exposed_spans"][0];
                let (a, b) = (span["start_fraction"].clone(), span["end_fraction"].clone());
                span["start_fraction"] = b;
                span["end_fraction"] = a;
            })),
        ];
        for (id, code, mutate) in &mutations {
            let mut wire = base.clone();
            let index = case_index(&wire, &hot);
            mutate(&mut wire["model"]["load_cases"][index]);
            let found = refused(&wire, code, &format!("{name} {id}"));
            assert!(found.iter().any(|d| d["affected_refs"].as_array().unwrap().contains(&json!(hot))), "{name} {id}: names the case");
        }
        let mut wire = base.clone();
        let index = case_index(&wire, &seismic);
        wire["model"]["load_cases"][index]["equivalent_static"]["seismic"].as_object_mut().unwrap().remove("gravity_acceleration");
        let found = refused(&wire, "EQUIVALENT_STATIC_INPUT_MISSING", &format!("{name} REFUSE-SEISMIC-NO-GRAVITY"));
        assert!(found.iter().any(|d| d["affected_refs"] == json!([seismic, "gravity_acceleration"])));
    }
}

// ------------------------------------------------------------------ generated-interactions package
fn gi_section(doc: &Value, s: &str) -> Value {
    json!({"outside_diameter": qv(input(doc, &format!("{s}_D")), "m"), "wall_thickness": qv(input(doc, &format!("{s}_t")), "m"),
        "mill_tolerance": qv(input(doc, &format!("{s}_cm")), "m"), "corrosion_allowance": qv(input(doc, &format!("{s}_ca")), "m"),
        "material_density": qv(input(doc, &format!("{s}_rho_metal")), "kg/m^3"),
        "contents_density": qv(input(doc, &format!("{s}_rho_contents")), "kg/m^3"),
        "insulation_thickness": qv(input(doc, &format!("{s}_tins")), "m"),
        "insulation_density": qv(input(doc, &format!("{s}_rho_insulation")), "kg/m^3")})
}

/// A model of the interactions package as a document without cases: every
/// member Euler-Bernoulli (the package's binding), the one material point
/// selected by an explicit base selection, the anchors, constant-effort and
/// lift-off supports, and JR's connector.
fn gi_model(doc: &Value, name: &str) -> Value {
    let m = &doc["models"][name];
    let nodes = m["nodes"].as_object().unwrap().iter().map(|(id, xyz)| {
        json!({"id": id, "position": {"x": ratio(&xyz[0]), "y": ratio(&xyz[1]), "z": ratio(&xyz[2])}, "provenance": PROV})
    }).collect::<Vec<_>>();
    let y_ref = |i: &str, j: &str| {
        let (a, b) = (&m["nodes"][i], &m["nodes"][j]);
        if ratio(&a[2]) != ratio(&b[2]) { json!({"x": 1.0, "y": 0.0, "z": 0.0}) } else { json!({"x": 0.0, "y": 0.0, "z": 1.0}) }
    };
    let mut pipes = m["members"].as_array().unwrap().iter().map(|p| {
        json!({"id": p["id"], "from": p["i"], "to": p["j"], "section": gi_section(doc, p["section"].as_str().unwrap()),
            "material": MATERIAL, "y_reference": y_ref(p["i"].as_str().unwrap(), p["j"].as_str().unwrap()), "provenance": PROV})
    }).collect::<Vec<_>>();
    for span in m["replaced_spans"].as_array().into_iter().flatten() {
        pipes.push(json!({"id": span["id"], "from": span["i"], "to": span["j"], "section": gi_section(doc, span["section"].as_str().unwrap()),
            "material": MATERIAL, "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV}));
    }
    let mut supports = m["anchors"].as_object().unwrap().iter().map(|(id, node)| json!({"id": id, "node": node, "family": "anchor",
        "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV})).collect::<Vec<_>>();
    for (id, s) in m["constant_effort_supports"].as_object().into_iter().flatten() {
        assert_eq!(s["dof"], "uz");
        supports.push(json!({"id": id, "node": s["node"], "family": "constant_effort_support", "restraints": ["UZ"],
            "hanger": {"hanger_type": "constant_effort_support", "constant_load": qv(ratio(&s["constant_load_N"]), "N"),
                "travel_range": qv(0.05, "m"), "source_reference": "invented generated-interactions reference"}, "provenance": PROV}));
    }
    for (id, s) in m["lift_off_supports"].as_object().into_iter().flatten() {
        assert_eq!(s["dof"], "uz");
        supports.push(json!({"id": id, "node": s["node"], "family": "nonlinear", "restraints": [],
            "nonlinear": {"behavior": "one_way", "dof": "UZ", "initial_state": "active", "active_when": "positive_reaction"}, "provenance": PROV}));
    }
    let mut components = Vec::new();
    for span in m["replaced_spans"].as_array().into_iter().flatten() {
        components.push(json!({"id": span["connector"], "label": "Invented connector (both faces anchored)", "kind": "expansion_joint",
            "node": span["j"], "provenance": PROV,
            "objective_connector": {"version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
                "end_i": {"node_ref": span["i"], "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
                "end_j": {"node_ref": span["j"], "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
                "connector_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
                "installed_reference_temperature": {"value": 20, "unit": "degC"},
                "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
                "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
                "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                    "translation_scale": {"value": 0.5, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"}, "coefficient_unit": "N*m",
                    "upper_triangle": [800000, 0, 0, 0, 0, 0, 225000, 0, 0, 0, 0, 225000, 0, 0, 0, 620000, 0, 0, 480000, 0, 480000],
                    "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                    "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none"}},
                "topology": {"type": "replaces_span", "span_ref": span["id"]},
                "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": false, "installed_geometry": "authored model geometry"},
                "hardware": {"kind": "untied"}, "pressure_model": {"kind": "unpressurized"},
                "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none", "validity_statement": "invented analytical control; no applicability claim"}}}));
    }
    let material = json!({"id": MATERIAL, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": qv(input(doc, "E"), "Pa"), "poisson_ratio": qv(input(doc, "nu"), "1"), "provenance": PROV});
    let ids = pipes.iter().map(|p| p["id"].clone()).collect::<Vec<_>>();
    json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": format!("project:{name}"), "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "K"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components, "materials": [material],
        "reference_configurations": [{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": ids.iter().map(|p| json!({"pipe_ref": p, "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV})).collect::<Vec<_>>(),
            "provenance": PROV}],
        "load_cases": [], "combinations": []})
}

/// The weight primitives of `name` as stored primitives, one per source label.
fn gi_weights(doc: &Value, name: &str) -> Vec<(String, Value)> {
    doc["weight_primitives"][name].as_array().unwrap().iter().map(|w| {
        let source = w["source"].as_str().unwrap().to_string();
        let id = format!("weight:{}", source.replace('/', ":"));
        assert_eq!(w["kind"], "member load per length, whole span", "{source}: only member weights are authored here");
        let q = w["intensity_N_per_m"].as_array().unwrap();
        assert!(value(&q[0]) == 0.0 && value(&q[1]) == 0.0);
        (source, json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": w["target"]}, "direction": "global_z",
            "magnitude": qv(value(&q[2]), "N/m"), "dimension": "force_per_length", "provenance": PROV}))
    }).collect()
}

/// A case of the interactions package: its declared ledger lists the weight
/// primitives the reference names, every support is an active device, every
/// member keeps its unchanged reference, and its generation is the case's
/// seismic or wind input.
fn gi_case(doc: &Value, case: &Value, model: &Value) -> Value {
    let name = case["model"].as_str().unwrap();
    let weights = if doc["weight_primitives"].get(name).is_some() { gi_weights(doc, name) } else { Vec::new() };
    let ledger = case["load_sources"]["declared_ledger"].as_array().unwrap();
    let mut primitives = Vec::new();
    let mut sources = Vec::new();
    for entry in ledger {
        let (_, primitive) = weights.iter().find(|(s, _)| *s == entry["source"]).unwrap_or_else(|| panic!("ledger source {}", entry["source"]));
        assert_eq!(entry["factor"], 1);
        // Primitive-load ids are unique across the model, so each case's copy
        // carries the case id.
        let mut primitive = primitive.clone();
        primitive["id"] = json!(format!("{}:{}", case["id"].as_str().unwrap(), primitive["id"].as_str().unwrap()));
        sources.push(json!({"source_ref": primitive["id"], "factor": 1.0}));
        primitives.push(primitive);
    }
    let element_states = model["pipe_segments"].as_array().unwrap().iter().map(|p| json!({"pipe_ref": p["id"],
        "material_selection": {"kind": "explicit_base_properties", "material_ref": MATERIAL, "applicability_reference": "invented analytical basis declared applicable to this member"},
        "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}})).collect::<Vec<_>>();
    let support_states = model["supports"].as_array().unwrap().iter().map(|s| json!({"support_ref": s["id"], "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>();
    let mut out = json!({"id": case["id"], "provenance": PROV, "primitive_loads": primitives, "pressure_regions": [],
        "analysis_state": {"contract": CONTRACT, "reference_configuration_ref": "reference:installed", "element_states": element_states,
            "support_states": support_states, "load_sources": sources, "history": {"kind": "independent_equilibrium"}, "provenance": PROV}});
    let seismic = if case["seismic"].is_object() { Some(&case["seismic"]) } else { case["inputs"].get("seismic") };
    if let Some(seismic) = seismic.filter(|s| s.is_object()) {
        let mut s = json!({"gravity_acceleration": qv(ratio(&seismic["gravity_acceleration_m_s2"]), "m/s^2")});
        for (axis, k) in ["x", "y", "z"].iter().zip(seismic["g_factors"].as_array().unwrap()) {
            if ratio(k) != 0.0 {
                s[format!("g_factor_{axis}")] = qv(ratio(k), "1");
            }
        }
        out["equivalent_static"] = json!({"seismic": s, "provenance": PROV});
    }
    if let Some(wind) = case["inputs"].get("wind") {
        let marks = wind["marks"].as_array().unwrap();
        out["equivalent_static"] = json!({"wind": {"pressure": qv(ratio(&wind["pressure_Pa"]), "Pa"), "shape_factor": qv(ratio(&wind["shape_factor"]), "1"),
            "direction": wind["direction"],
            "exposed_pipe_refs": marks.iter().filter(|m| m["mark"] == "whole span").map(|m| m["member"].clone()).collect::<Vec<_>>(),
            "exposed_spans": marks.iter().filter(|m| m["mark"] == "part span").map(|m| json!({"pipe_ref": m["member"],
                "start_fraction": qv(ratio(&m["start_fraction"]), "1"), "end_fraction": qv(ratio(&m["end_fraction"]), "1")})).collect::<Vec<_>>()},
            "provenance": PROV});
    }
    out
}

fn gi_case_ref<'a>(doc: &'a Value, id: &str) -> &'a Value {
    doc["cases"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("no case {id}"))
}
fn gi_document(doc: &Value, name: &str, case_ids: &[&str]) -> Value {
    let mut model = gi_model(doc, name);
    let cases = case_ids.iter().map(|id| gi_case(doc, gi_case_ref(doc, id), &model)).collect::<Vec<_>>();
    model["load_cases"] = json!(cases);
    json!({"model": model, "materials": []})
}
fn gi_rows(item: &Value) -> Vec<(String, String, f64)> {
    item["rows"].as_array().unwrap().iter().map(|r| (r["id"].as_str().unwrap().to_string(), r["family"].as_str().unwrap().to_string(), value(&r["expected"]))).collect()
}
/// (member index, kind, axis, part) for a case of the interactions package.
fn gi_expected_generated(doc: &Value, case: &Value, model: &Value) -> Vec<(usize, &'static str, &'static str, bool)> {
    let pipes = model["pipe_segments"].as_array().unwrap();
    let mut expected = Vec::new();
    for g in case["load_sources"]["generated"].as_array().unwrap() {
        let source = g["source"].as_str().unwrap();
        let (member, rest) = source.split_once('/').unwrap();
        let index = pipes.iter().position(|p| p["id"] == member).unwrap_or_else(|| panic!("{}: generated source {source} names no member", case["id"]));
        if rest.starts_with("seismic") {
            let seismic = if case["seismic"].is_object() { &case["seismic"] } else { &case["inputs"]["seismic"] };
            for (axis, k) in seismic["g_factors"].as_array().unwrap().iter().enumerate() {
                if ratio(k) != 0.0 {
                    expected.push((index, "seismic", axis_token(axis), false));
                }
            }
        } else {
            expected.push((index, "wind", axis_token(2), rest.contains('[')));
        }
    }
    let _ = doc;
    expected
}

#[test]
fn generated_interactions_references_are_the_frozen_bytes() {
    let doc = frozen(GI_REFERENCES, GI_SUMS);
    assert_eq!(doc["cases"].as_array().unwrap().len(), 11);
}

/// Model LO: the lift-off support stays in contact under the smaller g-factor
/// and lifts under the larger, the constant-effort force acts in every case,
/// every row of the qualifying state matches, both terms formed by plain
/// subtraction match (the lifted term is not the seated structure's response,
/// the control the reference freezes), and each generated source is recorded
/// once at factor 1. The lift-off threshold is formed from the product's own
/// rows as the package derives it (`lift_off_threshold.derivation`): with R0
/// the support's reaction in LO-SUS and R(k) = R0 + k R1 while seated,
/// R1 = (R(k_seated) - R0) / k_seated and k* = -R0 / R1, compared with the
/// frozen k* under the comparison rule.
#[test]
fn lift_off_support_under_seismic_uplift() {
    let doc = gi();
    let ids = ["LO-SUS", "LO-EQZ-SEATED", "LO-EQZ-LIFTED"];
    let mut wire = gi_document(&doc, "LO", &ids);
    wire["model"]["combinations"] = json!(doc["terms"].as_array().unwrap().iter().filter(|t| t["id"].as_str().unwrap().starts_with("LO-")).map(|t| {
        assert_eq!(t["basis"], "result_state_subtraction");
        json!({"id": t["id"], "basis": "result_state_subtraction", "minuend_id": t["minuend"], "subtrahend_id": t["subtrahend"], "provenance": PROV})
    }).collect::<Vec<_>>());
    let model = wire["model"].clone();
    let threshold = value(&doc["lift_off_threshold"]["g_factor_z"]);
    for mode in MODES {
        let out = solved(&wire, mode, "LO");
        let (mut compared, mut as_published) = (0, 0);
        for id in ids {
            let case = gi_case_ref(&doc, id);
            let want = if case["expected_support_status"]["support:lo-b"].as_str().unwrap().starts_with("in contact") { "active" } else { "inactive" };
            let state = out["results"].as_array().unwrap().iter().find(|r| r["kind"] == "nonlinear_support_active_set_state_code"
                && r["entity_ref"] == "support:lo-b" && r["basis_ref"]["ref_id"] == id).unwrap_or_else(|| panic!("{id}: state row"));
            let token = state["metadata"]["basis"].as_str().unwrap().split("final_state=").nth(1).unwrap().split(';').next().unwrap().trim().to_string();
            assert_eq!(token, want, "{id} {mode:?}: support status");
            compared += compare(&out, case_at(id), &gi_rows(case), &frozen_floors(case), &format!("LO {id} {mode:?}")).0;
            // The same two lift-off quantities as a combination publishes
            // them, against the same expected values, so that both forms are
            // checked wherever both are published.
            as_published += compare(&out, case_at(id), &lift_off_as_published(&doc, &gi_rows(case)), &frozen_floors(case), &format!("LO {id} {mode:?} (as published)")).0;
            assert_generated_record(&out, id, &gi_expected_generated(&doc, case, &model), &format!("LO {id} {mode:?}"));
        }
        for term in doc["terms"].as_array().unwrap().iter().filter(|t| t["id"].as_str().unwrap().starts_with("LO-")) {
            let id = term["id"].as_str().unwrap();
            compared += compare(&out, combination_at(id), &lift_off_as_published(&doc, &gi_rows(term)), &frozen_floors(term), &format!("LO term {id} {mode:?}")).0;
        }
        assert_eq!(compared, 255, "{mode:?}: the three cases' rows and the two terms' rows");
        assert_eq!(as_published, 153, "{mode:?}: the three cases' rows, the lift-off quantities as a combination publishes them");
        let reaction = |id: &str| observed(&out, case_at(id), "nonlinear_support/support:lo-b/reaction_Fz");
        let r0 = reaction("LO-SUS");
        let r1 = (reaction("LO-EQZ-SEATED") - r0) / input(&doc, "LO_k_seated");
        let formed = -r0 / r1;
        assert!((formed - threshold).abs() <= RELATIVE * threshold.abs(), "{mode:?}: threshold from the product's rows {formed:.17e}, frozen {threshold:.17e}");
    }
    // The threshold lies strictly between the two g-factors.
    let k = value(&doc["lift_off_threshold"]["g_factor_z"]);
    assert!(input(&doc, "LO_k_seated") < k && k < input(&doc, "LO_k_lifted"));
}

/// The package's lift-off rows (`nonlinear_support/<support>/reaction_Fz`:
/// the support's force on the pipe; `relative_uz`: the node's uz less its
/// ground's, the ground not moving; README, "Conventions") read as an exact
/// combination publishes them. A combination carries no
/// `nonlinear_support_*` row kinds; it carries the support's reaction
/// component (`reaction/<support>/Fz`) and the node's global translation
/// (`motion/<node>/uz`), which are those two quantities. The node is the
/// frozen `models.LO.lift_off_supports.<support>.node`. Expected values and
/// families are unchanged.
fn lift_off_as_published(doc: &Value, rows: &[(String, String, f64)]) -> Vec<(String, String, f64)> {
    rows.iter().filter_map(|(id, family, expected)| {
        let parts = id.split('/').collect::<Vec<_>>();
        let ["nonlinear_support", support, quantity] = parts.as_slice() else { return None };
        let id = match *quantity {
            "reaction_Fz" => format!("reaction/{support}/Fz"),
            "relative_uz" => format!("motion/{}/uz", doc["models"]["LO"]["lift_off_supports"][*support]["node"].as_str().unwrap()),
            other => panic!("lift-off row {other}"),
        };
        Some((id, family.clone(), *expected))
    }).chain(rows.iter().filter(|(id, _, _)| !id.starts_with("nonlinear_support/")).cloned()).collect()
}

/// Model JR: a case whose only action is generation solves (its ledger
/// selects no stored primitive), the replaced span carries no generated load
/// and publishes no member result, whole- and part-span wind match, and an
/// unlisted stored primitive with a distinct identity is excluded (the JR-EQ
/// rows).
#[test]
fn connector_replaced_span_and_generation_only_cases() {
    let doc = gi();
    let ids = ["JR-EQ", "JR-WIND", "JR-WIND-PART"];
    let wire = gi_document(&doc, "JR", &ids);
    let model = wire["model"].clone();
    for mode in MODES {
        let out = solved(&wire, mode, "JR");
        let mut compared = 0;
        for id in ids {
            let case = gi_case_ref(&doc, id);
            compared += compare(&out, case_at(id), &gi_rows(case), &frozen_floors(case), &format!("JR {id} {mode:?}")).0;
            assert_generated_record(&out, id, &gi_expected_generated(&doc, case, &model), &format!("JR {id} {mode:?}"));
            assert!(!generated_sources(&out, id).iter().any(|c| c["source_id"].as_str().unwrap().ends_with(&format!("element-{}", 4))),
                "{id}: no generated load on the replaced span (pipe index 4)");
        }
        assert!(!out["results"].as_array().unwrap().iter().any(|r| r["entity_ref"] == "pipe:b-c"), "the replaced span publishes no member result");
        assert_eq!(compared, 324, "{mode:?}: the three cases' rows");
    }
    // JR-EQ-UNLISTED-PRIMITIVE: JR-EQ with a distinct unlisted nodal force.
    let decoy = &doc["inputs"];
    let mut unlisted = gi_document(&doc, "JR", &["JR-EQ"]);
    let force = &decoy["JR_decoy_force"]["exact"];
    assert!(ratio(&force[0]) == 0.0 && ratio(&force[1]) == 0.0);
    let force = ratio(&force[2]);
    let node = decoy["JR_decoy_node"]["exact"].as_str().unwrap();
    unlisted["model"]["load_cases"][0]["primitive_loads"] = json!([{"id": "decoy:unlisted", "category": "concentrated_force",
        "target": {"type": "node", "node": node}, "direction": "global_z", "magnitude": qv(force, "N"), "dimension": "force", "provenance": PROV}]);
    let jr_eq = gi_case_ref(&doc, "JR-EQ");
    for mode in MODES {
        let out = solved(&unlisted, mode, "JR-EQ-UNLISTED-PRIMITIVE");
        assert!(codes(&out).iter().any(|c| c.starts_with("LOAD_STATE_SOURCE_EXCLUDED")), "{mode:?}: the unlisted primitive is excluded by name");
        let (compared, _) = compare(&out, case_at("JR-EQ"), &gi_rows(jr_eq), &frozen_floors(jr_eq), &format!("JR-EQ-UNLISTED-PRIMITIVE {mode:?}"));
        assert_eq!(compared, 108, "{mode:?}: JR-EQ's rows");
        let excluded = &record(&out, "JR-EQ")["excluded_sources"];
        assert_eq!(excluded.as_array().unwrap().len(), 1);
        assert_eq!(excluded[0]["source_id"], "decoy:unlisted");
    }
}

/// The interactions package's three refusals: wind on the replaced span,
/// whole and part span, refused by name identifying pipe:b-c; and the
/// generated identity that collides with an unlisted stored primitive,
/// refused before assembly by EQUIVALENT_STATIC_GENERATED_ID_COLLISION
/// (decision 2 (ii) of issue 452; the package leaves the code undecided).
#[test]
fn generated_interactions_refusals() {
    let doc = gi();
    for (id, part) in [("JR-REFUSE-WIND-ON-REPLACED-SPAN", false), ("JR-REFUSE-PART-SPAN-WIND-ON-REPLACED-SPAN", true)] {
        let mut wire = gi_document(&doc, "JR", &["JR-WIND"]);
        let wind = &mut wire["model"]["load_cases"][0]["equivalent_static"]["wind"];
        if part {
            wind["exposed_spans"].as_array_mut().unwrap().push(json!({"pipe_ref": "pipe:b-c", "start_fraction": qv(0.2, "1"), "end_fraction": qv(0.65, "1")}));
        } else {
            wind["exposed_pipe_refs"].as_array_mut().unwrap().push(json!("pipe:b-c"));
        }
        let found = refused(&wire, "EQUIVALENT_STATIC_WIND_SPAN_REPLACED", id);
        assert!(found.iter().all(|d| d["affected_refs"].as_array().unwrap().contains(&json!("pipe:b-c"))), "{id}: names pipe:b-c");
    }
    let mut wire = gi_document(&doc, "JR", &["JR-EQ"]);
    let generated = "JR-EQ:generated:seismic:global-z:element-0";
    wire["model"]["load_cases"][0]["primitive_loads"] = json!([{"id": generated, "category": "concentrated_force",
        "target": {"type": "node", "node": "node:m"}, "direction": "global_z", "magnitude": qv(5150.0, "N"), "dimension": "force", "provenance": PROV}]);
    let found = refused(&wire, "EQUIVALENT_STATIC_GENERATED_ID_COLLISION", "JR-REFUSE-GENERATED-IDENTITY-COLLISION");
    assert_eq!(found.len(), 1);
    assert_eq!(found[0]["affected_refs"], json!(["JR-EQ", generated]));
    assert!(found[0]["message"].as_str().unwrap().contains("not listed in analysis_state.load_sources"));
}

/// Decision 2 (iv) of issue 452 on the packages' models: a plain exact
/// document and the load/reference-state document over the same resolved
/// generation inputs (the same material point, no thermal state, every
/// support active, every stored primitive applied once at factor 1) are true
/// twins. In both modes every published row is equal whole (kind, entity,
/// basis, metadata, unit and value, the value bit for bit), the envelopes'
/// standing (status and numerical quality, with each case's) is the same, and
/// each case's generation record is the same. That holds for the lift-off,
/// the connector-replaced and generation-only, and the constant-effort models
/// (its terms and combinations included). STRAIGHT and FRAME have no plain
/// twin. Each of their cases selects every member's material point at a
/// stated operating temperature through the group (`exact_point`), and their
/// thermal cases carry a free-length state under the secant expansion law.
/// Neither is an input of a plain document, so no plain document has the
/// same resolved inputs.
#[test]
fn plain_and_load_state_documents_agree_bit_for_bit() {
    let doc = gi();
    let twins = [
        ("LO", gi_document(&doc, "LO", &["LO-SUS", "LO-EQZ-SEATED", "LO-EQZ-LIFTED"])),
        ("JR", gi_document(&doc, "JR", &["JR-EQ", "JR-WIND", "JR-WIND-PART"])),
        ("CEF", cef_document(&doc)),
    ];
    for (name, load_state) in twins {
        let mut plain = load_state.clone();
        plain["model"].as_object_mut().unwrap().remove("reference_configurations");
        for case in plain["model"]["load_cases"].as_array_mut().unwrap() {
            case.as_object_mut().unwrap().remove("analysis_state");
        }
        for mode in MODES {
            let whole = |out: &Value| {
                let mut rows = out["results"].as_array().unwrap().iter().map(|r| {
                    let mut r = r.clone();
                    r["value"] = json!(format!("{:016x}", r["value"].as_f64().unwrap().to_bits()));
                    serde_json::to_string(&r).unwrap()
                }).collect::<Vec<_>>();
                rows.sort();
                rows
            };
            let records = |out: &Value| out["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
                .map(|c| (c["load_case_id"].clone(), c["equivalent_static"].clone())).collect::<Vec<_>>();
            let standing = |out: &Value| json!({"status": out["status"], "numerical_quality": out["numerical_quality"]});
            let (p, l) = (solved(&plain, mode, &format!("{name} plain")), solved(&load_state, mode, &format!("{name} load state")));
            let (pr, lr) = (whole(&p), whole(&l));
            assert!(!pr.is_empty());
            let differing = pr.iter().zip(&lr).filter(|(a, b)| a != b).take(3).collect::<Vec<_>>();
            assert_eq!(pr.len(), lr.len(), "{name} {mode:?}: the same rows");
            assert!(differing.is_empty(), "{name} {mode:?}: rows differ: {differing:#?}");
            assert_eq!(standing(&l), standing(&p), "{name} {mode:?}: the same standing");
            assert!(records(&p).iter().any(|(_, r)| r.is_object()), "{name} {mode:?}: some case generates");
            assert_eq!(records(&l), records(&p), "{name} {mode:?}: the same generation records");
        }
    }
}

/// The CEF terms and combinations over generating cases are plain result
/// state subtractions (no difference_basis) and SRSS and absolute sums of
/// them. The existing combination path admits them on a load/reference-state
/// document: each term and the SRSS, absolute-sum and sustained-plus-terms
/// combinations publish rows, and the SRSS of the cases and the signed sum
/// of the terms are withheld whole by the fixed-load count gate, each by its
/// code. The numbers are compared by the test below.
#[test]
fn seismic_terms_and_their_combinations_are_admitted_over_generating_cases() {
    let doc = gi();
    let wire = cef_document(&doc);
    for mode in MODES {
        let out = solved(&wire, mode, "CEF");
        let mut withheld_ids = 0;
        for item in doc["terms"].as_array().unwrap().iter().chain(doc["combinations"].as_array().unwrap()).filter(|t| t["id"].as_str().unwrap().starts_with("CEF-")) {
            let id = item["id"].as_str().unwrap();
            let published = out["results"].as_array().unwrap().iter().any(|r| r["basis_ref"]["ref_type"] == "combination" && r["basis_ref"]["ref_id"] == id);
            let withheld = item["expected_outcome"].as_str().unwrap_or("").starts_with("withheld");
            assert_eq!(published, !withheld, "{id} {mode:?}: expected outcome {}; diagnostics {:?}", item["expected_outcome"], codes(&out));
            let naming = out["diagnostics"].as_array().unwrap().iter().filter(|d| d["affected_refs"].as_array().is_some_and(|r| r.contains(&json!(id))))
                .map(|d| d["code"].as_str().unwrap().to_string()).collect::<Vec<_>>();
            // The fixed-load count gate (DEL-05-02 REQ-011) by its code (the
            // user guide's names): a mechanics combination whose count is not
            // 1 needs a solve; an SRSS or absolute sum of operands whose count
            // is not exactly 0 counts the constant-effort force once per term.
            let gate = match item["basis"].as_str().unwrap() {
                "mechanics" => "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE",
                _ => "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND",
            };
            assert_eq!(naming.iter().any(|code| code == gate), withheld, "{id} {mode:?}: the gate {gate} names a withheld combination and only it; {naming:?}");
            withheld_ids += usize::from(withheld);
        }
        assert_eq!(withheld_ids, 2, "{mode:?}: the SRSS of the cases and the signed sum of the terms are withheld");
    }
}

/// The CEF document: the four cases, the three terms and the combinations
/// the reference names (SRSS and absolute sum of the terms, the SRSS of the
/// cases, the signed sum and the sustained-plus-terms sum).
fn cef_document(doc: &Value) -> Value {
    let mut wire = gi_document_cef(doc);
    let mut combinations = Vec::new();
    for t in doc["terms"].as_array().unwrap().iter().filter(|t| t["id"].as_str().unwrap().starts_with("CEF-")) {
        assert_eq!(t["basis"], "result_state_subtraction");
        combinations.push(json!({"id": t["id"], "basis": "result_state_subtraction", "minuend_id": t["minuend"], "subtrahend_id": t["subtrahend"], "provenance": PROV}));
    }
    for c in doc["combinations"].as_array().unwrap().iter().filter(|c| c["id"].as_str().unwrap().starts_with("CEF-")) {
        let terms = c["operands"].as_array().unwrap().iter().zip(c["factors"].as_array().unwrap()).map(|(operand, factor)| {
            if operand.as_str().unwrap().contains("-d") { json!({"combination": operand, "factor": factor}) } else { json!({"load_case": operand, "factor": factor}) }
        }).collect::<Vec<_>>();
        combinations.push(json!({"id": c["id"], "basis": c["basis"], "terms": terms, "provenance": PROV}));
    }
    wire["model"]["combinations"] = json!(combinations);
    wire
}

/// CEF's model and cases: the valve body pipe:b-c carries the valve's mapped
/// rigid component (weight W at node:c with its centre of gravity, stiffness
/// scale 5/2); the weight primitives are the reference's, the valve's weight
/// entering as its nodal force and moment at node:c.
fn gi_document_cef(doc: &Value) -> Value {
    let mut model = gi_model(doc, "CEF");
    let valve = &doc["models"]["CEF"]["components"]["component:valve-c"];
    model["components"] = json!([{"id": "component:valve-c", "kind": "valve", "node": valve["node"],
        "geometry": {"rigid_pipe_ref": valve["rigid_pipe_ref"], "weight": qv(ratio(&valve["weight_N"]), "N"),
            "center_of_gravity": {"x": ratio(&valve["center_of_gravity_m"][0]), "y": ratio(&valve["center_of_gravity_m"][1]), "z": ratio(&valve["center_of_gravity_m"][2]), "unit": "m"}},
        "modifiers": {"stiffness_scaling_user_value": qv(ratio(&valve["stiffness_scale"]), "1"), "source_reference": "invented generated-interactions reference"},
        "mechanics_interface": {"solver_consumption": valve["solver_consumption"]}, "provenance": PROV}]);
    let weights = doc["weight_primitives"]["CEF"].as_array().unwrap();
    let mut primitives = Vec::new();
    for w in weights {
        let source = w["source"].as_str().unwrap();
        let id = format!("weight:{}", source.replace('/', ":"));
        if let Some(q) = w["intensity_N_per_m"].as_array() {
            assert_eq!(w["kind"], "member load per length, whole span");
            primitives.push(json!({"id": id, "category": "distributed_force", "target": {"type": "element", "pipe": w["target"]}, "direction": "global_z",
                "magnitude": qv(value(&q[2]), "N/m"), "dimension": "force_per_length", "provenance": PROV}));
        } else {
            assert!(value(&w["force_N"][0]) == 0.0 && value(&w["force_N"][1]) == 0.0);
            for (axis, key) in [("global_z", "force_N"), ("rotation_x", "moment_N_m"), ("rotation_y", "moment_N_m"), ("rotation_z", "moment_N_m")] {
                let index = match axis { "global_z" => 2, "rotation_x" => 0, "rotation_y" => 1, _ => 2 };
                let magnitude = value(&w[key][index]);
                if magnitude != 0.0 {
                    let (category, dimension, unit) = if key == "force_N" { ("concentrated_force", "force", "N") } else { ("concentrated_moment", "moment", "N*m") };
                    primitives.push(json!({"id": format!("{id}:{axis}"), "category": category, "target": {"type": "node", "node": w["target"]}, "direction": axis,
                        "magnitude": qv(magnitude, unit), "dimension": dimension, "provenance": PROV}));
                }
            }
        }
    }
    let element_states = model["pipe_segments"].as_array().unwrap().iter().map(|p| json!({"pipe_ref": p["id"],
        "material_selection": {"kind": "explicit_base_properties", "material_ref": MATERIAL, "applicability_reference": "invented analytical basis declared applicable to this member"},
        "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}})).collect::<Vec<_>>();
    let support_states = model["supports"].as_array().unwrap().iter().map(|s| json!({"support_ref": s["id"], "participation": {"kind": "active_model_device"}})).collect::<Vec<_>>();
    let cases = ["CEF-SUS", "CEF-EQX", "CEF-EQY", "CEF-EQZ"].iter().map(|id| {
        let case = gi_case_ref(doc, id);
        // Primitive-load ids are unique across the model: each case's copy carries the case id.
        let primitives = primitives.iter().map(|p| { let mut p = p.clone(); p["id"] = json!(format!("{id}:{}", p["id"].as_str().unwrap())); p }).collect::<Vec<_>>();
        let mut out = json!({"id": id, "provenance": PROV, "primitive_loads": primitives, "pressure_regions": [],
            "analysis_state": {"contract": CONTRACT, "reference_configuration_ref": "reference:installed", "element_states": element_states,
                "support_states": support_states, "load_sources": primitives.iter().map(|p| json!({"source_ref": p["id"], "factor": 1.0})).collect::<Vec<_>>(),
                "history": {"kind": "independent_equilibrium"}, "provenance": PROV}});
        if case["seismic"].is_object() {
            let mut s = json!({"gravity_acceleration": qv(ratio(&case["seismic"]["gravity_acceleration_m_s2"]), "m/s^2")});
            for (axis, k) in ["x", "y", "z"].iter().zip(case["seismic"]["g_factors"].as_array().unwrap()) {
                if ratio(k) != 0.0 {
                    s[format!("g_factor_{axis}")] = qv(ratio(k), "1");
                }
            }
            out["equivalent_static"] = json!({"seismic": s, "provenance": PROV});
        }
        out
    }).collect::<Vec<_>>();
    model["load_cases"] = json!(cases);
    json!({"model": model, "materials": []})
}

/// DEL-04-07 REQ-011 on model CEF: the package lists one generated source
/// per unreplaced member and one for the valve (`load_sources.generated`,
/// labels local to the package), and states each generated load
/// (`generated_loads`): a member's whole-span load, and the valve's force and
/// moment about its node. The product records each generated primitive once
/// at factor 1, in the generator's order: the members' loads in model order
/// (by their element index), then the valve's force and each nonzero moment
/// component under its own identity
/// (`<case>:generated:seismic:<axis>:component:<length>:<component>:<force | moment:axis>`).
/// The expected list is formed from the package alone: the case's one
/// nonzero g-factor gives the axis, and the package's nonzero moment
/// components give the moments. On each seismic case of CEF that is six
/// entries: three members, the valve force and two moments; the sustained
/// case generates nothing and has no generation record. The record must
/// be exactly that list, each entry `generated_applied` of kind seismic at
/// factor 1, and it must equal the case's generation record.
fn assert_cef_generated_record(out: &Value, wire: &Value, case_id: &str, case: &Value, what: &str) {
    let pipes = wire["model"]["pipe_segments"].as_array().unwrap();
    let exact = out["contract_evidence"]["exact_cases"].as_array().unwrap().iter().find(|c| c["load_case_id"] == case_id).unwrap();
    if !case["seismic"].is_object() {
        assert!(generated_sources(out, case_id).is_empty() && case["load_sources"]["generated"].as_array().is_none_or(Vec::is_empty), "{what}: nothing generated");
        assert!(exact.get("equivalent_static").is_none(), "{what}: no generation record");
        return;
    }
    let factors = case["seismic"]["g_factors"].as_array().unwrap();
    let axes = (0..3).filter(|&a| ratio(&factors[a]) != 0.0).collect::<Vec<_>>();
    assert_eq!(axes.len(), 1, "{what}: one seismic axis");
    let axis = axis_token(axes[0]);
    let listed = case["load_sources"]["generated"].as_array().unwrap().iter().map(|g| {
        assert_eq!(g["factor"], 1, "{what}");
        g["source"].as_str().unwrap().to_string()
    }).collect::<Vec<_>>();
    let stated = case["generated_loads"].as_array().unwrap();
    assert_eq!(stated.iter().map(|g| g["source"].as_str().unwrap().to_string()).collect::<Vec<_>>(), listed, "{what}: the package states each listed source");
    let mut expected = Vec::new();
    for load in stated {
        if let Some(member) = load["member"].as_str() {
            let index = pipes.iter().position(|p| p["id"] == member).unwrap_or_else(|| panic!("{what}: {member}"));
            expected.push(format!("{case_id}:generated:seismic:{axis}:element-{index}"));
        } else {
            let component = load["component"].as_str().unwrap();
            let item = format!("{case_id}:generated:seismic:{axis}:component:{}:{component}", component.len());
            expected.push(format!("{item}:force"));
            for (i, name) in ["x", "y", "z"].iter().enumerate() {
                if value(&load["moment_about_node_N_m"][i]) != 0.0 {
                    expected.push(format!("{item}:moment:{name}"));
                }
            }
        }
    }
    assert_eq!(expected.len(), 6, "{what}: three members, the valve force and two moments; {expected:#?}");
    let published = generated_sources(out, case_id);
    let ids = published.iter().map(|c| c["source_id"].as_str().unwrap().to_string()).collect::<Vec<_>>();
    assert_eq!(ids, expected, "{what}: the record of generated sources");
    for entry in &published {
        assert_eq!(entry["factor"], json!(1.0), "{what}: {entry}");
        assert_eq!(entry["generation_kind"], "seismic", "{what}: {entry}");
        assert_eq!(entry["classification"], "generated_applied", "{what}: {entry}");
    }
    assert_eq!(exact["equivalent_static"]["generated_load_ids"], json!(ids), "{what}: the record equals the generation record");
}

/// Model CEF's rows: the four cases (the constant-effort force included),
/// the three seismic terms and the published combinations, against the
/// reference; each generated source recorded once at factor 1, the valve's
/// component force included.
#[test]
fn constant_effort_support_seismic_terms_and_combinations() {
    let doc = gi();
    let wire = cef_document(&doc);
    for mode in MODES {
        let out = solved(&wire, mode, "CEF");
        let mut compared = 0;
        for id in ["CEF-SUS", "CEF-EQX", "CEF-EQY", "CEF-EQZ"] {
            let case = gi_case_ref(&doc, id);
            compared += compare(&out, case_at(id), &gi_rows(case), &frozen_floors(case), &format!("CEF {id} {mode:?}")).0;
            assert_cef_generated_record(&out, &wire, id, case, &format!("CEF {id} {mode:?}"));
        }
        let mut published = Vec::new();
        for item in doc["terms"].as_array().unwrap().iter().chain(doc["combinations"].as_array().unwrap())
            .filter(|t| t["id"].as_str().unwrap().starts_with("CEF-") && t.get("rows").is_some_and(|r| r.as_array().is_some_and(|r| !r.is_empty()))) {
            let id = item["id"].as_str().unwrap();
            compared += compare(&out, combination_at(id), &gi_rows(item), &frozen_floors(item), &format!("CEF {id} {mode:?}")).0;
            assert_combination_arithmetic(&out, item, &format!("CEF {id} {mode:?}"));
            published.push(id.to_string());
        }
        assert_eq!(published, ["CEF-dEQX", "CEF-dEQY", "CEF-dEQZ", "CEF-EQ-SRSS", "CEF-EQ-ABS", "CEF-SUS-PLUS-EQ"], "{mode:?}");
        assert_eq!(compared, 910, "{mode:?}: the four cases' rows, the three terms' and the three published combinations'");
    }
}

/// A published row's identity within its basis: kind, entity, location and
/// component.
fn row_key(r: &Value) -> String {
    format!("{}|{}|{}|{}", r["kind"], r["entity_ref"], r["metadata"]["location"], r["metadata"]["component"])
}

/// The correctly rounded square root of the exact sum of the squares of
/// `zs` (DEL-05-02, SRSS): the binary64 c whose rounding interval holds the
/// exact sum, judged by exact arithmetic on c and the half gaps to its
/// neighbours (each a power of two, so each square and cross term is exact),
/// a tie going to the even significand.
fn exact_srss(zs: &[f64]) -> f64 {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    let mut sum = ExactAccumulator::new();
    for z in zs {
        sum.add_product(*z, *z).unwrap();
    }
    // The sign of S - (c + h)^2, h a signed power of two.
    let against = |c: f64, h: f64| {
        let mut d = sum.clone();
        d.add_product(-c, c).unwrap();
        d.add_product(-2.0 * c, h).unwrap();
        d.add_product(-h, h).unwrap();
        d.signum()
    };
    let mut c = sum.round().unwrap().sqrt();
    if c == 0.0 {
        return 0.0;
    }
    for _ in 0..8 {
        let below = c - f64::from_bits(c.to_bits() - 1);
        let above = f64::from_bits(c.to_bits() + 1) - c;
        let (low, high) = (against(c, -0.5 * below), against(c, 0.5 * above));
        let even = c.to_bits() % 2 == 0;
        if low < 0 || (low == 0 && !even) {
            c = f64::from_bits(c.to_bits() - 1);
        } else if high > 0 || (high == 0 && !even) {
            c = f64::from_bits(c.to_bits() + 1);
        } else {
            return c;
        }
    }
    panic!("no correctly rounded square root found near {c}");
}

/// DEL-05-02's arithmetic, bit for bit, on the operands the product
/// published: a result-state subtraction RN(a - b); a mechanics combination
/// the exact sum of the exact products rounded once; an absolute sum
/// RN(Σ |f·x|); an SRSS RN(√Σ RN(f·x)²). Every row of the combination of
/// the kinds the package binds (motions, reactions, member end forces and
/// moments, the constant-effort force) is checked, every operand publishes
/// it, and there are at least as many as the package's rows of the item.
fn assert_combination_arithmetic(out: &Value, item: &Value, what: &str) {
    use open_pipe_stress_frame_kernel::exact_sum::ExactAccumulator;
    let id = item["id"].as_str().unwrap();
    let operands: Vec<(At<'_>, f64)> = match item["basis"].as_str().unwrap() {
        "result_state_subtraction" => vec![(case_at(item["minuend"].as_str().unwrap()), 1.0), (case_at(item["subtrahend"].as_str().unwrap()), -1.0)],
        _ => item["operands"].as_array().unwrap().iter().zip(item["factors"].as_array().unwrap()).map(|(operand, factor)| {
            let operand = operand.as_str().unwrap();
            (if operand.contains("-d") { combination_at(operand) } else { case_at(operand) }, factor.as_f64().unwrap())
        }).collect(),
    };
    let rows = out["results"].as_array().unwrap();
    let bound = |kind: &str| kind.starts_with("global_nodal_displacement_") || kind.starts_with("global_nodal_rotation_")
        || ["support_reaction_component_v2", "element_global_end_force_v1", "element_global_end_moment_v1", "constant_effort_support_applied_load"].contains(&kind);
    let of = |at: At<'_>| rows.iter().filter(|r| r["basis_ref"]["ref_type"] == at.ref_type && r["basis_ref"]["ref_id"] == at.ref_id && bound(r["kind"].as_str().unwrap()))
        .map(|r| (row_key(r), r["value"].as_f64().unwrap())).collect::<BTreeMap<_, _>>();
    let combined = of(combination_at(id));
    let operand_rows = operands.iter().map(|(at, factor)| (of(*at), *factor)).collect::<Vec<_>>();
    assert!(combined.len() >= item["rows"].as_array().unwrap().len(), "{what}: {} rows checked", combined.len());
    for (key, published) in &combined {
        let terms = operand_rows.iter().map(|(rows, factor)| (*factor, *rows.get(key).unwrap_or_else(|| panic!("{what}: an operand publishes no {key}")))).collect::<Vec<_>>();
        let expected = match item["basis"].as_str().unwrap() {
            "result_state_subtraction" => terms[0].1 - terms[1].1,
            "mechanics" => {
                let mut sum = ExactAccumulator::new();
                terms.iter().for_each(|(f, x)| sum.add_product(*f, *x).unwrap());
                sum.round().unwrap()
            }
            "absolute_sum" => {
                let mut sum = ExactAccumulator::new();
                terms.iter().for_each(|(f, x)| sum.add_product(f.abs(), x.abs()).unwrap());
                sum.round().unwrap()
            }
            "srss" => exact_srss(&terms.iter().map(|(f, x)| f * x).collect::<Vec<_>>()),
            other => panic!("{what}: basis {other}"),
        };
        assert_eq!(published.to_bits(), expected.to_bits(), "{what} {key}: published {published:e}, DEL-05-02's arithmetic on the published operands {expected:e}");
    }
}
