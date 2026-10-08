# RV107: independent review of I84's B1 implementation plan (documents only)

TASK (Type 2), RV107, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance and wrote none of the plan. 2026-10-07 UTC (2026-10-06 host local time).

**Brief (verified before work):** `R/BRIEFS/RV107_B1_PLAN_REVIEW.md`, sha256 `e0095c32250153635f28c38a40649bd9c8d5e139ba4d6eb4f0732b8696d0fd05`. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**The candidate:** `R/I84/b1_plan_01/PLAN.md`, sha256 `7f9699f34de3a64a587d3e3e5f662b899d3e86ba2d9176b647df624012c4bb5c`. Its SHA256SUMS check 7/7 OK.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics/src`), RE, T, R, RR and VENV as in the dispatch. DESIGN, PROBE, STUDY, ADD, PLAN (I61's), CR and QUAL as the plan defines them. "The plan" is I84's PLAN.md; its sections are cited as §n.

**Basis.** NUM `37481d0f40`. Outside `P/execution`, NUM is main `47a3bdfcf5` plus SI1b's 3 rules-crate files. Every code file I cite has the same blob at main and at NUM's head (`evidence/checks.txt` §4). B6 is read at `WT/b6`, head `a79dbd2e4a` (one commit past the plan's `5e38293530`; corpus 07m is byte-identical in both).

**Limits kept.** Documents and code reading only, plus read-only Python (VENV) on committed bytes. No cargo, vitest, native or solver job; no installs; no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`). Scratch in `WT/scratch/rv107_b1_plan/`; nothing in the system temp directory. The host accepted my writes into this NUM records folder.

## Verdict

**PASS with findings: 0 BLOCKING, 7 SHOULD-FIX, 16 NOTE.**

The plan is sound in structure and nearly complete in coverage. Its four main findings hold, with one count wrong. Its re-qualification matches the milestone's, except that it drops two of the milestone's PR gates: the src-tauri suite, and a fresh complete-diff review. The lanes break ROOT's standing concurrency rule and share one worktree across a single build closure. The new RSS and time measurement may never reach the phases where the priced bound binds. All seven SHOULD-FIX items can be settled at R1 by amending the plan; none needs the owner.

## Findings

