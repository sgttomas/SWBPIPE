# B1-ST repair round 1 (I85): RV109's SF-1, N-1 and N-4

Read `R/BRIEFS/B1_COMMON.md` and `R/BRIEFS/B1_ST.md` again first; their rules still hold. Then read RV109's review, `R/REVIEW_RV109/rvp_round1_01/REVIEW.md` (`707045b9…`), and RR "Owner decision: SI1c is option D, a repair within grammar 1.0.0; RV108 passes B6; RV109 passes ST with SF-1".

## What to change (tests only, in ST's fence)

1. **SF-1.** Pin that T-4 runs before R-2's notice reservation. Today mutant R10 (reserve first, then T-4) survives, because every pin compares bytes only.
   - In the two-body B pin's private-driver half, assert that after `NoTriggeredCase` the envelope's diagnostics capacity equals its length: no slot was reserved.
   - Add a collision variant: the base already carries the notice id, and the result must still be `NoTriggeredCase` with the exact bytes (under R10 it reads `NoticeReservation`).
   - Use RV109's discriminator (`E/` in its report) as a reference, not as code to copy.
2. **N-1.** Add a classifier-test row: "Passed with no seed is `NotRequired`". Mutant R16 must then die.
3. **N-4.** Add default-suite (not `#[ignore]`) `NoTriggeredCase` pins for W2b's input and W6's PHYS-R4 input: exact plain bytes, no notice, no W1 work. Keep the ignored witnesses as they are.

**No product `src` change.** If a pin cannot be written without one, stop and return.

## Evidence

- Mutants R10 and R16 are now killed by assertions, and PLAN_v2's five are still killed.
- The PP registered suite against `a8e719f5b4`: the only differences are the added tests.
- The c = 1 successor pins pass.
- The s11f and in-fence guards pass, with their expected text unchanged.

## Host

- As B1_COMMON: every cargo through `WT/tools/t3_cargo.sh`.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your own waits before you return.
- **Your kept scratch and targets** (`WT/scratch/i85_b1_st/`, `WT/targets/i85-b1-st*`) are yours to reuse. Delete the `mut/` archive and its target when the round is done.

## Output

- **Commit** on `codex/piping-t3-b1-20261007` in `WT/b1`, with a truthful message. ROOT pushes.
- **The record:** `R/I85/b1_st_01/REPAIR_01.md`, with its `_run_records` additions and `SHA256SUMS.repair_01`.
- **Budget:** 1–2 h.
- **End your turn with:**
  - the new head;
  - REPAIR_01.md's sha256;
  - the mutants' outcomes;
  - the suite delta.
