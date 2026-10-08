# RV109 (RV-P), round 2: independent review of B1's SP slice (the n-case transaction)

You are RV109, RV-P for B1. Your earlier records continue: round 1 on ST with its addendum, and the early read at R3′. Same rules as round 1. **Build your own oracles;** don't rely on the implementer's.

## The candidate

- **The branch:** `codex/piping-t3-b1-20261007` at **`603e238517`**, in `WT/b1`.
  - SP's commits since I1 (`262bd687f0`): `56c5579f07`, then `7458527ff7` to `c17340d50b`.
  - ROOT's I2 merge, `eca6c00a72`, brings in SA (`b1-a` at `9812c83ded`), which RV112 passed. SA is not yours to re-review; check only that its seam reads SP's fields as SP writes them.
  - The last commit is `603e238517`.
- **SP's files:** `lib.rs`, `retained_product.rs`, `retained_receipt.rs`, `retained_wire.rs`, the facade, product and wire tests, and `grant2.rs`.
- **The implementer's return:** `R/I85/b1_sp_01/RETURN.md` (`f5c4797a…`), and CHECKPOINT_R3P.md. Read them after forming your own view.
- **The specification:** PLAN_v2 §2.2 with §1's fence and R8; DESIGN_v2 T-1 to T-13.
- **ROOT's rulings:**
  - R3′;
  - your early read's dispositions;
  - RV112's N-3 and N-4;
  - the seam saturation;
  - I2;
  - RR "SP returned before I3; the headline rule accepted; RV-P round 2 dispatched".

## Review, in priority order

Your round-1 oracles apply, as PLAN_v2 §5 lists them for RV-P.
1. **T-1 to T-13 are right,** against DESIGN_v2's text and outcome table, and C1 to C3. Pay most attention to these:
   - T-8's one batch call and the **ordinal-to-request mapping** (N-2): {0, 2} in the Runs, `owner_refs`, `execution_order`, sources and product attempts;
   - T-9's per-case freeze;
   - T-10 and T-11's staging order, receipt body and snapshot points;
   - decision 5's abandonment set;
   - T-12's notice count (|A|) and detail placement (C1:68, on cases selected when abandoned);
   - T-13's `ONE_RUN_THROUGH_G_C`.
2. **W-C2's receipt, re-derived independently:** snapshots, `charged`, `execution_order`, group and build sharing, and the per-case outcomes. Before I3 it reads A selected, B `not_required`, C unavailable (`kernel_unresolved`, Ceiling), with precommit at G5 `ATTEMPT_MISMATCH`.
   - **N-16:** check that the batch outcomes equal PROBE's one-case outcomes.
   - **I85's I3 front-run:** check the expected post-I3 pin shas and the one expected change, if you can reproduce them on a scratch merge with SR-RS at `b5cb7faaeb`. The T-7-on-C fault test is expected to end at G8 `PREPARATION_MISMATCH`.
3. **c = 1 byte identity:** every committed c = 1 successor pin, in both modes, and adapter counts at every W1 stage boundary on your c = 1 rows, as in your early read.
4. **The headline rule (RETURN §8), as ruled.** At c ≥ 2, each summary headline is recomputed as the governing row over the staged rows: greatest value, then smaller case id, then location.
   - Check that this is what the accepted base readers' G7 headline check requires, and that the alternatives fail it.
   - In W-C2 the stress headline moves from case A's row to case C's. Check that.
5. **Your early read's notes are closed:** R3P-1 to R3P-9, including R3P-8's S13, S14 and S20 now killed. **RV112's N-4:** the first G-B refusal stands, and later late hooks do nothing.
6. **Multi-case coexistence** (n05 with a second case; A1-N-7), and the seam's saturation pin.
7. **The fault tests:**
   - custody;
   - preparation on A and on C (with the case-targeted hook);
   - the call failure;
   - staging;
   - each serializer check;
   - precommit;
   - the R-b′ limit.
8. **The guards:**
   - s11f, the admission guard and RE's carriers;
   - the one changed in-crate expected text (`u3_n9`: `serialize_cases`) and its reason;
   - no new rule-8 site;
   - `u1_serializer_reads_no_legacy_work_field`.
9. **Weakening.** Read every removed line in SP's diff, especially the retired one-case path and the old custody prelude.
10. **Mutants.** Your own beyond I85's 29, and PLAN_v2 §2.2's list. Each must be killed by an assertion.
11. **Suites against I1, test by test:** PP registered and Stale, the runner, the witnesses and RE's carriers.

**Also extend the PR-head ledger** with every hunk of `98a77c716e..603e238517` that is SP's.

## Host

- **Every cargo goes through `WT/tools/t3_cargo.sh`.** Every heavy job, including direct runs of test binaries, goes under `WT/guard/cargo_job.lock`.
- **Waits:** one wait per job, ending when the job's process has gone. Stop your waits before returning.
- **Writes:** absolute paths only. Name no record folder `build`. Record paths are placeholders only.
- **Your own copies** under `WT/rv109/`, with fresh targets under `WT/targets/rv109-*`. Delete them afterwards; keep your scratch.
- **Not allowed:** DEC-025, installs and Git writes.

## Output

- **The report:** `R/REVIEW_RV109/rvp_round2_01/REVIEW.md`, with `evidence/` and SHA256SUMS. It contains:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table;
  - a section per item;
  - the ledger extension.
- **Budget:** 6–9 h.
- **End your turn with:**
  - the verdict;
  - the counts, with one line per finding;
  - the report's sha256;
  - anything ROOT must rule on.
