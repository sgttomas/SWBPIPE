"""§8.63, MODAL-CS, continued: a changed boundary source stops.

test_863_modal_completed_set.py holds the rules' summary and the shared helpers and constants. The tests of
§8.63 are in six files so that a hosted run, which keeps one file on one worker, spreads them over its workers.
"""
import pytest

from test_863_modal_completed_set import (
    LONG, MQ, MQ_PIN, OUTCOME, PP, PRICING, RB, RS, SKEW, copy_core, decisions, edit, sha, tool_with_pin,
)


@pytest.mark.parametrize("path, old, new, message", [
    # MODAL-CS-RK: run_modal_dense_with_budget no longer only delegates, or delegates with limits.
    (OUTCOME, "    run_modal_dense_within(req, params, budget, None)\n",
     "    let _ = format!(\"{}\", 1);\n    run_modal_dense_within(req, params, budget, None)\n",
     "run_modal_dense_with_budget is not run_modal_dense_within's delegation"),
    (OUTCOME, "    run_modal_dense_within(req, params, budget, None)\n", "    run_modal_dense_within(req, params, budget, limits())\n",
     "MODAL-CS-RK|MODAL-CS-RR"),
    # The whole-file pin.
    (RS, "over the caller's limit of {limit}; the whole set", "over the caller's stated limit of {limit}; the whole set",
     "modal/resource.rs changed"),
    # MODAL-CS-ST: a structure that is not a named constant; the reader's two bypasses; a forwarder's body.
    (OUTCOME, "storage.modal_completed_set_rows(total, MODAL_SET_COMPUTED_EIGENVALUES)?;",
     "storage.modal_completed_set_rows(total, req.name())?;", "MODAL-CS-ST: the reviewed inventory changed"),
    (SKEW, "        storage.modal_shape_reserve(&mut v, dimension, SKEW_LIFT_SCATTER)?;",
     f"        let (structure,) = ({LONG},);\n        storage.modal_shape_reserve(&mut v, dimension, structure)?;",
     "MODAL-CS-ST: the reviewed inventory changed"),
    (MQ, "let overflow = ShapeStorageError::CapacityOverflow { structure, elements: len, element_size };",
     f"let overflow = ShapeStorageError::CapacityOverflow {{ structure: {{ {LONG} }}, elements: len, element_size }};",
     "MODAL-CS-ST: the reviewed inventory changed"),
    (MQ, "    let bytes = requested_bytes::<T>(vec.len(), len, structure)?;\n    vec.try_reserve_exact",
     "    let bytes = requested_bytes::<T>(vec.len(), len, structure)?;\n    let _held = vec.len();\n    vec.try_reserve_exact",
     r"MODAL-CS-ST: core/solver/modal_eigen/src/modal_quantities.rs:reserve_exactly#1 changed"),
    # MODAL-CS-TM grounds: a test-item declaration (adaptive.rs) and a test-module's declaration (source_recovery.rs's
    # j7_t4_consequences) lose their #[cfg(test)], so their entries' grounds change.
    ("core/solver/frame_kernel/src/structural/retained/adaptive.rs",
     '#[cfg(test)]\n#[path = "../../../tests/retained_k4/adaptive_tests.rs"]', '#[path = "../../../tests/retained_k4/adaptive_tests.rs"]',
     "MODAL-CS-TM: the reviewed inventory changed"),
    # (J2 checks that declaration by text and stops first; TM's test-module ground is the same requirement by token.)
    (PP + "source_recovery.rs", '#[cfg(test)]\n#[path = "source_recovery/j7_t4_consequences.rs"]',
     '#[path = "source_recovery/j7_t4_consequences.rs"]', r"J2: j7_t4_consequences is no longer a #\[cfg\(test\)\] module"),
    # MODAL-CS-TM: the one include!'s target moved to a file the walk does not read.
    ("core/product_physics/build.rs", 'include!("src/build_identity.rs");', 'include!("src/build_identity.inc");',
     "does not include a .rs file of the walked set"),
    # MODAL-CS-M0: the reader's renamed format import in the dense body (the template scanner emits its row).
    (OUTCOME, "    let total = solution.eigenvalues.len();\n",
     "    let total = solution.eigenvalues.len();\n    use std::format as probe_format;\n    let _ = probe_format!(\"probe\");\n",
     "the dense body's TEXT rows changed"),
    # MODAL-CS-M0: an added format in the unpinned dense body.
    (OUTCOME, "    let total = solution.eigenvalues.len();\n",
     "    let total = solution.eigenvalues.len();\n    let _probe = format!(\"{total}\");\n", "the dense body's TEXT rows changed"),
    # MODAL-CS-TM: the modal document tests are no longer a test-only module.
    ("core/product_physics/src/modal.rs", "#[cfg(test)]\nmod tests_document;", "mod tests_document;",
     "MODAL-CS-TM: core/product_physics/src/modal.rs's `mod tests_document`"),
    # MODAL-CS-LI: a retained-side (frame kernel) call of lift_into, and an edit of modal_lift_within's body.
    (RB, "    /// Adds (Q v)_dof exactly to `acc` (for certificates and the exact",
     "    pub fn lift_twice(&self, v: &[f64], out: &mut [f64]) -> Result<(), BasisError> {\n"
     "        self.lift_into(v, out)\n    }\n\n    /// Adds (Q v)_dof exactly to `acc` (for certificates and the exact",
     "MODAL-CS-LI: the reviewed inventory changed"),
    (SKEW, "        v.resize(dimension, 0.0);\n", "        v.resize(dimension, 0.0);\n        let _k = 0;\n",
     "MODAL-CS-LI: core/product_physics/src/modal/skew_pencil.rs:modal_lift_within#1 changed"),
])
def test_a_changed_boundary_source_stops(tmp_path, monkeypatch, path, old, new, message):
    snap = copy_core(tmp_path / "src")
    edit(snap, path, old, new)
    # modal_quantities.rs is pinned whole (its own test below); past that pin, the inner checks still stop.
    tool = tool_with_pin(tmp_path, sha(snap, MQ), MQ_PIN) if path == MQ else PRICING
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap, tool=tool)
