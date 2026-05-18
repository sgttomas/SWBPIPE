# STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS

## Purpose

Invented stress recovery benchmark linking straight-pipe axial-effect
resultant recovery with explicit bending resultants and a local line load. It
verifies that the combined mechanics-only station resultants can feed
station stress recovery without pressure-basis inputs.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, allowables, SIF/flexibility factors, fatigue criteria, or private
  project data.

## Invented Pipe And Mechanics Inputs

The pipe is aligned to global `X`, with local `Y` matching global `Y`. The
fixture uses zero displacement evidence for the axial-effect path, then
combines the recovered I-end axial force with explicit I-end bending
resultants and a full-span local `Y` line load.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Length, `L` | 4.0 | m | length |
| Elastic modulus, `E` | 1000.0 | Pa | stress |
| Shear modulus, `G` | 400.0 | Pa | stress |
| Pipe area used by straight-pipe recovery | 6.0 | m^2 | area |
| Second moment about local `Y`, `I_y` | 1.5 | m^4 | second_moment_area |
| Second moment about local `Z`, `I_z` | 2.0 | m^4 | second_moment_area |
| Torsion constant, `J` | 1.0 | m^4 | second_moment_area |
| Axial-effect force, `F_a` | 120.0 | N | force |
| Element displacement vector for axial effect | all zeros | mixed | TBD |
| Explicit I-end shear, `V_y,i` | 4.0 | N | force |
| Explicit I-end bending, `M_z,i` | 8.0 | N-m | moment |
| Uniform local `Y` load, `q` | -2.0 | N/m | TBD |
| Station, `x/L` | 0.5 | ratio | dimensionless |

## Recovered And Combined Resultants

The straight-pipe axial-effect path supplies equivalent local axial loads of
`-F_a` at the I-end axial DOF and `+F_a` at the J-end axial DOF. With zero
displacement recovery, the corrected local force vector is the negative of
those equivalent loads, so:

```text
N_i = 120.0
```

The benchmark then combines that recovered axial force with explicit I-end
bending resultants and the local line load:

```text
V_y,i = 4.0
M_z,i = 8.0
q = -2.0
```

At midspan, `x = 2.0 m`. The full-span line load is active from `0.0 m` to
`2.0 m` on the I-side free body. The active length is `2.0 m`, and the lever
integral about the station is:

```text
lever = x active_length - active_end^2 / 2
      = 2.0 * 2.0 - 2.0^2 / 2
      = 2.0
```

The station resultants are:

```text
N_station = 120.0
M_z(x) = M_z,i - V_y,i x - q lever
       = 8.0 - 4.0 * 2.0 - (-2.0 * 2.0)
       = 4.0
```

| Resultant | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `N` | 120.0 | N | force |
| `M_z` | 4.0 | N-m | moment |

## Invented Stress Section Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Area, `A` | 6.0 | m^2 | area |
| Section modulus about `Y`, `Z_y` | 3.0 | m^3 | section_modulus |
| Section modulus about `Z`, `Z_z` | 2.0 | m^3 | section_modulus |
| Polar section modulus style input | 1.0 | m^3 | section_modulus |
| Torsion radius | 0.5 | m | length |

Pressure is not supplied for this benchmark.

## Expected Stress Components

Midspan station axial normal stress:

```text
sigma_axial = N_station / A
             = 120.0 / 6.0
             = 20.0
```

Bending normal stress from `M_z`:

```text
sigma_bending_z = M_z / Z_z
                = 4.0 / 2.0
                = 2.0
```

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `station_midspan_axial_force` | 120.0 | N | force |
| `station_midspan_bending_z` | 4.0 | N-m | moment |
| `station_midspan_axial_normal` | 20.0 | Pa | stress |
| `station_midspan_bending_normal_z` | 2.0 | Pa | stress |

## Boundary

This fixture is mechanics-only stress recovery evidence. It does not compare
to allowables, classify code stress categories, apply fatigue rules, use
SIF/flexibility factors, derive material behavior, or make professional
approval or code-compliance claims.

Tolerance policy: `TBD`.
