# MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD

## Purpose

Superposition witness for the ideal constant-effort spring-hanger support
consumed by the assembled solve (DEC-049 deeper assembled-solve consumption
tranche). The ideal constant-effort element contributes a constant nodal
force of the user-entered magnitude at its node, along the positive axis of
the single declared translational restraint DOF, in every solved load case,
with zero stiffness contribution and no solve restraint row.

Sign convention (verbatim, as recorded in the
`constant_effort_support_applied_load` result rows): positive value is the
user-entered constant support force applied along the positive axis of the
single declared translational restraint DOF in every solved load case; the
ideal constant-effort element contributes zero stiffness and adds no solve
restraint row; no gravity coupling, catalog/default value, or inferred
direction is supplied.

## Provenance

- Source: OpenPipeStress original mechanics benchmark (invented,
  project-original).
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or
  proprietary data. The constant-load magnitude is a user-entered value; no
  catalog sizing, protected standards value, or hidden default is encoded.

## Invented Inputs

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `L` | 10.0 | m | length |
| `E` | 1200.0 | Pa | stress |
| `I_z` | 4.0 | m^4 | second_moment_area |
| `W` (downward tip load) | 6.0 | N | force |
| `F` (user-entered constant support load, acting DOF `UY`) | 9.0 | N | force |

Two-node cantilever along global X: node 0 restrained in all six degrees of
freedom, node 1 free. The primitive tip load acts in `-Y`; the
constant-effort support acts on the positive `UY` axis at node 1.

## Derivation

For a linear cantilever, superposition holds: the solve with the
constant-effort support equals the solve without it plus the classical
cantilever solution for the equivalent tip point force `F`.

Tip deflection under a tip point force `P` (Euler-Bernoulli cantilever):

```text
delta_y(P) = P L^3 / (3 E I_z)
```

Without the constant-effort support (tip load `-W` only):

```text
delta_y_without = -W L^3 / (3 E I_z)
                = -6.0 * 10.0^3 / (3 * 1200.0 * 4.0)
                = -0.4166666666666667
```

With the constant-effort support (net tip force `F - W = +3.0 N`):

```text
delta_y_with = (F - W) L^3 / (3 E I_z)
             = 3.0 * 10.0^3 / (3 * 1200.0 * 4.0)
             = 0.2083333333333333
```

Superposition identity (classical solution for the equivalent point force):

```text
delta_y_with - delta_y_without = F L^3 / (3 E I_z)
                               = 9.0 * 10.0^3 / (3 * 1200.0 * 4.0)
                               = 0.625
```

Fixed-end reactions with the constant-effort support present, from
`r = K d - f` at the restrained node (net applied tip force `+3.0 N` on the
`+Y` axis):

```text
r_y = -(F - W)     = -3.0   N
m_z = -(F - W) L   = -30.0  N-m
```

## Expected Values

| Name | Value | Unit |
|---|---:|---|
| `tip_displacement_y_without_support` | -0.4166666666666667 | m |
| `tip_displacement_y_with_support` | 0.2083333333333333 | m |
| `superposition_delta_y` | 0.625 | m |
| `fixed_end_reaction_y` | -3.0 | N |
| `fixed_end_moment_z` | -30.0 | N-m |

Tolerance policy: `TBD`.
