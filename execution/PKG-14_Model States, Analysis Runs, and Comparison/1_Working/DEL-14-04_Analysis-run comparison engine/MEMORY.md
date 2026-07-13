# DEL-14-04 Memory

## 2026-07-12 - D-41 R5 T4 PDU-021 supporting evidence

- The selected T4 reader consumes persisted state/run records, not comparison results. Named result-family coverage and PDU-011/PDU-047 holds remain unchanged.
- Focused evidence is recorded in `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T4-PDU012-PDU021-PDU022-PDU040.md`; lifecycle remains `IN_PROGRESS`, with no review, validation, approval, compliance, release, or professional-reliance conclusion.

## 2026-07-12 - D-41 R5 T2C PDU-030 stable/manual mapping round trip

- Added `derive_exact_result_id_mappings` to produce deterministic DEL-14-05
  `MappingRecord` shapes only for result IDs that are unique on both sides and
  have matching family, object ref, basis ref, and dimension.
- Different, duplicate, ambiguous, or semantically inconsistent IDs are not
  automatically mapped; the existing explicit caller-supplied manual path is
  retained without adding heuristic matching or workflow policy.
- Focused evidence validates the automatic record against the existing
  DEL-14-05 mapping contract and JSON-round-trips both automatic and manual
  records into deltas preserving mapping ID and left/right result IDs.
- Validation passed `12` focused comparison/mapping-contract tests. No
  comparison-output schema, threshold, normalization policy, validation
  outcome, review/lifecycle state, dependency, DAG, or register changed.
- Existing PDU-011 and PDU-047 holds and the D-41 bootstrap remain.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T2C-PDU030.md`.

## 2026-06-18 - TP-UNITS-BTAIL-COMPARISONLINTUNITS-001 supporting comparison evidence

- Supporting role for DEL-08-05 report-lint inventory: the desktop Report
  Content Lint inventory now includes the Comparison workspace unit-matching
  policy surface.
- The lint inventory records `comparison-unit-policy` as an existing
  unit-policy surface and reports `unit_targets=33`,
  `conversion_witness_targets=2`, and `lint_conversion=false`. The Comparison
  workspace continues to report equal-explicit-unit matching over solved
  result units `MPa,N,N*m,mm,rad`, `conversion=false`, and
  `tolerance=not_tolerance_checked`.
- Validation passed: focused App Vitest workspace-render and local project
  round-trip selected tests, focused R2 Playwright smoke 2/2 configured
  project tests, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  single-worker R2/R3 Playwright smoke 18/18.
- Boundary preserved: no comparison delta math, result mapping, tolerance
  profile, default tolerance, solver behavior, unit conversion API, protected
  standards content, private data, lifecycle transition, release-readiness
  claim, professional approval, certification, sealing, authentication, or
  code-compliance claim changed.

## Implementation Notes

- Added a narrow diagnostic analysis-run comparison module at `core/comparison/analysis_run/engine.py`.
- The engine accepts structured analysis-run records, result envelopes, mapping records, optional tolerance profile records, caller-supplied unit conversion factors, and optional settings maps.
- Result deltas preserve raw magnitude deltas separately from unit-normalized deltas and tolerance-profile classification.
- Unit normalization only occurs when dimensions match, units are explicit, and required conversion factors are supplied by the caller.
- Diagnostics are emitted for unresolved mappings, missing result data, incompatible dimensions, missing units, unsupported conversion paths, unsupported result categories, and carried run diagnostics.
- Output professional-boundary flags remain diagnostic/review oriented and do not make external validation or professional determinations.

## Verification Notes

- `python3 tests/test_analysis_run_comparison.py`
- `python3 tests/test_analysis_run_schema.py`
- `python3 tests/test_results_schema.py`
- `python3 tests/test_comparison_contracts.py`
- `python3 tests/test_units_schema.py`
- `python3 tests/test_model_state_schema.py`
- `git diff --check`
- Scoped protected/private/prohibited-claim scan over `core/comparison/analysis_run` and `tests/test_analysis_run_comparison.py`

## 2026-05-11 TP-RECON-01 Reconciliation

- Reconciled DEL-14-04 history from TP-RECON-01 Wave 4, the DEL-14-04 dispatch row, archived DEV-001 revision 0.5 evidence rows, Tranche G handoff/closeout records, and commit `24b5717`.
- Evidence records DEL-14-04 as `CHECKING` with `COMMITTED` implementation evidence at `24b5717` (`core: implement tranche g engines`); this is evidence status only, not a release or professional-reliance state.
- Implemented slice recorded for this deliverable: `core/comparison/analysis_run/engine.py`, `tests/test_analysis_run_comparison.py`, this `MEMORY.md`, and `RUN_NOTES.md`.
- The recorded engine scope is deterministic analysis-run comparison with preserved run context, settings, diagnostics, mappings, raw delta evidence, and unit-normalized deltas only when unit and dimension metadata support comparison; tolerance-profile classification remains separate from raw evidence.
- Archived verification evidence records passing focused and adjacent tests, `py_compile`, `git diff --check`, and protected/private/secret/prohibited-claim scans over the Tranche G surfaces.
- Deferred boundary remains: commercial-prover ingestion, hard-coded tolerance values, GUI/runtime integration, external validation decisions, protected/private data, and professional-authority logic.

## 2026-05-16 - DEV-001 downstream PKG-02 audit memory addendum

Durable context preserved after reconciliation review:
- DEV-001 package-worker audit reviewed this deliverable for downstream compatibility with the accepted PKG-02 foundation contracts.
- Local audit artifacts are `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine/_REVIEW.md` and `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine/Review_Findings.csv`.
- Package audit summary is `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`; package run record is `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_run_records/TASK_RUN_2026-05-16_PKG14_PKG02_DOWNSTREAM_AUDIT.md`.
- This was audit evidence only. It did not change lifecycle state, authorize release, or make a professional, certification, sealing, approval, or code-compliance claim.
- The May 16 package-worker TASK run record did not fully preserve canonical per-deliverable TASK documentation context; this addendum preserves the durable deliverable-local pointer without modifying the completed run record.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-14-04`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-06 - Analysis-run comparison evidence hardening

