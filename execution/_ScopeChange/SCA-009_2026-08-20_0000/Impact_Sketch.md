---
doc_id: PIP-SCA009-IMPACT-SKETCH
doc_kind: scope_change.impact_sketch
status: preliminary_not_gate2
date: 2026-08-20
amendment_id: SCA-009
decomp_variant: SOFTWARE
durable_basis_commit: 7584de0a8d53d69a135c22fe39a78cb4a30b6cb2
---

# Piping SCA-009 impact sketch — PRELIMINARY

**State: `PRELIMINARY — NOT A GATE-2 IMPACT ASSESSMENT`**

This sketch accompanies the Gate-1 candidate package only. The formal
Gate-2 `Impact_Assessment.md` (four-lens tracing, derivative-package status
table, derivative-surface classification, orphan-risk counts, pre-change
AUDIT_DECOMP baseline) is produced by SCOPE_CHANGE only after the owner's
Gate-1 confirmation. Nothing here is accepted impact truth.

## 1. Affected entities (candidate set)

All `SOFTWARE_DECOMP.md` references at revision 0.11, basis
`7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`.

| Entity | Where | Option A exposure | Option B exposure |
|---|---|---|---|
| PKG-07 | line 220 | Assigned-scope/deliverable set grows (DEL-07-09) | Deliverable contracts amended (DEL-07-01/02) |
| PKG-16 | line 229 | Coverage-consumer of new vocabulary contract | DEL-16-01 contract amended |
| DEL-07-01 | line 315; ScopeOfWork line 16 | Boundary note vs new palette owner | ScopeOfWork/_CONTEXT amended (palette: viewport) |
| DEL-07-02 | line 316; ScopeOfWork line 16 | Boundary note vs new palette owner | ScopeOfWork/_CONTEXT amended (palette: tree/inspector) |
| DEL-07-03 | line 317; _CONTEXT lines 27-29 (WATCH); _STATUS Remaining R-005/R-006 | Candidate landing zone question for R-005/R-006 (owner decision) | Boundary note; WATCH risk grows if editors expand in place |
| DEL-07-08 | line 322 | Adjacent workspace; boundary note only | Adjacent; boundary note only |
| DEL-16-01 | line 407; ScopeOfWork lines 53/152/266 | Consumer of vocabulary coverage obligation | ScopeOfWork amended (vocabulary coverage obligation) |
| DEL-16-02/03/04 | lines 408-410 | No control change proposed | No control change proposed |
| DEL-00-05 | line 243; SOW-060 lines 166/525 | Advisory consistency check (GUI state/undo-redo basis) | Advisory consistency check |
| PKG-03 / PKG-05 / PKG-13 | lines 216/218/226 | Advisory: vocabulary items reference their component/load/constraint semantics; no write proposed | Same |
| SOW-020, SOW-021 | lines 126-127, 485-486 | Remap or new-row alternative | Coverage-note amendment candidates |
| SOW-069, SOW-070 | lines 534-535 | Unchanged routing constraint; mapping addition possible | Unchanged routing constraint |
| SOW-076 | line 541 | Unchanged | Unchanged |
| OBJ-006, OBJ-015 | lines 194, 203 | Mapping additions for DEL-07-09 | Mapping notes only |
| OI-012, OI-016 | lines 575, 580 | Possible cross-reference updates | Possible cross-reference updates |

## 2. Topology implication per option

- **Option A (ADD):** +1 deliverable row (DEL-07-09) and +1 context row
  (new `_CONTEXT.md` / `_STATUS.md` scaffold via PROJECT_SETUP /
  PREPARATION — not written by SCOPE_CHANGE); either +1 scope row
  (SOW-077) or a SOW-020/SOW-021 remap; PKG-07 assigned-scope and
  OBJ-006/OBJ-015 mapping updates. Current counts (SCA-008 baseline: 76
  scope items, 18 packages, 101 deliverables, 101 context rows, 18
  objectives) change: deliverables/context rows 101 → 102; scope items
  76 → 77 if a new SOW row is chosen. Context envelope distribution gains
  one entry (envelope class TBD at Gate 2/3).
- **Option B (MODIFY):** zero topology change; no new IDs; amends N
  existing deliverable contract surfaces (at minimum DEL-16-01, DEL-07-01,
  DEL-07-02 ScopeOfWork/_CONTEXT; likely a DEL-07-03 boundary note) plus
  scope-ledger notes. Envelope re-checks required for DEL-16-01 (M) and
  DEL-07-01/02 (L/M) — growth may trigger envelope reclassification, which
  is itself a Gate-2/Gate-3 question.

## 3. Candidate downstream reruns (mirroring SCA-008 rerun vocabulary)

| Derivative package / surface | Owner | Candidate post-amendment status | Candidate action |
|---|---|---|---|
| Pre/post `AUDIT_DECOMP` results | AUDIT_DECOMP under SCOPE_CHANGE | `RECOMPUTE REQUIRED` | Gate-1/Gate-5 baseline capture and comparison when the SCA formally runs |
| `_DAG/DAG-008` | PROJECT_SETUP / dependency-extract workflow | Option A: `STALE_REBUILD_REQUIRED` (topology change); Option B: `STALE_REVALIDATION_REQUIRED` | DAG rebuild or revalidation after Gate 5 |
| Deliverable-corpus concordance rows for touched deliverables | RECONCILIATION | `STALE_REBUILD_REQUIRED` (targeted) | Targeted refresh for DEL-16-01, DEL-07-01/02(/03) and any new DEL-07-09 rows |
| `Dependencies.csv` extraction for new/amended deliverables | `dependency-extract` via TASK | `STALE_REBUILD_REQUIRED` (targeted) | Re-extract after amendment |
| Estimates / schedule surfaces | `estimate-snapshot` via TASK / PROJECT_SETUP | `ADVISORY_STALE` | Advisory only; no estimate/schedule effect from this candidate |

No rerun is triggered by this package. All rows are candidate obligations
for the formal Gate-2 assessment.

## 4. Interaction with the active dev loop (write-disjointness)

This candidate package writes exactly one new directory,
`execution/_ScopeChange/SCA-009_2026-08-20_0000/`, and nothing else. It is
write-disjoint from: implementation surfaces (`apps/`, `core/`,
`schemas/`, `fixtures/`, `tests/`), every deliverable folder, the
decomposition document, `_LATEST.md`, `_DAG/`, `_Reconciliation/`,
`_Coordination/`, and all registers. Any in-flight dev-loop tranche can
proceed without contention. A future Gate-5 execution would confine writes
to the SCOPE_CHANGE variant write scope (decomposition document + affected
`_CONTEXT.md` / `_STATUS.md` + SCA snapshot) and would be sequenced against
the loop at that time; that sequencing is a Gate-4 concern, not settled
here.
