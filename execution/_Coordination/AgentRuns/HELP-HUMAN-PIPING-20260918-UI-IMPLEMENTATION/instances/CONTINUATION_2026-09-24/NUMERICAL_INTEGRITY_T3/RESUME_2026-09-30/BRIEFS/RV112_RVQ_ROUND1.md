# RV112 (RV-Q), round 1: independent review of B1's SA slice (admission at option S3)

TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance and wrote none of this change.** Build your own oracles.

**You hold RV-Q for B1** (PLAN_v2 §5; RV89's and RV87's roles):
- round 1 reviews SA's expressions, which is this brief;
- you will later review SQ (G5, G6, the witnesses, the challenge, RSS and time, the non-candidate sweep, the re-pins);
- you confirm SB's Pass B.

Keep your records so a later you can continue from the files alone.

## The candidate

- **The branch:** `codex/piping-t3-b1-a-20261007` in `WT/b1-a`.
  - SA's commit is `6b62606778` over I1 `262bd687f0`: 3 files, `PP/retained_memory.rs` (outside the GENERATED PROFILE block), `PP/retained_memory_law_tests.rs`, and the runner's `tests/retained_precision_admission.rs`.
  - **A follow-up is coming on the same branch.** I89 merges `b1` at SP's `56c5579f07` into `b1-a`, then commits a parked-slot patch: `retained_error_text` folds every case's slot, with the law test `b1_sa_retained_error_text_reads_every_case_slot`.
  - Review that too, as `b1..b1-a`. If it has not landed when you finish SA's commit, say so and return; ROOT will resume you for it.
- **The implementer's return:** `R/I89/b1_sa_01/RETURN.md` (`05c2687d…`). Read it after forming your own view.
- **The specification:** PLAN_v2 §2.3 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…`), with §1's fence, and `BRIEFS/B1_SA.md`. Also:
  - RV107's A1 amendments, including A1-N-11 (`LOAD_CASES` and `TOTAL_LOADS` change here);
  - ROOT's rulings: RR "I89's SA verified and ruled; …", and "R3′: …" ruling 5.
- **The study:** `R/I82/b1_cap_study_01/STUDY.md` and `ADDENDUM_01.md` (option S3), with I82's evaluator `b1_eval.py` in its `_run_records/`.

## Review, in priority order

1. **The expressions are right** (SF-4's back edge). Check every gate bound SA writes against DESIGN_v2 and I82's S3 forms, at C = 3, l = 128, L = 384:
   - the `cap_rows` (`LoadCasesCapacity`, per-case `Loads`/`LoadsCapacity`, `TotalLoads`);
   - G-B's `CaseLoadsTotal`;
   - G-C's EnvelopeResults ≤ C·P_final, with its capacity and text;
   - the contract-evidence facts ×C;
   - RetainedErrorTextBytes ≤ C·(3m + 1)·Text(err);
   - B-6 = `NOTICE_RESERVE_BYTES` × 3.

   **Evaluate them independently with I82's `b1_eval.py`.** Their numeric values are SQ's; round 1 reviews the expressions, not the numbers.
2. **D1.4, D1.5 and D1.7 per case.** 1 ≤ c ≤ 3, no combinations or components, and every case's facts checked. The census takes the maximum over cases and Σ l_i. Check the `u32` choice for Σ l_i: is there an overflow path, and is it checked?
3. **T-3 (e).** `ordinary_solve_attempted(capture, requested)` holds exactly when `requested ≥ 1`, `capture.ordinary.len() == requested`, and every seed's `initial` is set. Read `requested` from `CompleteFacts.requested_cases`.
4. **The out-of-domain oracles** at `LOAD_CASES + 1`, and the runner's literal 4 tied to the producer by a PP law test. No D1 visibility change.
5. **The parked-slot patch,** when it lands. RetainedErrorTextBytes reads every case's slot, parked or active, and the test fails against the unpatched reader.
6. **c = 1 byte identity** and the registered profile unchanged. The GENERATED PROFILE block and `REGISTERED_PROFILES` must be byte-identical to I1's.
7. **Mutants.** Write your own beyond I89's 21, and include PLAN_v2 §2.3's list. Each must be killed by an assertion.
8. **Suites against I1, test by test:** PP registered and Stale, the runner and the witnesses. The only differences should be the listed new tests.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`,** with `--offline --locked`. Other jobs share the lock (I85, I89, I90, RV109, RV111). Never kill another job.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your waits before returning.
- **Writes:** use absolute paths for every write; a relative path resolves against ROOT's working directory. Name no record folder `build`, because `P/.gitignore` hides it.
- **Your own copies:** `git archive` copies under `WT/rv112/`, with fresh targets under `WT/targets/rv112-*`. Delete them afterwards, but keep your scratch for SQ.
- **Not allowed:** DEC-025, sweeps, installs and Git writes.
- **Scratch** goes in `WT/scratch/rv112_rvq_01/`.

## Output

- **The report:** `R/REVIEW_RV112/rvq_round1_01/REVIEW.md`, with `evidence/` and SHA256SUMS, placeholder paths only. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item.
- **Budget:** 3–4 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - anything ROOT must rule on.
