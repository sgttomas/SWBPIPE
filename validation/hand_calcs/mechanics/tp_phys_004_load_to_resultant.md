# MECH-TP-PHYS-004-LOAD-TO-RESULTANT

## Purpose

Invented mechanics benchmark for load-to-resultant integration on one straight
pipe: distributed and point load recovery into equivalent nodal loads,
deterministic load-case assembly, cantilever displacement recovery, and
station-level shear and bending resultant recovery.

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
| `L` | 4.0 | m | length |
| `E` | 1000.0 | Pa | stress |
| `G` | 400.0 | Pa | stress |
| `A` | 3.0 | m^2 | area |
| `I_y` | 1.5 | m^4 | second_moment_area |
| `I_z` | 2.0 | m^4 | second_moment_area |
| `J` | 1.0 | m^4 | second_moment_area |
| Uniform local/global `Y` load, `q` | -2.0 | N/m | TBD |
| Point local/global `Y` force, `P` | -4.0 | N | force |
| Point station, `a/L` | 0.5 | ratio | dimensionless |
| Node `0` support | fixed | count | dimensionless |
| Node `1` support | free | count | dimensionless |

## Equivalent Nodal Loads

For the full-span uniform load:

```text
F_y,i = q L / 2
      = -2.0 * 4.0 / 2
      = -4.0

F_y,j = -4.0

M_z,i = q L^2 / 12
      = -2.0 * 16.0 / 12
      = -2.6666666666666665

M_z,j = -M_z,i
      = 2.6666666666666665
```

For the midspan point force with `r = a/L = 0.5`, the Hermite shape-function
weights are:

```text
h_i = 1 - 3 r^2 + 2 r^3 = 0.5
theta_i = L (r - 2 r^2 + r^3) = 0.5
h_j = 3 r^2 - 2 r^3 = 0.5
theta_j = L (-r^2 + r^3) = -0.5
```

Therefore:

```text
F_y,i = P h_i = -4.0 * 0.5 = -2.0
M_z,i = P theta_i = -4.0 * 0.5 = -2.0
F_y,j = P h_j = -2.0
M_z,j = P theta_j = -4.0 * -0.5 = 2.0
```

Combined assembled solver loads:

| DOF | Value | Unit |
|---|---:|---|
| Node `0`, `Uy` | -6.0 | N |
| Node `0`, `Rz` | -4.666666666666667 | N-m |
| Node `1`, `Uy` | -6.0 | N |
| Node `1`, `Rz` | 4.666666666666667 | N-m |

## Tip Displacement And Rotation

Use `E I_z = 1000.0 * 2.0 = 2000.0`.

Uniform-load tip displacement:

```text
v_q(L) = q L^4 / (8 E I_z)
       = -2.0 * 4.0^4 / (8 * 2000.0)
       = -0.032
```

Midspan point-load tip displacement:

```text
v_P(L) = P a^2 (3 L - a) / (6 E I_z)
       = -4.0 * 2.0^2 * (12.0 - 2.0) / (6 * 2000.0)
       = -0.013333333333333334
```

Combined:

```text
v(L) = -0.032 + -0.013333333333333334
     = -0.04533333333333334
```

Uniform-load free-end rotation:

```text
theta_q(L) = q L^3 / (6 E I_z)
           = -2.0 * 4.0^3 / (6 * 2000.0)
           = -0.010666666666666666
```

Midspan point-load free-end rotation:

```text
theta_P(L) = P a^2 / (2 E I_z)
           = -4.0 * 2.0^2 / (2 * 2000.0)
           = -0.004
```

Combined:

```text
theta(L) = -0.010666666666666666 + -0.004
         = -0.014666666666666668
```

## Midspan Station Resultants

The fixture uses the repository straight-pipe signed-resultant convention:
loaded I-end resultants are recovered from `K u - f_equivalent`, then load
effects are accumulated to the requested station. For this cantilever:

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

At the free end the same recurrence gives zero shear and zero bending moment,
which checks the cantilever boundary.

## Boundary

This note is a code-neutral mechanics derivation for an invented fixture. It
does not define release tolerances, code stress categories, allowables,
fatigue criteria, SIF/flexibility factors, professional approval, or
code-compliance claims.

Tolerance policy: `TBD`.
