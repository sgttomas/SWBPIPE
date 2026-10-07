# I79 — T3-SI1b return: the point-path panic repair in `expression_evaluator`

TASK (Type 2) I79, dispatched by ROOT (HELP_HUMAN). Brief `R/BRIEFS/SI1B_POINT_PATH_PANICS.md` (sha256 `a3c8954f41e6da93…`, verified). This is the single return; step 2's checkpoint did not apply (§3).

Placeholders: `WT` is the T3 host root; `NUM = WT/numerics`; `P = projects/chirality-piping`; `T`, `R`, `RR` and `VENV` as in the dispatch. No machine paths are recorded.

## 1. Basis read (origins and sha256)

| Record | sha256 |
|---|---|
| `NUM/AGENTS.md` | `f96feb19d297c74e…` |
| `NUM/agents/AGENT_TASK.md` | `1a13a5b00b3ce01f…` |
| `NUM/P/AGENTS.md` (opening only) | `d9f2b23ad50669fd…` |
| `R/BRIEFS/SI1B_POINT_PATH_PANICS.md` | `a3c8954f41e6da93…` |
| `R/I73/s_i1_01/CHECKPOINT_1.md` (§5, §6.3) | `b992efcfa216f8da…` |
| `R/I73/s_i1_01/_run_records/checkpoint1/point_diff_tail.rs`, `final/run_diff_template.rs`, `final/i73_panic_probe.rs`, `final/ee_mutants.py` | as recorded by I73 |
| `R/REVIEW_RV99/s_i1_01/REVIEW.md` (N-2, §1.5) and `ADDENDUM_02.md` (N-10) | `48cfdbc0fc148299…`, `fc08166e2303ec7b…` |
| `RR`: "I73's checkpoint 1 and I74's plan ruled…" (ruling 3, N-2) and "#1102 merged…" (dispatch) | file `27dabf3b1526a938…` |

NUM was at `ea0e288e8a` when dispatched and at `8eaa4403a6` when these records were written.

**Host notes.**
- My turn was cut off twice, each time ROOT reported nothing killed and the lock free, and each time I re-checked my state and resumed without redoing completed steps.
  - The first, an API connection error, came after the base runs. `WT/s-i1b` was clean at base, there were no records yet, and every base output was complete.
  - The second, a network error, came while I was finishing RETURN.md. The commits, dumps, suites and mutants were complete.
- Before this return I corrected `compare_dumps.py`'s finding pattern, which had missed findings whose message contains escaped quotes (the grammar-version message). I regenerated `diff_report.json` from the same dumps. The line and panic counts were unaffected; only the per-line finding breakdown changed (§5).
- The write guard did not refuse NUM, so these records are at `R/I79/si1b_01/`.
- I made no Git writes outside `WT/s-i1b`'s branch and did not push. There were no installs, no DEC-025, sweep, native or solver jobs, and nothing went to the system temp directory (`TMPDIR` was `WT/scratch/i79_si1b/tmp`).

## 2. Head and commits

Branch `codex/piping-t3-si1b-20261006` in `WT/s-i1b`, from main `f8ed4f0551`. **Head `966113396e7e2f5f3497ccc75f0cc11262559585`.**

| Commit | What |
|---|---|
| `90d2c1ebf8` | The repair in `P/core/rules/expression_evaluator/src/lib.rs`, plus 6 evaluator regression tests |
| `da0758064e` | New `P/core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` (2 runner tests) |
| `966113396e` | Comment-only alignment of the point-path oracle in `P/tests/test_rule_interval.py` (§6) |

**Files changed** (`git diff --numstat f8ed4f0551 966113396e`):
- `expression_evaluator/src/lib.rs`: +353 −3. The sha256 goes from `fbc42fee05211c45…` to `1e380b2ae43990bb…`.
  - The 3 removed lines are the old `ratio_quantity(left.value / right.value)` call.
  - There are 5 source hunks: the `NonFiniteInput` doc comment, the step check, the interpolation check, the new `nan_table_argument` helper, and the `divide` change.
  - The rest is tests, appended to `mod tests`.
