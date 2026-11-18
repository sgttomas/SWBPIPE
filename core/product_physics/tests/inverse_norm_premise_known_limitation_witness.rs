//! The integrity check's inverse-norm gate on the known-limitation witness
//! (DEL-04-01 REQ-015, exit criterion 18). The check may use Q = 1024 times
//! the condition estimate as its inverse operand only when a certificate has
//! established that the scaled matrix's inverse norm is at most B ≤ Q
//! (`verified_inverse_operand` in the kernel's formation check). These
//! invented models have true scaled inverse norm 2¹⁸ against Q of about 1024,
//! so no certificate can verify Q there, and the product must not publish
//! checks passed or an established support state resting on it.
//!
//! The models and their exact truth are
//! `validation/references/inverse_norm_witness` (frozen; its README states
//! every number's reason), generated without product code. The requests run
//! here unchanged, through the product's ordinary entry (the one hanger
//! sizing's derived solves use). The file keeps the name that package's
//! README gives its consumer.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::Value;
use sha2::{Digest, Sha256};

const REFERENCES: &str =
    include_str!("../../../validation/references/inverse_norm_witness/references.json");
const LINEAR: &str =
    include_str!("../../../validation/references/inverse_norm_witness/requests/linear.request.json");
const ONE_WAY_NEGATIVE: &str = include_str!(
    "../../../validation/references/inverse_norm_witness/requests/one_way_negative.request.json"
);
const ONE_WAY_POSITIVE: &str = include_str!(
    "../../../validation/references/inverse_norm_witness/requests/one_way_positive.request.json"
);
const PASSED: &str = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
const SENSITIVE: &str = "NUMERICAL_INTEGRITY_SENSITIVE";
const ESTABLISHED: &str = "Converged support state: established at 128-bit precision";
const NOT_VERIFIED: &str = "inverse-norm bound not verified";
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive];

/// The variant's truth, after checking that the reference binds these request bytes.
fn truth(variant: &str, request: &str) -> Value {
    let references: Value = serde_json::from_str(REFERENCES).unwrap();
    let truth = references["variants"][variant].clone();
    assert_eq!(
        format!("{:x}", Sha256::digest(request.as_bytes())),
        truth["request_sha256"].as_str().unwrap(),
        "{variant}: the reference binds the request bytes"
    );
    truth
}

fn run(request: &str, mode: PreviewSolverMode) -> Value {
    let envelope =
        run_linear_static_preview_value_with_mode(serde_json::from_str(request).unwrap(), mode)
            .unwrap();
    serde_json::to_value(envelope).unwrap()
}

/// The one integrity diagnostic of the witness's one load case.
fn integrity(envelope: &Value) -> (String, String) {
    let found = envelope["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["code"].as_str().is_some_and(|c| c.starts_with("NUMERICAL_INTEGRITY_")))
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{found:?}");
    (found[0]["code"].as_str().unwrap().into(), found[0]["message"].as_str().unwrap().into())
}

fn blocking(envelope: &Value) -> Vec<&Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["severity"] == "blocking").collect()
}

/// The envelope's numerical quality and its one case's solve quality.
fn quality(envelope: &Value) -> (String, String) {
    let cases = envelope["numerical_quality"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 1, "{cases:?}");
    (
        envelope["numerical_quality"]["status"].as_str().unwrap().into(),
        cases[0]["solve_quality"].as_str().unwrap().into(),
    )
}

fn result(envelope: &Value, kind: &str, entity: &str) -> f64 {
    let found = envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["kind"] == kind && r["entity_ref"] == entity)
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity}");
    found[0]["value"].as_f64().unwrap()
}

/// The gate's disposition: exactly one integrity diagnostic, Sensitive, naming
/// the not-verified bound; Sensitive everywhere quality is published; nothing
/// blocking (the solve itself is published as a binary64 result).
fn assert_gated(envelope: &Value, at: &str) -> String {
    assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{at}");
    let (code, message) = integrity(envelope);
    assert_eq!(code, SENSITIVE, "{at}: {message}");
    assert!(message.contains(NOT_VERIFIED), "{at}: {message}");
    assert!(!message.contains(ESTABLISHED), "{at}: {message}");
    assert_eq!(quality(envelope), ("sensitive".into(), "sensitive".into()), "{at}");
    assert!(blocking(envelope).is_empty(), "{at}: {:?}", blocking(envelope));
    message
}

/// Linear variant, both modes, through the derived-solve entry: Q is not
/// verified (true norm 2¹⁸ against Q ≈ 1024), so the check is unavailable with
/// the stable detail and the case is Sensitive, never checks passed.
#[test]
fn witness_linear_case_is_sensitive_with_the_bound_not_verified_in_both_modes() {
    let truth = truth("linear", LINEAR);
    assert_eq!(truth["exact_displacement_B"]["UZ"]["exact"], "0");
    for mode in MODES {
        let envelope = run(LINEAR, mode);
        let message = assert_gated(&envelope, &format!("linear {mode:?}"));
        assert!(
            message.contains("formation_check: reason=formation_check_unavailable; detail=inverse-norm bound not verified"),
            "{mode:?}: {message}"
        );
    }
}

/// One-way variants, both modes. In each variant's witness mode the converged
/// state is the wrong (inactive) one: its establishment rests on Q at stage B,
/// so it is refused with the stage named and the case is Sensitive. In the
/// other mode the product reached the exact (active) state; whatever it
/// publishes there, it never publishes an established state other than the
/// exact one, and never checks passed resting on an unverified bound.
#[test]
fn witness_one_way_state_is_not_established_on_an_unverified_bound() {
    for (variant, request, witness_mode) in [
        ("one_way_negative", ONE_WAY_NEGATIVE, PreviewSolverMode::DenseScrutiny),
        ("one_way_positive", ONE_WAY_POSITIVE, PreviewSolverMode::SparseInteractive),
    ] {
        let truth = truth(variant, request);
        assert_eq!(truth["exact_state"], "active", "{variant}");
        assert_eq!(truth["inactive_state"]["admissible"], false, "{variant}");
        for mode in MODES {
            let at = format!("{variant} {mode:?}");
            let envelope = run(request, mode);
            assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{at}");
            let state = result(&envelope, "nonlinear_support_active_set_state_code", "support:STOP");
            let (code, message) = integrity(&envelope);
            if mode == witness_mode {
                assert_eq!(state, 0.0, "{at}: the computed state is the inactive one");
                let message = assert_gated(&envelope, &at);
                assert!(message.contains("not established"), "{at}: {message}");
                assert!(message.contains(&format!("formation check: stage B: {NOT_VERIFIED}")), "{at}: {message}");
            } else if code == PASSED || message.contains(ESTABLISHED) {
                assert!(state != 0.0, "{at}: an established state must be the exact, active one");
                assert!(!message.contains(NOT_VERIFIED), "{at}: {message}");
            }
        }
    }
}

/// Positive control: a maintained request whose bound the certificate verifies
/// still passes its checks in both modes, so the gate refuses only where the
/// bound is not verified (a gate that refused everything would fail here).
#[test]
fn a_verified_maintained_request_still_passes_its_checks_in_both_modes() {
    const U_LINE: &str = include_str!("fixtures/kd5_maximum_u_line.json");
    for mode in MODES {
        let envelope = run(U_LINE, mode);
        let (code, message) = integrity(&envelope);
        assert_eq!(code, PASSED, "{mode:?}: {message}");
        assert!(!message.contains(NOT_VERIFIED), "{mode:?}: {message}");
        assert!(blocking(&envelope).is_empty(), "{mode:?}");
    }
}
