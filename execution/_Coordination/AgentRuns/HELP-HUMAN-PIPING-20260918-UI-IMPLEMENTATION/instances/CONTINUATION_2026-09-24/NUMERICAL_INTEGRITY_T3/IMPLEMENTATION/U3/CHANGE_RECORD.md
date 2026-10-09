# U3: the legacy pressure contract is retired product-wide: change record

- **Branch:** `codex/piping-t3-pressure-retire-20261008`, cut from main `7eae707bb7`.
- **Product head:** `fe657e3a68`. The package is a separate commit on top of it, and changes no product file.
- **Commits:** 18 product commits on the branch's first-parent line. One of them, `1e9724fb94`, merges I114's demo lane (`9744ed7e69`, 3 commits from `4c0d5d7c00`).
- **Size:** the PR changes 133 maintained files (121 modified, 6 deleted, 3 added, 3 renamed). Nothing under `execution/` changes outside this package.
- **Implemented by:**
  - I110 (T3): `R/I110/pressure_retire_01/` (the inventory and plan), then `pressure_retire_02/` to `pressure_retire_06/` (rounds 2 to 6);
  - I114: the demo lane, `R/I114/demo_fixtures_01/` and `demo_fixtures_02/`;
  - I111: M07's premise, `R/I111/m07_premise_01/REPORT.md`.
- **Path convention:**
  - `P` is `projects/chirality-piping`;
  - `PP` is `P/core/product_physics`;
  - `RE` is `P/core/reporting/result_export`;
  - `R` is the T3 records root `…/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30`;
  - `RR` is `…/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md`.

## 1. What changes and why

**Why.** The owner retired the legacy pressure contract, and the flawed computation behind it, product-wide: "If you have replaced old code with new because the old was flawed, don't maintain the flawed code or compatibility with it". The cited decisions are in §2.

Before this PR, the legacy computation was already unreachable for nonzero pressure: the ordinary route refused it with `PRESSURE_MODEL_REAUTHOR_REQUIRED`. It survived in three places:
- a test-only historical scope;
- a zero-pressure admission of documents labelled `1.0.0/legacy_pressure_v1`;
- the code itself.

**What the PR does:**
1. **Refuses** every legacy pressure input on every route. A pressure primitive of any value in a non-exact document is refused, zero included, and so is the legacy label. New pressure primitives cannot be authored (§3).
2. **Removes** the legacy pressure computation, the test-only historical scope, the oracles that pinned the flawed numbers, and the validation cases that were oracles of the retired computation (§4).
3. **Refuses** the flexibility joints that the builder used to skip silently while its review rows said "consumed" (G11; §3).
4. **Replaces** the browser's bundled demo results, which were computed with the flawed joint and legacy pressure, with results the current product computes from a valid demo model. The app's default session model becomes that model (§5).
5. **Corrects** every published text that described a treatment the product no longer performs, except one that is incomplete rather than false (§6).

**What it keeps:**
- 0.1.0 and 0.2.0 stay as the pressure-free namespace (D-1 A).
- The exact contract `2.0.0/exact_straight_pressure_v2` is unchanged. Every exact document's bytes are equal to main's (§7).
- The joint element's code (FK `user_stiffness_local_matrix` and its plumbing) stays. It is deleted in the PR that lands T4's corrected joint. Until then every model containing that joint is refused.

## 2. The owner's decisions and ROOT's rulings

