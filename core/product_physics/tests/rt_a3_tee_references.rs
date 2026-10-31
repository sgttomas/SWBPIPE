//! RT-A3 increment 1 (DEL-04-07 Design `pressure-contract.md`, "Tees and
//! pressure junctions"): tees and pressure junctions on the exact route
//! against the frozen independent references
//! (`validation/references/rt_a3_tee/`), through PP's public entry in both
//! solver modes.
//!
//! Every reference case is authored as a current `3.0.0/exact_pressure_v3`
//! document from the case's own inputs. A junction is a node where regions
//! end with `transfers_to_wall` at every terminal (D-1); a tee is the branch
//! connection record at its node, naming its header and branch pipes with the
//! user's SIFs. Each published value is compared at
//! |observed − expected| ≤ 1e-9·max(|expected|, zero_scale(family)):
//! nodal motions, support reactions, member end actions in member axes, the
//! wall, effective and per-basis membrane axial values, and the tee's
//! intensified rows on both section bases (absent on every other member
//! end). The mutation control T2-CTRL (one junction cap dropped) is a value
//! the product never publishes: the compensated terminal that would drop it
//! is refused by name, and the closed tee's anchor carries 0.
//!
//! A mismatch here is the product's to fix, never the reference's.

use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode, MODEL_SCHEMA_VERSION};
use serde_json::{json, Value};

