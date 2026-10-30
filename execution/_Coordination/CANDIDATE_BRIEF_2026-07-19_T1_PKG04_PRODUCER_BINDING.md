---
doc_id: CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-19
package_id: PKG-04
deliverable_id: DEL-04-02, DEL-04-04
decision_basis: DEC-049 (context), DEC-046 (preserved gate), DEC-065, DEC-025, DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — T1 Governed Producer-Path Binding (DEL-04-02 straight-pipe + DEL-04-04 nonlinear metadata)

**Status:** `EFFECTIVE (v3) — EXECUTION RELEASED BY W1 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V3 COMMIT-SAFE)`

**Amendment record (v2, 2026-07-19):** the first fresh-context verifier
(`instances/W1/T1/VERIFY_BRIEF.md`) returned `BLOCK`, refuting the v1 premise
that all eight committed runner witnesses are current at HEAD: offline
reproduction shows the five `del1005_payload_binding_*` witnesses
byte-identical at `6152908b3`, but the three `tp_runner_015_final_cli_*`
witnesses are truthful only for their pinned pre-#287 commits (as the
R13-refreshed reproduction manual records). §1, §2, §3.7, §4.3, and §6 are
amended accordingly: byte-stability for the tp_runner_015 cases is now
defined against the pre-tranche build at the base commit, not against the
historical committed witnesses; the DEC-025 sweep treatment now cites the
controlling W1 dispatch. No other section changed in meaning.

**Amendment record (v3, 2026-07-19):** the first executor dispatch returned
`BLOCKED` without any implementation write
(`instances/W1/T1/EXECUTE_RETURN.md`, preserved): on the live tree the
`MechanicsEnvelope` solve output includes result rows whose accepted unit
classification has no truthful DEL-08-04 `ResultFamily`/`DimensionId`
representation (user-stiffness review echoes in `N/m`/`N*m/rad`, nonlinear
count/flag/state-code and residual-observation rows), so the v2 predicate
set (§3.2 map-every-row fail-closed + §3.5 blocking-on-failure + §3.7(b)
byte-stability on the pinned solve fixture) was jointly unsatisfiable, and
the cure by DEL-08-04 vocabulary extension is fence-external. §3.2, §3.3,
§3.5, and §4 are amended to a bounded-coverage mapping: rows inside the
accepted DEL-08-04 vocabulary export as `QuantityResult` values; rows
outside it are disclosed per-row in a non-blocking vocabulary-boundary
diagnostic (no silent drop, no invented identifier, no exit-code change);
the DEL-08-04 vocabulary-extension need is reported to HELP_HUMAN as a
follow-on. The executor's secondary discovery (the
`headless_preview_runner` serialization surface) is cured by excluding the
new `PreviewRunnerOutput` field from serialization. No other section
changed in meaning.

**Prepared by:** WORKING_ITEMS (W1, PKG-04 package manager) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W1, tranche T1

**Selected work items** (one combined tranche, one integration owner, per the
R14 campaign plan):

1. DEL-04-02 `_STATUS.md ## Remaining` sole item: "Bind straight-pipe element
   diagnostics/results into the governed analysis-run/result-envelope producer
   path; the T4 persisted-record report reader does not create that
   solver-to-envelope integration."
2. DEL-04-04 `_STATUS.md ## Remaining` item 1 only: "Bind nonlinear solver
   version, warnings, assumptions, limitations, and diagnostic provenance into
   the governed analysis-run producer path; the T4 persisted-record report
   reader does not create that producer integration or alter
   PDU-035/threshold holds."

Both items share one surface — the analysis-run producer and the DEL-08-04
result envelope — so they are executed as one tranche with one write fence and
one serialized integration owner (the T1 executor child).

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on the
D-52/`DEC-085` standing-approval overlay. Adoption remains the human owner's
conditional act under the standing rule; this document classifies and proposes
only. The adoption effect is `HELD` until independent refutation returns
`COMMIT-SAFE` and the W1 manager progresses the chain under the R14
campaign-plan execution rules. No execution writes are authorized by this
document in its current state.

## 1. Purpose and Accepted Basis

