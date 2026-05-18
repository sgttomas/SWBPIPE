# Procedure: DEL-16-01 Structured model operation schema

## Purpose

Provide a conservative setup procedure for producing and later using the structured model operation schema. This procedure is bounded by available source context and does not implement product code.

## Prerequisites

| Prerequisite | Source / Status |
|---|---|
| Deliverable context and decomposition basis | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.5 |
| Scope and objective mapping | `docs/_Registers/ScopeLedger.csv` row `SOW-069`; `docs/_Registers/Deliverables.csv` row `DEL-16-01` |
| Architecture basis constraints | `_CONTEXT.md` "Architecture Basis Injection"; `execution/_Decomposition/SOFTWARE_DECOMP.md` section 8 |
| Approved dependency mirror | `Dependencies.csv`; preserve DAG-002 rows as ACTIVE unless a later approved workflow says otherwise |
| Exact schema field layout | TBD |
| Operation granularity and command/patch grammar | TBD |

## Steps

1. Confirm the operation schema remains scoped to DEL-16-01 and SOW-069.
2. Use JSON Schema 2020-12 as the public schema/interchange baseline.
3. Define only structure that is supported by source context: operation identity, operation category, target references, proposal/source metadata, validation/diff/application handoff hooks, diagnostics/provenance hooks, and TBD placeholders where exact fields are not yet authorized.
4. Cover the declared categories: add, move, modify, delete, reconnect, constraint, load, support, and design-knowledge operations.
5. Keep operation data separate from direct persisted mutation. Controlled application belongs downstream of schema validation, constraint validation, and diff preview.
6. Do not embed protected standards text, protected numeric tables, proprietary catalog values, private project data, or invented engineering defaults in public fixtures.
7. Add fixtures only when they can be kept invented and schema-focused. Mark any missing source decision as TBD.
8. Cross-check the future schema against canonical model/domain references so the operation schema does not redefine the whole model.
9. Record unresolved issues for operation granularity, hash binding, exact target-addressing semantics, and agent-autonomy behavior.

## Verification

| Check | Expected Result |
|---|---|
| Schema parser check | Future `schemas/model_operation.schema.json` validates as JSON Schema 2020-12. |
| Category fixture check | Fixtures cover each declared operation category without real engineering values. |
| Boundary check | Operation records do not claim certification, sealing, approval, authentication, or code compliance. |
| Mutation-route check | Fixtures demonstrate proposal/controlled-application route rather than direct durable mutation. |
| Source-fidelity check | Requirements trace to `_CONTEXT.md`, decomposition, registers, governance docs, or the approved local dependency mirror. Unsupported details remain TBD or ASSUMPTION. |
| Dependency mirror check | Existing DAG-002 mirror rows remain ACTIVE and are not reclassified by this setup pass. |

## Records

Records expected from this setup pass:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `_STATUS.md` lifecycle history
- dependency schema validation result

Records expected from a future implementation pass:

- `schemas/model_operation.schema.json`
- operation fixtures
- schema validation log
- protected-content/provenance review evidence
- unresolved TBD register or issue entries, if authorized by the project workflow
