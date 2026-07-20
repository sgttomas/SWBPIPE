# MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE

## Purpose

Invented mechanics witness for sub-span (partial-extent) wind exposure in
static-equivalent occasional-load generation (tranche
`R14-W2-T4`, brief `CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001`, ruling
`DEC-068` item 2). It derives, from elementary statics only:

1. The partial-extent wind intensity on a user-marked fraction range of a
   straight span.
2. The exposed-segment resultant and its centroid.
3. The lever-rule statically-equivalent end shares at the existing preview
   force-lumping tier.
4. The exact reduction of the lever rule to the existing 50/50 whole-span
   shares at full extent `(0, 1)`.
5. The superposition of multiple disjoint extents on one span.
6. The closed-form values consumed by the benchmark fixture
   `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE`.

Pure mechanics from user inputs: no code wind profile, exposure category,
coefficient catalog, threshold, or default is encoded. All numeric values
are invented non-engineering numbers.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, proprietary
  data, or private project data.

## Setup and Derivation

### Partial-extent intensity

A straight span of length `L` runs from end node `i` (at `x = 0`) to end
node `j` (at `x = L`). The user marks the fraction range `[a, b]` of the
span as wind-exposed, with `0 <= a < b <= 1`. The user-entered wind
pressure `p` and shape factor `G` project onto the exposed diameter
`D_exposed` (outside diameter plus twice the insulation thickness when
insulation is supplied — the same section basis as whole-span marking),
giving the uniform intensity over the exposed segment only:

```text
w = p * G * D_exposed          for x in [a*L, b*L]   (force/length)
w = 0                          elsewhere on the span
```

### Resultant and centroid of the exposed segment

```text
W    = Integral_{a*L}^{b*L} w dx = w * (b - a) * L
x_c  = (a*L + b*L) / 2 = c * L,   with c = (a + b) / 2
```

The resultant is exact for a uniform intensity: the centroid of a uniform
segment is its midpoint.

### Lever-rule statically-equivalent end shares

The preview tier applies generated distributed loads as
statically-equivalent end forces (no fixed-end moments anywhere at this
tier). Two end forces `R_i` (at `x = 0`) and `R_j` (at `x = L`) preserve
the force and moment resultants of the exposed-segment load:

```text
Force:              R_i + R_j = W
Moment about i:     R_j * L   = W * x_c = W * c * L   =>  R_j = W * c
Hence:              R_i = W * (1 - c)
Check (moment about j):  R_i * L = W * (1 - c) * L = W * (L - x_c)   OK
```

Two equations, two unknowns: the two-end-force reduction is unique and
exact. This is the lever rule — each end takes the share proportional to
the resultant's lever arm to the opposite end.

### Exact whole-span reduction at (a, b) = (0, 1)

```text
(a, b) = (0, 1)  =>  W = w * L,   c = 1/2
R_i = R_j = w * L / 2
```

This is exactly the existing 50/50 whole-span lumping
(`prepare_lumped_nodal_loads` half-total and the straight-span
`w * L / 2` end shares), so whole-span behavior is the `c = 1/2` special
case of the lever rule. In floating-point arithmetic the implemented forms
also agree exactly at `(0, 1)`: `(w*L*(1-0)) * (1-1/2) = (w*L) * 0.5 =
w*L/2` with no rounding difference, because multiplying by `1.0` and by
`0.5` are exact.

### Superposition of disjoint extents

For multiple disjoint extents `[a_1, b_1], [a_2, b_2], ...` on one span
(non-overlapping interiors), the total load is the sum of the per-extent
uniform loads. Resultants and first moments are additive integrals, and
the assembled nodal force vector is linear in the applied loads, so the
per-extent lever-rule shares superpose exactly:

```text
R_i,total = Sum_k W_k * (1 - c_k)
R_j,total = Sum_k W_k * c_k
```

No cross-term exists at this tier. Overlapping extents on one pipe are a
blocking input error (they would double-count exposure), not a summed
case.

### Curved-bend boundary (no derivation — fail closed)

