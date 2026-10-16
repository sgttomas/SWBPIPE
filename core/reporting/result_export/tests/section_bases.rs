//! T4-U6/U8 (DEL-05-03 `section-bases-and-timoshenko.md` §1-§3.1, §5, §8): the
//! product readers read the D-5 section bases and the D-6 beam theory, and
//! refuse each reader mutant.
//!
//! The documents are synthetic: a committed pressure-1 fixture (or arc corpus
//! envelope) is shaped in memory to §2.5's interface (both stress families,
//! basis tokens and id segments, the D-5 `pipe_sections`, per-basis maximum
//! records and headlines) with coincident bases, then given distinct bases (a
//! nonzero corrosion allowance) or a Timoshenko selection, then mutated. No
//! committed fixture is read as current §2.5 output, and none is written. The
//! Python verifiers' twins of these mutations are
//! `tests/test_section_bases_readers.py`.
use open_pipe_stress_result_export::{physics_source, section_basis as sb, semantic_contract as s};
use serde_json::{json, Value};
use std::collections::HashMap;

const UNDERSCORE_IDS: [&str; 2] = ["result:pressure-exact:", "result:elastic-maximum:"];

fn suffix(id: &str, basis: &str) -> String {
    if UNDERSCORE_IDS.iter().any(|p| id.starts_with(p)) {
        format!("{id}:{basis}")
    } else {
        format!("{id}:{}", basis.replace('_', "-"))
    }
}
/// The D-5 entry of a committed single-section entry, its effective wall taken
/// as the nominal wall (c_m = c_a = 0).
fn section(old: &Value) -> Value {
    let annulus = json!({"wall_m": old["effective_wall_thickness_m"], "ri_m": old["ri_m"], "As_m2": old["As_m2"],
        "I_m4": old["I_m4"], "J_m4": old["J_m4"], "Z_m3": old["Z_m3"]});
    json!({"pipe_id": old["pipe_id"], "geometry_basis": sb::GEOMETRY_BASIS, "outside_diameter_m": old["outside_diameter_m"],
        "ro_m": old["ro_m"], "wall_thickness_m": old["effective_wall_thickness_m"], "mill_tolerance_m": 0.0,
        "corrosion_allowance_m": 0.0, "nominal_section": annulus.clone(), "reduced_section": annulus,
        "pressure_bore": {"ri_m": old["ri_m"], "Ai_m2": old["Ai_m2"]}, "beam_theory": {"model": sb::EULER_BERNOULLI}})
}
/// §2.5 with coincident bases.
fn shape(envelope: &Value) -> Value {
    let mut doc = envelope.clone();
    let mut renamed: HashMap<String, Vec<String>> = HashMap::new();
    let mut rows = Vec::new();
    for row in doc["results"].as_array().unwrap() {
        if !sb::is_stress_kind(row["kind"].as_str().unwrap()) {
            rows.push(row.clone());
            continue;
        }
        let mut ids = Vec::new();
        for basis in sb::BASES {
            let mut twin = row.clone();
            let id = suffix(row["id"].as_str().unwrap(), basis);
            twin["id"] = json!(id);
            twin["metadata"]["basis"] = json!(format!("{}; section_basis={basis}", row["metadata"]["basis"].as_str().unwrap()));
            ids.push(id);
            rows.push(twin);
        }
        renamed.insert(row["id"].as_str().unwrap().to_string(), ids);
    }
    doc["results"] = json!(rows);
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        case["pipe_sections"] = json!(case["pipe_sections"].as_array().unwrap().iter().map(section).collect::<Vec<_>>());
        let old = case["stress_maximum_coverage"].clone();
        case["stress_maximum_coverage"] = json!(sb::BASES.map(|b| json!({"section_basis": b, "complete": old["complete"],
            "unavailable_pipe_ids": old["unavailable_pipe_ids"]})));
        let mut extrema = Vec::new();
        for ex in case["pipe_stress_extrema"].as_array().unwrap() {
            for basis in sb::BASES {
                let mut twin = ex.clone();
                twin["section_basis"] = json!(basis);
                twin["result_id"] = json!(suffix(ex["result_id"].as_str().unwrap(), basis));
                extrema.push(twin);
            }
        }
        case["pipe_stress_extrema"] = json!(extrema);
    }
    for region in doc["contract_evidence"]["pressure"].as_array_mut().unwrap() {
        let geometry: Vec<Value> = region["geometry"].as_array().unwrap().iter().map(|g| {
            let mut entry = section(g);
            entry["traversal_forward"] = g["traversal_forward"].clone();
            for key in ["member_kind", "arc_end_tangents_global"] {
                if let Some(v) = g.get(key) { entry[key] = v.clone(); }
            }
            entry
        }).collect();
        region["geometry"] = json!(geometry);
        let ids: Vec<Value> = region["result_ids"].as_array().unwrap().iter().flat_map(|id| {
            renamed.get(id.as_str().unwrap()).map_or_else(|| vec![id.clone()], |v| v.iter().map(|x| json!(x)).collect())
        }).collect();
        region["result_ids"] = json!(ids);
    }
    let old = doc["summary"].clone();
    let mut summary = serde_json::Map::new();
    for (key, value) in old.as_object().unwrap() {
        if key == "max_open_formula_stress" {
            summary.insert(key.clone(), Value::Null);
            for basis in sb::BASES {
                let mut headline = value.clone();
                if !headline.is_null() {
                    headline["result_ref"] = json!(suffix(value["result_ref"].as_str().unwrap(), basis));
                }
                summary.insert(format!("{key}_{basis}"), headline);
            }
        } else {
            summary.insert(key.clone(), value.clone());
        }
    }
    doc["summary"] = Value::Object(summary);
    doc
}
fn f(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn bore_bits(a: f64, b: f64) -> (String, String) {
    let hi = a - b;
    let b_virtual = a - hi;
    let a_virtual = hi + b_virtual;
    let lo = (a - a_virtual) + (b_virtual - b);
    (format!("{:016x}", hi.to_bits()), format!("{:016x}", if lo == 0.0 { 0 } else { lo.to_bits() }))
}
fn divisor(kind: &str) -> Option<&'static str> {
    match kind {
        "element_local_axial_normal_stress" | "pipe_axial_membrane_stress_v2" => Some("As_m2"),
        "element_local_bending_normal_stress_y" | "element_local_bending_normal_stress_z" => Some("Z_m3"),
        "element_local_torsional_shear_stress" => Some("J_m4"),
        _ => None,
    }
}
/// A nonzero corrosion allowance on every pipe: the reduced section, the
/// pressure bore, the reduced stress rows, the reduced Lame rows and the bore bits.
fn distinct(doc: &Value, allowance: f64) -> Value {
    let mut doc = doc.clone();
    let reduce = |s: &mut Value| {
        let (od, t) = (f(&s["outside_diameter_m"]), f(&s["wall_thickness_m"]));
        let t_r = sb::reduced_wall(t, f(&s["mill_tolerance_m"]), allowance);
        let (ro, ri) = (od * 0.5, od * 0.5 - t_r);
        let area = (std::f64::consts::PI * t_r) * (od - t_r);
        let second = (area * ((ro * ro) + (ri * ri))) * 0.25;
        s["corrosion_allowance_m"] = json!(allowance);
        s["reduced_section"] = json!({"wall_m": t_r, "ri_m": ri, "As_m2": area, "I_m4": second, "J_m4": second * 2.0, "Z_m3": second / ro});
        s["pressure_bore"] = json!({"ri_m": ri, "Ai_m2": (std::f64::consts::PI * ri) * ri});
    };
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        for s in case["pipe_sections"].as_array_mut().unwrap() { reduce(s); }
    }
    for region in doc["contract_evidence"]["pressure"].as_array_mut().unwrap() {
        for g in region["geometry"].as_array_mut().unwrap() { reduce(g); }
    }
    let evidence = doc["contract_evidence"].clone();
    let sections: HashMap<(String, String), Value> = evidence["exact_cases"].as_array().unwrap().iter().flat_map(|c| {
        c["pipe_sections"].as_array().unwrap().iter().map(|s| ((c["load_case_id"].as_str().unwrap().to_string(), s["pipe_id"].as_str().unwrap().to_string()), s.clone())).collect::<Vec<_>>()
    }).collect();
    let pressures: HashMap<(String, String), f64> = evidence["pressure"].as_array().unwrap().iter().flat_map(|r| {
        r["member_pipe_ids"].as_array().unwrap().iter().map(|m| ((r["load_case_id"].as_str().unwrap().to_string(), m.as_str().unwrap().to_string()), f(&r["p_pa"]))).collect::<Vec<_>>()
    }).collect();
    let by_id: HashMap<String, Value> = doc["results"].as_array().unwrap().iter().map(|r| (r["id"].as_str().unwrap().to_string(), r.clone())).collect();
    for row in doc["results"].as_array_mut().unwrap() {
        let kind = row["kind"].as_str().unwrap().to_string();
        let id = row["id"].as_str().unwrap().to_string();
        if !sb::is_stress_kind(&kind) || sb::id_basis(&id) != Some(sb::REDUCED) { continue; }
        let key = (row["basis_ref"]["ref_id"].as_str().unwrap().to_string(), row["entity_ref"].as_str().unwrap().to_string());
        let s = &sections[&key];
        let nominal = f(&by_id[&sb::twin_id(&id).unwrap()]["value"]);
        if let Some(leaf) = divisor(&kind) {
            row["value"] = json!(nominal * f(&s["nominal_section"][leaf]) / f(&s["reduced_section"][leaf]));
        } else if kind == "pipe_lame_hoop_stress_v2" {
            let p = pressures[&key];
            let ri = f(&s["reduced_section"]["ri_m"]);
            let outer = 2.0 * (p * ((std::f64::consts::PI * ri) * ri)) / f(&s["reduced_section"]["As_m2"]);
            row["value"] = json!(if row["metadata"]["component"] == "lame_outer_hoop_stress" { outer } else { outer + p });
        }
    }
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        let cid = case["load_case_id"].as_str().unwrap().to_string();
        for group in case["pressure_rhs_assembly"]["groups"].as_array_mut().unwrap() {
            if group.get("source_inner_radius_hi_bits").is_some() {
                let s = &sections[&(cid.clone(), group["terms"][0]["pipe_id"].as_str().unwrap().to_string())];
                let (hi, lo) = bore_bits(f(&s["outside_diameter_m"]) * 0.5, f(&s["reduced_section"]["wall_m"]));
                group["source_inner_radius_hi_bits"] = json!(hi);
                group["source_inner_radius_lo_bits"] = json!(lo);
            }
        }
    }
    doc
}
/// timoshenko_annulus_energy_v1 on every straight member (arcs stay Euler-Bernoulli),
/// kappa from the named radius basis, A_s on the named area basis, or a given factor.
fn timoshenko(doc: &Value, radius: &str, area: &str, factor: Option<f64>) -> Value {
    let mut doc = doc.clone();
    let arcs: Vec<(Value, Value)> = doc["contract_evidence"]["pressure"].as_array().unwrap().iter().flat_map(|r| {
        r["geometry"].as_array().unwrap().iter().filter(|g| g.get("member_kind").is_some()).map(|g| (r["load_case_id"].clone(), g["pipe_id"].clone())).collect::<Vec<_>>()
    }).collect();
    let mut beams: HashMap<(String, String), Value> = HashMap::new();
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        let nus: HashMap<String, f64> = case["pipe_materials"].as_array().unwrap().iter().map(|m| (m["pipe_id"].as_str().unwrap().to_string(), f(&m["nu"]))).collect();
        let cid = case["load_case_id"].clone();
        for s in case["pipe_sections"].as_array_mut().unwrap() {
            if arcs.contains(&(cid.clone(), s["pipe_id"].clone())) { continue; }
            let kappa = factor.unwrap_or_else(|| sb::timoshenko_shear_factor(f(&s[radius]["ri_m"]), f(&s["ro_m"]), nus[s["pipe_id"].as_str().unwrap()]));
            s["beam_theory"] = json!({"model": sb::TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": kappa * f(&s[area]["As_m2"])});
            beams.insert((cid.as_str().unwrap().to_string(), s["pipe_id"].as_str().unwrap().to_string()), s["beam_theory"].clone());
        }
    }
    for region in doc["contract_evidence"]["pressure"].as_array_mut().unwrap() {
        let cid = region["load_case_id"].as_str().unwrap().to_string();
        for g in region["geometry"].as_array_mut().unwrap() {
            if let Some(beam) = beams.get(&(cid.clone(), g["pipe_id"].as_str().unwrap().to_string())) {
                g["beam_theory"] = beam.clone();
            }
        }
    }
    doc
}
fn base() -> Value {
    shape(&serde_json::from_str(include_str!("../../../../fixtures/results/pressure_connected_mechanics_dense.json")).unwrap())
}
fn arc() -> Value {
    let corpus: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json")).unwrap();
    shape(&corpus["cases"][0]["envelope"])
}
fn refused(doc: &Value) -> String {
    s::validate_pressure_evidence(doc).expect_err("the mutant is refused")
}
fn find<'a>(doc: &'a mut Value, kind: &str, ends: &str) -> &'a mut Value {
    doc["results"].as_array_mut().unwrap().iter_mut()
        .find(|r| r["kind"] == kind && r["id"].as_str().unwrap().ends_with(ends) && r["value"].as_f64() != Some(0.0))
        .unwrap()
}

