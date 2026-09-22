# Context: DEL-04-07

**Application basis:** SCA-011 Group 2 amendment; audited poststate acceptance is recorded in the SCA-011 Group 3 closure. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984`.

**Deliverable ID:** DEL-04-07
**Name:** Product solve integration and nonlinear orchestration
**Package ID:** PKG-04
**Package Name:** Solver Core and Numerical Methods
**Discipline:** Software
**Type:** BACKEND_FEATURE_SLICE
**Responsible Party:** Project owner; bounded executing role assigned by the work graph

## Description

Compose accepted model, units, section/load/support contracts and kernel outputs into the product solve service, preserving input identity, missing-data diagnostics and output provenance; execute the DEC-044 assembled nonlinear loop under the existing convergence policy.

## Anticipated Artifacts

- product solve service
- nonlinear orchestration
- adapter conformance evidence

## Scope Coverage

- SOW-005
- SOW-011
- SOW-012
- SOW-052
- SOW-053

## Objective Support

- OBJ-003
- OBJ-012

## Context Envelope

- **Envelope:** L
- **Notes:** Finite integration boundary; mathematical primitives, convergence policy and engineering acceptance remain with their owners.
- One finite interface or interaction family per bounded TASK; expand only after sizing review.

## Decomposition and Registers

- Decomposition: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:c78301c67df5729d65c57963e6a915339049e3ffaa12d13961ee201445a9b984`
- Deliverables: `docs/_Registers/Deliverables.csv` row DEL-04-07
- Context budget: `docs/_Registers/ContextBudgetQA.csv` row DEL-04-07
- Scope: `docs/_Registers/ScopeLedger.csv` rows SOW-005,SOW-011,SOW-012,SOW-052,SOW-053

## Boundaries

No transfer of kernel, classifier, load-generation, section/mass mathematics or diagnostic-production ownership. CAP-PHYS-007 is integration-owned against DEL-03-08 authority; CAP-PHYS-022 remains DEL-04-01. Corrosion, friction, convergence criteria and engineering acceptance retain their owning decisions.
