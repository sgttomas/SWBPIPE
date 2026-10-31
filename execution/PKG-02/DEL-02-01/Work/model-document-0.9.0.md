# Model document 0.9.0: operating-condition sets (C1), support direction and difference basis

**Basis:** main `ad03de4a0c` (2026-10-27). Design only; no product code.

**Status:** revision 4, after Codex design reviews r1 (on `22dc6edcbd`: no P1;
four P2s and the OC-3, OC-5 and OC-7 conditions), r2 (on `a87d8df6b8`: the
canonical decimal, the rounding width and the zero result) and r3 (on
`9bf77e607c`: "Ready for Agent 0", with two P3s), all resolved. Agent 0 decided
OC-1 to OC-9 and ND-1 under the owner's 2026-10-27 delegation (#1703), adopted
as recommended (§6). Revision 5 (2026-10-30) adds the members SWBPIPE Agent 0
joined before S1 (§2.4); its review is pending. Design only; no product code yet.
Survey item C1 (operating-condition sets) with the solver lane's two 0.9.0
members. Risk class: persistence and numerical meaning, so every build slice
takes targeted regression checks and independent scrutiny by a separate agent
(AGENTS.md, Verification and authority).

**Lineage:** it follows the 0.7.0 → 0.8.0 step of
[`model-document-0.8.0.md`](model-document-0.8.0.md) (W-15 slice c, #1649) and
the format rules of [`model-document-0.7.0.md`](model-document-0.7.0.md). The
`direction` and `difference_basis` members carry Agent 0's decisions on the
solver lane's design note (operating case and skewed restraints, revision 2,
§0 R-7 and "Shape", §1.4, §2.5). The run-sheet column answers DEL-07-02 Design
[`run-sheet-entry.md`](../../../PKG-07/DEL-07-02/Work/run-sheet-entry.md) §8 O7.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Paths are
relative to `projects/chirality-piping/`. Nothing here records or implies
acceptance, issuance or release; those are the owner's acts.

## 0. Rulings this design applies

- **One current version per format** (owner, 2026-10-10: "nothing needs to be
  preserved for backwards compatibility … one current version per format"), and
  **OD-4** (owner, 2026-10-23): new slots arrive in a new document version.
- **Owner, 2026-10-26, "0.8.0 a2"**, as DEL-02-05 CLM-003 states it for any
  retired version: projects and saved states holding a retired version are
  refused by name; recorded runs keep their format and their read and compare
  behaviour; no model-document-version gate is added for them.
- **Agent 0, 2026-10-27, R-7** (`supports[].direction`), carried exactly in §2.1.
- **Agent 0, 2026-10-27, "Shape"** (`combinations[].difference_basis`, gates G-1
  and G-2, EX refusal), carried exactly in §2.2. Its "`subtrahend_support_state`
  not adopted" is superseded by Agent 0's 2026-10-30 decision below.
- **Agent 0, B1 O7:** the run sheet's operating-condition column is deferred to
  C1, and C1 carries it (§4.1).
- **Agent 0, 2026-10-27, #1703** (owner's delegation): OC-1 to OC-9 and ND-1
  adopted as recommended, with the notes on OC-4, OC-5 and OC-7 (§6).
- **SWBPIPE Agent 0, 2026-10-30, on the load-case plan:** `absolute_sum` and
  `srss`, combinations taking earlier combinations as inputs (acyclic), the
  hydrotest test-condition member and `subtrahend_support_state` (`own |
  minuend_final`) join 0.9.0 before S1, each refused by name until its slice
  lands; the dynamics lane's snubber family and spectra records join (§2.4).
  Agent 0 froze the list on 2026-10-31.
- **Brief for this design (Agent 0 through the manager):** all 0.9.0 members in
  one version step; `load_cases[].kind` token `operating` is desktop vocabulary
  with no schema change; the element assignment is preferably stored explicitly.

PRD anchors: §12.5 (temperature and thermal expansion, internal pressure
metadata, contents weight, wind load), §12.6 (primitive cases are solved;
combinations are algebra; no shipped code combinations) and FR-SOL-006 (weight,
thermal, pressure metadata … can be defined and solved).

## 1. The C1 members

### 1.1 What the model carries today

| Quantity | Today's carrier | Scope |
|---|---|---|
| Temperature, pressure-free (PF) route | a `thermal` primitive on a pipe, dimension `temperature_change` (ΔT), read by `build_thermal_element_loads` (`core/product_physics/src/lib.rs` ~11986) | per case, per pipe |
| Temperature, exact (EX) route | `analysis_state.element_states[].operating_temperature` (absolute), with the installation temperature in `reference_configurations` | per case, per pipe |
| Elastic basis | `load_cases[].modulus_basis_ref` or `modulus_basis_temperature` | per case, one value |
| Pressure | `load_cases[].pressure_regions[].pressure`, only under `pressure_contract`; pressure primitives are retired | per case, per region |
| Contents density | `pipe_segments[].section.contents_density`, read by seismic mass (`compute_pipe_mass_per_length`) and by self-weight generation | per pipe, case-independent |
| Added weight | `weight` primitives (`force_per_length`) on pipes | per case, per pipe |
| Wind exposure | `equivalent_static.wind.exposed_pipe_refs` / `exposed_spans` | per case |

A real system with several temperatures per line and several operating modes
must today repeat every pipe's ΔT in every case. C1 states each line's values
once, in a named set, and lets a case say "condition k".

**Principle.** A case's use of a condition resolves into exactly the per-case
records the solve already consumes. No new mechanics enters: the condition set is
a source of values, and resolution is a lowering of the document onto today's
records.

### 1.2 Members

All members are optional. Absence means none, and an authored `null` is refused,
not read as absence (the tolerance precedent, 0.7.0 §3.8). Nothing is defaulted:
the product supplies no temperature, pressure, density, weight, exposure or
condition, and creates no set unprompted.

**Top level: `condition_sets[]`**, each a closed `ConditionSet`:

| Member | Type | Required | Meaning |
|---|---|---|---|
| `id` | `Id` | yes | Unique among condition sets (`DUPLICATE_ID`, `EMPTY_ID`, entity `condition-set`) |
| `label` | string | no | Display |
| `conditions` | array of `OperatingCondition` | no | The set's keyed conditions; absent or empty means none |
| `installation_temperature` | `Quantity`, absolute temperature | no | The stress-free temperature the PF route subtracts (§1.5) |
| `design` | `ConditionValues` | no | Design temperature and pressure; read by no solve in 0.9.0 |
| `peak` | `ConditionValues` | no | Peak temperature and pressure; read by no solve in 0.9.0 |
| `added_weight` | `AddedWeight` | no | A uniform added weight on every pipe carrying the set |
| `exposure` | `ConditionExposure` | no | Exposure marks |
| `provenance` | string, non-blank | yes | Source of the set's values |
| `tags`, `notes` | as 0.7.0 | no | Unread annotations |

**`OperatingCondition`** (closed): `key` (`Id`, required, unique within its set);
`label` (string); `temperature` (`Quantity`, absolute temperature); `pressure`
(`Quantity`, internal differential pressure, the `internal_differential_zero_external_v1`
basis of pressure regions); `contents_density` (`Quantity`, density).

**`ConditionValues`** (closed): `temperature`, `pressure`, both optional `Quantity`.

**`AddedWeight`** (closed): `magnitude` (`Quantity`, `force_per_length`) and
`direction` (`global_x` | `global_y` | `global_z`), both required. These are the
fields a user types into a weight primitive; the model has no vertical axis, so
the direction is entered, never assumed.

**`ConditionExposure`** (closed): `wind`, `snow_ice`, `wave`, each optional
boolean. Absent means not marked, as a pipe absent from `exposed_pipe_refs` is
not exposed today. `snow_ice` and `wave` are read by no solve in 0.9.0; the
product has no generator for them.

**Members carried but not used by the solve are labelled as such.** In 0.9.0
these are `design`, `peak`, `exposure.snow_ice`, `exposure.wave` and each
condition's `contents_density` (until its consumer lands, §1.6). Wherever one is
shown, it carries the visible statement "not used by the solve in 0.9.0": in
the desktop's tables, inspector and load manager, and in any report, export or
handoff view that displays it. An exporter that writes the document verbatim
needs no label, because it displays nothing. The label is removed for a member
only in the slice that makes the solve read it.

**Element assignment: `pipe_segments[].condition_set_ref`** (`Id`), naming one
set.

**Case reference: `load_cases[].condition_use`**, a closed `ConditionUse` with at
least one member:

| Member | Type | Meaning |
|---|---|---|
| `temperature` | `Id` (a condition key) | Each pipe's temperature comes from condition `key` of its set |
| `pressure` | `Id` (a condition key) | Each region's pressure comes from condition `key` of its member pipes' sets (EX only) |
| `contents_density` | `Id` (a condition key) | Reserved consumer (§1.6); refused at solve in 0.9.0 |
| `added_weight` | `true` only | Each pipe's set's `added_weight` joins the case |
| `wind_exposure` | `true` only | The case's wind marks come from the sets' `exposure.wind` |

`added_weight` and `wind_exposure` admit only `true`; `false` is refused, so the
record has one form for "not used": absence. Keys are strings, not positions
(REQ-02-01-03), and one key may name the same mode across sets ("OPE1",
"HYDRO"); the product does not interpret a key.

### 1.3 Admission and refusal codes

The applier's document validator (`document_validator.rs`) and the solve give the
same code and message for document-level rules (the `shear_model` precedent,
0.8.0 §1.1). Malformed shapes take the existing closed-record paths
(`MODEL-SLOT-SHAPE` at the validator; `model_fields` at the solve wire).

**Document level (validator and solve, blocking):**
- `OPERATING_CONDITION_KEY_DUPLICATE`: two conditions in one set share a key.
- `OPERATING_CONDITION_QUANTITY_INVALID`: a value is non-finite, its unit is not
  in the units catalog or has the wrong dimension, an absolute temperature is
  below zero kelvin (`case_state/temperature.rs` `exact_kelvin`'s rule), or a
  contents density is negative (the `section.contents_density` rule). The message
  names the JSON path and the reason.
- `OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED`: an absolute temperature
  in a set lies outside the exact-arithmetic range of §1.5.1.
- `OPERATING_CONDITION_PROVENANCE_MISSING`: blank set provenance.
- `OPERATING_CONDITION_SET_UNRESOLVED`: a `condition_set_ref` names no set.
- `LOAD_CASE_CONDITION_USE_EMPTY`: an empty `condition_use`, or `false` in a
  `true`-only member.

Every set and reference is judged whether or not a case uses it, so a document's
admissibility does not depend on which cases use what.

**Case level (solve, blocking; the case names the pipe, set, key and quantity):**
- `OPERATING_CONDITION_SET_MISSING`: the case uses a quantity and a pipe carries
  no set.
- `OPERATING_CONDITION_KEY_MISSING`: the pipe's set has no condition `key`.
- `OPERATING_CONDITION_VALUE_MISSING`: condition `key` lacks the used quantity.
- `OPERATING_CONDITION_INSTALLATION_TEMPERATURE_MISSING`: PF temperature use on
  a pipe whose set has no `installation_temperature`.
- `LOAD_CASE_CONDITION_TEMPERATURE_CONFLICT`: the case uses `temperature` and
  carries a `thermal` primitive (§1.7).
- `LOAD_CASE_CONDITION_WIND_EXPOSURE_CONFLICT`: the case uses `wind_exposure`
  and authors `exposed_pipe_refs` or `exposed_spans`.
- `LOAD_CASE_CONDITION_WIND_INPUT_MISSING`: the case uses `wind_exposure` and has
  no `equivalent_static.wind`.
- `LOAD_CASE_CONDITION_PRESSURE_WITHOUT_PRESSURE_CONTRACT`: `pressure` on a
  pressure-free document, which has no pressure carrier.
- `LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED` (interim, §1.6).
- `LOAD_CASE_CONDITION_USE_EXACT_UNSUPPORTED` (interim until S4): any
  `condition_use` on a document with `pressure_contract`.
- `LOAD_CASE_CONDITION_USE_UNSUPPORTED` (interim, S1 only; S2 removes it): any
  `condition_use` before the PF resolution lands.

A pipe with no set is never skipped and a missing value is never zero: the case
is refused by name (OC-8).

### 1.4 Assignment and inheritance: stored explicitly

The document stores each pipe segment's `condition_set_ref`. Inheritance "down
the run, like material and section" is an authoring-tool behaviour (the run sheet,
bulk change, draw tools), which writes explicit references through reviewed
operations. The solve never walks topology to infer a set.

Why (OC-2):
- **The document says what each element carries.** Material and `section_ref`
  are already stored per pipe, and the run sheet already inherits them at
  authoring (`run-sheet-entry.md` §2.1).
- **Read-time inheritance has no defined rule.** At a tee, a loop or a branch
  started from a node (`F` rows), "the run" is ambiguous; `lines[]` are
  unevaluated records (0.7.0 §3.2) and would have to become evaluated.
- **No silent spread.** An upstream edit never changes a downstream element's
  conditions; each changed element is an operation in the review queue, and the
  model hash moves with what actually changed.

Cost: re-assigning a whole line is a bulk change over its pipes (B2), and the
document carries one reference per pipe.

**Operations that create a pipe from a pipe carry the reference.** Geometry
operations admit a pipe's optional keys by list (`geometry_operations.rs` :146:
`label`, `provenance`, `y_reference`, `section_ref`, `tags`, `notes`), so S1 adds
`condition_set_ref` and each of `split_pipe_run`, `transform_pipe_run` (copy),
`insert_bend_at_corner` and `reconnect_pipe_run` either carries it from its source
or refuses by name; one test each. `insert_bend_at_corner` requires both straights
to carry the same set or none (as it requires one section and material) and gives
the arc that set; a mismatch is `OP-BEND-CONDITION-SET-MISMATCH`.

