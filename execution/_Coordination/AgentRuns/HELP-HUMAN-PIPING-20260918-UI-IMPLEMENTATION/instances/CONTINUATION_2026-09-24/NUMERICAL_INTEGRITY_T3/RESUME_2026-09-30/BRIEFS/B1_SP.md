# B1-SP: the n-case transaction (DESIGN T-1 to T-13)

Read `R/BRIEFS/B1_COMMON.md` first; its rules hold.

**The specification** is PLAN_v2 §2.2, with §1's fence and source-text guards, R8's stops and §8's risks that apply to SP. The contract is DESIGN_v2 T-1 to T-13.

**Your worktree:** `WT/b1`, branch `codex/piping-t3-b1-20261007`, at ROOT's I1 commit. I1 is ST with its repair round, confirmed by RV109. Because B6 merged first, it coincides with I1′: `b1` has absorbed main `2007709549`. You are I-P, as for ST.

## Also carry

- **A1-S-2.** SP's multi-case tests depend on I2 (SA's `LOAD_CASES` and D1.4 change, in `WT/b1-a`). Until I2 they return `Domain`. Keep writing them, and pin them after I2.
- **RV107 ADDENDUM_01's notes for SP** (`R/REVIEW_RV107/b1_plan_01/ADDENDUM_01.md`):
  - **A1-N-1:** a test-only successor capture;
  - **A1-N-6:** the hook's request index and fault plumbing (`fail_preparation_of_case(index)` in `grant2.rs`);
  - **A1-N-7:** n05's two-case coexistence pin, after I2.
- **RV109 N-2:** add mutant R17 (`.all` for `.any` in `retained_w1`'s trigger test) to your list. W-C2 tells the two apart.
- **R3 ruling 2 and RV109 N-3** are SA's. Keep the seam fields as ST defined them.
- **N-16:** if W-C2's batch outcomes differ from PROBE's one-case outcomes inside the one `CaseBatchCall`, record the difference with its cause and return it to ROOT as a finding. Do not absorb it into a pin.

## Integration while you work (A1-N-4)

ROOT merges into `b1` only at a commit of yours.
- When SA is ready (I2) or SR-RS is ready (I3), ROOT will message you.
- Commit your work in progress at a clean point. Run the build at least, and the c = 1 pins if you can.
- Reply with the commit. ROOT merges with `--no-ff` and tells you to continue.

Don't merge anything into `b1` yourself.

## Checkpoint R3′

After T-2, T-6 and T-7, return to ROOT with:
- the head;
- what is done, and the tests so far;
- a re-estimate of the rest of SP;
- whether the serializer should be split out (PLAN_v2 R3′).

RV-P (RV109) may start reading then. ROOT then resumes you for the rest.

## Acceptance (the end of SP)

As PLAN_v2 §2.2:
- c = 1 byte identity;
- W-C2's outcomes (before SR-RS, the G5 precommit failure with per-case outcomes from the `before_precommit` hook; after I3, the pinned successor and fixtures);
- the ordinal mapping;
- multi-case coexistence;
- the multi-case fault tests with T-12's notice counts;
- T-12's base readers with several notices, with the bytes written out for SR-PY and SR-TS;
- `ONE_RUN_THROUGH_G_C`, empty hooks and Stale's plain bytes;
- the listed mutants (with R17) killed by assertions;
- the guards, with any changed expected text given its reason, and any new rule-8 site given a `TABLE` row.

**Return** with:
- the head and commits;
- the evidence for each acceptance item;
- the suite differences against I1;
- the mutants;
- anything for ROOT.

RV-P round 2 follows.

**Records:** `R/I85/b1_sp_01/` (`CHECKPOINT_R3P.md` at R3′, and `RETURN.md` at the end).

**Budget:** 16–25 h in all; R3′ comes at about half.
