# T3-SI1b: the point-path panic repair in `expression_evaluator`

TASK (Type 2), an implementer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance**: ROOT gives your ID at dispatch. It names a records folder and a role, and carries no memory of earlier sessions. Earlier holders of related roles (I73, RV99) left their work in records; cite those, and assume nothing beyond them.

## Why

The ordinary point `evaluate` panics, instead of returning a finding, on inputs reachable at extreme magnitudes. I73 found this during S-I1 and left it alone, because S-I1 had to keep point mode unchanged. RR routed the repair to this node: "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched", ruling 3. S-I1 is now on main (#1100), and so is U8 (#1102).

**The known panic sites,** at main `f8ed4f0551` in `P/core/rules/expression_evaluator/src/lib.rs`:
- **A same-dimension quotient that overflows.** `divide` reaches `Quantity::dimensionless(value, "ratio").expect(…)` (near line 1457).
- **A NaN argument to `interpolate` or to a step `lookup`.** The `.expect("in-range … always has a …")` calls (near lines 968 and 983). An exact lookup already blocks.
- **The evidence:** I73 saw 18 of its 36,069 differential inputs panic, and 117 of its property-test samples (`R/I73/s_i1_01/CHECKPOINT_1.md` §6.3 and §5; its harness is `_run_records/checkpoint1/point_diff_tail.rs`, with the final runs in `_run_records/final/`). RV99 N-2 and N-10 name the lookup kinds and confirm that a check with b = 0 runs the point path (`R/REVIEW_RV99/s_i1_01/`).

## The change

1. **Every input that panics today returns a blocking finding.** The quotient becomes a non-finite result, and a NaN table argument is neither in range nor out of it. Both should block the way the point path already blocks non-finite values.
2. **Reuse the existing vocabulary.** `FindingCode` and `EvaluationError` already have blocking codes, such as `NonFiniteInput`. If no existing code states the cause truthfully, **stop and return a checkpoint** with the options before implementing. A new public code is ROOT's ruling, and it may be the owner's (a change to public meaning).
3. **Look for others.** Search the point path, and the runner's use of it, for any other `expect`, `unwrap`, `panic!`, indexing or integer overflow reachable from a finite or non-finite numeric input.
   - Repair one in this unit if it has the same mechanism.
   - Otherwise, list it with a reproducer and leave it.
4. **Nothing else changes:**
   - **Every input that does not panic today** gets a byte-identical point result. That includes `RuleCheckRunResult` from `run_rule_checks`.
   - **Interval mode** is unchanged, as are `run_rule_checks_with_bounds` and its outcomes. S-I1's 94 shared cases and the Python parity still hold.
   - There are no schema, dependency or lockfile changes.
5. **The Python reference.** If `core/analysis_runs/rule_interval.py` (or another Python evaluator) mirrors the point path at these sites, align it and extend the shared cases. If it does not, say so.

## Evidence the return must carry

- **The point-mode differential,** base `f8ed4f0551` against your head, over I73's 36,069 inputs. Only the previously panicking inputs may differ, and each must now be a blocking finding. Extend I73's harness, and state any changes you make to it.
- **Regression tests** for each repaired site, in Rust (and in Python if step 5 applies). Use the smallest reproducer, plus at least one of I73's 18 inputs.
- **The runner:** a rule check whose input panics today now gives a blocked result and does not abort the run.
- **Suites, base against candidate:** `expression_evaluator`, `rule_check_runner`, `rule_pack_document`, `test_rule_interval.py`, and `rule_check_runner`'s dependents within `core/rules/`. Every count change must be an added test.
- **Mutants:** at least one per repaired site, all killed.

## Host and Git

- **Your worktree:** `WT/s-i1b`, branch `codex/piping-t3-si1b-20261006` from main `f8ed4f0551`, created by ROOT. Commit on it; ROOT pushes.
- **Cargo only through `WT/tools/t3_cargo.sh`,** the T3 host lock. Other T3 jobs share it; wait and do not kill them.
  - No DEC-025, no evidence sweep, no native or solver-at-scale jobs, and no installs.
  - pytest only on the named files.
- **Your scratch** goes in `WT/scratch/<your-id>_si1b/`. Nothing goes to the system temp directory.

## Output

- **The record:** `NUM/P/execution/…/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/<your-id>/si1b_01/`, holding RETURN.md, `_run_records/` and SHA256SUMS, with placeholder paths only (`WT`, `NUM`, `P`).
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<your-id>_si1b/records/` and say so.
- **RETURN.md** gives:
  - the head and the commits;
  - each repaired site, its finding code and why that code is truthful;
  - the other panics found, if any;
  - the differential, the suites and the mutants;
  - anything ROOT must rule on.
- **Budget:** 3–5 h. Return once, unless step 2 needs a checkpoint.
