use open_pipe_stress_headless_runner::{
    run_preview_model_value_with_mode, run_preview_model_value_with_retained_headless,
    PrivacyContext, ProfessionalBoundary, Provenance, RedistributionStatus, Reference,
    RunnerOperation, RunnerRequest, TbdDecisions,
};
use open_pipe_stress_product_physics::{
    run_linear_static_preview_value_with_mode as run_value_with_mode,
    run_linear_static_preview_value_with_retained_direct, MissingAdmissionTerm,
    PreviewSolverMode, ProfileStatus, RetainedCaller,
};
use serde_json::Value;

fn request(model: &Value) -> RunnerRequest {
    RunnerRequest {
        request_id: "actual-composite-headless-consumer".into(),
        operation: RunnerOperation::Solve,
        operation_ref: Reference::new("api_operation", "ops.solve.job"),
        project_ref: Reference::new("project", model["project"]["id"].as_str().unwrap()),
        model_ref: Reference::new("model", model["project"]["id"].as_str().unwrap()),
        unit_system_ref: Reference::new("unit_system", "invented-si"),
        load_basis_refs: model["load_cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| Reference::new("load_case", c["id"].as_str().unwrap()))
            .collect(),
        input_manifest_ref: Reference::new("audit_manifest", "actual-composite-input-manifest"),
        requested_outputs: vec![
            "result_envelope".into(),
            "audit_manifest".into(),
            "diagnostics".into(),
        ],
        privacy: PrivacyContext::local_first_public_metadata(),
        provenance: Provenance {
            source_name: "invented composite consumer fixture".into(),
            source_location: "fixtures/product_preview/physics_source".into(),
            source_license: "project invented".into(),
            contributor: "OpenPipeStress".into(),
            contributor_certification: "invented non-engineering example".into(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "pending".into(),
        },
        professional_boundary: ProfessionalBoundary::project_default(),
        tbd_decisions: TbdDecisions::d33_local_cli_policy(),
    }
}

fn ordinary() -> Value {
    serde_json::from_str(include_str!(
        "../../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
    ))
    .unwrap()
}
#[test]
fn explicit_headless_refusal_preserves_output_and_completion_fields_both_modes() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let raw = ordinary();
        let metadata = request(&raw["model"]);
        let expected =
            run_preview_model_value_with_mode(metadata.clone(), raw.clone(), mode).unwrap();
        let actual =
            run_preview_model_value_with_retained_headless(metadata, raw, mode, None).unwrap();
        assert_eq!(
            serde_json::to_vec(actual.output()).unwrap(),
            serde_json::to_vec(&expected).unwrap()
        );
        assert_eq!(
            actual.output().result_envelope_document,
            expected.result_envelope_document
        );
        assert_eq!(
            actual.output().canonical_export_unavailability,
            expected.canonical_export_unavailability
        );
        assert_eq!(
            actual.output().qualified_preview_evidence.is_some(),
            expected.qualified_preview_evidence.is_some()
        );
        let report = actual.admission().unwrap();
        assert_eq!(report.caller, RetainedCaller::Headless);
        // G6 registration (RV89 S-1): a profile is registered, so the report states this PP
        // build's own status: `Registered` in the qualified build, `Stale` in any other;
        // never `Missing`. Headless itself stays refused (D1.0): its output is the ordinary
        // run's (above). The oracle is the Direct entry on an input that D1 refuses after the
        // build clause, a second load case (D1.4), so this workspace, whose lock is not PP's
        // reviewed one, is never granted a permit and never runs W1 (RV89 G6r N-1).
        let mut refused = ordinary();
        let case = refused["model"]["load_cases"][0].clone();
        refused["model"]["load_cases"]
            .as_array_mut()
            .unwrap()
            .push(case);
        let plain = run_value_with_mode(refused.clone(), mode).unwrap();
        let direct = run_linear_static_preview_value_with_retained_direct(refused, mode).unwrap();
        assert_eq!(
            serde_json::to_vec(direct.envelope()).unwrap(),
            serde_json::to_vec(&plain).unwrap(),
            "no permit: the ordinary bytes"
        );
        assert!(direct.successor().is_none());
        let built = direct.admission().unwrap();
        assert_eq!(built.typed.load_cases.length, 2, "outside D1 (D1.4)");
        assert!(matches!(
            built.profile,
            ProfileStatus::Registered | ProfileStatus::Stale
        ));
        assert_eq!(report.profile, built.profile);
        assert!(report.headless.is_some());
        assert!(!report.headless.unwrap().payload_and_invocation_alias);
        assert!(report
            .required_unknown_terms()
            .contains(&MissingAdmissionTerm::CallerCompletion));
    }
}
#[test]
fn runner_validation_refusal_never_enters_producer_admission() {
    let raw = ordinary();
    let mut metadata = request(&raw["model"]);
    metadata.request_id.clear();
    let expected = run_preview_model_value_with_mode(
        metadata.clone(),
        raw.clone(),
        PreviewSolverMode::SparseInteractive,
    )
    .unwrap();
    let actual = run_preview_model_value_with_retained_headless(
        metadata,
        raw,
        PreviewSolverMode::SparseInteractive,
        None,
    )
    .unwrap();
    assert!(actual.admission().is_none());
    assert!(actual.output().mechanics_envelope.is_none());
    assert_eq!(
        serde_json::to_vec(actual.output()).unwrap(),
        serde_json::to_vec(&expected).unwrap()
    );
    assert_eq!(
        actual.output().canonical_export_unavailability,
        expected.canonical_export_unavailability
    );
}
#[test]
fn old_headless_and_typed_wrappers_stay_explicitly_ordinary() {
    let source = include_str!("../src/lib.rs");
    let begin = source.find("fn run_preview_model_value_mode(").unwrap();
    let end = source[begin..]
        .find("pub fn run_preview_model_value_with_retained_headless(")
        .unwrap()
        + begin;
    assert!(source[begin..end].contains("PreviewProducerEntry::Ordinary"));
    let begin = source.find("fn run_preview_in_memory_mode(").unwrap();
    let end = source[begin..]
        .find("fn run_preview_with_producer(")
        .unwrap()
        + begin;
    assert!(
        source[begin..end].contains("run_linear_static_preview_with_mode(preview_request, mode)")
    );
    assert!(!source[begin..end].contains("Retained"));
    assert!(!source.contains("with_retained_direct"));
}
