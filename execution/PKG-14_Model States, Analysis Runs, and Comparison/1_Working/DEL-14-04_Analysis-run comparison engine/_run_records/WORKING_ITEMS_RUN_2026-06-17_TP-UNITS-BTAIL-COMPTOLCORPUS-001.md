# WORKING_ITEMS RUN - TP-UNITS-BTAIL-COMPTOLCORPUS-001

- **Date:** 2026-06-17
- **Persona:** WORKING_ITEMS
- **Primary deliverable:** DEL-14-04 Analysis-run comparison engine
- **Package:** PKG-14 Model States, Analysis Runs, and Comparison
- **Tranche:** TP-UNITS-BTAIL-COMPTOLCORPUS-001
- **SMOKE:** TP-MAC-194
- **Scope items:** SOW-073, SOW-072
- **Objective:** OBJ-016

## Scope

Bounded Phase B-tail DEC-026 mixed-unit tolerance corpus slice while C5.7
remains human-execution gated. Harden analysis-run comparison evidence for
caller-supplied mixed-unit normalization and governed relative+absolute
tolerance pairs without adding default tolerance values or external validation
claims.

## Changes

- `core/comparison/analysis_run/engine.py` now classifies result deltas using
  a caller-supplied relative+absolute tolerance pair when both values are
  present on the matched tolerance rule.
- Existing scalar tolerance behavior remains for older fixtures.
- Missing conversion paths remain blocking diagnostics; no hidden conversion
  path or unit default was added.
- `tests/test_analysis_run_comparison.py` now includes a DEC-026-style corpus
  fixture:
  - stress: `1000 kPa` vs `1000500 Pa`, normalized to `Pa`, classified by
    the relative allowance;
  - near-zero force: `0 N` vs `0.01 lbf`, normalized to `N`, classified by
    the absolute floor;
  - omitted `lbf` to `N` conversion blocks with
    `ARC-UNIT-CONVERSION-UNSUPPORTED`.

## Validation

- `python3 tests/test_analysis_run_comparison.py` passed.
- `python3 tests/test_comparison_contracts.py` passed.
- `python3 -m pytest tests/test_analysis_run_comparison.py tests/test_comparison_contracts.py tests/test_design_authoring_comparison_workspace.py tests/test_state_comparison_handoff_report_sections.py -q` passed 23/23 tests.
- `python3 -m pytest -q tests` passed 360/360 tests.
- `git diff --check` passed.

## Evidence

- `apps/desktop/SMOKE.md` TP-MAC-194.
- `plans/PLAN_COMPLETION_LOG.md`.
- `plans/PLAN_2026-06-17_prd_completion.md` B-tail row.
- DEL-14-05 and DEL-02-02 supporting run records with the same tranche id.

## Boundary

Governed-profile comparison evidence only. No default tolerance, release
threshold, solver convergence policy, external validation decision, protected
standards content, private project payload, network/telemetry path, lifecycle
state transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## Handoff

- Accepted upstream snapshot: current `DAG-006` authority via
  `execution/_DAG/_LATEST.md`; DEC-018 unit-system basis; DEC-026
  relative+absolute tolerance-pair ruling.
- Derivative package status: evidence-only code/schema/test tranche; no
  lifecycle/status file, DAG pointer, decision register, or release artifact
  updated.
- Closure verdict: implementation and Python validation complete; DEC-025
  sweep remains required before git closeout.
- Remaining blockers: C5.7 human packaged journey execution remains
  human-gated; remaining B-tail items are broader app unit entry/pickers and
  target-format conversion witnesses outside already-covered boundaries.
