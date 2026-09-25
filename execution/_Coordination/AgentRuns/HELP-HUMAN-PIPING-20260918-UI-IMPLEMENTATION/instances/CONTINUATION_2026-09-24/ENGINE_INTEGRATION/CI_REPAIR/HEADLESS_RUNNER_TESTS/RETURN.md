# CI repair: `tests/test_headless_runner_contract.py`

**Result: repaired. All 22 tests pass, with no skip, no xfail and no weakened assertion.** Of the 22, 18 are the original tests (the 8 failures are now fixed) and 4 are new controls. The runner source is unchanged. No existing fixture was changed, and no new fixture file was needed.

**The diagnosis is only partly confirmed.** ROOT's legacy-pressure diagnosis explains **2 of the 8** failures. The other **6** are all export-results tests, and they have a different root cause:

- The DEL-10-05 witness inputs under `validation/witness/inputs/` record the solver identity `open_pipe_stress_product_physics@0.1.0`.
- PR905 raised the linked product-physics component to `0.2.0`, in commit `0a438a686` ("Preserve numerical integrity and contain unqualified result use").
- The runner deliberately requires the two identities to match, so it refuses these inputs with `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH`.
- These inputs never contain the legacy preview model.

This is not a runner defect.

Unless noted otherwise, paths are relative to `projects/chirality-piping`. Raw logs, probe scripts and their outputs are in [_run_records/](_run_records/). The candidate is HEAD `347e214fe265b13e6d24f847f163821f0c833018` on branch `claude/inspiring-ptolemy-zxofd6`. It was compared with main `aa312755e`. This work was executed by a Type 2 TASK dispatched by HELP_HUMAN (ROOT), without delegation.

## 1. Evidence for the two root causes

**Before the repair:** 8 failed and 10 passed ([pytest_before_repair.log](_run_records/pytest_before_repair.log)). The test file is byte-identical at HEAD and at main (blob `229783d9b9a1`), so the failures come from product and runner changes.

### A. Legacy pressure refusal (2 tests)

These tests solved `fixtures/product_preview/invented_preview_model.json`. That file has four nonzero legacy pressure loads:

- `load:L-100/load:L-100-P` and `load:L-100/load:L-100-P-EJ` (1.2 MPa)
- `load:L-200/load:L-200-P` and `load:L-200/load:L-200-P-EJ` (0.6 MPa)

`core/product_physics/src/pressure_runtime.rs:200-209` emits one blocking `PRESSURE_MODEL_REAUTHOR_REQUIRED` for each of these loads. Both runners then report the following ([probe_legacy_refusal.out](_run_records/probe_legacy_refusal.out), [probe_candidate_models.out](_run_records/probe_candidate_models.out)):

- mechanics status `MODEL_INCOMPLETE`, with `results == []`
- runner `analysis_status` containing `MODEL_INCOMPLETE`, with `result_refs == []`

The final runner's exit 1 comes from `execute_solve`. In `openpipestress-runner.rs:814-817`, `clean` is false because `validate_result` raises the blocking `HEADLESS_RUNNER_RESULT_REFS_MISSING`. The compatibility runner exits 1 through the same `clean` rule (`headless_preview_runner.rs:113-120`).

With `--explicit-local-private-intent`, the output is `blocked:false` and the payload is present. That means the refusal is visible rather than withheld. PR905's own Rust test says the same thing (`original_legacy_nonzero_pressure_fixture_is_not_a_current_cli_solve`).

### B. Stale solver identity in the export witness (6 tests)

- The runner passes `solver_component_name()` and `solver_component_version()` to `assemble_wire_request` (`openpipestress-runner.rs:443-452`). These return `env!("CARGO_PKG_NAME")` and `env!("CARGO_PKG_VERSION")` (`core/product_physics/src/lib.rs:11084-11092`).
- `core/product_physics/Cargo.toml` has version `0.1.0` at main and `0.2.0` at HEAD. The bump commit is `0a438a686`.
- `core/reporting/report_package/src/wire.rs:308-318` (`audit_manifest`) and `:519-528` (`result_envelope`) require exact equality with that identity. Otherwise they return `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH`.
- The witness inputs and their generator `generate_del1005_export_results_inputs.py` last changed in `f82bb28e2` (2026-07-23) and hard-code `0.1.0`.

