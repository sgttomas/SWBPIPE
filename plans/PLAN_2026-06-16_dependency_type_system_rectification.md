# Dependency Type System Rectification Instructions

**Date:** 2026-06-16
**Scope:** `projects/chirality-piping`
**Status:** implemented; `DAG-007` approved as current graph authority on 2026-06-22
**Owner workflow:** WORKING_ITEMS or RECONCILIATION, with CHANGE closeout after validation

## 2026-06-22 Approval Closeout

The prepared `DAG-007` package was human-approved on 2026-06-22 and
`execution/_DAG/_LATEST.md` now points to `execution/_DAG/DAG-007/`.
`DAG-001` through `DAG-006` remain immutable historical snapshots with legacy
dependency enum values; current dependency writes use the canonical v3.1 enum
surface. A later decomposition revision or SCA remains the trigger for
`DAG-008` or a later graph successor.

## Objective

Rectify the dependency type system used by `chirality-piping` so current dependency registers, current DAG authority, and project documentation comply with the canonical Chirality dependency model.

Canonical authority is the repo-root dependency model:

- `docs/TYPES.md` section 3
- `docs/SPEC.md` section 6
- `skills/dependency-extract/SKILL.md`
- `tools/validation/validate_enum.py`

The canonical core enum set is:

| Field | Allowed values |
|---|---|
| `DependencyClass` | `ANCHOR`, `EXECUTION` |
| `AnchorType` | `IMPLEMENTS_NODE`, `TRACES_TO_REQUIREMENT`, `NOT_APPLICABLE` |
| `Direction` | `UPSTREAM`, `DOWNSTREAM` |
| `DependencyType` | `PREREQUISITE`, `INTERFACE`, `HANDOVER`, `CONSTRAINT`, `ENABLES`, `OTHER` |
| `TargetType` | `DELIVERABLE`, `PACKAGE`, `WBS_NODE`, `REQUIREMENT`, `DOCUMENT`, `EQUIPMENT`, `EXTERNAL`, `UNKNOWN` |
| `Explicitness` | `EXPLICIT`, `IMPLICIT` |
| `SatisfactionStatus` | `TBD`, `PENDING`, `IN_PROGRESS`, `SATISFIED`, `WAIVED`, `NOT_APPLICABLE` |
| `Confidence` | `HIGH`, `MEDIUM`, `LOW` |
| `Origin` | `DECLARED`, `EXTRACTED` |
| `Status` | `ACTIVE`, `RETIRED` |

Do not treat project-specific values such as `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `GUI_PREDECESSOR`, `RUNNER_CONTRACT`, `RESULT_RECORD_MODEL`, `CANDIDATE`, `CONTEXT`, `GRAPH_REVIEW`, `INFERRED_DIRECT`, or `UNKNOWN` as valid values in the core enum fields.

## Impact Summary

The drift is not limited to `DependencyType`. The current `chirality-piping` dependency surfaces use noncanonical values across several core fields.

### Deliverable-local registers

Affected surface:

- `projects/chirality-piping/execution/PKG-*/1_Working/DEL-*/Dependencies.csv`
- paired `projects/chirality-piping/execution/PKG-*/1_Working/DEL-*/_DEPENDENCIES.md`

Measured current impact:

- 93 local `Dependencies.csv` files
- 1,289 local dependency rows
- 92 files have the 29-column v3.1 core schema
- 1 file has the 29 core columns plus extension columns `EstimateImpactClass`, `ConsumerHint`
- no duplicate `DependencyID` values within any individual local register

Core enum drift in local registers:

| Field | Bad rows | Files affected | Examples |
|---|---:|---:|---|
| `AnchorType` | 260 | 24 | `DELIVERABLE`, `GOVERNANCE`, `SCHEMA`, `SCOPE` |
| `Direction` | 6 | 4 | `SELF` |
| `DependencyType` | 270 | 28 | `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `PERSISTENCE_CONTRACT`, `GUI_PREDECESSOR`, `RUNNER_CONTRACT`, `TRACEABILITY` |
| `TargetType` | 10 | 4 | `REFERENCE`, `SCOPE_ITEM` |
| `Explicitness` | 110 | 20 | blank, `INFERRED_DIRECT`, `INFERRED_INDIRECT` |
| `SatisfactionStatus` | 101 | 18 | `UNKNOWN` |
| `Origin` | 277 | 28 | `CONTEXT`, `DECOMPOSITION`, `GRAPH_REVIEW`, `TASK_EXTRACT`, `REFRESH_LOCAL`, `AGENT_INFERENCE` |
| `Status` | 11 | 6 | `CANDIDATE` |

