# T0R S6 — integration and qualification record

WORKING_ITEMS (T0R manager), 2026-09-26. The candidate is branch `claude/inspiring-ptolemy-zxofd6`, over main `45a5381c8`. This record gives the state at the head named in §4. It records execution and evidence only. It is not engineering acceptance, issuance or release, and it closes no finding group.

## 1. Slices and commits

| Slice | Commits | Owner |
|---|---|---|
| S1 table and interface freeze | `cbec393f3`; amendments A1 `641086c47`, A2 `f660efcd4`, A3 `2140659e7`, A4 `2a3555f07` | manager |
| S2a producer core, with S2b (intensified measure) delivered alongside it rather than moved to T4 | `dcbd3cad7`; R1 repairs and M07 containment `641086c47` | manager (sole `core/product_physics` writer) |
| Report-package wire refusal (defence in depth) | `053850a86` | manager |
| S3 Rust readers, rule binding, headless, src-tauri | `a55d0faa5`, `3a73353f3` (the message says uncompiled, but it has since compiled and passed on this host, §4), and within `887902ace` and `9ab46503d` | TASK S3 |
| S4 Python readers, schemas, packaging, generator mode | `f41d676ae`, and within `887902ace` and `9ab46503d` | TASK S4 |
| S5 TS readers and text-only UI | `ba5ffba12`, and within `887902ace` and `9ab46503d` | TASK S5 |
| Shared id and tamper vectors | `68465df12`; `f660efcd4`, `2140659e7` and `2a3555f07` (vector) | manager |
| Records and briefs | `df1b423ec`, `6cdace086`, `e348a81b2`, and this record's commit | manager |

The producer, readers and supersession gates are in one candidate (SF-3).

## 2. Decisions and deviations, all ROOT-accepted

- **S1 arc discriminator.** The discriminator is a first-match `source_basis` variant placed before the generic variant, because combination rows relabel `metadata.basis` (S1 §1).
- **Joint element (M07 exposure found by R1 N-1).** A realized user-stiffness joint with lateral stiffness over a length has no rigid-body moment coupling. On the invented demo, L-100's support actions miss moment balance by 658.44 N·m = 900000 × 3.3254e-4 × 2.2. The ordinary route now refuses it with blocking `JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED`. The exact routes already refuse every component. The repair is M07 in T4. The frame kernel requires all four joint stiffnesses to be positive, so an axial-only or rotational-only joint cannot be realized today.
- **Blocked envelopes** (R1 SF-1). Retired codes are dropped, and non-blocking diagnostics that name rows are dropped. Blocking diagnostics are kept, with their `result:` refs stripped.
- **Tangent warning** (R1 N-5). It is judged against the closest adjacent pipe.
- **Runtime-test criterion** (R1 N-2). Nonzero expectations use pure relative 1e-9. Exact zeros use only frozen zero scales. Where no zero scale is frozen, the expectation must be exactly 0.0. Every such case passed with an exact 0.0, so nothing had to be reported.
- **Frozen references in CI** (ROOT). `core/product_physics/tests/fixtures/preview_physics/references.stdout.txt` is a byte-identical copy of the frozen file. A test pins it to sha256 `1d1bdeed…` from `_run_records/SHA256SUMS`. `references.py` and the original output file are unchanged.
- **Reader parity** (R2 SF-1 and the R2 backcheck). There is one strict list: S1 §9 plus A1, A2, A3 and A4. The shared tamper vector (22 entries) is tested in all three readers with identical outcomes. The shared unicode id vector is tested in all three. Integer fields are checked by numeric value (A4), because JavaScript cannot tell `4.0` from `4`.
- **Last parity round** (ROOT, on A4). A4 is the last reader-parity round for T0R. Honest output already reads identically in all three readers. Any further malformed-input difference found after A4 goes to T6 as hardening, with one generative cross-language parity harness, rather than another T0R round.
- **knownSemanticLimitations.test.ts** (R2 backcheck note, ROOT-accepted). The TS assertion for a validated mixed source-blocks-1 standing now covers only the receipt check. Rust covers the validated-mixed path, including the legacy-semantics reason.
- **Standing order** (A2 item 10). Validation runs first everywhere. This moves the Rust T0R early return below `for_source`. At the T1 merge, ROOT's hand merge (N-B) follows the same order.
- **Report outage.** The TS gate refuses by identity. The Rust wire gate (`report_package/src/wire.rs`, used by `save_report_package` and headless export-results) refuses by fresh-only metadata content, with the identical reason text, as defence in depth. Neither the calculation-report renderer nor the Python report_packet_preview is a report package.
- **`rule_check_runner::RuleCheckRunInput`** is not a serde type, so `refused_solver_results` is a plain field set at every construction site.
- **Historical scope.** `historical_pressure_reference::with_scope`, a private test-only scope, now also admits the refused joint premise. Four named tests keep their frozen oracles under it, and the joint-subject tests first assert the ordinary-route refusal.

