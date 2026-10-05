# I63 return: Rust reader, review repair wave 07, phase 1

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session, under `BRIEFS/I62_I64_REVIEW_REPAIR_07.md` ("Phase 1" and "I63"). The basis is the ruling "Reader review RV78–RV81: consolidated ruling and the repair wave" (D1–D15; NUM `d566e487f4`). I63 had no descendants.

- **Run:** first tool call 2026-10-03T23:24:51Z; freeze about 23:41Z, inside the 90-minute box. The checkpoint-A changes and D16 (NUM `5030826e1b`) arrived mid-run and are applied. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Toolchain:** the default, with **no `DEVELOPER_DIR`**, per this grant.
- **Basis files:** READER at `6b607fd01f`. The shared files are unchanged: corpus `d02701ed6a` (06d) and schema `f943ebd351`.
- **Paths** use the brief's placeholders.
- **Status:** phase 1 complete. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`6b607fd01f`) | After |
|---|---|---|
| src/retained_precision.rs | bd20dd9a8f88… | b3cbb8afd17850bba1af1149603e8f77a1b1f889ca6912ff72a34bac27b61c0e (163517 B) |
| tests/retained_precision_contract.rs | 5cbb6ba4ea47… | 79836bb241a0f56da2419bffce13f4901d0d0212adc000bbb891ce19040fe3ce (88734 B) |
| src/lib.rs | 375b073135… | unchanged |

## Decisions: status and pinning test

**Status values:**
- **new:** implemented in this grant.
- **existing:** the reader already complied; it is now pinned by a test.
- **waiting:** waits on checkpoint A.

**Discrimination:** "probe" means the test was run against the pre-repair source plus only the test hooks (run5), and failed there as expected. The source was then hash-restored.

