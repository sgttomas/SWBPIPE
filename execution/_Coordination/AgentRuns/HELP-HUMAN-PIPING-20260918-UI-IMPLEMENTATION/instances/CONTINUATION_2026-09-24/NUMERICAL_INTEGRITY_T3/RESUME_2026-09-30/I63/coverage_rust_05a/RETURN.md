# I63 return: the Rust reader aligned to snapshot 05a

I63 is a TASK (Type 2). ROOT (HELP_HUMAN) granted this work directly in the session as a follow-on to I63's snapshot-04 work, committed unaccepted as READER `4cc1b664c2`. The basis is the rulings at NUM `f870b10ca7`, under the same fence, command, target and rules as `BRIEFS/I63_I64_COVERAGE_READERS.md`. I63 had no descendants.

- **Run:** first tool call 2026-10-03T20:36:05Z; freeze about 20:45Z, inside the 60-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native, UI or DEC-025 job; one Cargo job at a time, each under a 1,200 s wall.
- **Basis files:** READER at `ccdfd04fd7`. NUM was at `059e876617` when hashed.
- **Paths** use the brief's placeholders.
- **Status:** all 05a checks pass. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`).

## Shared files verified at start

All five files match SHARED_SNAPSHOT_05A (`fed637869f`). Its SHA256SUMS_C1A verify.

| File | sha256 prefix |
|---|---|
| corpus | `159ef78c47` |
| schema | `f943ebd351` |
| definition | `3e0779a45a` |
| preview table | `c74742ce6a` |
| results yaml | `4585a45fcf` |

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before (`4cc1b664c2`) | After |
|---|---|---|
| src/retained_precision.rs | ba8a08b590… | 93fee7bd095b18ca1b67074b71d80e56483a0f9358ee19ea711c86dcde0592dd (137032 B) |
| tests/retained_precision_contract.rs | 3b9e6f9029… | 09eea1995cc03ef4cc79107ad694db78d766e74653fc4415bb680635af0bf88d (52968 B) |
| src/lib.rs | 375b073135… | unchanged |

### Source changes

1. **G5b Φ.**
   - The p512 floor equality `floor == bits(phi_512(e_hat(E, L)))`, failing as G5b SCALE_MISMATCH, **already existed** in the inherited Rust g5b. The C1a RETURN says Rust had no Φ check; that is not right.
   - It now calls two new public helpers, `rp::e_hat` and `rp::phi_512`, which mirror FK/verify.rs:321–334 and 365–376. They use the native constants `0x2490…` (2^-438) and `0x5B50…` (2^438) in place of `powi`.
   - The semantics are unchanged.
2. **G5a for unavailable attempts that keep a complete roster.** G5a now walks the cases in order:
   - a selected case runs `g5a_selected`;
   - an unavailable case whose attempt has non-null coverage runs `coverage_g5a(…, sel = None)`.
   
   `coverage_g5a` is the selected path's coverage logic, now shared by both paths. Without a Selection:
   - p comes from the Run's last attempt, and E and theta come from its 2p verification record;
   - it applies the canonical layout, the per-body extent, the feasibility rule (at p512, floor positivity is `phi_512(ê) > 0` from the record), the record bound/theta/data_blocks relations and the direct data facts;
   - it applies no Selection rosters (stop, estimate, charge, B).
   
   A new `body_extent` helper holds the existing extent code.
3. **Unchanged:** G3, G5, the selected-case order and the codes. The p128 ladder start was already enforced at G5 ATTEMPT_MISMATCH.

### Tests

- **`shared_must_pass_entries_validate`:** loops over the 15 `must_pass` entries. Each must be admitted, invocation-bound, not eligible, and carry exactly the base case's classifications.
- **`snapshot_05a_mutation_outcomes`:** mutations 77–103, with the tally and a printed table.
- **`p512_floor_phi_follows_native_rounding`:** seven Φ vectors and three ê vectors.
  - The Φ vectors cover zero, exact scaling, MAX, underflow to the minimum subnormal, two subnormal ties (one not below, one below), and the C1a below-nearest case.
  - Each expected value was derived independently as the least binary64 ≥ e·2^-438, using an exact rational oracle.
- **`snapshot_04_coverage_mutation_outcomes`:** now asserts 104 mutations in total and tallies the slice 30..77.

## Commands and results

Every run used the brief's command and environment variables, plus the deviation below from run4 on. Logs are listed under Bulk.

| Run | State | Result |
|---|---|---|
| run1 | baseline on 05a | 9 passed, 2 failed: 4 unavailable-attempt G5a mutations were missed, and the snapshot-04 count assertion failed |
| run2 | first restructure | compile error (type inference), fixed |
| run3 | fixed | **link failed: the host's Xcode licence was no longer accepted** (`xcrun` exit 69); see the deviation |
| run4 | with process-scoped `DEVELOPER_DIR` | 10 passed, 1 failed (only the old 77-count assertion) |
| run5 (**final, full command**) | final bytes | **14 passed, 0 failed** |
| run6 | final bytes, `--nocapture snapshot_0 shared_must_pass` | 3 passed; tables captured |

**Against the bar:**
- **Mutations:** all 104 match their expected first gate and code. That is 30 from snapshot 03 plus 47 from 04 plus 27 from 05a. The 05a tally: G5a 16, G5 PRODUCT_ATTEMPT 5, G3 3, G5 WORK 2, G5 ATTEMPT 1.
- **must_pass:** all 15 entries validate with the base case's classifications.
- **Cases:** all six validate with their expected classifications, and none is eligible.
- **Earlier tests:** all 11 still pass, so the run has 14 tests in all.
- **The four previously missed mutations now fail at their intended unavailable-attempt checks, by code reading:**
  - `unavailable_no_data_claim_with_free_loads` at the direct data fact;
  - `unavailable_stop_uncoupled` at feasibility;
  - `unavailable_layout_relabelled` at the canonical layout (it had reached G8);
  - `unavailable_verification_bound_duplicate` at the record bound.
- **Tables:** `OUTCOMES_05A.json`.

## Deviation (host)

- **What happened.** Between run1 (20:36:23Z) and run3 (20:39:53Z), the host's Xcode licence became unaccepted, presumably after an Xcode update. `cc` therefore fails at link time: xcrun exits 69 with "You have not agreed to the Xcode license agreements". Compilation was unaffected.
- **What I did.** I did not accept the licence: that needs the owner and sudo. From run4 on, I set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` for the cargo process only.
  - These are the Command Line Tools already installed on the host, with their MacOSX SDK.
  - No system setting, `xcode-select` state or installation changed.
  - Rust code generation is unchanged; only the SDK the linker uses differs.
