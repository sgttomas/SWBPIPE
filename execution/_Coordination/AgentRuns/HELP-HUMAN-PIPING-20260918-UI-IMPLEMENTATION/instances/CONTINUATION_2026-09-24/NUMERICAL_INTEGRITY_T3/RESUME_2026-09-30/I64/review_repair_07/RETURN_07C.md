# I64 return: D21 third indicator and snapshot 07c

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), on its standing ownership of the TypeScript reader. This addition was sent by ROOT's mid-run message in the same harness-native subagent session. I64 had no descendants.

- **Run:** 2026-10-04T01:03:08Z to the final checks at 01:05:52Z; this file was written about 01:08Z. Inside the 30-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, build, Cargo or native job.
- **Other authors:** the shared files and I62's and I63's files were not touched.
- **Basis:** NUM d232f0859f, the ruling paragraph "D21 widened by a third indicator". ROOT_RULINGS_V1.md was read at sha256 `8b2a9f6fd8c5e310c1873d17c2d6592806275bb9efdbcb562a879413185878e6`. I62's SHARED_SNAPSHOT_07C.json was read at sha256 `81623caa322ee530333b40692f06d6f83ec92f48025990d5cbc6f4ff156bd705`.

**No stop condition arose.** The change only adds a rejection condition.

## Changed files (READER at 2d82351ccf, inside the fence)

| File | Before (2d82351ccf) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 6447c4e51ea0ca179bb5818c859e952f693f92c0c40df66cb6517882273b5487 | 9a8e6d4aaf0bcaab9fdd088e18039e5d7f9d672718ef7d9e1dec728f59e96000 (113849 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 07b68be1844db33a87d77c1f5e26d3cab9ecd461973031d76bf16170a3eca446 | 96268c8c81022b4059fc42ec27affc8694b59c9c5cfca05e33b6c9b5277a1948 (77272 B) |

**Shared files:** the snapshot 07c corpus is `d33667719e777cd6d6881e207bf359b6f897bdb2f24094781943d44b68ea4b38`. It is READER's working-tree corpus and equals the hash in SHARED_SNAPSHOT_07C.json. The schema `07951edacf`, definition, table and inherited table are unchanged.

## Change

**D5b/D21.** In `nativeSchedule`, the solve-failure discriminator is now one helper, `solveFailure`. A failed verification counts as a solve failure only when nothing shows that its pass ran:
- no verification shared build (adaptive.rs:4286);
- zero `verification_lme`;
- **and, new, a null `verification` summary** (adaptive.rs:4333).

The helper is used at both sites:
- **Continuation:** a later attempt after an escalating failed verification.
- **Terminal:** the Ceiling after an escalating verification-solve failure.

An escalating failed verification that shows any of the three indicators fails G5 ATTEMPT.

**Pins:**
- **Reader-local:** "D21 widened: a non-null verification summary on an escalating failed verification shows the pass ran". It copies a real verification summary onto the failed p256 verification record of `verification_failure_skip_synthetic`, with `verification_lme` 0 and no verification build, and expects G5 ATTEMPT. On the reader before this change it passed (vitest_03_on_before), so the test is decisive.
- **Shared (07c):** `verification_summary_on_escalating_failed_verification` gives G5 ATTEMPT_MISMATCH, raised by the continuation check in `nativeSchedule`.

## Runs (from READER/P/apps/desktop)

| Run | Corpus | Result |
|---|---|---|
| vitest_01, tsc_02 | 07b `729c12574a` | 408/408; exit 0 |
| vitest_03_on_before | 07b, prior reader (6447c4e51e) | the new test fails, as intended |
| probe_07c (a temporary probe, removed; the test file was restored byte-identically) | 07c `d33667719e` | **254/254 mutations** at the expected first gate and code (G7 per reader); **19/19 must-pass** |
| **vitest_04 (final)** | 07c | **409 passed, 0 failed** (all 15 cases validate) |
| **tsc_05 (final)** | — | **exit 0** |

Commands: `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` and `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`.

The final run's shared hashes were identical before and after (final_hashes.txt).

## Remaining known differences

These are unchanged from RETURN_07B.md:
- the fail-closed fallback code (accepted, unreachable);
- the bundled-file G0 codes (unreachable from a receipt);
- the explicit G1/G2 guards;
- the `@internal` test hooks.

D21 is now the same three-indicator rule in all readers, as the ruling states for Rust.

## Bulk (WT/scratch/i64_review_repair_07c/)

| sha256 | Bytes | File |
|---|---|---|
| 6447c4e51ea0ca179bb5818c859e952f693f92c0c40df66cb6517882273b5487 | 113611 | before/retainedPrecision.ts |
| 07b68be1844db33a87d77c1f5e26d3cab9ecd461973031d76bf16170a3eca446 | 76607 | before/retainedPrecision.test.ts |
| 9a8e6d4aaf0bcaab9fdd088e18039e5d7f9d672718ef7d9e1dec728f59e96000 | 113849 | retainedPrecision.after.ts |
| 96268c8c81022b4059fc42ec27affc8694b59c9c5cfca05e33b6c9b5277a1948 | 77272 | retainedPrecision.test.after.ts |
| 80c64de428cc6b0dd957b703fc198fd99b7195f31e5943ccf3fc5d6b9e7585cc | 2217 | r07c_reader.diff |
| 204a928a438530e38c0742a3b3ca53e9144df76e2d1f34b9abd0e761c4a85926 | 1436 | r07c_test.diff |
| 582132a4e4320f1c2017a4c6bd6c72ac2c8c3334bbb2dfad29e181176796ba5c | 35781 | probe_07c.txt |
| 2f710bcd7b88399e8b088ea365fe39fb74c6f6a0f87cde5638f65275e73db1e2 | 338 | probe_07c.log |
| 9b8e39308736a39533ac31b6009bd93198cdf5e94bc38686de89dbb64dfae41a | 440 | vitest_01.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_02.log |
| df4b631125a3d094d2d5468284b8234fa9489859d524846bd2e36b3600d8cf20 | 1154 | vitest_03_on_before.log |
| 890ed295fa5a575ad4d00cdad5fa94c16e94a3b04bd293ed26c7b83690ee795c | 560 | vitest_04_final.log |
| 947a405b115c38d5ee00e7b97100caef54b69bd958e03efd022c32ed044585e8 | 112 | tsc_05_final.log |
| 246863fa565e6b8fb467e11b35c8e81dc75cba384a332c23f22f97f7a80fb1b4 | 259 | final_hashes.txt |