#[test]
fn the_shaped_interface_is_read_on_coincident_and_distinct_bases_under_both_shear_models() {
    let n = sb::NOMINAL;
    for doc in [base(), distinct(&base(), 0.001), timoshenko(&base(), n, n, None),
        timoshenko(&distinct(&base(), 0.001), n, n, None), arc(), timoshenko(&arc(), n, n, None), distinct(&arc(), 0.0005)] {
        assert_eq!(s::validate_pressure_evidence(&doc), Ok(()));
    }
}

#[test]
fn pressure_1_reader_refuses_cross_basis_stress() {
    // §8: one basis for both families (reduced rows copied from nominal) on distinct bases.
    let mut doc = distinct(&base(), 0.001);
    let id = find(&mut doc, "element_local_bending_normal_stress_y", ":reduced-section")["id"].as_str().unwrap().to_string();
    let nominal = doc["results"].as_array().unwrap().iter().find(|r| r["id"] == sb::twin_id(&id).unwrap().as_str()).unwrap()["value"].clone();
    find(&mut doc, "element_local_bending_normal_stress_y", ":reduced-section")["value"] = nominal;
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_BASIS_SECTION");
    // The nominal-basis Lame hoop evaluated at the pressure bore.
    let mut doc = distinct(&base(), 0.001);
    let id = find(&mut doc, "pipe_lame_hoop_stress_v2", ":reduced_section")["id"].as_str().unwrap().to_string();
    let reduced = doc["results"].as_array().unwrap().iter().find(|r| r["id"] == id.as_str()).unwrap()["value"].clone();
    let twin = sb::twin_id(&id).unwrap();
    doc["results"].as_array_mut().unwrap().iter_mut().find(|r| r["id"] == twin.as_str()).unwrap()["value"] = reduced;
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_BASIS_SECTION");
    // A reduced token on a nominal id.
    let mut doc = base();
    let row = find(&mut doc, "element_local_torsional_shear_stress", ":nominal-section");
    row["metadata"]["basis"] = json!(row["metadata"]["basis"].as_str().unwrap().replace("nominal_section", "reduced_section"));
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_SECTION_BASIS");
}

