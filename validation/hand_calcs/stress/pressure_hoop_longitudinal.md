# STRESS-PRESSURE-HOOP-LONGITUDINAL-ORIGINAL

## Purpose

Invented stress recovery benchmark for pressure behavior (DEL-09-02 RQ-001,
pressure fixture slot). One straight elastic annulus carries internal
pressure. The product's exact-pressure preview entry
(`run_linear_static_preview_value_with_mode`, model 0.9.0 under
`3.0.0/exact_pressure_v3`) must publish:

- the Lamé hoop and radial stresses at the inner and outer surfaces;
- the closed-end, open-end and axially restrained longitudinal membrane
  stress and the wall axial force, the restrained member both with pressure
  elongation included and with it excluded;
- each of these on both section bases, nominal and reduced.

The fixture compares every published row with closed forms that it evaluates
itself; no expected value is computed through the product.

The derivations are elementary linear elasticity: the Lamé solution of a long
thick-walled cylinder, axial force equilibrium of a closed cylinder, the
plane-strain axial stress of an axially restrained cylinder, and the same
restrained member under the product's reduced model with pressure elongation
excluded (DEL-04-07 REQ-009). No standard,
textbook example or commercial benchmark is reproduced.

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, code formulas, commercial software
  examples, proprietary data, or private project data.

## Invented Inputs

All values are invented non-engineering numbers in the fixture-local
`PKG09-STRESS-FIXTURE-UNITS-EXPLICIT-N-M-PA` basis.

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Outside diameter, `D` | 0.3 | m | length |
| Wall thickness, `t` | 0.02 | m | length |
| Corrosion allowance, `c_a` | 0.004 | m | length |
| Mill tolerance | absent (no reduction) | — | — |
| Member length A→B along global x | 3.0 | m | length |
| Elastic modulus, `E` | 2.0e11 | Pa | stress |
| Poisson ratio, `nu` | 0.25 | 1 | dimensionless |
| Internal pressure, `p` (zero external) | 4.0e6 | Pa | pressure |

The fixture uses two product documents. Both are Euler–Bernoulli, and each
terminal states its closure transfer explicitly:

| Document | Supports | Load case | Pressure | Closures at A and B |
|---|---|---|---:|---|
| cantilever | anchor at A, B free | `case:closed-end` | 4.0e6 Pa | `transfers_to_wall` |
| cantilever | anchor at A, B free | `case:open-end` | 4.0e6 Pa | `separately_supported_or_compensated` |
| cantilever | anchor at A, B free | `case:zero-pressure` | 0 Pa | `transfers_to_wall` |
| restrained | anchors at A and B | `case:restrained-closed-end`, named in `model_settings.pressure_elongation` | 4.0e6 Pa | `transfers_to_wall` |
| restrained | anchors at A and B | `case:restrained-closed-end-not-named`, not named in `model_settings.pressure_elongation` | 4.0e6 Pa | `transfers_to_wall` |

Each document is solved in both preview solver modes (`sparse_interactive`,
`dense_scrutiny`). Every row is checked at all five stations (`end_i`,
`quarter_1`, `midspan`, `quarter_3`, `end_j`).

## Section Bases

The product publishes every stress row on two bases (DEL-05-03 Design
`section-bases-and-timoshenko.md` §§1 and 2.1):

```text
r_o            = D/2                 = 0.15  m
nominal:  wall = t                   = 0.02  m,  r_i,n = r_o - t   = 0.13  m
reduced:  wall = t_r = t - c_a       = 0.016 m,  r_i,r = r_o - t_r = 0.134 m
pressure bore (closure/cap area) r_bore = r_i,r = 0.134 m
```

On each basis, the hoop and radial stresses are those of that basis's own
annulus, with `p` applied at its own inner radius `r_i,b`. On both bases, the
closure thrust acts on the pressure bore, and the wall force is divided by
that basis's wall area.

## Derivation

**Lamé thick-wall annulus** (internal pressure `p`, zero external pressure,
long straight cylinder, small strain, homogeneous isotropic):

```text
A_b           = p r_i,b^2 / (r_o^2 - r_i,b^2)
sigma_theta(r) = A_b (1 + r_o^2 / r^2)
sigma_r(r)     = A_b (1 - r_o^2 / r^2)

inner hoop  sigma_theta(r_i,b) = p (r_o^2 + r_i,b^2) / (r_o^2 - r_i,b^2)
outer hoop  sigma_theta(r_o)   = 2 A_b
inner radial sigma_r(r_i,b)    = -p
outer radial sigma_r(r_o)      = 0
```

