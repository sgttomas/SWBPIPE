# DEL-13-04 notes — W3 PKG-13, worker G1

Forward: 110 rows (92 required keys). Blocks split into .rNN: CLM-034 and CLM-047. Sub-claims: CLM-014.s01/.s02 and three .sNN on Architecture Basis Injection. Reverse: 323 capabilities. Shared judgments: `../_WORKER_DEL-13-01_NOTES.md`.

## Path aliases
- Code is at `core/model_transform/physical_to_analytical/contract.py` and `_solver_boundary_adapter.py`.
- Fixtures are `fixtures/domain/invented_physical_source_of_truth_model.json` and `invented_physical_to_analytical_trace_gap.json`.
- The SOW's "PRD v0.2 §8.3 / FR-MOD-007" is `docs/PRD.md` (v0.4) at the freeze.

## Judgment calls
- **REQ-007 is VERIFIED_NOT_VALIDATED · VALIDATION_GAP · INVARIANT · VALIDATION · ENGINEERING.** The SOW (CLM-021) and the D-41 T2B record say the section-property oracle does not validate the 3D frame-target suitability. STATUS#remaining/R02 is ALIGNED with `OPEN_ACTION` on REQ-007.
- **CLM-014.s02 (result-envelope posture) is PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · OWNER.** STATUS#remaining/R01 is ALIGNED with `OPEN_ACTION` on it.
- **STATUS#remaining/R03 is PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · VALIDATION.** It records broader field/runtime trace coverage and independent validation as open. REQ-004 is met at object level, so no governing row carries that action (F2). The tier follows the validation element (F8).
- **FG-DEL-13-04-01 (CLM-014.s01 and ABI.s03)** is the same DEC-009 Rust-core departure as DEL-13-03.
- **PRD pointers (CLM-007, CLM-018, CLM-047.r01) are CP-02.** The PRD they call unavailable is present with §8.3 and FR-MOD-007.
- **CLM-024 and CLM-037 are STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT.** Their "Evidence refresh applied 2026-06-07" metadata is overtaken by the 2026-07-12 tranche content.
- **SOW SURFACE is CP-04.** `contract.py` emits the provenance contributor "OpenPipeStress transform contract" into its output; the adapter emits similar strings; the docstring uses the former name. These are emitted data values, so the default CP-04 variant applies.
- **F7:** the transform and adapter have only test callers (the handoff exporter names `contract.py` only as a string). ALIGNED rows carry `PRODUCT_CALLER: NONE`.
- **CP-09.** Parity binds sha256 01ce58d6…, not the frozen a0190a66….

## Canonical departures
None.

## Convention friction
REQ-011 carries a stale "DAG-006" label in its verification column. The requirement itself holds, so it is kept ALIGNED (C6(b)). Pure pointer blocks take CP-02.

## UNKNOWN rows
None.

## Reverse pass and view of sealed rows
No change to this sealed ledger. The contract's dimension set includes `force_per_length`, matching PKG-02, so DEL-13-04 is unaffected by the schema vocabulary gap recorded for DEL-13-01 and DEL-13-02.

## Batch consistency
PASS, 0 findings.

## Selectability
NOT_APPLICABLE on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