### Aggregate DAG records

Affected surface:

- `projects/chirality-piping/execution/_DAG/DAG-001/` through `DAG-006/`
- especially `DependencyEdges.csv`, `DAG_Audit.md`, `DAG_Audit.json`, `APPROVAL_RECORD.md`, `Cycle_Report.md`, `TopologicalWaves.md`, `dag.json`
- pointer: `projects/chirality-piping/execution/_DAG/_LATEST.md`

Important handling rule:

Do not rewrite `DAG-001` through `DAG-006`. Treat them as historical approved snapshots with a legacy type system. Create a new canonical successor, expected to be `DAG-007`, and update `_LATEST.md` only after the new snapshot passes validation and receives the required human approval.

Measured historical DAG impact:

- 6 historical DAG `DependencyEdges.csv` files
- 5,276 historical DAG rows
- all 6 DAG files have the 29-column v3.1 core schema

Core enum drift in historical DAG edge registers:

| Field | Bad rows | Files affected | Examples |
|---|---:|---:|---|
| `AnchorType` | 2,580 | 6 | `DELIVERABLE` |
| `DependencyType` | 2,580 | 6 | `ARCHITECTURE_BASIS`, `DOMAIN_MODEL`, `GOVERNANCE_PREDECESSOR`, `UNIT_CONTRACT`, `SECURITY_PREDECESSOR`, `EXPORT_CONTRACT` |
| `Explicitness` | 975 | 6 | blank, `INFERRED_DIRECT`, `INFERRED_INDIRECT` |
| `SatisfactionStatus` | 991 | 6 | `UNKNOWN` |
| `Origin` | 2,588 | 6 | `CONTEXT`, `DECOMPOSITION`, `GRAPH_REVIEW`, `SCOPE_CHANGE`, `AGENT_INFERENCE` |
| `Status` | 61 | 6 | `CANDIDATE` |

### Documentation and coordination surfaces

These surfaces need refresh because they currently either omit the canonical dependency enum model or refer to the `DAG-006` legacy/candidate-layer conventions:

- `projects/chirality-piping/docs/TYPES.md`
- `projects/chirality-piping/docs/SPEC.md`
- `projects/chirality-piping/docs/README.md`
- `projects/chirality-piping/docs/PLAN.md`
- `projects/chirality-piping/docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
- `projects/chirality-piping/AGENTS.md`
- `projects/chirality-piping/execution/_Coordination/_COORDINATION.md`
- `projects/chirality-piping/execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
- `projects/chirality-piping/plans/PLAN_2026-06-10_prd_completion.md`
- `projects/chirality-piping/plans/PLAN_2026-06-13_cycle_driven_resolution_doctrine.md`

Do not update all `DAG-006` references mechanically before `DAG-007` exists. First add the canonical dependency-type rule, then update graph-authority references when the new canonical DAG has been built and approved.

### Tooling surfaces

Impacted tools:

- `projects/chirality-piping/tools/validation/validate_dependencies_schema.py`
- repo-root `tools/coordination/audit_dag.py`
- repo-root `tools/coordination/materialize_local_dependencies.py`
- repo-root `tools/coordination/build_dev001_blocker_queue.py`
- tests under repo-root `tools/coordination/test_*.py`

The project-local validator currently checks required columns and field counts only. It does not reject noncanonical enum values. Add enum validation, row-rule validation, and tests before regenerating data.

Root coordination tools currently understand `Status=CANDIDATE` and `DependencyType=ARCHITECTURE_BASIS`. If those tools are still used for `chirality-piping`, update them in a backwards-compatible way. They must be able to read old DAG snapshots for historical reporting while producing and validating a canonical current DAG.

## Required Semantic Decisions

The implementation must preserve dependency meaning while moving noncanonical labels out of core enum fields.

### Candidate edges

`Status=CANDIDATE` is not canonical. `Status` tracks extraction lifecycle only: `ACTIVE` or `RETIRED`.

