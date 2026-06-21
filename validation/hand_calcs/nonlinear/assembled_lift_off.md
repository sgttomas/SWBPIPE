# NL-ASSEMBLED-LIFT-OFF-ORIGINAL

## Purpose

Invented assembled-loop fixture for a two-node axial frame model where a
lift-off support loses contact, the frame re-solves, and the active set
converges.

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
| Maximum iterations | 4 | count | dimensionless |
| Active-set residual tolerance | 0.0 | count | dimensionless |
| Initial state | active | state label | dimensionless |

## Expected Values

The initially active lift-off support requires a positive reaction to remain in
contact. The first linearized solve produces the opposite reaction sign, so the
support releases. The second solve keeps the support inactive and the
active-set residual becomes zero.

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Expected iteration count | 2 | count | dimensionless |
| Expected final residual | 0.0 | count | dimensionless |
| Expected final state | inactive | state label | dimensionless |
| Expected converged flag | true | boolean | dimensionless |

Tolerance policy: `DEC-046-CV-B-active-set-count-validation-v1`.
