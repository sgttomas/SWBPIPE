# Datasheet: DEL-13-03 Constraint validation engine

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-13-03-DECL-002`.

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-13-03 | `_CONTEXT.md` |
| Name | Constraint validation engine | `_CONTEXT.md` |
| Package ID | PKG-13 | `_CONTEXT.md` |
| Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md` |
| Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-03 |
| Scope item | SOW-068 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW ledger |
| Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective map |
| Context envelope | M | `_CONTEXT.md` |
| Anticipated artifacts | constraint validation module; validation diagnostics tests | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-03 |

## Attributes

| Attribute | Current grounded value |
|---|---|
| Primary function | Implement deterministic validation messages for available design constraints and missing data. Source: `_CONTEXT.md` Description. |
| Validation scope categories | Connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. Source: SOW-068 in `_CONTEXT.md` and `execution/_Decomposition/SOFTWARE_DECOMP.md`. |
| Message boundary | Constraint failures are validation messages, not hidden report prose or agent text. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 Notes. |
| Authority boundary | The deliverable must not infer hidden owner standards, protected code requirements, or final engineering acceptance logic. Sources: `_CONTEXT.md` Context Envelope; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6. |
| Architecture basis | Rust core/application services, JSON Schema 2020-12 contracts, schema-first envelopes, canonical JSON/JCS-compatible hash basis where JSON payloads are hashed, and layered test gates apply as dispatch context. Source: `_CONTEXT.md` Architecture Basis Injection. |
| Current implementation location | `core/constraints/validation/engine.py`, exported through `core/constraints/validation/__init__.py`. Source: implementation evidence. |
| Current validation API | `validate_constraint_envelope(constraint_envelope, design_knowledge_envelope=None)` returns `ValidationResult`; `diagnostic_dicts(...)` serializes diagnostics for stable comparison. Source: `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py`. |
| Current diagnostic record shape | Implemented diagnostics carry `code`, `severity`, `class`, `affected_references`, `source_references`, `message`, and `remediation`; `ValidationResult.to_dict()` adds `has_blocking_findings`. Formal application result-envelope integration remains `TBD`. Source: `core/constraints/validation/engine.py`. |
| Current severity values | The implemented validator emits `blocking`, `warning`, and `info`; release readiness and human-acceptance policy for those severities remains `TBD`. Source: `core/constraints/validation/engine.py`. |

## Conditions

| Condition | Grounded constraint |
|---|---|
| Input knowledge boundary | Validation is over available design knowledge and constraints; missing data is an explicit finding rather than a silently supplied default. Sources: SOW-068; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3. |
| Public/private data boundary | Public artifacts must not bundle protected standards text, protected tables, proprietary values, owner standards, or private project data. Sources: `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6. |
| Professional boundary | Software outputs are decision support and must not automatically claim certification, approval, sealing, authentication, code compliance, or professional reliance. Sources: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 4.3; `docs/DIRECTIVE.md` Professional boundary. |
| Unit and provenance controls | Domain-core and adapter paths may not bypass unit checks, provenance checks, or public/private data boundaries. Sources: `docs/SPEC.md` sections 1 and 4. |
| Upstream evidence surface | `Dependencies.csv` is a deliverable-local evidence surface under the current DAG-006 coordination basis. It preserves historical DAG-002 dependency IDs for audit while listing ACTIVE rows for architecture basis, design-knowledge schema/provenance, constraint entity/provenance, unit, diagnostics, and persistence predecessors. Source: `_DEPENDENCIES.md`; `Dependencies.csv`; `_COORDINATION.md`. |
| Design-knowledge reference handling | The validator indexes supplied `design_knowledge.records` by `id` and emits unresolved-reference diagnostics when constraint references are absent from the supplied design-knowledge envelope. Source: `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py`. |
| Data-boundary handling | The validator checks data-boundary policy fields and emits `IP_BOUNDARY_WARNING` diagnostics for mismatches, protected-suspected provenance, private project data, or professional-boundary failures. Source: `core/constraints/validation/engine.py`; `tests/test_constraint_validation.py`. |

## Construction

Current product-code evidence exists and is limited to a stdlib-only Python validation slice. The implemented module accepts supplied mapping data so missing fields can become deterministic diagnostics instead of exceptions or hidden defaults.

| Expected construction element | Status |
|---|---|
| Constraint validation module | Implemented at `core/constraints/validation/engine.py`; package exports are listed in `core/constraints/validation/__init__.py`. |
| Validation diagnostics tests | Implemented in `tests/test_constraint_validation.py` using stdlib assertions and invented public test fixtures. |
| Represented diagnostic classes | Tests cover `CONNECTIVITY_CONFLICT`, `CLEARANCE_CONFLICT`, `ROUTE_CONFLICT`, `SUPPORT_ZONE_CONFLICT`, `SLOPE_DRAIN_VENT_CONFLICT`, `CONSTRAINT_MISSING_DATA`, and `SCHEMA_VALIDATION`; implementation also emits `PROVENANCE_WARNING`, `UNIT_WARNING`, and `IP_BOUNDARY_WARNING`. |
| Unit checks | Quantity parameters are checked for `value`, `unit`, `dimension`, and `provenance`; noncanonical dimensions produce `CV-UNIT-DIMENSION-UNKNOWN`. The validator does not convert units or invent tolerances. |
| Dependency integration | Current documentation refresh does not edit dependency files. Local dependency rows remain an evidence surface under DAG-006 coordination, with preserved historical row IDs where present. |
| Data examples | Executable invented fixtures exist in `tests/test_constraint_validation.py`. Publication-grade examples, owner/project examples, localization, full geometric conflict solving, runtime integration, release readiness, and human acceptance remain `TBD`. |

## References

- `_CONTEXT.md` - deliverable identity, scope, artifacts, architecture-basis injection.
- `_REFERENCES.md` - governing reference list for this DEL folder.
- `_DEPENDENCIES.md` and `Dependencies.csv` - deliverable-local dependency evidence surface under the current DAG-006 coordination basis.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.7 package, scope, objective, and deliverable entries.
- `docs/_Registers/Deliverables.csv` - row DEL-13-03.
- `docs/_Registers/ScopeLedger.csv` - row SOW-068.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-13-03.
- `docs/CONTRACT.md` - invariant catalog.
- `docs/SPEC.md` - technical and agentic implementation specification.
- `docs/TYPES.md` - vocabulary and epistemic labels.
- `docs/IP_AND_DATA_BOUNDARY.md` - protected-data and public/private data boundary policy.
- `core/constraints/validation/engine.py` - implemented validation engine.
- `core/constraints/validation/__init__.py` - implemented package export surface.
- `tests/test_constraint_validation.py` - focused validation diagnostics tests and invented fixtures.
## D-41 R5 T2C PDU-023 Evidence State

Paired result-trace scalar paths are schema-valid and unpaired paths are rejected. Runtime envelope production remains `TBD` because this validator has no accepted producer/home.
