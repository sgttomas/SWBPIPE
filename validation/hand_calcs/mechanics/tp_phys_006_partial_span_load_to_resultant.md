# MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT

## Purpose

Invented mechanics benchmark for a partial-span straight-pipe distributed load:
the fixture checks equivalent nodal load recovery, fixed-free displacement
solve, and station-level shear and bending recovery at midspan.

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
| Span start, `a/L` | 0.25 | ratio | dimensionless |
| Span end, `b/L` | 0.75 | ratio | dimensionless |
| Resultant station, `x/L` | 0.5 | ratio | dimensionless |
| Node `0` support | fixed | count | dimensionless |
| Node `1` support | free | count | dimensionless |

## Equivalent Nodal Loads

The transverse Hermite interpolation functions use nondimensional position
`r = x / L`:

```text
N_i = 1 - 3 r^2 + 2 r^3
Theta_i = L (r - 2 r^2 + r^3)
N_j = 3 r^2 - 2 r^3
Theta_j = L (-r^2 + r^3)
```

For a uniform load over `a <= r <= b`, integrate those functions over the
loaded interval:

```text
F_y,i = q L integral_a^b N_i dr
      = -2.0 * 4.0 * 0.25
      = -2.0

M_z,i = q L^2 integral_a^b (r - 2 r^2 + r^3) dr
      = -2.0 * 16.0 * 11 / 192
      = -11 / 6

F_y,j = q L integral_a^b N_j dr
      = -2.0

M_z,j = q L^2 integral_a^b (-r^2 + r^3) dr
      = 11 / 6
```

Expected equivalent nodal loads for the load alone:

| DOF | Value | Unit |
|---|---:|---|
| Node `0`, `Uy` | -2.0 | N |
| Node `0`, `Rz` | -1.8333333333333333 | N-m |
| Node `1`, `Uy` | -2.0 | N |
| Node `1`, `Rz` | 1.8333333333333333 | N-m |

## Fixed-Free Reduced Solve

Use `E I_z = 1000.0 * 2.0 = 2000.0`.

With node `0` fixed, the active reduced bending matrix for node `1` `Uy` and
`Rz` is:

```text
[ 12 E I_z / L^3    -6 E I_z / L^2 ]
[ -6 E I_z / L^2     4 E I_z / L   ]

=

[ 375   -750 ]
[ -750  2000 ]
```

The free reduced load vector is:

```text
F_free = [ -2.0, 11 / 6 ]
```

Solving `K_free u_free = F_free` gives the fixture's current reduced-system
benchmark values:

```text
u_y,1 = -7 / 500
      = -0.014

theta_z,1 = -13 / 3000
          = -0.004333333333333333
```

## Midspan Station Resultants

The fixture uses the repository straight-pipe signed-resultant convention:
loaded I-end resultants are recovered from `K u - f_equivalent`, then active
partial-span load effects are accumulated to the requested station.

For the fixed-free case, the I-end resultants from global equilibrium are:

```text
V_y,i = 4.0
M_z,i = 8.0
```

At midspan, `x = 2.0`, only the loaded interval from `1.0` to `2.0` is active:

```text
active_length = 2.0 - 1.0 = 1.0
lever_integral = integral_1^2 (2.0 - s) ds = 0.5
```

```text
V_y(x) = V_y,i + q active_length
       = 4.0 + (-2.0 * 1.0)
       = 2.0
```

```text
M_z(x) = M_z,i - V_y,i x - q lever_integral
       = 8.0 - 4.0 * 2.0 - (-2.0 * 0.5)
       = 1.0
```

At the free end, the same recurrence includes the full loaded interval and
returns zero shear and zero bending moment.

## Boundary

This note is a code-neutral mechanics derivation for an invented fixture. It
does not define release tolerances, code stress categories, allowables,
fatigue criteria, SIF/flexibility factors, professional approval, or
code-compliance claims.

Tolerance policy: `TBD`.
