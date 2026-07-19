# WORKING_ITEMS RUN — 2026-07-19 — DEL-10-05 Benchmark/Regression Payload Bindings (R12, N3)

**Run:** `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12`, node N3
(governed execution child; single serialized executor, no delegation)
**Adopted brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`
(doc_id `CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001`; adoption effective by
DEC-085 / D-52 §2 standing approval after N2 `COMMIT-SAFE`; attribution block
recorded in the brief §10)
**Base commit (recorded before any durable write):**
`96563e8e09b89908e13e6b2f1f1139aca3283855` on branch
`claude/chirality-piping-loop-init-a45657`; working tree clean at execution
start except this run's lawful pre-existing state (the R12 AgentRuns
directory and the candidate brief).
**Disposition:** `SUCCESS` (with the blocked/repaired sequence recorded, not
erased). Implementation and all code-level checks completed and passed. The
mandatory DEC-025 five-surface `evidence-sweep` gate initially could NOT run
in this worktree because required local prerequisites were absent
(unprovisioned `node_modules` binaries and incomplete offline Cargo caches for
unrelated crates whose lockfiles are gitignored and exist only in the primary
checkout); per the brief (§6/§7) the executor returned a truthful `BLOCKED`
and withheld state-changing closeout (same block class as the recorded
DEL-09-04 R7 precedent,
`WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R7_BLOCKED.md`). HELP_HUMAN
(R12 parent) accepted the BLOCKED verdict and executed the recorded
disposition-class environment repair **R12-ENVREPAIR-01**
(`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/ENVIRONMENT_REPAIR_DISPOSITION.md`):
offline copy of already-owner-recorded ignored build state (`node_modules/**`
+ 15 gitignored per-crate `Cargo.lock` files) from the primary checkout — no
network, no durable project write, `git status` unchanged. The registered
`evidence-sweep` check was then re-run identically: **PASS, exit 0**, exactly
one new sweep artifact
`validation/evidence/sweeps/SWEEP_20260719T220236Z_96563e8e09b8-dirty.json`
(285→286; rerun JSON persisted at
`…/instances/N3/CHECK_evidence-sweep_rerun.json`). The withheld §4.4 closeout
was then completed: the benchmark/regression `## Remaining` bullet removed
(`export-results` bullet kept open), `## History` and `MEMORY.md` entries
appended, lifecycle unchanged at `IN_PROGRESS`.

## 1. What was implemented (durable, retained for rerun/fan-in review)

- `core/runner/headless/Cargo.toml` — three new path dependencies on the suite
  crates (mechanics DEL-09-01, stress DEL-09-02, nonlinear DEL-09-03); nothing
  else changed.
- `core/runner/headless/src/benchmark_binding.rs` (NEW) — bounded binding
  module: verb-named payload execution, per-case recorded/observed/delta
  reporting in the suites' regression-evidence claim posture, per-case
  match/fail through crate-encoded predicates only, fail-closed blocking for
  cases without a reusable public comparison surface, and structured blocking
  diagnostics (`HEADLESS_RUNNER_BENCHMARK_*` / `HEADLESS_RUNNER_REGRESSION_*`
  families). No numeric tolerance constant exists in this module.
- `core/runner/headless/src/lib.rs` — one line: `pub mod benchmark_binding;`.
- `core/runner/headless/src/bin/openpipestress-runner.rs` — optional
  verb-named `benchmark`/`regression` payload objects beside `request`
  (`{suite, cases[]}`, mirroring the settled TP-RUNNER-015 `solve` wrapper);
  `run-benchmark` (suites `mechanics`, `stress`) and `run-regression` (suite
  `nonlinear`) execute through the binding module; omitted/empty `cases`
  selects the named suite's full `fixture_inventory()` set; payload-missing
  diagnostics `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` /
  `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING` follow the
  `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING` naming pattern; the stub diagnostic
  `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` is emitted for
  `export-results` ONLY; `downstream_operation_verbs_are_stable_but_stubbed`
  narrowed to `export-results`; ten new per-verb binding unit tests added.
  `solve`/`validate-input` behavior and the DEC-065 `policy` block unchanged.
- Minimal additive accessor changes (brief §4.2 / §5.4):
  - `validation/benchmarks/mechanics/src/lib.rs` — additive
    `pub fn recorded_comparison_holds(observed, recorded) -> bool` exposing the
    crate's already-encoded `INTERNAL_ASSERTION_EPSILON` comparison basis.
  - `validation/benchmarks/stress/src/lib.rs` — same additive accessor; the
    crate's existing `INTERNAL_ASSERTION_EPSILON` constant was un-gated from
    `#[cfg(test)]` (value `1.0e-9` unchanged) so the accessor compiles outside
    tests. No recorded fixture value, expected value, policy record, or README
    changed; both crates' test suites pass unchanged in meaning.
  - `validation/benchmarks/nonlinear/` — untouched; its public
    `NonlinearRegressionCase::run` / `matches_expected_outcome` are reused
    as-is.
- NEW witness surfaces (frozen TP-RUNNER-015 surfaces untouched):
  - `validation/witness/inputs/generate_del1005_payload_binding_inputs.py`
    (NEW deterministic generator) and five NEW
    `validation/witness/inputs/del1005_payload_binding_*_input.json` fixtures;
  - five NEW `validation/witness/generated/del1005_payload_binding_*.json`
    witnesses produced by the built binary.

Fail-closed coverage on the implementation head (regression evidence for
current solver behavior only): mechanics 11 of 21 inventory cases execute and
match, 10 fail closed (no public value-addressable observed surface); stress
12 of 15 execute and match, 3 fail closed; nonlinear 5 of 5 execute and match.
This is the brief §3.4 / N2 finding F3 anticipated fail-closed outcome, not a
tolerance invention.

## 2. Checks run (each with actual exit code)

| # | Check (command) | Exit | Result |
|---|---|---|---|
| 1 | `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check` | 0 | pass |
| 2 | `CARGO_NET_OFFLINE=true cargo test --offline --manifest-path core/runner/headless/Cargo.toml` | 0 | pass (16 + 1 + 15 tests) |
| 3 | `CARGO_NET_OFFLINE=true cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml` | 0 | pass (33 tests) |
| 4 | `CARGO_NET_OFFLINE=true cargo test --offline --manifest-path validation/benchmarks/stress/Cargo.toml` | 0 | pass (23 tests) |
| 5 | `python3 tests/test_headless_runner_contract.py` | 0 | pass (schema and contract test unmodified) |
| 6 | `python3 tools/software_workflow/run_registered_checks.py projects/chirality-piping/software-workflow.json --check piping-pytest --output …/instances/N3/CHECK_piping-pytest.json` (from REPO_ROOT) | 0 | PASS |
| 7 | same tool, `--check evidence-sweep`, output `…/instances/N3/CHECK_evidence-sweep.json` | 1 | FAIL — prerequisite preflight: missing `node_modules/.bin/{playwright,tsc,vite,vitest}`; offline Cargo cache incomplete for several unrelated crates; no provisioning attempted (forbidden) |
| 7b | same tool, `--check evidence-sweep`, re-run by HELP_HUMAN after recorded disposition R12-ENVREPAIR-01; output `…/instances/N3/CHECK_evidence-sweep_rerun.json` | 0 | PASS — sweep `overall_status: pass` at base commit `96563e8e…`; exactly one new artifact `SWEEP_20260719T220236Z_96563e8e09b8-dirty.json` (285→286) |
| 8 | same tool, `--check harness-pytest`, output `…/instances/N3/CHECK_harness-pytest.json` | 0 | PASS |
| 9 | same tool, `--check harness-self-check`, output `…/instances/N3/CHECK_harness-self-check.json` | 0 | PASS |
| 10 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | 0 | VALID (262 files) |
| 11 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | 0 | PASS (618 surfaces) |
| 12 | `git diff --check` | 0 | pass |
| 13 | JSON parse of every new `.json` file (14 files) | 0 | pass (a first attempt exited 1 from a harness-script pathlib bug, not a data defect; the corrected run parsed all 14) |
| 14 | `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" --base 96563e8e… --allowed <each §5 path>` (REPO_ROOT); JSON persisted at `…/instances/N3/CHANGE_SCOPE_CONTAINMENT.json` | 0 | `status: PASS`, `violations: []` |
| 15 | `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .` (read-only; no receipt appended this node) | 0 | VALID through Receipt-58 cursor |

Sweep artifact delta: the initial failed preflight created no `SWEEP_*.json`
(`…/instances/N3/sweeps_before.txt` and `…/instances/N3/sweeps_after.txt`
identical at 285 entries). After the R12-ENVREPAIR-01 repair, the single
successful rerun created exactly one new artifact —
`validation/evidence/sweeps/SWEEP_20260719T220236Z_96563e8e09b8-dirty.json`
(sweeps directory now 286 entries; `-dirty` suffix expected on this dirty
tree, acceptable per Receipt-57 precedent) — satisfying the §5.8
one-new-artifact condition.

Witness executions (built binary, offline):

| Input fixture | Verb | Exit | Content |
|---|---|---|---|
| `del1005_payload_binding_benchmark_single_case_input.json` | run-benchmark | 0 | mechanics, 1 case, 1 executed_and_matched |
| `del1005_payload_binding_benchmark_multi_case_input.json` | run-benchmark | 0 | stress, 3 cases, 3 executed_and_matched |
| `del1005_payload_binding_regression_full_suite_input.json` | run-regression | 0 | nonlinear whole-suite default, 5 of 5 executed_and_matched |
| `del1005_payload_binding_benchmark_payload_missing_input.json` | run-benchmark | 1 | `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` |
| `del1005_payload_binding_regression_payload_missing_input.json` | run-regression | 1 | `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING` |

Frozen-surface integrity: `git status --porcelain` over the three frozen
`tp_runner_015_final_cli_*_input.json` fixtures, their generator, and the
three committed `tp_runner_015_final_cli_*.json` generated witnesses is empty
(byte-identical). DEL-09-04 surfaces untouched.

## 3. Acceptance predicates (brief §3), one by one

1. Benchmark binding — holds on the implementation head (witness exit 0;
   binding unit tests).
2. Regression binding — holds (nonlinear whole-suite witness exit 0; 5/5
   matched against recorded expected states/residual records).
3. Whole-suite default — holds (empty/omitted `cases` selects the full
   `fixture_inventory()` set; exercised for stress and nonlinear).
4. Per-case reporting — holds; cases without a reusable encoded comparison
   basis fail closed with structured diagnostics (anticipated F3 outcome); no
   tolerance invented; claim posture restated in every report.
5. Exit policy (DEC-065) — holds (0 only when all requested cases execute and
   match; 1 for mismatch/unknown/unsupported/missing-payload/blocked; 2
   unchanged for usage/unreadable/malformed/unsupported-operation; every
   requested case appears as matched, mismatched, or blocked).
6. Missing-payload diagnostics — holds; stub diagnostic now `export-results`
   only.
7. DEC-065 policy preservation — holds (policy block byte-unchanged; stdout
   JSON with optional `--output`; single foreground local process; no
   network/daemon/telemetry surface added).
8. No collateral behavior change — holds (solve/validate-input tests
   unchanged and passing; frozen E1 fixtures/generator/witnesses
   byte-identical).
9. Evidence — holds after the recorded repair: witness coverage produced for
   all four required classes, and the full §6 validation plan now passes —
   the `evidence-sweep` gate was initially prerequisite-blocked (truthful
   BLOCKED, closeout withheld), repaired by the recorded R12-ENVREPAIR-01
   disposition, and the identical rerun PASSED with exactly one new sweep
   artifact.

Tally: 9 of 9 hold on the implementation head (with the §3.9
blocked→repaired→pass sequence recorded above).

## 4. Deviations and recorded deltas

- Parent-node instruction (recorded delta from brief §4.4): no
  `loop/LOOP_RECEIPTS.md` append at this node — receipt closeout happens after
  node N4. The receipt validator was still run read-only (VALID).
- While the run stood BLOCKED, the brief's failure disposition governed:
  `_STATUS.md` and `MEMORY.md` were deliberately NOT modified and this run
  record carried the blocked evidence. After HELP_HUMAN's recorded
  R12-ENVREPAIR-01 repair and the PASS rerun of the sweep gate, the withheld
  §4.4 closeout was completed (Remaining bullet removed, History and MEMORY
  entries appended, this record updated to final form) — the blocked/repaired
  sequence is preserved, not erased.
- Brief frontmatter `status`/`agent_classification`/`rule_activation` and the
  bold Status line were updated together with the §10 attribution block
  (fence item 1) so the status record is internally consistent.
- `schemas/headless_runner.schema.yaml` and
  `tests/test_headless_runner_contract.py` were NOT modified: §4.2 conditions
  edits on necessity; the payload objects live in the CLI wrapper beside
  `request` exactly like the settled `solve` wrapper, which the schema
  likewise does not model, and no schema statement became untrue (contract
  test passes unchanged).
- `core/runner/headless/Cargo.lock` was created by the offline build but the
  crate's pre-existing `.gitignore` ignores it; it remains local/untracked
  (writing it is enumerated in §5.2; no fence effect).
- Known recorded consequence (brief §7): the E1 case-3 documented expectation
  (`run-benchmark` stub, exit 1 with the stub diagnostic) is historical once
  this binding lands; `docs/validation_manual/**` and DEL-09-04 wording are
  out of fence and routed to HELP_HUMAN as follow-on.

## 5. Rerun requirement (brief §8) — RESOLVED

The §8 "prior failed/blocked result after the underlying condition is
resolved" trigger fired within the same run: HELP_HUMAN's recorded
R12-ENVREPAIR-01 disposition resolved the environment condition without
network, installation, toolchain update, or any durable project write, and
the `evidence-sweep` gate was re-run identically with PASS (exit 0; artifact
`SWEEP_20260719T220236Z_96563e8e09b8-dirty.json`; rerun JSON
`…/instances/N3/CHECK_evidence-sweep_rerun.json`). The withheld closeout was
then completed under the same brief and fence. No acceptance, lifecycle,
stage, release, or professional-reliance inference is permitted from this
record; per-case match results remain regression evidence for current solver
behavior only.
