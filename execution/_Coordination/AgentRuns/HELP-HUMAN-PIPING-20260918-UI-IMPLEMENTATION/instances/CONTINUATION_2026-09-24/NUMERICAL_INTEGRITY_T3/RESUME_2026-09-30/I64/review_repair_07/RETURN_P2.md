# I64 return: review repair 07, phase 2 (TypeScript adopts snapshot 07)

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This phase-2 grant was sent by ROOT's mid-run message in the same harness-native subagent session. I64 had no descendants.

- **Run:** 2026-10-04T00:09:44Z to the final checks at 00:14:46Z; this file was written about 00:19Z. Well inside the 90-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, build, Cargo, solver or native job.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's Rust files showed as modified in READER, from its concurrent phase 2.
- **Basis:** NUM b2dbfcfacb. T3/ROOT_RULINGS_V1.md (sha256 `4a6d5165…`) was read from "Checkpoint A: rulings on the native facts for snapshot 07" to the end: D8, D9, D16, D17 and the three settled readings.

**No 07 expectation looks wrong.**

## Changed files (READER at 04ea067b5c, inside the fence)

| File | Before (b86ef77191) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | d5854a9a6c1e7279ddcddbf034d7bd0f1e2235800f6ace97ba933559ffadb104 | 294ba19b8afaf388281049f8341fed76d1e318f94f2f08cc50cd22f1a04f1263 (111766 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 7e64148c15ed4ef80dfe3ea12e5c600f23a82bcd9a9789120c5461fd2c0cce20 | bf2ad6d593770eff5ffd1e9fbad539946dbe492d2cdbb9256a134066ab1a833e (67597 B) |

**Shared files (snapshot 07, unchanged):**
- corpus `90f6e4ed9b5ebd00eb5d459397476c1e459b0a923eb1209037f8f86e0009367e`;
- schema `07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c`;
- definition `3e0779a45a…`;
- table `c74742ce6a…`;
- inherited table `ae55503d44…`.

## Changes

1. **Settled reading 3 (G0).** With the retained-precision contract named, an absent or non-object `retained_precision` or `body` now fails G0. Before, the header skipped the body checks.
   - Pins: `g0_retained_precision_absent`, `g0_receipt_body_absent`.
   - `receipt_version` is already exactly 1 (`g0_receipt_version_not_1`).
2. **D8, at G5 WORK in class 4.** `accountingRules` now returns R1′, R2′, R3′ and R4, mirroring the checkpoint-A table.
   - **R2′** adds OperationalError accounting found in three places:
     - a MemberOperational error;
     - a CaptureError `prepared_arithmetic` cause;
     - a G5aError `operational`/`arithmetic` cause (in the result error or a failed G5a check).
   - **R3′** covers three spellings: `work_accounting{fault}`, a nested `stop/work_accounting`, and a view `work{fault}`. Each is checked against its owner scope (`faultOwner`), with `both` meaning overflow plus inconsistent. The owners are:
     - a member's PreparationWork;
     - a lane's work;
     - the values completion;
     - otherwise, the ProofTrace.
   - **R4:** a SectionError `accounting` needs a non-exact status in that member's PreparationWork; for `preparation.section`, that is the last member's.
   - Pins: `section_accounting_exact_status` (R4), `nested_stop_work_accounting_exact_status` (R3′), `view_work_fault_exact_status` (R3′), `old_operational_accounting_not_lost` (R2′), and the 06d R1–R3 entries.
3. **D8 kernel scope** (class 1, G5 ATTEMPT, before the call loop). An object tagged `work_accounting` anywhere in a Run, a build or a group preparation now fails. This matches Python's placement and match.
4. **D9a and D9b.** These are schema changes, enforced by the closed-shape walker at G1. All four 07 G1 pins pass unchanged: `refusal_work_accounting_variant`, `refusal_count_range_variant`, `unavailable_source_ref_absent` and `source_decline_with_source_ref`.
5. **D16.** The product attempt's `ordinary_attempt_ref` is now resolved explicitly at G5 PRODUCT_ATTEMPT (class 2). The ordinary-index equality was removed from G3, where it had made `dangling_ordinary_attempt_ref` report G3. The owner/attempt bijection stays at G3.
   - Class-1 references were already explicit in the follow-up: `dangling_candidate_record` gives ATTEMPT, `dangling_build_ref` gives WORK.
6. **D17.** Already the TypeScript order: the ordinary pass, then the D4c case pass, then per-attempt association. Pins: `ordinary_dangling_ref_plus_source_preparation_null` and the T4d dual.
7. **G5b (D10 reading).** The section-term echo now checks equality only. A zero term fails the stress-scale arithmetic, so `g5b_zero_section_area` gives G5b SCALE as expected; it gave SECTION before.
   - The earlier TypeScript-only G5b positivity and term-versus-member-map identity checks were removed. G8 still binds that identity (`section.area === A_K` and so on), as Python does.
8. **Harness.** `rehash` skips when the receipt or its body has been removed, per 07's format note. It still admits only `"all"`.
9. **Tests updated.**
   - The R isolation test now covers R1′–R4: every base attempt passes all four rules, and each of the 8 accounting mutations falsifies only its own.
   - The R3′ test checks owner scopes and the three spellings, including that a status outside the owner scope does not count.
   - A new kernel-scope test.

## Against the bar

| Item | Result |
|---|---|
| Mutations | **235/235** at the expected first gate and code (G7 per reader) |
| Must-pass entries | **19/19** pass, with the base case's classifications and `numerical_eligible=false` |
| Cases | **15/15** validate with their expected classifications |
| vitest | **377 passed, 0 failed** (vitest_07_final) |
| tsc | **exit 0** (tsc_08_final) |

`OUTCOMES_P2.json` lists every mutation with its observed result, raising line and source decision (for 07 entries), and every must-pass entry. The baseline before these changes was 368/376: the 8 entries named in changes 1–7.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_00 (baseline: the b86ef77191 files on 07) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 368 passed, 8 failed |
| vitest_01 / 02 / 05 | the same, during the work | 374/376 (two reader-local tests on the old R shape); 376/376; 377/377 |
| tsc_03 | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| probe_06 | a temporary probe recording raising lines and checking must-pass classifications; removed, with the test file restored byte-identically (bf2ad6d593) | 235/235; 19/19 |
| **vitest_07 (final)** | same as vitest_00 | **377 passed** |
| **tsc_08 (final)** | same as tsc_03 | **exit 0** |

## Remaining known differences from Python 04ea067b5c and Rust

1. **The fail-closed fallback code.** Python's is PRODUCT_ATTEMPT; TypeScript's native fallback is ATTEMPT. ROOT accepted this as unreachable: every reference is explicit, and no input is known to reach either fallback.
2. **D10 integral floats at G2** is Python-only by ruling. JSON.parse cannot tell `1.0` from `1`, so TypeScript cannot express it.
3. **Bundled-file G0 codes.** TypeScript reports a mismatch in the bundled table's field identity as FORMATION_MISMATCH; the byte hashes report SOURCE_PRODUCER_CONTRACT_UNSUPPORTED, as in Python. Neither can be reached from a receipt.
4. **D1, sourced complete coverage.** TypeScript compares old ids with the source map's `kernel_member` values; Python compares lengths, and old ids are already `0..len−1` at G3 in both. They differ only for a source whose map is not `0..n−1`. G8 rejects such a source in both readers, so the first gate could differ (G3 in TypeScript, G8 in Python). No shared pin.
5. **Explicit G1/G2 coverage guards** in TypeScript (accepted). They do not change any outcome.
6. **Rust.** I have not compared Rust's 07 code; I63 is adopting 07 concurrently. Parity with Rust is claimed only where snapshot 07 pins it.

No other known differences.

## Files read in this grant (sha256)

| sha256 | File |
|---|---|
| 4a6d516510afe9c768e655aa29c23ebb0126f07edc3af140f1f10f8a73c6fd23 | T3/ROOT_RULINGS_V1.md at b2dbfcfacb (from "Checkpoint A: rulings …" to the end) |
| 946a75ccd47c743c641312905f13a3308c08ea874dfdf7e5787d591ebc5ab766 | R/I62/review_repair_07/CHECKPOINT_A.md (D8 shape table, rules and owner scopes) |
| 82072054ecde2ba6b55582c19764ba31ecc616f3188470fe2a7abef8cc515c54 | R/I62/review_repair_07/SHARED_SNAPSHOT_07.json (format, schema changes, changed 06d content, new entries, deferred, counts) |
| c7d7fe253bac1b635d81623970f9f4b8845df6a0351a8ed40549dd109de6c3f2 | R/I62/review_repair_07/RETURN_B1.md (cross-reader differences) |
| 473b8a8ae05e271ea9601bec81714213a61b863586e5f5707d208f854e431d47 | R/I62/review_repair_07/RETURN_B2.md (the D17 pin) |
| dddac2fa96adc1e148cab4e305739cf7055873f515051b5151dc4ccf56bdf752 | READER/P/core/analysis_runs/retained_precision.py (`_accounting_rules`, `_fault_owner`, `_statuses`, the kernel scope, G3 D1 and G5b sections) |

I also read the corpus at 90f6e4ed9b (entries and expectations) and the Python harness `apply_mutation` (test file, for the 07 rehash rule).

## Bulk (WT/scratch/i64_review_repair_07_p2/)

| sha256 | Bytes | File |
|---|---|---|
| d5854a9a6c1e7279ddcddbf034d7bd0f1e2235800f6ace97ba933559ffadb104 | 107748 | before/retainedPrecision.ts |
| 7e64148c15ed4ef80dfe3ea12e5c600f23a82bcd9a9789120c5461fd2c0cce20 | 65169 | before/retainedPrecision.test.ts |
| 294ba19b8afaf388281049f8341fed76d1e318f94f2f08cc50cd22f1a04f1263 | 111766 | retainedPrecision.after.ts |
| bf2ad6d593770eff5ffd1e9fbad539946dbe492d2cdbb9256a134066ab1a833e | 67597 | retainedPrecision.test.after.ts |
| 36600a809cacefd3f4c5f6775d7556b48caa1a806db03cd3f9db26294ca86a06 | 11065 | p2_reader.diff |
| 28f6c78dd179a6445dd9c72164b7ea3c456ad293124cef9f6475fa20607f4649 | 7708 | p2_test.diff |
| 5c085a32efd31b60d3750d1705b0956fa9a5f99cab2651a897cd5fe65eeef353 | 35461 | probe_06.txt |
| 3709311ac60df36873a3095f8284b46fecea7f7e06bce37cea660eea83270c42 | 338 | probe_06.log |
| c395987ab4aa6626b5c77e57d4662cb679861c6555d7a407e12527dabb95a646 | 34715 | probe_04_superseded.txt (before the kernel-scope alignment) |
| f7b49f9481aa25bb2e6198a8f5ca8e2baf39602d67274c8f1ff98bfeef7dd805 | 6530 | vitest_00_baseline.log |
| 182e7833eef875fecd239e4291ff24c62042706c7a551f4894be40301d305296 | 2794 | vitest_01.log |
| b897ef87730e64fe2d46b71f2b2b3f9e9ecfd6a8d9cddbff485d67acaadd98ef | 440 | vitest_02.log |
| 59422c56feeba58a41626666524fd38327d5f8d73d2c6025e0e98e384f703afb | 440 | vitest_05.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_03.log |
| 3c4a9de06470499d47e814da56fcd3015c6056672c04a0f8770bb9c5a5999ceb | 560 | vitest_07_final.log |
| eafcff7bf2661b3de5235c93cfc818ebcfee1f9dffe37760efe8795f7fdb0ba0 | 112 | tsc_08_final.log |
| 7f138073131c6ea58890a88d2936d48cc78333a9477be11abf9b9911eab837cb | 259 | final_hashes.txt |
