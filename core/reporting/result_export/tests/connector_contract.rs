//! T4-U3 (S14): the pressure-2 reader admits objective connector records and
//! rows over the shared connector corpus (also read by the Python
//! reader), refuses each broken binding by name, and the retained-source route
//! (pressure-source-1) refuses connector evidence (`CONNECTOR_UNSUPPORTED`).
use open_pipe_stress_result_export::semantic_contract as s;
use serde_json::{json, Value};

fn corpus() -> Vec<(String, Value)> {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../fixtures/results/pressure_v3_connector_reader_corpus.json"
    ))
    .unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| (case["label"].as_str().unwrap().to_string(), case["envelope"].clone()))
        .collect()
}

fn load_state(envelope: &Value) -> bool {
    envelope["contract_evidence"].get("load_reference_states").is_some()
}

fn rows_mut(envelope: &mut Value) -> &mut Vec<Value> {
    envelope["results"].as_array_mut().unwrap()
}

fn first_connector_row(envelope: &mut Value) -> &mut Value {
    rows_mut(envelope)
        .iter_mut()
        .find(|r| r["kind"].as_str().unwrap().starts_with("connector_"))
        .unwrap()
}

fn record(envelope: &mut Value) -> &mut Value {
    &mut envelope["contract_evidence"]["connector"][0]
}

/// A constant-area connector outside every region carries Pe=0. This is a
/// result-reader shape/control, not a document-admission or mechanics oracle.
fn u5a_remote_control() -> Value {
    let mut envelope=corpus().remove(0).1;
    let record=record(&mut envelope);
    record["pressure_model"]=json!("constant_effective_area_reference_v1");
    record["effective_area_m2"]=json!(0.004);
    record["pressure_basis"]=json!("internal_differential_zero_external_v1");
    record["pressure_source_reference"]=json!("invented U5a reader control");
    let generalized=rows_mut(&mut envelope).iter().filter(|row|row["kind"]=="connector_generalized_force_v1" && row["metadata"]["component"]=="gt_x").cloned().collect::<Vec<_>>();
    for g in generalized {
        for (kind,component,value) in [("connector_pressure_force_v1","pe_x",json!(0.)),("connector_effective_axial_force_v1","s_x",g["value"].clone())] {
            let mut row=g.clone();
            row["id"]=json!(format!("{}:{component}",g["id"].as_str().unwrap()));
            row["kind"]=json!(kind);row["metadata"]["component"]=json!(component);row["value"]=value;
            rows_mut(&mut envelope).push(row);
        }
    }
    envelope
}
#[test]
fn u5a_remote_constant_area_rows_are_read_and_pressure_tampering_is_refused() {
    let envelope=u5a_remote_control();
    s::validate_pressure_evidence(&envelope).unwrap();
    for field in ["effective_area_m2","pressure_basis","pressure_source_reference"] {
        let mut missing=envelope.clone();record(&mut missing).as_object_mut().unwrap().remove(field);
        assert!(s::validate_pressure_evidence(&missing).is_err(),"missing {field}");
    }
    let mut forged=envelope.clone();
    rows_mut(&mut forged).iter_mut().find(|row|row["kind"]=="connector_pressure_force_v1").unwrap()["value"]=json!(f64::from_bits(1));
    assert!(s::validate_pressure_evidence(&forged).unwrap_err().contains("PRESSURE_ROW_BINDING"));
    let mut forged=envelope.clone();
    rows_mut(&mut forged).iter_mut().find(|row|row["kind"]=="connector_effective_axial_force_v1").unwrap()["value"]=json!(1e6);
    assert!(s::validate_pressure_evidence(&forged).unwrap_err().contains("EFFECTIVE_ROW_BINDING"));
    let mut unpressurized=envelope.clone();record(&mut unpressurized)["pressure_model"]=json!("unpressurized");
    assert!(s::validate_pressure_evidence(&unpressurized).unwrap_err().contains("RECORD_SHAPE"));
}

