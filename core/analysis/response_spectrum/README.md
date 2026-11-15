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
5. **Combines the directions** in X, Y, Z order: SRSS, 100-40-40 (with the exact 2/5) or absolute
   sum.

### Accuracy (Agent 0's route A/E2)

Every published combined magnitude is a binary64 that a runtime enclosure shows within 1e-9
relative of the exact value of its rule on the represented operands. The operands are the
supplied binary64 contributions, rigid responses, frequencies and damping ratio. A magnitude is
published as zero only when exact arithmetic proves it zero. Anything else refuses the request,
naming the row (DEL-04-10 scope, "Combined magnitudes are certified against their exact values").

- **Exact parts.** Every finite binary64 is an exact dyadic m·2^e. Cluster sums, squares,
  absolute products, absolute sums and the rigid absolute sum are formed exactly, with
  crate-local big integers and no new dependency. Nothing overflows or underflows in between.
  A cluster sum is published as the binary64 nearest its exact value.
- **ρ.** ρ = N·√(ab)/(D·b), with:
  - a = f_low and b = f_high;
  - N = 8ζ²(b + a)·a·b² and D = (b² − a²)² + 4ζ²·a·b·(b + a)², exact dyadics;
  - √(ab) and the division enclosed outward. ρ is not rational in general; only its kernel N/(D·b)
    is.
- **CQC.** Q = Σ R_i² + 2·Σ_{i<j} ρ_ij·R_i·R_j is enclosed from the exact cluster sums and ρ's
  enclosure: each pair takes the end that makes it smallest or largest. Its square root is taken
  only from an enclosure with a positive lower end (a positive radicand), or of an exact zero.
- **The other rules.** The rigid rules (√(R_p² + R_r²), R_p + |R_r|) and the directional rules
  (SRSS, the absolute sum, and 100-40-40 as (3·max + 2·ΣR)/5 with the exact 2/5) are monotone in
  non-negative arguments, so they carry enclosures. Their square roots and the division by 5 are
  enclosed from a binary64 candidate on exactly power-of-two-scaled operands, widened by 2^-50 and
  checked exactly. Exact integer arithmetic is the fallback when the check fails.
- **Publication.** The binary64 v nearest the enclosure's midpoint is published only when
  10⁹·(v − lo) ≤ lo and 10⁹·(hi − v) ≤ hi, decided exactly. This includes the final conversion to
  binary64.
- **Precision ladder.**
  - ρ's enclosures are tried at 53 bits (binary64-guided and checked, with ends rounded outward to
    64 bits so the CQC inner sums accumulate exactly without allocation), then 128, 256, 512 and
    1,024 bits, from exact integer division and square roots.
  - A row moves up the ladder only while it is not certified. A row not certified at 1,024 bits
    refuses the request.
  - Scaling a row by a power of two leaves its conditioning unchanged. A large result can still be
    refused (for example the scaled R2 below), and a tiny one certified.

What is refused, with witnesses from review round 1 (`tests/represented_input_oracle.rs`):

| Witness | Exact value | Outcome |
|---|---|---|
| 100-40-40 of three 2^-1074 | 9/5 = 1.8 quanta | refused: the nearest binary64 (2 quanta) is 1/9 away |
| 100-40-40 of [2, 1, 1]·2^-1074 | 14/5 = 2.8 quanta | refused: 3 quanta is 1/14 away |
| CQC, f_i = 1 + i·2^-19 Hz, R_k = (−1)^k C(39, k) N, ζ = 0.2, n = 40 | below 1e-150 N | refused: the radicand's enclosure still reaches zero at 1,024 bits |
| The same family, n = 20 (about 605 bits under the pricing model) | 1.7888529003e-79 N | certified |
| The same family, n = 7, scaled by 2^90 | 3.2174063837e-1 N | certified |
| [1e16, 1, −1e16] in one cluster, every modal rule | 1 | certified, exactly 1 |

### Cost and the cap

The `#[ignore]`d scenarios in `tests/combination_cost.rs` were measured in release, one process
each through the host's slot wrapper, on 15 November 2026. Logs are kept outside Git. The
baseline is the binary64 combinations of 42aeb90ba.

| Scenario | Binary64 | Certified | Peak RSS |
|---|---|---|---|
| Frozen references: 78 combinations, 28,626 magnitudes | 8.8 ms | 55 ms | 12.7 MB |
| Many-mode CQC, 30 / 60 / 120 clusters × 300 rows × 3 directions | | 19 / 43 / 111 ms | 5.5 / 8.0 / 13.3 MB |
| Many-mode CQC, 120 clusters × 1,200 rows × 3 directions | 18.8 ms | 380 ms | 37.8 MB (binary64: 11.5 MB) |
| Many-mode SRSS, 120 clusters × 1,200 rows × 3 directions | 1.8 ms | 75 ms | 36.2 MB |
| Demanding refinement, n = 20: 1 row / 50 rows | | 4.5 ms / 18 ms | 3.4 / 3.6 MB |
| Refusal path, n = 40: 1 row / 50 rows | | 17 ms / 65 ms | 4.3 / 4.6 MB |
| Refusal path, n = 40 among 120 clusters, 10 rows | | 177 ms | 12.3 MB |
| The same with a 2,048-bit level added (scratch build): n = 40 is then certified | | 341 ms | 17.7 MB |

**How cost scales.**
- Ordinary work grows with clusters² × rows × directions for CQC, and with clusters × rows ×
  directions for the other rules.
- Escalation is dominated by one ρ table per level, which grows with clusters² × p².
- Memory holds the exact cluster sums (clusters × rows × directions) and one ρ table
  (clusters² × p bits).

**Why the cap is 1,024 bits.**
- It certifies the evenly spaced alternating-binomial family up to about n = 36 at ζ = 0.2 (the
  pricing model needs about 1,020 bits there), including every witness of the plan.
- A refused row then costs tens of milliseconds, and a hundred-cluster request well under a second.
- A 2,048-bit level would certify about 40 more alternating clusters, at about twice the
  worst-case refusal cost and memory. The precision cap does not by itself bound work or memory;
  the measurements above do.

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
