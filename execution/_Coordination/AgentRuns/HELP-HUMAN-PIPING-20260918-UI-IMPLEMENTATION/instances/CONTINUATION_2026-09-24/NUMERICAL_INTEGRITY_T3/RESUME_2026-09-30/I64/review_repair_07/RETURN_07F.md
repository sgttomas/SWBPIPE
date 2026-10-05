# I64 return: D37, RV81-N1 (R35), and snapshot 07f

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), on its standing ownership of the TypeScript reader. ROOT sent this round as a mid-run message in the same harness-native subagent session. I64 had no descendants.

## Run conditions

- **Run:** the grant was read at 2026-10-04T02:35:30Z and the final checks finished at 02:45:27Z. This file was written at about 02:48Z, inside the 60-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:**
  - no Git writes or index operations;
  - no install, build, Cargo, solver, native, UI or DEC-025 job;
  - the existing node_modules link and WASM assets were used as found.
- **Other authors' files:** the shared files and I62's and I63's files were not touched.

## Basis

- **Rulings:** NUM c8e97c6df1, ROOT_RULINGS_V1.md from "RV78 final check (confirm 05)" to the end (D35, D36, D37). The file's sha256 was `2702b3cf8dd07a8d02674667a79c861abc01f6c4ffd26df3f058116c6cb6957e`.
  - NUM has since moved to 8f90ec3a45. That commit adds the 07f commit record and I63's status, and neither changes this scope.
- **Native code:** PP `core/product_physics/src/retained_product.rs` at NUM, sha256 `d07383fc02…`, the same bytes I62 cites. I read these functions:
  - `prepare_owned_case` (3136–3260);
  - `solve_native` (3272–3291);
  - `project_candidate` (3456–3550).
- **I62's native table:** RETURN_07F.md, sha256 `5ef4b15c114e7227a702758fa4d4c40c808489cddab7b064b2c6b1ebf138b8ca`.
  - Its code form is `ERROR_STAGE_RECORDS` in the frozen Python reader, sha256 `d77008e24f…`.
  - The return appeared after I had drafted the table from the native code. My draft matched it except for `capture` (a), a native failure before any Run, which I then added. The final TypeScript table equals I62's row for row.
- **Snapshot:** I62's SHARED_SNAPSHOT_07F.json, sha256 `c26a419f93c34d0d5be66567393a3769845406facfeb9820ea20d22729bffbf7`.

**No stop condition arose.**
- **Only tightened:** D37 is a new rejection in class 3. The existing one-direction check, the first failed stage against the allowed kinds, is kept beside it, so nothing is removed or weakened.
  - The new check rejects `capture` with a failed preparation stage, which the old rule admitted. It is now reported as `preparation`, as I62's table states.
- **D36 triage:** no new finding in this round.

## Changed files (inside the fence)

READER was at 63355a91d2 when the round began. It is now at fd76145542, I62's 07f commit, which touched only the corpus and Python files. HEAD's copies of both fenced files equal the before copies below.

