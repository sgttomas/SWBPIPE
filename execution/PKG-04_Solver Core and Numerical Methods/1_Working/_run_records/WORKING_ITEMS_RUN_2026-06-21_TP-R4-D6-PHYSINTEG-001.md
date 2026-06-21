# WORKING_ITEMS Run Record - TP-R4-D6-PHYSINTEG-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Target stage: R4 / Phase D
Tranche: TP-R4-D6-PHYSINTEG-001
Decision basis: `DEC-044` and `DEC-046`

## Objective

Continue D6 after `TP-R4-D6-LOOPCORE-001` by binding the dense assembled
nonlinear loop into the invented product preview path and result envelope,
without claiming R4 exit validation or final convergence policy.

This is a partial D6 landing. It proves one invented product/app/result-envelope
path for an explicitly declared nonlinear support. It does not bind the sparse
solver into the live path, integrate a governed friction normal-force model,
set measured `DEC-046` convergence values, or close D9 validation.

## Changes

- Added `open_pipe_stress_nonlinear_integration`,
  `open_pipe_stress_nonlinear_supports`, and
  `open_pipe_stress_solver_diagnostics` dependencies to `core/product_physics`.
- Added `PreviewSupport.nonlinear` parsing for explicit invented nonlinear
  support data, including behavior, DOF, initial active-set state, gap/friction
  fields, and optional friction normal reaction.
- Split nonlinear supports out of the linear-support application path so active
  nonlinear restraints are not also applied as linear restraints.
- Called `solve_active_set_frame` from `product_physics` for each preview load
  case when explicit nonlinear supports are present.
- Emitted mechanics result rows for nonlinear active-set iteration count, final
  residual count, converged flag, final support state, final displacement, and
  final reaction, plus load-case-qualified and combination rows where
  applicable.
- Preserved visible `TOLERANCE_POLICY_TBD` diagnostics under
  `DEC-046-CV-B-preview-active-set-count-TBD`; no release-quality tolerance
  value was invented.
- Added `NONLINEAR_SUPPORT_STATE_REVIEW` and
  `NONLINEAR_SUPPORT_LOOP_CONVERGED` diagnostics for preview loop evidence.
- Added `support:NL-140` to the invented preview model as a user-entered
  one-way UY terminal stop at `node:N-140`.
- Regenerated the invented mechanics fixture to 770 result rows and 4 supports.
- Taught desktop analysis-run result classification to map nonlinear support
  rows to `nonlinear_support`.
- Updated Python, Vitest, and Playwright expectations for the new nonlinear
  rows, diagnostics, units, and result counts.

## Validation

Passed:

- `cargo test --manifest-path core/product_physics/Cargo.toml`
  - 35 tests
- `python3 -m pytest tests/product_preview/test_product_preview_service.py`
  - 9 tests
- `npm --prefix apps/desktop test -- --run src/services/previewService.test.ts`
  - 10 tests
- `npm --prefix apps/desktop test -- --run src/App.test.tsx`
  - 57 tests
- `npm run build:wasm && npx playwright test --workers=1`
  - 18 Playwright tests from `apps/desktop`
- `npm run test:e2e:dist:desktop`
  - 1 Playwright dist test
- `python3 tools/release/run_evidence_sweep.py --execute`
  - overall pass across cargo crate sweep, Python pytest, desktop Vitest,
    desktop Playwright dev/dist, and desktop production build
  - summary:
    `validation/evidence/sweeps/SWEEP_20260621T071342Z_251dbcd8ce97-dirty.json`

Intermediate failed evidence:

- `validation/evidence/sweeps/SWEEP_20260621T070045Z_251dbcd8ce97-dirty.json`
  failed at the Playwright dev-server lane because a result-family reaction
  count still expected 9 instead of the new 15. The assertion was corrected and
  the later sweep passed.

## Boundaries

- Dense invented preview sidecar only; no sparse live-path binding.
- No final or measured convergence tolerance values; unmeasured entries remain
  `TBD` under `DEC-046`.
- No R4 exit claim and no D9 validation packet.
- Friction nonlinear supports still require explicit normal-reaction evidence
  until a governed normal-force model is integrated.
- The product preview sidecar does not establish full component macro-element
  solve behavior or pressure-thrust load generation.
- No protected standards content, proprietary catalog value, private data,
  network/telemetry feature, public default, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Continue D6 with friction normal-force model integration when scoped.
- Keep D7 sparse live-path adoption gated by `D-17`.
- Draft D9 assembled nonlinear validation cases and use those runs to seed
  measured `DEC-046` class entries where justified.
- Keep D5 spring-hanger behavior gated by `D-15`.
