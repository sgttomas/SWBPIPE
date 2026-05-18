# Integrated Straight Pipe Resultants Stress Fixture

Fixture ID: `STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL`

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- License basis: project-original-public-content.
- Contributor: OpenPipeStress agentic development workflow.
- Source note: generated from elementary open mechanics; not copied from
  protected standards, code formulas, commercial software examples, or
  proprietary data.

## Purpose

This fixture links a straight-pipe element end-resultant recovery to
code-neutral stress recovery. It verifies that direct local end-resultants from
the solver boundary can feed axial and torsional mechanics stress components
without any code stress category, allowable comparison, fatigue rule,
SIF/flexibility factor, or professional conclusion.

## Inputs

The invented straight pipe starts at node `0` `(0.0, 0.0, 0.0)` and ends at node
`1` `(5.0, 0.0, 0.0)`.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Elastic modulus, `E` | 2000.0 | Pa | stress |
| Shear modulus, `G` | 800.0 | Pa | stress |
| Area, `A` | 4.0 | m^2 | area |
| Torsion constant, `J` | 2.0 | m^4 | second_moment_area |
| Torsion radius, `r` | 0.5 | m | length |
| Length, `L` | 5.0 | m | length |
| Node `1` axial displacement, `u_x` | 0.01 | m | length |
| Node `1` torsional rotation, `theta_x` | 0.02 | rad | rotation |

## Expected End Resultants

Axial stiffness and direct end-j axial force:

```text
k_x = E A / L
    = 2000.0 * 4.0 / 5.0
    = 1600.0

F_x,j = k_x u_x
      = 1600.0 * 0.01
      = 16.0
```

Torsional stiffness and direct end-j torsional moment:

```text
k_t = G J / L
    = 800.0 * 2.0 / 5.0
    = 320.0

M_x,j = k_t theta_x
      = 320.0 * 0.02
      = 6.4
```

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `end_j_axial_force` | 16.0 | N | force |
| `end_j_torsional_moment` | 6.4 | N-m | moment |

## Expected Stress Components

Axial normal stress:

```text
sigma_axial = F_x,j / A
             = 16.0 / 4.0
             = 4.0
```

Torsional shear stress:

```text
tau_torsion = M_x,j r / J
             = 6.4 * 0.5 / 2.0
             = 1.6
```

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `axial_normal` | 4.0 | Pa | stress |
| `torsional_shear` | 1.6 | Pa | stress |

## Boundary

The end-resultants are direct local force-vector components for the requested
pipe end. This fixture does not reverse signs, classify code stress categories,
compare allowables, apply fatigue rules, use SIF/flexibility factors, or make
professional approval or code-compliance claims.

Tolerance policy: `TBD`.
