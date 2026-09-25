# Product exact-gap publication controls — tests-only return

A single new public-entrypoint integration test file is frozen before production
repair. SOURCE_FREEZE.json and SOURCE.diff identify the complete owned change;
paths are relative to REPO_ROOT, the parent's numerical checkout. Exact test bytes
and actual source/fixture/design origins are under _run_records. No existing
product/source-capture/numerical-evidence/manifests or reviewed freezes were edited.

The actual invented_preview_model.json model0.1 is the input basis. Its version,
project metadata and status remain intact. Tests explicitly replace entities with
an invented 1 m straight circular pipe, OD0.1 m, wall0.005 m, E200 GPa and G77 GPa,
root6 restraints and tip UY/UZ/RX/RY/RZ constraints leaving UX free. The tip uses
an explicitly authored anchor-family DOF subset: source inspection showed Guide
rejects rotations, while Anchor permits the listed subset and the product mapping
preserves it. This is an invented kinematic boundary, not a support-product claim.
All pressure, thermal, nonlinear composition, shared-section and combination
inputs are removed explicitly. Embedded materials remain explicit, with empty
request override; every replacement carries invented provenance.

Sixteen separate gap tests cover both modes, both closing senses, both initial
states and signed loads directed toward the stop at25 kN/open and100 kN/contact.
The0.1 mm UX gap is translational. Independent A=pi*t*(OD-t) and u=FL/(EA) place
these loads on opposite sides of the stop; calculated preconditions are in
CHECKS.json. These seed transitions also avoid deliberately exceeding the retained
110 kN reaction-delta limit. The no-gap companion runs both signed100 kN loads in
both modes, requires ordinary checks_passed and compares tip UX with FL/(EA) at
the existing1e-9 relative criterion. Assumptions: uniform linear-elastic circular
annulus, small axial strain, fixed root, no axial tip restraint, no pressure or
eigenload. No solver output supplies an expected value.

Gap tests first require case-bound loop-converged diagnostics, converged flag1,
final changed-support count0 and the independently expected final contact state.
They then require the exact proposed diagnostic
NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED, bound through the case's evidence
refs, and aggregate/case unresolved with numerically_unresolved/accuracy unresolved.
Ordinary checks_passed/sensitive diagnostics cannot qualify the same case. They
do not require MODEL_INCOMPLETE or removal of computed inspection rows. Full
mode/sense/seed/load, quality and diagnostic context appears on failure. An unrelated
unsupported/input error therefore fails the execution prerequisites rather than
masquerading as expected containment.

Observed baseline: none. No Rust/Cargo/build or runtime command was executed.
Predicted baseline: the16 gap tests fail at the missing specific diagnostic after
completed-loop checks because current product appends the ordinary preprojection
report; the ordinary companion should pass. These are predictions to be tested,
not observed behavior. Any earlier prerequisite failure requires diagnosis before
claiming the intended regression has been reproduced. This suite tests the
producer's nonpassing eligibility precondition, not the separate consumer Current
resolver, a complete mixed-field proof or precision-2 qualification.

Parent's focused baseline command after the retained-kernel checks:
`cargo test --manifest-path projects/chirality-piping/core/product_physics/Cargo.toml --test represented_gap_publication -- --nocapture`
Apply the parent's approved CPU/target/lock policy. Production containment repair
remains parent-owned and must follow actual baseline evidence and review.

Authorship: TASK /root/solver_manager/evidence_primitives under
/root/solver_manager via native followup delegation; no descendants.
