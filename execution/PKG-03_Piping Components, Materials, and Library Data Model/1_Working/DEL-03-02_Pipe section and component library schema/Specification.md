---
doc_id: DEL-03-02-SPECIFICATION
doc_kind: deliverable.specification
status: draft
created: 2026-04-30
deliverable_id: DEL-03-02
package_id: PKG-03
---

# Specification: Pipe Section and Component Library Schema

## Scope

DEL-03-02 defines setup evidence for the future pipe section and component library schema artifacts listed in the decomposition: `schemas/section.schema.yaml` and `schemas/component.schema.yaml`. The schema subject is private library records for user-entered dimensions, weights, centers of gravity, and source/license metadata with provenance and redistribution status.

This setup pass does not implement repository-level schema files, does not add public pipe dimensional tables, does not add component catalog values, and does not resolve exact field names or reusable schema references. Those implementation details remain `TBD` until a later sealed implementation brief.

## Requirements

| ID | Requirement | Evidence |
|---|---|---|
| DEL-03-02-REQ-01 | The future section schema shall support private pipe section library records with provenance and redistribution status. | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-018 |
| DEL-03-02-REQ-02 | The future component schema shall support private component library records with user-entered dimensions, weights, centers of gravity, source/license metadata, provenance, and redistribution status. | SourcePath: `_CONTEXT.md`; SectionRef: Description |
| DEL-03-02-REQ-03 | Public schema artifacts shall not bundle protected dimensional tables, protected standards data, proprietary catalog data, material allowables, SIF/flexibility tables, or code-derived values. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1, OPS-K-DATA-1 |
| DEL-03-02-REQ-04 | Schema validation shall preserve missing required values as explicit findings or validation failures rather than silent defaults. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-2 |
| DEL-03-02-REQ-05 | Library records shall carry provenance metadata for values that may affect engineering reliance, including source and redistribution/license status. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2, OPS-K-DATA-3 |
| DEL-03-02-REQ-06 | Unit-bearing dimensional, weight, mass-property, center-of-gravity, and property fields shall be unit-aware and dimensionally checked. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1 |
| DEL-03-02-REQ-07 | Suspected protected content shall be representable as a quarantined/escalated status, not accepted as public library content. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-3 |
| DEL-03-02-REQ-08 | The future schema design shall align with JSON Schema 2020-12 and schema-first command/query/job result envelope architecture where service-facing. | SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
| DEL-03-02-REQ-09 | Persistence-facing records shall be deterministic, versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, and round-trip testable. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 |
| DEL-03-02-REQ-10 | Library import/export or adapter-facing use shall preserve internal/public API boundaries and shall not bypass validation, provenance, redistribution, diagnostics, or public/private data boundaries. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07 |
| DEL-03-02-REQ-11 | Diagnostics for invalid, incomplete, private, unknown-source, or suspected protected library records shall include code, class, severity, source, affected object, message, remediation, and provenance where applicable. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06 |
| DEL-03-02-REQ-12 | Future tests shall include schema validation, unit checks, provenance/redistribution gates, protected-content gates, persistence round-trip tests, and regression coverage where relevant. | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-08 |
| DEL-03-02-REQ-13 | Exact schema filenames are anticipated as `schemas/component.schema.yaml` and `schemas/section.schema.yaml`, but this setup pass shall not edit repository-level artifacts. | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts |

## Standards

No protected external engineering standards, code clauses, tables, examples, or proprietary data are incorporated into this setup deliverable.

Applicable internal standards and baselines:

- `docs/CONTRACT.md`: OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, OPS-K-AGENT-1 through OPS-K-AGENT-4.
- `docs/TYPES.md`: epistemic labels and data provenance labels.
- `docs/SPEC.md`: domain objects, schema-governed development workflow, and acceptance semantics.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4: SOW-018, OBJ-004, and architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08.
- JSON Schema 2020-12 is the accepted schema-contract basis; this document does not reproduce external specification text.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-01, REQ-02, REQ-13 | Review future schema files for section/component record coverage and traceability to DEL-03-02. |
| REQ-03, REQ-07 | Protected-content review confirms no protected tables, code-derived values, proprietary catalog defaults, or private data are present in public artifacts. |
| REQ-04 | Schema and validation tests confirm missing required values become validation findings, not defaults. |
| REQ-05 | Provenance/redistribution tests confirm required source and status fields exist and are not silently omitted. |
| REQ-06 | Unit-schema tests confirm dimensional and mass-property fields are unit-bearing and dimensionally checked. |
| REQ-08, REQ-09, REQ-10 | Architecture review confirms schema-first, deterministic, persistence-aware, and boundary-preserving behavior. |
| REQ-11 | Diagnostic tests confirm incomplete, unknown-source, private, and suspected-protected records produce governed diagnostics. |
| REQ-12 | Test plan review confirms schema, unit, provenance, protected-content, round-trip, and regression gates are represented. |

## Documentation

Required future artifacts:

- `schemas/section.schema.yaml` with schema versioning, unit-bearing section fields, provenance/redistribution metadata, completeness status, and validation hooks. Exact field names are `TBD`.
- `schemas/component.schema.yaml` with schema versioning, component-library record identity, user-entered dimensions/weights/COG fields, provenance/redistribution metadata, completeness status, and validation hooks. Exact field names are `TBD`.
- Schema test fixtures using invented/public-safe values only.
- Provenance and protected-content review notes for any public example library data.
- Migration/round-trip notes once persistence implementation exists.