### 1.5 Resolution at solve (the lowering)

**Where.** One function, `operating_conditions::lower(&mut model, &mut
diagnostics)` in `core/product_physics`, runs first in
`run_linear_static_preview_observed_inner` after the version and document-kind
checks, before `pressure_runtime::validate_profile`. Everything after it is
today's pipeline, unchanged, applied to the lowered model.

**What it does, per case with `condition_use`, in this order:**

1. **Temperature (PF).** For each pipe in `pipe_segments` order: find its set,
   condition `key`, the condition's `temperature` T and the set's
   `installation_temperature` T₀ (refusals of §1.3). Form

   ΔT = RN( K(T) − K(T₀) )

   where K is the exact rational kelvin value of the shortest round-trip decimal
   of the authored binary64 under the unit's exact affine definition, and RN
   rounds the exact difference once to binary64, to nearest, ties to even, with
   no binary64 intermediate. The arithmetic is defined on integers below
   (§1.5.1). Append the primitive
   `{id, category: "thermal", target: {type: "element", pipe}, direction: "global_z", magnitude: {value: ΔT, unit: "K"}, dimension: "temperature_change", provenance}`.
   The thermal direction does not change the thermal magnitude or sign (the
   existing test `thermal_load_direction_does_not_change_thermal_magnitude_or_sign`);
   the lowering fixes it to the token the desktop writes for thermal primitives
   (`primitiveLoadVocabulary.ts`) so the twin is unique.
2. **Added weight.** For each pipe in `pipe_segments` order whose set carries
   `added_weight`: append `{id, category: "weight", target: pipe, direction,
   magnitude, dimension: "force_per_length", provenance}`, with `magnitude` and
   `direction` copied verbatim. A set without `added_weight` contributes none
   (absence means none; it is not a value).
3. **Wind exposure.** Set the case's `equivalent_static.wind.exposed_pipe_refs` to
   the pipes, in `pipe_segments` order, whose set has `exposure.wind: true`. An
   empty result meets the existing `EQUIVALENT_STATIC_INPUT_MISSING` finding.

#### 1.5.1 The exact temperature difference: range and algorithm

This is normative, so that Rust (production), Python (the E3 reference) and
TypeScript (with `BigInt`, should it ever compute ΔT; the desktop uses the wasm
lowering) give the same bits. No step uses a language's float formatter.

- **Width.** All arithmetic is exact on unsigned integers of at least 256 bits:
  in Rust a fixed 256-bit type on the stack (no heap allocation, no new
  arbitrary-precision dependency), in Python `int`, in TypeScript `BigInt`.
  Every intermediate below is bounded under 2^192, so 256 bits leaves margin.
- **The operand's decimal (the canonical decimal).** Why a decimal and not the
  binary value itself: the user types a decimal, and the binary value of 20.3
  minus that of 20.1 is 0.19999999999999928…, not 0.2. Every decimal with at
  most 15 significant digits is recovered exactly by the rule below (binary64
  keeps 15 decimal digits), so for typed values the operand is what the user
  typed, whatever the tie rule. The tie rule matters only for 16- or 17-digit
  values (the regression's 7814407348632812 has 16), and it is fixed here because Rust's formatter and Python's or JavaScript's
  disagree on them.
  1. **Binary pre-gate.** v = 0 gives the decimal 0. Otherwise v must satisfy
     2^−80 ≤ |v| ≤ 2^20 (outside it,
     `OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED`). Write
     |v| = B·2^E exactly, with B ∈ [2^52, 2^53), so −132 ≤ E ≤ −32 (every
     admitted value is normal).
  2. **Decade.** p is the largest integer with 10^p ≤ |v|, found by integer
     comparison; −25 ≤ p ≤ 6.
  3. **Search.** For n = 1, 2, …, 17, let q = p − n + 1 and
     d₀ = floor(|v| / 10^q), computed as floor(B·10^(−q) / 2^(−E)) for q ≤ 0,
     or floor(B / (2^(−E)·10^q)) for q > 0. The candidates are d₀ and d₀ + 1.
     A candidate d round-trips when X = d·10^q lies in the rounding interval of
     |v|: below by half the lower gap and above by half the upper gap. The upper
     gap is 2^E; the lower gap is 2^E, or 2^(E−1) when B = 2^52. X must lie
     strictly inside, or on an end only when B is even (the parser's
     ties-to-even). Comparisons, including "closer" in step 4, are made on
     integers after multiplying by S = 2^(2−E)·10^max(0, −q): then |v|·S = 4B·10^max(0,−q),
     X·S, and each half gap times S (2·10^max(0,−q) for 2^E, 10^max(0,−q) for
     2^(E−1)) are all integers, including the B = 2^52 lower end (for v = 2^19
     and q ≥ 0, the lower end is 4B − 1, not 2^53 − ½).
  4. **Choice.** The first n with a round-tripping candidate decides. If both
     candidates round-trip, take the one closer to |v|; if they are equally
     close, take the even d. This is the rule of Python's `repr` and of
     ECMAScript's `Number::toString`; Rust's `format!("{v:e}")` takes the other
     candidate on such ties.
  5. **Form.** Strip trailing zeros: |v|'s decimal is m·10^e with m not divisible
     by 10; restore the sign. Bound: d < 10^17, 10^(−q) ≤ 10^41 and
     2^(2−E) ≤ 2^134, so every integer of this stage is below 2^192.
- **Admitted operand range.** After the pre-gate, |v| ≤ 10^6 in its own unit
  and e ≥ −24. The pre-gate only bounds the work and excludes nothing this
  range admits: any v with |v| ≤ 10^6 and e ≥ −24 lies within [2^−80, 2^20]. Typed values (350.1, 20.3, 1e-5) are inside it. Every absolute
  temperature in a condition set (each condition's, the installation, design and
  peak values) is judged at document level; one outside the range is
  `OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED`, and one below zero kelvin
  is `OPERATING_CONDITION_QUANTITY_INVALID`.
- **Integer form.** With D = 9·10^24 and M = m·10^(e+24) (an integer, as
  e ≥ −24), each operand's kelvin value is N/D with
  - K: N = 9·M;
  - degC: N = 9·(M + 27315·10^22);
  - degF: N = 5·(M + 45967·10^22);
  - degR: N = 5·M.

  In range |M| ≤ 10^30, so |N| < 10^31 < 2^104, and the difference
  N(T) − N(T₀) has |N| < 2·10^31 < 2^105.
- **Rounding.** ΔT = RN(N/D) with N = N(T) − N(T₀). If N = 0, ΔT = +0.0.
  Otherwise, with a = |N|, let k = bitlen(a) − bitlen(D) − 53; bitlen(D) = 83.
  Since a < 2^105, k < 0 throughout the range. Divide a·2^(−k) by D to get
  quotient q and remainder r; if q ≥ 2^53, add 1 to k and divide again. Round up
  when 2r > D, or 2r = D and q is odd. If q reaches 2^53, set q = 2^52 and add 1
  to k. ΔT = sign(N)·q·2^k. **Bound:** the shifted numerator a·2^(−k) has
  bitlen(D) + 53 = 136 bits, so it is below 2^136 (it exceeds a signed 128-bit
  integer; for 1e-24 K − 0 K, N = 9, k = −132 and a·2^(−k) = 9·2^132 ≈
  4.9·10^40); 2r < 2D < 2^84.
- **Result range.** Every nonzero result is a normal binary64: |N| ≥ 1 gives
  |ΔT| ≥ 1/D ≈ 1.1·10^−25, and in range |ΔT| < 1.1·10^6, so it is never
  subnormal and never overflows. An exact zero produces +0.0. An exact tie
  cannot occur: a dyadic N/D has its lowest bit at 2^−24 or above (D's power of
  two is 2^24), so 54 significant bits would need |ΔT| ≥ 2^29, outside the
  range. Ties-to-even is stated for completeness.
- **One implementation.** The canonical decimal, the domain and the integer
  form above are implemented once, in PP `case_state/temperature/canonical_decimal.rs`
  (`canonical_decimal`, `kelvin_numerator`, `KELVIN_DENOMINATOR`), which the EX
  route's temperature identity (`exact_kelvin`) uses. There, normalization runs
  first and refuses a quantity it cannot convert, a non-finite one or one below
  zero kelvin with `LOAD_STATE_QUANTITY_INVALID`, whatever its magnitude; an
  otherwise valid consumed absolute temperature outside the domain blocks with
  `LOAD_STATE_TEMPERATURE_RANGE_UNSUPPORTED`. C1's lowering is not
  built yet. When it is, it shall call the same functions and add only the
  rounding step, so the two agree on every row, ties included; E3 then compares
  them on all rows.

Then, for every document: remove `condition_use` from each case,
`condition_set_ref` from each pipe and `condition_sets` from the model; and
remove `direction` from a support whose direction is exactly axis-aligned,
setting its DOF token to that axis when it differs (§2.1).

**Generated identity.** Ids are length-prefixed, as self-weight's are
(`self_weight.rs` `load:self-weight:{len}:{case}:{len}:{pipe}`), so they are
injective: `load:condition-temperature:{len}:{case}:{len}:{pipe}` and
`load:condition-added-weight:{len}:{case}:{len}:{pipe}`. Resolution runs before
the duplicate-id check, so a collision with an authored id is the existing
`DUPLICATE_ID`. `provenance` is the JCS text of
`{"condition":key,"condition_set":set_id,"method":"operating_condition_set_v1","quantity":"temperature"}`
(for added weight, without `condition` and with `"quantity":"added_weight"`).
Self-weight's lineage check (`inspect_applied_self_weight`) neither recognizes
nor claims these ids or methods.

**Order.** Generated primitives follow the case's authored primitives:
temperature first, then added weight, each in `pipe_segments` order. Load
assembly sums in record order, so the order is part of the definition.

### 1.6 Pressure and contents

**Pressure (EX, slice S4).** The case's `pressure_regions` keep their authored
members and terminals; a region omits `pressure` when the case uses `pressure`,
and takes the `pressure` Quantity, verbatim, of condition `key` of its member
pipes' sets. In S4's design, members that disagree (value bits or unit text) are
`LOAD_CASE_CONDITION_PRESSURE_DIFFERS`, and a pipe with nonzero condition
pressure in no region of the case is `LOAD_CASE_CONDITION_PRESSURE_UNREGIONED`
(a stated pressure is never silently dropped). The EX temperature use fills
`element_states[].operating_temperature` verbatim (absolute, no arithmetic), and
generated primitives need `analysis_state.load_sources` entries. These touch
DEL-04-07's exact contract and its D3, so 0.9.0 carries the format (the region's
`pressure` becomes optional exactly when the case uses `pressure`) and S1 refuses
every EX use by name until S4.

