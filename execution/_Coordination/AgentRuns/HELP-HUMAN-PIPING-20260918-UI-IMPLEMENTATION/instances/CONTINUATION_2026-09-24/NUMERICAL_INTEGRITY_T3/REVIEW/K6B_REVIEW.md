# RV22: independent review of slice K6b (W1 observations on the K6 harness)

- **Reviewer:** RV22 (Type 2 TASK, independent reviewer). I did not write K6b.
- **PR:** [#1058](https://github.com/sgttomas/chirality/pull/1058), branch `codex/piping-k6b-20260929`.
- **Head reviewed:** `1123d19b9` (`1123d19b9ab0a39b7ad5bdb33fc707846ae58a9c`). Base main `0f5d8c7b4` (KF1), merged into the branch by ROOT as `b86081221`.
- **Date:** 2026-09-29.
- **Verdict: PASS.** No BLOCKING finding. 3 SHOULD-FIX and 7 NOTEs.

**In short.**
- **No product byte changes.**
  - FK's diff against main is A0 alone, visibility only. It has the same patch-id (`b43efeb4…`) as K6b's `bb89e4f8f` and V-K's `3018343c2`.
  - Its 260 removed lines are 240 `pub(crate)` and 20 dead-code markers. Its 269 added lines are those 240 edits as `pub`, plus the 29-line facade.
  - The facade exports exactly K4 `RETURN.md` §16's list, less `PrecisionState` (C-2). `PrimitiveSource`, `RetainedSolve`, `CaseLimit`, `InvocationMeter`, `AttemptWork`, `WidthWork` and `SumWork` keep private fields.
  - No exported function accepts a matrix, factor, closure or label. The only exported entry that takes a K4 object is `RetainedCombination::solve`, and it takes `&RetainedSolve`, which only K4 can construct.
  - `retained_api` is named only by `FK/structural.rs` and by H. No `Cargo.toml` but H's own names H, and no manifest or lock changes. H builds with 0 warnings. The DEC-050/053 pins pass.
- **The W1 mode is correct where it counts.**
  - My decode of the adapter's K4SRC agrees with R1's `model_json(full=True)` on all 12 RF-LARGE models at 10 and 100 members. Every node, member end, E, G, constraint, load and station matches exactly. The section values are K6's binary64 formula, by design, within 2.1 ulps (A) and 5.5 ulps (I, J) of R1's exact values.
  - My four release `w1a` runs, with prefixes, reproduce b3's counts, outcome, attempt, parity and prefix lines, the relative stage heap peaks and the rows dumps, byte for byte.
  - I re-derived the segment ends and prefix limits from the attempt lines.
  - I recomputed the work closure, the stage identities and the per-precision charged totals from the raw attempt lines: 330 outcomes and 660 attempts, 0 discrepancies.
- **The W1 estimate.**
  - My Python re-implementation reproduces E_max and E_sel128 on all 33 committed counts lines.
  - KF1's tracker terms are correct bounds at every site.
  - E_max bounds every measured run with a wide margin.
  - It is not an upper bound everywhere it claims to be (RV22-2), and its solve-phase terms are untested (RV22-3).
  - The runner's backstop is the binary's refusal: the same inequality, applied to the same figure.
- **The records hold.**
  - Both `SHA256SUMS` verify (3 and 1,777 files), and the folder's coverage is exact.
  - The packet regenerates byte for byte from the committed records.
  - 934 RETURN values in §6.1–§7.2, and the §7.4 figures, trace to the raw JSONL with 0 mismatches.
  - The 13 trimmed files and the 525 dropped ones match their recorded sha256. The other 1,152 b and b3 record files are byte-identical to I16's originals.
  - No machine paths appear. The pre-KF3 labelling is complete.
- **The merge `b86081221` adds exactly main's delta:** the same patch-id and the same stat, and its combined diff is empty.
- **Mutations.**
  - I re-ran five of I16's mutants (M1, M15, M17, M6b and M14), and all are killed.
  - Of my eight, four are killed and four survive (RV22-M2, M5, M6 and M7). One of the survivors, RV22-M6, is on the estimate (RV22-3).
- **The three SHOULD-FIX findings:**
  - **RV22-1:** yes, the fixed `stages_equal_totals` can hide a real mismatch on completed builds. It relaxes every `Failed(Stop(_))` attempt, including a candidate that stopped in its stop rule with every build complete. It also relaxes both sums, though only one can be short.
  - **RV22-2:** E_max is not an upper bound on the 1024 verification pass. By code reading, that phase holds about 2–5% more than E_max counts at 10,000 members. RETURN §9.1 says the estimate "bounds the peak".
  - **RV22-3:** the estimate's solve-phase terms (KF1's solve trackers, the fallback row list, `residual_rows`) are untested, and no test ties the committed `counts.jsonl` to the code. RV22-M6 survives, and it changes E_sel128 on 12 committed lines.