## 3. Test dispositions (for review)

- **C-150 test basis** (`core/product_physics/src/lib.rs` tests). `request()` and `mechanical_fixture_for_test` omit the demo's joint C-150. `request_with_refused_joint()` keeps it. The joint-premise tests are:
  - `current_composite_derived_normal_friction_and_reversal`
  - `valid_invented_model_exposes_nonlinear_support_loop_evidence_historical_pressure_premise`
  - `expansion_joint_user_stiffness_emits_macro_element_review_rows`, which now first asserts the refusal
  - `expansion_joint_pressure_thrust_uses_user_effective_area_as_load_side_evidence_historical_pressure_premise`

  These run under the historical scope. Every other test that uses the helpers keeps its assertions unchanged on the joint-free basis. The R2 review built the full old→new table.
- **Retired-row migrations.** Each is marked `T0R` in comments. They cover:
  - `reaction_resultant` → the v2 force magnitude or signed components;
  - `open_formula_stress_summary` → the certified maximum;
  - SIF×k rows → the equal-factor rows;
  - arc labels;
  - B-1: nonlinear combination rows are now withheld;
  - the precision-1 header → preview-physics-1;
  - the case-edge count: 24 edges become 16, and every edge must now resolve.
- **self_weight_wasm** (SF-C). The five root-reaction assertions now check the signed root Fy, with Fx = Fz = 0. **physics_audit_regression** (R-3). The zero-force assertion is kept, and the signed Mz = −100 N·m is added.
- **src-tauri demo-solve migrations** (S3). These tests fail on base because of the PR905 legacy-pressure refusal:

  | Test | Old | New |
  |---|---|---|
  | `solve_job_seam_completes_and_reports_result_without_cancellation` | bundled demo; completed, rows > 0 | derived model; same assertions. Companion: the default load is refused (`PRESSURE_MODEL_REAUTHOR_REQUIRED`) |
  | `run_preview_mechanics_uses_supplied_model_payload` | bundled demo with edits; SOLVED, rows | derived model with the same edits; same assertions. Companion: `run_preview_mechanics(None)` is refused |
  | `solve_job_seam_uses_supplied_model_payload` | bundled with edits via the job | derived; same assertions |
  | `solve_job_cancellation_after_completion_is_rejected_without_success_claim` | bundled | derived; same assertions. This test passes on base too |
  | `apply_model_operation_command_applies_inspector_intent_to_bundled_fixture_model` | apply on bundled; solve rows non-empty | the bundled apply assertions are unchanged, and the applied bundled demo is asserted refused. The same intent on the derived model solves with rows |
  | `saved_edited_load_model_round_trips_and_solves_from_restored_payload` | bundled; round trip; restored solve SOLVED | derived; the baseline also asserts SOLVED; the rest is unchanged |