**What the probe showed** ([probe_export_identity.out](_run_records/probe_export_identity.out)): I sent each witness through the real runner twice, once as recorded and once with only the six identity fields changed to `0.2.0`.

| Witness | As recorded (0.1.0) | Identity set to 0.2.0 |
|---|---|---|
| success | exit 1, `PAYLOAD_INVALID: …SOLVER-IDENTITY-MISMATCH` | exit 0, 28,997-byte package |
| producer_blocked | exit 1, `…SOLVER-IDENTITY-MISMATCH` | exit 1, `…EXPORT_RESULTS_PACKAGE_BLOCKED` (the expected code) |
| missing_payload | exit 1, `PAYLOAD_MISSING` | same |
| binding_mismatch | exit 1, `BINDING_MISMATCH` | same |

The identity check runs before the SHA-256 check and before the producer. This explains two of the failures: the invalid-wire test saw an identity message instead of `SHA256-INVALID`, and the producer-blocked case never reached `PACKAGE_BLOCKED`.

The export tests pass on main because main links `0.1.0`.

## 2. Model choice

I probed every candidate model through both runners, with `materials: []` wherever the file is model-only ([probe_candidate_models.out](_run_records/probe_candidate_models.out)). All of the re-authored candidates solve.

The chosen model is **`fixtures/model_operations/exact_pressure_authoring_model.json`**, used unchanged (sha256 `7217f923…2fa0e`). It contains:

- a synthetic user-input model with `constitutive_basis: homogeneous_isotropic_E_nu_v1`, E = 200 GPa and nu = 0.3
- `pressure_contract` mode `exact_straight_pressure_v2`
- one pressure region at 2000 kPa, with both terminals set to `closure_transfer: transfers_to_wall`
- a six-component load case

Through both runners it gives exit 0, `MECHANICS_SOLVED` and 175 results. Its producer identity is `open_pipe_stress_product_physics` `0.2.0`.

I chose it over `precision_connected_ui_model.json`, which PR905's own Rust stdout test uses, for two reasons:

- It keeps a pressure-bearing solve on the CLI success path, which the legacy demo used to provide.
- It is the explicitly re-authored positive counterpart of the refusal control.

No new fixture was needed. **Gap:** the legacy demo's richer topology (5 nodes, 4 segments, 7 supports including hangers, 5 components, 1 combination) is no longer solved by these CLI tests. None of their assertions depended on that topology. Re-authoring that loop with exact regions and closures is engineering authoring and is outside this repair. I did not strip any feature from the legacy demo.

## 3. Changes to each test

| Test | Root cause | Change | Protection kept |
|---|---|---|---|
| `test_final_runner_subprocess_covers_every_active_verb_and_stdout` | A | The solve case uses the authored model. It also asserts `MECHANICS_SOLVED`, non-empty results, and a producer identity equal to the linked identity. | Every active verb exits 0 under intent, with unblocked controlled stdout and matching `command`/`operation`. |
| `…export_results_success_is_deterministic_and_zip_exact` | B | Input comes from `export_input()`, which sets the witness identity to the linked identity. | Two runs give byte-identical stdout. The container SHA-256 matches. There is exactly one decision and one finding at `$.report_package`. The ZIP namelist equals the member list exactly. Each member's length and SHA-256 match. |
| `…export_results_output_is_only_named_json_file` | B | Same binding | Only `runner-result.json` is written, and its content equals stdout. |
| `…failures_have_no_payload_or_file[input_path2-…PACKAGE_BLOCKED]` | B | Same binding, applied to all three parameter cases. The missing and mismatch cases fail before the identity check either way. | Exit 1, no `report_package`, the named diagnostic, and no file. |
| `…requires_intent_once_and_writes_no_file` | B: no package, so no `$.report_package` decision (0 ≠ 1) | Same binding | `blocked:true`, `payload:null`, exactly one decision and one finding with `LOCAL_PRIVATE_INTENT_REQUIRED`, and no file. |
| `…invalid_wire_payload_preserves_report_code` | B: the identity check fired before the hash check | Same binding | `model_hash "ABC"` gives `PAYLOAD_INVALID` carrying `REPORT-PACKAGE-SHA256-INVALID`. |
| `…native_size_is_exact_with_constant_cardinality` | B | Same binding | The package is at least 3,189,621 bytes, the SHA-256 matches, and the cardinality stays at 1. |
| `test_compatibility_runner_subprocess_controlled_stdout_and_exits` | A | Both the intent and no-intent halves use the authored model. The intent half also asserts `MECHANICS_SOLVED`. | Intent gives exit 0 with an unblocked `runner_result`. No intent gives exit 1 with `blocked:true`, `payload:null` and no `runner_result`. Usage gives exit 2 with empty stdout. |

