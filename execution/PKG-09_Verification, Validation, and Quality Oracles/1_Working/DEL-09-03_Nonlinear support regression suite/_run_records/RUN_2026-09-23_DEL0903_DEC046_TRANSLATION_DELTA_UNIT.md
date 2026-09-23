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

Repository checks and independent review are recorded in the PR.

## Boundary

Unit correction only. No other limit, record id, status, scope, the
release-scope record or the product-preview records changed. Development
verification evidence only (BS-VALID, DEC-081).