- **The 14 report-package tests** (S5). None is skipped.
  - 12 are re-homed to a legacy 0.1.0 carrier with a 0.2 analysis run, through the public path.
  - 2 (producer header against recorded identity) assert the public refusal `REPORT-PACKAGE-PRECISION-1-HISTORICAL: <N-P1>` and exercise the original check below the gate, through the seam `assembleReportPackageRequestBelowAvailabilityGate`. No production code calls the seam.
  - The App save-race cases assert the N-REPORT gate refusal and no save call. They then run the unchanged sequencer scenario with the gate lifted in the test only.
- **physicsSourceIntegration isolation fix** (test-only). Teardown is scoped to a generation, so a timed-out case can no longer drive the next one. No timeout was raised.
- **Precision-1 standing expectations** in Rust, Python and TS move from eligible to `needs_recompute`, with the reason. T0R retires precision-1 as Current.

## 4. Checks

Final head: `9ab46503d`. Every run below is on this head with a clean tree, run one at a time while T1 held heavy work.

- **DEC-025 local sweep**, `run_evidence_sweep.py --execute --only-capability sandboxed`, overall **pass**. It ran 08:56:10–09:18:08 UTC (1318 s). The summary is `_run_records/SWEEP_20260926T085610Z_9ab46503deb1.json`, with `working_tree_dirty: false`.
  - Surface 1, cargo crate sweep: 39 crates, 120 test binaries, **1503 passed, 0 failed**, 1 ignored.
  - Surface 2, pytest: **1890 passed, 0 failed**, 15 skipped, and 130 subtests passed.
  - Surface 3, vitest: the wasm build first, then **121 files and 2138 tests passed, 0 failed**, with no timeouts.
  - Surface 5, production build (`tsc -b && vite build`): **pass**.
  - Surface 4, Playwright dual-viewport: not run locally (§8). ROOT runs it as the hosted dispatch.
- **src-tauri full suite**, compiled on this host (§8, WebKitGTK):
  - candidate `9ab46503d`: **109 passed, 0 failed**, 0 ignored, compiled from this head with `CARGO_INCREMENTAL=0`;
  - base `45a5381c8`: **103 passed, 5 failed**. The 5 are the pre-existing PR905 demo-solve failures listed in §3.
- **physicsSourceIntegration per-case timings**, candidate against base, run alternately (`_run_records/S6_TIMINGS.txt`):
  - 17 of 17 passed in all four runs;
  - mean total: base 135.2 s, candidate 131.6 s (ratio 0.974);
  - per-case ratios run from 0.924 to 1.022;
  - the slowest case is 17.03 s, against the 30 s default. No timeout was raised, and no hot-path fix was needed, since ROOT's trigger was 20% slower.
- **Tamper vector** (22 entries), A4 targeted runs:
  - Rust result_export: 81 passed.
  - Python consumer contract: 140 passed.
  - TS: tsc exits 0; previewPhysicsEvidence, knownSemanticLimitations and KnownSemanticNotices give 67 passed.
  - The independent A4 backcheck is CLEAR (§9).
- **Frozen inputs.** `references.py`, `_run_records/references.stdout.txt` and `SHA256SUMS` are unchanged against main. The CI copy is pinned by sha256.

## 5. Consumer sites beyond DESIGN §6

- `core/reporting/report_package/src/wire.rs` (wire gate)
- `core/runner/headless/src/result_envelope_binding.rs`
- the src-tauri demo-solve tests
- `StressNeutralExportPanel`, `resultExportAdapter`, `RuleCheckRunPanel`
- `historical_pressure_reference.rs`
- `validation/witness` has no solving consumer

## 6. Findings and follow-ups (none closes a group)

- **M07.** Joint moment coupling: contained here, repaired in T4 (ROOT recorded).
- **M27.**
  - The src-tauri suite ran nowhere, not in hosted CI and not in the DEC-025 cargo sweep, which discovered 39 crates and no src-tauri. As a result 5 tests had been failing since PR905. They are fixed here.
  - The physicsSourceIntegration and App tests sit close to the 30 s default under load. The unloaded timings are in §4: the slowest case is 17.03 s, and the candidate is no slower than base.
