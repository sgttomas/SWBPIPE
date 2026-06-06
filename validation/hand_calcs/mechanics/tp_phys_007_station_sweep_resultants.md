# MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS

## Purpose

Invented mechanics benchmark for an ordered station-resultant sweep on a
partial-span straight-pipe distributed load. The fixture checks that the sweep
API preserves the requested station order while recovering shear and bending
from the solved fixed-free displacement path.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor note: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or
  proprietary data.

## Invented Inputs

The member is a two-node straight pipe aligned to global `X`.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `L` | 4.0 | m | length |
| `E` | 1000.0 | Pa | stress |
| `G` | 400.0 | Pa | stress |
| `A` | 3.0 | m^2 | area |
| `I_y` | 1.5 | m^4 | second_moment_area |
| `I_z` | 2.0 | m^4 | second_moment_area |
| `J` | 1.0 | m^4 | second_moment_area |
| Uniform local/global `Y` load, `q` | -2.0 | N/m | force_per_length |
| Span start, `a/L` | 0.25 | ratio | dimensionless |
| Span end, `b/L` | 0.75 | ratio | dimensionless |
| Requested station fractions | 0.75, 0.25, 0.5, 1.0 | ratio | dimensionless |
| Node `0` support | fixed | count | dimensionless |
| Node `1` support | free | count | dimensionless |

## Fixed-Free Basis

The equivalent nodal loads and fixed-free solve are the same invented public
case as `MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT`.

For the load over `0.25 <= x/L <= 0.75`:

```text
F_y,i = -2.0
M_z,i = -11 / 6
F_y,j = -2.0
M_z,j =  11 / 6
```

With node `0` fixed, the solved node `1` displacement path is:

```text
u_y,1 = -7 / 500
theta_z,1 = -13 / 3000
```

The loaded I-end resultants recovered from `K u - f_equivalent` are:

```text
V_y,i = 4.0
M_z,i = 8.0
```

## Station Sweep Resultants

For a station at distance `x` from the I end, the active loaded interval is:

```text
s0 = 1.0
s1 = 3.0
active_start = s0
active_end = min(x, s1)
active_length = max(0.0, active_end - active_start)
lever_integral = integral_active_start^active_end (x - s) ds
```

The repository straight-pipe signed-resultant recurrence is:

```text
V_y(x) = V_y,i + q active_length
M_z(x) = M_z,i - V_y,i x - q lever_integral
```

The requested station fractions are intentionally unsorted. Expected output
order and values are:

| Requested index | Station fraction | `x` | `V_y(x)` | `M_z(x)` |
|---:|---:|---:|---:|---:|
| 0 | 0.75 | 3.0 | 0.0 | 0.0 |
| 1 | 0.25 | 1.0 | 4.0 | 4.0 |
| 2 | 0.5 | 2.0 | 2.0 | 1.0 |
| 3 | 1.0 | 4.0 | 0.0 | 0.0 |

At station `0.75`, the full loaded span has just been accumulated and the
internal shear and bending return to the free-side zero values. At station
`1.0`, no additional load exists beyond the loaded span, so the same zero
resultants remain.

## Boundary

This note is a code-neutral mechanics derivation for an invented fixture. It
does not define release tolerances, code stress categories, allowables,
fatigue criteria, SIF/flexibility factors, professional approval, or
code-compliance claims.

Tolerance policy: `TBD`.
