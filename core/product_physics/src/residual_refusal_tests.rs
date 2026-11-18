//! DEL-04-01 AC-012 and VER-007, publication-helper coverage: supplying the
//! kernel's REQ-014 refusal ("original-equation residual failed after bounded
//! refinement") to append_integrity_failure creates the named, blocking
//! integrity finding. The constructed kernel systems reach that refusal
//! through each finish. This test supplies the error directly; it does not
//! exercise a product solve or subsequent retained-source selection, which
//! can demote a rejected ordinary attempt's finding to informational.

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
    append_integrity_failure(&mut diagnostics, &mut crate::integrity::InverseNormSink::default(), "load:WITNESS", &error, &model);
    assert_eq!(diagnostics.len(), 1);
    let finding = &diagnostics[0];
    assert_eq!(finding.id, integrity_diagnostic_id("load:WITNESS"));
    assert_eq!(finding.code, "NUMERICAL_INTEGRITY_UNRESOLVED");
    assert_eq!(finding.severity, "blocking");
    assert!(finding.message.contains("original-equation residual failed after bounded refinement"), "{}", finding.message);
    assert_eq!(finding.affected_refs, vec!["load:WITNESS".to_string()]);
}
