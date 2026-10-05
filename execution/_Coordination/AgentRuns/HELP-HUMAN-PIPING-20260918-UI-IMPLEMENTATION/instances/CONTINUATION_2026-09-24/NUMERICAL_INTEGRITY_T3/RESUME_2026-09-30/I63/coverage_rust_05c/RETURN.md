# I63 return: the Rust reader on snapshot 05c

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session. The basis is the ruling "Snapshot 05b verified; three non-native must-pass entries retired as 05c" (NUM `1586bca12e`), under the same fence, command, target and rules as `BRIEFS/I63_I64_COVERAGE_READERS.md`. I63 had no descendants.

- **Run:** first tool call 2026-10-03T21:01:13Z; freeze about 21:04Z, well inside the 45-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native, UI or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Basis files:** READER at `e510266332`. NUM was at `6bebd01f15` when hashed.
- **Paths** use the brief's placeholders.
- **Status:** all 05c checks pass. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Shared files verified at start

All five files match SHARED_SNAPSHOT_05C (`a3791af2b9`). SHA256SUMS_C1B and SHA256SUMS_C1C verify.

| File | sha256 prefix |
|---|---|
| corpus | `85bff98ea7` |
| schema | `f943ebd351` |
| definition | `3e0779a45a` |
| preview table | `c74742ce6a` |
| results yaml | `4585a45fcf` |

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`706c8558f2`) | After |
|---|---|---|
| src/retained_precision.rs | 93fee7bd095b18ca1b67074b71d80e56483a0f9358ee19ea711c86dcde0592dd | **unchanged** |
| tests/retained_precision_contract.rs | 09eea1995cc0… | 4abc640ca71ee17388da85215ca98f22f6b40592b783a7192e9e57faefbdc7b3 (54664 B) |
| src/lib.rs | 375b073135… | unchanged |

**No reader change was needed.** At baseline, the 05a reader already gave every one of the 121 mutations its expected first gate and code, and validated all 9 cases. The only failures were the count assertions in I63's own tests.

The test changes:
- the snapshot-04 test asserts 121 mutations in total;
- the 05a table covers the slice 77..104;
- the must-pass loop asserts 16 entries;
- a new `snapshot_05b_mutation_outcomes` covers mutations 104..121, with the tally (G5a 13, G3 2, G5 PRODUCT_ATTEMPT 1, G5b 1) and a printed table.

## Commands and results

Every run used the brief's command and environment variables plus the disclosed `DEVELOPER_DIR` (see Host).

| Run | State | Result |
|---|---|---|
| run1 | baseline on 05c | 11 passed, 3 failed. The failures are only the 77/104, 15/16 and 05a-tally assertions. `shared_rehashed_first_failure_mutations` (all 121) and the 9-case test passed |
| run2 (**final, full command**) | final bytes | **15 passed, 0 failed** |
| run3 | final bytes, `--nocapture snapshot_0 shared_must_pass` | 4 passed; tables captured |

**Against the bar:**
- **Mutations:** all 121 match: 30 from snapshot 03, 47 from 04, 27 from 05a and 17 from 05b.
- **must_pass:** all 16 validate with the base case's classifications, invocation-bound and not eligible.
- **Cases:** all 9 validate with their expected classifications.
- **Earlier tests:** all pass.
- **Specific checks:**
  - An empty body or a non-finite normalized value fails at G5a through `numeric_cases`/`body_extent`.
  - The p512 ladder base now exercises G5b Φ positively: Φ > 0 on body 0 and Φ = 0 on body 1.
  - `p512_floor_not_phi` fails only at the G5b `floor == bits(phi_512(e_hat))` line, because body_scales use the computed Φ.
- **Reasons, by code reading:**
  - `two_body_swap_has_data`: the B list.
  - `two_body_swap_has_data_with_rosters`: the direct data fact.
  - `two_body_swap_stop_only`: the stop list or feasibility.
  - Body order swapped or a body missing: G3.
  - Resolution order: the Selection list order.
  - The p512 positive-floor, charge and floor-null mutations: G5a. A positive floor on the zero-floor body fails feasibility before G5b.
  - `copy_flags_into_loaded`: the B list or data facts.
  - Rebinding source/run: the existing G5 association.
  - Both cross-case pins: G5a, because the order is gate-major.
  - The two record-order pins: the unavailable path's `0..n-1` resolution/theta order.
- **No expected outcome looks wrong.** No retired entry is referenced by any I63 test.
- **Tables:** `OUTCOMES_05C.json`.

## Host

The Xcode licence is still unaccepted, so every cargo run set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for the process only, as ROOT directed. No system setting changed.

## Open items

- **The 05b deferrals stand:** the Ceiling, L = 0 and the source-construction base. I63 still has no reader-local L = 0 feasibility test.
- **Not claimed:** acceptance, eligibility, three-reader parity or independent review. I63 ran no Python or TypeScript.

## Files read (sha256)

These are in addition to the earlier I63 RETURNs.

| sha256 | File |
|---|---|
| 7d9cd76a9e48d2f17adf893a12d5efa9368c38d4ee6b2bb2e781004b1ad9f87a | R/I62/coverage_shared_python_01/RETURN_C1B.md |
| fb77ba8511d72cdb77693e6bb76ae03537b242f5a9c4223eb64c3b195eff324c | R/I62/coverage_shared_python_01/RETURN_C1C.md |
| 63df061ad04980ed1cd761e20f611e2d3708c3ef4ad0b00e2857c226ff688a6a | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_05B.json (pins, correction, deferrals, the new-mutation table) |
| a3791af2b909083ed13a4bf1199439ded7acd968ff9adbea5bc041aaacf6a4d4 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_05C.json |
| c30b517161aa3fc1a4607e6ff289ebc356574b38b33f522646fd5add83b60d14 | R/I62/coverage_shared_python_01/SHA256SUMS_C1B |
| 273a3623a5c1796f0e062eb08e7cf3c5c81f5b778a5242297bbef09e67167d78 | R/I62/coverage_shared_python_01/SHA256SUMS_C1C |

## Bulk (WT/scratch/i63_coverage_rust_05c/)

| sha256 | bytes | file |
|---|---|---|
| eed346685f1ffea3727b41109fb304d3b618f2b4af5b73a88440b44bdce5955f | 3343 | I63_05C_DELTA_test.diff |
| 28afe695d355ecc22365bcaf87ce7fddf113df03c89a0afd1174cb68dd9bc350 | 14116 | run1_baseline.log |
| fb3e53ef88183935b2da30c88784f2063767f4898e544415692633a76c2687eb | 1750 | run2.log (final) |
| 37306168c29720d03fce3ed297278a0a93296a44e069a9b32ca22a105cf14a65 | 23900 | run3_outcomes.log |
| 93fee7bd095b18ca1b67074b71d80e56483a0f9358ee19ea711c86dcde0592dd | 137032 | before/retained_precision.rs (= final) |
| 09eea1995cc03ef4cc79107ad694db78d766e74653fc4415bb680635af0bf88d | 52968 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| 4abc640ca71ee17388da85215ca98f22f6b40592b783a7192e9e57faefbdc7b3 | 54664 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
