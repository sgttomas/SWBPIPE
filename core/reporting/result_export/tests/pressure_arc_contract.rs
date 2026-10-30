//! T4-U2: the pressure-2 reader admits realized-arc envelopes over the shared
//! arc corpus (also read by the Python reader): N_w = N_el +
//! pAi and S = N_el in the arc's own sign conventions, end rows along the end
//! tangents, and the straight Lame rows withheld. T4-U4b: the section stress
//! rows, the toroidal hoop and radial rows, the certified arc maximum and the
//! directional intensified rows are recomputed from the evidence. Each broken
//! arc binding is refused by name; a model whose only pipe is a replaced span
//! is covered by its connector (T4-RV23 NOTE-4). (B7b deleted the v2 contract
//! and its arc-refusal check with it.)
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

pub fn corpus() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/pressure_v3_arc_reader_corpus.json"
    ))
    .unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| (case["label"].as_str().unwrap().to_string(), case["envelope"].clone()))
        .collect()
}

fn has_arc_region(envelope: &Value) -> bool {
    envelope["contract_evidence"]["pressure"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r.get("bend_adjacent_junctions").is_some())
}

fn rows(e: &mut Value) -> &mut Vec<Value> {
    e["results"].as_array_mut().unwrap()
}
fn row<'a>(e: &'a mut Value, kind: &str, entity: &str, location: &str) -> &'a mut Value {
    rows(e)
        .iter_mut()
        .find(|r| r["kind"] == kind && r["entity_ref"] == entity && r["metadata"]["location"] == location)
        .unwrap()
}
fn region(e: &mut Value) -> &mut Value {
    &mut e["contract_evidence"]["pressure"][0]
}
fn member<'a>(list: &'a mut Value, pipe: &str) -> &'a mut Value {
    list.as_array_mut().unwrap().iter_mut().find(|g| g["pipe_id"] == pipe).unwrap()
}
fn bend_rows<'a>(e: &'a mut Value, kind: &'a str, location: &'a str, component: Option<&'a str>) -> impl Iterator<Item = &'a mut Value> + 'a {
    rows(e).iter_mut().filter(move |r| {
        r["kind"] == kind
            && r["entity_ref"] == "pipe:BEND"
            && r["metadata"]["location"] == location
            && component.is_none_or(|c| r["metadata"]["component"] == c)
    })
}
/// Scales the first matching pipe:BEND row's value (T4-U4b tampers).
fn scale(e: &mut Value, kind: &str, location: &str, component: Option<&str>, factor: f64) {
    let row = bend_rows(e, kind, location, component).next().unwrap();
    row["value"] = json!(row["value"].as_f64().unwrap() * factor);
}
/// Drops the first (or every) matching pipe:BEND row.
fn drop_rows(e: &mut Value, kind: &str, location: &str, component: Option<&str>, every: bool) {
    let ids: Vec<Value> = bend_rows(e, kind, location, component).map(|r| r["id"].clone()).collect();
    let ids = if every { ids } else { ids[..1].to_vec() };
    rows(e).retain(|r| !ids.contains(&r["id"]));
}

