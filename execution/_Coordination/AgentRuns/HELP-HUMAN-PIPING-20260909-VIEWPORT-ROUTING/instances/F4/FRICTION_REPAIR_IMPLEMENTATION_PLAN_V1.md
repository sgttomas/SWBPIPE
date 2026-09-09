# F4 friction repair implementation plan V1

Status: **PREPARATION COMPLETE; SOURCE HELD**
Manager: WORKING_ITEMS Agent 1, instance `F4`, parent `/root`
Package / deliverable: `PKG-04` / `DEL-04-04`
Source basis: `533332349a4607eee561d4ef90fb05a62d86519e`

## Selected bounded method

Keep the solver static and path independent. Initial friction states are numerical warm starts only. They may change the iteration route and count, but every accepted result must satisfy the same final Coulomb state for the same input.

The implementation should replace unconditional prior-sliding persistence with an integration-resolved next-state update:

1. Run the existing base classifier using the candidate's current displacement, current tangential support force, and current normal force. Refactor active-set result formation just enough to accept integration-resolved friction-state overrides expressed with the existing `SupportStateRecord`; apply overrides before computing changed supports, residual, convergence, and diagnostics. This adds no public input/result field.
2. Preserve the base `Inactive` result whenever the existing contact convention reports `N <= 0`. No sliding force may remain in the accepted inactive result.
3. Preserve `Sticking` when `u_t == 0` and `|R_t| <= mu*N`. Equality is inclusive. Choosing sticking at `u_t == 0` on the exact Coulomb boundary is this project's deterministic static convention; it must not be attributed to Abaqus as a rule that every boundary point sticks. The cited Abaqus basis distinguishes interior stick from boundary sliding.
4. Preserve one narrow warm-start exception: on the explicitly deferred first iterate of an initially `Sliding` row, retain `Sliding` even though no force has yet been applied. The existing `sliding_force_deferred` gate already prohibits convergence there. Its free displacement supplies numerical direction for the bounded-force trial on iteration 2; it is not friction history.
5. After a bounded-force trial, derive an explicit next state for every current `Sliding` row, regardless of whether the base cone classifier returned `Sticking` or `Sliding`. Preserve `Inactive` on contact loss. Otherwise:
   - when computed `mu*N == 0` and branch construction produced no applied-force record, use `Sliding` for `u_t != 0`; do not require an exactly zero free-DOF reaction because the existing equilibrium residual owns roundoff;
   - when computed `mu*N > 0`, use `Sliding` only when an applied-force record exists and that applied force and the reported support force both oppose final nonzero displacement;
   - use `Sticking` for every other positive-contact candidate, including `u_t == 0`, missing applied force after the deferred iterate, same-sign/assisting force, or reverse final motion.
   - do not re-stick solely because a derived signed-normal branch assumption is inadmissible. When tangential force/direction and contact are admissible, retain `Sliding`, block convergence through the existing derived-normal branch gate, and retry using the observed current normal sign. This preserves the PR-760 sign-flip path. When contact or tangential direction is also invalid, those state rules govern.
6. The explicit state change in step 5 is required even when roundoff makes `|R_t| > mu*N`; a convergence boolean alone would retry the same bad slide until the cap. Applying the override before residual/diagnostic formation makes the next iteration solve the exact sticking candidate. If that reaction is outside the inclusive cone, the classifier changes the row back to `Sliding`, and the next released solve takes the opposite direction from that sticking reaction.
7. Reconsider all invalid friction rows together through one active-set state vector. Keep support-state sorting and original nonlinear-support input order for deterministic records; keep the existing simultaneous affine solve for coupled derived-normal rows. Do not introduce sequential per-row correction, a tolerance, or a combinatorial branch search.
8. Retain private tangential-consistency and derived-normal-branch evidence as separate convergence gates after the explicit state update. Extend private sliding-solve evidence to retain per-support applied-force presence and derived-branch validity without conflating their state transitions. The bound magnitude remains established by existing branch construction: explicit normals use `F_t = -sign(u_assumed)*mu*|N|`; derived normals use the simultaneous same-iterate affine solve plus signed-normal validation. Add no force-equality tolerance.

The general accepted inequality is `F_t*u_t <= 0`. Strict opposition applies only for a sliding row with `mu*N > 0` and `u_t != 0`. The degenerate `mu == 0`, `N > 0`, `u_t != 0`, `F_t == 0` state remains frictionless and released/sliding. A zero or negative explicit normal remains inactive under the current convention.

## Sign and branch trace

`u_t` uses the positive direction of the support's `FrameDof`. `AppliedSlidingFrictionForce.force` is the support action added to the global right-hand side. `reported_reactions` adds that same action to the selected reaction vector, so the free tangential result reports the support force sign. For `k=100 N/m`, `mu*N=3 N`:

