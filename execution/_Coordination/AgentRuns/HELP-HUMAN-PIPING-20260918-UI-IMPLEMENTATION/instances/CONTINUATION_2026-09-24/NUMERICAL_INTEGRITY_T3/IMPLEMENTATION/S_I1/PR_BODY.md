**piping(T3 S-I1): sound interval rule evaluation (option C), runner outcomes and a Python reference**

This PR adds a sound, three-valued interval mode to rule evaluation. When a solver result carries a verified absolute bound, a rule check reads pass only if every value inside the bound passes. It reads fail only if every value fails, and indeterminate otherwise, so a result that straddles a limit can never pass.

## What changes

- **`expression_evaluator`:** an interval mode over the whole formula language. It rounds outward after every floating operation and is three-valued. Point mode is unchanged.
- **`rule_check_runner`:** an additive `run_rule_checks_with_bounds(&input, &[SolverResultBound])`, with the outcomes `RULE_INTERVAL_ALL_PASS`, `RULE_INTERVAL_ALL_FAIL` and `RULE_RESULT_INDETERMINATE`. `run_rule_checks` and every existing struct are unchanged.
- **`core/analysis_runs/rule_interval.py`:** a Python reference evaluator, in parity with Rust bit for bit over 94 shared cases (`fixtures/rule_interval/`).
- **`rule_check_run_result.schema.json`:** the `RunFinding.severity` description now names the full vocabulary. Only the description changes.

## What stays the same

- **Without bounds,** `RuleCheckRunResult` is byte-identical to main: 63,086 runs over the committed rule packs and run fixtures, plus generated packs and variants.
- **No caller yet:** the desktop and src-tauri do not call the bounded entry. Wiring solver result bounds into rule binding is a later slice (S-I2).
- **No schema shape, dependency or lock change.**
- **The rules crates are outside the F2a D1 milestone's build,** so the registered identity is untouched.

## Review and gates

- **Independent review (RV99):** PASS. Three should-fix findings were repaired and confirmed.
  - No soundness violation over 30,609 evaluator cases (exact-rational and point-path oracles) and 6,615 runner cases (point-path oracle).
  - 46 of 46 mutants killed.
  - These are agent reviews, not personal review by the owner.
- **The package:** `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/S_I1/`, with the change record and a citation index. D2 §4.11 resolves at revision 5b.3 on the integration branch.
- **Gates run before the merge, and recorded on the integration branch:** source equality, citations, GEN-8, hosted CI with the full-SHA dispatch, and the Mac DEC-025 against a fresh main baseline.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