/// (label, mutation, the pressure-2 reader's error suffix); arc-region cases.
#[allow(clippy::type_complexity)]
pub fn mutations() -> Vec<(&'static str, Box<dyn Fn(&mut Value)>, &'static str)> {
    vec![
        ("arc sign", Box::new(|e: &mut Value| {
            row(e, "pipe_wall_axial_force_v2", "pipe:BEND", "midspan")["metadata"]["sign_convention"] =
                json!("tension-positive material wall section resultant Nw");
        }), "ROW_SEMANTICS"),
        // T4-U6 (D-5 §2.1, §2.5): the straight's hoop row is published on both
        // section bases, each id ending in its basis; the arc copy keeps the pair.
        ("arc hoop row", Box::new(|e: &mut Value| {
            let twins: Vec<Value> = rows(e)
                .iter()
                .filter(|r| r["kind"] == "pipe_lame_hoop_stress_v2" && r["entity_ref"] == "pipe:S1" && r["metadata"]["location"] == "midspan")
                .take(2)
                .cloned()
                .collect();
            for mut hoop in twins {
                let basis = hoop["id"].as_str().unwrap().rsplit(':').next().unwrap().to_string();
                assert!(basis == "nominal_section" || basis == "reduced_section", "{basis}");
                hoop["entity_ref"] = json!("pipe:BEND");
                hoop["id"] = json!(format!("result:arc-hoop:{basis}"));
                rows(e).push(hoop);
            }
        }), "ARC_WITHHELD"),
        ("arc region key removed", Box::new(|e: &mut Value| {
            region(e).as_object_mut().unwrap().remove("tangency_rule");
        }), "REGION_SHAPE"),
        ("arc approximation", Box::new(|e: &mut Value| {
            region(e)["approximation"] = json!("long_straight_annulus_small_strain_v2");
        }), "ARC_REGION_PROFILE"),
        ("arc stress definitions", Box::new(|e: &mut Value| {
            region(e)["arc_stress_definitions"][0] = json!("changed");
        }), "ARC_REGION_PROFILE"),
        // T4-U4b (§2.1): arcs no longer withhold their maximum.
        ("arc withholding revived", Box::new(|e: &mut Value| {
            region(e)["withheld_on_arcs"] = json!({"reason": "withheld", "kinds": ["pipe_elastic_normal_stress_maximum_v2"]});
        }), "REGION_SHAPE"),
        ("arc coverage stale", Box::new(|e: &mut Value| {
            for coverage in e["contract_evidence"]["exact_cases"][0]["stress_maximum_coverage"].as_array_mut().unwrap() {
                coverage["unavailable_pipe_ids"] = json!(["pipe:BEND"]);
                coverage["complete"] = json!(false);
            }
        }), "EXTREMA_MEMBER_COVERAGE"),
        ("arc sigma plus", Box::new(|e: &mut Value| scale(e, "element_local_normal_stress_extreme", "midspan", Some("sigma_plus"), 1.001)), "EXTREME_VALUE"),
        ("arc sigma minus missing", Box::new(|e: &mut Value| drop_rows(e, "element_local_normal_stress_extreme", "midspan", Some("sigma_minus"), false)), "ROW_BASIS_TWIN"),
        ("arc transverse shear", Box::new(|e: &mut Value| scale(e, "element_local_transverse_shear_stress_maximum", "midspan", None, 1.001)), "SHEAR_VALUE"),
        ("arc toroidal hoop", Box::new(|e: &mut Value| scale(e, "pipe_toroidal_membrane_hoop_stress_v1", "midspan", None, 1.001)), "TOROIDAL_VALUE"),
        ("arc radial", Box::new(|e: &mut Value| scale(e, "pipe_toroidal_radial_surface_stress_v1", "midspan", None, 1.001)), "TOROIDAL_VALUE"),
        ("arc maximum value", Box::new(|e: &mut Value| scale(e, "pipe_elastic_normal_stress_maximum_v2", "governing_station", None, 0.9)), "EXTREMA_BOUNDS"),
        ("arc maximum removed", Box::new(|e: &mut Value| drop_rows(e, "pipe_elastic_normal_stress_maximum_v2", "governing_station", None, true)), "EXTREMA_RESULT_MISSING"),
        ("arc bend radius", Box::new(|e: &mut Value| {
            let g = member(&mut region(e)["geometry"], "pipe:BEND");
            g["bend_radius_m"] = json!(g["bend_radius_m"].as_f64().unwrap() * 1.01);
        }), "TOROIDAL_VALUE"),
        ("arc member kind", Box::new(|e: &mut Value| {
            member(&mut region(e)["geometry"], "pipe:BEND")["member_kind"] = json!("realized_mitre");
        }), "ARC_GEOMETRY"),
        ("arc strain sign", Box::new(|e: &mut Value| {
            let load = member(&mut region(e)["applied_loads"], "pipe:BEND");
            load["arc_pressure_strain"] = json!(-load["arc_pressure_strain"].as_f64().unwrap());
        }), "ARC_APPLIED_LOAD"),
        ("arc cap removal", Box::new(|e: &mut Value| {
            let load = member(&mut region(e)["applied_loads"], "pipe:BEND");
            let value = load["bend_cap_pair_removed_global_n"][0][0].as_f64().unwrap();
            load["bend_cap_pair_removed_global_n"][0][0] = json!(2.0 * value + 1.0);
        }), "ARC_APPLIED_LOAD"),
        ("junction dropped", Box::new(|e: &mut Value| {
            region(e)["bend_adjacent_junctions"].as_array_mut().unwrap().pop();
        }), "ARC_JUNCTIONS"),
        ("junction theta", Box::new(|e: &mut Value| {
            region(e)["bend_adjacent_junctions"][0]["theta_rad"] = json!(2e-3);
        }), "ARC_JUNCTIONS"),
        ("junction tangent", Box::new(|e: &mut Value| {
            let t = region(e)["bend_adjacent_junctions"][0]["t_in_global"][1].as_f64().unwrap();
            region(e)["bend_adjacent_junctions"][0]["t_in_global"][1] = json!(t + 1e-3);
        }), "ARC_JUNCTION_TANGENT"),
        ("arc end row missing", Box::new(|e: &mut Value| {
            let id = row(e, "pipe_wall_endpoint_action_v2", "pipe:BEND", "end_j")["id"].clone();
            rows(e).retain(|r| r["id"] != id);
            region(e)["result_ids"].as_array_mut().unwrap().retain(|r| *r != id);
        }), "PRESSURE_ROW_COVERAGE"),
        ("bend term on a straight", Box::new(|e: &mut Value| {
            let groups = e["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"].as_array_mut().unwrap();
            let term = groups
                .iter_mut()
                .flat_map(|g| g["terms"].as_array_mut().unwrap().iter_mut())
                .find(|t| t["kind"] == "terminal_cap")
                .unwrap();
            term["kind"] = json!("bend_cap_removed");
        }), "ARC_RHS_TERM"),
        ("case-free quantity row", Box::new(|e: &mut Value| {
            row(e, "global_nodal_displacement_x", "node:B", "node")
                .as_object_mut()
                .unwrap()
                .remove("basis_ref");
        }), "CASE_BASIS"),
    ]
}

