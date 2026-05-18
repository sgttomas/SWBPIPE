# Datasheet: DEL-13-01 Design knowledge schema and provenance model

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-13-01 | `_CONTEXT.md` |
| Deliverable name | Design knowledge schema and provenance model | `_CONTEXT.md` |
| Package ID | PKG-13 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Deliverable type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Scope item | SOW-067 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/ScopeLedger.csv` |
| Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/Deliverables.csv` |
| Anticipated artifacts | `schemas/design_knowledge.schema.json`; design knowledge provenance model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |

## Attributes

| Attribute | Current value | Source |
|---|---|---|
| Primary subject | User-supplied design knowledge for the physical design model | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 |
| Supported knowledge categories named by scope | Endpoints; line data; routing corridors; no-go volumes; supportable zones; equipment interfaces; access constraints; slope, drain, and vent requirements; owner/project metadata | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 |
| Additional named record concerns | Source notes and unresolved assumptions | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-01 row |
| Physical-model relationship | Physical model is the source of truth for editable design data; detailed design-knowledge records are owned by PKG-13 specialized schemas/services | `docs/SPEC.md` section 3; `docs/TYPES.md` Canonical domain object registry |
| Schema baseline | JSON Schema 2020-12 contracts are the accepted architecture basis for schema surfaces | `_CONTEXT.md` Architecture Basis Injection |
| Public-data boundary | No protected owner standards, protected code data, or private project data may be bundled in public examples | `_CONTEXT.md`; `docs/CONTRACT.md` OPS-K-IP-1/OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` |
| Professional boundary | Records and software output must not claim certification, sealing, professional approval, or code compliance for reliance | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Analysis-status vocabulary |
| Unit-bearing data boundary | Unit-bearing physical values crossing schema or service boundaries require explicit unit metadata unless explicitly dimensionless | `docs/SPEC.md` section 4 |
| Missing-data posture | Missing solve-required or rule-check-required values must be explicit findings, never silent defaults | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` section 4.4 |

## Conditions

| Condition | Datasheet treatment | Source |
|---|---|---|
| Protected data exclusion | The deliverable must define schema/provenance slots without embedding owner standards, protected code criteria, proprietary project data, protected dimensional tables, protected standards text, or proprietary vendor data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 exclusions |
| User-supplied knowledge | Design knowledge remains user/project supplied. Public examples, if later created, must use invented or otherwise cleared data. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067; `docs/IP_AND_DATA_BOUNDARY.md` section 2 |
| Provenance need | Public data records require source, location, license or redistribution basis, contributor/certification, redistribution status, and review status. | `docs/IP_AND_DATA_BOUNDARY.md` section 4 |
| Dependency evidence | Approved local `Dependencies.csv` is a DAG-002 mirror/evidence surface with 11 ACTIVE rows; it is not an independent graph authority. | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Architecture-basis context | DEL-13-01 has upstream active architecture-basis rows from DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-06, DEL-00-07, and DEL-00-08. | `Dependencies.csv` |
| Domain/governance context | DEL-13-01 has upstream active rows for canonical domain model schema, unit system contract, copyright/protected-data boundary policy, and professional responsibility/product-claims policy. | `Dependencies.csv` |

## Construction

| Construction item | Status |
|---|---|
| `schemas/design_knowledge.schema.json` | TBD. No implementation artifact exists in this deliverable folder at setup time. |
| Schema identifier, `$id`, `$schema`, and versioning fields | TBD. JSON Schema 2020-12 is the accepted baseline, but exact identifiers are not specified in accessible sources. |
| Design knowledge record taxonomy | Partially sourced. Scope names categories, but exact object names, enum values, and required/optional fields remain TBD. |
| Provenance model fields | Partially sourced. Required public-data provenance fields are named in `docs/IP_AND_DATA_BOUNDARY.md`; exact embedding into design knowledge records remains TBD. |
| Unit-bearing quantity representation | Partially sourced. Unit metadata is required by `docs/SPEC.md`; exact reuse/import of a units schema is a downstream implementation detail and remains TBD. |
| Validation and tests | TBD. Accessible sources require schema validation, unit checks, provenance checks, private-data controls, and protected-content screening, but this setup pass does not implement tests. |
| Product code evidence | None. This setup pass creates planning/drafting artifacts only. |

## References

| Reference | Use in this datasheet |
|---|---|
| `_CONTEXT.md` | Deliverable identity, scope, artifacts, context envelope, architecture-basis injection. |
| `_REFERENCES.md` | Reference index and source boundary for this setup pass. |
| `_DEPENDENCIES.md` and `Dependencies.csv` | Approved DAG-002 mirror/evidence surface and active upstream dependency context. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-067, OBJ-014, PKG-13 boundary, DEL-13-01 row, OI-013. |
| `docs/_Registers/Deliverables.csv` | Deliverable row and artifacts. |
| `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-067. |
| `docs/_Registers/ContextBudgetQA.csv` | Context envelope and no-protected-data note. |
| `docs/CONTRACT.md` | Invariants for IP, data, authority, units, and agent behavior. |
| `docs/TYPES.md` | Identifier rules, epistemic labels, analysis-status vocabulary, and domain object registry. |
| `docs/SPEC.md` | Architecture, physical-model source-of-truth framing, schema/unit/provenance constraints, acceptance semantics. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data boundary and required provenance fields. |
