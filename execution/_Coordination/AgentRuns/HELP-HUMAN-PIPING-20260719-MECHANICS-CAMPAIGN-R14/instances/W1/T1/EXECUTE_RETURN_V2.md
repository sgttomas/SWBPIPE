# EXECUTE_RETURN_V2 — T1 Governed Producer-Path Binding (DEL-04-02 + DEL-04-04 item 1)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Executor:** T1 executor child (governed Agent 2, serialized, non-delegating)
**Parent:** W1 PKG-04 package manager (WORKING_ITEMS)
**Sealed brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`
(`CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001`, v3 EFFECTIVE; verifier
COMMIT-SAFE at `instances/W1/T1/VERIFY_BRIEF_V3.md`)
**Prior return:** `instances/W1/T1/EXECUTE_RETURN.md` (v2-brief execution,
BLOCKED without implementation writes; preserved history — this v2 return is
the v3-brief execution record)
**Branch / base commit:** `claude/piping-r14-pkg04-mechanics` at
`6152908b3246df61150dc91e3558788b05dfb643`
**Date:** 2026-07-19

---

## 1. Overall Status

**PASS.** The v3 bounded-coverage producer binding is implemented, all §6
checks pass, CLI stdout is byte-stable across all eight pinned cases, and the
§3.8 deliverable-state closeout is applied. The executor made no commit,
push, branch operation, receipt append, or network access; the W1 manager
commits after independent implementation verification.

## 2. Per-Predicate Table (brief §3.1–§3.9, v3)

| Predicate | Status | Evidence |
|---|---|---|
| §3.1 producer exists in the governed path | HOLDS | `core/runner/headless/src/result_envelope_binding.rs`: public `build_result_export_document(&RunnerRequest, &RunnerResult, &MechanicsEnvelope) -> Result<serde_json::Value, Diagnostic>` builds the exact `result_export_document` wrapper (`deliverable_id: "DEL-08-04"`) using only `open_pipe_stress_result_export` public vocabulary; `envelope_id` equals `result_envelope_ref.envelope_ref.ref_id` (unit-tested). `core/runner/headless/Cargo.toml` gained the path dependency (Cargo.lock needed no change — the crate was already in the lock graph — and is git-clean). |
| §3.2 bounded-coverage mapping + per-row disclosure | HOLDS | Enumerated deterministic `(kind, unit)` table (module `mapped_family_dimension`, doc-commented against D-01): straight-pipe element displacement/rotation/force/moment/stress and reaction classes export as `QuantityResult` values (element force/moment rows carry the five-field metadata through the validator gate); every out-of-table row — stiffness review echoes (`N/m`, `N*m/rad`), count/flag/state-code, residual-observation rows incl. the `N*m` work residual, solver-mode basis, travel-range rows, any unknown pair — is disclosed per-row (id, kind, unit) via non-blocking `HEADLESS_RUNNER_ENVELOPE_VOCABULARY_BOUNDARY_ROW` (severity info) in the envelope document itself. Unit-test invariant on both fixtures: exported values + disclosure rows == mechanics result-row count (no silent drop). No new family/dimension identifier; no coercion. Every `MechanicsEnvelope` diagnostic maps into envelope diagnostics with its `source` preserved in the diagnostic provenance. Vocabulary-extension follow-on recorded (§7). |
| §3.3 nonlinear metadata bound | HOLDS | Solver-version block: `solver_name`/`solver_version` from `open_pipe_stress_product_physics` crate constants (new accessors); `solver_build_ref` appends `nonlinear_component=open_pipe_stress_nonlinear_integration@<version>` (crate-constant pass-through) only when the solve exercised nonlinear supports. Assembled-loop assumptions (7) and limitations (6) ride as non-blocking `NONLINEAR_ASSEMBLED_LOOP_ASSUMPTION` (info) / `NONLINEAR_ASSEMBLED_LOOP_LIMITATION` (warning) diagnostics with provenance naming `open_pipe_stress_nonlinear_integration`. Linear-only solve: no nonlinear rows, no nonlinear build-ref token (unit-tested both ways). Count/flag/state-code and residual rows ride the §3.2 disclosure per the v3 text. |
| §3.4 both validators pass on success fixtures | HOLDS | Producer gates on both validators internally; unit tests re-assert no blocking diagnostics for the straight-pipe/nonlinear-bearing fixture (`invented_preview_model.json`) and the linear-only variant (same model minus nonlinear supports). |
| §3.5 fail-closed wiring, serde-excluded field | HOLDS | `run_preview_in_memory_with_rule_check` attaches the document to new `PreviewRunnerOutput.result_envelope_document` (`#[serde(skip_serializing)]`; struct derives `Serialize` only — no round-trip surface). STRUCTURAL failure appends blocking `HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED` (unit-tested via doctored envelope-ref mismatch and missing envelope checksum). Disclosure path is non-blocking and changed no exit code (all eight pinned cases byte-identical, §6). Serialization exclusion unit-tested: serialized output carries no `result_envelope_document` key. Bounded judgment recorded: a solve that did not reach `MECHANICS_SOLVED` attaches nothing and appends nothing — it is not a completed solve product, and its existing not-clean signaling (`HEADLESS_RUNNER_RESULT_REFS_MISSING` under `validate_result`) is preserved byte-for-byte (unit-tested). |
| §3.6 status/boundary invariants | HOLDS | Envelope carries `HUMAN_REVIEW_REQUIRED` (mapped from the runner result), project-default professional boundary, `deterministic_ordering: true` (source-order export of a deterministic solve), and reproducibility from the existing checksum machinery only (runner-request checksum as model hash with truthful `payload_ref`, result-envelope checksum as run hash, existing audit-manifest ref). No tolerance constant, threshold, acceptance criterion, or release/CI vocabulary introduced. |
| §3.7 CLI stability | HOLDS | (a) five del1005 inputs: stdout byte-identical to committed witnesses, exits 0/0/0/1/1; (b) three tp_runner_015 inputs: stdout byte-identical to the pre-tranche baseline captures made before the first source edit at the base commit (post-change SHA-256 equal to baseline: solve `738d3c07…0151614`, validation-blocking `5fb2f8a9…b31a5505`, benchmark-stub `9596c052…fc2449a4`); committed tp_runner_015 witness files untouched; (c) `CliOutput` shape unchanged (`schemas/headless_runner.schema.yaml` and `tests/test_headless_runner_contract.py` unchanged; contract test passes). |
| §3.8 bounded state update | HOLDS | DEL-04-02 `_STATUS.md`: sole Remaining item struck (section left empty per repo precedent), one new History entry citing the brief doc id and R14-W1-T1, `Last Updated` 2026-07-19. DEL-04-04 `_STATUS.md`: Remaining item 1 struck, items 2–6 byte-identical, one new History entry, `Last Updated` updated. One new MEMORY.md entry each (newest-first). One new `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md` in each folder (identical content; records the bounded-coverage boundary and names the vocabulary-extension follow-on as reported to HELP_HUMAN). No new Remaining rows added anywhere. |
| §3.9 checks | HOLDS | §3 tally below. |

