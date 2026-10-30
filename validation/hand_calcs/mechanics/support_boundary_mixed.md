# MECH-SUPPORT-BOUNDARY-MIXED

## Purpose

Support-boundary benchmark for a mixed anchor, spring, and imposed rotation
preparation path.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

| Item | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Node `0` anchor | 6 restrained DOFs | count | dimensionless |
| Node `1`, `Uy` spring stiffness | 250.0 | N/m | linear_stiffness |
| Node `2`, `Rz` imposed rotation | 0.015 | rad | rotation |

## Expected Values

The anchor contributes six restrained DOFs. The imposed rotation also restrains
the affected rotational DOF. The spring remains a spring contribution and does
not enter the rigid restrained-DOF list.

```text
restrained_dof_count = 6 + 1
                     = 7

spring_stiffness = 250.0
imposed_rotation = 0.015
```

Tolerance policy: `TBD`.
