# Native signature-coverage preparation V2

**Status:** PREPARED — read/calculation/evidence only; no runtime qualification has occurred.

This addendum was prepared under `NATIVE_SIGNATURE_COVERAGE_PREPARATION.md` at SHA-256 `23671cf541a3b85487747358f2fd3991485175ef2802a7597d1be5d8fb9396f8`. It preserves every V1 artifact. The work consisted only of maintained-source/evidence reads, local JSON parsing and hashing, and writes to the named V2 paths. No build, test, browser, native app, Git, network, product-source edit or delegation occurred.

The machine-readable evidence is in `_run_records/SIGNATURE_PREPARATION_V2/CONTEXT_V2.json` and `_run_records/SIGNATURE_PREPARATION_V2/INPUT_MAP_V2.json`.

## Frozen population finding

The quantities describe two related but different artifacts:

- `fixtures/results/semantic_contract_v0_2.json` (`4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da`) declares **60 exact `(kind, unit, metadata.component)` signatures across 47 kinds**.
- `fixtures/product_preview/invented_mechanics_result.json` (`fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`) contains **830 result rows, 40 kinds and 48 of those exact signatures**.
- `fixtures/results/invented/result_export_v0_2.json` (`ef09b38c1619e06c3fbf2082a331a8cb94936f4b8d1b8ecd025e0f424424cff2`) contains one semantics fixture for each of the 60 signatures and seven producer-case models. Its status fields expressly classify signatures 022, 029 and 037 as original producer-row snapshots rather than a new runtime, and 050–059 as authored source-semantics fixtures rather than executed producer output. It cannot prove a current native producer pass.

The 830-row fixture omits these twelve exact signatures:

| ID | Exact signature | Legacy behavior | Current producer evidence |
|---|---|---|---|
| 022 | `curved_bend_macro_element_review` / `unitless` / `curved_bend_flexibility` | throws undeclared | prior actual product output and maintained input |
| 029 | `nonlinear_support_friction_normal_reaction_input` / `N` / `friction_normal_reaction_input` | throws undeclared | prior actual product output and maintained input |
| 037 | `sparse_live_path_dense_parity_relative_delta` / `unitless` / `sparse_live_path` | throws undeclared | prior actual dense product replay of shipped model |
| 050 | `modulus_basis_record` / `record` / `material_modulus_basis` | throws undeclared | prior actual headless product output and maintained input |
| 051 | `combination_modulus_basis_record` / `record` / `material_modulus_basis` | throws undeclared | prior actual headless product output and maintained input |
| 052 | `constant_effort_support_applied_load` / `N` / same component | throws undeclared | source plus product unit-test trigger; no captured exact output |
| 053 | `nonlinear_support_final_displacement` / `rad` / same component | preserves legacy `length` | source branch only; no positive exact producer snapshot |
| 054 | `nonlinear_support_final_reaction` / `N*m` / same component | preserves legacy `force` | source branch only; no positive exact producer snapshot |
| 056 | `spring_hanger_user_input_review` / `m` / `variable_spring_hanger_movement_limit` | throws undeclared | source branch and analogous product test trigger; no captured exact output |
| 057 | `constant_effort_user_input_review` / `m` / `constant_effort_support_movement_limit` | throws undeclared | source plus product unit-test trigger; no captured exact output |
| 058 | `spring_hanger_user_input_review` / `N*m/rad` / `variable_spring_hanger_stiffness` | preserves legacy `linear_stiffness` | source branch only; no captured exact output |
| 059 | `pipe_section_pressure_longitudinal_stress` / `MPa` / `pressure_longitudinal_stress` | throws undeclared | source branch; actual reachability unresolved |

Thus nine missing signatures are legacy admission failures and three preserve a legacy dimension different from the new physical semantics. Signature 055 is present in the shipped 830-row population even though its separate one-row semantics fixture is classified as authored.

The stress-neutral counts remain tied to the 830-row shipped reference population: semantic eligibility yields **828 raw eligible rows and 830 reference eligible rows**, while all 830 source rows remain retained. The solver-mode disclosure is discrete evidence and does not replace either of the two raw work-row withholdings.

## Actual producer trace