**Longitudinal membrane.** The wall axial force is `N_w = k · p · pi · r_bore^2`.
Its stress on basis b is `N_w / (pi (r_o^2 - r_i,b^2))`, where:

- `k = 1` for closed ends transferring to the wall on a free member. Axial
  equilibrium of the closed cylinder: the cap thrust is carried by the wall.
- `k = 0` for separately supported closures on a free member (open end). No
  thrust reaches the wall, and the free member carries no axial stress.
- `k = 2 nu` for a member restrained axially at both ends. Zero axial strain
  gives `sigma_x = nu (sigma_r + sigma_theta) = 2 nu A`, constant through the
  wall, so `N_w = 2 nu p pi r_bore^2`. This is the full plane-strain
  cylinder: the restraint suppresses the closed-end free pressure strain, so
  the case includes pressure elongation and is named in
  `model_settings.pressure_elongation`.
- `k = 1` for the same restrained member when its case is not named. This is
  the deliberately reduced model of DEL-04-07 REQ-009 with pressure
  elongation excluded, not the full plane-strain cylinder: the closed-end free
  pressure strain `eps_p = (1 - 2 nu) p A_i / (E A_s)` is omitted. With both
  end displacements fixed and zero free pressure strain, the elastic axial
  force is `N_el = 0`. By REQ-009's force definitions, `N_w = N_el + p A_i =
  p pi r_bore^2` and the effective force `S = N_el = 0`.

The general relation between the two restrained cases: with the free strain
included, `N_el = -E A_s eps_p = -(1 - 2 nu) p A_i`, so `N_w = 2 nu p A_i`;
with it excluded, `N_w = p A_i`. The included and excluded wall forces differ
by the factor `2 nu`. That factor is 1/2 here only because `nu = 0.25`.

**Equilibrium mean hoop** (a thin-wall sanity relation). Integrating the Lamé
hoop through the wall gives exactly `int sigma_theta dr = p r_i,b`. This is
the equilibrium of a half-cylinder slice, so the through-wall mean hoop is
`p r_i,b / t_b`. It lies strictly between the two surface values. The
outside-diameter form `p D / (2 t_b)` lies above the inner hoop.

## Expected Values

Exact decimal values, rounded here to 10 significant digits:

| Expected value | Closed form | Value | Unit |
|---|---|---:|---|
| `nominal_lame_inner_hoop` | p (r_o²+r_i,n²)/(r_o²−r_i,n²) = 4e6 · 0.0394/0.0056 | 28142857.14 | Pa |
| `nominal_lame_outer_hoop` | 2 p r_i,n²/(r_o²−r_i,n²) = 8e6 · 0.0169/0.0056 | 24142857.14 | Pa |
| `nominal_lame_inner_radial` | −p | −4000000 | Pa |
| `nominal_lame_outer_radial` | 0 | 0 | Pa |
| `reduced_lame_inner_hoop` | 4e6 · 0.040456/0.004544 | 35612676.06 | Pa |
| `reduced_lame_outer_hoop` | 8e6 · 0.017956/0.004544 | 31612676.06 | Pa |
| `reduced_lame_inner_radial` | −p | −4000000 | Pa |
| `reduced_lame_outer_radial` | 0 | 0 | Pa |
| `closed_end_wall_axial_force` | p π r_bore² = 71824 π | 225641.7508 | N |
| `closed_end_nominal_axial_membrane` | p r_bore²/(r_o²−r_i,n²) = 4e6 · 0.017956/0.0056 | 12825714.29 | Pa |
| `closed_end_reduced_axial_membrane` | p r_bore²/(r_o²−r_bore²) = 4e6 · 0.017956/0.004544 | 15806338.03 | Pa |
| `open_end_wall_axial_force` | 0 | 0 | N |
| `open_end_nominal_axial_membrane` | 0 | 0 | Pa |
| `open_end_reduced_axial_membrane` | 0 | 0 | Pa |
| `restrained_wall_axial_force` | 2ν p π r_bore² = 35912 π | 112820.8754 | N |
| `restrained_nominal_axial_membrane` | 2ν × closed-end nominal | 6412857.143 | Pa |
| `restrained_reduced_axial_membrane` | 2ν × closed-end reduced | 7903169.014 | Pa |
| `restrained_not_named_wall_axial_force` | p π r_bore² = 71824 π (N_el = 0) | 225641.7508 | N |
| `restrained_not_named_nominal_axial_membrane` | closed-end nominal | 12825714.29 | Pa |
| `restrained_not_named_reduced_axial_membrane` | closed-end reduced | 15806338.03 | Pa |
| `restrained_not_named_effective_axial_force` | S = N_el = 0 | 0 | N |
| `zero_pressure_stress` | every Lamé and membrane row at p = 0 | 0 | Pa |
| `reduced_equilibrium_mean_hoop` | p r_bore/t_r = 4e6 · 0.134/0.016 | 33500000 | Pa |

The Lamé rows are the same in every pressurized case: closures, axial
restraint and pressure elongation do not change them.

## Discrimination

The tests also assert relations on the published rows. Each relation fails
for a specific wrong mapping:

- On each basis, inner hoop minus outer hoop equals `p`, and
  `sigma_r + sigma_theta` takes the same value at both surfaces. A thin-wall
  hoop, or a hoop evaluated on the wrong radius, breaks both relations.
- On the reduced basis, the closed-end membrane equals half the outer hoop:
  the cap bore and the annulus coincide. On the nominal basis, it differs from
  half the nominal outer hoop (12825714.29 against 12071428.57 Pa), because
  the cap thrust acts on the pressure bore and is divided by the nominal area.
  If a cap were placed on the nominal bore, or the membrane were derived from
  the hoop, the result would be wrong.
- On each basis, the equilibrium mean `p r_i,b / t_b` lies strictly between
  the outer and inner hoop: nominal 26000000 Pa, reduced 33500000 Pa. The
  `p D / (2 t_b)` form (nominal 30000000 Pa, reduced 37500000 Pa) lies above
  the inner hoop. So neither thin-wall value can stand in for a published
  Lamé row.
- The product's toroidal crown membrane hoop (`toroidal_membrane_hoop` in
  `core/loads/stress_recovery`, crown value, any bend radius above `r_o`;
  1.0 m here) on the pressure bore and reduced wall equals
  `reduced_equilibrium_mean_hoop`.
- Open-end (`k = 0`) and restrained (`k = 2 nu`) cases separate the closure
  and restraint paths from the closed-end path. Their membranes are 0 and 0.5
  times the closed-end value.
- The restrained case that is not named (`k = 1`) separates the pressure
  elongation path. Its wall force is `1/(2 nu)` times the named case's (2
  here), its membranes equal the closed-end values, and its effective force
  is zero. If the setting were ignored, or applied to a case it does not
  name, one of the two restrained cases would fail.

## What The Stress Benchmark Asserts

The benchmark computes the closed forms above with its own binary64
arithmetic, and only the product path produces the observed rows:
`pipe_lame_hoop_stress_v2`, `pipe_lame_radial_stress_v2`,
`pipe_axial_membrane_stress_v2` (each with its `section_basis`) and
`pipe_wall_axial_force_v2` (published once), and for the restrained case that
is not named, `pipe_effective_axial_force_v2`. Each pressurized case's result
must state whether it includes pressure elongation: the named restrained case
`included`, the restrained case that is not named and the cantilever cases
`not_included`. The tests require exactly one
published row per kind, component, station, load case and basis. They also
require the units `Pa` and `N`, `MECHANICS_SOLVED`, and the
`exact_pressure_v3` profile.

## Tolerance Policy

Under `DEC-024`/`DEC-026`, governed tolerance values remain `TBD`. The
fixture records no fixture-local `tolerance_policy` override. The comparisons
use the analytic-class relative seed `1.0e-9`, scaled by the larger of the
expected magnitude and the case's load scale: `p` for stresses and the
closed-end cap thrust for forces. A zero expectation is therefore judged on
the scale of the applied load. At the time of writing, every observed row
agreed with its closed form to binary64 rounding.

## Boundary

No code stress category, stress index, allowable, design-pressure or
minimum-wall formula, stress intensification or flexibility factor, or
acceptance criterion is encoded. No lifecycle, release, professional,
certification, or code-compliance claim is made.
