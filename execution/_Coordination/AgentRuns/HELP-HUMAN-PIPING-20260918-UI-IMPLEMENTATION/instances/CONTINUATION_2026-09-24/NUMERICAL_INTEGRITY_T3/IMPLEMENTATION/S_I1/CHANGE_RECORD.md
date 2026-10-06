# S-I1: sound interval rule evaluation (option C): change record

T3 (numerical integrity), slice S-I1. Records live on the integration branch `codex/piping-numerical-integrity-20260926` (NUM). This package is the PR's only execution content.

## 1. What the PR contains

Eight maintained files under `projects/chirality-piping/`. Their maintained diff from main equals S-I1's branch, and NUM carries the same blobs (`source_equality.py`).

| File | Change | sha256 (prefix) |
|---|---|---|
| `core/rules/expression_evaluator/src/lib.rs` | +2,225 / −0: interval mode and its tests | `fbc42fee05211c45` |
| `core/rules/rule_check_runner/src/lib.rs` | +585 / −3: `run_rule_checks_with_bounds`, `SolverResultBound`, interval outcomes (the 3 removed lines are an import rewrap and one match arm that now also records an interval) | `3f58533fc46e8f14` |
| `core/rules/rule_check_runner/tests/interval_bounds_run.rs` | new | `6834c2114f3db48a` |
| `core/rules/rule_check_runner/tests/rule_interval_cases.rs` | new: Rust side of the shared cases | `4fd95e6024798bb2` |
| `core/analysis_runs/rule_interval.py` | new: the Python reference evaluator | `e7dd8e0bb750b7c0` |
| `fixtures/rule_interval/rule_interval_cases.json` | new: 94 shared cases | `c3709b53569e05f3` |
| `tests/test_rule_interval.py` | new: Python parity, the exact-rational soundness oracle | `41f021dfb4f73bc4` |
| `schemas/rule_check_run_result.schema.json` | +1 / −1: `RunFinding.severity`'s description only | `9b75a1290ab07d2c` |

## 2. What it does

- **The design.** It implements D2 §4.11 (option C, conservative interval binding; DD-13 ruled C) at revision 5b.3. ROOT amended §4.11.3–§4.11.4 during S-I1: interpolation is evaluated as the point path's chain of rounded operations, and the predicate is "eager U".
- **An interval mode in `expression_evaluator`.** It covers the whole formula language and is sound, outward-rounded and three-valued.
  - **Outward rounding:** after every floating operation, the lower end steps down one ulp and the upper end steps up one.
  - **Three values:** T only if every value in the input box passes, F only if every value fails, U otherwise.
  - **Eager U:** a possible block or non-finite value anywhere makes the whole predicate U, because the point path is eager.
- **The runner.** An additive entry, `run_rule_checks_with_bounds`, adds the outcomes T → `USER_RULE_CHECKED` (`RULE_INTERVAL_ALL_PASS`), F → `USER_RULE_FAILED` (`RULE_INTERVAL_ALL_FAIL`) and U → `RULE_INPUTS_INCOMPLETE` (`RULE_RESULT_INDETERMINATE`).
  - `run_rule_checks` and every existing struct are unchanged.
  - An invalid bound, or a duplicate bound for one input, is a blocking finding.
  - With b = 0, the point path runs.
- **A Python reference evaluator,** in parity with Rust bit for bit over the 94 shared cases.

## 3. What it does not do

- **No product caller.** src-tauri and the desktop do not call the bounded entry. Wiring solver result bounds into rule binding is S-I2.
- **No point-mode change.** Without bounds, `RuleCheckRunResult` is byte-identical to main.
- **No schema shape change, and no dependency or lock change.** src-tauri's lock is unchanged.
- **Outside the F2a D1 milestone's build closure,** so there is no Pass B and no registered-identity change.
- **The point path still panics** on an overflowing same-dimension quotient and on a NaN table argument. That predates S-I1 and is routed to T3-SI1b. Interval mode does not panic on those inputs.

## 4. Review

**RV99** (fresh, independent, not an F2a reviewer) built its own oracles.
- **Soundness:** no violation over 30,609 evaluator cases (about 4.9 million point samples and 450,000 exact samples) and 6,615 runner cases. Every box in which the point path both passes and fails reads U.
- **Point mode:** byte-identical over 63,086 runs.
- **Parity:** 94 shared cases plus 30,515 of RV99's own.
- **Mutants:** 46 of 46 killed.
- **Verdict:** PASS with 3 SHOULD-FIX, all repaired by I73 and confirmed by RV99.
- **Notes carried to S-I2:** N-6 (message forms) and N-7 (one bound per input).

## 5. Gates

| Gate | Result |
|---|---|
| Independent complete-diff review (RV99) | PASS; repairs CONFIRMED |
| Suites against main (I73, RV99) | `expression_evaluator` 49+1, `rule_check_runner` 33, `rule_pack_document` 10, src-tauri 116, Python `test_rule_interval.py` 193. Every count change is an added test |
| `source_equality.py` (main's #1082 tool) | see the PR-head record |
| `check_citations.py` with this package's index | PASS: 34 D2 citations resolved at the NUM pin (revision 5b.3) |
| GEN-8 on the exact head | see the merge record |
| Hosted CI and the full-SHA dispatch (`target_base` = main) | see the merge record |
| Exact-head Mac DEC-025 against a fresh main baseline | see the merge record |

## 6. Open obligations carried forward

- **T3-SI1b:** the point-path panic repair, after this PR.
- **S-I2:** the binding wiring; N-6 and N-7.
