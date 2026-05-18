# Datasheet: DEL-16-02 Operation validation and diff preview

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-16-02 | `_CONTEXT.md` |
| Name | Operation validation and diff preview | `_CONTEXT.md` |
| Package | PKG-16 Model Operation and Agent Proposal Framework | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16` |
| Type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Scope item | SOW-069 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Objective support | OBJ-015 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objectives` |
| Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |

## Attributes

| Attribute | Current value |
|---|---|
| Primary function | Run schema validation, constraint validation, and deterministic diff preview before model operations are applied. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`. |
| Anticipated artifacts | Operation validator; diff preview service; validation tests. Source: `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`. |
| Operation input boundary | Structured model operation schema from DEL-16-01 is an approved upstream dependency. Source: `Dependencies.csv` rows `DAG-002-E0827`; `execution/_DAG/DAG-002/DAG-002_EdgeDispositionReview.md#DAG2-RD-012`. |
| Constraint input boundary | Constraint validation engine DEL-13-03 is an approved upstream dependency. Source: `Dependencies.csv` row `DAG-002-E0828`. |
| Diff input boundary | Model-state comparison engine DEL-14-03 and comparison mapping/tolerance/export contracts DEL-14-05 are approved upstream dependencies. Source: `Dependencies.csv` rows `DAG-002-E0829` and `DAG-002-E0830`. |
| Diagnostics input boundary | Solver diagnostics and singularity detection DEL-04-06 is an approved upstream dependency. Source: `Dependencies.csv` row `DAG-002-E0831`. |
| Architecture basis | Rust core/application services, schema-first envelopes, JSON Schema 2020-12, JCS-compatible hash basis where JSON payloads are hashed, and layered test gates are dispatchable context constraints. Source: `_CONTEXT.md#Architecture Basis Injection`; `execution/_Decomposition/SOFTWARE_DECOMP.md#8`. |
| Implementation location | TBD. No approved source names a concrete module path for this backend feature slice. |

## Conditions

| Condition | Data state |
|---|---|
| Mutation boundary | GUI and agent edits are structured model operations and must not mutate accepted engineering state directly. Source: SOW-069 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Scope Ledger`. |
| Invalid operation behavior | Invalid operations are blocked before application. Source: `_CONTEXT.md#Context Envelope`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-16`. |
| Professional boundary | The package excludes hidden model mutations and autonomous engineering acceptance. Source: `_CONTEXT.md#Package Reference`; `docs/CONTRACT.md#Invariant index`. |
| Diagnostics and result envelopes | Diagnostics/result envelopes must preserve source, severity/class, affected object, message/remediation, provenance, and no certification/compliance claims where applicable. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md#8`; `docs/SPEC.md#4.3`. |
| Exact validation ordering | ASSUMPTION: schema validation should precede constraint validation and diff preview because SOW-069 lists schema validation before constraint validation, diff preview, and controlled application. Exact execution ordering remains TBD until implementation sources define it. |
| Exact diff payload shape | TBD. Sources require deterministic previews but do not define preview fields. |
| Exact operation schema fields | TBD in this deliverable; owned by DEL-16-01. |

## Construction

| Construct | Expected role | Status |
|---|---|---|
| Operation validator | Accepts or rejects proposed structured model operations using schema and constraint checks before controlled application. | Drafted as expected artifact; implementation details TBD. |
| Diff preview service | Produces deterministic previews of the model-state effect of a proposed operation before application. | Drafted as expected artifact; payload and API details TBD. |
| Validation tests | Exercise schema-validation, constraint-validation, preview determinism, and blocked-invalid-operation cases. | Drafted as expected artifact; exact test harness TBD beyond approved layered test baseline. |
| Result/diagnostic envelope integration | Reports validation/preview outcomes without professional approval or compliance claims. | Source-supported boundary; exact schema fields TBD. |

## References

- `_CONTEXT.md` - deliverable identity, scope, objective, package reference, architecture-basis injection.
- `_REFERENCES.md` - source inventory for this deliverable.
- `Dependencies.csv` - approved DAG-002 local mirror/evidence surface for active upstream dependencies.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.5 scope, package, deliverable, objective, and architecture-basis context.
- `docs/_Registers/Deliverables.csv` - deliverable identity and anticipated artifacts.
- `docs/_Registers/ScopeLedger.csv` - SOW-069 wording and product-boundary notes.
- `docs/_Registers/ContextBudgetQA.csv` - context-envelope row.
- `docs/CONTRACT.md` - invariants for data, authority, units, diagnostics, and agent behavior.
- `docs/SPEC.md` - schema-first, persistence, diagnostics, result-envelope, and professional-boundary context.
