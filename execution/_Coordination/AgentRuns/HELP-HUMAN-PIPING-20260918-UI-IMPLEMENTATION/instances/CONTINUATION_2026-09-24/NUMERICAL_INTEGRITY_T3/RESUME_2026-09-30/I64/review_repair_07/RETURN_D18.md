# I64 return: ruling D18 (positive G5b section echo terms)

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This grant was sent by ROOT's mid-run message in the same harness-native subagent session. I64 had no descendants.

- **Run:** 2026-10-04T00:19:29Z to the final checks at 00:21:12Z; this file was written about 00:24Z. Inside the 30-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, build, Cargo or native job.
- **Other authors:** the shared files and I62's and I63's files were not touched.
- **Basis:** the ruling "Phase 2 returns; D18 section truth includes positive echo terms" (NUM 253ac9404e). ROOT_RULINGS_V1.md was read at sha256 `6300b61792e93c9f3d0cc3b15bfb95de39dd6c1b8520357c97ded456909d8fe8`.

## Changed files (READER at babcce075e, inside the fence)

| File | Before (babcce075e) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 294ba19b8afaf388281049f8341fed76d1e318f94f2f08cc50cd22f1a04f1263 | b92f29250880393cda55a1bc49de81abcded7da825906ff6716543c06da5ef0e (111823 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | bf2ad6d593770eff5ffd1e9fbad539946dbe492d2cdbb9256a134066ab1a833e | 4a883f07714fe761358afdaeaff682ba998f036dc3e93c9affe507f082aa2751 (68525 B) |

## Changes

1. **Positivity restored (D18).** At G5b, each of the five echoed section terms (`area`, `section_modulus`, `length`, `axial_stiffness`, `torsional_stiffness`) must equal the source's term and be positive. Otherwise the check fails with G5b SECTION_MISMATCH.
   - The code comment now cites D18 and its native basis: PP:2442 and 2462, PP:2360, and endpoint_maximum.rs:128.
2. **The other phase-2 removal stays removed:** the G5b duplicate of the member-property identity (`area = A_K`, `Iy_K = Iz_K`, …). G8 still enforces it.
3. **Tests.** A reader-local test, "D18: …", checks two things:
   - a zero in each of the five terms, echoed identically in source and selection, gives G5b SECTION;
   - a positive term that differs from the source gives G5b SECTION. Its first draft set length to 1.0, which equals the base value and so was a no-op; it now uses 2.0.

   No phase-2 test comment mentioned the old SCALE reading, so none needed rewording.

## Runs

- **On snapshot 07** (corpus `90f6e4ed9b`): the first run gave exactly the one mismatch ROOT expected. `g5b_zero_section_area` observed G5b SECTION against the old SCALE expectation (vitest_01). In that same run, the new D18 test failed only on its no-op echo case, fixed as above.
- **07a appeared mid-run.** READER's working-tree corpus changed to `a6fa398731c35245baca098e322a9846399c7535b2d4cd882a5058010de456c9`. It was I62's 07a installation, still uncommitted when I checked (READER head babcce075e). It has 15 cases, 236 mutations and 19 must-pass entries:
  - `g5b_zero_section_area` now expects G5b SECTION;
  - `g5b_zero_section_length` is new, expecting G5b SECTION.

| Run | Command (from READER/P/apps/desktop) | Result |
|---|---|---|
| vitest_03 (on 07a) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 379 passed |
| probe_04 (on 07a; temporary probe, removed, with the test file restored byte-identically) | — | **236/236 mutations** at the expected first gate and code (G7 per reader); **19/19 must-pass** pass with the base classifications |
| **vitest_05 (final, on 07a)** | same as vitest_03 | **379 passed, 0 failed** |
| **tsc_06 (final)** | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | **exit 0** |

The final run's shared hashes were identical before and after: schema `07951edacf`, corpus `a6fa398731`, definition, table and inherited table (final_hashes.txt). **If I62 changes the 07a corpus after `a6fa398731`, ROOT should rerun.**

## Remaining known differences

These are unchanged from RETURN_P2.md:
- the fail-closed fallback code (accepted);
- D10 integral floats (Python-only);
- the bundled-file G0 codes (unreachable from a receipt);
- the D1 sourced id comparison (first gate only, for a malformed map);
- the explicit G1/G2 guards.

D18 adds no new difference.

## Bulk (WT/scratch/i64_review_repair_07_d18/)

| sha256 | Bytes | File |
|---|---|---|
| 294ba19b8afaf388281049f8341fed76d1e318f94f2f08cc50cd22f1a04f1263 | 111766 | before/retainedPrecision.ts |
| bf2ad6d593770eff5ffd1e9fbad539946dbe492d2cdbb9256a134066ab1a833e | 67597 | before/retainedPrecision.test.ts |
| b92f29250880393cda55a1bc49de81abcded7da825906ff6716543c06da5ef0e | 111823 | retainedPrecision.after.ts |
| 4a883f07714fe761358afdaeaff682ba998f036dc3e93c9affe507f082aa2751 | 68525 | retainedPrecision.test.after.ts |
| 98329897b680cfe2afaeaea67408e6447d92fbc438456d011febb1ca746c6d9b | 1687 | d18_reader.diff |
| b43a71f470c73efb0780e5688512c8ae2524a43205479dd4107d147bbe306610 | 1570 | d18_test.diff |
| 3cfe4a652a5a467863b4b65619649e3aac71eb34a243e7525b8c84b8a693b458 | 3145 | vitest_01.log (on 07: the expected single mismatch plus the no-op echo case) |
| d7d472f19557a4bdda547e875ee5938d28aec073d7b15ebd3e58f526e6351514 | 458 | d18_probe.json (diagnosis of the no-op echo case) |
| 91591e28d4f59fd1a73cc196fdb6ff214001dbbbcdb394b8ebd0f67f2a2bd317 | 337 | d18_probe.log |
| 609fe81b98c8897b1660914889feb24a36a84083e59d3f13287b17305b74425d | 440 | vitest_03.log |
| 2dabe7023c23eeb49a75132bd201baf515ba048fc0c6adcab9b11032ffcf6ed0 | 18426 | probe_04.txt (07a outcomes) |
| ca9e7458a2de810b1964606ca0f5cc7096f1d8676d85edf27730e2dfd9af41c3 | 338 | probe_04.log |
| 131f6c03b7e4f787c9a2bbed180e8a4b964fd5a6481c23dc8982da78dabfd8c2 | 560 | vitest_05_final.log |
| 7a1f267a449c9f49223490d142184f4d1936c71f995e5e6eaca56f512994b0e6 | 112 | tsc_06_final.log |
| e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | 0 | tsc_02.log |
| 9591d1c6fed4de9419763445fc909e88e9ddf54ff5411eb7e948d5867144e492 | 259 | final_hashes.txt |