| RR heading | What it decides for this PR |
|---|---|
| "Owner decisions: the legacy pressure contract is retired product-wide; T3 gains a WORKING_ITEMS manager" | Owner decision 1 ("1a, retire it product-wide"). `1.0.0/legacy_pressure_v1` is accepted nowhere. The legacy computation, the historical scope's pressure part and its oracles are removed. B3a is dropped. The change goes as its own PR with independent review and T3's gate set |
| "U3 rulings on I110's pressure inventory: D-1 A, D-2 A1, D-3 held with M07" | **D-1 A:** 0.1.0 and 0.2.0 stay as the pressure-free namespace, with every legacy pressure semantic removed. **D-2 A1:** a pressure primitive of any value, zero included, in a non-exact document is refused with `PRESSURE_MODEL_REAUTHOR_REQUIRED`. **D-3:** the demo's frozen legacy result is held until M07 is settled. **D-4:** reuse `PRESSURE_MODEL_REAUTHOR_REQUIRED` |
| "Owner decision: M07's flawed joint element, option A; U3 Stage 2 released" | Remove the test-only historical scope, the joint bypass and the four C-150 oracles. **G11:** a joint with no lateral value is refused. **G10 and D-3:** the bundled demo results are replaced by product-computed results from a valid model. Stage 2 is released. In the same section: RV127's N-2 (`OP-PRESSURE-PRIMITIVE-RETIRED`) is accepted |
| "U3 Stage 2 rulings: the 800 B profile re-pin, two retired validation cases, G11's code and the builder's other silent skips, and no published text for a retired treatment" | (1) The B1 in-build profile re-pin (−800 B per phase) is accepted, conditional on Pass B, T9 and the both-entry gate. (2) The two validation cases are removed. (3) `JOINT_ELEMENT_STIFFNESS_INCOMPLETE` is accepted, and the builder's other silent skips are refused too. (4) Published text must not describe a treatment the product no longer performs. In the same section: the app's default model switches to the demo; I114's derived 0.1.0 carrier is accepted; A1-S-1 (`primitive_loads`' thrust) is removed in this PR; the governance texts go to the owner (A1-N-2); the friction-reversal re-author is a follow-up (A1-N-3); the round-4 stop is ruled: T2 corrected, T4 unchanged, the joint messages under the same test, MECH-TP-PHYS-008's id kept |
| "Owner decisions: T4 starts now; the governance-documents PR is authorized" | The governance-documents PR is cut after this PR merges. In the same section: **T2's re-pin check, option (a)**, stated exactly (§6) |

## 3. The refusals and their codes

| Code | Where | What is refused |
|---|---|---|
| `PRESSURE_MODEL_REAUTHOR_REQUIRED` (blocking; an existing code, reused per D-4) | PP `pressure_runtime::validate_profile` | (a) the label `1.0.0/legacy_pressure_v1` (ref `pressure_contract`); (b) any pressure primitive in a non-exact document, zero included (refs: case and load). One re-author text names `2.0.0/exact_straight_pressure_v2` for every value (RV127 S-1). It applies on the ordinary route, the runner and the retained entry; the retained entry publishes the ordinary refusal |
| `PRESSURE_CONTRACT_UNSUPPORTED` (unchanged code) | the same | an unknown contract. Its text now names only the exact contract |
| `OP-PRESSURE-PRIMITIVE-RETIRED` | the operation applier | authoring a new pressure primitive (`create_primitive_load`). Existing primitives stay editable and deletable, so a document can be re-authored |
| `JOINT_ELEMENT_STIFFNESS_INCOMPLETE` (blocking; new) | PP `preview_physics::refuse_unqualified_joint_elements` | a `mechanics_geometry_and_user_flexibility` joint that lacks any of its four user stiffness values (lateral, axial, angular, torsional). The message names the missing values; refs: the joint and its pipe (G11) |
| `JOINT_ELEMENT_MAPPING_UNRESOLVED` (blocking; new) | the same | such a joint with no pipe, an unknown pipe, an unknown joint node, or a node that is not an end of its pipe (G11 extended). A new code, because `EXPANSION_JOINT_MAPPING_INPUT_INVALID` is a non-blocking warning on the same subject |
| `JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED` (unchanged) | the same | M07's realized user-stiffness joint. Nothing bypasses it any more, since the test-only scope is gone |
| `UnsupportedTargetForCategory` (`primitive_loads`) | `prepare_straight_pipe_axial_effects` | a pressure load: its `Pressure` arm is removed (A1-S-1). It now blocks every output |

**Each G11 test fails before its fix.** Before the fix, every case of `flexibility_joint_missing_a_user_stiffness_is_refused_not_dropped` and `flexibility_joint_with_an_unresolved_mapping_is_refused_not_dropped` solved, and published 2 or 3 review rows saying "consumed". The lateral value is zero wherever it is not the missing value, so M07 cannot mask the defect (RV127 A1-N-4; `R/I110/pressure_retire_04/`).

A pipe without `y_reference`, or with an unknown end node, needed no new code: the pipe itself is refused (`PIPE_ORIENTATION_INPUT_MISSING`, `PIPE_ENDPOINT_UNKNOWN`). `flexibility_joint_pipe_without_orientation_or_a_known_end_is_refused_by_the_pipe` pins this.

