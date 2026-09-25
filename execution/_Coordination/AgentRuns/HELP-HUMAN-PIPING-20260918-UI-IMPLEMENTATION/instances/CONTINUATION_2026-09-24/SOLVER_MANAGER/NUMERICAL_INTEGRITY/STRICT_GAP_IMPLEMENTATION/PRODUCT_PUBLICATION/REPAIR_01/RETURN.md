# Minimal precision-1 publication containment repair

Source is frozen for independent review, not postfix validation. The observed
parent-run baseline is preserved under
`SOLVER_FIRST_CHECKS/_run_records/product_publication_baseline` relative to the
NUMERICAL_INTEGRITY evidence root: one ordinary companion test (four solves)
passed; all 16 gap tests reached the supported converged loop/contact controls
and failed on the missing specific containment diagnostic with checks_passed
quality. No baseline bytes or tests were changed by this repair.

Only product_physics/src/lib.rs changed, in three bounded hunks: import the
existing StrictGapEvidence enum, map the specific recovery diagnostic to
numerically_unresolved/unresolved/not_assessed/unresolved, and branch on the
actual selected iteration's StrictGapEvidence::Qualified. That branch emits a
warning at the existing case integrity diagnostic ID instead of appending the
ordinary preprojection passing report. The warning binds the actual case and
explains exact-ratio-derived global/support values versus projected-binary64
member recovery, retaining the ordinary structural and selected-public-state
equilibrium facts as inspection evidence. It does not label the latter exact
ratio equilibrium or full field-recovery verification.

The ordinary no-gap report branch is unchanged. Other nonlinear evidence follows
its existing report path. Convergence/contact decisions, numerical computation,
row publication, preliminary linear observations, input serialization and raw
computed mechanics status are unchanged. No broad gap-family prohibition, new
schema/field, p2 dependency or Current framework is introduced. The existing
quality gate receives unresolved; no reliance or physical correctness is claimed.
The warning preserves inspection rather than forcing blocked_envelope and losing
computed rows.

SOURCE_FREEZE.json and SOURCE.diff identify the complete owned delta, with paths
relative to REPO_ROOT (the active numerical checkout). Actual runtime origins and
exact before/after source bytes are under _run_records. The frozen integration
test remains SHA-256
`1c33c4f0604b3eb0e31e689f3f965e65fd5d832377737f366b1b25e038fc1b78`.
The parent owns independent review and the exact postfix plus full affected checks.
No Rust tests, Cargo, build, Git, formatting, manifest, nonlinear-source,
source_capture or app operation was performed by this TASK.

Suggested postfix command, not executed here:
`cargo test --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --test represented_gap_publication -- --nocapture`
under the parent's approved target/lock/CPU lease. Passing is not claimed before
that execution; consumer/full affected verification remains separately required.

Actual author: TASK /root/solver_manager/evidence_primitives, parent
/root/solver_manager, delegated-harness-native followup. No descendants. The
parent relayed ROOT's approved minimal containment scope; this return does not
assert personal owner review, engineering acceptance or release.
