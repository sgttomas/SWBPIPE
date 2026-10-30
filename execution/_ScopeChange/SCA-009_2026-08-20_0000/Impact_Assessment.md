# Piping SCA-009 Gate 2 — impact assessment

## State

`CANDIDATE — GATE 2 NOT ACCEPTED`

This assessment is derivative, read-only planning evidence. It changes no
PRD, decomposition, register, pointer, snapshot, deliverable, product,
runtime, lifecycle, release, or Git state. **No amendment has been applied.**
`Impact_Sketch.md` remains unchanged as Gate-1 history; this document
supersedes it as the Gate-2 assessment.

## Basis

Evaluated against `main@89758a32634ee6cedbd1dbadf35e3728fb48d2eb` (merge of
PR #591), with `SOFTWARE_DECOMP.md` at revision 0.11 and `_ScopeChange/
_LATEST.md` still pointing to SCA-008. Gate 1 is `CONFIRMED` per
`ACCEPTANCE_RECORD.md`: D2 = Option A (ADD `DEL-07-09`), D3 = two-class
vocabulary ratified (`Vocabulary_Annex.md`), D4 = single palette owner
(`DEL-07-09`), D5 = fold `DEL-07-03-R-005`/`R-006` landing into `DEL-07-09`
with the `DEL-16-04` generator left out, D6 = new scope row `SOW-077`.
Option B is closed and is not assessed further. All `SOFTWARE_DECOMP.md`
line references below were re-verified at this basis.

## Impact conclusion

SCA-009 Option A is a bounded additive topology change: **+1 scope item
(`SOW-077`), +1 deliverable (`DEL-07-09`), +1 context row** — 76 → 77 scope
items, 101 → 102 deliverables/context rows — with all other counts
unchanged (18 packages, 18 objectives). It adds no package, removes,
renames, merges, splits, or reclassifies nothing, changes no existing
stable ID, and touches no dependency edge of any existing deliverable. The
vocabulary contract and palette ownership land in the new deliverable; the
PKG-16 mutation route, acceptance/audit controls (DEL-16-02/03/04), PRD R7,
and the professional/IP boundaries are unchanged.

## Exact entity additions (candidate text for Gate 3)

Exact wording is approved at Gate 3; the rows below are the Gate-2 candidate
forms.

### DEL-07-09 — new PKG-07 deliverable row (§7, PKG-07 table, after line 322)

Table columns `|DeliverableID|Name|Type|Scope Items|Objectives|Context|Sizing Notes|`:

> `|DEL-07-09|Interactive operation vocabulary and tool palette contract|UX_UI_SLICE|SOW-077|OBJ-006,OBJ-015|M|Owns the ratified two-class operation-vocabulary coverage contract (SCA-009 Vocabulary_Annex) and the single tool-palette surface organization over the DEL-07-01/DEL-07-02 surfaces; all palette commands route through the PKG-16 operation layer; accepted landing zone for the DEL-07-03-R-005/R-006 editor residuals.|`

**Context-envelope proposal: `M`, with justification.** DEL-07-09 is a
contract/coverage slice — a vocabulary annex, palette organization
contract, and coverage ledger over surfaces that DEL-07-01/02 already
implement — not a new implementation surface. That matches the `M` profile
of DEL-07-02 ("Single UI work surface", line 316) rather than the `L`
implementation surfaces (DEL-07-01/03/05/08). The folded R-005/R-006
landing raises watch pressure but the fold is an ownership binding, not an
implementation transfer into this deliverable's envelope. Proposed
`ContextBudgetQA.csv` row:

> `DEL-07-09,PKG-07,M,OK,Proceed with bounded Type 2 brief,Contract/coverage slice owning the SCA-009 ratified operation vocabulary and single palette surface; watch scope if editor implementation (R-005/R-006 successors) lands here rather than in sibling slices.`

### SOW-077 — new scope row (§4 SSOW register, after line 182; §9 Scope ledger summary, after line 541)

Register form (`|ScopeItemID|Status|Statement|SourceRef|Notes|`):

> `|SOW-077|IN|The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.|PRD v0.3 §14; SCA-009 owner request 2026-08-20|Vocabulary classes NORMATIVE-NOW / ROADMAP per SCA-009 D3 (Vocabulary_Annex); binds to the implemented operation taxonomy; single palette owner DEL-07-09; no second mutation route.|`

Scope-ledger summary form (`|ScopeItemID|Status|Statement|SourceRef|PackageID|DeliverableID(s)|ObjectiveID(s)|OpenIssue|Notes|`):

> `|SOW-077|IN|The GUI shall provide a ratified interactive model-building operation vocabulary and a governing tool-palette contract that organizes the human command surface for building and modifying the piping model, with every palette command routed through structured model operations.|PRD v0.3 §14; SCA-009 owner request 2026-08-20|PKG-07|DEL-07-09|OBJ-006, OBJ-015|FALSE|Vocabulary classes NORMATIVE-NOW / ROADMAP per SCA-009 D3; single palette owner DEL-07-09; no second mutation route.|`

`docs/_Registers/ScopeLedger.csv` and `docs/_Registers/Deliverables.csv`
gain the corresponding rows (exact CSV text at Gate 3).

### Objective mappings (§5, lines 194 and 203)

- **OBJ-006** (line 194): Mapped Scope Items += `SOW-077`; Mapped
  Deliverables += `DEL-07-09` (palette/vocabulary is GUI workflow
  visibility).
- **OBJ-015** (line 203): Mapped Scope Items += `SOW-077`; Mapped
  Deliverables += `DEL-07-09` (vocabulary is a coverage obligation over the
  controlled model-authoring operation layer).

### Package row (§6, line 220)

PKG-07 `Assigned Scope Items` += `SOW-077` (no scope-description change
required; "interactive modeler, editors" already spans the palette
surface — Gate 3 may propose an optional one-clause clarification).

## Topology delta

| Invariant | Pre-change (rev 0.11) | Expected post-change | Risk |
|---|---:|---:|---|
| Scope items | 76, all `IN` | **77**, all `IN` | none — additive |
| Packages | 18 | 18 | none |
| Deliverables | 101 | **102** | none — additive |
| Context-budget rows | 101 | **102** | none — additive |
| Objectives | 18 | 18 | none |
| Context envelope distribution | S=9, M=69, L=23, XL=0 | S=9, **M=70**, L=23, XL=0 | none if `M` accepted |
| Unassigned scope items | 0 | 0 | SOW-077 maps at birth |
| Scope items without deliverable mappings | 0 | 0 | SOW-077 → DEL-07-09 |
| Unmapped objectives | 0 | 0 | OBJ-006/OBJ-015 gain mappings |
| Dependency edges (existing) | unchanged | unchanged | new DEL-07-09 edges are additive only |
| Existing stable IDs | preserved | preserved | no reuse or renumbering |
| New IDs | — | `DEL-07-09`, `SOW-077`, one DEC row | both verified free at rev 0.11 (PKG-07 ends DEL-07-08 line 322; ledger ends SOW-076 line 541) |
| Open issues | 17 | 17 | OI-012/OI-016 cross-references optional at Gate 3, not required |

## Affected sections of `SOFTWARE_DECOMP.md` (by name)

| Section | Change |
|---|---|
| §1 Intake summary | New revision paragraph recording SCA-009 (exact text at Gate 3) |
| §4 Structured Scope of Work (SSOW) | +1 row `SOW-077` |
| §5 Objectives | OBJ-006 and OBJ-015 mapping additions |
| §6 Packages | PKG-07 assigned-scope addition (`SOW-077`) |
| §7 Deliverables (PKG-07 subsection) | +1 row `DEL-07-09` |
| §9 Scope ledger summary | +1 row `SOW-077` |
| §10 Coverage and telemetry | ScopeItemCount 77, DeliverableCount 102, ContextEnvelopeCounts M=70, Revision advance |
| §12 Decision log | One new forward DEC row recording SCA-009 (exact text and next-free number at Gate 3) |

Unchanged: §2 References, §3 Vocabulary map (product-naming terms only —
the operation vocabulary lives in the DEL-07-09 contract, not §3), §8
Architecture basis, §11 Open issues (cross-references optional), §13 Gate
posture (revision-basis sentence updates with the revision advance).
Companion registers changed: `ScopeLedger.csv`, `Deliverables.csv`,
`ContextBudgetQA.csv` (+1 row each).

## D5 fold — adjacent-residual disposition

- **`DEL-07-03-R-005` (load-case editor) and `R-006` (support/restraint
  editor):** `DEL-07-09` becomes their accepted ownership landing zone. The
  residuals stop being "documented GUI absences with no authorizing
  ownership" and become vocabulary-coverage obligations under the
  NORMATIVE-NOW class (Tier 1 item 4/5 and Tier 2 items 12/13/16 in
  `Vocabulary_Annex.md`). Gate 3/4 must include the bounded
  `DEL-07-03/_STATUS.md` (and `_CONTEXT.md` boundary-note) edits that
  re-point R-005/R-006 to the DEL-07-09 contract; no lifecycle change and
  no other DEL-07-03 text is touched. This honors the DEL-07-03 WATCH
  advice (line 317 "may split later"; `_CONTEXT` lines 27–29) by splitting
  ownership out rather than expanding in place.
- **`DEL-16-04` route/support candidate generator: explicitly OUT.** Its
  ownership residual ("generator currently has no owning deliverable —
  ownership needs a decomposition act") is **not** resolved by SCA-009 and
  remains reserved to a separate owner act. No `DEL-16-04` surface is
  touched.

## Dependency and DAG implications

Proposed depends-on edges for the new deliverable (additive; no existing
edge changes):

| From | Depends on | Basis |
|---|---|---|
| `DEL-07-09` | `DEL-16-01` | the vocabulary binds to the structured-operation schema/taxonomy that DEL-16-01 defines (line 407) |
| `DEL-07-09` | `DEL-07-01` | the palette organizes the viewport command surface (line 315; ScopeOfWork OUT-001 "command-routed edits") |
| `DEL-07-09` | `DEL-07-02` | the palette organizes the tree/inspector command surface (line 316; ScopeOfWork OUT-001 "command-routed mutations") |

Downstream mechanics: `_DAG/DAG-008` requires **rebuild** (topology
change, not merely revalidation); a `Dependencies.csv` (register schema
v3.1) extraction for `DEL-07-09` runs after PREPARATION scaffolds the
deliverable folder; a DEL-07-03 boundary note is advisory metadata, not an
edge. No cycle risk: all three proposed edges point upstream to existing
deliverables and create no back-edge.

## Derivative-package status after amendment

| Derivative package / consumer | Owner | Status after amendment | Required action |
|---|---|---|---|
| Pre/post `AUDIT_DECOMP` results | AUDIT_DECOMP under SCOPE_CHANGE | `RECOMPUTE REQUIRED` | Pre-change baseline capture at Gate 5 entry; post-change comparison must show exactly the +1/+1/+1 additive deltas |
| `_DAG/DAG-008` | PROJECT_SETUP / dependency-extract workflow | `STALE_REBUILD_REQUIRED` | DAG rebuild and revalidation after Gate 5 (topology change; SCA-008 needed only revalidation — this differs) |
| Deliverable-corpus concordance rows | RECONCILIATION | `STALE_REBUILD_REQUIRED` (targeted) | Current-authority refresh for new DEL-07-09 rows and touched DEL-07-03 (and boundary-adjacent DEL-07-01/02, DEL-16-01) rows; historical frozen runs preserved |
| `Dependencies.csv` for DEL-07-09 | `dependency-extract` via TASK | `STALE_REBUILD_REQUIRED` (targeted) | Extract after PREPARATION scaffold |
| DEL-07-09 folder scaffold | PREPARATION (dispatched post-Gate-5) | `REQUIRED` | Scaffold package/deliverable folder set; SCOPE_CHANGE does not write it |
| Estimate / schedule surfaces | estimate owner / PROJECT_SETUP | `ADVISORY_STALE` | +1 deliverable of M envelope; revision-basis revalidation; no existing-edge or sequencing change |
| Implementation surfaces (`apps/`, `core/`, `schemas/`, tests) | dev loop workflows | `NO_CHANGE` | No rerun or write authorized by SCA-009 |

No derivative package may be represented as authoritative decomposition
truth. All rows above are downstream obligations recorded for the Gate-5
handoff state; none is triggered by this assessment.

## Derivative-surface classification

| Surface | Classification | Authority basis |
|---|---|---|
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | `DIRECT_EDIT` after Gates 3–4 | Gate-3 approved exact text |
| `docs/_Registers/ScopeLedger.csv`, `Deliverables.csv`, `ContextBudgetQA.csv` | `DIRECT_EDIT` after Gates 3–4 | Companion authoritative registers gain the approved rows |
| `DEL-07-03/_STATUS.md`, `DEL-07-03/_CONTEXT.md` | `DIRECT_EDIT` after Gates 3–4 (bounded R-005/R-006 re-pointing only) | D5 fold; lifecycle-neutral |
| `DEL-16-04/_STATUS.md` and all other DEL-16-04 surfaces | `NO_CHANGE` | Generator ownership reserved to a separate act (D5) |
| DEL-07-01 / DEL-07-02 / DEL-16-01 ScopeOfWork and metadata | `NO_CHANGE` (boundary notes optional at Gate 3/4) | Option A places the contract in DEL-07-09; existing contracts stand |
| `_Decomposition/_LATEST.md`, `_ScopeChange/_LATEST.md` | `DIRECT_EDIT AT GATE 5` only | Pointer-last after validation |
| `SCA-009_2026-08-20_0000/` snapshot completion | `ADD AT GATE 5` | Immutable complete SCA record incl. pre/post coverage and supersession artifacts |
| Pre/post audit results | `RECOMPUTE` | Gate-5 baseline comparison |
| All implementation, PRD, decision-register, and historical-snapshot surfaces | `NO_CHANGE` | Not structurally implicated; history immutable |

## Orphan-risk summary

All zero: `DEL-07-09` is born mapped (SOW-077; OBJ-006/OBJ-015; PKG-07);
`SOW-077` is born assigned (PKG-07 → DEL-07-09); no entity loses a parent,
mapping, or reference; no parent partition is removed, merged, split, or
reclassified, so no child-closure set exists. Execution-time collision
scan at Gate 5 re-proves `DEL-07-09` / `SOW-077` uniqueness.

## Estimate and schedule risk

Advisory only: one added M-envelope deliverable changes no existing edge,
lifecycle, or sequencing. Consumers keyed to decomposition revision must
revalidate their basis; they must not infer changes to existing effort.

## Interaction with active dev loops

This candidate package writes only inside
`execution/_ScopeChange/SCA-009_2026-08-20_0000/` and is **write-disjoint
from all active dev-loop surfaces until Gate-4-approved application**:
implementation trees (`apps/`, `core/`, `schemas/`, `fixtures/`, `tests/`),
every deliverable folder, the decomposition document, registers, `_DAG/`,
`_Reconciliation/`, `_Coordination/`, and both `_LATEST.md` pointers. Any
in-flight dev tranche can proceed without contention. Gate-5 writes confine
to the SCOPE_CHANGE variant write scope (decomposition document, companion
registers, the bounded DEL-07-03 metadata edits, SCA snapshot,
pointer-last); sequencing against the loop at that time is a Gate-4
concern.

## Active snapshot / handoff-state impact

SCA-008 remains the sole active accepted snapshot; `_LATEST.md` is
untouched by this assessment. On Gate-5 closure, SCA-009 must ship the
complete active-snapshot artifact set (per the SCA-008-established
contract: pre/post coverage, supersession map, amendment actions,
propagation plan, acceptance record, run summary) before the pointer
advances; the handoff state must carry the derivative-package rows above as
explicit downstream reruns.

## Gate-3 decision needs

Gate 3 must present and obtain approval for:

1. exact `SOW-077` register and ledger row text and register CSV rows;
2. exact `DEL-07-09` deliverable row, context row, and envelope class;
3. exact OBJ-006 / OBJ-015 / PKG-07 mapping edits;
4. exact §1 revision paragraph, §10 count/revision updates, and the new
   DEC row (next-free number at execution time);
5. exact bounded `DEL-07-03` `_STATUS.md`/`_CONTEXT.md` R-005/R-006
   re-pointing text (before → after), lifecycle-neutral;
6. the `Vocabulary_Annex.md` content as the DEL-07-09 coverage contract
   basis (by SHA-256), including the taxonomy-binding constraint;
7. proposed DEL-07-09 depends-on edges as listed above; and
8. proof that no other surface, ID, edge, lifecycle, or historical record
   changes.

Any new affected surface, ID collision, topology change beyond the
+1/+1/+1 delta, or DEL-16-04 scope creep requires renewed impact review
rather than silent absorption.

## Gate-2 recommendation

Accept this impact assessment. Open Gate 3 only.
