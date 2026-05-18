# Guidance: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Purpose

This deliverable prepares the evidence frame for implementing rigid and semi-rigid component models without importing protected component data. Its value is to make the later implementation boundary explicit: the software may structure and validate user-supplied component information, but it must not ship hidden catalog knowledge as defaults.

## Principles

- Use schema and validation to separate component shape from component data.
- Treat dimensions, weights, COGs, and stiffness behavior as project/library inputs with provenance.
- Prefer explicit `TBD` or diagnostics over plausible defaults.
- Keep public examples synthetic, minimal, and labeled as examples rather than engineering recommendations.
- Keep mechanics-facing data compatible with a 3D centerline/frame model while preserving local-analysis handoff options.

## Considerations

Reducer, flange, valve, and specialty-item records will likely need different descriptive fields, but this setup pass does not decide the final field taxonomy. Human/project authority must still decide coordinate conventions, stiffness representation, and which fields are mandatory per component family.

Public fixture data is especially sensitive. Even common-looking valve, flange, or reducer dimensions and weights may originate from protected standards, catalogs, or vendor sources. Use invented-looking but clearly synthetic examples only when future implementation needs public fixtures, and keep them out of engineering-result claims.

Preferred vocabulary for downstream work:

- `user-supplied data`: project data entered by a user or imported into a private project/library with provenance.
- `rights-cleared library data`: reusable data with documented source, redistribution status, contributor certification, and review disposition.
- `synthetic fixture`: public test/example data created only to exercise validation paths and labeled non-authoritative.
- `rigid component`: a component represented for global analysis without flexible element behavior except explicitly modeled mass/geometry effects.
- `semi-rigid component`: a component with user/manufacturer-supplied stiffness behavior; no default stiffness may be inferred.

## Trade-offs

| Choice | Benefit | Risk / mitigation |
|---|---|---|
| One shared rigid-component base model | Reduces duplication across valves, flanges, reducers, and specialty items | May hide family-specific validation; use component-family validation profiles |
| Family-specific records | Clearer validation per family | More schema surface; keep common provenance/unit fields consistent |
| Allow semi-rigid stiffness fields | Supports cases beyond ideal rigid masses | Must avoid default stiffness assumptions; require user/manufacturer provenance |
| Synthetic public fixtures | Enables tests without protected data | Must be labeled non-authoritative and reviewed for protected-content risk |

## Examples

- `TBD`: Future fixture value examples must be synthetic or user-supplied with documented rights.
- Synthetic public fixtures are acceptable only as validation examples. They must be labeled non-authoritative and must not be used to imply engineering recommendations, code compliance, catalog equivalence, or vendor performance.
- `ASSUMPTION`: A future implementation may use a common component identity/provenance envelope across component families, because AB-00-04 and OPS-K-DATA-3 both require deterministic, provenance-preserving records.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No direct conflict found in accessible setup sources. | N/A | N/A | N/A | N/A | TBD |
