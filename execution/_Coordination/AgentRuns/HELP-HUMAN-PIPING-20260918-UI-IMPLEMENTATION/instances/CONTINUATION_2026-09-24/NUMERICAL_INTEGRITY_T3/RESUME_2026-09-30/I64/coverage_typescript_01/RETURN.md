# I64 return: TypeScript reader summary-coverage checks on snapshot 04

I64 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent (Claude Code harness-native subagent). It had no descendants.

- **Run:** first tool call 2026-10-03T20:00:52Z; final checks 20:11:37Z; return written about 20:15Z. Well inside the 90-minute box.
- **Host:** the M5 host. The memory guard (memguard.sh, PID 5387) was running at the start.
- **Limits held:** no Git writes or index operations, and Git reads used `GIT_OPTIONAL_LOCKS=0`. No install, build, Cargo, solver, native, UI or DEC-025 job. The test runs used the existing `node_modules` link and the source-bound WASM assets as they were.
- **Other authors:** I62's, I63's and the shared files were not touched.
- **Paths** use the brief's placeholders (WT, READER, NUM, P, T3, R).

## Changed files (READER, inside the fence)

| File | Before (I60 freeze) | After |
|---|---|---|
| P/apps/desktop/src/features/results/retainedPrecision.ts | f2e2d1868c973d22568dfbe615cf999e24a79f6b5291935db6c4801686e7760c (80752 B) | a21487a4e19b894063f6efcbec24c0910d944fa7b71076bd0d2d80310a607d5f (91374 B) |
| P/apps/desktop/src/features/results/retainedPrecision.test.ts | 77cee1aab77deaa9c51c289156fbc65f346937a59a70073b3f4c43fe2cf5ca06 (13427 B) | 9f98268ef2cd6bbcd6416ce9b053c210eca2438dfcf18be357d64e1b260291dd (16160 B) |

## Shared files verified at start, and unchanged at the end

| File | sha256 (= SHARED_SNAPSHOT_04) |
|---|---|
| P/schemas/retained_precision_mp_v2.schema.json | f943ebd3511e31bcdfe09bac2788362d3a5734040bc3088fbe585361b571cd21 |
| P/fixtures/results/retained_precision_cases.json | 8e333e632cde3b70cb7fcce13d5f9eb724fbc22246c304a30dbdbefcd0cce760 |
| P/fixtures/results/retained_precision_prepared_ordinary_v1.json | 3e0779a45a… (checked at the final run) |
| P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json | c74742ce6a… (checked at the final run) |

READER head is ae97b7d5c2.

## Reader changes, by gate

Gate order and error codes are unchanged, and earlier original checks still win. **The completeness hold stays in place:** `SUMMARY_COVERAGE_COMPLETE = false`, so no case becomes eligible.

The TypeScript draft had no Cartesian roster to remove. It checked the summary lists only for uniqueness, kind, order and range. Those checks remain as the "existing encodings and ranges" step. The exact I57 rules are added after them.

- **G1.** The pinned schema walker already rejects every shape defect. An explicit `coverageShape` check was added as a second guard that does not depend on the schema bytes. It requires the member, null or an array, closed `{body, stop, has_data}` objects, exactly four booleans in `stop`, and a boolean `has_data`.
- **G2.** The schema walker's `uint` encoding catches every corpus defect. An explicit `uint(body)` check was also added in the G2 pass.
- **G3** (`coverage()`). A non-null array must be non-empty, must have exactly one entry per body in the attempt's source `body_membership`, and must list the bodies as `0..n-1`. A null `source_ref` is left to G5.
- **G5** (`productAttempts()`, in the association pass and before the deferred WORK pass).
  - **Non-null coverage requires all of the following:**
    - the attempt's own source;
    - non-null `source_ref` and `run_ref`;
    - a selected native Run whose id, origin source and origin owner match the attempt;
    - lanes `[admitted_k, annular_source]`, both completed;
    - proof_start, projection, maxima, values and aliases completed;
    - a certificate that was entered.
  - **Non-null coverage is required by any of these:** a completed certificate stage, a passed certificate check, a completed G5a stage, a passed G5a check, or Ready.
