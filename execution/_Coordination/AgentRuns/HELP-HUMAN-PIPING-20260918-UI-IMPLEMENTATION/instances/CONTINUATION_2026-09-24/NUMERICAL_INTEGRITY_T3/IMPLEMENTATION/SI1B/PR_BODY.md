**piping(T3 SI1b): point-path rule evaluation blocks instead of panicking on non-finite ratios and NaN table arguments**

On extreme magnitudes, rule evaluation's ordinary point path could panic, crashing the check, instead of reporting a finding. This PR makes those inputs return the existing blocking `NonFiniteInput` finding. Every other input gives byte-identical results.

## What changes

- **`expression_evaluator`:** two sites now return a blocking `NonFiniteInput` finding with a cause-specific message:
  - an overflowing same-dimension quotient (a ratio);
  - a NaN argument to `interpolate` or a step `lookup`.

  An infinite table argument still reads `TableOutOfRange`, and an exact lookup of NaN still reads `TableKeyNotFound`.
- **`rule_check_runner`:** new tests show that a check which used to panic now blocks, and that the rest of the run is still evaluated.
- **`tests/test_rule_interval.py`:** the Python oracle's comments no longer say the point path panics. No behaviour changes.

## What stays the same

- **Every input that did not panic gives byte-identical results.** That holds for the evaluator and for `run_rule_checks`, checked by two independent differentials over more than 600,000 evaluations.
- **Interval mode is unchanged.** S-I1's shared cases and the Python parity pass.
- **No new finding code, and no schema, dependency or lock change.**
- **The rules crates are outside the F2a D1 milestone's build,** so the registered identity is untouched.

## Review and gates

- **Independent review (RV104):** PASS (0 blocking, 0 should-fix).
  - With its own generators, no panic is reachable on the candidate. On main, the same inputs panicked 5,877 times in the evaluator.
  - A repair round added assertions for its three surviving mutants.
  - These are agent reviews, not personal review by the owner.
- **The package:** `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/SI1B/`, with the change record and a citation index.
- **Gates run before the merge and recorded on the integration branch:** source equality, citations, GEN-8, hosted CI with the full-SHA dispatch, and the Mac DEC-025 against a fresh main baseline.
- **Not in this PR:** a boolean rule over a NaN or infinite intermediate can still be decided on the point path. That changes outputs for inputs that do not panic today, so it is a separate, planned item (T3-SI1c).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
