# WORKING_ITEMS RUN — R14 W1 T1 Governed Producer-Path Binding (DEL-04-02 + DEL-04-04 item 1)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Executor:** T1 executor child (governed Agent 2, serialized, non-delegating)
**Sealed brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`
(`CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001`, v3 COMMIT-SAFE per
`instances/W1/T1/VERIFY_BRIEF_V3.md`)
**Deliverables:** DEL-04-02 (sole Remaining binding item), DEL-04-04 (Remaining item 1 only)
**Branch / base commit:** `claude/piping-r14-pkg04-mechanics` at
`6152908b3246df61150dc91e3558788b05dfb643`
**Date:** 2026-07-19

This record is identical in both deliverable folders (one combined tranche,
one integration owner). The v1 executor dispatch on the v2 brief returned
BLOCKED without implementation writes
(`instances/W1/T1/EXECUTE_RETURN.md`, preserved); this run executed the v3
bounded-coverage amendment.

## What Was Implemented

1. **Producer module** `core/runner/headless/src/result_envelope_binding.rs`
   (new): public `build_result_export_document(request, runner_result,
   mechanics)` builds a DEL-08-04 result-export envelope document in the
   exact `result_export_document` wrapper shape from a completed preview
   solve, using only `open_pipe_stress_result_export` public vocabulary.
   `envelope_id` equals the runner result's `result_envelope_ref` reference
   id; both validators (`validate_result_envelope`, typed, and
   `validate_result_with_optional_envelope_payload`) are structural gates
   inside the producer.
2. **Fail-closed wiring** in `core/runner/headless/src/lib.rs`:
   `run_preview_in_memory_with_rule_check` attaches the document to a NEW
   `PreviewRunnerOutput.result_envelope_document` field, marked
   `#[serde(skip_serializing)]` so every existing serialized surface (the
   DEC-065 runner-bin `CliOutput` and the DEC-064 `headless_preview_runner`
   witness stdout) is byte-unchanged. A STRUCTURAL production or validation
   failure on a completed solve appends the blocking runner diagnostic
   `HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED` so the DEC-065 exit
   policy reports the run as not clean. A solve that did not reach
   `MECHANICS_SOLVED` attaches nothing and adds no diagnostic (its existing
   not-clean signaling via missing result references is preserved
   byte-for-byte).
3. **Bounded-coverage mapping (brief §3.2 v3):** an enumerated deterministic
   `(kind, unit)` → (`ResultFamily`, `DimensionId`) table exports rows whose
   semantics match the accepted D-01 unit classification; every other row is
   disclosed per-row (result id, kind, unit) in the non-blocking
   envelope-carried diagnostic `HEADLESS_RUNNER_ENVELOPE_VOCABULARY_BOUNDARY_ROW`
   (class UNIT_WARNING, severity info). No silent drop: exported values plus
   disclosure rows always equal the mechanics result-row count (asserted by
   unit test on both fixtures).
4. **Nonlinear metadata binding (brief §3.3):** the envelope solver-version
   block names `open_pipe_stress_product_physics` with its crate version and,
   only when the solve exercised nonlinear supports, appends
   `nonlinear_component=open_pipe_stress_nonlinear_integration@<version>` to
   the build reference (crate-constant-derived). The assembled loop's
   assumptions/limitations ride as non-blocking
   `NONLINEAR_ASSEMBLED_LOOP_ASSUMPTION` (info) /
   `NONLINEAR_ASSEMBLED_LOOP_LIMITATION` (warning) diagnostics with
   provenance naming the source crate. Linear-only solves carry no nonlinear
   context (asserted by unit test).
5. **Additive public surfaces:**
   `core/product_physics/src/lib.rs` gained `solver_component_name()`,
   `solver_component_version()`, `NonlinearAssembledLoopContext`, and
   `nonlinear_assembled_loop_context()` (pure pass-through of
   `assembled_loop_assumptions()`/`assembled_loop_limitations()` plus
   component identity). `core/solver/nonlinear_integration/src/lib.rs`
   gained `assembled_loop_component_name()`/`assembled_loop_component_version()`.
   No `MechanicsEnvelope` serialized-shape change, no signature change, no
   solve-behavior change, no assumptions/limitations text change.
6. **Dependency:** `core/runner/headless/Cargo.toml` gained a path dependency
   on `core/reporting/result_export` (crate consumed read-only through its
   public API). `Cargo.lock` updated accordingly.

## Bounded Deterministic Judgment Calls (recorded per the v3 brief / V3-D7)

- Exported beyond the straight-pipe floor because D-01 semantics and the
  DEL-08-04 family genuinely agree: `reaction_resultant` (N → reaction/force),
  `open_formula_stress_summary`, `pipe_section_pressure_hoop_stress`, and
  `component_user_stress_multiplier_review` (MPa → stress/stress),
  `expansion_joint_pressure_thrust_load_review`,
  `spring_hanger_user_input_review` (N), and
  `constant_effort_user_input_review` (N) (force values with complete
  five-field metadata → force/force), `nonlinear_support_final_displacement`
  (mm → displacement/length), `nonlinear_support_final_reaction` and both
  friction-normal-reaction rows (N → reaction/force).