#[test]
fn pressure_1_reader_refuses_cross_basis_headline() {
    let mut doc = base();
    let reduced = &mut doc["summary"]["max_open_formula_stress_reduced_section"];
    reduced["result_ref"] = json!(reduced["result_ref"].as_str().unwrap().replace(":reduced_section", ":nominal_section"));
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_HEADLINE_RESULT_BINDING");
    let mut doc = base();
    doc["summary"]["max_open_formula_stress"] = doc["summary"]["max_open_formula_stress_nominal_section"].clone();
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_HEADLINE_BASES");
    let mut doc = base();
    doc["summary"].as_object_mut().unwrap().remove("max_open_formula_stress_reduced_section");
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_HEADLINE_BASES");
}

#[test]
fn pressure_1_reader_refuses_a_non_maximal_basis_headline() {
    let mut doc = base();
    let mut maxima: Vec<Value> = doc["results"].as_array().unwrap().iter()
        .filter(|r| r["kind"] == "pipe_elastic_normal_stress_maximum_v2" && r["id"].as_str().unwrap().ends_with(":reduced_section"))
        .cloned().collect();
    maxima.sort_by(|a, b| f(&a["value"]).total_cmp(&f(&b["value"])));
    assert!(maxima.len() == 2 && f(&maxima[0]["value"]) < f(&maxima[1]["value"]));
    doc["summary"]["max_open_formula_stress_reduced_section"] = json!({"value": maxima[0]["value"], "unit": "Pa",
        "location_ref": maxima[0]["entity_ref"], "result_ref": maxima[0]["id"]});
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_HEADLINE_MAXIMUM");
}