- **G5a,** for each selected case. The new `selectedCoverage` runs after the existing summary encodings, ranges and p/P/floor rules, in this order:
  1. **Canonical source layout and facts** (`coverageFacts`, from the bound source maps only).
     - Every prescription must be exactly +0.
     - The layout is rebuilt from the maps with `sourceLayout`, which G8 now also uses, and must equal `s.layout`. Only constrained displacement/rotation rows are input-derived, so D = false.
     - `present` and `non_input_present` are taken per body and kind.
     - L is computed in the native `body_extent` order and must be finite and non-negative.
     - The free DOFs are found, and so are any individually nonzero original nodal terms at free DOFs.
  2. **The 16-vector feasibility rule** (`stopFeasible`), with D = false. Floor positivity is ORed after the L ≠ 0 coupling and outside it, as in final_case.rs:1394–1408.
  3. **The estimate/charge rederivation:** the L-coupled hats from E; charge equals estimate below p512 and equals the force/moment stop bits at p512.
  4. **Exact rosters, I57 §4 items 1–4.**
     - The stop, estimate and charge lists hold exactly one entry per true bit and none per false bit, in canonical order, with values that are not −0, non-negative, finite and within their limits.
     - The resolution and theta values are canonical.
     - The B list equals exactly the bodies with `has_data`, each with a positive value.
     - In the verification record, a body with data has exactly one non-null bound, and a body without data has only null bounds and theta +0.
     - `data_blocks` is 0 if and only if no body has data, and otherwise at least the number of bodies with data.
  5. **Direct data facts.** A body with no free DOF must have `has_data = false`. A body with any individually nonzero original nodal term at a free DOF must have `has_data = true`.
  
  No final row feeds any coverage fact.
- **G5a, non-selected attempts with a complete roster** (`unselectedCoverage`).
  - **What it checks:** source layout, feasibility, record relations and direct data facts.
  - **Floor at p512:** no Selection exists, so the floor's sign is unknown and every combination of floor signs is admitted (an existential floor).
  - **Not checked:** the estimate/charge lists, since they belong to the Selection.
  - **Untested:** snapshot 04 has no such base. See the parity notes.
- **G8.** The layout construction was refactored onto the shared `sourceLayout`. The bytes and the order it compares are unchanged, and every G8 test still passes.

## Commands (from READER/P/apps/desktop)

| Run | Command | Result |
|---|---|---|
| vitest_00 (baseline, I60 source on snapshot 04) | `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2` | 100 tests: 76 passed, 24 failed (all 24 were new coverage mutations; the no-data case already validated) |
| vitest_01 | same | 100 passed |
| tsc_02 | `READER/P/node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` | exit 0 |
| probe_04 | temporary probe test that records the throwing line for each mutation; removed, with the test file restored byte-identically (77cee1aab7) | 47 of 47 |
| vitest_05 | same as vitest_00 | 102 passed |
| vitest_06 | the two new layout controls run against the pre-edit reader, which was swapped back in and then restored (a21487a4e1 verified) | both fail at G8 PREPARATION_MISMATCH, so the new G5a check is decisive |
| vitest_09 | same as vitest_00 | 106 passed |
| **vitest_10 (final)** | same as vitest_00 | **107 passed, 0 failed** |
| **tsc_11 (final)** | same as tsc_02 | **exit 0** |

The final runs used reader a21487a4e1 and tests 9f98268ef2. All four shared hashes were identical before and after (final_hashes.txt). No vitest or tsc process remained afterwards.

## Against the bar (snapshot 04)

- **All 47 new mutations** produce their expected first gate and code: 47 of 47. They are also asserted by the shared corpus test.
  - The per-mutation table, with the raising line in the final reader, is in `MUTATION_OUTCOMES.json`.
  - The counts match SHARED_SNAPSHOT_04: G1 RECEIPT 14, G2 ENCODING 4, G3 COVERAGE 6, G5 PRODUCT_ATTEMPT 2, G5 ATTEMPT 1, G5a SCALE 20.