**Two passing tests were also adjusted, without changing any assertion:**

- `test_final_runner_subprocess_blocking_exit_one_writes_no_file` now uses the authored model. At HEAD, its exit 1 had two causes at once, the model refusal and the missing intent. With the authored model, exit 1 comes from the missing intent alone, which restores what the test protected on main.
- `…wire_deserialization_failures_exit_one` gets its input from `export_input()` for uniformity. Its serde failures happen before the identity check, so the outcome is unchanged.

**New controls:**

- `test_final_runner_refuses_legacy_pressure_demo_as_explicit_control` checks the final runner on the unchanged legacy demo with intent:
  - exit 1, `blocked:false`, payload `command`/`operation` = `solve`
  - `MODEL_INCOMPLETE` with no results
  - the `PRESSURE_MODEL_REAUTHOR_REQUIRED` diagnostics are blocking, come from `core/product_physics/src/pressure_runtime.rs`, and have `affected_refs` exactly equal to the fixture's nonzero pressure `(case, load)` pairs, which are computed from the input rather than the output
  - `analysis_status` contains `MODEL_INCOMPLETE` and not `MECHANICS_SOLVED`, and `result_refs` is empty
  - blocking `HEADLESS_RUNNER_RESULT_REFS_MISSING`
- `test_compatibility_runner_refuses_legacy_pressure_demo_as_explicit_control` applies the same refusal checks to the compatibility runner: exit 1 and `blocked:false`.
- `test_final_runner_export_results_refuses_unlinked_solver_identity`, in 2 cases, puts the prior identity `0.1.0` on only the audit manifest in one case and only the result envelopes in the other. Each case must give exit 1, `blocked:false`, no `report_package`, `PAYLOAD_INVALID` with `REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH` and the specific message, and no file. It asserts that the linked version is not `0.1.0`.

**Where the linked identity comes from.** The module fixture `linked_solver_identity` reads it from `cargo metadata --no-deps` in `core/product_physics`. That is the same `CARGO_PKG_NAME`/`CARGO_PKG_VERSION` the runner embeds. The every-verb test cross-checks it against the producer identity the binary actually reports.

## 4. Constants

**No constant was changed.** None of the asserted constants comes from the legacy model's output:

- **`3_189_621`** is an independent oracle, so it stays. It is the measured native report-package byte count: `bytes=3189621` in `apps/desktop/SMOKE.md:9845`, and in the DEL-08-01 native save receipt. `execution/_Coordination/CANDIDATE_BRIEF_2026-07-23_DEL-10-05_EXPORT_RESULTS.md:184,256` adopts it as the lower bound. `core/runner/headless/src/redaction_binding.rs:614` also uses it. The export input does not contain the preview model at all. Observed now: 3,229,023 bytes, which is at least 3,189,621 ([observed_values.json](_run_records/observed_values.json)).
- **The `== 1` decision/finding cardinalities at `$.report_package`** come from the redaction contract (one package path), not from model output. They stay.
- **Diagnostic codes and messages** are independent oracles and stay: `…PAYLOAD_MISSING`, `…BINDING_MISMATCH`, `…PACKAGE_BLOCKED`, `LOCAL_PRIVATE_INTENT_REQUIRED`, `REPORT-PACKAGE-SHA256-INVALID` and `REPORT-PACKAGE-WIRE-INCOMPLETE`.
- **No hash or size is pinned from output.** For the record only, and not asserted: the success package is 28,997 bytes with sha256 `79f5b27c…6fa932`, and its six members are listed in [observed_values.json](_run_records/observed_values.json).

## 5. Verification

