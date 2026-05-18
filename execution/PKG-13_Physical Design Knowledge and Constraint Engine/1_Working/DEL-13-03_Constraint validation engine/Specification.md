# Specification: DEL-13-03 Constraint validation engine

## Scope

DEL-13-03 covers a backend feature slice for deterministic validation messages over available physical-design constraints and missing required data. The in-scope validation categories are connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. Sources: `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` rows SOW-068 and DEL-13-03.

Excluded from this deliverable are hidden owner standards, protected code requirements, protected standards content, proprietary project data, final engineering acceptance logic, and automatic professional/code-compliance claims. Sources: `_CONTEXT.md` Package Exclusions; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6; `docs/SPEC.md` section 4.3.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| DEL-13-03-R1 | The validation engine shall validate available design knowledge for the SOW-068 categories: connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 |
| DEL-13-03-R2 | Validation outputs shall be deterministic validation messages rather than hidden report prose or agent text. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-068 Notes; DEL-13-03 row |
| DEL-13-03-R3 | Missing required data shall be surfaced as explicit findings/diagnostics and shall not be silently defaulted. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.3 |
| DEL-13-03-R4 | Validation shall preserve provenance visibility for messages where source or availability of design knowledge affects reliance. | SOW-068 provenance-aware message requirement; `docs/SPEC.md` sections 1 and 4 |
| DEL-13-03-R5 | The engine shall not infer hidden owner standards, protected code requirements, protected standards values, proprietary values, or final engineering acceptance logic. | `_CONTEXT.md` Context Envelope and Package Exclusions; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1, OPS-K-AUTH-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 |
| DEL-13-03-R6 | Outputs shall remain compatible with the accepted schema-first architecture basis and diagnostics/result-envelope boundary. Exact constraint diagnostic schema remains TBD. | `_CONTEXT.md` Architecture Basis Injection; `docs/SPEC.md` sections 1, 4.3, and 9 |
| DEL-13-03-R7 | The deliverable shall include validation diagnostics tests covering the grounded message categories and boundary conditions. | `_CONTEXT.md` Anticipated Artifacts; `docs/_Registers/Deliverables.csv` row DEL-13-03 |
| DEL-13-03-R8 | Later implementation shall treat approved ACTIVE dependency rows in `Dependencies.csv` as predecessor evidence and shall not use this setup pass to retire, delete, or reclassify them. | `_DEPENDENCIES.md`; `Dependencies.csv`; project instruction for this run |

## Standards

No external engineering-code text, tables, values, or clause-level requirements are locally available for this deliverable. The governing local standards for this setup pass are project governance and architecture sources:

| Reference | Applicability |
|---|---|
| `docs/CONTRACT.md` | Binding invariants for IP/data boundary, missing-data handling, units, authority boundaries, and agent non-invention. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content boundary. |
| `docs/SPEC.md` | Technical architecture, domain-core boundary, unit/provenance controls, diagnostics/result-envelope posture, and no-compliance-claim boundary. |
| `docs/TYPES.md` | Epistemic labels and canonical vocabulary for protected data, user-supplied code data, diagnostics, and professional approval. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Accepted revision 0.5 decomposition basis for DEL-13-03 scope and exclusions. |
| `_CONTEXT.md` | Deliverable-local identity, architecture-basis injection, and control-surface constraints. |

## Verification

| Requirement | Verification approach |
|---|---|
| DEL-13-03-R1 | Unit or integration tests enumerate the SOW-068 categories and assert that validation reports supported categories from available inputs. Exact fixture schema TBD. |
| DEL-13-03-R2 | Determinism test repeats the same input and compares message identity/order/content according to the accepted diagnostic schema. Exact comparison basis TBD. |
| DEL-13-03-R3 | Missing-data tests assert explicit findings for required missing inputs and no silent engineering defaults. |
| DEL-13-03-R4 | Provenance tests assert that messages retain or reference source/provenance when the input record provides it. Exact field names TBD. |
| DEL-13-03-R5 | Protected-content/professional-boundary tests assert that validation does not emit standards text, code-specific values, owner-default criteria, or professional approval/compliance statuses. |
| DEL-13-03-R6 | Schema compatibility tests assert that outputs fit the accepted schema-first envelope once the constraint diagnostic schema is resolved. |
| DEL-13-03-R7 | Validation diagnostics tests are recorded as part of the deliverable artifact set. |
| DEL-13-03-R8 | Dependency handling check confirms all approved DAG-002 mirror rows remain ACTIVE and unchanged unless a later human-approved CHANGE process replaces that instruction. |

## Documentation

Required records for later execution:

- Constraint validation module path and API boundary: TBD.
- Constraint diagnostic schema or envelope mapping: TBD.
- Validation diagnostics test inventory: required.
- Public-safe validation fixtures: TBD; must be invented or otherwise permitted, with provenance/review status.
- Assumptions and unsupported inputs: must be recorded as `TBD` or explicit findings rather than inferred defaults.
- Dependency mirror handling: preserve approved DAG-002 rows as ACTIVE during this setup workflow.