#[test]
fn pressure_1_reader_refuses_a_missing_reduced_twin() {
    let mut doc = base();
    let gone = find(&mut doc, "element_local_torsional_shear_stress", ":reduced-section")["id"].clone();
    doc["results"].as_array_mut().unwrap().retain(|r| r["id"] != gone);
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_BASIS_TWIN");
    let mut doc = base();
    let gone = find(&mut doc, "pipe_axial_membrane_stress_v2", ":reduced_section")["id"].clone();
    doc["results"].as_array_mut().unwrap().retain(|r| r["id"] != gone);
    for region in doc["contract_evidence"]["pressure"].as_array_mut().unwrap() {
        region["result_ids"].as_array_mut().unwrap().retain(|id| *id != gone);
    }
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_BASIS_TWIN");
}

#[test]
fn pressure_1_reader_refuses_a_wrong_kappa_a_cowper_kappa_or_kappa_on_the_reduced_section() {
    let (n, r) = (sb::NOMINAL, sb::REDUCED);
    let distinct = distinct(&base(), 0.001);
    let good = timoshenko(&distinct, n, n, None);
    assert_eq!(s::validate_pressure_evidence(&good), Ok(()));
    let section = &good["contract_evidence"]["exact_cases"][0]["pipe_sections"][0];
    let kappa = f(&section["beam_theory"]["shear_factor"]);
    let nu = f(&good["contract_evidence"]["exact_cases"][0]["pipe_materials"][0]["nu"]);
    let m = f(&section["nominal_section"]["ri_m"]) / f(&section["ro_m"]);
    let m2 = (1.0 + m * m) * (1.0 + m * m);
    let cowper = 6.0 * (1.0 + nu) * m2 / ((7.0 + 6.0 * nu) * m2 + (20.0 + 12.0 * nu) * m * m);
    for doc in [timoshenko(&distinct, n, n, Some(kappa.next_up())), timoshenko(&distinct, n, n, Some(cowper)),
        timoshenko(&distinct, n, n, Some(5.0 / 6.0)), timoshenko(&distinct, r, n, None), timoshenko(&distinct, n, r, None)] {
        assert_eq!(refused(&doc), "SOURCE_PHYSICS_SHEAR_FACTOR");
    }
    // One straight shear model per document.
    let mut mixed = good.clone();
    mixed["contract_evidence"]["exact_cases"][1]["pipe_sections"][0]["beam_theory"] = json!({"model": sb::EULER_BERNOULLI});
    assert_eq!(refused(&mixed), "SOURCE_PHYSICS_SHEAR_MODEL_CONSISTENCY");
}