Purpose: create the missing solver-to-envelope integration in the governed
analysis-run producer path, so that a live preview solve produces a validated
DEL-08-04 result-export envelope document that (a) carries the straight-pipe
element results and diagnostics of the solve and (b) carries nonlinear solver
version, warnings, assumptions, limitations, and diagnostic provenance
whenever the solve exercised the nonlinear active-set loop — with no
threshold, tolerance, review-disposition, or lifecycle act, and no change to
the committed runner CLI witness surfaces.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(HEAD `6152908b3246df61150dc91e3558788b05dfb643`, branch
`claude/piping-r14-pkg04-mechanics`):

- root `AGENTS.md` and project `AGENTS.md`;
- active committed-HEAD workplan `loop/WORKPLAN_2026-07-18b_piping_loop.md`
  and structurally valid `loop/LOOP_RECEIPTS.md` through cursor `Receipt-60`;
- campaign plan
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/ORCHESTRATION_PLAN.md`
  (plan version 1, posture MIXED, HUMAN selection authority for the
  mechanics-lane queue; W1 = PKG-04 manager; serialized
  author→verify→execute→verify child chain per tranche);
- `execution/_DAG/_LATEST.md` resolving to approved `DAG-007`
  (`approved_active_graph_authority`);
- DEL-04-02 deliverable folder (under
  `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/`):
  `_STATUS.md` (`IN_PROGRESS`; single Remaining item), `MEMORY.md` (2026-07-12
  PDU-040 entry: report reader "does not bind straight-pipe
  diagnostics/results into the analysis-run producer"), `Dependencies.csv`,
  and `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T4-PDU012-PDU021-PDU022-PDU040.md`;
- DEL-04-04 deliverable folder (under
  `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/`):
  `_STATUS.md` (`IN_PROGRESS`; six Remaining items, item 1 selected),
  `MEMORY.md` (2026-07-12 PDU-021 entry: "Direct nonlinear-solver producer
  binding and existing PDU-035/threshold holds remain open"),
  `Dependencies.csv`;
- producer-path ground truth (live source):
  - `core/runner/headless/src/lib.rs` — `run_preview_in_memory` /
    `run_preview_in_memory_with_rule_check` produce `PreviewRunnerOutput`
    (`RunnerResult` + `MechanicsEnvelope`); the result-envelope surface today
    is reference-and-checksum only (`ResultEnvelopeRef`, a `result_envelope`
    checksum over the serialized `MechanicsEnvelope`); the existing
    `validate_result_with_optional_envelope_payload` contract already
    validates a DEL-08-04 result-export document (`deliverable_id ==
    "DEL-08-04"`, envelope-id match, `HUMAN_REVIEW_REQUIRED`, non-empty
    `result_sets`, `deterministic_ordering`, checksum presence) but no
    production code path produces such a document from a live solve;
  - `core/reporting/result_export/src/lib.rs` (DEL-08-04) — public
    `ResultEnvelope` vocabulary, `validate_result_envelope`, and
    `result_export_document` (the schema-first JSON wrapper with
    `deliverable_id: "DEL-08-04"`);
  - `core/product_physics/src/lib.rs` — `run_linear_static_preview*` returns
    `MechanicsEnvelope` (results, diagnostics with optional `source`,
    professional boundary); straight-pipe element results and diagnostics
    already populate the envelope; `append_nonlinear_support_loop_results`
    maps nonlinear solver diagnostics into envelope diagnostics and emits
    iteration/residual/state/convergence result rows, but drops the
    `NonlinearFrameSolveResult.assumptions` and `.limitations` vectors;
  - `core/solver/nonlinear_integration/src/lib.rs` —
    `NonlinearFrameSolveResult { policy_ref, diagnostics, assumptions,
    limitations, ... }`, `assembled_loop_assumptions()`,
    `assembled_loop_limitations()`; crate version `0.1.0`;
  - `core/analysis_runs/records.py` and `core/product_preview/service.py` —
    the Python DEL-14-02 preview analysis-run record builder (fixture-driven;
    not this tranche's binding surface; unchanged);
- frozen CLI witness surfaces (read-only; CLI-stability predicate §3.7):
  the three `validation/witness/inputs/tp_runner_015_final_cli_*_input.json`
  fixtures, `validation/witness/inputs/generate_tp_runner_015_inputs.py`, the
  three committed `validation/witness/generated/tp_runner_015_final_cli_*.json`
  witnesses (historical records, truthful for their pinned pre-#287 commits
  per `docs/validation_manual/headless_runner_reproduction.md`; NOT current
  at HEAD), the five
  `validation/witness/inputs/del1005_payload_binding_*_input.json` fixtures,
  `validation/witness/inputs/generate_del1005_payload_binding_inputs.py`, and
  the five committed
  `validation/witness/generated/del1005_payload_binding_*.json` witnesses
  (current at HEAD, verified by the v1 verifier's offline reproduction);
- governance: `DEC-065` (local CLI/process and exit-code policy), `DEC-046`
  (governed convergence tolerance records; promotion owner-gated and
  untouched), `DEC-049` (spring-hanger scope context), `DEC-025`
  (five-surface evidence sweep as pre-push gate for code-touching branches),
  `DEC-081` (claims taxonomy), D-52/`DEC-085`, D-54/`DEC-087`, and the
  structural model briefs
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`
  and `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-09-04_VALMANUAL_REFRESH.md`;