| Decision | Status | Pinned by |
|---|---|---|
| D1 member ids `0..len−1`; prepared and new as prefixes (RV80-S2, RV78 R2b) | new (G3 COVERAGE; was unique ids) | `d1_g3_coverage_relations`: "unsourced old id 1" (probe: G8 before), "prepared id 1" |
| D1 sourced complete old = the source's member map | existing (G3) | `d1…`: "sourced old short of source" |
| D1 unsourced complete old: the member count of every CaseSource (none: G8). **Checkpoint A withdrew the non-empty clause, so emptiness alone is not rejected** | new (G3, count only) | `d1…`: "old longer than the model" (probe: G8 before); an empty list against a one-member CaseSource fails by count |
| D1 `captured_prefix`: members at G3; null source/run and unavailable result at G5 PRODUCT_ATTEMPT | new (split) | `d1…`: `prefix_captured` plus a `run_ref`, now G5 PRODUCT_ATTEMPT; shared `prefix_captured_with_members` stays G3 |
| D1 run id = execution-order position; execution order a bijection | existing (G3), now keyed by the owning case, not origin | `d1…`: "execution order swapped", "run id not its position" (RV78 R1a/R1b; kills M07) |
| D1 Run origin owner in G5 class 1 | new (moved from G3) | `d1…`: "run origin owner moved" (probe: G3 before) |
| D2 the G0 union | new | `d2_g0_union`, nine relations (probe: G1 or G7 before for eight). A non-union shape defect stays G1 |
| D3 class order (native, then ordinary and C3 references, then typed, then C3 work) | existing | 06c/06d order pins; `d6…` ordinary-before-work (RV78 T4d) |
| D3 class-1 convention: native WORK deferred to the end of class 1 | new; I63 confirms that all 178 mutations and 18 must-pass entries keep their outcomes | `d3_native_attempt_before_work` (probe: WORK before); WORK alone still reports WORK |
| D16 dangling references take their check's code: in class 1 ATTEMPT, but a build reference is WORK (deferred); C3 references PRODUCT_ATTEMPT; ordinary references ATTEMPT | new (build reference deferred as WORK); the other references were already on their checks' codes | `d3…`: a dangling build reference alone gives WORK, and with an ATTEMPT defect gives ATTEMPT; a dangling group call gives ATTEMPT over WORK |
| D4a every sourced attempt's `source.preparation.attempt_ref` is the attempt | existing | `d4_association_relations`: R4, preparation null (kills M15) |
| D4b attempt basis = the ordinary attempt's | existing | `d4…`: R5 (kills M14) |
| D4c a `prepared_product_failure` case has its own attempt, equal to the cause's (RV81-B2) | new (class-2 case pass) | `d4…`: "cause without its own attempt" (probe: admitted before) |
| D4d reason table: `native` needs the case's nonselected Run, with `run_ref` = its id; `preparation` needs a null Run and a failed preparation; `capture` by Run presence and terminal (S06 §1) | new `native` arm; the rest existing | `d4…`: R6a native+selected, PR5 foreign `run_ref` (probe: admitted before; kills M13) |
| D4e `run_ref` null iff no native call | existing | `d4…`: "run_ref without a native call" |
| D5a candidate record: null verification, no verification shared build, `verification_lme` 0 | new (`verification_lme`) | `d5_native_record_relations`: T3, verification-pass work (probe: WORK before) |
| D5b an escalating failed verification is a solve failure: no report and no verification-pass work | new | `d5…`: T1 (probe: WORK before) |
| D5c `rejected(verification_failed)` requires the failed phase | new (explicit; was implied by the replay) | `d5…`: "verification_failed with a completed phase" |
| D5d a stop-rule reason's quantity names a layout row of the Run's source with the same body and kind | new | `d5…`: T2 (probe: admitted before) |
| D5e group call exists; its sources are unique and listed in that call | existing | `d5…`: R8 |
| D6a (checkpoint A) the untyped `diagnostic_refs` are unique and resolve; a listed diagnostic need not name the case. Typed references stay strict (O2) | new (the list no longer requires the case) | `d6_d7…`: another scope's diagnostic is admitted; duplicate (T4a) and dangling (T4b) give G5 ATTEMPT |
| D6b (checkpoint A) a selected case's `solve_quality` is `sensitive`, `unresolved` or `failed` | new | `d6_d7…`: `checks_passed` (RV78 T4e; probe: admitted before) and `not_assessed` give G5 ATTEMPT |
| D6c published W2: nonzero `b`, and the trigger matches the initial failure | existing | `d6_d7…`: a Formation-preserving W2 is admitted with b = 1 and rejected with b = 0; shared `w2_published_without_initial_failure` |
| D6d legacy `work_ref` resolves, at ATTEMPT | existing | `d6_d7…`: a dangling `work_ref` gives G5 ATTEMPT |
| D7 a SELECTED/UNAVAILABLE diagnostic names exactly one requested case | existing | `d6_d7…`: `affected_refs` ["case:absent"] gives G4 |
| D8 accounting class (R1′–R4) and D9 | phase 2, with snapshot 07 (per ROOT) | R1–R3 (06d) unchanged |
| D9, D10, D11 corpus, D12 | not Rust phase-1 items | D11 harness part done: `apply_entry` rejects any `rehash` other than "all", and `edit` removes array elements (RV78-N5) |
| D13 theta = +0 on a no-data body | existing | `d13_reader_local_pins_and_mutant_kills` |
| D13 the Ceiling after a p128 verification-solve failure | existing | `d13…` (V base: v256 escalating, p512 rejected, v1024 solved → Ceiling; refused → ATTEMPT) |
| D13 R3 with `both` | existing | `d13…` (`reader_logic::accounting`) |
| D13 the 2^-988 switch on both sides | existing | `d13_absolute_bound_small_scale_switch`: six vectors from an exact rational oracle |
| D13 the strict bracket | existing | the shared 06c bracket mutations. RV78's equal-E variants are D11 corpus items |
| D13 surviving RV80 mutants | M07, M08, M12, M13, M14, M15, M16 and M18 now killed by tests | `d1…` (M07); `d13…` (M08 idle boundary via `schedule_in`, M12 F′ record bound, M16 Budget payload, M18 G7 detail); `d4…` (M13–M15). **Not killed:** M02 (the N17 scope rule needs a ≥20B base or a native-loop hook) and M06 (the relative-class boundary needs an equality row) |
| D14 test-only exports | existing `#[doc(hidden)]`, now documented as internal and not a public API | the `reader_logic` additions `schedule_in`, `accounting` and `g7_error` are test hooks |
| D15 stale comment (RV80-N6); the outcome file lists all 178 | done | `OUTCOMES_07P1.json` (178 in corpus order, plus 18 must-pass) |