## 3. Check Tally (§6 order; all cargo offline: `CARGO_NET_OFFLINE=true`, `--offline`)

| # | Command (from `WORKING_ROOT` unless noted) | Outcome |
|---|---|---|
| 1 | `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check` | PASS |
| 2 | `cargo fmt --manifest-path core/product_physics/Cargo.toml --check` | PASS |
| 3 | `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check` (crate touched) | PASS |
| 4 | `cargo test --offline --manifest-path core/runner/headless/Cargo.toml` | PASS: 23 lib tests (7 new `result_envelope_binding` tests) + 1 `headless_preview_runner` test + 15 `openpipestress-runner` tests; 0 failed |
| 5 | `cargo test --offline --manifest-path core/product_physics/Cargo.toml` | PASS: 75 tests (incl. new `nonlinear_context_passthrough_tests`); 0 failed |
| 6 | `cargo test --offline --manifest-path core/solver/nonlinear_integration/Cargo.toml` | PASS: 17 tests (incl. new `component_identity_tests`); 0 failed |
| 7 | `cargo test --offline --manifest-path core/reporting/result_export/Cargo.toml` (read-only regression) | PASS: 12 tests; crate byte-unchanged |
| 8 | `python3 tests/test_headless_runner_contract.py` | PASS (exit 0) |
| 9 | §4.3 CLI-stability diffs: `openpipestress-runner <verb> --input validation/witness/inputs/<case>_input.json` piped to `cmp` against committed witnesses (five del1005 cases) and against the pre-tranche baseline captures (three tp_runner_015 cases) | PASS: all eight byte-identical (§2 row §3.7) |
| 10 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | PASS: VALID, DEC-081 taxonomy satisfied (re-run after every durable write incl. this return) |
| 11 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | PASS (re-run after every durable write incl. this return) |
| 12 | `git diff --check` (REPO_ROOT) | PASS (re-run after every durable write) |
| 13 | JSON parse of new/changed `.json` files | PASS: `CHANGE_SCOPE_CONTAINMENT_V2.json` parses (tool-emitted, re-parsed); no other `.json` file written |
| 14 | `python3 tools/software_workflow/validate_change_scope.py <REPO_ROOT> --base 6152908b3… --allowed <§5 fence paths>` (REPO_ROOT) over the executor's durable write set | PASS, 0 violations, 16 paths; persisted at `instances/W1/T1/CHANGE_SCOPE_CONTAINMENT_V2.json` (the v1 BLOCKED-run JSON is preserved unchanged at `CHANGE_SCOPE_CONTAINMENT.json`) |

