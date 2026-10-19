//! T4-U6/U8 (DEL-05-03 `section-bases-and-timoshenko.md` §1-§3.1, §5, §8): the
//! product readers read the D-5 section bases and the D-6 beam theory, and
//! refuse each reader mutant.
//!
//! The documents are the regenerated committed pressure-2 fixtures, which are
//! the producer's §2.5 output (both stress families, basis tokens and id
//! segments, the D-5 `pipe_sections` with each realized arc's marker,
//! per-basis maximum records and headlines): `pressure_connected_mechanics_dense`
//! (coincident bases) and the arc corpus's envelopes (their I7 sketches carry
//! a mill tolerance, so their bases differ). Each is then given distinct bases
//! (a nonzero corrosion allowance) or a Timoshenko selection, then mutated; T4-U4b:
//! the section-extreme and transverse shear rows formed from the moved rows are
//! re-formed by the readers' fixed sequences (`reform_u4b`). An arc's certified
//! maximum cannot be re-formed here, so the arc document with an allowance is the
//! producer's own (the arc corpus's directional-SIF case, 0.0005 m on every pipe).
//! No fixture is written. The Python verifiers' twins of these mutations are
//! `tests/test_section_bases_readers.py`.
use open_pipe_stress_correct_norm::norm2;
use open_pipe_stress_result_export::{physics_source, reader_verdict, section_basis as sb, semantic_contract as s};
use serde_json::{json, Value};
use std::collections::HashMap;