- **T6 must restore coverage lost during the outage** (R2 N5): the rule_check_aggregate null check, comparison units, joint unit normalization, and the strict 0.3.0 report refusals.
- **T6 results hardening** (A3 note, narrowed by A4). A4 now checks that every diagnostic has an id and that `basis_ref` has a closed shape. The readers still do not check these invariants:
  - intensified `source_result_refs` being exactly the two bending rows;
  - load-case rows lying in a preview case;
  - the `basis_ref` `ref_type` value;
  - unique diagnostic ids.
- **T6 parity harness** (ROOT, on A4). A4 was the last T0R parity round. Any further malformed-input difference goes to T6, which adds one generative cross-language parity harness. The A4 backcheck's note N9 is also recorded here: a producer header with an extra key is refused only on Rust's full `for_source` path, which every admission uses, so outcomes match.
- **T4.** M07 joint-element moment coupling repair (above). S2b was delivered in T0R, so T4 does not take it over.
- **R1 N-4.** The attribution-withholding path is unreachable end to end, because both ambiguous shapes are refused upstream. It is pinned by a unit test and by reader tamper tests.
- **R1 N-5.** SF-E also covers exact 0.3.0 blocked envelopes, as the design wording says.
- **T1 merge (R1 N-3).**
  - A 3-way merge of lib.rs has 0 textual conflicts, but two T1-added `LoadCaseSolve` early-return literals need `preview: None`.
  - The standing early return merges by hand with T1's `load-reference-source-1` return (N-B), validation first.
  - T1 obligations (DESIGN §10.3): add `load-reference-1` and `load-reference-source-1` to the static fresh set in all three languages, and keep 0.4.0 exact-route only.
  - A blocked 0.4.0 envelope currently gets the preview id until T1 routes it.
- **Pre-existing warnings.** product_physics has an unused import (`source_recovery.rs`) and a dead fn (`is_exact_pressure_result_kind`). Neither is T0R.

## 7. Native witnesses outstanding (owner's Mac)

- **Build and suite.** Build the macOS app from the frozen candidate and run the full src-tauri test suite there.
- **DESIGN §9.3.** Record the candidate, the model hash and the case ids for each of these:
  - A fresh 0.2.0 model with a marker, a combination and one nonlinear support: the combination is withheld with its reason, and the Current standing is recorded.
  - A rule pack bound to the new maximum or intensified id.
  - A rule pack bound to a retired id reports `RULE_INPUTS_INCOMPLETE` and shows N-RULE-RETIRED.
  - Exports, with the report package unavailable (N-REPORT), and N-SB shown in the export UI for an all-selected source-blocks-1 result.
  - Save and reopen. A precision-1 result reopens with N-P1 and is not Current.
  - An arc model: the headline is withheld, and the kink warning appears when `y_reference` is inconsistent.
  - An imposed-displacement load on a support shows the refusal.
  - A model with a realized expansion joint shows the `JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED` refusal.
  - The packaged binary self-test runs, with its signed support witness.
- **After those.** Hosted CI, including the full dual-viewport dispatch (surface 4), and a clean DEC-025 sweep on the merging revision (ROOT).