**Contents density.** It differs by mode (operating contents, hydrotest water,
empty), so it is per condition. Its consumers (self-weight generation from
condition `key`, seismic and modal mass) and their relation to
`section.contents_density` and to self-weight's staleness check are not designed
here. 0.9.0 carries the member and the case key so that no later version step is
needed, and the solve refuses the key by name (OC-7).

### 1.7 Coexistence with per-case values: exclusive, except added weight

A case takes its temperature, pressure and wind exposure either from conditions
or from its own records, never both (OC-3); added weight is the one additive
exception (below):
- `temperature` excludes the case's `thermal` primitives (PF) or authored
  `operating_temperature` values (EX, S4);
- `pressure` excludes authored region `pressure` values;
- `wind_exposure` excludes authored exposure marks.

Precedence was rejected: either direction would let a value the user stated be
overridden without a trace. `added_weight` is additive: it adds loads alongside
the case's other weight primitives (self-weight among them), as a hand-typed
added-weight primitive does. Double counting a weight typed both ways is not
detectable, so the desktop states the source of each weight row.

The case's elastic basis is unchanged: condition temperatures drive thermal
strain only, and `modulus_basis_ref`/`modulus_basis_temperature` remain the
case's one elastic basis. Per-element E(T) is not introduced.

### 1.8 Numerical meaning and its tests

**The twin.** For a document D, its twin T(D) is D with every C1 member removed,
each axis-aligned `direction` removed (its DOF token set to that axis, §2.1), and each case's generated records added as
§1.5 defines them (ids, provenance, values, order). T(D) is an ordinary 0.9.0
document that today's per-case path solves.

**The equivalence.** `lower(decode(D)) = decode(T(D))` as `PreviewModel` values.
Every step after the lowering is today's deterministic pipeline, so solve(D) and
solve(T(D)) give byte-identical envelopes: rows, mechanics, diagnostics, evidence
and standing. The only exception is the fields bound to the authored document's
hash (`model_hash` and the bindings derived from it,
`result_envelope_binding.rs`), because D and T(D) are different documents. This
is `shear_model`'s "results identical, model hash differs" (0.8.0 §0).

**What the temperature claim is, and is not.** ΔT is the exact difference of the
two authored temperatures, rounded once.
- For K and degC inputs typed with up to 15 significant digits, it is the
  binary64 value of the decimal difference, so a user's own ΔT typed in K or
  degC solves byte-identically too. Example:
  20.3 degC − 20.1 degC gives the binary64 nearest 0.2; the units crate's binary64 affine
  route (`temperature_difference`) gives 0.19999999999998863.
- For degF and degR inputs the twin's ΔT is in K. A ΔT the user types in degF
  passes today's interval conversion, which multiplies by fl(5/9), so it may
  differ in its last bits. Example: 212 degF − 70 degF gives 78.88888888888889 K
  here and 78.88888888888886 K by the binary64 affine route.

