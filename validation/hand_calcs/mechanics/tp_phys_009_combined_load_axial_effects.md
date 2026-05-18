# MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS

## Purpose

Invented mechanics benchmark for combined straight-pipe load assembly and
resultant recovery. The fixture prepares thermal and pressure primitive axial
effects, then includes them with an explicit partial-span distributed user load
in straight-pipe equivalent user-load assembly.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or
  proprietary data.

## Invented Inputs

The member is a two-node straight pipe aligned to global `X`.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `L` | 6.0 | m | length |
| `E` | 1000.0 | Pa | stress |
| `G` | 400.0 | Pa | stress |
| `A` | 4.0 | m^2 | area |
| `I_y` | 1.5 | m^4 | second_moment_area |
| `I_z` | 2.0 | m^4 | second_moment_area |
| `J` | 1.0 | m^4 | second_moment_area |
| `alpha` | 0.00001 | 1/K | thermal_expansion_coefficient |
| `DeltaT` | 75.0 | K | temperature_interval |
| `p` | 90.0 | Pa | pressure |
| `A_internal` | 0.1 | m^2 | area |
| Uniform local/global `Y` load, `q` | -2.0 | N/m | TBD |
| Span start, `a/L` | 0.25 | ratio | dimensionless |
| Span end, `b/L` | 0.75 | ratio | dimensionless |
| Node `0` support | fixed | count | dimensionless |
| Node `1` axial support | `U_x` restrained | count | dimensionless |

The fixture-local unit basis is explicit and no conversions are performed.

## Primitive Axial Effects

Thermal axial effect:

```text
F_thermal = E A alpha DeltaT
          = 1000.0 * 4.0 * 0.00001 * 75.0
          = 3.0 N
```

Pressure thrust effect:

```text
F_pressure = p A_internal
           = 90.0 * 0.1
           = 9.0 N
```

Total axial effect:

```text
F_total = 3.0 + 9.0
        = 12.0 N
```

The straight-pipe axial-effect equivalent load pair is:

```text
Node i, Ux = -12.0 N
Node j, Ux =  12.0 N
```

## Distributed User Load

For a local/global `Y` distributed load over `0.25 <= x/L <= 0.75`, the
straight-pipe equivalent nodal loads are computed from the element shape
functions over the loaded span:

```text
F_y,i = -3.0 N
M_z,i = -4.125 N-m
F_y,j = -3.0 N
M_z,j =  4.125 N-m
```

Combining distributed and axial-effect assembly gives the global load-vector
entries:

```text
Node 0 Ux = -12.0 N
Node 0 Uy =  -3.0 N
Node 0 Rz =  -4.125 N-m
Node 1 Ux =  12.0 N
Node 1 Uy =  -3.0 N
Node 1 Rz =   4.125 N-m
```

## Fixed-Free Transverse Solve With Axial Stop

Node `0` is fixed. Node `1` axial translation is restrained, so the axial
effect remains a recovered resultant. Node `1` transverse displacement and
rotation are solved from the free-end transverse load vector.

With `E I_z = 2000.0 N-m^2`, the node `1` transverse displacement path is:

```text
u_y,1     = -0.070875 m
theta_z,1 = -0.014625 rad
u_x,1     =  0.0 m
```

## Combined Resultants

The combined recovery helper subtracts both the distributed-load equivalent
loads and the axial-effect equivalent loads before station accumulation.

At the I end:

```text
N_i   = 12.0 N
V_y,i =  6.0 N
M_z,i = 18.0 N-m
```

For a station at distance `x` from the I end, the active loaded interval is:

```text
s0 = 1.5
s1 = 4.5
active_start = s0
active_end = min(x, s1)
active_length = max(0.0, active_end - active_start)
lever_integral = integral_active_start^active_end (x - s) ds
```

The repository straight-pipe signed-resultant recurrence is:

```text
N(x) = 12.0
V_y(x) = V_y,i + q active_length
M_z(x) = M_z,i - V_y,i x - q lever_integral
```

Expected station sweep values:

| Requested index | Station fraction | `x` | `N(x)` | `V_y(x)` | `M_z(x)` |
|---:|---:|---:|---:|---:|---:|
| 0 | 0.25 | 1.5 | 12.0 | 6.0 | 9.0 |
| 1 | 0.5 | 3.0 | 12.0 | 3.0 | 2.25 |
| 2 | 0.75 | 4.5 | 12.0 | 0.0 | 0.0 |
| 3 | 1.0 | 6.0 | 12.0 | 0.0 | 0.0 |

## Boundary

This note records a code-neutral mechanics derivation for an invented fixture.
It does not define release tolerances, rule checks, allowables, stress
categories, SIF/flexibility factors, or project-specific decisions.

Tolerance policy: `TBD`.