fn u5a_participating_control()->Value {
    let control:Value=serde_json::from_str(include_str!("../../../../fixtures/results/pressure_v3_joint_reader_control.json")).unwrap();
    control["envelope"].clone()
}
#[test]
fn u5a_participating_reader_control_binds_edges_groups_and_each_pressure_row() {
    let envelope=u5a_participating_control();s::validate_pressure_evidence(&envelope).unwrap();
    for (field,value) in [("connector_id",json!("component:wrong")),("face_i_pipe_id",json!("pipe:wrong")),("effective_area_m2",json!(0.005)),("p_pa",json!(1.)),("area_difference_force_n",json!([0.,0.])),("attachment_pair_global_n",json!([[0.,0.,0.],[0.,0.,0.]]))] {
        let mut forged=envelope.clone();forged["contract_evidence"]["pressure"][0]["joint_edges"][0][field]=value;
        assert!(s::validate_pressure_evidence(&forged).is_err(),"edge {field}");
    }
    let mut missing=envelope.clone();missing["contract_evidence"]["pressure"][0]["joint_edges"]=json!([]);
    assert!(s::validate_pressure_evidence(&missing).unwrap_err().contains("PRESSURE_EDGE_COVERAGE"));
    let mut basis=envelope.clone();record(&mut basis)["pressure_basis"]=json!("wrong");
    assert!(s::validate_pressure_evidence(&basis).is_err());
    for field in ["effective_area_bits","pressure_bits","direction_component_magnitude"] {
        let mut forged=envelope.clone();
        let group=forged["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"].as_array_mut().unwrap().iter_mut().find(|group|group.get("effective_area_bits").is_some()).unwrap();
        group[field]=if field=="direction_component_magnitude" {json!(0.5)} else {json!("0000000000000000")};
        assert!(s::validate_pressure_evidence(&forged).is_err(),"group {field}");
    }
    let mut duplicate=envelope.clone();
    let group=duplicate["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"].as_array_mut().unwrap().iter_mut().find(|group|group.get("effective_area_bits").is_some()).unwrap();
    let term=group["terms"][0].clone();group["terms"].as_array_mut().unwrap().push(term);
    group["coefficient_sum"]=json!(2.*group["coefficient_sum"].as_f64().unwrap());
    group["assembled_force_n"]=json!(2.*group["assembled_force_n"].as_f64().unwrap());
    assert!(s::validate_pressure_evidence(&duplicate).unwrap_err().contains("RHS_JOINT_SOURCE_DUPLICATE"));
    let mut wrong_face=envelope.clone();
    let group=wrong_face["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"].as_array_mut().unwrap().iter_mut().find(|group|group["terms"].as_array().unwrap().iter().any(|term|term["kind"]=="joint_face_cap")).unwrap();
    group["terms"].as_array_mut().unwrap().iter_mut().find(|term|term["kind"]=="joint_face_cap").unwrap()["pipe_id"]=json!("pipe:J");
    assert!(s::validate_pressure_evidence(&wrong_face).is_err());
}

/// (label, mutation, the pressure-2 reader's error suffix).
#[allow(clippy::type_complexity)]
fn mutations() -> Vec<(&'static str, Box<dyn Fn(&mut Value)>, &'static str)> {
    vec![
        ("record removed", Box::new(|e: &mut Value| e["contract_evidence"]["connector"] = json!([])), "CONNECTOR_ROW_UNBOUND"),
        ("record duplicated", Box::new(|e: &mut Value| {
            let r = e["contract_evidence"]["connector"][0].clone();
            e["contract_evidence"]["connector"].as_array_mut().unwrap().push(r);
        }), "CONNECTOR_RECORD_DUPLICATE"),
        ("record extra key", Box::new(|e: &mut Value| record(e)["extra"] = json!(1)), "CONNECTOR_RECORD_SHAPE"),
        ("record hardware tied", Box::new(|e: &mut Value| record(e)["hardware"] = json!("tied")), "CONNECTOR_RECORD_LAW"),
        ("record q_ref short", Box::new(|e: &mut Value| record(e)["q_ref"] = json!([0.0, 0.0])), "CONNECTOR_RECORD_FRAME"),
        ("record rotation scale", Box::new(|e: &mut Value| record(e)["work_matrix"]["rotation_scale_rad"] = json!(2.0)), "CONNECTOR_WORK_MATRIX"),
        ("replaced span published", Box::new(|e: &mut Value| record(e)["replaced_pipe_id"] = json!("pipe:P-120")), "CONNECTOR_REPLACED_SPAN_PUBLISHED"),
        ("row removed", Box::new(|e: &mut Value| {
            let index = rows_mut(e).iter().position(|r| r["kind"].as_str().unwrap().starts_with("connector_")).unwrap();
            rows_mut(e).remove(index);
        }), "CONNECTOR_ROW_COVERAGE"),
        ("row unbound", Box::new(|e: &mut Value| first_connector_row(e)["entity_ref"] = json!("component:C-999")), "CONNECTOR_ROW_UNBOUND"),
        ("row unknown kind", Box::new(|e: &mut Value| first_connector_row(e)["kind"] = json!("connector_energy_v1")), "CONNECTOR_ROW_KIND"),
        // B7b (O-10): pressure-2's own resident table carries the connector
        // signatures, so its unit check refuses a wrong unit before the
        // connector row binding is read.
        ("row unit", Box::new(|e: &mut Value| first_connector_row(e)["unit"] = json!("mm")), "SOURCE_UNIT_CONTRADICTION: connector_generalized_translation_v1"),
        ("row basis", Box::new(|e: &mut Value| first_connector_row(e)["metadata"]["basis"] = json!("objective_connector_v1;replaces_span=pipe:P-120;symmetric_midpoint_small_rotation_v1")), "CONNECTOR_ROW_SEMANTICS"),
        ("row sign", Box::new(|e: &mut Value| first_connector_row(e)["metadata"]["sign_convention"] = json!("global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f")), "CONNECTOR_ROW_SEMANTICS"),
        ("row duplicated", Box::new(|e: &mut Value| {
            let mut r = first_connector_row(e).clone();
            r["id"] = json!("result:connector:duplicate");
            rows_mut(e).push(r);
        }), "CONNECTOR_ROW_DUPLICATE"),
        ("replaced span row", Box::new(|e: &mut Value| {
            let mut r = rows_mut(e).iter().find(|r| r["kind"] == "element_local_axial_force").unwrap().clone();
            r["id"] = json!("result:replaced-span");
            r["entity_ref"] = json!("pipe:P-130");
            rows_mut(e).push(r);
        }), "CONNECTOR_REPLACED_SPAN_PUBLISHED"),
    ]
}

