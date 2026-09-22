# Context: DEL-16-06

**Application basis:** SCA-011 Group 2 amendment; audited poststate acceptance remains pending Group 3. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:f1d6474e35d0fd42800ff8acfbb3148e7ade9b43aef72f9cab2df69a26786577`.

**Deliverable ID:** DEL-16-06
**Name:** Controlled model operation application
**Package ID:** PKG-16
**Package Name:** Model Operation and Agent Proposal Framework
**Discipline:** Software
**Type:** BACKEND_FEATURE_SLICE
**Responsible Party:** Project owner; bounded executing role assigned by the work graph

## Description

Apply only explicitly accepted, contract-valid structured operations to their current model basis through the sole DEC-020 operation engine; preserve rejected-state nonmutation and hand actual changed-state outcomes to audit and persistence owners.

## Anticipated Artifacts

- controlled operation application
- native/wasm conformance
- audit and persistence handoff evidence

## Scope Coverage

- SOW-069
- SOW-070

## Objective Support

- OBJ-015

## Context Envelope

- **Envelope:** M
- **Notes:** One mutation seam; schema, preview, user acceptance, audit policy and durable persistence remain separate responsibilities.
- One finite interface or interaction family per bounded TASK; expand only after sizing review.

## Decomposition and Registers

- Decomposition: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:f1d6474e35d0fd42800ff8acfbb3148e7ade9b43aef72f9cab2df69a26786577`
- Deliverables: `docs/_Registers/Deliverables.csv` row DEL-16-06
- Context budget: `docs/_Registers/ContextBudgetQA.csv` row DEL-16-06
- Scope: `docs/_Registers/ScopeLedger.csv` rows SOW-069,SOW-070

## Boundaries

DEL-16-01 owns schema/taxonomy, DEL-16-02 validation/preview, DEL-16-03 acceptance/audit policy, DEL-02-05 persistence and DEL-16-04 professional-boundary controls. A user actor-type string is not identity evidence. Retention, actor policy and unsupported-operation decisions remain open; no live agent/provider activation.