| File | Before (HEAD) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | eb8b9d5aa04eb1984e60315959b4d64c474e4c255908f0e163c3de447e01abee | 47f6ea2468b9603010d6498090c2a4611c9e69325ebe8c8203fc476806f4080c (117173 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | a8c03ee1014feb1eac144ee65799cb028506f335ae780393a2603f7044f5f199 | 9836b32856c79b92eba5a5391c7e63a06d33d3fcae60bbd8d51201aac5e1f5ab (92705 B) |

**Shared files:**
- The 07f corpus `2cae6d68231f945e21f883e4b7d4dd7ed0c53e533b7ca7740c2cf402c50fdabe` is READER's working-tree and HEAD corpus. It equals the hash in SHARED_SNAPSHOT_07F.json, so 07f was adopted on that match.
- The following are unchanged:
  - the schema `07951edacf`;
  - the definition `3e0779a45a`;
  - the table `c74742ce6a`;
  - the inherited table `ae55503d44`.

## Changes

### D37 (G5 PRODUCT_ATTEMPT, class 3)

**The table (lines 632–649).** `NATIVE_STAGE_RECORDS` lists, for each unavailable error kind, the ten-stage records the native sequence can leave. Each record is a string of marks, one per stage in `STAGE_ORDER`:
- `-` means not entered;
- `C` means completed;
- `F` means failed;
- `?` means completed or failed.

| Kind | Records | Native site |
|---|---|---|
| preparation | `F---------` | `prepare_owned_case` fails |
| native | `CF--------` | `solve_native`, nonselected Run |
| capture | `CF--------`, `CC--------`, `CCCCCCCC--`, `CCCCCCCCCC` | `solve_native` before a Run; before ProofStart; after a passed certificate; the commit |
| proof | `CCF-------`, `CCCF------`, `CCCCCCCF--`, `CCCCCCCF??` | `begin_prepared_product`; `project`; `certify_final` (Observables and G5a are both entered and checked when the verdicts were copied, PP:3501–3508) |
| values | `CCCCCF----` | `complete_maxima` |
| abandoned | `CCCCF-----`, `CCCCCCF---`, `CCCCCCC---` | maxima; aliases; `bind_rows_view` |
| numeric | `CCCCCCCCCC` | both checks passed, but the case did not pass |
| observable | `CCCCCCCCF?` | Observables failed (it takes precedence over G5a) |
| g5a | `CCCCCCCCCF` | Observables passed, G5a failed |

**The check (lines 778–780)** sits in the P9 loop, after the existing first-failed-stage check. An unavailable attempt fails G5 PRODUCT_ATTEMPT unless its record matches one of its kind's records.
- **Both directions are enforced together.** The kind must be one that the first failed stage, or the last stage reached, can produce. And every stage the kind presupposes must be recorded as entered, completed or passed.
- **Stage and check agreement** (completed means passed, failed means failed with that kind's error) is already enforced in class 2: lines 706–708, and the error-to-check pairing at lines 754–757.
- **TS:737–738 (D35's S1)** are unchanged; they are now lines 755–756.

**Reader-local tests: "07f round (D37)".** Each test starts from attempt 1 of `two_case_facade_after_certificate_synthetic` and replaces its error, its stage record and its three checks.

- **Consistent shapes (Y6) do not give PRODUCT_ATTEMPT.** These are:
  - `capture` after a passed certificate;
  - `capture` at the commit;
  - `numeric` with all three checks passed;
  - `observable` with G5a passed, and with G5a failed;
  - `g5a`.
- **Eleven inconsistent shapes give G5 PRODUCT_ATTEMPT.** These are:
  - the five X1 shapes;
  - `numeric` with G5a failed, or with Observables failed;
  - `g5a` with Observables failed;
  - `capture` with a failed check after the certificate;
  - `abandoned` after a completed certificate.
- **Failed preparation.** On `two_case_preparation_failure_synthetic`, a `capture` error with a failed preparation gives PRODUCT_ATTEMPT. The base's own `preparation` error passes.

**Decisiveness on the prior reader** (eb8b9d5aa0):
- **Already caught (4):** g5a with G5a not entered, observable with Observables not entered, proof with the certificate passed, and values with Values completed. TS:736–739 already rejected these.
- **Passed (7):** numeric with neither check entered, numeric with G5a failed, numeric with Observables failed, g5a with Observables failed, both post-certificate `capture` shapes, and abandoned after the certificate (vitest_08_d37_on_before).
- **Also passed:** `capture` with a failed preparation (probe_capture_prep_on_before).

### RV81-N1 (R35), optional and done

The test "07f round (RV81-N1 R35)" checks that a value near a const or enum member still fails G1 RECEIPT:
- `directional_springs` (const 0) set to ±`Number.MIN_VALUE`, 2^-1022 or `Number.EPSILON`;
- `quantity_kind` (enum [0, 1]) set to the same tiny values, and to the neighbours of 1, `1 + EPSILON` and `1 − EPSILON/2`. (`1 + MIN_VALUE` rounds to 1.)

Controls: −0 at `directional_springs` gives G2, and the member 1 passes G2.

A temporary R35-style mutant mapped `|v| < 1e-300` to 0 in G1's comparison. The test killed it (vitest_03_r35_mutant), and the reader was restored byte-identically.

## Runs (from READER/P/apps/desktop)

| Run | Corpus | Result |
|---|---|---|
| vitest_04_draft, tsc_09_draft | 07e `bbca15d940` | 429/429; exit 0 |
| probe_wip (temporary probe) | 07f working tree, before freeze | 268/268; 22/22 |
| vitest_10_07f, tsc_11_07f | 07f `2cae6d6823` | 436/436; exit 0 |
| probe_07f (temporary probe) | 07f | **268/268 mutations** at the expected first gate and code; **22/22 must-pass** |
| probe_07f_on_before (prior reader swapped in) | 07f | 267/268; 22/22. The one miss is `numeric_error_with_checks_not_entered` (Y4), which passed on the prior reader. |
| **vitest_16_final** | 07f | **436 passed, 0 failed**; all 15 cases validate |
| **tsc_17_final** | — | **exit 0**, no output |

- **Probe hygiene:** each temporary probe was removed afterwards, and the reader and test file were restored byte-identically.
- **Commands:**
  - `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`
  - `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`
- **Final hashes:** the final run's hashes were identical before and after (final_hashes.txt).

### The 07f delta on this reader

OUTCOMES_07F.json has every entry with its raising line and source.

| Entry | Expected | Final reader | Prior reader |
|---|---|---|---|
| g5a_error_with_g5a_not_entered (X1 = Y1) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:756, class 2) | same |
| observable_error_with_observables_not_entered (X1 = Y2) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:755) | same |
| numeric_error_with_checks_not_entered (X1 = Y4) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:780, D37) | **passed** |
| proof_error_with_certificate_passed (X1) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:757) | same |
| values_error_with_values_completed (X1) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:754) | same |

## Remaining known differences

These are unchanged from RETURN_07B.md:
- the fail-closed fallback code (accepted, unreachable);
- the bundled-file G0 codes (unreachable from a receipt);
- the explicit G1/G2 guards;
- the `@internal` test hooks.

