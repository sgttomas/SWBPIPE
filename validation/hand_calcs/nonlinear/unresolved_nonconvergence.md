# NL-NONCONVERGENCE-LIMIT-ORIGINAL

## Purpose

Invented non-convergence fixture for a support that changes state at the
configured iteration limit.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Iteration | 4 | count | dimensionless |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Trial rotation | 0.002 | rad | rotation |
| Trial rotational reaction | 0.0 | N-m | moment |
| Prior state | inactive | state label | dimensionless |

## Expected Values

The released negative-reaction support bears against positive rotation, so the
penetrating `0.002 rad` trial rotation re-engages the support at the
configured iteration limit (DEC-067 state-switched test). The expected
diagnostic is the software non-convergence diagnostic from the committed solver
diagnostics API.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `trial_rotational_displacement` | 0.002 | rad | rotation |
| `iteration_count` | 4.0 | count | dimensionless |
| `active_set_residual` | 1.0 | count | dimensionless |
| Expected converged flag | false | boolean | dimensionless |

Tolerance policy: `TBD`.
