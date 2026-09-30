# KF3 change record: an unformed certified bound becomes unavailable, not a stop; partial stage work is recorded

This is the draft PR record for slice KF3 of T3 (numerical integrity), following `.agents/skills/chirality-change/SKILL.md`. I19, a TASK, implemented it. The details are in `RETURN.md`.

> **RV23's review (PASS) and its fix are addendum 1.** RV23-1 is fixed there, so "filled on every path" below holds as written.

- **Branch:** `codex/piping-kf3-20260929`, from main `0f5d8c7b4` (K4 and KF1).
- **Commits (made by ROOT):**
  - `75a1222a6`: checkpoint 0, the plan;
  - `29c0b69e4` and `a7ec4981a`: checkpoint A, the code, the tests and the records;
  - `c0473301e`: merge of main `f8400d290` (V-K), with one conflict hunk resolved by I19;
  - `e114b23c1`: merge of main `78f55f927` (K6b);
  - `ae831ca51`: B, with H's parity update and the scale records;
  - the next commit: D's records only (`RETURN.md`, this file, `_run_records/{d,h,merge}/`, `toolchain.txt`, `SHA256SUMS`).
- **Size (against main `78f55f927`), with records under `T3/IMPLEMENTATION/KF3/` besides:**
  - `FK/src/structural.rs`: +3, the `retained_api` facade line;
  - `FK/src/structural/retained/bound.rs`: +594 −93;
  - `FK/src/structural/retained/verify.rs`: +102 −50;
  - `FK/src/structural/retained/adaptive.rs`: +134 −1;
  - `FK/src/structural/retained/wide_sum.rs`: +14;
  - `FK/tests/retained_k4/`:
    - `bound_tests.rs`: new, 332 lines;
    - `kf3_tests.rs`: new, 521 lines;
    - `kf3.txt`: new, 19,587 lines of GEN vectors;
    - `gen_k4_vectors.py`: +291 −35;
    - `method_tests.rs`: +39 −15;
    - `scale_tests.rs`: +62 −11;
    - `wide_sum_tests.rs`: +45;
    - `SHA256SUMS`: +1;
  - `H/src/k6/w1/staged.rs`: +21 −33;
  - `H/tests/k6b_w1.rs`: +64 −26.
- **Basis:**
  - the I19 brief (`TASK_BRIEFS/I19_KF3_IMPLEMENTATION.md`, `98e57d3d…`);
  - the checkpoint-0 plan (`721fc2d6…`);
  - these sections of `ROOT_RULINGS_V1.md`: "KF3: rulings on I19's diagnosis and plan; D1 revision 5a.3 amendment A2", "KF3: checkpoint A accepted", "Main merged into K6b and KF3", "KF3: main merged; K6b's parity restored in KF3; B's slot granted" and "KF3: checkpoint B accepted; two findings for D".
- **Platform:** Mac (`aarch64-apple-darwin`, 128 GiB), rustc 1.97.1.

## What changes

- **D1 revision 5a.3, amendment A2, as ROOT ruled it.**
  - A Uc_c or S_c whose exact-sum formation is refused (a span beyond 8,128 bits, or an exponent overflow) is unavailable for its own block only. It is R7's "does not exist" (+∞), and B_c is the minimum over the bounds formed.
  - A block with data left with no bound after a refusal stops the attempt with that refusal (`Span` or `Exponent`, as before A2), before any `uc` rejection.
  - Budget stops are never refusals.
  - `u_pass`, `nl_pass`, `uc_bounds`, `shifted_factor` and `shift_schedule` carry per-block refusal slots.
  - The new helpers `shift_start`, `shift_run`, `certificates` and `block_refusals` serve `verify_state` and the test entry `certify`.
  - `ExactWideSum::reset` clears the accumulator after a refusal.
