# MECH-CANTILEVER-TIP-FORCE

## Purpose

Cantilever benchmark for a two-node frame member with a lateral tip force.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `L` | 10.0 | m | length |
| `E` | 1200.0 | Pa | stress |
| `I_z` | 4.0 | m^4 | second_moment_area |
| `P_y` | 6.0 | N | force |

## Expected Values

Tip displacement in the local/global `Y` direction:

```text
delta_y = P_y L^3 / (3 E I_z)
        = 6.0 * 10.0^3 / (3 * 1200.0 * 4.0)
        = 0.4166666666666667
```

Fixed-end moment magnitude about local/global `Z`:

```text
M_z = P_y L
    = 60.0
```

Tolerance policy: `TBD`.