The arc-consistent machinery for curved-bend macro-realized spans
integrates a full uniform intensity along the arc. No partial-arc closed
form is derived or invented here: a partial extent marked on a
macro-realized bend span is a blocking diagnostic directing whole-span
marking or separate pipes. This witness derives straight-span statics
only.

## Invented Inputs (benchmark fixture)

All values are invented non-engineering numbers in the fixture-local
`PKG09-FIXTURE-UNITS-EXPLICIT-N-M-RAD-K` basis, reusing the occloadgen
fixture section so the intensity closed form is shared:

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Outside diameter, `D_o` | 0.2 | m | length |
| Insulation thickness, `t_ins` | 0.025 | m | length |
| Exposed diameter, `D_exposed = D_o + 2 t_ins` | 0.25 | m | length |
| Wind pressure, `p` | 480.0 | Pa | pressure |
| Wind shape factor, `G` | 0.7 | ratio | dimensionless |
| Wind direction | global `Y` | - | - |
| Span length, `L` | 3.0 | m | length |
| Extent A, `[a_1, b_1]` | [0.2, 0.7] | ratio | dimensionless |
| Extent B, `[a_2, b_2]` | [0.8, 1.0] | ratio | dimensionless |

Extents A and B are disjoint (`b_1 = 0.7 <= a_2 = 0.8`), so both may be
marked on the same span and superpose.

## Closed-Form Values (benchmark fixture)

```text
w = p * G * D_exposed = 480.0 * 0.7 * 0.25 = 84.0 N/m

Extent A: [0.2, 0.7]
  W_A  = 84.0 * (0.7 - 0.2) * 3.0 = 126.0 N
  c_A  = (0.2 + 0.7) / 2 = 0.45
  R_iA = W_A * (1 - c_A) = 126.0 * 0.55 = 69.3 N
  R_jA = W_A * c_A       = 126.0 * 0.45 = 56.7 N

Extent B: [0.8, 1.0]
  W_B  = 84.0 * (1.0 - 0.8) * 3.0 = 50.4 N
  c_B  = (0.8 + 1.0) / 2 = 0.9
  R_iB = W_B * (1 - c_B) = 50.4 * 0.1 = 5.04 N
  R_jB = W_B * c_B       = 50.4 * 0.9 = 45.36 N

Superposed (A + B on one span):
  R_i = 69.3 + 5.04  = 74.34 N
  R_j = 56.7 + 45.36 = 102.06 N
  W   = 126.0 + 50.4 = 176.4 N        (= 84.0 * 0.7 * 3.0, the combined
                                        covered fraction 0.5 + 0.2 = 0.7)

Whole-span reduction check at (0, 1):
  W = 84.0 * 3.0 = 252.0 N, c = 0.5
  R_i = R_j = 126.0 N  = w * L / 2    (the existing 50/50 shares)
```

Cross-checks: force balance `R_i + R_j = W` holds for each extent and for
the superposition; the moment about node i of the superposed end forces,
`R_j * L = 102.06 * 3.0 = 306.18 N-m`, equals the sum of the exposed
resultant moments `W_A * c_A * L + W_B * c_B * L = 126.0 * 1.35 + 50.4 *
2.7 = 170.1 + 136.08 = 306.18 N-m`. The arithmetic above was recomputed
independently in decimal arithmetic from the closed forms; the benchmark
fixture recomputes the same closed forms in code, so agreement is at
floating-point identity (internal assertion epsilon 1.0e-9).

## Tolerance Policy

Per `DEC-024`/`DEC-026`, governed tolerance values remain `TBD` and the
fixture records no fixture-local `tolerance_policy` override; any future
override may only tighten the governed value.

## Boundary

No dynamics (response spectra, time history, modal content — disposition
stays with `D-12`), no code wind coefficient or catalog, no default, no
fixed-end/work-equivalent tier change, no partial-arc bend integration
(curved-bend macro spans fail closed for partial extents). No lifecycle,
release, professional, certification, or code-compliance claim is made.
