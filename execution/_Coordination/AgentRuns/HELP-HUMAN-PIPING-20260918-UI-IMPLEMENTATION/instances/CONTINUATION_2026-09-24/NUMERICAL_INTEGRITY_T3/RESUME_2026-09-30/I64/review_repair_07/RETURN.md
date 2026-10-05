# I64 return: review repair 07, phase 1 (TypeScript reader)

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This grant, sent by ROOT's mid-run message in the same harness-native subagent session, is phase 1 of `BRIEFS/I62_I64_REVIEW_REPAIR_07.md`. I64 had no descendants.

- **Run:** 2026-10-03T23:24:52Z to the final checks at 23:34:49Z; return written about 23:40Z. Well inside the 90-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations, and Git reads used `GIT_OPTIONAL_LOCKS=0`. No install, build, Cargo, solver, native or DEC-025 job.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's Rust files showed as modified in READER, from its concurrent work.
- **Paths** use the brief's placeholders. FK and PP are as in the reviews.

**No stop condition arose:**
- no 06d outcome changed;
- no decision conflicts with native code I could cite;
- no path outside the fence was needed.

## Changed files (READER at 6b607fd01f, inside the fence)

| File | Before | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 7c802881a280385926930a90e624fb60a65069100268bdffcd3a77eb84d18845 (103060 B) | 240ccf4532104f559f51029d08332b0f827d9ad4f1713a44334fce1f1cb02ade (107200 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 654caced2ead914297b0d26c72ab1a6735924396ca7f17ed18897c2fca408a1f (28872 B) | 03aa2d347d7dfd8f976242f59f1bca015f283cb1e99ef007234f7f671a813fdd (62142 B) |

**Shared files, unchanged:**
- corpus 06d `d02701ed6a…`;
- schema `f943ebd351…`;
- definition `3e0779a45a…`;
- table `c74742ce6a…`;
- inherited table `ae55503d44…`, now read as bytes at G0.

## Decisions: status and the test that pins each

Test names are those in `retainedPrecision.test.ts`. "Probe X" means the RV78 PROBES.json entry X, embedded verbatim and run through the shared-entry harness. "Was" gives the pre-repair reader's result on the same test.

| Decision | Status | Pinning test(s) |
|---|---|---|
| D1 run ids and execution order at G3 | already compliant | probes R1a, R1b (was pass) |
| D1 member ids `0..len−1`, with prepared and new as prefixes (RV81-B1) | **implemented** at G3 | probe R2b (was G8); "D1/RV81-B1 …" without the invocation (was pass/needs_recompute); probe R2 |
| D1 sourced complete old coverage equals the source map | already compliant | probes R3, R3b |
| D1 unsourced complete old coverage is non-empty and equals every CaseSource member count | **implemented** at G3 (otherwise G8 compares with the invocation, as before) | "D1: unsourced complete old coverage …" |
| D1 captured_prefix split (RV81-S1) | **implemented**: members at G3; null source/run and unavailable result at G5 PRODUCT_ATTEMPT | "D1/RV81-S1 …" (was G3); 06d `prefix_captured_with_members` stays G3 |
| D1 Run origin checks in G5 class 1 | already compliant (`nativeRuns`) | 06d native mutations |
| D2 G0 union (RV81-S2) | **implemented**. Added the producer `component_name`/`component_version` and `schema_version`; sha256 of the bundled table bytes against TABLE_HASH, and of the inherited table bytes against the table's `inherited_semantic_contract_sha256` (raw imports); type-tolerant: G0 never walks a malformed body | "D2: G0 owns …": component, schema, canonicalization and missing `work` give G0; non-array `product_attempts` and a non-object attempt give G1 (was G0). The bundled-byte negatives cannot be expressed through a receipt; every validation exercises the positive |
| D3 class order | already compliant: native, then ordinary, association, typed, C3 work | probe T4d (dual: ordinary plus adapter fault gives ATTEMPT); RV81-M14 |
| D3 class-1 convention | **implemented**: native WORK predicates (including the checked sums and build lookups) are collected and reported only at the end of class 1; an ATTEMPT defect anywhere in the class wins | "D3: …" (R8 plus `invocation_before` gives ATTEMPT, was WORK; WORK alone stays WORK); all 06d native WORK mutations unchanged |
| D4a | already compliant | probe R4 |
| D4b | already compliant | probe R5 |
| D4c (RV81-B2) | **implemented** at G5 PRODUCT_ATTEMPT, class 2, for every unavailable case with the cause | "D4c/RV81-B2 …": dangling cause and other case's attempt (was pass) |
| D4d | already compliant | probe R6a |
| D4e | already compliant | "D4e: …" (both directions) |
| D5a | already compliant | probe T3; "D5a/RV81-M04 …" (verification_lme alone gives ATTEMPT under D3) |
| D5b | already compliant | probe T1 (RV81-M02) |
| D5c | **implemented** in the schedule replay | "D5c: …" (was pass) |
| D5d | already compliant | probe T2 (RV81-M03) |
| D5e | already compliant | probe R8 |
| D6a | **unchanged, waiting for checkpoint A** (list-level "names the case" kept) | probes T4a (unique) and T4b (resolve), RV81-M05 |
| D6b `checks_passed` exclusion | already compliant | probe T4e (RV81-M06) |
| D6b `not_assessed` exclusion | **unchanged, waiting for checkpoint A** | — |
| D6c | already compliant | "D6c: …" (exponent 0, trigger tag, trigger error) |
| D6d (RV81-N2) | **implemented**: `legacy_source.work_ref` reference check now G5 ATTEMPT (was WORK) | "D6d/RV81-N2 …" |
| D7 | already compliant | "D7: …" |
| D8, D9, D10 | not phase 1 (I62/Python) | — |
| D11 harness format | **implemented** in the TypeScript harness: only `rehash:"all"`; array removal splices | "D11/RV78-N5 …" |
| D12 N11 | already follows native code: group index kept, refused terminal, no attempts | "D12/N11 …" (`nativeRuns` unit) |
| D13 theta = +0 on a no-data body | already compliant | "D13/RV81-M13 …" |
| D13 Ceiling after a p128 verification-solve failure | already compliant | "D13/RV81-M16 …" |
| D13 R3 with `both` | already compliant | "D13/RV81-M08 …" |
| D13 2^-988 switch, both sides | already compliant | "D13: the absolute bound switches …" (independent fraction oracle) |
| D13 strict bracket | already compliant | probes B_interpolation_* ×4 plus the bracketed control |
| D14 | **implemented**: `@internal` on `eHat`, `phi512`, `stopFeasible`, `nativeSchedule`, `nativeRuns` (new export), `ordinaryAttempts` and `accountingRules` | "D14: no non-test module imports …" |
| D15 | nothing TypeScript-specific | — |
| RV81-M07 (N17 scope precedence) | already compliant | "RV81-M07 (N17) …" (`nativeRuns` unit with lowered limits) |
| RV81-M12 | already compliant | "RV81-M12 …" (empty roster against an empty inventory gives G3) |

`OUTCOMES.json` lists all 44 reader-local relation tests: 23 probes and 21 reader-local tests.
- On the pre-repair reader, 9 failed and 34 passed. The harness test was added after that run.
- Each failure is a repaired relation; each pass is a compliance now pinned by a test.

**RV81 mutants:**
- **Targeted by the new tests:** M02, M03, M04, M05, M06, M07, M08, M12, M13, M14 and M16. I did not re-run RV81's `mutants.py`, so each kill is by test design, not observed.
- **M11** (the G8 member-sequence check weakened) would still survive: the rule now fires first at G3, so the G8 copy is only a backstop.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_01 | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` (06d, after the reader changes) | 271/271 |
| tsc_02 | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| vitest_03 / vitest_08 | the new tests, during the work | all passing |
| vitest_09 | the 43 repair tests that existed then, run on the pre-repair reader. A temporary copy exported `nativeRuns` for this run only; the final reader was restored and verified (240ccf4532). | 9 failed, 34 passed |
| probe_05 | raising line for every 06d mutation on the final reader (temporary probe block, removed) | 178/178 at the expected gate and code. Raise sites moved only by line shifts, the `nativeRuns`/`nativeClass` split, and native WORK now raised at the end of class 1 |
| **vitest_12 (final)** | same as vitest_01 | **315 passed, 0 failed**: all 178 mutations, 18 must-pass entries and 15 cases of 06d, the earlier tests, and the 44 relation tests |
| **tsc_13 (final)** | same as tsc_02 | **exit 0** |

All shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Remaining known differences from the other readers

Python and Rust are mid-repair (I62 phase B, I63 phase 1), so this list compares each reader with the rulings, not with their current bytes.

1. **Waiting on checkpoint A, by ruling.**
   - **D6a:** TypeScript keeps "every listed diagnostic names the case" (Python 06d does the same).
   - **D6b:** TypeScript excludes `not_assessed` for selected cases.
   - **D1:** the empty-CaseSource-inventory rule.
   - **D8:** R1′–R4.
2. **Unruled TypeScript choices** (none is observable on 06d):
   - **(a) Class-1 structural crash.** A non-gate exception inside the native class (for example a malformed native graph) reports WORK if a native WORK predicate already failed, otherwise ATTEMPT. Other readers map an uncaught exception to their G5 default, PRODUCT_ATTEMPT in Python. One convention could be ruled for all three.
   - **(b) G0 codes for the bundled-file checks.** The bundled table's field identity keeps FORMATION_MISMATCH. The new byte-hash checks use SOURCE_PRODUCER_CONTRACT_UNSUPPORTED, as Python does. Neither can be reached from a receipt.
   - **(c) D1's "any CaseSource".** TypeScript compares the unsourced complete old list with every CaseSource's member count. With one model these are all equal.
3. **Until the other readers land their phase-1/B repairs,** RV78's probes still separate them from TypeScript:
   - Python: R1a/R1b, R3b, R4/R5, T1–T4a, T4d and T4e;
   - Rust: R2b, R6a, T1 and T2.

   TypeScript now gives the contract reading on every RV78 probe in its scope.

## Files read in this grant (sha256)

| sha256 | File |
|---|---|
| 230e16cb8818e0d49e0eacf38732e31cc74ba1b4599f3a8ea9d364d6d28e25e9 | T3/ROOT_RULINGS_V1.md at d566e487f4 (the "Reader review RV78–RV81" ruling, D1–D15) |
| 4e9e4c0ddebe4f57c8e88c00c2ee89598be6f08957f533dba6d5fb29b0f00565 | R/BRIEFS/I62_I64_REVIEW_REPAIR_07.md (all of it) |
| 15d23fc25fb148890435733a1751decf63afc10296666bfccd70788c58e8923a | R/REVIEW_RV81/reader_review_01/REVIEW.md (all of it) |
| 731d683c320b47362a67c745c27a207cf06ddb45d16a066b423ea5164145e502 | R/REVIEW_RV78/reader_review_01/REVIEW.md (the TypeScript-relevant rows: B1, S1, S2, N5 and the R-/T- table) |
| d559ef26b8077e3c0589c33a64e4b961f2ead52a926b078422196d3e858aac97 | R/REVIEW_RV78/reader_review_01/PROBES.json (all 31 probes; 23 embedded) |
| 55736ea65aee641fb22288d869632c5ea30f8c016ce56307a974ea598826192f | READER/P/core/analysis_runs/retained_precision.py (G0 block, lines 1366–1388 only, for the D2 field list) |

RV79 and RV80 were not opened. Their cross-reader relations reached me through the consolidated ruling.

## Bulk (WT/scratch/i64_review_repair_07/)

| sha256 | Bytes | File |
|---|---|---|
| 7c802881a280385926930a90e624fb60a65069100268bdffcd3a77eb84d18845 | 103060 | before/retainedPrecision.ts |
| 654caced2ead914297b0d26c72ab1a6735924396ca7f17ed18897c2fca408a1f | 28872 | before/retainedPrecision.test.ts |
| 240ccf4532104f559f51029d08332b0f827d9ad4f1713a44334fce1f1cb02ade | 107200 | retainedPrecision.after.ts |
| 03aa2d347d7dfd8f976242f59f1bca015f283cb1e99ef007234f7f671a813fdd | 62142 | retainedPrecision.test.after.ts |
| 9f10717d1914ff76a2f2eb01fa70aba31abe60e5bbecd069df4951559e535742 | 19937 | i64_repair07_reader.diff |
| a18cd4ef72e5b0e0ce92bdab017e4a49f16473d9b7ec6906b42e1b289a11387b | 35583 | i64_repair07_test.diff |
| d559ef26b8077e3c0589c33a64e4b961f2ead52a926b078422196d3e858aac97 | 285588 | rv78_PROBES.json (copy) |
| 390b363beb05abf46eacca961dfea0a3a539bbcff0742a7a721ff0b6da11443a | 16697 | rv78_probes_embedded.json (the 23 embedded probes) |
| cb99b80170edee2a1d2f42b4582519d76dc20876eb4cb3738fb770f2344cc1d2 | 26266 | probe_05.txt |
| 9e6ce7e735c5058bdfe4c75a6a352e9341fe213523f8a5f5d0cd1e490fc44362 | 338 | probe_05.log |
| 003456c9bd13c8e191d19f00c62452ceb92c236ce7136ec621e44af8e668e49e | 9317 | vitest_09_new_tests_on_before.log |
| 06170f78ee5c0975ab9868efd5883395f3e76108e97ca6dc9f4b1b48eafc5485 | 560 | vitest_12_final.log |
| 2648892224fb1267c962e77b4c6697b079d1e25ca9fed05d305ebb4696e97a0b | 112 | tsc_13_final.log |
| 3cac2c2f5271183105d16a03ced661a446bcb0b08294f17aaf8fe0c1672acbcd | 259 | final_hashes.txt |
| d89133f18c72b49137ab11fe934d380f7a95231c09746fa56b64a26164e1145d | 440 | vitest_01.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_02.log |
| 9e086f0119e52c1bdbf5cfcc6184d8aa43de97b89984412b4fbccb6e1c6e964d | 440 | vitest_03.log |
| deef73e170e9aa532520fd73bf3f3fa1c644cb7999c5ebeffb39f2ebb2182381 | 336 | vitest_08_n11.log |