#[test]
fn the_connector_corpus_is_admitted_by_the_pressure_1_reader_and_dispatch() {
    let cases = corpus();
    assert_eq!(cases.len(), 2);
    assert!(cases.iter().any(|(_, e)| load_state(e)) && cases.iter().any(|(_, e)| !load_state(e)));
    for (label, envelope) in &cases {
        assert_eq!(envelope["producer"]["semantic_contract_id"], s::PRESSURE_ID, "{label}");
        assert_eq!(envelope["contract_evidence"]["connector"].as_array().unwrap().len(), 1, "{label}");
        s::validate_pressure_evidence(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
        s::for_source(envelope).unwrap_or_else(|e| panic!("{label}: {e}"));
    }
}

#[test]
fn broken_connector_bindings_are_refused_by_name() {
    for (label, envelope) in corpus() {
        for (what, mutate, code) in mutations() {
            let mut broken = envelope.clone();
            mutate(&mut broken);
            let error = s::validate_pressure_evidence(&broken).unwrap_err();
            assert!(error.ends_with(code), "{label} {what}: {error}");
        }
    }
}

/// B7b: the retained-source route (pressure-source-1, both source families)
/// declines objective connectors (b7-plan J7/D7), so a connector record grafted
/// into a genuine pressure-source-1 statement is refused, its receipt resealed
/// so that the evidence checks are reached. (The retired v2 readers' refusal of
/// connector evidence went with them.)
#[test]
fn the_source_route_refuses_connector_evidence() {
    use open_pipe_stress_result_export::{physics_source as ps, source_blocks::domain_hash};
    let read = |path: &str| -> Value {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../");
        serde_json::from_str(&std::fs::read_to_string(format!("{root}{path}")).unwrap()).unwrap()
    };
    let record = corpus().remove(0).1["contract_evidence"]["connector"][0].clone();
    // The 0.3 family's closed receipt evidence shape admits only an empty
    // connector list; the load/reference-state pre-pass refuses it by name first.
    for (family, path, code) in [
        ("0.3 family", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", "PHYSICS_SOURCE_PHYSICAL_SHAPE"),
        ("load/reference-state family", "fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json", "SOURCE_LOAD_REFERENCE_CONNECTOR_UNSUPPORTED"),
    ] {
        let mut source = read(path);
        assert_eq!(ps::validate(&source, None).map(|_| ()), Ok(()), "{family}");
        source["contract_evidence"]["connector"] = json!([record.clone()]);
        let mut publication = source.clone();
        publication.as_object_mut().unwrap().remove("source_block_recovery");
        source["source_block_recovery"]["body"]["publication_sha256"] =
            json!(domain_hash("source_blocks_publication_v1", &publication).unwrap());
        source["source_block_recovery"]["receipt_sha256"] =
            json!(domain_hash("source_blocks_receipt_v1", &source["source_block_recovery"]["body"]).unwrap());
        assert_eq!(ps::validate(&source, None).unwrap_err(), code, "{family}");
        assert_eq!(s::for_source(&source).unwrap_err(), code, "{family}");
        assert_eq!(ps::validate_transport_metadata(&source).unwrap_err(), code, "{family}");
    }
}