- **For ROOT:**
  - Other Cargo users on this host, such as I61, will hit the same link failure.
  - The owner should accept the Xcode licence, or ROOT should rule on the `DEVELOPER_DIR` workaround.

## Open items for ROOT

- **The Φ check is unexercised by the corpus** until C1b's p512 base. Only the unit test pins it now.
- **L = 0 and absent kinds** are still not exercised by the shared corpus. Per the ruling, readers' own tests cover them, but I63 has no L = 0 feasibility test yet; this is a candidate for the next grant.
- **Cross-case G5a/G5b order (parity note, not in the corpus):**
  - Rust runs G5a for all cases before G5b for any.
  - Python runs G5a and then G5b per selected case, in case order.
  - So a selected-case G5b defect plus a later unavailable-case G5a defect gives G5a in Rust but G5b in Python. A shared pin would settle it.
- **Record resolution and theta for an unavailable attempt:** Rust requires the bodies in order `0..n-1`; Python compares only the sorted set. They differ only on a malformed record.
- **Not claimed:** acceptance, eligibility, three-reader parity or independent review. I63 ran no Python or TypeScript.

## Files read (sha256)

These are in addition to the snapshot-04 RETURN's list.

| sha256 | File |
|---|---|
| 2760b5553967834b7fb835eb2b9a7fe813729f2e5474e8376cdd818e47f065a4 | R/I62/coverage_shared_python_01/RETURN_C1A.md |
| fed637869f4111442a55fe5afb3b0e9ad8c9a77fbd97902292d7a653b1ff8a22 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_05A.json (keys, files, must_pass, deferrals) |
| 85da9832897f1787a957234de8b0c3e70207f54e92236757eed816a3d1ea4328 | R/I62/coverage_shared_python_01/SHA256SUMS_C1A |
| 989a9b916d0c23187a2702a1028ace7a368a6bab66248a083345c6634ba96b00 | T3/ROOT_RULINGS_V1.md (the 05a-verified and RV77 sections) |
| 66022cc78bb5779d021a35e33fafc7cc7bc2d64723a5d0fdaf3a48cb6fea795e | NUM/P/core/solver/frame_kernel/src/structural/retained/verify.rs (310–376) |
| 6a2fc382bf8cae0502c41030da8ac9bc0cfe1f1b80b7aceac60ff7e40f345eda | NUM/P/core/solver/frame_kernel/src/structural/retained/adaptive.rs (304–349, 2280–2305) |
| 27fc1797c2fecb9fc27dc1c99facdb8a260fa98ab725e6113caa616185cefc0f | READER/P/core/analysis_runs/retained_precision.py (05a coverage sections; read only) |
| 7eab5b3793a504755313d3f3fb7236c4448f5a9f901112622f4734e90db3b5c9 | READER/P/tests/test_retained_precision_contract.py (Φ and must_pass tests; read only) |

## Bulk (WT/scratch/i63_coverage_rust_05a/)

| sha256 | bytes | file |
|---|---|---|
| 436556c6e7cc0fa414dfdace977212b46d9e94658b3df04769ec26ae6620c959 | 22717 | I63_05A_DELTA_src.diff |
| bfce0b3787477a695fd34ffdf7ebcadec8fd4feb1a110494d93f8e52098d56dc | 7230 | I63_05A_DELTA_test.diff |
| 43a2f74f82fd6e194cda700a2edde92aa8c7278c860389a01015149f3eb06336 | 2861 | run1_baseline.log |
| 1c5d2c46e798587b7f3d4995d857278f7f6f551f8fed42cfcafa0bb81a116104 | 1425 | run2.log |
| 6597e6ac6990d99c53ab36fa1ce08176a8823fb7c74773b5482bfca32f34afb1 | 3258 | run3.log (link failure) |
| 55e6932fabbe780c02245176db8c5840dd0a8955fd81c60cab178471a2e97ff0 | 2019 | run4.log |
| 8929b28bd64ef4a14140475b661fbb17750ef103767fe998485ca93d97252da0 | 1707 | run5.log (final) |
| e04886193e19fdc24461e75e2c01ce2a24ab98eeac6c77bb64cc3d37967f4821 | 19724 | run6_outcomes.log |
| ba8a08b590114212e0a12f5de65471ae0dc1aa462d80c8d2f1050965bbdc1f7c | 133707 | before/retained_precision.rs |
| 3b9e6f90292848c61301da28bc201784e21bb8f0de2639e1a62efbada24d7f6d | 46903 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| 93fee7bd095b18ca1b67074b71d80e56483a0f9358ee19ea711c86dcde0592dd | 137032 | final/retained_precision.rs |
| 09eea1995cc03ef4cc79107ad694db78d766e74653fc4415bb680635af0bf88d | 52968 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
