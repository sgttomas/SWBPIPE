# EXECUTE_RETURN — T1 Governed Producer-Path Binding (DEL-04-02 + DEL-04-04 item 1)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Executor:** T1 executor child (governed Agent 2, serialized, non-delegating)
**Parent:** W1 PKG-04 package manager (WORKING_ITEMS)
**Sealed brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`
(`CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001`, v2 COMMIT-SAFE)
**Branch / base commit:** `claude/piping-r14-pkg04-mechanics` at
`6152908b3246df61150dc91e3558788b05dfb643` (verified by `git rev-parse HEAD`
before any durable write; working tree clean apart from the lawful
pre-existing R14 AgentRuns directory and the T1 candidate brief)
**Date:** 2026-07-19

---

## 1. Overall Status

**BLOCKED** — no implementation write was made. Execution stopped under brief
§7 on a discovery made while freezing the execution basis (§4.1) and preparing
the §3.2 mapping design: the brief's §3.2 + §3.5 + §3.7(b) acceptance
predicates are jointly unsatisfiable on the live tree without inventing
semantics, because the live `MechanicsEnvelope` result-row vocabulary contains
rows whose quantities have no truthful `ResultFamily`/`DimensionId` assignment
in the closed DEL-08-04 result-export vocabulary. The §7 disposition applies
verbatim: "any discovery that the mapping cannot be built without inventing
semantics is a failure that stops closeout", and the repair need sits in the
DEL-08-04 crate / results schema, which §7 expressly reserves to "a new lawful
selection, not fixed here". Both deliverables' `_STATUS.md` and `MEMORY.md`
are untouched, as §7 requires for a blocked run.

## 2. Blocking Discovery (Evidence)

### 2.1 The closed DEL-08-04 vocabulary

- `core/reporting/result_export/src/lib.rs` lines 41–56: `DimensionId` has 14
  members (`Dimensionless, Length, Angle, Force, Moment, Stress, Area,
  SectionModulus, SecondMomentArea, Ratio, Time, Temperature, Pressure, Tbd`).
  Lines 58–69: `ResultFamily` has 9 members (`Displacement, Rotation, Force,
  Moment, Reaction, Stress, SectionProperty, Ratio, RuleCheck`) and no
  `Tbd`/evidence member. `schemas/results.schema.yaml` lines 304–322 and
  788–801 carry the same closed enums.
- `validate_result_envelope` blocks `dimension == Tbd`
  (`core/reporting/result_export/src/lib.rs` lines 790–804,
  `RESULT_EXPORT_VALUE_METADATA_INCOMPLETE`), so `Tbd` is not a lawful escape
  for a produced value row on any validator-clean path (§3.4).

### 2.2 Live result rows with no truthful mapping into that vocabulary

Empirical basis: the pre-change offline build at the base commit was run on
the pinned §3.7(b) fixture
`validation/witness/inputs/tp_runner_015_final_cli_solve_input.json`
(invented preview model; straight-pipe spans present). Its
`mechanics_envelope` carries 830 result rows across 44 distinct
`(kind, unit)` pairs, including:

1. **Stiffness user-input echo rows** (decisive):
   - `component_user_stiffness_macro_element_review` — 2 rows unit `N/m`,
     2 rows unit `N*m/rad` (`core/product_physics/src/lib.rs` lines
     6799–6817);
   - `spring_hanger_user_input_review` — 1 row unit `N/m`
     (`core/product_physics/src/lib.rs` lines 6941–6943).
   The accepted project dimension vocabulary classifies these quantities as
   `linear_stiffness` / `rotational_stiffness` (`docs/SPEC.md` lines 175 and
   182–183: "`stiffness` must be classified as `linear_stiffness` or
   `rotational_stiffness`"; `schemas/units.schema.yaml` lines 266–267;
   `schemas/model.schema.yaml` lines 1441–1442; recorded as accepted in
   `execution/_Coordination/_DECISIONS/D-01_unit_catalog_acceptance.md` row
   E3). Neither identifier exists in the DEL-08-04 `DimensionId` enum. Every
   assignable dimension for these rows misstates the accepted classification;
   under §3.2's own terms that is inventing semantics, so the rows are
   fail-closed-unmappable.
2. **Solver-evidence scalar rows** (secondary, independently sufficient for
   the nonlinear-bearing success predicate): `linear_solver_mode_basis`
   (unit `mode_code`; `core/product_physics/src/lib.rs` lines 1978–1983),
   `nonlinear_support_active_set_iteration_count` and
   `..._final_residual_count` (unit `count`), `..._converged_flag` (unit
   `boolean`), `..._state_code` (unit `state_code`) (lines 1614, 1632, 1648,
   1692). `dimensionless` is a truthful dimension for these, but no
   `ResultFamily` member truthfully classifies a count/flag/mode/state
   evidence scalar; the existing project mapping precedent
   (`core/analysis_runs/records.py` lines 270–284, `_result_family`) maps
   exactly such rows to `"TBD"` — a value the DEL-08-04 Rust `ResultFamily`
   does not have.

### 2.3 Why this makes §3.2 + §3.5 + §3.7(b) jointly unsatisfiable

- §3.2 requires, for a solve whose model contains straight-pipe spans, that
  **every** `MechanicsEnvelope` result row be mapped into `result_sets` as a
  `QuantityResult`, with a fail-closed **blocking** diagnostic for any row
  not mappable without inventing semantics, and no silent skip.
- §3.5 requires `run_preview_in_memory_with_rule_check` to append a blocking
  runner diagnostic on a production failure so the DEC-065 exit policy
  reports the run not clean, and forbids a partial envelope on the success
  path. The `solve` verb calls exactly this function
  (`core/runner/headless/src/bin/openpipestress-runner.rs` line 432) and
  computes its exit/clean state from `runner_result.diagnostics`
  (lines 437–441).
- §3.7(b) pins the `solve` verb's stdout on the pinned fixture to the
  pre-tranche baseline, byte-identical.
- The pinned fixture's model contains straight-pipe spans **and** the
  stiffness echo rows of item 2.2(1). Fail-closing on those rows (the only
  truthful §3.2 treatment) appends a blocking diagnostic to
  `runner_result.diagnostics`, which changes the serialized `CliOutput`
  bytes and the exit code — violating §3.7(b). Mapping them instead requires
  a dimension assignment that contradicts the accepted D-01/SPEC §4
  classification — violating §3.2. Attaching a partial envelope that skips
  them while carrying an envelope-internal diagnostic violates §3.2 ("every
  ... result row is mapped") and §3.5 ("No partial/silently-absent envelope
  on the success path"). No fourth option exists inside the §5 fence.
- Independently, §3.3/§3.4 require a nonlinear-bearing success fixture whose
  produced envelope passes both validators with no blocking diagnostics.
  `append_nonlinear_support_loop_results` emits the iteration-count,
  residual-count, converged-flag, and state-code rows unconditionally on
  every successful nonlinear solve (`core/product_physics/src/lib.rs` lines
  1540–1551 and 1614–1692), so the same family-vocabulary gap of item
  2.2(2) applies to every such fixture.

### 2.4 Fence-external repair need (§7 routing)

The cure is a vocabulary extension of the DEL-08-04 surface — for example
adding `linear_stiffness`/`rotational_stiffness` (and an evidence-scalar
family or an evidence-row treatment) to
`core/reporting/result_export/src/lib.rs` and `schemas/results.schema.yaml`
consistent with the accepted D-01 vocabulary — or an owner-approved narrowing
of the §3.2 mapping obligation. Both paths write surfaces the §5 fence
excludes (`core/reporting/result_export/**`, `schemas/results.schema.yaml`)
or amend the sealed predicate text itself, and §7 reserves exactly this case
("any defect discovered in the DEL-08-04 crate, the results schema, ...")
to a new lawful selection by the manager chain. This return does not propose
which cure to select.

## 3. Per-Predicate Table (brief §3.1–§3.9)

| Predicate | Status | Note |
|---|---|---|
| §3.1 producer exists | NOT IMPLEMENTED | Blocked before first source edit by the §3.2 discovery; no code written. |
| §3.2 straight-pipe rows/diagnostics bound | UNSATISFIABLE ON LIVE TREE | Stiffness echo rows (`N/m`, `N*m/rad`) have no truthful `DimensionId`; see §2.2(1)/§2.3. |
| §3.3 nonlinear metadata bound | NOT IMPLEMENTED | The assumptions/limitations diagnostic-row design itself was not refuted; blocked as part of the tranche. |
| §3.4 both validators pass on success fixtures | UNSATISFIABLE FOR NONLINEAR-BEARING FIXTURES | Unconditional count/flag/state rows lack a truthful `ResultFamily`; see §2.2(2)/§2.3. |
| §3.5 fail-closed wiring | NOT IMPLEMENTED | Jointly unsatisfiable with §3.2 + §3.7(b) on the pinned fixture. |
| §3.6 status/boundary invariants | NOT REACHED | — |
| §3.7 CLI stability | BASELINES SECURED; NO DRIFT | (a) five del1005 stdouts reproduced byte-identical to committed witnesses at the base commit; (b) three tp_runner_015 baselines captured pre-edit (ephemeral scratch, SHA-256 in §6); (c) no source edit was made, so `CliOutput` and all witness surfaces are unchanged. |
| §3.8 bounded deliverable-state update | CORRECTLY NOT PERFORMED | Success-only writes; §7 blocked disposition leaves `_STATUS.md`/`MEMORY.md` untouched. |
| §3.9 §6 checks pass | NOT RUN TO CLOSEOUT | Closeout never started; evidence-relevant checks that were run are tallied in §4. |

## 4. Work Performed and Check Tally

All cargo invocations offline (`CARGO_NET_OFFLINE=true`, `--offline`); no
network, install, or toolchain change. Commands run from `WORKING_ROOT`
unless noted; `REPO_ROOT` is the checkout root, `WORKING_ROOT` is
`REPO_ROOT/projects/chirality-piping`.

| # | Command (paraphrased where long) | Outcome |
|---|---|---|
| 1 | `git status --porcelain`; `git rev-parse HEAD` (REPO_ROOT) | Clean apart from lawful pre-existing untracked R14 AgentRuns dir + T1 brief; HEAD `6152908b3` matches the brief's base commit. |
| 2 | `CARGO_NET_OFFLINE=true cargo build --offline --manifest-path core/runner/headless/Cargo.toml --bin openpipestress-runner` | Build succeeded (pre-existing warnings in a benchmark crate only). |
| 3 | Pre-edit baseline capture: `openpipestress-runner <verb> --input validation/witness/inputs/tp_runner_015_final_cli_{solve,validation_blocking,benchmark_stub}_input.json` → ephemeral scratch stdout files | Exit codes 0 / 1 / 1; SHA-256 recorded in §6. Captured BEFORE any source edit per §4.3 (no source edit was ever made). |
| 4 | Same runner on the five `del1005_payload_binding_*` inputs; `cmp` stdout vs the five committed `validation/witness/generated/del1005_payload_binding_*.json` witnesses | Exit codes 0/0/0/1/1 in manual table order; all five byte-identical to the committed witnesses at the base commit. |
| 5 | Vocabulary enumeration over the captured solve stdout (python3, read-only) | 830 result rows; 44 distinct `(kind, unit)` pairs; the unmappable sets of §2.2 confirmed present. |
| 6 | `python3 tools/software_workflow/validate_change_scope.py <REPO_ROOT> --base 6152908b3... --allowed <§5 fence paths>` (REPO_ROOT) | PASS; JSON stdout persisted to `instances/W1/T1/CHANGE_SCOPE_CONTAINMENT.json`. |
| 7 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | Run after writing this return; result recorded in `CHECK_LOG.md` beside this file. |
| 8 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | Run after writing this return; result recorded in `CHECK_LOG.md`. |
| 9 | `git diff --check` (REPO_ROOT) | Run after writing this return; result recorded in `CHECK_LOG.md`. |
| 10 | JSON parse of new/changed `.json` files | `CHANGE_SCOPE_CONTAINMENT.json` parses (tool-emitted JSON re-parsed). |

Not run (closeout-only, never reached): `cargo fmt --check` /
`cargo test --offline` for the four crates, `python3
tests/test_headless_runner_contract.py`, post-change CLI diffs. The
wave-level registered checks belong to the W1 manager, not this executor.

## 5. Changed-Path List

Durable writes of this execution (all inside §5 fence item 8):

1. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/EXECUTE_RETURN.md` (this file)
2. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CHANGE_SCOPE_CONTAINMENT.json`
3. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CHECK_LOG.md`

Zero source, schema, test, witness, deliverable-folder, or receipt writes.
Ephemeral writes: cargo target dir and baseline stdout captures in the
task-local scratch area outside durable project paths. The other untracked
paths under the R14 AgentRuns directory (`ORCHESTRATION_PLAN.md`,
`instances/W1/T1/{CURRENT_CANDIDATE_RATIONALE,VERIFY_BRIEF,VERIFY_BRIEF_V2}.md`,
`instances/W3/RETURN.md`) and the T1 candidate brief are pre-existing
parent-owned state, not writes of this executor.

## 6. CLI-Stability Evidence

Committed-witness comparison at base commit `6152908b3` (pre-change build):

| Case | Exit | vs committed witness |
|---|---|---|
| `del1005_payload_binding_benchmark_single_case` | 0 | byte-identical |
| `del1005_payload_binding_benchmark_multi_case` | 0 | byte-identical |
| `del1005_payload_binding_regression_full_suite` | 0 | byte-identical |
| `del1005_payload_binding_benchmark_payload_missing` | 1 | byte-identical |
| `del1005_payload_binding_regression_payload_missing` | 1 | byte-identical |

Pre-tranche tp_runner_015 baseline captures (ephemeral scratch, stdout bytes,
SHA-256):

| Baseline | Exit | SHA-256 |
|---|---|---|
| `tp_runner_015_final_cli_solve` | 0 | `738d3c074dd90ca97497f2710aac424385e0e85144e93bcee09ba6c2a0151614` |
| `tp_runner_015_final_cli_validation_blocking` | 1 | `5fb2f8a9c8264be665581d58fb55e342a96636a87dc23f7c0db743c3b31a5505` |
| `tp_runner_015_final_cli_benchmark_stub` | 1 | `9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4` |

No source was edited, so the post-change halves of the §4.3 diffs are moot;
the committed witness files themselves are untouched (`git status` clean on
`validation/**`).

## 7. Secondary Discoveries for the Successor Selection

Recorded for the manager's rerun/reselection; none of these was acted on:

1. **`PreviewRunnerOutput` serializes directly on a second frozen surface.**
   `core/runner/headless/src/bin/headless_preview_runner.rs` (lines 92–96)
   prints `serde_json::to_string_pretty(&PreviewRunnerOutput)` to stdout —
   the DEC-064 witness surface recorded at
   `validation/witness/generated/tp_runner_014_headless_entrypoint_preview_run.json`.
   Any always-serialized new field on `PreviewRunnerOutput` changes that
   bin's stdout. A successor implementation should mark the new envelope
   field `#[serde(skip_serializing)]` (library-only surface) or equivalent.
2. **`CliOutput` is shape-safe by construction.** The DEC-065 bin
   destructures `PreviewRunnerOutput` field-by-field (bin lines 464–467), so
   an added field does not change `CliOutput` serialization.
3. **Work-residual classification.** `nonlinear_support_free_dof_work_residual`
   carries unit `N*m` for a work/energy quantity; the accepted D-01
   vocabulary has no work/energy identifier, so its dimension assignment
   under any extended vocabulary needs an explicit ruling (moment vs a new
   identifier).
4. The five del1005 witnesses are byte-current at the base commit (§6),
   re-confirming the v2 brief's C7 premise.

## 8. Enumerated Refutable Claims

- **E1.** The working tree at execution start was clean except the untracked
  R14 AgentRuns directory and the T1 candidate brief, and HEAD was
  `6152908b3246df61150dc91e3558788b05dfb643` (§4 row 1).
- **E2.** No file outside `instances/W1/T1/` was created or modified by this
  execution; both deliverables' `_STATUS.md` and `MEMORY.md` are byte-
  unchanged from the base commit (refutable via `git status` /
  `git diff 6152908b3`).
- **E3.** The three tp_runner_015 baselines were captured from the offline
  build at the base commit BEFORE any source edit (no source edit exists),
  with the SHA-256 values of §6.
- **E4.** At the base commit, runner stdout for the five del1005 inputs is
  byte-identical to the five committed generated witnesses (§6, `cmp`).
- **E5.** The pinned `tp_runner_015_final_cli_solve_input.json` solve output
  contains 830 result rows with 44 distinct `(kind, unit)` pairs, including
  2 rows `(component_user_stiffness_macro_element_review, N/m)`, 2 rows
  `(component_user_stiffness_macro_element_review, N*m/rad)`, and 1 row
  `(spring_hanger_user_input_review, N/m)` (reproducible from the baseline
  capture or a fresh base-commit run).
- **E6.** The DEL-08-04 `DimensionId` enum (crate lines 41–56; schema lines
  304–322) contains no stiffness identifier, while the accepted D-01 unit
  vocabulary (`docs/SPEC.md` 175/182–183; `schemas/units.schema.yaml`
  266–267) requires stiffness quantities to be classified as
  `linear_stiffness` or `rotational_stiffness`.
- **E7.** `validate_result_envelope` emits a blocking diagnostic for any
  value with `dimension == Tbd` (crate lines ~790–804), so `Tbd` cannot
  appear on a §3.4-clean path.
- **E8.** The DEL-08-04 `ResultFamily` enum has no TBD/evidence member, and
  the existing project family-mapping precedent
  (`core/analysis_runs/records.py::_result_family`) returns `"TBD"` for
  count/flag/mode/state evidence rows.
- **E9.** `append_nonlinear_support_loop_results` emits iteration-count,
  final-residual-count, converged-flag, and per-support state-code rows
  unconditionally on every successful nonlinear solve
  (`core/product_physics/src/lib.rs` 1540–1551, 1614–1692), so every
  §3.3/§3.4 nonlinear-bearing success fixture contains rows from E8's class.
- **E10.** The `solve` verb reaches the producer wiring point via
  `run_preview_in_memory_with_rule_check` (bin line 432) and derives its
  exit/clean state from `runner_result.diagnostics` (bin lines 437–441), so
  a §3.5 blocking diagnostic on the pinned fixture necessarily changes the
  §3.7(b) stdout bytes and exit code.
- **E11.** Jointly, E5–E10 entail that §3.2, §3.5, and §3.7(b) cannot all
  hold on the live tree unless the producer assigns the stiffness rows a
  dimension that contradicts the accepted D-01/SPEC classification; this
  execution classifies that assignment as the "inventing semantics" §3.2
  forbids. (An owner or manager may refute the classification judgment; the
  underlying vocabulary facts E5–E10 are independently checkable.)
- **E12.** The cure requires writing `core/reporting/result_export/**` or
  `schemas/results.schema.yaml`, or amending the sealed §3.2 predicate —
  all outside the §5 fence; §7 routes exactly this case back to the manager
  for a new lawful selection.
- **E13.** `validate_change_scope.py` over the base commit with the §5 fence
  as allowed roots reports PASS with zero violations
  (`CHANGE_SCOPE_CONTAINMENT.json`).
- **E14.** `headless_preview_runner` serializes `PreviewRunnerOutput`
  directly to stdout (bin lines 92–96), which is the tp_runner_014 witness
  surface (§7 item 1).
- **E15.** No commit, branch operation, push, fetch, receipt append, or
  network access was performed by this execution.

## 9. Claim Boundary

This return records development screening evidence for a blocked tranche. It
makes no run-quality, review-disposition, or reliance claim; PDU-035,
DEC-046, and every threshold and lifecycle hold are untouched.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