**Concurrent-state observation (surfaced, not a violation of this fence):** a
full-tree containment sweep additionally lists two coordination files this
executor did not write and that were not present at execution start:
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`
and
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T3_DEL-04-01_ARC_PRESSURE_THRUST.md`
(plus `instances/W1/T2/` and `instances/W1/T3/` rationale files inside the
R14 AgentRuns directory). These are the W1 manager's own concurrent T2/T3
authoring state in the shared checkout, reported here for the manager's
containment accounting at commit time.

Wave-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) are the W1 manager's at wave
closeout, per §6, and were not run by this executor.

## 4. Changed-Path List (executor durable writes; all inside the §5 fence)

1. `core/runner/headless/Cargo.toml` (result_export path dependency)
2. `core/runner/headless/src/lib.rs` (module decl; serde-excluded `PreviewRunnerOutput.result_envelope_document`; fail-closed `attach_result_envelope_document` wiring; clone-not-move of two request fields)
3. `core/runner/headless/src/result_envelope_binding.rs` (new module + 7 unit tests)
4. `core/product_physics/src/lib.rs` (additive: `solver_component_name/version`, `NonlinearAssembledLoopContext`, `nonlinear_assembled_loop_context()`, pass-through test; no serialized-shape, signature, or solve-behavior change)
5. `core/solver/nonlinear_integration/src/lib.rs` (additive: `assembled_loop_component_name/version` + test; no solve-behavior or assumptions/limitations-text change)
6–8. DEL-04-02 folder: `_STATUS.md`, `MEMORY.md`, new `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`
9–11. DEL-04-04 folder: `_STATUS.md`, `MEMORY.md`, new `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`
12–16. `instances/W1/T1/`: `EXECUTE_RETURN.md` (v1 BLOCKED return, prior dispatch), `CHECK_LOG.md` (v1), `CHANGE_SCOPE_CONTAINMENT.json` (v1, preserved), `CHANGE_SCOPE_CONTAINMENT_V2.json`, `EXECUTE_RETURN_V2.md` (this file)

Not changed: `core/runner/headless/Cargo.lock` (already carried the crate in
its lock graph; git-clean), `core/runner/headless/src/bin/openpipestress-runner.rs`,
`schemas/headless_runner.schema.yaml`, `tests/test_headless_runner_contract.py`,
`core/reporting/result_export/**`, `schemas/results.schema.yaml`, all
`validation/**` and `docs/**` paths, `loop/**`, and every witness file.
Ephemeral: cargo target dirs and baseline/post-change stdout captures in the
task-local scratch area outside durable project paths.

