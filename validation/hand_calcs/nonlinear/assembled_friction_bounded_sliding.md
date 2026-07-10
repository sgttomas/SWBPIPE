# NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
sliding friction support applies the DEC-067 bounded `+/- mu*N` tangential
force opposing motion instead of a full DOF release, using explicit
normal-reaction input evidence from the current iterate.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Node count | 2 | count | dimensionless |
| Member length | 1.0 | mm | length |
| Axial stiffness basis | 100.0 | N/mm | linear_stiffness |
| Applied axial force | 8.0 | N | force |
| Friction coefficient | 0.25 | ratio | dimensionless |
| Explicit normal reaction evidence | 12.0 | N | force |
| Friction limit `mu*N` | 3.0 | N | force |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | sliding | state label | dimensionless |

## Expected Values

The support is seeded sliding, so no solved iterate exists yet to orient or
scale the bounded force: the first assembled linear solve releases the DOF
(free trial displacement `8.0 / 100.0 = 0.08 mm`) and the loop defers
convergence past this first iterate so the seeded state cannot converge on a
fully released (unbounded) result.

On the second iteration the sliding direction is taken from the prior
iterate's positive displacement, and the bounded tangential force
`-(0.25 * 12.0) = -3.0 N` opposes it. The net drive is `8.0 - 3.0 = 5.0 N`,
so the displacement is `5.0 / 100.0 = 0.05 mm` and the support's reported
tangential reaction is the bounded `-3.0 N`. The state persists sliding under
the deterministic anti-chatter rule and the active-set residual reaches zero.
The converged result is identical when the same fixture is seeded sticking
instead (transition path), so the outcome does not depend on the seeded
initial state. No path-dependent or load-step friction history model is
implied.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Released free displacement (iteration 1) | 0.08 | mm | length |
| Bounded sliding force (iteration 2) | -3.0 | N | force |
| Expected final displacement | 0.05 | mm | length |
| Expected final tangential reaction | -3.0 | N | force |
| Expected iteration count | 2 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | sliding | state label | dimensionless |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-free-dof-work-residual-validation-v1`.
Displacement/reaction delta policy: `DEC-046-CV-B-displacement-reaction-delta-threshold-validation-v1`.
