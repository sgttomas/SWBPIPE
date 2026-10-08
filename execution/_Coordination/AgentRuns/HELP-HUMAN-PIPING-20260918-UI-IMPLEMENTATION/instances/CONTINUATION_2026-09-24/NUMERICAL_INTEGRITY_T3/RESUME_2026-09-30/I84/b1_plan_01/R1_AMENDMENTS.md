# I84 B1-P, revision 1: where each RV107 finding is answered

**The inputs:**
- RV107's review: `R/REVIEW_RV107/b1_plan_01/REVIEW.md`, sha256 `03c3111d8baec21503b8212e50dc40ccb002a77e79db8127bfbd6467375aa426`;
- ROOT's ruling: RR "R1: B1's plan ruled with seven amendments; I84 writes PLAN_v2; B6 first as its own PR".

**The files:**
- **Revision 0:** `PLAN.md` (`7f9699f3…4bb5c`), unchanged.
- **Revision 1:** `PLAN_v2.md`, whose sha256 is in `SHA256SUMS.v2`.

**Notation.** In PLAN_v2, `[r1: X]` marks a passage that answers X. Section numbers below are PLAN_v2's.

## The SHOULD-FIX findings, as ruled

| Finding | R1's ruling, in short | Answered in PLAN_v2 |
|---|---|---|
| **SF-1:** concurrency, worktrees, sequencing | At most three implementers; each concurrent PP- or RE-building lane in its own worktree and branch from a common base; ROOT integrates at named points; ST and SA serialized or split by file; SP and SR-RS each in their own worktree | **§2:** the worktree and branch per slice. **§4:** the host rules; integration points I1, I1′, I2, I3, I4 and I5; phases 0–7 with the implementers per phase (never more than three). **§2.1:** ST precedes SA, and the seam lands in ST. **§2.3:** SA is split from SP by file. **§2.4:** SR-RS in its own worktree. **§7:** decision 1. **§8:** risk 14 |
| **SF-2:** seven oracles; the runner's literal; `retained_w1`'s refusals | Seven oracles plus the D1.4 line; the runner uses a literal C + 1 with a tying test; no D1 visibility change; SP states `retained_w1`'s refusal of c > C and of combinations | **§0:** finding 4. **§2.1:** the four facade oracles at `caps::LOAD_CASES + 1`, including `u3g2_no_permit_path_runs_once_without_a_copy`. **§2.3:** the law-test and `retained_memory.rs` oracles; the D1.4 line of `every_family_clause_refuses_with_its_fact`; the runner's literal 4 with its PP-side tie test. **§2.2:** `retained_w1` returns `Domain` for c = 0, c > `LOAD_CASES`, or any combination, with its mutant. **§1:** no visibility change. **§7:** decision 3 |
| **SF-3:** fence and guards | Add s11f's site test and PP-tests' admission test, every guard inside the fence, and `grant2.rs`; new accumulation sites go into s11f's table | **§1:** the fence rows (`s11f_site_test.rs`, PP-tests' `retained_precision_admission.rs`, `grant2.rs`) and the complete guard list, inside and outside `src`. **§2.2:** rule-8 shapes and `TABLE` rows; the serializer's forbidden reads. **§5:** RV-P reviews the new `TABLE` rows. **§8:** risk 8 |
| **SF-4:** the SA–SP interface | The running G-B total lives in SP's capture; `CompleteFacts` carries the requested count; SA's three-case gate tests are re-pinned at SQ | **§2.1:** the seam (`ProductCapture.late_loads_total`, `CompleteFacts.requested_cases`, and the law tests' seven literals), in ST, before SA forks. **§2.3:** G-B's `CaseLoadsTotal` (`LATE_FACTS` 9 → 10); `ordinary_solve_attempted(capture, requested)`; the form-valued tests assert expressions and are re-pinned at SQ, which RV-Q round 1 reviews as expressions. **§3.3:** the re-pins. **§7:** decisions 6 and 22 |
| **SF-5:** the measurement | Record the furthest phase; SW seeks a publishing cap-maximal input, or W3–W5 are stated unmeasured; the counting-allocator peak beside RSS, with `CAP_BYTES` raised within 64 GiB; one mode per process; the host statement; a new ROOT point to report to the owner | **§2.0:** item 2, the publishing input, with construction (c) and its stop. **§3.4:** one entry point per mode. **§3.5:** `CAP_BYTES` = 16 GiB; the bound chosen by phase; new phase constants. **§3.6:** the furthest phase; the counting-allocator peak from the same process; one mode per process; the phase-coverage, host, build and RSS statements. **§4:** R9. **§7:** decisions 14, 24 and 25. **§8:** risk 10 |
| **SF-6:** the src-tauri gate | Add it to the full suite before the freeze and to PR-B1's gate set | **§3.9:** the method (U9 G7's: archives of base and candidate, `cargo test --offline --locked --no-fail-fast`, identical per-test outcomes). **§6:** gate items 3 and 6, and the package |
| **SF-7:** a fresh complete-diff reviewer | A fresh, independent complete-diff reviewer for the whole PR, in addition to the slice reviews; decision 16 is overruled | **§5:** RV-X. **§6:** gate item 2. **§4:** phase 7. **§7:** decision 16. **§10:** RV-X's hours and its repair round |

