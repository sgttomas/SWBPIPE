# Hanger-sizing arithmetic reference (R-3, R-3b, R-4/R-4b arithmetic)

Independent reference values for DEL-04-11 hanger sizing.

- **Basis:** the DEL-04-11 hanger-sizing design, §4.2 (one origin,
  the installed position), §6 (loads, variation, travel and allowance, fit tests,
  family policy and selection) and §7 (the verification pass). It covers R-3
  (the arithmetic, the fit tests and both selection rules), R-3b (refusal when
  W ≤ 0) and the arithmetic part of R-4 and R-4b (verified loads, variation,
  departure, failed tests, verification standing). It does not cover the derived
  solves themselves.
- **Independence:** `generate_r3.py` was written from that design text and the
  task brief only. It reads, searches and imports no product code (`core/`,
  `apps/`, `tests/`) and no other generator under `validation/`. It uses only the
  Python 3 standard library.
- **Precision:** inputs are exact rationals of decimal strings (or stated
  fractions). Results are computed exactly with `fractions.Fraction` and emitted
  rounded to the nearest binary64. Fit tests, tie-breaks and verification tests
  are decided on exact values. Boundary-equality cases use inputs that binary64
  represents exactly. The generator checks that every discrete outcome stays the
  same when each input is replaced by the exact value of its emitted binary64.
  It also checks each case's intended outcome and the values stated in the brief.
- **Conventions:** SI units (N, m, N/m). `design_index` is 0-based into
  `u_operating_m`.
- **Absent verified ratios:** in a `verification` block, an entry of the
  `variation` list is `null` where the verified reference load is not positive
  (F_i^v ≤ 0 under `operating_load`). No ratio is formed for that case, V0
  reports it, and V is judged only on the ratios that are formed (§7). The
  sizing-time `variation` list is never `null`: sizing forms no ratio with a
  non-positive reference, because V0 fails the record first.
- **Invented data:** every value is invented. No supplier catalogue or standards
  data is used.

## Regenerate and check

```sh
python3 validation/references/hanger_sizing/generate_r3.py          # writes r3_arithmetic.json
python3 validation/references/hanger_sizing/generate_r3.py --check  # exit 1 if the frozen file differs
```

# Derived-solve reference (R-1, R-2, R-5)

`generate_r125.py` writes `r125_derived_solves.json`. It holds the sizing quantities
of the derived solves of §4.2 for each candidate: W (S_W), u_C (S_C) and u_i (S_Oi).
It also holds each lift-off support's state in every solve (§5).

- **Basis:** `hanger-sizing.md` §4, §5 and §12, the task brief and classical
  Euler–Bernoulli beam and frame mechanics, with axial deformation (EA) included and
  shear deformation neglected. The cases are:
  - R-1a, R-1b and R-1c: continuous beams with two equal, three equal and two
    unequal spans (three-moment equation);
  - R-2: a free riser with u_i = α·ΔT·H;
  - R-5a: a lift-off support that is active in S_W and S_C and inactive in S_O;
  - R-5b: the same beam with a P too small to lift the support.
- **Independence:** written from that text and the brief only. It reads, searches
  and imports no product code (`core/`, `apps/`, `tests/`) and uses only the Python 3
  standard library. It shares no code with `generate_r3.py`.
- **Precision:** every W and every u_C is exact (`fractions.Fraction`), and so is
  every u_i except R-5a's. R-5a's u_i is a rational divided by π. It is evaluated
  with π to 101 digits in 80-digit decimal arithmetic. All values are emitted
  rounded to the nearest binary64.
- **Self-checks:** each closed form must agree exactly with an exact-rational
  plane-frame stiffness solve written in the generator, which uses active-set
  iteration for lift-off. Every solve's vertical equilibrium is asserted. R-5's
  active-set consistency is asserted: a compressive reaction while the support is
  active, and a zero reaction with an upward gap while it is inactive. The lift-off
  states must not change when every input is replaced by its binary64 value.
- **Support model:** the R-1 ends are `rigid_uz`, with a `guide` at one end on UX,
  UY and RX and a `guide` at the other end on UY. No end restrains RY. RX is needed
  because no translational restraint on collinear nodes stops rotation about the
  beam axis. The out-of-plane DOFs carry no load, so no value depends on the shear
  modulus.

```sh
python3 validation/references/hanger_sizing/generate_r125.py          # writes r125_derived_solves.json
python3 validation/references/hanger_sizing/generate_r125.py --check  # exit 1 if the frozen file differs
```