- `F=+1 N`: sticking gives `u=0`, `R_t=-1 N`. A sliding-seed free first iterate gives `u=+0.01 m` and is retained only for the deferred bounded-force trial. Applying `-3 N` gives `u=-0.02 m` and `F_t*u_t=+0.06 J`; iteration 2 explicitly changes to sticking, and iteration 3 accepts `u=0`, `R_t=-1 N`.
- `F=-1 N`: the mirrored accepted result is `u=0`, `R_t=+1 N`; the corresponding wrong branch is rejected by the same product test.
- `F=+3/-3 N`: the exact-boundary result is sticking at `u=0`, `R_t=-3/+3 N`.
- `F=+10/-10 N`: the accepted result is sliding with `u=+0.07/-0.07 m` and `F_t=-3/+3 N`, hence `F_t*u_t=-0.21 J`.

For multiple friction rows, this is a deterministic active-set method, not a proof of global uniqueness for every piecewise or current-normal-coupled model. If the iteration cap is reached before a jointly admissible state is found, preserve the current structured `NonConvergence` failure and the last iteration evidence.

## Exact source fence and ownership

Only the future bounded implementation Agent 2 may write these files after a root source-release amendment:

- `projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs`
  - remove displacement-only persistence from `classify_iteration_support_state`;
  - refactor iteration result formation to apply integration-resolved friction states before changed-state, residual, convergence, and diagnostic calculation, using existing record types and no new public data field;
  - update the now-false anti-chatter assumption text;
  - replace/add co-located unit tests for opposing, assisting, zero-force-positive-limit, exact-boundary, zero-coefficient, and no-contact classification.
- `projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs`
  - preserve the explicitly deferred first sliding warm-start iterate;
  - extend only private sliding-solve evidence with per-support applied-force presence and derived-branch validity, keeping tangential re-stick and derived-normal retry as separate decisions;
  - resolve simultaneous next-state overrides from current final displacement/reaction/normal plus that evidence, then retain a defensive convergence gate; do not change a public input/result struct;
  - retain same-iterate derived-normal affine coupling and existing branch admissibility;
  - update static-friction assumption text;
  - add the table-driven scalar seed/sign/load/mode oracle and focused current-normal, order, no-contact, `mu=0`, and cap regressions.

No Cargo manifest, public struct, product adapter, schema, validation artifact, deliverable, dependency, DAG, lifecycle, decision, receipt, or Git file belongs in the source fence. The two-file fence is sufficient because both the classifier correction and assembled-loop convergence gate/tests live in these existing crate roots. Existing benchmark and product tests are downstream checks and should remain byte-for-byte unchanged unless a new defect independently proves otherwise; such a finding returns to root as a fence amendment.

## Execution and review sequence

1. Root reviews this FREFUTE-reconciled method and the completed current-SHA probe, then either releases this exact fence or amends the plan.
2. A bounded nondelegating Agent 2 performs implementation and co-located tests in the two-file fence. It records pre/post hashes, exact diff, commands, results, and any divergence.
3. WORKING_ITEMS validates write containment and the analytical matrix, then freezes the diff.
4. A fresh read-only Agent 2 reviews 100% of that frozen diff using `software-code-review`. Any actionable finding returns to implementation and requires a fresh full-diff review after remediation.
5. After review passes, request the root-controlled isolated heavy-build slots for focused crates and downstream validations. CHANGE alone owns build integration and Git closeout.
6. Future comparison with an industry-standard solver is a separate validation phase. It has not run and is not a release claim of this tranche.

## Rollback and diagnostics

The repair has no persistence or migration effect. Rollback is the exact two-file source diff under CHANGE ownership. If a candidate branch cannot establish final admissibility within `max_iterations`, return `converged=false`, retain the final state/iteration/applied-force evidence, and emit exactly one structured `NonConvergence` failure through the existing diagnostic path. Do not silently fall back to the prior persistence rule, add artificial friction, loosen convergence, or substitute a history model.

## Evidence state

- The three sealed input hashes, source basis, and two source hashes were verified in this preparation run.
- P0 NS returned an exact scalar Python replay and current-source trace; its first written return did not itself compile a new current-SHA Rust product fixture. Prior dense/sparse executable evidence is cited there.
- The dedicated current-SHA Rust nonlinear-solver API counterexample completed in one Cargo run with exit `0` in `1.831292209 s`: sticking seed converged in one iteration to `Sticking`, `u=0`, reaction `-1`, no applied friction; sliding seed converged in two iterations to `Sliding`, `u=-0.02`, reaction/applied friction `-3`, and work `+0.06`. F4 inspected `returns/NS/RUNTIME_PROBE.md`; it did not repeat the build.
- FREFUTE returned `CHANGES_REQUIRED` on the provisional proposal and conditionally accepted the static model and exact two-file fence. This plan incorporates its zero-limit rule, app-owned equality tie-break, explicit final-direction/current-normal state update, deferred warm-start compatibility, simultaneous-row retry with bounded nonuniqueness, and oracle/mutation fence. Reviewed artifact: `instances/FREFUTE/design/DESIGN_REFUTATION_V1.md`, SHA-256 `2fb285c5e48f7b03b4ff690506c57a4adf159d8d26be11e6b5684320a5f5e6f9`.
