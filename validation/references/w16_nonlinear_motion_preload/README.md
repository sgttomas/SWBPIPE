# W-16 stage 2 preload addendum: `c1_hanger` under the hanger preload law

This package supplements the frozen package
[`../w16_nonlinear_motion`](../w16_nonlinear_motion/README.md), file
`references.json`, request `requests/c1_hanger.json`. The frozen package and
its files are unchanged. The frozen package derived `c1_hanger` with the
variable spring hanger as a bare spring of its entered rate. Its record carries
an entered cold load of 1000 N, which the frozen package did not apply. This
package derives the same quantities under the law that
`execution/PKG-04/DEL-04-03/ScopeOfWork.md` REQ-014 and
`execution/PKG-04/DEL-04-11/ScopeOfWork.md` REQ-014 state:

- the hanger is a spring of its entered rate k on its declared translational
  DOF (UX here), plus its entered `cold_load` F_cold as a constant force on
  the pipe along +UX at the support's node, in every load case;
- its published support force on that DOF is F = F_cold − k·(u − g), where g
  is the case's movement of the spring's base;
- the cold load is also published separately as the support's applied force;
- `installed_load` and `hot_load` are review data and are not applied.

Status: these are candidate references. Their author TASK derived them
without seeing any product output. They have not yet been independently
refuted.

No maintained product test reads this package yet, because main does not yet
implement the preload law; the preload landing (DEL-04-11 REQ-014) points the
product tests at it. Until then, its own check and the tests of the
reference-package table (`tests/reference_packages.py`) are what maintain it.

## Files

| File | Role |
|---|---|
| `generators/w16_nl_preload_reference.py` | the generator. It reads the committed request `../w16_nonlinear_motion/requests/c1_hanger.json` and the frozen `references.json`, and imports the frozen generator `../w16_nonlinear_motion/generators/w16_nl_reference.py` for its exact frame, state enumeration and published-row rules. It writes `references.json` and `SHA256SUMS` |
| `references.json` | the frozen layout. `requests.c1_hanger.cases` holds the case rows under the law, in exactly the frozen format, so that a test compares what it compared before. `preload.c1_hanger` holds, per case, the published applied force, the hanger's support force and every changed leaf next to its frozen (stiffness-only) value. `crosscheck` lists the reproduction and closed-form checks |
| `SHA256SUMS` | the hashes of every other file of this package. The generator writes it |

## Check

From this directory:

```sh
python3 -I generators/w16_nl_preload_reference.py --check
```

The check rebuilds every output into a temporary directory and compares it
byte for byte with the committed files. It changes no file. `--write`
rewrites the outputs, for intentional changes only.

## Method and results

1. **Reproduction.** With F_cold set to zero, the generator's rows for all
   three cases (`load:AXIAL`, `load:AXIAL-NEG` and `load:INEXACT`) equal the
   frozen rows exactly, as JSON values.
2. **The law.** The generator adds F_cold to the force vector at the hanger's
   DOF and solves exactly with the frozen generator's frame. It forms the
   hanger's published row as −k·(u − g) + F_cold, which is the frozen
   spring-row rule plus the cold load. Closed form: the bar's condensed tip
   stiffness is k_m = EA/L = k. That gives u = (k·g + F_cold)/(k + k_m),
   F = (F_cold + k·g)/2, and an anchor UX row of −F. The generator checks
   these exactly. There is no nonlinear support, so each case has exactly one
   state, a strict one.
3. **Effect.** Each case gains F_cold/2 = 500 N in the hanger's and the
   anchor's force, with the anchor's acting in the opposite sense. The bar's
   axial force changes by the same amount, and the tip UX moves by
   F_cold/(2k). In `load:AXIAL-NEG` the magnitudes fall by 500 N, because
   k·g < 0 there. All other leaves are unchanged: the transverse and rotational
   displacements, shear, bending, torsion and moments.

**Axis and sign of the cold load.** The cold load acts along the positive axis
of the hanger's declared stiffness DOF (+X here). The positive axis is the
integrator's decision for both hanger families, read from DEL-04-03 REQ-014's
convention for the constant-effort support and DEL-04-11 REQ-014's "through
the constant-effort seam"; the words "positive axis" enter the spring's own
sentence in both scopes with the preload landing. If the sign
were decided otherwise, every value here that the cold load moves would
change.

**Sign convention.** The frozen generator publishes a spring's row as
−k·(u − g) (`published()`). That is the force the support exerts on the pipe,
positive along +DOF, and the anchor's row K·u − f uses the same sense. This
package adds +F_cold to that row, which is the law's F = F_cold − k·(u − g).

**Unaffected requests.** No other request of the frozen package carries a
variable spring hanger, and the generator checks this. Their expectations are
not repeated here.

## Independence

This package is independent of the product. It is not independent of the
frozen package it builds on: it uses the frozen generator's frame and solver.
Its check reads nothing outside this package and `../w16_nonlinear_motion`.

What is independent: the generator imports, reads and runs no product code and
no product output, and copies no number. Every value comes from the leaves of
the committed request, in exact rationals. Its author saw no product value for
this model under the law.

What is not independent: it reuses the frozen generator's frame, enumeration
and published-row rules, so it inherits their correctness. That shared code is
reference code, not product code. The law as modelled is the author's reading
of the two REQ-014 texts.

All data are invented. Standard claim fence applies (F-PIP-2).