**This is a new numerical policy (OC-5), adopted by Agent 0 for the conditions
route (owner's 2026-10-27 delegation, #1703).** No product
path today forms a temperature difference from two absolute temperatures on the
PF route. The twin carries RN(exact difference) in kelvin: 0.2, not
0.19999999999998863. The equivalence is to that twin, not to today's
absolute-temperature conversion route (`temperature_difference`, or any
user-side conversion of absolutes to a ΔT). **Consequence:** for the same two
temperatures, a per-case ΔT that a user formed through the unit route, as an
0.8.0-era document would carry, can differ by a few ulps from the 0.9.0
condition route's ΔT, and so can the results that follow from it. A typed
decimal ΔT in K or degC matches exactly; a typed degF or degR ΔT may not.

**Disclosure: the degF/degR cross-route difference.** A ΔT typed in degF or
degR is unchanged by 0.9.0: it still passes today's interval conversion
(value × fl(5/9)), so no existing result moves. The same temperature change
reached through conditions is rounded once from the exact value. Example: a
typed 7 degF ΔT gives 3.8888888888888893 K (`400f1c71c71c71c8`), while
77 degF − 70 degF through conditions gives 3.888888888888889 K
(`400f1c71c71c71c7`), one ulp apart. DEL-05-01 Design
`thermal-temperature-change.md` carries the same disclosure.

**Remaining item: make typed degF/degR interval conversion correctly rounded.**
Not done now, because it would move existing results; any effect on
`apps/desktop/native-smoke/expected.json` or other frozen references is the
owner's to decide.

**Tests** (S2; each fails before the change it guards):
- **E1, structure.** For each fixture, the lowered model equals the decoded
  twin. The twin is written by an independent test helper from §1.5's text, not
  by calling `lower`.
- **E2, envelope.** Solve D and T(D) on both solver modes; the envelopes are
  byte-identical after removing the hash-bound fields, which are listed.
- **E3, ΔT.** A table of (T, unit, T₀, unit) → expected bits, generated by an
  independent Python reference (`fractions.Fraction` of `repr(v)`, which
  follows the canonical decimal's tie rule, and the exact unit definitions;
  `float()` of a Fraction rounds correctly). It is cross-checked against a
  Python transcription of §1.5.1's integer algorithm, decimal stage included,
  which also asserts every intermediate is below 2^192 (decimal stage) and 2^136
  (rounding). It covers:
  - K, degC, degF and degR, and mixed units;
  - equal temperatures (+0.0) and negative ΔT;
  - the two examples above;
  - the range edges: |v| = 10^6 and e = −24 (admitted, including the smallest
    nonzero |ΔT| ≈ 1.1·10^−25, a normal binary64), and |v| just above 10^6,
    e = −25 (for example 1.5e-24) and 1e-310 (refused,
    `OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED`);
  - a value below zero kelvin (`OPERATING_CONDITION_QUANTITY_INVALID`);
  - near-halfway cases (an exact tie cannot occur in range, §1.5.1);
  - **the decimal tie regression:** binary64 `0x3f80010000000000` has the
    canonical decimal 0.007814407348632812 (m = 7814407348632812, e = −18),
    not Rust's formatter's 7.814407348632813e-3; that value K − 0.007 K gives
    ΔT bits `3f4aafbe76c8b435`, where the formatter's decimal would give
    `3f4aafbe76c8b43e`;
  - **the width regression:** 1e-24 K − 0 K, with N = 9, k = −132 and shifted
    numerator 9·2^132 ≈ 4.9·10^40 (above signed `i128`), gives ΔT bits
    `3af357c299a88ea7`;
  - the decimal stage against `repr` (Python) and `Number#toString` (Node) on a
    random sample of admitted values, and against Rust's formatter, listing the
    tie rows where Rust differs;
  - agreement with `exact_kelvin`'s decimal on all admitted rows, ties
    included.

  Mutants killed: the binary64 affine route; conversion of each temperature
  before subtraction; truncation in place of rounding; the formatter's decimal
  in place of the canonical one; a 128-bit intermediate in the rounding.
- **E4, order.** A case where authored and generated weight on one pipe sum
  differently in another order; a mutant that prepends generated primitives, or
  uses set order, changes E2's bytes.
- **E5, unused sets.** A document with sets and assignments but no
  `condition_use` solves byte-identically to the same document without them
  (0.7.0's annotated-model precedent).
- **E6, refusals.** Every §1.3 code, at the solve and, for document-level codes,
  at the validator.
- **E7, G-1 after lowering.** An operating case using `temperature` and a
  sustained case without it pass G-1 (the generated thermal primitives are in the
  minuend only); swapped, G-1 withholds. A sustained case using `added_weight`
  against an operating case that also authors the same added-weight primitive
  by hand (two equal primitives in the minuend, one in the subtrahend) passes;
  the reverse multiplicity withholds.

## 2. `direction` and `difference_basis`

### 2.1 `supports[].direction` (R-7, carried exactly)

- **Type.** `Vector3` `{x, y, z}` in the global frame, dimensionless: an
  optional, finite, non-zero binary64 3-vector. There is no unit-length
  tolerance: no binary64 vector with two or more nonzero components has exactly
  unit length.
- **Stored exactly as authored**, never as a normalized copy. The applier writes
  the entered numbers (shortest round-trip JSON) and nothing else.
- **Admitted only on** a support whose explicit `family` is `line_stop` or
  `spring`, that acts on exactly one DOF (its single restraint token, or its
  stiffness DOF), that DOF translational (UX, UY, UZ), and that carries no
  `nonlinear` member. Rotational skew is refused for now. A `snubber` (§2.4)
  is admitted on the same terms (Agent 0's list, 2026-10-31, at the dynamics
  lane's request): exactly one translational restraint, with any `stiffness` on
  that DOF; it acts along d as a skewed line stop does. With more than one
  restraint, `direction` stays `SUPPORT_DIRECTION_RECORD_UNSUPPORTED`. A
  skewed snubber is under the snubber's interim static refusal like any
  snubber, so the axis-aligned lowering below never reaches one; its dynamic
  meaning is DEL-04-10's.
- **Absent means axis-aligned** along the record's DOF; the product never writes
  `direction` unprompted.
- **Sign.** Published scalars are positive along +d.
- **The DOF token.** R-7 asks only that the record's single DOF be
  translational; when `direction` is present, d governs the axis, and the token
  states nothing more. This design adds no relation between the token and d
  (see ND-1 in §6 for the one Agent 0 may choose to add).
- **Exact ±axis directions take today's path, byte for byte.** A d with exactly
  one nonzero component, on axis a, is removed by the lowering (§1.5). If a is
  the record's DOF axis, the token is kept as authored; otherwise the token
  (the single restraint token, or `stiffness.dof`) is rewritten to `UX`, `UY` or
  `UZ` for a. The solve is that twin's, and every published value is today's for
  it. Example: a single-DOF UX spring with d = (0,3,0) solves as a UY spring.
  The along-d scalar is a new publication of the skew kernel; for d = −e_x it is
  the exact negation of today's UX component.
- **Codes** (validator and solve unless marked):
  - `SUPPORT_DIRECTION_INVALID`: null, not an object, a missing or non-number
    component, or a non-finite component;
  - `SUPPORT_DIRECTION_ZERO`: every component is ±0;
  - `SUPPORT_DIRECTION_RECORD_UNSUPPORTED`: any other family, an inferred family,
    more than one DOF, or a `nonlinear` member;
  - `SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED`: the record's DOF is rotational;
  - `SUPPORT_DIRECTION_SKEW_UNSUPPORTED` (solve and applier only; interim until
    the skew kernel): two or more nonzero components. The kernel's own refusals
    (range, redundant constraints, prescribed motion at the node, motion targets,
    nonlinear models, friction-normal sources, EX) are the kernel design's
    (solver note §2.3-§2.4).

### 2.2 `combinations[].difference_basis` (carried exactly)

- **Type.** String enum `["operating_minus_sustained"]`. Absent means a plain
  subtraction.
- **Admitted only with `basis: result_state_subtraction`**; otherwise
  `LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED` (blocking).
- **Null or an unknown token** gives `LOAD_COMBINATION_DIFFERENCE_BASIS_UNKNOWN`
  (blocking). The wire decodes any present value so that these codes, not a
  decode error, name it.
- **Gates at solve** are withholding warnings, as the solver note §1.2 states
  them:
  - **G-1**, verbatim: "a conservative containment check. The subtrahend's
    primitive loads, as a multiset, are contained in the minuend's by content:
    category, target, direction, dimension, and value and unit bits after the
    solve's own normalization; ids are ignored. Code
    `EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`. It does **not** require equal
    final active sets." Multiplicity counts: for each content key (those six
    fields), the subtrahend's count must not exceed the minuend's, so each
    subtrahend primitive is matched to a distinct minuend primitive. Two
    identical sustained weight primitives against one matching operating
    primitive fail G-1. Provenance is not a content field.
  - **G-2**: the same modulus basis (`EXPANSION_BASIS_MODULUS_MIXED`), binding
    the resolved elastic basis.

  G-1 runs on the lowered model, so condition-generated primitives count by
  content (E7).
- **Refused by name on EX:** `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`
  (blocking).
- **Written only when the user sets it.** Choosing the "operating − sustained"
  template is such a setting.
- **Interim (S1 until the solver lane's 1b):** the solve withholds the
  combination with the warning `LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED`,
  and the applier refuses authoring it. 1b removes both and adds G-1/G-2.
- **`subtrahend_support_state` is adopted in 0.9.0** (Agent 0, 2026-10-30;
  RT-A5's Should item), in the room reserved here: the enum `own |
  minuend_final`, absent meaning `own`, admitted only with `difference_basis`
  (otherwise `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED`, blocking; a
  null or unknown token is `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNKNOWN`,
  blocking). `minuend_final` solves the subtrahend at the minuend's final
  support status, a held-constraint comparison that is not an admissible
  sustained state, and publishes under the combination, never as the sustained
  case's rows (DEL-05-02 `operating-case.md` §5 option 2). Interim until its
  slice: `own` behaves as absence; the solve withholds a `minuend_final`
  combination with the warning `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED`,
  and the applier refuses authoring it.

### 2.3 `load_cases[].kind` token `operating`

Desktop vocabulary only: the load manager offers `operating` among its kind
suggestions (`LoadCaseManagerPanel.tsx` metadata options). `kind` stays a free
string the solver ignores. No schema change.

### 2.4 Members joined before S1 (Agent 0, 2026-10-30)

Each is optional, closed and refused by name at solve until its slice lands. A
document carrying none of them has today's meaning.

- **`combinations[].basis` tokens `absolute_sum` and `srss`** (RT-A4), and
  **combinations as inputs**: `CombinationTerm.combination`, exactly one of
  `load_case` and `combination` per term; `minuend_id`, `subtrahend_id` and
  `operand_ids` naming a load case or a combination; acyclic. Codes, numerical
  definition and interim refusals: DEL-05-02 Work
  [`absolute-sum-srss-combinations.md`](../../../PKG-05/DEL-05-02/Work/absolute-sum-srss-combinations.md)
  §2-§3, §6.
- **`load_cases[].test_condition`** (RT-A6): `{kind: "hydrostatic",
  held_supports: "springs_and_constant_effort" | "none"}`, both required. The
  test contents density is the case's `condition_use.contents_density` (§1.6),
  whose consumer this is. Interim: `LOAD_CASE_TEST_CONDITION_UNSUPPORTED`,
  blocking at solve. DEL-05-01 Work
  [`hydrostatic-test.md`](../../../PKG-05/DEL-05-01/Work/hydrostatic-test.md).
- **`combinations[].subtrahend_support_state`** (RT-A5 Should): §2.2.
- **Snubber support family** (RT-A8; the dynamics lane's text, 2026-10-30;
  its final meaning is DEL-04-10's). `SupportFamily` gains the token
  `"snubber"`; no new object member. A snubber uses `Support`'s existing members.
  - `restraints`: one or more of `ux`, `uy`, `uz`; a rotational DOF is
    `SNUBBER_ROTATIONAL_RESTRAINT_UNSUPPORTED`.
  - `stiffness` (the existing `SupportStiffness`) is optional. Absent, the
    snubber is rigid in its restraints. Present, `restraints` is exactly
    `[stiffness.dof]` with a positive finite value in a stiffness unit;
    otherwise `SNUBBER_STIFFNESS_INVALID`.
  - A `hanger`, or a non-null `nonlinear`, on a snubber is `SNUBBER_MEMBER_CONFLICT`.
  - `direction` (§2.1) is admitted on a snubber with exactly one translational
    restraint, any `stiffness` being on that DOF; the snubber then acts along
    `direction`.
  - These are document-level (schema, `validate_support_family_tokens`, the
    applier and the solve's validation), blocking.
  - **Existing support gates (r2 F5).** `validation.rs`
    `validate_connected_input_contracts` refuses a finite stiffness on any
    family but spring and variable-spring hanger
    (`SUPPORT_STIFFNESS_FAMILY_UNSUPPORTED`). S1 admits an explicit `snubber`
    there, its stiffness being dynamic; null-family inference and the hanger and
    constant-effort detection are unchanged, and an inferred family is never a
    snubber. Regression: a valid stiff snubber reaches
    `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED` and no other code.
  - **Interim:** every static solve on both routes refuses a model containing a
    snubber, blocking, as `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`, naming the
    support: "snubbers are dynamic-only restraints; the response-spectrum
    analysis that uses them is not built yet". DEL-04-10's slice replaces the
    refusal with its final meaning: static solves ignore a snubber, disclosed
    as "dynamic-only restraint, not active in static cases"; modal (when
    requested) and spectrum restrain with it.
- **`spectra[]`** (RT-A8; the dynamics lane's text, 2026-10-30): top-level
  records of user-entered spectra, data only, so that a spectrum is entered
  once. Each is a closed `Spectrum` (r2 F8): required `id`, `abscissa`,
  `ordinate`, `units`, `damping_ratio`, `interpolation`, `points`, and `g_value`
  exactly when `ordinate` is `g_multiple`; optional `label` (string), `tags`
  (`Tags`) and `notes` (`Notes`). An authored `null` is refused for every
  member, as §1.2 rules. `units.abscissa` is always required and
  `units.ordinate` exactly when `ordinate` is `acceleration` (r3);
  `interpolation` requires both its members.
  - `id` (`Id`, unique among spectra, `SPECTRUM_ID_DUPLICATE`; `EMPTY_ID` as
    for any entity);
  - `abscissa`: `"frequency"` | `"period"`; `ordinate`: `"acceleration"` |
    `"g_multiple"`;
  - `units`: closed `{abscissa, ordinate}`, catalog unit strings: the abscissa
    unit of the frequency family for `frequency` or the time family for
    `period`; the ordinate unit of the acceleration family, present exactly when
    `ordinate` is `acceleration`. **The unit catalog has no frequency family
    today (r2 F6), so S1 carries it, with the rest of the dynamics unit set**
    (SWBPIPE Agent 0 and the dynamics lane, 2026-10-31), so that `core/units`
    changes and is priced once. These are DEL-02-02's families, needed by
    DEL-04-09 Design §1.5 (D-p); M1b reuses them and defines none of its own.
    Each is a separate named dimension, checked by name as `LinearStiffness`
    and `ForcePerLength` are, never matched by vector; vectors are (length,
    mass, time, temperature, angle):

    | Dimension | Vector | Canonical unit | Display units |
    |---|---|---|---|
    | `frequency` | (0,0,−1,0,0) | `unit:hertz`, `Hz` | none |
    | `angular_frequency` | (0,0,−1,0,1) | `unit:radian_per_second`, `rad/s` | none |
    | `rotation_per_length` | (−1,0,0,0,1) | `unit:radian_per_meter`, `rad/m` | `rad/mm`, `rad/in` |
    | `length_per_rotation` | (1,0,0,0,−1) | `unit:meter_per_radian`, `m/rad` | `mm/rad`, `in/rad` |
    | `rotational_generalized_mass` | (2,1,0,0,−2) | `unit:kilogram_square_meter_per_square_radian`, `kg*m^2/rad^2` | `lb*in^2/rad^2`, `lb*ft^2/rad^2` |

    `angular_frequency` carries the angle exponent, so it never converts to
    Hz. `rotational_generalized_mass` replaces a plain kg·m² family (the
    dynamics lane's review, 2026-10-31): with angle tracked as an exponent,
    Γ in rad/m and a generalized mass in kg·m²·rad⁻² keep Γ²·m = m_eff in kg,
    and match kinetic energy in J with rotations in rad; nothing needs plain
    kg·m². Display factors are definitional only (0.0254 m/in, 0.3048 m/ft,
    0.45359237 kg/lb), each with a conversion round-trip witness as the crate's
    others have. All five join `schemas/units.schema.yaml`, the crate
    vocabulary (`schema_dimension_enum_matches_crate_vocabulary`) and
    `canonical_unit`. Period stays on `time`; modal shapes use the existing
    `rotation` and dimensionless. `core/units/src` is priced, so S1's D3 turn
    prices it;
  - `g_value`: a `Quantity` in acceleration units with a positive, finite
    value, required exactly when `ordinate` is `g_multiple` (r2 F7: zero or a
    negative value is `SPECTRUM_RECORD_INVALID`);
  - `damping_ratio`: dimensionless, required, finite, in [0, 1);
  - `interpolation`: closed `{abscissa, ordinate}`, each `"linear"` | `"log"`,
    both required, no default;
  - `points`: an array of `[x, y]` number pairs, at least two, x strictly
    increasing and positive, y positive and finite.

  Any other violation is `SPECTRUM_RECORD_INVALID`, naming the record and the
  field (document-level, blocking). Spectra are referenced only from
  DEL-04-10's spectrum analysis request, never from load cases; direction,
  scale and method stay on that request. No solve reads them in 0.9.0, so a
  document's static results do not depend on them (as E5 for unused condition
  sets), and every displayed spectrum carries §1.2's label "not used by the
  solve in 0.9.0" until DEL-04-10's slice.

The dynamics lane needs no other member (no per-mode mass and no contents
member; concentrated masses are W-17's, later). **The member list is frozen**
(SWBPIPE Agent 0, 2026-10-31, at `31bde47a2`, with the dynamics lane's
confirmation): any further member is a 0.10.0 step, planned as one step.

## 3. The 0.9.0 version step

### 3.1 Version 0.9.0

0.9.0 is 0.8.0 plus the optional members of §1.2, §2.1, §2.2 and §2.4. An 0.8.0
document whose version string alone is changed is a valid 0.9.0 document with the
same meaning: it carries none of the new members, and an absent member changes
nothing. No member is renamed or re-keyed (FR-MOD-003).

**Why a version.** The records that gain members are closed on the Rust wire
(`PreviewModelWire`, `PreviewPipe`, `PreviewLoadCase`, `PreviewSupport`,
`PreviewCombination`, all `deny_unknown_fields`) and in the schema, so an 0.8.0
build refuses each new member as an unknown field. A document carrying one is
not an 0.8.0 document.

### 3.2 0.8.0 is retired; nothing is migrated

Under the one-version rule, 0.8.0 joins the retired list and is refused by name
(`MODEL_SCHEMA_VERSION_RETIRED`) on every 0.8.0 §3.1 surface, with the message
the constants form:

> model document 0.8.0 is retired; re-author it as model document 0.9.0 (a pressure-free model omits pressure_contract). Nothing is migrated.

- **Projects and saved states** holding 0.8.0 are refused on open, save and
  compare, with `RETIRED_PROJECT_RUNS_NOTE` appended; stored rows are never
  rewritten. The stored-document recovery export still writes their bytes for
  re-authoring.
- **Recorded runs** keep their read and compare behaviour ("0.8.0 a2").
- **The recorded table basis.** The token edit moves pressure-2's table digest
  again, as at 0.8.0 (0.8.0 §3.2, Codex r6 F1). `RECORDED_RUN_TABLE_BASES`
  (`analysisRunCompatibility.ts`, `compatibility.py`) gains the 0.8.0-era digest
  as a recorded basis of exactly the current table. Regression: an ordinary
  pressure-2 record captured at 0.8.0 (`fixtures/persistence/recorded_model_0_8_0`)
  reads, displays with no finding and is offered for comparison.
- **The user guide** gains the 0.8.0 paragraph's successor ("Projects saved
  before model document 0.9.0 …"), naming the 0.9.0 members and the same
  practical limit.
- **No migration path** (OC-4): DEL-02-05 CLM-003 says "no migration code is
  kept"; a desktop "re-author as 0.9.0" action would be migration code and would
  change that commitment, which is the owner's.

### 3.3 The version-state table (`fixtures/model_document/version_states.json`)

`current` becomes `0.9.0`, and every row's `target_schema_version` and detail
text follow. Rows that change:

| Label | `schema_version` | State |
|---|---|---|
| current | `0.9.0` | `current` |
| retired 0.8.0 (new) | `0.8.0` | `retired_schema` (listed message) |
| retired 0.7.0, retired 0.1.0 | unchanged | `retired_schema` (listed) |
| older, unlisted | `0.4.1`, `0.0.1` | `retired_schema` (unlisted message) |
| newer patch | `0.8.1` → `0.9.1` | `newer_than_supported` |
| newer minor | `0.9.0` → `0.10.0` | `newer_than_supported`; exercises the longer-part rule |
| newer major | `1.0.0` | `newer_than_supported` |
| two parts, four parts, prefixed, leading space, signed | `0.9`, `0.9.0.0`, `v0.9.0`, ` 0.9.0`, `+0.9.0` | `unsupported_schema` |
| leading zero, older | `0.08.0` | `retired_schema` |
| leading zeros, equal to current | `00.8.0` → `00.9.0`; `0.8.00` → `0.9.00` | `retired_schema` |
| leading zero, newer minor / patch | `0.09.0` → `0.010.0`; `0.8.01` → `0.9.01` | `newer_than_supported` |
| older, 4301-digit patch | `0.7.9…` → `0.8.9…` | `retired_schema` |
| 4301 leading zeros, equal to current | `…0.8.0` → `…0.9.0` | `retired_schema` |

The missing, empty, null, number, object, `u64` and other 4,301-digit rows keep
their states.

### 3.4 The four implementations

- **Rust:** `core/product_physics/src/model_format.rs`
  (`MODEL_SCHEMA_VERSION = "0.9.0"`; `RETIRED_MODEL_SCHEMA_VERSIONS: [&str; 8]`
  gains `"0.8.0"`); the applier's evaluator
  (`operation_applier/src/model_document_version.rs`, which src-tauri delegates
  to); its tests' "newer" probes become `0.9.1`, `0.10.0`, `1.0.0`.
- **Python:** `core/project_persistence/service.py` (`MODEL_VERSION`,
  `RETIRED_MODEL_VERSIONS`), and every module-level `MODEL_VERSION` of the
  verifiers and validation tools (the literal guard checks each is current).
- **TypeScript:** `apps/desktop/src/services/modelSchemaVersion.ts` (constant and
  retired list); `projectService.ts` follows. Probes that use `0.9.0` as "newer"
  (`projectService.test.ts`, src-tauri `model_document_save.rs`,
  `loadReferenceRoundTrip.test.ts`) move to `0.10.0` or `0.9.1`.
- **Schema:** `schemas/model.schema.yaml`: `schema_version` `enum: ["0.9.0"]`;
  the description names 0.1.0–0.8.0 as retired; new `$defs` `ConditionSet`,
  `OperatingCondition`, `ConditionValues`, `AddedWeight`, `ConditionExposure`,
  `ConditionUse`; members `condition_sets`, `PipeSegment.condition_set_ref`,
  `LoadCase.condition_use`, `Support.direction` (`$ref` `Vector3`, described as
  dimensionless), `Combination.difference_basis`, `Combination.subtrahend_support_state`,
  the `Combination.basis` tokens `absolute_sum` and `srss`,
  `CombinationTerm.combination` (a `oneOf` with `load_case`), `$defs`
  `TestCondition` with `LoadCase.test_condition`, the `SupportFamily` token
  `snubber`, and `$defs` `Spectrum` with top-level `spectra` (§2.4).
  `PressureRegion.pressure`
  stays required in the schema except where the case carries
  `condition_use.pressure` (an `if`/`then` on `LoadCase`). Every record stays
  closed; the schema stays structural.

All four are checked against the one table.

### 3.5 Other production code in S1

- **The wire** gains the members as closed records (`model_fields` declares the
  keys; the nested scan's key-loop bound rises).
- **The lowering entry** (§1.5) lands in S1 with the interim refusals and the
  axis-aligned `direction` removal; S2 replaces the `condition_use` refusal by the
  PF resolution.
- **Readers and exporters.** Every model reader is listed and classified: carries
  the document verbatim (handoff package, native JSON export, recovery export),
  lowers through the engine, or refuses a member it cannot express by name, never
  dropping it. In S1 the CAEPIPE SI writer (`si_writer.py`) refuses
  `condition_use`, non-axis `direction` and `difference_basis` by name; the
  report's load-case section reads the lowered case through the engine. A
  report or export view that displays a member the solve does not read labels it
  "not used by the solve in 0.9.0" (§1.2).
- **Literals, audit and guards** as at 0.8.0 §2 and §5: every file containing
  `0.8.0` is listed and classified; `READER_TOKEN_SITES` is re-checked;
  `test_model_document_schema.py` gains an explicit 0.8.0 exception list.

### 3.6 Fixtures and documents that re-author

By their maintained writers, never by hand, under the 0.8.0 §5 standard (version
token plus following digests; no physical value, status or reader outcome moves;
numerical leaves compared separately):
- the bundled demo, product-preview fixtures and generation records;
- the ui-foundation fixtures and manifest; `t4_u5b_requests`;
- the applier contract corpus; the qualification packages and MANIFEST;
- the reader corpora; result envelopes; persistence fixtures;
- `version_states.json`;
- the emission goldens (`desktop_emitted.json`, `applier_emitted.json`) and
  `invented_every_slot_model.json`, which also gain every new member as reviewed
  additions;
- the retained successors (16 `retained_precision_*successor*`,
  `retained_precision_cases.json`, the two `failed_ordinary`);
- whatever other lanes have landed at 0.8.0 by the ripple.

**Exceptions**, each listed with its reason: SHA-bound frozen requests under
`validation/**` (re-authored in memory; the helpers gain 0.8.0), the committed
0.7.0 handoff control, and one new 0.8.0 control,
`fixtures/handoff/retired_0_8_0_witness.swbhandoff`.

`apps/desktop/native-smoke/expected.json` is not edited. It pins no version or
hash, and the newer-version case writes `9.0.0`. A native smoke run on the build
head confirms it; any value that moves is reported to the owner.

### 3.7 D3 pricing and AC-004

D3 is touched; AC-004's text is not.
- **S1 re-prices** (`tools/retained_pricing`): `model_format.rs`, `lib.rs` (wire
  members, closed records, the lowering call), `model_fields.rs`,
  `pressure_runtime.rs`, `retained_wire.rs` (`EXACT_DEFINITION_SHA256` follows
  DEF-E), `physics_source.rs`, `retained_precision.rs`, `semantic_contract.rs`,
  and the new module where it is reachable on D1. The lowering's document-level
  checks are reachable on D1, so coefficients are expected to move, as
  `shear_model`'s check moved them (0.8.0 §13).
- **Registration** follows 0.8.0 §7: the token edits to DEF-E
  `scope.model_format`, the pressure-source-1 limitation and the pressure-2
  admitted-families sentence (`semantic_contract_tables.py derive`/`repin`);
  `retained_memory.rs` rebinds; successors and the synthetic-bases writer
  regenerate on the registered build; `compare` against main's tree, expecting
  `identical_but_basis` for the token part as a hypothesis to verify.
- **S2 re-prices** if the resolution changes D1-reachable code. **Retained
  routes:** `retained_memory::assess` prices the raw request before the
  lowering, so the D1 domain would not count generated primitives. Until S2
  settles where the lowering sits for those routes, the retained Direct and
  headless routes and the independent source readers decline a document carrying
  `condition_use` by name, and the ordinary route publishes.
- **AC-004.** Its six stack witnesses re-run on each priced build under an
  exclusive lane granted by Agent 0. This is test evidence, not acceptance.
- **Queue.** The 0.9.0 D3 follows the queued §8.27 (W-16b/c), §8.28 (stage 2)
  and #1678; Agent 0 sets its place.

## 4. Run sheet and desktop authoring

**Split between lanes (agreed between the load-cases and authoring lanes,
2026-10-31; placed by Agent 0, 2026-10-31).** S3a, the applier operations of §4.2
with their refusals, guards and contract corpus, admitted in atomic
`apply_operation_batch` batches with per-step refusals, and a wasm export of
the engine's per-pipe condition resolution for the load manager's preview, is
the load-cases lane's. S3b, every desktop surface of §4.1 and §4.2 (the
run-sheet column, the condition-set manager, inspector and bulk-change fields,
support Direction, the load manager's Conditions control and the combination
editor with its templates), is the authoring lane's (DEL-07-02, DEL-07-03).
Each reviews the other's side of the operation payloads before they freeze.

### 4.1 The run sheet's Condition set column (O7)

- **Column.** "Condition set": an existing set id, chosen from the model's sets.
  The sheet creates no set, and no set is suggested.
- **Inheritance.** It joins the Inherits list beside Material
  (`run-sheet-entry.md` §2.1): a blank cell takes the predecessor's effective
  value; an `F` row from a stored node inherits when every meeting pipe carries
  the same set, otherwise the cell is blank (§2.3); `-` means absent and ends
  inheritance (§2.4). Origins show as for any inherited cell (§2.5).
- **Emission.** After a run row's `connect_pipe_run` (and `assign_section`), one
  `assign_condition_set` (target `Element`, `field_path` `condition_set_ref`,
  `after` `{"condition_set_ref": id}`), only when the effective value is not
  absent. `connect_pipe_run`'s payload is unchanged.
- **Bends.** A bend row and its next run row need the same effective set (or
  both absent), as for section and material; otherwise the row is refused. The
  record projection (§3.3) adds `condition_set_ref` to the projected pipe record,
  since a later bend's `before` context holds full pipe records.
- **Unset is allowed.** A pipe without a set is valid; a case that uses
  conditions refuses it by name at solve, and the load manager shows that.

The run-sheet design's §1.2 "Operating condition" paragraph, §2.1 and §6.1 are
updated in S3.

### 4.2 Desktop surfaces (outline)

- **Condition sets** in the model tree, with a table per set whose rows are its
  conditions (key, T, P, contents) and fields for installation temperature,
  design, peak, added weight and exposure. Every value is entered with a catalog
  unit; empty cells read "not stated"; nothing is prefilled. Design, peak,
  snow/ice, wave and contents density carry the label "not used by the solve in
  0.9.0" (§1.2).
- **Pipe inspector and engineering table:** a "Condition set" field
  (`assign_condition_set`, or clear).
- **Bulk change** (B2): "Assign condition set" over the selection.
- **Load manager** (DEL-07-03): a "Conditions" control per case (temperature
  key, pressure key on EX documents, added weight, wind exposure), with the
  engine's per-pipe resolution preview (through the wasm lowering; the desktop
  computes no ΔT) and refusals by name; `operating` among kind suggestions; the
  combination editor's `difference_basis` with the "operating − sustained"
  template, offered only on a result-state subtraction.
- **Support inspector:** an optional Direction (three numbers) on single-DOF
  translational line stops and springs; empty means axis-aligned; never
  prefilled.

**Applier operations (S3):** `create_condition_set`, `update_condition_set`,
`delete_condition_set` (refused while referenced, `OP-CONDITION-SET-IN-USE`),
`assign_condition_set`, `set_load_case_condition_use` and its clear; `direction`
on `create_support` and `update_support`; `difference_basis` on combination
create and update (after 1b). A support-kind replacement that would leave a
`direction` on an inadmissible record is refused, with the reason in the bulk
preview. `transform_pipe_run` refuses runs with attached supports today
(`geometry_operations.rs` reference guard), so no operation rotates a direction;
any later one must transform it exactly or refuse.

## 5. Slices

| Slice | Content | Checks and review gates |
|---|---|---|
| **S1. The 0.9.0 version step** | Every 0.9.0 member at once: schema, closed wire, validator, all document-level codes, interim solve refusals (`condition_use`, skew `direction`, `difference_basis`, and §2.4's members), axis-aligned `direction` on today's path, geometry operations carrying `condition_set_ref`, the dynamics unit families (§2.4), the version constants in all four implementations, the table, the ripple, reader classification, user guide, D3 and registration | `version_states.json` on the applier, src-tauri, TS and Python; refusal on every 0.8.0 §3.1 surface; ruling a2 and the recorded table basis; annotated equivalence (E5) and axis `direction` equivalence (twin without `direction`, byte for byte, both modes, including d = −e_x, d = (2,0,0), and d = (0,3,0) on a UX record, which solves as its UY twin); leaf comparison; literal guard; D3 `price`, `compare`, `generate --check`; AC-004 witnesses under the exclusive lane; native smoke compared with main; `piping-pytest`, `desktop-test`, affected crates, hosted `harness`. Independent review of the ripple and the registration. |
| **S2. PF resolution** | Temperature, added weight and wind exposure resolution (§1.5); the correctly rounded ΔT; removal of `LOAD_CASE_CONDITION_USE_UNSUPPORTED`; retained-route placement or decline; wasm preview export | E1–E7; the Python reference table; D3 if D1-reachable code moves. The PR states explicitly that typed ΔT is unchanged (OC-5 as decided): a test shows every frozen reference (SHA-bound requests, the reader corpora, the retained successors and `expected.json`'s inputs) solves byte-identically before and after, and `expected.json` is not edited. Independent numerical scrutiny by a separate agent of ΔT, order and the twin. |
| **S3a. Applier authoring** | §4.2's operations; reference guards; bend and split rules | One regression per operation and refusal; save and reopen byte for byte; contract corpus additions. Persistence review. |
| **S3b. Desktop** | Condition-set manager, inspector field, bulk change, load-manager control, support direction, the run-sheet column (with B1's slice if B1 has not landed) | `desktop-test`; run-sheet inheritance and projection tests; native smoke where open/save changes. |
| **S4. EX resolution** | Region pressure and `operating_temperature` from conditions, `load_sources` entries, removal of the EX interim refusal; designed with DEL-04-07 | Its own design check, D3, and independent scrutiny. |

**Why S1 carries the format but not the PF resolution.** The format and its
refusals change every document reader once, which is the ripple; the resolution
adds numerical meaning and needs its own scrutiny. A separate S2 keeps each
reviewable, and the interim refusal follows 0.8.0's Timoshenko precedent. Between
S1 and S2, a document using conditions opens and edits but does not solve.
Merging S1 and S2 is OC-9.

**Outside this plan:** the solver lane's 1b (G-1/G-2) and skew kernel remove
their interim refusals; the contents-density consumer; a CAEPIPE MBF import of
per-element temperatures into condition sets (DEL-17-04).

## 6. Decisions (Agent 0)

OC-1 to OC-9 and ND-1 were decided by Agent 0 under the owner's 2026-10-27
delegation (#1703) and adopted as recommended. The reasons are kept.

| # | Choice | Decision (recommendation) | Consequence |
|---|---|---|---|
| OC-1 | **Names** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. `condition_sets[]` with `conditions[].key`; `pipe_segments[].condition_set_ref` (parallel to `section_ref`); `load_cases[].condition_use` | Names freeze at S1. "Operating" is avoided in the record names because design, peak and hydrotest conditions are not all operating; the alternative `operating_condition_sets` / `condition_refs` costs nothing now and a version step later. |
| OC-2 | **Stored or read-resolved** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. (a) Assignment stored explicitly per pipe; inheritance in authoring tools (§1.4). (b) Values resolved at read by the lowering, not generated into cases | (a) Re-assigning a line is a bulk change; no topology rule is needed. Read-time inheritance would need a "run" or evaluated `lines[]` rule, ambiguous at tees. (b) One source of truth; authoring-time generation would duplicate values in every case and need a staleness check like self-weight's. |
| OC-3 | **Coexistence with per-case values** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Exclusive per case for temperature, pressure and wind exposure; added weight additive (§1.7) | A case cannot mix condition and hand-typed temperatures; per-pipe exceptions are made with another set. Precedence would override stated values without a trace. |
| OC-4 | **0.8.0 documents** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Retired, refused by name, nothing migrated (§3.2) | Same burden as at 0.8.0: users re-author by changing the token, using the recovery export for stored projects. A desktop rewrite action is migration code and changes DEL-02-05 CLM-003, an owner decision. |
| OC-5 | **PF temperature basis: a new numerical policy for Agent 0's approval** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Absolute condition temperatures, the set's installation temperature, each operand read as its canonical decimal (the typed decimal for values of up to 15 significant digits; ties to the even digit), exact difference rounded once in kelvin within §1.5.1's range (§1.5, §1.8) | The twin carries 0.2, not 0.19999999999998863; equivalence is to the twin, not to today's absolute-temperature conversion route, so a per-case ΔT formed through the unit route (as 0.8.0-era documents carry) can differ by ulps from the condition route for the same temperatures. Needs a new canonical-decimal stage and a correctly rounded conversion on 256-bit integers, with the E3 reference; the EX route's `exact_kelvin` reads the same canonical decimal (`case_state/temperature/canonical_decimal.rs`). Alternatives: the units crate's `temperature_difference` (no new numerics; last-bit differences from the decimal ΔT, e.g. 0.19999999999998863 for 0.2); or storing ΔT in the condition (no arithmetic, but not an operating temperature, and EX needs absolutes). A model-level installation temperature in place of the per-set one is smaller but cannot express lines installed at different temperatures. |
| OC-6 | **Key form** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. String keys (REQ-02-01-03) | Typos are caught as `OPERATING_CONDITION_KEY_MISSING` at solve, not at entry. An integer index k is positional; a model-level key registry adds a member and catches typos earlier. |
| OC-7 | **Members read by no solve in 0.9.0** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Carry per-condition `contents_density` with its case key (refused at solve), `design`, `peak`, `snow_ice` and `wave` now, each visibly labelled "not used by the solve in 0.9.0" wherever shown (§1.2) | Avoids a 0.10.0 step when their consumers come; the format is fixed before those consumers are designed. Omitting them keeps 0.9.0 smaller, and each later adoption is a version step that strands projects again. |
| OC-8 | **A pipe without a set in a case that uses conditions** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Refuse the case by name | Every pipe needs a set (an ambient pipe states its temperature). Skipping would be a silent zero. |
| OC-9 | **S1 and S2 separate** | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Separate, with the interim refusal | Two reviewable steps and one more D3 turn if S2 moves D1-reachable code. Merging gives one turn and one larger review. |

**A further decision, not part of R-7 (ND-1).** Codex r1 found that an earlier
revision refused a d with a zero component on the record's DOF axis
(`SUPPORT_DIRECTION_AXIS_COMPONENT_ZERO`), a restriction R-7 does not make; it is
removed, and `direction` is carried as decided (§2.1). Agent 0 may add it as a
separate rule:

| # | Choice | Decision (recommendation) | Consequence |
|---|---|---|---|
| ND-1 | **Tie the DOF token to d** by refusing a d whose component on the record's DOF axis is zero | Decided by Agent 0 (owner's 2026-10-27 delegation, #1703): adopted as recommended. Do not adopt; keep R-7 as decided | Not adopting: d governs, the token only marks the record translational, and an axis-aligned d on another axis lowers to the record on d's axis (a UX spring with d = (0,3,4) is admitted; one with d = (0,3,0) solves as a UY spring). The inspector shows d as governing. Adopting: the token and d never disagree, so a reader of the token alone is never misled, at the cost of refusing records R-7 admits. |

**Notes on the decisions (Agent 0, #1703):**
- **OC-4** follows the owner's own precedent for 0.8.0 ("R-c1 A, 0.8.0 a2").
  Any rewrite or migration action remains the owner's call, through DEL-02-05's
  commitment that no migration code is kept (CLM-003).
- **OC-5** is adopted for the conditions route only. Typed ΔT stays exactly as
  today, so no existing result or `expected.json` moves; S2's PR confirms this
  with a test over every frozen reference (§5). The degF/degR cross-route
  difference is disclosed (§1.8; typed 7 degF → 3.8888888888888893 K; 77 − 70
  degF through conditions → 3.888888888888889 K), here and in DEL-05-01 Design
  `thermal-temperature-change.md`. Remaining item: make typed degF/degR interval
  conversion correctly rounded; deferred because it would move existing results,
  and any `expected.json` effect is the owner's.
- **OC-7** is accepted because a case's `condition_use.contents_density` is
  refused by name (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`), so nothing
  silently drops contents weight. That refusal stays until contents resolution
  is designed.

## 7. Proposed ScopeOfWork text (not applied)

Each edit replaces only the quoted Old text, which occurs exactly the stated
number of times in the file at this basis. Commitment changes and new
commitments are Agent 0's under the owner's delegation; current-implementation
wording is applied with the slice that makes it true.

### 7.1 DEL-02-01 (`execution/PKG-02/DEL-02-01/ScopeOfWork.md`)

1. **CLM-006 construction (line 84), 1 occurrence.**
   - **Old:** `loads, load cases and combinations; lines,`
   - **New:** `loads, load cases and combinations; operating-condition sets, assigned per pipe segment and used by load cases (model document 0.9.0); lines,`
2. **CLM-006 current implementation (line 93), 1 occurrence; at S1's merge.**
   - **Old:** `` Current implementation (S1a and S1b, 2026-10-23, and W-15 slice c; Design `Design/model-document-0.7.0.md` §2-§4 and `Design/model-document-0.8.0.md`): `schemas/model.schema.yaml` describes the desktop's model document, `document_kind` `openpipestress.product_preview.model` at version `0.8.0` (owner OD-4, 2026-10-23: the desktop's model document is canonical; the desktop open/save and mechanics solve paths refuse 0.7.0 and older by name). 0.8.0 adds ``
   - **New:** `` Current implementation (S1a and S1b, 2026-10-23, W-15 slice c, and the 0.9.0 step; Design `Design/model-document-0.7.0.md` §2-§4, `Design/model-document-0.8.0.md` and `Design/model-document-0.9.0.md`): `schemas/model.schema.yaml` describes the desktop's model document, `document_kind` `openpipestress.product_preview.model` at version `0.9.0` (owner OD-4, 2026-10-23: the desktop's model document is canonical; the desktop open/save and mechanics solve paths refuse 0.8.0 and older by name). 0.9.0 adds the optional `condition_sets`, a pipe segment's `condition_set_ref`, a load case's `condition_use`, a single-DOF support's `direction` and a result-state subtraction's `difference_basis` (Design `model-document-0.9.0.md` §1-§2). 0.8.0 adds ``
3. **New commitment, inserted after the DEL-02-01:1 bullet (anchor `- **DEL-02-01:1** —`, 1 occurrence).**
   - **New:** `- **DEL-02-01:2** — Model document 0.9.0 (survey item C1 with the solver lane's direction and difference basis; Agent 0, 2026-10-27). Carry operating-condition sets, the per-pipe set reference, the load case's condition use, a single-DOF support's direction and a subtraction's difference basis in one version step under the one-version rule; 0.8.0 is retired and refused by name, and nothing is migrated. The document stores each pipe's set explicitly; inheritance along a run belongs to authoring tools. A case's condition use resolves at read into the per-case records the existing solve consumes, and a document solves exactly as its resolved twin except for the identities bound to the authored document. A missing set, condition or value refuses the case by name; nothing is defaulted. Design: Design/model-document-0.9.0.md. Implementation, D3 registration and tests remain open.`

### 7.2 DEL-05-01 (`execution/PKG-05/DEL-05-01/ScopeOfWork.md`)

1. **CLM-006, Thermal row, 1 occurrence.**
   - **Old:** `Reference temperature, material property provenance, expansion policy, and broader thermal behavior remain bounded to future sealed scopes.`
   - **New:** `Reference temperature, material property provenance, expansion policy, and broader thermal behavior remain bounded to future sealed scopes, except that a load case may take each pipe's temperature change from its operating-condition set and the set's installation temperature (DEL-05-01:2).`
2. **New commitment, inserted after the DEL-05-01:1 bullet (anchor `- **DEL-05-01:1** —`, 1 occurrence).**
   - **New:** `- **DEL-05-01:2** — Operating-condition resolution (survey item C1; model document 0.9.0; DEL-02-01 Design model-document-0.9.0.md §1.5-§1.8). A load case's condition use resolves, before validation, into the records the existing solve consumes: per-pipe thermal temperature changes (the exact difference of the condition temperature and the set's installation temperature, rounded once to binary64 kelvin), per-pipe added-weight primitives and wind-exposed pipes, in a fixed order with deterministic ids. A case takes its temperature, pressure and wind exposure from conditions or from its own records, never both; a set's added weight adds to the case's own weight primitives. A missing set, condition or value refuses the case by name; nothing is defaulted. Pressure and contents resolution remain refused by name until designed with DEL-04-07. Implementation and independent numerical review remain open.`

### 7.3 DEL-07-02 (`execution/PKG-07/DEL-07-02/ScopeOfWork.md`)

1. **DEL-07-02:13, 1 occurrence.**
   - **Old:** `No model-document member is added. Design: Design/run-sheet-entry.md.`
   - **New:** `The sheet adds no model-document member of its own; its Condition set column writes model document 0.9.0's pipe condition-set reference through assign_condition_set and inherits like Material (DEL-02-01 Design model-document-0.9.0.md §4.1). Design: Design/run-sheet-entry.md.`
2. **DEL-07-02:14, 1 occurrence.**
   - **Old:** `operating-condition sets await C1.`
   - **New:** `assigning an operating-condition set (assign_condition_set, model document 0.9.0) joins the bulk changes with C1's authoring slice.`
3. **New commitment, inserted after the DEL-07-02:14 bullet (anchor `- **DEL-07-02:14** —`, 1 occurrence).**
   - **New:** `- **DEL-07-02:15** — Operating-condition sets and support direction in the tree, tables and inspector (C1; model document 0.9.0). Create, edit and delete condition sets and their keyed conditions; show and assign a pipe's set; enter an optional direction on single-DOF translational line stops and springs. Values are entered, never suggested or defaulted, and refusals are shown by name. Design: DEL-02-01 Design model-document-0.9.0.md §4. Implementation, tests and native verification remain open.`

### 7.4 DEL-07-03 (`execution/PKG-07/DEL-07-03/ScopeOfWork.md`), which owns load-manager editing (DEL-07-02 CLM-038)

1. **New commitment, inserted after the DEL-07-03:4 bullet (anchor `- **DEL-07-03:4** —`, 1 occurrence).**
   - **New:** `- **DEL-07-03:5** — Condition use and difference basis in the load manager (C1 and the solver lane's 1b; model document 0.9.0). Select a case's condition keys and set-level uses, show the engine's per-pipe resolution and refusals by name, offer "operating" among kind suggestions, and set a difference basis only on a result-state subtraction, written only when the user sets it. Nothing is defaulted. Design: DEL-02-01 Design model-document-0.9.0.md §4.2. Implementation and tests remain open.`

### 7.5 Version-token text (at S1's merge; no commitment changes in substance)

- **DEL-02-05** (5 edits): line 33 `` `0.8.0`, defined by DEL-02-01 Design `model-document-0.7.0.md` and `model-document-0.8.0.md` under the owner's OD-4 ruling (2026-10-23) and W-15 slice c (OD-2), `` → `` `0.9.0`, defined by DEL-02-01 Design `model-document-0.7.0.md`, `model-document-0.8.0.md` and `model-document-0.9.0.md` under the owner's OD-4 ruling (2026-10-23), W-15 slice c (OD-2) and the 0.9.0 step, `` (1); line 34 `` `0.6.0` and `0.7.0` are retired `` → `` `0.6.0`, `0.7.0` and `0.8.0` are retired `` (1); line 35 `` `current` for exactly `0.8.0`; `retired_schema` for any other parsed version not above `0.8.0` `` → the same with `0.9.0` (1 occurrence, 2 tokens); line 38 `` Current code: `0.8.0` is `` → `` Current code: `0.9.0` is `` (1) and `` Current implementation (W-15 slice c, #1649): `RETIRED_MODEL_SCHEMA_VERSIONS` in `model_format.rs` names `0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0` and `0.7.0`, and the `model_document_version.rs` tests refuse all seven by name. `` → `` Current implementation (the 0.9.0 step, #<PR>): `RETIRED_MODEL_SCHEMA_VERSIONS` in `model_format.rs` names `0.1.0`, `0.2.0`, `0.3.0`, `0.4.0`, `0.5.0`, `0.6.0`, `0.7.0` and `0.8.0`, and the `model_document_version.rs` tests refuse all eight by name. `` (1); line 180 `` only `0.8.0` reports `current` `` → `` only `0.9.0` reports `current` `` (1). Line 36 (the dated ruling) is unchanged.
- **DEL-04-08** (3 edits, each anchor 1 occurrence): line 108 `` current model 0.8.0 with the explicit `3.0.0/exact_pressure_v3` contract; `` → `` current model 0.9.0 with the explicit `3.0.0/exact_pressure_v3` contract; ``; line 122 `` current model 0.8.0 with the `3.0.0/exact_pressure_v3` contract; `` → `` current model 0.9.0 with the `3.0.0/exact_pressure_v3` contract; ``; line 262 `DEF-E and the code admit current model 0.8.0 with` → `DEF-E and the code admit current model 0.9.0 with`. TBD-002's held sentence is unchanged.
- **DEL-03-01** line 145: `` is `0.8.0` and `0.1.0`–`0.7.0` are refused by name `` → `` is `0.9.0` and `0.1.0`–`0.8.0` are refused by name `` (1).
- **Unchanged:** DEL-03-05 line 56 ("model document 0.8.0" names the version that introduced the slot).

**Not proposed here:** DEL-05-02 (`difference_basis` semantics) and DEL-04-03
(`direction` in the kernel) receive their commitment text with the solver lane's
1b and skew-kernel designs. Reconciliation pointers (not commitments): the
`deliverable.yaml` conditions that name the current version, `b7-plan.md`
"Current model format", `RELEASE_NOTES_v0.1.0_DRAFT.md` and `docs/user_guide`.

### 7.6 Exact wording files

The exact Old and New texts of §7.1-§7.5's long items are kept, one file per
item, in the session scratch `scratch/w17-c1/` beside the repository (not
committed); `.old.txt` is the exact anchor and `.new.txt` the exact replacement
or inserted bullet, with no trailing newline. Only the line-33 pair of
DEL-02-05 ends with a space, which is mid-line on main. Every Old anchor was
counted on main `66a83a2b68`: each occurs once.

| File | sha256 |
|---|---|
| `DEL-02-01_item1_CLM-006-line84.new.txt` | `1771d186c6720678051ca855edd3601d9a33df4ff774fae76b7f1c4166a42dd0` |
| `DEL-02-01_item1_CLM-006-line84.old.txt` | `082b2cb1d8e1a4062446629c73c8dcde4a07b4bdb7f80d5212ef91cfede48775` |
| `DEL-02-01_item2_CLM-006-line93.new.txt` | `dfec0d40c591755ea35af2b5dee6a94b30b734a5be5a9e105617776861611eb3` |
| `DEL-02-01_item2_CLM-006-line93.old.txt` | `e250fcf6ae5a7770ec018398c79eca84c3aad4478982714477b1008ef01ea34a` |
| `DEL-02-01_item3_new-DEL-02-01-2.new.txt` | `b0d0cdc7da8bdb528e95bb8fb0e960dfaa5e9300e081631bc3c6c4f8bafae696` |
| `DEL-02-05_item1_line33.new.txt` | `731450d776bbc7aa9860448b773e08cb37b08605c90a4044a8f45b51a28593f7` |
| `DEL-02-05_item1_line33.old.txt` | `2fc4282167e8ac92fea390b485b1f5018d3427f2a0e1e689c4a8fa10f07eee72` |
| `DEL-02-05_item4b_line38.new.txt` | `20488c46240381ddff6a622d41e26e325ba4f2a325144031e04108e91804d173` |
| `DEL-02-05_item4b_line38.old.txt` | `c2776d22b75cf59cebeab46d865cc35f6f0ee7532e3d639493000caef43b69bf` |
| `DEL-05-01_item1_CLM-006-thermal.new.txt` | `5f8d5d13e49993d22cc8c76048ecd1167f19c5f1fb4dee0316923e4739fe54de` |
| `DEL-05-01_item1_CLM-006-thermal.old.txt` | `9e123a80f3b349537cdb762f7ac7b7ca5d6995871843d7536c6c3bec6833dfb6` |
| `DEL-05-01_item2_new-DEL-05-01-2.new.txt` | `ab7c8ce6f55bc441482d33be1e3baf989c87368aa4d2ce0425f4af4e59ec6e14` |
| `DEL-07-02_item1_DEL-07-02-13.new.txt` | `f4c5db6e32922157270a07525ba43a6af021d2c3d08fa98a1676d111b8375d1d` |
| `DEL-07-02_item1_DEL-07-02-13.old.txt` | `b1310542f920071276685af6eeb546b8b4aeb2d8c461c22f62d3fc0a2dc5d71f` |
| `DEL-07-02_item2_DEL-07-02-14.new.txt` | `f7dec8ef44a9b85ec6fa227eb3384bf7c11a9ad52b5cf4541ffc25b9358cdeaf` |
| `DEL-07-02_item2_DEL-07-02-14.old.txt` | `a04025f025486854e798ec3fd1a17d0f41b706c5ad002d1a521edbb74d12e7ab` |
| `DEL-07-02_item3_new-DEL-07-02-15.new.txt` | `9c56ec280858af1413b28ffb11761d4723268bf8bd5b42ceb5728d30c3cd60f9` |
| `DEL-07-03_item1_new-DEL-07-03-5.new.txt` | `27a775b10aba47a4aa29bf9101858cab1acacbfee99ae11b106cdd884c5ef80b` |

## 8. Coordination

- **0.9.0 members.** C1's members, `direction`, `difference_basis` and the
  members of §2.4 are the whole list, frozen by Agent 0 on 2026-10-31
  (§2.4). Anything further is one planned 0.10.0 step. Any other lane that needs a member joins through Agent 0
  before S1's ripple; after it, the next member is a 0.10.0 step.
- **Order.** S1 after #1678 in the pricing queue; the solver lane's 1b and skew
  kernel after S1; S2 after S1; S3a after S1 (it can run beside S2); S3b after
  S3a; S4 after S2, with DEL-04-07.
- **B1.** The run-sheet column lands with S3b, or with B1's own slice if B1
  lands after S3a.