For the canonical current DAG:

1. Do not use `Status=CANDIDATE` in `DependencyEdges.csv`.
2. Keep approved/gating execution edges in `DependencyEdges.csv` with `Status=ACTIVE`.
3. Move non-gating candidate edges to a separate candidate worklist or review packet, for example `CandidateDependencyEdges.csv` or `DAG-007_CandidateEdgeWorklist.csv`.
4. If a single machine graph still needs candidate visibility, use a non-authoritative derived view that is explicitly not a v3.1 dependency register.
5. Do not promote any candidate edge to gating status unless a human approval record explicitly does so and graph validation passes afterward.

### Legacy dependency-type labels

Do not leave project-specific labels in `DependencyType`.

Map them by relationship semantics:

| Legacy pattern | Canonical target |
|---|---|
| `*_PREDECESSOR`, `ARCHITECTURE_BASIS`, `SOURCE_BASIS`, `IDENTITY_BASIS` | Usually `PREREQUISITE`, unless the evidence is only a constraint or trace anchor |
| `*_CONTRACT`, `SERVICE_API`, `ADAPTER_FRAMEWORK`, `PLUGIN_CONTRACT`, `EXPORT_PROFILE` | Usually `INTERFACE` or `CONSTRAINT`; choose from source statement, not from the label alone |
| `CONSUMED_BY` | Usually invert direction and use `HANDOVER`, or use `ENABLES` if the host enables downstream work |
| `TRACEABILITY`, `SCOPE_ITEM` targets | Usually `ANCHOR` plus `TRACES_TO_REQUIREMENT`, or `TargetType=REQUIREMENT` on an execution row only if it is truly an execution dependency |
| Ambiguous labels | Use `OTHER` only if no stronger canonical type is supported; preserve the legacy value in `Notes` |

Preserve the original label in `Notes` as `legacy_dependency_type=<value>` or in a clearly documented non-core extension column. Do not create new core enum values.

### Anchor rows

Enforce these rules:

1. `ANCHOR` rows use `DependencyType=OTHER`.
2. `ANCHOR` rows use `AnchorType=IMPLEMENTS_NODE` or `TRACES_TO_REQUIREMENT`.
3. `EXECUTION` rows use `AnchorType=NOT_APPLICABLE`.
4. `AnchorType=DELIVERABLE`, `GOVERNANCE`, `SCHEMA`, or `SCOPE` must not remain.

### Direction and self edges

`Direction=SELF` is not canonical.

For each self row:

1. If it is actually a traceability link, convert it to an `ANCHOR` row.
2. If it is an internal note with no cross-deliverable dependency, retire the row or move it to documentation outside the dependency register.
3. If it is a real execution relationship, resolve it to `UPSTREAM` or `DOWNSTREAM` relative to the host deliverable and make the target non-self if possible.

Do not keep execution self-loops in the active DAG.

### Other enum normalizations

Apply these rules:

| Field | Legacy value | Canonical value |
|---|---|---|
| `TargetType` | `REFERENCE` | `DOCUMENT` unless the target is not a document |
| `TargetType` | `SCOPE_ITEM` | `REQUIREMENT` or `WBS_NODE`, depending on source |
| `Explicitness` | `INFERRED_DIRECT`, `INFERRED_INDIRECT` | `IMPLICIT` |
| `Explicitness` | blank | `IMPLICIT` unless the row has a direct source statement |
| `SatisfactionStatus` | `UNKNOWN` | `TBD` |
| `Origin` | agent/system labels such as `CONTEXT`, `DECOMPOSITION`, `GRAPH_REVIEW`, `TASK_EXTRACT`, `SCOPE_CHANGE` | `EXTRACTED`, unless the row is truly human-declared |
| `Status` | `CANDIDATE` | Do not normalize blindly; move to candidate worklist or set `ACTIVE` only if approved as a current observed edge |

Preserve legacy details in `Notes`; do not discard provenance.

## Implementation Phases

### Phase 0 - Freeze and audit baseline

1. Record `git status --short`.
2. Snapshot the current drift report to a new evidence folder, for example:
   - `projects/chirality-piping/execution/_Reconciliation/DependencyTypeSystem/TYPE_RECTIFICATION_2026-06-16/Evidence/`
