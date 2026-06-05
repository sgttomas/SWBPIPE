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

DEL-03-02 defines the pipe section and component library schema artifacts listed in the decomposition: `schemas/section.schema.yaml` and `schemas/component.schema.yaml`. The schema subject is private library records for user-entered dimensions, weights, centers of gravity, stiffness/modifier slots, source/license metadata, provenance, redistribution status, review status, completeness findings, diagnostics, and open decisions.

The repository-level schema files, strict split fixtures, legacy combined fixture pointer, and focused schema tests are implemented evidence for this deliverable. This reconciliation does not edit those repository-level artifacts, does not add public pipe dimensional tables, does not add component catalog values, and does not resolve human-owned policy decisions for public source catalogs, fixture values, redistribution acceptance, dependency satisfaction, lifecycle, or review disposition.

## Requirements

| ID | Requirement | Evidence |
|---|---|---|
| DEL-03-02-REQ-01 | The section schema supports private pipe section library records with provenance and redistribution status. | SourcePath: `schemas/section.schema.yaml`; SectionRef: top-level `section_library`, `section_records`, `$defs.Provenance`, `$defs.RedistributionStatus` |
| DEL-03-02-REQ-02 | The component schema supports private component library records with user-entered dimensions, weights, centers of gravity, stiffness/modifier slots, source/license metadata, provenance, and redistribution status. | SourcePath: `schemas/component.schema.yaml`; SectionRef: top-level `component_library`, `component_family_contracts`, `component_records`, `field_definitions`, `$defs.ComponentFieldKind`, `$defs.Provenance` |
| DEL-03-02-REQ-03 | Public schema and fixture artifacts do not bundle protected dimensional tables, protected standards data, proprietary catalog data, material allowables, SIF/flexibility tables, or code-derived values. | SourcePath: `tests/test_component_section_schema.py`; SectionRef: `FORBIDDEN_PUBLIC_DATA_TEXT`; SourcePath: `fixtures/component/invented_section_library_valid.json`; SourcePath: `fixtures/component/invented_component_library_valid.json` |
| DEL-03-02-REQ-04 | Schema and fixture evidence preserve missing required values as explicit value statuses, completeness findings, diagnostics, or validation failures rather than silent defaults. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.CompletenessFinding`, `$defs.SectionDiagnostic`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.CompletenessFinding`, `$defs.ComponentDiagnostic`; SourcePath: `tests/test_component_section_schema.py` |
| DEL-03-02-REQ-05 | Library records carry provenance metadata for values that may affect engineering reliance, including source and redistribution/license status. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.Provenance`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.Provenance` |
| DEL-03-02-REQ-06 | Unit-bearing dimensional, weight, mass-property, center-of-gravity, stiffness, and property fields are unit-aware and dimensionally checked against accepted unit dimensions. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.QuantityValue`, `$defs.SectionDimension`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.QuantityValue`, `$defs.ComponentQuantityDimension`; SourcePath: `tests/test_component_section_schema.py` |
| DEL-03-02-REQ-07 | Suspected protected content is representable as protected-suspected/quarantined/rejected state or protected-content diagnostics, not accepted as public library content. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.RedistributionStatus`, `$defs.ReviewStatus`, `$defs.SectionDiagnosticCode`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.RedistributionStatus`, `$defs.ReviewStatus`, `$defs.ComponentDiagnosticCode` |
| DEL-03-02-REQ-08 | The schema artifacts align with JSON Schema 2020-12 and remain schema-first; service-facing command/query/job result-envelope integration is governed by adjacent architecture deliverables. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$schema`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$schema`; SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
| DEL-03-02-REQ-09 | Persistence-facing records are versioned, unit-aware, provenance-preserving, schema-governed, and testable as strict schema instances. Broader migration and round-trip persistence behavior remains governed outside this reconciliation scope. | SourcePath: `schemas/section.schema.yaml`; SourcePath: `schemas/component.schema.yaml`; SourcePath: `tests/test_component_section_schema.py`; SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 |
| DEL-03-02-REQ-10 | Library import/export or adapter-facing use shall preserve internal/public API boundaries and shall not bypass validation, provenance, redistribution, diagnostics, or public/private data boundaries. Concrete import formats remain `TBD`. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07; SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.OpenDecision`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.OpenDecision` |
| DEL-03-02-REQ-11 | Diagnostics for invalid, incomplete, private, unknown-source, or suspected protected library records include code, class, severity, source, affected object, message, remediation, and provenance where applicable. | SourcePath: `schemas/section.schema.yaml`; SectionRef: `$defs.SectionDiagnostic`; SourcePath: `schemas/component.schema.yaml`; SectionRef: `$defs.ComponentDiagnostic`; SourcePath: `fixtures/component/invented_section_library_valid.json`; SourcePath: `fixtures/component/invented_component_library_valid.json` |
| DEL-03-02-REQ-12 | Current tests include schema validation, unit-dimension checks, provenance/redistribution gates, protected-content text checks, split fixture validation, and regression coverage for PKG-02 enum/dimension compatibility. Persistence round-trip coverage remains governed outside this reconciliation scope. | SourcePath: `tests/test_component_section_schema.py` |
| DEL-03-02-REQ-13 | Exact schema filenames are `schemas/component.schema.yaml` and `schemas/section.schema.yaml`; this reconciliation run shall not edit repository-level schema artifacts. | SourcePath: `_CONTEXT.md`; SectionRef: artifact list; SourcePath: `schemas/component.schema.yaml`; SourcePath: `schemas/section.schema.yaml` |

