---
doc_id: PLAN-SCA-001-DOWNSTREAM-FOUR-DOC-REFRESH
doc_kind: plan.workflow
status: draft
created: 2026-04-30
scope_change: SCA-001
---

# SCA-001 Downstream Four-Document Refresh Plan

## Purpose

This plan records the follow-on workflow after `SCA-001` propagates `PKG-00 - Software Architecture Runway` into downstream `_CONTEXT.md` files.

The immediate SCA-001 propagation updates decomposition, registers, coordination state, and all downstream deliverable contexts. It does not directly edit downstream production artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`

Those four-document kits are deliverable production artifacts. Updating them is Type 2 deliverable execution, not a scope-change propagation step.

## Oversight Answer

There is an `ORCHESTRATOR` agent in this project (`agents/AGENT_ORCHESTRATOR.md`). It can oversee the broad control loop, scan state, dispatch bounded setup/document tasks, and route handoffs.

There does not appear to be a single dedicated agent whose exact named responsibility is:

> "After SCA-001 context injection, refresh all downstream four-document kits against the injected architecture basis."

So the correct interpretation is mixed:

- **Yes, an overseeing role exists:** `ORCHESTRATOR` is the appropriate coordinator for the pass.
- **No, there is not a specialized standalone agent for this exact workflow:** the work must be expressed as an ORCHESTRATOR-run tranche using existing `TASK`, `REVIEW`, `RECONCILIATION`, `AUDIT_*`, and `CHANGE` roles.

The human project authority remains responsible for approving the tranche, any batching strategy, and final acceptance.

## Governance Boundary

`_CONTEXT.md` files are dispatch and control surfaces. They tell future sealed TASK briefs what constraints apply.

The four-document kit files are production deliverables. Editing them means the downstream deliverable content has been revised, and must follow the Type 2 rule from `AGENTS.md`:

- one `DeliverableID`;
- one parent `PackageID`;
- scope items and objectives from `_Registers/Deliverables.csv`;
- applicable invariants from `CONTRACT.md`;
- acceptance criteria from `_CONTEXT.md` or sealed brief;
- explicit write scope.

Bulk-editing all downstream four-document kits during SCA-001 would bypass that execution boundary and could falsely imply that `PKG-00` `SEMANTIC_READY` material has already been fully issued into every deliverable.

## Intended Workflow

### Phase 0 - Prerequisite

Complete SCA-001 Gate 5 propagation first.

Expected SCA-001 outputs:

- `docs/_Decomposition/SOFTWARE_DECOMP.md` updated to revision `0.4`;
- companion registers updated without changing row counts;
- all 65 downstream `_CONTEXT.md` files receive an architecture-basis injection block;
- coordination state records the new decomposition basis;
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`;
- downstream deliverable lifecycle states are not advanced merely by context injection.

### Phase 1 - ORCHESTRATOR Tranche Setup

`ORCHESTRATOR` should scan the workspace and establish a human-approved tranche for downstream production-doc refresh.

The scan should report:

- package count;
- deliverable count;
- lifecycle distribution;
- number of downstream `_CONTEXT.md` files containing `SCA-001`;
- which downstream deliverables already have four-document kits;
- which downstream deliverables are missing any of `Datasheet.md`, `Specification.md`, `Guidance.md`, or `Procedure.md`;
- whether dependency/DAG blocker computation remains disabled.

The human should then approve a batching policy, such as:

- package-by-package;
- architecture-critical first;
- GUI/API/persistence first;
- validation/security last;
- all 65, if the tooling and review capacity support it.

### Phase 2 - TASK Dispatch Per Deliverable

For each selected deliverable, `ORCHESTRATOR` dispatches one bounded `TASK`.

Each sealed TASK brief should include:

- `DeliverableID`;
- `PackageID`;
- `ScopePath`;
- applicable SOW/OBJ rows from `_Registers/Deliverables.csv`;
- applicable invariants from `docs/CONTRACT.md`;
- the deliverable `_CONTEXT.md` after SCA-001 injection;
- explicit write scope limited to that deliverable folder;
- the applicable architecture basis IDs (`AB-00-*`);
- the resolved baseline choices from SCA-001;
- the remaining implementation-level TBDs that must not be over-resolved.

The TASK may update only the deliverable-local production artifacts authorized by the brief.

### Phase 3 - Four-Document Refresh Rules

The TASK should refresh the four-document kit to align with the injected architecture basis.

Required behavior:

