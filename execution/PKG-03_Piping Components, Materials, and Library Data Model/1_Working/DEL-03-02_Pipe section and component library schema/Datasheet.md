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
| Objective | OBJ-004 | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: objective row OBJ-004 |
| Anticipated artifacts | `schemas/component.schema.yaml`; `schemas/section.schema.yaml` | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts |
| Decomposition basis | `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 | SourcePath: `_CONTEXT.md`; SectionRef: Decomposition Reference |
| Responsible party | TBD | Not assigned in accessible sources |

## Attributes

| Attribute | Value | Evidence |
|---|---|---|
| Primary subject | Private pipe section and component library records | SourcePath: `docs/_Registers/ScopeLedger.csv`; SectionRef: row SOW-018 |
| Required public boundary | No protected pipe dimensional tables, protected standards data, or proprietary catalog data are bundled as public defaults | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-IP-1, OPS-K-DATA-1 |
| Section record intent | User-entered dimensions, weights/mass-property inputs, provenance, and redistribution status | SourcePath: `_CONTEXT.md`; SectionRef: Description |
| Component record intent | User-entered component dimensions, weights, centers of gravity, source/license metadata, and redistribution status | SourcePath: `_CONTEXT.md`; SectionRef: Description |
| Schema baseline | JSON Schema 2020-12 contracts | SourcePath: `_CONTEXT.md`; SectionRef: Architecture Basis Injection |
| Persistence baseline | Deterministic, versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, and round-trip testable | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-04 |
| Diagnostics baseline | Diagnostics/result envelopes carry code, class, severity, source, affected object, message, remediation, and provenance | SourcePath: `docs/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-06 |
| Exact schema fields | TBD | This setup pass does not implement `schemas/component.schema.yaml` or `schemas/section.schema.yaml` |

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

This setup deliverable should prepare later schema implementation without defining protected values.

| Part | Status | Notes |
|---|---|---|
| `schemas/section.schema.yaml` | Anticipated, not implemented in this setup pass | Should model section/library records with unit-bearing fields, provenance, redistribution status, completeness/missing-data status, and schema versioning. Exact fields are TBD. |
| `schemas/component.schema.yaml` | Anticipated, not implemented in this setup pass | Should model component-library records with user-entered dimensions, weights, centers of gravity, source/license metadata, and redistribution status. Exact component-family specialization is handled by later DEL-03 deliverables. |
| Provenance metadata | Required concept | Must include source/provenance and redistribution status sufficient to distinguish private, public-permissive, unknown, and suspected protected content. Exact enum layout is TBD. |
| Unit-bearing value model | Required concept | Must align with project unit safety and schema contracts. Exact reusable unit-value reference is TBD. |
| Protected-content gate hooks | Required concept | Future schema validation and import/review workflows must prevent protected or private values from becoming public defaults. |
| Engineering values | Out of scope | No pipe dimensional table values, component catalog values, allowables, SIFs, flexibility factors, or code-derived values are introduced here. |

## References

- `_CONTEXT.md`, accepted basis for DEL-03-02.
- `_REFERENCES.md`, local reference index for DEL-03-02.
- `_DEPENDENCIES.md`, human-owned dependency declarations for DEL-03-02.
- `docs/_Registers/Deliverables.csv`, row DEL-03-02.
- `docs/_Registers/ScopeLedger.csv`, row SOW-018.
- `docs/_Registers/ContextBudgetQA.csv`, row DEL-03-02.
- `docs/_Decomposition/SOFTWARE_DECOMP.md`, revision 0.4, especially PKG-03, DEL-03-02, OBJ-004, SOW-018, and AB-00-01/02/04/06/07/08.
- `docs/CONTRACT.md`, invariant catalog.
- `docs/TYPES.md`, epistemic labels and data provenance labels.
- `docs/SPEC.md`, sections 1, 3, 10, and 11.
- `docs/DIRECTIVE.md`, data-boundary and stop-rule basis.

