# Context: DEL-07-12

**Application basis:** SCA-011 Group 2 amendment; audited poststate acceptance remains pending Group 3. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:f1d6474e35d0fd42800ff8acfbb3148e7ade9b43aef72f9cab2df69a26786577`.

**Deliverable ID:** DEL-07-12
**Name:** Interoperability and evidence-review workspace
**Package ID:** PKG-07
**Package Name:** Graphical User Interface and Engineering Workflow
**Discipline:** Software
**Type:** UX_UI_SLICE
**Responsible Party:** Project owner; bounded executing role assigned by the work graph

## Description

Provide the finite interoperability and evidence-review GUI inventory for inspecting, preparing and explicitly handing off already-scoped record families, preserving their contracts and distinguishing illustrative previews from actual generated records.

## Anticipated Artifacts

- record-family review panels
- explicit handoff interactions
- provenance and preview-state evidence

## Scope Coverage

- SOW-079

## Objective Support

- OBJ-006
- OBJ-007
- OBJ-018

## Context Envelope

- **Envelope:** L
- **Notes:** Consumption, rendering and explicit dispatch only; no exporters, schema semantics, licensed process invocation or new adapter families.
- One finite interface or interaction family per bounded TASK; expand only after sizing review.

## Decomposition and Registers

- Decomposition: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:f1d6474e35d0fd42800ff8acfbb3148e7ade9b43aef72f9cab2df69a26786577`
- Deliverables: `docs/_Registers/Deliverables.csv` row DEL-07-12
- Context budget: `docs/_Registers/ContextBudgetQA.csv` row DEL-07-12
- Scope: `docs/_Registers/ScopeLedger.csv` rows SOW-079

## Boundaries

Finite inventory: native JSON package; .mbf model export; licensed external-run evidence review; stress-neutral, PCF and review-geometry export; handoff/local-FEA/external-prover boundary panels; adapter-framework, headless-runner and export-adapter SDK review panels; validation-evidence review. Contract/engine/API ownership stays PKG-15, PKG-17, DEL-10-02/03/05 and PKG-09; DEL-17-02 owns the export-unit helper contract. DEL-10-04 retains build-readiness under DEC-076. DEL-07-08 retains offline/agent proposal review. No exporter, schema, licensed-process invocation, new adapter family, private-data permission or professional-acceptance workflow is added.