- `rule_check_runner/tests/point_path_non_finite_run.rs`: +233, new (`de59c4177135ac10…`).
- `tests/test_rule_interval.py`: +5 −3, comments only (`41f021dfb4f73bc4…` → `46ee5156d657039d…`).

There is no `Cargo.toml`, lockfile, schema or dependency change. `966113396e` touches only the `.py` file, so the Rust bytes at head equal `da0758064e`'s. Every Rust run below used `da0758064e`.

## 3. The repaired sites, their finding code, and why the code is truthful

| Site (base `f8ed4f0551`) | Panic mechanism | Now |
|---|---|---|
| `divide`, same-dimension arm → `ratio_quantity(..)` `.expect` (line 1457) | `Quantity::dimensionless` returns `Err(EvaluationError::NonFiniteInput)` for a non-finite quotient. That happens with an overflowing quotient of finite operands, or with a non-finite operand carried from an earlier operation. `.expect` panicked on it. | After the unit check, `let ratio = left.value / right.value`. If it is not finite, the evaluator pushes `NonFiniteInput` with subject `divide` and message "same-dimension quotient (ratio) must be finite", and returns `None`. |
| `eval_table_expression`, step arm, `.expect("in-range step lookup always has a governing row")` (line 968) | A NaN argument fails both `x < first` and `x > last`, so it passes the range check, and the row search finds nothing. | Before the range check, `x.is_nan()` pushes `NonFiniteInput`, with the table id as subject and the message "table argument must be finite: a NaN argument is neither inside nor outside the table range". It returns `None`. |
| `eval_table_expression`, interpolation arm, `.expect("in-range interpolation always has a bracketing pair")` (line 983) | Same mechanism as the step arm. | Same check and finding, through the shared helper `nan_table_argument`. |

**Deliberately unchanged** (each is guarded by a test):
- An infinite table argument still reads `TableOutOfRange`, in every mode.
- An exact lookup of NaN still reads `TableKeyNotFound`.
- A unit mismatch still reports first, and alone.
- A zero divisor is still `DivisionByZero`.
- The largest finite ratio (`f64::MAX / 1`) still evaluates.
- A finite numerator over a carried infinite divisor is still the ratio 0.
- The non-same-dimension divide arms still carry non-finite values, as before, because they never panicked.

After the repair, the three `.expect`s are unreachable by their own invariants. `ratio_quantity` now only sees finite values, and its unit is the constant `"ratio"`. A non-NaN `x` inside `[first, last]`, over a validated, strictly increasing, finite table, always has a governing row and a bracketing pair. I left them in place rather than add untestable branches.