- **Reasons:**
  - G1 and G2 fail at the schema shape and encoding walkers.
  - G3 fails at the new inventory line; `attempt_owner_only` fails at the existing owner check.
  - `coverage_null_*` fail at the new G5 non-null rule.
  - `certified_bound_unbound_drop_existing_g5` fails at the existing G5 record binding.
  - `stop_rule_duplicate_zero`, `estimate_forbidden_translation_zero` and `certified_bound_duplicate` fail at the existing G5a uniqueness and kind checks, which run first.
  - `coverage_stop_forbidden_entry` and `coverage_stop_uncoupled_consistent_roster` fail at feasibility. The label of the first names item 1, but the I57 within-G5a order puts feasibility first; Python reports the same.
  - The stop, estimate and charge controls fail at the exact-roster line.
  - The has_data and B controls fail at the B list.
  - `coverage_no_data_claim_with_free_loads` fails at the direct data constraint.
  - The six no-data mutations fail at their own I57 lines.
- **The no-data synthetic case** validates, with exactly the expected classifications.
- **Both complete cases** still validate.
- **All 30 earlier mutations and the earlier tests** pass.
- **Seven TypeScript-only tests were added.** They are not in the shared corpus.
  - **Three layout controls fail G5a:** a force row marked input-derived; a constrained displacement not marked input-derived; a nonzero prescription.
  - **Four publicly consistent rewrites must validate:** stop [T,T,F,F]; all-false stop; no-data all-true stop; a no-data body attesting one data block. These mirror I62's Python-only tests.

## Deviations and parity notes for ROOT

1. **The unavailable-attempt G5a direct checks are implemented in TypeScript, but untested.** Python deferred them (I62 RETURN_B), and C0 plans them.
   - They use an existential p512 floor, the record relations and the direct data facts.
   - If ROOT wants exact parity before snapshot 05, either Python adopts the same rule, or ROOT directs I64 or a successor to remove `unselectedCoverage` until C0 pins the expected outcomes.
2. **The record-bound rule is slightly stricter than Python's wording.** For a body with data, TypeScript requires exactly one record entry, and it must be non-null. Python requires a non-null record bound if and only if `has_data`. The two differ only on a duplicate null entry, which the native record cannot produce.
3. **The explicit G1/G2 guards are redundant with the pinned schema walkers.** They do not change any outcome on snapshot 04.
4. **No defect was found in snapshot 04,** and no expected first failure looks wrong.

## Open items

- Snapshot 05 bases are still needed: multi-body, absent kind, L = 0, p512 floors, a second owner, failure prefixes, and unavailable or failed-certificate attempts. The coverage-plus-WORK dual defect is also unpinned.
- After that, a fresh independent review of the complete TypeScript source. Eligibility stays closed.
- I60's other remaining obligations are out of scope.

## Files read (sha256)

| sha256 | File |
|---|---|
| c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd | NUM/AGENTS.md |
| 1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 | NUM/agents/AGENT_TASK.md |
| d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792 | NUM/P/AGENTS.md |
| 45b6fe4f1695741698ee80ca09efd250411c4384efd488040bdc0aabbb25e911 | R/BRIEFS/I63_I64_COVERAGE_READERS.md (at e02f02a93d) |
| 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 | R/I57/summary_coverage_01/ADDENDUM.md (all of it) |
| 10341323b974d7a24155f23013d16b43e803f1d3cb6638df052a6ce2f51a61f3 | R/REVIEW_RV76/summary_coverage_01/REVIEW.md |
| 4a249ad480abab54bd8ea27cdb5e327aa466c69d39e0ede94623241f1606a538 | T3/ROOT_RULINGS_V1.md at e02f02a93d (the two named sections, plus the adjacent handoff and resumption sections) |
| cbb86603db350e410f04e49a3bffcc70964a5357d3d89b669610399dac89522d | T3/ROOT_RULINGS_V1.md at 027c0912e7 (only the new "I62 checkpoint B verified" section) |
| cb7aa0bbd7df93e48be1377d9d71e1902a683952077782d2eb09a00c9502ead0 | R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_04.json |
| 1f8789066cd6a30a017e8b48bd541b3c1133590858c30bfd05dd0c2994c8ac0d | R/I62/coverage_shared_python_01/RETURN.md |
| 90ccf013eb2f24e3826f700d5b0ba7d0812ec7657b11041f7982198b940d10ad | R/I62/coverage_shared_python_01/RETURN_B.md (read after implementing, for parity) |
| 7eb2ae3ff9b752ac116298210517de238358f4ee21f692835874ca07a94c84f1 | R/I60/typescript_reader_01/RETURN.md |
| 3bc84bf1138b227f759049d2f255a97e257978ff2cd1bbd95b22613515015634 | NUM/P/core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs (lines 1365–1450 only) |
| 6a2fc382bf8cae0502c41030da8ac9bc0cfe1f1b80b7aceac60ff7e40f345eda | NUM/P/core/solver/frame_kernel/src/structural/retained/adaptive.rs (`body_extent` lines 321–336 only) |

