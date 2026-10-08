# RV104 ADDENDUM_01: confirmation of I79's repair round for N-1 and N-2

TASK (Type 2) RV104, continued by ROOT. This addendum confirms I79's repair round against my review, `REVIEW.md` (sha256 `689c5e1cd6605cbe…`, unchanged; `SHA256SUMS` unchanged). This round's files are sealed in `SHA256SUMS.addendum_01`. Placeholders are as in REVIEW.md, and no machine paths are recorded.

## Verdict

**CONFIRMED.** There are no new findings. N-1 and N-2 are closed.

| Check asked | Result |
|---|---|
| 1. The diff is exactly the doc comment plus two tests, with no executable change | **Yes.** |
| 2. My R2, R3 and R6 are each killed by the new tests, and my other 8 are still killed | **Yes, 11 of 11.** |
| 3. My differential is byte-identical between `966113396e` and `0730c87aef` | **Yes, in full** (not a subset). |
| 4. The doc comment is now accurate | **Yes.** |

## 1. The candidate and the basis

- **Branch** `codex/piping-t3-si1b-20261006` at **`0730c87aef796ebb6213b327d433e22b687fa529`**.
  - It is the pushed head (remote ref equal), and `WT/s-i1b` is clean.
  - Its parent is `966113396e`.
  - It carries one commit, "Pin the ratio block's scope and its stop, and complete a doc list", with the agent co-author line. The message is accurate.
- **I79's record:** `R/I79/si1b_01/REPAIR_01.md`, sha256 `a44a274dbb7b995e…` (verified), with `SHA256SUMS.repair_01` 13/13 OK. I read it after checking the diff myself.
- **One stale line in it:** REPAIR_01 §1 says the head "is not pushed; ROOT pushes". That was true when it was written; ROOT has since pushed.

## 2. Check 1: only the doc comment and two tests

**Scope.** `git diff 966113396e 0730c87aef` touches one file, `P/core/rules/expression_evaluator/src/lib.rs` (+79 −2). The sha256 goes from `1e380b2ae43990bb…` to `07d9a7a525d6d151…`.

**My own split** (`evidence/addendum_01/non_test_equality.txt`), of each file into `mod tests` and the rest:
- **Non-test lines:** 3,727 on each side.
- **Non-test code**, with doc-comment lines removed, is **identical** (hash `e3a4e8baf7e1bc5a` both).
- **Every non-test difference is a `///` line:** the two lines of the `NonFiniteInput` comment.
- **The `mod tests` block** grows from 1,180 to 1,257 lines.

**Test names** (`cargo test --lib -- --list`): exactly 2 added, none removed (55 → 57):
- `a_blocked_ratio_stops_the_enclosing_expression`;
- `non_finite_quotients_outside_the_ratio_arm_still_carry_their_value`.

**Reading the two tests:**
- **The first pins the ratio block's stop.** Both expressions evaluate to findings equal to exactly `[NON_FINITE_RATIO]`, with value `None` and sources `["a", "b"]`:
  - `(a/b)/(c/d)` with `d = 0`;
  - `a/b + missing`.

  So neither the zero divisor nor the unbound variable is reached.
- **The second pins the neighbouring arms.** A stress over a `1e-308` ratio literal, and `Moment/Length` at 1e308/1e-308, each give an exact `+inf` quantity with no finding, as on main:
  - `Stress, "stress_unit"` for the first;
  - `Force, "moment_unit/length_unit"` for the second.

**Formatting:** `cargo fmt --check` is clean on the crate.

## 3. Check 2: mutants

These are my 11 patch strings, unchanged from REVIEW §4.5 (`evidence/harness/rv104_mutants.py`). I applied them to a `git archive` copy of `0730c87aef`. Each ran the evaluator's `--lib` tests and the runner's `point_path_non_finite_run` through the lock (`evidence/addendum_01/mutants_a1.json`). Unmutated: 57 passed, and the runner file passes.

