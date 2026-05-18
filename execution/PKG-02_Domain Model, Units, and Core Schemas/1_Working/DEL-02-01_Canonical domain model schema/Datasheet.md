# Datasheet: DEL-02-01 Canonical domain model schema

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-02-01 |
| Package ID | PKG-02 |
| Package | Domain Model, Units, and Core Schemas |
| Type | DATA_MODEL_CHANGE |
| Scope item | SOW-041 |
| Deliverable objective | OBJ-001 |
| Primary artifact target | `schemas/model.schema.yaml` |
| Secondary artifact target | `docs/TYPES.md` update |
| Current drafting basis | `_CONTEXT.md` accepted revision 0.4; `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 |
| Source posture | Public governance/register sources only; no protected standards/code data introduced |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Schema baseline | JSON Schema 2020-12 | `docs/_Registers/ScopeLedger.csv` row SOW-041; `docs/_Registers/ContextBudgetQA.csv` row DEL-02-01; `docs/_Decomposition/SOFTWARE_DECOMP.md` Section 8.2 |
| Canonical object families in scope | Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, Report | `_CONTEXT.md` Description; `docs/SPEC.md` Section 3; `docs/DIRECTIVE.md` Section 2.1 |
| Adjacent referenced object families | Section, Support/Restraint, Combination, RulePack references | `docs/SPEC.md` Sections 3, 5, and 6; `docs/PRD.md` Sections 11.6, 12.2, and 13.2 |
| Required cross-cutting fields | Stable identity, units where values are dimensional, provenance where engineering reliance may be affected | `docs/DIRECTIVE.md` Section 2.1; `docs/PRD.md` FR-002 and Section 13.5; `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-UNIT-1 |
| Result and diagnostic data | Result/report records must preserve warnings, assumptions, diagnostics, provenance, and no certification/compliance claims | `docs/SPEC.md` Sections 7 and 8; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/CONTRACT.md` OPS-K-AUTH-1 |
| Persistence compatibility | Schema-governed, versioned, deterministic, unit-aware, provenance-preserving, round-trip testable JSON payloads; JCS-compatible hash basis where JSON payloads are hashed | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and Section 8.2 |
| Public/private data boundary | Public schemas may describe user/private data fields, but must not bundle protected standards text, tables, formulas, allowables, proprietary catalogs, or private user data | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Sections 2 and 3; `docs/PRD.md` Sections 17.1 and 17.3 |

## Conditions

- The deliverable covers the canonical model schema contract only. It does not implement numerical solving, GUI views, rule evaluation, import/export adapters, report rendering, or library population. Source: `_CONTEXT.md` Package Exclusions; `docs/_Decomposition/SOFTWARE_DECOMP.md` PKG-02 row.
- The schema must support a code-neutral model: mechanics data and user-owned rule/code data are separable, and software states must not imply automatic code compliance. Source: `docs/TYPES.md` Sections 4 and 6; `docs/DIRECTIVE.md` Sections 2.2 and 3.
- Missing solve-required or rule-check-required values are explicit findings, not silent defaults. Source: `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` Section 7.
- ASSUMPTION: DEL-02-01 may define lightweight references or common definitions for sections, supports, combinations, and rule-pack references when required by the canonical model, but detailed material/component library semantics remain in PKG-03 and rule-pack internals remain in PKG-06.
- TBD: Exact schema file layout, `$id` URI, code-generation tooling, and fixture organization.
- TBD: Physical project package/container and migration framework remain outside this deliverable unless separately approved. Source: `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-04 and Section 8.2.

## Construction

The expected schema construction is a public JSON Schema 2020-12 contract with reusable definitions for:

- document/root metadata: schema version, application/schema identity, provenance summary, and migration/compatibility status where persisted;
- project/model containers: units, coordinate system, storage policy hooks, rule-pack references, report settings, and model object collections;
- model objects: node coordinates and degrees of freedom, element endpoints/material/section references, components, supports, loads, load cases, and combinations;
- engineering data records: unit-bearing quantities, provenance records, redistribution status, source quality, and review status;
- result/report records: result envelopes, warnings/diagnostics, input manifests, hashes/checksums, and professional-boundary notices;
- validation fixtures: invented or public/permissive examples only, with protected-content and provenance checks.

The construction above is source-grounded as an initial schema plan, not an implemented schema file. The primary artifact path `schemas/model.schema.yaml` is outside the write scope of this Pass 1+2 document run.

## References

- `_CONTEXT.md` revision 0.4 for deliverable identity, accepted decomposition reference, SCA-001 basis IDs, and write-scope constraints.
- `_REFERENCES.md` for the locally declared reference set.
- `_DEPENDENCIES.md` for human-owned dependency posture; no concrete upstream/downstream dependency list was declared.
- `docs/_Registers/Deliverables.csv` row DEL-02-01 for deliverable identity and artifact targets.
- `docs/_Registers/ScopeLedger.csv` row SOW-041 for machine-readable schema scope and JSON Schema 2020-12 baseline.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-02-01 for context-envelope and SCA-001 notes.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4, especially PKG-02, DEL-02-01, SOW-041, and AB-00-01/02/03/04/06/07/08.
- `docs/CONTRACT.md` for IP, data, authority, unit, report, and agent invariants.
- `docs/TYPES.md` for stable IDs, deliverable types, analysis-status vocabulary, epistemic labels, and provenance labels.
- `docs/DIRECTIVE.md` for object ontology, no-silent-defaults, code-neutrality, unit safety, provenance, and professional-boundary principles.
- `docs/SPEC.md` Sections 1 through 9 for layers, domain objects, loads/stress, rule-pack boundary, diagnostics, reports, and validation.
- `docs/PRD.md` Sections 10, 11.6 through 11.8, 12.1 through 12.2, 13.1 through 13.5, 15, 17, 18, 19.1, 22.1, and Appendix A for product requirements and schema-relevant examples.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data and provenance policy.
