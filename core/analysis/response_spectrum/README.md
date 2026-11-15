# response_spectrum (DEL-04-10, unit S2)

This crate admits a user-entered spectrum table and evaluates its pseudo-spectral acceleration at a
query frequency. It is the first unit of DEL-04-10's response-spectrum analysis
(`execution/PKG-04/DEL-04-10/ScopeOfWork.md`). It is not a spectrum solve. It forms no modal
response, combination or rigid response, and it publishes no spectrum result.

## What it takes and gives

- A spectrum record in the model document's form (`spectra[]`, DEL-02-01 §2.4), which needs:
  - the abscissa (`frequency` or `period`) and its catalog unit;
  - the ordinate, either `acceleration` with a catalog unit or `g_multiple` with an explicit `g_value`;
  - the damping ratio;
  - the interpolation on each axis (`linear` or `log`);
  - at least two points.
- A query frequency with its catalog unit, and a positive finite scale factor.
- The result is the location (on a knot or strictly inside a segment), the interpolated ordinate S
  and s·S in m/s², or a named refusal.

Nothing is defaulted, and no table is extrapolated. The damping ratio is recorded and checked
(0 ≤ ζ ≤ 0.2 for a solve); the ordinates are already at that damping.

## Method

The DEL-04-10 scope records Agent 0's decisions on the method:

- A table is interpolated on its authored axis: a period table in period, a frequency table in
  frequency.
- A period query is located exactly. The operands are represented values: the query frequency f
  as the binary64 the modal result publishes (or the user enters), and each knot P as the binary64
  of the authored abscissa. The period T = 1/f of that f is compared with each knot and with the
  range ends as a rational, through the sign of P·f − 1 with integer mantissas. Rounding 1/f never
  decides whether a query is in range or which side of a knot it falls on.
- The exact comparison selects the interval and decides in or out of range, and nothing more. It
  does not make the modal frequency exact (it carries the modal solve's own error), nor a knot the
  user's intended decimal.

  A consequence: on a period table whose first knot is authored as `0.1` s, a query represented
  as exactly 10 Hz is outside the range. The binary64 0.1 is larger than 1/10, so the query's
  period lies below the first knot.
- After location, the arithmetic is binary64 on significands with exact exponent bookkeeping, so
  no intermediate overflows or underflows. The two weights are formed directly as positive
  quotients (never as 1 − t), a linear ordinate is w₀·y₀ + w₁·y₁, and a logarithmic ordinate is
  reached from the knot with the larger weight. `SpectrumTable::evaluate` documents each formula.

## Accuracy of each published value

The interpolated ordinate, S and s·S each carry a certified relative error bound
(`Ordinate::*_bound`) against the exact interpolation of the represented table at the represented
query. A value is published only when its bound is at most 1e-9 (`STATED_ACCURACY`). Otherwise the
evaluation is refused as `SPECTRUM_EVALUATION_ACCURACY_UNSUPPORTED`.

- **What the argument proves, under its premises.**
  - Each binary64 operation on a normal significand rounds once (relative error at most u = 2⁻⁵³).
  - The exponent bookkeeping and the significand splits are exact.
  - The logarithm helpers are within 2.11κ + 3u, and the bound composes these errors through each
    formula with its condition number.
  - The result: a few u for a linear ordinate, and about 62u·|z| + 10u for a logarithmic one, with
    |z| ≤ |ln(y₁/y₀)|/2 ≤ 727, so below 10⁻¹¹ for any positive finite table.
  - Rounding the published value adds nothing in the normal range, and at most 2⁻¹⁰⁷⁵ absolutely in
    the subnormal range.
- **Premises.** IEEE 754 binary64 with round to nearest (Rust's `+ − × ÷` and `mul_add`). The
  platform's `ln`, `ln_1p` and `exp` are each within κ = 2⁻⁵⁰ (4 ulp) relative of the exact value. This
  is assumed, not checked, for Apple's libm on macOS, glibc on Linux and the Microsoft C runtime on
  Windows.
- **What is tested, and is not a proof.**
  - The frozen F2 references.
  - The first review's ten probes.
  - Published values either side of the subnormal threshold and of 2^1024.
  - 240 seeded adversarial tables over the eight axis and rule combinations, each compared with
    150-digit expectations from `tests/oracle/generate.py`.

  In every accepted case the observed error is within the certified bound, and each refusal is the
  one the exact values predict.
- **Refused.** A published value that exceeds the largest binary64, rounds to zero, or lies below
  about 2.5·10⁻³¹⁵ in the subnormal range. `docs/RELEASE_LIMITATIONS_v1.0.md` LIM-024 states the
  refusal.

## Refusals

| Code | When |
|---|---|
| `SPECTRUM_RECORD_INVALID` | The record breaks a §2.4 rule. The refusal names the field. |
| `SPECTRUM_DAMPING_OUT_OF_RANGE` | A valid record has ζ above 0.2. |
| `SPECTRUM_DAMPING_MISMATCH` | The spectra of one request have different damping ratios (`common_damping_ratio`). |
| `SPECTRUM_FREQUENCY_OUTSIDE_RANGE` | The query is outside the tabulated range. |
| `SPECTRUM_QUERY_FREQUENCY_INVALID` | The query is not a positive finite frequency in a catalog unit. |
| `SPECTRUM_SCALE_FACTOR_INVALID` | The scale factor is not positive and finite. |
| `SPECTRUM_EVALUATION_ACCURACY_UNSUPPORTED` | A published value cannot be stated in binary64 within 1e-9 relative. |

## Example

```sh
cargo run --manifest-path core/analysis/response_spectrum/Cargo.toml --example evaluate_spectrum < request.json
```

The request on standard input holds one spectrum and its queries. The table below is invented: it is T1 of the
frozen reference `validation/references/f2_spectrum/closed_form/spectrum_tables.json`. The crate's
tests run this exact request.

```json
{
  "spectrum": {
    "id": "T1",
    "abscissa": "frequency",
    "ordinate": "acceleration",
    "units": {"abscissa": "Hz", "ordinate": "m/s^2"},
    "damping_ratio": 0.05,
    "interpolation": {"abscissa": "log", "ordinate": "log"},
    "points": [[0.3, 0.9], [1.5, 3.6], [6.25, 9.81], [25.0, 7.5], [80.0, 2.2]]
  },
  "queries": [
    {"frequency": {"value": 6.25, "unit": "Hz"}, "scale_factor": 1.5},
    {"frequency": {"value": 3.0, "unit": "Hz"}, "scale_factor": 1.5},
    {"frequency": {"value": 80.5, "unit": "Hz"}, "scale_factor": 1.5},
    {"frequency": {"value": 3.0, "unit": "Hz"}, "scale_factor": 0.0}
  ]
}
```

The output gives:

- the first query exactly on a knot (S = 9.81 m/s²);
- the second interpolated (S ≈ 5.858 m/s², s·S ≈ 8.787 m/s²);
- the third refused as `SPECTRUM_FREQUENCY_OUTSIDE_RANGE`;
- the fourth refused as `SPECTRUM_SCALE_FACTOR_INVALID`.
