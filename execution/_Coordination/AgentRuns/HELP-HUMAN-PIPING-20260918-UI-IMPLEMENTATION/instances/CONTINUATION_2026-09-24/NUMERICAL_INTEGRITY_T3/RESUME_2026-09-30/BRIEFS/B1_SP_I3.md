# B1-SP after I3 (I85): W-C2's pins, and RV109's SF-1 and SF-2

Read `R/BRIEFS/B1_COMMON.md` and `R/BRIEFS/B1_SP.md` again first; their rules still hold. Then read:
- RV109's round 2 review, `R/REVIEW_RV109/rvp_round2_01/REVIEW.md` (`206fd360…`), with its `evidence/mutants/`;
- RR "RV109 passes SP in RV-P round 2; SF-1 and SF-2 go to I3's pinning step; B2-C dispatched as I97";
- the I3 section of RR, which names the merge commit.

## Where you start

- **I3 is merged:** ROOT merged `b1-r` (SR-RS, `b5cb7faaeb`, confirmed by RV113) into `b1` at `2ba2f81863`. Your worktree `WT/b1` is at that commit.
- **Your SP return** (`R/I85/b1_sp_01/RETURN.md`) recorded the I3 front-run:
  - W-C2's expected pins: sparse `7922e3e5`/`cccb9664`/`c7a18593`, dense `f2800bd4`/`612e23ca`/`a77c010b`;
  - the one expected change: T-7-on-C ends at G8 `PREPARATION_MISMATCH`.

  RV109 reproduced all six on its own scratch merge.

## What to change (tests and records only, in SP's fence)

1. **W-C2's pins after I3.**
   - Land the pinned successor and its fixtures in both modes, as RETURN §7 front-ran.
   - Change the T-7-on-C fault test's expectation to G8 `PREPARATION_MISMATCH`.
   - If any pin's bytes differ from the front-run's, stop and return before going on.
2. **SF-1 (N-16).**
   - Record in your records that, in every two-Run batch, the second Run's records differ from its one-case run in exactly two flags:
     - `shared_built_here` at p128 and p256;
     - `verification_shared_built_here` at p256.

     The cause is C2 §4 group-build sharing: one stiffness gives one group, and the second Run reuses s128, s256 and v256, built by the first.
   - Strengthen the N-16 test to compare the whole record set. The only exception is those flags on the second Run, asserted exactly.
3. **SF-2. Add two default-suite pins, in both modes:**
   - (C, B, A): a selected case that is not first;
   - (A, A2): two selected cases.

   With RV109's patch strings (`evidence/mutants/`), show that M15, M16, M17, M19, M28 and M32 are each killed by an assertion, and that your earlier mutants are still killed.

**No product `src` change.** If a pin cannot be written without one, stop and return.

## Evidence

- **The suites:**
  - PP registered and Stale, the runner and the witnesses, against `2ba2f81863` before your commits;
  - the only differences are the listed tests and the T-7-on-C expectation.
- The c = 1 successor pins pass.
- The s11f and in-fence guards pass, with their expected text unchanged.
- **The mutants:** RV109's six, and yours from SP.

## Host

- As B1_COMMON: every cargo goes through `WT/tools/t3_cargo.sh`, and every direct test binary or heavy pytest runs under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your own waits before you return.
- **Paths:** absolute paths only.
- **Records:** no symlink in a record (RR "NUM absorbs main with #1109's RV58 fixture repair; …"), and no folder named `build`.

## Output

- **Commit** on `codex/piping-t3-b1-20261007` in `WT/b1`, with a truthful message. ROOT pushes.
- **The record:** `R/I85/b1_sp_01/I3_01.md`, with its `_run_records/i3_01/` and `SHA256SUMS.i3_01`.
- **Budget:** 2–3 h.
- **End your turn with:**
  - the new head;
  - I3_01.md's sha256;
  - the pins' hashes;
  - the mutants' outcomes;
  - the suite delta.