**Why `NonFiniteInput` is truthful, so that step 2's checkpoint did not apply:**
1. **It is the crate's own classification of this exact condition.** The panic was `.expect` on `Err(EvaluationError::NonFiniteInput { name: "quantity", value })`, returned by the evaluator's own `Quantity` constructor. The repair turns that error into the matching blocking finding instead of a panic.
2. **The point path already uses this code for non-finite values.** That covers bindings ("variable binding quantity must be finite") and literals ("literal quantity must be finite"). In the runner, a non-finite *computed* formula quantity already blocks as `NonFiniteInput`/`literal`, through the synthesized `Compare` literal. So this is "the way the point path already blocks non-finite values" (the brief's words).
3. **There is a precedent in a sibling crate.** `P/core/loads/stress_recovery` emits `FindingCode::NonFiniteInput` for a computed value ("{subject} recovery produced a non-finite value", `checked_recovered`).
4. **For the tables, the reading is literal.** The table's input, its argument, is not finite.
5. **No other code is true.**
   - `DivisionByZero` is false: the divisor is non-zero.
   - `TableOutOfRange` is false: NaN is neither in nor out, as the brief says.
   - `TableKeyNotFound` is exact-mode only.
   - `UnsupportedExpressionForm` is false.
6. **The messages state the actual cause.** In the runner, the wire code is the `Debug` name `"NonFiniteInput"`, severity `blocking`, mapped exactly as every evaluator finding. No new public code was added.

**One addition for ROOT to see:** a doc comment on `FindingCode::NonFiniteInput`, which had none: "A value that must be finite is not: a variable binding, a literal, a same-dimension quotient (ratio), or a NaN interpolation or step-lookup argument. Always blocking." It describes where the code is emitted after this change. ROOT may prefer to drop it (§8).

## 4. Other panics found

**None reachable from a finite or non-finite numeric input**, beyond the three repaired sites.

**Static scan** of non-test code for `expect`, `unwrap`, `panic!`, `unreachable!`, indexing, slicing and integer arithmetic:
- **The point path** (`evaluate` and every helper):
  - `quantities[0]` and `[1..]` sit behind the non-empty check.
  - `table.rows[0]` and `rows.len() - 1` sit behind `validate_table`'s minimum row count, with an early return.
  - `windows(2)` indexing is safe.
  - `f64::min`/`max` accept NaN.
  - No integer arithmetic depends on a value.
- **The runner's point path** (`run_rule_checks`, `run_one_check`, `resolve_limit`, `normalize_value_to_declared_unit`, the synthesized compare):
  - No non-test `expect`/`unwrap`.
  - `Quantity::new` errors become a missing binding.
  - Unit conversion is `Result`-based, and `units::convert_for_dimension` validates finiteness.
  - `rule_pack_document::encode_dimension`'s `expect` is total over the closed `Dimension` enum, so it is not numeric.
  - `completeness_checker` and `units` have no non-test panic sites.
- **Interval mode** reaches the point helpers only with shadow value 1.0 (`IValue::shadow`), so `divide` there is 1/1, and it never calls `eval_table_expression`.

**Empirical check:**
- **Base panics.** Across all base dumps (§5), every panic is at line 1457, 968 or 983. The runner set includes caller-supplied NaN, +inf and −inf for the solver value, the limit and the slot, plus NaN and +inf bounds, and still shows no other site.
- **Candidate panics.** The candidate has 0 panics everywhere.

**Out of scope, not a panic, listed for routing** (§8, item 3): **the point path decides boolean formulas over carried non-finite intermediates.**
- Reproducer (`_run_records/probe/`, `harness/i79_carried_non_finite_probe.rs`): with `x` = 1e300 stress and `inf = 1e300·x`, the point path gives `true` with no finding for each of `not((inf − inf) > 100)`, `(inf − inf) ≠ 100` and `inf ≥ 100`. Interval mode reads all three `Indeterminate` (`non_finite_enclosure`).
- By the runner's boolean mapping, a boolean-formula check would read `USER_RULE_CHECKED` there. A quantity formula is safe, because its non-finite value blocks at the synthesized compare.
- I did not touch this. Brief rule 4 requires byte-identical results for every input that does not panic today.

## 5. The point-mode differential

**The harness.** I73's harness is `main`'s `conformance_corpus.rs` up to its first `#[test]` (704 lines). That prefix is byte-identical to I73's, and so is the 69-case corpus. I73's `point_diff_tail.rs` follows it.

**My changes to the tail** (`_run_records/harness/i79_point_diff_tail.rs`; the diff against the original is all additive except the signatures):
1. A panic hook records each panic's `file:line:col` and the input's `Debug` in a side file, `<dump>.panics`. The main dump bytes are unchanged.
2. `gen` delegates to `gen_l(r, lits, …)`, which takes its literal pool as a parameter. I73's call sequence on the RNG is unchanged.
3. **Set 2, `i79_extreme_dump`:** 6,000 formulas × 6 bindings, with extreme literals and bindings: ±1e308, `f64::MAX`, ±5e-324, 1e-308 and 1e154.
4. **Set 3, `i79_table_dump`:** 3,000 table-rooted formulas × 4 bindings, each an interpolation, step lookup or exact lookup over a depth-3 extreme ratio argument. I added it because I73's 36,069 inputs and set 2 contain no NaN table argument.
5. Every input of all three sets also runs through `evaluate_interval` with four overlay variants (points; first input ±0.5; first input with no enclosure; all inputs ±0.25), into `<dump>.interval`.

**Base `f8ed4f0551` against candidate `da0758064e`.** Trees were made by `git archive`, with identical harness files. The report is `_run_records/differential/diff_report.json`, from `compare_dumps.py`.

| Set | Lines | Base panics (site) | Lines that differ | Candidate panics |
|---|---|---|---|---|
| I73's inputs | 36,069 | 18 (all line 1457) | **18**: exactly the base panics; each is now blocked with `NonFiniteInput`/`divide` | 0 |
| Set 2, extreme | 36,000 | 276 (all 1457) | 276: exactly the base panics; each `NonFiniteInput`/`divide` | 0 |
| Set 3, table-rooted | 12,000 | 209: 186 at 1457, 4 at 968, 19 at 983 | 209: exactly the base panics. 186 are `NonFiniteInput`/`divide`; 23 are `NonFiniteInput`/table id | 0 |

**Hashes:**
- The base dump of I73's inputs has **sha256 `e74a69c43af2b277…`**, the same value I73 recorded on `c1bfc460fc`. So the point path on main equals the point path I73 measured.
- The candidate dump is `f2a59dcf3e7a60c9…`.
- Every other dump hash is in `differential/dump_sha256.txt`. The dumps themselves stay in `WT/scratch/i79_si1b/diff_ee|diff_rcr/`.

**What the differing lines contain.** In I73's 18, 13 carry only the new finding. The other 5 also carry findings that the input already raised before evaluation reached the quotient: `StatusBoundaryViolation` (4) or `UnsupportedGrammarVersion` (1). In every differing line of all three sets, the new finding is the last one, and the earlier findings are of those kinds or `MissingRequiredValue`.

**Mix.** Every other category is the same count on both sides:
- I73's inputs: true 2,669, false 2,598, quantity 5,970; blocked 24,814 → 24,832.
- Set 2: true 2,501, false 2,391, quantity 6,217; blocked 24,615 → 24,891.
- Set 3: quantity 3,620; blocked 8,171 → 8,380.

**Interval mode, all three sets** (144,176, 144,000 and 48,000 lines): **byte-identical**, with 0 panics on either side.

**The runner** (`harness/i79_run_diff.rs`): I73's `run_diff_template.rs`, with `run_under_test = run_rule_checks`, plus `i79_runner_extreme_dump`.
- **I73's fixture-row dump** (32,821 lines): **identical, sha256 `697b31fd1c32c420…`**. That is the value I73 recorded.
- **The extreme dump** (39,438 lines) covers:
  - the 10 packs I73 used, plus a two-check pack (the ratio check plus a boolean `actual <= limit` check);
  - values 14 × 9 × 4, including NaN and ±inf;
  - each in plain, b = 0, and b ∈ {0.5, 1e-300, 1e300, NaN, +inf};
  - three table packs (interpolate, step, exact) over `actual·1e300 − actual·1e300`.

  **2,436 lines differ, exactly the base panics.** By base site:
  - 2,376 are at line 1457, in the ratio packs;
  - 30 are at line 968 (step pack, plain and b = 0);
  - 30 are at line 983 (interpolate pack, plain and b = 0).

  Each is now:
  - `RULE_INPUTS_INCOMPLETE`, with one blocking `NonFiniteInput` evaluator finding (`divide`, or the table id);
  - `diagnostic_codes = ["RULE_EVALUATOR_ERROR"]`, `acceptability_relation = "none"`, no computed value.

  In the two-check pack, the other check is still evaluated: `USER_RULE_FAILED` 184 times and `USER_RULE_CHECKED` 32 times.
- **Every bounded line other than b = 0** is **identical** on both sides, and none panicked on base. That covers b ∈ {0.5, 1e-300, 1e300}, which run interval mode, and the invalid bounds NaN and +inf, which block the input.
- **`plain == b = 0`** on all 5,670 cases, on both sides (RV99 N-2's route).

## 6. The Python reference (brief step 5)

**`P/core/analysis_runs/rule_interval.py` does not mirror the point path at these sites.**
- It is interval-mode only.
- Its exact-lookup point branch takes only finite enclosures, because a non-finite end is a note.
- Its same-dimension quotient is an enclosure that becomes U when it is not finite.
- The 94 shared cases are interval-mode cases (`evaluate_interval`), so there is nothing to align, and no shared case was added.

**Another Python transcription does exist:** the soundness oracle `point_value` in `P/tests/test_rule_interval.py`.
- It already reads a non-finite quotient and a NaN table argument as `Block` (neither pass nor fail). That matches the repaired point path.
- Only its comments said the point path panics there. Commit `966113396e` corrects those three comments, with no behaviour change. 193 passed before and after.
- The oracle reads *every* non-finite quotient as `Block`, while Rust blocks only the same-dimension one and carries the others. That is the oracle's conservative reading, and it is pre-existing.

## 7. Suites and mutants

**Suites.** Base is `WT/s-i1b` at `f8ed4f0551` (target `WT/targets/i79-si1b-base`). Candidate is at `da0758064e` (target `…-cand`). Logs are in `_run_records/suites/`.

| Suite | Base | Candidate | Change |
|---|---|---|---|
| `expression_evaluator` lib | 49 | 55 | +6, all added (listed below) |
| `expression_evaluator` `conformance_corpus` / doc-tests | 1 / 0 | 1 / 0 | — |
| `rule_check_runner` lib / acceptability / interval_bounds / invented_demo / rule_interval_cases | 14 / 4 / 11 / 3 / 1 | the same | — |
| `rule_check_runner` `point_path_non_finite_run` | — | 2 | +2, new file |
| `rule_pack_document` lib / corpus_parity / invented_demo_document | 6 / 1 / 3 | the same | — |
| `test_rule_interval.py` (VENV python) | 193 passed | 193 passed (also at head `966113396e`) | same test set |

- **Dependents of `rule_check_runner` within `core/rules/`:** none. Only `apps/desktop/src-tauri` depends on it, and that is outside `core/rules/`, so it was not run.
- **Test-name sets:** no test was removed, and every count change is an added test. S-I1's 94 shared cases still pass on both sides (`rule_interval_cases`, `test_rule_interval.py`).

**The added tests:**
- **Evaluator:**
  - `blocks_overflowing_same_dimension_quotient_instead_of_panicking` (smallest reproducer 1e308/1e-308, plus −1e308 and `f64::MAX`/0.5, plus carried +inf and NaN operands);
  - `blocks_an_i73_differential_quotient_input` (I73's input `gen_400_5`, `(z·y − 1e300)/y` with y = 1e-300);
  - `same_dimension_quotients_that_did_not_panic_are_unchanged`;
  - `blocks_nan_interpolation_and_step_lookup_arguments_instead_of_panicking` (smallest reproducer `1e300·T − 1e300·T`);
  - `non_finite_table_arguments_that_did_not_panic_are_unchanged`;
  - `blocks_generated_nan_table_arguments` (set-3 inputs `t_259_1` and `t_2856_3`; all 18 of I73's inputs are quotient panics).
- **Runner:**
  - `an_overflowing_ratio_check_blocks_and_the_run_carries_on` (the committed `examples/rule_packs/invented_demo.yaml` plus a second boolean check; b = 0 must serialize identically);
  - `a_nan_table_argument_check_blocks` (interpolate and step block with `NonFiniteInput`; exact still gives `TableKeyNotFound`; b = 0 is identical).

**Formatting:**
- The evaluator crate is `cargo fmt --check` clean on base. I applied `cargo fmt` to it, and only my hunks moved.
- `rule_check_runner` is **not** fmt-clean on base (`src/lib.rs:2035`, `:2042`, pre-existing). I formatted only my new file, with `rustfmt --edition 2021` (toolchain 1.97.1), and left `src/lib.rs` alone.

**Mutants** (`harness/si1b_mutants.py`, results in `mutants/`).
- **Method.** Each patch applies exactly once to the candidate `lib.rs`. A mutant is killed when the evaluator's `cargo test --lib` fails and still compiles. The runner regression file was also run on each mutant.
- **Baseline.** The unmutated baseline passes both.
- **Result: 10 of 10 killed.**

| Mutant | Site | Killed by |
|---|---|---|
| Q1: quotient check removed | quotient | quotient tests and I73-input test (they panic); runner test also fails |
| Q2: quotient check only for ±inf | quotient | carried-NaN case |
| Q3: quotient code → `DivisionByZero` | quotient | quotient tests and I73-input test; runner test |
| Q4: quotient check before the unit check | quotient | `…did_not_panic_are_unchanged` (unit mismatch first) |
| S1: step NaN check removed | step | NaN-table tests and generated-input test; runner test |
| S2: step check widened to `!is_finite` | step | `non_finite_table_arguments…unchanged` (±inf stays `TableOutOfRange`) |
| I1: interpolation NaN check removed | interpolate | NaN-table tests and generated-input test; runner test |
| I2: interpolation check widened to `!is_finite` | interpolate | `non_finite_table_arguments…unchanged` |
| T1: NaN-argument code → `TableOutOfRange` | both table sites | NaN-table tests and generated-input test; runner test |
| T2: exact lookup also blocks NaN | exact (must stay unchanged) | `non_finite_table_arguments…unchanged`; runner test |

## 8. For ROOT to rule on

1. **Confirm `NonFiniteInput`** for both causes (§3). This is my judgment that an existing code states the cause truthfully, so I did not stop for step 2's checkpoint. If ROOT reads it otherwise, the change is three call sites and one helper, and the tests pin the code.
2. **The new doc comment on `FindingCode::NonFiniteInput`** (§3): keep it or drop it. It is documentation only.
3. **Route the out-of-scope observation** (§4): on the point path, a boolean formula over a carried NaN or infinity is decided, and can pass, while interval mode reads U. It is not a panic, and changing it would change non-panicking point results, so it needs its own node and ruling.
4. **The comment-only commit `966113396e`** to `P/tests/test_rule_interval.py`: keep it, or drop it if ROOT wants the unit limited to the two Rust crates.
5. **The pre-existing rustfmt drift** in `rule_check_runner/src/lib.rs` (lines 2035 and 2042) is untouched. It is noted in case the reviewer or CI checks formatting.

## 9. Run records

All of these are under `_run_records/`, with paths sanitized to `WT` and `VENV`.
- `tools/`: `cargo_run.sh` (the lock wrapper invocation used for every cargo command: `RUSTUP_TOOLCHAIN=1.97.1`, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `--offline --locked`, through `WT/tools/t3_cargo.sh`) and `run_py.sh` (pytest with VENV's python, memory guard checked).
- `harness/`: the harness tail, the runner harness, `compare_dumps.py`, `si1b_mutants.py`, the carried-non-finite probe, and `inputs_sha256.txt` (the prefix, I73's originals and the assembled harness).
- `differential/`:
  - `diff_report.json`;
  - `dump_sha256.txt`;
  - `i73_inputs_base_panics.tsv` (I73's 18 panic inputs, with site and input);
  - the four final differential logs.
- `suites/`: the base and candidate suite logs, pytest logs and fmt logs.
- `mutants/`: `mut_results.json` and `mutant_logs_summary.txt`.
- `probe/probe_carried_cand.txt`.
