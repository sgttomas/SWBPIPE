# R1 Notes — Verification Index (test-suite inventory)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305 · Phase R1 (read-only inventory, plan §8 R1)
Frozen source state: `551f84ef6be656f1603ce0acfa5e3935aa9683c7` at `.claude-worktrees/piping-frozen-551f84ef6`, project root `projects/chirality-piping`. All paths below are repo-relative to that project root.

## Method

- **Static counting only.** No test suite was executed for this index (no `EXECUTED_AT_FROZEN` rows). Counts are grep-derived from the frozen tree:
  - Rust: `#[test]` occurrences under each crate's `src/` (unit) and `tests/` (integration). No `#[tokio::test]`, `#[rstest]`, `#[wasm_bindgen_test]`, or `#[test_case]` variants exist in the tree. No doc-tests: zero fenced code blocks in any crate's `src/` rustdoc.
  - Python: `def test_` definitions (pytest-collectable) per file under `tests/**`.
  - TypeScript: line-anchored `it(`/`test(` call sites per Vitest/Playwright file (regex-method `.test(` calls verified and excluded).
- **Evidence binding.** The latest all-pass DEC-025 sweep `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json` records execution at commit `e648462f1d0521e26df15d04a988391343018886` (clean tree), which is an **ancestor of the frozen SHA**. `git diff e648462f1d05..551f84ef6` over `projects/chirality-piping` touches ONLY: `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/TYPES.md`, 105 `execution/**/_STATUS.md` files, 5 `execution/_Coordination|_Decomposition` governance files, `loop/LOOP_RECEIPTS.md`, `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md`, and the sweep JSON itself. **No `core/`, `apps/`, `tests/`, `tools/`, `schemas/`, `fixtures/`, `api/`, `examples/`, `validation/{benchmarks,witness,hand_calcs}` or package manifest path differs.** Rows citing that sweep therefore carry the addendum-10 qualifier `CONTENT_IDENTICAL (content-identical at frozen SHA … (diff empty over <suite input paths>))` — a recorded citation, not a frozen-SHA execution (convention 7).
- **Exception kept honest:** `tests/test_coordination_maintenance.py` contains `test_live_status_discovery_command_passes`, which runs `tools/coordination/list_deliverable_status.py` against the LIVE `execution/` tree; since 105 `_STATUS.md` inputs differ from the sweep commit, that one suite is `RECORDED_ONLY` (no content-identity claim).
- **Side-effect audit:** frozen worktree verified clean (`git status --porcelain` empty) before and after inventory; only `git` read commands and file reads were run inside it (R1_CONVENTIONS addendum 9 not needed — nothing re-executed).

## Count summary by framework

