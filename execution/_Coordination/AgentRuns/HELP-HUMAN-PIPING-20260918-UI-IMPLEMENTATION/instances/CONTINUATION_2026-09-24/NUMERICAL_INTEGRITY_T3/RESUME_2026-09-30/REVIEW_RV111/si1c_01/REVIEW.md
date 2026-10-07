# RV111: independent complete-diff review of T3-SI1c (option D, with N-4 and N-5)

TASK (Type 2) RV111, an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. I am a fresh instance, I wrote none of this change, and I delegated nothing. Brief: `R/BRIEFS/RV111_SI1C_REVIEW.md` (sha256 `a562ab15514d11b3…`, verified before I relied on it).

Placeholders are as in the dispatch: `WT`, `NUM = WT/numerics`, `P = projects/chirality-piping`, `EE`, `RCR`, `T`, `R`, `RR`, `VENV`. No machine paths are recorded here or in `evidence/`.

## Verdict

**PASS.** BLOCKING 0, SHOULD-FIX 1, NOTE 3.

The candidate `7f233b2e01` does what the owner's decision and ROOT's rulings ask:
- **No point-path result rests on a non-finite intermediate.** The candidate's point path holds a finiteness invariant (§4.1). Over 144,407 evaluator cases from my own generator, every one of the 88,155 lines where main computed a non-finite intermediate now blocks at its producer, exactly as an instrumented copy of main predicts. On main, 46,902 of them were decided booleans and 21,868 finite quantities. An independent Python transcription agrees on every case, in both modes.
- **Everything else is byte-identical:** 56,252 evaluator lines, 60,174 runner lines and all 433,221 interval lines. The bounded runner lines differ only where an N-4 value is present.
- **N-4 matches I87 §5.1** on 35,304 runner lines, with no status, diagnostic or relation change. Ruling 1's reasoning holds.
- **The schema holds** on all 106,884 candidate runner lines; no `null` remains.
- **N-5 is comment-only.**
- **The fence is exactly the five files.**

**The SHOULD-FIX is a test gap, not a code defect.** Two N-4 properties are carried by the code but by no committed test: two of my mutants survive the suites, and each changes outputs. It fits the repair round already set by rulings 2 and 3.

## Findings

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| SF-1 | SHOULD-FIX | `P/core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` (N-4 in `RCR` `run_one_check`) | **Two N-4 properties of I87 §5.1 are pinned by no committed test.** (a) **One finding per input.** Mutant N06 removes the `non_finite_inputs.contains` check. The suites pass, but a formula that lists a non-finite input twice then reports two `NonFiniteInput` findings for it: 1,896 of my runner lines differ, and probe `x_listed_twice_nan`. (b) **The raw-value test trims units as normalization does.** Mutant N09 compares `u != unit_ref` without `trim`. The suites pass, but a NaN entered as `" Pa "` for a declared `Pa` becomes "raw, other unit": it is unsupplied, so completeness blocks. An unreferenced such input then turns `USER_RULE_CHECKED` into `RULE_INPUTS_INCOMPLETE` (a status change, which N4-1 forbids), and a referenced one's diagnostic moves from `RULE_EVALUATOR_ERROR` to `RULE_INPUT_MISSING` (probes `u_nan_padded_unit_unreferenced`, `x_nan_padded_unit_referenced`). The candidate itself is correct on both (`evidence/probe/probe_cand.tsv`). | **In the repair round, add two runner test cases:** a formula whose `input_refs` list a non-finite input twice (exactly one `NonFiniteInput`); and a NaN with a whitespace-padded copy of the declared unit, both unreferenced (the check stays `USER_RULE_CHECKED`, with the note) and referenced (one `NonFiniteInput`, `RULE_EVALUATOR_ERROR`). My probe (`evidence/harness/rv111_probe.rs`) has both inputs. RV111 confirms by re-running N06 and N09. |
| N-1 | NOTE | `RCR` `run_one_check` (`note: if non_finite …`) | **Ruling 2 applies in two places.** The N-4 note replaced an existing note on 7,560 bounded solver inputs ("interval ±b from receipt") and 2,520 library inputs (the provenance note). No committed test pins an existing note, so the append needs its own pins. | In ruling 2's repair, append in both cases, and add one assertion each (a library input and a b > 0 solver input). |
| N-2 | NOTE | `EE` `mod tests`; `point_path_non_finite_run.rs` | **Four kept test names still describe SI1b's sites, which D makes unreachable:** `blocks_nan_interpolation_and_step_lookup_arguments_instead_of_panicking`, `blocks_generated_nan_table_arguments`, `blocks_overflowing_same_dimension_quotient_instead_of_panicking` (its second half now pins `multiply`), and the runner's `a_nan_table_argument_check_blocks`. None is false: each still blocks a formula built to give a NaN or overflowing argument, and none panics. But each now pins the producer, not the site its name gives. The two renames are truthful (§4.7). | Optional: rename or comment them with ruling 3's rename in the same round, for example `…_block_at_the_producer`. |
| N-3 | NOTE (predates SI1c; outside the fence) | `P/apps/desktop/src-tauri/src/lib.rs` (`parse_quantity_magnitude`, `resolve_library_value_bindings_with_connection`, `parse_library_value_bindings`) | **A library magnitude stored as a string is parsed with Rust's `f64` parser,** which accepts "NaN", "inf" and "1e400". The non-finite value is then re-serialized with `json!` (a non-finite `f64` becomes `null`). `parse_library_value_bindings` rejects the `null`, so the whole rule-check command fails with "library value binding '…' requires a numeric value" and the runner is never called. This confirms ruling 1's "Rust-API only" for raw non-finite values: the desktop fails closed before N-4. It also means such a library value gets no N-4 naming. | None for SI1c. If wanted, route to the desktop or S-I2 text work: reject non-finite strings in `parse_quantity_magnitude` with a message naming the library slot. |

