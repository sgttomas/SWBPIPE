**piping(T3 SI1c): point-path rule evaluation blocks at an overflowing or NaN intermediate instead of deciding on it**

On the ordinary point path, a rule formula whose arithmetic overflowed or produced NaN could still decide a pass or a fail. For example, `NaN ≠ 100` and `inf ≥ 100` read as decided. Interval mode reads the same formulas as indeterminate. This PR makes the point path block, with the existing `NonFiniteInput` finding, at the operation whose result is not finite. This is the owner's decision (option D, a repair within grammar 1.0.0). It also names a non-finite caller value or limit in the runner (N-4). Every other input gives byte-identical results.

## What changes

- **`expression_evaluator`:** every arithmetic and interpolation producer checks that its result is finite. If it is not, the producer blocks with `NonFiniteInput` and a message naming the operation. The producers are add and subtract, multiply, divide, and each interpolation step. SI1b's ratio block is kept.
- **`rule_check_runner`:** a caller-supplied value that is NaN or infinite, before or after unit normalization, is named with one blocking `NonFiniteInput`, never bound, and noted. An existing note keeps its place, and the N-4 note follows it. A non-finite value-slot limit is named with the slot as the finding's subject. This applies in point and bounded runs alike. No status changes.
- **Comments and the README** state the `Logical` and `Select` evaluation order. The Python point-path oracle in `tests/test_rule_interval.py` follows the same rule.

## What stays the same

- **Every input with no non-finite intermediate and no non-finite caller value or limit gives byte-identical results,** in the evaluator and in `run_rule_checks`. Two independent differentials checked this, over about 370,000 point-path evaluator lines and 400,000 runner lines in all.
- **The interval evaluator is unchanged:** all 433,221 interval evaluation lines in the independent differential are identical. In a bounded rule run, a check that binds a solver bound reads in interval mode and changes only where N-4 applies. A check that binds no bound reads on the point path, so option D applies to it as above.
- **No new finding code, and no grammar version, conformance-corpus, schema, dependency or lock change.**
- **The rules crates are outside the F2a D1 milestone's build,** so the registered identity is untouched.

## Review and gates

- **Independent review (RV111):** PASS, with 0 blocking findings.
  - Every one of the 88,155 evaluator lines where main computed a non-finite intermediate now blocks at its producer, as an instrumented copy of main predicts. On main, 46,902 of those had been decided booleans.
  - A repair round added the tests RV111 asked for, and RV111 confirmed it.
  - These are agent reviews, not personal review by the owner.
- **The package** is `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/SI1C/`, with the change record and a citation index.
- **Gates run before the merge, and recorded on the integration branch:**
  - source equality;
  - citations;
  - GEN-8;
  - hosted CI with the full-SHA dispatch;
  - the Mac DEC-025 against a fresh main baseline.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