## The notes, as assigned

| Note | Answered in PLAN_v2 |
|---|---|
| **N-1:** how SP observes W-C2's outcomes before SR-RS | **§2.2:** assert `Precommit { G5, ATTEMPT_MISMATCH }` on the private driver, and read the per-case outcomes through the existing `before_precommit` hook. **§0:** finding 6. **§7:** decision 7 |
| **N-2:** kernel owners are batch ordinals | **§2.2:** T-8 maps ordinals to request indices for `owner_refs` and the Run's `owner_ref`; W-C2's {0, 2}; an acceptance test and a mutant. **§5:** RV-P's oracle |
| **N-3:** the verdict lookup by id | **§2.1:** the lookup is by `basis_ref.ref_id`, with a unit test and a mutant |
| **N-4:** `grant2.rs` and a per-case hook | **§1:** `grant2.rs` is in the fence. **§2.2:** `fail_preparation_of_case(index)`. **§7:** decision 23 |
| **N-5:** a multi-case coexistence pin | **§2.2:** a committed pin built from n05 with a second case, with its fallback. **§3.10:** reused by SG. **§7:** decision 26 |
| **N-6:** the D38 audit for all three readers | **§2.4:** each reader's owner lists its own checks. **§5:** RV-R checks all three |
| **N-7:** text and depth stress; the admission assertion | **§2.0:** item 3, with RV107's census figures. **§3.4:** escaping, depth 16 and `assess` |
| **N-8:** what the registration re-pins | **§3.3:** the law tests' `const M`, the threshold, the margin assertions, `PINNED_RECORD` and its ratio, `challenge_bounds_are_the_profile`, SA's form-valued tests, and the challenge's constants, all assigned to SQ. **§1:** the registration's scope |
| **N-9:** ordering inside the phases | **§4:** every repair and confirmation lands before G5-final; SC starts after RV-P round 2's repairs; the SR chain is marked near-critical, and B6's PR goes first. **§2.5:** SC's start condition |
| **N-10:** calibration and repair rounds | **§10:** repair rounds for RV-Q (two), RV-P (one to two), RV-R and RV-X, calibrated against U4's G5–G7 (RV87's two NOT CONFIRMED, and RV89's three routings); the U8 and T6S wall-time caveat |
| **N-11:** the B2 consequence is unpriced | **§0:** finding 8. **§7:** decision 21 ("likely; the combination's price is not yet computed"). **§11:** item 2 |
| **N-12:** the contingency's citation | **§9:** C = 3 trims cite STUDY §3.2 (`t_c3_m12`, `t_c3_k20_l128`); ADD §2 is C = 4 |
| **N-13:** SA's mutants; the EnvelopeResults bound | **§2.3:** the EnvelopeResults ≤ 3·P_final law test, and the five mutant groups |
| **N-14:** B6's actual change set | **§6:** the eleven files at `a79dbd2e4a`, with 07m unchanged (`c21112fd…`); the fixture note left to B6's reviewer. **Basis** paragraph |
| **N-15:** relaying the measurement to the owner | **§4:** R9. **§3.6:** its last paragraph. **§6:** the package |
| **N-16:** batch outcomes; TEXT loop rules | **§2.2:** a batch-against-one-case comparison, with any difference returned as a finding. **§3.1:** the loop rules are written for every new or changed loop, with I82's rebinds as the checklist. **§5:** RV-Q's site-by-site comparison. **§4:** R8. **§8:** risk 13 |

## Decision 15, as ruled at R1

B6 goes first, as its own compact PR. PLAN_v2 §6 and §7 record it, with B6's gate set (items 1–6) and its actual files `[r1: D15]`.

## What else changed, and why

- **Basis.** NUM `ef43a1d694`. The code is unchanged since revision 0: `git diff` from `f59542f71b` is empty outside `P/execution`. B6 is at `a79dbd2e4a`.
- **Estimates (§10).** Agent hours go from 67–102 h to 88–137 h, and review hours from 26–41 h to 38–61 h. Elapsed time goes from 6–9 to 8–12 working sessions. The causes are the repair rounds, RV-X, the measurement changes, the seam, oracles and hooks, the per-reader D38 audits, and the three-implementer sequencing.
- **Decisions 22–26 are new** (§7). They record choices this revision makes to carry out the amendments: the seam's placement, the hook's name, the challenge's cap and bounds, SW's publishing construction, and the coexistence input.
- **R2 is retired** (§4), because option S3 is selected.
