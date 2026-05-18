---
amendment_id: SCA-002
doc_kind: scope_change.propagation_plan
package_role: snapshot / handoff artifact
created: 2026-05-02
status: accepted_for_downstream_refresh_planning
---

# Propagation Plan

## Direct SCA Writes

|Surface|Action|
|---|---|
|`execution/_Decomposition/SOFTWARE_DECOMP.md`|Update to revision 0.5.|
|`docs/_Registers/ScopeLedger.csv`|Add SOW-064 through SOW-076 rows.|
|`docs/_Registers/Deliverables.csv`|Add 19 new deliverable rows.|
|`docs/_Registers/ContextBudgetQA.csv`|Add context-budget rows for new deliverables.|
|`docs/_ScopeChange/SCA-002_2026-05-02_1854/Authority.md`|Record docs-side SCA authority.|
|`execution/_ScopeChange/SCA-002_2026-05-02_1854/*`|Record immutable operational snapshot.|
|`execution/_ScopeChange/_LATEST.md`|Point latest SCA state to SCA-002.|
|`execution/_Decomposition/_LATEST.md`|Point latest decomposition state to `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 after corrective cleanup.|
|`execution/_ScopeChange/SCA-002_2026-05-02_1854/Supersession_Delta.csv` and `Supersession_Map.csv`|Use canonical supersession schema and regenerate cumulative map with `tools/coordination/accumulate_supersession_map.py`.|

## Corrective Cleanup Propagation

The corrective cleanup is limited to SCA-owned decomposition truth, companion registers, and SCA handoff records. It does not authorize downstream refresh by itself; downstream owners still require human acceptance or explicit authorization before consuming revision 0.5.

## Acceptance Propagation

Human acceptance was recorded on 2026-05-03. `ORCHESTRATOR` may now perform downstream refresh planning from `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`. The plan still gates actual graph refresh, preparation scaffolding, blocker regeneration, lifecycle changes, and Type 2 dispatch behind later approval points.

## Deferred Downstream Refresh

|Owner|Required Work|
|---|---|
|`ORCHESTRATOR`|Plan downstream refresh from revision 0.5, including package/dependency/DAG strategy.|
|`PREPARATION`|Scaffold `PKG-13` through `PKG-16`, `DEL-07-08`, `DEL-08-06`, and new deliverable contexts/status surfaces.|
|`TASK`|Execute only sealed deliverables with explicit write scope and applicable PRD v0.2/SCA-002 constraints.|
|`REVIEW`|Review refreshed deliverables against SCA-002 scope, IP boundary, professional boundary, and tests.|
|`RECONCILIATION`|Detect conflicts between existing DEV-001 dispatch state and revision 0.5 package/deliverable truth.|
|`AUDIT_*`|Run decomposition/register integrity, coverage, protected-data, dependency, and stale-artifact audits.|
|`CHANGE`|Manage git state, staging, commit, and human approval record if requested.|

## Explicit Non-Writes

SCA-002 does not directly update knowledge artifacts, package-local production docs, DAG files, implementation evidence, lifecycle state, dependency registers, downstream dispatch briefs, or existing Type 2 deliverable folders.
