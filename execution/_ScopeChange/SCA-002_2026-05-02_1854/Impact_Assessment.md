---
amendment_id: SCA-002
doc_kind: scope_change.impact_assessment
package_role: snapshot / handoff artifact
created: 2026-05-02
status: accepted_for_downstream_refresh_planning
---

# Impact Assessment

## Evidence Basis

|Surface|Package Role|Impact|
|---|---|---|
|`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`|authoritative companion register|Authoritative SCA-002 product-design basis.|
|`docs/_ScopeChange/OpenPipeStress_PRD_v0.2_Scope_Change_Brief.md`|authoritative companion register|Parsed comparison evidence against earlier PRD framing.|
|`execution/_Decomposition/SOFTWARE_DECOMP.md`|working surface|Updated from revision 0.4 to 0.5.|
|`docs/_Registers/ScopeLedger.csv`|authoritative companion register|Adds SOW-064 through SOW-076.|
|`docs/_Registers/Deliverables.csv`|authoritative companion register|Adds DEL-07-08, DEL-08-06, and DEL-13-01 through DEL-16-04.|
|`docs/_Registers/ContextBudgetQA.csv`|authoritative companion register|Adds context-budget records for the new deliverables.|
|`execution/_Coordination/*`|derived / downstream coordination artifact|Stale relative to revision 0.5; not directly modified by SCA-002.|
|Package/deliverable production folders|derived / downstream execution artifacts|Stale or missing for new packages/deliverables; not directly modified by SCA-002.|
|Dependency/DAG/lifecycle/evidence surfaces|derived / downstream coordination artifacts|Must be refreshed by owning workflows; not directly modified by SCA-002.|

## Corrective Cleanup Impact

The hindsight cleanup corrected internal decomposition/register consistency only. Direct scope coverage is now package-local, scope-ledger deliverable mappings are reciprocal with `Deliverables.csv`, package-table assignments match the ledger, `execution/_Decomposition/_LATEST.md` points to revision 0.5, and SCA-002 supersession CSVs use the canonical tool schema. Counts remain unchanged: 76 scope items, 92 deliverables, 92 context-budget rows, and context envelopes `S=9, M=66, L=17, XL=0`.

## Acceptance Impact

The 2026-05-03 human acceptance changes SCA-002 from pending acceptance to accepted for downstream refresh planning. It does not change the stale state of downstream artifacts; it authorizes ORCHESTRATOR planning from `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.

## Product Scope Impact

SCA-002 preserves existing solver, rule-pack, protected-data, reporting, validation, build, docs, security, and professional-boundary scope. It adds first-class decomposition coverage for:

- schema-backed physical design model as the source of truth;
- physical-to-analytical transformation with warnings and traceability;
- design knowledge and constraint validation;
- structured model operations for GUI and agent proposals;
- immutable model states and analysis runs;
- deterministic model-state and analysis-run comparison;
- canonical handoff packages and external-prover workflow metadata;
- GUI design-authoring and comparison workspaces;
- reporting from state/run/comparison/handoff records.

## Structural Impact

Existing IDs are preserved. New package IDs begin at `PKG-13`; new deliverables preserve deterministic `DEL-XX-YY` coupling. No existing package, deliverable, or scope item is retired or renumbered.

## Downstream Impact

Downstream surfaces must be treated as stale until refreshed. Required follow-on owners include `ORCHESTRATOR`, `PREPARATION`, `TASK`, `REVIEW`, `RECONCILIATION`, `AUDIT_*`, and `CHANGE`.
