# STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS

## Purpose

Invented stress recovery benchmark linking partial-span straight-pipe
distributed-load station-resultant recovery to code-neutral station stress
recovery. It verifies that a valid local `Y` distributed load over normalized
span `[0.25, 0.75]` can feed mechanics-only station stress components.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, allowables, SIF/flexibility factors, or fatigue criteria.

## Invented Pipe And Load Inputs

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
| Uniform local/global `Y` load, `q` | -2.0 | N/m | force_per_length |
| Load span, `a/L` to `b/L` | 0.25 to 0.75 | ratio | dimensionless |
| Station, `x/L` | 0.5 | ratio | dimensionless |

## Displacement Evidence

For this cantilever-style fixture, node `0` is the fixed end and node `1` is
the free end. The spanned distributed-load equivalent nodal load at node `1`
is:

```text
F_y,j = -2.0
M_z,j = 1.8333333333333333
```

Using `E I_z = 1000.0 * 2.0 = 2000.0` and `L = 4.0`, the free-end bending
stiffness terms are:

```text
k_vv = 12 E I_z / L^3 = 375.0
k_vtheta = -6 E I_z / L^2 = -750.0
k_thetatheta = 4 E I_z / L = 2000.0
```

Solving the two-degree bending system gives the fixture displacement evidence:

| DOF | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Node `1`, `UY` | -0.014 | m | displacement |
| Node `1`, `RZ` | -0.004333333333333333 | rad | rotation |

## Midspan Station Resultants

The recovered loaded I-end resultants for the displacement and partial-span
load are:

```text
V_y,i = 4.0
M_z,i = 8.0
```

At station `x = 2.0 m`, the active loaded interval is from `1.0 m` to `2.0 m`.
The active length is `1.0 m`, and the lever integral about the station is:

```text
lever = x active_length - (active_end^2 - span_start^2) / 2
      = 2.0 * 1.0 - (2.0^2 - 1.0^2) / 2
      = 0.5
```

The station bending resultant is:

```text
M_z(x) = M_z,i - V_y,i x - q lever
       = 8.0 - 4.0 * 2.0 - (-2.0 * 0.5)
       = 1.0
```

| Resultant | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `N` | 0.0 | N | force |
| `M_z` | 1.0 | N-m | moment |

## Invented Stress Section Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Area, `A` | 3.0 | m^2 | area |
| Section modulus about `Y`, `Z_y` | 2.5 | m^3 | section_modulus |
| Section modulus about `Z`, `Z_z` | 2.0 | m^3 | section_modulus |
| Polar section modulus style input | 1.0 | m^3 | section_modulus |
| Pressure wall thickness | 0.5 | m | length |

## Expected Stress Components

Axial normal stress:

```text
sigma_axial = N / A
             = 0.0 / 3.0
             = 0.0
```

Bending normal stress from `M_z`:

```text
sigma_bending_z = M_z / Z_z
                = 1.0 / 2.0
                = 0.5
```

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `midspan_bending_z` | 1.0 | N-m | moment |
| `bending_normal_z` | 0.5 | Pa | stress |
| `axial_normal` | 0.0 | Pa | stress |

## Boundary

This fixture is mechanics-only stress recovery evidence. It does not compare
to allowables, classify code stress categories, apply fatigue rules, use
SIF/flexibility factors, or make professional approval or code-compliance
claims.

Tolerance policy: `TBD`.
