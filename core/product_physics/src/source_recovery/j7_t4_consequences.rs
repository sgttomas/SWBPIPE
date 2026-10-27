//! J7's genuine admitted connector Input: the named connector-family decline at work 64
//! (T4 batch 4's D7 order; formerly SourceMismatch64), with no invented frames.
use super::*;
use crate::*;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
#[path="../../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod corpus_storage;
fn sha(bytes:&[u8])->String{format!("{:x}",Sha256::digest(bytes))}
fn request()->Value{
    let bytes=include_bytes!("j7_i12_request.json");
    assert_eq!(sha(bytes),"2d8d8ed78a4c8a799c8247944ec63aa78246be780ce2eece89757ca08053e0f6");
    let control=corpus_storage::expand(serde_json::from_slice(bytes).unwrap());
    assert_eq!(control["reference_sha256"],"3d6f547dacac9c47d88670197bc7cc83813667331e27f7930523f6687fe3f063");
    assert_eq!(control["recipe_source_sha256"],"a52c2cc283c9ac3a81d67f6bf58306775b1f26914a4a830ebf7c11dc559c2231");
    control["request"].clone()
}
#[test]
fn j7_t4_i12_genuine_nodal_case_declines_the_connector_family_by_name(){
    let stored=request();
    assert_eq!(sha(serde_json::to_string(&stored).unwrap().as_bytes()),"99401eec19e6f5524fb1fd9e0f534c35fe07e1df384b15ec4ec4e92a0d0c527a");
    // B7b: the stored 0.3.0 control is unchanged; only its copy is re-authored to
    // the current 0.8.0 exact document. The captured invocation digests below are
    // those of the re-authored copy.
    let raw=crate::b7b_admission_tests::reauthor_current_request(stored);
    assert_eq!(raw["model"]["load_cases"].as_array().unwrap().len(),3);
    for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny]{
        let (request,captured)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let expected=match mode{PreviewSolverMode::SparseInteractive=>"b219c09824e77d2b8ffaa9b0ff8076ab8ea423c010a2ff660ac560539387e82d",PreviewSolverMode::DenseScrutiny=>"bfc60747f5e5031e94fc921c631c4250455025eb9ad59638a7c58dee301d61f0"};
        assert_eq!(captured.borrowed_digest(),expected);
        let mut model=request.model;let mut materials=model.materials.clone();let mut diagnostics=Vec::new();
        pressure_runtime::validate_profile(&model,&mut diagnostics);
        validate_model_inputs(&model,&materials,&mut diagnostics);validate_support_family_tokens(&model,&mut diagnostics);
        resolve_shared_sections(&mut model,&mut diagnostics);normalize_model_units(&mut model,&mut materials,&mut diagnostics);
        pressure_material::resolve_base(&model,&mut materials,&mut diagnostics);
        assert!(!has_blocking(&diagnostics),"actual admission/material normalization: {diagnostics:?}");
        let built=build_model_for_members(&model,&materials,None,&mut diagnostics).unwrap();
        assert!(!has_blocking(&diagnostics),"actual built source: {diagnostics:?}");model.materials=materials;
        assert_eq!((built.pipes.len(),built.frame_elements.len(),built.connectors.len()),(4,3,1));
        let case=model.load_cases.iter().find(|c|c.id=="load:L-300").unwrap();
        assert_eq!(raw["model"]["load_cases"][2]["id"],case.id);
        assert!(raw["model"]["load_cases"][2]["primitive_loads"].as_array().unwrap().iter().all(|p|matches!(p["dimension"].as_str(),Some("force"|"moment"))));
        let boundary=prepare_boundary(built.nodes.len(),&built.supports);assert!(boundary.findings.is_empty());
        let mut stiffness=assemble_global_stiffness_with_connectors(built.nodes.len(),&built.frame_elements,&built.connectors).unwrap();
        for spring in &boundary.springs{let d=spring.node_dof.global_index();stiffness[d][d]+=spring.stiffness.value;}
        let primitives=build_load_case_primitive_loads(&model,case,&mut diagnostics);
        let application=prepare_loads(built.nodes.len(),built.pipes.len(),&primitives);assert!(!application.is_blocked()&&!has_blocking(&diagnostics));
        assert_eq!(application.nodal_loads.len(),3);
        let force=nodal_and_eigen_case_force(&application,&[],&built).unwrap();
        let prescribed:Vec<_>=boundary.restrained_dofs.iter().map(|&d|(d,0.0)).collect();
        let free:Vec<_>=(0..force.len()).filter(|d|!prescribed.iter().any(|(p,_)|p==d)).collect();
        assert_eq!((free.len(),prescribed.len()),(21,9));
        let result=solve_ordinary(Input{model:&model,built:&built,stiffness:&stiffness,force:&force,free:&free,prescribed:&prescribed,spring_entries:&boundary.springs,load_case:case,load_application:&application,thermal_loads:&[],load_state:None},exact::Limits{operations:PHYSICS_SOURCE_WORK_LIMIT,..Default::default()},ForceScale::UNSCALED);
        let failure=result.expect_err("actual valid replaced span source is declined by family before the frame-count check");
        eprintln!("J7_T4_I12_SOURCE {}",json!({"request_native_json_sha256":sha(serde_json::to_string(&raw).unwrap().as_bytes()),"captured_invocation_sha256":captured.borrowed_digest(),"mode":mode.as_str(),"case_id":case.id,"case_ordinal":2,"pipes":built.pipes.len(),"frames":built.frame_elements.len(),"connectors":built.connectors.len(),"load_occurrences":application.nodal_loads.len(),"free":free,"prescribed":prescribed,"outcome":format!("{failure:?}"),"scope":"genuine normalized/build/boundary/load-owned private method call; no ordinary observations, public selection or source standing manufactured"}));
        assert_eq!(failure.stage,"source closure");assert_eq!(failure.helper_stage,AttemptStage::SourceClosure);
        // T4 batch 4's D7 decline (#1314): the connector family is named before the
        // dimension closure that formerly failed as SourceMismatch at the same work.
        assert_eq!(failure.error,RecoveryError::Unsupported("objective connector is not a straight source family"));
        assert_eq!(failure.work,exact::WorkReport::new(64,0,PHYSICS_SOURCE_WORK_LIMIT));
    }
}
