# SI1c: the point path blocks at a non-finite intermediate (option D), with N-4 and N-5

TASK (Type 2), an implementer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Your ID names a records folder and a role, with no memory of earlier sessions. Earlier holders of related roles left their work in records:
- I73 implemented S-I1;
- I79 implemented SI1b;
- I87 planned this slice;
- RV99 and RV104 reviewed S-I1 and SI1b.

Cite them, and assume nothing beyond them.

## Why, and what is decided

On the ordinary point path, a rule formula whose arithmetic overflows past `f64::MAX`, or produces NaN, still decides a pass or a fail. Interval mode reads the same formula as indeterminate.
- **The owner decided (2026-10-07):** option D, block at the operation that overflows, as a repair within grammar 1.0.0. There is no version bump and no conformance-corpus change.
- **ROOT ruled decisions 4–15.** See RR "I87's SI1c plan verified; SI1c changes public meaning, so the remedy goes to the owner" and "Owner decision: SI1c is option D, a repair within grammar 1.0.0; RV108 passes B6; RV109 passes ST with SF-1".

## The specification

**I87's plan** is `R/I87/si1c_plan_01/PLAN.md` (sha256 `0eb2459d…`); verify the hash first. Implement:
- **§3 (D):** at each producer site (add and subtract; multiply, all three arms; divide, the dimensionless-divisor and derived arms; each of interpolation's six floating steps), push `NonFiniteInput` with the existing subject and §3's message, and return `None`, when the result is not finite. The ratio arm is unchanged.
- **§5.1 (N4-1):** test the raw value before normalization and the normalized value after. A non-finite input is recorded in `bound_inputs` with `supplied: true`, no `value` and the note; one blocking `NonFiniteInput` names the input. A non-finite limit gets `NonFiniteInput` with the slot as its subject, in both limit blocks. No status changes.
- **§5.2 (N-5):** the doc and comment wording, the README line, `FindingCode::NonFiniteInput`'s list, and the interval header.
- **§6.2:** the write set, including the optional Python oracle alignment (decision 9: align `point_value`).
- **§6.3:** the tests, new and revised, with the renames listed.
- **§6.4:** the differential. Use the instrumented-base oracle and all six pass conditions, over I79's and RV104's committed harnesses plus the new SI1c family.
- **§6.5:** the mutants.

**Keep:**
- SI1b's checks that become unreachable, with their mutants recorded as equivalent (decision 11);
- interval mode unchanged, byte for byte;
- every input with no non-finite intermediate and no N-4 value byte-identical.

**Stops.** Return to ROOT before going on if:
- a pass condition of §6.4 fails for a reason the plan does not foresee;
- the write set must grow beyond §6.2;
- a schema, `Cargo.toml`, lockfile, src-tauri or desktop file would change.

## Where and how

- **Worktree:** `WT/s-i1c`, branch `codex/piping-t3-si1c-20261007` from main `025c1cf326`, created by ROOT. Commit on it with truthful messages; ROOT pushes. Make no other Git writes.
- **Every cargo** goes through `WT/tools/t3_cargo.sh` (`--locked --offline`).
  - Heavy pytest runs go under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
  - pytest under `P/tests` either sets `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to your own builds, or runs under the lock.
  - Other T3 jobs share the lock (I85's repair round and ROOT's DEC-025 for B6). Wait for it, and never kill another job.
- **Waits.** Use one wait per job. Every wait loop must also end when the job's process has gone (for example `while kill -0 <pid> 2>/dev/null; do sleep 20; done`). Stop your own waits before you return.
- **Differential copies:** `git archive` copies of base and candidate in `WT/scratch/<id>_si1c/`, with fresh targets under `WT/targets/<id>-si1c-*`. The instrumented base exists only in scratch.
- **Not allowed:** DEC-025, evidence sweeps, native or solver jobs, and installs.
- **Scratch** goes in `WT/scratch/<id>_si1c/`, and so does `TMPDIR`. Nothing goes to the system temp directory. Delete copies and targets when done, but keep the dumps' hashes and the harnesses.

## Output

- **The record:** `R/<id>/si1c_01/RETURN.md`, with `_run_records/` and SHA256SUMS, placeholder paths only. It contains:
  - the head and commits;
  - each change, with its evidence;
  - the differential's counts, by family and by consumer row, and each pass condition's result;
  - the suites, base against head, test by test (`expression_evaluator`, `rule_check_runner`, `rule_pack_document` and `test_rule_interval.py`);
  - the mutants;
  - the renamed tests;
  - anything ROOT must rule on.
- **If the host's write guard refuses a write into NUM,** write to `WT/scratch/<id>_si1c/records/` and say so.
- **Budget:** 10–14 h. Return once, unless a stop fires.
