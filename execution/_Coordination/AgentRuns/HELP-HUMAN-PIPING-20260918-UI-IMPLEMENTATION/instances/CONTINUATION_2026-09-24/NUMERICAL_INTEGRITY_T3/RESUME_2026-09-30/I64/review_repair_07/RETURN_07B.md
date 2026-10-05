# I64 return: confirmation repair round (D19–D30) and snapshot 07b

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), on a standing assignment for this round. ROOT's mid-run messages in the same harness-native subagent session sent the original assignment and the D27–D30 addition. I64 had no descendants. This is the single return for the round.

- **Run:** 2026-10-04T00:45:31Z to the final checks at 00:58:47Z; this file was written about 01:03Z. Inside the 2-hour box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, build, Cargo or native job.
- **Other authors:** the shared files and I62's and I63's files were not touched.
- **Basis:**
  - ROOT_RULINGS_V1.md: "Confirmation findings: disposition D19–D26 and the repair round" (NUM 886c1a865c) and "RV80 confirmation: disposition D27–D30" (NUM b76f82289f). Read at sha256 `aac61941…`.
  - The confirmation reviews: RV81 `14beeb33…`; RV78 `41dcba81…` (only its N4 row).
  - I62's SHARED_SNAPSHOT_07B.json (`d5fb2022…`), including its D19 native check: "passed (no stop)". So both halves of D19 are implemented.

**No stop condition arose:** no expectation looks wrong, no check was removed or weakened, and nothing outside the fence was needed.
- D20 moves the selected-case attempt requirement from the ordinary check (ATTEMPT) to class-2 association (PRODUCT_ATTEMPT), as the ruling directs. The relation is still enforced, so this is not a weakening.
- D27 replaces the WORK-derived value with the recorded one, as the ruling directs; the condition itself remains.

## Changed files (READER at b36739112a, inside the fence)

