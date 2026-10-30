# MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION

## Purpose

Integrated invented mechanics benchmark for one deterministic linear static
path: nodal load preparation, uniform element load lumping, linear support
application, dense frame solve, straight-pipe local force recovery, and
diagnostic mapping for invalid inputs.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

The straight member starts at node `0` `(0.0, 0.0, 0.0)` and ends at node `1`
`(4.0, 0.0, 0.0)`.

Section values are invented:

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `E` | 1500.0 | Pa | stress |
| `G` | 600.0 | Pa | stress |
| `A` | 2.0 | m^2 | area |
| `I_y` | 1.8 | m^4 | second_moment_area |
| `I_z` | 2.2 | m^4 | second_moment_area |
| `J` | 0.9 | m^4 | second_moment_area |
| `L` | 4.0 | m | length |

Loads and supports:

| Item | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Node `1`, global `X` nodal force | 12.0 | N | force |
| Element `0`, global `Y` uniform load | -2.0 | N/m | force_per_length |
| Node `0` anchor | 6 restrained DOFs | count | dimensionless |
| Node `1`, `Uy` spring stiffness | 40.0 | N/m | linear_stiffness |
| Node `1`, `Uz` imposed displacement | -0.01 | m | length |

## Expected Values

Uniform load lumping over the explicit span:

```text
F_i = w L / 2
    = -2.0 * 4.0 / 2
    = -4.0

F_j = w L / 2
    = -4.0
```

Axial displacement from the nodal force:

```text
k_x = E A / L
    = 1500.0 * 2.0 / 4.0
    = 750.0

u_x,j = P_x / k_x
      = 12.0 / 750.0
      = 0.016
```

Recovered local axial force at node `j`:

```text
F_x,j = k_x u_x,j
      = 750.0 * 0.016
      = 12.0
```

Global `Y` displacement and local `Y` shear at node `j`. Node `0` is anchored.
Node `1` is free in `Uy` and `Rz`, carries no applied moment (the lumped load
has no moment terms), and is held by the `Uy` spring. The `Uz` imposed
displacement and the axial force act in planes that do not couple to `Uy`/`Rz`
for a straight member on global `X`. Eliminating the free end rotation leaves
the cantilever tip stiffness in parallel with the spring:

```text
k_b = 3 E I_z / L^3
    = 3 * 1500.0 * 2.2 / 4.0^3
    = 154.6875

u_y,j = F_j / (k_b + k_s)
      = -4.0 / (154.6875 + 40.0)
      = -4.0 / 194.6875
      = -0.020545746388443017

V_y,j = k_b u_y,j
      = 154.6875 * -0.020545746388443017
      = -3.178170144462279
```

The spring carries the remainder, `k_s u_y,j = -0.8218298555377207`, and the
two together equal the lumped node `1` load of `-4.0`.

Diagnostic checks are negative-path mappings only:

- missing spring stiffness maps to a blocking restraint diagnostic;
- missing load target maps to a blocking model-topology diagnostic.

Tolerance policy: `TBD`.