## 5. CLI-Stability Results

| Case | Exit | Comparison target | Result |
|---|---|---|---|
| `del1005_payload_binding_benchmark_single_case` | 0 | committed witness | byte-identical |
| `del1005_payload_binding_benchmark_multi_case` | 0 | committed witness | byte-identical |
| `del1005_payload_binding_regression_full_suite` | 0 | committed witness | byte-identical |
| `del1005_payload_binding_benchmark_payload_missing` | 1 | committed witness | byte-identical |
| `del1005_payload_binding_regression_payload_missing` | 1 | committed witness | byte-identical |
| `tp_runner_015_final_cli_solve` | 0 | pre-tranche baseline (SHA `738d3c07…0151614`) | byte-identical |
| `tp_runner_015_final_cli_validation_blocking` | 1 | pre-tranche baseline (SHA `5fb2f8a9…b31a5505`) | byte-identical |
| `tp_runner_015_final_cli_benchmark_stub` | 1 | pre-tranche baseline (SHA `9596c052…fc2449a4`) | byte-identical |

Baselines were captured from the offline build at the base commit before the
first source edit (during the prior BLOCKED dispatch, same checkout, same
base commit; SHA-256 recorded in `EXECUTE_RETURN.md` §6) and reused per the
manager dispatch. Full baseline SHA-256 values:
`738d3c074dd90ca97497f2710aac424385e0e85144e93bcee09ba6c2a0151614`,
`5fb2f8a9c8264be665581d58fb55e342a96636a87dc23f7c0db743c3b31a5505`,
`9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4`.

## 6. Bounded Deterministic Judgment (recorded per v3 §4 / verifier V3-D7)

Recorded in full in both deliverable run records; summary:

- Exported beyond the straight-pipe floor (D-01 and family semantics agree):
  `reaction_resultant`; the three MPa stress-summary/hoop/multiplier kinds;
  hanger and expansion-joint force-valued review rows (complete five-field
  metadata); `nonlinear_support_final_displacement`/`final_reaction` and both
  friction-normal-reaction kinds.
- Disclosed, never coerced: stiffness echoes (`N/m`, `N*m/rad`),
  travel-range (`m`), `linear_solver_mode_basis`, all count/flag/state-code
  and residual-observation rows, the `N*m` work residual, and every unlisted
  `(kind, unit)` pair (table is unit-exact).
- Defensive gate: a table row whose family requires five-field metadata but
  whose metadata is incomplete is disclosed, not exported (no live row hits
  this branch).
- Diagnostic class/severity mapping follows the `records.py` precedent
  extended with the nonlinear class; unknown severities map conservatively to
  blocking.
- `MECHANICS_SOLVED` gate on production (see §2, §3.5 row).

## 7. Follow-On Reported to HELP_HUMAN (not resolved here)

DEL-08-04 vocabulary extension: dimensions for stiffness
(`linear_stiffness`/`rotational_stiffness` per D-01 / `docs/SPEC.md` §4),
energy/work, and count/state evidence scalars, plus an evidence-scalar
family treatment, in `core/reporting/result_export` and
`schemas/results.schema.yaml`. Until then the disclosure path carries those
rows. This is a routing statement only; no repair was made and no new
Remaining row was added anywhere.

## 8. Enumerated Refutable Claims

- **E1.** At v3-execution start the tracked tree was clean at base commit
  `6152908b3` with untracked state limited to the R14 AgentRuns directory
  and the T1 brief; the T2/T3 briefs appeared during execution and were not
  written by this executor (refutable via file timestamps and the §3
  concurrent-state observation).
- **E2.** The executor's durable writes are exactly the 16 paths of §4; all
  lie inside the §5 fence (`CHANGE_SCOPE_CONTAINMENT_V2.json`, PASS, 0
  violations).
- **E3.** `build_result_export_document` is public in
  `core/runner/headless/src/result_envelope_binding.rs`, constructs only
  `open_pipe_stress_result_export` public types, and emits the exact
  `result_export_document` wrapper (deliverable id asserted in test).