#[test]
fn the_arc_corpus_is_admitted_by_the_pressure_1_reader_and_dispatch() {
    let cases = corpus();
    assert_eq!(cases.len(), 6);
    assert_eq!(cases.iter().filter(|(_, e)| has_arc_region(e)).count(), 4);
    for (label, envelope) in &cases {
        assert_eq!(envelope["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{label}");
        s::validate_pressure_evidence(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        s::for_source(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
    }
}

#[test]
fn broken_arc_bindings_are_refused_by_name() {
    for (label, envelope) in corpus().into_iter().filter(|(_, e)| has_arc_region(e)) {
        for (what, mutate, code) in mutations() {
            let mut broken = envelope.clone();
            mutate(&mut broken);
            let error = s::validate_pressure_evidence(&broken).unwrap_err();
        assert!(error.ends_with(code), "{label} / {what}: {error}");
        }
    }
}

const DIRECTIONAL: &str = "curved_bend_directional_intensified_bending_stress_v1";
const DIRECTIONAL_MAXIMUM: &str = "curved_bend_directional_intensified_bending_maximum_v1";
fn measures(e: &mut Value) -> &mut Vec<Value> {
    e["contract_evidence"]["exact_cases"][0]["intensified_measures"].as_array_mut().unwrap()
}

/// T4-U4b (§2.4): directional intensified rows on the directional-SIF arc case.
#[allow(clippy::type_complexity)]
pub fn directional_mutations() -> Vec<(&'static str, Box<dyn Fn(&mut Value)>, &'static str)> {
    vec![
        ("directional value", Box::new(|e: &mut Value| scale(e, DIRECTIONAL, "midspan", None, 1.001)), "INTENSIFIED_VALUE"),
        ("directional maximum value", Box::new(|e: &mut Value| scale(e, DIRECTIONAL_MAXIMUM, "governing_station", None, 0.9)), "EXTREMA_BOUNDS"),
        ("directional factor", Box::new(|e: &mut Value| measures(e)[0]["i_in_plane"] = json!(2.4)), "INTENSIFIED_VALUE"),
        ("directional pair swapped", Box::new(|e: &mut Value| {
            for m in measures(e) {
                let (a, b) = (m["i_in_plane"].clone(), m["i_out_of_plane"].clone());
                m["i_in_plane"] = b;
                m["i_out_of_plane"] = a;
            }
        }), "INTENSIFIED_VALUE"),
        ("directional record dropped", Box::new(|e: &mut Value| {
            measures(e).pop();
        }), "INTENSIFIED_ROW_UNBOUND"),
        ("directional row dropped", Box::new(|e: &mut Value| drop_rows(e, DIRECTIONAL, "midspan", None, false)), "ROW_BASIS_TWIN"),
        ("directional maximum dropped", Box::new(|e: &mut Value| drop_rows(e, DIRECTIONAL_MAXIMUM, "governing_station", None, true)), "INTENSIFIED_RESULT_MISSING"),
        ("directional record on a straight", Box::new(|e: &mut Value| measures(e)[0]["pipe_id"] = json!("pipe:S1")), "ARC_ROW_ON_STRAIGHT"),
    ]
}

#[test]
fn broken_directional_bindings_are_refused_by_name() {
    let (label, envelope) = corpus().into_iter().find(|(l, _)| l.contains("directional SIF")).unwrap();
    for (what, mutate, code) in directional_mutations() {
        let mut broken = envelope.clone();
        mutate(&mut broken);
        let error = s::validate_pressure_evidence(&broken).unwrap_err();
        assert!(error.ends_with(code), "{label} / {what}: {error}");
    }
}

#[test]
fn a_replaced_span_only_model_needs_its_connector() {
    let (label, envelope) = corpus().into_iter().find(|(l, _)| l.starts_with("replaced span only")).unwrap();
    assert!(envelope["contract_evidence"]["exact_cases"][0]["pipe_sections"].as_array().unwrap().is_empty(), "{label}");
    let mut broken = envelope.clone();
    broken["contract_evidence"]["connector"] = json!([]);
    let error = s::validate_pressure_evidence(&broken).unwrap_err();
    assert!(error.ends_with("MEMBER_COVERAGE"), "{label}: {error}");
}