- **Evidence:** `RefusalKind`, `BoundPass`, `BoundRefusal`, `CertifiedBound` and `BlockRefusal` are public and exported through `retained_api` (ROOT's merge ruling). `AttemptRecord.bound_refusals` is public and filled on every path.
- **Partial stage work:** each of the four builds that record stages adds, when it stops, its unstaged charged work to the stage in progress (`StageWork::close_stopped`). The stages then equal the charged totals on every path.
- **H:** K6b's `stages_equal_totals` requires equality on every attempt, completed or stopped. `k6b_w1.rs` tests a partial `uc` stage and adds `every_stopped_build_leaves_nothing_unstaged`.
- **GEN:** the span mirror (`sum_span`, `add_dir`) and A2 in the emulated passes, the shift and 7d. Also `hp_only` expectations and `kf3.txt`, with the two constructed models. Every earlier record is byte-identical.
- **Unchanged:**
  - every success path's operations and order;
  - the S11 site table;
  - est_c;
  - every file outside the write set.

## Results

- **Honesty** (RETURN §4). Every reader of Uc, S and B uses B_c only through B_c ≥ ‖K̃_c⁻¹‖₁. A2 takes the minimum over formed bounds only, and a refusal changes no formed value (bit-identity tested). No step of the guarantee depends on A2.
- **Controls:** no K4 control changes outcome, row, class, bound or golden work count. Failed attempts' stage records now hold their partial stage, with totals unchanged.
- **The constructed models:**
  - KF3-UC-SPAN (390 members, α = 20.8 bits per member) moves from `Unresolved(ExactSumSpan)` to selected at 128, with B = S_c. It is token-equal to GEN and honest on 8,983 checks against GEN's high-precision solves.
  - KF3-UC-SPAN-ZERO is selected, its refusal recorded.
  - With S disabled, the refusal still stops the attempt.
- **Scale** (vk_scale, release):
  - 100 and 1,000 members: byte-identical to V-K's B.
  - At 10,000 members:
    - CHAIN-AX, CHAIN-ROT and CONT-ROT move to selected at 128 and pass R1 on every row. Charged work is 10.1, 12.1 and 10.4 G LME, against 6.7, 8.3 and 6.8 before.
    - CONT-AX is unchanged.
    - TREE-AX and TREE-ROT move to `Unresolved(Ceiling)`, rejected by R7's estimate (b), and publish nothing. Their 194 `fail` rows are unpublished rows. They charge 65.3 and 74.2 G LME.
- **Suites:**
  - FK's full suite: 348 lib tests, 7 integration files and 6 doc-tests;
  - VR: 47 of 47;
  - H, `--all-targets`;
  - H's runner: 47 of 47;
  - `gen_k4_vectors.py --check`;
  - no warnings.
- **Mutations:** NONE passes, and 11 of 11 are killed. They include the brief's four: unavailability as 0, B as the max, the dropped stop, and the unrecorded partial stage.

## Findings at B (derived at D; RETURN §12 and §13)

- **KF3-B1:** the TREE frames' `Ceiling` is (b)'s.
  - (b)'s ratio Ŵ_q/(2^(6−P)·ê) at the root's end force does not depend on P: 1.0125195 at P = 256, 512 and 1024 on a 4,000-member tree. So no precision passes.
  - Near the threshold (c = 64.8 against (b)'s 64) it is slack in (b): the Corollary holds with its stated margin.
  - From about 6,000 members (c = 143 and 177) the row needs more than the 127 that R7's split of λ = 2^8 leaves for (b): a design limit of the split, like THIN.
  - The 10,000-member value is not in the records. ROOT rules.
- **KF3-B2:** B compared the heap with VR's port of an earlier K6b E_max.
  - K6b's final formula, on `vk_scale`'s own fixed term, bounds both TREE peaks by about 7 MB.
  - K6b's pass term leaves out `nl_pass`'s `at`, `bt` and `ct` during the shift, and counts `shifted_factor`'s `work`, which is already freed. Net of an 8 B `Option` over-count, it is 10.8 MB short at 1024.
  - Pre-KF3 code holds the same allocations on the same path. KF3 only makes the path reachable at scale.
  - So this is not KF3-induced. It is routed to K6b's follow-up, and there is no fix in KF3.

## Limits

- **Mac only.** No timing or memory-growth claims.
- **Not observed in any model:** the Exponent refusal and S's refusals. They are exercised at unit level only.
- **No certified norm** exists for the constructed models, which are beyond GEN's bound. Their honesty rests on the high-precision solves.
- **The kill matrix** was not re-run after the V-K merge. ROOT's condition was a VR record change, and none occurred.
- **Not measured:**
  - (b)'s ratio at 10,000 members;
  - H's own run (`k6_observe`) of the TREE frames.

## Gates (ROOT runs the PR)

- An independent reviewer, directed to:
  - the honesty argument for B = min over the available bounds, and every reader of Uc, S and B (RETURN §4);
  - the stage-accounting identity on stopped builds;
  - H's parity change.
- Hosted CI with the full-SHA dispatch.
- DEC-025 with a fresh sweep target.
- GEN-8 before the records commit that goes to main.
- **T9 and the both-entry gate:** the brief says kernel only, "`retained` is private on main". Since the V-K merge, `retained_api` exports KF3's five refusal types, so whether T9 and the gate stay unrun is ROOT's to confirm.

## Downstream notices

- **K6b (H):**
  - KF3-B2's E_max term: +2·nf·w(l) at the 1024 shift, in `counts.rs:515`, then `counts.jsonl` regenerated and W1-T4 re-run (ROOT's routing).
  - The stale doc comment in `k6_observe/w1.rs:6-10`.
  - The TREE frames' charged work before `Ceiling`, for W1's limits.
- **V-K (VR):**
  - `src/scale.rs` ports K6b's E_max at `082990c8d`, before RV22's review, and the runner admits with it. Its kernel terms are 153 MB below K6b's final formula on the TREE frames.
  - At 10,000 members, three frames are now selected and two end `Unresolved(Ceiling)`. How the TREE frames are listed follows ROOT's KF3-B1 ruling.
- **W1 limits (ROOT):** they wait for E_max to bound every phase at 10,000 members (ROOT's B ruling).

## Addendum 1: RV23's review (PASS) and its fix

- **Review:** RV23 reviewed head `b8c55c92e`: PASS, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs.
  - The report is `T3/REVIEW/KF3_REVIEW.md` (sha256 `96060f9e…`).
  - ROOT's rulings are "KF3: rulings on RV23's review" (numerics `24ef16301`): fix RV23-1 before merge, add RV23-N1's test, correct RETURN §12's wording, and record N2 to N5.
- **RV23-1, fixed, evidence only.**
  - **The defect:** a refusal recorded in the verification's shared build (`uc_bounds`), or in 7c's schedule (`shift_schedule`), was dropped from `AttemptRecord.bound_refusals` when a later stop, a budget stop say, ended the same build.
  - **The fix:** the caller owns the refusal slots.
    - `uc_bounds` and `shifted_factor` take them, and `ShiftedFactor.refused` is removed.
    - `build_verify_shared` returns a stopped build's Uc_c refusals in `VerifySpent.refusals`. The verification cache keeps them with a cached non-budget failure, and `verify_precision` records them before `vs?`.
    - When it stops, `shift_schedule` records in the attempt's `s_refused` the refusals kept in its results and those of the factorization in progress for the blocks it shifts. Its factorizations run in a closure, which keeps the S11 site table unchanged.
  - **Unchanged:** every operation, its order and every formed value on every path; the public API; the site table.
  - No code outside FK reads `bound_refusals`.
- **Tests (RETURN A1.1):**
  - RV23's probe at the shared build: KF3-UC-SPAN 1 and 100 LME short. The `Failed(Stop(Budget(Case)))` attempt carries `refused:span:backward:66`.
  - A three-block schedule swept over every case room (step 19 of 114,530 LME). 327 stops fall in the factorization in progress, and 4,011 after the results kept both refusals.
  - RV23-N1's `verify_state` precedence test: a refused block and a `uc` block, and the refusal stop wins. Its control shows that the `uc` branch is live.
- **Size (against `b8c55c92e`):**
  - `K4R/bound.rs` +134 −83, mostly the loop re-indented;
  - `K4R/adaptive.rs` +36 −12;
  - `K4R/verify.rs` +14 −4;
  - `K4T/bound_tests.rs` +175 −13;
  - `K4T/kf3_tests.rs` +132;
  - `K4T/scale_tests.rs` +20 −5.
- **Mutants:** NONE passes 14 of 14. All six are killed:
  - RV23-1-M1 and M2 (the shared build's refusals dropped, in `verify_precision` or `build_verify_shared`);
  - RV23-1-M3, M3a and M3b (the schedule's carry dropped whole, for the results, or for the factorization in progress);
  - RV23's M4b.
  - Every anchor of RV23's other mutants still matches.
- **Suites:**
  - FK's full suite: 351 lib tests, 7 integration files (S11 3 of 3) and 6 doc-tests, with no warnings;
  - `gen_k4_vectors.py --check`: 24 of 24 OK;
  - rustfmt clean.
  - No outcome, row, class, bound or work count moves.
- **Records:** RETURN addendum 1 (with RETURN §12's wording corrected and N2 to N5 recorded), `_run_records/rv23/` and SHA256SUMS, refreshed.
- **Next (ROOT's ruling):** RV23 confirms the new head. Then come CI with the dispatch, DEC-025, GEN-8 and the merge.