/// `envelope` with the realized-arc marker removed from `pipe`'s case sections.
fn unmarked(envelope: &Value, pipe: &str) -> Value {
    let mut doc = envelope.clone();
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        for s in case["pipe_sections"].as_array_mut().unwrap() {
            if s["pipe_id"] == pipe {
                assert_eq!(s.as_object_mut().unwrap().remove(sb::MEMBER_KIND), Some(json!(sb::REALIZED_ARC)), "{pipe} is marked");
            }
        }
    }
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
/// T4-U4b: each straight member's sigma+/- and transverse shear rows re-formed
/// from the published rows of the same station and basis (and the basis annulus)
/// by the readers' fixed sequences, after a helper moved their operands.
fn reform_u4b(mut doc: Value) -> Value {
    let evidence = doc["contract_evidence"].clone();
    let mut sections: HashMap<(String, String), Value> = HashMap::new();
    let mut nus: HashMap<(String, String), f64> = HashMap::new();
    for case in evidence["exact_cases"].as_array().unwrap() {
        let cid = case["load_case_id"].as_str().unwrap().to_string();
        for s in case["pipe_sections"].as_array().unwrap() {
            sections.insert((cid.clone(), s["pipe_id"].as_str().unwrap().to_string()), s.clone());
        }
        for m in case["pipe_materials"].as_array().unwrap() {
            nus.insert((cid.clone(), m["pipe_id"].as_str().unwrap().to_string()), f(&m["nu"]));
        }
    }
    let rows = doc["results"].as_array().unwrap().clone();
    let basis_of = |r: &Value| r["metadata"]["basis"].as_str().and_then(|b| b.rsplit_once("; section_basis=")).map(|(_, b)| b.to_string());
    let lookup = |case: &str, pipe: &str, kind: &str, location: &str, basis: Option<&str>| -> Option<f64> {
        rows.iter()
            .find(|r| r["basis_ref"]["ref_id"] == case && r["entity_ref"] == pipe && r["kind"] == kind && r["metadata"]["location"] == location
                && basis.is_none_or(|b| basis_of(r).as_deref() == Some(b)))
            .map(|r| f(&r["value"]))
    };
    for row in doc["results"].as_array_mut().unwrap() {
        let kind = row["kind"].as_str().unwrap();
        if kind != "element_local_normal_stress_extreme" && kind != "element_local_transverse_shear_stress_maximum" { continue; }
        let (case, pipe) = (row["basis_ref"]["ref_id"].as_str().unwrap().to_string(), row["entity_ref"].as_str().unwrap().to_string());
        let section = &sections[&(case.clone(), pipe.clone())];
        if section.get(sb::MEMBER_KIND).is_some() { continue; }
        let location = row["metadata"]["location"].as_str().unwrap().to_string();
        let basis = basis_of(row).unwrap();
        let at = |kind: &str, basis: Option<&str>| lookup(&case, &pipe, kind, &location, basis);
        let value = if kind == "element_local_normal_stress_extreme" {
            let y = at("element_local_bending_normal_stress_y", Some(&basis)).unwrap();
            let z = at("element_local_bending_normal_stress_z", Some(&basis)).unwrap();
            let n = at("element_local_axial_normal_stress", Some(&basis))
                .unwrap_or_else(|| at("pipe_axial_membrane_stress_v2", Some(&basis)).unwrap() / 1e6);
            let h = norm2(y, z);
            if row["metadata"]["component"] == "sigma_plus" { n + h } else { n - h }
        } else {
            let (nu, ro, ri, inertia) = (nus[&(case.clone(), pipe.clone())], f(&section["ro_m"]), f(&section[basis.as_str()]["ri_m"]), f(&section[basis.as_str()]["I_m4"]));
            let factor = ((3.0 + 2.0 * nu) * (ro * ro) + (1.0 + 2.0 * nu) * (ri * ri)) / ((4.0 * (1.0 + nu)) * inertia);
            let (vy, vz) = (at("element_local_shear_force_y", None).unwrap(), at("element_local_shear_force_z", None).unwrap());
            norm2(vy, vz) * factor / 1e6
        };
        row["value"] = json!(value);
    }
    doc
}
/// A nonzero corrosion allowance on every pipe of a straight-only document: the
/// reduced section, the pressure bore, the reduced stress rows, the reduced Lame
/// rows, the bore bits and (T4-U4b) the re-formed section-extreme and shear rows.
fn distinct(doc: &Value, allowance: f64) -> Value {
    let mut doc = doc.clone();
    assert!(
        doc["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
            .all(|c| c["pipe_sections"].as_array().unwrap().iter().all(|s| s.get(sb::MEMBER_KIND).is_none())),
        "an arc's certified maximum cannot be re-formed"
    );
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
    reform_u4b(doc)
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
            if s.get(sb::MEMBER_KIND).is_some() || arcs.contains(&(cid.clone(), s["pipe_id"].clone())) { continue; }
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
    serde_json::from_str(include_str!("../../../../fixtures/results/pressure_connected_mechanics_dense.json")).unwrap()
}
/// The arc corpus's producer document with a 0.0005 m allowance on every pipe.
fn allowance_arc() -> Value {
    arc_corpus_entry("U2-L-ANCH-PTW-K2 directional SIF")["envelope"].clone()
}
fn arc() -> Value {
    let corpus: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json")).unwrap();
    corpus["cases"][0]["envelope"].clone()
}

#[test]
fn the_fixtures_are_the_d5_interface_with_the_expected_bases() {
    // The regenerated fixtures carry §2.5's interface; the connected fixture's
    // bases coincide (no mill tolerance or allowance), the arc's differ (its
    // I7 sketch's mill tolerance).
    for (doc, coincident) in [(base(), true), (arc(), false)] {
        assert!(doc["summary"]["max_open_formula_stress"].is_null());
        for case in doc["contract_evidence"]["exact_cases"].as_array().unwrap() {
            assert_eq!(case["stress_maximum_coverage"].as_array().unwrap().len(), 2);
            for s in case["pipe_sections"].as_array().unwrap() {
                assert_eq!(s["geometry_basis"], sb::GEOMETRY_BASIS);
                assert_eq!(s["nominal_section"] == s["reduced_section"], coincident, "{}", s["pipe_id"]);
            }
        }
    }
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
        timoshenko(&distinct(&base(), 0.001), n, n, None), arc(), timoshenko(&arc(), n, n, None),
        allowance_arc(), timoshenko(&allowance_arc(), n, n, None)] {
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
    // One basis for both Lame families: the reduced hoop copied onto its nominal twin.
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

/// (case, pipe) to its case section, and to its pressure region's p.
fn case_sections_and_pressures(doc: &Value) -> (HashMap<(String, String), Value>, HashMap<(String, String), f64>) {
    let evidence = &doc["contract_evidence"];
    let sections = evidence["exact_cases"].as_array().unwrap().iter().flat_map(|c| {
        c["pipe_sections"].as_array().unwrap().iter().map(|s| ((c["load_case_id"].as_str().unwrap().to_string(), s["pipe_id"].as_str().unwrap().to_string()), s.clone())).collect::<Vec<_>>()
    }).collect();
    let pressures = evidence["pressure"].as_array().unwrap().iter().flat_map(|r| {
        r["member_pipe_ids"].as_array().unwrap().iter().map(|m| ((r["load_case_id"].as_str().unwrap().to_string(), m.as_str().unwrap().to_string()), f(&r["p_pa"]))).collect::<Vec<_>>()
    }).collect();
    (sections, pressures)
}

#[test]
fn pressure_1_reader_refuses_nominal_lame_at_the_pressure_bore() {
    // §8, §2.1: the nominal-basis Lame rows evaluated at the pressure bore. On
    // distinct bases every nominal hoop row is published as 2p*Ai/A_n at the
    // outer surface and that plus p at the inner, with Ai = pi*r_i,r^2 the
    // pressure bore's area and A_n the nominal area: neither the nominal
    // annulus's own hoop nor the reduced twin's.
    let mut doc = distinct(&base(), 0.001);
    let (sections, pressures) = case_sections_and_pressures(&doc);
    let by_id: HashMap<String, Value> = doc["results"].as_array().unwrap().iter().map(|r| (r["id"].as_str().unwrap().to_string(), r.clone())).collect();
    let mut published = 0;
    for row in doc["results"].as_array_mut().unwrap() {
        let id = row["id"].as_str().unwrap().to_string();
        if row["kind"] != "pipe_lame_hoop_stress_v2" || sb::id_basis(&id) != Some(sb::NOMINAL) { continue; }
        let key = (row["basis_ref"]["ref_id"].as_str().unwrap().to_string(), row["entity_ref"].as_str().unwrap().to_string());
        let (s, p) = (&sections[&key], pressures[&key]);
        let outer = 2.0 * (p * f(&s["pressure_bore"]["Ai_m2"])) / f(&s["nominal_section"]["As_m2"]);
        let value = if row["metadata"]["component"] == "lame_outer_hoop_stress" { outer } else { outer + p };
        assert!(value != f(&row["value"]) && value != f(&by_id[&sb::twin_id(&id).unwrap()]["value"]), "{id}: a third value");
        row["value"] = json!(value);
        published += 1;
    }
    assert!(published > 0);
    assert_eq!(refused(&doc), "SOURCE_PHYSICS_ROW_BASIS_SECTION");
}

#[test]
fn pressure_1_reader_compares_basis_twins_by_bits() {
    // A2: a twin pair's numbers compare by their binary64 bits, in Rust as in
    // the Python verifier. An exact positive zero's twin written -0.0 is
    // refused; written `0` it is the same binary64 number and is accepted.
    let rows_at = |doc: &Value, kind: &str| -> Vec<usize> {
        doc["results"].as_array().unwrap().iter().enumerate()
            .filter(|(_, r)| r["kind"] == kind && sb::id_basis(r["id"].as_str().unwrap()) == Some(sb::REDUCED)
                && r["value"].as_f64().map(f64::to_bits) == Some(0))
            .map(|(i, _)| i).collect()
    };
    // Coincident bases: stress row twins (twin_values).
    for kind in ["element_local_bending_normal_stress_y", "element_local_torsional_shear_stress"] {
        let index = rows_at(&base(), kind)[0];
        for (value, expected) in [(json!(-0.0), Err("SOURCE_PHYSICS_ROW_BASIS_SECTION".to_string())), (json!(0), Ok(()))] {
            let mut doc = base();
            doc["results"][index]["value"] = value.clone();
            assert_eq!(s::validate_pressure_evidence(&doc), expected, "{kind} = {value}");
        }
    }
    // Distinct bases: the Lame radial twins at the outer surface.
    let doc = distinct(&base(), 0.001);
    let radial = rows_at(&doc, "pipe_lame_radial_stress_v2");
    assert!(!radial.is_empty());
    for (value, expected) in [(json!(-0.0), Err("SOURCE_PHYSICS_ROW_BASIS_SECTION".to_string())), (json!(0), Ok(()))] {
        let mut changed = doc.clone();
        changed["results"][radial[0]]["value"] = value.clone();
        assert_eq!(s::validate_pressure_evidence(&changed), expected, "radial = {value}");
    }
    // Coincident bases: the maximum records (extrema_twins).
    let extremum = base()["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"].as_array().unwrap().iter()
        .position(|e| e["section_basis"] == sb::REDUCED).unwrap();
    for key in ["station_fraction", "local_fraction"] {
        let pointer = format!("/contract_evidence/exact_cases/0/pipe_stress_extrema/{extremum}/{key}");
        assert_eq!(base().pointer(&pointer).unwrap().as_f64().map(f64::to_bits), Some(0), "{key}");
        for (value, expected) in [(json!(-0.0), Err("SOURCE_PHYSICS_ROW_BASIS_SECTION".to_string())), (json!(0), Ok(()))] {
            let mut doc = base();
            *doc.pointer_mut(&pointer).unwrap() = value.clone();
            assert_eq!(s::validate_pressure_evidence(&doc), expected, "{key} = {value}");
        }
    }
}

#[test]
fn basis_headlines_bind_their_maximum_rows_by_bits() {
    // A2: a headline binds its maximum row's value bit for bit. On a zero
    // maximum, the headline written -0.0 is refused and written `0` accepted.
    let rows: Vec<Value> = sb::BASES.iter().map(|b| json!({"id": format!("result:elastic-maximum:case:a:pipe:a:{b}"),
        "kind": "pipe_elastic_normal_stress_maximum_v2", "value": 0.0, "unit": "Pa", "entity_ref": "pipe:a",
        "metadata": {"basis": format!("recovered_from_open_mechanics_stress_components{}{b}", sb::TOKEN_SEPARATOR)}})).collect();
    let by_id: HashMap<&str, &Value> = rows.iter().map(|r| (r["id"].as_str().unwrap(), r)).collect();
    let maxima: Vec<&Value> = rows.iter().collect();
    let summary = |reduced: Value| {
        let mut summary = json!({"max_open_formula_stress": null});
        for (b, r) in sb::BASES.iter().zip(&rows) {
            summary[format!("max_open_formula_stress_{b}")] = json!({"value": 0.0, "unit": "Pa", "location_ref": "pipe:a", "result_ref": r["id"]});
        }
        summary["max_open_formula_stress_reduced_section"]["value"] = reduced;
        summary
    };
    for (value, expected) in [(json!(0.0), Ok(())), (json!(0), Ok(())), (json!(-0.0), Err("SOURCE_PHYSICS_HEADLINE_RESULT_BINDING".to_string()))] {
        assert_eq!(sb::headlines(&summary(value.clone()), &by_id, &maxima, [true, true]), expected, "{value}");
    }
}

/// Sets one authored section quantity of `model`'s pipe `pipe`.
fn set_model_section(model: &mut Value, pipe: &str, key: &str, value: Value) {
    let segment = model["pipe_segments"].as_array_mut().unwrap().iter_mut().find(|p| p["id"] == pipe).unwrap();
    segment["section"][key] = value;
}

#[test]
fn pressure_1_model_checks_bind_each_section_input_to_the_model() {
    // §1, §5 (FINAL-B): G2 binds each case section's captured D, t, c_m (0.0
    // when the model has none) and c_a to the model's section of that pipe,
    // bit for bit after the units authority: each one ulp off is refused by
    // name, as is a signed zero and a pipe's section swapped with another's.
    let request: Value = serde_json::from_str(include_str!("../../../../core/product_physics/tests/fixtures/exact_pressure_connected_request.json")).unwrap();
    let connected = (base(), request["model"].clone());
    let entry = arc_corpus_entry("U2-L-ANCH-PTW-K2 0.3.0");
    let milled = (entry["envelope"].clone(), entry["document"]["model"].clone());
    // An allowance on the arc document (T4-U4b: the producer's, 0.0005 m on every
    // pipe): its distinct bases and the model.
    let entry = arc_corpus_entry("U2-L-ANCH-PTW-K2 directional SIF");
    let allowed = (entry["envelope"].clone(), entry["document"]["model"].clone());
    assert!(allowed.0["contract_evidence"]["exact_cases"][0]["pipe_sections"].as_array().unwrap().iter().all(|s| s["corrosion_allowance_m"] == 0.0005));
    let code = "SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY";
    let leaves = [("outside_diameter", "outside_diameter_m"), ("wall_thickness", "wall_thickness_m"),
        ("mill_tolerance", "mill_tolerance_m"), ("corrosion_allowance", "corrosion_allowance_m")];
    for (label, (doc, model)) in [("connected", &connected), ("milled arc", &milled), ("allowance on the arc", &allowed)] {
        assert_eq!(reader_verdict::validate("pressure", doc, Some(model), None), json!({"ok": true}), "{label}");
        let section = &doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0];
        let pipe = section["pipe_id"].as_str().unwrap();
        for (key, leaf) in leaves {
            let mut changed = model.clone();
            set_model_section(&mut changed, pipe, key, json!({"value": f(&section[leaf]).next_up(), "unit": "m"}));
            let verdict = reader_verdict::validate("pressure", doc, Some(&changed), None);
            assert_eq!(verdict["error"]["code"], code, "{label}: {key} one ulp up: {verdict}");
            assert_eq!(reader_verdict::physics_model_checks(doc, &changed), Err(code.to_string()), "{label}: {key}");
            // The same quantity in metres, bit for bit, is bound.
            set_model_section(&mut changed, pipe, key, json!({"value": section[leaf], "unit": "m"}));
            assert_eq!(reader_verdict::physics_model_checks(doc, &changed), Ok(()), "{label}: {key} restated");
        }
    }
    // The connected document has no mill tolerance and a zero allowance: an
    // absent or null mill tolerance is 0.0; a signed zero is not.
    let (doc, model) = &connected;
    let pipe = doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["pipe_id"].as_str().unwrap();
    assert!(model["pipe_segments"][0]["section"].get("mill_tolerance").is_none());
    for (key, value, expected) in [
        ("mill_tolerance", Value::Null, Ok(())),
        ("mill_tolerance", json!({"value": 0, "unit": "mm"}), Ok(())),
        ("mill_tolerance", json!({"value": -0.0, "unit": "m"}), Err(code.to_string())),
        ("corrosion_allowance", json!({"value": -0.0, "unit": "m"}), Err(code.to_string())),
        ("corrosion_allowance", Value::Null, Err(code.to_string())),
        ("wall_thickness", json!({"value": 10.0, "unit": "mm"}), Ok(())),
        ("wall_thickness", json!({"value": 10.0, "unit": "N"}), Err(code.to_string())),
    ] {
        let mut changed = model.clone();
        set_model_section(&mut changed, pipe, key, value.clone());
        assert_eq!(reader_verdict::physics_model_checks(doc, &changed), expected, "{key} = {value}");
    }
    // A pipe whose section is another pipe's: the arc document's members with
    // their own walls, then two of them swapped in the model.
    let (mut doc, mut model) = milled.clone();
    let pipes: Vec<String> = doc["contract_evidence"]["exact_cases"][0]["pipe_sections"].as_array().unwrap().iter()
        .map(|s| s["pipe_id"].as_str().unwrap().to_string()).collect();
    assert!(pipes.len() >= 2);
    let wall = |i: usize| 0.004 + 0.001 * i as f64;
    for (i, pipe) in pipes.iter().enumerate() {
        set_model_section(&mut model, pipe, "wall_thickness", json!({"value": wall(i), "unit": "m"}));
        for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            for s in case["pipe_sections"].as_array_mut().unwrap() {
                if s["pipe_id"] == pipe.as_str() { s["wall_thickness_m"] = json!(wall(i)); }
            }
        }
    }
    assert_eq!(reader_verdict::physics_model_checks(&doc, &model), Ok(()));
    let mut reversed = model.clone();
    reversed["pipe_segments"].as_array_mut().unwrap().reverse();
    assert_eq!(reader_verdict::physics_model_checks(&doc, &reversed), Ok(()), "bound by pipe id, not by order");
    let (a, b) = (pipes[0].as_str(), pipes[1].as_str());
    let section = |m: &Value, p: &str| m["pipe_segments"].as_array().unwrap().iter().find(|s| s["id"] == p).unwrap()["section"].clone();
    let mut swapped = model.clone();
    let (section_a, section_b) = (section(&model, a), section(&model, b));
    swapped["pipe_segments"].as_array_mut().unwrap().iter_mut().for_each(|s| {
        if s["id"] == a { s["section"] = section_b.clone(); } else if s["id"] == b { s["section"] = section_a.clone(); }
    });
    assert_eq!(reader_verdict::physics_model_checks(&doc, &swapped), Err(code.to_string()));
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

/// A model carrying only what G2's checks read here: the solved cases and the
/// exact contract naming `shear_model`.
fn contract_model(doc: &Value, shear_model: &str) -> Value {
    let cases: Vec<Value> = doc["contract_evidence"]["exact_cases"].as_array().unwrap().iter()
        .map(|c| json!({"id": c["load_case_id"]})).collect();
    json!({"load_cases": cases, "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model}})
}
fn arc_corpus_entry(label: &str) -> Value {
    let corpus: Value = serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_arc_reader_corpus.json")).unwrap();
    corpus["cases"].as_array().unwrap().iter().find(|c| c["label"].as_str().unwrap().starts_with(label)).unwrap().clone()
}
fn set_case_beam(doc: &mut Value, pipe: &str, beam: &Value) {
    for case in doc["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
        for s in case["pipe_sections"].as_array_mut().unwrap() {
            if s["pipe_id"] == pipe { s["beam_theory"] = beam.clone(); }
        }
    }
}

#[test]
fn pressure_1_reader_refuses_beam_theory_contradicting_the_contract() {
    // §5: a straight member's beam theory is the captured contract's shear
    // model (G2, against the model); kappa and A_s stay the evidence reader's.
    let (n, eb, tim) = (sb::NOMINAL, sb::EULER_BERNOULLI, sb::TIMOSHENKO);
    let euler = distinct(&base(), 0.001);
    let timo = timoshenko(&euler, n, n, None);
    let check = reader_verdict::physics_model_checks;
    assert_eq!(check(&euler, &contract_model(&euler, eb)), Ok(()));
    assert_eq!(check(&timo, &contract_model(&timo, tim)), Ok(()));
    assert_eq!(check(&euler, &contract_model(&euler, tim)), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    assert_eq!(check(&timo, &contract_model(&timo, eb)), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    // A contract naming no admitted model selects nothing.
    assert_eq!(check(&euler, &contract_model(&euler, "cowper")), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    // A region geometry record contradicting the contract, its case section agreeing.
    let mut region = timo.clone();
    let geometry = region["contract_evidence"]["pressure"].as_array_mut().unwrap().iter_mut()
        .flat_map(|r| r["geometry"].as_array_mut().unwrap().iter_mut()).next().expect("a pressure region");
    geometry["beam_theory"] = json!({"model": eb});
    assert_eq!(check(&region, &contract_model(&region, tim)), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    // The pressure route: the evidence reader, then G2 against the model.
    for (doc, model, refused) in [(&timo, eb, true), (&timo, tim, false), (&euler, tim, true), (&euler, eb, false)] {
        let verdict = reader_verdict::validate("pressure", doc, Some(&contract_model(doc, model)), None);
        if refused {
            assert_eq!(verdict["error"]["code"], "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT", "{verdict}");
        } else {
            assert_eq!(verdict, json!({"ok": true}));
        }
    }
}

#[test]
fn pressure_1_model_checks_take_realized_arcs_from_the_model() {
    // §3.6: a realized arc is Euler-Bernoulli under either contract. The model
    // names the arcs, including one no pressure region marks (the L line with
    // a connector has explicitly empty regions); §2.5: the marked case sections
    // are exactly those arcs.
    let (n, eb, tim) = (sb::NOMINAL, sb::EULER_BERNOULLI, sb::TIMOSHENKO);
    let entry = arc_corpus_entry("L line with connector");
    let mut model = entry["document"]["model"].clone();
    let arcs: Vec<String> = sb::realized_arcs(&model).into_iter().map(String::from).collect();
    assert_eq!(arcs, ["pipe:BEND"]);
    // The producer marks the arc outside every region on its case sections.
    let doc = entry["envelope"].clone();
    assert!(doc["contract_evidence"]["pressure"].as_array().unwrap().iter()
        .all(|r| r["geometry"].as_array().unwrap().iter().all(|g| g.get("member_kind").is_none())));
    model["pressure_contract"]["shear_model"] = json!(tim);
    // The arc's marked section stays Euler-Bernoulli; the straights take Timoshenko.
    let stated = timoshenko(&doc, n, n, None);
    assert_eq!(reader_verdict::physics_model_checks(&stated, &model), Ok(()));
    // The arc claiming Timoshenko (its kappa and A_s consistent).
    let unmarked = timoshenko(&unmarked(&doc, "pipe:BEND"), n, n, None);
    let claim = unmarked["contract_evidence"]["exact_cases"][0]["pipe_sections"].as_array().unwrap().iter()
        .find(|s| s["pipe_id"] == "pipe:BEND").unwrap()["beam_theory"].clone();
    assert_eq!(claim["model"], tim);
    let mut claimed = stated.clone();
    set_case_beam(&mut claimed, "pipe:BEND", &claim);
    assert_eq!(reader_verdict::physics_model_checks(&claimed, &model), Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()));
    // The realized arc's section without its marker.
    let mut unmarked_stated = unmarked.clone();
    set_case_beam(&mut unmarked_stated, "pipe:BEND", &json!({"model": eb}));
    assert_eq!(reader_verdict::physics_model_checks(&unmarked_stated, &model), Err("SOURCE_PHYSICS_ARC_MARKER".into()));
    // A bend the model does not realize as an arc is a straight member: its
    // marker is refused, and unmarked it states the contract's model.
    let mut geometric = model.clone();
    for c in geometric["components"].as_array_mut().unwrap() {
        if c["kind"] == "bend" { c["mechanics_interface"]["solver_consumption"] = json!("mechanics_geometry_only"); }
    }
    assert!(sb::realized_arcs(&geometric).is_empty());
    assert_eq!(reader_verdict::physics_model_checks(&stated, &geometric), Err("SOURCE_PHYSICS_ARC_MARKER".into()));
    assert_eq!(reader_verdict::physics_model_checks(&unmarked_stated, &geometric), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    assert_eq!(reader_verdict::physics_model_checks(&unmarked, &geometric), Ok(()));
    // Under Euler-Bernoulli every member states Euler-Bernoulli.
    model["pressure_contract"]["shear_model"] = json!(eb);
    assert_eq!(reader_verdict::physics_model_checks(&doc, &model), Ok(()));
    assert_eq!(reader_verdict::physics_model_checks(&stated, &model), Err("SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT".into()));
    // An arc a region marks: its marked section is the model's realized arc.
    let entry = arc_corpus_entry("U2-L-ANCH-PTW-K2 0.3.0");
    let timo = timoshenko(&entry["envelope"], n, n, None);
    let mut model = entry["document"]["model"].clone();
    model["pressure_contract"]["shear_model"] = json!(tim);
    assert_eq!(reader_verdict::physics_model_checks(&timo, &model), Ok(()));
    let mut claimed = timo.clone();
    let bend = claimed["contract_evidence"]["pressure"][0]["geometry"].as_array().unwrap().iter()
        .find(|g| g.get("member_kind").is_some()).unwrap()["pipe_id"].as_str().unwrap().to_string();
    let straight = claimed["contract_evidence"]["exact_cases"][0]["pipe_sections"].as_array().unwrap().iter()
        .find(|s| s["pipe_id"] != bend.as_str()).unwrap()["beam_theory"].clone();
    set_case_beam(&mut claimed, &bend, &straight);
    assert_eq!(reader_verdict::physics_model_checks(&claimed, &model), Err("SOURCE_PHYSICS_ARC_BEAM_THEORY".into()));
    // A model without components realizes no arc: the region's mark contradicts it.
    model.as_object_mut().unwrap().remove("components");
    assert_eq!(reader_verdict::physics_model_checks(&timo, &model), Err("SOURCE_PHYSICS_ARC_MARKER".into()));
}

#[test]
fn pressure_1_reader_binds_the_case_section_marker_to_the_region_arc() {
    // §2.5: the region marks exactly the members the case sections mark.
    let doc = arc();
    assert_eq!(s::validate_pressure_evidence(&doc), Ok(()));
    let region = doc["contract_evidence"]["pressure"].as_array().unwrap().iter()
        .find(|r| r["geometry"].as_array().unwrap().iter().any(|g| g.get("member_kind").is_some())).unwrap().clone();
    let bend = region["geometry"].as_array().unwrap().iter().find(|g| g.get("member_kind").is_some()).unwrap()["pipe_id"].clone();
    let straight = region["geometry"].as_array().unwrap().iter().find(|g| g.get("member_kind").is_none()).unwrap()["pipe_id"].clone();
    let marking = |doc: &Value, pipe: &Value, marked: bool| {
        let mut changed = doc.clone();
        for case in changed["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            for s in case["pipe_sections"].as_array_mut().unwrap() {
                if s["pipe_id"] == *pipe {
                    if marked { s[sb::MEMBER_KIND] = json!(sb::REALIZED_ARC); } else { s.as_object_mut().unwrap().remove(sb::MEMBER_KIND); }
                }
            }
        }
        changed
    };
    // The region's arc without the marker on its case section.
    assert_eq!(refused(&marking(&doc, &bend, false)), "SOURCE_PHYSICS_ARC_MARKER");
    // A region straight marked on its case section, its region record unmarked.
    assert_eq!(refused(&marking(&doc, &straight, true)), "SOURCE_PHYSICS_ARC_MARKER");
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

/// O4 (DEL-04-08 Design `o4-source-extension.md` §3.4): `PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE`
/// is retired. The invocation check no longer refuses a nonzero mill tolerance; it binds
/// by value through `ACTUAL_SOURCE_GEOMETRY` and its range through the D-5 rederivation. A
/// present mill tolerance that is not a finite length >= 0, or that with the allowance
/// reaches the wall, leaves the section sum unestablished and is refused with the
/// allowance (`PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE`).
#[test]
fn pressure_source_1_admits_mill_tolerance_within_the_section_sum() {
    let request: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/pressure_source/physics_source/n05.request.json"
    )).unwrap();
    // Any source: the invocation's contract is checked first (`SourceContract::check_invocation`).
    let source = json!({});
    let invocation = json!({"request": request, "solver_mode": "sparse_interactive"});
    let first = physics_source::validate(&source, Some(&invocation)).unwrap_err();
    assert!(!first.contains("ACTUAL_MILL_TOLERANCE") && !first.contains("ACTUAL_CORROSION_ALLOWANCE"), "{first}");
    for (value, refused) in [(json!({"value": 0.5, "unit": "mm"}), false), (json!({"value": 9.999, "unit": "mm"}), false),
        (json!({"value": 0, "unit": "mm"}), false), (Value::Null, false), (json!({"value": 10, "unit": "mm"}), true),
        (json!({"value": -0.5, "unit": "mm"}), true), (json!({"value": 1, "unit": "N"}), true), (json!({"value": "0", "unit": "mm"}), true)] {
        let mut changed = invocation.clone();
        changed["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = value.clone();
        let outcome = physics_source::validate(&source, Some(&changed)).unwrap_err();
        assert!(!outcome.contains("ACTUAL_MILL_TOLERANCE"), "{value}: {outcome}");
        assert_eq!(outcome == "PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE", refused, "{value}: {outcome}");
    }
}
