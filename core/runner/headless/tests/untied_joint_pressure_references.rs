//! Current U5a references through actual typed, captured and headless consumers.
//! Reference mathematics and criteria are frozen; each SHA-bound frozen request is
//! re-authored in memory to the current model version by the shared rule
//! (`core/product_physics/tests/common/current_model.rs`; a 0.5.0 request by its
//! version alone). These controls grant no retained permit or qualification.
use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, PrivacyContext, ProfessionalBoundary, Provenance,
    RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode, run_linear_static_preview_with_mode, PreviewSolverMode,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

#[path = "../../../product_physics/tests/common/current_model.rs"]
mod current_model;

const MODES: [PreviewSolverMode; 2] = [
    PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny,
];
const ORACLE: &str = include_str!("../../../../validation/references/t4_u5a/oracle.json");
const PRESSURE_ID: &str = "openpipestress.result_semantics/0.3.0/pressure-2";

fn references() -> Value { serde_json::from_str(ORACLE).unwrap() }
fn payload(case: &Value) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../validation/references/t4_u5a").join(case["request"].as_str().unwrap());
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), case["request_sha256"].as_str().unwrap());
    current_model::reauthor_request(serde_json::from_slice(&bytes).unwrap())
}
fn runner_request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "u5-current-reference".into(), operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", model["project"]["id"].as_str().unwrap()),
        model_ref: Reference::new("model", model["project"]["id"].as_str().unwrap()),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"].as_array().unwrap().iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap())).collect(),
        input_manifest_ref: Reference::new("audit_manifest", "invented-u5-current-reference"),
        requested_outputs: vec!["result_envelope".into(), "audit_manifest".into(), "diagnostics".into()],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented independent U5a controls".into(),
            source_location: "validation/references/t4_u5a".into(),
            source_license: "project invented".into(), contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering controls".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "independently refuted reference".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}
