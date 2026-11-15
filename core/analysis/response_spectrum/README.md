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
- A period query T = 1/f is located exactly against the knots and the range ends, as a rational,
  through the sign of P·f − 1 with integer mantissas. Rounding 1/f never decides whether a query is
  in range or which side of a knot it falls on.

  A consequence: on a period table whose first knot is authored as `0.1` s, a query of exactly
  10 Hz is outside the range. The binary64 0.1 is larger than 1/10, so the query's period lies
  below the first knot.
- After location, the arithmetic is binary64 with the rounding that `SpectrumTable::evaluate`
  documents.

## Refusals

| Code | When |
|---|---|
| `SPECTRUM_RECORD_INVALID` | The record breaks a §2.4 rule. The refusal names the field. |
| `SPECTRUM_DAMPING_OUT_OF_RANGE` | A valid record has ζ above 0.2. |
| `SPECTRUM_DAMPING_MISMATCH` | The spectra of one request have different damping ratios (`common_damping_ratio`). |
| `SPECTRUM_FREQUENCY_OUTSIDE_RANGE` | The query is outside the tabulated range. |
| `SPECTRUM_QUERY_FREQUENCY_INVALID` | The query is not a positive finite frequency in a catalog unit. |
| `SPECTRUM_SCALE_FACTOR_INVALID` | The scale factor is not positive and finite. |
| `SPECTRUM_EVALUATION_NONFINITE` | A value would leave the binary64 range. |

## Example

```sh
cargo run --manifest-path core/analysis/response_spectrum/Cargo.toml --example evaluate_spectrum -- request.json
```

`request.json` holds one spectrum and its queries. The table below is invented: it is T1 of the
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
