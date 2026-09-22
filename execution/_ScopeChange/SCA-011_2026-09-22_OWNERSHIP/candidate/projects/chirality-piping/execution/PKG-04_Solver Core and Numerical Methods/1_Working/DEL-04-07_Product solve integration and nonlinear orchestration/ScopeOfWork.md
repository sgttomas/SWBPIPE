---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-07
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:0581fd5fec1f56ef08d958bc0895557d4fd7ca3d66d2579b70fea03d81e074f8
project_scope_refs: [SOW-005,SOW-011,SOW-012,SOW-052,SOW-053]
package_objective_refs: [OBJ-003,OBJ-012]
---

# Scope of Work — DEL-04-07

**Candidate only:** SCA-011 Group 2 postimage; no active deliverable or accepted lifecycle is created by this draft. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:0581fd5fec1f56ef08d958bc0895557d4fd7ca3d66d2579b70fea03d81e074f8`.

## Purpose and Objective Traceability

Compose accepted model, units, section/load/support contracts and kernel outputs into the product solve service, preserving input identity, missing-data diagnostics and output provenance; execute the DEC-044 assembled nonlinear loop under the existing convergence policy.

- **OUT-001** — Verified contract for DEL-04-07-REQ-001.
- **OUT-002** — Verified contract for DEL-04-07-REQ-002.
- **OUT-003** — Verified contract for DEL-04-07-REQ-003.
- **OUT-004** — Verified contract for DEL-04-07-REQ-004.

## Deliverable Definition — Ontology

### CLM-001 — DEL-04-07-REQ-001

Compose the accepted model, unit, boundary, section, load and kernel contracts into linear/desktop/headless solve entrypoints while preserving source identity and explicit missing-data diagnostics.

### CLM-002 — DEL-04-07-REQ-002

Execute the DEC-044 assembled nonlinear loop while DEL-04-01 retains frame assembly/linear solving and DEL-04-04 retains per-iteration classification; preserve DEC-046 convergence controls and nonconvergence reporting.

### CLM-003 — DEL-04-07-REQ-003

Own the product adapter implementation of section/mass conversion against DEL-03-08 contracts, including an explicit corrosion-allowance difference assessment and rejection or disclosure of unsupported inputs.

### CLM-004 — DEL-04-07-REQ-004

Preserve typed diagnostic, unit and provenance information across product composition and result handoff.

## Completion and Reliance Basis — Epistemology

Each requirement needs named candidate-bound evidence. Existing code, route visibility or historical tests alone do not establish current conformance.

- **AC-001** — CLM-001 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-002** — CLM-002 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-003** — CLM-003 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-004** — CLM-004 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.

## Production and Verification Method — Praxeology

- **VER-001** — Trace the same accepted and invalid model through each entrypoint; assert input identity, equivalent supported outputs and no bypass of primitive validation.
- **VER-002** — Exercise linear/nonlinear switching, converged and nonconverged cases, classifier/assembly handoffs and governed cap/tolerance behavior without changing protected values.
- **VER-003** — Compare product adapter and DEL-03-08 reference-contract cases for geometry, corrosion allowance, density, mass and units; record any unresolved engineering discrepancy rather than declaring equivalence.
- **VER-004** — Trace valid/invalid inputs and diagnostic provenance through each mode to the owned result envelope; no numerical validation follows from route parity.

## Governing Values and Decisions — Axiology

No transfer of kernel, classifier, load-generation, section/mass mathematics or diagnostic-production ownership. CAP-PHYS-007 is integration-owned against DEL-03-08 authority; CAP-PHYS-022 remains DEL-04-01. Corrosion, friction, convergence criteria and engineering acceptance retain their owning decisions.

Apply existing schema, units, provenance, protected-content and professional-boundary contracts. Source integration, internal checking, engineering acceptance and release remain distinct. Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-003 OBJ-012 | CLM-001 | AC-001 | VER-001 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-002 | OBJ-003 OBJ-012 | CLM-002 | AC-002 | VER-002 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-003 | OBJ-003 OBJ-012 | CLM-003 | AC-003 | VER-003 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-004 | OBJ-003 OBJ-012 | CLM-004 | AC-004 | VER-004 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
