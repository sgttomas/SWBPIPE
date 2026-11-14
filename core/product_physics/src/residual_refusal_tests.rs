//! DEL-04-01 AC-012 and VER-007, at product level: the kernel's REQ-014 refusal
//! ("original-equation residual failed after bounded refinement", no solution)
//! is published as the case's named, blocking integrity finding. The kernel's own
//! test (`structural/residual_refusal_tests.rs`) forces that refusal through the
//! dense and the sparse finish; the seam is test-only in the kernel, so here the
//! kernel's error value is supplied directly to the publication helper.

use super::*;

#[test]
fn the_kernels_residual_refusal_is_published_as_the_named_blocking_finding() {
    let error = open_pipe_stress_frame_kernel::structural::StructuralError::NumericallyUnresolved {
        reason: "original-equation residual failed after bounded refinement",
        global_dof: None,
    };
    assert_eq!(integrity_failure_code(&error), "NUMERICAL_INTEGRITY_UNRESOLVED");
    let wire: serde_json::Value = serde_json::from_str(include_str!(
        "../../../validation/references/inverse_norm_witness/requests/linear.request.json"
    ))
    .unwrap();
    let model: PreviewModel = serde_json::from_value(wire["model"].clone()).unwrap();
    let mut diagnostics = Vec::new();
    append_integrity_failure(&mut diagnostics, "load:WITNESS", &error, &model);
    assert_eq!(diagnostics.len(), 1);
    let finding = &diagnostics[0];
    assert_eq!(finding.id, integrity_diagnostic_id("load:WITNESS"));
    assert_eq!(finding.code, "NUMERICAL_INTEGRITY_UNRESOLVED");
    assert_eq!(finding.severity, "blocking");
    assert!(finding.message.contains("original-equation residual failed after bounded refinement"), "{}", finding.message);
    assert_eq!(finding.affected_refs, vec!["load:WITNESS".to_string()]);
}
