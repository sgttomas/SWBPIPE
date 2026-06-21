# NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a free
trial displacement exceeds an explicit positive gap clearance and the next
linearized solve converges with the gap closed.

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
| Applied axial force | 10.0 | N | force |
| Gap clearance | 0.05 | mm | length |
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | inactive | state label | dimensionless |

## Expected Values

The first free linear solve produces a displacement larger than the explicit
clearance, so the gap becomes active. The second solve prescribes the clearance
and the active set remains unchanged.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Expected iteration count | 2 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | active | state label | dimensionless |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `TBD`.
