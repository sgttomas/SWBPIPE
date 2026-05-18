# STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS

## Purpose

Invented stress recovery benchmark linking straight-pipe thermal axial-effect
resultant recovery to mechanics-only stress recovery. It verifies that an
explicit axial-effect resultant can feed end and station stress recovery
without pressure-basis inputs.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, or private project data.

## Invented Inputs

The pipe is aligned to global `X`, with local `Y` matching global `Y`. The
fixture uses zero displacement evidence so the recovered axial resultants come
from the explicit axial-effect path only. The axial effect is treated as an
invented thermal axial-effect force input; it is not derived from material,
thermal-expansion, or temperature defaults.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Length, `L` | 6.0 | m | length |
| Elastic modulus, `E` | 1200.0 | Pa | stress |
| Shear modulus, `G` | 500.0 | Pa | stress |
| Pipe area used by straight-pipe recovery | 6.0 | m^2 | area |
| Second moment about local `Y`, `I_y` | 2.0 | m^4 | second_moment_area |
| Second moment about local `Z`, `I_z` | 2.0 | m^4 | second_moment_area |
| Torsion constant, `J` | 1.0 | m^4 | second_moment_area |
| Thermal axial-effect force, `F_t` | 240.0 | N | force |
| Element displacement vector | all zeros | mixed | TBD |
| Global model displacement vector | all zeros | mixed | TBD |
| Station fraction | 0.5 | ratio | dimensionless |
| Station sweep fractions | 0.0, 0.5, 1.0 | ratio | dimensionless |

## Recovered Resultants

For this fixture, the straight-pipe axial-effect path supplies equivalent
local axial loads of `-F_t` at the I-end axial DOF and `+F_t` at the J-end
axial DOF. With zero displacement recovery, the corrected local force vector is
the negative of those equivalent loads.

```text
F_x,i = 240.0
F_x,j = -240.0
```

The station resultant path starts from the recovered I-end resultants. With no
transverse or torsional loads, the recovered station axial force is constant:

```text
N_station = 240.0
```

## Invented Stress Section Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Area, `A` | 6.0 | m^2 | area |
| Section modulus about `Y`, `Z_y` | 3.0 | m^3 | section_modulus |
| Section modulus about `Z`, `Z_z` | 3.0 | m^3 | section_modulus |
| Polar section modulus style input | 1.0 | m^3 | section_modulus |
| Torsion radius | 0.5 | m | length |

Pressure is not supplied for this benchmark.

## Expected Stress Components

End-J axial normal stress:

```text
sigma_end_j = F_x,j / A
            = -240.0 / 6.0
            = -40.0
```

Midspan station axial normal stress:

```text
sigma_station = N_station / A
              = 240.0 / 6.0
              = 40.0
```

The station sweep uses the same recovered axial force at fractions `0.0`,
`0.5`, and `1.0`, so each station sweep stress has `axial_normal = 40.0 Pa`.

## Boundary

This fixture is mechanics-only stress recovery evidence. It does not compare
to acceptance thresholds, derive material thermal expansion behavior, use
pressure-basis stress recovery, or include private/proprietary source values.

Tolerance policy: `TBD`.
