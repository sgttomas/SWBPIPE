# B1-SR-TS: the TypeScript reader (R-D38, F-1 text B, G8 per case, G5's `not_required` rule), with RV108's TS items

Read `R/BRIEFS/B1_COMMON.md` first (sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75`); its rules hold.

**The specification** is PLAN_v2 §2.4: the common part and SR-TS. The contract is DESIGN_v2 §2 (R-D38 with (4b), m1–m8), §3.2 (text B, P1–P4) and §3.3 (G8 per case, G5's `not_required` rule).

**SR-TS's items:**
- (4b) in `productAttempts`;
- G8's codes change to `PREPARATION_MISMATCH`;
- mode code 3 is dropped;
- the checks apply to every case.

This builds on B6's G7 header change, which is on main since #1107.

**Your worktree:** `WT/b1-t`, branch `codex/piping-t3-b1-t-20261007`, cut by ROOT from I1 (`262bd687f0`). You are I-TS. SP, SA and SR-PY run beside you in `WT/b1`, `WT/b1-a` and `WT/b1-p`; touch none of their files.

## The fence

TS (`P/apps/desktop/src/features/results/retainedPrecision.ts`) and `retainedPrecision.test.ts`, as PLAN_v2 §1. R8's stops apply to everything else.

## First: the cascade census (R5)

Run the aligned reader over 07m's 294 mutations and 28 must-pass entries.
- **Expected: zero changes,** apart from entries that RV108 N4's repair would move, if any exist. List those.
- **Any other change stops the work.** Return under R5.

## Then

- **The SR-TS items above.**
- **D38's obligation** `[r1: N-6]`:
  - list every check in TS that assumes a prepared source has a Call or a Run;
  - relax each to (4b), or show it does not apply.

  The list goes in your RETURN.
- **RV108 N4** (`R/REVIEW_RV108/b6_01/REVIEW.md`).
  - **Today:** a transported successor whose `results` contains `null` makes `projection` raise `TypeError` (`Object.hasOwn` on null), which G7's default catches as `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`. Rust admits the same input, and since B6 so does Python.
  - **The fix:** make TS refuse or admit it by an explicit rule, consistent with Rust's transport reading (transport reads no rows). State which, and pin it.
- **RV108 N6 (TS's doc comment).** B6's `baseHeaderCode` doc claims agreement with Python on every header branch. That becomes true only after SR-PY repairs RV108 N1. Reword it to say what holds now, or write it to hold after SR-PY. Your choice; record it.
- **I90's notes on first failures** (for SC; `R/I90/b1_sr_rs_01/RETURN.md` §"For ROOT" item 2) tell you which gate each D38 mutation first fails at in Rust. TS should agree, or the difference is a finding for ROOT.

## Acceptance

- The census holds.
- TS passes 07m in full; every count change is an added test.
- G8 and G5 behave per DESIGN §3.3 on synthetic n-case receipts.
- D38's list is complete.
- N4 and N6 are done, with tests.
- The whole vitest desktop suite, base against head, test by test; and `tsc --noEmit` clean.
- **Mutants, killed by assertions:**
  - the per-case G8 loop;
  - each of G5's three dropped conjuncts restored;
  - each relaxed D38 check restored;
  - P2–P4;
  - mode code 3 restored;
  - the N4 rule removed.

**Return** with:
- the head and commits;
- the census;
- D38's list;
- the evidence per item;
- the suite differences;
- the mutants;
- anything for ROOT.

RV-R (RV113) reviews SR-TS with SR-PY (I4).

## Host

- **vitest:** link `node_modules` and copy the eight wasm assets as I83 did for B6 (`R/I83/b6_01/RETURN.md`, "Node and the wasm assets"), and remove both afterwards. Heavy vitest runs go under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your waits before returning.
- **Writes:** absolute paths for every write. Name no record folder `build`. Record paths are placeholders only.
- Never kill another job.
- **Scratch** goes in `WT/scratch/<id>_b1_sr_ts/`.

**Records:** `R/<id>/b1_sr_ts_01/`. **Budget:** 5–7 h, plus about 1 h for N4 and N6.
