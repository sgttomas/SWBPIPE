# Guidance: DEL-13-02 Constraint entity and provenance model

## Purpose

DEL-13-02 exists to make physical design constraints explicit, traceable, and reviewable before a later validation engine evaluates available design knowledge. The deliverable supports OBJ-014 by contributing constraint records to the schema-backed piping design model, and it supports OBJ-018 by preserving IP and professional-boundary limits across constraint data.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Represent findings, not hidden defaults | Missing data and unmet constraints should surface as explicit findings or diagnostics with provenance. Do not silently supply engineering values. | `INIT.md`; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md#4.3 Analysis status and authority boundary` |
| Keep design knowledge user/project supplied | Constraint categories may refer to owner/project design knowledge, but public artifacts must not bundle owner standards, protected code criteria, proprietary project data, or code-specific values. | SOW-067; `docs/IP_AND_DATA_BOUNDARY.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13` |
| Preserve provenance | Constraint records should identify where known information came from, including user, project, import, agent, or source provenance. Unknown provenance should remain visible as `TBD`, not normalized away. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`; `docs/CONTRACT.md` OPS-K-IP-2, OPS-K-DATA-3 |
| Separate schema from validation engine | This deliverable defines representation and provenance. Deterministic validation behavior belongs to DEL-13-03. | `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13` |
| Preserve professional responsibility | Constraint records and validation messages support review but must not become automatic professional approval or code-compliance certification. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md#4.3 Analysis status and authority boundary` |

## Considerations

- Constraint categories are source-grounded by SOW-068 and implemented as `ConstraintKind`: `connectivity`, `clearance`, `no_go_volume`, `support_zone`, `route_conflict`, `slope`, `drain`, `vent`, `access`, `equipment_interface`, `missing_required_data`, and `TBD`.
- Design-knowledge associations are source-grounded by SOW-067 and represented through typed `Reference` objects plus `design_knowledge_refs`; runtime interpretation of those references remains downstream.
- Unit-bearing quantities referenced by constraints should not bypass the canonical unit contract. The implemented `Quantity` definition requires `value`, `unit`, `dimension`, and `provenance`, and the test fixes the dimension enum to the accepted PKG-02 vocabulary.
- Provenance should distinguish known facts from unresolved assumptions or imported claims. The implemented `Provenance` definition requires source, contributor, redistribution, review, and privacy-classification fields.
- The approved dependency mirror and current DAG-006 coordination basis indicate upstream architecture, canonical model, unit, persistence, design knowledge, and professional-boundary context. They are evidence for predecessor context, not authority to reclassify dependencies.
- Runtime constraint validation, GUI presentation/blocking behavior, physical-to-analytical transform consumption, and actual public example payload policy remain `TBD`.

## Trade-offs

| Topic | Conservative position |
|---|---|
| Category breadth | Include the categories explicitly named by SOW-068; defer additional categories to human-approved scope or later decomposition change. |
| Example payloads | Prefer no examples until invented/public-permissive examples are reviewed. Do not use real owner/project standards or protected code examples. |
| Validation detail | Use the implemented diagnostic and validation-status slots, but avoid encoding DEL-13-03 engine behavior as if already implemented. |
| Professional status | Use diagnostics and review-needed findings; avoid approval/certification/compliance statuses. |

## Examples

No source-grounded example payload is available in the local references. The schema includes `data_boundary.public_examples_policy = invented_or_cleared_data_only`, but example records and any public example publication policy remain `TBD` and must be invented or otherwise cleared for redistribution before inclusion in public artifacts.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| NONE | No direct source conflict detected in the accessible DEL-13-02 context. Remaining `TBD` items are downstream runtime validation, GUI behavior, transform consumption, and public examples. | `_CONTEXT.md`; `_REFERENCES.md`; `Dependencies.csv`; decomposition/register slices; `schemas/constraint.schema.json`; `tests/test_constraint_schema.py` | N/A | N/A | N/A | N/A |
