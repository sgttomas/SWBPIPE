# Guidance: DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Purpose

This deliverable reconciles the implemented rigid and semi-rigid component schema evidence without importing protected component data. Its value is to keep the implementation boundary explicit: the software may structure and validate user-supplied component information, but it must not ship hidden catalog knowledge as defaults.

## Principles

- Use schema and validation to separate implemented component shape from protected component data.
- Treat dimensions, weights, COGs, and stiffness behavior as project/library inputs with provenance.
- Prefer explicit `TBD` or diagnostics over plausible defaults.
- Keep public examples synthetic, minimal, and labeled as examples rather than engineering recommendations.
- Keep mechanics-facing data compatible with a 3D centerline/frame model while preserving local-analysis handoff options.

## Considerations

The current component schema and strict fixture implement a base rigid/semi-rigid family contract for `valve`, `flange`, `reducer`, `rigid`, and `specialty`. The implemented base contract includes rigid body length, end-size/reference slots, weight, center of gravity, `linear_stiffness`, `rotational_stiffness`, stiffness behavior reference, and source/manufacturer reference slots. Human/project authority must still decide coordinate conventions, exact solver treatment of stiffness inputs, accepted source catalogs, public fixture-value policy, import formats, review disposition, dependency satisfaction, lifecycle closure, and any stricter per-family profiles.

Public fixture data is especially sensitive. Even common-looking valve, flange, reducer, rigid, or specialty dimensions, weights, COGs, and stiffness values may originate from protected standards, catalogs, or vendor sources. The current invented fixture keeps actual values missing and uses schema-shape-only/private-value policies; any public value example still needs accepted source and redistribution policy.

Preferred vocabulary for downstream work:

- `user-supplied data`: project data entered by a user or imported into a private project/library with provenance.
- `rights-cleared library data`: reusable data with documented source, redistribution status, contributor certification, and review disposition.
- `synthetic fixture`: public test/example data created only to exercise validation paths and labeled non-authoritative.
- `rigid component`: a component represented for global analysis without flexible element behavior except explicitly modeled mass/geometry effects.
- `semi-rigid component`: a component with user/manufacturer-supplied stiffness behavior; no default stiffness may be inferred.

## Trade-offs

| Choice | Benefit | Risk / mitigation |
|---|---|---|
| Implemented shared rigid/semi-rigid family contract | Reduces duplication across valves, flanges, reducers, rigid placeholders, and specialty items | May hide family-specific validation; stricter per-family profiles remain `TBD` |
| Family-specific records | Clearer validation per family | More schema surface; keep common provenance/unit fields consistent |
| Split stiffness into `linear_stiffness` and `rotational_stiffness` | Aligns with accepted unit dimensions and addresses the generic-stiffness audit concern technically | Exact solver consumption remains `TBD`; require user/manufacturer provenance |
| Schema-shape public fixtures | Enables tests without protected data | Must remain non-authoritative and blocked from mechanics use until reviewed values and source policy exist |

## Examples

- Current strict fixture example: `comp.invented.rigid.alpha` contains schema slots for rigid body length, connection reference, weight, COG, `linear_stiffness`, and `rotational_stiffness`, but each actual value remains missing.
- `TBD`: Public fixture value examples must be synthetic or user-supplied with documented rights and accepted redistribution policy before they can be added.
- Synthetic public fixtures are acceptable only as validation examples. They must be labeled non-authoritative and must not be used to imply engineering recommendations, code compliance, catalog equivalence, or vendor performance.
- `ASSUMPTION`: The implemented common component identity/provenance envelope remains appropriate across component families because AB-00-04 and OPS-K-DATA-3 require deterministic, provenance-preserving records.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| PKG03-DEL-03-05-PKG02-001 | Earlier audit found `specialty` in PKG-03 but not in the PKG-02 canonical component enum. | `Review_Findings.csv` row PKG03-DEL-03-05-PKG02-001 | `tests/test_component_section_schema.py` verifies `ComponentType` equals the PKG-02 canonical enum. | `Specification.md`; schema evidence interpretation | Preserve the recorded Gate C disposition. | `ACCEPT_AS_IS` / `RESOLVED` (2026-06-05) |
| PKG03-DEL-03-05-PKG02-002 | Earlier audit found generic `stiffness` dimension ambiguous against PKG-02 units. | `Review_Findings.csv` row PKG03-DEL-03-05-PKG02-002 | `ComponentQuantityDimension` uses `linear_stiffness` and `rotational_stiffness`; tests verify accepted dimensions and retired dimensions are absent. | `Specification.md`; schema evidence interpretation | Preserve the recorded Gate C disposition. | `ACCEPT_AS_IS` / `RESOLVED` (2026-06-05) |

## D-41 R5 T2B PDU-013 Boundary

An invented or rights-safe COG magnitude would not close R10 by itself. Without an accepted coordinate convention and reference frame, such a value is ambiguous at the mechanics boundary. Preserve that ambiguity as a held residual rather than inventing a convention.

PDU-023 scalar paths document identity-preserving copy-through only. They do not turn component metadata into a solver result or runtime envelope.
