//! Private test-only execution of retained historical pressure premises.
//! No request field, environment variable, Cargo feature, or normal-library symbol selects this route.
//! This preserves named assertions under the test engine; it does not qualify Current or byte-faithful replay.

use crate::{LinearStaticPreviewRequest, MechanicsEnvelope, PreviewSolverMode};
use std::cell::Cell;

thread_local! { static ACTIVE: Cell<bool> = const { Cell::new(false) }; }

pub(super) fn active() -> bool { ACTIVE.with(Cell::get) }

pub(super) fn with_scope<T>(operation: impl FnOnce() -> T) -> T {
    struct Restore(bool);
    impl Drop for Restore { fn drop(&mut self) { ACTIVE.with(|active| active.set(self.0)); } }
    let previous = ACTIVE.with(|active| active.replace(true));
    let _restore = Restore(previous);
    operation()
}

pub(super) fn run(input: LinearStaticPreviewRequest, mode: PreviewSolverMode) -> MechanicsEnvelope {
    assert!(matches!(input.model.schema_version.as_str(), "0.1.0" | "0.2.0"));
    assert!(input.model.pressure_contract.is_none());
    assert!(input.model.load_cases.iter().all(|case| case.pressure_regions.is_none()));
    assert!(input.model.components.iter().all(|component| component.objective_connector.is_none()));
    with_scope(|| crate::run_linear_static_preview_with_mode(input, mode))
}
