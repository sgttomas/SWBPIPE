# I64 return: TypeScript reader G5 checklist audit and snapshot 06a

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This was its fourth grant, sent by ROOT's mid-run message in the same harness-native subagent session. It had no descendants.

- **Run:** 2026-10-03T21:26:44Z to the final checks at 21:35:46Z; return written about 21:40Z. Well inside the 90-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations, and Git reads used `GIT_OPTIONAL_LOCKS=0`. No install, build, Cargo, solver, native, UI or DEC-025 job.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's two Rust files showed as modified in READER.
- **Paths** use the brief's placeholders. FK is P/core/solver/frame_kernel/src/structural/retained, and PP is P/core/product_physics/src.

## Changed files (READER at e7dac8d4d9, inside the fence)

| File | Before | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | cbc71d9b6a543f329f4daa2dddf9d12e2a26fa1bbf2e65badff567bed117df5b (93026 B) | 66fae3b25a5dcafb9b5e69243998565b7de676fdf6efad590873f71e1055fb8c (100659 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | c643e8734da08a6157b7752ad97e68b49556ec2b67b1ddedb8dc5daf29bd2395 (21063 B) | fd6c1c4f8bffa25c011185e71866e37265021c16efb3556a51ad614c1b60a0c7 (26947 B) |

**Shared files, verified at the start and unchanged at the end:**
- corpus `58562c88dc492b7e71f663332208045484b04a0cdc125574837c5d08a62ddfe8`;
- schema `f943ebd351…`;
- definition `3e0779a45a…`;
- table `c74742ce6a…`.

## Reader changes

Eligibility stays held (`SUMMARY_COVERAGE_COMPLETE = false`).

- **G2 → G5, P5 (conversions).** G2 now checks only the conversion encodings: finite bits, and relative precision that is non-negative and not −0. The kind/bits consistency moved to G5 PRODUCT_ATTEMPT (`conversionKind`): Normal must be normal or ±0, and Subnormal must have nonzero subnormal bits. This applies to both the preparation-member conversions and the projection outcomes.
- **G3, P4 row index (ruling).** Every `row_index` must name a hull-projected row of its case, strictly ascending; `HULL_EXCLUDED` covers non-quantity rows, support norms and maxima. A valid sorted subset on a Ready case is still the G5 exact-set check.
- **G5 native (P1 class):**
  - N14: `corrections` ≤ 3.
  - N1/N10/N11: a pre-schedule run has no records, zero charges, and a non-selected terminal with a non-null reason.
  - N7: the terminal is selected if and only if the last logical attempt is accepted.
  - N13: a plain verification record is Verified only for the accepted candidate and otherwise Solved, and a reused verification belongs to a rejected candidate. Logical outcomes are limited to accepted, rejected and failed.
  - N9: WorkAccounting is accepted wherever the ladder ended at a stop (see divergence 1). I also fixed a draft defect: the inherited `terminalFor` added `prior: null`, which the wire `Unresolved.work_accounting` does not have.
  - N10 (adaptive.rs:4994–5015): an exhausted meter forces an idle, group-null run. An idle run carries either WorkAccounting (a meter fault, which takes precedence) or Budget(invocation) once exhausted.
  - C1 (`obtain`, 3984–4026): build state and reason must agree (WORK), and a non-success build must fail its requesting record with the same stop (ATTEMPT).
  - C5 (C2:143): within each call, groups partition the non-idle sources by stiffness, in first-seen order, and each run names its own group.
- **G5 ordinary (O5):** a source decline requires an unavailable case with no source or run.
- **G5 C3 association:**
  - P2: preparation is completed if and only if a source exists; observables and G5a are entered together (PP:3511–3537); observables entered implies the certificate ended.
  - P6: values failed implies `separate_failure`; certificate entered, maxima failed or aliases entered implies `merged`; otherwise, before projection completes, `not_entered`.
- **G5 order (C3:304):** native schedule, then ordinary, then association, then the typed pass (P9), then the deferred C3 work equations.
  - P9 now runs after every attempt's association. Each failed check must carry its own wrapper, and an unavailable result's error must match its first failed stage (Python's table).
- **G7 (ruling):** the error code is the bare base code (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`). `RetainedPrecisionError.detail` carries the validator's text, for example "maximum outside or off its enclosure".
- **G8, P7 (F1:101–106):** every retained old operational tuple, whether or not a source was built, binds its end positions and selected E/G (`inputs[0..8]`) to the invocation.
- **Exports for unit tests only:** `nativeSchedule` and `ordinaryAttempts`.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_00 (baseline: the 05c reader on 06a) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 227 tests: 220 passed, 7 failed. The failures were `failed_build_reason_mismatch`, `corrections_above_three`, both conversion-kind mutations, `g7_maximum_off_enclosure` (code included the detail), `row_index_foreign_support_norm` (G5 instead of G3) and `prefix_old_inputs_unbound`. |
| vitest_01 / vitest_02 / vitest_03 | same, during the work | 226/227, then 227/227, then 232/232 (after adding 5 reader-logic tests) |
| tsc_04 | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| probe_05 | a temporary probe block recording the raising line for every mutation; removed, with the test file restored byte-identically | 151 of 151 |
| **vitest_06 (final)** | same as vitest_00 | **232 passed, 0 failed** |
| **tsc_07 (final)** | same as tsc_04 | **exit 0** |

All four shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Against the bar (06a)

- **All 151 mutations** produce their expected first gate and code. `MUTATION_OUTCOMES.json` gives the checklist ID and raising line for each of the 30 new ones.
- **All 16 must-pass entries pass.**
- **All 12 cases validate.**
- **All earlier tests pass.**
- **tsc** exits 0.
- **New TypeScript reader-logic tests,** mirroring Python's:
  - N10/N1: idle entry;
  - N6: a rejected p128 candidate cannot end in the Ceiling;
  - N5: a verification-pass failure is terminal; the terminal must be exact; an escalating pass stop is admitted only as WorkAccounting;
  - N8: the Ceiling, with WorkAccounting refused there;
  - O5: source-decline ownership and status.

## Checklist status (TypeScript, 06a)

| ID | TypeScript status | Evidence |
|---|---|---|
| N1 | checked | `records.length ≤ 4`, record indices 0..n−1 and increasing precision; attempts ≤ 3; records present if and only if attempts are; pre-schedule shape. Test: N10/N1 idle |
| N2 | checked | `schedule_fresh_first_p256` |
| N3 | checked | `skip_after_failed_candidate_reused`, `_wrong_slot`; base `candidate_failure_skip_synthetic` |
| N4 | checked | `failed_verification_reused_as_candidate`, `_one_slot`; base `verification_failure_skip_synthetic` |
| N5 | checked (logic) | reader-logic test "N5". The TypeScript rule is stricter than Python's: see divergence 2 |
| N6 | checked | p512 ladder (05b); reader-logic test "N6" |
| N7 | checked | every selected base; the selected-if-and-only-if-accepted check |
| N8 | checked (logic) | reader-logic test "N8". The shared Ceiling base is **deferred** (needs a producer-solved witness) |
| N9 | checked: exact `terminal(stop)` translation, or WorkAccounting at a stop-ending point | no shared row yet (C2-2 item 16). The fault itself is attested. See divergence 1 |
| N10 | checked (logic) | reader-logic test "N10/N1"; the exhausted-implies-idle rule. The shared base is **deferred** (≥60B of work). See divergence 3 |
| N11 | implemented: a refused group means no attempts and a refused terminal with the group's reason | no native-faithful base yet |
| N12 | checked by the G1 schema; the replay reads the Reason structure | — |
| N13 | checked | bases K and V; the new verification-outcome and logical-outcome rules |
| N14 | checked | `corrections_above_three` |
| N15 | checked (existing) | `certified_bound_unbound_drop_existing_g5` |
| N16 | checked (existing) | the `physical_*` mutations |
| N17 | partly: fragments and sums, chaining, selected final guard, Budget scope rule | no shared base (**deferred**, ≥20B/60B). See divergence 4 |
| C1 | checked | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt`; base S |
| C2 | checked (existing) | all bases |
| C3 | checked (cache_after equals the derived inventory) | `failed_slot_not_cached` |
| C4 | checked (existing) | `native_work`; call chaining |
| C5 | checked (first-seen stiffness partition; run group) | `group_sources_out_of_order` |
| C6 | checked (existing, G3 `execution_order`) | all bases |
| O1 | checked (existing, G3) | — |
| O2 | checked: diagnostic references resolve, and an attempted report is required for selected or not_required cases | `formation_d5_dangling_diagnostic` |
| O3 | checked | `w2_published_without_initial_failure` |
| O4 | checked | `legacy_source_dangling_diagnostic` |
| O5 | checked (logic) | reader-logic test "O5". The shared base is **deferred** (no natural trigger) |
| P1 | checked (existing) | `rebind_source_run_only` |
| P2 | checked | `stage_entered_after_failure`, `certificate_stage_check_disagree` |
| P3 | checked (existing) | `lane_k_failed_with_coverage` |
| P4 | checked per the ruling (G3 / G5) | `row_index_*` (4) |
| P5 | checked at G5, including the preparation conversions | `conversion_*` (5). Positive subnormal/underflow rows are **deferred** |
| P6 | checked | `maxima_abandoned_separate_failure` |
| P7 | checked, including G8 old-tuple binding | the 05a prefix bases; `prefix_old_inputs_unbound` |
| P8 | checked (existing) | the reason/phase table on bases F and P |
| P9 | checked (separate typed pass) | `certificate_check_wrong_wrapper` |
| P10 | checked | the 05c coverage set |
| P11 | checked | `native_stage_disagrees_with_run` |
| W1 | checked (existing: Ready exact) | `product_work_only` |
| W2 | checked (existing) | lane and conversion counts |
| W3 | checked (existing) | — |
| W4 | not publicly checkable (the cumulative adapter prefix is attested) | — |
| G7 ruling | applied (bare code, detail carried separately) | `g7_maximum_off_enclosure` |

## Where the TypeScript reading of contract and native code differs from Python (for ROOT)

None of these changes any 06a outcome. Each is unpinned by the corpus.

1. **N9: WorkAccounting after an escalating stop.**
   - **Native:** `finish_terminal` (adaptive.rs:4769–4800) returns Unresolved{WorkAccounting{fault}} at every stop-ending point when the work status is not exact. That includes a candidate or verification *solve* failure with an escalating stop, because the fault test precedes escalation (4545–4548 and 4581–4583).
   - **TypeScript** accepts WorkAccounting there, but never at the Ceiling (a loop exit).
   - **Python** requires the Ceiling whenever the last stop escalates, so it would reject this native receipt.
   - This bears on C2-2 item 16.
2. **N5/N9: terminal exactness.**
   - **TypeScript:** a non-Ceiling terminal must equal `terminal(stop)`, translated (or WorkAccounting). A verification-pass failure with an escalating stop is rejected unless the terminal is WorkAccounting, because native `terminal()` is unreachable for escalating stops (adaptive.rs:4349–4378).
   - **Python** accepts any non-selected terminal with a non-null reason once the ladder ended.
3. **N10: invocation entry.** A meter fault takes precedence over exhaustion (adaptive.rs:4996–5000).
   - **TypeScript** accepts an idle WorkAccounting run even when the meter is exhausted.
   - **Python** requires Budget(invocation) whenever `invocation_before ≥ limit`.
4. **N17: Budget scope.** The inherited TypeScript rule requires that an invocation-scope Budget terminal did not also exceed the case limit; the case scope wins. Python checks only the reported scope's own limit. I did not verify this against native code (it is not in `CaseBudget::retain`), so ROOT should route it with the C2-2 budget work.
5. **P5: conversion checks on preparation members.** TypeScript checks conversion kind/bits consistency on preparation-member conversions as well (C3:183–195). Python checks only the projection outcomes, plus the normal values of prepared members.
6. **P7/G8: old-tuple binding.** TypeScript binds `inputs[0..8]` of every retained old tuple; Python binds only those of sourceless attempts, because sourced tuples are bound elsewhere. The two are equivalent on the corpus.

**No expected outcome looks wrong.** I found no defect in 06a.

## Files read in this grant (sha256)

| sha256 | File |
|---|---|
| ee3cc5918cec2a789116702fd4bdbce78dd0a243dde388b21b2b6828954a7790 | R/I62/coverage_shared_python_01/READER_AUDIT_PLAN.md |
| 06419414e214e0675110b9ffd2d4370130991d83bf79a77d9b3e9a346ca15935 | R/I62/coverage_shared_python_01/RETURN_C2_1.md |
| 1d11fc08fc17d5bbc088e70a53e0fbf0702a9ff02f8e6c644d969efb520208d2 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06A.json (summary fields and new_mutations) |
| 5bb6357a3633d2693313ffc80396e4e4015a830800ba2320bc5456b8b28e1d14 | READER/P/core/analysis_runs/retained_precision.py (lines 278–750 and 1228–1252, plus the G3 row-index lines) |
| 514745e24569ee28585bc0047370c900786d4bd79cacc3a1546cf3071fb7f9d3 | READER/P/tests/test_retained_precision_contract.py (lines 1–40 and 300–345) |
| 6a2fc382bf8cae0502c41030da8ac9bc0cfe1f1b80b7aceac60ff7e40f345eda | NUM/FK/adaptive.rs (lines 3930–3985, 4349–4378, 4515–4810 and 4985–5030) |
| d07383fc026e61e494a2b0a307271eaf2eb0329c333da95533f4b39c5d52af1a | NUM/PP/retained_product.rs (lines 3465–3545) |

I also used the corpus at 58562c88dc and the fenced files at their e7dac8d4d9 bytes. I did not open the contract texts C1, C2, C3, F1 or S06–S08; they were relied on only through the plan's clause citations and native code.

## Bulk (WT/scratch/i64_reader_audit_06a/)

| sha256 | Bytes | File |
|---|---|---|
| cbc71d9b6a543f329f4daa2dddf9d12e2a26fa1bbf2e65badff567bed117df5b | 93026 | before/retainedPrecision.ts |
| c643e8734da08a6157b7752ad97e68b49556ec2b67b1ddedb8dc5daf29bd2395 | 21063 | before/retainedPrecision.test.ts |
| 66fae3b25a5dcafb9b5e69243998565b7de676fdf6efad590873f71e1055fb8c | 100659 | retainedPrecision.after.ts |
| fd6c1c4f8bffa25c011185e71866e37265021c16efb3556a51ad614c1b60a0c7 | 26947 | retainedPrecision.test.after.ts |
| d38ffa3212f88b3b89a8c46f4ddeaad0bba831d3b7d90c7ba33d3dc9c36051cc | 24856 | i64_06a_reader.diff |
| 96de7184dcdc08d8fcaaf08990661c8cc511c0de6b8bacec1bf7c41579f007ee | 6907 | i64_06a_test.diff |
| ab0ecfab54d244d6f33c730fab476e984273b132fdcc19a898afa111d79afbe6 | 21719 | probe_05.txt |
| 8bf7c4c68127ee7194e101ecc2c5a32894c0f4c27d67987c87c3c94946566438 | 338 | probe_05.log |
| 5849240c3d5fc3e30d9a06b3ef6262aaa2a190654120c7990c8679abb77f319c | 5729 | vitest_00_baseline.log |
| cbb157f91e1de4c9d90581f396fc9a476b933eee40f96aa27f5a830cbfc76736 | 1886 | vitest_01.log |
| d215085af3d51e33301be6f6e4408628022f77c9da7406f4dfd012c545d7d260 | 440 | vitest_02.log |
| 34759d479a54ed70ea145ba3739d28435d152fcc9f8f758ec203f5bbcd8bf1c9 | 440 | vitest_03.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_04.log (exit 0, no output) |
| d41c2d695298333b9350e8c6ede9121e1f0ade2167f06e6901c5da4e1c7574e6 | 560 | vitest_06_final.log |
| 9a5a9b736a95bab887b8f795f3620d7c4cb2296f090726932702f5dbf899b743 | 112 | tsc_07_final.log |
| 408fc0d240726751fef28642ae0b605bde3b09e929ff14d66173ece8346a901c | 237 | final_hashes.txt |
