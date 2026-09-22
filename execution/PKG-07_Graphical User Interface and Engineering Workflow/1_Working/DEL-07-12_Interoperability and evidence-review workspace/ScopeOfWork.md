---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-12
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:f1d6474e35d0fd42800ff8acfbb3148e7ade9b43aef72f9cab2df69a26786577
project_scope_refs: [SOW-079]
package_objective_refs: [OBJ-006,OBJ-007,OBJ-018]
---

# Scope of Work — DEL-07-12

**Application basis:** SCA-011 Group 2 amendment; audited poststate acceptance remains pending Group 3. Basis: `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@sha256:f1d6474e35d0fd42800ff8acfbb3148e7ade9b43aef72f9cab2df69a26786577`.

## Purpose and Objective Traceability

Provide the finite interoperability and evidence-review GUI inventory for inspecting, preparing and explicitly handing off already-scoped record families, preserving their contracts and distinguishing illustrative previews from actual generated records.

- **OUT-001** — Verified contract for DEL-07-12-REQ-001.
- **OUT-002** — Verified contract for DEL-07-12-REQ-002.
- **OUT-003** — Verified contract for DEL-07-12-REQ-003.
- **OUT-004** — Verified contract for DEL-07-12-REQ-004.

## Deliverable Definition — Ontology

### CLM-001 — DEL-07-12-REQ-001

Show the consumed record family, version and identity; validate against the owning contract before presenting a record as actual current product output.

### CLM-002 — DEL-07-12-REQ-002

Preserve diagnostic, unit, provenance, loss and unsupported-behavior information through presentation and explicit export dispatch.

### CLM-003 — DEL-07-12-REQ-003

Visibly distinguish illustrative fixtures, previews, references and historical records from actual current generated records.

### CLM-004 — DEL-07-12-REQ-004

Use only explicit authorized handoff paths under protected-data controls, including native-host evidence where applicable.

## Completion and Reliance Basis — Epistemology

Each requirement needs named candidate-bound evidence. Existing code, route visibility or historical tests alone do not establish current conformance.

- **AC-001** — CLM-001 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-002** — CLM-002 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-003** — CLM-003 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.
- **AC-004** — CLM-004 is supported by the corresponding witness, including invalid/unsupported paths and declared limitations.

## Production and Verification Method — Praxeology

- **VER-001** — For each finite panel family, bind a producer, authoritative contract and supported record fixture; reject or visibly diagnose invalid/version-mismatched records.
- **VER-002** — Round-trip valid/invalid records across UI-to-service boundaries and compare identifiers, units, losses and diagnostic classes with the owning contract.
- **VER-003** — Inspect fixture, preview, historical and current-output states; screenshots alone cannot prove schema identity or provenance.
- **VER-004** — Test denied and permitted explicit handoffs using non-secret fixtures and owning policy; no provider, destination, license or plugin activation is inferred from GUI ownership.

## Governing Values and Decisions — Axiology

Finite inventory: native JSON package; .mbf model export; licensed external-run evidence review; stress-neutral, PCF and review-geometry export; handoff/local-FEA/external-prover boundary panels; adapter-framework, headless-runner and export-adapter SDK review panels; validation-evidence review. Contract/engine/API ownership stays PKG-15, PKG-17, DEL-10-02/03/05 and PKG-09; DEL-17-02 owns the export-unit helper contract. DEL-10-04 retains build-readiness under DEC-076. DEL-07-08 retains offline/agent proposal review. No exporter, schema, licensed-process invocation, new adapter family, private-data permission or professional-acceptance workflow is added.

Apply existing schema, units, provenance, protected-content and professional-boundary contracts. Source integration, internal checking, engineering acceptance and release remain distinct. Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | OBJ-006 OBJ-007 OBJ-018 | CLM-001 | AC-001 | VER-001 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-002 | OBJ-006 OBJ-007 OBJ-018 | CLM-002 | AC-002 | VER-002 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-003 | OBJ-006 OBJ-007 OBJ-018 | CLM-003 | AC-003 | VER-003 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
| OUT-004 | OBJ-006 OBJ-007 OBJ-018 | CLM-004 | AC-004 | VER-004 | Named claim-bound contract witness with valid/invalid cases and candidate identity |