The shipped model is `fixtures/product_preview/invented_preview_model.json` at SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`. The earlier audit input `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/I1/children/R/fixtures/invented_nonlinear.json` has SHA-256 `fb374cc68aaa3e2db9fce70251c4d90e4a3084c989ff4caa28180597433b2b3c`; JSON parsing shows it is exactly the same model despite different byte formatting.

The accepted audit replay used a real Rust driver that deserialized `PreviewModel` and called `run_linear_static_preview_with_mode(LinearStaticPreviewRequest { model, materials: vec![] }, mode)`, the same product function and empty material list used by the Tauri backend. Its captured sparse output is `cd514bf44ee7ba0ca1b019e3ee162471dfc33c390ba848f50e062468c69bfef8`: 830 rows, 40 kinds and 48 exact signatures. It has the same 830 result IDs as the maintained golden, but 534 rows differ in value or metadata, so it proves the population and product path rather than golden byte/value identity. The dense replay is `18dc0ac047938250961ccb06f64e7a5cb057557ecbfcd3d2a50372b0e064d465`: 832 rows, 41 kinds and 49 exact signatures, adding signature 037 for the two load cases.

Maintained, previously executed inputs and outputs cover four more missing signatures:

| Signature | Model SHA-256 | Captured output SHA-256 |
|---|---|---|
| 022 curved-bend review | `1f6125237912846153fb79414df0fd717d26799a5c5c55faedb84898e4b70dac` | `26d39653b1913156e33846906a683f8c4f3f94be7e5c924ccff169eb766cdcd2` |
| 029 friction normal reaction input | `c7c6898e11b144aad4733cbdffb445bdc6b7b1074d373d767d85b8f5a71e69e4` | `d50cb5a2d2bf16ab4dc2219ac6ca3a16fc286abe9ae86b45e6eb8a6f9e7c4699` |
| 050 modulus basis record | `516d8581ad50e28f656f1a353985925bcffcea1d966bc7e9e8bc679792f03e7c` | `42aea70c00ac32374641f786102ca3a59e01ff54954352f1a5284c5fda48f506` |
| 051 combination modulus basis | `426397a6655403dac5521bbb75227f5b4b39f1e6bd0bbcc5f55977f80e116076` | `069a3dde110d2464c87c104b04e7d2c502cd707d2d9a6717ec894f27767f0d7a` |

These are historical product-output evidence. They must be rerun against the frozen final candidate before they qualify that candidate.

For 052, `constant_effort_consumption_matches_superposition_identity` already constructs the operative condition: a two-node cantilever with a tip constant-effort support, one `UY` restraint and a 375 N constant load. For 056 and 057, the source emits review rows when the corresponding hanger has a positive `movement_limit` in metres; `constant_effort_user_limit_comparison_warns_from_user_data_only` supplies an existing constant-effort test pattern. For 058, the variable-spring stiffness DOF must be rotational and its value must use `N*m/rad`. For 053 and 054, an accepted nonlinear support must use a rotational DOF so final displacement and reaction are emitted in `rad` and `N*m`. No maintained positive exact producer snapshots were found for these rows, so they require new evidence-local model execution.

Signature 059 requires a separate reachability qualification. The current producer sets `include_pressure_longitudinal = !pressure_thrust_active`. A nonzero genuine pressure normally activates pressure thrust and suppresses the longitudinal pressure row. A zero-valued genuine pressure record appears capable of reaching a zero-valued longitudinal row, but no actual product output proves that path. It must remain unresolved until execution; an authored single row cannot substitute.

## Existing routes and evidence levels

The Tauri backend (`apps/desktop/src-tauri/src/lib.rs`, `5c480a4e9317fc72a1c81a043eb8f48c1cc7d0c174e8d5aee0aabfb53e556f23`) resolves a supplied or bundled model, parses `PreviewModel`, and calls the product function from `core/product_physics/src/lib.rs` (`aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15`) with `materials: vec![]`. The frontend (`apps/desktop/src/services/previewService.ts`, `2c4d459e62689a24d831d88ab92770b7f923f63f3419947b0963e9d413b62800`) uses the asynchronous backend-job path in Tauri and does not replace invocation errors with the fixture. The Solve panel exposes Sparse interactive and Dense scrutiny. `save_local_result_json` provides actual native result-file delivery.

The stable file-backed route is `openpipestress-runner solve --input <request.json> --output <result.json>` from `core/runner/headless/src/bin/openpipestress-runner.rs` (`f873b5b9ac6518e1361049d101a323fbc8a1e54b67017810f48d97a1ba9f33ee`). A valid wrapper supplies `request` and `solve.preview_model`. It is a local foreground product-mechanics execution route with an explicit output path. It proves actual producer behavior and file delivery. It does not prove GUI authoring or GUI delivery.

The browser route is deliberately fixture-backed. With no model, or with a model canonically equal to the bundled model, it returns the maintained 830-row mechanics fixture; an edited browser model returns a blocked result. `gui-workflow-validation.spec.ts` (`5e3013dfc1d7e6b7b77f6bfa6274e7f845b9ef299609dfe3b79d4d95e1a55f10`) and `r2-smoke.spec.ts` (`e52e799b7b6aefe3c6122f32a6bb9d51f6892160f4ae932b09bf1360659900dc`) therefore prove actual browser-app consumption and display of the shipped fixture while explicitly disclosing `seam=browser_fixture_no_backend_job`. They do not prove Rust mechanics execution or delivery of the twelve missing signatures.

The 60-signature checks in `resultSemantics.test.ts` (`69081dbf91c303b8b5bf9dcd19437d86d49c0459fe55287b27fb0e481ca48b60`) and `resultExportAdapter.test.ts` (`3fc719f89973bc3b41bf041de8af8f05129af6f5cb4b595dbc02278ebd442e78`) are component evidence. Repeating those checks with captured real producer rows would be strong compositional evidence: actual CLI producer plus actual frontend adapter. It would still not be an actual browser-app delivery witness.

No arbitrary PreviewModel JSON import or alternate MechanicsResult JSON import control was found in the maintained UI. Project open/save exists, but it is not evidence of such a transport. The final procedure must not demand or claim a nonexistent import seam.

## Smallest truthful execution matrix after lease

1. **Packaged native shipped sparse.** Use the frozen packaged application, solve the bundled model through the asynchronous backend job, assert `seam=backend_job`, save the result through the native result-save control, and verify 830 rows / 40 kinds / 48 exact signatures, the 830 result-ID population, and stress-neutral 828 raw / 830 reference eligibility with all 830 retained.
2. **Packaged native shipped dense.** Select Dense scrutiny in the existing GUI, solve and save. Verify 832 rows / 41 kinds / 49 exact signatures and exact signature 037. These two steps are actual native producer, actual frontend delivery and actual GUI/file delivery; they do not establish GUI authoring of supplemental models.
3. **Stable CLI, maintained cases.** Run the exact curved, friction, modulus and combination-modulus models through the frozen final `openpipestress-runner` binary. Verify 022, 029, 050 and 051 in captured mechanics envelopes. This is actual producer plus file delivery.
4. **Stable CLI, minimal evidence-local supplements.** Use a small constant-effort/hanger model for 052/056/057/058, a separate stable rotational nonlinear-support model for 053/054, and a separate pressure-longitudinal reachability case for 059. Split cases further if solver stability requires it. Verify the exact triples in actual output and both final 0.2 and legacy behaviors. Never mark 059 passed without the exact row in actual output.
5. **Frontend composition over captured outputs.** Feed the actual rows from steps 3–4 through the final result-semantics, analysis-record and export-adapter paths. Verify category, physical dimension, exact component, admission/preserved legacy dimensions and stress-neutral disposition. Label this producer-plus-component evidence, not browser application delivery.
6. **Existing browser GUI workflow.** Retain the real Playwright UI witness for the shipped 830-row fixture and its explicit browser fixture seam. It covers browser delivery of the base 48 signatures only. There is no truthful existing browser route for the supplemental twelve.

This is the smallest matrix that covers actual native production and GUI delivery for the shipped population, actual file-backed product production for supplemental signatures, frontend consumption of actual producer rows, and honest browser coverage without inventing a transport.

## Execution holds and pass boundary

Preparation is complete. It is not a source, build, runtime, browser, native or 60-signature qualification pass. The seven source-only or unresolved signatures are 052, 053, 054, 056, 057, 058 and 059. Final execution requires the immutable candidate, candidate-bound hashes, root's serialized native build/GUI lease, evidence-local inputs and captured outputs, and independent acceptance. If 059 is unreachable in the final producer, return it as a blocker or contract/source inconsistency; do not replace it with fixture evidence.
