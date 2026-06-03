---
doc_id: OPS-AGENTIC-DEVELOPMENT-WORKFLOW
doc_kind: governance.agentic_workflow
status: draft
created: 2026-04-30
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: depends_on
    to: OPS-SOFTWARE-DECOMP
---

# Agentic Development Workflow

This document is the execution-discipline companion to
`execution/_Coordination/_COORDINATION.md`. The active session loop, work
selection rules, authority intake tiers, status discovery, DAG-guided context
selection, fan-in, validation, and handoff rules live in `_COORDINATION.md`.
Use this document after a deliverable or tranche has been selected to keep
agent execution bounded, evidenced, and reviewable.

For a contributor-facing walkthrough of this workflow, see
`docs/contributor_guide/index.md`. That guide explains the existing process; it
does not add authority beyond this workflow, the governing contract, sealed
briefs, or human project-authority decisions.

## 1. Roles

| Role | Function | Authority |
|---|---|---|
| Human project authority | Confirms scope, resolves ambiguity, accepts deliverables. | Binding decisions. |
| Type 1 persona agent | Decomposes, routes, reconciles, prepares briefs. | Draft/proposal authority only. |
| Type 2 specialist agent | Executes one sealed deliverable with bounded context. | Draft/proposal authority only. |
| Deterministic tools | Validate schemas, tests, hashes, reports, lint rules. | Evidence only. |

## 2. Standard flow

1. Human supplies intent, review direction, or change request.
2. The active coordination loop in `_COORDINATION.md` performs authority
   intake, local status discovery, and DAG-guided candidate selection.
3. `SOFTWARE_DECOMP` and relevant register rows define scope; deliverable-local
   `_STATUS.md` files define lifecycle state for work selection.
4. DAG authority identifies upstream and downstream context, but local
   deliverable folders remain the evidence surface for artifact presence,
   lifecycle history, and working memory.
5. Human project authority approves the bounded tranche or deliverable scope
   when required.
6. A Type 2 `TASK` agent or equivalent bounded worker receives one explicit
   deliverable context and declared write scope.
7. The worker produces artifacts, tests, evidence, and local run records.
8. Review checks scope, tests, IP boundary, provenance, warnings, dependencies,
   and warrant for claims.
9. Human project authority accepts, rejects, requests revision, or explicitly
   defers remaining issues where governance requires a gate.

## 3. Deliverable document kit

Each deliverable folder should contain:

| File | Purpose |
|---|---|
| `_STATUS.md` | Lifecycle state and history. |
| `_CONTEXT.md` | Identity, package, scope items, objectives, acceptance criteria. |
| `_REFERENCES.md` | Source documents and design references. |
| `_DEPENDENCIES.md` | Upstream/downstream dependencies and local dependency evidence. |
| `Datasheet.md` | Key parameters and structured metadata. |
| `Specification.md` | Requirements and acceptance criteria. |
| `Guidance.md` | Rationale, principles, and implementation guidance. |
| `Procedure.md` | Step-by-step execution procedure and checks. |

Some deliverables also carry `Dependencies.csv`, `Review_Findings.csv`,
semantic or lensing files, `_run_records/**`, implementation artifacts, tests,
or generated evidence. Inspect those local files before relying on project-wide
summaries.

## 4. Type 2 execution rules

A Type 2 agent must:

- execute only the assigned `DEL-XX-YY`;
- not expand scope silently;
- not introduce protected public data;
- label unknowns as `TBD`;
- surface conflicts and missing inputs;
- produce tests or evidence appropriate to the deliverable type;
- update only its declared write scope and deliverable-local evidence surface;
- return a concise run summary with artifacts, evidence, warnings, and open
  issues;
- query only the relevant rows from `docs/_Registers/*.csv` rather than treating
  full-register reading as evidence of comprehension;
- use the active DAG only to discover relationship context, never as a
  substitute for inspecting local deliverable artifacts;
- avoid creating project-wide readiness or blocker authority unless the human
  explicitly requests that derivative package;
- avoid lifecycle transitions, candidate-edge promotion, release claims,
  professional claims, acceptance records, and commits unless explicitly
  authorized by the human project authority.

## 5. Review checklist

A deliverable is ready for human acceptance only when:

- deliverable ID and package match the decomposition;
- anticipated artifacts exist or deferrals are recorded;
- tests/lints pass where applicable;
- no suspected protected data is present;
- private data paths are respected;
- assumptions and warnings are visible;
- cross-deliverable dependencies are recorded;
- local status, memory, run records, dependency files, and review files are
  internally consistent or discrepancies are surfaced;
- no claim exceeds its warrant.

## 6. Change management

Material changes to scope, package boundaries, IDs, solver behavior, rule-pack
semantics, lifecycle state, graph authority, candidate-edge status, release
posture, or data-boundary policy require a decomposition amendment, owning
workflow update, or explicit human decision as applicable. Do not renumber
stable IDs unless the human explicitly requests it.