- **E4.** The produced document's `envelope_id` equals the runner result's
  `result_envelope_ref.envelope_ref.ref_id` (asserted in
  `nonlinear_bearing_solve_attaches_validated_envelope_with_context`).
- **E5.** On both success fixtures, exported `QuantityResult` count plus
  per-row disclosure count equals the `MechanicsEnvelope` result-row count
  (asserted in both fixture tests) — no silent drop.
- **E6.** No disclosure or assumption/limitation diagnostic carries blocking
  severity (asserted in tests), and the disclosure rides only the envelope
  document (the serde-excluded surface), never `runner_result.diagnostics`.
- **E7.** The mapping table contains no entry whose family/dimension
  contradicts the accepted D-01 classification of its unit; stiffness,
  energy/work, count, flag, mode-code, and state-code rows have no table
  entry (refutable by reading `mapped_family_dimension` against
  `docs/SPEC.md` §4).
- **E8.** Every `MechanicsEnvelope` diagnostic maps into the envelope
  diagnostics with its `source` string preserved as the provenance
  `source_name` (code path `export_mechanics_diagnostic`; count equality
  implicit in document construction).
- **E9.** Nonlinear context (build-ref token, 7 assumption + 6 limitation
  rows naming `open_pipe_stress_nonlinear_integration`) appears for the
  nonlinear-bearing fixture and is entirely absent for the linear-only
  fixture (both asserted).
- **E10.** Crate identities/versions in the solver-version block are derived
  from `CARGO_PKG_NAME`/`CARGO_PKG_VERSION` constants via the new additive
  accessors — no hardcoded duplicates (refutable by reading the accessors
  and their tests).
- **E11.** `PreviewRunnerOutput` serialization contains no
  `result_envelope_document` key (asserted), so the DEC-064
  `headless_preview_runner` stdout surface and the DEC-065 `CliOutput`
  surface are shape-unchanged; the struct derives `Serialize` only.
- **E12.** Structural failures append blocking
  `HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED` to
  `runner_result.diagnostics` (asserted for envelope-ref mismatch and
  missing envelope checksum), which the unchanged DEC-065 bin logic maps to
  a not-clean exit.
- **E13.** All eight pinned CLI cases are byte-identical at the
  implementation head (five vs committed witnesses; three vs pre-tranche
  baselines with SHA-256 equality per §5) with exit codes 0/0/0/1/1 and
  0/1/1.
- **E14.** The four crate test suites and the contract test pass with the
  counts of §3; `core/reporting/result_export` is byte-unchanged
  (`git status` clean on that path).
- **E15.** DEL-04-02's Remaining section is empty (sole item struck) and
  DEL-04-04's Remaining items 2–6 are byte-identical to the base commit
  (refutable via `git diff 6152908b3 -- <path>`); each `_STATUS.md` gained
  exactly one History entry and an updated `Last Updated`; each `MEMORY.md`
  gained exactly one new newest-first entry; exactly one new run record
  exists per folder.
- **E16.** No `validation/**`, `docs/**`, `schemas/results.schema.yaml`,
  `core/reporting/**`, receipt, DAG, decision, or loop file was written; no
  commit/push/branch/network action was taken by this executor.
- **E17.** `MechanicsEnvelope`'s serialized shape, all existing
  product_physics function signatures, solve behavior, and the
  nonlinear-integration assumptions/limitations text are unchanged
  (refutable via `git diff` over the two crates: additions only after the
  existing definitions, plus the two clone-not-move expressions in the
  headless success path).
- **E18.** The vocabulary-extension follow-on is recorded in both run
  records, both MEMORY entries, both History entries, and §7 here, and no
  new Remaining row was added to any `_STATUS.md`.

## 9. Claim Boundary

This return records development verification and screening evidence for one
bounded producer-binding tranche. It makes no run-quality,
review-disposition, or reliance claim; PDU-035, DEC-046, and every
threshold, lifecycle, and evidence-posture hold are untouched.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
