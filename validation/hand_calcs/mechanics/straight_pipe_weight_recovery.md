# MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY

## Purpose

Straight-pipe benchmark for the explicit weight hook and local axial recovery
path.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

The straight pipe starts at node `1` `(0.0, 0.0, 0.0)` and ends at node `3`
`(2.0, 0.0, 0.0)`.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `L` | 2.0 | m | length |
| `E` | 1200.0 | Pa | stress |
| `A` | 3.0 | m^2 | area |
| `m` | 2.5 | kg/m | mass_per_length |
| `g` | 9.0 | m/s^2 | acceleration |
| `u_j` | 0.01 | m | length |

## Expected Values

Weight force per length:

```text
w = m g
  = 2.5 * 9.0
  = 22.5
```

Recovered local axial end force at node `j`:

```text
F_j = E A u_j / L
    = 1200.0 * 3.0 * 0.01 / 2.0
    = 18.0
```

Tolerance policy: `TBD`.
