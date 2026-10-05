# I64 return: D31–D33 and snapshot 07d

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), on its standing ownership of the TypeScript reader. The 07d repair round was sent by ROOT's mid-run message in the same harness-native subagent session. I64 had no descendants.

- **Run:** the before copies were taken at 2026-10-04T01:29:57Z, and the final checks finished at 01:41:24Z. This file was written about 01:45Z. All of this is inside the 90-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:**
  - No Git writes or index operations.
  - No install, build, Cargo, solver, native, UI or DEC-025 job.
  - The existing node_modules link and WASM assets were used as they are.
- **Other authors:** the shared files and I62's and I63's files were not touched.
- **Basis:**
  - NUM f4d5cbbe49, ROOT_RULINGS_V1.md from "T1 and T2: I61's analysis and the rulings" through "Round 03 closed; the 07d repair round".
  - NUM later moved to aca9ad785c. Its ROOT_RULINGS_V1.md has sha256 `65b21261e9311c01db736da30c1b21a87c1f29366cff03d03044a31654b3df87`. It adds two sections:
    - "T1 confirmed by the owner", which says the readers need no change;
    - the 07d commit record.
    I read both. Neither changes this scope.
  - I62's SHARED_SNAPSHOT_07D.json has sha256 `a4fe71870cb466180d03b4d8d820b5ece6835f9c10bdd55db6130eb00e99ad8d`.
  - RV81's round-03 confirmation was the starting point.

**No stop condition arose.**
- Neither rule change removes or weakens a check:
  - D31 admits one more version string that native code accepts.
  - D33 adds a rejection condition.
- No 07d expectation appears wrong.
- No path outside the fence was written.

## Changed files (READER at 2af4a5dc50, inside the fence)

READER moved from a894d9d0ba to 2af4a5dc50, which is I62's 07d commit: corpus and Python only. HEAD's copies of both fenced files equal the before copies below.

After the final checks, READER moved again, to 265f764fa4: I63's Rust reader for 07d, touching only `retained_precision.rs` and `retained_precision_contract.rs`. At that head, the corpus is still `12da125d9d` and both fenced files are unchanged.

| File | Before (HEAD) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 9a8e6d4aaf0bcaab9fdd088e18039e5d7f9d672718ef7d9e1dec728f59e96000 | 136f39108d30760695e3c632d699462dac66650ae5d364b162931482ad60c8dc (114288 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 96268c8c81022b4059fc42ec27affc8694b59c9c5cfca05e33b6c9b5277a1948 | ca980d70672e87c229e5b30d6b88283ed63ec735a51eca7fa94da2199eedf87e (79948 B) |

**Shared files:**
- The snapshot 07d corpus is `12da125d9dcfcb55debe2fc1ab1eadcdaf1b35fd188206b251d48f563309e184`. That is READER's working-tree and HEAD corpus, and it equals the hash in SHARED_SNAPSHOT_07D.json. 07d was adopted on that match.
- The following are unchanged:
  - the schema `07951edacf`;
  - the definition `3e0779a45a`;
  - the table `c74742ce6a`;
  - the inherited table `ae55503d44`.

## Changes

**D31 (G8, `invocationBinding`).** The model `schema_version` may be `0.1.0`, `0.2.0` or `0.3.0`. Before this change, only `0.2.0` and `0.3.0` were accepted.
- The comment cites PP pressure_runtime.rs:113-118, where `0.1.0` and `0.2.0` share one branch.
- `0.4.0` stays excluded, under the C1 G8 row "no 0.4 extension".

**D32 (audit; no code change).** Every integer site in the reader is a value test: `Number.isSafeInteger`, non-negative, and not −0 where the field is unsigned. None is a type check. The sites are:

| Line | Site |
|---|---|
| 126 | `at` (index) |
| 128 | `uint` (`Number.isSafeInteger(v) && v >= 0 && !Object.is(v, -0)`); every work, limit, count and coverage-body integer passes through it |
| 149 | the schema `uint`/`i32` encodings, with minimum and maximum |
| 239 | the D22 attempt `source_ref` lookup |
| 284 | `corrections` |
| 391 | `buildOf` |
| 859 | `data_blocks` |
| 1024 | the K4 `u32` preparation words |

The only `typeof === 'number'` tests are G1 shape checks:
- **139:** schema `number`/`integer`. The schema has exactly two integer-typed nodes, and both carry a `uint` or `i32` encoding, so line 149 applies a value test to each at G2.
- **197:** the coverage-entry shape. Its `body` gets a value test through `uint` at 1259.
- **1089:** a quantity's `value`, which is not an integer.

JS `JSON.parse` gives `0.0` and `0` the same number. A float-written integer is therefore the same value at every site, and canonical JCS hashing is unchanged.

**D33 (G5, `nativeSchedule` record loop, line 307).** A record reason in the attempt space tagged `verification_estimate` must name a `force` or `moment` row (FK/retained/verify.rs:880); otherwise it is ATTEMPT. The check follows the D28 layout-row check, which still requires the named row to exist.

**Reader-local tests** are in a new block, "07d round (D31-D33): reader-local relations".

| Rule | Test |
|---|---|
| D31 | `invocation_edits` set `request.model.schema_version`. `0.1.0`, `0.2.0` and `0.3.0` pass; `0.4.0` gives G8 INVOCATION_MISMATCH. |
| D32 | Every bare integer literal in the base receipt's JSON is rewritten as an integral float, for example `"source_ref":0.0`. The receipt validates with the base classifications, so integral-float integers pass. A forged `source_identity_sha256` on the floated receipt gives G1 RECEIPT_MISMATCH, so the float-written `source_ref` resolves to its source. |
| D33 | On `p512_ladder_synthetic`, the reason tag of record 0 and attempt 0 (a translation row) is set. `stop_rule` and `charge` pass; `verification_estimate` gives G5 ATTEMPT_MISMATCH. |

On the prior reader, the D31 and D33 tests fail and the D32 test passes (vitest_03_on_before). This is as expected: D31 and D33 are decisive, and D32 confirms existing behaviour.

## Runs (from READER/P/apps/desktop)

| Run | Corpus | Result |
|---|---|---|
| vitest_01, tsc_02 | 07c `d33667719e` | 412/412; exit 0 |
| vitest_03_on_before | 07c, prior reader (9a8e6d4aaf) | D31 and D33 fail, D32 passes |
| probe_07c (temporary probe) | 07c | 254/254 mutations; 19/19 must-pass |
| vitest_04_07d, tsc_05_07d | 07d `12da125d9d` | 419/419; exit 0 |
| probe_07d (temporary probe) | 07d | **259/259 mutations** at the expected first gate and code (expected_by_reader.typescript where present); **21/21 must-pass** |
| probe_07d_on_before (temporary probe; prior reader swapped in) | 07d, prior reader 9a8e6d4aaf | 258/259; 20/21 (see below) |
| **vitest_06_final** | 07d | **419 passed, 0 failed** (all 15 cases validate) |
| **tsc_07_final** | — | **exit 0**, no output |

**Probe handling:** the probe was removed after each probe run. The test file was restored byte-identically, and so was the reader after the before-reader probe.

**Commands:**
- `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`
- `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`

The final run's hashes were identical before and after (final_hashes.txt). The hashed files were the reader, the test, the schema, the corpus, the definition and both tables.

**The 07d delta on this reader** (OUTCOMES_07D.json has every entry, its raising line and its source decision):

| Entry | Expected | Final reader | Prior reader |
|---|---|---|---|
| model_schema_version_0_4_0_rejected | G8 INVOCATION | G8 INVOCATION (invocationBinding:1081) | same |
| forged_source_identity_float_source_ref | G1 RECEIPT | G1 RECEIPT (integrity:208) | same |
| verification_estimate_names_translation_row | G5 ATTEMPT | G5 ATTEMPT (nativeSchedule:307, D33) | **passed** |
| ready_attempt_under_facade_failure_cause | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:642, D19 Ready) | same |
| ready_attempt_under_prepared_product_failure | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:642, D19 Ready) | same |
| MP integral_float_integers_and_references | pass | pass | pass |
| MP model_schema_version_0_1_0_accepted | pass | pass | **G8 INVOCATION** |
| unavailable_attempt_under_source_error_cause (changed in 07d) | G5 PRODUCT_ATTEMPT | G5 PRODUCT_ATTEMPT (productAttempts:640) | same |