| Framework | Suites (CSV rows) | Static test count |
|---|---|---|
| Rust `cargo test` (33 `core/**` + 3 `validation/benchmarks/*` + 1 `apps/desktop/src-tauri`) | 37 | 841 |
| Python pytest (`tests/**`, incl. `security/`, `product_preview/`) | 76 | 459 (collected) |
| Vitest (apps/desktop/src) | 22 | 282 |
| Playwright e2e (apps/desktop/e2e) | 2 | 10 |
| Tool/validator instruments (tools/**, witness validator) | 10 | — (their pytest wrappers are counted in the pytest rows) |
| **Total** | **147** | **1592** |

Rust split: 779 tests in the 36 sweep-covered crates (`core/**` ×33 + `validation/benchmarks/*` ×3) + 62 in `apps/desktop/src-tauri` (not sweep-covered).

## Suites with NO recorded execution evidence

1. `apps/desktop/src-tauri` (RUST-37, 62 `#[test]`): the DEC-025 sweep's cargo surface discovers manifests only under `core/` and `validation/benchmarks/` (`CARGO_SEARCH_ROOTS` in `tools/release/check_release_readiness.py`); no run record for this crate's tests was located anywhere in the frozen tree.
2. Nine script-style checker files in `tests/` that define `main()` + module functions but **zero pytest-collectable tests** — `pytest -q tests` (the sweep's python surface) imports them but executes no assertions, and no direct `python tests/<file>.py` run record was located:
   - `tests/test_analysis_run_schema.py`
   - `tests/test_api_boundary_contract.py`
   - `tests/test_constraint_schema.py`
   - `tests/test_design_knowledge_schema.py`
   - `tests/test_handoff_package_schema.py`
   - `tests/test_model_operation_schema.py`
   - `tests/test_report_generator_contract.py`
   - `tests/test_report_protected_content_linter.py`
   - `tests/test_report_sections_contract.py`
   (`tests/schema_validation.py` is a shared jsonschema helper module, not a suite; excluded from the CSV.)
3. `tools/validation/validate_dependencies_schema.py` (TOOL-01): executed only by readiness profiles `skeleton`/`python`/`all`; sweeps run the `cargo` profile only, and no recorded run of the other profiles was found.
4. `tools/release/run_release_candidate_scan.py` (TOOL-06): its record home `validation/evidence/releases/` does not exist in the frozen tree.
5. `tools/release/export_public_openpipestress.py` (TOOL-08): no export run record located (its pytest wrapper is recorded).

## Anomalies and observations

- **Sweep cargo surface under-covers Rust:** `cargo_crate_sweep` executes 36 crates; the desktop Tauri crate (62 tests) is structurally outside it (item 1 above).
- **Script-style pytest files silently no-op under the sweep** (item 2 above): the python surface reports pass while collecting 0 tests from those 9 files. They carry substantial static assertion bodies (schema/contract checks) reachable only via direct invocation.
- **R1 named repair context (DEL-09-01):** not part of this index's scope; fixture/witness count correction is an R2 wave-ledger item per R1_CONVENTIONS Part C.
- **CI does not run this project's suites:** repo-root `.github/workflows/` runs only the root governance harness (`tools/practitioner_harness`, outside this project) — all suite evidence is local sweep/gate/coverage records.
- **Gate records:** the five DEC-025 gate-family records under `validation/evidence/gates/` (`GATE_*_20260711T032542Z_e2ea37194c8a.json`) evaluate commit `e2ea37194c8acf8e4a77c5fb2fecf92adb1c939c` (whose sweep passed all 5 surfaces) and were emitted at `6dcb328ac5cf789fefc0586af00b59cd2ae9e819`; both commits are ancestors of the frozen SHA. Recorded citations only.
- **Coverage telemetry:** `COVERAGE_20260710T232606Z_e9cd806811b3.json` (DEC-060, recorded-never-blocking) at ancestor commit `e9cd806811b3…dfb`.
- **Release artifact record:** `RELEASE_ARTIFACT_20260711T030046Z_8e436704b52b.json` present; explicitly not a release claim per its boundary note.
- **Playwright lane split:** `playwright.config.ts` (dev-server lane) ignores `**/*-dist.spec.ts`; `playwright.dist.config.ts` matches only them — so `r2-smoke.spec.ts` (9 cases) and `wasm-engine-dist.spec.ts` (1 case) are disjoint lanes, both exercised by sweep surface `desktop_playwright_e2e`.
- **Excluded from inventory:** build outputs/caches (`target/`, `node_modules/`, `dist/`), `tools/release/generate_app_icon.py` (asset generator, not a verification instrument), `provenance/build-artifacts` (build provenance, not a test suite), and `docs/validation_manual/cases/generate_validation_case_pages.py` (doc generator).

## Appendix — DEC-025 five-surface sweep records (274 files, `validation/evidence/sweeps/`)

Totals: 265 pass / 9 with at least one failed surface; 94 recorded against a dirty working tree (filename `-dirty` suffix; weaker evidence), 180 clean. Commit bindings below are the sweep's own recorded `git.commit_hash` (first 12 shown; full hash inside each JSON). These are recorded executions at their own commits — none is an execution at the frozen SHA (`not re-executed at frozen SHA 551f84ef6`).

| Sweep file | Commit | Tree | Result |
|---|---|---|---|
| SWEEP_20260612T031241Z_0f402fc48424-dirty.json | 0f402fc48424 | dirty | pass |
| SWEEP_20260612T031643Z_a69a86790f03.json | a69a86790f03 | clean | pass |
| SWEEP_20260612T035121Z_cc82a75de872-dirty.json | cc82a75de872 | dirty | pass |
| SWEEP_20260612T043906Z_6e36f5da47ec-dirty.json | 6e36f5da47ec | dirty | pass |
| SWEEP_20260612T044123Z_519dc2e159c2-dirty.json | 519dc2e159c2 | dirty | pass |
| SWEEP_20260612T053738Z_0b7a24c93e2c-dirty.json | 0b7a24c93e2c | dirty | pass |
| SWEEP_20260612T055357Z_bd8d8b1d6081-dirty.json | bd8d8b1d6081 | dirty | pass |
| SWEEP_20260612T061708Z_d9fad1315e82.json | d9fad1315e82 | clean | pass |
| SWEEP_20260612T063124Z_9f82206cfa45.json | 9f82206cfa45 | clean | pass |
| SWEEP_20260612T064059Z_b72d5b4bf049.json | b72d5b4bf049 | clean | pass |
| SWEEP_20260612T065252Z_547f90f48a14.json | 547f90f48a14 | clean | pass |
| SWEEP_20260612T070630Z_645b73d9d1ab.json | 645b73d9d1ab | clean | pass |
| SWEEP_20260612T071501Z_f58cb8452188.json | f58cb8452188 | clean | pass |
| SWEEP_20260612T072559Z_d6c4f3e87efe.json | d6c4f3e87efe | clean | pass |
| SWEEP_20260612T074248Z_9f3c8491ee6e.json | 9f3c8491ee6e | clean | pass |
| SWEEP_20260612T074922Z_7c8585bd22f6.json | 7c8585bd22f6 | clean | pass |
| SWEEP_20260612T080331Z_ede8dd424381.json | ede8dd424381 | clean | pass |
| SWEEP_20260612T081346Z_4270cbeef9cc.json | 4270cbeef9cc | clean | pass |
| SWEEP_20260612T082712Z_06d1e6456104.json | 06d1e6456104 | clean | pass |
| SWEEP_20260612T083919Z_067b5749a57b.json | 067b5749a57b | clean | pass |
| SWEEP_20260612T084306Z_594a5fc17585.json | 594a5fc17585 | clean | pass |
| SWEEP_20260612T085121Z_c22a3d88ffcd.json | c22a3d88ffcd | clean | pass |
| SWEEP_20260612T085843Z_f5a8c9a585c6.json | f5a8c9a585c6 | clean | pass |
| SWEEP_20260612T090335Z_0dbbb8972311.json | 0dbbb8972311 | clean | pass |
| SWEEP_20260612T091402Z_7bf6795365e8.json | 7bf6795365e8 | clean | pass |
| SWEEP_20260612T092120Z_926ea48957a8.json | 926ea48957a8 | clean | pass |
| SWEEP_20260612T093217Z_a2ca746200c9.json | a2ca746200c9 | clean | pass |
| SWEEP_20260612T094710Z_f7d973685f9e.json | f7d973685f9e | clean | pass |
| SWEEP_20260612T095550Z_1061c6b8f2ac.json | 1061c6b8f2ac | clean | pass |
| SWEEP_20260612T100813Z_bd1d9e766af6.json | bd1d9e766af6 | clean | pass |
| SWEEP_20260612T101756Z_2c4334ad1d1f.json | 2c4334ad1d1f | clean | pass |
| SWEEP_20260612T103045Z_ff5dd7cb9ba4.json | ff5dd7cb9ba4 | clean | pass |
| SWEEP_20260612T104250Z_4e21bf9baa78.json | 4e21bf9baa78 | clean | pass |
| SWEEP_20260612T142244Z_898da32565f7.json | 898da32565f7 | clean | pass |
| SWEEP_20260612T163436Z_78f618e09638.json | 78f618e09638 | clean | pass |
| SWEEP_20260612T184117Z_f0868120eba5.json | f0868120eba5 | clean | pass |
| SWEEP_20260612T201615Z_333c76320986.json | 333c76320986 | clean | pass |
| SWEEP_20260613T001653Z_e05527c2c283.json | e05527c2c283 | clean | pass |
| SWEEP_20260613T022515Z_7f0c09490d70-dirty.json | 7f0c09490d70 | dirty | pass |
| SWEEP_20260613T200943Z_add584756bb1-dirty.json | add584756bb1 | dirty | FAIL(desktop_vitest,desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260613T210818Z_6e27d417909c-dirty.json | 6e27d417909c | dirty | pass |
| SWEEP_20260613T215015Z_2217cc390851-dirty.json | 2217cc390851 | dirty | pass |
| SWEEP_20260613T234909Z_4d4a4a7ee955-dirty.json | 4d4a4a7ee955 | dirty | pass |
| SWEEP_20260614T004823Z_0da28bd79613-dirty.json | 0da28bd79613 | dirty | pass |
| SWEEP_20260614T010906Z_dc8e4fc884a1-dirty.json | dc8e4fc884a1 | dirty | pass |
| SWEEP_20260614T012831Z_a9ffd8023550-dirty.json | a9ffd8023550 | dirty | pass |
| SWEEP_20260614T061915Z_e473cca3323b-dirty.json | e473cca3323b | dirty | pass |
| SWEEP_20260614T064824Z_953c63dd98de-dirty.json | 953c63dd98de | dirty | pass |
| SWEEP_20260614T071458Z_97829f6d4603-dirty.json | 97829f6d4603 | dirty | pass |
| SWEEP_20260614T073613Z_9c755db288f7-dirty.json | 9c755db288f7 | dirty | pass |
| SWEEP_20260614T185306Z_ebfac31d79dd-dirty.json | ebfac31d79dd | dirty | pass |
| SWEEP_20260614T194532Z_a0625a0d5717-dirty.json | a0625a0d5717 | dirty | pass |
| SWEEP_20260614T205229Z_8aea77ec988f-dirty.json | 8aea77ec988f | dirty | pass |
| SWEEP_20260614T213434Z_3c2c0eb87c71-dirty.json | 3c2c0eb87c71 | dirty | pass |
| SWEEP_20260615T000741Z_625b039f8310-dirty.json | 625b039f8310 | dirty | pass |
| SWEEP_20260615T021424Z_3663a754706e-dirty.json | 3663a754706e | dirty | pass |
| SWEEP_20260615T033429Z_c161a7053cb5-dirty.json | c161a7053cb5 | dirty | pass |
| SWEEP_20260615T043935Z_acd0f6bfbdbf-dirty.json | acd0f6bfbdbf | dirty | pass |
| SWEEP_20260615T065727Z_31cd9ece4289-dirty.json | 31cd9ece4289 | dirty | pass |
| SWEEP_20260615T171142Z_e239471d3b20-dirty.json | e239471d3b20 | dirty | pass |
| SWEEP_20260615T204515Z_06ca76e7cb9a.json | 06ca76e7cb9a | clean | pass |
| SWEEP_20260615T210910Z_fff64041a898.json | fff64041a898 | clean | pass |
| SWEEP_20260615T214645Z_b29de6990da0.json | b29de6990da0 | clean | pass |
| SWEEP_20260616T000215Z_6105c3415796.json | 6105c3415796 | clean | pass |
| SWEEP_20260616T001724Z_d02886bddd97.json | d02886bddd97 | clean | pass |
| SWEEP_20260616T002614Z_7d2f1e1e4e57.json | 7d2f1e1e4e57 | clean | pass |
| SWEEP_20260616T004409Z_c236d06cd993.json | c236d06cd993 | clean | pass |
| SWEEP_20260616T004652Z_0c8645879a4e.json | 0c8645879a4e | clean | pass |
| SWEEP_20260616T005749Z_c48813dc97e8.json | c48813dc97e8 | clean | pass |
| SWEEP_20260616T010702Z_9e1b4eb1564c.json | 9e1b4eb1564c | clean | pass |
| SWEEP_20260616T011545Z_9dfca230e769.json | 9dfca230e769 | clean | pass |
| SWEEP_20260616T012836Z_b5a69a10898e-dirty.json | b5a69a10898e | dirty | pass |
| SWEEP_20260616T013908Z_673a4de9628e-dirty.json | 673a4de9628e | dirty | pass |
| SWEEP_20260616T014325Z_c873b821aa8f.json | c873b821aa8f | clean | pass |
| SWEEP_20260616T025334Z_92ba64e2b4e3-dirty.json | 92ba64e2b4e3 | dirty | pass |
| SWEEP_20260616T025633Z_826dcdd4e773.json | 826dcdd4e773 | clean | pass |
| SWEEP_20260616T031013Z_b431a1676620-dirty.json | b431a1676620 | dirty | pass |
| SWEEP_20260616T031411Z_c12d924cb4d8.json | c12d924cb4d8 | clean | pass |
| SWEEP_20260616T033649Z_40c8d2530ca8-dirty.json | 40c8d2530ca8 | dirty | pass |
| SWEEP_20260616T033935Z_19c990872dbd.json | 19c990872dbd | clean | pass |
| SWEEP_20260616T034816Z_9be2d805ab17-dirty.json | 9be2d805ab17 | dirty | pass |
| SWEEP_20260616T035138Z_43b95ab14a50.json | 43b95ab14a50 | clean | pass |
| SWEEP_20260616T154222Z_e6b34b9848f8.json | e6b34b9848f8 | clean | pass |
| SWEEP_20260616T163207Z_8c8f7a6e7b49.json | 8c8f7a6e7b49 | clean | pass |
| SWEEP_20260616T170957Z_4d04aef8882e.json | 4d04aef8882e | clean | pass |
| SWEEP_20260616T174614Z_a441aeb44fe9.json | a441aeb44fe9 | clean | pass |
| SWEEP_20260616T181031Z_2495f89e7933.json | 2495f89e7933 | clean | pass |
| SWEEP_20260616T182954Z_962e565b2d94-dirty.json | 962e565b2d94 | dirty | pass |
| SWEEP_20260617T200823Z_1b0780d0d509.json | 1b0780d0d509 | clean | pass |
| SWEEP_20260617T201751Z_48500e138e12.json | 48500e138e12 | clean | pass |
| SWEEP_20260617T202603Z_be7c5cec0260.json | be7c5cec0260 | clean | pass |
| SWEEP_20260617T203552Z_fa76dd0ef161.json | fa76dd0ef161 | clean | pass |
| SWEEP_20260617T204609Z_1cf3d2120eb4.json | 1cf3d2120eb4 | clean | pass |
| SWEEP_20260617T205527Z_f41257f62d08.json | f41257f62d08 | clean | pass |
| SWEEP_20260617T210336Z_f79a8070d1d0.json | f79a8070d1d0 | clean | pass |
| SWEEP_20260617T232320Z_dcdb21bf91d9.json | dcdb21bf91d9 | clean | pass |
| SWEEP_20260617T233318Z_e3c245cf625b.json | e3c245cf625b | clean | pass |
| SWEEP_20260617T234352Z_321c27aedc46.json | 321c27aedc46 | clean | pass |
| SWEEP_20260618T000226Z_4deb836a4f1a.json | 4deb836a4f1a | clean | pass |
| SWEEP_20260618T001130Z_39b82243f73f.json | 39b82243f73f | clean | pass |
| SWEEP_20260618T002114Z_bb0ba6918d7d.json | bb0ba6918d7d | clean | pass |
| SWEEP_20260618T003217Z_b76c853c5d04.json | b76c853c5d04 | clean | pass |
| SWEEP_20260618T004246Z_93804b8c20d3.json | 93804b8c20d3 | clean | pass |
| SWEEP_20260618T005606Z_07b73146e886.json | 07b73146e886 | clean | pass |
| SWEEP_20260618T010513Z_63df39cb8d9c.json | 63df39cb8d9c | clean | pass |
| SWEEP_20260618T011741Z_33b3abeb762a.json | 33b3abeb762a | clean | pass |
| SWEEP_20260618T013142Z_b940db7b5ad9.json | b940db7b5ad9 | clean | pass |
| SWEEP_20260618T014451Z_690f3068a22b.json | 690f3068a22b | clean | pass |
| SWEEP_20260618T015545Z_5270626a1481.json | 5270626a1481 | clean | pass |
| SWEEP_20260618T020700Z_968a7fb96cef.json | 968a7fb96cef | clean | pass |
| SWEEP_20260618T021612Z_e57b6f60bf4a.json | e57b6f60bf4a | clean | pass |
| SWEEP_20260618T022712Z_b28ec6893bf1.json | b28ec6893bf1 | clean | pass |
| SWEEP_20260618T023725Z_959fa76064e1.json | 959fa76064e1 | clean | pass |
| SWEEP_20260618T024728Z_f75d19975f6b.json | f75d19975f6b | clean | pass |
| SWEEP_20260618T025541Z_c0a9517e645c.json | c0a9517e645c | clean | pass |
| SWEEP_20260618T031519Z_cd80ccb35e12.json | cd80ccb35e12 | clean | pass |
| SWEEP_20260618T032514Z_1885ab52255c.json | 1885ab52255c | clean | pass |
| SWEEP_20260618T033309Z_c15fa63137fc.json | c15fa63137fc | clean | pass |
| SWEEP_20260618T034325Z_b522641d6886.json | b522641d6886 | clean | pass |
| SWEEP_20260618T035234Z_bcc7ba571deb.json | bcc7ba571deb | clean | pass |
| SWEEP_20260618T040207Z_a6276312016b.json | a6276312016b | clean | pass |
| SWEEP_20260618T042052Z_ee7a6d2d997a.json | ee7a6d2d997a | clean | pass |
| SWEEP_20260618T043107Z_cbe3d4eac8eb.json | cbe3d4eac8eb | clean | pass |
| SWEEP_20260618T044055Z_eb8ec785b409.json | eb8ec785b409 | clean | pass |
| SWEEP_20260618T045109Z_4c784da20850.json | 4c784da20850 | clean | pass |
| SWEEP_20260618T050040Z_45bbef0eb1fd.json | 45bbef0eb1fd | clean | pass |
| SWEEP_20260618T050846Z_03c7375d5056.json | 03c7375d5056 | clean | pass |
| SWEEP_20260618T051720Z_2af205fca49f.json | 2af205fca49f | clean | pass |
| SWEEP_20260618T053059Z_8e56a3796d24.json | 8e56a3796d24 | clean | pass |
| SWEEP_20260618T054415Z_6da5e2a5766b.json | 6da5e2a5766b | clean | pass |
| SWEEP_20260618T060440Z_bb9315fac83e.json | bb9315fac83e | clean | pass |
| SWEEP_20260618T061721Z_326af20b607c.json | 326af20b607c | clean | pass |
| SWEEP_20260618T062957Z_c01bfd12a56b.json | c01bfd12a56b | clean | pass |
| SWEEP_20260618T064228Z_2aa15b62848d.json | 2aa15b62848d | clean | pass |
| SWEEP_20260618T065256Z_65d974a2b0e8.json | 65d974a2b0e8 | clean | pass |
| SWEEP_20260618T070355Z_46df2903ec8e.json | 46df2903ec8e | clean | pass |
| SWEEP_20260618T071625Z_7b539d4547b1.json | 7b539d4547b1 | clean | pass |
| SWEEP_20260618T072730Z_0d410029c7a3.json | 0d410029c7a3 | clean | pass |
| SWEEP_20260618T073913Z_9878252c8f25.json | 9878252c8f25 | clean | pass |
| SWEEP_20260618T075139Z_89f18262f5c8.json | 89f18262f5c8 | clean | pass |
| SWEEP_20260618T080255Z_f00ab6fe6735.json | f00ab6fe6735 | clean | pass |
| SWEEP_20260618T081221Z_0e7186f4fc71.json | 0e7186f4fc71 | clean | pass |
| SWEEP_20260618T082444Z_4625fe56485b.json | 4625fe56485b | clean | pass |
| SWEEP_20260618T083844Z_6d48667db622.json | 6d48667db622 | clean | pass |
| SWEEP_20260618T084932Z_92aec03f700c.json | 92aec03f700c | clean | pass |
| SWEEP_20260618T085850Z_4d4821cf3e23.json | 4d4821cf3e23 | clean | pass |
| SWEEP_20260618T090918Z_d7e4682ae6e2.json | d7e4682ae6e2 | clean | pass |
| SWEEP_20260618T091837Z_6afa0219d3b8.json | 6afa0219d3b8 | clean | pass |
| SWEEP_20260618T092924Z_dbb6d20882e6.json | dbb6d20882e6 | clean | pass |
| SWEEP_20260618T093809Z_44b10bf44761.json | 44b10bf44761 | clean | pass |
| SWEEP_20260618T094821Z_ac669bbaa39a.json | ac669bbaa39a | clean | pass |
| SWEEP_20260618T100046Z_59d2112149ef.json | 59d2112149ef | clean | pass |
| SWEEP_20260618T101102Z_e801faf6ca35.json | e801faf6ca35 | clean | pass |
| SWEEP_20260618T102003Z_de0e4bd8e5cc.json | de0e4bd8e5cc | clean | pass |
| SWEEP_20260618T103248Z_ab32474caffb.json | ab32474caffb | clean | pass |
| SWEEP_20260618T104502Z_f520bf8236dc.json | f520bf8236dc | clean | pass |
| SWEEP_20260618T110430Z_1b57394e5213.json | 1b57394e5213 | clean | pass |
| SWEEP_20260618T114246Z_a2e02043f151.json | a2e02043f151 | clean | pass |
| SWEEP_20260618T115152Z_35d6dc6efe3c.json | 35d6dc6efe3c | clean | pass |
| SWEEP_20260618T120403Z_c5be58aeaed1.json | c5be58aeaed1 | clean | pass |
| SWEEP_20260618T121357Z_64e6b997c343.json | 64e6b997c343 | clean | pass |
| SWEEP_20260618T122213Z_d9045c7fc602.json | d9045c7fc602 | clean | pass |
| SWEEP_20260618T123150Z_bd3329fc8ea6.json | bd3329fc8ea6 | clean | pass |
| SWEEP_20260618T124214Z_5340244585df.json | 5340244585df | clean | pass |
| SWEEP_20260618T125411Z_e5dd4a230a12.json | e5dd4a230a12 | clean | pass |
| SWEEP_20260619T072613Z_5dbd406023ba-dirty.json | 5dbd406023ba | dirty | pass |
| SWEEP_20260619T144814Z_48083bd29407-dirty.json | 48083bd29407 | dirty | pass |
| SWEEP_20260620T230911Z_a4c29023a499.json | a4c29023a499 | clean | pass |
| SWEEP_20260621T011612Z_e0c22d7ac716.json | e0c22d7ac716 | clean | pass |
| SWEEP_20260621T014041Z_9586deb6c15e.json | 9586deb6c15e | clean | pass |
| SWEEP_20260621T022113Z_f314fc1a67d7.json | f314fc1a67d7 | clean | pass |
| SWEEP_20260621T023012Z_f9194f0d3267.json | f9194f0d3267 | clean | pass |
| SWEEP_20260621T023850Z_b72cbaad613e.json | b72cbaad613e | clean | pass |
| SWEEP_20260621T030615Z_55b641633661.json | 55b641633661 | clean | pass |
| SWEEP_20260621T034405Z_b3fcb59502af.json | b3fcb59502af | clean | pass |
| SWEEP_20260621T041239Z_4eafea33de99.json | 4eafea33de99 | clean | pass |
| SWEEP_20260621T050444Z_b564f5bf54ec.json | b564f5bf54ec | clean | pass |
| SWEEP_20260621T053817Z_c6442ffb0788.json | c6442ffb0788 | clean | pass |
| SWEEP_20260621T062128Z_43e333abcb87.json | 43e333abcb87 | clean | pass |
| SWEEP_20260621T070045Z_251dbcd8ce97-dirty.json | 251dbcd8ce97 | dirty | FAIL(desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260621T071342Z_251dbcd8ce97-dirty.json | 251dbcd8ce97 | dirty | pass |
| SWEEP_20260621T073226Z_4cb593a09376-dirty.json | 4cb593a09376 | dirty | pass |
| SWEEP_20260621T074615Z_a83ced203fac-dirty.json | a83ced203fac | dirty | pass |
| SWEEP_20260621T075607Z_837d0febe6d9-dirty.json | 837d0febe6d9 | dirty | pass |
| SWEEP_20260621T081228Z_97770008ed2b.json | 97770008ed2b | clean | pass |
| SWEEP_20260621T085427Z_93a25e03201f-dirty.json | 93a25e03201f | dirty | pass |
| SWEEP_20260621T092312Z_53b592aee006-dirty.json | 53b592aee006 | dirty | pass |
| SWEEP_20260621T094346Z_87eb336e1b0a-dirty.json | 87eb336e1b0a | dirty | pass |
| SWEEP_20260621T095800Z_4d3bae24de12-dirty.json | 4d3bae24de12 | dirty | pass |
| SWEEP_20260621T101429Z_f74897170d66-dirty.json | f74897170d66 | dirty | pass |
| SWEEP_20260621T103336Z_a02ab8f77612-dirty.json | a02ab8f77612 | dirty | pass |
| SWEEP_20260621T104441Z_977483bf478b-dirty.json | 977483bf478b | dirty | pass |
| SWEEP_20260621T105725Z_54f581a0b912-dirty.json | 54f581a0b912 | dirty | pass |
| SWEEP_20260621T111451Z_eaf10ef3c49a-dirty.json | eaf10ef3c49a | dirty | pass |
| SWEEP_20260621T113918Z_756f319251c0-dirty.json | 756f319251c0 | dirty | pass |
| SWEEP_20260621T114725Z_df60a628ee9e-dirty.json | df60a628ee9e | dirty | pass |
| SWEEP_20260621T115235Z_0efaab1e6b48-dirty.json | 0efaab1e6b48 | dirty | pass |
| SWEEP_20260621T190328Z_0313bfb0302d-dirty.json | 0313bfb0302d | dirty | pass |
| SWEEP_20260621T202442Z_4829dea6c2e0-dirty.json | 4829dea6c2e0 | dirty | pass |
| SWEEP_20260621T205711Z_c771567ed6a8-dirty.json | c771567ed6a8 | dirty | pass |
| SWEEP_20260622T054832Z_1859416a9236.json | 1859416a9236 | clean | pass |
| SWEEP_20260622T063558Z_d3f658288543-dirty.json | d3f658288543 | dirty | pass |
| SWEEP_20260622T065649Z_150b107259dd-dirty.json | 150b107259dd | dirty | pass |
| SWEEP_20260622T071252Z_00effc54b1ad-dirty.json | 00effc54b1ad | dirty | pass |
| SWEEP_20260622T072805Z_be55844f721d-dirty.json | be55844f721d | dirty | pass |
| SWEEP_20260622T074318Z_c2e41707f1d5-dirty.json | c2e41707f1d5 | dirty | pass |
| SWEEP_20260622T081626Z_719254ef37f8-dirty.json | 719254ef37f8 | dirty | pass |
| SWEEP_20260622T083239Z_5a955e5e84b1-dirty.json | 5a955e5e84b1 | dirty | pass |
| SWEEP_20260622T085210Z_799ebcc0dee5-dirty.json | 799ebcc0dee5 | dirty | pass |
| SWEEP_20260622T090832Z_9b32ef0b35d0-dirty.json | 9b32ef0b35d0 | dirty | pass |
| SWEEP_20260622T093149Z_c1f9fe81811d-dirty.json | c1f9fe81811d | dirty | pass |
| SWEEP_20260622T094835Z_68f720407e61-dirty.json | 68f720407e61 | dirty | pass |
| SWEEP_20260622T100438Z_f0f0ddd44e5b-dirty.json | f0f0ddd44e5b | dirty | pass |
| SWEEP_20260622T102208Z_479c6fd7d97a-dirty.json | 479c6fd7d97a | dirty | pass |
| SWEEP_20260622T103936Z_5058ec872d37-dirty.json | 5058ec872d37 | dirty | pass |
| SWEEP_20260622T105454Z_d7d8ecc8cab7-dirty.json | d7d8ecc8cab7 | dirty | pass |
| SWEEP_20260622T111513Z_7ab178763ccc-dirty.json | 7ab178763ccc | dirty | pass |
| SWEEP_20260622T113515Z_521fdfe1e613-dirty.json | 521fdfe1e613 | dirty | pass |
| SWEEP_20260622T115411Z_b7426f628f24-dirty.json | b7426f628f24 | dirty | pass |
| SWEEP_20260622T121215Z_3bc51d2b2eed-dirty.json | 3bc51d2b2eed | dirty | pass |
| SWEEP_20260622T122628Z_2fb363448820-dirty.json | 2fb363448820 | dirty | pass |
| SWEEP_20260622T123917Z_315cb4c609bd-dirty.json | 315cb4c609bd | dirty | pass |
| SWEEP_20260622T130443Z_2ead1626d231-dirty.json | 2ead1626d231 | dirty | pass |
| SWEEP_20260622T131130Z_d4165f4fa5fe-dirty.json | d4165f4fa5fe | dirty | pass |
| SWEEP_20260622T132612Z_052c8cb5e277-dirty.json | 052c8cb5e277 | dirty | pass |
| SWEEP_20260622T135307Z_95d0a6b5e0c0-dirty.json | 95d0a6b5e0c0 | dirty | pass |
| SWEEP_20260622T142119Z_1b32c80965d5-dirty.json | 1b32c80965d5 | dirty | pass |
| SWEEP_20260622T143831Z_7f73b463bd54-dirty.json | 7f73b463bd54 | dirty | pass |
| SWEEP_20260622T150957Z_cc492e9d7eb9-dirty.json | cc492e9d7eb9 | dirty | pass |
| SWEEP_20260622T163532Z_3068ec39ed5e-dirty.json | 3068ec39ed5e | dirty | pass |
| SWEEP_20260622T165533Z_25634e332118-dirty.json | 25634e332118 | dirty | pass |
| SWEEP_20260622T170222Z_f69e970d3347.json | f69e970d3347 | clean | pass |
| SWEEP_20260623T001842Z_bf2e089aa97b.json | bf2e089aa97b | clean | pass |
| SWEEP_20260623T003028Z_21f3cdcf0eb3.json | 21f3cdcf0eb3 | clean | pass |
| SWEEP_20260623T020002Z_3194bd29f417-dirty.json | 3194bd29f417 | dirty | pass |
| SWEEP_20260623T051552Z_16cca07f3b64.json | 16cca07f3b64 | clean | pass |
| SWEEP_20260704T193342Z_98dfde1d18ac-dirty.json | 98dfde1d18ac | dirty | pass |
| SWEEP_20260705T033815Z_9fe446eeab35.json | 9fe446eeab35 | clean | pass |
| SWEEP_20260705T042325Z_f033dd24d3a4.json | f033dd24d3a4 | clean | pass |
| SWEEP_20260705T050428Z_8c57cf5f4671.json | 8c57cf5f4671 | clean | pass |
| SWEEP_20260710T134128Z_16ba9db23114.json | 16ba9db23114 | clean | pass |
| SWEEP_20260710T151446Z_b77e721b2028-dirty.json | b77e721b2028 | dirty | pass |
| SWEEP_20260710T201841Z_c6430cbd5c9b.json | c6430cbd5c9b | clean | pass |
| SWEEP_20260710T214950Z_37b327cf0687.json | 37b327cf0687 | clean | pass |
| SWEEP_20260710T232952Z_b1ff985c72eb.json | b1ff985c72eb | clean | FAIL(desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260710T233408Z_b1ff985c72eb-dirty.json | b1ff985c72eb | dirty | pass |
| SWEEP_20260710T233804Z_1094b0deb266.json | 1094b0deb266 | clean | pass |
| SWEEP_20260710T234618Z_dc63c4a83eed.json | dc63c4a83eed | clean | pass |
| SWEEP_20260711T012340Z_968b4f7de74e.json | 968b4f7de74e | clean | FAIL(desktop_vitest,desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260711T012615Z_968b4f7de74e-dirty.json | 968b4f7de74e | dirty | pass |
| SWEEP_20260711T012810Z_1d160589cbea.json | 1d160589cbea | clean | pass |
| SWEEP_20260711T012843Z_dfa5dd344429.json | dfa5dd344429 | clean | FAIL(desktop_vitest,desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260711T012926Z_658e15a5bb46.json | 658e15a5bb46 | clean | pass |
| SWEEP_20260711T013150Z_dfa5dd344429-dirty.json | dfa5dd344429 | dirty | pass |
| SWEEP_20260711T013618Z_4012b4c57496.json | 4012b4c57496 | clean | pass |
| SWEEP_20260711T014521Z_8b456e6e1c02.json | 8b456e6e1c02 | clean | pass |
| SWEEP_20260711T020214Z_50f230b09885.json | 50f230b09885 | clean | FAIL(desktop_vitest,desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260711T020414Z_50f230b09885-dirty.json | 50f230b09885 | dirty | pass |
| SWEEP_20260711T020757Z_67f06454737f.json | 67f06454737f | clean | pass |
| SWEEP_20260711T023430Z_a4fbed24b6b4.json | a4fbed24b6b4 | clean | pass |
| SWEEP_20260711T024049Z_f940b5b7c1af.json | f940b5b7c1af | clean | FAIL(desktop_vitest,desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260711T024350Z_b9ba51af9f3c.json | b9ba51af9f3c | clean | pass |
| SWEEP_20260711T024551Z_73e4b6f7c848.json | 73e4b6f7c848 | clean | FAIL(desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260711T024706Z_9fc322fcad8d.json | 9fc322fcad8d | clean | pass |
| SWEEP_20260711T024949Z_3c12cc27b158.json | 3c12cc27b158 | clean | FAIL(desktop_playwright_e2e,desktop_production_build) |
| SWEEP_20260711T025412Z_0b9944768560.json | 0b9944768560 | clean | pass |
| SWEEP_20260711T025817Z_3115a08cdfee.json | 3115a08cdfee | clean | pass |
| SWEEP_20260711T031432Z_1581b8c0de72.json | 1581b8c0de72 | clean | pass |
| SWEEP_20260711T031732Z_aceac0b6c288.json | aceac0b6c288 | clean | pass |
| SWEEP_20260711T032140Z_e2ea37194c8a.json | e2ea37194c8a | clean | pass |
| SWEEP_20260711T032717Z_af74a1096ff0.json | af74a1096ff0 | clean | pass |
| SWEEP_20260711T034808Z_049c49328bfc.json | 049c49328bfc | clean | pass |
| SWEEP_20260711T035249Z_9aa5dcdc1a29.json | 9aa5dcdc1a29 | clean | pass |
| SWEEP_20260711T040758Z_e648462f1d05.json | e648462f1d05 | clean | pass |

Latest clean all-pass sweep: `SWEEP_20260711T040758Z_e648462f1d05.json` — the execution-evidence anchor used throughout the CSV.