| File | Before (D18 return) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | b92f29250880393cda55a1bc49de81abcded7da825906ff6716543c06da5ef0e | 6447c4e51ea0ca179bb5818c859e952f693f92c0c40df66cb6517882273b5487 (113611 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 4a883f07714fe761358afdaeaff682ba998f036dc3e93c9affe507f082aa2751 | 07b68be1844db33a87d77c1f5e26d3cab9ecd461973031d76bf16170a3eca446 (76607 B) |

**Shared files:** the snapshot 07b corpus is `729c12574afe5f46f7e37ec3805cf5b50d684100a1ba46465501939df7cb9ce4`. It is READER's working-tree corpus and equals the hash recorded in SHARED_SNAPSHOT_07B.json. The schema `07951edacf`, definition, table and inherited table are unchanged.

## Decisions: status, and the test that pins each

"Was" gives the result of the same test on the pre-round reader (vitest_06, with `productAttempts` exported for that run only).

| Decision | Status | Pin |
|---|---|---|
| **D19, unavailable half** | **Implemented.** A case whose attempt is unavailable must carry `prepared_product_failure` naming that attempt (G5 PRODUCT_ATTEMPT, class 2) | Shared: `unavailable_attempt_under_facade_failure_cause`, `unavailable_attempt_under_source_error_cause`, `preparation_error_selected_run_under_receipt_cause`. Reader-local: "D19: an unavailable C3 attempt …" (was pass) |
| **D19, Ready half** | **Implemented** (I62's native check passed). A Ready attempt belongs to a selected case, or to an unavailable case with a `receipt_failure` cause | Reader-local: "D19: a Ready attempt …". A facade cause gives G5 PRODUCT_ATTEMPT (was pass); a `receipt_failure` cause passes class 2. 07b defers the shared pin |
| **D20** | **Implemented.** The attempt requirement moved out of `ordinaryAttempts` into the class-2 case pass, after the ordinary checks; the ordinary facts stay ATTEMPT | Shared: `selected_case_without_c3_attempt`. Reader-local: "D20: …" (was G5 ATTEMPT) |
| **D21** | already compliant (`verification_shared_build_ref` and `verification_lme` discriminator) | Shared: `vbuild_on_escalating_failed_verification`. Reader-local: "D21/RV81-N1: …" (kills M02) |
| **D22 (RV81-S1)** | **Implemented.** The G3 source-dependent checks (sourced D1 comparison and roster) run only when `source_ref` resolves; a dangling one fails at G5 PRODUCT_ATTEMPT through the explicit `at` lookup | Shared: `dangling_attempt_source_ref`. Reader-local: "D22/RV81-S1: …" on two bases (was G3) |
| D23 | already compliant (ids against `kernel_member` at G3) | Shared: `source_member_map_kernel_id_noncanonical` |
| **D24** | **Implemented in the harness:** `after_rehash` is applied literally after rehash "all", with nothing rehashed afterwards | Shared: `forged_receipt_hash`, `forged_publication_hash`, `forged_preparation_hash`, `source_identity_stale_receipt_rehashed`. Reader-local: "D24: …" |
| D25 | nothing to change: JSON.parse already reads values | — |
| D26 | already compliant | Shared: `g5a_sanity_margin_between_2m40_and_2m39` |
| **D27** | **Implemented.** The idle-Run rule reads the Run's recorded `invocation_before` against `invocation_limit`. A scan of class 1 found no other ATTEMPT check reading a WORK-derived value: the fragment checks read record indices; the sums feed only WORK predicates | Shared: `idle_run_exhausted_meter_chain_broken` (G5 WORK). Reader-local: "D27: …" (was ATTEMPT) |
| D28 | already compliant. The locator applies to every record reason carrying a `quantity`, which covers all four tags; logical outcomes and verification reasons are tied to record outcomes | Shared: `verification_estimate_…`, `charge_…`, `publication_enclosure_quantity_not_in_layout`. Reader-local: "D28: …" (all four tags) |
| **D29** | **Implemented.** G3 requires every CaseSource's body inventory to be non-empty | Shared: `empty_body_inventory`. Reader-local: "D29: …" (was G5) |
| D30 | already compliant (a native error's `run_ref` names its own nonselected Run) | Reader-local: "D30: …", through the new `@internal` test hook `productAttempts` (no faithful nonselected-Run base) |
| **RV81-N2** | **Fixed:** the kernel-scope test now edits the `nonbudget_failure` build. A new isolating test adds an unreferenced build: a non-accounting reason gives WORK, while a `work_accounting` stop gives ATTEMPT, so the kernel scope alone decides | "RV81-N2: …" |
| **RV78-N4** | **Refreshed:** OUTCOMES_07B.json, after D18. On 07a this reader also gave 236/236 and 19/19 (probe_07a) | — |

## Runs (from READER/P/apps/desktop)

| Run | Corpus | Result |
|---|---|---|
| vitest_01, tsc_02 | 07a `a6fa398731` | 379/379; exit 0 (after D19, D20 and D22) |
| vitest_05 | 07a | 390/390 (all of this round's tests added) |
| vitest_06 | 07a, pre-round reader | the round's tests: 6 failed (D19 ×2, D20, D22, D27, D29), 5 passed (D24, D28, D30, D21/N1, N2 isolation) |
| probe_07a, vitest_07, tsc_08 | 07a | 236/236; 19/19; 390/390; exit 0 |
| vitest_09 | 07b `729c12574a` | 407/407 |
| probe_07b | 07b | **253/253 mutations** at the expected first gate and code (G7 per reader); **19/19 must-pass** pass |
| **vitest_10 (final)** | 07b | **407 passed, 0 failed** (all 15 cases validate) |
| **tsc_11 (final)** | — | **exit 0** |

Commands: `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` and `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`.

The final run's shared hashes were identical before and after (final_hashes.txt). The probe blocks were removed, and the test file was restored byte-identically (`07b68be184`).

## Remaining known differences from Python and Rust

1. **The fail-closed fallback code:** ATTEMPT here, PRODUCT_ATTEMPT in Python (accepted; unreachable).
2. **Bundled-file G0 codes** (unreachable from a receipt).
3. **Explicit G1/G2 coverage guards** in TypeScript (accepted; no outcome change).
4. **Test hooks:** TypeScript has `@internal` exports `productAttempts` (new, for D30), `nativeRuns`, `nativeSchedule`, `ordinaryAttempts`, `accountingRules`, `stopFeasible`, `phi512` and `eHat`. None is a path to eligibility (D14).

The D1 sourced id comparison (difference 4 in RETURN_P2) is closed by D23. D25 removes the integral-float difference. Rust was not compared beyond the shared pins; I63 is adopting 07b concurrently.

## Bulk (WT/scratch/i64_review_repair_07b/)

| sha256 | Bytes | File |
|---|---|---|
| b92f29250880393cda55a1bc49de81abcded7da825906ff6716543c06da5ef0e | 111823 | before/retainedPrecision.ts |
| 4a883f07714fe761358afdaeaff682ba998f036dc3e93c9affe507f082aa2751 | 68525 | before/retainedPrecision.test.ts |
| 6447c4e51ea0ca179bb5818c859e952f693f92c0c40df66cb6517882273b5487 | 113611 | retainedPrecision.after.ts |
| 07b68be1844db33a87d77c1f5e26d3cab9ecd461973031d76bf16170a3eca446 | 76607 | retainedPrecision.test.after.ts |
| 7d43f36891c25c0e50cb7dc2763f012d4aec077a9edea142d57750d72b8299c0 | 7318 | r07b_reader.diff |
| f2d83c66465519a7dfcea1e248472b029908dc984e1b23f3510be8491f67210d | 10683 | r07b_test.diff |
| aa2b069a2e59abe7f36d1cde27a2cf0f86f16586b880bb08103271f60146505e | 35625 | probe_07b.txt |
| cb444411d89443404a5a9fe33eae21fc2fe28ad535a8e2dbc393ff2c58fa6cca | 338 | probe_07b.log |
| 0774a13a4434d6ccd1b4ce3b4f8ad238d53e35ce4b9b950f9c83462ea298ebc4 | 33181 | probe_07a.txt |
| b112166ff095122ec546aa0225c76611f5b96310dc1568281219281df40399ef | 338 | probe_07a.log |
| c46439d5821664105520271a0a1d9411fa17e7e11b7c044475cb68aa692aab4d | 1329 | probe_block.ts |
| d88877f340eeebd035805d44ad8a28ec94ecbdde94967fcd42d6e7518e014d27 | 5899 | vitest_06_new_tests_on_before.log |
| 20cf24f62b960c59fde61b426c5b967a42b7f733d4f0dbddaf444ae9ede3fb99 | 560 | vitest_07_on_07a.log |
| 54d418bc1ba17b785950e751e1234719ed2fbd3fbf3c8c796b0991f20bc2d8f4 | 112 | tsc_08_on_07a.log |
| d15d8c53d3202e90f90d9102ffd74a601511c22eb78ac8ae66259297fb3afe15 | 238 | hashes_07a.txt |
| 657e25795421c90cdd4a5b41cb67e4076f6d4ec270e20a9c7d352536c950a76a | 441 | vitest_09_07b.log |
| ee7b726624bc3b5b48f8e9ae6e95a9d591cc0fd2065e442ed09b810ad4f0ddae | 560 | vitest_10_final.log |
| e212f111c2a8eaab78ae1e148c5929bb04b1c534485243a98651ab56023f4d0e | 112 | tsc_11_final.log |
| 0ee56d193a80027d7c241ead2cdddd14c9e4dde3e48d0f9896e7178f40157e2e | 259 | final_hashes.txt |
| 44e776e1d97803c35058f4cabbf3d02b2a84090551bd73837d6a30ed76645270 | 440 | vitest_01.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_02.log |
| fb98354ecc6edf07e75952642c2a36c3cc1e762ca9692b43826828f8c1869843 | 440 | vitest_03.log |
| 5382fe9ba55395cc555b3d8f2aed5ebd3f7af9c82c2edd94efaee4ac2d6ca651 | 3682 | vitest_04.log |
| 6e2929dd452c16788c1ca91b9d3d6bd882ee786fe67f077bcac46f415a21c054 | 440 | vitest_05.log |