## Remaining known differences

These are unchanged from RETURN_07B.md:
- the fail-closed fallback code (accepted, unreachable);
- the bundled-file G0 codes (unreachable from a receipt);
- the explicit G1/G2 guards;
- the `@internal` test hooks.

`SUMMARY_COVERAGE_COMPLETE` stays false.

## Bulk (WT/scratch/i64_review_repair_07d/)

| sha256 | Bytes | File |
|---|---|---|
| 9a8e6d4aaf0bcaab9fdd088e18039e5d7f9d672718ef7d9e1dec728f59e96000 | 113849 | before/retainedPrecision.ts |
| 96268c8c81022b4059fc42ec27affc8694b59c9c5cfca05e33b6c9b5277a1948 | 77272 | before/retainedPrecision.test.ts |
| 136f39108d30760695e3c632d699462dac66650ae5d364b162931482ad60c8dc | 114288 | retainedPrecision.after.ts |
| ca980d70672e87c229e5b30d6b88283ed63ec735a51eca7fa94da2199eedf87e | 79948 | retainedPrecision.test.after.ts |
| 92ec8900d20d8003c12d361831c2f61b202c17b3ecfba8a66847f4ac4f2ed188 | 2001 | r07d_reader.diff |
| 8ce87325422449893e9be172a3275738297477bfa22859616cc78ab4ec0db524 | 3166 | r07d_test.diff |
| 10eb962dcbb390258ab5baa275fcddaca75712b40662a19491fc526f589316c7 | 440 | vitest_01.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_02.log |
| b626a98646034604f486e83891b2979f3379df29503f9dcfc8f752b6315be23c | 1954 | vitest_03_on_before.log |
| 0c83970219715a2baa20768d3c7de7a6f6f012c244034d9aa07a743a3f477e76 | 35781 | probe_07c.txt |
| aade37aab1339179f5dccb36be9b59ab5db81ccbef0517f0dad33b00101c2908 | 338 | probe_07c.log |
| 93f93f8d6668e4bf15d2d79aebede4097efca42e53fbbd6f22ec7683c7ad1ea0 | 36640 | probe_07d.txt |
| 76524e4bb430101227a79ce158706196e85f634bf3e0b95e4136bead3608ee1b | 338 | probe_07d.log |
| c6027c2a95e375d468996e65c8f8262bbd9129735d272b5b7b18956d5d167380 | 36587 | probe_07d_on_before.txt |
| d7865fb5f5ab9c0012349f4d0696f27132c52fa472ee6aa01bf31d1e1d2db10f | 338 | probe_07d_on_before.log |
| 2494ae536819050185eff506f517bf40cc52791c6ed23ff215c2937981a6b021 | 440 | vitest_04_07d.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_05_07d.log |
| 0a189eeb45df9f42b24a0d2aad048997318dede80ac082b37b3525d57fd16fc3 | 440 | vitest_06_final.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_07_final.log |
| 164889d1d2f7872752ac6a80b9b35df1a4a7b18ea9bb5f72b187bd3d594cfae5 | 259 | final_hashes.txt |