I also read the two fenced files at their pre-edit hashes, and the parts of the shared schema and corpus listed above.

## Bulk (WT/scratch/i64_coverage_typescript_01/)

| sha256 | Bytes | File |
|---|---|---|
| f2e2d1868c973d22568dfbe615cf999e24a79f6b5291935db6c4801686e7760c | 80752 | before/retainedPrecision.ts |
| 77cee1aab77deaa9c51c289156fbc65f346937a59a70073b3f4c43fe2cf5ca06 | 13427 | before/retainedPrecision.test.ts |
| a21487a4e19b894063f6efcbec24c0910d944fa7b71076bd0d2d80310a607d5f | 91374 | retainedPrecision.after.ts |
| 9f98268ef2cd6bbcd6416ce9b053c210eca2438dfcf18be357d64e1b260291dd | 16160 | retainedPrecision.test.after.ts |
| 73c4a1c081978a59269f90a49900bab4415778ef8863f1056219773dfa65eeca | 21289 | i64_reader.diff |
| d0437052fccdaa4d8c137cf9254522b433de084c1bea84bcf71f4d4271f61046 | 3262 | i64_test.diff |
| 57f6ab6650a06b5bc7744d0e8aec3a57ffcd2ce89bf4235065259b6ae6bc062b | 13340 | probe_04.txt |
| cc9e18e4403bbca63dd81b80ad1a36dae2e1892130cb429aa19392ebb35621d9 | 339 | probe_04.log |
| 77f1b5f58cbedcd739d140f35f8de5d86415d71af473ede336188923150c65c7 | 339 | probe_03.log (console output suppressed; superseded by probe_04) |
| 77cee1aab77deaa9c51c289156fbc65f346937a59a70073b3f4c43fe2cf5ca06 | 13427 | test_before_probe.ts |
| 012e6baa0b29d87887e4f806e32dd5dadb88f9c69581fb0d2df308a6d6ea0471 | 9810 | vitest_00_baseline.log |
| 5c1aa70f909bf46f23167ca4c14f9090dacb3f0d25154b50d702c410b7ce113f | 560 | vitest_01.log |
| cffe47c13890d3efec188a1bd5a7c35187e551aae4c52decd804a545f27ba44c | 124 | tsc_02.log |
| b98b7669b95b576325ccc9418bf887b9353406f855fcc0ec3cc065aa5ae20385 | 560 | vitest_05.log |
| 1f34cc986c49d50ed574987976daf119c9391ce41f36c0f97c2cbe431c53d4dc | 2904 | vitest_06_local_controls_on_before.log |
| db3ed46624f65268020ab5037d57997a89a702480b54a9da8b2a566a82d28361 | 560 | vitest_07_final.log (superseded by vitest_10) |
| f4a073b0ba8bbda04dd3fbbd55342629cd25c325be595071c958f5d5239b9dda | 112 | tsc_08_final.log (superseded by tsc_11) |
| e42d6db47aa7c41707d7b55c6cfe2a3181d7a1d2e4cbb24a564de593d7c27365 | 440 | vitest_09.log |
| 31118c60401f2f7bcbe76648efc21e7af3263a8c879ab162aa60682e65996ba1 | 560 | vitest_10_final.log |
| 5933444f94c3535a003b2c0c4d5ccf513029d08e6741bacd26ca4176889ee4ef | 112 | tsc_11_final.log |
| 8529e2a2a649e87f326cef0599e65d0a2b9c9b43501098b2f23041a737324006 | 237 | final_hashes.txt |