| ID | Sev. | Section | Finding and evidence | Remedy |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | §4, §2.1, decision 1 | **The lanes exceed the standing concurrency rule, and concurrent Rust lanes share one worktree and build closure.**<br>• RR "RV98 confirms U8's Pass B; operating adjustments…", item 4: "at most two or three implementers at once, plus reviewers". Phase 2 runs four: SP, SR-RS, SR-PY and SR-TS.<br>• §2.1 puts ST, SP, SA and SQ on one branch and worktree (`WT/b1`), and SR follows on the same branch. ST and SA (phase 1) both edit and build PP. In phase 2, SR-RS edits RS, which PP compiles in for precommit (`retained_w1` calls `retained_precision::validate`). One lane's uncommitted edits then break the other lane's builds.<br>• The precedents for same-worktree concurrency (T6S I75 with I76; U8 I70 with I71) had separate build closures. | • Re-sequence to at most three implementers. For example, stagger SR-PY and SR-TS, or start SR-TS after SR-RS returns.<br>• Give each concurrent Rust lane its own worktree or sub-branch, and have ROOT merge them into the B1 branch at named points; or serialize those lanes.<br>• Name the merge points in §4. |
| **SF-2** | SHOULD-FIX | §0 finding 4, §2.2, §2.3, decision 3 | **The out-of-domain oracle list is incomplete: seven tests, not six.**<br>• `u3g2_no_permit_path_runs_once_without_a_copy` (PP `retained_facade_tests.rs`) also pushes a second case and asserts an admission refusal (`law().refusal.is_some()`). Under C = 3 the registered build admits it.<br>• The full set is four facade tests (the plan's three plus this one), the law tests' `admit_grants_…`, `retained_memory.rs`'s `actual_retained_entry_dispatches_ordinary_once`, and the runner's `explicit_headless_refusal_…`. The D1.4 line of `every_family_clause_refuses_with_its_fact` also changes.<br>• `caps::LOAD_CASES` is `pub(crate)` inside `pub(super) mod caps`. The runner crate and PP's `tests/` therefore cannot derive "C + 1 from the cap constant".<br>• `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` calls `retained_w1` directly and asserts `W1Fallback::Domain`. Its re-based check presumes that `retained_w1` keeps a defensive check refusing c > C or a combination, which `w1_case_id` does today. SP's code list does not say so. | • Add the fourth facade test to SP's list, and the D1.4 assertion to SA's.<br>• For the runner, use a literal 4 with a PP-side pin that `LOAD_CASES` = 3, or the combination oracle.<br>• State in SP that `w1_case_id`'s replacement returns `Domain` for c = 0, c > C, or any combination. |
| **SF-3** | SHOULD-FIX | §1 fence, §8 risk 8 | **Source-text guards outside the plan's list.**<br>• `P/core/product_physics/tests/s11f_site_test.rs`, rule 8: `PP/lib.rs` (non-test code) is in `RULE8_FILES`. Per function, any `+=`, `-=`, `.sum(`, `.sum::<` or `fold(` must match the `TABLE` count exactly, and an unlisted one fails. Rules 1–6 scan `retained_product.rs` too.<br>• `P/core/product_physics/tests/retained_precision_admission.rs` (`legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission`) reads sections of `PP/lib.rs`.<br>• Neither file is in the fence. Risk 8 names only three facade tests in PP.<br>• Guards inside the fence are also unlisted: `u1_serializer_reads_no_legacy_work_field` forbids `.charged()` in `retained_wire.rs` production code; the law tests read `admit`'s source; `u3g2_no_permit_path_runs_once_without_a_copy` reads lib.rs. | • List all of these in risk 8.<br>• Either SP avoids rule-8 shapes in lib.rs's non-test code, or the s11f `TABLE` joins SP's fence, with a disposition for each new row and a review.<br>• I-P runs both integration tests early. |
| **SF-4** | SHOULD-FIX | §2.3 vs §2.2, decision 6 | **The SA–SP interface is unstated, though SA starts at R1 alongside ST.**<br>• G-B runs inside `ProductCapture::prepared_case_source` (`retained_product.rs`, SP's file), with `LateFacts { case, capture, … }`. A running total ≤ 384 needs state in SP's capture, or a model read; today no G-B fact reads `model`.<br>• T-3 (e) needs the requested count, but `CompleteFacts` carries only the envelope and the capture. Adding the model changes `permitted_run` in lib.rs, which is SP's file.<br>• G-B and G-C byte bounds read generated forms and atoms: `profile::F_T11`, `F_T11_LATE_CAPTURE`, `F_T11_ORDINARY_SEED`, `text_atoms::*` (`phase_caps`). These hold c = 1 values until SQ regenerates, so SA's law tests at c = 3 for those facts cannot be final before SQ. | • At R1, define which capture fields and constructor arguments SA reads, and that SP adds them.<br>• Mark SA's form-valued bound tests as re-pinned at SQ, a back edge from SA to SQ.<br>• RV-Q's round 1 then reviews SA's expressions, not their values. |
| **SF-5** | SHOULD-FIX | §3.6, §3.4, decision 14 | **The RSS and time measurement may not reach the phases where the priced bound binds.**<br>• S3's E_mov,max (9.02 GiB) binds at W3, publication with the staged copy (ADD §1), with W4 next. The measured inputs may all fall back before staging. The c = 1 cases are W2 (Preparation) and W2b's replacement ("Candidate, Native or a successor"); the c = 3 input accepts "Preparation or Candidate".<br>• RSS_TIME.md would then set a W1- or W2-phase peak beside the W3-priced figure, and could understate the peak the owner will judge the 16 GB floor on.<br>• RSS and footprint are not like-for-like with E_mov,max (requested plus moving heap). The challenge's counting allocator is. But its `CAP_BYTES` (6 GiB, abort) is below the new E_mov,max. Its bound is chosen by `successor().is_some()`, so a permitted fallback is held to the W1 phase.<br>• Every witness test runs both modes in one process (`witness_*`; QUAL §4's logs). | • (a) Record, for each run, the furthest W1 phase reached and the outcome.<br>• (b) Have SW prefer a cap-maximal variant that publishes a successor, at c = 1 and c = 3. If none exists, say plainly that W3–W5 are unmeasured at the caps.<br>• (c) Record the counting-allocator peak for the same inputs. Raise `CAP_BYTES` above the regenerated E_mov,max, and choose the bound by the phase reached.<br>• (d) Give each mode its own test entry point, one process per mode.<br>• (e) State that the host has 128 GB and runs under no memory pressure, so 16 GB behaviour (compression, swap) is inferred, not observed. |
| **SF-6** | SHOULD-FIX | §6 gate set, §3.10–§3.11 | **The milestone's src-tauri gate is missing.**<br>• #1082 ran U9's G7, the src-tauri suite: M 116, C 116, per-test outcomes identical (RR "U9 gates G7 and G8 pass…"; CR §7).<br>• `apps/desktop/src-tauri` builds PP, and it is not among DEC-025's 40 manifests (T6S's DEC-025 `compare.txt` and `suites_candidate.log`).<br>• B1 changes PP `src` on the D1 call graph. | Add the src-tauri suite, base against candidate with identical per-test outcomes, to SG or to §3.11 (ROOT). |
| **SF-7** | SHOULD-FIX | §5, §6 gate item 2, decision 16 | **Gate item 2 is weakened.**<br>• RR "T3's gate set…", item 2, requires "a fresh independent complete-diff review, with the same reviewer confirming each repair". §6 rewrites it as "fresh independent reviews (§5)… and the PR-head ledger".<br>• The plan's analogy ("as RV97 did for U8 and RV101 for T6S") fails: each of those was the whole PR's reviewer. B1 has three disjoint slice reviewers, so nobody reviews the whole diff.<br>• The closer precedent is #1082, a D1 call-graph PR of similar scale. A fresh complete-diff reviewer (RV95) found S-1 and S-2 there. | • Make a fresh complete-diff reviewer the default for PR-B1 (about +5–8 h), with RV-P's ledger as its input.<br>• Otherwise, ROOT rules an explicit variance from item 2, with its reason. |
| N-1 | NOTE | finding 6, §2.2 | **Finding 6 is confirmed.** RS's G5 ordinary block requires `initial.kind == report`, `outcome == checks_passed` and `w2 == not_triggered` for `not_required`. The private driver still runs precommit (`permitted_work` → `retained_w1` → `validate`). So before SR-RS, W-C2 falls back at precommit in both modes, with case B's G5 failing before dense case A's G8. | Say how SP observes the per-case outcomes on the driver before SR-RS: for example, assert the precommit fallback and use a `cfg(test)` dump, as I68 and I81 did. |
| N-2 | NOTE | §2.2 T-8, T-11 | **Kernel owners are batch ordinals.** The kernel names them `NativeOwner::Case(used_cases + position)` (`origins.rs` `solve_cases`). Today's serializer emits that directly as the receipt case index (`invocation_arrays`' `owner_refs`; `one_run`'s check). In W-C2, A = {0, 2}, so the ordinals and the request indices differ. | Add to SP's acceptance and RV-P's mutants: `owner_refs` and the Run origin's `owner_ref` name the request index. |
| N-3 | NOTE | §2.1 (T-4) | **The verdict lookup should be by id.** Key the published verdict by `numerical_quality.cases[].basis_ref.ref_id`, as `ordinary_value` does, not by position. | Same as the finding. |
| N-4 | NOTE | §1 fence | **A fault hook is outside the fence.** SP's "one case's preparation fails beside a selected case" needs a hook aimed at one case. `fail_next_preparation` lives in `PP/retained_tests_hooks/grant2.rs`, outside the fence, and as written it would fail the first attempted case, W-C2's A. | Add `grant2.rs` to SP's fence. |
| N-5 | NOTE | coverage | **Multi-case coexistence has no committed test.** T-3 (c) over several cases (CR §5.2, "the complete invocation's exact-block no-attempt rule"; PLAN §2.1) is checked only at SG's gate. | Add a committed SP pin: exact-block selects one case of several; exact bytes, no notice, no reservation, `{runs: 1, complete_gates: 0}`. |
| N-6 | NOTE | §2.4 | **D38's audit is assigned to only one reader.** DESIGN §2's "B1's obligation" (list every check that assumes a prepared source has a Call or Run) covers all three readers. The plan gives the list only to SR-RS. | Give the PY and TS halves to SR-PY and SR-TS; RV-R checks all three. |
| N-7 | NOTE | §3.4, §2.0 | **The three-case cap-maximal witness should stress text and depth.** STUDY §6 item 3 specified W2's escaping and a depth-16 raw value. And because the private driver skips admission, the witness should assert that its input is inside D1. My census approximation (`evidence/raw_census_s3.*`) of a 3 × 128-load `cap_maximal`: 6,384 of 16,384 raw values, and 49,428 (57,043 escaped) of 65,536 raw string bytes. That is close enough that longer provenance would leave the domain unnoticed on the driver path. | Add both stresses and the admission assertion to the witness. |
| N-8 | NOTE | §1, §3.3 | **The registration also re-pins tests.** It changes the law tests' `const M` and threshold assertion, the pinned phase record (`retained_memory_law_tests.rs`), and the challenge's `MAX_PHASE_BYTES` and `W1_PHASE_BYTES`. "The registration diff changes only three things" undercounts. | Assign those lines to SQ explicitly. |
| N-9 | NOTE | §4 | **Ordering within phases 3 and 4.**<br>• Phase 4 shows lane 1's "Repairs" beside lane 2's "G5 final". The repairs and their confirmations must land before G5 final.<br>• SC runs beside RV-P round 2. A round-2 repair that changes W-C2's bytes forces 07n's copies and reseal to be redone.<br>• The SR chain (B6's PR gates → SR-RS → RV-R → repairs) is near-critical, not off the critical path. | Order them in §4. |
| N-10 | NOTE | §10 | **The estimates are not calibrated against actuals.** The arithmetic is consistent (67–102 h and 26–41 h). By RR timestamps, U8 (planned 1.5–2.5 sessions) and T6S (planned 3–5 working days) each went from dispatch to merge in about a day of wall time. But SQ's closest precedent, U4's G5–G7, took several review rounds (RV83 FAIL; RV87 NOT CONFIRMED; CR §3, U4). RV-Q's 8–13 h has no repair-round allowance, unlike RV-P's. | Add a repair-round allowance for RV-Q. |
| N-11 | NOTE | finding 8, decision 21 | **"A retained combination is priced like [a case]" is unpriced.** A combination has no ordinary per-case solve owners. The conclusion is still likely: its rows alone grow TAV_W and T16 by about 1.3 and 1.2 GB per case's rows at D1's caps (ADD §1; STUDY §3.3), already beyond the 1,848,686,021 B margin at 12 GiB. | Record it as expected, and price it in B2's study. |
| N-12 | NOTE | §9 | **A citation error.** The contingency's "trimmed single tier at C = 3 (… ADD §2)" cites trims priced at C = 4. The C = 3 trims are in STUDY §3.2 (for example `t_c3_m12`). | Correct the citation. |
| N-13 | NOTE | §2.3 | **SA's changes have no required mutants.** The milestone's admission changes each had a set (QUAL §9: part 1's 84; G6's 19, including five on the G-C attempt fact). STUDY §6 item 5's EnvelopeResults ≤ c·P_final is also not in SA's law list. | Require mutants killed by assertions for D1.4's bound, the per-case load rows, G-B's total and T-3 (e)'s requested count. Add the EnvelopeResults bound to SA's law tests. |
| N-14 | NOTE | §6, decision 15 | **B6's change set is wider than the plan describes.** B6 is now at `a79dbd2e4a`, with 07m unchanged. Against `bfb26596bf` it also changes `compatibility.py`, `RE/tests/retained_precision_carriers.rs`, `test_retained_precision_carriers.py`, `retainedPrecisionIntegration.test.tsx` and `fixtures/results/retained_precision_carrier_cases.json`. None is D1 crate `src`, so decision 15's premise holds. B6's brief said its fixtures would be unchanged; that is for B6's reviewer. | B6's package lists these files. |
| N-15 | NOTE | §4 ruling points | **No point to relay the measurement to the owner.** The owner asked for the measured peak and time to judge the 16 GB floor, and M is built into the product. | Add a point (at R6b or R7) where ROOT reads RSS_TIME.md against the 32 GB target and 16 GB floor before PR-B1 merges, and decides whether the owner sees it then. |
| N-16 | NOTE | §2.0 item 2, §2.2 | **Batch outcomes are untested.** It is not established that a case's native outcome inside one `CaseBatchCall`, with shared group builds and one meter, equals its one-case outcome (PROBE §6). | Treat a difference from PROBE's one-case outcomes as a finding for ROOT, recorded with its cause. Also note SQ's G5 must author TEXT loop rules for B1's new and changed loops (`g7_linemap.py` only re-keys existing rules); I82's 22 rebinds serve as the checklist, and RV-Q compares real multiplicities with I82's emulated ones. |

## 1. Coverage

I checked every obligation against the plan's slices.

**DESIGN T-1 to T-13:**
- T-1: SA.
- T-2 and T-5 to T-13: SP.
- T-3's order: SP. Fact (e): SA. Coexistence (c) over many cases: SG only (N-5).
- T-4 with decision 21 and the seedless case: ST.

**DESIGN §1.3 and §1.4:**
- W-C1, W6 and W2b: ST.
- W2 stays in A: ST.
- The `NoTriggeredCase` pins: ST, by ruling 4.
- W-C2: SP and SC.

**DESIGN §2 (R-D38 with (4b), m1–m8, the synthetic pin):**
- SR and SC.
- The three-reader audit is given to RS alone (N-6).

**DESIGN §3:**
- Text B's P1–P4, with P5 deferred; the G8 order and codes; G5's `not_required` rule: SR.
- The corpus: SC.

**DESIGN §6:**
- The cap rows and G-B/G-C scopes: SA.
- The re-pricing: SQ.
- Step 7: §3.

**DESIGN §7's B1 rows:**
- N-1: SA. N-4: SP. N-5: SP's hook test.
- R2-N-2 and RV95 N-5's direct test (with NT-1 and A2-N3): SR-RS.
- I77's renames: ST.
- T6S consistency: §3.12.

**I81's rulings 1–4:** SW, ST and SQ.

**I82's S3:**
- The single tier at C = 3, with L stated: SA.
- The M rule: §3.2.
- The 12 GiB contingency: §9.
- The measurement obligation: §3.6.

**STUDY §6:** G5, G6, the S1 witnesses and the challenge are in §3. The gates are in SA (N-13 for EnvelopeResults).

**PLAN §2.1 and CR §5.2:**
- The transaction: SP. Multi-case: SA.
- The exact-block no-attempt rule: SG (N-5).
- D38's pin: SC. W-C2: SP and SC.
- Re-qualification: §3.

**Missing or incomplete:**
- One out-of-domain oracle (SF-2).
- The src-tauri gate (SF-6).
- The PY and TS halves of D38's audit (N-6).
- A committed multi-case coexistence pin (N-5).

**Assigned twice, or across lanes:**
- The law tests: SA's file, but their M and record pins change at SQ's registration (N-8).
- The witness tests: ST, then SQ. These are sequential, which is fine.
- The capture fields that G-B and G-C read: SA's gates, but SP's struct (SF-4).

## 2. Re-qualification

Against QUAL §11 and PLAN §2.1, §3 is sufficient on:
- G5, with TEXT complete, the identifier audit and its controls, the forms gate, the per-mode law print, and a c = 1 control;
- M's rule, which matches RR "I82's addendum…" word for word, ruled in two steps (R6a, R6b);
- G6: the `admission_bound` boundaries, the seven-phase pure maximum, and the registration diff applied in scratch;
- the S1 witnesses at R/16, with headroom at R/64, including the re-based and new ones and a re-derived deepest chain;
- the challenge;
- RV87's by-type sweep, as QUAL §11 allows;
- Pass A (SQ) and Pass B (SB, confirmed by RV-Q);
- T9 and both-entry part 1, with part 2 conditional;
- the registered and Stale sweep (SG);
- the full suite before the freeze;
- the exact-head DEC-025.

**Its claims about the milestone are true where I checked them:**
- QUAL §4's 40 frames, "frames do not scale with counts";
- W2b's 43 s (debug) for both modes (`witnesses.test.txt`: 43.31 s; 1.39 s in release);
- the milestone's peaks 3,541,898 and 2,252,863 B;
- the c = 1 record, 3,575,778,286 and 3,595,488,734 B;
- T6S's "Pass B not applicable" precedent for B6.

**Nothing is skipped by an unsupported argument, except:**
- the src-tauri suite, which is silently dropped (SF-6);
- the complete-diff review, which is replaced on a false analogy (SF-7).

Part 2 of both-entry is made conditional on Pass B's classification. That is a ruled condition, not a skip, and I agree with it.

## 3. Workability

**Write sets:** disjoint by file within each phase, except as SF-1 describes: one shared worktree and one build closure. Three gaps in the fence: `s11f_site_test.rs` and PP's `tests/retained_precision_admission.rs` (SF-3), and `retained_tests_hooks/grant2.rs` (N-4).

**Dependencies:** sound, with two gaps:
- SA's c = 3 gate values depend on SQ (SF-4);
- SC depends on RV-P round 2 settling W-C2's bytes (N-9).

**The corpus's single writer holds.**
- SC (I-PY) is B1's only CORPUS writer.
- B6's 07m lands first. Its sha256 `c21112fd…6807` is unchanged at B6's newer head.
- RV-R reseals in its own scratch.

**Ruling points:** R1 and R3 to R8 sit where they are needed. Add the relay of the measurement to the owner (N-15). R2's number is unused; that is cosmetic.

**Estimates:** honest as reading estimates, and arithmetically consistent. They are not calibrated (N-10).

## 4. The findings

| Plan finding | Verdict | Evidence |
|---|---|---|
| W-C2 cannot be pinned before SR-RS | **True** | RS's G5 `not_required` conjuncts (`retained_precision.rs`, G5 ordinary block). Precommit runs on every path (N-1) |
| The re-pin cascade on 07m is empty | **True, statically** | The plan's `cascade_static.py` reruns byte-identically on 07l and 07m (`evidence/checks.txt` §2). RS's G5 change only relaxes RS, and PY and TS already agree on every 07m entry. 07m's one `not_required` must-pass is sparse, with one mode row for that case. SR's mechanical census remains the check, with R5 |
| The milestone is now in the domain, so the six out-of-domain oracles must be re-based | **True in substance; the count is seven** (SF-2) | `u3g2_no_permit_path_runs_once_without_a_copy` is missing |
| B2: a D1-cap combination does not fit beside three cases within 12 GiB | **Likely; one premise is unpriced** (N-11) | ADD §1's margin is 1,848,686,021 B; per-case increments are 3.0–3.3 GB (STUDY §3.3) |

**Finding 1 (the producer is single-case throughout) is also confirmed in code:**
- `ProductCapture`'s single `source` and `case_id`;
- the `load_cases.len() != 1` refusals in `capture_case_source`, `prepared_case_seen` and `prepared_case_source`;
- `solve_native`'s `for_calls(&[1], &[])`;
- `one_case` and `one_run`'s scope checks.

The kernel's `solve_cases` is already n-case.

## 5. The decisions

None of the 21 is on the work graph's owner-held list. M ≤ 12 GiB stays ROOT's (decision 20), and every decider is right.

| # | Verdict | Reason |
|---|---|---|
| 1 | AGREE, subject to SF-1 | The owners fit the files: I-P for the PP transaction; I-A for continuity with I65's profile; one owner per reader. The lane schedule must respect the implementer cap and isolate the Rust build closures |
| 2 | AGREE | It matches the S3 selection: one cap table, L stated, G-B and G-C at C = 3, pricing functions unchanged in form |
| 3 | AGREE, subject to SF-2 | Four cases is the right oracle. Add the fourth facade test, and use a literal or a combination oracle in the runner crate |
| 4 | AGREE | RR ruling 1 asks for the replacement "at B1's selected caps", and S3's caps are D1's, so SW can start at R1 |
| 5 | AGREE | It follows ruling 4; the new name says what the test pins |
| 6 | AGREE, subject to SF-4 | `retained_memory.rs` is the right place. The requested count's source must be agreed with SP |
| 7 | AGREE | Finding 6 forces this order |
| 8 | AGREE | One writer, D-U6-5 copies, and a labelled synthetic D38 base, as DESIGN §2 says |
| 9 | AGREE | A cheap precondition with a clear stop |
| 10 | AGREE | The milestone's Pass A / Pass B method, with SQ as the new reference |
| 11 | AGREE | QUAL §11 allows either; the sweep is cheaper now |
| 12 | AGREE, with SF-6 | Run T9 and part 1 because lib.rs hosts the observer sites. Part 2 waits on Pass B's classification. Add src-tauri |
| 13 | AGREE | The milestone's G8 method; an unexplained row stops the work |
| 14 | AGREE, subject to SF-5 | The method is right in kind: one process per run, `time -l`, under the lock, three repetitions, controls. It needs phase coverage and a like-for-like heap figure before its results are set beside E_mov,max |
| 15 | AGREE | B6's 11 files touch no D1 crate `src`, so B6 needs no Pass B, T9, both-entry gate or re-qualification (the T6S precedent). It keeps NUM to one unmerged slice at a time, and PLAN decision 8's two re-qualification runs |
| 16 | **DISAGREE** | Gate item 2 requires a fresh complete-diff review. RV-P has reviewed only ST and SP, and the RV97/RV101 analogy does not hold (SF-7). The fresh reviewer should be the default |
| 17 | AGREE | The profile is stale on the branch until SQ, so nothing should leave it |
| 18 | AGREE | It gives early warning of a memory overrun, at low cost |
| 19 | AGREE | It lowers the c² terms. RV-P checks that `bind_observations` keeps its envelope-binding checks when it binds by index |
| 20 | AGREE | It matches RR word for word. ROOT decides up to 12 GiB; above that, the owner |
| 21 | AGREE, with N-11 | Record it as an expectation for B2's study, not as a priced fact |

## 6. PR-B1's packaging

**B6 first, as its own PR, is right** (decision 15; N-14 lists its actual files). Its gate set is the product set without item 7. The plan's list of B6's gates omits the compact cut, source equality, citations and the package (gate item 1). Those apply too.

**PR-B1's gate set is complete** once two changes are made:
1. gate item 2 is restored as a fresh complete-diff review (SF-7);
2. the src-tauri suite is added (SF-6).

The rest of items 1 to 7 is present.

## 7. The measurement method (§3.6)

**What it gets right.** On this host and under the lock, it measures peak RSS, peak footprint and real, user and sys time:
- one process per run, running the binary directly, not through cargo;
- through `/usr/bin/time -l`;
- for cap-maximal c = 1 and c = 3 inputs in both modes, in the dev/test build (Direct, registered) and the release build (private driver);
- with a process-floor control and a value-route control, and three repetitions;
- with honest non-claims about debug times and macOS RSS.

**What it does not ensure** (SF-5):
- that any measured cap-maximal run reaches W3–W5, where the priced maximum binds;
- a figure like-for-like with E_mov,max. The challenge's counting allocator gives one, once its 6 GiB abort cap is raised.

With SF-5's remedies, the method gives what the owner asked for: the measured peak resident memory and the run time of cap-maximal W1 invocations in both modes, so the 16 GB floor can be judged.

## For ROOT to rule on at R1

1. **Amend the lanes** (SF-1): at most three implementers, and isolated worktrees for the concurrent Rust lanes.
2. **PR-B1's gates:** the complete-diff reviewer as the default (SF-7, decision 16), and the src-tauri suite (SF-6).
3. **The measurement** (SF-5): phase coverage, a publishing variant if SW finds one, the counting-allocator peak, and per-mode processes.
4. **The SA–SP interface** (SF-4), the seventh oracle (SF-2), and the fence additions (SF-3, N-4).

No owner decision is needed.

## What I read (sha256)

| Input | sha256 |
|---|---|
| The brief | `e0095c32…0fd05` |
| PLAN.md (I84) and its `_run_records/` | `7f9699f3…4bb5c` (7/7 OK) |
| `R/BRIEFS/B1_PLAN.md` | `d9b8bf0d…6cf0` |
| DESIGN_v2 | `5933b90b…1114` |
| PROBE | `3e32726d…392c` |
| STUDY | `d8b18220…7188` |
| ADDENDUM_01 | `7c155ceb…0ce2` |
| I61 PLAN | `f274a614…3def` |
| CR | `f4207994…d476` |
| QUAL | `8edbf4b4…2c29` |
| B6's brief | `a3634160…3777` |
| I74's PLAN (§4, §7) | `0350c918…2ed9` |
| RR, at NUM `37481d0f40` | `9ce18445…dcc32` |
| The work graph, at NUM `37481d0f40` | `95c06d50…cfdb` |

**From RR, read in full:**
- "Owner direction: proportionate CI…";
- "T3's gate set and Git rules…";
- RR:12796–12940 (U8's review, Pass B, sequencing and operating adjustments);
- "U9 gates G7 and G8 pass…";
- every section from "Owner decision: ROOT may raise M up to 6.0 GiB…" to "I84's B1 plan returned…".

**The work graph's T3 section, with its owner-held list:**
- dense and lane ceilings;
- PHYS-R4;
- observation framing;
- KF3 and KF2;
- the formal supported-machine statement, or M above 12 GiB;
- public meaning;
- the native-app witnesses;
- B8's R-2.

**Code:** `evidence/checks.txt` §4 lists the symbols and blobs read.

## Records

- `REVIEW.md` (this file);
- `evidence/checks.txt`: commands, hashes, and the code read;
- `evidence/raw_census_s3.py`, with its output `evidence/raw_census_s3.out.txt`;
- `SHA256SUMS`.

Placeholder paths only. Scratch, including the extracted corpus copies, stays in `WT/scratch/rv107_b1_plan/` and is safe to delete.
