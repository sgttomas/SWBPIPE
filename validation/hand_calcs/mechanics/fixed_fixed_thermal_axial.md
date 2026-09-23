# MECH-FIXED-FIXED-THERMAL-AXIAL

## Purpose

Thermal-growth benchmark for a fully restrained prismatic member.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

| Symbol | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `E` | 2000.0 | Pa | stress |
| `A` | 3.0 | m^2 | area |
| `alpha` | 0.000012 | 1/K | thermal_expansion_coefficient |
| `DeltaT` | 75.0 | K | temperature_interval |

## Expected Values

Free thermal strain:

```text
epsilon = alpha DeltaT
        = 0.000012 * 75.0
        = 0.0009
```

Restrained axial force magnitude:

```text
F = E A alpha DeltaT
  = 2000.0 * 3.0 * 0.000012 * 75.0
  = 5.4
```

The benchmark computes the restrained force through the product's
straight-pipe axial-effect preparation path (the path used by
`MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS`) from the same invented
inputs and compares it with this value. The free thermal strain is checked as
reference arithmetic only. With both ends fully restrained no free degree of
freedom remains, so no global stiffness solve is involved.

Tolerance policy: `TBD`.
