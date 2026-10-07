# T3-SI1c: the point path blocks at a non-finite intermediate: change record

T3 (numerical integrity), node T3-SI1c. Records live on the integration branch `codex/piping-numerical-integrity-20260926` (NUM). This package is the PR's only execution content.

## 1. What the PR contains

Five maintained files under `projects/chirality-piping/`. Their maintained diff from main equals SI1c's branch at `f5665f8862`, and NUM carries the same blobs (`source_equality.py`).

| File | Change | sha256 (prefix) |
|---|---|---|
| `core/rules/expression_evaluator/src/lib.rs` | +823 / −139: the finiteness check at every arithmetic and interpolation producer (`finite_result`, `interpolate_point`); doc comments; 7 new tests, 4 renamed and 1 split in two (57 → 65) | `bfa5878cb140c15f` |
| `core/rules/expression_evaluator/README.md` | +9 / −2: the evaluation-order and finiteness lines | `08499c9664830d7d` |
| `core/rules/rule_check_runner/src/lib.rs` | +87 / −4: N-4. A non-finite caller value is named and never bound; a non-finite limit is named | `c4ed16f9db4483ea` |
| `core/rules/rule_check_runner/tests/point_path_non_finite_run.rs` | +534 / −11: 9 new runner tests, and 1 renamed with its expectation revised (2 → 11) | `4ad39fd8e99ac8b2` |
| `tests/test_rule_interval.py` | +23 / −5: the Python point-path oracle aligned to the same rule | `6bcd6508d8006fd7` |

## 2. What it does

- **The problem.** On the ordinary point path, a rule formula whose arithmetic overflowed past `f64::MAX`, or produced NaN, still decided a pass or a fail. For example, `NaN ≠ 100` and `inf ≥ 100` read as decided booleans, and a finite value over an overflowed divisor read as 0. Interval mode reads the same formulas as indeterminate.
  - I79 found this during T3-SI1b. SI1b had repaired only the sites that panicked.
  - It changes public meaning, so the remedy went to the owner. **The owner chose option D, "block at overflow", as a repair within grammar 1.0.0** (2026-10-07; RR "Owner decision: SI1c is option D, a repair within grammar 1.0.0; …"). There is no grammar version bump and no conformance-corpus change.
- **D, in `expression_evaluator`.** Each producer checks its result after its existing structural checks. If the result is not finite, it pushes the existing blocking `NonFiniteInput` finding, with the producer as subject and a cause-specific message, and returns no value. The producers are:
  - add and subtract;
  - multiply, in all three arms;
  - divide, in the dimensionless-divisor and derived arms;
  - each of interpolation's six floating steps.

  The same-dimension ratio arm keeps SI1b's block. So no point-path result rests on a non-finite intermediate.
- **N-4, in `rule_check_runner`** (RV104's N-4, I87's variant N4-1). The runner tests a caller-supplied value and a value-slot limit before and after unit normalization. A non-finite input is named with one blocking `NonFiniteInput`, never bound, and noted. If the input already carries a note, N-4's note follows it after "; ". A non-finite limit is named with one blocking `NonFiniteInput` whose subject is the slot. This applies in point and bounded runs alike. No status, diagnostic or relation changes.
- **N-5, comments only:** the `Logical` and `Select` evaluation-order wording, and the README.
- **The Python oracle** (`point_value`'s float branch) blocks on the same non-finite steps. Every assertion is unchanged.

## 3. What it does not do

- **No byte changes for other inputs.** Every input with no non-finite intermediate and no non-finite caller value or limit (N-4) gives byte-identical results, in the evaluator and in `run_rule_checks`.
- **The interval evaluator is unchanged, byte for byte** (`evaluate_interval`). In `run_rule_checks_with_bounds` with b > 0, a check that binds a bound is byte-identical except where N-4 applies. A check that binds no bound follows the point path, so D applies to it as in point runs (RV111 ADDENDUM_03 R-1; I88 counted 1,513 such lines).
- **No new finding code, and no schema, dependency, lock, src-tauri or desktop change.**
- **Outside the F2a D1 milestone's build closure** (the rules crates; `R/I61/u8_plan_01/PLAN.md` §3), so there is no Pass B.
- **A test comment's "I87 §2.4"** means `R/I87/si1c_plan_01/PLAN.md` §2.4, the plan's comparison truth table. `check_citations.py` does not parse that form, and the path resolves on NUM. Record names elsewhere in comments (I79's reproducers, RV104's probe 29, RV111's input) are evidence labels, not document citations.

## 4. Review

- **I87 planned it,** and ROOT ruled decisions 4–15. **I88 implemented it**, with a differential over I79's and RV104's committed harnesses plus a new SI1c family:
  - 226,336 point and 906,300 interval evaluator lines, and 300,594 runner lines;
  - an instrumented copy of main as the oracle;
  - all six pass conditions hold, with 0 violations.
- **RV111 reviewed it independently,** with its own generator, an instrumented base, and a Python transcription:
  - all 88,155 evaluator lines where main computed a non-finite intermediate now block at their producer (46,902 had been decided booleans and 21,868 finite quantities);
  - everything else is byte-identical: 56,252 evaluator lines, 60,174 runner lines, and all 433,221 interval evaluation lines. The bounded runner lines differ only where an N-4 value is present;
  - N-4 holds on 35,304 runner lines, with no status change;
  - **Verdict:** PASS, 0 BLOCKING, 1 SHOULD-FIX, 3 NOTE.
- **The repair round** (I88 REPAIR_01, with ROOT's rulings 2 and 3):
  - two runner tests pin N-4's one finding per input, and the trimmed-unit raw test (RV111's SF-1);
  - the N-4 note is appended to an existing note, with a pin for each kind;
  - a test is split, and test names now say what they pin (N-2).

  RV111 confirmed the round, with its own mutants and harness and no residual finding (ADDENDUM_01). Its mutants are killed by assertions. Its harness found that every changed runner line differs only by an appended note, and that the interval evaluation lines are identical to main's.
- **Mutants:** I88 ran 57 at the head, and 40 were killed. The 17 survivors are equivalent: reasoned, and with 0 of 808,318 differential lines differing. RV111 confirmed I88's equivalences, and its own note mutants (replace, prepend, separator, existing-only) and SF-1's N06 and N09 are killed by assertions.

## 5. Gates

| Gate | Result |
|---|---|
| Independent complete-diff review (RV111) | PASS (0/1/3); repair round confirmed (ADDENDUM_01) |
| Suites against main | `expression_evaluator` lib 57 → 65; `rule_check_runner` 35 → 44 (`point_path_non_finite_run` 2 → 11); `rule_pack_document` 10; Python `test_rule_interval.py` 193. Every count change is an added test or a split. The renames are listed in I88's records (RETURN §5, REPAIR_01) |
| `source_equality.py`, `check_citations.py`, GEN-8 on the exact head, hosted CI with the full-SHA dispatch, and the exact-head Mac DEC-025 against a fresh main baseline | See the merge record |
| Pass B | Not applicable: the rules crates are outside PP's closure |

## 6. Open obligations carried forward

- **S-I2:** binding solver bounds into rules accounts for SI1c.
- **RV111's N-3** (predates SI1c; outside this PR) goes to S-I2's planning. The desktop parses a library magnitude string such as "NaN", "inf" or "1e400" into a non-finite value, and the whole rule-check command then fails closed before the runner.
