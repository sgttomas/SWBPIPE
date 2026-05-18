# DEL-14-04 Memory

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