| Mutant | Killed | By |
|---|---|---|
| R1: ratio check NaN-only | yes | `a_blocked_ratio_stops…`, the quotient tests, the I73-input test, the runner test |
| **R2**: also block over a dimensionless divisor | **yes** | **`non_finite_quotients_outside_the_ratio_arm_still_carry_their_value`** (alone) |
| **R3**: also block a derived quotient | **yes** | **`non_finite_quotients_outside_the_ratio_arm_still_carry_their_value`** (alone) |
| R4: ratio subject changed | yes | `a_blocked_ratio_stops…`, the quotient tests, the I73-input test, the runner test |
| R5: ratio message changed | yes | the same |
| **R6**: ratio block continues with 0 | **yes** | **`a_blocked_ratio_stops_the_enclosing_expression`** (alone) |
| S1: step subject changed | yes | NaN-table tests, the generated-input test, the runner test |
| S2: step pushes but falls through | yes | the same (panic) |
| I1: interpolation of NaN returns the first row's result | yes | the same |
| I2: interpolation pushes but falls through | yes | the same (panic) |
| N1: NaN-argument message changed | yes | `blocks_nan_interpolation_and_step_lookup…` |

**11 of 11 are killed by the committed tests.** Each former survivor is killed by exactly the new test aimed at it. This matches I79's report for R2, R3 and R6.

## 4. Check 3: the differential is unchanged

I ran my full evaluator and runner harnesses, at the same bytes as `evidence/harness/`, on fresh `git archive` copies of `966113396e` and `0730c87aef`, each with its own target (`evidence/addendum_01/dump_sha256_a1.txt`):

| Dump | Lines | `966113396e` | `0730c87aef` | The review's stored `966113396e` dump |
|---|---|---|---|---|
| Evaluator (point + interval) | 626,721 | `be1266be2f1799be…` | `be1266be2f1799be…` | `be1266be2f1799be…` |
| Runner (7 modes) | 216,478 | `31dc84473bc88028…` | `31dc84473bc88028…` | `31dc84473bc88028…` |

- **Byte-identical**, and identical to the dumps behind REVIEW.md. That also shows the harnesses are deterministic across separate builds.
- **0 panics** at the repaired head.
- **The committed suites at `0730c87aef`:**
  - `expression_evaluator`: lib 57, conformance 1, doc 0;
  - `rule_check_runner`: 14 / 4 / 11 / 3 / 2 / 1;
  - `rule_pack_document`: 6 / 1 / 3;
  - `test_rule_interval.py`: 193 passed, with both `OPENPIPESTRESS_*_BIN` variables set and no cargo build by the conftest (0 `Compiling` lines).

## 5. Check 4: the doc comment

`FindingCode::NonFiniteInput` now reads: "A value that must be finite is not: a variable binding, a literal, a same-dimension quotient (ratio), a NaN interpolation or step-lookup argument, or an interval binding end (interval mode). Always blocking."

The crate's non-test code has exactly five emitters (`evidence/addendum_01/doc_comment_check.txt`):

| Line | Emitter | Message |
|---|---|---|
| 451 | binding | "variable binding quantity must be finite" |
| 516 | literal | "literal quantity must be finite" |
| 1035 | `nan_table_argument` | "table argument must be finite: a NaN argument…" |
| 1294 | ratio | "same-dimension quotient (ratio) must be finite" |
| 1952 | `build_interval_overlays` | "interval binding ends must be finite" |

The one other occurrence after `mod tests` is in the test-only `interval_tests` module. The list is complete and each entry is correct, so **the comment is accurate**.

## 6. Host

- **Cargo:** every cargo command went through `WT/tools/t3_cargo.sh` with `--offline --locked`, sharing the lock with I83's jobs. Nothing was killed (`evidence/addendum_01/cargo_jobs_a1.txt`).
- **pytest** followed the new N-6 rule: both `OPENPIPESTRESS_*_BIN` variables set.
- **Git:** no writes. Reads were with `GIT_OPTIONAL_LOCKS=0`, plus one `ls-remote`.
- **Scratch:** `WT/scratch/rv104_si1b_01/a1/`, with `TMPDIR` there. Nothing went to the system temp directory.
- **Cleanup:** the copies `WT/rv104/{prev,rep,mut}` and the targets `WT/targets/rv104-{prev,rep,mut}` are deleted. The dumps are kept, gzipped, in scratch.

## 7. Evidence

`evidence/addendum_01/` holds:
- `non_test_equality.txt` and `doc_comment_check.txt`;
- `mutants_a1.json`, `mutant_logs_summary.txt` and `mutants_driver.log`;
- `dump_sha256_a1.txt`;
- the logs for `ee_*` and `run_*`, `suite_rep_*`, `fmt_rep_ee` and `pytest_rep`;
- `list_{prev,rep}_ee_lib.txt`;
- `chain.log`, `cargo_jobs_a1.txt`, `rv104_a1_chain.sh` and `rv104_mutants_a1.py`.
