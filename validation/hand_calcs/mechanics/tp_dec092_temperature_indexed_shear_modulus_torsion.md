# MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION

## Purpose and boundary

This is an independent hand calculation for the D-45 Option B / `DEC-092`
temperature-indexed shear-modulus behavior. A uniform hollow straight pipe is
fixed at one end and subjected only to a torque about its longitudinal axis at
the other. The note supplies exact-ID and linearly interpolated `G` targets
that a later product-physics comparison can consume without copying the
product's material-selection implementation.

All values are invented public fixture inputs. No material catalog, protected
table, piping-code value, material curve, extrapolation, or default is used.
This is development verification evidence only; it creates no lifecycle,
release, professional, certification, sealing, authentication, or
code-compliance claim.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: derived from elementary St. Venant torsion and
  linear interpolation, not copied from protected standards, commercial
  software examples, proprietary data, or private project data.

## Invented inputs

The fixture-local basis is `PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K`.

| Symbol | Description | Value | Unit |
|---|---|---:|---|
| `L` | pipe length | 4.0 | m |
| `D_o` | outside diameter | 0.12 | m |
| `D_i` | inside diameter | 0.10 | m |
| `T` | applied tip torque | 12,000 | N-m |
| `G_base` | explicit material base shear modulus | 80.0e9 | Pa |
| `G_exact` | explicit shear modulus at selected point `TP-EXACT` | 50.0e9 | Pa |
| `T_1` | lower bracket temperature, point `TP-LOW` | 300.0 | K |
| `G_1` | explicit shear modulus at `TP-LOW` | 60.0e9 | Pa |
| `T_2` | upper bracket temperature, point `TP-HIGH` | 500.0 | K |
| `G_2` | explicit shear modulus at `TP-HIGH` | 40.0e9 | Pa |
| `T_s` | selected interpolation temperature | 425.0 | K |

Assumptions: uniform circular annulus; small-rotation linear elasticity; pure
St. Venant torsion; the fixed end restrains all six degrees of freedom; the
only applied load is `T`; the selected point values are explicit user inputs.

## Section property

For a hollow circular section,

```text
J = (pi / 32) (D_o^4 - D_i^4)
  = (pi / 32) [(0.12 m)^4 - (0.10 m)^4]
  = 1.0540043352793751e-5 m^4
```

## Shear-modulus selections

Exact-ID selection consumes the explicit value at `TP-EXACT`:

```text
G_exact = 50.0e9 Pa
```

For the temperature selection, the normalized coordinate and adjacent linear
interpolation are

```text
f = (T_s - T_1) / (T_2 - T_1)
  = (425 - 300) / (500 - 300)
  = 0.625

G_interp = G_1 + f (G_2 - G_1)
         = 60.0e9 + 0.625 (40.0e9 - 60.0e9)
         = 47.5e9 Pa
```

Both bracketing values are explicit. No base value enters either selected
basis calculation.

## Pure-torsion derivation and results

For a prismatic circular shaft in pure torsion,

```text
torsional rigidity = G J                         [Pa m^4 = N m^2]
twist per length   = T / (G J)                  [(N m)/(N m^2) = 1/m]
theta              = T L / (G J)                [rad]
```

Exact-ID target:

```text
theta_exact = (12,000 N-m)(4.0 m)
              / [(50.0e9 Pa)(1.0540043352793751e-5 m^4)]
            = 9.108121929551094e-2 rad
```

Interpolated-G target:

```text
theta_interp = (12,000 N-m)(4.0 m)
               / [(47.5e9 Pa)(1.0540043352793751e-5 m^4)]
             = 9.587496767948521e-2 rad
```

Deliberate base-G fallback mutation:

```text
theta_base = (12,000 N-m)(4.0 m)
             / [(80.0e9 Pa)(1.0540043352793751e-5 m^4)]
           = 5.6925762059694338e-2 rad

|theta_base - theta_exact| / theta_exact   = 0.37500   (37.5%)
|theta_base - theta_interp| / theta_interp = 0.40625   (40.625%)
```

Thus a selected-basis path that silently consumes `G_base` cannot match
either selected target accidentally. Holding geometry and torque fixed also
gives the inverse-modulus identities
`theta_exact / theta_base = G_base / G_exact = 1.6` and
`theta_interp / theta_base = G_base / G_interp = 32/19`.

## Benchmark comparison posture

The mechanics crate independently instantiates the annular section in the
frame kernel, applies only the node-tip torsional moment, and compares its
rotation with the values above. The comparison reuses the existing
`DEC-024`/`DEC-026` analytic-class `1.0e-9` relative tier. It selects no new
tolerance: fixture `tolerance_policy` remains `None`, while final tolerance
policy, release thresholds, CI gate policy, publication scope, external
validation claims, and professional reliance remain `TBD`.

## Expected values for later product comparison

| Quantity | Expected value | Unit |
|---|---:|---|
| `J` | 1.0540043352793751e-5 | m^4 |
| exact-ID `G` | 5.0000000000000000e10 | Pa |
| interpolated `G` | 4.7500000000000000e10 | Pa |
| exact-ID tip rotation | 9.108121929551094e-2 | rad |
| interpolated-G tip rotation | 9.587496767948521e-2 | rad |
| base-G fallback tip rotation (must not be selected) | 5.6925762059694338e-2 | rad |
