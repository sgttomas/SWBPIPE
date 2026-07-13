# Guidance: DEL-09-01 Mechanics benchmark suite

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-09-01-DECL-003`.

## Purpose

This deliverable prepares the evidence boundary for a future mechanics benchmark suite. It exists to make open mechanics verification repeatable and reviewable while preserving the data boundary between public mechanics cases and protected standards or proprietary examples.

## Principles

- Treat benchmarks as verification evidence, not as solver implementation.
- Use public/original mechanics cases with clear provenance and redistribution status.
- Keep final comparison tolerances and release thresholds as `TBD` until approved by the proper human authority.
- Make units, assumptions, expected result fields, diagnostics, and limitations explicit.
- Preserve the distinction between mechanics verification, user-rule checks, and professional approval.

## Considerations

Cantilever, frame, thermal-growth, imposed-displacement, and stiffness-transform cases exercise different solver interfaces. Future implementation should identify which solver/load/support contracts each case depends on before marking a benchmark executable.

Thermal-growth and imposed-displacement cases can look similar to code or owner-standard examples if copied from protected sources. Future workers should create original cases or use permissively licensed/public-domain mechanics references, then record source and license status before publishing fixtures.

Stiffness-transform cases should isolate local-to-global coordinate behavior and sign conventions. Concrete transform matrices, geometry, expected values, and numerical tolerances are not authorized by this setup pass.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Simple hand checks vs representative piping behavior | Start with analytically transparent cases; broaden only when provenance and expected-result derivations remain reviewable. |
| Strict tolerances vs solver maturity | Record exact and approximate expected values separately when authorized; final acceptance tolerances remain `TBD`. |
| Public benchmark usefulness vs IP risk | Prefer original/public mechanics cases even when protected examples are familiar. |
| Release gating vs exploratory regression | Distinguish required release benchmarks from advisory regression checks after human QA policy is approved. |

## Examples

Concrete benchmark geometries, material values, loads, boundary conditions, expected results, and comparison tolerances are `TBD`. Future examples must be original, public-domain, or permissively licensed and must include provenance.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A | N/A | N/A | N/A |

## Open Enrichment Items

| Item | Status |
|---|---|
| Approved benchmark fixture list and source basis | TBD |
| Approved numerical tolerance/comparison policy | TBD |
| Approved fixture schema and result-envelope comparison format | TBD |
| Release-gating vs advisory benchmark classification | TBD |

## D-41 R5 T2B PDU-013 Boundary

Fixture-local dimensional rigor is useful evidence, but it must not be promoted into a claim about an accepted project-wide unit catalog. The remaining catalog/conversion decision is upstream and is recorded as held rather than filled with new constants.
