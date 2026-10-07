# I79 — T3-SI1b repair round 01: RV104 N-1 and N-2

TASK (Type 2) I79, continued by ROOT for the repair round that RR's last section names ("RV104 passes SI1b; a small test repair round; notes routed"). The basis is RV104's review, `R/REVIEW_RV104/si1b_01/REVIEW.md` (sha256 `689c5e1cd6605cbe…`, verified): §Findings N-1 and N-2, and §4.5 with its mutant table. My RETURN.md and its `SHA256SUMS` are unchanged; this round's files are covered by `SHA256SUMS.repair_01`.

Placeholders are as in RETURN.md (`WT`, `NUM`, `P`, `R`, `VENV`). No machine paths are recorded.

## 1. Head and commit

**New head `0730c87aef796ebb6213b327d433e22b687fa529`** on `codex/piping-t3-si1b-20261006` in `WT/s-i1b`. Its parent is the previous head `966113396e`. It is not pushed; ROOT pushes.

**`0730c87aef`, "Pin the ratio block's scope and its stop, and complete a doc list".** It changes one file, `P/core/rules/expression_evaluator/src/lib.rs` (+79 −2). The sha256 goes from `1e380b2ae43990bb…` to `07d9a7a525d6d151…`.
- 2 changed doc-comment lines (N-2).
- 2 new tests in `mod tests` (N-1).
- No other line changes: there is no executable change, and no change to any other file.

`cargo fmt --check` is clean on the crate.

## 2. N-1: the three surviving mutants are now killed

