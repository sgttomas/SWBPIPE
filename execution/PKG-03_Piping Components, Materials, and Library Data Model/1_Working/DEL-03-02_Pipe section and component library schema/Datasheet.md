---
doc_id: DEL-03-02-DATASHEET
doc_kind: deliverable.datasheet
status: draft
created: 2026-04-30
deliverable_id: DEL-03-02
package_id: PKG-03
---

# Datasheet: Pipe Section and Component Library Schema

## Identification

| Field | Value | Evidence |
|---|---|---|
| Deliverable ID | DEL-03-02 | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-03-02 |
| Name | Pipe section and component library schema | SourcePath: `_CONTEXT.md`; SectionRef: Context: DEL-03-02 |
| Package | PKG-03 - Piping Components, Materials, and Library Data Model | SourcePath: `_CONTEXT.md`; SectionRef: Package Reference |
| Type | DATA_MODEL_CHANGE | SourcePath: `_CONTEXT.md`; SectionRef: Type |
| Scope item | SOW-018 | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-018 |
| Objective | OBJ-004 | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: objective row OBJ-004 |
| Implemented schema artifacts | `schemas/component.schema.yaml`; `schemas/section.schema.yaml` | SourcePath: `schemas/component.schema.yaml`; SourcePath: `schemas/section.schema.yaml` |
| Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | SourcePath: `_CONTEXT.md`; SectionRef: Decomposition Reference |
| Responsible party | TBD | Not assigned in accessible sources |

## Attributes

| Attribute | Value | Evidence |
|---|---|---|
| Primary subject | Private pipe section and component library records | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-018 |
| Required public boundary | No protected pipe dimensional tables, protected standards data, or proprietary catalog data are bundled as public defaults | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1, OPS-K-DATA-1 |
| Section record intent | User-entered dimensions, weights/mass-property inputs, provenance, and redistribution status | SourcePath: `_CONTEXT.md`; SectionRef: Description |
| Component record intent | User-entered component dimensions, weights, centers of gravity, source/license metadata, and redistribution status | SourcePath: `_CONTEXT.md`; SectionRef: Description |
| Schema baseline | JSON Schema 2020-12 contracts | SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
| Persistence baseline | Deterministic, versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, and round-trip testable | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 |
| Diagnostics baseline | Diagnostics/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06 |
| Schema implementation evidence | Implemented with strict JSON Schema 2020-12 documents, split strict fixtures, combined legacy fixture pointers, and focused schema tests | SourcePath: `schemas/section.schema.yaml`; SourcePath: `schemas/component.schema.yaml`; SourcePath: `fixtures/component/invented_section_library_valid.json`; SourcePath: `fixtures/component/invented_component_library_valid.json`; SourcePath: `fixtures/component/invented_section_component_library_valid.json`; SourcePath: `tests/test_component_section_schema.py` |

## Conditions

- Public repository artifacts must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1.
- Public data contributions require source, provenance, license/redistribution status, contributor certification, and review disposition. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-2.
- Suspected protected content must be quarantined and escalated; agents must not paraphrase protected tables into public data. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-3.
- Code-specific and proprietary component/section values are user-supplied or lawfully imported private data, not bundled public defaults. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-1.
- Missing solve-required or rule-check-required values are explicit findings, never silent defaults. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-2.
- Component, section, SIF, flexibility, allowable, and rule-pack values carry provenance fields where applicable. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-DATA-3.
- All dimensional, mass, weight, and property fields must be unit-aware and dimensionally checked. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-UNIT-1.
- Private project, material, component, and rule-pack data must not be transmitted or committed publicly by default. SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-PRIV-1.

## Construction

This deliverable now has repository-level schema implementation evidence. This reconciliation records the implemented artifacts without changing schemas, fixtures, tests, lifecycle files, dependency files, or review dispositions.

| Part | Status | Notes |
|---|---|---|
| `schemas/section.schema.yaml` | Implemented evidence | Defines a strict pipe/section library schema with `schema_version`, `section_library`, `section_records`, `dimension_definitions`, `property_definitions`, `completeness_rules`, `diagnostics`, and `open_decisions`; it includes unit-bearing section dimension/property slots, provenance, redistribution status, review status, completeness findings, diagnostics, and open decisions. |
| `schemas/component.schema.yaml` | Implemented evidence | Defines a strict component library schema with `schema_version`, `component_library`, `component_family_contracts`, `component_records`, `field_definitions`, `completeness_rules`, `diagnostics`, and `open_decisions`; it includes component-family contracts, user-entered field slots, protected-value policies, provenance, redistribution status, review status, completeness findings, diagnostics, and open decisions. |
| Strict section fixture | Implemented evidence | `fixtures/component/invented_section_library_valid.json` validates as schema-shape evidence and intentionally omits engineering values through `missing` value statuses, incomplete completeness findings, and diagnostics. |
| Strict component fixture | Implemented evidence | `fixtures/component/invented_component_library_valid.json` validates as schema-shape evidence for bend, branch, rigid, and expansion-joint component records while keeping engineering values absent or private/user-supplied only. |
| Combined legacy fixture | Implemented pointer evidence | `fixtures/component/invented_section_component_library_valid.json` carries section/component content plus references to the strict split fixtures; strict validation relies on the split fixtures. |
| Test coverage | Implemented evidence | `tests/test_component_section_schema.py` checks strict schema shape, provenance requirements, redistribution statuses, unit-dimension alignment with PKG-02, component enum equality with the canonical model enum, split fixture validation, and forbidden public-data text. |
| Provenance metadata | Implemented schema mechanism; policy TBDs remain | Schemas require source/provenance, license, contributor certification, redistribution status, and review status. Accepted public source catalogs, source-license disposition, redistribution acceptance, and public contribution approval remain human/policy decisions. |
| Unit-bearing value model | Implemented schema mechanism | Schemas define quantity-value slots with magnitude, unit, dimension, value status, and provenance; tests assert accepted dimension vocabulary alignment with `schemas/units.schema.yaml`. |
| Protected-content gate hooks | Implemented schema/test guardrails; workflow policy TBDs remain | Schemas and fixtures represent protected-suspected/rejected states, protected-value policies, diagnostics, and no-public-value policies. Human review workflow and accepted source policy remain unresolved. |
| Engineering values | Out of scope | No pipe dimensional table values, component catalog values, allowables, SIFs, flexibility factors, or code-derived values are introduced here. |

## References

- `_CONTEXT.md`, accepted basis for DEL-03-02.
- `_REFERENCES.md`, local reference index for DEL-03-02.
- `_DEPENDENCIES.md`, human-owned dependency declarations for DEL-03-02.
- `docs/_Registers/Deliverables.csv`, row DEL-03-02.
- `docs/_Registers/ScopeLedger.csv`, row SOW-018.
- `docs/_Registers/ContextBudgetQA.csv`, row DEL-03-02.
- `execution/_Decomposition/SOFTWARE_DECOMP.md`, revision 0.7, especially PKG-03, DEL-03-02, OBJ-004, SOW-018, and AB-00-01/02/04/06/07/08.
- `docs/CONTRACT.md`, invariant catalog.
- `docs/TYPES.md`, epistemic labels and data provenance labels.
- `docs/SPEC.md`, sections 1, 3, 10, and 11.
- `docs/DIRECTIVE.md`, data-boundary and stop-rule basis.