## Findings

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| RV22-1 | SHOULD-FIX | `H/src/k6/w1/staged.rs:183-185` (`builds_completed`), `:203-212` (`stages_equal_totals`); `H/src/bin/k6_observe/w1.rs:150` (`stages_complete`). K4 marks `Failed(Stop(_))` at `K4R/adaptive.rs:2979`, `:3004`, `:3028` and `:3057`. | **This answers ROOT's question: can the fixed check hide a real mismatch on a completed build? Yes.**<br>– `builds_completed` is `!Failed(Stop(_))`. K4 also sets that outcome when every build completed: on a stop in the stop rule (`:3055-3058`), and after the post-build budget checks (`:2749`; `:2897`).<br>– On such an attempt the equality is replaced by ≤ on both sums.<br>**My probe** (`checks/probe_relaxed_check.out`; `scripts/rv22_probe.rs.txt`, not committed to H) runs CHAIN-n00010-AX with six case limits:<br>– At the `verify_256` and `into_decide_128` limits, the 128 candidate is `Failed(Stop(Budget(Case)))`, with own = 375,671/375,671 and 958,933/958,933 and shared = 2,074,272/2,074,272. Every build completed. Taking 1 LME off an own stage, or off its completed shared build, still passes.<br>– On the genuinely stopped builds (`solve_128`, `solve_256`, `into_uc`, `into_solve_128`), exactly one side is short: (0, 28,740), (0, 32,946), (0, 102,396) and (100,152, 0). A 1-LME shortfall on the complete side still passes.<br>– The ruling said "checked on completed builds only; on a failed build … at most the charged total". The code relaxes by outcome, not by build.<br>**Nothing is hidden in the records.** All 25 stopped attempts in b3 (the five Span models × 5 repeats) have own unstaged 0 and shared unstaged > 0 (`checks/closure_check.out`). Row 247 is the same in b2 and b3.<br>The attempt line would also record `stages_complete: false` for a stop-rule stop whose stages are complete. | Hold a `Failed(Stop(_))` attempt to "at most one side short": `own == own_total \|\| shared == shared_total`, each side ≤ its total. This holds on all six probed paths and on every b3 attempt.<br>Report `stages_complete` as `unstaged == (0, 0)`.<br>Add the `verify_256`-limit path (a candidate stopped in its stop rule) to `a_build_that_stops_partway_is_checked_against_its_charged_total`, with a 1-LME perturbation on each side, and show it fail.<br>KF3 restores equality later (ROOT's KF3 ruling); this tightens the interim. If ROOT prefers, record it as a NOTE in RETURN §7.3 instead. |
| RV22-2 | SHOULD-FIX | `H/src/k6/w1/counts.rs:429-431` (`report`, `pass`), `:412-414` (`solve`), `:350-351` and `:11-15` (the claim); RETURN.md:371 ("so the estimate bounds the peak rather than tracks it"). | **The 1024 verification pass.** `verify_state` keeps these alive through the pass (`K4R/verify.rs:731-925`):<br>– `w_abs`, `delta_full` and `w_s` (3n wide);<br>– `u_free`, `r_hat`, `sr_row`, `delta`, `sr2_row`, `sas_inf_row`, `sas_one_col` and `sau_row` (8n_f);<br>– `recover`'s output for δ̂ ((rows + 6m) wide, `:828`);<br>– `terms_w`/`terms_p` headers.<br>With a shift, `shifted_factor` also clones the scaled profile's rows (`K4R/bound.rs:560`, called from `verify.rs:959-966`), with its `shifted` and `work` vectors.<br>The formula counts 5 row vectors + 2n_f wide + one profile.<br>**My re-derivation** (`scripts/rv22_estimate.py.txt`) first reproduces E_max and E_sel128 on all 33 committed lines. Adding the omitted terms (the larger of at-the-shift and after-the-shift) moves E_max:<br>– +5.4% on CHAIN and TREE at 10,000 members (pass_1024 is already their max phase);<br>– +2.1% on CONT at 10,000 (pass_1024 overtakes decide_1024);<br>– 0 at ≤ 1,000 members and on the DEC-053 nine, where the stop rule's 19.8 MB tracker term dominates.<br>(`checks/estimate_rederivation.out`)<br>**The solve phase.** Also omitted: the gate fallback's `abar_q` (nnz·w_R) and its per-member blocks while `assemble_bounded` runs; `evaluated` (up to 4·n_f·w_L); `rhs`; `u_free`; and the per-state copy of `u` (`K4R/adaptive.rs:1456-1476`, `:1606-1612`). On every sealed model these are absorbed by the slack in the 2·n_f row-list term.<br>Under the move model (which ρ uses), the row list's last growth can hold old + new, up to n_f·4,304 B more (246 MiB at CHAIN-10,000). That would take solve_1024 to +7.1% of E_max.<br>**What it does not affect:**<br>– the measured paths (heap/E_sel128 ≤ 0.865, heap/E_max ≤ 0.31, ρ_fp ≤ 0.43);<br>– b3's admissions: the largest E/(heap cap/2) is 0.71, and +5.4% keeps every row admitted (`checks/b3_admission.out`).<br>It matters for ROOT's use of E_max as "the upper bound derived from the code" (A1 ruling) when setting W1's limits. | Either add the terms and regenerate `counts.jsonl` (the post-KF3 addendum regenerates it anyway), or qualify RETURN §9.1 and `counts.rs:11-15`/`:350-351` now: "bounds the modelled phases; the 512/1024 pass omits …, ≤ 5.4% at 10,000 members".<br>Do one of them before ROOT's W1 limits use E_max. |
| RV22-3 | SHOULD-FIX | `H/tests/k6b_w1.rs:381` (`the_estimate_equals_a_hand_derivation`); `H/observations/k6b/counts.jsonl`; `H/runner/k6_runner.py:518` (the runner reads the file's figure). | **The solve-phase terms are untested.** The hand derivation runs only at CHAIN-n00010-AX. There, E_max and E_sel128 are both set by the stop rule's tracker term, so no solve term reaches either result.<br>**RV22-M6 (`SOLVE_TRACKER_PEAK_ROWS` 2816 → 2048) survives** K6b's four test binaries and the runner suite (`mutations/mut_results.jsonl`).<br>– It changes E_sel128 on 12 of the 33 committed lines: every RF-LARGE model at 1,000 and 10,000 members, where solve_256 sets E_sel128 (`checks/estimate_m6_effect.out`).<br>– The same holds for the fallback row list and `residual_rows` terms, which ROOT asked me to re-derive.<br>**No test ties the committed counts to the code.** The runner admits on the file's `estimate_adm_bytes_w1a`, while the binary recomputes its estimate from the file's W1 counts. They are equal today (my re-implementation reproduces all 33 lines), but only because they were regenerated together (`082990c8d`). | Add a Rust test that parses every line of `observations/k6b/counts.jsonl` (`parse_counts_line`) and asserts that `estimate(…)` equals both `estimate_adm_bytes_w1a` and `estimate_w1_sel128_bytes`. It takes well under a second and kills RV22-M6.<br>It also binds the backstop's two figures ahead of the post-KF3 addendum. |
| RV22-N1 | NOTE | RETURN §6.1 and §7.2 (the `heap/E` columns); `_run_records/d/d_tables.py.txt` (T1 and T7). | "heap MiB" is the in-place `repeats_heap_peak`, but "heap/E" divides the move-model `repeats_heap_peak_move`. They differ at 10 and 100 members: CHAIN-n00010-AX gives 0.0484 in-place against RETURN's 0.0515.<br>With that definition, all 518 §6.1 values match the raw JSONL (`checks/tables_6_1.out`). | Label the column "heap move / E". |
| RV22-N2 | NOTE | `H/runner/test_k6_runner.py:716-742`. | **RV22-M7 survives:** the runner's backstop compares with `rss_cap_bytes // 2` in place of the binary's heap cap. The test's rows lie at 9.23 GB, at exactly heap cap/2, or far below both. A row between 3.75 and 4 GiB would pass the mutant runner and be refused by the binary.<br>RV22-M8 (`>=`) is killed. | Add a row at `heap_cap_bytes // 2 + 1` and assert it is deferred by name. |
| RV22-N3 | NOTE | `H/src/bin/k6_observe/w1.rs:258-282` (`prefix_line`). | **RV22-M5 survives:** the binary's `w1_prefix_segments` item forced to true. `k6b_bin` asserts that every item is true and that there are three prefix lines. The library's segment equality is tested (`k6b_w1.rs:322`), but the binary's own slice and `budget_stop` logic has no negative case. | Unit-test `prefix_line`'s predicate on a truncated or unequal segment list. |
| RV22-N4 | NOTE | `H/src/k6/w1/staged.rs:151-160` (`charged_by`). | **RV22-M2 survives:** the `shared_built_here` and `verification_shared_built_here` branches are ignored. The mutant is equivalent on every model W1a runs: within one case, K4 builds each precision's shared data once (`K4R/adaptive.rs:2963-3090`). Only a multi-case group reuses it, and `solve_cases` is tested with one source (C-9). | Add a two-case `solve_cases` test (same stiffness, different loads) asserting `work_closes`. Or record that the branch is untested. |
| RV22-N5 | NOTE | RETURN.md:97 (§5, "kept the 1-minute load at 3.4–6.0 throughout"). | The records show 1-minute loads of 5.59–6.65 in b, 3.40–4.72 in b2 and 3.55–6.48 in b3 (`checks/load_ranges.out`). Every row carries its own load, and the §6.1 load column is right. | Correct the range when the records are next touched (addendum 1). |
| RV22-N6 | NOTE | `_run_records/a1/k6b_adapter_check.py.txt`; RETURN §4 Q6(a). | I16's check builds the section with K6's binary64 formula, so "EQUAL" means equal to R1's model realized through that formula. The plan says this (§3.2), and it is right by design.<br>My decode-based check agrees exactly on every other field. The section is within 2.1 ulps (A) and 5.5 ulps (I, J) of R1's exact values, far inside the 1e-9 predicate (`checks/adapter_check.out`). | None needed; RETURN §4 could say "R1's model through K6's section formula". |
| RV22-N7 | NOTE | RETURN §7.3 F3 and §13.5 ("For F2a"). | F3 names the verification's shared build (`verify.rs:471-485`) as the error path whose stages fall short. My probe also finds remainders on `build_shared` (the `solve_128` limit: shared 0/28,740) and `solve_case_at` (the `into_solve_128` limit: own 576/100,728). KF3's ruling already covers all four builds. | For KF3: use these two constructed limits as test paths. |

## 1. No product byte changes

- **The export** (`checks/export_scan.txt`):
  - It is visibility only, as quoted in the summary above.
  - I checked the facade item by item against K4 `RETURN.md` §16's export list and its "Revision 5a.3 adds" line. It matches, with `PrecisionState` and `RetainedSolve::state` left out (C-2).
  - The exported functions take scalars, slices of binary64 values, `&PrimitiveSource` or `&[QuantityMeta]`. `solve_case`/`solve_cases` take sources. `RetainedCombination::solve` takes `&RetainedSolve`, whose fields are `pub(crate)` or private.
  - Nothing exported builds or accepts a matrix, factor, closure or label: `ExactWideSum`, the formation and factor functions, `bound.rs` and `directed.rs` stay private.
  - `classify_rows(_floored)` return a `Publication` from caller values. This is K4's approved 5a.3 list, and it cannot feed the solver.
- **The product scan.**
  - The PR's files outside H and the K6B records are the 11 FK files of A0.
  - `retained_api` is named only by `FK/structural.rs`, H's sources, tests and README.
  - The only `Cargo.toml` naming H is H's own, and there is no `Cargo.toml` or `Cargo.lock` change.
  - `git grep` finds no `structural::retained` path outside FK apart from H.
- **H does not reach a product path.**
  - H is a standalone package that nothing depends on.
  - The shared K6 modules change additively. `Mode::W1a`, `K6Counts.w1` (None in K6's modes) and three stages are added. `--counts-only` now also computes W1's O(nnz) counts.
  - K6's 138 plan rows are unchanged on every field. Only `pass` is added (`checks/plan_rows_check.out`).
- **Suites** (clean `git archive` of the head, my own target):
  - H's debug suite: `cargo test --all-targets` passes 71 libtest tests and `k6_alloc`, with 0 warnings, in 103 s wall from a cold target (`suites/h_suite.log`; an observation, not comparable to RETURN §11's warm figures).
  - The runner suite passes 47 of 47.
  - The pytest wrapper with the two DEC-050/053 pins passes 49 of 49.

## 2. The W1 mode

- **The adapter against `references.py --model`** (R1 `80d473a7…`; `scripts/rv22_adapter_check.py.txt`). My check decodes the K4SRC that `k6_observe --emit-source` prints, following `source.rs`'s `encoding`/`encode_stiffness_part`. It then compares each field with R1's `model_json(defn, full=True)`:
  - node coordinates, bit for bit;
  - member ends, by R1's labels;
  - E and G, bit for bit;
  - Iy = Iz and J = 2Iy, exactly;
  - `y_reference`, a unit axis not parallel to the member;
  - the constrained set, equal to R1's rigid supports, every value +0.0;
  - the loads, equal to R1's nonzero components (bits), with source ids `k6:<DOF>`;
  - one station per member at 0.5;
  - no springs, directional springs or support groups;
  - nothing in R1's model that K4SRC lacks.
  - **Result: 12 of 12 agree at 10 and 100 members.** The section is as in RV22-N6.
- **The staged sequence and prefixes.**
  - `w1_solve` is exactly `solve_case` under the observer (`staged.rs:68-88`).
  - I re-derived `segments` against `run_schedule` (`K4R/adaptive.rs:2963-3090`): the solve of an attempt (shared + own − verification − stop rule), its verification (shared + pass), then the previous candidate's decision. This is correct in case-work units: K4 charges shared work in full against the case (`:2742-2745`). On CHAIN-n00010-AX, my recomputation gives limits 2,367,491, 5,447,988 and 8,002,626, and the last segment ends at the meter's 9,129,471.
  - Each prefix ends `Unresolved(Budget(Case))`, and its completed segments equal the full call's (my runs, and b3's).