- `software-workflow.json` under the ratified root
  `docs/SOFTWARE_WORKFLOW_PROFILE.md` contract, and the root tools
  `tools/software_workflow/run_registered_checks.py` (CLI: `profile`
  positional, `--check`, mandatory `--output`) and
  `tools/software_workflow/validate_change_scope.py` (CLI: `repo` positional,
  `--allowed`, `--base`).

## 2. Live Selection Facts

- DEL-04-02 is `IN_PROGRESS`; its single Remaining item is the selected
  binding. All eight deliverable-local `EXECUTION UPSTREAM` rows are
  `SATISFIED` (`DAG-002-E0105..E0109`, `E0432..E0434`); the deliverable is
  DAG-unblocked.
- DEL-04-04 is `IN_PROGRESS`; item 1 is selected. Items 2–6 (PDU-035 REVIEW
  disposition, friction path-history D-XX, three threshold-promotion rows)
  are owner-gated or stage-gated and are excluded (§9). Its four
  deliverable-local non-constraint `EXECUTION UPSTREAM` rows
  (`TP-DAG-004-DEL-04-04-E006..E009`) carry `SatisfactionStatus PENDING`
  while the five root constraints are `SATISFIED`. The `PENDING` values
  record satisfaction bookkeeping, not a live defect: each target deliverable
  (DEL-04-03, DEL-04-01, DEL-04-06, DEL-02-02) has committed implementation
  evidence in the live tree that this tranche consumes read-only, the same
  posture under which the deliverable's prior landed slices executed. This
  tranche does not resolve, promote, or edit any dependency row. Selection
  authority for the R14 queue is HUMAN (owner in-session direction recorded
  in the campaign plan), which covers working these named Remaining items.
- The producer gap is confirmed in the live tree at preparation: no
  non-test call site constructs `open_pipe_stress_result_export::ResultEnvelope`
  (`grep` over `core/**` finds constructors only in
  `core/reporting/result_export/src/lib.rs` tests and
  `core/reporting/report_package/tests/container.rs`), and
  `NonlinearFrameSolveResult.assumptions`/`.limitations` have no consumer in
  `core/product_physics`.