- WORKING_ITEMS completed Wave 2 evidence hardening after the `DEL-14-01`,
  `DEL-14-02`, and `DEL-14-05` Wave 1 workers returned `SUCCESS`.
- Added focused regression coverage in `tests/test_analysis_run_comparison.py`
  proving carried left/right analysis-run diagnostics remain explicit
  comparison diagnostics with affected run references and blocking/warning
  severity preserved.
- Reverified existing comparison behavior: run context is preserved,
  raw magnitude deltas remain separate from unit-normalized deltas, unit
  normalization requires caller-supplied conversion factors, missing mappings
  and result data emit diagnostics, and professional-boundary fields remain
  explicit and negative.
- Verification passed:
  `python3 -m pytest tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_comparison_contracts.py tests/test_analysis_run_comparison.py -q`.
- No production engine change, lifecycle update, review disposition,
  protected standards content, private/proprietary data, external-validation
  decision, hard-coded tolerance value, professional approval claim,
  certification claim, sealing claim, authentication claim, or code-compliance
  claim was introduced.

## 2026-06-07 - TP-PKG14-Remaining Checking Alignment

- Bounded TASK validation confirmed `tests/test_analysis_run_comparison.py`
  covers deterministic run context preservation, raw versus unit-normalized
  deltas, explicit conversion-factor requirements, mapping/result diagnostics,
  carried run diagnostics, settings deltas, and professional-boundary flags.
- `tests/test_comparison_contracts.py` and current DEL-14-04 evidence cover
  consumption of DEL-14-05 mapping and tolerance contracts without hard-coded
  public defaults.
- Deliverable-local consistency scan passed structural checks with no missing
  core files, no missing four-document kit files, no identity mismatches, and
  no candidate unsourced numerics. The reported `TBD`/`ASSUMPTION` markers are
  non-blocking for CHECKING: they are intentional deferred decisions under
  OI-014 or architecture/export/default policy, or stale setup-kit placeholders
  superseded by current implementation/test evidence.
- Focused validation passed:
  `python3 -m pytest tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_comparison_contracts.py tests/test_analysis_run_comparison.py -q`
  with 18 tests passing.
