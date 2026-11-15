# response_spectrum (DEL-04-10, units S2 and S4)

This crate holds two units of DEL-04-10's response-spectrum analysis
(`execution/PKG-04/DEL-04-10/ScopeOfWork.md`):

- **S2** admits a user-entered spectrum table and evaluates its pseudo-spectral acceleration at a
  query frequency.
- **S4** combines supplied signed contributions by the modal, rigid-response and directional rules,
  and fixes the signed-contribution contract
  ([Combinations (unit S4)](#combinations-unit-s4)).

The crate is not a spectrum solve. It forms no modal response or rigid response, and it publishes
no spectrum result.

## The spectrum table (unit S2)

### What it takes and gives

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

### Method

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
- After location, the arithmetic is binary64 with the rounding that `SpectrumTable::evaluate`
  documents.

### Refusals

| Code | When |
|---|---|
| `SPECTRUM_RECORD_INVALID` | The record breaks a §2.4 rule. The refusal names the field. |
| `SPECTRUM_DAMPING_OUT_OF_RANGE` | A valid record has ζ above 0.2. |
| `SPECTRUM_DAMPING_MISMATCH` | The spectra of one request have different damping ratios (`common_damping_ratio`). |
| `SPECTRUM_FREQUENCY_OUTSIDE_RANGE` | The query is outside the tabulated range. |
| `SPECTRUM_QUERY_FREQUENCY_INVALID` | The query is not a positive finite frequency in a catalog unit. |
| `SPECTRUM_SCALE_FACTOR_INVALID` | The scale factor is not positive and finite. |
| `SPECTRUM_EVALUATION_NONFINITE` | A value would leave the binary64 range. |

### Example

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

## Combinations (unit S4)

`combine` takes a `CombinationRequest` and returns a `Combination` of unsigned magnitudes, or named
refusals. It does not solve, recover or evaluate anything: every number it combines is supplied.

### What it takes

- **The choices.** The modal rule, the directional rule and whether to apply the missing-mass
  correction, with f_r and the rigid rule when it is applied, and the spectra's damping ratio. None
  has a default.
- **The modal metadata.** For every extracted mode: its `mode_index`, its DEL-04-09 cluster
  (`cluster:N`) and its frequency in Hz, as the modal result publishes them. No cluster is inferred
  and no cluster tolerance is applied. The structure is checked: modes 1..n in ascending frequency,
  and clusters chained from 1 over adjacent modes.
- **The rows** (`contract::Quantity`), each with a unit and frame that follow from its kind:
  - node displacements and rotations in global axes (m, rad);
  - element end forces and moments on the element at end i or j, in global or member axes (N, N*m);
  - support reactions applied to the piping, in global axes, with `Fd` along a skew support's d/|d|.

  Keys follow the frozen references: `N3.uy`, `P1.i.Fx`, `P1.i.Fx_m`, `A1.Fx`.
- **The signed contributions.** One per mode of a periodic cluster and requested direction, already
  scaled by the direction's factor. A contribution for a mode of a rigid cluster is refused.
- **The rigid responses**, when the correction is applied: one signed response per direction,
  stating the clusters its load left out as periodic. A mismatch with the request's classification
  is refused.

### What it does (CLM-001)

1. **Classifies.** A cluster is periodic when its lowest frequency is below f_r, and every cluster is
   periodic without the correction.
2. **Sums each periodic cluster's contributions with their signs** in ascending mode order. The sums
   are published with their sign tag.
3. **Applies the modal rule** across the cluster sums: SRSS, CQC, grouping, ten percent or absolute
   sum. Each cluster's lowest frequency stands for it in ρ and in the closeness tests.
4. **Combines the rigid response** with the periodic part: SRSS √(R_p² + R_r²), or the absolute sum
   R_p + |R_r|.
5. **Combines the directions** in X, Y, Z order: SRSS, 100-40-40 or absolute sum.

### Numerical choices (to be confirmed by Agent 0)

- **ρ.** Evaluated with s = f_low/f_high ≤ 1 on the published frequencies. This is equal in exact
  arithmetic to the scope's form in r = ω_j/ω_i, and no intermediate can overflow. ρ_ii = 1.
- **The closeness tests.** Decided exactly on the represented frequencies, 10·f ≤ 11·f_low, so
  rounding never decides the inclusive 10 % boundary.
- **Scaling.** Every rule with a square root is evaluated on its values divided by a power of two
  near their largest. No square overflows or underflows, and the result rounds once on the way
  back. A result beyond binary64 is refused.
- **Cancellation.** Cluster sums and the CQC double sum are plain ordered binary64 sums, and their
  rounding is relative to their terms. A CQC sum below zero is refused, never clamped. An
  intermediate overflow of a cluster sum (terms near 1e308) is refused even if the exact sum is
  finite.
- **100-40-40.** The governing direction is the largest magnitude (the first in X, Y, Z order on a
  tie). The factor is the binary64 0.4, above 2/5 by 2⁻⁵⁴ of it.

### Refusals

| Code | When |
|---|---|
| `SPECTRUM_METHOD_CHOICE_MISSING` | A method choice is absent or unknown (JSON entry). |
| `SPECTRUM_DAMPING_OUT_OF_RANGE` | ζ is outside 0 ≤ ζ ≤ 0.2. |
| `SPECTRUM_CQC_DAMPING_UNSUPPORTED` | CQC with ζ = 0. |
| `SPECTRUM_RIGID_FREQUENCY_INVALID` | f_r is absent or not positive and finite. |
| `SPECTRUM_DIRECTION_INVALID` | No direction, a repeated direction, or an unknown one. |
| `SPECTRUM_MODAL_METADATA_INVALID` | The modal metadata breaks the modal result's structure. |
| `SPECTRUM_ROW_INVALID` | A row is empty, repeated, or stated with a unit, frame or key its kind does not have. |
| `SPECTRUM_CONTRIBUTION_INVALID` | A contribution is missing, repeated, not finite, of the wrong length, unsigned, or for a mode or direction the request does not have. |
| `SPECTRUM_CONTRIBUTION_NOT_PERIODIC` | A contribution for a mode of a rigid cluster. |
| `SPECTRUM_RIGID_RESPONSE_INVALID` | A rigid response is missing, repeated, not finite or of the wrong length, or is supplied without the correction. |
| `SPECTRUM_RIGID_RESPONSE_PREMISE_MISMATCH` | The clusters a rigid load left out are not the periodic ones. |
| `SPECTRUM_COMBINATION_RADICAND_NEGATIVE` | A CQC sum rounds below zero. |
| `SPECTRUM_COMBINATION_NONFINITE` | A combined value leaves binary64. |

### JSON entry and example

`combination_json` reads and writes the unit's documents
(`openpipestress.spectrum_combination.input/v1` and `output/v1`; not `spectrum-1`). The
`combine_contributions` example reads an input document on standard input and prints the output
document:

```sh
cargo run --manifest-path core/analysis/response_spectrum/Cargo.toml --example combine_contributions \
  < core/analysis/response_spectrum/fixtures/combination/c4_base.input.json
```

`fixtures/combination/README.md` lists the contract fixtures.