const REFERENCES: &str = include_str!("../../../validation/references/rt_a3_tee/references.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const CASE: &str = "case:pressure";
const BRANCH_KIND: &str = "branch_intensified_bending_stress_v1";
const PROVENANCE: &str = "invented_rt_a3_tee_reference";
const BASES: [&str; 2] = ["nominal_section", "reduced_section"];

fn references() -> Value {
    serde_json::from_str(REFERENCES).unwrap()
}

fn value(v: &Value) -> f64 {
    v["value"].as_f64().unwrap_or_else(|| panic!("no value in {v}"))
}

fn quantity(v: f64, unit: &str) -> Value {
    json!({"value": v, "unit": unit})
}

fn support_id(node: &str) -> String {
    format!("support:{}", node.trim_start_matches("node:"))
}

/// The case as a current exact document.
fn document(material: &Value, case: &Value) -> Value {
    let inputs = &case["inputs"];
    let nodes = inputs["nodes"].as_object().unwrap().iter().map(|(id, node)| {
        let p = node["value"].as_array().unwrap();
        json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let pipes = inputs["pipes"].as_array().unwrap().iter().map(|pipe| {
        let id = pipe["id"].as_str().unwrap();
        let section = &inputs["pipe_sections"][id];
        let y = pipe["y_reference"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().parse::<f64>().unwrap()).collect::<Vec<_>>();
        json!({"id": id, "from": pipe["from"], "to": pipe["to"],
            "section": {"outside_diameter": quantity(value(&section["outside_diameter_m"]), "m"),
                "wall_thickness": quantity(value(&section["wall_thickness_m"]), "m"),
                "mill_tolerance": quantity(value(&section["mill_tolerance_m"]), "m"),
                "corrosion_allowance": quantity(value(&section["corrosion_allowance_m"]), "m")},
            "material": "material:rt-a3", "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let supports = inputs["supports"].as_array().unwrap().iter().map(|support| {
        let node = support["node"].as_str().unwrap();
        let dofs = support["dofs"].as_array().unwrap().iter().map(|d| d.as_str().unwrap().to_uppercase()).collect::<Vec<_>>();
        assert_eq!(dofs.len(), 6, "every reference support is an anchor");
        json!({"id": support_id(node), "node": node, "family": "anchor", "restraints": dofs, "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let mut loads = Vec::new();
    for load in inputs["nodal_loads"].as_array().map(Vec::as_slice).unwrap_or(&[]) {
        let node = load["node"].as_str().unwrap();
        for (axis, f) in ["x", "y", "z"].iter().zip(load["force_N"]["value"].as_array().unwrap()) {
            let f = f.as_f64().unwrap();
            if f != 0.0 {
                loads.push(json!({"id": format!("force:{axis}"), "category": "concentrated_force", "target": {"type": "node", "node": node},
                    "direction": format!("global_{axis}"), "magnitude": quantity(f, "N"), "dimension": "force", "provenance": PROVENANCE}));
            }
        }
        assert!(load["moment_N_m"]["value"].as_array().unwrap().iter().all(|m| m.as_f64() == Some(0.0)));
    }
    let regions = inputs["pressure_regions"].as_array().unwrap().iter().map(|region| {
        json!({"id": region["id"], "member_pipe_ids": region["pipes"],
            "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": quantity(value(&region["pressure_Pa"]), "Pa"),
            "terminals": region["terminals"].as_array().unwrap().iter().map(|t| json!({"node_ref": t["node"],
                "closure_transfer": t["closure_transfer"], "provenance": PROVENANCE})).collect::<Vec<_>>(),
            "provenance": PROVENANCE})
    }).collect::<Vec<_>>();
    let components = match &inputs["tee"] {
        Value::Null => json!([]),
        tee => json!([{"id": "component:tee", "kind": "tee", "node": tee["node"],
            "geometry": {"branch_header_pipe_ref": tee["header_pipe_ref"], "branch_branch_pipe_ref": tee["branch_pipe_ref"],
                "branch_geometry_source_reference": "invented"},
            "modifiers": {"branch_header_sif_user_value": quantity(value(&tee["i_header"]), "none"),
                "branch_branch_sif_user_value": quantity(value(&tee["i_branch"]), "none"),
                "flexibility_factor_user_value": quantity(1.0, "none"),
                "source_reference": "invented_rt_a3_tee"},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only", "rule_check_consumption": "user_rule_pack_inputs_only"},
            "provenance": PROVENANCE}]),
    };
    let e = material["elastic_modulus_Pa"].as_str().unwrap().parse::<f64>().unwrap();
    assert_eq!(material["poisson_ratio"], "3/10");
    json!({"model": {"schema_version": MODEL_SCHEMA_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:rt-a3-tee", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes,
        "materials": [{"id": "material:rt-a3", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "elastic_modulus": quantity(e, "Pa"), "poisson_ratio": quantity(0.3, "1"), "provenance": PROVENANCE}],
        "supports": supports, "components": components,
        "load_cases": [{"id": CASE, "primitive_loads": loads, "pressure_regions": regions, "provenance": PROVENANCE}],
        "combinations": []},
        "materials": []})
}

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn basis_of(row: &Value) -> &str {
    let basis = row["metadata"]["basis"].as_str().unwrap_or("");
    basis.split("; section_basis=").nth(1).unwrap_or("")
}

/// The one published case row matching.
fn row(envelope: &Value, kind: &str, entity: &str, location: Option<&str>, component: Option<&str>, basis: &str) -> f64 {
    let found = envelope["results"].as_array().unwrap().iter().filter(|r| {
        r["kind"] == kind && r["entity_ref"] == entity && basis_of(r) == basis
            && location.is_none_or(|l| r["metadata"]["location"] == l)
            && component.is_none_or(|c| r["metadata"]["component"] == c)
            && r["basis_ref"]["ref_id"] == CASE
    }).collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity} {location:?} {component:?} [{basis}]: {} rows", found.len());
    found[0]["value"].as_f64().unwrap()
}

fn close(observed: f64, expected: f64, scale: f64, what: &str) {
    let tolerance = 1e-9 * expected.abs().max(scale);
    assert!((observed - expected).abs() <= tolerance, "{what}: observed {observed:e}, expected {expected:e} (tolerance {tolerance:e})");
}

/// The member end of `pipe` at `node` in the case's inputs.
fn end_of(case: &Value, pipe: &str, node: &str) -> &'static str {
    let p = case["inputs"]["pipes"].as_array().unwrap().iter().find(|p| p["id"] == pipe).unwrap();
    if p["from"] == node { "end_i" } else { assert_eq!(p["to"], node); "end_j" }
}

fn check_case(case: &Value, envelope: &Value, label: &str) {
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {:#?}", envelope["diagnostics"]);
    let blocking = envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").count();
    assert_eq!(blocking, 0, "{label}");
    let scale = |family: &str| case["zero_scale"][family]["value"].as_f64().unwrap_or(0.0);
    let mut tee_rows = 0;
    for r in case["rows"].as_array().unwrap() {
        let q = r["quantity"].as_str().unwrap();
        let entity = r["entity"].as_str().unwrap();
        let component = r["component"].as_str().unwrap_or("");
        let expected = r["value"].as_f64().unwrap();
        let s = scale(r["family"].as_str().unwrap());
        let what = format!("{label} {q} {entity} {component} {}", r["location"].as_str().or(r["basis"].as_str()).unwrap_or(""));
        match q {
            "nodal_motion" => {
                let (kind, factor) = match component {
                    "ux" => ("global_nodal_displacement_x", 1000.0),
                    "uy" => ("global_nodal_displacement_y", 1000.0),
                    "uz" => ("global_nodal_displacement_z", 1000.0),
                    "rx" => ("global_nodal_rotation_x", 1.0),
                    "ry" => ("global_nodal_rotation_y", 1.0),
                    _ => ("global_nodal_rotation_z", 1.0),
                };
                close(row(envelope, kind, entity, None, None, ""), expected * factor, s * factor, &what);
            }
            "support_reaction" => {
                let observed = row(envelope, "support_reaction_component_v2", &support_id(entity), None, Some(component), "");
                close(observed, expected, s, &what);
            }
            "member_end_action" => {
                let end = end_of(case, entity, r["location"].as_str().unwrap());
                let observed = match component {
                    "Fx" => row(envelope, "pipe_wall_endpoint_action_v2", entity, Some(end), Some("wall_axial_end_action"), ""),
                    "Fy" => row(envelope, "element_local_shear_force_y", entity, Some(end), None, ""),
                    "Fz" => row(envelope, "element_local_shear_force_z", entity, Some(end), None, ""),
                    "Mx" => row(envelope, "element_local_torsional_moment", entity, Some(end), None, ""),
                    "My" => row(envelope, "element_local_bending_moment_y", entity, Some(end), None, ""),
                    _ => row(envelope, "element_local_bending_moment_z", entity, Some(end), None, ""),
                };
                close(observed, expected, s, &what);
            }
            "wall_axial_force" => close(row(envelope, "pipe_wall_axial_force_v2", entity, Some("end_i"), None, ""), expected, s, &what),
            "effective_axial_force" => close(row(envelope, "pipe_effective_axial_force_v2", entity, Some("end_i"), None, ""), expected, s, &what),
            "mechanical_axial_force" => {} // N_w - 2*nu*p*A_i: covered by the wall force
            "axial_membrane_stress" => {
                let basis = r["basis"].as_str().unwrap();
                close(row(envelope, "pipe_axial_membrane_stress_v2", entity, Some("end_i"), None, basis), expected, s, &what);
            }
            "tee_intensified_bending_stress" => {
                let end = end_of(case, entity, r["location"].as_str().unwrap());
                let basis = r["basis"].as_str().unwrap();
                close(row(envelope, BRANCH_KIND, entity, Some(end), Some("intensified_bending"), basis), expected, s, &what);
                tee_rows += 1;
            }
            other => panic!("{label}: unbound reference quantity {other}"),
        }
    }
    // The tee's rows are exactly the referenced ends', on both bases.
    let published = envelope["results"].as_array().unwrap().iter().filter(|r| r["kind"] == BRANCH_KIND).count();
    assert_eq!(published, tee_rows, "{label}: branch rows");
    let records = envelope["contract_evidence"]["exact_cases"][0]["intensified_measures"].as_array().map_or(0, Vec::len);
    assert_eq!(records, tee_rows, "{label}: branch records");
}

#[test]
fn every_reference_case_solves_on_the_exact_route_within_its_criterion() {
    let references = references();
    let material = &references["material"];
    let mut checked = 0;
    for case in references["cases"].as_array().unwrap().iter().filter(|c| c["status"] == "case") {
        let document = document(material, case);
        for mode in MODES {
            let envelope = solve(&document, mode);
            check_case(case, &envelope, &format!("{} {mode:?}", case["id"].as_str().unwrap()));
        }
        checked += 1;
    }
    assert_eq!(checked, 10);
}

/// T2-CTRL: dropping pipe:C's cap at T gives -20,250*pi y N at the anchor. The
/// product admits no document that drops it: a compensated terminal at the
/// shared node is refused by name, and with every terminal transferring the
/// anchor carries 0 (T1).
#[test]
fn the_dropped_junction_cap_control_is_never_published() {
    let references = references();
    let control = references["cases"].as_array().unwrap().iter().find(|c| c["id"] == "T2-CTRL").unwrap();
    let wrong = control["rows"].as_array().unwrap().iter()
        .find(|r| r["quantity"] == "support_reaction" && r["component"] == "Fy").unwrap()["value"].as_f64().unwrap();
    assert!(wrong < -6.0e4);
    let t1 = references["cases"].as_array().unwrap().iter().find(|c| c["id"] == "T1").unwrap();
    let mut document = document(&references["material"], t1);
    for mode in MODES {
        let envelope = solve(&document, mode);
        let observed = row(&envelope, "support_reaction_component_v2", "support:T", None, Some("Fy"), "");
        assert!((observed - wrong).abs() > 6.0e4, "{mode:?}");
    }
    let region = document["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap()
        .iter_mut().find(|r| r["id"] == "region:C").unwrap();
    region["terminals"][0]["closure_transfer"] = json!("separately_supported_or_compensated");
    for mode in MODES {
        let envelope = solve(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{mode:?}");
        assert!(envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED"
            && d["severity"] == "blocking"), "{mode:?}");
        assert_eq!(envelope["results"], json!([]), "{mode:?}");
    }
}

/// The tee is admitted, its rows bind bit for bit to the published bending
/// rows of their end and basis (the correctly rounded `norm2`), and the model-level diagnostics name an unreferenced pipe
/// end and an absent SIF, as on the pressure-free route.
#[test]
fn a_tee_publishes_its_rows_from_the_basis_bending_rows_and_names_its_gaps() {
    let references = references();
    let t5 = references["cases"].as_array().unwrap().iter().find(|c| c["id"] == "T5-CA").unwrap();
    let mut document = document(&references["material"], t5);
    for mode in MODES {
        let envelope = solve(&document, mode);
        assert!(!envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "EXACT_PRESSURE_FAMILY_NOT_ADMITTED"));
        for (pipe, sif) in [("pipe:A", 2.1), ("pipe:C", 1.6)] {
            for basis in BASES {
                let y = row(&envelope, "element_local_bending_normal_stress_y", pipe, Some("end_i"), None, basis);
                let z = row(&envelope, "element_local_bending_normal_stress_z", pipe, Some("end_i"), None, basis);
                let published = row(&envelope, BRANCH_KIND, pipe, Some("end_i"), None, basis);
                assert_eq!(published.to_bits(), (1e6 * open_pipe_stress_correct_norm::norm2(sif * z, sif * y)).to_bits(), "{pipe} {basis} {mode:?}");
            }
        }
        let coverage = envelope["diagnostics"].as_array().unwrap().iter()
            .filter(|d| d["code"] == "COMPONENT_INTENSIFIED_COVERAGE_INCOMPLETE").collect::<Vec<_>>();
        assert_eq!(coverage.len(), 1, "{mode:?}");
        assert_eq!(coverage[0]["affected_refs"], json!(["component:tee", "pipe:B"]));
    }
    // Without the branch SIF: no branch-side row, and the info names it.
    document["model"]["components"][0]["modifiers"].as_object_mut().unwrap().remove("branch_branch_sif_user_value");
    for mode in MODES {
        let envelope = solve(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        let rows = envelope["results"].as_array().unwrap().iter().filter(|r| r["kind"] == BRANCH_KIND).collect::<Vec<_>>();
        assert_eq!(rows.len(), 2, "{mode:?}");
        assert!(rows.iter().all(|r| r["entity_ref"] == "pipe:A"));
        assert!(envelope["diagnostics"].as_array().unwrap().iter().any(|d| d["code"] == "COMPONENT_INTENSIFICATION_SIF_NOT_SUPPLIED"
            && d["message"].as_str().unwrap().contains("branch_branch")), "{mode:?}");
    }
}

fn warnings<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code && d["severity"] == "warning").collect()
}

/// The T1 value of a nodal motion (m or rad) from the frozen reference.
fn t1_motion(t1: &Value, node: &str, component: &str) -> f64 {
    t1["rows"].as_array().unwrap().iter()
        .find(|r| r["quantity"] == "nodal_motion" && r["entity"] == node && r["component"] == component)
        .unwrap()["value"].as_f64().unwrap()
}

/// D-1 (Agent 0, 2026-10-30): a junction whose regions carry different
/// pressures, or with an incident pipe outside every region, is admitted as a
/// closure at the fitting with its own named warning, and the numbers are the
/// closure's. On T1's anchored closed tee each leg is a free capped member, so
/// by linearity in p the branch's free elongation scales with its own
/// pressure (half T1's at 5 MPa) and is 0 when the branch is dry, the header
/// legs keep T1's motions, and the anchor carries 0 in every case.
#[test]
fn a_junction_closure_is_admitted_with_its_named_warning_and_unchanged_numbers() {
    let references = references();
    let t1 = references["cases"].as_array().unwrap().iter().find(|c| c["id"] == "T1").unwrap();
    let base = document(&references["material"], t1);
    let scale = t1["zero_scale"]["translation"]["value"].as_f64().unwrap();
    let check = |envelope: &Value, branch_factor: f64, label: &str| {
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
        for (node, component) in [("node:A", "ux"), ("node:B", "ux"), ("node:C", "uy")] {
            let factor = if node == "node:C" { branch_factor } else { 1.0 };
            let kind = format!("global_nodal_displacement_{}", &component[1..]);
            close(row(envelope, &kind, node, None, None, ""), 1000.0 * factor * t1_motion(t1, node, component), 1000.0 * scale, label);
        }
        for component in ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] {
            close(row(envelope, "support_reaction_component_v2", "support:T", None, Some(component), ""), 0.0, 1e4, label);
        }
    };
    for mode in MODES {
        let envelope = solve(&base, mode);
        assert!(warnings(&envelope, "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER").is_empty(), "{mode:?}");
        assert!(warnings(&envelope, "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS").is_empty(), "{mode:?}");
    }

    // The branch region at 5 MPa, written in MPa: the header's 1e7 Pa and
    // 10 MPa elsewhere would compare equal, 5 MPa does not.
    let mut differ = base.clone();
    for region in differ["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap() {
        region["pressure"] = if region["id"] == "region:C" { quantity(5.0, "MPa") } else { quantity(10.0, "MPa") };
    }
    let mut same_units = differ.clone();
    same_units["model"]["load_cases"][0]["pressure_regions"][1]["pressure"] = quantity(1.0e7, "Pa");
    for mode in MODES {
        let envelope = solve(&differ, mode);
        check(&envelope, 0.5, &format!("differ {mode:?}"));
        let found = warnings(&envelope, "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER");
        assert_eq!(found.len(), 1, "{mode:?}: {found:#?}");
        assert_eq!(found[0]["affected_refs"], json!([CASE, "node:T", "region:A", "region:B", "region:C"]));
        let message = found[0]["message"].as_str().unwrap();
        assert!(message.contains("node node:T") && message.contains("region:C 5 MPa") && message.contains("region:A 10 MPa"), "{message}");
        // Pa and MPa spellings of one pressure compare equal.
        let envelope = solve(&same_units, mode);
        assert_eq!(warnings(&envelope, "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER").len(), 1, "{mode:?}");
        let message = warnings(&envelope, "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER")[0]["message"].as_str().unwrap().to_string();
        assert!(message.contains("region:B 10000000 Pa"), "{message}");
    }
    let mut equal = base.clone();
    equal["model"]["load_cases"][0]["pressure_regions"][0]["pressure"] = quantity(10.0, "MPa");
    for mode in MODES {
        assert!(warnings(&solve(&equal, mode), "EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER").is_empty(), "{mode:?}");
    }

    // The branch dry: A and B still meet at T, and pipe:C belongs to no region.
    let mut dry = base.clone();
    dry["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().retain(|r| r["id"] != "region:C");
    for mode in MODES {
        let envelope = solve(&dry, mode);
        check(&envelope, 0.0, &format!("dry {mode:?}"));
        let found = warnings(&envelope, "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS");
        assert_eq!(found.len(), 1, "{mode:?}: {found:#?}");
        assert_eq!(found[0]["affected_refs"], json!([CASE, "node:T", "pipe:C"]));
        assert!(found[0]["message"].as_str().unwrap().contains("regions region:A, region:B"));
    }
    // One region's closure beside dry pipes is no junction: no warning.
    let mut single = dry.clone();
    single["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().retain(|r| r["id"] != "region:B");
    for mode in MODES {
        assert!(warnings(&solve(&single, mode), "EXACT_PRESSURE_JUNCTION_PIPE_OUTSIDE_REGIONS").is_empty(), "{mode:?}");
    }
}

fn verdict(envelope: &Value, model: &Value) -> Value {
    open_pipe_stress_result_export::reader_verdict::validate("pressure", envelope, Some(model), None)
}

/// Independent review (RT-A3 increment 1): the readers bind a branch row by
/// its record, not by its (member, end, basis) slot, so two tees naming one
/// pipe end publish two readable rows; and the record's source reference is
/// the model's, so a forged one is refused with the model.
#[test]
fn the_readers_admit_two_tees_on_one_end_and_refuse_a_forged_source_reference() {
    let references = references();
    let t5 = references["cases"].as_array().unwrap().iter().find(|c| c["id"] == "T5").unwrap();
    let mut document = document(&references["material"], t5);
    // A second tee at T: header pipe:B, branch pipe:C (shared with the first).
    let mut second = document["model"]["components"][0].clone();
    second["id"] = json!("component:tee-b");
    second["geometry"]["branch_header_pipe_ref"] = json!("pipe:B");
    second["modifiers"]["branch_header_sif_user_value"] = quantity(1.9, "none");
    second["modifiers"]["branch_branch_sif_user_value"] = quantity(1.3, "none");
    document["model"]["components"].as_array_mut().unwrap().push(second);
    for mode in MODES {
        let envelope = solve(&document, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        let on_c = envelope["results"].as_array().unwrap().iter()
            .filter(|r| r["kind"] == BRANCH_KIND && r["entity_ref"] == "pipe:C").count();
        assert_eq!(on_c, 4, "two tees x two bases on pipe:C's end at T ({mode:?})");
        assert_eq!(verdict(&envelope, &document["model"]), json!({"ok": true}), "{mode:?}");
        let mut forged = envelope.clone();
        for record in forged["contract_evidence"]["exact_cases"][0]["intensified_measures"].as_array_mut().unwrap() {
            if record["component_id"] == "component:tee" {
                record["sif_source_reference"] = json!("FORGED_SOURCE");
            }
        }
        assert_ne!(verdict(&forged, &document["model"]), json!({"ok": true}), "{mode:?}");
    }
}
