# I63 return: summary-coverage checks in the Rust reader

I63 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN) under `BRIEFS/I63_I64_COVERAGE_READERS.md` (NUM `e02f02a93d`). It had no descendants.

- **Run:** first tool call 2026-10-03T20:00:52Z; freeze 20:15Z, well inside the 90-minute box. The memory guard (PID 5387) was running throughout.
- **Limits held:** no Git writes or index operations; no install, new tooling, solver, native, UI or DEC-025 job. One Cargo job at a time, each under a 1,200 s `perl alarm` wall, on the brief's target. I61's worktree was not touched.
- **Paths** use the brief's placeholders WT, NUM, READER, P, T3 and R.
- **Status:** snapshot-04 implementation complete. **Not accepted; eligibility still held** (`IMPLEMENTATION_COMPLETE = false`). Snapshot 05 and a fresh independent review come first.

## Shared files verified at start (READER, read-only)

All match SHARED_SNAPSHOT_04 (`cb7aa0bbd7`, whose SHA256SUMS verify).

| File | sha256 |
|---|---|
| P/schemas/retained_precision_mp_v2.schema.json | f943ebd3511e31bcdfe09bac2788362d3a5734040bc3088fbe585361b571cd21 |
| P/fixtures/results/retained_precision_cases.json | 8e333e632cde3b70cb7fcce13d5f9eb724fbc22246c304a30dbdbefcd0cce760 |
| P/fixtures/results/retained_precision_prepared_ordinary_v1.json | 3e0779a45a74cf0bb3a4ed08ed3a6b44347aea8a3c33b59e9dd92130426ee296 |
| P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json | c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8 |
| P/schemas/results.v0.3.schema.yaml | 4585a45fcfd4aa522acac4843b099d11158b4c7d84f1b99870e9b0a378a05586 |

At start, the fenced files matched I59's SOURCE_FREEZE prefixes: `b43595136e`, `9a3b557780` and `375b073135`.

## Changed files (READER, inside the fence)

Paths are under P/core/reporting/result_export.

| File | Before | After |
|---|---|---|
| src/retained_precision.rs | b43595136e2cba5b813f4b82e19371c3fde422ee82bd8eb90c66f36a63446d17 | ba8a08b590114212e0a12f5de65471ae0dc1aa462d80c8d2f1050965bbdc1f7c (133707 B) |
| tests/retained_precision_contract.rs | 9a3b557780b9b14eba4d88e1206f38b4d7a66c545e7868591ed1998143cd70ac | 3b9e6f90292848c61301da28bc201784e21bb8f0de2639e1a62efbada24d7f6d (46903 B) |
| src/lib.rs | 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | unchanged (no wiring needed) |

## What changed, by gate

The gate order and error codes are unchanged. Earlier original checks still win.

- **G1 and G2:** no code change. The snapshot-04 schema drives the existing pinned shape and encoding walkers. They enforce the closed nullable member, the four booleans, the boolean has_data and the U body encoding.
- **G3**, in the existing product-attempt loop:
  - a non-null `summary_coverage` must list the attempt's source inventory exactly, as body ids `0..body_count-1` in ascending order;
  - that inventory must be non-empty and itself numbered `0..n-1`;
  - a null `source_ref` is left to G5.
- **G5**, new `g5_coverage`. It runs in the C3 association pass after the existing native schedule. Its failures come before the deferred C3 WORK pass.
  - Ready, a completed certificate, a passed certificate, a completed G5a or a passed G5a each require non-null coverage.
  - Non-null coverage requires all of the following:
    - the attempt's own source and Run: case `source_ref`, Run id, and Run origin `source_ref`/`owner_ref`;
    - a selected native Run;
    - lanes `[admitted_k, annular_source]`, both completed;
    - proof_start, projection, maxima, values and aliases completed;
    - an entered certificate (stage completed or failed, check passed or failed).
- **G5a**, `g5a` rewritten. The draft's conditional estimate/charge count check is removed, and the existing numeric checks are kept. It runs in this order:
  1. The existing ranges and entry checks, plus native p ∈ {128, 256, 512}, P = 2p, the floor rules, and the selected verification record at 2p.
  2. **The canonical layout, rebuilt in full** by `canonical_layout` from the bound source maps in native `recover::layout` order. It requires exact equality, and every prescription must be exactly +0, so D = false.
  3. Per body: the 16-vector feasibility rule. Floor positivity is ORed after the L≠0 coupling and independently of it, as in final_case.rs:1371–1448.
  4. Per body: the estimate and charge rederivation. Charge equals estimate below p512, and equals the force/moment stop bits at p512.
  5. The exact stop, estimate and charge rosters (items 1–2).
  6. The existing resolution numeric checks (item 3).
  7. The B roster ↔ has_data, with the record bound per body (null if and only if no data), theta +0 for no-data bodies, and the `data_blocks` rule (item 4).
  8. The direct data facts: no free DOF implies false; an individually nonzero nodal term at a free DOF implies true.
  
  No final row feeds a coverage fact.