**RV80-S1** is D4d, and **RV80-S2** is D1. **RV78 rows:** R-6a (D4d), T-1 (D5b), T-2 (D5d), and T-4e/T4d (ordinary before work) are pinned as above. RV78's T-4c content is D6b.

## Commands and results

Every run used the 06d command and environment variables, with the default toolchain and no `DEVELOPER_DIR`.

| Run | State | Result |
|---|---|---|
| run1 | baseline (06d reader, default toolchain) | 23 passed |
| run2 | D1, D2, D3, D4, D5, D6b, D14, D15 | 23 passed: **no 06d entry's outcome changed**, including under the D3 convention |
| run3, run4 | plus reader-local tests | 30/1 (a harness rehash panic on a missing attempt, fixed to skip it as G1 does), then 31 passed |
| run5 (probe) | the new tests against the pre-repair source plus hooks; source restored and hash-verified at `b9228c2401` | the discriminations listed above |
| run6 to run8 | 0..30 outcome slice; discriminating D6c pin | 32 passed |
| run9 | phase-1 bytes before checkpoint A | 32 passed |
| run11, run12 | checkpoint-A changes (D1 empty, D6a, D6b) | 32 passed |
| run14 (**final, full command**) | plus D16 (build reference WORK) | **32 passed, 0 failed** |
| run15 | `--nocapture snapshot_0 shared_must_pass` | 9 passed; 178 outcomes and 18 must-pass verdicts captured |

**06d still passes in full:** all 178 mutations match their expected first gate and code (per reader for G7), all 18 must-pass entries pass, and all 15 cases validate. No stop condition was met.

## Remaining known differences from the other readers

Python and TypeScript are being repaired at the same time, so I compared against the decisions, not their new heads. Their pre-repair differences are exactly the items the ruling assigns to them.

**Rust-side readings the other readers should match, or ROOT should confirm:**
1. **D2 `receipt_version`.** Rust requires the schema's `const 1` at G0, reading "v2" as the mp_v2 family; a v1 relabel is the policy check. I cite only the schema here.
2. **D2 with no receipt.** Rust checks the G0 body fields even when `retained_precision`/`body` is absent, so absent fails G0. Python's pre-repair G0 checked them only when the body is an object.
3. **D3, deferred computation faults.** Rust also defers WORK that arises from computation: an overflowing checked sum, a non-object stage map, or a negative fragment difference. It continues with a saturated or skipped value, so an ATTEMPT defect later in class 1 still wins. A dangling build reference is likewise deferred WORK, per D16. Whether Python and TypeScript defer these the same way is unverified.
4. **D4c placement.** Rust checks it in a case pass at the start of class 2. The code is the same as the per-attempt checks, so first codes cannot differ.

**Phase 2 (with snapshot 07):** D8 (R1′–R4 and the kernel-scope rule) and D9. The known D1 limit is ruled: without an invocation and without any CaseSource, a wrongly sized unsourced complete list is admitted, but can never be eligible.

**Not killed:** RV80 mutants M02 and M06 (no base, as above).

## Host

The Xcode licence is accepted, and the default toolchain linked normally.

## Files read (sha256)