- Disclosed, never coerced: user-stiffness review echoes (`N/m`,
  `N*m/rad` — D-01 classifies these `linear_stiffness`/`rotational_stiffness`,
  absent from the DEL-08-04 dimension enum), hanger travel-range rows (`m`),
  `linear_solver_mode_basis` (`mode_code`), nonlinear count/flag/state-code
  rows, all residual-observation rows, and the `N*m` free-DOF work residual
  (an energy quantity, never exported as a moment). Any `(kind, unit)` pair
  outside the table — including unexpected units on known kinds — is
  disclosed.
- Rows whose table family would require the five-field metadata gate
  (force/moment) but whose metadata is incomplete are disclosed rather than
  exported (defensive; no live row currently hits this branch).
- Solve-diagnostic class/severity mapping follows the existing
  `core/analysis_runs/records.py::_diagnostic_class` precedent extended with
  the nonlinear class; unknown severity strings map conservatively to
  blocking (never downgraded). The mechanics diagnostic's `source` string is
  preserved as the provenance `source_name` and the source `Reference`.
- Reproducibility rides the existing checksum machinery only: the runner
  request checksum (as `model_hash`, its `payload_ref` truthfully naming
  `runner_request`), the result-envelope checksum (as the run hash), and the
  runner result's audit-manifest reference. No invented facts.

## Evidence (commands run offline; `CARGO_NET_OFFLINE=true`, `--offline`)

| Check | Outcome |
|---|---|
| `cargo fmt --check` (headless, product_physics, nonlinear_integration) | PASS (all three) |
| `cargo test` headless | PASS: 23 lib (7 new binding tests) + 1 + 15 bin tests, 0 failed |
| `cargo test` product_physics | PASS: 75 + 17 tests, 0 failed (includes new pass-through test) |
| `cargo test` nonlinear_integration | PASS: 12 tests, 0 failed (includes new identity test) |
| `cargo test` result_export (read-only regression) | PASS: 12 tests, 0 failed; crate unchanged |
| `python3 tests/test_headless_runner_contract.py` | PASS (exit 0); `CliOutput` contract surface unchanged, no schema edit needed |
| CLI stability — five `del1005_payload_binding_*` inputs vs committed witnesses | byte-identical (exits 0/0/0/1/1) |
| CLI stability — three `tp_runner_015_final_cli_*` inputs vs pre-tranche baseline captures (taken before the first source edit, at the base commit; ephemeral scratch) | byte-identical (exits 0/1/1); post-change SHA-256 equal to baseline SHA-256: solve `738d3c07…0151614`, validation-blocking `5fb2f8a9…b31a5505`, benchmark-stub `9596c052…fc2449a4` |
| `python3 tools/validation/validate_claims_language.py --repo-root .` (repo root) | PASS (VALID) |
| `python3 tools/validation/validate_path_anchors.py . --text` (repo root) | PASS |
| `git diff --check` (repo root) | PASS |
| Change-scope containment vs §5 fence | PASS; JSON persisted at `instances/W1/T1/CHANGE_SCOPE_CONTAINMENT.json` |

Full command shapes and the §3 predicate table are in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/EXECUTE_RETURN_V2.md`.

## Follow-On Reported (not resolved here)

The DEL-08-04 vocabulary-extension need — dimensions for stiffness
(`linear_stiffness`/`rotational_stiffness` per D-01 and `docs/SPEC.md` §4),
energy/work, and count/state evidence scalars, plus an evidence-scalar
family treatment — is reported to HELP_HUMAN for a later lawful selection.
No `core/reporting/result_export/**` or `schemas/results.schema.yaml` edit
was made.

## Boundaries Preserved

- No CLI output-shape change; no `export-results` binding (DEL-10-05
  follow-on untouched); no witness, benchmark, hand-calc, sweep,
  validation-manual, or docs write; frozen witness files untouched.
- DEL-04-04 Remaining items 2–6 byte-identical: PDU-035 REVIEW disposition
  and dimensional/conversion basis, friction path-history D-XX gate, and all
  threshold-promotion rows (DEC-046/DEC-052/DEC-054 lineage) untouched.
- No new tolerance constant, threshold, acceptance criterion, or release/CI
  vocabulary; no lifecycle transition (both deliverables remain
  `IN_PROGRESS`); no dependency-register, DAG, decomposition, decision, or
  receipt write; no commit, push, branch operation, or network access by the
  executor (the W1 manager commits after independent verification).
- No protected standards content, code-specific allowables, private data, or
  professional/code-compliance claim was introduced; the produced envelope
  pins `HUMAN_REVIEW_REQUIRED` and the project-default professional boundary.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
