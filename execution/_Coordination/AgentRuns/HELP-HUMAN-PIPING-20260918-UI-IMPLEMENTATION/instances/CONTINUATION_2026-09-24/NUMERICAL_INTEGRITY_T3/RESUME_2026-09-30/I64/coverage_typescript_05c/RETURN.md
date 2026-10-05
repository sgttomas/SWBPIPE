# I64 return: TypeScript reader aligned to snapshot 05c

I64 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). This was its third grant, sent by ROOT's mid-run message in the same harness-native subagent session. It had no descendants.

- **Run:** 2026-10-03T21:01:14Z to the final checks at 21:03:13Z; return written about 21:06Z. Well inside the 45-minute box.
- **Host:** the M5 host. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations, and Git reads used `GIT_OPTIONAL_LOCKS=0`. No install, build, Cargo, solver, native, UI or DEC-025 job. The runs used the existing `node_modules` link and WASM assets as they were.
- **Other authors:** the shared files and I62's and I63's files were not touched. I63's `retained_precision_contract.rs` showed as modified in READER; it is not mine.
- **Paths** use the brief's placeholders.

## Changed files (READER at e510266332, inside the fence)

| File | Before (= ee90efd18d) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | 4ce47b8a89471688bc8b70cc8327ce8edfa7a57f9891f46cd8ca86ed41f247e2 (92606 B) | cbc71d9b6a543f329f4daa2dddf9d12e2a26fa1bbf2e65badff567bed117df5b (93026 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | c643e8734da08a6157b7752ad97e68b49556ec2b67b1ddedb8dc5daf29bd2395 | unchanged (byte-identical) |

**Shared files, verified at the start and unchanged at the end:**
- corpus `85bff98ea76e5d3433076d12593c91bc49350cc374981db259259282926e3824`;
- schema `f943ebd351…`;
- definition `3e0779a45a…`;
- table `c74742ce6a…`.

## Changes

Eligibility stays held (`SUMMARY_COVERAGE_COMPLETE = false`).

1. **Record body order** (`recordCoverage`, used for both selected and unavailable attempts): the verification record's `resolution` and `theta` entries must list bodies exactly 0..n−1, in order. Failure is G5a SCALE_MISMATCH.
2. **G5a prerequisites:** every body must have at least one node, and every row's normalized value must be finite. Failure is G5a SCALE_MISMATCH, ahead of any G5b or G5c use.
3. **Gate-major order across cases needed no change.** The reader already runs G5a for every selected case, then the unavailable-attempt G5a checks, then G5b for every case, then G5c for every case. The same holds for the earlier gates, each of which runs over all cases or attempts before the next gate starts. The two cross-case pins pass at G5a.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_00 (baseline: the ee90efd18d files on 05c) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 188 tests: 186 passed, 2 failed (`unavailable_record_resolution_duplicate` and `unavailable_record_theta_duplicate`, both the record-order pin) |
| vitest_01 | same | 188 passed |
| probe_02 | a temporary probe block recording the raising line for every mutation; removed, with the test file restored byte-identically | 121 of 121 |
| vitest_02 | a trial TypeScript-only control: a non-finite normalized value made by switching a row's unit to MPa with value 1e305 | Not expressible. The checked canonical JSON refuses the out-of-profile number during rehash. The control was withdrawn, so the test file is unchanged. |
| **vitest_03 (final)** | same as vitest_00 | **188 passed, 0 failed** |
| **tsc_04 (final)** | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | **exit 0** |

The final runs used reader cbc71d9b6a and tests c643e8734d. All four shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Against the bar (snapshot 05c)

- **All 121 mutations** produce their expected first gate and code. `MUTATION_OUTCOMES.json` has the raising line for each.
- **Raising lines for the 17 mutations new since 05a:**

  | Mutations | Raising check |
  |---|---|
  | `two_body_swap_has_data` | the B list |
  | `two_body_swap_has_data_with_rosters` | the direct data fact on body 0's free loads |
  | `two_body_swap_stop_only` | the stop list |
  | `two_body_body_order_swapped`, `two_body_missing_body` | G3 inventory |
  | `two_body_resolution_order_swapped` | the existing G5a body-list check |
  | `p512_floor_not_phi` | **G5b, the Φ line** |
  | `p512_positive_floor_forces_stop`, `p512_zero_floor_body_given_positive_floor` | feasibility |
  | `p512_charge_follows_estimate` | the charge list |
  | `p512_floor_null` | the existing p512 floor-presence check |
  | `copy_flags_into_loaded` | the direct data fact |
  | `rebind_source_run_only` | G5 product association |
  | `cross_case_gate_order_selected` | case 1's G5a roster, ahead of case 0's G5b |
  | `cross_case_gate_order_unavailable` | the unavailable-attempt G5a data fact, ahead of case 0's G5b |
  | `unavailable_record_resolution_duplicate`, `unavailable_record_theta_duplicate` | the new record-order line |

- **All 16 must-pass entries pass,** with the base classifications and `numerical_eligible=false`. They include `p512_charge_follows_stop_zero_floor_body` and `cert_failed_after_summary_accounting`.
- **All 9 cases validate** with their expected classifications. The real p512 ladder base passes the G5b Φ equality.
- **All earlier tests pass,** including the TypeScript-only Φ-rounding and exhaustive feasibility tests and the seven snapshot-04 controls.
- **tsc** exits 0.

## Notes for ROOT

- **No expected outcome looks wrong.** I found no defect in 05c.
- **The non-finite normalized-value rule cannot be expressed in the corpus.** The checked JSON profile refuses a number large enough to overflow on normalization. The rule is in the reader, but no corpus mutation or TypeScript-only test exercises it.
- **The empty-body prerequisite** is likewise not exercised by any shared mutation.
- **The removed must-pass entries need no TypeScript follow-up.** No TypeScript test depended on them, and TypeScript has no equivalent of the I58 Python-only `coefficient_range` test.

## Files read in this grant (sha256)

| sha256 | File |
|---|---|
| 7d9cd76a9e48d2f17adf893a12d5efa9368c38d4ee6b2bb2e781004b1ad9f87a | R/I62/coverage_shared_python_01/RETURN_C1B.md |
| fb77ba8511d72cdb77693e6bb76ae03537b242f5a9c4223eb64c3b195eff324c | R/I62/coverage_shared_python_01/RETURN_C1C.md |
| 63df061ad04980ed1cd761e20f611e2d3708c3ef4ad0b00e2857c226ff688a6a | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_05B.json (summary fields) |
| a3791af2b909083ed13a4bf1199439ded7acd968ff9adbea5bc041aaacf6a4d4 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_05C.json (summary fields) |
| 6365d00ba9b11babd55ad1566196411cb068fae548d24937ab898fbef273961f | T3/ROOT_RULINGS_V1.md at NUM 6bebd01f15 (sections from "Rust and TypeScript aligned to snapshot 05a" onward, searched for TypeScript items) |
| 3b12ca7511c698143b77af4058b7e41932026714453dad97240031183ed87994 | READER/P/core/analysis_runs/retained_precision.py (Python reference; lines 625–650 and 702–760 only, for parity) |

I also used the corpus at 85bff98ea7 (cases, mutations and must_pass) and the fenced files at their ee90efd18d bytes.

## Bulk (WT/scratch/i64_coverage_typescript_05c/)

| sha256 | Bytes | File |
|---|---|---|
| 4ce47b8a89471688bc8b70cc8327ce8edfa7a57f9891f46cd8ca86ed41f247e2 | 92606 | before/retainedPrecision.ts |
| c643e8734da08a6157b7752ad97e68b49556ec2b67b1ddedb8dc5daf29bd2395 | 21063 | before/retainedPrecision.test.ts |
| cbc71d9b6a543f329f4daa2dddf9d12e2a26fa1bbf2e65badff567bed117df5b | 93026 | retainedPrecision.after.ts |
| 505a9fccb2a42549d08d3ce57ff61f22be6122583d8d7cddbe2e22ad2a1ba5f4 | 2025 | i64_05c_reader.diff |
| d106660334db996a970ce4c32b8eef8dce2515434826e561f0487f35882b4e72 | 16023 | probe_02.txt |
| 2479566e9b835bbd34dc4a12e1667f0c72acdeeeba862666e761ff547aabf6a0 | 338 | probe_02.log |
| 6e7ad68b1b2ee540fda0b67ec231535dc01a611a5788f3eaa2db6ca8b9430339 | 2131 | vitest_00_baseline.log |
| 7af15110e15cc0e0593aa765935a211c4b9ffa704d2017fcaea81c5c2756431e | 440 | vitest_01.log |
| 428b870b170b9b3737aad64f86f83e51b83c53367886e61e2b78d9fc16079e8b | 1632 | vitest_02_probe_nonfinite.log |
| 96e801ddc69fa3e6000ca99a10a891f557ec20d78fd7487d8c652a53eaec7b1d | 560 | vitest_03_final.log |
| 22b5cefb12a612a94b10820722b4d84ead640bbec3e47379476f27127c69c177 | 112 | tsc_04_final.log |
| 263081e4babc13b1b4c61ed30f84233ef8690d98a750cbf4a7ff5a11cdbcd5a2 | 237 | final_hashes.txt |
