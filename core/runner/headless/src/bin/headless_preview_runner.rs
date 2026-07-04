//! PROVISIONAL headless preview-runner entrypoint (DEC-064 / D-32 O-A).
//!
//! Thin `[[bin]]` wrapper over the existing library surface: constructs a
//! schema-first `RunnerRequest` in code (the headless request types are
//! Serialize-only by design — no request-JSON parsing surface is added),
//! loads a `LinearStaticPreviewRequest` model from the caller-named fixture
//! path, executes the validated-kernel preview solve via
//! `run_preview_in_memory`, and emits the structured `PreviewRunnerOutput`
//! JSON on stdout.
//!
//! Command shape is PROVISIONAL: no final CLI syntax is claimed or settled
//! (DEL-10-05 Datasheet "Exact CLI command names" stays TBD; DEC-064
//! rider 1). read_only posture: the process reads the caller-named input and
//! writes only to stdout/stderr; it never mutates the filesystem, spawns
//! processes, or touches the network (`filesystem_mutation_policy` stays
//! deferred per `TbdDecisions::all_deferred`). Exit status: 0 only when the
//! run completed with no blocking diagnostics and the result validates
//! clean; 1 when the run carries blocking diagnostics or fails validation;
//! 2 on usage or input errors.

use std::process::ExitCode;

use open_pipe_stress_headless_runner::{
    run_preview_in_memory, validate_result, JobStateKind, PrivacyContext, ProfessionalBoundary,
    Provenance, RedistributionStatus, Reference, RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::LinearStaticPreviewRequest;

fn provisional_request(model_input_path: &str) -> RunnerRequest {
    RunnerRequest {
        request_id: "headless-preview-entrypoint-run".to_string(),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", "invented-project"),
        model_ref: Reference::new("model_fixture", model_input_path),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: vec![Reference::new("load_case", "LC1")],
        input_manifest_ref: Reference::new("audit_manifest", "manifest-entrypoint-1"),
        requested_outputs: vec![
            "result_envelope".to_string(),
            "audit_manifest".to_string(),
            "diagnostics".to_string(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented preview model fixture".to_string(),
            source_location: model_input_path.to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::all_deferred(),
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(model_path) = args.next() else {
        eprintln!(
            "usage (PROVISIONAL, not final CLI syntax): headless_preview_runner <preview-model.json>"
        );
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!(
            "usage (PROVISIONAL, not final CLI syntax): headless_preview_runner <preview-model.json>"
        );
        return ExitCode::from(2);
    }

    let raw = match std::fs::read_to_string(&model_path) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("headless_preview_runner: cannot read {model_path}: {error}");
            return ExitCode::from(2);
        }
    };
    let model = match serde_json::from_str(&raw) {
        Ok(model) => model,
        Err(error) => {
            eprintln!("headless_preview_runner: {model_path} is not a preview model: {error}");
            return ExitCode::from(2);
        }
    };
    let preview_request = LinearStaticPreviewRequest {
        model,
        materials: Vec::new(),
    };

    let output = run_preview_in_memory(provisional_request(&model_path), preview_request);
    let validation = validate_result(&output.runner_result);

    let rendered =
        serde_json::to_string_pretty(&output).expect("PreviewRunnerOutput always serializes");
    println!("{rendered}");

    let clean = output.runner_result.job.state == JobStateKind::Completed
        && output.runner_result.diagnostics.is_empty()
        && !validation.has_blocking_diagnostics();
    if clean {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::provisional_request;
    use open_pipe_stress_headless_runner::validate_request;

    #[test]
    fn provisional_request_is_schema_complete() {
        let validation = validate_request(&provisional_request(
            "fixtures/product_preview/invented_preview_model.json",
        ));
        assert!(!validation.has_blocking_diagnostics());
    }
}
