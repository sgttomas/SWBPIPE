---
run_id: RUN-2026-09-23-DEL0903-DEC046-TRANSLATION-DELTA-UNIT
package_id: PKG-09
deliverable_id: DEL-09-03
status: CANDIDATE_FOR_REVIEW
branch: claude/dec046-translation-delta-unit-20260923
base_sha: 23a33eb1d
prepared: 2026-09-23
executor: Claude Code (Claude Opus 5.5), untyped session with the owner
---

# DEL-09-03 — DEC-046 translation-delta unit correction (D-75)

Basis: owner ruling `D-75` (`execution/_Coordination/_DECISIONS/D-75_RULING_2026-09-23.md`,
`DEC-113`), option 1 of `execution/_Coordination/NOTICE_2026-09-22_DEC046_TRANSLATION_DELTA_UNIT.md`.

## Measurement

Translation deltas from `assembled_force_displacement_residual_observations()`,
`assembled_multisupport_depth_residual_observations()` and
`assembled_multisupport_acceptance_residual_observations()`, before (×1000,
labelled mm) and after (fixture-local mm):

| Set | Fixtures | Before | After |
|---|---|---:|---:|
| Seed one-way deactivate / lift-off / one-way re-engage | 3 | 100.0 | 0.1 |
| Seed gap closure | 1 | 50.0 | 0.05 |
| Seed friction slide | 1 | 40.0 | 0.04 |
| Seed gap lift-off, friction bounded slide | 2 | 30.0 | 0.03 |
| Seed friction stick, friction derived normal | 2 | none | none |
| Multi-support (depth observation + 13 acceptance) | 14 | 1.3333333333333335 to 100.0 | 0.0013333333333333335 to 0.1 |

Force, rotation and moment deltas are unchanged. Every multi-support value is at
or below `0.1` mm, so the restated multi-support limit keeps the same envelope.

## Commands and results (from `projects/chirality-piping`)

| Command | Result |
|---|---|
| `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml` | `ok. 19 passed` (includes the tests asserting every seed and multi-support observation is within its governed limit) |
| `python3 -m pytest -q tests/test_nonlinear_support_regression.py` | `8 passed` |

Repository checks:

| Check | Result |
|---|---|
| `harness-self-check` (incl. GEN-13 claims language) | exit 0 |
| `harness-pytest` | `379 passed` |
| `piping-pytest` | `1139 passed` |
| DEC-025 evidence sweep, candidate `899563944` (clean tree) | overall `pass`: cargo, pytest, desktop Vitest, Playwright dev lane `455 passed` and dist lane `53 passed`, production build. Summary `validation/evidence/sweeps/SWEEP_20260923T172503Z_899563944f1d.json`. An earlier run on `060561832` failed at Playwright with 46 `ERR_CONNECTION_REFUSED` on the dev server from test 300 onward (409 passed); the rerun on the next commit, which changed only documentation, passed. |

After `main` advanced, it was merged into the branch (`d572dad0e`, no
conflicts); the nonlinear suite (`19 passed`), the regression test (`8 passed`)
and the self-check were rerun, and the DEC-025 sweep passed again on that clean
merged candidate: `validation/evidence/sweeps/SWEEP_20260923T175638Z_d572dad0ef00.json`.

Independent review (fresh-context agent, did not implement): PASS WITH
NON-BLOCKING NOTES on `060561832`. The one medium note (the benchmark README
still stated `100.0` / `50.0` mm) is fixed in `899563944`. Recorded but not
changed: dated history in `plans/PLAN_COMPLETION_LOG.md` and older MEMORY
entries keeps the former figures, and the observation table lists six of the
nine seed fixtures (pre-existing). Separately observed, out of scope: with
fixture lengths in mm, moment and work quantities labelled `N-m` are N·mm.

## Boundary

Unit correction only. No other limit, record id, status, scope, the
release-scope record or the product-preview records changed. Development
verification evidence only (BS-VALID, DEC-081).
