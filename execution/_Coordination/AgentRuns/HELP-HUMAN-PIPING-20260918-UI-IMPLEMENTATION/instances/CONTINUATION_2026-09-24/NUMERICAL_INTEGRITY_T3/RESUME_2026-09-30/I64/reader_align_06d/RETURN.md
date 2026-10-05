# I64 return: TypeScript reader aligned to snapshot 06d (last alignment before review)

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This was its sixth grant, sent by ROOT's mid-run message in the same harness-native subagent session. It had no descendants.

- **Run:** 2026-10-03T22:37:41Z to the final checks at 22:42:37Z; return written about 22:47Z. Well inside the 45-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations, and Git reads used `GIT_OPTIONAL_LOCKS=0`. No install, build, Cargo, solver, native, UI or DEC-025 job.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's two Rust files showed as modified in READER.
- **Paths** use the brief's placeholders.

## Changed files (READER at f1ff7ebddc, inside the fence)

| File | Before (= 8e70db0198) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 2d0c710073b815aee77e5a7ae70b66d5a358c6ade1ff1442e707057049c5e5a6 (100714 B) | 7c802881a280385926930a90e624fb60a65069100268bdffcd3a77eb84d18845 (103060 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 2a00c815c4092cc9fb65a2eab4a72b2d3880a134b591b8cbce9e29c349414f64 (27967 B) | 654caced2ead914297b0d26c72ab1a6735924396ca7f17ed18897c2fca408a1f (28872 B) |

**Shared files, verified at the start and unchanged at the end:**
- corpus `d02701ed6afb82fdd8d925900119185412c3169276ed281423dff7dd726afdf4`;
- schema `f943ebd351…`;
- definition `3e0779a45a…`;
- table `c74742ce6a…`.

The Python reference is `55736ea65a…`.

## Changes

Eligibility stays held (`SUMMARY_COVERAGE_COMPLETE = false`).

1. **R1–R3** (ruling 06d; ACCOUNTING_CAUSES). `accountingRules` mirrors Python's `_accounting_rules`, and its results join the deferred C3 work list for every product attempt. They are therefore checked at G5 WORK_MISMATCH, after the association and typed passes.
   - **R1:** `adapter.fault` must be null, and no `{kind:'accounting', event}` object may appear in the attempt.
   - **R2:** no `lost: true` anywhere in the attempt.
   - **R3:** every `{kind:'work_accounting', fault}` needs its fault ("both" means overflow plus inconsistent) to be contained in the join of the attempt's `{kind:'unavailable', fault}` Count faults and `sticky_status` values.
2. **O2 aligned to Python's settled rule.** Every typed diagnostic reference must be listed in `diagnostic_refs`, must resolve, and must name the case. Report and published/failed W2 references must be non-null. A report initial's `outcome` must equal the case's `solve_quality`; this Python rule had no TypeScript counterpart before.
3. **06d corpus adopted through the existing harness:** the rebased F′ and P′, the six replaced must-pass entries, `cert_failed_after_summary_storage`, the five removals and the five new mutations. No harness change was needed, because 06d has no format change.
4. **A new reader-logic test.** It isolates R1–R3: every base attempt passes all three rules, and each R mutation falsifies exactly its own rule on exactly one attempt. `accountingRules` is exported for this test only.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_00 (baseline: the 06c reader on 06d) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 270 tests: 266 passed, 4 failed (the four R mutations) |
| vitest_01 / tsc_02 | same; `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | 270/270; exit 0 |
| vitest_03 / tsc_04 | after the O2 listed rule | 270/270; exit 0 |
| vitest_06 | after the R isolation test | 271/271 |
| vitest_10 | after the report-outcome rule | 271/271 |
| probe_11 | a temporary probe recording the raising line for every mutation on the final reader; removed, with the test file restored byte-identically | 178 of 178. Identical to the earlier probe_07. |
| **vitest_12 (final)** | same as vitest_00 | **271 passed, 0 failed** |
| **tsc_13 (final)** | same as tsc_02 | **exit 0** |

All four shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Against the bar (06d)

- **All 178 mutations** produce their expected first gate and code. G7 is checked against its TypeScript per-reader expectation.
  - The four R mutations raise from the deferred WORK list.
  - `idle_budget_not_exhausted_no_group` raises at the N10 group-null idle line (ATTEMPT).
  - `MUTATION_OUTCOMES.json` has every raising line.
- **All 18 must-pass entries pass.**
- **All 15 cases validate.**
- **All earlier tests pass.**
- **tsc** exits 0.

## Checklist status (TypeScript, 06d). Changes since 06c are in bold.

| ID | TypeScript status | Evidence |
|---|---|---|
| N1 | checked | counts; pre-schedule shape; reader-logic test N10/N1 |
| N2 | checked | `schedule_fresh_first_p256` |
| N3 | checked | `skip_after_failed_candidate_*` |
| N4 | checked | `failed_verification_*` |
| N5 | checked (exact translation; no WorkAccounting) | `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`; reader-logic test N5 |
| N6 | checked | p512 ladder; reader-logic test N6 |
| N7 | checked | every selected base |
| N8 | checked | `work_accounting_at_last_slot`; reader-logic test N8. The shared Ceiling base is deferred |
| N9 | checked | `ceiling_before_last_slot`, `escalating_end_not_a_terminal_translation`, `work_accounting_after_escalating_stop` |
| N10 | checked | `idle_budget_below_invocation_limit`, `idle_ready_group_not_ledger_refusal`, `idle_work_accounting_run`, **`idle_budget_not_exhausted_no_group`** |
| N11 | implemented | no native-faithful base |
| N12 | checked (G1 schema) | `work_accounting_prior_not_on_wire` |
| N13 | checked | bases K and V |
| N14 | checked | `corrections_above_three` |
| N15 | checked | `certified_bound_unbound_drop_existing_g5` |
| N16 | checked | the `physical_*` mutations |
| N17 | partly (case scope wins, as Python's `test()` reading) | no shared base (deferred, ≥20B/60B) |
| C1 | checked | `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt` |
| C2 | checked | all bases |
| C3 | checked | `failed_slot_not_cached` |
| C4 | checked | `native_work` |
| C5 | checked | `group_sources_out_of_order`, `distinct_stiffness_merged_group` |
| C6 | checked | all bases |
| O1 | checked (G3) | — |
| O2 | **checked: listed, resolving and case-naming references; required W2 and report references; report outcome equals solve quality** | `formation_d5_dangling_diagnostic`, `report_reference_unrelated_diagnostic`, `legacy_source_dangling_diagnostic` |
| O3 | checked | `w2_published_without_initial_failure` |
| O4 | checked | `legacy_source_dangling_diagnostic` |
| O5 | checked (logic) | reader-logic test O5. The shared base is deferred |
| P1 | checked | `rebind_source_run_only` |
| P2 | checked | `stage_entered_after_failure`, `certificate_stage_check_disagree` |
| P3 | checked | `lane_k_failed_with_coverage` |
| P4 | checked | `row_index_*` |
| P5 | checked (every conversion) | `conversion_*`, `refused_member_conversion_kind_bits` |
| P6 | checked | `maxima_abandoned_separate_failure`; **must-pass `values_failed_separate_completion`, `maxima_abandoned`, `aliases_abandoned`, `bind_rows_abandoned` (storage)** |
| P7 | checked (attached entries) | `prefix_attached_old_input_unbound`; **must-pass `prefix_unattached_old_operand_attested` (on P′)** |
| P8 | checked | the reason/phase table on **F′/P′**; **must-pass `prefix_captured` (storage)** |
| P9 | checked | `certificate_check_wrong_wrapper` |
| P10 | checked | 05c coverage set; **must-pass `cert_failed_before_summary` (null) and `cert_failed_after_summary_storage` (retained)** |
| P11 | checked | `native_stage_disagrees_with_run` |
| W1 | checked | `product_work_only` |
| W2 | checked | lane and conversion counts |
| W3 | checked | — |
| W4 | not publicly checkable (attested) | — |
| **R1** | **checked** | `adapter_fault_present`, `accounting_cause_without_fault`; reader-logic R isolation test |
| **R2** | **checked** | `scalar_trace_lost_unavailable`; `product_work_only` |
| **R3** | **checked** | `work_accounting_cause_exact_status` |
| G7 | per-reader code, detail separate | `g7_maximum_off_enclosure` |
| G8 invocation | checked | the seven invocation-edit mutations |

## Every known behavioral difference from Python (`55736ea65a`)

**Scope of this comparison.** I compared the checklist rules and the Python functions I read: `_g5_schedule`, `_g5_cache`, `_g5_native`, `_g5_ordinary`, `_g5_stages`, `_g5_typed`, `_g5_products`, `_accounting_rules`, `_g5a_coverage` and `_g5_numeric` (heads), `selected_material`, and the draft-level ordinary/G7 block at lines 1445–1470. This is not a line-by-line equivalence proof of the two readers. None of the differences below changes a 06d outcome.

1. **Failed verification: solve versus pass (N4/N5).**
   - **TypeScript** treats a failed verification as a *solve* failure only if its record has no verification shared build and zero `verification_lme`. Only then may an escalating stop advance two slots, or end in the Ceiling at ≥p256. Otherwise an escalating stop is rejected (G5 ATTEMPT).
   - **Python** treats every escalating failed-verification stop as a solve failure, without consulting those record fields.
   - **On emitted receipts the two agree,** because a pass failure never carries an escalating stop. They differ only when the record says the pass was entered: TypeScript rejects, Python accepts.
2. **Stop-rule reason locator (TypeScript only, inherited from I60).** A record outcome reason carrying a `quantity` must match a source layout row with the same body and kind (G5 ATTEMPT). I found no Python counterpart.
3. **Candidate record shape (TypeScript only, inherited from I60).** A `candidate`-role record must have `verification` null, no verification shared build, and `verification_lme` 0 (G5 ATTEMPT). Python checks the candidate role and fragment rules, but not these three fields.
4. **Ordinary-pass extras and order.**
   - **TypeScript only:**
     - every entry of `diagnostic_refs` must resolve and name the case, and the list must be unique;
     - W2 must match the initial failure's trigger tag and error, and a published W2 must have `force_scale_exponent` ≠ 0;
     - a selected case needs `solve_quality` in {sensitive, unresolved, failed}.
   - **Python only:** the selected-case `source_identity_sha256` re-check at G5 PRODUCT_ATTEMPT. TypeScript checks the same hash earlier, at G1 RECEIPT.
   - **Order:** Python runs its report/quality/not_required ordinary checks after the product pass, including the product WORK list. TypeScript runs them in the ordinary pass, before products. Single defects give the same result; a dual ordinary-plus-product defect would differ.
5. **Unobservable or structural only.**
   - TypeScript's explicit G1/G2 coverage guards (accepted earlier).
   - TypeScript's G5a check rejecting −0 in summary values. G2 `nonnegative_bits` rejects −0 first.
   - Raise locus: `idle_work_accounting_run` raises at TypeScript's N10 line, but at Python's WorkAccounting line. Both are G5 ATTEMPT.

**No expected outcome looks wrong,** and I found no defect in 06d.

## Files read in this grant (sha256)

| sha256 | File |
|---|---|
| 1255ce9a4473f82050d4b8f8d2efa622e8215c5a3a29ff288025762f322c452a | T3/ROOT_RULINGS_V1.md at d16fd47d21 (the two 06d sections) |
| d562af562ddf057b55a3e790ded7056172ff607ae0dfb87be3a0020da14fb628 | R/I62/coverage_shared_python_01/ACCOUNTING_CAUSES.md (headings, and through the ruling and snapshot summaries) |
| 1bfa35c514c302bf16b5cd00f8e34627609c01b4ab5a83dcfa8ea83affbffbea | R/I62/coverage_shared_python_01/RETURN_C2_5.md |
| becb83842f61765f5478efb9deaabaabfaf2726af416e2314f0cb2b9b0d0cbc6 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06D.json (keys, `counts`, `format_change`, `reader_rules`, `retired_or_deferred`) |
| 55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f | READER/P/core/analysis_runs/retained_precision.py (`_accounting_rules` 195–245, `_g5_ordinary`, the `_g5_schedule` replay, and lines 805–832 and 1445–1470) |

I also used the corpus at d02701ed6a and the fenced files at their 8e70db0198 bytes.

## Bulk (WT/scratch/i64_reader_align_06d/)

| sha256 | Bytes | File |
|---|---|---|
| 2d0c710073b815aee77e5a7ae70b66d5a358c6ade1ff1442e707057049c5e5a6 | 100714 | before/retainedPrecision.ts |
| 2a00c815c4092cc9fb65a2eab4a72b2d3880a134b591b8cbce9e29c349414f64 | 27967 | before/retainedPrecision.test.ts |
| 7c802881a280385926930a90e624fb60a65069100268bdffcd3a77eb84d18845 | 103060 | retainedPrecision.after.ts |
| 654caced2ead914297b0d26c72ab1a6735924396ca7f17ed18897c2fca408a1f | 28872 | retainedPrecision.test.after.ts |
| 959db378bbec5a5990ea258975fae2062161b480711869aca19e64160b8fb5c3 | 5468 | i64_06d_reader.diff |
| e83524c68dce06e3056e01e4ce6797a8ab93c5bd98ed1b56253637e1f99a67d7 | 2265 | i64_06d_test.diff |
| 6c89d60ad3505cdd1ad0af56099b9ea54d78bf25f3a72e48669299daa9bda181 | 25722 | probe_11.txt (final reader; identical to probe_07_superseded) |
| 8e693c37f499121f2803dccbeb0ca5f2a2822bc9258600f9ac22b50e2fb116c3 | 338 | probe_11.log |
| 6c89d60ad3505cdd1ad0af56099b9ea54d78bf25f3a72e48669299daa9bda181 | 25722 | probe_07_superseded.txt |
| e5711888395ee75eafa7e3f08ddec556a076f7208cab421bd6492f41318eec46 | 25722 | probe_05_superseded.txt (before the export comment shifted lines) |
| 36fcc5e3b0ae49f18b890d7f7c6c0bbe74ba9e11a7dbb9443f927e5822cf7289 | 2595 | vitest_00_baseline.log |
| c1c54c7e5a86b1585d18599be572e179fef524ff7b3030c803a24498c2756718 | 440 | vitest_01.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_02.log and tsc_04.log (exit 0, no output) |
| f2ab4d688ca3430a31eabec1c6c3354447652677f97a4940b7b70bbc4a880c90 | 440 | vitest_03.log |
| b8fa078b8c1324cc608f5dd67d02d60ff1d3cfb83c896cf597e2b8ec262f5226 | 440 | vitest_06.log |
| 7b6b236e6050bac44ecee4b1ea1c3c16ebd1a9ab165d33e89f7740f104a491ec | 440 | vitest_10.log |
| 3399457179960df50a1f685dabf22ad58fcd59aae390ac91f226f4a969987a8c | 560 | vitest_12_final.log |
| 53d225d985219d3d0b9129c6fcfd4dac6c5970dbf46d2e3ec3214bc2e751d1a7 | 112 | tsc_13_final.log |
| 0276862ef76784bbff3373b726d86d53f1fb3cec2b68bad22eda626aa4552268 | 237 | final_hashes.txt |
