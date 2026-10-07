# I88 — T3-SI1c return: the point path blocks at a non-finite intermediate (option D), with N-4 and N-5

TASK (Type 2) I88, an implementer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path; I delegated nothing. Brief: `R/BRIEFS/SI1C_IMPLEMENT.md` (sha256 `4bb3f92ffc4ec2a5…`, verified before I relied on it). Specification: I87's plan `R/I87/si1c_plan_01/PLAN.md` (sha256 `0eb2459d9018e22c…`, verified; its `SHA256SUMS` 5 of 5 OK), with the owner's decision D within grammar 1.0.0 and ROOT's decisions 4–15 (RR, the two sections named in the brief). This is the single return; no stop fired.

Placeholders are as in the dispatch: `WT`, `NUM = WT/numerics`, `P = projects/chirality-piping`, `EE`, `RCR`, `T`, `R`, `RR`, `VENV`. No machine paths are recorded here or in `_run_records/`.

## 0. In brief

- **Head `7f233b2e017f1f62f7f1ad9c044f21f947c8acda`** on `codex/piping-t3-si1c-20261007` in `WT/s-i1c`: four commits over main `025c1cf326`, five files, nothing pushed (§2).
- **D (§3.1).** Each producer site pushes `NonFiniteInput` with its existing subject and §3's message and returns no value when its result is not finite: add/subtract; multiply in all three arms; divide in the dimensionless-divisor and derived arms; each of interpolation's six floating steps. The ratio arm is unchanged.
- **N-4, variant N4-1 (§3.2).** Raw and normalized caller values and limits are tested; a non-finite input carries no `value`, the note, and one blocking `NonFiniteInput` naming it; a non-finite limit names its slot in both limit blocks. No status and no diagnostic changes (verified on every N-4 check in the differential).
- **N-5 (§3.3)** is its own comment-only commit: the evaluator's non-test code with comment lines removed is byte-identical to main's.
- **Differential (§4): all six pass conditions hold, with 0 violations,** over 226,336 point and 906,300 interval evaluator lines and 300,594 runner lines (all content-checked, including RV104's hash-only bounded lines through an all-full variant that reproduces RV104's dump byte for byte). The instrumented base's dumps equal the plain base's byte for byte, and the base dumps equal I79's and RV104's recorded hashes.
- **Suites (§5):** `expression_evaluator` lib 57 → 64, `rule_check_runner` 35 → 41, `rule_pack_document` 10 → 10, `test_rule_interval.py` 193 → 193. Two tests renamed (§3.5).
- **Mutants (§6):** 53 (I87's 17, D1–D10 and N1–N4; 15 of mine; 21 of SI1b's, re-run): 36 killed by the committed tests, 17 survive, and each survivor is equivalent (reasoned, and 0 of 808,318 differential lines differ).
- **For ROOT (§7):** the raw non-finite value in another unit stays unsupplied (`supplied: false`) so that no status changes; the N-4 note replaces an existing note; a second rename; comment edits beyond I87's list; an interpolation that rounds past `MAX` from finite rows now blocks; equivalent mutants; my reading of pass condition 4.

## 1. Basis read (origins and sha256 prefixes)

| Record | sha256 |
|---|---|
| `NUM/AGENTS.md`; `NUM/agents/AGENT_TASK.md` | `f96feb19d297c74e…`; `1a13a5b00b3ce01f…` |
| `R/BRIEFS/SI1C_IMPLEMENT.md` | `4bb3f92ffc4ec2a5…` |
| `R/I87/si1c_plan_01/PLAN.md` (all; `SHA256SUMS` 5/5 OK) | `0eb2459d9018e22c…` |
| `RR`: "I87's SI1c plan verified; SI1c changes public meaning, …" and "Owner decision: SI1c is option D, …"; "T3's gate set and Git rules, consolidated …" | file `0c41c577deec3640…` when re-hashed at this return (append-only) |
| `R/I79/si1b_01/RETURN.md`; `REPAIR_01.md`; I79's harness files (`_run_records/harness/`) | `44d82fc89d45cfb5…`; `a44a274dbb7b995e…`; as in I79's `SHA256SUMS` |
| `R/REVIEW_RV104/si1b_01/REVIEW.md`; `ADDENDUM_01.md`; RV104's harness files (`evidence/harness/`) | `689c5e1cd6605cbe…`; `cd6ee1b0c398c370…`; as in RV104's `SHA256SUMS` |
| `EE`, `RCR`, `P/core/rules/rule_check_runner/tests/point_path_non_finite_run.rs`, `P/core/rules/expression_evaluator/README.md`, `P/tests/test_rule_interval.py` at main `025c1cf326` | `07d9a7a525d6d151…`, `3f58533fc46e8f14…`, `de59c4177135ac10…`, `9a582419937b54b5…`, `46ee5156d657039d…` (equal to I87 §1) |
| `P/schemas/rule_check_run_result.schema.json` (read by the differential) | at main, unchanged |

NUM was at `d69b5599e7` when these records were written; I read no NUM content beyond the records above.

## 2. Head and commits

Branch `codex/piping-t3-si1c-20261007` in `WT/s-i1c`, from main `025c1cf326f1b9ef827626f27d0e2b9cf5b415fe`. **Head `7f233b2e017f1f62f7f1ad9c044f21f947c8acda`.** Author and committer are the configured identity; each message carries the agent co-author line. Not pushed (ROOT pushes).

