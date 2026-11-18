//! The preload on the binary64 support-action path (DEL-04-11 REQ-014).
//!
//! Wherever the formation check gives a member bound, the published support
//! action is the certified row (`certified_publication::support`), which
//! `tests/spring_preload_references.rs` examines. Without the bound the case
//! loop's binary64 action is published ([`super::with_cold_load`]); the
//! test-only `U4A_FORCE_BINARY64` hook withdraws the bound so that this path
//! is compared with the same frozen reference, on both routes and modes.
use crate::PreviewSolverMode;
use serde_json::Value;

#[path = "../tests/common/spring_preload_model.rs"]
mod model;
use model::{compare_block, document, model_of, moves_a_base, reference, Route, ROUTES, UNHELD};

fn binary64_solve(document: &Value, mode: PreviewSolverMode) -> Value {
    crate::U4A_FORCE_BINARY64.with(|flag| flag.set(true));
    let envelope = crate::run_linear_static_preview_value_with_mode(document.clone(), mode);
    crate::U4A_FORCE_BINARY64.with(|flag| flag.set(false));
    serde_json::to_value(envelope.expect("the entry accepts the document")).unwrap()
}

#[test]
fn the_binary64_support_action_carries_the_cold_load() {
    let reference = reference();
    for case_id in UNHELD {
        let (_, case) = model_of(&reference, case_id);
        for route in ROUTES {
            if route == Route::Exact && moves_a_base(case_id) {
                continue; // EX refuses a moved spring base (the reference test asserts the refusal).
            }
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let what = format!("{case_id} {route:?} {mode:?} binary64");
                let envelope = binary64_solve(&document(case_id, route, crate::MODEL_SCHEMA_VERSION), mode);
                assert_eq!(envelope["status"]["mechanics"], "MECHANICS_SOLVED", "{what}: {:#?}", envelope["diagnostics"]);
                compare_block(&envelope, &case["expected"], &what);
            }
        }
    }
}