## Standards

No protected external engineering standards, code clauses, tables, examples, or proprietary data are incorporated into this deliverable.

Applicable internal standards and baselines:

- `docs/CONTRACT.md`: OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-PRIV-1, OPS-K-GOV-4, OPS-K-AGENT-1 through OPS-K-AGENT-4.
- `docs/TYPES.md`: epistemic labels and data provenance labels.
- `docs/SPEC.md`: domain objects, schema-governed development workflow, and acceptance semantics.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7: SOW-018, OBJ-004, and architecture basis AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08.
- JSON Schema 2020-12 is the accepted schema-contract basis; this document does not reproduce external specification text.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-01, REQ-02, REQ-13 | Review implemented schema files for section/component record coverage and traceability to DEL-03-02. |
| REQ-03, REQ-07 | Protected-content review confirms no protected tables, code-derived values, proprietary catalog defaults, or private data are present in public artifacts. |
| REQ-04 | Schema and validation tests confirm missing required values become validation findings, not defaults. |
| REQ-05 | Provenance/redistribution tests confirm required source and status fields exist and are not silently omitted. |
| REQ-06 | Unit-schema tests confirm dimensional and mass-property fields are unit-bearing and dimensionally checked. |
| REQ-08, REQ-09, REQ-10 | Architecture review confirms schema-first, deterministic, persistence-aware, and boundary-preserving behavior. |
| REQ-11 | Diagnostic tests confirm incomplete, unknown-source, private, and suspected-protected records produce governed diagnostics. |
| REQ-12 | Test review confirms schema, unit, provenance, protected-content, split-fixture, and compatibility regression gates are represented; persistence round-trip coverage remains outside this reconciliation scope. |

## Documentation

Implemented artifacts and open items:

- `schemas/section.schema.yaml` implements schema versioning, unit-bearing section dimension/property fields, provenance/redistribution metadata, review status, completeness status, diagnostics, and open decisions.
- `schemas/component.schema.yaml` implements schema versioning, component-library record identity, component-family contracts, user-entered dimensions/weights/COG/stiffness/modifier slots, provenance/redistribution metadata, review status, completeness status, diagnostics, and open decisions.
- `fixtures/component/invented_section_library_valid.json` and `fixtures/component/invented_component_library_valid.json` are strict invented/public-safe fixtures with engineering values intentionally omitted.
- `fixtures/component/invented_section_component_library_valid.json` remains legacy combined evidence and points to the strict split fixtures.
- `tests/test_component_section_schema.py` is the focused verification surface for schema/fixture compatibility.
- Public source catalogs, public fixture value policy, source/license disposition, redistribution acceptance, dependency satisfaction, human review disposition, lifecycle state, and persistence round-trip notes remain `TBD` or governed outside this reconciliation scope.
