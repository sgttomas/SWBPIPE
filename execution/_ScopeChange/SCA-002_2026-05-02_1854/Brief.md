---
amendment_id: SCA-002
doc_kind: scope_change.brief
package_role: snapshot / handoff artifact
created: 2026-05-02
status: accepted_for_downstream_refresh_planning
---

# SCA-002 Brief

## Intake

Human intent, via the SCA-002 Bootstrap and Integration Plan, is to process PRD v0.2 as a controlled decomposition amendment. `docs/_ScopeChange/` is authoritative for SCA-002; `execution/_ScopeChange/` records operational evidence and handoff.

## Parsed Actions

|Seq|ActionType|EntityType|EntityID|Requested Change|
|---:|---|---|---|---|
|1|MODIFY|DECOMPOSITION|SOFTWARE_DECOMP|Revise accepted software decomposition from revision 0.4 to 0.5 using PRD v0.2 as working design basis.|
|2|ADD|SCOPE_ITEM|SOW-064..SOW-076|Add design-engine, physical-model, constraint, operation, state/run/comparison, handoff, prover-boundary, and GUI comparison scope.|
|3|ADD|OBJECTIVE|OBJ-014..OBJ-018|Add objectives for physical design modeling, controlled authoring, states/runs/comparison, handoff, and boundary preservation.|
|4|ADD|PACKAGE|PKG-13..PKG-16|Add flat packages for design knowledge/constraints, states-runs-comparison, handoff/prover workflow, and operations/agent proposals.|
|5|ADD|DELIVERABLE|DEL-07-08, DEL-08-06, DEL-13-01..DEL-16-04|Add 19 new deliverables while preserving existing stable IDs.|
|6|MODIFY|REGISTER|ScopeLedger, Deliverables, ContextBudgetQA|Synchronize companion register rows with revision 0.5 decomposition truth.|
|7|MODIFY|SCOPE_CHANGE_STATE|_LATEST.md|Point latest SCA state to this immutable snapshot.|

## Corrective Cleanup

After hindsight review, SCA-002 received a corrective cleanup for package-local scope mapping, reciprocal scope-ledger mapping, package-table coverage, decomposition latest pointer, and canonical supersession CSV schema. This cleanup does not add scope, retire IDs, refresh downstream artifacts, or change the PRD v0.2 design basis.

## Acceptance

The human project authority accepted corrected SCA-002 for acceptance recording and downstream refresh planning on 2026-05-03. Acceptance is recorded in `ACCEPTANCE_RECORD.md`, and the downstream planning surface is `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.

## Gate Disposition

The previous agent plan supplied the human-initiated request and authorized workflow posture for SCA-002 implementation. The corrective cleanup from `REVIEW_HANDOFF_TO_NEXT_INSTANCE.md` was applied, then accepted by the human project authority for downstream refresh planning. Downstream execution remains gated by `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.