**Pins on the other surfaces:**
- **Readers:** the RS, PY and TS readers pin the label case as G8 `INVOCATION_MISMATCH`, at the milestone schema and at 0.3.0.
- **Runner:** a CLI solve is refused in both modes, for the label and for a zero primitive.
- **src-tauri:** a labelled document opens unchanged, and its solve is refused.
- **App:** the load-case manager no longer offers the pressure category, and points to exact regions. The pressure panel shows a contract-free model as "none (pressure-free; …)", names the count of any legacy primitives it still carries, marks exactly `1.0.0/legacy_pressure_v1` as "retired", and marks any other contract "unsupported".

## 4. The removals

**The test-only historical scope** (`cda85e06d5`):
- `PP/src/historical_pressure_reference.rs` and its `mod`;
- the joint bypass and the legacy-pressure bypass;
- O1, O2 and O4, and O3's second half. O3's first half stays as `realized_user_stiffness_joint_is_refused_on_the_ordinary_route`;
- `historical_pressure_preview(_with_mode)` and the two scope tests. The demo's ordinary refusal stays as `bundled_demo_with_legacy_nonzero_pressure_is_refused_on_the_ordinary_route`.

**The pressure-only oracles** (`390c882619`):
- 11 `*_historical_pressure_premise` tests and `curved_bend_pressure_load`;
- S-11F's F10 (2 tests, 3 helpers);
- S-11G's T6a pressure run.

One of the 11 also had a thermal half. `769c109d82` restores it as a pressure-free test in both modes (RV127 B-1).

**The legacy computation** (`9930cfe6db`). In PP:
- the thrust types, builders and assembly;
- the bend radial thrust;
- the joint thrust rows and `EXPANSION_JOINT_PRESSURE_THRUST_APPLIED`;
- `pressure_for_pipe`;
- the hoop and longitudinal rows and `include_pressure_longitudinal`;
- W2's `pressure_thrust_load` family;
- `recover_section_stress`'s `pressure` parameter;
- the always-empty plumbing in `source_recovery.rs`, `source_receipt.rs` and `retained_product.rs`;
- `membrane_radius`.

In the other crates:
- `stress_recovery`: `PressureBasis`, the membrane, and the pressure components and ranges;
- `curved_bend`: the radial-pressure API;
- `primitive_loads`: the `Pressure` arm of `prepare_straight_pipe_axial_effects` and `ElementAxialEffectProperties.internal_area` (`5bc6f269da`, A1-S-1). There is no exact-route caller: its only callers are its own tests and the mechanics benchmark.

**Signed zeros (H-1).** The retired term entered three sums as `+ unwrap_or(0.0)`. Each keeps an explicit `+ 0.0`: `open_formula_summary_mpa`, `straight_summary_extrema` and `stress_recovery`'s `summarize_components`.

**Kept:**
- `component_pressure_thrust_load_count`, always 0;
- the exact retain list;
- the preview-physics-1 kind lists and the result schemas;
- the primitive `pressure` category;
- `pressure_thrust_reference`.

**The B1 in-build profile re-pin.** Deleting the dead pressure fields shrinks two profile atoms:
- `s((&str,StressRecoveryResult))`: 192 → 160 B;
- `s((String,DerivedSection))`: 88 → 80 B.

Every in-build phase is therefore exactly 800 B lower in both modes. `PINNED_RECORD` and the challenge literals (`W1_PHASE_BYTES`, `MAX_PHASE_BYTES`) are regenerated by the code's own rule. No binding, form or phase changes. RV127 confirmed exactly these two atoms, and ROOT accepted the re-pin on conditions (§8). `QUAL_B1.md`'s figures become historical.

**The validation cases** (ruled; confirmed by RV127):
- `STRESS-PRESSURE-MEMBRANE-ORIGINAL`: the benchmark, runner binding, hand calculation, manual page and index rows are removed.
- `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` is removed (benchmark, hand calculation, README rows).
- `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` loses its two membrane values and keeps its four mechanics components.
- MECH-TP-PHYS-008/009 lose their pressure halves. Their axial totals become 3.0 N (was 12.0 N); the ids are kept.
- **Counts:**
  - the mechanics suite goes from 25 cases and 206 values to 24 and 192;
  - the stress suite goes from 15 cases (12 matched, 3 blocked) to 14 (11, 3);
  - the manual inventory goes from 64 to 63 pages.