| sha256 | File |
|---|---|
| 0e7a2a0c930629f1a87ed2e8449b4807525d616844770785b5c4cec4f601962d | T3/ROOT_RULINGS_V1.md at NUM `5030826e1b` (the ruling "Reader review RV78–RV81", D1–D15; "Checkpoint A: rulings…"; D16) |
| 4e9e4c0ddebe4f57c8e88c00c2ee89598be6f08957f533dba6d5fb29b0f00565 | R/BRIEFS/I62_I64_REVIEW_REPAIR_07.md |
| 9256831ecbacd64b6ef7be70728516d67b3eac61f8862913979c7950123d7ab0 | R/REVIEW_RV80/reader_review_01/REVIEW.md |
| 731d683c320b47362a67c745c27a207cf06ddb45d16a066b423ea5164145e502 | R/REVIEW_RV78/reader_review_01/REVIEW.md (the Rust rows and the R-/T- tables) |
| d559ef26b8077e3c0589c33a64e4b961f2ead52a926b078422196d3e858aac97 | R/REVIEW_RV78/reader_review_01/PROBES.json |
| 031b2a150545df4d80da8d8078b956e98760e0df757a75f98bd39b914dec42ab | R/I52/reader_contract_seams_06/ADDENDUM.md (S06 §1 reason table) |
| (C2 hash in RV80's basis list) | R/I32/f2a_wire_c2/CONTRACT_DELTA.md (the initial/W2 lines, around C2:150–166) |
| 55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f | READER/P/core/analysis_runs/retained_precision.py (G0 only, read only) |

I did not read RV79 or RV81 in this grant.

## Bulk (WT/scratch/i63_review_repair_07/)

| sha256 | bytes | file |
|---|---|---|
| 36f9863887af2e954af826694f90bf4952721cc5b8a2e846b65f9ffaaa759ca2 | 26515 | I63_07P1_DELTA_src.diff |
| 109a1d6d910102c78f259692c4ab13445d6108f63bb46802fa8d0479bb71b2fb | 25462 | I63_07P1_DELTA_test.diff |
| 254501f605cb5c0324458a6436048f270962dd5ce0ec4b321be6ac01ca80d034 | 1926 | run1_baseline.log |
| da141da73857ac341504974d4748473f788df7b3bf12c578543c42443dfa6e0f | 2131 | run2.log |
| 05ae402c8aa81b5e5412b1b0c3baa761b5df89f44fa0a4c9e3a542e70ba20503 | 2861 | run3.log |
| 2fca58cc4587dc845c9ac98e168b9a862974fcaada3a772e8ac6f6d2b9109499 | 2463 | run4.log |
| 3814760d07a5052939313c26d51099206883781d37a8673714e163eb7b9b68d6 | 4660 | run5_probe_pre_repair.log |
| 1f59fb844fc907d8356a718023d5c5a015f5a7baf86c6921469e516b5b27f5b0 | 2505 | run6.log |
| cbea60ea25e8e4314663095f1d0cac5183b0d3cd0f5a2d8d4d7ca29a96d82748 | 1005 | run8_d6.log |
| 74680089df02b0d580ab7d3959793d0bb9e591ab0d615a62c28ad0a03f05334e | 2301 | run9.log |
| a0e4b82eba24e87a0d399cca4c032f941708d5664f505d1a6fa71d39930d8390 | 2505 | run11.log |
| aa0241ca323de4e1d99f9f922060ab96ed4b5875025fd024bdc1bcd3bf3dfaa3 | 2505 | run12.log |
| 1cc01ce7ecdb608923afd9162d71e6e8ad920ee33c76eb44de54f1c96c42a343 | 2505 | run14.log (final) |
| b5dedf857ec7fe566ec9285db086e5496f101d131ba07390cff1f40922fe1c49 | 46117 | run15_outcomes.log |
| bd20dd9a8f888f0a3e81c981db7c02c7cfd48901ddc1ef203914d00be8b19ed0 | 156406 | before/retained_precision.rs |
| 5cbb6ba4ea47add234b5bf827bfaf1390c4e953a38d73c8d61136a07c14eaf20 | 65226 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| b3cbb8afd17850bba1af1149603e8f77a1b1f889ca6912ff72a34bac27b61c0e | 163517 | final/retained_precision.rs |
| 79836bb241a0f56da2419bffce13f4901d0d0212adc000bbb891ce19040fe3ce | 88734 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