## Commands and results

Every run used the brief's command from READER, with the brief's four environment variables, under the 1,200 s wall. Logs are listed under Bulk.

| Run | State | Result |
|---|---|---|
| run1 | baseline, before any edit | 7 passed, 1 failed: 18 of the 47 new mutations missed |
| run2 | G3/G5/G5a implemented | 8 passed |
| run3, run5 | plus the outcome-table and layout-control tests | 10 passed |
| run6 (probe) | canonical-layout line temporarily disabled; only the layout test run | failed as intended: that control reached G8 PREPARATION_MISMATCH. The source was restored, hash-verified at `ba8a08b590` |
| run7 (**final, full command**) | final bytes | **11 passed, 0 failed** |
| run8 | final bytes, `--nocapture snapshot_04_coverage_mutation_outcomes` | 1 passed; 47 outcomes captured |

Only the existing unrelated dead-code warning (`source_blocks.rs` `derived`) appears.

## Against the bar (snapshot 04)

- **All 47 new mutations** produce their expected first gate and code: 47 of 47. Tally: G1 14, G2 4, G3 6, G5 PRODUCT_ATTEMPT 2, G5 ATTEMPT 1, G5a 20. The table is in `MUTATION_OUTCOMES.json`.
- **The 30 earlier mutations** still pass, so all 77 pass.
- **The no-data synthetic case** validates with its expected classifications.
- **The two complete cases** still validate.
- **Eligibility** stays false on all three cases.
- **All 8 earlier tests pass.**
- **Every G5a mutation fails at its I57 check, by code reading:**
  - `coverage_stop_forbidden_entry` and `coverage_stop_uncoupled_consistent_roster` fail at feasibility;
  - `stop_rule_duplicate_zero`, `estimate_forbidden_translation_zero` and `certified_bound_duplicate` fail at the existing entry checks, which come first;
  - the roster, B and data controls fail at their own lines.
- **No expected first failure looks wrong.** No snapshot defect was found.

### New tests (reader-local, synthetic; not shared corpus)

- **`snapshot_04_coverage_mutation_outcomes`:** the tally and the per-mutation outcome print.
- **`coverage_layout_controls_fail_at_g5a`:**
  - mirrors I62's three Python-only layout controls: a force row flagged input-derived, an unflagged constrained displacement, and a nonzero prescription. All three fail at G5a.
  - adds one Rust-specific control, a non-input kind relabel. See deviation 1.
- **`publicly_consistent_coverage_attestations_are_not_rejected`:** mirrors I62's four over-rejection guards. All four are admitted with the base classifications.

## Deviations

1. **The Rust canonical-layout check is stricter than the Python checkpoint-B draft** (`94330e168f`, read only).
   - Rust rebuilds the whole layout from the source maps and requires equality. This is the literal reading of I57 §2: "derive the canonical layout/flags from the bound source maps".
   - Python checks only the input-derived relation, plus the kind of input rows.
   - A non-input layout edit, such as a kind relabel or a dropped, reordered or foreign-body row, therefore fails G5a in Rust. By code reading, Python reaches G8 PREPARATION_MISMATCH.
   - No snapshot-04 mutation edits the layout, so the 47 are unaffected.
2. **G3 with a zero-body inventory.** Rust fails a non-null coverage vector at G3 when the source has no bodies (I57 §1: no empty complete vector). Python's draft compares `[] == []` and passes G3. This is an edge case only.
3. **One temporary probe (run6)** edited a fenced line and then restored it. The restore is hash-verified.

## Open items for ROOT

- **Rule on deviation 1:** full canonical rebuild or per-row relation. Either way, snapshot 05 should pin it with a shared non-input layout mutation.
- **Rule on deviation 2.**
- **Not implemented, as in Python:** G5a direct source-consistency checks for unavailable attempts that keep complete coverage. Snapshot 04 has no such base.
  - Design note for C0: an unavailable p512 attempt has no Selection floor.
  - G5b's φ rounds upward, so φ > 0 exactly when the coupled hat > 0. Floor positivity could then come from the Run's verification resolution.
  - This is unverified, and it needs ROOT/I62 agreement.
- **The coverage-plus-WORK dual-defect order is unpinned by the corpus.**
  - Rust defers every C3 WORK check until all attempts' association checks, including coverage, have passed. The coverage error therefore wins, which matches the order I62 describes.
  - Snapshot 05's planned control will verify it.
- **The remaining I59 obligations stay out of scope:** the G5 audit, the failure-prefix controls, and G7/G8 branches.
- **Not claimed:** acceptance, eligibility, three-reader parity or independent review. No Python or TypeScript run was made by I63.

## Files read (sha256)

NUM was at `027c0912e7` when hashed.