- Stale references are removed (RV127 A1-S-2): the `STRESS-RANGE-MECHANICS-ORIGINAL` generator entry and page, the `stress_recovery` README, and the DEL-10-05 multi-case witness. The witness now equals the current runner's payload.
- **The DEL-10-05 procedure** (`P/docs/validation_manual/headless_runner_reproduction.md`): its five bound-path commands now pass `--explicit-local-private-intent`. Each output's `payload` equals the committed witness.

**The bundled results computed with flawed premises** (I114; §5):
- `invented_mechanics_result.json` (D-3);
- `invented_mechanics_result_precision_1_{sparse,dense}.json` (G10);
- `precision_fixture_generation.json`;
- `PRECISION_FIXTURES.md`, which becomes `DEMO_FIXTURES.md`.

## 5. The demo model

**The model:** `P/fixtures/product_preview/invented_demo_model.json` (`project:invented-demo-loop-01`, I114).
- It is PP's joint-free test model (`PP/tests/fixtures/preview_physics_invented_model.json`): the invented loop without its four legacy pressure primitives and without joint C-150. Only its identity differs.
- It keeps the bend, branch, valve and terminal components; the one-way and friction nonlinear supports; the spring hanger; constant effort; thermal; and the user combination.
- It solves: `MECHANICS_SOLVED`, `checks_passed`.
- **One independent tie:** the friction normal is 52.373281987314456 N, which matches I111's independent record (52.37 N).

**The generated outputs.** The recipe (`npm run generate:product-preview-mechanics`) installs only a solved result. Its outputs are:
- the preview-physics-1 pair, `invented_demo_result_preview_physics_1_{sparse,dense}.json`;
- the historical-format carrier `invented_demo_result_legacy_0_1.json`, derived from the sparse output and labelled as derived (accepted by ROOT);
- the record `demo_fixture_generation.json`.

`--preview-physics-1` generates the refused model's envelopes (`invented_mechanics_result_preview_physics_1_{sparse,dense}.json`) and their record.

**The consumers:**
- The browser's reference inspection and the Python preview service read the demo set.
- **The default session model** is the demo, in both the browser fallback and native `load_preview_model` (ROOT; I114 round 2). `default_session_model_solves_in_both_modes` pins that it solves.
- `invented_preview_model.json` stays as PP's refusal fixture.

**Regenerated on the merged head** (round 6, `fe657e3a68`; `R/I110/pressure_retire_06/_run_records/demo_regeneration_check.txt`):
- Both modes ran at `11a026b628`. Every fixture output is byte-identical to the committed one.
- The two generation records differ from I114's only in their source inventory: 27 changed entries, plus the deleted `historical_pressure_reference.rs`. Each entry equals this branch's own source file.
- Outputs, tools, dependencies, recipe, generator and input models are unchanged. The records are committed in `fe657e3a68`.

**RV128's other items** are closed in I114 round 2 (CONFIRMED). The latent `buildPreviewComparison` defect on preview-physics-1 combination rows is routed to T6, and is marked by an `it.todo` naming T6.

## 6. The published-text corrections and their checks

ROOT's test: published text must not describe a treatment the product no longer performs, and byte identity is not a reason to keep it. Each correction is checked mechanically.

| Id | Where | Old | New | Commit |
|---|---|---|---|---|
| T1 | the curved-bend review row | `pressure_thrust_treatment=arc_end_cap_tangent_pair_plus_consistent_radial_wall_load` | `pressure_thrust_treatment=none_pressure_refused_outside_the_exact_straight_contract` | `4aad4f42fa` |
| T3a | the joint review row's basis | `pressure_thrust_generation=load_side_user_effective_area;pressure_thrust=` | `pressure_thrust_generation=none_pressure_refused_outside_the_exact_straight_contract;user_pressure_thrust_reference=` | `4aad4f42fa` |
| T3b | the joint review row's sign convention | "…; pressure-thrust generation is load-side effective-area evidence and no compliance claim is made" | "…; no joint pressure thrust is generated and no compliance claim is made" | `4aad4f42fa` |
| V1 | `EXPANSION_JOINT_MECHANICS_INTERFACE_UNSUPPORTED` (`validation.rs`) | "…; pressure thrust remains load-side input evidence" | "…; no joint pressure thrust is generated" | `5dc62d764c` |
| V2 | `EXPANSION_JOINT_GEOMETRY_INPUT_INVALID` (`validation.rs`) | "… must be finite positive user-entered values before load-side pressure-thrust evidence can be generated" | "… must be finite positive user-entered values; they are recorded as input evidence only, and no joint pressure thrust is generated" | `5dc62d764c` |
| T2 | PP `preview_formulation_basis`, limitation [1] (`product_preview_mechanics_v1`) | "Pressure thrust and pressure stress retain the existing preview formulation and capability qualifications; pressure formulation qualification remains open." | "Pressure is not solved on this profile: legacy pressure inputs are refused, and pressure is solved only on the exact straight-pressure profile, under that profile's own qualifications." | `11a026b628` |