fn run(request: &Value, mode: PreviewSolverMode) -> (Vec<Value>, Option<Value>) {
    let captured = serde_json::to_value(
        run_linear_static_preview_value_with_mode(request.clone(), mode).unwrap()).unwrap();
    let typed = serde_json::to_value(run_linear_static_preview_with_mode(
        serde_json::from_value(request.clone()).unwrap(), mode)).unwrap();
    let headless = run_preview_model_value_with_mode(
        runner_request(&request["model"]), request.clone(), mode).unwrap();
    let raw = serde_json::to_value(headless.mechanics_envelope.unwrap()).unwrap();
    for other in [&typed, &raw] {
        assert_eq!(captured["status"], other["status"]);
        assert_eq!(captured["results"], other["results"]);
        assert_eq!(captured["contract_evidence"], other["contract_evidence"]);
    }
    (vec![captured, typed, raw], headless.result_envelope_document)
}
fn number(v: &Value) -> f64 { v["value"].as_f64().unwrap() }
fn close(observed: f64, expected: &Value, floor: f64, oracle: &Value, label: &str) {
    let expected = number(expected);
    let relative = oracle["criterion"]["relative"].as_f64().unwrap();
    assert!(observed.is_finite() && (observed-expected).abs() <= relative*expected.abs().max(floor),
        "{label}: observed={observed:.17e}, expected={expected:.17e}");
}
fn rows<'a>(source: &'a Value, kind: &str, entity: &str, component: &str, location: Option<&str>) -> Vec<&'a Value> {
    source["results"].as_array().unwrap().iter().filter(|row|
        row["kind"] == kind && row["entity_ref"] == entity && row["metadata"]["component"] == component
        && location.is_none_or(|s| row["metadata"]["location"] == s)).collect()
}
fn row(source: &Value, kind: &str, entity: &str, component: &str, location: Option<&str>) -> f64 {
    let found = rows(source, kind, entity, component, location);
    assert_eq!(found.len(),1,"{kind} {entity} {component} {location:?}");
    let value = found[0]["value"].as_f64().unwrap();
    match found[0]["unit"].as_str().unwrap() {
        "mm" => value/1000.0, "m" | "rad" | "N" | "N*m" | "Pa" => value,
        unit => panic!("unexpected published unit {unit}"),
    }
}
fn assert_values(source: &Value, request: &Value, case: &Value, oracle: &Value, label: &str) {
    assert_eq!(source["status"]["mechanics"], "MECHANICS_SOLVED", "{label}: {}", source["diagnostics"]);
    assert_eq!(source["producer"]["semantic_contract_id"], PRESSURE_ID);
    assert!(source["diagnostics"].as_array().unwrap().iter().all(|d| d["severity"]!="blocking"));
    let floor = |name: &str| oracle["criterion"]["floors"][name].as_f64().unwrap();
    let axis = |i: usize| number(&case["axis_global"][i]);
    let displacement = |node: &str, i: usize| row(source,
        &format!("global_nodal_displacement_{}", ["x","y","z"][i]), node,
        &format!("nodal_displacement_{}", ["x","y","z"][i]), None);
    for (node, values) in case["nodes"].as_object().unwrap() {
        let mut axial = 0.0;
        for (i,a) in ["x","y","z"].iter().enumerate() {
            let value=displacement(node,i); axial+=value*axis(i);
            close(value,&values["displacement_global_m"][i],floor("length_m"),oracle,label);
            close(row(source,&format!("global_nodal_rotation_{a}"),node,&format!("nodal_rotation_{a}"),None),
                &values["rotation_global_rad"][i],floor("rotation_rad"),oracle,label);
        }
        close(axial,&values["axial_displacement_m"],floor("length_m"),oracle,label);
    }
    for (support, values) in case["reactions_support_on_model"].as_object().unwrap() {
        for (i,component) in ["Fx","Fy","Fz","Mx","My","Mz"].iter().enumerate() {
            close(row(source,"support_reaction_component_v2",support,component,None),
                &values[if i<3 {"force_global_n"} else {"moment_global_nm"}][i%3],
                floor(if i<3 {"force_n"} else {"moment_nm"}),oracle,label);
        }
    }
    let connector=case["connector"].as_str().unwrap();
    for (family, values) in case["connector_rows"].as_object().unwrap() {
        let (kind, location)=family.split_once(':').unwrap_or((family.as_str(),"connector_local"));
        let (prefix,quantity,count)=match kind {
            "connector_generalized_translation_v1"=>("qt_","length_m",3),
            "connector_generalized_rotation_v1"=>("qr_","rotation_rad",3),
            "connector_generalized_force_v1"=>("gt_","force_n",3),
            "connector_generalized_moment_v1"=>("gr_","moment_nm",3),
            "connector_endpoint_force_v1"=>("F","force_n",3),
            "connector_endpoint_moment_v1"=>("M","moment_nm",3),
            "connector_pressure_force_v1"=>("pe_","force_n",1),
            "connector_effective_axial_force_v1"=>("s_","force_n",1),
            other=>panic!("unknown oracle family {other}"),
        };
        for (i,a) in ["x","y","z"].iter().take(count).enumerate() {
            close(row(source,kind,connector,&format!("{prefix}{a}"),Some(location)),
                &values[i],floor(quantity),oracle,label);
        }
        if count==1 {
            // Oracle three-array padding is not three public scalar signatures.
            assert_eq!(number(&values[1]),0.0); assert_eq!(number(&values[2]),0.0);
            assert_eq!(source["results"].as_array().unwrap().iter().filter(|r|r["kind"]==kind).count(),1);
        }
    }
    let model=&request["model"];
    let record=&source["contract_evidence"]["connector"][0];
    let qt=row(source,"connector_generalized_translation_v1",connector,"qt_x",Some("connector_local"));
    close(qt,&case["scalars"]["qt_x_minus_q_ref_m"],floor("length_m"),oracle,label);
    close(qt+record["q_ref"][0].as_f64().unwrap(),&case["scalars"]["q_m"],floor("length_m"),oracle,label);
    close(row(source,"connector_generalized_force_v1",connector,"gt_x",Some("connector_local")),
        &case["scalars"]["Tb_gt_x_n"],floor("force_n"),oracle,label);
    if case["scalars"].get("Pe_n").is_some() {
        close(row(source,"connector_pressure_force_v1",connector,"pe_x",Some("connector_local")),
            &case["scalars"]["Pe_n"],floor("force_n"),oracle,label);
        close(row(source,"connector_effective_axial_force_v1",connector,"s_x",Some("connector_local")),
            &case["scalars"]["S_joint_n"],floor("force_n"),oracle,label);
    }
    let mut joint_edges=Vec::new();
    for region in source["contract_evidence"]["pressure"].as_array().unwrap() {
        for geometry in region["geometry"].as_array().unwrap() {
            // T4-U6 (D-5, §2.3): Ai is the pressure bore's.
            close(geometry["pressure_bore"]["Ai_m2"].as_f64().unwrap(),&case["scalars"]["Ai_m2"],0.0,oracle,label);
        }
        for load in region["applied_loads"].as_array().unwrap() {
            // Both pairs are the member's applied local RHS at [end i, end j]
            // (pressure_exact.rs `cap_pair`, `eigenload_pair`): caps [-Pi, +Pi]
            // and the Poisson pair [+h, -h]. The oracle's h_2nuPi_n is the
            // scalar h = 2*nu*Pi, applied +h at a member's first node and -h
            // at its second (u5a_reference.py `build`), so end j carries -h.
            let pi=number(&case["scalars"]["Pi_n"]); let h=number(&case["scalars"]["h_2nuPi_n"]);
            for (key,pair) in [("mathematical_cap_pair_local_n",[-pi,pi]),("eigenload_pair_local_n",[h,-h])] {
                for (end,expected) in pair.into_iter().enumerate() {
                    close(load[key][end].as_f64().unwrap(),&json!({"value":expected}),floor("force_n"),oracle,
                        &format!("{label} {} {key}[{end}]",load["pipe_id"]));
                }
            }
        }
        if let Some(edges)=region.get("joint_edges") {
            joint_edges.extend(edges.as_array().expect("joint_edges must be an array"));
        }
    }
    assert_eq!(joint_edges.len(),usize::from(case["connector_in_region"].as_bool().unwrap()),
        "{label}: participating joint evidence coverage");
    for edge in joint_edges {
        assert_eq!(edge["connector_id"],connector,"{label}: participating connector identity");
        assert_eq!(edge["node_i_ref"],record["node_i"]);
        assert_eq!(edge["node_j_ref"],record["node_j"]);
        close(edge["joint_pressure_force_n"].as_f64().unwrap(),&case["scalars"]["Pe_n"],
            floor("force_n"),oracle,label);
        let differences=edge["area_difference_force_n"].as_array().unwrap();
        assert_eq!(differences.len(),2);
        for difference in differences {
            close(difference.as_f64().unwrap(),&case["scalars"]["joint_area_difference_force_n"],
                floor("force_n"),oracle,label);
        }
    }
    for (pipe, values) in case["pipes"].as_object().unwrap() {
        let p=model["pipe_segments"].as_array().unwrap().iter().find(|p|p["id"]==*pipe).unwrap();
        let from=p["from"].as_str().unwrap(); let to=p["to"].as_str().unwrap();
        let position=|id: &str,i: usize| model["nodes"].as_array().unwrap().iter()
            .find(|n|n["id"]==id).unwrap()["position"][["x","y","z"][i]].as_f64().unwrap();
        let chord:[f64;3]=std::array::from_fn(|i|position(to,i)-position(from,i));
        let length=chord.iter().map(|x|x*x).sum::<f64>().sqrt();
        let elongation=(0..3).map(|i|(displacement(to,i)-displacement(from,i))*chord[i]/length).sum();
        close(elongation,&values["elongation_m"],floor("length_m"),oracle,label);
        let pressurized=model["load_cases"][0]["pressure_regions"].as_array().unwrap().iter()
            .any(|r|r["member_pipe_ids"].as_array().unwrap().iter().any(|id|id==pipe));
        let wall=if pressurized {row(source,"pipe_wall_axial_force_v2",pipe,"wall_axial_force",Some("midspan"))}
            else {row(source,"element_local_axial_force",pipe,"axial_force",Some("end_j"))};
        let effective=if pressurized {row(source,"pipe_effective_axial_force_v2",pipe,"effective_axial_force",Some("midspan"))} else {wall};
        close(wall,&values["Nw_n"],floor("force_n"),oracle,label);
        close(effective,&values["S_n"],floor("force_n"),oracle,label);
    }
    if let Some(expected)=case["scalars"].get("outer_end_separation_change_m") {
        let nodes=model["nodes"].as_array().unwrap();
        let first=nodes.first().unwrap()["id"].as_str().unwrap();
        let last=nodes.last().unwrap()["id"].as_str().unwrap();
        close((0..3).map(|i|(displacement(last,i)-displacement(first,i))*axis(i)).sum(),
            expected,floor("length_m"),oracle,label);
    }
    let assembly=&source["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"];
    let nodes=assembly["node_order"].as_array().unwrap();
    let node_ids:BTreeSet<&str>=nodes.iter().map(|n|n.as_str().unwrap()).collect();
    let expected:BTreeSet<&str>=case["pressure_rhs"]["global_n"].as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(nodes.len(),expected.len(),"{label}: unique complete RHS node order");
    assert_eq!(node_ids,expected,"{label}: complete oracle RHS node coverage");
    for key in ["assembled_pressure_rhs_global","rounded_cap_rhs_global","rounded_poisson_rhs_global"] {
        let values=assembly[key].as_array().unwrap();
        assert_eq!(values.len(),6*nodes.len(),"{label}: complete {key}");
        assert!(values.iter().all(|v|v.as_f64().is_some_and(f64::is_finite)));
    }
    for (i,node) in nodes.iter().enumerate() {
        let node=node.as_str().unwrap();
        for a in 0..3 {
            close(assembly["assembled_pressure_rhs_global"][6*i+a].as_f64().unwrap(),
                &case["pressure_rhs"]["global_n"][node][a],floor("force_n"),oracle,label);
            assert_eq!(assembly["assembled_pressure_rhs_global"][6*i+3+a].as_f64().unwrap(),0.0);
        }
    }
    let replaced=model["components"][0]["objective_connector"]["topology"]["span_ref"].as_str().unwrap();
    assert!(source["results"].as_array().unwrap().iter().all(|r|r["entity_ref"]!=replaced));
}
fn patched(base: &Value, operations: &Value) -> Value {
    let mut document=base.clone();
    for op in operations.as_array().unwrap() {
        let (parent,key)=op["path"].as_str().unwrap().rsplit_once('/').unwrap();
        let target=document.pointer_mut(parent).unwrap();
        match op["op"].as_str().unwrap() {
            "remove"=>{target.as_object_mut().unwrap().remove(key).unwrap();}
            "add" if key=="-"=>target.as_array_mut().unwrap().push(op["value"].clone()),
            "add" | "replace"=>{if let Some(a)=target.as_array_mut() {a[key.parse::<usize>().unwrap()]=op["value"].clone();}
                else {target[key]=op["value"].clone();}}
            op=>panic!("unsupported reference patch {op}"),
        }
    }
    document
}
fn blocking(source: &Value) -> BTreeSet<String> {
    source["diagnostics"].as_array().unwrap().iter().filter(|d|d["severity"]=="blocking")
        .map(|d|d["code"].as_str().unwrap().into()).collect()
}
#[test]
fn u5_nineteen_oracles_match_actual_public_typed_and_headless_consumers_in_both_modes() {
    let oracle=references(); assert_eq!(oracle["cases"].as_object().unwrap().len(),19);
    for (name,case) in oracle["cases"].as_object().unwrap() {
        let request=payload(case);
        for mode in MODES {
            let (sources,document)=run(&request,mode);
            for source in sources {assert_values(&source,&request,case,&oracle,&format!("{name} {mode:?}"));}
            assert!(document.is_some(),"{name} {mode:?}: current pressure export missing");
        }
    }
}
#[test]
fn u5_seventeen_refusal_variants_keep_exact_codes_refs_and_no_export_in_both_modes() {
    let oracle=references(); let base=payload(&oracle["cases"]["J3-1"]);
    assert_eq!(oracle["refusals"].as_object().unwrap().len(),17);
    for (name,variant) in oracle["refusals"].as_object().unwrap() {
        let request=patched(&base,&variant["operations"]);
        let mut expected:BTreeSet<String>=variant["expected"]["blocking_codes_exactly"].as_array().unwrap()
            .iter().map(|v|v.as_str().unwrap().into()).collect();
        // T4-U5b supersedes RV-11 by name (validation/references/t4_u5a/README.md,
        // "RV-11 supersession"; DEL-04-07 Design u4b-u5b-implementation.md §3.1):
        // explicit_axial_rods_v1 is defined, and its empty rods list is refused
        // as JOINT_TIE_ROD_INPUT_INVALID. The frozen patch and expectation are
        // unchanged; every other variant keeps its frozen code set.
        if name=="RV-11" {
            assert_eq!(expected,BTreeSet::from(["JOINT_HARDWARE_LAW_UNSUPPORTED".to_string()]),"RV-11's frozen expectation");
            expected=BTreeSet::from(["JOINT_TIE_ROD_INPUT_INVALID".to_string()]);
        }
        for mode in MODES {
            let (sources,document)=run(&request,mode);
            for source in sources {
                assert_eq!(source["status"]["mechanics"],"MODEL_INCOMPLETE","{name} {mode:?}");
                assert_eq!(blocking(&source),expected,"{name} {mode:?}");
                assert_eq!(source["results"],json!([]),"{name} {mode:?}");
                let refs:BTreeSet<&str>=source["diagnostics"].as_array().unwrap().iter()
                    .filter(|d|d["severity"]=="blocking").flat_map(|d|d["affected_refs"].as_array().unwrap())
                    .map(|v|v.as_str().unwrap()).collect();
                for expected in variant["expected"]["affected_refs_include"].as_array().unwrap() {
                    assert!(refs.contains(expected.as_str().unwrap()),"{name} {mode:?}: ref {expected}");
                }
            }
            assert!(document.is_none(),"{name} {mode:?}: refused request exported");
        }
    }
}

