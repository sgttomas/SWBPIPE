//! DEL-08-01 REQ-024's connecting-path witnesses: two invented exact-route
//! results that are solved but not checks passed, produced by this producer and
//! committed with the requests that produce them, so the desktop's report
//! builder and the native host can be exercised on real output.
//!
//! - `pressure_3_gap_unresolved`: `validation/references/rt_a3_s6/requests/c1_gap.json`
//!   (a frozen reference input, read and never rewritten): a gap support, so
//!   every load case is quality unresolved with
//!   `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`, and the integrity
//!   message records no support-state confirmation.
//! - `pressure_3_coupled_friction_sensitive`: the RT-A3 S6 reference system's
//!   no-gap overlay (`rt_a3_s6_reference_system.rs`, `overlay_without_gap`) as
//!   it stood when this witness was made, with only its sustained case and no
//!   combinations; committed as its own request so it does not follow later
//!   changes to the reference system. Its friction support RS-FR1 takes its
//!   normal force from RS-FR1V's reaction and slides; the case is sensitive and
//!   carries the producer's confirmed-state line.
//!
//! The raw results are the producer's exact bytes. Regenerate with
//! `OPENPIPESTRESS_WRITE_REPORT_STANDING_WITNESSES=1` when the producer's
//! published bytes change for an independent reason; the shape assertions
//! below must keep holding.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::Value;
use std::path::PathBuf;

const WRITE: &str = "OPENPIPESTRESS_WRITE_REPORT_STANDING_WITNESSES";
const DIR: &str = "fixtures/reports/invented/report_standing_witnesses";
const CONFIRMED: &str = "Converged support state: established at 128-bit precision by one formation-checked solve;";
const NOT_ESTABLISHED: &str = "the converged support state is not established at 128-bit precision (";

fn project(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(path)
}

fn integrity<'a>(envelope: &'a Value, case: &str) -> &'a Value {
    let id = format!("diagnostic:numerical-integrity:{case}");
    let found: Vec<&Value> = envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["id"] == id.as_str()).collect();
    assert_eq!(found.len(), 1, "{case}: one integrity diagnostic");
    found[0]
}

fn state(envelope: &Value, case: &str, support: &str) -> f64 {
    envelope["results"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["kind"] == "nonlinear_support_active_set_state_code" && r["basis_ref"]["ref_id"] == case && r["entity_ref"] == support)
        .and_then(|r| r["value"].as_f64())
        .unwrap_or_else(|| panic!("{case}: no state row for {support}"))
}

#[test]
fn report_standing_witnesses_are_current_and_keep_their_shapes() {
    let gap_request: Value = serde_json::from_str(include_str!("../../../validation/references/rt_a3_s6/requests/c1_gap.json")).unwrap();
    let friction_request: Value =
        serde_json::from_str(include_str!("../../../fixtures/reports/invented/report_standing_witnesses/coupled_friction.request.json")).unwrap();
    let write = std::env::var(WRITE).is_ok();
    for (name, request) in [("pressure_3_gap_unresolved", gap_request), ("pressure_3_coupled_friction_sensitive", friction_request)] {
        let envelope = run_linear_static_preview_value_with_mode(request, PreviewSolverMode::SparseInteractive)
            .unwrap_or_else(|error| panic!("{name}: the producer refused: {error}"));
        let raw = serde_json::to_string(&envelope).unwrap();
        let value: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(value["status"]["mechanics"], "MECHANICS_SOLVED", "{name}");
        assert_eq!(value["producer"]["semantic_contract_id"], "openpipestress.result_semantics/0.3.0/pressure-3", "{name}: the exact pressure route");
        match name {
            "pressure_3_gap_unresolved" => {
                assert_eq!(value["numerical_quality"]["status"], "unresolved");
                for case in ["load:PRESSURE", "load:VACUUM"] {
                    let d = integrity(&value, case);
                    assert_eq!(d["code"], "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "{case}");
                    let message = d["message"].as_str().unwrap();
                    assert!(!message.contains(CONFIRMED) && !message.contains(NOT_ESTABLISHED), "{case}: no confirmation sentence is published");
                }
            }
            _ => {
                assert_eq!(value["numerical_quality"]["status"], "sensitive");
                let d = integrity(&value, "load:RS-SUS");
                assert_eq!(d["code"], "NUMERICAL_INTEGRITY_SENSITIVE");
                assert!(d["message"].as_str().unwrap().contains(CONFIRMED), "the confirmed-state line is published");
                assert_eq!(state(&value, "load:RS-SUS", "support:RS-FR1"), 3.0, "RS-FR1 slides");
            }
        }
        let path = project(&format!("{DIR}/{name}-sparse_interactive.raw.json"));
        if write {
            std::fs::write(&path, &raw).unwrap();
            continue;
        }
        let committed = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(committed == raw, "{name}: the committed raw bytes are stale; regenerate with {WRITE}=1");
    }
}
