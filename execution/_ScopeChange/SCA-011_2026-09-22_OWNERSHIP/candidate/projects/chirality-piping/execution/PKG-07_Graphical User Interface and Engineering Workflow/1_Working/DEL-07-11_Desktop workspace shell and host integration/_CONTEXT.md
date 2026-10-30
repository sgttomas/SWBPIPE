# Context: DEL-07-11

**Candidate only:** SCA-011 Group 2 postimage; no active deliverable or accepted lifecycle is created by this draft. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:0581fd5fec1f56ef08d958bc0895557d4fd7ca3d66d2579b70fea03d81e074f8`.

**Deliverable ID:** DEL-07-11
**Name:** Desktop workspace shell and host integration
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Discipline:** Software
**Type:** UX_UI_SLICE
**Responsible Party:** Project owner; bounded executing role assigned by the work graph

## Description

Provide the desktop workspace shell and native-host boundary for project/session navigation, shared selection/history coordination, panel mounting, jobs and explicit file operations, preserving durable/transient separation and owned service routes.

## Anticipated Artifacts

- workspace shell
- session coordinator
- native-host route and recovery evidence

## Scope Coverage

- SOW-078

## Objective Support

- OBJ-006
- OBJ-010
- OBJ-015

## Context Envelope

- **Envelope:** L
- **Notes:** Own common hosting and coordination only; feature-panel semantics, storage, solve UX and architecture remain with their owners.
- One finite interface or interaction family per bounded TASK; expand only after sizing review.

## Decomposition and Registers

- Decomposition: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:0581fd5fec1f56ef08d958bc0895557d4fd7ca3d66d2579b70fea03d81e074f8`
- Deliverables: `docs/_Registers/Deliverables.csv` row DEL-07-11
- Context budget: `docs/_Registers/ContextBudgetQA.csv` row DEL-07-11
- Scope: `docs/_Registers/ScopeLedger.csv` rows SOW-078

## Boundaries

DEL-07-08 retains operation review/apply interaction and ledger presentation; DEL-16-06 owns application execution; DEL-07-07 owns solve-progress/diagnostic UX. DEL-00-03/05/07 constrain architecture. Shell hosting transfers no feature semantics, persistence policy or security-policy authority.
