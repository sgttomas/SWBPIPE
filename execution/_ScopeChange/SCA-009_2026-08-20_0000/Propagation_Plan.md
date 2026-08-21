# Piping SCA-009 — propagation plan and downstream-rerun obligations

## State and basis

`GATE 5 CANDIDATE — CLOSURE RULING PENDING`

- Branch basis: `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb`; gates
  accumulated on `claude/piping-sca-009-gate2-20260820` (single landing
  PR #593).
- Gate-3 approved candidate: `Amendment_Preview.md` SHA-256
  `802c2ce92c5a48651f4d06312d4ba26593f0134e3b7c443988e74b79c0e170d4` at
  commit `d50e72c4b` (post-cleanup preview `44eaf63a…`; six pair hashes
  unchanged).
- Gate 4: `APPLIED` — the six approved postimages are live and hash-proven
  (`RUN_SUMMARY.md` Gate-4 proof table). Live decomposition revision 0.12.

## Already applied (Gate 4; no further action)

`SOFTWARE_DECOMP.md`, `ScopeLedger.csv`, `Deliverables.csv`,
`ContextBudgetQA.csv`, `DEL-07-03/_STATUS.md`, `DEL-07-03/_CONTEXT.md` —
exactly the six approved pairs, nothing else.

## Applied in this Gate-5 candidate tranche

- SCA-009 snapshot completion members: `Pre_Change_Coverage.json`,
  `Post_Change_Coverage.json`, `Supersession_Delta.csv`,
  `Supersession_Map.csv`, this `Propagation_Plan.md`,
  `Validation_Record.md`.
- One coordination notice on the Piping loop's own coordination surface
  (`execution/_Coordination/NOTICE_2026-08-20_PIPING_SCA-009_DEL-07-09_VOCABULARY_PALETTE.md`)
  — the only live-surface write of this tranche. SCA-009 is
  Piping-internal: no Root, App, or Tier-0 notice is required (contrast
  SCA-008, which changed cross-project current-effect authority).

## Remaining Gate-5 closing-commit write scope (after the owner's closure ruling only)

| Path | Action | Control |
|---|---|---|
| `execution/_Decomposition/_LATEST.md` | advance to revision 0.12 | only after the owner's closure ruling; staged before the scope-change pointer |
| `execution/_ScopeChange/_LATEST.md` | advance to SCA-009 | pointer-last: the final live write, only after snapshot completeness validates |

No other path is written by the closing commit beyond the SCA-009 records
that transcribe the closure ruling.

## Downstream-rerun obligations (owned outside SCA-009; none executed by SCA-009 itself)

Exactly the derivative-package obligations recorded in
`Impact_Assessment.md`:

| Obligation | Owner instrument | Trigger | Status |
|---|---|---|---|
| `_DAG/DAG-008` **REBUILD** (not mere revalidation — topology changed: +1 deliverable, +1 scope item, 3 new edges proposed for DEL-07-09) | PROJECT_SETUP / dependency-extract workflow | SCA-009 Gate-5 closure (pointer advance) | `NOT RUN — OBLIGATION RECORDED` |
| Formal pre/post `AUDIT_DECOMP` comparison (the synthesized deterministic baselines in `Pre_/Post_Change_Coverage.json` are inputs, not a substitute) | AUDIT_DECOMP under SCOPE_CHANGE / owning loop | Gate-5 closure evidence assembly or first post-closure audit window | `NOT RUN — OBLIGATION RECORDED` |
| Targeted RECONCILIATION current-authority refresh (new DEL-07-09 rows; touched DEL-07-03; boundary-adjacent DEL-07-01/02, DEL-16-01; the R-005/R-006 supersession binding) | RECONCILIATION | SCA-009 Gate-5 closure | `NOT RUN — OBLIGATION RECORDED` |
| `Dependencies.csv` (register schema v3.1) extraction for DEL-07-09 | `dependency-extract` via TASK | after PREPARATION scaffold exists | `NOT RUN — OBLIGATION RECORDED` |
| PREPARATION scaffold for the DEL-07-09 deliverable folder | PREPARATION (dispatched by the owning loop) | SCA-009 Gate-5 closure | `NOT RUN — OBLIGATION RECORDED` |
| Estimate / schedule surfaces | estimate owner / PROJECT_SETUP | advisory only: +1 L-envelope deliverable, no existing-edge or sequencing change; revision-basis revalidation for revision-keyed consumers | `ADVISORY_STALE — NO RECOMPUTATION REQUIRED` |

No derivative package may be represented as authoritative decomposition
truth. SCA-009 can close `CLOSED_FOR_SCOPE_CHANGE_ONLY` after the owner's
ruling while every row above remains explicit downstream work.