| RV104 mutant | New assertion | Reproducer |
|---|---|---|
| **R2**: also block a non-finite quotient over a dimensionless divisor | `non_finite_quotients_outside_the_ratio_arm_still_carry_their_value` | `actual / 1e-308` (ratio literal), with `actual` = 1e308 stress. It must give exactly `Quantity { value: +inf, Stress, "stress_unit", true, true }`, with no findings, as on main. |
| **R3**: also block a non-finite derived (unique) quotient | the same test | `moment / length`, with 1e308 Moment over 1e-308 Length. It must give exactly `Quantity { value: +inf, Force, "moment_unit/length_unit", true, true }`, with no findings, as on main. |
| **R6**: the ratio block continues with a 0 ratio instead of `None` | `a_blocked_ratio_stops_the_enclosing_expression` | `(a/b)/(c/d)` with d = 0 (RV104's example), and `a/b + missing`, where `a/b` = 1e308/1e-308. The findings must equal exactly `[NonFiniteInput, "divide", "same-dimension quotient (ratio) must be finite"]`, with value `None` and `source_variable_ids == ["a", "b"]`. That rules out a later `DivisionByZero` or `MissingVariable`, and any evaluation of `c`, `d` or `missing`. |

**Mutants at the new head** (`_run_records/repair_01/mut_results_repair_01.json`, `mutant_logs_summary.txt`; `si1b_mutants.py` now also carries RV104's R2, R3 and R6).
- **Method.** R2 and R3 are RV104's patch strings verbatim. R6 has the same effect as RV104's: it replaces the ratio block's `return None;` with `return Some(EvaluationValue::Quantity(ratio_quantity(0.0)));`.
- **Kill rule.** A mutant is killed when the evaluator's `cargo test --lib` fails and still compiles. The runner file `point_path_non_finite_run` is also recorded.
- **Baseline.** Unmutated: 57 passed, and the runner file passes.
- **Result: 13 of 13 killed.**

| Mutant | Killed by (evaluator `--lib`) | Runner file |
|---|---|---|
| Q1 quotient check removed | the quotient tests, the I73-input test and `a_blocked_ratio_stops…` (they panic) | fails |
| Q2 quotient check only for ±inf | `blocks_overflowing_same_dimension_quotient…` | passes |
| Q3 quotient code → `DivisionByZero` | the quotient tests, the I73-input test and `a_blocked_ratio_stops…` | fails |
| Q4 quotient check before the unit check | `same_dimension_quotients_that_did_not_panic_are_unchanged` | passes |
| S1 step NaN check removed | the NaN-table tests and the generated-input test | fails |
| S2 step check widened to `!is_finite` | `non_finite_table_arguments_that_did_not_panic_are_unchanged` | passes |
| I1 interpolation NaN check removed | the NaN-table tests and the generated-input test | fails |
| I2 interpolation check widened to `!is_finite` | `non_finite_table_arguments_that_did_not_panic_are_unchanged` | passes |
| T1 NaN-argument code → `TableOutOfRange` | the NaN-table tests and the generated-input test | fails |
| T2 exact lookup also blocks NaN | `non_finite_table_arguments_that_did_not_panic_are_unchanged` | fails |
| **RV104 R2** | **`non_finite_quotients_outside_the_ratio_arm_still_carry_their_value`** | passes |
| **RV104 R3** | **`non_finite_quotients_outside_the_ratio_arm_still_carry_their_value`** | passes |
| **RV104 R6** | **`a_blocked_ratio_stops_the_enclosing_expression`** | passes |

So my 10 earlier mutants are still killed, and RV104's three survivors are killed by the new assertions, each by exactly the new test aimed at it.

## 3. N-2: the doc comment

`FindingCode::NonFiniteInput` now reads: "A value that must be finite is not: a variable binding, a literal, a same-dimension quotient (ratio), a NaN interpolation or step-lookup argument, or an interval binding end (interval mode). Always blocking."

That is the complete list of emitters in the crate's non-test code: the binding check, the literal check, the ratio block, `nan_table_argument`, and `build_interval_overlays` ("interval binding ends must be finite").

## 4. No behaviour change

**The differential, at the same harness bytes as RETURN.md §5** (`harness_sha256.txt`). I compared the previous candidate (`da0758064e`, whose Rust bytes equal `966113396e`'s) against the new head `0730c87aef`, in a `git archive` tree of each. **Every dump is byte-identical** (`identity_vs_previous_head.txt`):

| Dump | Lines | sha256 (both) |
|---|---|---|
| I73's 36,069 inputs, point mode | 36,069 | `f2a59dcf3e7a60c9…` |
| Set 2 (extreme), point | 36,000 | `5ce0f68ad653c6ad…` |
| Set 3 (table-rooted), point | 12,000 | `80f57a9b007c4a7f…` |
| Interval mode of the three sets | 144,176 / 144,000 / 48,000 | `d9eb9732…` / `b4aeff03…` / `7f8c7c15…` |
| Panic side files | 0 / 0 / 0 | empty |
| Runner, I73's fixture rows | 32,821 | `697b31fd1c32c420…` |
| Runner, extreme set (plain, b = 0, bounded) | 39,438 | `9b88cc365db7824c…` |

**Suites at `0730c87aef`** (target `WT/targets/i79-si1b-cand`; `rep_suite_*.log`):

| Suite | Base `f8ed4f0551` | Previous head | New head |
|---|---|---|---|
| `expression_evaluator` lib / conformance / doc | 49 / 1 / 0 | 55 / 1 / 0 | **57** / 1 / 0 |
| `rule_check_runner` (lib, acceptability, interval_bounds, invented_demo, point_path_non_finite_run, rule_interval_cases) | 14 / 4 / 11 / 3 / — / 1 | 14 / 4 / 11 / 3 / 2 / 1 | the same |
| `rule_pack_document` lib / corpus_parity / invented_demo_document | 6 / 1 / 3 | the same | the same |

The only count change is the 2 added tests. No test was removed against the base or the previous head, and every test passes.

**pytest:** not run this round. The Python files are unchanged since `966113396e`, and the round names only the three Rust suites.

**RV104 N-6, on my earlier runs:** my RETURN-round `run_py.sh` set neither `OPENPIPESTRESS_CHECKED_JSON_BIN` nor `OPENPIPESTRESS_UNITS_BIN`. So the session conftest ran its own `cargo build --locked --release` outside the T3 lock, as RV104 found (16:54:48Z–16:54:56Z, with no overlapping lock job). My cargo runs all went through `WT/tools/t3_cargo.sh`, but that build did not. Any future pytest run of mine will set both variables or run under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.

## 5. Host

- Every cargo command went through `WT/tools/t3_cargo.sh` (via `WT/scratch/i79_si1b/cargo_run.sh`) with `--offline --locked`.
- There was no DEC-025, sweep, native job or install.
- `TMPDIR` was `WT/scratch/i79_si1b/tmp`, and nothing went to the system temp directory.
- The only Git writes were the one commit on the branch in `WT/s-i1b`. Nothing was pushed, and nothing was committed in NUM.
- Scratch trees: `WT/scratch/i79_si1b/cand2` (the new head) and `…/mut` (the mutant tree, restored after each mutant).

## 6. Run records (`_run_records/repair_01/`)

- `identity_vs_previous_head.txt` and `harness_sha256.txt`;
- `si1b_mutants.py`, `mut_results_repair_01.json` and `mutant_logs_summary.txt`;
- `rep_suite_expression_evaluator.log`, `rep_suite_rule_check_runner.log` and `rep_suite_rule_pack_document.log`;
- `rep_diff_ee.log` and `rep_diff_rcr.log`;
- `rep_fmt_apply.log` and `rep_fmt_check.log`.
