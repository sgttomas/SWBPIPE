// Evidence-only current public-product witness. No configurable convergence cap.
use open_pipe_stress_headless_runner::{run_preview_in_memory, result_envelope_binding::build_result_export_document, RunnerRequest};
use open_pipe_stress_product_physics::LinearStaticPreviewRequest;
use serde_json::{json, Value};
use std::{env,fs,path::Path};

fn write(path: &Path, value: &impl serde::Serialize) {
    fs::write(path, format!("{}\n",serde_json::to_string_pretty(value).unwrap())).unwrap();
}
fn main() {
    let args:Vec<_>=env::args_os().collect(); assert_eq!(args.len(),3);
    let input=Path::new(&args[1]); let out=Path::new(&args[2]); fs::create_dir_all(out).unwrap();
    let suite:Value=serde_json::from_slice(&fs::read(input.join("CASE_INPUTS_V1.json")).unwrap()).unwrap();
    let mut records=Vec::new(); let mut failed=false;
    for case in suite["cases"].as_array().unwrap() {
        let id=case["case_id"].as_str().unwrap();
        let model:Value=serde_json::from_slice(&fs::read(input.join(case["model_file"].as_str().unwrap())).unwrap()).unwrap();
        let request:RunnerRequest=serde_json::from_slice(&fs::read(input.join(case["request_file"].as_str().unwrap())).unwrap()).unwrap();
        let preview=LinearStaticPreviewRequest{model:serde_json::from_value(model.clone()).unwrap(),materials:vec![]};
        write(&out.join(format!("{id}.model.json")),&model);
        write(&out.join(format!("{id}.request.json")),&request);
        let output=run_preview_in_memory(request.clone(),preview);
        write(&out.join(format!("{id}.public_output.json")),&output);
        write(&out.join(format!("{id}.runner.json")),&output.runner_result);
        let mechanics=output.mechanics_envelope.as_ref().expect("public raw mechanics must be present");
        write(&out.join(format!("{id}.mechanics.json")),mechanics);
        let codes:Vec<_>=mechanics.diagnostics.iter().map(|d|d.code.as_str()).collect();
        let library=build_result_export_document(&request,&output.runner_result,mechanics);
        match &library {
            Ok(doc)=>write(&out.join(format!("{id}.library_document.json")),doc),
            Err(error)=>write(&out.join(format!("{id}.library_error.json")),error),
        }
        if let Some(doc)=&output.result_envelope_document {write(&out.join(format!("{id}.attached_document.json")),doc);}
        else {write(&out.join(format!("{id}.canonical_absence.json")),&json!({"canonical_solved_export":false,"producer_mechanics_status":mechanics.status.mechanics,"route":"run_preview_in_memory_attached_default_sparse"}));}
        let witness=case["expectation"].as_str()==Some("nonconverged_public_absence");
        let passed=if witness {
            mechanics.status.mechanics=="MODEL_INCOMPLETE" && mechanics.results.is_empty()
            && codes.contains(&"NONLINEAR_SUPPORT_NONCONVERGENCE")
            && codes.contains(&"NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED")
            && codes.contains(&"SOLVER_SYSTEM_BLOCKED")
            && !codes.contains(&"NONLINEAR_SUPPORT_LOOP_CONVERGED")
            && mechanics.diagnostics.iter().any(|d| d.code=="NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED" && d.message.contains("completed 4 iteration(s); final residual count 1;"))
            && output.result_envelope_document.is_none()
        } else {
            mechanics.status.mechanics=="MECHANICS_SOLVED" && codes.contains(&"NONLINEAR_SUPPORT_LOOP_CONVERGED")
            && !codes.contains(&"NONLINEAR_SUPPORT_NONCONVERGENCE")
            && output.result_envelope_document.is_some()
            && library.as_ref().ok()==output.result_envelope_document.as_ref()
            && mechanics.results.iter().any(|r| r.kind=="nonlinear_support_active_set_iteration_count" && r.value==case["expected_iterations"].as_f64().unwrap())
        };
        failed|=!passed;
        records.push(json!({"case_id":id,"passed":passed,"mechanics_status":mechanics.status.mechanics,"raw_row_count":mechanics.results.len(),"diagnostic_codes":codes,"attached_document_present":output.result_envelope_document.is_some(),"library_document_present":library.is_ok()}));
    }
    let verdict=json!({"verdict":if failed{"WITNESS_EXPECTATION_FAILED"}else{"PUBLIC_PRODUCT_WITNESS_AND_CONTROLS_PASSED"},"cases":records});
    write(&out.join("WITNESS_RETURN_V1.json"),&verdict); println!("{}",serde_json::to_string_pretty(&verdict).unwrap());
    if failed {std::process::exit(1);}
}
