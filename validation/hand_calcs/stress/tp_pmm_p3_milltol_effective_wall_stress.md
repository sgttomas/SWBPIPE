# STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS

## Purpose

Invented stress recovery benchmark for the user-entered mill-tolerance
dimension slot (tranche `TP-PMM-P3-MILLTOL-001`, ruling `DEC-068` item 3).
It verifies that section properties derived from the effective wall

```text
t_eff = t_nominal - corrosion_allowance - mill_tolerance
```

feed mechanics-only stress recovery, and that adding the mill-tolerance
reduction strictly reduces the section modulus (raising recovered bending
stress) relative to the corrosion-only effective wall. Mill tolerance is a
user-entered absolute thickness dimension (length); no fractional form,
catalog value, or default is encoded. Absence of the slot means no
reduction — absence is not a default value of zero.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, or private project data.

## Invented Inputs

All values are invented non-engineering numbers in the fixture-local
`PKG09-STRESS-FIXTURE-UNITS-EXPLICIT-N-M-PA` basis.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Outside diameter, `D_o` | 0.2 | m | length |
| Nominal wall thickness, `t_nom` | 0.01 | m | length |
| Corrosion allowance, `c` | 0.002 | m | length |
| Mill tolerance, `m` | 0.00125 | m | length |
| Axial force, `F` | 5000.0 | N | force |
| Bending moment about local `Y`, `M_y` | 1000.0 | N*m | moment |
| Bending moment about local `Z`, `M_z` | -400.0 | N*m | moment |
| Torsional moment, `T` | 250.0 | N*m | moment |
| Internal pressure, `p` | 2000.0 | Pa | pressure |

## Derived Effective-Wall Section

```text
t_eff = 0.01 - 0.002 - 0.00125 = 0.00675 m
D_i   = D_o - 2 t_eff = 0.2 - 0.0135 = 0.1865 m

A = pi/4 (D_o^2 - D_i^2) = pi/4 (0.04 - 0.03478225)
  = 4.098011267067042e-3 m^2
I = pi/64 (D_o^4 - D_i^4)
  = 1.9153656442289003e-5 m^4
Z = I / (D_o/2) = 1.9153656442289e-4 m^3
J = 2 I = 3.8307312884578006e-5 m^4
r_torsion = D_o/2 = 0.1 m
r_membrane = (D_o - t_eff)/2 = 0.096625 m
```

Corrosion-only comparison (mill-tolerance slot absent):

```text
t_eff,c = 0.01 - 0.002 = 0.008 m
D_i,c   = 0.184 m
Z_c     = pi/64 (D_o^4 - D_i,c^4) / (D_o/2) = 2.2274444834258686e-4 m^3
Z < Z_c   (mill tolerance strictly reduces the section modulus)
```

## Expected Stress Components

Mechanics-only recovery (`sigma = F/A`, `sigma_b = M/Z`,
`tau = T r / J`, `sigma_hoop = p r_m / t_eff`,
`sigma_long = sigma_hoop / 2`):

```text
axial_normal          = 5000.0   / 4.098011267067042e-3 = 1.220104014886839e6  Pa
bending_normal_y      = 1000.0   / 1.9153656442289e-4   = 5.220935245513325e6  Pa
bending_normal_z      = -400.0   / 1.9153656442289e-4   = -2.0883740982053299e6 Pa
torsional_shear       = 250.0 * 0.1 / 3.8307312884578006e-5 = 6.526169056891656e5 Pa
pressure_hoop         = 2000.0 * 0.096625 / 0.00675     = 2.862962962962963e4  Pa
pressure_longitudinal = pressure_hoop / 2               = 1.4314814814814816e4 Pa
```

Cross-checks: values above were recomputed independently in decimal
arithmetic from the closed forms; the benchmark fixture recomputes the same
closed forms in code so agreement is at floating-point identity
(internal assertion epsilon 1.0e-9).

## Tolerance Policy

Per `DEC-024`/`DEC-026`, governed tolerance values remain `TBD` and the
fixture records no fixture-local `tolerance_policy` override; any future
override may only tighten the governed value. The crate-internal assertion
epsilon is 1.0e-9 (absolute), consistent with the analytic-class seed.

## Boundary

No code stress category, stress index, allowable, mill-tolerance catalog
fraction, or acceptance criterion is encoded. No lifecycle, release,
professional, certification, or code-compliance claim is made.
