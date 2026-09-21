# DEL-16-01 notes — Structured model operation schema (worker G1, W1)

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

- **FG-DEL-16-01-01 (W-4, contract substitution).** CLM-009.r01 (R001) and
  .r03 (R003) are `IMPLEMENTED_DIFFERENTLY · OWNERSHIP_ELSEWHERE ·
  PROJECT_BASELINE · OWNER`. The runtime GUI/agent edit route is
  `EditorOperationIntent`, validated and applied by
  `core/model_operations/operation_applier` (DEC-020 sole engine), whose
  intent taxonomy sits in the DEL-07-01 `schemas/viewport_editor.schema.yaml`.
  The applier never loads `schemas/model_operation.schema.json`. DEC-094 binds
  the operation vocabulary to the applier taxonomy, not the schema enum.
  Tier PROJECT_BASELINE because the SOW treats the schema as the SOW-069 edit
  contract against DEC-094. OWNER because someone must choose which contract
  governs.
- **FG-DEL-16-01-02 (W-2).** Declared-open TBDs for operation granularity
  (overtaken by DEC-094) and the physical project package/container
  (overtaken by SCA-003) → `STALE_REVIEW_OR_EVIDENCE ·
  SCOPE_REDIRECTED_BY_RULING`. Hash partitioning and durable per-operation
  persistence are recorded as still open.
- **Enumeration growth (W-1).** The SOW lists 9 change kinds. Commit
  46f3aed34 (PR #715) grew the frozen enum to 16, and the test asserts all 16.
  The SOW rows CLM-003, CLM-017 and CLM-018 are therefore `DOC_BEHIND_CODE`.
  The invented fixture still exercises only the original 9.
- R002, the Description and AC-001 are ALIGNED because the schema defines all
  nine operation kinds. The lack of runtime resolvers for move, reconnect,
  constraint and design_knowledge is recorded as FG-01 context, not a defect
  of those rows.
- CT-002 (autonomy) is ALIGNED as an accurate open governance item: OI-016 is
  open and DEC-042 holds R7.
- STATUS R01 is ALIGNED under W-6: an accurate open governance action (human
  dispositions pending, per RF-1601).
- Split: only CLM-009 was split into `.rNN` (all 10 rows), because its rows
  take different dispositions. Other tables are assessed directly.

## Canonical departures

None. CP-03 blocks use CP-02 fields as that pattern directs. The CS-04
sub-claims `.s01` (Still-TBD list) and `.s02` (PKG-00 SEMANTIC_READY framing)
are added as CS-04 instructs.

## Convention friction

- A SURFACE row holds one disposition. CP-04 takes the SOW SURFACE row, so
  the frontmatter revision pin goes to the sub-claim `SOW.s01`.
- Deliverable paths containing spaces cannot be evidence tokens (W-13).

## Reverse pass

- It did not change my view of anything sealed. One observation: RC-16-0349
  notes that `apps/desktop/src/App.tsx` imports the schema. It is passed
  only as a downloadable capability/schema reference in offline proposal
  intake, never used for validation. This is consistent with FG-01, whose
  note speaks of the applier.
- Answers: CLAIMED_BY for the schema (RC-16-0349). PARTIAL for the invented
  fixtures and for the applier operation-kind resolvers plus the taxonomy
  (DEL-16-01 defines operation kinds; FG-01 context). COVERS for the Python
  preview engine. NOT_MINE for the rest.

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