3. Generate CSV reports:
   - `local_enum_drift.csv`
   - `dag_enum_drift.csv`
   - `local_files_with_noncanonical_values.csv`
   - `candidate_edges_inventory.csv`
   - `legacy_dependency_type_inventory.csv`
4. Do not edit any dependency artifact until these reports exist.

### Phase 1 - Update documentation authority

Update `projects/chirality-piping/docs/TYPES.md`:

- Add a dependency vocabulary section that explicitly adopts the repo-root canonical dependency model.
- State that `chirality-piping` may add domain context only in `Notes`, source evidence, or documented extension columns, not in core enum fields.
- State that candidate/non-gating graph decisions are graph-governance dispositions, not `Status` values.

Update `projects/chirality-piping/docs/SPEC.md`:

- Add or refresh a `Dependencies.csv v3.1` section.
- Include the 29 required columns, canonical enum values, row rules, and evidence requirements.
- State that rows are retired, not deleted.
- State that `DAG-001` through `DAG-006` are historical legacy snapshots and that current successors must be canonical.

Update workflow docs:

- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`: dependency-resolution phase must require canonical enum validation.
- `docs/PLAN.md`: route "what depends on what" to the current approved canonical DAG. As of the 2026-06-22 approval closeout, that graph is `DAG-007`.
- `docs/README.md`: mention that current dependency authority must pass canonical enum validation.
- `AGENTS.md`: add a project-local instruction that dependency edits must preserve root canonical enum fields.
- `_COORDINATION.md` and `NEXT_INSTANCE_PROMPT.md`: after `DAG-007` approval, update current DAG references from `DAG-006` to `DAG-007`.
- `plans/PLAN_2026-06-10_prd_completion.md` and `PLAN_2026-06-13_cycle_driven_resolution_doctrine.md`: replace the old "DAG-007 only when Phase D changes structure" trigger with the new immediate reason for `DAG-007`: canonical dependency type-system rectification. Preserve the Phase D graph refresh as a later possible `DAG-008` or successor trigger.

### Phase 2 - Harden validators and tools

Update `projects/chirality-piping/tools/validation/validate_dependencies_schema.py` or add a sibling validator so that validation fails on:

- missing required v3.1 columns
- invalid core enum values
- `ANCHOR` row with `DependencyType` other than `OTHER`
- `ANCHOR` row with `AnchorType=NOT_APPLICABLE`
- `EXECUTION` row with `AnchorType` other than `NOT_APPLICABLE`
- `TargetType=DELIVERABLE` without `TargetDeliverableID`
- non-deliverable targets with populated `TargetDeliverableID`, unless explicitly justified and documented
- `Direction` outside `UPSTREAM`/`DOWNSTREAM`
- `Status=CANDIDATE`

Add tests covering:

- a valid canonical row set
- each invalid legacy enum family named above
- extension columns remaining non-breaking
- candidate edges being rejected from canonical `Dependencies.csv`

Update root coordination tools only if needed and only in a backwards-compatible way:

- `audit_dag.py` must read old snapshots but validate new snapshots in canonical mode.
- `materialize_local_dependencies.py` must not write `Status=CANDIDATE` or noncanonical `DependencyType` values into local `Dependencies.csv`.
- `build_dev001_blocker_queue.py` must not depend on `Status=CANDIDATE` in a canonical current graph.

If touching repo-root tools, call out the cross-project impact in the run record.

### Phase 3 - Normalize deliverable-local registers

For each local `Dependencies.csv`:

1. Normalize core enum values according to this plan.
2. Preserve legacy labels and provenance in `Notes`.
3. Move non-gating candidates out of the core register or mark them only through a documented non-authoritative candidate worklist.
4. Ensure `DependencyID` values remain stable unless a row is split into distinct canonical rows.
5. Do not delete rows. If a row no longer belongs in the dependency register, mark it `RETIRED` and explain why in `Notes`.
6. Regenerate the paired `_DEPENDENCIES.md` summary so it reports canonical classes, dependency types, statuses, and candidate-worklist references.
7. Add a run record under each touched deliverable only if that is the local convention for dependency refresh work. Otherwise write one aggregate reconciliation run record that names every touched register.

Run the hardened validator on all 93 local registers.

### Phase 4 - Build canonical DAG successor

Create a new DAG snapshot, expected path:

- `projects/chirality-piping/execution/_DAG/DAG-007/`

Inputs:

- normalized deliverable-local `Dependencies.csv`
- existing `DAG-006` topology as semantic baseline
- candidate edge inventory from Phase 0
- cycle-driven resolution doctrine

Outputs:

- `DependencyEdges.csv` with only canonical core enum values
- `DeliverableNodes.csv`
- `dag.json`
- `DAG_Audit.md`
- `DAG_Audit.json`
- `Cycle_Report.md`
- `TopologicalWaves.md`
- `DAG-007_APPROVAL_REVIEW_PACKET.md`
- `APPROVAL_RECORD.md` after human approval
- candidate worklist if candidate rows still exist conceptually

Rules:

1. Do not mutate `DAG-001` through `DAG-006`.
2. Preserve approved active topology from `DAG-006` unless a human explicitly approves a graph change.
3. Keep candidate edges non-gating and outside the canonical active edge register.
4. If candidate edges are kept in a worklist, that worklist must say it is not a v3.1 dependency register.
5. Active graph must have zero active SCCs before approval.
6. Any cycle-participating candidate cluster must be documented with a resolution state: unresolved candidate, decompose, invert, merge, or cut.

### Phase 5 - Refresh current documentation and pointers

After `DAG-007` exists and passes validation:

1. Update `execution/_DAG/_LATEST.md` to point to `DAG-007` only after approval.
2. Update `_COORDINATION.md`, `NEXT_INSTANCE_PROMPT.md`, `docs/PLAN.md`, `docs/README.md`, and workflow docs to refer to `DAG-007` as current authority.
3. Update any local plan text that says a `DAG-007` refresh is not warranted before Phase D. It is warranted now by type-system rectification.
4. Preserve historical references to `DAG-006` where the sentence is explicitly historical.
5. Refresh any generated dependency documentation that reports old type counts.

### Phase 6 - Validation and closeout

Required validation:

```bash
python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py <each local Dependencies.csv>
python3 projects/chirality-piping/tools/validation/validate_dependencies_schema.py projects/chirality-piping/execution/_DAG/DAG-007/DependencyEdges.csv
python3 tools/coordination/audit_dag.py --dag-dir projects/chirality-piping/execution/_DAG/DAG-007 --strict
python3 -m json.tool projects/chirality-piping/execution/_DAG/DAG-007/dag.json
python3 -m json.tool projects/chirality-piping/execution/_DAG/DAG-007/DAG_Audit.json
```

Also rerun any project-local tests added for the validator and any root coordination-tool tests touched during Phase 2.

Closeout evidence must include:

- before/after enum drift summary
- list of touched local dependency registers
- list of candidate edges moved out of canonical registers
- DAG-007 validation results
- documentation refresh summary
- unresolved blockers or human rulings still required

## Acceptance Criteria

The work is complete only when all criteria below hold:

1. Every current local `Dependencies.csv` under `projects/chirality-piping/execution/PKG-*/1_Working/DEL-*` uses only canonical core enum values.
2. Every paired `_DEPENDENCIES.md` summary reports canonical values and does not present legacy labels as current dependency types.
3. The current approved DAG successor uses only canonical core enum values in `DependencyEdges.csv`.
4. Candidate/non-gating edges are represented outside the canonical core register or in a documented non-authoritative view; `Status=CANDIDATE` no longer appears in any current v3.1 dependency register.
5. Historical `DAG-001` through `DAG-006` remain unmodified and are clearly understood as legacy snapshots.
6. `docs/TYPES.md` and `docs/SPEC.md` in `chirality-piping` define or route to the canonical dependency model.
7. Project workflow docs and coordination prompts point to the current canonical DAG after approval.
8. Validator coverage catches all legacy enum families listed in this plan.
9. The active graph remains acyclic for approved active edges.
10. No candidate edge is promoted, cut, or merged without the required human gate.

## Explicit Non-Goals

- Do not change product code behavior.
- Do not change deliverable lifecycle states.
- Do not issue or retire deliverables.
- Do not make release-readiness, professional approval, certification, sealing, authentication, or code-compliance claims.
- Do not rewrite historical approved DAG snapshots.
- Do not silently linearize cycles.
- Do not discard legacy provenance; preserve it in notes, worklists, or audit evidence.