- Witness currency at HEAD is mixed (v1 verifier's offline reproduction):
  the five committed `del1005_payload_binding_*` witnesses are byte-current;
  the three committed `tp_runner_015_final_cli_*` witnesses are historical
  (pre-#287 pinned commits — solve output has since gained nonlinear-support
  delta rows, `CliOutput` gained `suite_run`, and the benchmark-stub case now
  reports `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`), exactly as the
  R13-refreshed reproduction manual records. Any tranche-induced change to
  runner CLI stdout would stale the just-refreshed DEL-09-04 manual again;
  this brief therefore selects a library-level binding whose CLI stdout is
  unchanged relative to the pre-tranche build (§3.7, §4).
- Receipt cursor is `Receipt-60`; the R14 campaign plan is the parent
  authority for this wave; the branch for the wave is
  `claude/piping-r14-pkg04-mechanics` at `6152908b3`.

## 3. Objective and Acceptance Predicates

Create the solver-to-envelope binding so that all of the following hold on
the implementation head:

1. **Envelope producer exists in the governed path.** A public producer in
   `core/runner/headless` builds a DEL-08-04 result-export envelope document
   (`serde_json::Value` in the exact `result_export_document` wrapper shape,
   `deliverable_id: "DEL-08-04"`) from a completed preview solve
   (`RunnerRequest` identity + `RunnerResult` + `MechanicsEnvelope`), using
   only `open_pipe_stress_result_export` public vocabulary. The document's
   `envelope_id` equals the runner result's `result_envelope_ref` reference id.
2. **Straight-pipe results and diagnostics are bound (bounded-coverage
   mapping, v3).** For a solve whose model contains straight-pipe spans:
   every `MechanicsEnvelope` result row whose `(kind, unit)` pair resolves
   through an explicit, enumerated, deterministic mapping table to an
   existing DEL-08-04 `ResultFamily` and `DimensionId` whose semantic
   meaning matches the accepted unit classification is exported into the
   envelope's `result_sets` as a `QuantityResult`; every row outside that
   accepted vocabulary (including user-stiffness review echoes, nonlinear
   count/flag/state-code rows, and residual-observation rows) is disclosed
   per-row — by result id, kind, and unit — in a dedicated NON-BLOCKING
   vocabulary-boundary diagnostic with provenance; no row is silently
   dropped, no new family/dimension identifier is introduced, and the
   mapping table never assigns a semantically false family or dimension.
   Every `MechanicsEnvelope` diagnostic is mapped into the envelope's
   `diagnostics` with its source preserved in the diagnostic provenance.
   Straight-pipe element displacement/rotation/force/moment/stress/reaction
   rows produced by the existing solve appear as exported `QuantityResult`
   values for the test fixtures. The DEL-08-04 vocabulary-extension need
   (stiffness, energy/work, count/state dimensions) is recorded as a
   reported follow-on for HELP_HUMAN, not resolved here.
3. **Nonlinear solver metadata is bound.** For a solve whose model contains
   nonlinear supports: the envelope's `solver_version` block names the
   product-physics solver with its crate version and records the
   nonlinear-integration component identity/version in its build reference
   or provenance fields (crate-constant-derived, not hardcoded duplicates);
   the `assumptions` and `limitations` vectors of the assembled active-set
   loop (as exposed by `core/solver/nonlinear_integration`) are bound into
   the envelope as non-blocking assumption/limitation diagnostic rows with
   provenance naming their source crate; nonlinear solver warning
   diagnostics already carried by the `MechanicsEnvelope` arrive with their
   diagnostic provenance preserved (predicate 2 covers the mapping). For a
   solve with no nonlinear supports, no nonlinear assumption/limitation rows
   are emitted (no false context). The nonlinear count/flag/state-code and
   residual-observation result rows fall under the predicate-2
   vocabulary-boundary disclosure; the named metadata of this item (solver
   version, warnings, assumptions, limitations, diagnostic provenance) binds
   fully through the solver-version block and diagnostics.
4. **Both validators pass on the success path.**
   `open_pipe_stress_result_export::validate_result_envelope` returns no
   blocking diagnostics for the produced typed envelope, and
   `validate_result_with_optional_envelope_payload(runner_result,
   Some(&document))` returns no blocking diagnostics, for the success-path
   test fixtures (straight-pipe-only and nonlinear-bearing).
5. **Producer is wired into the preview producer path, fail-closed (v3).**
   `run_preview_in_memory_with_rule_check` attaches the produced envelope
   document to `PreviewRunnerOutput` (new optional field, excluded from
   serde serialization so every existing serialization surface — including
   the `headless_preview_runner` witness surface and the runner bin — is
   byte-unchanged) when the solve completes and the document validates
   cleanly. A STRUCTURAL production or validation failure (serialization
   failure, validator blocking diagnostics, envelope-id mismatch) on an
   otherwise-completed solve appends a blocking runner diagnostic (new code
   following the existing `HEADLESS_RUNNER_*` naming pattern) so the
   DEC-065 exit policy reports the run as not clean. The predicate-2
   vocabulary-boundary disclosure is NOT a structural failure: it is
   non-blocking, changes no exit code, and must occur on the pinned solve
   fixture without violating predicate 7. No partial/silently-absent
   envelope on the success path.
6. **Analysis-status and boundary invariants preserved.** The envelope
   carries `HUMAN_REVIEW_REQUIRED` in `analysis_status`, the project-default
   professional boundary, deterministic ordering evidence, and truthful
   provenance/reproducibility references (existing checksum machinery; no
   invented facts). No new tolerance constant, threshold, acceptance
   criterion, or release/CI vocabulary is introduced anywhere in the tranche.
7. **CLI stability relative to the pre-tranche build.** (a) For the five
   `del1005_payload_binding_*` inputs, runner stdout at the implementation
   head is byte-identical to the five committed generated witnesses. (b) For
   the three `tp_runner_015_final_cli_*` inputs, runner stdout at the
   implementation head is byte-identical to runner stdout captured from the
   pre-tranche build at the recorded base commit (baseline captured first,
   as ephemeral files outside durable project paths); the historical
   committed tp_runner_015 witnesses remain untouched and are NOT the
   comparison target, because they are truthful only for their pinned
   pre-#287 commits. (c) The `CliOutput` serialization shape is unchanged
   this tranche; CLI exposure of the envelope document is the DEL-10-05
   `export-results` follow-on, not this tranche.
8. **Bounded state update.** On success only: DEL-04-02 `_STATUS.md` strikes
   exactly its Remaining binding item; DEL-04-04 `_STATUS.md` strikes exactly
   its Remaining item 1, leaving items 2–6 byte-identical; each gets exactly
   one new History entry and updated `Last Updated`; each `MEMORY.md` gains
   exactly one new entry; one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`
   in each of the two deliverable folders. The History/MEMORY/run-record
   wording records the bounded-coverage boundary truthfully (exported
   vocabulary vs disclosed classes) and names the DEL-08-04
   vocabulary-extension follow-on as reported to HELP_HUMAN.
9. **Checks.** The full §6 validation plan passes.

A successful run closes only the two named Remaining items. It does not close
any other DEL-04-04 item, any DEL-10-05 item, or any PKG-08/PKG-09/PKG-14
scope; it does not change PDU-035, DEC-046, or any threshold hold.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several binding shapes were defensible, the selection below was made
under D-54/`DEC-087` reasoned selection; the four-lens analysis and the
materially rejected alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape:

- **Binding module lives in the headless runner crate** (for example
  `core/runner/headless/src/result_envelope_binding.rs`), the crate that
  already owns the analysis-run producer path and the DEL-08-04 payload
  validation contract. `core/runner/headless/Cargo.toml` gains a path
  dependency on `core/reporting/result_export`. The DEL-08-04 crate itself
  is consumed read-only through its public API.
- **`core/product_physics` changes are additive-only:** new public
  surface exposing the nonlinear assembled-loop context for producer
  consumption (component crate name/version plus pass-through of
  `assembled_loop_assumptions()` / `assembled_loop_limitations()`), and, if
  strictly required, additive public accessors; no change to
  `MechanicsEnvelope`'s serialized shape, no change to existing function
  signatures or solve behavior, no recorded-value change.
  `core/solver/nonlinear_integration` may gain an additive public
  crate-version accessor if needed; its solve behavior, assumptions, and
  limitations text are read-only.
- **CLI stdout is deliberately unchanged this tranche.** The envelope
  document rides `PreviewRunnerOutput` (library surface) only; the runner
  bin's `CliOutput` struct and all committed witness bytes stay stable
  (§3.7). Exposing the envelope through a CLI verb is the already-recorded
  DEL-10-05 `export-results` follow-on and is excluded here.
- **Assumptions/limitations ride as diagnostics rows** in the envelope
  document (the DEL-08-04 schema is `additionalProperties: false`; no schema
  edit), using a non-blocking severity and an assumption/limitation
  diagnostic class with provenance naming `core/solver/nonlinear_integration`.
- **Bounded-coverage export with per-row disclosure (v3).** The mapping
  table enumerates the accepted `(kind, unit)` → (`ResultFamily`,
  `DimensionId`) pairs; everything else rides the non-blocking
  vocabulary-boundary disclosure diagnostic(s). The executor derives the
  table from the live DEL-08-04 vocabulary and the accepted D-01 unit
  classification; where they cannot agree for a row class, that class is
  disclosed, never coerced. The new `PreviewRunnerOutput` field carries
  `#[serde(skip_serializing)]` (or equivalent) so no existing serialized
  surface changes.

Bounded tasks for the executor child:

### 4.1 Freeze the execution basis

- Begin on the wave branch `claude/piping-r14-pkg04-mechanics` in the
  integration checkout; record the base commit before any durable write;
  verify the tree is clean apart from this run's lawful pre-existing state
  (the R14 AgentRuns directory and this brief).
- Stop if the DAG pointer, either deliverable's lifecycle/Remaining text, the
  producer-path sources named in §1, or `software-workflow.json` has changed
  materially since this brief. Do not silently reinterpret scope.

### 4.2 Implement the binding

- Add the binding module and producer function per §3.1–§3.6 and the
  selected shape; wire it into `run_preview_in_memory_with_rule_check`
  fail-closed per §3.5.
- Add unit tests in the headless crate covering at minimum: straight-pipe
  solve → envelope carries the straight-pipe exported rows and diagnostics
  with preserved provenance and both validators pass; nonlinear solve →
  envelope carries solver-version/component identity and
  assumption/limitation rows, the count/flag rows appear in the
  vocabulary-boundary disclosure, and both validators pass; linear-only
  solve → no nonlinear context rows; out-of-vocabulary rows → per-row
  non-blocking disclosure with no exit-code effect and no silent drop;
  structural envelope production failure → blocking diagnostic and
  not-clean run.
- Add or extend product_physics tests only as needed for the additive public
  surface.
- Update `schemas/headless_runner.schema.yaml` and
  `tests/test_headless_runner_contract.py` only if the library-level change
  is actually visible in their contract surface (expected: not visible,
  because `CliOutput` is unchanged); never weaken existing checks.

### 4.3 CLI-stability evidence

- BEFORE the first source edit: build the runner offline
  (`CARGO_NET_OFFLINE=true`, `--offline`) at the recorded base commit state
  and capture baseline stdout for the three `tp_runner_015_final_cli_*`
  inputs into ephemeral files outside durable project paths.
- AFTER implementation: rebuild offline and (a) diff stdout for the five
  `del1005_payload_binding_*` inputs against the five committed generated
  witnesses (must be byte-identical), and (b) diff stdout for the three
  tp_runner_015 inputs against the captured baselines (must be
  byte-identical). Record both results in the run records. Do not write any
  new file under `validation/witness/**` and do not touch the historical
  committed tp_runner_015 witnesses.

### 4.4 Update deliverable state and close out

On success only: apply the §3.8 writes, then run the §6 plan in order. On
failure or block: leave both deliverables' `_STATUS.md`/`MEMORY.md` unchanged
(or record the truthful partial state if a failure follows their edit),
record truthful failed/blocked evidence in the tranche instance directory,
write `EXECUTE_RETURN.md`, and return to the W1 manager. The executor does
not commit; the W1 manager commits after independent implementation
verification.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to
(paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed standing-approval /
   classification / activation status record or a later superseding
   hold/rejection record;
2. `core/runner/headless/Cargo.toml`, `core/runner/headless/Cargo.lock`,
   `core/runner/headless/src/lib.rs`, and new bounded binding module file(s)
   under `core/runner/headless/src/` (the bin
   `core/runner/headless/src/bin/openpipestress-runner.rs` only if a purely
   internal pass-through edit is unavoidable, with §3.7 byte-stability still
   holding);
3. `core/product_physics/src/lib.rs` (additive-only per §4) and
   `core/product_physics/Cargo.toml`/`Cargo.lock` only if a dependency or
   version metadata line is strictly required;
4. `core/solver/nonlinear_integration/src/lib.rs` additive public
   crate-version/context accessor only; no solve-behavior, assumptions-text,
   or limitations-text change;
5. `schemas/headless_runner.schema.yaml` and
   `tests/test_headless_runner_contract.py`, only under the §4.2 condition;
6. DEL-04-02 deliverable folder
   (`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/`):
   `_STATUS.md`, `MEMORY.md`, and one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`;
7. DEL-04-04 deliverable folder
   (`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/`):
   `_STATUS.md`, `MEMORY.md`, and one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_R14_W1_T1_PRODUCER_BINDING.md`;
8. the tranche instance directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/**`;
9. no evidence-sweep artifact in this tranche: the single wave-level DEC-025
   sweep runs once at W1 closeout per the controlling HELP_HUMAN W1 dispatch
   (§6) and is the pre-push gate for the whole code-touching branch.

Ephemeral writes are limited to task-local Cargo target/build directories and
scratch logs outside durable project paths.

No other project file is writable. In particular, do not write or fix:

- `core/reporting/result_export/**` (DEL-08-04 is consumed read-only through
  its public API), any other `core/reporting/**` path, `core/analysis_runs/**`,
  `core/product_preview/**`, or any other crate;
- any `validation/**` path (no witness, benchmark, hand-calc, sweep, or
  reproduction write);
- `schemas/results.schema.yaml` or any schema other than
  `schemas/headless_runner.schema.yaml`;
- any `docs/**` path (a validation-manual or claims-registry consequence, if
  discovered, is reported and returned, not fixed here);
- any other deliverable or package folder, dependency register, DAG,
  decomposition, decision register/packet, coordination stage record,
  PRD/PLAN, or loop workplan; `loop/LOOP_RECEIPTS.md` is not written by this
  tranche (receipts are HELP_HUMAN's fan-in act for the wave);
- root governance, `_DomainEngines/**`, app-dev, PEC, or any external path.

Local branch/commit metadata is ordinary closeout mechanics after the
adoption chain becomes effective; the tranche commit itself is performed by
the W1 manager after implementation verification. This brief does not
authorize a push, pull, fetch, PR creation, self-merge, or any other
network/external state change.

## 6. Evidence and Validation Plan

Tranche-level checks, run by the executor from `WORKING_ROOT` unless noted,
in sequence, every failure stopping subsequent state-changing closeout; all
cargo invocations offline (`CARGO_NET_OFFLINE=true`, `--offline`); no check
may install dependencies, use the network, or update a toolchain (a missing
local prerequisite is a truthful blocked result):

- `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check`;
- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check`;
- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
  (skip if untouched);
- `cargo test --offline --manifest-path core/runner/headless/Cargo.toml`;
- `cargo test --offline --manifest-path core/product_physics/Cargo.toml`;
- `cargo test --offline --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  (skip if untouched);
- `cargo test --offline --manifest-path core/reporting/result_export/Cargo.toml`
  (read-only regression: the consumed crate must still pass unchanged);
- `python3 tests/test_headless_runner_contract.py`;
- the §4.3 CLI-stability diffs (five del1005 cases against committed
  witnesses; three tp_runner_015 cases against the pre-tranche baseline
  captures; all byte-identical);
- `python3 tools/validation/validate_claims_language.py --repo-root .` from
  `REPO_ROOT`;
- `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`;
- JSON parsing for every new/changed `.json` file;
- changed-path containment against the §5 fence:
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`, persisting
  its JSON stdout to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CHANGE_SCOPE_CONTAINMENT.json`.

The branch-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) run once at W1 wave closeout, before
any push or fan-in, per the controlling HELP_HUMAN dispatch to the W1
manager ("Run the registered checks once for the branch ... The single
evidence-sweep must yield exactly one new
`validation/evidence/sweeps/SWEEP_*.json` ... commit it as the pre-push
DEC-025 gate"). The R14 `ORCHESTRATION_PLAN.md` execution-rules sentence
("per-tranche commits, each gated by the tranche's full check set including
the DEC-025 evidence sweep") is refined by that later parent dispatch: the
sweep remains the DEC-025 pre-push gate for the whole code-touching branch
and runs exactly once before fan-in, so no tranche commit is ever pushed
un-swept. This plan-text refinement is recorded here and is surfaced in the
W1 manager return for HELP_HUMAN fan-in visibility.

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate that does not hold, any §6 check failure,
  any committed-witness byte drift, any unexpected changed path, or any
  discovery that the mapping cannot be built without inventing semantics is
  a failure that stops closeout; record truthful evidence in
  `instances/W1/T1/` and return to the W1 manager.
- No scope drift: no CLI output-shape change, no `export-results` binding, no
  DEL-08-04 crate edit, no results-schema edit, no threshold/tolerance
  creation or promotion (DEC-046 and PDU-035 holds untouched), no
  persisted-project or report-reader change, no lifecycle/stage/release/
  acceptance act.
- A repair need outside the §5 fence — including any defect discovered in
  the DEL-08-04 crate, the results schema, the report reader, or the Python
  preview builder — is reported and returned for a new lawful selection, not
  fixed here.
- If the checkout is not clean before execution (beyond this run's lawful
  pre-existing state), stop and return the condition; do not stash, reset,
  or interpret unrelated work.

## 8. Rerun Triggers

A rerun (new execution record, same governed brief unless superseded) is
required when any of these changes after the implementation base commit:

- the producer-path sources (`core/runner/headless`, `core/product_physics`,
  `core/solver/nonlinear_integration`, `core/reporting/result_export`), the
  DEL-08-04 schema surface, or `DEC-065` policy;
- either deliverable's Remaining scope or lifecycle state;
- applicable active DAG-007 rows or the approved DAG pointer;
- `software-workflow.json` or the root software workflow profile;
- a prior failed/blocked result after the underlying condition is resolved.

A material governance change (new decision touching the producer surface,
supersession of the D-52/D-54 lanes) returns the brief itself to HELP_HUMAN
before any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- DEL-04-04 Remaining items 2–6: the PDU-035 formal REVIEW disposition and
  dimensional/conversion basis, the friction path-history D-XX ruling, and
  every threshold-promotion row (DEC-046/DEC-052/DEC-054 lineage) — all
  owner-gated or stage-gated and untouched;
- promotion of release thresholds, final tolerance policy, CI gate policy,
  or any DEC-046 tolerance record; new tolerance constants, acceptance
  criteria, or normative content of any kind;
- the DEL-10-05 `export-results` binding or any CLI output-shape change;
- edits to the DEL-08-04 crate, `schemas/results.schema.yaml`, suite fixture
  values, recorded expected values, policy JSON records, READMEs, hand-calcs,
  or any frozen witness surface;
- G1/G2/G4 + M2/M3 re-disposition, DEC-046 promotions, D-45, D-05b or
  stage-gated rows, or any PDU-035 act;
- reproduction acceptance, evidence-posture promotion, prover
  activation/correlation, or `PROVER_CORRELATED`/`ENGINEER_ACCEPTED` status;
- lifecycle transition, stage or milestone advancement, issuance, release,
  packaging, signing, publication, push, PR creation/merge, hosted CI,
  network or external service use, or any external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (W1 manager, R14 campaign)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W1 / T1
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: library-level DEL-08-04 envelope producer bound into run_preview_in_memory* per §3–§4 within the §5 fence, CLI stdout unchanged
JudgedBy: WORKING_ITEMS (Agent 1), HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14 / W1 / T1
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (CLI-exposing binding; result_export-crate producer; Python-side binding; MechanicsEnvelope shape change)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/W1/T1/VERIFY_BRIEF_V3.md` (18/18 claims confirmed; independent 10-class re-screen pass; empirical re-run of the pinned fixture and all five del1005 witness comparisons); history: v1 BLOCK at `instances/W1/T1/VERIFY_BRIEF.md` (cured by v2 amendment), v2 COMMIT-SAFE at `instances/W1/T1/VERIFY_BRIEF_V2.md` (superseded by the v3 material amendment after the executor BLOCK at `instances/W1/T1/EXECUTE_RETURN.md`)
EffectStatus: EFFECTIVE (v3) — EXECUTION RELEASED BY W1 MANAGER UNDER THE R14 CAMPAIGN CHAIN (V3 COMMIT-SAFE)
PreservedGates: DEL-04-04 items 2-6 (PDU-035 REVIEW; friction D-XX; threshold promotions); DEC-046 tolerance promotion; export-results binding (DEL-10-05); lifecycle/stage/issuance/release/acceptance; reproduction acceptance and evidence-posture promotion; prover activation/correlation; publication/external action; merge authority; D-45; D-05b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the agent
classifies, selects among defensible binding shapes under D-54, and proposes.
The W1 manager progresses `EffectStatus` only after the independent
refutation returns `COMMIT-SAFE`, under the R14 campaign plan's execution
rules and the durably landed D-52/D-54 records. No execution is released by
this document in its current state.
