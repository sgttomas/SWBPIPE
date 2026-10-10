//! T4-U2: the pressure-1 reader admits realized-arc envelopes over the shared
//! arc corpus (also read by the Python and TypeScript readers): N_w = N_el +
//! pAi and S = N_el in the arc's own sign conventions, end rows along the end
//! tangents, and the straight Lame rows and maximum withheld with the named
//! reason. Each broken arc binding is refused by name; arc evidence under the
//! v2 contract is refused; a model whose only pipe is a replaced span is
//! covered by its connector (T4-RV23 NOTE-4).
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

/// (label, mutation, the pressure-1 reader's error suffix); arc-region cases.
#[allow(clippy::type_complexity)]
pub fn mutations() -> Vec<(&'static str, Box<dyn Fn(&mut Value)>, &'static str)> {
    vec![
        ("arc sign", Box::new(|e: &mut Value| {
            row(e, "pipe_wall_axial_force_v2", "pipe:BEND", "midspan")["metadata"]["sign_convention"] =
                json!("tension-positive material wall section resultant Nw");
        }), "ROW_SEMANTICS"),
        ("arc hoop row", Box::new(|e: &mut Value| {
            let mut hoop = row(e, "pipe_lame_hoop_stress_v2", "pipe:S1", "midspan").clone();
            hoop["entity_ref"] = json!("pipe:BEND");
            hoop["id"] = json!("result:arc-hoop");
            rows(e).push(hoop);
        }), "ARC_WITHHELD"),
        ("arc maximum not withheld", Box::new(|e: &mut Value| {
            let coverage = &mut e["contract_evidence"]["exact_cases"][0]["stress_maximum_coverage"];
            coverage["unavailable_pipe_ids"] = json!([]);
            coverage["complete"] = json!(true);
        }), "ARC_WITHHELD"),
        ("arc region key removed", Box::new(|e: &mut Value| {
            region(e).as_object_mut().unwrap().remove("tangency_rule");
        }), "REGION_SHAPE"),
        ("arc approximation", Box::new(|e: &mut Value| {
            region(e)["approximation"] = json!("long_straight_annulus_small_strain_v2");
        }), "ARC_REGION_PROFILE"),
        ("arc withheld reason", Box::new(|e: &mut Value| {
            region(e)["withheld_on_arcs"]["reason"] = json!("withheld");
        }), "ARC_REGION_PROFILE"),
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
    assert_eq!(cases.len(), 5);
    assert_eq!(cases.iter().filter(|(_, e)| has_arc_region(e)).count(), 3);
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

#[test]
fn arc_evidence_is_refused_under_v2() {
    for (label, envelope) in corpus().into_iter().filter(|(_, e)| has_arc_region(e)) {
        let mut as_v2 = envelope.clone();
        for case in as_v2["contract_evidence"]["exact_cases"].as_array_mut().unwrap() {
            case["profile_mode"] = json!("exact_straight_pressure_v2");
        }
        for region in as_v2["contract_evidence"]["pressure"].as_array_mut().unwrap() {
            region["profile_version"] = json!("2.0.0");
            region["profile_mode"] = json!("exact_straight_pressure_v2");
        }
        let error = if as_v2["contract_evidence"].get("load_reference_states").is_some() {
            s::validate_load_reference_evidence(&as_v2)
        } else {
            s::validate_physics_evidence(&as_v2)
        }
        .unwrap_err();
        assert!(error.ends_with("ARC_UNSUPPORTED"), "{label}: {error}");
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
