# RV104: independent complete-diff review of T3-SI1b (the point-path panic repair)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this change.** Build your own oracles; don't rely on the implementer's.

## The candidate

- **The branch:** `codex/piping-t3-si1b-20261006` at `966113396e`, in worktree `WT/s-i1b`. It is three commits over main `f8ed4f0551`:
  - `90d2c1ebf8`: the repair and 6 evaluator tests;
  - `da0758064e`: the runner test file;
  - `966113396e`: comments in `tests/test_rule_interval.py`.
- **The implementer's return:** `R/I79/si1b_01/RETURN.md` (`44d82fc8…`). Read it after forming your own view.
- **The brief it answered:** `R/BRIEFS/SI1B_POINT_PATH_PANICS.md`.
- **The rulings:**
  - RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched", ruling 3;
  - RR "RV103 passes #1103; …", which accepts `NonFiniteInput`, keeps the doc comment and routes the boolean-formula observation to T3-SI1c.
- **The prior evidence:** I73's `CHECKPOINT_1.md` §6.3 and RV99's review `R/REVIEW_RV99/s_i1_01/`, notes N-2 and N-10.

## Review, in priority order

1. **No panic remains reachable** from `evaluate` or from the runner's point path, for finite or non-finite numeric inputs. That covers the three known sites and anything else you find: `expect`, `unwrap`, indexing, integer overflow, and `panic!`. Build your own generator. Include:
   - quotients near overflow;
   - `inf − inf` and `0·inf` feeding tables;
   - NaN, ±inf and subnormal bindings, limits and slot values;
   - nested ratios.
2. **Every input that did not panic on main gives a byte-identical point result.** That holds for the evaluator and for `run_rule_checks`' `RuleCheckRunResult`.
   - Use your own differential: base `f8ed4f0551` against `966113396e`, over the committed packs and fixtures plus your own generated set.
   - Report any differing line that was not a base panic.
3. **Interval mode is unchanged:**
   - `evaluate_interval`, and `run_rule_checks_with_bounds` with b > 0;
   - S-I1's 94 shared cases;
   - `test_rule_interval.py`.
4. **The findings tell the truth.** Check `NonFiniteInput`'s subject and message for each cause, and the ordering against `UnitMismatch`, `DivisionByZero`, `TableOutOfRange` and `TableKeyNotFound`. A run with a blocked check still evaluates the others.
5. **Tests and mutants.** The new tests fail on main where they should, and pass on the candidate. Write your own mutants, at least one per site, beyond I79's 10.
6. **Scope.** Exactly the 3 files. No Cargo, lockfile, schema or dependency change. The Python change is comments only.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`,** the host-wide T3 lock. Use `--offline --locked`. Other T3 jobs share the lock; wait for it and never kill another job.
- Work in your own copies: `git archive` of each revision into `WT/rv104/{base,cand}`, with fresh targets under `WT/targets/rv104-*`.
- pytest only on `tests/test_rule_interval.py`, with VENV's Python.
- No DEC-025, no evidence sweep, no native jobs, no installs, and no Git writes.
- Scratch goes in `WT/scratch/rv104_si1b_01/`. Nothing goes to the system temp directory.
- Delete your copies and targets afterwards.

## Output

- **The report:** `R/REVIEW_RV104/si1b_01/REVIEW.md`, with `evidence/` and SHA256SUMS, placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/rv104_si1b_01/records/` and say so.
- **Budget:** 3–5 h.
- **End your turn** with the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