| Commit | What | Files (`+`/`−`) |
|---|---|---|
| `3fea0678df` | N-5, comments only: the `Logical` and `Select` doc comments, their two inline comments, the interval-mode select comment, the README line | `EE` +19 −11; README +5 −2 |
| `35a14b76d8` | D in the evaluator: the producer checks, `interpolate_point`, `finite_result`, the doc comments D needs, the README line, 7 new and 6 revised tests | `EE` +779 −110; README +4 |
| `62712b001c` | N-4 in the runner, 6 new and 1 revised runner test | `RCR` +83 −4; runner test +358 −8 |
| `7f233b2e01` | The Python point-path oracle aligned to D (decision 9) | `test_rule_interval.py` +23 −5 |

**Files at head** (base → head sha256): `EE` `07d9a7a525d6d151…` → `a6766e684119f084…`; README `9a582419937b54b5…` → `08499c9664830d7d…`; `RCR` `3f58533fc46e8f14…` → `ce2281e25cafa40f…`; `point_path_non_finite_run.rs` `de59c4177135ac10…` → `5c1b4e03ea746e5f…`; `test_rule_interval.py` `46ee5156d657039d…` → `6bcd6508d8006fd7…`.

**The fence.** `git diff --name-only 025c1cf326 7f233b2e01` lists exactly those five files, all in I87 §6.2's write set. No schema, `Cargo.toml`, lockfile, src-tauri, desktop, conformance-corpus, `SPEC.md` or `TYPES.md` change. In `EE`, the non-test part changes by +112 −34 code lines and +47 −20 comment lines (rustfmt splits each producer call over six lines); the rest is tests.

**Formatting.** `cargo fmt --check` on the evaluator crate is clean at head (`_run_records/suites/fmt_cand_ee.log`). In `RCR` only my hunks are formatted: `rustfmt --check` shows the same two pre-existing drift hunks on base and head and nothing else (`rustfmt_check_{base,cand}_rcr_lib.txt`); the runner test file is clean.

## 3. The changes and their evidence

### 3.1 D: every arithmetic and interpolation result must be finite (`35a14b76d8`)

**The rule.** A new helper `finite_result(value, subject, message, findings)` pushes `NonFiniteInput` and returns `None` when `value` is not finite. It is called after each producer's existing structural checks:

| Site | Placed after | Subject | Message |
|---|---|---|---|
| `add_or_subtract` | the dimension and unit checks | `add_subtract` | "sum or difference must be finite (it overflowed)" |
| `multiply`, dimensionless-left, dimensionless-right and derived arms | (none; the derived arm after `dimension_product` resolves) | `multiply` | "product must be finite (it overflowed)" |
| `divide`, dimensionless-divisor arm and derived (unique-quotient) arm | the zero-divisor check; the derived arm after `dimension_quotient` | `divide` | "quotient must be finite (it overflowed)" |
| `divide`, same-dimension ratio arm | — | `divide` | **unchanged**: "same-dimension quotient (ratio) must be finite" |
| interpolation | the NaN-argument and range checks, and the exact-row check | the trimmed table id | "interpolated table value must be finite (a step of the interpolation overflowed)" |