**T4 is unchanged** (ROOT). `PP/src/preview_physics.rs`'s `LIMITATIONS[1]` reads "Nonzero pressure is refused on this route, …". That is true: the refusal is merely broader, since it covers zero too. Its radius is the whole retained-precision and reader corpus, so the wider wording waits for the next scheduled corpus generation.

**The radius of T1, T3 and V1/V2.** No committed test carries them. The committed files that still carry the old T1 or T3 strings are left as captured: `P/fixtures/results/invented/result_export_v0_2.json` and 10 historical DEL-09-04 reproduction and witness outputs. In the corpus:
- T1 changes `result_export_v0_2.json`'s producer case 1 in both modes;
- V1 changes contract-corpus case 67's base and applied models in both modes.

These are 6 rows, and each differs from main only in its declared string (§7).

**T2's re-pin under ROOT's exact check (option (a)).** T2 is published only where source-block (retained-source) recovery is selected. Its carriers bind their publication with `source_block_recovery.body.publication_sha256` and `source_block_recovery.receipt_sha256`, which the readers verify. So a string-only edit cannot hold, and ROOT stated the check as four steps:
1. Take the old bytes and replace only the declared string.
2. Recompute both digests from the edited content by the product's own rule, then `generation.json`'s raw hashes from the resulting files.
3. The result must equal the head's own output, byte for byte.
4. The three readers verify it.

The product's digest rule is PP `source_receipt::hash`: the SHA-256 of the RFC 8785 text of `{"domain": d, "payload": p}`, where:
- the publication digest's payload is the document without `source_block_recovery`, under domain `source_blocks_publication_v1`;
- the receipt digest's payload is `body`, under domain `source_blocks_receipt_v1`.

First, a baseline at the head without T2 confirmed two things for all 12 files: the rule reproduces every committed digest, and the head reproduces every committed byte. The per-file check is `R/I110/pressure_retire_06/_run_records/t2_per_file_check.txt`.

| Carrier | Step 1 + 2 | Step 3 |
|---|---|---|
| the 12 source-block raws (`P/fixtures/product_preview/source_blocks/{,ui/}{multicase,n05,n06}-{dense_scrutiny,sparse_interactive}.raw.json`; 2 in struct order, 10 with sorted keys) | 3 leaves change in each file: the string and the two digests | 12 of 12 equal the head's output |
| `f1b_w2_exact_block_selection_of_a_range_triggered_case_publishes_mains_bytes`'s 2 full-envelope SHA-256 constants (`PP/tests/f1b_w2_runtime.rs`) | the same edit, applied to main's compact envelope | 2 of 2 equal: sparse `d953a683…` → `aaabc777…`, dense `10d312a1…` → `7db0edaf…` |
| `generation.json` | its 12 raw-file hashes, recomputed from the resulting files; the 12 request hashes and every other member unchanged | equal to the files |
| `P/fixtures/results/retained_precision_carrier_cases.json`, `fixtures.source_blocks_n05_sparse.sha256` | the file hash of the re-pinned `n05-sparse_interactive.raw.json`, recomputed from the resulting file. **Not in the ruled radius** (below) | equal to the file; no other byte changes |
| the precision UI pair `P/fixtures/results/precision_connected_ui_mechanics_{sparse,dense}.json` | a string-only edit (these documents carry no digest) | — (captured precision-1 documents) |
| the 2 `rejected_stress_range` historical captures and their `ORACLE.json` | unchanged, as captured | — (the head already does not reproduce them) |