## 8. Environment and results
- **Toolchain.** rustc and cargo 1.97.1, on the repo's rustup default. Node 24.21.0, vitest 4.1.10. Python 3.11.15 in the DEC-025 venv.
- **WebKitGTK.** ROOT installed `libwebkit2gtk-4.1-dev` 2.52.6. This is what lets src-tauri compile and run its tests on this Linux host. It is not part of the DEC-025 sweep, which discovers 39 crates and does not include src-tauri.
- **wasm32.** S5 installed the `wasm32-unknown-unknown` rustup target. The vitest surface's `build:wasm:desktop` step needs it.
- **Chromium override.** Surface 4 (Playwright) is not run locally, because the sweep's surface-4 preflight fails: Playwright's browser revision 1223 is absent on this host. `playwright.config.ts` accepts a `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` override for ad hoc local runs, but this record claims no local surface-4 evidence. The local sweep therefore uses `--only-capability sandboxed` (surfaces 1, 2, 3 and 5). Surface 4 is ROOT's hosted dual-viewport dispatch.
- **src-tauri shared-target collision.** The first src-tauri compile on the final head failed with E0308, reporting two `serde_json` versions (1.0.149 and 1.0.151).
  - This was an environment artifact, not a code defect. `self_weight_wasm` is a cdylib crate, so its lib artifact has no metadata hash. The sweep compiled that crate from its own lockfile, which pins serde_json 1.0.151, and overwrote the unhashed rlib in the shared target. src-tauri's lockfile resolves only 1.0.149 (`cargo tree -d`), and its fingerprint still read that unit as fresh.
  - After I deleted that one crate's fingerprints and artifacts, src-tauri rebuilt it against its own lock and passed 109 of 109.
  - Hosted CI builds src-tauri in its own target, so it is not exposed to this.
- **Targets.** There was one shared scratch `CARGO_TARGET_DIR` outside the repository, with `CARGO_INCREMENTAL=0` for the final runs. Disk was limited, so incremental directories, stale test binaries and the base-revision src-tauri binaries were pruned before the final runs.

## 9. Reviews and backchecks (independent, non-author TASKs)

| Review | Scope | Verdict | Dispositions |
|---|---|---|---|
| S0 reference refutation (before T0R implementation) | frozen references | DISCREPANCIES, none numeric: 102 values agree at 1e-9 | Handled in the design phase before the freeze. See the design records. The references have not changed since they were frozen. |
| Design review, revision 1, with two backchecks (before T0R implementation) | DESIGN option (d) | BLOCKING (B-1, B-2), then resolved in revision 3 and ROOT_SELECTION | Recorded in `REVIEW/RETURN.md`, `REVIEW/BACKCHECK_R2.md` and `ROOT_SELECTION.md`. This candidate implements B-1: nonlinear combinations are withheld with `NONLINEAR_COMBINATION_REQUIRES_SOLVE`. It also implements the source-blocks-1 containment: a mixed envelope is refused as Current with `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`, and source-blocks-1 bytes are unchanged. F-1, N-A and N-B are applied as ruled. |
| R1 producer | `cbec393f3..dcbd3cad7` | FINDINGS: 1 should-fix, no blocking | **SF-1** (blocked envelope carries a retired code and dangling refs): fixed in `641086c47`, with blocking diagnostics kept and refs stripped, plus a runtime test. **N-1** (joint moment coupling, M07): ROOT option (a), refused on the ordinary route; repair in T4. **N-2** (scales): the runtime criterion was tightened (§2). **N-3** (T1 merge): recorded in §6. **N-4**: recorded in §6. **N-5**: tangent check uses the closest pipe; the rest recorded. |
| R2 complete diff | `45a5381c8..ba5ffba12`, 94 files | FINDINGS: 2 should-fix, 10 notes, no blocking, no silent-wrong exposure | **SF-1** (reader parity): A2 and A3 plus the shared tamper vector (`f660efcd4`, `2140659e7`, `887902ace`). **SF-2** (slow notice test): split, with no timeout raised. **N1**: standing validates first in all three readers and in src-tauri. **N3**: wire note in A2. **N4**: comments. **N5**: coverage restored in T6 (§6). **N6**: S1 §11 corrected. **N8**: skip replaced by a hard assert. The other notes are recorded. |
| R2 backcheck | `e348a81b2..887902ace` | FINDINGS: 1 should-fix (9 residual malformed-input differences), 3 notes | A4 (`2a3555f07`, `9ab46503d`). The knownSemanticLimitations note is accepted (§2). |
| A4 backcheck | `887902ace..9ab46503d` | **CLEAR** | 22 of 22 vector entries agree in Rust, Python and TS. A 57-variant probe found no difference. N9 (extra producer-header key) is refused only by Rust's full `for_source` path, which every admission uses; outcomes match. |