- **Interpolation** now computes `low + (high − low)·((x − x_low)/(x_high − x_low))` one step at a time in `interpolate_point` (rise, offset, run, fraction, product, sum, in that order, as interval mode's `interpolate_segment` does) and returns `None` at the first non-finite step. The operations and their order are those of the old single expression, so every finite result is bit-identical (the differential confirms it).
- **No consumer ever sees a non-finite value:** bindings, literals and table rows were already checked, `negate`/`abs` and `min`/`max`/`select`/lookups produce nothing new, and every producer now blocks. So comparisons, `not`/`and`/`or`, both `select`s, `min`/`max`, a divisor, a table argument and the final value never decide over an infinity or NaN.
- **Unreachable and kept (decision 11):** SI1b's NaN table-argument checks, the ratio arm's carried-operand case, `TableOutOfRange` for an infinite argument, and the synthesized compare's literal block for a formula value. The `nan_table_argument` doc comment and the ratio-arm comment now say so (§7 item 4).
- **Doc comments D needs:** `FindingCode::NonFiniteInput`'s list gains "an arithmetic or interpolation result (an overflow)"; the interval header's "where the point path computes infinities or NaN, and can fail on them" reads "where the point path blocks"; the four `interval_tests` comments that described the point path as computing infinities or panicking now say it blocks. README gains: "An intermediate result that is not finite (an overflow of an arithmetic or interpolation step) is a blocking `NonFiniteInput` finding at the operation that produced it, …".

**One finding of my own while writing the tests.** An interpolation between finite rows can overflow at the **sum** step: rows (−1e20, 3·2^970), (1, `MAX`) at x = 0.5 give rise = `MAX − 3·2^970` rounded up one ulp, a fraction that rounds to exactly 1, and a sum that ties past `MAX` to +inf. Main computes +inf there; D blocks it. The run and sum checks are the load-bearing ones; the rise, offset, fraction and product checks are each implied by another (§6, equivalent mutants). A test pins the sum case.

**Evidence.** The seven new evaluator tests (§3.5) and the differential (§4).

### 3.2 N-4, variant N4-1: a non-finite caller value or limit is named, never bound (`62712b001c`)

| Case | Before (main) | Now |
|---|---|---|
| Formula input NaN/±inf in the declared unit, or ±inf after normalization (1e300 GPa → +inf Pa) | `bound_inputs`: `supplied: true, value: null`; evaluator: `MissingRequiredValue` ×2 (plus any other evaluator finding); `RULE_EVALUATOR_ERROR` | `supplied: true`, `value` omitted, note "non-finite value (NaN or ±inf, after unit normalization): not bound"; one blocking `NonFiniteInput` per such input, subject the input id, message "supplied value must be finite (NaN or ±inf after unit normalization)"; the evaluator is not called; status, diagnostic and relation as before. Same in point and interval mode (inputs are resolved before the mode is chosen) |
| The same input not referenced by the formula | blocks nothing | blocks nothing; carries the note and no value |
| A raw NaN/±inf in a different entered unit | `UnitMismatch` ("… unit conversion failed: quantity must be finite, got NaN"); unsupplied; completeness blocks (`RULE_INPUT_MISSING`) | tested before normalization: `NonFiniteInput` naming the input replaces `UnitMismatch`; the note; **still unsupplied (`supplied: false`)**, so completeness blocks exactly as before (§7 item 1) |
| A slot limit NaN/±inf, raw (either unit) or after normalization | "value-slot limit has missing or unknown unit/dimension metadata" (`RULE_EVALUATOR_ERROR`), or `UnitMismatch` | `NonFiniteInput`, subject the slot id, message "value-slot limit must be finite (NaN or ±inf after unit normalization)"; status, diagnostic, relation and `computed_value` as before |

- **Where.** The input tests sit in `run_one_check`'s resolution loop (raw, before normalization; normalized, after) and its formula-binding loop (one finding per non-finite formula input, then `blocked_after_completeness` before the mode is chosen). The limit tests sit in `resolve_limit`, which both limit blocks call, so the two metadata blocks are untouched and now fire only for real metadata problems (§7 item 7).
- **No JSON `null`.** `value` is `value.filter(is_finite)`; with D, `computed_value` is never non-finite.
- **The finding code** is `format!("{:?}", FindingCode::NonFiniteInput)` from the evaluator, as the runner maps every evaluator finding.

**Evidence.** The runner tests (§3.5); the differential's 92,199 (all-full RV104) + 20,898 (I79) + 6,010 (SI1c) N-4 checks, each rebuilt from the base check and compared exactly, with no status, diagnostic or relation change (§4.2, conditions 1, 4 and 6).

### 3.3 N-5: the evaluation-order wording, comments only (`3fea0678df`)

The `Expression::Logical` and `Expression::Select` doc comments now read as I87 §5.2 proposes; the two inline comments in `eval_expression` and the interval-mode select comment say the same, and so does the README line. **Check (RV104 ADDENDUM_01's method):** `EE` at `3fea0678df` against main with every `//`, `///` and `//!` line removed is byte-identical (code-only sha256 `5841f4ec1735988c…` on both sides, 4,661 lines; `_run_records/differential/n5_code_only_check.txt`). The commit changes no other file than the README.

### 3.4 The Python oracle (`7f233b2e01`, decision 9)

`point_value` (float branch only) now raises `Block` on a non-finite sum, difference or product, on a non-finite quotient in every arm, and on any non-finite interpolation step. The exact-rational branch and every assertion are unchanged. 193 passed before and after, with the same 193 test ids (§5).

### 3.5 Tests

**New evaluator tests** (`mod tests`, exact `finding_records` throughout):

| Test | Pins |
|---|---|
| `each_producer_blocks_where_its_result_overflows` | sum; difference (−inf); multiply in its three arms (ratio × stress, stress × ratio at −inf, force × length); divide in the dimensionless-divisor arm (stress/ratio, ratio/ratio) and the derived arm (moment/length); interpolation rise (RV104 probe 29's table), run (rows ±`MAX`, x = 0, which main read 0) and sum (§3.1) |
| `the_carried_non_finite_reproducers_block_at_the_multiply` | I79's three: `not((inf−inf) > 100)`, `(inf−inf) ≠ 100`, `inf ≥ 100` (each `true` on main) |
| `absorbing_forms_block_instead_of_deciding` | `max(NaN, s)`, `min(inf, s)`, `s / inf` in all three divide arms, `select` taken and untaken |
| `the_comparison_truth_table_is_unreachable` | I87 §2.4's eight operand pairs × six operators, each blocked at its first producer |
| `finite_boundaries_still_evaluate` | `MAX + 0`, `MAX·1`, `MAX/2 + MAX/2`, an underflow to a subnormal and to 0, `−0 + −0`, `MAX/1`, `5e-324/MAX`, a wide finite interpolation; exact bits |
| `a_producer_blocks_after_its_structural_checks_and_stops_the_expression` | unit and dimension mismatch, an unrepresentable product and quotient, `DivisionByZero` (incl. −0) first and alone; `(x·1e300) + missing`, `(x·1e300)/(c/0)` and `and(x·1e300 > 1, missing > 1)` give exactly one finding with `source_variable_ids == ["x"]` |
| `interval_mode_still_reads_these_overflows_indeterminate_without_a_finding` | the same inputs in interval mode, with no overlay and with a b = 0 overlay: U with `non_finite_enclosure`, no finding |

**Revised evaluator tests** (each pinned SI1b's carry scope, which D reverses; decision 12):

| Test | Change |
|---|---|
| `blocks_overflowing_same_dimension_quotient_instead_of_panicking` | the carried +inf and NaN numerators now block at `multiply` (sources `["actual"]`) |
| `same_dimension_quotients_that_did_not_panic_are_unchanged` | "a finite numerator over a carried infinite divisor is 0" now blocks at `multiply`; the other three cases unchanged; name kept (§7 item 3) |
| `non_finite_quotients_outside_the_ratio_arm_still_carry_their_value` | **renamed** `non_finite_quotients_outside_the_ratio_arm_block_at_divide`; asserts the quotient finding for stress/ratio and moment/length |
| `blocks_nan_interpolation_and_step_lookup_arguments_instead_of_panicking` | blocks at `multiply` |
| `non_finite_table_arguments_that_did_not_panic_are_unchanged` | **renamed** `non_finite_table_arguments_block_at_the_overflowing_multiply`; NaN, +inf and −inf in all three modes block at `multiply` |
| `blocks_generated_nan_table_arguments` | blocks at `multiply` (`t_259_1`) and at `add_subtract` for `z + z` (`t_2856_3`) |

Unchanged, as I87 says: `a_blocked_ratio_stops_the_enclosing_expression`, `blocks_an_i73_differential_quotient_input`.

**Runner** (`point_path_non_finite_run.rs`; every new test asserts that no serialized outcome carries a `null`, and the plain run equals b = 0 where it runs the point path):
- revised: `a_nan_table_argument_check_blocks` (interpolate, step and exact all block at `multiply` with the product message; exact used to read `TableKeyNotFound`); name kept;
- new: `a_boolean_check_over_an_overflow_blocks_at_the_producer` (I79's reproducer; a finite control still passes), `a_quantity_check_over_an_overflow_blocks_with_no_computed_value` (`max` absorption and the final overflow), `a_non_finite_input_is_named_and_never_bound` (1e300 GPa, NaN and +inf through the Rust API, point and b = 0.5), `a_non_finite_raw_input_in_another_unit_is_named_and_still_unsupplied`, `an_unreferenced_non_finite_input_blocks_nothing`, `a_non_finite_limit_is_named_in_both_limit_blocks` (1e300 GPa, NaN, NaN in kPa; point and interval).

## 4. The differential

### 4.1 Method

- **Trees.** `git archive` copies of `P/{core,examples,fixtures,tests,schemas}` at main `025c1cf326` (base) and at the head (candidate) in `WT/scratch/i88_si1c/trees/`, each with a fresh target `WT/targets/i88-si1c-{base,cand}`. The candidate tree's five changed files equal the worktree's.
- **The instrumented base** (`trees/ibase`, target `i88-si1c-ibase`) is the base plus a side channel written by `si1c_instrument_base.py` (its diff is `_run_records/instrumented_base/ibase_vs_base.diff`). Per top-level `evaluate` it records the first D producer site whose result is not finite (`add_subtract`, `multiply`, `divide` in the two D arms, or the table id when any of the six interpolation steps is not finite), the first consumer that then receives a non-finite operand, and the count; per `run_rule_checks_with_bounds` call it records, per check, the formula's record, the synthesized comparison's, and every N-4 value (raw or normalized, input or limit, same or converted unit). Records go to a side file, aligned to the dump lines by call order and test thread; their counts equal the line counts of every dump (36,069, 36,000, 12,000, 125,133 and 17,134 evaluator records; 32,820, 39,438, 216,478, 11,858 and 216,478 runner records). **The instrumented base's dumps equal the plain base's byte for byte, on all 16 dumps.**
- **Inputs, all at their recorded hashes:**
  - I79's point harness (`i79_point_diff.rs`, `18e220f0029ce892…`, rebuilt from main's conformance prefix `1c06803d7cf462a8…` and I79's committed tail): I73's 36,069 inputs, set 2 (36,000), set 3 (12,000), each also in interval mode;
  - I79's runner harness (`i79_run_diff.rs`, `59712a323846aafc…`): the fixture rows (32,821 lines with the header) and the extreme set (39,438);
  - RV104's `rv104_ee_diff.rs` (`080d4237ec986254…`): 125,133 point and 501,588 interval evaluations; `rv104_run_diff.rs` (`c3ed27e883b7f8a0…`): 216,478 lines in 7 modes;
  - **RV104's runner harness, all-full variant** (`rv104_run_full.rs`): the five `full` arguments set, nothing else, so the bounded modes are dumped as JSON too. Rebuilding RV104's recorded format from it (keeping only the FNV-1a hash where RV104 did) reproduces RV104's dump byte for byte on both sides (base `31dc84473bc88028…`, candidate equal to the candidate's `rv104_run.txt`), with no FNV mismatch;
  - **the new SI1c family** (`si1c_family.rs`): 14 producer forms (every site and arm, a length product for the derived divide-by arm, interpolation rise, run, offset and sum) × +inf, −inf and NaN (`inf − inf`, `0·inf`, and `inf/inf` for ratios) × depths 1–3, against 32–36 consumers each (the six comparisons both ways, `not`, `and`/`or` both sides, boolean `select` as condition and taken/untaken, quantity `select` taken/untaken in both branches, `min`/`max` first and last, divide-by in all three arms, the three table modes, the final value), under an overflowing, a negative and a finite binding set, plus 34 boundary controls: 17,134 point and 68,536 interval lines. Its runner part: 11 producer formulas × 7 check forms × 22 value sets (overflow, finite controls, NaN/±inf, overflow at normalization, raw non-finite in another unit, for x, s, an unreferenced u and the limit) × 7 modes, all full JSON: 11,858 lines.
- **Oracle and checks:** `si1c_compare.py` (VENV python with `jsonschema` 4.26.0), `si1c_status_tally.py`.
- **Base identity.** The base dumps equal the recorded ones: I79's repair-head dumps (`f2a59dcf…`, `5ce0f68a…`, `80f57a9b…`, intervals `d9eb9732…`, `b4aeff03…`, `7f8c7c15…`, runner `697b31fd…`, `9b88cc36…`) and RV104's main-behaviour dumps (`be1266be…`, `31dc8447…`). So the base is main's point path as measured by I79 and RV104.

### 4.2 Pass conditions

| # | Condition | Result |
|---|---|---|
| 1 | Every line with no flag and no N-4 value is byte-identical, in every mode | **Holds.** Evaluator: 175,201 unflagged point lines identical (and every interval line, condition 3). Runner: every unflagged, non-N-4 line identical (32,820 + 19,654 + 130,864 + 5,330); and inside differing lines, every check with neither a flag nor an N-4 value is identical (24,405 in the all-full RV104 dump) |
| 2 | Every flagged line is blocked on the candidate, with `NonFiniteInput` last and a producer subject; no flagged line passes or fails | **Holds**, more strictly: the last finding equals the **first** flagged site's subject and message, the earlier findings are a prefix of the base's, sources are a subset and statuses equal. Evaluator: 51,135 flagged lines, all verified. Runner: 11,064 (all-full RV104) + 90 (I79) + 518 (SI1c) flagged checks, each `RULE_INPUTS_INCOMPLETE` with the producer finding last, no `computed_value` or `limit_value`, relation `none`, inputs and completeness unchanged |
| 3 | Interval evaluator dumps byte-identical on every line | **Holds**: 906,300 interval lines (I79 144,176 + 144,000 + 48,000; RV104 501,588; SI1c 68,536), 0 differ |
| 4 | Bounded runner lines byte-identical except N-4 lines, each checked against §5.1 | **Holds, read per check** (§7 item 8): bounded lines with neither flag nor N-4 are identical (e.g. 15,191 b1 lines in the all-full RV104 dump); every N-4 check matches §5.1; the only other bounded lines that differ (1,513 across b1, bR, bH, bN, bD) contain a point-path check flagged inside a bounded run (RV104's user-input table checks, as RV104 §4.3 found), each verified as in 2, and all 2,384 interval-mode or unevaluated checks in those lines are identical |
| 5 | Plain equals b = 0 on every candidate line | **Holds**: 28,479 (RV104), 5,670 (I79), 1,694 (SI1c) |
| 6 | Every candidate runner line validates against `rule_check_run_result.schema.json`; on the base the failures are exactly the two `null` classes | **Holds**: candidate 32,820 + 39,438 + 216,478 + 11,858 lines valid, 0 invalid. Base failures are only `bound_inputs[].value: null` and `computed_value.value: null`; on RV104's point lines, 14,851 failing lines with 18,927 `bound_inputs` nulls and 321 `computed_value` nulls, exactly I87 §2.7's figures |

**N-4 status check.** Every N-4 check (92,199 + 20,898 + 6,010) has the base's status and diagnostic. The classes: blocked at completeness as on the base, only the records changing (20,102 + 2,688 + 2,002); a formula input (61,815 + 11,895 + 2,772); a limit (10,282 + 6,315 + 494); an unreferenced input only (742, SI1c).

### 4.3 Counts by family (evaluator point lines)

| Family | Lines | Identical (unflagged) | Flagged (all verified) | Flagged, base outcome: blocked / true / false / finite quantity / non-finite quantity |
|---|---|---|---|---|
| I73's inputs | 36,069 | 36,056 | 13 | 11 / 2 / 0 / 0 / 0 |
| Set 2 (extreme) | 36,000 | 34,869 | 1,131 | 674 / 61 / 75 / 14 / 307 |
| Set 3 (table-rooted) | 12,000 | 11,130 | 870 | 856 / 0 / 0 / 14 / 0 |
| RV104 corpus | 69 | 69 | 0 | — |
| RV104 non-finite bindings | 264 | 264 | 0 | — |
| RV104 quotients | 40,920 | 23,737 | 17,183 | 10,478 / 17 / 0 / 2,310 / 4,378 |
| RV104 tables | 38,880 | 23,432 | 15,448 | 15,364 / 0 / 28 / 28 / 28 |
| RV104 generated | 45,000 | 41,448 | 3,552 | 2,244 / 312 / 325 / 154 / 517 |
| SI1c family | 17,100 | 4,164 | 12,936 | 384 / 3,544 / 5,168 / 2,400 / 1,440 |
| SI1c boundary controls | 34 | 32 | 2 (the deliberate `MAX` + half-ulp twin and its compare) | 0 / 1 / 0 / 0 / 1 |

**The flagged count replaces I87 §2.7's upper bounds** (RV104's evaluator set, point path decided over a non-finite intermediate): decided booleans **682** (329 true / 353 false) against ≤ 1,257; decided finite quantities **2,492** against ≤ 3,482. By RV104 family: generated 637 booleans and 154 quantities (≤ 921 and ≤ 413), quotients 17 and 2,310 (≤ 168 and ≤ 2,702), tables 28 and 28 (≤ 168 and ≤ 367). A further 4,923 carried a non-finite final quantity (row 9 at the runner).

### 4.4 Counts by consumer row (I87 §2.3), all evaluator families

The first consumer that received a non-finite operand on the base, against the base's outcome for the line:

| Row | Consumer | Blocked | True | False | Finite qty | Non-finite qty | Total |
|---|---|---|---|---|---|---|---|
| 1–4 | a comparison (then `not`/`and`/`or`/boolean `select`) | 710 | 2,081 | 3,294 | 24 | 0 | 6,109 |
| 5 | quantity `select`, taken | 164 | 946 | 1,543 | 637 | 938 | 4,228 |
| 5 | quantity `select`, untaken | 49 | 16 | 6 | 480 | 5 | 556 |
| 6 | `min`/`max` | 89 | 158 | 94 | 708 | 226 | 1,275 |
| 7 | a divisor (`x / inf`) | 5,541 | 112 | 227 | 2,639 | 2,558 | 11,077 |
| 8 | none: the interpolation absorbed it internally, or the base blocked before any consumer | 15,206 | 624 | 432 | 432 | 0 | 16,694 |
| 9 | the final value | 0 | 0 | 0 | 0 | 2,944 | 2,944 |
| 10 | a table argument | 7,380 | 0 | 0 | 0 | 0 | 7,380 |
| 11 | the ratio arm | 872 | 0 | 0 | 0 | 0 | 872 |
| | **All** | 30,011 | 3,937 | 5,596 | 4,920 | 6,671 | **51,135** |

By D site (first producer): `multiply` 38,874; `divide` 5,245; `add_subtract` 2,296; interpolation 4,720 (counted per dump in `diff_report.json`).

### 4.5 The runner: what D changes in public meaning

On the **point lines** (plain), the flagged checks with no N-4 value went from (`_run_records/differential/status_transitions.json`):

| Dump | `USER_RULE_CHECKED` → `RULE_INPUTS_INCOMPLETE` | `USER_RULE_FAILED` → `RULE_INPUTS_INCOMPLETE` | already `RULE_INPUTS_INCOMPLETE` |
|---|---|---|---|
| RV104 runner | 244 | 216 | 2,987 |
| SI1c family | 109 | 71 | 39 |
| I79 extreme | 0 | 0 | 45 |

That is the owner-decided change. Across all modes of the all-full RV104 dump the flagged checks by first consumer and base status are in `diff_report.json` (`compare` 329 checked / 258 failed; `divide_by` 52 / 43; `min_max` 34 / 52; `select` 34 / 35; the rest already blocked: `table_argument` 9,504, `final_quantity` 479, …).

## 5. Suites, base against head, test by test

Base `trees/base` (main `025c1cf326`), candidate `trees/cand` (head); targets `i88-si1c-{base,cand}`; each `cargo test --locked --offline` and `-- --list` through the lock (`_run_records/suites/`). The worktree itself ran the same suites before the commits (`wt_*.log`, identical counts).

| Suite | Base | Head | Change |
|---|---|---|---|
| `expression_evaluator` lib / `conformance_corpus` / doc | 57 / 1 / 0 | 64 / 1 / 0 | −2 renamed away, +9 (7 new, 2 renamed in) |
| `rule_check_runner` lib / acceptability / interval_bounds / invented_demo / point_path_non_finite_run / rule_interval_cases / doc | 14 / 4 / 11 / 3 / 2 / 1 / 0 | 14 / 4 / 11 / 3 / **8** / 1 / 0 | +6 new |
| `rule_pack_document` lib / corpus_parity / invented_demo_document / doc | 6 / 1 / 3 / 0 | the same | — |
| `P/tests/test_rule_interval.py` (VENV python) | 193 passed | 193 passed | same 193 ids |

- **Test-name delta** (`test_name_delta.txt`): removed `tests::non_finite_quotients_outside_the_ratio_arm_still_carry_their_value` and `tests::non_finite_table_arguments_that_did_not_panic_are_unchanged` (both renamed); added their new names and the seven new evaluator tests, and the six new runner tests. Every test passes on both sides.
- **pytest host rule (RV104 N-6).** Both runs set `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to my own release builds of the two binaries (built through the lock with `--locked --offline` into `WT/targets/i88-si1c-bins`) and ran under `/usr/bin/lockf -k WT/guard/cargo_job.lock`; the conftest ran no cargo.
- **Dependents.** Within `core/rules/` only `rule_pack_document` depends on the evaluator (run above); the runner's only dependent is src-tauri, outside the write set (DEC-025 covers it).

## 6. Mutants

**Method** (`si1c_mutants.py`). Each mutant patches the candidate's `EE` or `RCR` in a copy (`trees/mut`, target `WT/targets/i88-si1c-mut`); each patch applies exactly once. A mutant is killed when it compiles and the evaluator's `cargo test --lib` or the runner's `point_path_non_finite_run` fails; both always run. Unmutated: 64 and 8 pass (twice). Each survivor was then run through the SI1c family (evaluator and runner), RV104's evaluator harness and I79's point harness, and its six dumps compared with the candidate's. Results: `_run_records/mutants/mutants.json`, `mutant_logs_summary.txt`.

**I87 §6.5's set: all killed.**

| Mutant | Killed by (examples) |
|---|---|
| D1 add/subtract check removed | `each_producer_…`, `the_comparison_truth_table_…`, `blocks_generated_nan_table_arguments` |
| D2a, D2b, D2c multiply check removed per arm | `each_producer_…` (all three); D2a also 11 others, D2b also two runner tests |
| D3a, D3b divide check removed per arm | `each_producer_…`, `non_finite_quotients_outside_the_ratio_arm_block_at_divide` |
| D4 interpolation checked only on the final value | `each_producer_…` (the run case), `interval_mode_still_reads_…` |
| D5 a check pushes its finding but returns the value | `a_producer_blocks_after_its_structural_checks_and_stops_the_expression` and 10 others |
| D6 the add check before the dimension and unit checks | `a_producer_blocks_after_its_structural_checks_…` |
| D7 `MAX` treated as overflow | `finite_boundaries_still_evaluate` |
| D8 a subnormal result blocked | `finite_boundaries_still_evaluate`, and the interval soundness property |
| D9 the check moved into `eval_compare` (option A) | `absorbing_forms_block_instead_of_deciding` and 14 others |
| D10 a wrong subject (`product`) | exact `finding_records`, 11 tests |
| N1 N-4's input check removed | `a_non_finite_input_is_named_and_never_bound` (`MissingRequiredValue` returns) |
| N2 N-4 keeps `value: Some(NaN)` | the `null` check in `a_non_finite_input_…` and `an_unreferenced_…` |
| N3 the limit message reverted | `a_non_finite_limit_is_named_in_both_limit_blocks` |
| N4 N-4 blocks an unreferenced input (a status change) | `an_unreferenced_non_finite_input_blocks_nothing`, `a_non_finite_input_…` |

**Mine: all killed except the equivalent ones below.** E1 (the run step unchecked) and E6 (the sum step unchecked) are killed by `each_producer_…`'s run and sum cases; E8 (interpolation subject) and E9 (the quotient message replaced by the ratio message) by `each_producer_…` and others; N5 (the raw value not tested before normalization), N6 (a raw non-finite value in another unit recorded as supplied), N7 (no note), N8 (the limit's raw check removed), N9 (its normalized check removed) and N10 (the formula-input block skipped, so the evaluator runs) by the runner N-4 tests.

**SI1b's mutants re-run on SI1c's candidate.** The ratio arm is still reachable, so I79's Q1, Q3, Q4 and RV104's R1, R2, R3, R4, R5, R6 are killed (R2 and R3, re-anchored on SI1c's arms, are now also killed by D's quotient tests).

**Equivalent, and recorded as such (17; each 0 of 808,318 dump lines differ):**
- **The NaN/infinite table-argument mutants** (decision 11; D makes the sites unreachable): I79's S1, S2, I1, I2, T1, T2 and RV104's S1, S2, I1, I2, N1. I87 listed nine of these; I79's S2 and I2 (the check widened to `!is_finite`) are equivalent for the same reason.
- **I79's Q2** (the ratio check only for ±inf): a same-dimension quotient of finite operands is never NaN; only a carried NaN made it live, and D blocks that first.
- **E7, `is_infinite` for `!is_finite` at the producers:** from finite operands no single operation gives NaN (I87 foresaw this).
- **E2–E5, four of the six interpolation step checks removed one at a time:** a non-finite rise makes the product non-finite (caught there); offset ≤ run after rounding, so a non-finite offset makes the run non-finite (caught there); with a finite offset and a finite run > 0 the fraction is finite and at most 1; and |rise·fraction| ≤ |rise|. The run and sum checks are the load-bearing ones (E1, E6 killed). The six checks are kept, mirroring `interpolate_segment`.

## 7. For ROOT to rule on

1. **A raw non-finite value in a different entered unit stays unsupplied.** The brief says a non-finite input is recorded with `supplied: true`. For this one route I kept `supplied: false`: main already treats it as unsupplied (the conversion fails), so completeness blocks the check, with `RULE_INPUT_MISSING`, whether or not the formula uses the input. Recording it as supplied would change that diagnostic for every such check and would let a check whose formula does not use the input pass where it is blocked today, a status change the brief forbids. The cause is still named (`NonFiniteInput` replaces `UnitMismatch`), the value is absent and the note is present. The route is Rust-API only (JSON cannot carry NaN or infinity). The alternative is `supplied: true` with the completeness block kept, which would contradict completeness's own "required rule-check input is not supplied".
2. **The N-4 note replaces an existing note.** A non-finite library value loses its provenance note, and a bounded solver value its "interval ±b from receipt". Appending instead ("…; non-finite value …") keeps both; I followed the plan's single text.
3. **A second rename.** I87 lists one rename. `non_finite_table_arguments_that_did_not_panic_are_unchanged` asserted the opposite of its name after D in every case, so it is renamed too (`…_block_at_the_overflowing_multiply`). `same_dimension_quotients_that_did_not_panic_are_unchanged` keeps its name: three of its four cases are unchanged, and the fourth now blocks before the quotient is formed.
4. **Comment edits beyond I87's list:** the `nan_table_argument` doc comment and the ratio-arm comment (both described carried non-finite operands that D makes impossible), and the two `interval_tests` comments that still said the point path panics (stale since SI1b). Comments only.
5. **Interpolation that rounds past `MAX`.** With finite rows, the point formula can overflow at the sum step (§3.1); main computed +inf, interval mode reads U, D now blocks with the interpolation message. The real value is at most `MAX` there, so the message's "overflowed" describes the binary64 step, not the interpolated quantity. Noted for S-I2's texts.
6. **Equivalent mutants** (§6): SI1b's NaN-argument mutants, as I87 foresaw, and also I79's Q2, S2 and I2 (they need a NaN ratio or an infinite table argument, which D makes impossible); `is_infinite` for `!is_finite` at a producer; and four of the six interpolation step checks, each implied by another. Recorded, not repaired (decision 11 keeps the guards).
7. **The limit check sits in `resolve_limit`,** which both limit blocks call, rather than in each block. The outputs are those I87 §5.1 specifies, in both modes; the two "missing or unknown unit/dimension metadata" blocks are untouched and now report only real metadata problems.
8. **Pass condition 4, read per check.** Taken literally ("bounded lines identical except N-4 lines"), it would also have to exclude the bounded lines in which a point-path check is flagged; RV104's harness runs user-input table checks on the point path inside bounded runs. I checked those checks as in condition 2 and every interval-mode check in those lines for identity (§4.2). This is a reading, not a failure, so no stop fired.
9. **`±` in messages.** The two N-4 messages and the note use "±", as I87 wrote them; JSON carries it as UTF-8. Every other runner message is ASCII.

## 8. Host

- **Cargo.** Every cargo command went through `WT/tools/t3_cargo.sh` with `--locked --offline` (fmt without them, as RV104 did), with fresh targets `WT/targets/i88-si1c-{wt,base,ibase,cand,mut,bins}`. My jobs waited behind ROOT's DEC-025 `B6_1199726f69` and interleaved with I85's and RV109's jobs; I killed nothing (`_run_records/host/cargo_jobs_i88.txt`).
- **Waits.** One wait per job; each loop ended when the job's process was gone. No wait of mine is running.
- **pytest** as in §5 (own builds and the lock).
- **Not run:** DEC-025, evidence sweeps, native or solver jobs, installs.
- **Git.** Four commits on `codex/piping-t3-si1c-20261007` in `WT/s-i1c` (to build the comment-only commit I restored two files with `git checkout --` and re-applied the change set); no other Git writes, nothing pushed, nothing committed in NUM. The worktree is clean at the head. NUM reads used `GIT_OPTIONAL_LOCKS=0` except one early `git status`.
- **Scratch** in `WT/scratch/i88_si1c/`, with `TMPDIR` there; nothing went to the system temp directory.
- **Cleanup.** The copies `WT/scratch/i88_si1c/trees/{base,ibase,cand,mut}` and the targets `WT/targets/i88-si1c-*` are deleted. Kept in `WT/scratch/i88_si1c/`: the harnesses (`harness/`), the logs, and the base and candidate dumps and the instrumented base's side records, gzipped (`dumps/`, 41 MB); the instrumented base's dumps (equal to the base's) and the mutants' equivalence dumps (equal to the candidate's, `mutant_dump_identity.txt`) are deleted after their hashes were recorded.
- **NUM's write guard** did not refuse, so this record is at `R/I88/si1c_01/`.

## 9. Run records (`_run_records/`, sealed in `SHA256SUMS`)

- `harness/`: `si1c_family.rs`, `rv104_run_full.rs`, `si1c_instrument_base.py`, `si1c_compare.py`, `si1c_status_tally.py`, `si1c_mutants.py`, `n5_comment_only.py`, `si1c_driver.sh`, `wt_check.sh`, `sanitize_copy.py`, `stage_records.sh`, `harness_sha256.txt` (the I79 and RV104 harnesses used, at their recorded hashes).
- `instrumented_base/ibase_vs_base.diff`.
- `differential/`: `diff_report.json`, `dump_sha256.txt`, `status_transitions.json`, `bounded_lines.json`, `n5_code_only_check.txt`.
- `suites/`: the suite and list logs (base, candidate, worktree), `test_name_delta.txt`, `pytest_{base,cand}.log`, the fmt and rustfmt checks.
- `mutants/`: `mutants.json`, `mutant_logs_summary.txt`, `mutant_dump_identity.txt`.
- `host/`: `driver.log`, `cargo_jobs_i88.txt`, the binary build logs.
