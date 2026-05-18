# MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD

## Purpose

Invented mechanics benchmark proving that a canonical analytical solver-model
payload can be parsed without hidden mechanics defaults and consumed by the
current straight-pipe and user-load solver APIs.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or
  proprietary data.

## Canonical Payload Inputs

The payload fixture is
`validation/benchmarks/mechanics/fixtures/tp_phys_014_canonical_analytical_payload.json`.
It is an invented `analytical_solver_model` payload with one straight pipe,
one anchor, and one load case.

The member is a two-node straight pipe aligned to global `X`; the governed
straight-pipe `y_reference` is global `Y`.

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `L` | 4.0 | m | length |
| `E` | 1000.0 | Pa | stress |
| `G` | 400.0 | Pa | stress |
| `A` | 3.0 | m^2 | area |
| `I_y` | 1.5 | m^4 | second_moment_area |
| `I_z` | 2.0 | m^4 | second_moment_area |
| `J` | 1.0 | m^4 | second_moment_area |
| `y_reference` | `[0.0, 1.0, 0.0]` | ratio | dimensionless |
| Uniform `element_uniform_distributed_force`, `q` | -2.0 | N/m | force_per_length |
| Uniform span | 0.0 to 1.0 | ratio | dimensionless |
| `element_point_force`, `P` | -4.0 | N | force |
| Point station, `a/L` | 0.5 | ratio | dimensionless |
| Node `N-1` support | anchor | count | dimensionless |
| Node `N-2` support | free | count | dimensionless |

The fixture-local unit basis is explicit and no conversions are performed.

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

The point force is included at the station by the deterministic inclusive
station rule.

```text
V_y,i = 12.0
M_z,i = 24.0
x = 2.0
a = 2.0
```

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

## Boundary

This note records a code-neutral mechanics derivation for an invented fixture.
It does not define release tolerances, rule checks, allowables, stress
categories, SIF/flexibility factors, or project-specific decisions.

Tolerance policy: `TBD`.
