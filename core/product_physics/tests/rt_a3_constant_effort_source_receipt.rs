//! RT-A3 increment 3 (DEL-04-07 `pressure-contract.md`, "Constant-effort
//! supports (RT-A3 increment 3)"): an admitted constant-effort support on the
//! exact route gives an honest outcome on the retained-source path.
//!
//! The retained-source family is straight members, linear restraints or
//! springs, nodal loads and explicitly empty regions; its replay forms the
//! case force from the nodal loads plus eigen loads only
//! (`nodal_and_eigen_case_force`, `source_receipt.rs`, the load-state replay
//! included), so it could not replay a constant force. Source recovery
//! therefore declines any model holding a constant-effort record
//! (`source_recovery.rs` `prepare_sources`: first at the support-dimension
//! closure, then "non-nodal load producer present"), and the reader refuses a
//! plan naming one (`source_blocks.rs` `solver_built_support`,
//! CONSTANT_EFFORT_SUPPORT).
//!
//! The controls are the committed plain and load/reference-state documents
//! whose single case the ordinary route leaves numerically unresolved and the
//! retained-source response rescues
//! (`fixtures/product_preview/pressure_source/{physics_source,load_reference_source}/n06`).
//! With a consuming constant-effort record added (UZ at the tip; in the
//! load-state case the D-2 active device), in both modes: the bounded method
//! is attempted and declined by name, no response is selected or joined, and
//! the ordinary route's own blocking finding stands, so nothing is published
//! (never a pressure-source-1 publication with a mismatched replay). Where the
//! ordinary checks pass, no attempt is needed and the ordinary route publishes
//! the constant force (`rt_a3_constant_effort_references.rs`).

use open_pipe_stress_product_physics::result_export::reader_verdict;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};

const PLAIN: &str = include_str!("../../../fixtures/product_preview/pressure_source/physics_source/n06.request.json");
const LOAD_STATE: &str = include_str!("../../../fixtures/product_preview/pressure_source/load_reference_source/n06.request.json");
const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const PRESSURE_2: &str = "openpipestress.result_semantics/0.3.0/pressure-2";
const CE: &str = "support:ce";

fn solve(document: &Value, mode: PreviewSolverMode) -> Value {
    serde_json::to_value(run_linear_static_preview_value_with_mode(document.clone(), mode).expect("the entry accepts the document")).unwrap()
}

fn codes<'a>(envelope: &'a Value, code: &str) -> Vec<&'a Value> {
    envelope["diagnostics"].as_array().unwrap().iter().filter(|d| d["code"] == code).collect()
}

fn with_constant_effort(text: &str) -> Value {
    let mut document: Value = serde_json::from_str(text).unwrap();
    let provenance = document["model"]["supports"][0]["provenance"].clone();
    document["model"]["supports"].as_array_mut().unwrap().push(json!({"id": CE, "node": "independent-tip",
        "family": "constant_effort_support", "restraints": ["UZ"],
        "hanger": {"hanger_type": "constant_effort_support", "constant_load": {"value": 375.0, "unit": "N"},
            "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "invented RT-A3 increment 3 control"},
        "provenance": provenance}));
    for case in document["model"]["load_cases"].as_array_mut().unwrap() {
        if let Some(states) = case.pointer_mut("/analysis_state/support_states").and_then(Value::as_array_mut) {
            states.push(json!({"support_ref": CE, "participation": {"kind": "active_model_device"}}));
        }
    }
    document
}

#[test]
fn constant_effort_models_decline_retained_source_recovery_by_name() {
    for (name, text, load_state) in [("physics_source/n06", PLAIN, false), ("load_reference_source/n06", LOAD_STATE, true)] {
        let control: Value = serde_json::from_str(text).unwrap();
        let document = with_constant_effort(text);
        let cases = document["model"]["load_cases"].as_array().unwrap().len();
        for mode in MODES {
            let label = format!("{name} {mode:?}");
            // The control: the retained-source response is selected and published.
            let selected = solve(&control, mode);
            assert_eq!(selected["status"]["mechanics"], "MECHANICS_SOLVED", "{label}");
            assert_eq!(codes(&selected, "SOURCE_BLOCK_RECOVERY_SELECTED").len(), cases, "{label}: control");
            assert_ne!(selected["producer"]["semantic_contract_id"], PRESSURE_2, "{label}: control");

            let envelope = solve(&document, mode);
            // Admitted: the family is not refused; the method is attempted and declined by name.
            assert!(codes(&envelope, "EXACT_PRESSURE_FAMILY_NOT_ADMITTED").is_empty(), "{label}");
            assert!(codes(&envelope, "SOURCE_BLOCK_RECOVERY_SELECTED").is_empty(), "{label}");
            let unavailable = codes(&envelope, "SOURCE_BLOCK_RECOVERY_UNAVAILABLE");
            assert_eq!(unavailable.len(), cases, "{label}: {:#?}", envelope["diagnostics"]);
            for d in &unavailable {
                assert_eq!(d["severity"], "info");
                let message = d["message"].as_str().unwrap();
                assert!(message.starts_with("The bounded retained-source method did not produce a selected response"), "{label}: {message}");
                assert!(message.contains("SourceMismatch(\"actual model/source dimensions\")"), "{label}: {message}");
            }
            if load_state {
                let joined = codes(&envelope, "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED");
                assert_eq!(joined.len(), cases, "{label}: {:#?}", envelope["diagnostics"]);
                assert!(joined.iter().all(|d| d["message"].as_str().unwrap().ends_with("retained_source_attempt=unavailable")), "{label}");
            }
            // The ordinary route's own finding stands: nothing is published,
            // under pressure-2, with no selected-source member.
            assert!(codes(&envelope, "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED").iter().any(|d| d["severity"] == "blocking"), "{label}");
            assert_eq!(envelope["status"]["mechanics"], "MODEL_INCOMPLETE", "{label}");
            assert_eq!(envelope["results"], json!([]), "{label}");
            assert_eq!(envelope["producer"]["semantic_contract_id"], PRESSURE_2, "{label}");
            for member in ["source_receipt", "retained_source", "carrier_evidence"] {
                assert!(envelope.get(member).is_none_or(Value::is_null), "{label}: {member}");
            }
            let verdict = reader_verdict::validate("pressure", &envelope, Some(&document["model"]), None);
            assert_eq!(verdict["ok"], true, "{label}: {verdict:#?}");
            // Not a source route: the source-recovery reader finds no receipt to read.
            assert_eq!(reader_verdict::validate("source-recovery", &envelope, None, None)["ok"], false, "{label}");
        }
    }
}
