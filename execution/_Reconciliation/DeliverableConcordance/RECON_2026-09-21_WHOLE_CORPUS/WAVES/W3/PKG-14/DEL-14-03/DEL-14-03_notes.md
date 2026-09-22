# Notes — DEL-14-03 Model-state comparison engine (W3, PKG-14)

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-03_Model-state comparison engine/` (the path has spaces; tokens resolve at the freeze).
- Project-root tokens (`schemas/`, `core/`, `apps/`, `tests/`, `fixtures/`) resolve under `projects/chirality-piping/`.
- Parity records are cited at the repository-root path `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`, where they exist at the freeze.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is the suite-level gate record. No per-test status is asserted beyond it ("not rerun").

## Judgment calls

- **Engine claims (F7).** `compare_model_states` is called only by its tests.
  The Python design-workspace and report-section engines take a precomputed
  comparison, and no desktop or Rust path calls it. Engine-subject rows are
  `ALIGNED` with `PRODUCT_CALLER: NONE`.
- **Architecture basis (FG-DEL-14-03-01).** CLM-004, CLM-006 and CS-04 `.s03`
  name Rust core/application services and schema-first envelopes. The engine
  is a Python library with no service boundary or product caller, so these
  rows are `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
  BASELINE`. The engine hashes nothing, so the JCS clause is not engaged.
- **Verification column counts.** REQ-14-03-001 is `PARTIALLY_IMPLEMENTED`
  because the comparison tests do not validate their state fixtures against
  `schemas/model_state.schema.json`. REQ-14-03-010 is `UNKNOWN`: no
  protected-content gate over the fixtures was located, although inspection
  shows invented provenance.
- **C-14-03-001 and AC-001 (FG-DEL-14-03-05).** The human-approved DAG-007
  record adopted canonical v3.1 dependency enums, and the 2026-06-16 refresh
  found the mirror already canonical. Both rows are therefore overtaken, at
  MEDIUM confidence because the approval does not name this conflict.
- **"Approved DAG-002 mirror" labels.** CLM-005 and CLM-020 are CP-02. The
  rows survive in DAG-010, and only the approval label is stale.
- **Remaining R01.** The text is accurate: RF-001 and RF-002 have no human
  disposition. No governing SOW row carries the review-disposition act, so
  under F2 it is `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED`, `AuthorityNeeded
  REVIEW`.
- **Scope Detail.** This block runs together on one line (CS-06 note). It is
  judged normally as `ALIGNED`: both statements match ScopeLedger.
- **T2B unit blocks** (CLM-008, 016, 024, 033). `ALIGNED`; the engine and
  tests match.

## Canonical departures

None.

## Convention friction

- F4 flags engine-behaviour wording ("missing mapping", "without") on
  `ALIGNED` rows. These carry `GAP_WORDING_CHECKED` because the wording
  describes the required diagnostic behaviour, not a gap.

## Smallest checks for UNKNOWN rows

- REQ-14-03-010: locate or run a protected-content gate over the inline
  fixtures in `tests/test_model_state_comparison.py` and record it.

## Reverse pass

- Claimed: RC-14-0126 (the comparison engine).
- F5-specific NOT_MINE reasons are given for the model-state schema
  (RC-14-0289, the input contract), the mapping schema (RC-14-0186, owned by
  DEL-14-05) and the registers.
- The reverse pass did not change the sealed view.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-14 worker ledgers
(DEL-14-01, DEL-14-02, DEL-14-03): **PASS, 0 consistency findings**. No
flagged pair needed a justification.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Claim fence

These notes and ledgers make no release, approval, compliance or
certification claim. Dispositions are agent judgments, not owner rulings. No
protected standards text, vendor or private data is quoted, and no external
equation artifact is used as evidence (DEC-043).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
