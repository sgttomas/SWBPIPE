# Datasheet: DEL-16-01 Structured model operation schema

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-16-01 |
| Name | Structured model operation schema |
| Package ID | PKG-16 |
| Package Name | Model Operation and Agent Proposal Framework |
| Type | DATA_MODEL_CHANGE |
| Scope Coverage | SOW-069 |
| Objective Support | OBJ-015 |
| Anticipated Artifacts | `schemas/model_operation.schema.json`; operation fixtures |
| Lifecycle note | Draft setup artifact; not implementation evidence |

Sources: `_CONTEXT.md` sections "Description", "Anticipated Artifacts", "Scope Coverage", "Objective Support"; `docs/_Registers/Deliverables.csv` row `DEL-16-01`.

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Operation route | GUI and agent edits are represented as structured model operations. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069; `docs/_Registers/ScopeLedger.csv` row `SOW-069` |
| Mutation boundary | Operations are the only model mutation route for GUI and agent proposals. | `_CONTEXT.md` Context Envelope; `docs/_Registers/ContextBudgetQA.csv` row `DEL-16-01` |
| Operation categories in scope | add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge operations. | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-16 table |
| Schema/interchange baseline | JSON Schema 2020-12 for public schemas/interchange. | `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8.2; `docs/TYPES.md` section 8 |
| Downstream validation surfaces | schema validation, constraint validation, diff preview, and controlled application through the model engine. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 |
| Professional boundary | Agent outputs remain proposals until human acceptance; software/agents must not claim approval, certification, sealing, or code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1 and OPS-K-AGENT-4; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-069 note |

## Conditions

| Condition | Status |
|---|---|
| Exact schema field layout | TBD - not specified by available sources. |
| Exact operation granularity for physical model edits | TBD - decomposition open issue `OI-012` says operation granularity requires technical architecture detail before implementation briefs are sealed. |
| Exact dependency versions or code-generation tooling | TBD - SCA-001/SCA-002 basis defers exact dependency versions and code-generation tooling. |
| Direct product code implementation | None in this setup pass; no implementation artifact is claimed. |
| Protected standards or proprietary data | Must not be embedded in public fixtures or schema defaults. |

Sources: `execution/_Decomposition/SOFTWARE_DECOMP.md` open issues `OI-002`, `OI-012`; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AGENT-1.

## Construction

The deliverable is expected to materialize as a schema-first data model plus fixtures:

| Artifact | Construction Status | Notes |
|---|---|---|
| `schemas/model_operation.schema.json` | TBD | Should define the public structure for model operation records, but exact field names and schema placement are not present in the available sources. |
| Operation fixtures | TBD | Should exercise the operation categories and validation route without protected standards content, proprietary catalog values, private project data, or software-generated professional approval claims. |
| Validation hooks | TBD | Sources require schema validation, constraint validation, diff preview, and controlled application, but DEL-16-02 owns validation/diff service behavior. |

## References

- `_CONTEXT.md` - deliverable identity, scope, objective, architecture-basis injection.
- `_REFERENCES.md` - source list for this setup pass.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - accepted revision 0.5 decomposition basis, SOW-069, OBJ-015, PKG-16, DEL-16-01, architecture basis.
- `docs/_Registers/Deliverables.csv` - deliverable register row `DEL-16-01`.
- `docs/_Registers/ScopeLedger.csv` - scope ledger row `SOW-069`.
- `docs/_Registers/ContextBudgetQA.csv` - context budget row `DEL-16-01`.
- `docs/CONTRACT.md` - project invariants, including data boundary, professional boundary, and agent non-invention constraints.
- `docs/TYPES.md` - canonical object registry and schema boundary notes.
- `docs/SPEC.md` - technical specification slices for domain objects, persistence, viewport command intents, and professional boundaries.
- `Dependencies.csv` - approved local DAG-002 mirror/evidence surface, read only for this setup pass.