- **Determinism against b3** (`checks/w1a_runs_compare.out`). My release runs of CHAIN-n00010-AX, CHAIN-n00100-AX, CONT-n00100-AX and TREE-n00100-ROT (5 repeats, prefixes, rows dump) give byte-identical counts, outcome, attempt, parity and prefix lines and relative stage heap peaks. The rows dumps are identical too. All 11 parity items are true in each run.
- **The work closure, the stage identities and the per-precision charged totals** (`checks/closure_check.out`, my own code over every b3 `w1a` process and repeat):
  - Σ over attempts of own + shared-if-built + verification-shared-if-built equals `meter_charged` on all 330 outcomes.
  - Every attempt not marked `Failed(Stop(_))` has its stage sums equal to its totals.
  - `own_unstaged`/`shared_unstaged` equal the recomputed remainders.
  - `work_<p>_{attempts,own,shared}` equal the per-precision sums of the charged totals.
  - Row 247's attempts are identical in b2 and b3. Only the outcome's per-precision shared figure changed, which is the fix.
- **`stages_equal_totals`:** RV22-1.

## 3. The W1 admission estimate

- **The formula as coded.** My independent Python re-implementation (`scripts/rv22_estimate.py.txt`, using this build's `size_of` figures from the probe) reproduces `estimate_adm_bytes_w1a` and `estimate_w1_sel128_bytes` on all 33 committed lines.
- **KF1's sites, re-derived from `K4R/adaptive.rs` after KF1:**
  - **The stop rule** (`rule`, `:1893`; one `TrackerSet` for (a), (b) and (d), `:1976`). The set collapses when the held capacity passes G = 4,096 (`:783-812`). One offer can grow one tracker from c ≤ 256 to 2c while the old buffer lives, so the peak is G + T = 4,608 rows × 4,304 B.
    - The tables hold at most one 40 B entry per offered row per test, plus prune transients: 3 × 2 × rows × 40 is right to within a 512-entry collapse transient.
    - The coded `decide` term is exactly this.
  - **The pivot margin** (`:1236`): one standalone tracker, whose peak is 1.5T = 768 during a growth. Its table has ≤ n_f entries.
  - **The residual gate and fallback** (`:1301`, `:1437`):
    - the gate's tracker of the current iteration, ≤ 512 at rest, is alive through the fallback;
    - the fallback keeps up to 3 finished state trackers (≤ 512 each) while the fourth grows (768);
    - that is 512 + 3·512 + 768 = 2,816 rows, as coded;
    - its tables hold ≤ 5·n_f entries.
  - **`residual_rows`:** n_f × (16 + w_L), exact.
  - **The fallback row list** (`:1476`): at most n_f entries of (ExactWideSum, ExactWideSum, f64) = 4,304 B, one state at a time. Its in-place capacity is below 2n_f, so VEC_SLACK = 2 bounds it. The move-model caveat is in RV22-2.
- **Is it an upper bound?**
  - On the paths K6b measured, yes, with a wide margin: heap/E_max ≤ 0.31, ρ_fp ≤ 0.43, heap/E_sel128 ≤ 0.865.
  - As a derived bound, no, on the 512/1024 verification pass (RV22-2).
  - At 1024 the modelled phases lie within about 8% of each other (solve_1024 −1.9%, vbuild −2.6%, pass 0, decide −0.1% on CHAIN-10,000), so any omitted term can move the maximum.
- **The backstop.**
  - The runner defers when `estimate > heap_cap_bytes // 2` (`k6_runner.py:562`). The binary refuses when `admission_estimate_bytes > cap / 2` (`main.rs:656-658`). This is the same inequality in integer arithmetic.
  - For W1a both use E_max: the runner from the counts line, the binary recomputed from the same line's W1 counts. They are equal at the head, and b3 ran a `4eeb206c0` binary against a counts file from `082990c8d`, whose `counts.rs` it shares.
  - Every admitting branch of `admission()` passes the check.
  - The remaining gaps are RV22-3 (the file is not tied to the code) and RV22-N2 (the cap is not pinned by a test).

## 4. The records

- **SHA256SUMS** (`checks/records_checks.txt`):
  - `H/observations/k6b/SHA256SUMS` verifies 3 of 3.
  - `T3/IMPLEMENTATION/K6B/SHA256SUMS` verifies 1,777 of 1,777, and it lists every file in the folder, and nothing else.
- **The packet.** `k6b_analysis.py --packet --records _run_records/b3/records`, with D's three notes, reproduces `f97ea239…` byte for byte from the committed records.
- **RETURN's numbers against the raw JSONL**, recomputed by my own code, not `k6b_analysis.py` or `d_tables.py`:
  - §6.1: 518 values, 0 mismatches, with heap/E as in RV22-N1 (`checks/tables_6_1.out`).
  - §6.2 (per-attempt work, 264), §6.3 (prefix heaps, times and increments, 86), §7.2 (ρ_fp and heap/E ranges, 50) and §7.1 (heap-move slopes, 16): 0 mismatches (`checks/tables_6_2_to_7_2.out`).
  - §6.5: 1,668 parity lines, 0 false.
  - §6.4: `r1_compare.out`'s 19 lines sum to 10,293 comparisons.
  - §7.4 CONT-AX at 10,000:
    - own 2,774,962,760, with a stop rule of 2,596,783,796 (31.5%);
    - the pass: scale 26,375,782, estimate 126,810,091, charge 135,922,335, bound 942;
    - uc 112,052,571;
    - 265,013 rows, split 148,718 / 101,289 / 15,006 / 0;
    - W1/binary64 ×7.55 and ×7.59 in the two passes.
  - The net-RSS slopes of §7.1 were not recomputed.
- **The trimmed and dropped files.**
  - I checked the 13 trimmed rows dumps (b and b3) against I16's untrimmed originals in `<wt>/scratch/i16` (read only). The sha256, bytes and line counts match, and each committed `.head.txt` equals the original's first 40 lines.
  - All 525 of b2's dropped files match `DROPPED.txt`.
  - The other 531 (b) and 621 (b3) committed record files are byte-identical to the originals.
- **The pre-KF3 labelling is complete:**
  - RETURN's header and every section heading with 10,000-member figures (§6.1–§6.3, §7.1–§7.2, §7.4, §8.1 and §13.5);
  - CHANGE_RECORD's header and Results;
  - the packet's notes, `tables.md`'s header and H's README.
  - §8.2–§8.3 rely on the header.
- **Machine paths.** None appear in the K6B folder or H. The only pattern hit is the scrubber's own regex in `d/assemble_records.py.txt`. There are no non-empty binary files.
- **b3's runs:**
  - 132 records, all `ok` and admitted, with no parity failure, watchdog kill, timeout or stop reason;
  - W1-T4 ran by the recorded override (`conditional: false` in b3's rows), as ROOT approved (`checks/b3_admission.out`).

## 5. Mutations

Each mutant ran from a clean `git archive` of the head (`projects/chirality-piping/core`), with its own target, deleted afterwards (`scripts/rv22_mut.py.txt`; `mutations/`). Rust mutants run K6b's four test binaries (`k6b_w1`, `k6b_bin`, `k6b_adapter`, `k6b_export`). Python mutants run the runner suite.

| Mutant | Change | Result | Killing test |
|---|---|---|---|
| NONE, NONE-PY | the head | pass | — |
| M1 (I16) | the adapter drops the last member | killed | 14 tests, e.g. `the_adapters_bytes_equal_the_independent_python_bytes` |
| M15 (I16) | a stopped build held to equality | killed | `a_build_that_stops_partway_is_checked_against_its_charged_total` |
| M17 (I16) | a stopped build passes unchecked | killed | same |
| M6b (variant) | V(P) never added to the kept bytes | killed | `the_estimate_equals_a_hand_derivation` |
| M14 (I16) | admission omits the backstop | killed | `test_every_admitted_row_passes_the_binarys_backstop` |
| RV22-M1 | `builds_completed` always false (every attempt relaxed) | killed | the stopped-build test, `w1a_prints_its_lines_and_every_parity_holds`, `work_by_precision_…` |
| RV22-M2 | `charged_by` ignores the built-here flags | **survives**, equivalent on single-case models | RV22-N4 |
| RV22-M3 | station fraction 0.25 | killed | R1's predicate tests at 10 and 100 members, `the_adapters_bytes_equal_the_independent_python_bytes` |
| RV22-M5 | the binary's `w1_prefix_segments` always true | **survives** | RV22-N3 |
| RV22-M6 | `SOLVE_TRACKER_PEAK_ROWS` 2816 → 2048 | **survives**; changes E_sel128 on 12 committed lines | RV22-3 |
| RV22-M7 | the backstop against `rss_cap_bytes // 2` | **survives** | RV22-N2 |
| RV22-M8 | the backstop with `>=` | killed | `test_every_admitted_row_passes_the_binarys_backstop` |
| RV22-M9 | a completed build's own side held only to ≤ | killed | the stopped-build test |

- **I16's table** (C and fix: 24 mutants, 23 killed, M3e equivalent) is consistent with its `results*.jsonl`. I re-ran five, and all agree.
- **M3e's equivalence holds:** R1's sections are tubes, and my adapter check finds Iy = Iz on every member.

## 6. The merge `b86081221`

`checks/merge_check.txt`:
- Its parents are `1f5c1a6d0` and `0f5d8c7b4`, and the merge-base is `ab02ee3a6`.
- first-parent..merge and merge-base..main have the same stat (448 files, +60,572 −1,418) and the same patch-id (`69e6d2e7…`). The two diffs differ only in `index` and `@@` offset lines, because `adaptive.rs` carries A0's visibility edits.
- `git diff-tree --cc` is empty.
- `adaptive.rs` is the only file changed on both sides, and it merged without manual change.
- The four commits after the merge touch only H and the K6B records. The Rust sources are unchanged since `4eeb206c0`, the binary b3 ran.

## Reviewer, brief and delegation

- **Reviewer.** RV22 is a Type 2 TASK, dispatched by ROOT (HELP_HUMAN, the SWBPIPE session) as a background subagent. ROOT is the only return path. I delegated nothing.
- **Writes.** This file and `T3/REVIEW/_run_records/k6b_review/**` (with its `SHA256SUMS`), left uncommitted. I also used scratch copies under `<wt>/rv22/` and the targets `<wt>/rv22-target`, deleted at the end.
- **Git: read-only.**
  - In `<wt>/k6b`: `rev-parse`, `status --short`, `log`, `show`, `diff`, `diff-tree`, `patch-id`, `merge-base`, `grep`, `ls-files` and `archive`.
  - In `<wt>/numerics`: `log`, `branch --show-current` and `status --short`.
  - `status` may refresh an index's stat cache.
  - No commit, stash, reset, checkout, merge or fetch, and no GitHub access.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `T3/TASK_BRIEFS/_COMMON.md` and `I8R_K1_RESUME.md:20-55`;
  - `I16_K6B_IMPLEMENTATION.md`, and `I15_K6_IMPLEMENTATION.md` where I16 reuses it;
  - `ROOT_RULINGS_V1.md` at the numerics head `b60a2022a`, lines 2077-2583: every K6b, KF1 and V-K section from "K6b and V-K: spawn" to "K6b: D accepted; PR to review", including the W1-T4 stops and the KF1 and KF3 routing;
  - K6b's `PLAN_CHECKPOINT0.md` (§3.2–§3.5), `RETURN.md` (all), `CHANGE_RECORD.md` and the `_run_records/` I cite;
  - K4 `RETURN.md` §16;
  - `REVIEW/K6_REVIEW.md`, for method;
  - the complete diff of H and FK;
  - the K4 code the W1 mode and the estimate rest on: `adaptive.rs` (trackers `:525-840`, the pivot margin, `residual_rows`, `bounded_fallback`, `solve_case_at`, `rule`, `solve_precision`, `verify_precision` and `run_schedule`), `verify.rs` (`build_verify_shared` and `verify_state`), `bound.rs` (`uc_bounds`, `scaled_profile` and `shifted_factor`), `assemble.rs` (`assemble_bounded` and `reduced_rhs`) and `source.rs` (the encoding);
  - R1's `references.py` (`80d473a7…`): `section_props`, `Model`, `build_large` and `model_json`.
- **Ran** (clean `git archive` of `1123d19b9` under `<wt>/rv22/`; `RUSTUP_TOOLCHAIN=1.97.1`, `--offline --locked`, `-j 4`, `RUST_TEST_THREADS=2`, one cargo job at a time; `checks/toolchain.txt`):
  - H's debug suite (`--all-targets`), the runner suite and the pytest wrapper with the DEC-050/053 pins (`suites/`);
  - a release `k6_observe`, then `w1a` at 10 and 100 members only (four models; `w1a_runs/`);
  - `--emit-source` at 10 and 100 members;
  - a probe test in my copy only (`scripts/rv22_probe.rs.txt`);
  - 15 mutant runs, the packet regeneration, and my checks (`checks/`).
  - The memory guard ran throughout, and its log shows no kill.
- **Not done:**
  - FK's full suite, K4's suite, T9 and the both-entry gate (ROOT's gates; A0's visibility-only diff was checked mechanically);
  - runs above 100 members;
  - the net-RSS fit slopes;
  - an independent re-run of the R1 comparison at 1,000 and 10,000 members;
  - the 10,000-member K4SRC decode (I16's A1 check covers it).

## Records (`T3/REVIEW/_run_records/k6b_review/`)

- `README.txt`, `SHA256SUMS`.
- `scripts/`: my checks and probes (`.txt`).
- `suites/`: H's suite, the runner suite, the wrapper, the release build.
- `checks/`: the export and product scans, the merge, the records checks, the RETURN table traces, the closure, the adapter decode, the estimate re-derivation and RV22-M6's effect, the plan rows, b3's admission, the load ranges, my runs against b3, the relaxed-check probe, the toolchain.
- `w1a_runs/`: my four `w1a` JSONL outputs.
- `mutations/`: `mut_results.jsonl`, the driver's output and each mutant's log.
- Paths are shown as `<wt>`, `<scratch>`, `<home>` and `<VENV>`.

## Confirmation at 011911e4e (RV22, 2026-09-29)

- **Head confirmed:** `011911e4e` (`011911e4e9ab8b73b502066089276ab356a03bf9`), one commit on `1123d19b9`. It changes H and the K6B records only.
  - FK is unchanged since `1123d19b9`, and its diff against main still has A0's patch-id (`b43efeb4…`).
  - There is no manifest or lock change, and `retained_api` is still named only by FK and H.
- **Verdict: PASS.** RV22-1, RV22-2 and RV22-3 are closed. There are two new NOTEs (C-N1, C-N2), and no BLOCKING or SHOULD-FIX finding.
- **Read:**
  - "K6b: rulings on RV22's review" (`ROOT_RULINGS_V1.md` at the numerics head `899f28965`), with I16's correction on the 1,000-member change;
  - the KF3 section's RV22 lines, which accept I16's extension to RV22-1;
  - the full diff `1123d19b9..011911e4e` of H;
  - RETURN addendum 2 and CHANGE_RECORD's changes;
  - `_run_records/rv22fix/`.

### C.1 RV22-1: the stage check on stopped attempts

- **The code:**
  - `builds_completed` is `!Failed(Stop(_)) || stop_rule_work > 0` (`H/src/k6/w1/staged.rs:189-190`);
  - on any other stopped attempt, each side is ≤ its total and at least one side is equal (`:224`);
  - `stages_complete` is `unstaged == (0, 0)` (`:206-207`), and the attempt line uses it.
- **My probe, re-run** (`final/checks/probe_relaxed_check_c.out`, CHAIN-n00010-AX at my six case limits):
  - **The two stop-rule stops** (`verify_256` and `into_decide_128`, `stop_rule_work` 82,452 and 665,714) are now held to equality. Both 1-LME under-records, own and shared, fail the check. These were the cases my first probe showed passing.
  - **On the three stopped builds**, a 1-LME under-record on the complete side now fails (`solve_256` own, `into_uc` own, `into_solve_128` shared).
  - **The genuine K4 attempts pass the check on all six paths.**
- **Could it reject a genuine K4 path? No.**
  - Only a candidate's decision charges `stop_rule_work` (`K4R/adaptive.rs:3040-3045`), and it files the same total in `stages.stop_rule`. The decision runs only after the candidate's builds and its verification's have completed. The only later outcome written to that record comes from the same decision (`:3055-3058`). So such an attempt's stages equal its totals.
  - Every other stop falls inside exactly one build or pass, so at most one side is short:
    - in `build_shared`, own is 0 and its total is 0;
    - in `solve_case_at` or the verification pass, shared is complete;
    - in `build_verify_shared`, own is complete;
    - after the post-build budget checks, both sides are complete.
  - A decision that stops before charging any work would fall to the one-side rule with both sides equal, and is accepted.
  - After KF3 both sides equal their totals on every path, which the check accepts.
  - b3's 25 stopped attempts all pass (own exact, shared short).
- **C-N1 (NOTE): a mis-tightening would go unseen.**
  - RV22-C4 survives. It requires the own side to be exact on a stopped attempt, which would reject a genuine stop inside the solve: `into_solve_128`, own 576/100,728, shared exact.
  - The tests cover only shared-short and stop-rule paths.
  - Fix: add RETURN A2.2's `into_solve_128` limit (the 128 shared build's end + 10 LME) to a test asserting that the check passes with own short. This kills RV22-C4. KF3 changes these paths anyway.
- **C-N2 (NOTE): the residual inside the short side.**
  - `shared_stages` sums the attempt's build and the verification's build. On a stopped verification build (the Span path), a shortfall in the attempt's own completed build cannot be told apart from more unstaged `uc` work. In my probe, 1 LME off `formation` at `into_uc` still passes.
  - This is within ROOT's ruling, and KF3's equality closes it.
  - Optional: when `verification_shared_work > 0`, check that the attempt-build fields (formation to condition) sum to `shared_work`.

### C.2 RV22-2: E_max bounds every phase I identified

- **The code** (`counts.rs:475-541`):
  - The solve term now carries the fallback: `u_free`, 4 evaluated states and `abar_q`, then the larger of `assemble_bounded`'s blocks and a state's copy of u plus the row list. The row list is `fallback_row_list_rows` = 1.5 × 2^⌈log2 n_f⌉ (`:274`), its move-model peak.
  - The pass term is the pass's locals plus the larger of (three row vectors, both profiles, the clone's `first`, `shifted` and `work`) and (all five row vectors). The pass phase is `kept + pass[v]` (`:541`).
  - The tables carry the growing buffer.
- **My independent itemization.** I wrote it from K4R under the move model, not from `counts.rs` (`final/scripts/rv22_estimate_c.py.txt`, `final/checks/estimate_rederivation_c.out`). **E_max is at or above my maximum on all 33 committed lines.**
  - **RF-LARGE-CHAIN-n10000-AX:**
    - E_max is 3,009,837,834 B, against my 3,009,836,106 B at pass_1024. The 1,728 B gap is the formula's allowance for prescribed terms, r·(w_W + w_L), which are zero here.
    - The next phases are decide_1024 at −4.84% and solve_1024 at −5.41%.
    - The pass term matches my list item by item: 3n + 8n_f wide, `recover`'s (rows + 6m), 50 B per DOF, and at the shift 3 row vectors, 2·P·w and 68·n_f + n_f·(2w + 8).
  - **CHAIN, TREE and CONT at 10,000:** E_max equals mine to within 0.15% at pass_1024.
  - **At 1,000 members:** E_max is 2.0–2.3% above mine, because the solve term's 4n·w_R also covers u and `rhs`. My maximum is solve_1024 on CHAIN and TREE, which agrees with I16's correction (+3.28%, +2.33%).
- **`counts.jsonl`** (`final/checks/counts_diff.out`):
  - It has the same 33 models in the same order.
  - Added keys: `estimate_w1_solve_{128,256,512,1024}` and `estimate_w1_pass_{256,512,1024}`.
  - Changed keys: `estimate_adm_bytes_w1a`, `estimate_w1_sel128_bytes` and `estimate_w1_decide_bytes` on all 33 lines, plus `phase_elapsed_ns`, a measured time.
  - The 28 `w1_*` count keys and K6's 8 estimate keys are identical.
  - Three counts-only lines from my release build (CHAIN-n00010-AX, CONT-n00100-ROT and DEC053 grid-frame-7x8) reproduce the committed lines except `phase_elapsed_ns` (`final/checks/counts_regen_sample.out`).
- **b3's admissions**, replayed with the head's runner and the new counts, each row against the rows recorded before it (`final/checks/b3_admission_replay_c.out`):
  - 132 rows, 0 decisions changed;
  - the largest W1 E_max/(heap cap/2) is 0.748;
  - the largest projected RSS is 1.48 GB, against 0.8 C = 6.87 GB.

### C.3 RV22-3: the estimate is tied to the committed counts

- `the_committed_counts_carry_this_codes_estimate` (`tests/k6b_w1.rs:661`) recomputes E_max and E_sel128 for all 33 lines.
- `the_solve_and_pass_terms_bind_on_the_large_models` (`:679`) checks the estimate against the hand derivation on the 12 large lines. It also asserts that solve_256 sets E_sel128, and that a solve or pass phase sets E_max at 10,000 members.
- Both pass in my run.
- **RV22-M6 is killed** by all three estimate tests. So are my RV22-C1 (the row list at its in-place capacity C) and RV22-C2 (the shift's profile clone dropped).
- RV22-M7 is killed, which closes RV22-N2.
- The hand derivation re-types the formula with literal constants, so it pins the formula. The independent check that the formula bounds the code is C.2's itemization.

### C.4 RV22-M5B: acceptable

- The predicate now lives in `staged::prefix_matches` (`staged.rs:343`). The prefix test checks that it is true at each prefix, false one segment further, and false on a completed call. RV22-M5L is killed.
- The binary passes its arguments straight through. `main.rs:967` calls `prefix_line(j + 1, …)`, and `w1.rs:276` calls `prefix_matches(j, full, &solve.outcome)` with the 1-based j that the predicate documents. I reviewed the call site.
- I re-ran RV22-M5B, and it survives as recorded.
- A genuine run cannot produce a false prefix, so killing it would need a fault hook in the binary. Recording it is proportionate.

### C.5 Records

- `H/observations/k6b/SHA256SUMS` verifies 3 of 3. `counts.jsonl` has its new hash. `k6b_packet.json` is unchanged: b3's packet, computed against the E b3 ran with, as addendum 2 says.
- `T3/IMPLEMENTATION/K6B/SHA256SUMS` verifies 1,941 of 1,941, and its coverage is exact. There are 164 new files in `rv22fix/`, and RETURN, CHANGE_RECORD and SHA256SUMS changed.
- The 167 added or changed files carry no machine path. The one hit is the scrubber's own pattern in `rv22fix/assemble_rv22fix.py.txt`. There are no non-text files (`final/checks/records_checks_c.txt`).

### C.6 Suites, mutants, host and Git

- **Suites** (clean `git archive` of `011911e4e`; `final/suites/`):
  - H's debug suite (`--all-targets`) passes 74 libtest tests and `k6_alloc`, with 0 warnings, in 84 s from a cold target.
  - The runner suite passes 47 of 47.
  - The pytest wrapper with the DEC-050/053 pins passes 49 of 49.
  - The release build is warning-free.
- **Mutants** (each from a clean archive of the head with its own target, deleted afterwards; `final/mutations/`):

| Mutant | Change | Result | Killing test |
|---|---|---|---|
| NONE, NONE-PY | the head | pass | — |
| RV22-M6 | `SOLVE_TRACKER_PEAK_ROWS` 2816 → 2048 | killed | the three estimate tests |
| RV22-M7 | the backstop against `rss_cap_bytes // 2` | killed | `test_every_admitted_row_passes_the_binarys_backstop` |
| RV22-M5B | the binary's call site forced true | survives (C.4) | — |
| RV22-C1 | the fallback row list at C, not C + C/2 | killed | the three estimate tests |
| RV22-C2 | the pass without the shift's profile clone | killed | the three estimate tests |
| RV22-C3 | `builds_completed` keyed on `verification_work` | killed | `a_stop_in_the_stop_rule_is_held_to_equality` |
| RV22-C4 | a stopped attempt's own side required exact | **survives** (C-N1) | — |

- **Host:** one cargo job at `-j 4`, with `RUST_TEST_THREADS=2`. No DEC-025 sweep was running at my start. The memory guard ran throughout and logged no kill. `<wt>/rv22c` and `<wt>/rv22c-target` are deleted.
- **Git: read-only.**
  - In `<wt>/k6b`: `rev-parse`, `status`, `log`, `diff`, `show`, `patch-id`, `grep` and `archive`.
  - In `<wt>/numerics`: `log` and `status`.
  - No writes, no fetch and no GitHub access.
- **My writes:** this section and `_run_records/k6b_review/final/`, with `k6b_review/SHA256SUMS` updated. All are uncommitted.

## Merge check at 597c81ba4 (RV22, 2026-09-29)

- **Head:** `597c81ba4`. It is ROOT's merge of main `f8400d290` (V-K, PR #1057) into `codex/piping-k6b-20260929`, with parents `011911e4e` and `f8400d290`, and merge-base `0f5d8c7b4`.
- **Verdict: PASS.** There are no findings. The records are in `_run_records/k6b_review/merge_597c81ba4/`.

1. **The first-parent diff is main's delta, less the A0 both sides already carry** (`merge_check.txt`).
   - Checked path by path over 59,471 paths, blob and mode, the merge tree is: K6b's version where only K6b changed (1,966 paths); main's where only main changed (482); and main's on the 11 paths both changed. Those 11 are exactly A0's FK files, and 5 of them were already identical on both sides. There are 0 mismatches.
   - **The stats reconcile.**
     - first-parent: 488 files, +91,067 −0;
     - main's delta: 493 files, +91,336 −260;
     - A0 (bb89e4f8f): 11 files, +269 −260;
     - 493 − 5 = 488, 91,336 − 269 = 91,067, and 260 − 260 = 0.
   - Outside A0's 11 files, the first-parent diff and main's delta have the same patch-id (`02b94fb1…`).
   - **A0 is kept.** K6b's FK to main's FK is +312 −0, V-K's `cfg`-gated fault sites only. Main's FK change since the merge-base removes exactly A0's 260 lines.
   - **The remerge diff is clean** (`remerge.diff`, 44 lines). It covers only the two conflicts, in `adaptive.rs` (`classify`) and `verify.rs` (`e_hat`). Each resolution only drops the markers and the base's `pub(crate) fn` line, keeping main's side; A0's `pub fn` line is the common context. The resolution adds 0 lines.
2. **FK at `597c81ba4` equals main's FK byte for byte:** all 112 FK paths have the same blob and mode, and `git ls-tree -r` of FK is identical in both.
3. **The suites pass on a clean `git archive` of `597c81ba4`** (`projects/chirality-piping/core`; one cargo job at `-j 4`, alongside a DEC-025 sweep's cargo):
   - H's debug suite (`--all-targets`) passes 74 tests and `k6_alloc`, with 0 warnings, in 87 s (`h_suite.log`).
   - The runner suite passes 47 of 47 (`runner_unittest.txt`).
   - FK's `mutation-controls` feature has no default, and H names FK without features, so V-K's fault sites are compiled out of FK as H's dependency. H itself is unchanged by the merge.
- **Git, read-only:** `rev-parse`, `status`, `log`, `diff`, `show --remerge-diff`, `ls-tree`, `merge-base`, `patch-id` and `archive` in `<wt>/k6b`. `<wt>/rv22m` and `<wt>/rv22m-target` are deleted. My writes are this section and the records folder, left uncommitted.
