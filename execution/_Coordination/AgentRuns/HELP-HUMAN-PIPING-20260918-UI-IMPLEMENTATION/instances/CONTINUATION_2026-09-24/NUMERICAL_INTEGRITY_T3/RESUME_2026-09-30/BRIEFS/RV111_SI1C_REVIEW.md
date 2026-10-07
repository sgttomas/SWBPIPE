# RV111: independent complete-diff review of T3-SI1c (option D, with N-4 and N-5)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this change.** Build your own oracles; don't rely on the implementer's.

## The candidate

- **The branch:** `codex/piping-t3-si1c-20261007` at `7f233b2e01`, in worktree `WT/s-i1c`. It is four commits over main `025c1cf326`:
  - `3fea0678df`: N-5 (comments only);
  - `35a14b76d8`: D in the evaluator;
  - `62712b001c`: N-4 in the runner;
  - `7f233b2e01`: the Python oracle.
- **5 files:**
  - `expression_evaluator/src/lib.rs` and its `README.md`;
  - `rule_check_runner/src/lib.rs`;
  - `rule_check_runner/tests/point_path_non_finite_run.rs`;
  - `tests/test_rule_interval.py`.
- **The implementer's return:** `R/I88/si1c_01/RETURN.md` (`7459391d…`). Read it after forming your own view.
- **The specification:** I87's plan (`R/I87/si1c_plan_01/PLAN.md`, `0eb2459d…`) §3 D, §5 and §6.
- **The decisions:**
  - the owner's (option D, a repair within grammar 1.0.0);
  - ROOT's 4–15 (RR "I87's SI1c plan verified; …" and "Owner decision: SI1c is option D, …");
  - ROOT's rulings on I88's points (RR "I88's SI1c verified and ruled; RV111 reviews it; SR-RS dispatched as I90"). Ruling 2 (append the N-4 note) and ruling 3 (a misleading test name) are already set for the repair round.
- **The prior reviews:** RV104's (`R/REVIEW_RV104/si1b_01/`), with its probes and dumps. RV99's on S-I1.

## Review, in priority order

1. **No point-path result rests on a non-finite intermediate.** With your own generator, beyond I88's and RV104's, cover every producer and consumer pair of I87 §2.2–§2.3 at depth 1–3:
   - +inf, −inf and NaN from finite operands;
   - the absorbing forms (`min`, `max`, `select` taken and untaken, `x/inf`, interpolation absorption);
   - finite boundaries (`MAX`, subnormals, −0).

   No candidate result may pass or fail on such a value. Report any producer D misses.
2. **Every input with no non-finite intermediate and no N-4 value is byte-identical** to main, in the evaluator and in `run_rule_checks`. Use your own instrumented-base differential (base `025c1cf326` against `7f233b2e01`); don't reuse I88's instrumentation.
3. **Interval mode is byte-identical** on every line: `evaluate_interval`, `run_rule_checks_with_bounds` with b > 0, S-I1's shared cases and `test_rule_interval.py` (193).
4. **The findings tell the truth.** Check subjects, messages and order:
   - mismatches before the producer check;
   - `DivisionByZero` before the arms;
   - the first block stops the enclosing expression.

   N-4 (N4-1) matches I87 §5.1 with no status change, apart from ruling 1's case (a raw non-finite value in a different unit stays unsupplied). Check that ruling's reasoning.
5. **The schema holds:** every candidate runner line validates against `rule_check_run_result.schema.json`. No JSON `null` is left where a number is required.
6. **N-5 is comments only.** With comment lines removed, non-test code is byte-identical to main's.
7. **Tests and mutants.**
   - The new and revised tests fail on main where they should, and pass on the candidate.
   - The renames are truthful (see ruling 3).
   - Write your own mutants, at least one per site, beyond I88's 53. Confirm or contest I88's 17 equivalences, especially the four interpolation step checks.
8. **Scope.** Exactly the 5 files. No schema, `Cargo.toml`, lockfile, src-tauri, desktop or corpus change. The Python oracle's assertions are unchanged.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`,** the host-wide T3 lock, with `--offline --locked`. pytest under `P/tests` runs under the lock or with your own built bins. Other T3 jobs share the lock (I85, I89, I90). Wait for it, and never kill another job.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your own waits before you return.
- **Your own copies:** `git archive` of each revision into `WT/rv111/{base,cand}`, with fresh targets under `WT/targets/rv111-*`.
- **Not allowed:** DEC-025, evidence sweeps, native jobs, installs and Git writes.
- **Scratch** goes in `WT/scratch/rv111_si1c_01/`, and so does `TMPDIR`. Delete your copies and targets afterwards. Keep your dumps' hashes.

## Output

- **The report:** `R/REVIEW_RV111/si1c_01/REVIEW.md`, with `evidence/` and SHA256SUMS, placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path, evidence and remedy;
  - a section per item.
- **Budget:** 5–8 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - anything ROOT must rule on.
