# STRESS-TP-PHYS-007-STATION-SWEEP-STRESS

## Purpose

Invented stress recovery benchmark linking ordered straight-pipe
station-resultant sweep recovery to ordered mechanics-only station stress
recovery. It verifies that caller-supplied station order is preserved through
the station-resultant sweep and the station-stress sweep.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor note: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, allowables, SIF/flexibility factors, or fatigue criteria.

## Invented Pipe And Sweep Inputs

The pipe is aligned to global `X`, with local `Y` matching global `Y`.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Length, `L` | 4.0 | m | length |
| Elastic modulus, `E` | 1000.0 | Pa | stress |
| Shear modulus, `G` | 400.0 | Pa | stress |
| Area, `A` | 3.0 | m^2 | area |
| Second moment about local `Y`, `I_y` | 1.5 | m^4 | second_moment_area |
| Second moment about local `Z`, `I_z` | 2.0 | m^4 | second_moment_area |
| Torsion constant, `J` | 1.0 | m^4 | second_moment_area |
| I-end shear, `V_y,i` | 4.0 | N | force |
| I-end bending, `M_z,i` | 8.0 | N-m | moment |
| Uniform local `Y` load, `q` | -2.0 | N/m | force_per_length |
| Load span, `a/L` to `b/L` | 0.25 to 0.75 | ratio | dimensionless |
| Requested station fractions | 0.75, 0.25, 0.5, 1.0 | ratio | dimensionless |

The station fractions are intentionally unsorted. The benchmark checks that
the result vectors remain in this requested order.

## Station Resultant Sweep

For a station at distance `x` from the I end, the active load segment is the
portion of `[a, b]` that lies at or before `x`.

```text
active_length = max(0, min(x, b) - a)
lever = x active_length - (active_end^2 - a^2) / 2
V_y(x) = V_y,i + q active_length
M_z(x) = M_z,i - V_y,i x - q lever
```

With `L = 4.0 m`, `a = 1.0 m`, and `b = 3.0 m`, the expected station
resultants in requested order are:

| Requested index | Station fraction | `V_y` | `M_z` |
|---:|---:|---:|---:|
| 0 | 0.75 | 0.0 N | 0.0 N-m |
| 1 | 0.25 | 4.0 N | 4.0 N-m |
| 2 | 0.5 | 2.0 N | 1.0 N-m |
| 3 | 1.0 | 0.0 N | 0.0 N-m |

## Invented Stress Section Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Area, `A` | 3.0 | m^2 | area |
| Section modulus about `Y`, `Z_y` | 2.5 | m^3 | section_modulus |
| Section modulus about `Z`, `Z_z` | 2.0 | m^3 | section_modulus |
| Polar section modulus style input | 1.0 | m^3 | section_modulus |
| Pressure wall thickness | 0.5 | m | length |

Only the mechanics station stress components are recovered. Pressure is not
supplied.

## Expected Stress Components

Axial force is zero at every station in this fixture, so:

```text
sigma_axial = 0.0 / 3.0 = 0.0
```

Bending normal stress from `M_z` uses the explicit invented section modulus
`Z_z = 2.0`:

```text
sigma_bending_z = M_z / Z_z
```

| Requested index | Station fraction | `M_z` | `bending_normal_z` | `axial_normal` |
|---:|---:|---:|---:|---:|
| 0 | 0.75 | 0.0 N-m | 0.0 Pa | 0.0 Pa |
| 1 | 0.25 | 4.0 N-m | 2.0 Pa | 0.0 Pa |
| 2 | 0.5 | 1.0 N-m | 0.5 Pa | 0.0 Pa |
| 3 | 1.0 | 0.0 N-m | 0.0 Pa | 0.0 Pa |

## Boundary

This fixture is mechanics-only stress recovery evidence. It does not compare
to allowables, classify code stress categories, apply fatigue rules, use
SIF/flexibility factors, or make professional approval or code-compliance
claims.

Tolerance policy: `TBD`.