#[test]
fn pressure_1_reader_refuses_an_arc_claiming_timoshenko() {
    let doc = timoshenko(&arc(), sb::NOMINAL, sb::NOMINAL, None);
    let (region_index, geometry_index, pipe) = doc["contract_evidence"]["pressure"].as_array().unwrap().iter().enumerate()
        .find_map(|(i, r)| r["geometry"].as_array().unwrap().iter().position(|g| g.get("member_kind").is_some())
            .map(|j| (i, j, r["geometry"][j]["pipe_id"].clone())))
        .unwrap();
    let bend = &doc["contract_evidence"]["pressure"][region_index]["geometry"][geometry_index];
    let nu = doc["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
        .flat_map(|c| c["pipe_materials"].as_array().unwrap()).find(|m| m["pipe_id"] == pipe).unwrap()["nu"].as_f64().unwrap();
    let kappa = sb::timoshenko_shear_factor(f(&bend["nominal_section"]["ri_m"]), f(&bend["ro_m"]), nu);
    let claim = json!({"model": sb::TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": kappa * f(&bend["nominal_section"]["As_m2"])});
    // The arc's region geometry record claims Timoshenko.
    let mut claimed = doc.clone();
    claimed["contract_evidence"]["pressure"][region_index]["geometry"][geometry_index]["beam_theory"] = claim.clone();
    assert_eq!(refused(&claimed), "SOURCE_PHYSICS_ARC_BEAM_THEORY");
    // The arc's case section claims Timoshenko.
    let mut claimed = doc.clone();
    for case in claimed["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        for s in case["pipe_sections"].as_array_mut().unwrap() {
            if s["pipe_id"] == pipe { s["beam_theory"] = claim.clone(); }
        }
    }
    assert_eq!(refused(&claimed), "SOURCE_PHYSICS_ARC_BEAM_THEORY");
}

#[test]
fn pressure_1_reader_refuses_a_basis_less_thin_wall_row_on_an_exact_envelope() {
    let mut doc = base();
    let case = doc["contract_evidence"]["exact_cases"][0].clone();
    doc["results"].as_array_mut().unwrap().push(json!({"id": "result:thin-wall:hoop", "kind": "pipe_section_pressure_hoop_stress",
        "value": 1.0, "unit": "MPa", "entity_ref": case["pipe_sections"][0]["pipe_id"],
        "basis_ref": {"ref_type": "load_case", "ref_id": case["load_case_id"]},
        "metadata": {"component": "pressure_hoop_stress", "coordinate_system": "pipe_section", "location": "midspan",
            "basis": "recovered_from_open_mechanics_stress_components", "sign_convention": "thin wall"}}));
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_SECTION_BASIS");
    // A bare recovery token on a stress row names no basis and matches no signature.
    let mut doc = base();
    let row = find(&mut doc, "element_local_bending_normal_stress_z", ":nominal-section");
    row["metadata"]["basis"] = json!(row["metadata"]["basis"].as_str().unwrap().split("; section_basis=").next().unwrap());
    assert_eq!(refused(&doc), "SOURCE_BASIS_CONTRADICTION: element_local_bending_normal_stress_z");
}

#[test]
fn the_d5_geometry_is_rederived_in_its_fixed_order() {
    type Edit = fn(&mut Value);
    let cases: [(Edit, &str); 6] = [
        (|s| { let w = f(&s["reduced_section"]["wall_m"]); s["reduced_section"]["wall_m"] = json!(w * (1.0 + 2f64.powi(-50))); }, "SOURCE_PHYSICS_GEOMETRY_SOURCE_BINDING"),
        (|s| { s["nominal_section"]["ri_m"] = s["pressure_bore"]["ri_m"].clone(); }, "SOURCE_PHYSICS_GEOMETRY_SOURCE_BINDING"),
        (|s| { s["pressure_bore"]["ri_m"] = s["nominal_section"]["ri_m"].clone(); }, "SOURCE_PHYSICS_SECTION_BASES"),
        (|s| { s["corrosion_allowance_m"] = s["wall_thickness_m"].clone(); }, "SOURCE_PHYSICS_SECTION_ALLOWANCE"),
        (|s| { s["corrosion_allowance_m"] = json!(-0.001); }, "SOURCE_PHYSICS_GEOMETRY_RANGE"),
        (|s| { s["geometry_basis"] = json!("authored_normalized_od_wall_v1"); }, "SOURCE_PHYSICS_GEOMETRY_BASIS"),
    ];
    for (edit, code) in cases {
        let mut doc = distinct(&base(), 0.001);
        for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            for s in case["pipe_sections"].as_array_mut().unwrap() { edit(s); }
        }
        assert_eq!(refused(&doc), code);
    }
    // Coincident walls with different leaves are one section stated twice.
    let mut doc = base();
    let s = &mut doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0];
    s["reduced_section"]["As_m2"] = json!(f(&s["reduced_section"]["As_m2"]).next_up());
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_SECTION_BASES");
}

#[test]
fn pressure_source_1_refuses_nonzero_mill_tolerance() {
    let request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
    )).unwrap();
    // Any source: the invocation's contract is checked first (`SourceContract::check_invocation`).
    let source = json!({});
    let invocation = json!({"request": request, "solver_mode": "sparse_interactive"});
    let first = physics_source::validate(&source, Some(&invocation)).unwrap_err();
    assert!(!first.contains("ACTUAL_MILL_TOLERANCE"), "{first}");
    for (value, refused) in [(json!({"value": 0.5, "unit": "mm"}), true), (json!({"value": 1, "unit": "N"}), true),
        (json!({"value": "0", "unit": "mm"}), true), (json!({"value": 0, "unit": "mm"}), false), (Value::Null, false)] {
        let mut changed = invocation.clone();
        changed["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = value.clone();
        let outcome = physics_source::validate(&source, Some(&changed)).unwrap_err();
        assert_eq!(outcome == "PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE", refused, "{value}: {outcome}");
    }
}
