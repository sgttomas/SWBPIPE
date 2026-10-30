# MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE

## Purpose

Invented validation-local evidence proving that the TP-PHYS-014 canonical
`analytical_solver_model` payload can produce a traceable solve-result
boundary using existing result-export vocabulary.

This note does not add public export, headless-runner, GUI, report,
persistence, release, rule-check, or professional-reliance behavior.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Payload:
  `validation/benchmarks/mechanics/fixtures/tp_phys_014_canonical_analytical_payload.json`.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or
  proprietary data.

## Traceability Chain

```text
physical source ref PHYS-1
  -> analytical_solver_model ANALYTICAL-TP-PHYS-014
  -> validation-local straight-pipe/user-load DTOs
  -> solver load vector LC-TP-PHYS-014
  -> result envelope MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE
```

Existing result-export vocabulary used by the validation crate:

- `Reference`
- `Provenance`
- `QuantityResult`
- `ResultTraceLink`
- `ResultMetadata`
- `ResultSet`
- `Diagnostic`
- `ResultEnvelope`
- `ProfessionalBoundary`

The envelope uses `MECHANICS_SOLVED` and `HUMAN_REVIEW_REQUIRED`. It does not
include a rule-pack reference or any compliance/certification/approval claim.

## Result Records

The solved mechanics values are inherited from the TP-PHYS-014 canonical
payload derivation.

| Result | Family | Object ref | Basis ref | Value | Unit | Dimension |
|---|---|---|---|---:|---|---|
| `result:disp:node-N-2:uy` | displacement | `node:N-2` | `load_case:LC-TP-PHYS-014` | -0.04533333333333334 | m | length |
| `result:rotation:node-N-2:rz` | rotation | `node:N-2` | `load_case:LC-TP-PHYS-014` | -0.014666666666666668 | rad | angle |
| `result:load-vector:node-N-1:uy` | force | `solver_load_vector:node-N-1` | `load_case:LC-TP-PHYS-014` | -6.0 | N | force |
| `result:load-vector:node-N-1:rz` | moment | `solver_load_vector:node-N-1` | `load_case:LC-TP-PHYS-014` | -4.666666666666667 | N-m | moment |
| `result:load-vector:node-N-2:uy` | force | `solver_load_vector:node-N-2` | `load_case:LC-TP-PHYS-014` | -6.0 | N | force |
| `result:load-vector:node-N-2:rz` | moment | `solver_load_vector:node-N-2` | `load_case:LC-TP-PHYS-014` | 4.666666666666667 | N-m | moment |
| `result:reaction:support-N-1:uy` | reaction | `support:N-1-anchor` | `load_case:LC-TP-PHYS-014` | 12.0 | N | force |
| `result:reaction:support-N-1:rz` | reaction | `support:N-1-anchor` | `load_case:LC-TP-PHYS-014` | 24.0 | N-m | moment |
| `result:force:element-E-1:midspan:shear-y` | force | `element:E-1` | `load_case:LC-TP-PHYS-014` | 4.0 | N | force |
| `result:moment:element-E-1:midspan:bending-z` | moment | `element:E-1` | `load_case:LC-TP-PHYS-014` | 4.0 | N-m | moment |

Force and moment records carry result metadata with component, coordinate
system, location, recovery basis, and sign convention. The station records use
the existing `midspan` location and `interpolated_from_endpoint_resultants`
basis vocabulary because no more specific station-resultant schema term exists
in the current result-export contract.

## Runtime Trace-Chain Evidence

The four load-vector result records carry runtime-derived trace chains. The
fixture extracts the canonical payload load-case ID and load record index,
builds adapter DTO anchors using the accepted
`dto:load_application:<load-case>:<index>` convention, then joins those anchors
to actual `apply_straight_pipe_equivalent_user_loads` nodal contribution traces
before constructing each result value.

Each load-vector value has six links: two payload load records times three
runtime hops:

```text
load_case LC-TP-PHYS-014:load:<index>
  -> adapter_dto dto:load_application:LC-TP-PHYS-014:<index>
  -> solver_nodal_load_contribution tp-phys-014-load-...:node-<N>:<dof>
  -> result_value result:load-vector:node-<N>:<dof>
```

The current slice does not add station/resultant trace chains, reaction trace
chains, or displacement/rotation scalar trace chains. Those remain future
schema/runtime alignment work unless the same runtime evidence is extended
without section-property transport changes.

## Diagnostic And Boundary Evidence

The validation-local envelope includes one informational diagnostic record:

```text
code = TP_PHYS_015A_RESULT_BOUNDARY_EVIDENCE
class = ASSUMPTION_WARNING
severity = info
```

This diagnostic records that the envelope is benchmark evidence only and that
result-export/headless-runner fit remains assigned to adjacent deliverables.

The result-export validator reports zero blocking diagnostics for the in-memory
envelope. That validation proves the existing result vocabulary can carry the
canonical displacement, reaction/load-vector, station-resultant, load-vector
trace-chain, diagnostic, provenance, and source-reference evidence.

## Explicit Gaps

- Public result export serialization remains outside this TP-PHYS-015A slice.
- Headless-runner contract fit remains outside this TP-PHYS-015A slice.
- Non-load result values do not yet carry field-level scalar trace chains.
- Release tolerance policy remains `TBD`.
- Final export schema expansion for station-resultant-specific basis wording
  remains `TBD`.

## Boundary

This note records validation-local mechanics evidence only. It does not define
release tolerances, rule checks, allowables, stress categories,
SIF/flexibility factors, public API/CLI behavior, report wording, code
compliance, certification, sealing, approval, or professional acceptance.

Tolerance policy: `TBD`.
