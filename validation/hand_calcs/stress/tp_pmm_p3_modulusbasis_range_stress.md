# STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS

## Purpose

Invented stress benchmark for the per-load-case modulus basis (rulings
`DEC-068` item 1 and `DEC-077`). It verifies that:

1. A hot mechanics state solved with a user-entered temperature-point
   elastic modulus and thermal expansion coefficient (`E_hot`,
   `alpha_hot`) produces the exact fixed-fixed thermal axial resultant
   `F = E_hot * A * alpha_hot * dT` — the hot point's values, not the
   material base values.
2. A stress range between the hot-solved state and the cold
   (base-values) state records the modulus basis of both states
   explicitly and verbatim (`recover_stress_range_with_modulus_basis`),
   with no basis inference; and
3. a solve temperature of 400 K linearly interpolates E and alpha between
   adjacent user-entered points at 300 K and 500 K, with both source-point
   ids and `linear_temperature_interpolation` recorded as provenance.

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
| Hot-point elastic modulus, `E_hot` | 1.8e11 | Pa | stress |
| Hot-point expansion coefficient, `alpha_hot` | 1.3e-5 | 1/K | thermal_expansion_coefficient |
| Temperature change, `dT` | 10.0 | K | temperature_interval |
| Pipe metal area, `A` | 0.004 | m^2 | area |
| Hot basis label | `temperature_point:hot` | - | - |
| Cold basis label | `material_base_values` | - | - |
| Lower interpolation temperature | 300.0 | K | temperature |
| Upper interpolation temperature | 500.0 | K | temperature |
| Declared solve temperature | 400.0 | K | temperature |
| Lower/upper elastic modulus | 2.0e11 / 1.8e11 | Pa | stress |
| Lower/upper expansion coefficient | 1.2e-5 / 1.4e-5 | 1/K | thermal_expansion_coefficient |

The cold state is the unloaded base-values state (zero resultants).

## Hot-Solve Resultant And Stress

Fixed-fixed uniform-temperature axial identity:

```text
F_hot = E_hot * A * alpha_hot * dT
      = 1.8e11 * 0.004 * 1.3e-5 * 10.0
      = 93600.0 N

sigma_axial,hot = F_hot / A = E_hot * alpha_hot * dT
                = 1.8e11 * 1.3e-5 * 10.0
                = 2.34e7 Pa
```

## Range With Recorded Basis

```text
sigma_axial,cold = 0.0 Pa

axial_normal_range = sigma_axial,hot - sigma_axial,cold = 2.34e7 Pa

modulus_basis record:
  first_state_basis_ref  = temperature_point:hot
  second_state_basis_ref = material_base_values
```

The record is a verbatim declaration of the solved bases. Exact selection
remains available: a load case naming a basis with no stored
temperature-indexed value is a blocking diagnostic upstream.

## DEC-077 Linear Interpolation Witness

For adjacent source points `(T1, y1)` and `(T2, y2)`, the declared method is:

```text
r = (Tsolve - T1) / (T2 - T1)
y(Tsolve) = y1 + r * (y2 - y1)

r = (400 - 300) / (500 - 300) = 0.5
E(400 K) = 2.0e11 + 0.5 * (1.8e11 - 2.0e11) = 1.9e11 Pa
alpha(400 K) = 1.2e-5 + 0.5 * (1.4e-5 - 1.2e-5) = 1.3e-5 1/K
```

The source ids `temperature-point:cold` and `temperature-point:hot` apply
to both derived values. The method string is
`linear_temperature_interpolation`. The software blocks requests at or
beyond 300 K and 500 K; this witness does not authorize extrapolation.

Cross-checks: the arithmetic above was recomputed independently in decimal
arithmetic from the closed forms; the benchmark fixture recomputes the same
closed forms in code so agreement is at floating-point identity
(internal assertion epsilon 1.0e-9).

## Tolerance Policy

Per `DEC-024`/`DEC-026`, governed tolerance values remain `TBD` and the
fixture records no fixture-local `tolerance_policy` override; any future
override may only tighten the governed value.

## Boundary

All source property values and the solve temperature are user-entered; no
material catalog, temperature curve, code table, extrapolation, or default is encoded. No
lifecycle, release, professional, certification, or code-compliance claim
is made.
