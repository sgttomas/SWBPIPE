# STRESS-TP-PHYS-004-LOAD-TO-RESULTANT

## Purpose

Invented stress recovery benchmark linking the TP-PHYS-004 load-to-resultant
mechanics fixture to station stress recovery. It verifies that recovered
station resultants can feed code-neutral axial, bending, and torsional stress
components.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, allowables, SIF/flexibility factors, or fatigue criteria.

## Station Resultants

The mechanics hand calculation in
`validation/hand_calcs/mechanics/tp_phys_004_load_to_resultant.md` gives the
midspan station resultants:

| Resultant | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `N` | 0.0 | N | force |
| `V_y` | 4.0 | N | force |
| `V_z` | 0.0 | N | force |
| `T` | 0.0 | N-m | moment |
| `M_y` | 0.0 | N-m | moment |
| `M_z` | 4.0 | N-m | moment |

The current stress module uses axial force, bending moments, and torsional
moment for these recovered components. Transverse shear stress recovery is not
part of this fixture.

## Invented Section Inputs

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

Bending normal stress from `M_y`:

```text
sigma_bending_y = M_y / Z_y
                = 0.0 / 2.5
                = 0.0
```

Torsional shear stress:

```text
tau_torsion = T r / J
             = 0.0 * 0.5 / 1.0
             = 0.0
```

## Boundary

This fixture is mechanics-only stress recovery. It does not compare to
allowables, classify code stress categories, apply fatigue rules, use
SIF/flexibility factors, or make professional approval or code-compliance
claims.

Tolerance policy: `TBD`.