TypeScript keeps the old first-failed-stage check next to D37. Python replaced it. The outcome is the same, because every record the old check refuses is also outside the D37 table.

`SUMMARY_COVERAGE_COMPLETE` stays false.

## Bulk (WT/scratch/i64_review_repair_07f/)

| sha256 | Bytes | File |
|---|---|---|
| eb8b9d5aa04eb1984e60315959b4d64c474e4c255908f0e163c3de447e01abee | 114945 | before/retainedPrecision.ts |
| a8c03ee1014feb1eac144ee65799cb028506f335ae780393a2603f7044f5f199 | 86020 | before/retainedPrecision.test.ts |
| 47f6ea2468b9603010d6498090c2a4611c9e69325ebe8c8203fc476806f4080c | 117173 | retainedPrecision.after.ts |
| 9836b32856c79b92eba5a5391c7e63a06d33d3fcae60bbd8d51201aac5e1f5ab | 92705 | retainedPrecision.test.after.ts |
| 2e669777c09e7d6badc544d94a89be5d048c065dabf76d29fd837a51f2cffcd4 | 3156 | r07f_reader.diff |
| 66dbf14d866cc7a8f59fba8a08b06b65da5cd05a394b0d27d1f0ba7b5c79f32e | 7782 | r07f_test.diff |
| 63cbba3e0ba59ebc11fea9878abf943d18cf9b3cbd7e1c5fe1fd1c96b20abf63 | 2470 | stage_census.py |
| b3c095cbbacaeac0ef229e7e0da2a1303a4dc2457146267f7cbbe1cae9a8ae3d | 44570 | stage_census_07e.json |
| c8131b60bbba29c1717da5dba0e97fb5595ef7ec1bb0e447d66ceb2a927fd7bd | 1609 | vitest_01_r35.log |
| ca690213529664635033dcfc2cf68b79406c86be51c4ff2ba6db0657141d69ce | 338 | vitest_02_r35.log |
| 3d56e67384a8acc3ccf2479b9606b27d1e45bce199ec85588d454b0998e5dc4b | 1225 | vitest_03_r35_mutant.log |
| 3b99a7187c6669c9f326a6575c4cec31bb24c801b8559db2fd2a20ed74540051 | 440 | vitest_04_draft.log |
| d2f99b7c6837f2fe88d0f56981c1d324916cf57de3da1ca804c44a729990a13b | 338 | vitest_05_d37.log |
| d048106a4224c8251063efac2ec834efaf0e93cb78f01d726ba925fe64652e23 | 1603 | vitest_06_d37_on_before.log |
| 777a66759b2a9f055060664f7bfa64f021a3e9fc06a93491d5474e05969681fc | 338 | vitest_07_d37.log |
| 8078632fc9be5f30736e2bdb2162f148cb857763720bc161902094a7542f785a | 2683 | vitest_08_d37_on_before.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_09_draft.log |
| 2b5672be1fb93e7dc25767f138af606c01c04844423ff2d4aec89ffbb5f9b29e | 440 | vitest_10_07f.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_11_07f.log |
| 1b78ae2babf3a5c5ce1797520256bbbc2973de98befce241f16603f886acfa17 | 338 | vitest_13_new.log |
| 8c13f6fa3d92943ea50a1471399c6542b5f1ad298bbf34fbf044f6a222a2dd6b | 2403 | vitest_14_new_on_before.log |
| 3edfd6f4030a2c36ca8fdd8dbf9f070a14d8b71c9cf6a14037e0cbad21f4c928 | 906 | probe_capture_prep_on_before.log |
| 07f1767388f3af3f2c5df37ad288621e8785fa87da0e7d01e162717f8a60f527 | 38003 | probe_wip.txt |
| abf0e027f28c8ea50e08d5877dcffdb628e594b30210b21a41fc1401fefbd5ca | 338 | probe_wip.log |
| 41112eac11c507a14aeeb92a6cb59e6b3d354826bdecd9401a112ab17501e412 | 38003 | probe_07f.txt |
| 782a5c1eab873513a33fdd0614a533910b4fb6bccd7e1629961f48fb1eefb7ef | 338 | probe_07f.log |
| d500b7758f6164de227f91c4e6daf8acba8941551da4b72d8a3e129d458dd004 | 37902 | probe_07f_on_before.txt |
| ed0f93f143ad82f73265f29dfb8291c2bece129fa0ee7238246e201c515a6f32 | 338 | probe_07f_on_before.log |
| 9f2335bcba88d2ed72d9d7bc6b403ce0355d4c734f3d806867ed5bcef51968f6 | 2403 | vitest_15_new_on_before.log |
| edc19d53c475775abbd92e5b3c73e16e01da9bb1eb174669959bcb2b59939a75 | 440 | vitest_16_final.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_17_final.log |
| 5ca1d6a61324915764e8120dd44bc85174d8dbb2f11c12b960e456c0f2c41168 | 259 | final_hashes.txt |
| d304c361e5633ff71a5dcbfb5078e0342472421d2c99fcf7cebcba7109100d38 | 4772 | make_outcomes.py |