## 1. Basis read (origins and sha256)

| Record | sha256 |
|---|---|
| `NUM/AGENTS.md`; `NUM/agents/AGENT_TASK.md` | `f96feb19d297c74e…`; `1a13a5b00b3ce01f…` |
| `R/BRIEFS/RV111_SI1C_REVIEW.md` | `a562ab15514d11b3…` (verified before reading on) |
| `R/I87/si1c_plan_01/PLAN.md` (§2, §3 D, §5, §6) | `0eb2459d9018e22c…` |
| `RR`: "I87's SI1c plan verified; …", "Owner decision: SI1c is option D, …", "I88's SI1c verified and ruled; …" | file `d2197d8f123553d6…` when read (append-only) |
| `R/REVIEW_RV104/si1b_01/REVIEW.md` (method, probes, N-1 to N-6) | as in RV104's `SHA256SUMS` |
| `R/I79/si1b_01/_run_records/harness/si1b_mutants.py` (I79's SI1b mutant definitions, to judge I88's equivalences) | as in I79's `SHA256SUMS` |
| `R/I88/si1c_01/RETURN.md`, **read after** my static review, my harness and my oracles were written | `7459391d847e9f51…` |
| The candidate's five files at `7f233b2e01`; main's at `025c1cf326` | §2 |
| `P/apps/desktop/src-tauri/src/lib.rs` (binding parsers, read only, for ruling 1's "Rust-API only") | at main = candidate (untouched) |
| `P/schemas/rule_check_run_result.schema.json` | at main = candidate (untouched) |

I read no other NUM content. NUM was at `a1180c37ce` at dispatch.

## 2. The candidate

- **Branch `codex/piping-t3-si1c-20261007`, head `7f233b2e017f1f62f7f1ad9c044f21f947c8acda`**, in `WT/s-i1c` (clean). Four commits over main `025c1cf326f1b9ef827626f27d0e2b9cf5b415fe`, which is the merge base: `3fea0678df` (N-5), `35a14b76d8` (D), `62712b001c` (N-4), `7f233b2e01` (the Python oracle). Each message carries the agent co-author line; author and committer are the configured identity. Each commit touches only the files its message names.
- **Five files** (main → head sha256 prefixes): `EE` `07d9a7a525d6d151` → `a6766e684119f084`; `expression_evaluator/README.md` `9a582419937b54b5` → `08499c9664830d7d`; `RCR` `3f58533fc46e8f14` → `ce2281e25cafa40f`; `rule_check_runner/tests/point_path_non_finite_run.rs` `de59c4177135ac10` → `5c1b4e03ea746e5f`; `P/tests/test_rule_interval.py` `46ee5156d657039d` → `6bcd6508d8006fd7`. My `git archive` copies carry exactly these bytes.

## 3. Method: my own oracles

All harnesses are written from scratch (`evidence/harness/`), from my own reading of the code, before I read I88's return. None of I88's, I79's or RV104's harnesses, instrumentation or dumps was used.

- **Copies.** `git archive` of each revision into `WT/rv111/base` (main `025c1cf326`) and `WT/rv111/cand` (`7f233b2e01`), with `execution/` removed (not built); then `WT/rv111/ibase` (base plus instrumentation), `WT/rv111/graft` (base code with the candidate's tests) and `WT/rv111/mut` (the candidate, for mutants). Fresh targets `WT/targets/rv111-{base,cand,ibase,graft,mut}`.
- **Oracle 1, the instrumented base** (`rv111_instrument_base.py`; diff in `evidence/differential/ibase_vs_base.diff`). A thread-local event log in a scratch copy of main's evaluator, with no change to any computed value or control flow. At each D producer (add/subtract; multiply in its three arms; divide in the dimensionless-divisor and derived arms) it records an event when the operands are finite and the result is not, with the number of findings already pushed and the source ids pushed so far. For interpolation it computes the six steps separately (rise, offset, run, fraction, product, sum) and records the mask of non-finite steps. It also records every consumer that receives a non-finite operand (comparison, negate/abs, quantity select taken/untaken, min/max, divisor or numerator, table argument, final value) for the per-row counts. **The prediction for a flagged line:** the candidate's findings are main's first k findings plus the producer's `NonFiniteInput` (exact subject and message), its value is `None`, and its sources are those pushed before the producer. **The instrumented base's dumps, minus the event column, equal the plain base's byte for byte** (all three dumps).
- **Oracle 2, an independent transcription** (`Oracle` in `rv111_ee_compare.py`). A Python re-implementation of the point path from my reading of `EE`, with the dimension-product table transcribed, in two modes: `base` (IEEE carry, as on main) and `cand` (option D). For every generated case it predicts the value bits, the ordered findings (code and subject, and the full message for every `NonFiniteInput`) and the source ids, and is compared with both Rust dumps. Its D-block classifier is compared with oracle 1's flag.
- **Oracle 3, exact rationals at the boundary.** For the boundary family, the exact rational result of each operation (Python `Fraction`) decides overflow against binary64's round-to-nearest-even threshold `2^1024 − 2^970`, and otherwise gives the correctly rounded value, signed zeros included.
- **Negative control.** I corrupted one flagged line, one unflagged generated line, one boundary line and one interval line in a copy of the candidate's dumps: all four were reported (oracle 1: 1 flagged, 2 unflagged, 1 interval; oracle 2: 3 disagreements; oracle 3: 1).
- **Families (evaluator, `rv111_si1c.rs`; 144,407 cases, each run by `evaluate` and by `evaluate_interval` with no overlay, a b = 0 overlay and a relative overlay, 433,221 interval lines):**
  - **pc, 82,814 cases: every producer × every consumer at depth 1–3.** 35 sources from finite operands: add/subtract to ±inf; multiply in all three arms (including a derived product with a dimensionless result, thermal-expansion × temperature interval); divide in the dimensionless-divisor arm (including a subnormal divisor) and the derived arm (stress, length and force results); the ratio arm (control); interpolation rise (±), offset, sum and the absorbed run; and NaN from two infinities (`inf − inf`, `inf + (−inf)`, `0·inf`, `inf·0`, `inf/inf`). Each source is used bare (depth 1), under one of 10 wrappers (depth 2: negate, abs, + finite, ×2, ÷2, select taken in either branch, select untaken, min, max) and under 36 ordered pairs of 6 wrappers (depth 3), against 47–52 consumers: the six comparisons with the value on the left, on the right and on both sides; `not`; `and`/`or` on either side; boolean `select` as condition, taken branch and untaken branch; quantity `select` taken and untaken in either branch; `min`/`max` first, last and both, alone and then compared; the value as a divisor in the ratio, dimensionless-divisor and derived arms and as a numerator in each; `+` and `×0`; and the three table modes over a regular and a ±`MAX` table.
  - **bd, 11,664 cases: finite boundaries per arm.** 36 values (±0, ±5e-324, ±`MIN_POSITIVE`, ±1e-308, ±1e-300, ±0.5, ±1, ±2, ±1e154, ±1.4e154, ±1e300, ±1e308, ±`MAX`/2, ±2^970 and its two neighbours, ±`next_down(MAX)`, ±`MAX`) squared, in nine arms (add, subtract, multiply ×3, divide ×4 including the ratio arm).
  - **tbl, 9,860 cases: interpolation steps.** Two-row tables over 9 extreme arguments (pairs with a0 < a1) × 8 extreme results each, at up to five interior points, plus a three-row table.
  - **gen, 40,000 cases:** 20,000 random typed formulas over the whole grammar (10 dimensions, derived units, select, min/max over up to three operands, interpolation and lookups over six table shapes, 4% deliberate type, dimension or unit mismatches, rare missing variables), each under two random binding sets of extreme values.
  - **corpus, 69 cases:** the committed conformance corpus (not given to oracle 2).
- **Families (runner; 106,884 JSON lines):** every pack is run plain, b = 0, b = 1, b relative, b = NaN and with a duplicate bound (the committed packs plain, b = 0 and b = 1).
  - **rpc, 10,000 single-check packs:** 10 producers over the solver input x (stress, Pa) × 4 chains × every pc consumer, boolean formulas as predicates and quantity formulas against a stress limit, at x ∈ {1e300, 1, −1e300, 1e-300, `MAX`};
  - **rn4, 6,174 packs: N-4.** 7 formulas (boolean, quantity, a sum, a library input, a formula that does not use x, a three-input `max`, and a formula listing x twice) × referenced/unreferenced extra input × three status and policy variants × 15 values (NaN, −NaN, ±inf, ±1e300 GPa, `MAX` kPa, NaN and +inf in kPa, NaN in an unknown unit, and finite controls) in x, s, the library input, an unreferenced input and the limit, one at a time and in pairs;
  - **rmulti, 72 three-check packs,** each check also run alone;
  - **rdemo:** both committed packs (`examples/rule_packs/invented_demo.yaml`, `fixtures/product_preview/invented_demo_rule_pack.json`) over a 13 × 13 × 4 × 2 grid including NaN, ±inf, ±0, subnormals and `MAX`.
- **Schema:** every runner line on both sides validated with VENV's `jsonschema` 4.26.0 (`rv111_schema.py`).
- **Host:** every cargo command through `WT/tools/t3_cargo.sh` with `--offline --locked` (`rv111_job.sh`); test profile (overflow checks on).

## 4. The review, by item

### 4.1 No point-path result rests on a non-finite intermediate (item 1)

**Static: the candidate's point path holds a finiteness invariant.** By structural induction over `eval_expression`, every quantity value it produces is finite:
- literals and bindings are checked (`EE` literal arm; `build_binding_map`);
- `negate` and `abs` of a finite value are finite;
- add/subtract, all three multiply arms, the dimensionless-divisor and derived divide arms call `finite_result` after their structural checks; the ratio arm keeps SI1b's check; division by zero blocks first; unrepresentable or ambiguous dimensions block;
- `select`, `min`/`max` and step/exact lookups return one of their (finite) operands or a validated row result;
- interpolation checks all six floating steps (`interpolate_point`).

So no consumer (a comparison, `not`/`and`/`or`, either `select`, `min`/`max`, a divisor, a table argument, the final value, the runner's synthesized comparison) ever receives an infinity or NaN. **D misses no producer:** the evaluator has no other floating operation on the point path, and the interval path calls the point helpers only with shadow values of 1.0 (sum 2, difference 0, product and quotient 1) and never calls `eval_table_expression`. The runner's only other value source, unit normalization of caller values and limits, is N-4's.

**Empirical.**
- **Oracle 1:** 88,155 evaluator lines are flagged (a D producer's result is not finite on main). **Every one blocks on the candidate exactly as predicted** (findings = main's prefix + the producer's finding, value `None`, sources as pushed before the producer): pc 80,417, bd 1,892, tbl 3,135, gen 2,711. By site: add/subtract 19,135; multiply 31,025 (dimensionless-left 19,787, dimensionless-right 5,934, derived 5,304); divide 20,056 (dimensionless divisor 12,688, derived 7,368); interpolation 17,939.
- **What main did with them:** decided `true` 21,932; decided `false` 24,970; a finite (absorbed) quantity 21,868; a non-finite final quantity 11,335; already blocked 8,050. **No flagged line passes or fails on the candidate.**
- **Oracle 2 agrees on all 144,338 generated cases in both modes** (value bits, ordered findings, messages of every `NonFiniteInput`, sources), and its D classifier agrees with oracle 1 on every case.
- **No candidate value is non-finite:** 0 of 144,407 candidate point results carry a non-finite value (main: 11,335).
- **Per consumer (pc family; every line now blocked):**

| I87 §2.3 row | Consumer | Main: true / false | Main: finite / non-finite quantity | Main: blocked |
|---|---|---|---|---|
| 1 | six comparisons (left, right, both) | 13,668 / 15,096 | — | — |
| 2 | `not` | 1,068 / 530 | — | — |
| 3 | `and` / `or` (either side) | 2,120 / 4,272 | — | — |
| 4 | boolean `select` (condition; taken or untaken branch) | 1,060 / 3,734 | — | — |
| 5 | quantity `select` taken / untaken | — | 829 / 769 taken; 3,196 / 0 untaken | — |
| 6 | `min` / `max` (alone; then compared) | 2,818 / 378 | 5,921 / 2,069 | — |
| 7 | as a divisor (three arms; then compared) | 878 / 601 | 2,877 / 136 | 255 |
| 8 | interpolation absorbing its own overflow | (in every row through the `interp_run_absorbed` source) | | |
| 9 | the final value | — | 829 / 769 | — |
| 10 | table argument (interpolate, step, exact) | — | 3,333 / 0 | 6,255 |
| — | as a numerator; as an arithmetic operand | — | 3,588 / 2,741 | 627 |

- **Consumer events across all families** (main, first non-finite operand per consumer kind, from oracle 1's log) include: comparison 21,314 lines, `min` 19,627, `max` 19,725, quantity `select` untaken 19,535 and taken 4,090, divisor 4,725, table argument 4,768, final value 11,335. Every line with such an event is flagged (a non-finite operand can only come from a D producer; the only events in unflagged lines are 2,849 ratio-arm blocks of finite operands, which block on main too), and blocked on the candidate.
- **Interpolation masks on main** (steps rise, offset, run, fraction, product, sum): `000001` 2,489 (only the sum overflows, I88's "rounds past `MAX`" case), `001000` 3,733 (only the run: the absorption that main returned as a finite wrong value), `100011` 8,743, `101011` 216, `011111` 2,686, `111111` 72. No other mask occurs, as the step analysis in §4.7 predicts.

### 4.2 Byte identity without a non-finite intermediate or an N-4 value (item 2)

- **Evaluator:** every one of the 56,252 unflagged point lines is byte-identical (pc 2,397, bd 9,772, tbl 6,725, gen 37,289, corpus 69). No panic on either side.
- **Runner (`run_rule_checks` and the bounded modes):** every line with neither a flagged point-path check nor a relevant N-4 value is byte-identical: **60,174 lines**. Multi-check lines with a flagged or N-4 check (422) are checked through their single-check runs (§4.4).
- **Oracle 3 (exact rationals, bd family):** all 11,664 lines agree: 9,296 finite results carry exactly the correctly rounded bits (signed zeros, subnormals and underflow to ±0 included), 268 of them exactly ±`MAX` and unblocked; 2,368 block (D overflow, `DivisionByZero` at ±0, and ratio overflow) exactly where the exact result reaches `2^1024 − 2^970` or the divisor is zero.

### 4.3 Interval mode is byte-identical (item 3)

- **`evaluate_interval`:** all 433,221 interval lines (three overlay variants of every case) are byte-identical, base against candidate (and the instrumented base against both). No panic.
- **`run_rule_checks_with_bounds` with b > 0:** every bounded line with no N-4 value is identical: b = 1, 11,858 of 11,858; b relative, 11,058 of 11,058. They carry 10,106 and 9,706 interval-mode outcomes (`RULE_INTERVAL_ALL_PASS`, `…_ALL_FAIL`, `RULE_RESULT_INDETERMINATE`). The bounded lines that differ (b = 1: 6,506; b relative: 5,002) all carry an N-4 value and are checked as in §4.4. No D flag occurs in a bounded line: every check with a producer binds x as an interval and runs interval mode, and the point-path checks inside bounded runs (in rmulti) have no producer.
- **S-I1's shared cases:** `rule_interval_cases` (1 test, 94 cases) passes on both sides (1 of 1 on each).
- **`test_rule_interval.py`:** 193 passed on both sides, with main's file and with the candidate's (VENV's python, under the lock).
- **Why:** the only interval-path changes are comments (§4.6). Interval mode reaches the changed producers only through `eval_binary` with shadow values of 1.0, so `finite_result` always passes there.

### 4.4 The findings tell the truth (item 4)

- **Order: structural checks precede the producer check.** Oracle 2 transcribes every structural check and its order, and agrees with the candidate on all 144,338 cases. Among them, 1,619 cases block on a structural finding at an operation whose result would not have been finite: `DivisionByZero` 838 (every zero divisor), `UnsupportedExpressionForm` 542 (multiply) and 235 (divide), `DimensionMismatch` 4 (add/subtract). In each the structural finding is reported, alone. The committed test `a_producer_blocks_after_its_structural_checks_and_stops_the_expression` pins each structural case (unit and dimension mismatch, an unrepresentable product and quotient, `DivisionByZero` including −0), and my mutants E12 to E15 (each check moved before its structural check) are all killed.
- **`DivisionByZero` precedes the arms:** in the bd family every divisor ±0 blocks with `DivisionByZero` in all four divide arms, including 0/0 and `MAX`/0 (oracle 3).
- **The first block stops the enclosing expression:** in each of the 88,155 flagged lines the candidate's last finding is the first producer's, nothing follows it, and the sources stop there (oracle 1). Examples, main then candidate (`evidence/probe/samples.txt`): `(MAX + MAX) − (MAX + MAX) ≠ 3` read `true`, now `[NonFiniteInput|add_subtract|sum or difference must be finite (it overflowed)]`; `1e300·x ≥ 3` with x = 1e300 read `true`, now the product finding; `select(false, 1e300·x, 3)` read 3, now the product finding; `3 / (x·1e300)` (the ratio arm) read 0, now the product finding; an exact lookup of `MAX + MAX` read `TableOutOfRange`, now the sum finding; an interpolation over rows (−`MAX`, 0), (`MAX`, 10) at 0 read 0, now the table's interpolation finding; `MAX + 2^970` read +inf, now the sum finding, while `MAX + next_down(2^970)` still reads `MAX`.
- **Subjects and messages** are the producers' existing subjects (`add_subtract`, `multiply`, `divide`, the trimmed table id) and I87 §3's messages; the ratio arm keeps its SI1b message. From finite operands the only cause is an overflow (no single step can give NaN; 0/0 is a `DivisionByZero`), so "(it overflowed)" is true; for interpolation it describes the step, as ruling 5 notes.
- **N-4 against I87 §5.1 (N4-1).** 35,304 runner lines carry a relevant N-4 value (raw 33,504 values, normalized 14,508, raw in another unit 4,014). On every one, check by check:
  - the status, the diagnostic codes, the acceptability relation and any `computed_value` are main's (0 differ);
  - each N-4 input's `bound_inputs` entry has no `value`, carries the note, keeps main's `supplied` and `unit` (`supplied: true` except the raw value in another unit, `supplied: false`);
  - every `NonFiniteInput` finding naming an input or the limit carries I87's message, and no input is named twice (including the formula that lists x twice);
  - no `MissingRequiredValue` names an N-4 input, and no "missing or unknown unit/dimension metadata" finding remains for an N-4 limit.

  The classes on the candidate's rn4 lines: a formula input named with the evaluator not called 14,784; a raw value in another unit named while completeness blocks 2,394; a limit named 960; the note only (an unreferenced input, or a check that blocked earlier) 5,922; a value never read (an invalid or duplicate bound makes the input unsupplied before its value is read) 6,684, identical to main. An unreferenced N-4 input blocks nothing: 2,376 such checks stay `USER_RULE_CHECKED`, as on main.
- **Ruling 1 (a raw non-finite value in another unit stays unsupplied): the reasoning holds.** On main the conversion fails, so the input is unsupplied and completeness blocks with `missing_input` whether or not the formula uses it. Recording it as supplied would move a referenced input to the evaluator block (`evaluator_error`, a diagnostic change) and let an unreferenced one pass (a status change). My differential shows the candidate keeps main's status and diagnostic on all 4,014 such values. "Rust-API only" also holds for the desktop: supplied and solver values arrive as JSON numbers, and a library magnitude given as a string is parsed with Rust's `f64` parser, which accepts "NaN" or "inf", but the value is then re-serialized with `json!` (a non-finite `f64` becomes `null`) and the command stops at "requires a numeric value" before the runner (N-4 below).
- **Ruling 2 is needed in both places:** the N-4 note replaced an existing note on 7,560 bounded solver values ("interval ±b from receipt") and 2,520 library values (the provenance note). See NOTE N-1.
- **A multi-check run** still evaluates every check: on both sides, all 1,296 checks inside the 432 three-check runs equal the same check run alone, and the aggregate is worst-of.
- **Plain equals b = 0:** on all 19,166 packs, on the candidate and on main.

### 4.5 The schema holds (item 5)

- **Candidate: 106,884 of 106,884 runner lines validate** against `rule_check_run_result.schema.json`; no line contains `null`.
- **Main: 25,752 lines fail, in exactly the two classes I87 named:** 32,022 `checks[].bound_inputs[].value: null` (N-4) and 918 `checks[].computed_value.value: null` (row 9). No other schema failure on either side.

### 4.6 N-5 is comments only (item 6)

- **`3fea0678df` against main:** with every full-line `//`, `///` and `//!` comment and every blank line removed, the whole of `EE` (tests included) is byte-identical (`rv111_strip_comments.py`; code-only sha256 `d181061fefe1b8e4…` on both sides). The commit also changes one README paragraph, prose only.
- **The whole slice's non-test code, comments removed** (`evidence/n5/ee_code_nontest.diff`, 143 changed lines): exactly the five producer sites, `finite_result`, `interpolate_point`, the four message constants and the interpolation call site. Every other non-test change in `EE` (the `NonFiniteInput` list, the `nan_table_argument` doc, the ratio-arm comment, the interval header, the interval `select` comment) is comment text, and each now says what the code does.
- **The README's new line** is true of the code: "An intermediate result that is not finite … is a blocking `NonFiniteInput` finding at the operation that produced it, so no comparison, `min`/`max`, `select`, divisor or table argument ever decides over an infinity or NaN."

### 4.7 Tests and mutants (item 7)

**Suites, base against candidate** (`evidence/suites/`; each `cargo test --offline --locked --no-fail-fast` through the lock):

| Suite | Main | Candidate |
|---|---|---|
| `expression_evaluator` lib / conformance_corpus / doc | 57 / 1 / 0 | 64 / 1 / 0 |
| `rule_check_runner` lib / acceptability / interval_bounds / invented_demo / point_path_non_finite_run / rule_interval_cases / doc | 14 / 4 / 11 / 3 / 2 / 1 / 0 | 14 / 4 / 11 / 3 / 8 / 1 / 0 |
| `rule_pack_document` lib / corpus_parity / invented_demo_document / doc | 6 / 1 / 3 / 0 | 6 / 1 / 3 / 0 |
| `P/tests/test_rule_interval.py` | 193 passed | 193 passed |

- **Test-name delta:** removed `tests::non_finite_quotients_outside_the_ratio_arm_still_carry_their_value` and `tests::non_finite_table_arguments_that_did_not_panic_are_unchanged` (the two renames); added their new names, the seven new evaluator tests and the six new runner tests. Every test passes on the candidate.

**The new and revised tests fail on main where they should** (the graft: main's code with the candidate's tests):
- **Evaluator: 52 pass, 12 fail.** The 12 are the six new D tests and the six revised SI1b tests. `finite_boundaries_still_evaluate` passes on main, as it must: it pins behaviour D keeps. Every unchanged test passes.
- **Runner `point_path_non_finite_run`: 1 passes and 7 fail: the six new tests and the revised `a_nan_table_argument_check_blocks`. SI1b's `an_overflowing_ratio_check_blocks_and_the_run_carries_on` passes, as it should, and every other runner suite passes (14 / 4 / 11 / 3 / 1).**

**The renames are truthful.** `non_finite_quotients_outside_the_ratio_arm_block_at_divide` asserts the quotient finding at `divide` for stress/ratio and moment/length; `non_finite_table_arguments_block_at_the_overflowing_multiply` asserts the product finding in all three modes for NaN, +inf and −inf. Ruling 3's case (`same_dimension_quotients_that_did_not_panic_are_unchanged`, one of whose four cases now blocks) is set for the repair round. Four kept names still describe SI1b's sites, which D makes unreachable (NOTE N-2).

**My mutants** (`rv111_mutants.py`, `evidence/mutants/`). Each is an exact single-occurrence patch of the candidate in `WT/rv111/mut`. Killing tests: for an evaluator mutant, the evaluator's `cargo test --lib`, then the runner's `point_path_non_finite_run` if it survived; for a runner mutant, the runner crate's whole `cargo test`. Every survivor was then run through my whole differential and probe, and compared line by line with the candidate's dumps.

**I defined 47 and ran 39.** Lock contention made each mutant take 2 to 20 minutes, so I stopped my own driver while it was waiting and ran a trimmed second phase. The eight not run each repeat the kind of one that ran: I08 (only the final interpolation value checked; I03 and I88's D4 cover it), I09 (interpolation subject; E16 covers the kind), N01 and N05 (the raw or formula check removed; N02 and N07 cover them), N10 to N12 (NaN-only note or check, input message; N03, E08 and E17 cover the kinds) and L03 (limit subject).

| Site | Mutants | Result |
|---|---|---|
| add/subtract | E01 check removed; E12 check before the dimension/unit checks | killed (3 tests; 1) |
| multiply, three arms | E02, E03, E04 each arm unchecked; E13 check before the product lookup; E17 derived arm with the sum message | all killed |
| divide, two arms | E05, E06 each arm unchecked; E14 check before the zero test; E15 check before the quotient lookup; E16 subject `quotient` | all killed |
| `finite_result` | E07 pushes but returns the value; E08 NaN only; E10 `MAX` blocks; E11 subnormal blocks | all killed (E11 also by the interval soundness property) |
| `finite_result` | E09 `!value.is_infinite()` in place of `value.is_finite()` (a NaN would pass) | **survives; equivalent** (0 of 684,512 lines and 0 probe lines differ) |
| the ratio arm | E18 the ratio arm with D's quotient message | killed |
| interpolation | I03 run unchecked; I06 sum unchecked | killed (`each_producer_…`; I03 also `interval_mode_still_reads_…`) |
| interpolation | I01 rise, I02 offset, I04 fraction, I05 product unchecked; **I07 all four unchecked jointly** | **survive; equivalent** (0 lines differ each) |
| N-4 inputs | N02 raw value in another unit recorded as supplied; N03 note dropped; N04 `value` kept (a `null`); N07 the N-4 block after the interval dispatch; N08 an unreferenced input blocks | all killed |
| N-4 inputs | **N06 one finding per input (dedup) removed; N09 `trim` dropped from the raw unit test** | **survive; not equivalent** (SF-1) |
| N-4 limits | L01 raw check removed; L02 normalized check removed; L04 a non-finite limit read as missing | all killed |
| SI1b's guards | X01 step NaN check removed; X02 interpolation NaN check removed; X03 ratio check `is_infinite` only; X04 NaN-argument code `TableOutOfRange` | **survive; equivalent** (0 lines differ each) |

28 of 39 are killed by the committed tests. Of the 11 survivors, 9 are equivalent (0 of 684,512 dump lines and 0 of 5 probe lines differ) and 2 are not (N06, N09; SF-1).

**I88's 17 equivalences: confirmed, all 17.**
- **SI1b's table-argument guards (I79's S1, S2, I1, I2, T1, T2; RV104's S1, S2, I1, I2, N1; 11).** Each changes only what happens when the table argument is NaN (S1/I1/T2 of I79, all of RV104's) or non-finite (I79's S2/I2 widen the test to `!is_finite`; T1 changes the NaN finding's code). By the invariant of §4.1 the argument of `eval_table_expression` is always finite on the candidate, and interval mode never calls it, so none of these can change an output. My X01, X02 and X04 (the same sites) survive the committed tests and change 0 of 684,512 dump lines and 0 probe lines.
- **I79's Q2** (the ratio check `is_infinite` only): a same-dimension quotient of finite operands with a non-zero divisor is never NaN (0/0 is a `DivisionByZero`), and no operand can be non-finite any more. My X03 survives the committed tests and changes 0 of 684,512 dump lines and 0 probe lines.
- **E7, `is_infinite` for `!is_finite` in the producers:** no single producer step can give NaN from finite operands (add, subtract and multiply of finite values overflow only to ±inf; a finite quotient with a non-zero divisor is never NaN; each interpolation step sees finite operands because the previous ones were checked). My E09 survives the committed tests and changes 0 of 684,512 dump lines and 0 probe lines.
- **E2–E5, four of the six interpolation step checks** (rise, offset, fraction, product). With finite rows a0 < a1, finite x strictly between them and not equal to a row:
  1. offset = fl(x − a0) > 0 (gradual underflow), and offset ≤ run, because rounding is monotone and x − a0 < a1 − a0;
  2. so a non-finite offset forces a non-finite run;
  3. with finite offset and run, the fraction lies in [0, 1] and is never non-finite;
  4. with a finite rise, |product| ≤ |rise| is finite; with an infinite rise the product is ±inf (fraction > 0) or NaN (fraction = 0), and the sum r0 + product is then non-finite;
  5. so a non-finite rise, offset, fraction or product always makes the run or the sum non-finite. **These four checks are redundant not only one at a time but jointly, given the run and sum checks**, and only the run (the absorption) and the sum (the tie past `MAX`) are load-bearing.

  My differential agrees: the only interpolation masks on main are `000001`, `001000`, `100011`, `101011`, `011111` and `111111` (§4.1), each of which sets the run or the sum. My mutants I01, I02, I04, I05 (one at a time) and I07 (all four jointly) survive the committed tests and change 0 of 684,512 lines; I03 (run) and I06 (sum) are killed.

### 4.8 Scope (item 8)

- `git diff --name-status 025c1cf326 7f233b2e01` lists exactly the five files of I87 §6.2, all modified, +1,271/−140. No schema, `Cargo.toml`, lockfile, src-tauri, desktop, fixture or conformance-corpus file changes; `docs/SPEC.md` and `docs/TYPES.md` are untouched.
- **The Python oracle:** every change is inside `point_value` (four hunks, the float branch: a non-finite sum, difference, product or quotient, and any non-finite interpolation step, raises `Block`; the exact-rational branch is unchanged). No `assert` line changes. The alignment makes the soundness test stricter, not weaker: an interval T or F now also requires that the point path does not block at any sample.
- **Formatting:** `rustfmt --check --edition 2021` is clean on the candidate's evaluator `lib.rs` and on the runner test file. On the runner's `lib.rs` it reports the same two drift hunks on main and on the candidate, with identical content (moved down by the inserted lines), and nothing in SI1c's hunks.
- **Dependents:** within `core/rules/` only `rule_pack_document` depends on the evaluator (6 / 1 / 3 / 0 on both sides); the runner's only dependent is src-tauri, outside the write set (DEC-025 covers it).

## 5. I88's return, spot-checked against my own runs (read after my oracles)

Confirmed by my independent runs:
- the head, the four commits, the five files and their hashes, and the fence;
- D at every producer and arm, with I87 §3's subjects and messages, after each structural check; the ratio arm unchanged;
- N-4 as N4-1: no status, diagnostic or relation change; the raw value in another unit kept unsupplied; the limit check in `resolve_limit` for both limit blocks; no JSON `null`;
- N-5 comment-only by code equality (my strip gives the same equality, independently);
- the suite counts (57 → 64; 35 → 41; `rule_pack_document` 10 → 10; pytest 193 → 193) and the test-name delta;
- the main-side schema failures are exactly the two `null` classes;
- the sum-step interpolation overflow (rows (−1e20, 3·2^970), (1, `MAX`) at 0.5): my tbl family finds 2,489 such lines, all now blocked;
- the 17 recorded equivalences (§4.7);
- that interval mode and every unflagged, non-N-4 line are unchanged.

I did not re-run I88's harnesses or read its dumps. My instrumentation, families, transcription and exact-rational oracle are my own, so the agreement is between independent oracles.

## 6. For ROOT to rule on

1. **SF-1:** add the two runner test cases to I88's repair round, with rulings 2 and 3. RV111 confirms by re-running N06, N09 and the probe against the repaired head.
2. **N-1:** confirm that ruling 2's append covers both kinds of existing note (bounded solver inputs and library inputs), each with its own assertion.
3. **N-2:** whether to rename or comment the four tests with ruling 3's rename (optional).
4. **N-3:** whether to route the desktop's string-magnitude parse to desktop or S-I2 text work. It predates SI1c, is outside the fence, and fails closed today.
5. **Coverage:** 8 of my 47 mutants were not run, because of lock contention (§4.7). Each repeats a kind that was run and killed. I recommend not running them in the confirmation round.
6. **For the record (§7):** three host slips of mine, all corrected and none touching the candidate. An empty file in ROOT's worktree (ROOT removed it; the check was rerun with absolute paths, as ROOT asked). A file in the system temp directory, deleted within a minute. Four `cargo` invocations, from an unquoted here-document, that ran nothing.

## 7. Host and cleanup

- **Cargo:** every cargo command went through `WT/tools/t3_cargo.sh` with `--offline --locked` (via `rv111_job.sh` and `rv111_mutants.py`), into fresh targets `WT/targets/rv111-{base,cand,ibase,graft,mut}`. My jobs waited behind other T3 jobs (I85, I89, I90 and ROOT); I killed none of them (`evidence/host/cargo_jobs_rv111.txt`). `rustfmt --check` ran directly (it is not cargo).
- **Built test binaries** ran only inside `cargo test` through `t3_cargo.sh`, so under the lock; none was run directly.
- **Python analysis:** my comparison scripts (seconds each) and the two schema validations (VENV `jsonschema`, about two minutes of one core each) ran outside the lock, before ROOT restated the host rule (every heavy job under `WT/guard/cargo_job.lock`). After the restatement, every analysis over the dumps ran under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
- **pytest:** only `tests/test_rule_interval.py`, with VENV's python, under the T3 lock (`lockf` on the cargo job lock), with `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` set to unused paths so the session conftest ran no cargo (RV104 N-6); `-p no:cacheprovider`, no bytecode written.
- **Not run:** DEC-025, evidence sweeps, native or solver jobs, installs. **Git:** no writes; reads with `GIT_OPTIONAL_LOCKS=0`.
- **Waits:** one wait per job, ending with the job's process. Once I started a second wait loop on a job that already had one (the instrumented base's run) and stopped it at once; another stray loop of mine (a disowned until-loop) I found and stopped within a minute. No wait of mine is running at return.
- **Two writes outside my scratch, both mine, both corrected** (ROOT's message about the first asked me to rerun the check and record it here):
  1. **An empty file `ee_code_nontest.diff` at the root of `WT/records-pr-b` at 14:27:43Z.** A `diff … > ee_code_nontest.diff` ran with a relative output path before my `cd` into scratch; the working directory was then ROOT's worktree. The diff itself failed (its inputs were relative too), so the file was empty. ROOT removed it before squash-merging records PR #1108 and asked me to rerun the check with absolute paths. **I reran it** (main `025c1cf326` against `7f233b2e01`, and against the N-5 commit `3fea0678df`), writing only by absolute path into `WT/scratch/rv111_si1c_01/n5/`; the results are those of §4.6 (code-only sha256 `d181061fefe1b8e4…` for main and `3fea0678df`, whole file; non-test `56df3ba83d020d32…` for both; the slice's code-only non-test diff `f1265a4ee8ccc39b…`, 143 changed lines). Every later write used an absolute path, or a `cd` into my scratch in the same command.
  2. **A file named `x` in the system temp directory,** from an `awk` redirect while I located the test module; it held the candidate evaluator's non-test source (repository content). I deleted it within a minute.
- **Four `cargo` invocations outside the lock, none of which ran anything (about 14:46Z).** An unquoted shell here-document that edited my mutant script's docstring expanded the back-quoted words in that text as shell commands: `cargo test --lib` and `cargo test` (twice each) and `point_path_non_finite_run` (not a command). Each `cargo` call ran in ROOT's working directory (`WT/records-pr-b`), which has no `Cargo.toml`, and stopped at once with "could not find `Cargo.toml`": no build, no lock taken, no target and no file created (I checked the worktree). From then on every here-document of mine was quoted.
- **Scratch:** `WT/scratch/rv111_si1c_01/`, with `TMPDIR` there.
- **Deleted afterwards:** `WT/rv111/` (all five copies) and `WT/targets/rv111-*`, and the mutants' dumps once compared (`mutant_summary.json` records the result).
- **Kept in scratch:** the dumps, gzipped, with their hashes in `evidence/differential/dump_sha256.txt` (uncompressed).
- **NUM's write guard** did not refuse, so these records are at `R/REVIEW_RV111/si1c_01/`.

## 8. Evidence

`evidence/` (sealed in `SHA256SUMS`; placeholder paths only):
- `harness/`: `rv111_si1c.rs` (the differential harness), `rv111_probe.rs` (the N-4 probe), `rv111_instrument_base.py` (oracle 1), `rv111_ee_compare.py` (oracles 1–3 for the evaluator), `rv111_run_compare.py` (the runner conditions), `rv111_schema.py`, `rv111_strip_comments.py`, `rv111_mutants.py`, `rv111_mutants_phase2.sh`, `rv111_mutant_summary.py`, `rv111_job.sh`, `rv111_suites.sh`, `rv111_stage.sh`, `rv111_assemble.py`;
- `differential/`: `ee_report.json`, `run_report.json`, `schema_cand.json`, `schema_base.json`, `ibase_vs_base.diff`, `dump_sha256.txt` (dump hashes and line counts, uncompressed), `negative_control_report.json` and its README;
- `probe/`: `samples.txt` (main and candidate lines for the cases cited in §4), `probe_{base,cand,mut_N06,mut_N09}.tsv`;
- `n5/`: `ee_code_nontest.diff` (the slice's comment-free non-test diff, from the rerun) and `n5_check.txt`;
- `suites/`: the suite logs on main, the candidate and the graft, the pytest logs, the rustfmt checks, `test_names_{base,cand}.txt`;
- `mutants/`: `mutants.json`, `mutant_summary.json`, `mutant_dump_sha256.txt` (every survivor's dumps: 12 of 12 evaluator dumps and 11 of 12 runner dumps equal the candidate's; N06's runner dump does not), the per-mutant logs;
- `host/`: `driver.log`, `suites_driver.log`, `mutants_driver.log`, `mutants_phase2.log`, `cargo_jobs_rv111.txt` (my lines of the T3 lock log: 71 waits, 70 starts and 70 ends; the one wait with no start is the job I stopped while it waited).