**Step 4:** the three readers verify every re-pinned carrier, and the suites pass (§7). The 4 tests on the precision UI pair (RV128 A1-N-4) pass: `physicsResultExport`, `KnownSemanticNotices`, `test_analysis_run_compatibility.py` and `test_preview_physics_consumer_contract.py`. Before the full suites, the targeted readers also passed:
- PP `--test f1b_w2_runtime`: 14/14;
- the source-block, consumer-contract and carrier pytest files: 311 passed, 11 skipped;
- the 15 vitest files that read these carriers: 625/625.

**The carrier-cases file is outside the ruled radius.** `retained_precision_carrier_cases.json` pins the file hash of every fixture it names, and one of them is `n05-sparse_interactive.raw.json`. Round 5's radius missed this, so ROOT's ruling does not list it. The ruling says T2 "reaches no retained-precision successor or reader corpus". This file is the carriers' standing-parity scenario set, not the reader corpus `retained_precision_cases.json`.
- Only the one fixture hash changes, by the same rule as `generation.json`'s hashes. No case, expectation or other pin changes.
- Without the change, the fixture-integrity assertions in RS, PY and TS fail.
- It is listed here for ROOT's confirmation (§8).

**T2's radius is complete** (ROOT's condition on the carrier-set hash; RR "U3, T2's re-pin check, option (a)" and its follow-up). `git grep -F` of the 12 old source-block raw hashes and the f1b_w2 pin's 4 old constants finds no file outside `P/execution`; the only hits are agents' records, kept as captured. The command and its output are in `radius_sweep.txt`.

## 7. The evidence

B is main `7eae707bb7`. The candidate is the product head `fe657e3a68`. All runs are on the Mac, with the B side from round 2 (same commit, host, toolchain and scripts). The records are `R/I110/pressure_retire_06/_run_records/`, and the earlier rounds' are in `pressure_retire_02/` to `_05/`.

**Per-test outcomes** (`outcome_and_byte_diff.json`, `test_name_diff.txt`):

