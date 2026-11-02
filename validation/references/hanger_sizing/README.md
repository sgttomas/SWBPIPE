# Hanger-sizing arithmetic reference (R-3, R-3b, R-4/R-4b arithmetic)

Independent reference values for DEL-04-11 hanger sizing.

- **Basis:** `execution/PKG-04/DEL-04-11/Work/hanger-sizing.md` §4.2 (one origin,
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
- **Invented data:** every value is invented. No supplier catalogue or standards
  data is used.

## Regenerate and check

```sh
python3 validation/references/hanger_sizing/generate_r3.py          # writes r3_arithmetic.json
python3 validation/references/hanger_sizing/generate_r3.py --check  # exit 1 if the frozen file differs
```