- preserve deliverable identity, package identity, scope items, and objectives;
- carry forward applicable architecture-basis constraints;
- resolve TBDs only where SCA-001 or the sealed brief authorizes resolution;
- leave implementation-specific choices as `TBD` where they depend on later coding or human approval;
- avoid copying full `PKG-00` prose into downstream artifacts;
- preserve the protected data boundary;
- avoid compliance/certification claims;
- record run evidence in the deliverable folder.

The refresh should distinguish:

- architecture constraints that are now binding for implementation briefs;
- human-approved baseline choices;
- implementation decisions still deferred;
- deliverable-specific assumptions that need later review.

### Phase 4 - REVIEW

After each deliverable or approved batch, `REVIEW` checks the output against:

- the sealed TASK brief;
- the deliverable `_CONTEXT.md`;
- `docs/CONTRACT.md`;
- protected-data rules;
- SCA-001 architecture basis IDs;
- acceptance criteria;
- any generated tests or validation notes.

REVIEW should identify:

- architecture drift;
- stale TBDs that should have been resolved;
- over-resolved TBDs that should remain deferred;
- contradictions between the four documents;
- missing acceptance evidence.

### Phase 5 - RECONCILIATION

When a batch touches interfaces across packages, `RECONCILIATION` should run before broader propagation continues.

Examples requiring reconciliation:

- schema/API contract changes affecting multiple packages;
- persistence format assumptions;
- GUI command/service boundary assumptions;
- diagnostic/result-envelope shape;
- adapter/plugin boundary assumptions;
- test strategy or release-gate assumptions.

RECONCILIATION does not edit repo files directly. It produces findings, routing, and handoff requests.

### Phase 6 - AUDIT

Run bounded audits after each meaningful batch and after the full refresh.

Likely audits:

- decomposition/register coverage audit;
- governance/protected-data audit;
- dependency closure audit only after a human-approved DAG exists;
- epistemic/TBD audit for stale or over-resolved assumptions;
- four-document presence/consistency validation.

Audit outputs should be treated as evidence for human review, not automatic final acceptance.

### Phase 7 - CHANGE Handoff

`CHANGE` owns project file-state management after the human approves the refreshed outputs.

CHANGE should:

- present changed files and material diffs;
- verify coordination state reflects the actual handoff state;
- stage/commit only after explicit human approval, if this workspace is under git;
- record any snapshot or handoff artifacts required by the active workflow.

If the workspace is not a git repository, CHANGE should still provide a file-state report and handoff summary.

## Proposed Initial Batching

A conservative refresh order is:

1. `PKG-02` domain model, schema, and persistence-facing deliverables.
2. `PKG-06` rule-pack/data-contract deliverables.
3. `PKG-10` API, adapter, packaging, and headless runner deliverables.
4. `PKG-07` GUI workflow deliverables.
5. `PKG-04` and `PKG-05` solver and analysis deliverables.
6. `PKG-08` reporting and audit deliverables.
7. `PKG-09` validation and release-gate deliverables.
8. `PKG-12` privacy/security/telemetry deliverables.
9. `PKG-01`, `PKG-03`, and `PKG-11` governance, libraries, and documentation deliverables as needed.

This order prioritizes contract surfaces before user-facing and release-facing documents.

## Non-Goals

This plan does not:

- mark `PKG-00` as `ISSUED`;
- edit downstream four-document kits by itself;
- compute blockers before a human-approved DAG exists;
- create protected standards/code data;
- authorize implementation code changes;
- replace human engineering review.

## Open Decisions

| ID | Decision | Owner |
|---|---|---|
| PLAN-SCA001-OD-001 | Confirm whether ORCHESTRATOR should run this as the next tranche after SCA-001 Gate 5. | Human |
| PLAN-SCA001-OD-002 | Choose batching policy for 65 downstream deliverables. | Human |
| PLAN-SCA001-OD-003 | Decide whether to create a dedicated named workflow/profile for this refresh pattern. | Human / SOFTWARE_DECOMP |
| PLAN-SCA001-OD-004 | Decide which audits are mandatory per batch versus only at final closeout. | Human / REVIEW / AUDIT_* |

## Recommended Next Action

After SCA-001 Gate 5 closes, run `ORCHESTRATOR` against this plan and request a concrete tranche proposal for downstream four-document refresh. The first tranche should be small enough to prove the sealed TASK brief format, review checks, and reconciliation routing before scaling to all 65 downstream deliverables.
