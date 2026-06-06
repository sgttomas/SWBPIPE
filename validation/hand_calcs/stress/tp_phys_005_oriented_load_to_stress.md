# STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS

## Purpose

Invented stress recovery benchmark linking an oriented straight-pipe global
model displacement state to station resultant recovery and code-neutral stress
recovery. It verifies that explicit local orientation is preserved when the
pipe is not aligned to global `X`.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, allowables, SIF/flexibility factors, or fatigue criteria.

## Oriented Pipe Inputs

The invented pipe starts at node `0` `(0.0, 0.0, 0.0)` and ends at node `1`
`(0.0, 4.0, 0.0)`. The explicit `y_reference` is `(1.0, 0.0, 0.0)`, so the
fixture local axes are:

| Local axis | Global direction |
|---|---|
| `x` | global `+Y` |
| `y` | global `+X` |
| `z` | global `-Z` |

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Length, `L` | 4.0 | m | length |
| Elastic modulus, `E` | 1000.0 | Pa | stress |
| Shear modulus, `G` | 400.0 | Pa | stress |
| Area, `A` | 3.0 | m^2 | area |
| Second moment about local `Y`, `I_y` | 1.5 | m^4 | second_moment_area |
| Second moment about local `Z`, `I_z` | 2.0 | m^4 | second_moment_area |
| Torsion constant, `J` | 1.0 | m^4 | second_moment_area |
| Uniform local `Y` load, `q` | -2.0 | N/m | force_per_length |
| Point local `Y` force, `P` | -4.0 | N | force |
| Point station, `a/L` | 0.5 | ratio | dimensionless |

## Global Displacement Evidence

The oriented mechanics evidence supplies global model displacements at node
`1`:

| DOF | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `UX` | -0.04533333333333334 | m | displacement |
| `RZ` | 0.014666666666666668 | rad | rotation |

Because local `y` is global `+X` and local `z` is global `-Z`, these correspond
to the same local transverse displacement and local bending rotation used by
the aligned TP-PHYS-004 mechanics fixture:

```text
v_local_y = -0.04533333333333334
theta_local_z = -0.014666666666666668
```

## Midspan Station Resultants

The station recovery uses `recover_station_resultants_from_global_model` with
the global model displacement vector, station `0.5`, local uniform load
`q = -2.0`, and local point force `P = -4.0` at station `0.5`.

The local station-resultant recurrence is unchanged by the global orientation
because the pipe transform maps the global displacement evidence into local
element coordinates before recovering resultants.

```text
V_y,i = 12.0
M_z,i = 24.0
x = 2.0
a = 2.0
```

The point force is included at the station by the deterministic inclusive
station rule.

```text
V_y(x) = V_y,i + q x + P
       = 12.0 + (-2.0 * 2.0) + -4.0
       = 4.0
```

```text
M_z(x) = M_z,i - V_y,i x - q x^2 / 2 - P (x - a)
       = 24.0 - 12.0 * 2.0 - (-2.0 * 2.0^2 / 2) - (-4.0 * 0.0)
       = 4.0
```

| Resultant | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `N` | 0.0 | N | force |
| `M_z` | 4.0 | N-m | moment |

## Invented Stress Section Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Area, `A` | 3.0 | m^2 | area |
| Section modulus about `Y`, `Z_y` | 2.5 | m^3 | section_modulus |
| Section modulus about `Z`, `Z_z` | 2.0 | m^3 | section_modulus |
| Torsion constant, `J` | 1.0 | m^4 | second_moment_area |
| Torsion radius, `r` | 0.5 | m | length |

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
                = 4.0 / 2.0
                = 2.0
```

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `midspan_bending_z` | 4.0 | N-m | moment |
| `bending_normal_z` | 2.0 | Pa | stress |
| `axial_normal` | 0.0 | Pa | stress |

## Boundary

This fixture is mechanics-only stress recovery evidence. It does not compare
to allowables, classify code stress categories, apply fatigue rules, use
SIF/flexibility factors, or make professional approval or code-compliance
claims.

Tolerance policy: `TBD`.
