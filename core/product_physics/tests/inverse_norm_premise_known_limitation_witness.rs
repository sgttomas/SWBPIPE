//! KNOWN-LIMITATION WITNESS of the integrity check's unproved inverse-norm
//! premise (DEL-04-01 REQ-015, REQ-016, AC-013; the OUT-001 row of REQ-015 in
//! `execution/PKG-04/DEL-04-01/ScopeOfWork.md`). It is NOT a requirement: it
//! asserts the present, wrong behaviour on invented models where taking 1024
//! times the condition estimate as an upper bound on the scaled inverse norm
//! fails (1024 times the estimate is 1/256 of the true norm). A change in its
//! outcome needs the repair's independent evidence, and DEL-04-01's OUT-001 row
//! reconciled in the same change.
//!
//! The models and their exact truth are
//! `validation/references/inverse_norm_witness` (its README states every
//! number's reason), generated without product code. The requests run here
//! unchanged, through the product's ordinary entry.
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
const ESTABLISHED: &str = "Converged support state: established at 128-bit precision";

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

fn result(envelope: &Value, kind: &str, entity: &str) -> f64 {
    let found = envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["kind"] == kind && r["entity_ref"] == entity)
        .collect::<Vec<_>>();
    assert_eq!(found.len(), 1, "{kind} {entity}");
    let value = found[0]["value"].as_f64().unwrap();
    match found[0]["unit"].as_str().unwrap() {
        "mm" => value / 1000.0,
        _ => value,
    }
}

fn exact_sign(value: &Value) -> f64 {
    let text = value["exact"].as_str().unwrap();
    if text == "0" {
        0.0
    } else if text.starts_with('-') {
        -1.0
    } else {
        1.0
    }
}

/// One-way witnesses: the stop is published inactive, with its state
/// established and the case passing its checks, where the exact solution has it
/// active; the published stop displacement lies on the open side while the exact
/// inactive-state displacement lies on the closed side.
#[test]
fn known_limitation_witness_one_way_stop_published_inactive_where_exact_state_is_active() {
    for (variant, request, mode) in [
        ("one_way_negative", ONE_WAY_NEGATIVE, PreviewSolverMode::DenseScrutiny),
        ("one_way_positive", ONE_WAY_POSITIVE, PreviewSolverMode::SparseInteractive),
    ] {
        let truth = truth(variant, request);
        assert_eq!(truth["exact_state"], "active", "{variant}");
        assert_eq!(truth["inactive_state"]["admissible"], false, "{variant}");
        let envelope = run(request, mode);
        let at = format!("{variant} {mode:?}");
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{at}");
        let (code, message) = integrity(&envelope);
        assert_eq!(code, PASSED, "{at}: {message}");
        assert!(message.contains(ESTABLISHED), "{at}: {message}");
        assert_eq!(
            result(&envelope, "nonlinear_support_active_set_state_code", "support:STOP"),
            0.0,
            "{at}: published inactive"
        );
        let published = result(&envelope, "nonlinear_support_final_displacement", "support:STOP");
        let exact = exact_sign(&truth["inactive_state"]["stop_displacement"]);
        assert!(exact != 0.0 && published * exact < 0.0, "{at}: published {published:e}, exact sign {exact}");
    }
}

/// Linear variant. The check's internal enclosure of B's displacement excludes
/// the exact value (observed only with a print-only probe; these result
/// documents do not print the enclosure). What is observable: the case passes
/// its checks, and B's published UZ is not the exact value 0.
#[test]
fn known_limitation_witness_linear_case_passes_with_inexact_published_displacement() {
    let truth = truth("linear", LINEAR);
    assert_eq!(truth["exact_displacement_B"]["UZ"]["exact"], "0");
    for mode in [PreviewSolverMode::DenseScrutiny, PreviewSolverMode::SparseInteractive] {
        let envelope = run(LINEAR, mode);
        assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{mode:?}");
        let (code, message) = integrity(&envelope);
        assert_eq!(code, PASSED, "{mode:?}: {message}");
        let uz = result(&envelope, "global_nodal_displacement_z", "node:B");
        assert!(uz != 0.0, "{mode:?}: published UZ {uz:e}");
    }
}
