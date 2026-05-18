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

- Constraint categories are source-grounded by SOW-068. The exact schema taxonomy, enum names, severity levels, and normalization rules are `TBD`.
- Design-knowledge associations are source-grounded by SOW-067. Exact reference shapes are `TBD` and should remain compatible with the physical source-of-truth model and DEL-13-01 design knowledge model.
- Unit-bearing quantities referenced by constraints should not bypass the canonical unit contract. If a constraint needs a length, slope, elevation, clearance, or similar physical quantity, the quantity representation is `TBD` until it is tied to the unit-aware schema basis.
- Provenance should distinguish known facts from unresolved assumptions or imported claims. The exact provenance submodel is `TBD`.
- The approved dependency mirror indicates upstream architecture, canonical model, unit, persistence, design knowledge, and professional-boundary context. It is evidence for predecessor context, not authority to reclassify dependencies.

## Trade-offs

| Topic | Conservative position |
|---|---|
| Category breadth | Include the categories explicitly named by SOW-068; defer additional categories to human-approved scope or later decomposition change. |
| Example payloads | Prefer no examples until invented/public-permissive examples are reviewed. Do not use real owner/project standards or protected code examples. |
| Validation detail | Store enough structure for later validation, but avoid encoding DEL-13-03 engine behavior as if already implemented. |
| Professional status | Use diagnostics and review-needed findings; avoid approval/certification/compliance statuses. |

## Examples

No source-grounded example payload is available in the local references. Example records are `TBD` and must be invented or otherwise cleared for redistribution before inclusion in public artifacts.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| NONE | No direct source conflict detected in the accessible DEL-13-02 context. Missing implementation specifics remain `TBD`. | `_CONTEXT.md`; `_REFERENCES.md`; `Dependencies.csv`; decomposition/register slices | N/A | N/A | N/A | N/A |
