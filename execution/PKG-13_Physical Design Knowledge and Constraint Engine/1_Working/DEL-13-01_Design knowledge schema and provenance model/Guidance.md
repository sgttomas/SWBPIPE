# Guidance: DEL-13-01 Design knowledge schema and provenance model

## Purpose

This deliverable prepares the design-knowledge schema/provenance surface for the PKG-13 physical design knowledge and constraint engine. It exists so the physical model can carry user-supplied design context, record where that context came from, expose missing assumptions, and remain inside the public/private data and professional-responsibility boundaries. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067, OBJ-014, and PKG-13.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Schema first | Treat the implemented `schemas/design_knowledge.schema.json` as the contract surface for design knowledge records; runtime behavior and adjacent deliverable consumption remain deferred where not implemented. | `_CONTEXT.md` Anticipated Artifacts; `_CONTEXT.md` Architecture Basis Injection; `RUN_2026-05-04_IMPLEMENTATION.md`; `MEMORY.md` |
| User-supplied by default | Design knowledge values are user/project supplied unless explicitly invented or otherwise cleared for public redistribution. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067; `docs/IP_AND_DATA_BOUNDARY.md` |
| Provenance is part of the record | Source notes, source status, review status, redistribution status, and assumptions should be close to the data they qualify. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `_CONTEXT.md` Description |
| No silent defaults | Missing design knowledge, missing units, or unresolved assumptions should surface as findings/TBD records. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.4 |
| Physical source of truth | Design knowledge should support the editable physical model without bypassing model schema validation or downstream transformation traceability. | `docs/SPEC.md` section 3; `docs/TYPES.md` Model registry entry |
| Professional boundary | Avoid terms that imply software certification, approval, sealing, authentication, or code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4 |

## Considerations

| Topic | Consideration |
|---|---|
| Category granularity | The implemented schema defines concrete record kinds for endpoints, line data, routing corridors, zones, equipment interfaces, requirements, and owner/project metadata. GUI presentation, editing workflows, and constraint-engine behavior remain downstream. |
| Owner/project metadata | Owner/project metadata is in scope, but public examples must not include private project data or protected owner standards. |
| Equipment interfaces | The schema provides `equipment_ref`, `interface_role`, `location`, `interface_parameters`, source notes, assumptions, and provenance. Runtime equipment-picking behavior and downstream transform semantics remain `TBD`. |
| Access, slope, drain, and vent requirements | These may reflect owner/project requirements. The schema should provide slots and provenance without embedding protected or private criteria. |
| No-go and supportable zones | The schema provides `ZoneRecord` and `GeometryPayload` structures for no-go, supportable, access, maintenance, and operating zones. Coordinate-frame policy, tolerance handling, and numeric clearance/support criteria remain `TBD` and must not be invented in public artifacts. |
| Relationship to constraints | DEL-13-02 and DEL-13-03 own constraint entity/provenance and validation behavior. DEL-13-01 should provide design knowledge records that can later be consumed by those deliverables. |
| Relationship to transformation | DEL-13-04 owns physical-to-analytical transformation. DEL-13-01 should preserve enough provenance/assumption structure for transformation warnings, but exact warning classes remain downstream. |

## Trade-offs

| Trade-off | Preferred posture | Rationale |
|---|---|---|
| Rich taxonomy vs premature specificity | Use the implemented schema taxonomy for records and provenance; leave downstream numeric criteria, validation algorithms, GUI workflows, and transform behavior `TBD` where sources are silent. | `docs/CONTRACT.md` OPS-K-AGENT-1 prohibits invented engineering values or scope. |
| Public examples vs realistic owner data | Use invented or cleared public data only. | `docs/IP_AND_DATA_BOUNDARY.md` excludes private owner standards and protected data. |
| Embedded validation vs separate constraint engine | Keep record schema and provenance in this deliverable; leave deterministic constraint validation to DEL-13-03. | `execution/_Decomposition/SOFTWARE_DECOMP.md` separates DEL-13-01, DEL-13-02, DEL-13-03, and DEL-13-04. |
| Flexibility vs interoperability | Preserve stable identifiers, source notes, unit metadata, and review status so records can participate in persistence, diagnostics, and handoff workflows. | `docs/SPEC.md` sections 3, 4, and 4.5. |

## Examples

No authoritative example payloads or public fixtures are implemented for this deliverable. Example JSON records, numeric values, owner/project metadata samples, and public fixtures remain `TBD`. Any later examples must be invented or otherwise cleared for public redistribution and must not copy protected standards text, protected tables, private project data, or proprietary vendor data.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflicts identified during the implementation-evidence refresh. Unsupported downstream details remain marked `TBD` rather than resolved by inference. | N/A | N/A | N/A | N/A | N/A |
