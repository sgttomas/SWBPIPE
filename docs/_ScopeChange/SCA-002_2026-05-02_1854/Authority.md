---
amendment_id: SCA-002
doc_kind: scope_change.authority_record
package_role: authoritative_companion_register
created: 2026-05-02
status: accepted_for_downstream_refresh_planning
---

# SCA-002 Authority Record

|Field|Value|
|---|---|
|Authoritative Input|`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`|
|Comparison Brief|`docs/_ScopeChange/OpenPipeStress_PRD_v0.2_Scope_Change_Brief.md`|
|Execution Snapshot|`execution/_ScopeChange/SCA-002_2026-05-02_1854/`|
|Decomposition Path|`execution/_Decomposition/SOFTWARE_DECOMP.md`|
|Accepted Revision|0.5|
|Authority Scope|Decomposition and companion register amendment only|
|Acceptance Record|`execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md`|
|Downstream Refresh Plan|`plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`|

## Design Basis

SCA-002 admits PRD v0.2 as the working product-design basis for OpenPipeStress. The amendment changes the product framing from a narrower open piping stress analysis platform toward an analysis-grade piping design engine and stress-model authoring environment with a full internal solver, schema-backed physical model, immutable model states, analysis runs, deterministic comparison, and handoff packages.

The PRD v0.2 basis does not reduce solver seriousness. It preserves the full analytical engine, open mechanics/private code-data boundary, unit safety, rule-pack structure, validation discipline, local-first posture, and professional responsibility boundary.

## Authority Boundary

This docs-side record is authoritative for the SCA-002 product-design basis. The execution-side snapshot records operational evidence, amendment actions, impact assessment, propagation plan, and handoff state.

SCA-002 does not directly refresh downstream deliverable production documents, DAG files, implementation evidence, lifecycle states, dependency registers, or package-local artifacts. Those surfaces are stale relative to revision 0.5 until refreshed by their owning workflows.

## Acceptance

On 2026-05-03, the human project authority accepted corrected SCA-002 for downstream refresh planning. This acceptance authorizes `ORCHESTRATOR` planning from `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`; it does not authorize Type 2 implementation dispatch or direct downstream artifact refresh without the plan's later gates.
