# MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS

## Purpose

Invented mechanics benchmark for the straight-pipe axial-effect path. The
fixture combines fixed-fixed thermal restraint with closed-end pressure thrust,
then checks equivalent nodal loads and axial-resultant recovery with zero
displacement.

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
| Node `0` support | fixed | count | dimensionless |
| Node `1` support | fixed | count | dimensionless |

The fixture-local unit basis is explicit and no conversions are performed.

## Axial Effects

Thermal axial restraint force:

```text
F_thermal = E A alpha DeltaT
          = 1000.0 * 4.0 * 0.00001 * 75.0
          = 3.0 N
```

Closed-end pressure thrust:

```text
F_pressure = p A_internal
           = 90.0 * 0.1
           = 9.0 N
```

Total axial-effect force:

```text
F_total = F_thermal + F_pressure
        = 3.0 + 9.0
        = 12.0 N
```

## Equivalent Nodal Loads

The straight-pipe axial-effect path applies the local axial load pair:

```text
Node i, Ux = -F_total = -12.0 N
Node j, Ux =  F_total =  12.0 N
```

The member is aligned to global `X`, so local and global axial load components
are identical for this fixture.

## Fixed-Fixed Recovery

For fixed-fixed recovery, the element displacement vector is zero:

```text
u_i = u_j = 0.0
```

The recovered fixed-end local force vector is:

```text
K u - f_equivalent = 0 - [-12.0, 12.0]
```

Expected local axial resultants:

```text
N_i =  12.0 N
N_j = -12.0 N
```

## Station Resultants

With no transverse load and zero displacement, axial force remains constant
along the member:

```text
N(x/L = 0.5) = 12.0 N
V_y = 0.0 N
M_z = 0.0 N-m
```

The station sweep is intentionally requested out of geometric order:

| Requested index | Station fraction | Axial force | `V_y` | `M_z` |
|---:|---:|---:|---:|---:|
| 0 | 1.0 | 12.0 | 0.0 | 0.0 |
| 1 | 0.0 | 12.0 | 0.0 | 0.0 |
| 2 | 0.5 | 12.0 | 0.0 | 0.0 |

## Boundary

This note records a code-neutral mechanics derivation for an invented fixture.
It does not define release tolerances, rule checks, allowables, stress
categories, SIF/flexibility factors, or project-specific decisions.

Tolerance policy: `TBD`.
