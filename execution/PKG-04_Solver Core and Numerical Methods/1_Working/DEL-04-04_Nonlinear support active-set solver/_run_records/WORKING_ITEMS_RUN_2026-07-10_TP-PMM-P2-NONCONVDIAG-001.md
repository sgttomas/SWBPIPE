# WORKING_ITEMS Run Record - TP-PMM-P2-NONCONVDIAG-001

Date: 2026-07-10
Agent: WORKING_ITEMS (claude worker; owner-adopted tranche, branch
`claude/piping-p2-nonconv-diagnostic`)
Deliverable: DEL-04-04 - Nonlinear support active-set solver
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P2-NONCONVDIAG-001
Target stage: R5 / diagnostics tightening inside the adopted DEC-067
mechanism (`execution/_Decomposition/SOFTWARE_DECOMP.md` section 12); closes
the nonconvergence-diagnostic corner recorded in
`WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P2-FRICTION-001.md`.

## Scope

Make the assembled nonlinear active-set solve emit its nonconvergence
diagnostic on every non-converged exit path, including the recorded
`max_iterations == 1` corner where a sliding-seeded friction support's
deferred first-iterate convergence let the loop exit `converged == false`
with a zero active-set residual and no diagnostic. Diagnostics only; no
mechanics change.

## Defect

`solve_active_set_frame_with_mode`
(`core/solver/nonlinear_integration/src/lib.rs`) computed
`converged = active_set.converged && !blocked && !sliding_force_deferred`,
but the only `NonConvergence` diagnostics in the result came from the
classifier's residual-based `convergence_diagnostic`, which emits only when
the state-change residual exceeds the tolerance. A sliding-seeded friction
support at `max_iterations == 1` confirms its sliding state on the first
iterate (residual 0, classifier converged) while `sliding_force_deferred`
forces the overall result non-converged, so the capped exit returned
`converged == false` with empty diagnostics - a silent non-converged pass.

## Fix

- Non-converged exit guard in the loop's single exit block: when
  `converged == false` and the final diagnostics contain no
  `NonConvergence` code, push a `NonConvergence` failure diagnostic
  (`DiagnosticSource::SolverIteration`) via the new
  `nonconverged_exit_diagnostic` helper. The message names the cause (the
  deferred sliding-force first iterate at the cap, or a generic
  non-converged exit), the iteration count, and the final active-set state
  summary; remediation says to raise `max_iterations`. The guard never
  double-emits when the classifier already reported nonconvergence.
- `core/solver/nonlinear_integration/README.md` records the loud
  non-converged-exit contract.
- No change to `core/solver/nonlinear_supports` classification (the prior-
  iterate sliding-persistence convention is untouched) or to the
  `convergence_diagnostic` surface in `core/solver/diagnostics`.

## Files Touched

- `core/solver/nonlinear_integration/{README.md, src/lib.rs}`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/MEMORY.md`
- this run record

## Tests

- New `sliding_seed_at_single_iteration_cap_emits_nonconvergence_diagnostic`:
  sliding-seeded friction support, `max_iterations == 1`; asserts
  `converged == false`, `is_blocked()`, classifier residual 0 with classifier
  `converged == true`, exactly one `NonConvergence` diagnostic with
  `Failure` severity, deferred-sliding cause text, final state summary, and
  remediation present.
- New
  `sticking_friction_converged_at_single_iteration_cap_has_no_false_positive`:
  sticking friction support (mu*N = 12 N bounds the 10 N tangential
  reaction) converging on its single allowed iteration; asserts
  `converged == true` and empty diagnostics.
- Tightened existing `iteration_cap_returns_nonconvergence_failure_diagnostic`
  (state-switching gap support at `max_iterations == 1`): now pins exactly
  one diagnostic with `NonConvergence` code and `Failure` severity, proving
  the guard does not double-emit beside the classifier's residual-based
  diagnostic.
- Existing DEC-067 transition tests (one-way re-engagement, gap lift-off,
  bounded sliding, seed-independence) stay green.

## Checks

- `cargo test` `core/solver/nonlinear_integration`: 16 passed (14 prior + 2
  new).
- `cargo test` `core/solver/nonlinear_supports`: 19 passed.
- `cargo test` `core/product_physics`: 71 passed.
- `cargo test` `validation/benchmarks/nonlinear`: 19 passed (DEC-067
  transition fixtures green).
- `cargo fmt --check` clean on the touched crate.
- Repo-wide practitioner harness `self-check`: exit 0.
- DEC-025 evidence sweep at the committed clean head recorded as its own
  commit (artifact named in the sweep commit).

## Boundaries And Residuals

- Diagnostics only: no path/load-step friction history model (DEC-067 fence
  verbatim: "no path/load-step friction history model is adopted (future
  D-XX if sought)"); the prior-iterate direction convention is unchanged.
- No DEC-046 threshold changes; unmeasured axes stay TBD; the classifier's
  residual-based diagnostic path is unchanged.
- The anti-chatter oscillation residual from TP-PMM-P2-FRICTION-001 remains
  (bound exceeding drive can oscillate to the visible cap) - it was already
  loud and stays loud.
- No lifecycle transition, no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.
