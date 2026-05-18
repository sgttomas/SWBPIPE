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

This guidance explains how to think about DEL-03-02 during later schema implementation. The deliverable exists to enable private pipe section and component libraries while preserving provenance, redistribution status, unit safety, and the public/private data boundary.

## Principles

- Treat the schema as a carrier for user/private or lawfully redistributable library records, not as a source of public engineering tables.
- Keep protected dimensional tables, protected standards content, proprietary catalog values, allowables, SIFs, flexibility factors, and code-derived values out of public defaults.
- Make missing values visible as validation findings, completeness flags, or diagnostics; do not fill them silently.
- Require provenance and redistribution status wherever a value could affect engineering reliance or public contribution review.
- Keep dimensional and mass-property values unit-aware.
- Separate component-family mechanics from the generic library schema unless a later sealed deliverable authorizes specialization.

## Considerations

- `DEL-03-02` is schema setup evidence only. It may describe future schema concepts, but it must not edit `schemas/component.schema.yaml` or `schemas/section.schema.yaml` during this pass.
- SOW-018 is narrower than several adjacent PKG-03 deliverables. Bends, branches, rigid components, expansion joints, and calculated section/mass properties have their own later deliverables and should not be fully implemented here.
- Provenance status should be expressive enough to distinguish private manufacturer/vendor data, public-permissive data, public-domain/original data, unknown-source data, and suspected protected content. Exact enum names are `TBD`.
- Public example data must be invented, original, public-domain, or otherwise supported by documented redistribution rights.
- If a later implementation needs a protected source table to define defaults, stop and route the value to user-supplied/private data handling instead.

## Trade-offs

| Topic | Conservative setup position |
|---|---|
| Exact field names | `TBD`; defer to implementation brief and schema review. |
| Component-family depth | Keep generic records here; defer bend/branch/rigid/expansion-joint specialization to later DEL-03 deliverables. |
| Default dimensional values | Do not provide. Public defaults risk protected-table or proprietary-data leakage. |
| Provenance strictness | Prefer explicit `UNKNOWN_SOURCE` or `TBD` over weak inferred source claims. |
| Public contribution acceptance | Requires documented rights and review; schema can support the metadata but does not itself approve contribution. |

## Examples

- Safe example approach: use invented pipe section/component records in tests with clearly artificial dimensions and explicit `PUBLIC_DOMAIN_OR_ORIGINAL` or equivalent provenance status.
- Unsafe example approach: copying nominal pipe dimensions, vendor catalog weights, protected code tables, or proprietary component geometry into public fixtures.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| DEL-03-02-CF-001 | `_CONTEXT.md` identifies SOFTWARE_DECOMP revision 0.4, while `_REFERENCES.md` still describes the decomposition reference as accepted v0.2. | `_CONTEXT.md#Decomposition Reference` | `_REFERENCES.md#Decomposition and Registers` | Datasheet References; Procedure Records | Treat `_CONTEXT.md` and sealed brief revision 0.4 as current basis for this run; route `_REFERENCES.md` cleanup to a metadata owner because it is outside the four-doc write target. | TBD |