| Suite | B | Candidate | Changes |
|---|---|---|---|
| The 40 manifests (CI's numerical cargo profile) | 2,776 ok, 3 FAILED, 80 ignored | 2,755 ok, 3 FAILED, 80 ignored | 57: 39 removed and 18 added. They equal, name for name, the `#[test]` functions removed and added in source. No test changes outcome |
| src-tauri | 116 | 118 | +2: the retired label's test (I110) and `default_session_model_solves_in_both_modes` (I114) |
| `P/tests` pytest | 4,426 passed, 32 skipped | 4,428 passed, 32 skipped | +2: I114's record-truth test of the demo set, in both modes |
| vitest | 4,245 tests in 141 files | 4,250 tests in 141 files (4,249 passed, 1 todo, 0 failed) | 17 renames and additions (I110 and I114). The todo names T6's comparison defect |
| e2e: `r2-smoke`, `gui-workflow-validation`, `result-compatibility`, `ui-foundation`, both projects | — | **118/118 passed** | Playwright's bundled chrome-headless-shell, not installed Chrome; one worker. The B4 wheel-scroll tests are a known artefact of this host and are not in these specs |

- The 3 FAILED are the same tests at B and the candidate, all known Mac failures: PP `t13_committed_fallback_uz_is_byte_identical` and the runner's two load-reference frozen goldens. PR-N fixes them on main `ec5d397359`.
- The wasm engine build exits 0.

**Bytes** (`declared_text_check.txt`). A probe-only harness hashes each output of every document in both modes: PP's ordinary envelope, the runner's mechanics envelope, its export document and its unavailability. W1 adds the retained direct entry. An output passes when it is equal to B or differs only as follows:
- in declared strings: the normalized output, with each new string replaced by its old one, equals B;
- under ROOT's extended check: also the source-block receipt's two digests, recomputed by the product's rule from the normalized content.

| Set | Rows | Result |
|---|---|---|
| E (48 exact documents) | 96 | 96 equal |
| F (182 pressure-free implicit documents outside B1) | 364 | 332 equal. 6 differ only in a declared string: T1 (`result_export_v0_2.json`'s producer case 1, 2 rows) and V1 (case 67, 4 rows). 26 carry T2: 13 documents (the 12 source-block requests and `numerical_sensitive_torsion_model.json`) in both modes. Their ordinary and mechanics envelopes pass ROOT's extended check; their export documents pass as described below |
| B1 (32 documents) | 64 | 64 equal |
| W1 (B1 through the retained direct entry, debug build) | 64 | 64 equal: 34 successors and 30 ordinary publications |

- **H-1:** the count of `-0.0` tokens equals round 5's head on all 524 rows; 502 of them carry such tokens.
- **The export documents, beyond ROOT's two digests.** The runner's export document embeds the source's receipt. It also binds the source with two product hashes:
  - the source checksum: RE `derivative::digest` of the mechanics envelope, at four places (`raw_source_hashes`, `run_hashes` and both `source_origin_bindings` checksums);
  - `derivative_hash`: the digest of the document without it.

  Both change whenever a source byte changes. With them also recomputed by their own product rule from the normalized content, all 26 documents equal B. At the head without T2, the same procedure reproduces each document unchanged. ROOT's check names only the two receipt digests, so this is listed for ROOT (§8).

## 8. The reviews and gates

WORKING_ITEMS gives each verdict.

| Gate | Revision | Verdict |
|---|---|---|
| RV127: Stage 1 review | `4c0d5d7c00` | **FAIL on B-1 only** (1 BLOCKING, 1 SHOULD-FIX, 8 NOTE; `R/REVIEW_RV127/u3_stage1_01/REVIEW.md`). B-1 was repaired in `769c109d82` |
| RV127 addendum 01: Stage 2 and the repairs | `6b6543dc1e` | **PASS**, 0 BLOCKING, 2 SHOULD-FIX, 7 NOTE (`ADDENDUM_01.md`). It confirms the profile re-pin is exactly two atoms and that the validation removals were only the retired computation's oracles. A1-S-1 and A1-S-2 were done in round 4 |
| RV128: the demo lane | `180bf9b26d` | **PASS**, 0 BLOCKING, 2 SHOULD-FIX, 7 NOTE (`R/REVIEW_RV128/u3_demo_01/REVIEW.md`) |
| RV128 addendum 01 | `9744ed7e69` | **CONFIRMED**, 0 BLOCKING, 0 SHOULD-FIX, 5 NOTE |
| An independent review of rounds 4 to 6 on the final head: G11 extended, A1-S-1, the text corrections, T2's re-pin, the merge and the regenerated records | `fe657e3a68` | **pending** |
| ROOT: T2's carrier outside the ruled radius (`retained_precision_carrier_cases.json`'s one fixture hash; §6) | `11a026b628` | **pending** |
| ROOT: the export documents' two source bindings, recomputed alongside the receipt digests (§7) | `fe657e3a68` | **pending** |
| Pass B (I107, then RV124): a condition of the profile re-pin | the U3 head | **pending** |
| T9 and the both-entry gate: conditions of the profile re-pin | the U3 head | **pending** |
| Linux diagnostic dispatch 37830652481 | `4c0d5d7c00` | **success** |
| Linux diagnostic dispatch 37854458718 | `6b6543dc1e` | **success** |
| Linux and full-SHA dispatches, hosted CI on the PR, GEN-8, exact-head DEC-025 with src-tauri | the final head | **pending** |
| The merge with main `ec5d397359` (PR-N) | — | **pending** (WORKING_ITEMS). One conflict, in `PP/src/lib.rs`'s imports: main adds `correct_norm::{norm2, norm3}`; this branch drops `exact_rounded_sum`, whose last use was the legacy pressure path. Keeping both changes resolves it, and PP compiles with the resolution (`cargo check --tests`, the same warnings as main). Every other file merges cleanly |
| `source_equality.py` (checks 1 to 5) and `check_citations.py` (main's `IMPLEMENTATION/F2A_D1/` tools) | the package commit, as scratch heads: this branch with main `ec5d397359` merged (the PR side) and NUM `a31c14e4d3` with this branch merged (the integration side), each with the import resolution above | **PASS**: 133 maintained paths, equal in blob and mode; 4 execution files, all in this package, whose SHA256SUMS verify; citations 2 resolved, 0 unresolved. Recorded in `R/I110/pressure_retire_06/`. WORKING_ITEMS reruns both against its NUM commit |

**Follow-ups outside this PR:**
- the joint element's code is deleted with T4's corrected joint;
- the five governance documents' pressure statements go in their own PR, authorized and cut after this one merges;
- O1's friction-reversal coverage is re-authored on the valid demo model (A1-N-3);
- T4's wider wording waits for the next scheduled corpus generation;
- `buildPreviewComparison` goes to T6.
