# DEL-16-02 notes — Operation validation and diff preview (worker G1, W1)

## Path aliases

- Deliverable-folder paths contain spaces, so they cannot be evidence tokens
  (W-13). They appear in `ContextRefs` or by alias in Notes:
  `RF-16xx` = `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-xx_<name>/Review_Findings.csv`.
- `SOW`, `STATUS`, `CONTEXT`, `MEMORY` = that deliverable folder's
  `ScopeOfWork.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md`.
- Parity evidence is the repository-root record
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P4/checks/DEL-16-xx/{parity.md,claim-map.csv}`
  (latest per `EVIDENCE_MAP.csv`); migration parity `.../WORKING-P4-PKG16/children/AUTHOR-B1/members/DEL-16-xx/parity-a.md` goes in ContextRefs.
- Project-root tokens (`schemas/`, `core/`, `apps/`, `tests/`, `fixtures/`) resolve under `projects/chirality-piping/`; `tools/validation/...` is the repository-root lint (DEC-081 grant).

## Judgment calls

- **FG-DEL-16-02-01 (W-3).** The SOW presents the Python
  `core/model_operations/validation_preview/engine.py` (called only by tests)
  as the implementation. It declares application, persistence, the final diff
  payload and the envelope mapping TBD or outside the slice. The authoritative
  runtime validate/diff/apply seam is the Rust operation_applier crate. Its
  header names it the DEL-16-02/DEL-16-03 runtime seam; it is the sole engine
  under DEC-020/ADR-0001; MEMORY records it from 2026-06-10; and it predates
  the 2026-07-14 SOW migration. Rows declaring absence or TBD →
  `STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE · LOCAL_DESIGN · NO`. Rows only
  describing the Python engine accurately → ALIGNED (W-5).
- **FG-DEL-16-02-02 (W-4).** REQ-16-02-002: on the runtime route the applier
  reports `schema_validation` from intent-structure checks and never runs
  JSON Schema 2020-12 against the DEL-16-01 schema →
  `IMPLEMENTED_DIFFERENTLY · DOC_BEHIND_CODE · LOCAL_DESIGN · OWNER`. Linked
  to FG-DEL-16-01-01.
- **FG-DEL-16-02-03.** REQ-16-02-003 and the Description: the runtime route
  runs no constraint validation. Intents carry `constraint_validation:
  not_run` and the applier has no constraint stage → `PARTIALLY_IMPLEMENTED ·
  PARTIAL_SLICE · PROJECT_BASELINE · BASELINE`, against the SOW-069 scope.
- CLM-004 → CP-02: it cites `execution/_DAG/DAG-006/DAG-002_EdgeDispositionReview.md`,
  which does not exist at that path; the file is under `execution/_DAG/DAG-002/`.
- STATUS SURFACE → CP-05: Last Updated 2026-07-12 is older than the
  2026-07-16 history entry.
- STATUS R01 (centre-of-gravity design ruling) is ALIGNED (W-6) for the
  set_field path, with MEDIUM confidence. **Verifier flag:** the vocabulary
  round 3 `insert_component_symbol` creation path (2026-08-21) requires and
  persists a finite centre-of-gravity vector with explicit unit for rigid
  components. That may be a declared hold settled in code on the creation
  path (possible CP-10). I did not dispose it as such.
- STATUS R02 is ALIGNED (W-6). Its text is incomplete: cases 76–81 and case
  15's regenerated expectation are also pending human review (corpus README).
- No `.rNN` split was used.

## Canonical departures

None.

## Convention friction

- The declared scope is the "slice", while the crate is shared by two
  deliverables. Whether application counts as DEL-16-02's slice is not
  settled by any key. I recorded the overtaken exclusion statements under
  FG-01 and did not treat them as scope violations.

## Reverse pass

- It did not change my view of anything sealed.
- Answers: CLAIMED_BY for the Python engine, the runtime validate seam
  (RC-16-0157), the outcome envelope and stale-hash blocking. PARTIAL for
  single and batch apply (the validation part), the operation-kind resolvers
  and the diff-preview panel. UNKEYED for the wasm build (RC-16-0040) and the
  contract corpus (RC-16-0285): no key names them, consistent with FG-01.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-16 forward ledgers: **PASS, 0
consistency findings**. No `CANONICAL_DEPARTURE` was needed or used.

## UNKNOWN rows

None. No row in this ledger is `UNKNOWN`, so no smallest-next-check is owed.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Fences

Gate evidence is suite-level only (`GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`;
no suite rerun). No protected standards, vendor or private data is quoted. No
external-corpus equation artifact is used. Nothing here states or implies
release, approval, compliance or certification, and no disposition is an owner
ruling.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