| Check | Result | Log |
|---|---|---|
| Headless contract, before the repair | 8 failed, 10 passed | [pytest_before_repair.log](_run_records/pytest_before_repair.log) |
| Headless contract, after the repair, run 1 | 22 passed | [pytest_after_repair_run1.log](_run_records/pytest_after_repair_run1.log) |
| Headless contract, after the repair, run 2 | 22 passed | [pytest_after_repair_run2.log](_run_records/pytest_after_repair_run2.log) |
| Headless contract, `-n 2` (xdist 3.8.0) | 22 passed | [pytest_after_repair_xdist_n2.log](_run_records/pytest_after_repair_xdist_n2.log) |
| `tests/test_qualification_gate.py`, run once without `-n` | 31 passed, 45.6 s | [pytest_qualification_gate_after_repair.log](_run_records/pytest_qualification_gate_after_repair.log) |
| Negative mutations | all 13 detected | [mutation_checks.out](_run_records/mutation_checks.out) |

**The 13 mutations:**

- **A:** a solvable model in the legacy-control slot fails both controls.
- **B:** the legacy model in the success slot reproduces both original legacy-model failures.
- **C:** the unbound 0.1.0 witness reproduces all 6 original export failures.
- **D:** a non-linked identity is refused, and it fails the producer cross-check.
- **E:** the identity control fails when no identity mismatch is present.

Commands, run from `projects/chirality-piping` with `CARGO_BUILD_JOBS=2`, rustc/cargo 1.97.1, and Python 3.11.15 with pytest 9.1.1 from the dec025 venv:

```
python -m pytest -q tests/test_headless_runner_contract.py -p no:cacheprovider [-rA | -n 2]
python -m pytest -q tests/test_qualification_gate.py -p no:cacheprovider -rfE
python _run_records/{probe_candidate_models,probe_export_identity,probe_legacy_refusal,mutation_checks,observed_values}.py
```

The `_run_records` paths above are relative to this folder. The scripts import `tests/test_headless_runner_contract.py` and must be run from `projects/chirality-piping`. The first test run builds the runner itself into the Git-ignored `core/runner/headless/target`.

## 6. Hashes

| File | sha256 |
|---|---|
| `tests/test_headless_runner_contract.py`, before (HEAD = main) | `e81952a191f7a3ce7e80adceb7d7e45c952d82f32c406136f79e58fe99c82706` |
| `tests/test_headless_runner_contract.py`, after | `f2d99f9350a662ae32fa05f57ba9e57c0a44e2ff45dd34a07bafff6e60ae9b50` |
| `fixtures/model_operations/exact_pressure_authoring_model.json` (unchanged) | `7217f923667dbf366c2d6e8449967c8f3f9efb7e96d77c2ecb34679289d2fa0e` |
| `fixtures/product_preview/invented_preview_model.json` (unchanged) | `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c` |
| `validation/witness/inputs/del1005_export_results_success_input.json` (unchanged) | `8f11ad6af8bc4f3a94beb0b216d0b97cedfb9df6cb9e83a12617d0840c22cfea` |
| Unified diff of the test file | `6ab663e97732ed01f0c44a175f218cdba1f1fc7ef6a072692e57f08f264b61b6` ([test_headless_runner_contract.diff](_run_records/test_headless_runner_contract.diff)) |

[ENVIRONMENT.txt](_run_records/ENVIRONMENT.txt) records the source and binary hashes, the toolchain and the Git identities.

## 7. For ROOT to resolve or carry forward

1. **Correct the diagnosis record.** `../ROOT_DISCOVERY/RETURN.md:26` attributes all 8 failures to the legacy demo. Six of them are caused by the stale export witness identity described in §1B. This TASK did not edit that record.
2. **The witness inputs are still stale.** `validation/witness/inputs/del1005_export_results_*_input.json` and `generate_del1005_export_results_inputs.py` still hard-code `0.1.0`, and changing them was outside this TASK's write boundary. Regenerating them under an authorized scope would make the tests' identity binding a no-op on the success path. The identity control uses an explicit `0.1.0` identity, so it would stay valid. The `wire.rs` unit tests inject their own `0.1.0` identity through `linked_product_solver()`, so they pass whatever the linked version is. They need no change, but they do not exercise the real linked identity.
3. **The repair needs independent review.** Under the project `AGENTS.md`, this repair needs fresh-context review of the frozen diff before merge. This TASK implemented the repair and did not review it.
4. **Earlier qualification-gate flake.** The earlier `-n 3` load-timing flake (`input_closed` vs `completed`) did not recur in this single run without `-n`.
5. **Untouched files.** No Git operation, browser, native run or runner/fixture edit was performed. The files other writers are working on were not touched.
