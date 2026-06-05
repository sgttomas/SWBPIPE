---
doc_id: DEL-03-02-GUIDANCE
doc_kind: deliverable.guidance
status: draft
created: 2026-04-30
deliverable_id: DEL-03-02
package_id: PKG-03
---

# Guidance: Pipe Section and Component Library Schema

## Purpose

This guidance explains how to use and extend the implemented DEL-03-02 schema evidence. The deliverable enables private pipe section and component libraries while preserving provenance, redistribution status, review status, unit safety, completeness findings, diagnostics, and the public/private data boundary.

## Principles

- Treat the schema as a carrier for user/private or lawfully redistributable library records, not as a source of public engineering tables.
- Keep protected dimensional tables, protected standards content, proprietary catalog values, allowables, SIFs, flexibility factors, and code-derived values out of public defaults.
- Make missing values visible as validation findings, completeness flags, or diagnostics; do not fill them silently.
- Require provenance and redistribution status wherever a value could affect engineering reliance or public contribution review.
- Keep dimensional and mass-property values unit-aware.
- Separate detailed component-family mechanics from the generic library schema unless a sealed adjacent DEL-03 deliverable authorizes specialization.

## Considerations

- This reconciliation records implemented evidence in `schemas/component.schema.yaml`, `schemas/section.schema.yaml`, the strict split fixtures, the combined legacy fixture pointer, and `tests/test_component_section_schema.py`; it does not edit those repository-level artifacts.
- SOW-018 is narrower than several adjacent PKG-03 deliverables. The component schema includes generic bend, branch, rigid, and expansion-joint family contracts and slots, but detailed mechanics, accepted values, and editor behavior remain outside this reconciliation scope.
- Provenance and redistribution schema mechanisms distinguish private, public-permissive, unknown-source, protected-suspected, rejected, pending, accepted, and quarantined states. Accepted public source catalogs, source/license disposition, redistribution acceptance, and public fixture value policy remain `TBD`.
- Public example data must be invented, original, public-domain, or otherwise supported by documented redistribution rights.
- If any implementation needs a protected source table to define defaults, stop and route the value to user-supplied/private data handling instead.

## Trade-offs

| Topic | Conservative position |
|---|---|
| Exact field names | Implemented schema field names are evidence; changes require a sealed schema review. |
| Component-family depth | Keep generic family contracts and schema slots here; defer detailed mechanics and accepted engineering values to their owning DEL-03 deliverables. |
| Default dimensional values | Do not provide. Public defaults risk protected-table or proprietary-data leakage. |
| Provenance strictness | Prefer explicit `unknown`, `protected_suspected`, `quarantined`, or `TBD` over weak inferred source claims. |
| Public contribution acceptance | Requires documented rights and review; schema can support the metadata but does not itself approve contribution. |

## Examples

- Safe example approach: use invented pipe section/component records in tests with engineering values omitted or clearly non-engineering, plus explicit provenance/review status.
- Unsafe example approach: copying nominal pipe dimensions, vendor catalog weights, protected code tables, or proprietary component geometry into public fixtures.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-03-02-CF-001 | `_CONTEXT.md` identifies SOFTWARE_DECOMP revision 0.7, while `_REFERENCES.md` still describes the decomposition reference as accepted v0.2. | `_CONTEXT.md#Decomposition Reference` | `_REFERENCES.md#Decomposition and Registers` | Datasheet References; Procedure Records | Treat `_CONTEXT.md` and sealed brief revision 0.7 as current basis for this run; route `_REFERENCES.md` cleanup to a metadata owner because it is outside the four-doc write target. | RESOLVED_BY_HUMAN: current `_CONTEXT.md` and `_REFERENCES.md` both cite SOFTWARE_DECOMP revision 0.7; prior v0.2 reference is stale/superseded. |
