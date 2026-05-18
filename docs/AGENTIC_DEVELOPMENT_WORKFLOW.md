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

1. Human supplies intent/PRD or change request.
2. SOFTWARE_DECOMP updates `_Decomposition/SOFTWARE_DECOMP.md` and registers.
3. Human confirms package/deliverable scope.
4. PREPARATION scaffolds deliverable folders.
5. Type 2 TASK agent receives one deliverable context.
6. TASK agent produces artifacts and evidence.
7. Review checks scope, tests, IP boundary, provenance, and warnings.
8. Human accepts, rejects, or requests revision.

## 3. Deliverable document kit

Each deliverable folder should contain:

| File | Purpose |
|---|---|
| `_STATUS.md` | Lifecycle state and history. |
| `_CONTEXT.md` | Identity, package, scope items, objectives, acceptance criteria. |
| `_REFERENCES.md` | Source documents and design references. |
| `_DEPENDENCIES.md` | Upstream/downstream dependencies and blockers. |
| `Datasheet.md` | Key parameters and structured metadata. |
| `Specification.md` | Requirements and acceptance criteria. |
| `Guidance.md` | Rationale, principles, and implementation guidance. |
| `Procedure.md` | Step-by-step execution procedure and checks. |

## 4. Type 2 execution rules

A Type 2 agent must:

- execute only the assigned `DEL-XX-YY`;
- not expand scope silently;
- not introduce protected public data;
- label unknowns as `TBD`;
- surface conflicts and missing inputs;
- produce tests or evidence appropriate to the deliverable type;
- update only its declared write scope;
- return a concise run summary with artifacts, evidence, warnings, and open issues.

## 5. Review checklist

A deliverable is ready for human acceptance only when:

- deliverable ID and package match the decomposition;
- anticipated artifacts exist or deferrals are recorded;
- tests/lints pass where applicable;
- no suspected protected data is present;
- private data paths are respected;
- assumptions and warnings are visible;
- cross-deliverable dependencies are recorded;
- no claim exceeds its warrant.

## 6. Change management

Material changes to scope, package boundaries, IDs, solver behavior, rule-pack semantics, or data-boundary policy require a decomposition amendment and human decision. Do not renumber stable IDs unless the human explicitly requests it.