- `_STATUS.md` was set to `CHECKING` for this lifecycle-alignment task only.
  No release, professional approval, certification, sealing, authentication,
  external-validation, or code-compliance claim was made.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-COMPTOLCORPUS-001 mixed-unit tolerance corpus

- Added DEC-026-style relative+absolute tolerance pair classification support
  to `core/comparison/analysis_run/engine.py` when a caller-supplied governed
  tolerance profile provides both pair values.
- Preserved the existing scalar tolerance path for older fixtures and kept
  missing conversion factors as blocking `ARC-UNIT-CONVERSION-UNSUPPORTED`
  diagnostics.
- Added `tests/test_analysis_run_comparison.py` corpus coverage for mixed
  stress units (`kPa` to `Pa`) and near-zero force units (`lbf` to `N`),
  proving relative allowance and absolute-floor behavior without adding any
  default tolerance.
- Validation passed: `python3 tests/test_analysis_run_comparison.py`;
  `python3 tests/test_comparison_contracts.py`;
  `python3 -m pytest tests/test_analysis_run_comparison.py tests/test_comparison_contracts.py tests/test_design_authoring_comparison_workspace.py tests/test_state_comparison_handoff_report_sections.py -q`
  (23/23); `python3 -m pytest -q tests` (360/360); `git diff --check`.
- Boundary preserved: governed-profile comparison evidence only. No default
  tolerance, release threshold, solver convergence policy, external
  validation decision, protected standards content, private payload, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-COMPARISONGUIUNITS-001 primary evidence

- Primary role for comparison workspace/unit-evidence tranche: the desktop
  preview comparison packet now carries `unit_policy_evidence` for mapped
  result deltas, and the Comparison panel exposes the policy in
  `comparison-unit-policy`.
- The packet records that comparison deltas are produced only after stable
  result references match and left/right result units are equal. It records
  matched result units, unmatched row counts, DEC-018/DEC-026/DEL-14-05 basis
  refs, `conversion_performed=false`, and `tolerance_profile_ref=TBD`.
- Validation passed: focused App Vitest 56/56, focused R2/R3 Playwright smoke
  file 14/14, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  `git diff --check`. DEC-025 sweep evidence is recorded in closeout
  artifacts.
- Boundary preserved: no comparison delta math, tolerance profile, default
  tolerance, solver convergence policy, external validation decision,
  protected standards content, private payload, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-06-17 - TP-UNITS-BTAIL-DESIGNWORKSPACEUNITS-001 supporting comparison evidence

- Supporting role for DEL-07-08 design workspace tranche: the desktop
  design-authoring workspace packet now records the embedded DEL-14-04
  comparison unit-policy evidence ref and comparison matched-unit set when a
  comparison exists.
- The design workspace evidence consumes the comparison packet's existing
  equal-explicit-unit matching policy and matched units without changing
  comparison delta math, tolerance classification, unmatched row accounting,
  or solver behavior.
- Validation passed: focused App Vitest 56/56, focused R2/R3 Playwright smoke
  file 14/14, full desktop Vitest 18/18 files and 399/399 tests, desktop
  production build with the existing Vite large-chunk warning, and
  `git diff --check`. DEC-025 sweep evidence is recorded in closeout
  artifacts.
- Boundary preserved: no comparison delta math, tolerance profile, default
  tolerance, solver convergence policy, external validation decision,
  protected standards content, private payload, lifecycle transition,
  release-readiness claim, professional approval, certification, sealing,
  authentication, or code-compliance claim changed.

## 2026-07-12 - D-41 R5 T2B PDU-011/PDU-047 evidence backcheck

- Confirmed no accepted schema currently governs `AnalysisRunComparison.to_dict()` output; mapping and tolerance schemas are input contracts. PDU-011 is held rather than closed with a test-authored contract.
- Confirmed the TP-PHYS-015 section-property oracle cannot validate unit-normalized comparison mechanics and `section_property` is outside the current supported result-family set. PDU-047 remains held at the engineering-suitability grain.
- Evidence is in `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T2B-PDU011-PDU047.md`.
- No schema, comparison behavior, threshold, validation outcome, review disposition, lifecycle, dependency, DAG/register state, or engineering-validation claim changed.
