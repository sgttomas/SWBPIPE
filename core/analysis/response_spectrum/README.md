# response_spectrum (DEL-04-10, units S2, S4 and the spectrum result document)

This crate holds two units of DEL-04-10's response-spectrum analysis
(`execution/PKG-04/DEL-04-10/ScopeOfWork.md`):

- **S2** admits a user-entered spectrum table and evaluates its pseudo-spectral acceleration at a
  query frequency.
- **S4** combines supplied signed contributions by the modal, rigid-response and directional rules,
  and fixes the signed-contribution contract
  ([Combinations (unit S4)](#combinations-unit-s4)).
- **The document** (`document`) assembles an `openpipestress.spectrum_result` document
  (REQ-006) from a combination request, its bindings, the modal basis as used and the census
  ([The spectrum result document](#the-spectrum-result-document)).

The crate is not a spectrum solve. It forms no modal response or rigid response. Every committed
document is constructed from a frozen reference's published contributions and says so.

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
- After location, the arithmetic is binary64 on significands with exact exponent bookkeeping, so
  no intermediate overflows or underflows. The two weights are formed directly as positive
  quotients (never as 1 − t), a linear ordinate is w₀·y₀ + w₁·y₁, and a logarithmic ordinate is
  reached from the knot with the larger weight. `SpectrumTable::evaluate` documents each formula.

## Accuracy of each published value

The interpolated ordinate, S and s·S each carry a relative error bound conditional on IEEE 754 round to
nearest and the unchecked libm assumption below
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
  platform's `ln`, `ln_1p` and `exp` are each within κ = 2⁻⁵⁰ relative of the exact value. This
  is assumed, not checked, for Apple's libm on macOS, glibc on Linux and the Microsoft C runtime on
  Windows. The inspected platform documentation establishes no universal guarantee of this
  constant. Downstream consumers must retain this qualification beside any accuracy claim. A
  supported platform found to violate the constant reopens the premise and affected claims,
  even if final-result comparisons still pass.
- **What is tested, and is not a proof.**
  - The frozen F2 references.
  - The first review's ten probes.
  - Published values either side of the subnormal threshold and of 2^1024.
  - 240 seeded adversarial tables over the eight axis and rule combinations, each compared with
    150-digit expectations from `tests/oracle/generate.py`.

  The maintained higher precision sweep tests final results on the configuration that executes
  it. Record the source, Rust target/compiler, OS and math runtime with each local or hosted run;
  only configurations actually exercised by that run have evidence. Passing it neither directly
  checks nor establishes the primitive-function premise. The separate direct premise check has
  not been established. The selected corpus also checks today's conservative refusal policy.
- **Refused.** This implemented argument does not establish 1e-9 relative accuracy under its
  assumptions. It may refuse a representable accurate answer: an exact `1e-320` knot is charged
  the worst-case half-subnormal spacing; a constant `f64::MAX` table can have a computed blend
  that rounds above the finite range. Neither proves that the exact result is unrepresentable.
  A future argument may justifiably publish these cases. LIM-024 states this refusal policy.

### Refusals

| Code | When |
|---|---|
| `SPECTRUM_RECORD_INVALID` | The record breaks a §2.4 rule. The refusal names the field. |
| `SPECTRUM_DAMPING_OUT_OF_RANGE` | A valid record has ζ above 0.2. |
| `SPECTRUM_DAMPING_MISMATCH` | The spectra of one request have different damping ratios (`common_damping_ratio`). |
| `SPECTRUM_FREQUENCY_OUTSIDE_RANGE` | The query is outside the tabulated range. |
| `SPECTRUM_QUERY_FREQUENCY_INVALID` | The query is not a positive finite frequency in a catalog unit. |
| `SPECTRUM_SCALE_FACTOR_INVALID` | The scale factor is not positive and finite. |
| `SPECTRUM_EVALUATION_ACCURACY_UNSUPPORTED` | This implemented argument does not establish 1e-9 relative accuracy under its stated assumptions. |

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
- **The rows** (`contract::Quantity`), REQ-006's roster, each with the unit, frame and sign
  convention its kind fixes:
  - node displacements and rotations in global axes (m, rad);
  - straight member end actions on the element at end i or j, in global or member axes (N, N*m);
  - arc end actions in global axes or the arc's tangent frame at that end (member axes are refused);
  - connector generalized forces in connector axes, and connector end actions in global axes;
  - each tie rod's axial force along its axis, tension positive, identified by connector and rod;
  - rigid constraint reactions by node and component, naming every holder, in global axes;
  - compliant support forces by support and component, in global axes.

  A row's identity is its fields (`Quantity::identity`): any non-blank identity is accepted, and
  a duplicate is refused structurally. `Quantity::label` composes a display label only; no reader
  parses it. There is no row along a skew support's axis.
- **The signed contributions.** One per mode of a periodic cluster and requested direction, already
  scaled by the direction's factor. A contribution for a mode of a rigid cluster is refused.
- **The rigid responses**, when the correction is applied: one signed response per direction,
  stating the clusters its load left out as periodic. A mismatch with the request's classification
  is refused.

### What it does (CLM-001)

1. **Classifies.** A cluster is periodic when its lowest frequency is below f_r, and every cluster is
   periodic without the correction.
2. **Sums each periodic cluster's contributions with their signs** in ascending mode order. The sums
   of clusters of more than one mode are published with their sign tag.
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
    64 bits so the CQC inner sums accumulate exactly without allocation), then 128, 256, 512,
    1,024 and 2,048 bits, from exact integer division and square roots.
  - A row moves up the ladder only while it is not certified. A row not certified at 2,048 bits
    refuses the request.
  - Scaling a row by a power of two leaves its conditioning unchanged. A large result can still be
    refused (for example the scaled R2 below), and a tiny one certified.

**Field semantics for readers** (released by Agent 0): published cluster sums are the correctly
rounded binary64 of exact sums while combinations use the retained exact sums; the published
correlation is informational, one coefficient per unordered pair of distinct periodic clusters, the
public `rules::correlation_coefficient` value, while combined magnitudes come from
the enclosure at the level required; a reader derives its arithmetic from the original represented
signed contributions, frequencies, damping and method, and never from the rounded display values.

**Accuracy predicates.** Every published magnitude passes all of these:
- the final conversion to binary64 is inside the check: the published v itself is tested against
  the enclosure;
- a zero is published only when exact arithmetic proves it, as an enclosure that is exactly [0, 0];
- 100-40-40 uses the exact 2/5;
- CQC takes a square root only of a radicand enclosure with a positive lower end.

What is refused, with witnesses from review round 1 (`tests/represented_input_oracle.rs`):

| Witness | Exact value | Outcome |
|---|---|---|
| 100-40-40 of three 2^-1074 | 9/5 = 1.8 quanta | `SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED`; analytically, the nearest binary64 (2 quanta) is 1/9 away |
| 100-40-40 of [2, 1, 1]·2^-1074 | 14/5 = 2.8 quanta | `SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED`; analytically, 3 quanta is 1/14 away |
| CQC, f_i = 1 + i·2^-19 Hz, R_k = (−1)^k C(39, k), ζ = 0.2, n = 40 | 9.71048724966594e-151 | published at 2,048 bits; every lower rung fails to establish accuracy |
| CQC, 88 exact alternating binomial cluster sums, each scaled by 2^512 and split into two exact binary64 contributions | 1.12409512915645e-151 (independent Decimal check) | `SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED`: the radicand enclosure still reaches zero at the 2,048-bit cap |
| The same family, n = 20 | 1.7888529003e-79 N | certified |
| The same family, n = 7, scaled by 2^90 | 3.2174063837e-1 N | certified |
| [1e16, 1, −1e16] in one cluster, every modal rule | 1 | certified, exactly 1 |

### Cost and the cap

**Observed comparison, 16 November 2026.** Apple M5 Max (18 cores, 128 GiB), macOS
27.2 (26B123), rustc/cargo 1.97.1, locked Cargo release builds. Source `4c5e1b058` already
contains landed S2 and W1. The comparison build adds only the 2,048-bit rung to that source.
Each maintained cost scenario runs in its own process through the host slot wrapper; times
are the workload timer excluding compilation, and memory is process peak RSS in decimal MB.
This was a shared host, with no exclusive window or release-baseline qualification. Raw commands,
build identity and logs are retained outside Git for the open review. These replace the earlier
unauthenticated historical timing account.

| Scenario | 1,024-bit cap: time / peak RSS | 2,048-bit cap: time / peak RSS |
|---|---|---|
| Frozen references, 78 combinations / 28,626 magnitudes | 52.4 ms / 12.76 MB | 62.8 ms / 12.75 MB |
| CQC, 30 clusters × 300 rows × 3 directions | 18.4 ms / 5.52 MB | 22.6 ms / 5.51 MB |
| CQC, 120 clusters × 1,200 rows × 3 directions | 351.5 ms / 37.85 MB | 466.1 ms / 37.86 MB |
| SRSS, 120 clusters × 1,200 rows × 3 directions | 74.7 ms / 36.24 MB | 87.8 ms / 36.24 MB |
| Demanding n = 20, 1 row | 4.25 ms / 3.47 MB, published | 4.58 ms / 3.54 MB, published |
| Demanding n = 20, 50 rows | 15.3 ms / 3.57 MB, published | 17.8 ms / 3.77 MB, published |
| n = 40, 10 rows | 27.1 ms / 4.47 MB, refused | 46.7 ms / 5.29 MB, published |
| n = 40, 50 rows | 65.3 ms / 4.65 MB, refused | 116.5 ms / 5.41 MB, published |
| n = 40 among 120 clusters, 10 rows | 180.1 ms / 12.21 MB, refused | 326.1 ms / 17.76 MB, published |

These are single-run observations, not bounds. Ordinary-case timing differences cannot be
attributed to an unused extra rung. Separate instrumented runs observed the maintained suite
reaching 1,024 bits and the 40-among-120 probe reaching 2,048 bits; the latter then published.
The maintained tests now check the 40-term case at every lower rung and at 2,048, compare its
published value to the independent represented-input oracle, and require a genuine cap refusal
for 88 exact binomial cluster sums. The rounded 80-term cost workload publishes and is named as
such; it is not a refusal witness or evidence of monotonic growth in required precision.

**Work and retained workspace.** These describe the implemented loops and the measured
scenarios, not a bound on time or memory. Integer-operation cost also depends on the actual
intermediate bit lengths.
- With C clusters, R response rows and D directions, CQC evaluates all C(C−1)/2 pairs per
  row and direction. The measured SRSS scenarios instead traverse C terms per row and
  direction; absolute sum also traverses C terms but has no separate timing scenario here.
- Grouping and ten percent retain P qualifying pairs and evaluate them for every row and
  direction, in addition to C diagonal terms. P can be C(C−1)/2: 120 mutually close clusters
  produce 7,140 pairs. Ten-percent preparation tests every pair even when none qualifies.
  The SRSS measurements do not measure these pair-rule costs.
- CQC escalation builds a ρ table for each attempted precision level. Every such table is
  retained until the request finishes, along with the exact cluster sums (C × R × D) and
  exact rigid responses. Grouping and ten percent retain their P pairs instead.
- The precision cap limits the coefficient refinement levels. Stored endpoint integers can
  be wider because exact accumulation aligns intermediates of different exponents; the
  actual spans include damping, the coefficient kernel and cluster products. Neither the
  precision cap nor a two-factor binary64 exponent span bounds this storage. No storage
  bound is claimed. The reader observed a 5,356-bit radicand endpoint at the 53-bit level for
  frequencies [2^-1074, f64::MAX], damping 2^-1074 and contributions [1, 1].

**Why the cap is 2,048 bits.** Agent 0 chose this rung on 16 November: it resolves a demanding
case that 1,024 refuses, at the observed extra time and memory above. The policy is to publish
what the runtime enclosure establishes and refuse otherwise. The cap is a reversible choice
about how far `PRECISION_LADDER` climbs, not an accuracy, work or memory bound. The actual
enclosure, including final binary64 conversion, still decides every publication and exact zero.

### JSON entry and example

`combination_json` reads and writes the unit's documents
(`openpipestress.spectrum_combination.input/v2` and `output/v2`; not `spectrum-1`). Each row is
stated by its identity fields and sign convention; v1's composed `key` is refused. The
`combine_contributions` example reads an input document on standard input and prints the output
document:

```sh
cargo run --manifest-path core/analysis/response_spectrum/Cargo.toml --example combine_contributions \
  < core/analysis/response_spectrum/fixtures/combination/c4_base.input.json
```

`fixtures/combination/README.md` lists the contract fixtures.

## The spectrum result document

`document::spectrum_result_document` is a pure function from `DocumentInputs` to a
`openpipestress.spectrum_result` document (profile `product_preview_spectrum_v1`, semantic table
`openpipestress.result_semantics/spectrum-1`, canonical profile `openpipestress_jcs_ijson_v1`).

- **Bindings.** The model document and the spectrum request are each bound by the SHA-256 of their
  canonical text, after an iterative check of their depth. A modal result is bound by its run id and
  its own binding digests (request, materials and integrity parameters, the model digest where one
  is supplied, and a state case where one is named), not by a digest of the whole modal document;
  the run id is checked against the modal family's derivation from the request and
  integrity-parameter digests. A document built without a modal result states the binding as not
  applicable, with its reason.
- **Formation records.** The signed contributions enter as `SignedContributions`, which carries for
  each requested direction the spectrum identity, the digest of that spectrum's record in the model
  that supplied the table, and the scale factor the contributions were formed with. The assembler
  refuses a direction whose record is missing or repeated, or is not the bound request's spectrum
  and scale factor (bit for bit) and the bound model's record. This establishes which choices and
  which record the producer was told the contributions were formed with, and that they are the
  bound ones; it does not establish that the contributions were formed correctly.
- **What it states.** The experimental admission and origin, the request choices as read, the
  normalized spectra, the modal basis as stated to it (modes with frequencies and circular
  frequencies, clusters, cluster tolerance, standings, state, extraction endpoint), the standing,
  the disclosure operands (each frequency's location in each spectrum, the extraction premise, the
  missing-mass masses and fractions with their stated arithmetic), the census as supplied (each
  rigid constraint with `held`: `single` or `jointly_undivided`), the signed rows with their sign
  conventions, the signed cluster sums, the unsigned combined rows and the rule statements,
  including the closeness rule of the grouping and ten percent methods. The semantic table's
  `document_shape` declares every member of every object, its type and when it may be null.
- **Stages.** The assembler checks, in order, stopping at the first stage that refuses: the
  bindings; the inputs against one another (request, spectra, combination input, formation records,
  modal statement, census, origin); the rows' own fields (`SPECTRUM_ROW_INVALID`), the roster and
  the restrained components; the modes' structure (`SPECTRUM_MODAL_METADATA_INVALID`), the range and
  the extraction premise; the combinations; the published numbers; and the document as a whole.
- **What it refuses**, each as `SPECTRUM_DOCUMENT_INPUT_INVALID` unless named otherwise:
  - a combination input whose methods, missing-mass fields or directions are not the request's, or
    whose damping ratio is not the spectra's;
  - a requested direction with a formation record missing, repeated, or not the request's spectrum
    and scale factor and the model's record; a model that holds more than one spectrum of an
    identity a requested direction names;
  - a modal statement that does not state every extracted mode;
  - an extraction statement whose endpoint, counts and next computed mode disagree: the whole finite
    spectrum without an established count equal to the reported count, or with a next computed
    mode; below a computed cluster without a reported mode or without both next-frequency values,
    or with the next frequency below the last reported one; below the lowest mode with a reported
    mode or without both next-frequency values; an established count below the reported count, or
    not above it where a next computed mode is stated; blank completion evidence;
  - a bound modal result whose digests are not 64 lowercase hexadecimal digits, whose run id is not
    the modal family's, or whose state case has a blank id; an unbound one with a blank reason;
  - a total or effective mass that is negative, or an effective mass that is not zero where the
    total is zero (a nonfinite one is `SPECTRUM_DOCUMENT_OUTSIDE_CANONICAL_PROFILE`);
  - stated clusters that contradict the cluster rule on the stated circular frequencies and
    cluster tolerance beyond the modal reader's margin (16u(1 + δ_c)·ω_b², in binary64 with the
    modal reader's expressions), including between the last extracted mode and the next computed
    one;
  - a circular frequency w with |w − c·f| > 8u·w, where c = 2.0·π in binary64;
  - a census frame whose rows are not orthonormal and right-handed to 2^-40, or a rod axis that is
    not a unit vector to 2^-40;
  - an entity that occurs twice in its census table (compared by its identity fields), or a rigid
    constraint with no holder or with a holder named twice;
  - a census that names a node or connector it does not hold;
  - a modal result binding that does not match the origin;
  - a row the roster does not give, a roster row with no row, a row supplied twice, or a row whose
    member family or holder set is not the census's (the holders' order is not compared; the
    document states them in the census's order);
  - a node displacement or rotation row whose component a census rigid constraint restrains at
    that node, with a nonzero contribution or rigid value;
  - a periodic frequency or f_r outside a spectrum's range, and an applied correction whose
    extraction premise fails (`SPECTRUM_EXTRACTION_BELOW_RIGID_FREQUENCY`);
  - a model, request or modal state too deep for the checked parse at its place, an integer of
    2^53 or more, a number passed to the document's number encoder that is nonfinite or of
    magnitude 2^53 or more, or an assembled document outside the profile
    (`SPECTRUM_DOCUMENT_OUTSIDE_CANONICAL_PROFILE`);
  - anything the combinations refuse.

  The `refused` document carries the refusals of the first stage that refused.
- **Allocation** (by reading the code; not measured) is stated in the module documentation of
  `src/document.rs`: what the document holds, what the checks before the combinations and the
  projection hold (ordered maps and sets, the checksum arrays, the three texts held while hashing,
  the row copies), and the time of the per-cell lookups and the knot search. The ten percent
  method's coupled pairs can be quadratic in the periodic clusters.

`fixtures/results/semantic_contract_spectrum_1.json` is the semantic table
(`document::spectrum_semantic_table`). The committed documents `fixtures/results/spectrum_*.json`,
each beside its model stand-in and request, are generated by the ignored writer in
`tests/spectrum_documents.rs` and guarded byte for byte:

```sh
cargo test --manifest-path core/analysis/response_spectrum/Cargo.toml --test spectrum_documents \
  write_spectrum_documents -- --ignored
```