#[test]
fn u5_current_closed_area_and_region_ownership_counterfactuals_fail_closed() {
    let oracle=references(); let base=payload(&oracle["cases"]["J3-1"]);
    // (request, blocking codes exactly, regions every joint-pressure finding set must name)
    let mut variants:Vec<(Value,&[&str],&[&str])>=Vec::new();
    for area in [json!({"value":-1.0,"unit":"m^2"}), json!({"value":1.0,"unit":"N"}),
        json!({"value":null,"unit":"m^2"}), json!({"value":0.004,"unit":"m^2","extra":true})] {
        let mut request=base.clone();
        request["model"]["components"][0]["objective_connector"]["pressure_model"]["effective_area"]=area;
        variants.push((request,&["JOINT_PRESSURE_AREA_MISSING"],&[]));
    }
    let mut request=base.clone();
    request["model"]["components"][0]["objective_connector"]["pressure_model"]["extra"]=Value::Null;
    variants.push((request,&["JOINT_PRESSURE_INTERFACE_UNRESOLVED"],&[]));
    let mut request=base.clone();
    request["model"]["load_cases"][0]["pressure_regions"][0]["member_connector_ids"]
        .as_array_mut().unwrap().push(json!("component:C-J"));
    variants.push((request,&["JOINT_PRESSURE_INTERFACE_UNRESOLVED"],&[]));
    let mut request=base.clone();
    let mut second=request["model"]["load_cases"][0]["pressure_regions"][0].clone();
    second["id"]=json!("region:second");
    request["model"]["load_cases"][0]["pressure_regions"].as_array_mut().unwrap().push(second);
    // The copied region also relists pipes L and R, which the region input
    // check independently and correctly refuses (PRESSURE_REGION_PIPE_OVERLAP);
    // J3-1 has no other pipe for a second region. The ownership finding names
    // both listing regions (a touching, unlisting region would name only one).
    variants.push((request,&["JOINT_PRESSURE_INTERFACE_UNRESOLVED","PRESSURE_REGION_PIPE_OVERLAP"],
        &["region:U5A","region:second"]));
    for (request,codes,regions) in variants {
        for mode in MODES {
            let (sources,document)=run(&request,mode);
            for source in sources {
                assert_eq!(blocking(&source),codes.iter().map(|c|c.to_string()).collect(),"{mode:?}");
                assert_eq!(source["status"]["mechanics"],"MODEL_INCOMPLETE");
                assert_eq!(source["results"],json!([]));
                let joint_refs:BTreeSet<&str>=source["diagnostics"].as_array().unwrap().iter()
                    .filter(|d|d["code"]=="JOINT_PRESSURE_INTERFACE_UNRESOLVED")
                    .flat_map(|d|d["affected_refs"].as_array().unwrap()).map(|v|v.as_str().unwrap()).collect();
                for region in regions {
                    assert!(joint_refs.contains(region),"{mode:?}: joint finding for {region}");
                }
            }
            assert!(document.is_none());
        }
    }
    // JSON has no nonfinite-number form: its actual shared text input refuses
    // an out-of-range area before the model finding; do not turn it into zero.
    let text=serde_json::to_string(&base).unwrap();
    let text=text.replace("\"effective_area\":{\"unit\":\"m^2\",\"value\":0.004}",
        "\"effective_area\":{\"unit\":\"m^2\",\"value\":1e400}");
    assert!(text.contains("1e400"));
    assert!(serde_json::from_str::<Value>(&text).is_err());
}
