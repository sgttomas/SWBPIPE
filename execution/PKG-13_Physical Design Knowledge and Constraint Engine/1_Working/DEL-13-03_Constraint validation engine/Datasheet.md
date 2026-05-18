# Datasheet: DEL-13-03 Constraint validation engine

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
| Exact implementation location | TBD. The accessible sources do not name a module path for this validation engine. |
| Exact diagnostic schema | TBD. The accessible sources require deterministic diagnostics/messages and schema-first envelopes but do not define the constraint-validation diagnostic record shape for this deliverable. |

## Conditions

| Condition | Grounded constraint |
|---|---|
| Input knowledge boundary | Validation is over available design knowledge and constraints; missing data is an explicit finding rather than a silently supplied default. Sources: SOW-068; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3. |
| Public/private data boundary | Public artifacts must not bundle protected standards text, protected tables, proprietary values, owner standards, or private project data. Sources: `docs/CONTRACT.md` OPS-K-IP-1 through OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` sections 2-6. |
| Professional boundary | Software outputs are decision support and must not automatically claim certification, approval, sealing, authentication, code compliance, or professional reliance. Sources: `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` section 4.3; `docs/DIRECTIVE.md` Professional boundary. |
| Unit and provenance controls | Domain-core and adapter paths may not bypass unit checks, provenance checks, or public/private data boundaries. Sources: `docs/SPEC.md` sections 1 and 4. |
| Upstream evidence surface | `Dependencies.csv` is an approved DAG-002 mirror/evidence surface with ACTIVE rows for architecture basis, design-knowledge schema/provenance, constraint entity/provenance, unit, diagnostics, and persistence predecessors. Source: `_DEPENDENCIES.md`; `Dependencies.csv`. |

## Construction

This setup pass does not create product code. It records the deliverable-local documentation basis for later sealed Type 2 execution.

| Expected construction element | Status |
|---|---|
| Constraint validation module | Required anticipated artifact; implementation details TBD. |
| Validation diagnostics tests | Required anticipated artifact; exact test framework and fixtures TBD beyond accepted Cargo/Vitest/Playwright/validation gate context in `_CONTEXT.md`. |
| Dependency integration | Must respect approved ACTIVE DAG-002 mirror rows; this pass does not read upstream deliverable folders or reclassify mirror rows. |
| Data examples | TBD. No accessible source provides concrete public-safe example payloads for constraint validation. Any later examples must be invented or otherwise permitted and provenance-reviewed. |

## References

- `_CONTEXT.md` - deliverable identity, scope, artifacts, architecture-basis injection.
- `_REFERENCES.md` - governing reference list for this DEL folder.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-002 local dependency mirror/evidence surface.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.5 package, scope, objective, and deliverable entries.
- `docs/_Registers/Deliverables.csv` - row DEL-13-03.
- `docs/_Registers/ScopeLedger.csv` - row SOW-068.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-13-03.
- `docs/CONTRACT.md` - invariant catalog.
- `docs/SPEC.md` - technical and agentic implementation specification.
- `docs/TYPES.md` - vocabulary and epistemic labels.
- `docs/IP_AND_DATA_BOUNDARY.md` - protected-data and public/private data boundary policy.
