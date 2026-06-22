# WORKING ITEMS RUN - TP-R4-D9-PRODDISPREACTIONPOLICY-001

Date: 2026-06-22
Agent: WORKING_ITEMS
Package: PKG-09 - Verification, Validation, and Quality Oracles
Deliverable: DEL-09-03 - Nonlinear support regression suite
Target stage: R4 / Phase D

## Objective

Promote the invented mixed product-preview displacement/reaction delta rows
from observation-only metadata to a bounded product-preview-only `DEC-046`
threshold policy, without broadening release, external, sparse-default, or
general validation thresholds.

## Authority Basis

- PRD section 22.5: R4 requires nonlinear support validation cases to converge.
- `DEC-044`: the assembled nonlinear solve is owned by the PKG-04 integration
  tranche bridging DEL-04-04 and DEL-04-01; the classifier remains the
  per-iteration state oracle.
- `DEC-046`: class-tiered convergence policy can promote measured entries, but
  unmeasured and broader entries remain `TBD`.
- Prior evidence:
  - `TP-R4-D6-LIVEBUNDLE-001` added the invented mixed one-way/gap/friction
    product-preview dense-loop surface.
  - `TP-R4-D9-DISPREACTIONOBS-001` added product-preview displacement/reaction
    delta observation metadata.
  - `TP-R4-D9-DISPREACTIONPOLICY-001` promoted current validation-seed and
    accepted eight-fixture multi-support displacement/reaction delta policies,
    while leaving product-preview thresholds open.

## Changes

- Added
  `DEC-046-CV-B-product-preview-displacement-reaction-delta-threshold-v1` to
  `core/product_physics`.
- Promoted emitted product-preview displacement/reaction delta row metadata to
  `threshold_policy_status=accepted` while preserving the existing observation
  ref.
- Bound the invented mixed product-preview delta surface to explicit product
  limits:
  - translation delta: `50.0 mm`;
  - rotation delta: `0.05 rad`;
  - force reaction delta: `110000.0 N`;
  - moment reaction delta: `110000.0 N*m`.
- Regenerated `fixtures/product_preview/invented_mechanics_result.json` so the
  fixture-backed product-preview diagnostics surface the accepted policy ref
  for emitted delta rows. The canned one-iteration fixture cases still do not
  synthesize displacement/reaction delta rows.
- Updated product-preview tests, DEL-09-03 memory, coordination text, active
  completion plan, completion log, and R4 exit-gap matrix.

## Validation

Focused validation:

- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` -
  passed.
- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed, 44
  tests.
- `python3 -m pytest -q tests/product_preview/test_product_preview_service.py tests/test_results_schema.py tests/test_analysis_run_records.py`
  - passed, 20 tests.
- `python3 -m json.tool fixtures/product_preview/invented_mechanics_result.json`
  - passed.
- `git diff --check` - passed.

Full DEC-025 evidence sweep:

- `python3 tools/release/run_evidence_sweep.py --execute` - passed all five
  surfaces:
  - cargo crate sweep;
  - repository pytest, 363 tests;
  - desktop Vitest, 407 tests;
  - Playwright dev/dist desktop lanes, 18 + 1 tests;
  - desktop production build.
- Passing sweep summary:
  `validation/evidence/sweeps/SWEEP_20260622T121215Z_3bc51d2b2eed-dirty.json`

## Boundaries

- This tranche promotes only the invented mixed product-preview
  displacement/reaction delta rows.
- It does not promote broader non-seed force/displacement thresholds, broader
  displacement/reaction delta thresholds, general energy thresholds,
  sparse-default behavior, external validation thresholds, release thresholds,
  CI thresholds, or any lifecycle exit decision.
- No protected standards content, private project data, hidden support
  defaults, professional approval, certification, sealing, authentication, or
  code-compliance claim was added.

## Residuals

- Remaining D6/D9 work includes default sparse promotion, sparse timing/memory
  and conditioning threshold promotion, non-seed force/displacement threshold
  promotion beyond the accepted eight-fixture set, broader displacement/reaction
  delta thresholds beyond the accepted current-seed/eight-fixture/product
  surfaces, general energy threshold promotion, deeper spring-hanger behavior,
  external validation threshold evidence, broader R4 validation packaging, and
  final R4 exit-chain evidence.