| sha256 | File |
|---|---|
| c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd | NUM/AGENTS.md (the auto-loaded control and READER copies are identical) |
| 1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 | NUM/agents/AGENT_TASK.md |
| d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792 | NUM/P/AGENTS.md (first 40 lines) |
| 45b6fe4f1695741698ee80ca09efd250411c4384efd488040bdc0aabbb25e911 | R/BRIEFS/I63_I64_COVERAGE_READERS.md |
| 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 | R/I57/summary_coverage_01/ADDENDUM.md |
| 10341323b974d7a24155f23013d16b43e803f1d3cb6638df052a6ce2f51a61f3 | R/REVIEW_RV76/summary_coverage_01/REVIEW.md |
| cbb86603db350e410f04e49a3bffcc70964a5357d3d89b669610399dac89522d | T3/ROOT_RULINGS_V1.md (the coverage selection, checkpoint A and checkpoint B sections) |
| cb7aa0bbd7df93e48be1377d9d71e1902a683952077782d2eb09a00c9502ead0 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_04.json |
| 1f8789066cd6a30a017e8b48bd541b3c1133590858c30bfd05dd0c2994c8ac0d | R/I62/coverage_shared_python_01/RETURN.md |
| b7338dfadeaf6a89500fd2156c06fbdb794513387bd0c6875bbe7f5a93661d80 | R/I62/coverage_shared_python_01/SHA256SUMS |
| 90ccf013eb2f24e3826f700d5b0ba7d0812ec7657b11041f7982198b940d10ad | R/I62/coverage_shared_python_01/RETURN_B.md |
| d77574b804d4cfc94f8dbcac6ea91f2408e4123a5244fd80a75df2a42b9302b5 | R/I59/rust_reader_01/RETURN.md |
| 3bc84bf1138b227f759049d2f255a97e257978ff2cd1bbd95b22613515015634 | NUM/P/core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs (1365–1450) |
| e452e3467608a4680359a36d30b6a580d72a12b778d506ff8118646257b7d388 | NUM/P/core/solver/frame_kernel/src/structural/retained/recover.rs (95–200) |
| 94330e168f2a71a86a05102a091dd791e4266037680fd915c1e80e12be36dfc0 | READER/P/core/analysis_runs/retained_precision.py (coverage sections, for parity; read only) |
| de1c401503f82d20ff6ffb2baf059ee4017aff456d6881f1ed628df4e658537a | READER/P/tests/test_retained_precision_contract.py (coverage controls; read only) |
| (above) | the READER fenced files, schema and corpus |

## Bulk (WT/scratch/i63_coverage_rust_01/)

| sha256 | bytes | file |
|---|---|---|
| 086455542a30cd4ff3844490ccfe64671764d56cb67f8211df30e56d0e0001b7 | 20158 | I63_DELTA_src.diff |
| 6d11cf892117159c6bcae9056d75de975e1f25390fcf8b1451023d27302b5a70 | 8855 | I63_DELTA_test.diff |
| 2c3e4105df2d83ba7eeaacf04725e8744df3ebfa093f114a0656f51238e124a6 | 5529 | run1_baseline.log |
| 1d73b8b3204d658ec1e96368abb69cffa2569b826db53a29efb8f95bdf88b6ba | 1394 | run2.log |
| 49600a05629b77a1976e5d134b3c223fb637daec8d2ac112c2b050866796898e | 1496 | run3.log |
| cfc1d0eed0bab4578a4392b2505735bcd4f8b66e3034e7e7e8b09433e117d364 | 12082 | run4_outcomes.log |
| bb44257ec482ddc95dd350814bc0f3a13e56428ff7f2aba9e3dfd049b635367a | 1496 | run5.log |
| fe67de60893b4ef1e40f4fc431ef758052148f05e9c706c149838f4f49f399c5 | 1577 | run6_probe_no_layout_check.log |
| 21ffde656f4fbc6d7c89940af99be7a923019c0b6bb97ef2d6906d3813927645 | 1567 | run7.log (final) |
| 9f2789011b4e4528c8a11c2f41b13088d4d51cbb3e40d461f8d0f052cd36e68e | 12083 | run8_outcomes_final.log |
| b43595136e2cba5b813f4b82e19371c3fde422ee82bd8eb90c66f36a63446d17 | 119761 | before/retained_precision.rs |
| 9a3b557780b9b14eba4d88e1206f38b4d7a66c545e7868591ed1998143cd70ac | 38728 | before/retained_precision_contract.rs |
| 375b07313518a0cdd09e317b83b9ca7536acef5c80fd4cd0a94920fd8bf2486d | 66051 | before/lib.rs |
| ba8a08b590114212e0a12f5de65471ae0dc1aa462d80c8d2f1050965bbdc1f7c | 133707 | final/retained_precision.rs |
| 3b9e6f90292848c61301da28bc201784e21bb8f0de2639e1a62efbada24d7f6d | 46903 | final/retained_precision_contract.rs |

The run*.exit and run*_start.txt files hold the exit codes and UTC bounds.
