# CP3 wire addendum: expansion-law data labels

This addendum supplements three files, all unchanged:

| File | sha256 |
|---|---|
| `CP2_WIRE.md` | `81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996` |
| `CP2_WIRE_ADDENDUM_1.md` | `c389f5e3878c6a32d8c72b80d1374ea6f30c4350c2870ab7ac653972df3dc760` |
| `CP2_WIRE_ADDENDUM_2.md` | `ec66628ef8db1ac70500b80abbcc754b82133fb8d97d4cf0f41ecd4a32ab9133` |

It fixes the method-dependent meaning of the law-data fields in each `members[]` record:

- `consumed_law_point_indices`
- `consumed_law_segments`
- `consulted_law_point_indices`
- `consulted_law_segments`

ADDENDUM_1 §3 left these open, and EXTENSION_1 test 7 found the gap. The method for `engineering_dilation` is a ROOT technical selection, made during CP3 on the manager's clarification. This records the implemented kernel (`core/product_physics/src/case_state/thermal.rs`, CP1-reviewed); nothing in the producer changes. Paths are WORKING_ROOT-relative.

## 1. Record vocabulary

**Point index.** The index of an authored table point, in the table's temperature order. A point enters a record in either of two ways:
- the law is evaluated exactly at that point's temperature;
- the point is an endpoint of a recorded segment.

**Segment entry.** Each entry has this shape:

```
{use, lower_index, upper_index, start_k, end_k}
```

- `upper_index` is always `lower_index + 1`.
- `start_k ≤ end_k`, and both lie in `[T_lower, T_upper]`.
- Entries are unique, and their order is evaluation order, which carries no meaning.

**`use: "interpolation_sample"`, with `start_k == end_k`.** This is the degenerate entry. The piecewise-linear table was evaluated at the single temperature `start_k`, which lies strictly inside the segment `(lower_index, upper_index)`. Both endpoint values of that segment entered the sample.
- An evaluation exactly at a table point's temperature is never a segment entry. It is recorded only as that point index.

**`use: "integration_interval"`, with `start_k < end_k`.** The segment's data entered over the sub-interval `[start_k, end_k]`, as a whole interval rather than at one temperature. That happens in one of two ways:
- by trapezoid integration of a coefficient table;
- by summing the segment's dilation increment over the sub-interval.

**Consumed.** The data entered a published value. That means the member's `thermal_strain`, or its published datum stretches `installation_datum_stretch` and `operating_datum_stretch`.

**Consulted.** The data was examined to establish coverage, positivity or the datum condition.

**Overlap.** Consumed and consulted are recorded independently and may overlap. A point or segment entry that entered the value can also appear in the consulted record because it was checked for admissibility there. Consulted is **not** "consulted minus consumed".

## 2. Per definition

Below, `T_i` is the installation temperature, `T` the operating temperature, `T_m` the law's datum, `lo = min(...)` and `hi = max(...)`. "Sample at X" means one of:
- a point index, when X equals a table point;
- an `interpolation_sample` entry at X, otherwise.

### `engineering_secant`, table data

**Consumed.**
- A sample at `T_i` and a sample at `T`. These are the two secant coefficients.
- Nothing is integrated, so there is never a consumed `integration_interval`.

**Consulted.**
- Coverage and positivity over `[min(T_i, T), max(T_i, T)]` only. The datum needs no coverage, because λ(T_m) = 1 by definition (ROOT's SF2 selection).
- Samples at `lo`, at `hi`, at `T_i` and `T`, at each table point inside the interval, and at any interior stationary temperature of the secant stretch.
- A segment that contains such a stationary candidate is recorded as a consulted `integration_interval` over the part of the segment inside the interval.

**Constant coefficient.** A constant coefficient consumes and consults no table data.

### `differential_per_datum_length` and `logarithmic_per_current_length`

**Consumed.** The value integrates the coefficient over three intervals:
- `[T_m, T_i]`, for the installation datum stretch;
- `[T_m, T]`, for the operating datum stretch;
- `[T_i, T]`, for the strain numerator.

Each integral records, for every segment piece `[a, b]` with `a < b`:
- one consumed `integration_interval` entry;
- samples at `a` and `b`.

An integral whose endpoints are equal records only a sample.

**Consulted.**
- Coverage over `[min(T_m, T_i, T), max(T_m, T_i, T)]`.
- Positivity of the path stretch at every candidate temperature: the interval ends, `T_m`, `T_i`, `T`, the table points inside, and interior stationary candidates. This is evaluated by consulted integrals and samples between the candidates.
- Segments with stationary candidates are recorded as consulted `integration_interval` entries.

### `engineering_dilation`

The strain is `Δd / λ_install`.
- `λ_install = 1 + d(T_i)`.
- `Δd = d(T) − d(T_i)` is evaluated by summing each spanned segment's increment over its piece of `[min(T_i, T), max(T_i, T)]`, not as the difference of two rounded samples. This is `dilation_difference`, and ROOT selected this method.
- In binary64, the table points inside that interval genuinely enter the value.

**Consumed.**
- A sample at `T_i` and a sample at `T`, for `λ_install` and the published `operating_datum_stretch`.
- One `integration_interval` entry for every segment piece `[a, b]`, with `a < b`, of `[min(T_i, T), max(T_i, T)]`. The endpoints of every spanned segment, interior table points included, are consumed points.
- If `T_i == T`, then `Δd = 0` and only the samples are consumed.

**Consulted.**
- The datum sample, which must be exactly zero dilation.
- Coverage over `[min(T_m, T_i, T), max(T_m, T_i, T)]`.
- Positivity of `1 + d` at every candidate: the interval ends, `T_m`, `T_i`, `T`, and the table points inside. These are recorded as samples.
- Linear dilation needs no stationary candidates, so there is never a consulted `integration_interval`.

### Direct definitions

`unchanged_reference`, `explicit_interval_strain` and `constant_alpha_interval` consume and consult no law data. All four fields are empty.

## 3. Examples

These are the EXTENSION_1 cases.

**Table:** points `(20 °C, 0)`, `(50 °C, 9/25000)`, `(150 °C, 13/6250)`. **Datum:** 20 °C. **Law:** `engineering_dilation`, with `T_i = 50 °C`.

**`T = 150 °C`:**
- Consumed points: `[1, 2]`.
- Consumed segments: `[{use: integration_interval, lower_index: 1, upper_index: 2, start_k: 323.15, end_k: 423.15}]`.
- Consulted points include the datum point `0`.

**`T = 100 °C`:**
- Consumed segments: `{interpolation_sample, 1–2, 373.15 → 373.15}`, the operating sample, and `{integration_interval, 1–2, 323.15 → 373.15}`, the increment.
- Consulted segments include the same `interpolation_sample`, from the positivity check at `T`. That is a permitted overlap.
