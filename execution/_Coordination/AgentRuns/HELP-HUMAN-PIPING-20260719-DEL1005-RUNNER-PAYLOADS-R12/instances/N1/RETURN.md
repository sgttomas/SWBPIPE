# N1 TERMINAL RETURN — Candidate Brief + Rationale Authoring

**Run:** `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12`, node N1
**Role:** ORCHESTRATOR (Agent 1) preparing coordination artifacts for
HELP_HUMAN; no further delegation performed
**Date:** 2026-07-19
**Live-tree basis:** HEAD `96563e8e09b89908e13e6b2f1f1139aca3283855`, branch
`claude/chirality-piping-loop-init-a45657`, clean at intake except this run's
own pre-existing `AgentRuns/.../R12/` directory (untracked)

## 1. What Was Read

Paths relative to `projects/chirality-piping/` unless noted; all read from the
live tree, not summaries.

- DEL-10-05 folder (`execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/`):
  `_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `ScopeOfWork.md`,
  `Dependencies.csv`, `_run_records/WORKING_ITEMS_RUN_2026-07-05_TP-RUNNER-015.md`,
  `_run_records/WORKING_ITEMS_RUN_2026-07-04_TP-RUNNER-014.md` (via MEMORY
  digest plus direct read of the 015 record).
- Runner: `core/runner/headless/src/bin/openpipestress-runner.rs` (full),
  `core/runner/headless/src/lib.rs` (targeted reads),
  `core/runner/headless/Cargo.toml`, `schemas/headless_runner.schema.yaml`
  (targeted greps), `tests/test_headless_runner_contract.py` (targeted greps).
- Suites: `validation/benchmarks/{mechanics,stress,nonlinear}/` READMEs,
  `Cargo.toml` files, `src/lib.rs` inventory/struct surfaces, DEC-046 policy
  JSON listing.
- E1: `docs/validation_manual/headless_runner_reproduction.md` (full),
  `validation/witness/inputs/generate_tp_runner_015_inputs.py` (head), the
  three `tp_runner_015_final_cli_*_input.json` fixture shapes.
- Governance: D-52 packet (full), D-54 packet (full), `_REGISTER.md` rows
  D-33/D-49/D-52/D-54, `SOFTWARE_DECOMP.md` §12 rows DEC-025/DEC-046/DEC-064/
  DEC-065/DEC-080/DEC-081/DEC-087, prior brief
  `CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md` (full).
- Graph/loop: `execution/_DAG/_LATEST.md`, DAG-007 `DependencyEdges.csv`
  DEL-10-05 rows and `DeliverableNodes.csv` row, `loop/` listing,
  `loop/LOOP_RECEIPTS.md` tail (Receipts 55–58).
- DEL-09-04 `_STATUS.md` (Remaining + history head).
- Workflow: `software-workflow.json` (parsed), root
  `docs/SOFTWARE_WORKFLOW_PROFILE.md` (existence),
  `tools/software_workflow/run_registered_checks.py --help` (live CLI shape),
  `tools/software_workflow/validate_change_scope.py` and
  `tools/validation/validate_{claims_language,path_anchors,piping_loop_receipts}.py`
  (existence).
- This run's `ORCHESTRATION_PLAN.md` and `WORK_GRAPH.json`.

## 2. What Was Written (complete list)

1. `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`
   (doc_id `CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001`).
2. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/CURRENT_CANDIDATE_RATIONALE.md`.
3. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N1/RETURN.md`
   (this file).

Nothing else was written. No code, schema, test, fixture, deliverable-local,
receipt, register, or workplan file was modified. No commands other than
read/list/parse operations and the one `--help` probe were run; no cargo
command was executed; no commit was made.

## 3. Classification Result

**`ELIGIBLE_PENDING_VERIFICATION`** — the D-52 §4.1 fast-reject screen passed
on all 10 classes (item-by-item verdicts in `CURRENT_CANDIDATE_RATIONALE.md`
§2); no D-49/D-52 limit hit and therefore no near-miss record; multiple
defensible implementation shapes were resolved by D-54/`DEC-087` reasoned
selection with rejected alternatives recorded. `EffectStatus: HELD`;
`RuleActivation: NOT_ACTIVATED`; adoption/progression is for HELP_HUMAN after
N2 independent refutation.

## 4. Noted Deltas / Consequences (none papered over)

- **E1 case-3 staleness consequence (flagged, not CONTESTED):** the E1
  procedure `docs/validation_manual/headless_runner_reproduction.md` (case 3)
  and the committed generated witness
  `validation/witness/generated/tp_runner_015_final_cli_benchmark_stub.json`
  record `run-benchmark` exiting 1 with the stub diagnostic. Once the binding
  lands, that expectation becomes historical (the fixture carries no benchmark
  payload and would receive a payload-missing diagnostic instead). The manual
  page is DEL-09-04-owned and OUTSIDE this tranche's fence; the brief records
  this as a return-to-HELP_HUMAN follow-on (§7 of the brief). The completed
  immutable DEL-09-04 reproduction bundle remains valid evidence at its pinned
  source commit.
- **Tasking wording vs live column detail:** the tasking said to state
  "DAG-007 execution-upstream rows ... — state whether satisfied." The live
  DAG-007 rows are NOT all `SATISFIED`: six active rows are `TBD` (see claim
  5). The recorded 2026-06-07 REVIEW ruling accepted those `TBD` rows as
  deferred for the bounded runner-contract boundary; the brief states this
  precisely rather than claiming blanket satisfaction.
- No other contradiction between the tasking and the live tree was found.

## 5. Enumerated Refutable Claims (for the fresh-context N2 verifier)

Each claim is one factual assertion the brief depends on, with its exact
live-tree source path (relative to `projects/chirality-piping/` unless noted).

1. **Remaining-item text (selected scope).** DEL-10-05 `_STATUS.md ## Remaining`
   contains the bullet "Bind benchmark/regression runner payloads (E2
   per-case reproduction currently runs through suite tests) (source:
   TP-E2-VALMANUAL-001 residuals)" as a distinct item from the
   `export-results` bullet. Source:
   `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_STATUS.md` (Remaining, lines 7–8).
2. **DEL-10-05 lifecycle.** DEL-10-05 `_STATUS.md` records
   `Current State: IN_PROGRESS`. Same source, line 3.
3. **Stub location/behavior.** In
   `core/runner/headless/src/bin/openpipestress-runner.rs`, the single match
   arm `RunnerOperation::ExportResults | RunnerOperation::RunBenchmark |
   RunnerOperation::RunRegression` pushes blocking diagnostic
   `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` and returns
   exit code 1 (lines ~237–257); the test
   `downstream_operation_verbs_are_stable_but_stubbed` asserts this for
   `run-benchmark`.
4. **Verb mapping is stable.** `operation_for_verb` maps `solve`,
   `validate-input`, `export-results`, `run-benchmark`, `run-regression` to
   the `RunnerOperation` enum (also defined in
   `core/runner/headless/src/lib.rs`, lines 53–61). Source: same bin file,
   `operation_for_verb`, and `lib.rs`.
5. **DAG-007 execution-upstream posture for DEL-10-05.** In
   `execution/_DAG/DAG-007/DependencyEdges.csv`, the ACTIVE
   EXECUTION/UPSTREAM rows for FromDeliverable DEL-10-05 are:
   `DEP-10-05-E001` (DEL-00-03) SATISFIED; `DEP-10-05-E002` (DEL-00-06)
   SATISFIED; `DEP-10-05-E003` (DEL-08-04) TBD; `DEP-10-05-E004` (DEL-10-04)
   TBD; `DEP-10-05-E005` (DEL-02-02) TBD; `DEP-10-05-E006` (DEL-02-05) TBD;
   `DEP-10-05-E007` (DEL-08-02) TBD; `DEP-10-05-E008` (DEL-04-06) TBD;
   `DEV-001-STAGE2-DEL-10-05-PKG02-001` (DEL-02-01) SATISFIED;
   `DEV-001-STAGE2-DEL-10-05-PKG02-003` (DEL-02-03) SATISFIED (approx. lines
   869–887). `_LATEST.md` resolves DAG-007 as
   `approved_active_graph_authority` (`execution/_DAG/_LATEST.md`).
6. **TBD rows accepted as deferred.** DEL-10-05 `_STATUS.md` history entry
   dated 2026-06-07 records that human-approved REVIEW rulings "accepted
   active dependency `TBD` rows as deferred for the current bounded
   runner-contract boundary." Source: `_STATUS.md` (same file as claim 1),
   history entry 2026-06-07.
7. **Mechanics suite existence/shape/posture.**
   `validation/benchmarks/mechanics/` is crate
   `open_pipe_stress_mechanics_benchmarks` with `pub fn fixture_inventory()`
   (src/lib.rs ~line 546) and `fixture_inventory_ids()` (~572);
   `MechanicsBenchmark` has `fixture_id` and recorded `expected_values`
   (~239–247); its README states comparison values are "regression evidence"
   and that release thresholds/tolerance policy/CI gate policy/professional
   reliance remain `TBD` pending human approval. Sources:
   `validation/benchmarks/mechanics/{Cargo.toml,src/lib.rs,README.md}`.
8. **Stress suite existence/shape/posture.** `validation/benchmarks/stress/`
   is crate `open_pipe_stress_stress_benchmarks` with `fixture_inventory()`
   (~line 437) and `StressBenchmark.fixture_id`; README carries the same
   regression-evidence / `TBD`-thresholds posture. Sources:
   `validation/benchmarks/stress/{Cargo.toml,src/lib.rs,README.md}`.
9. **Nonlinear suite existence/shape/posture.**
   `validation/benchmarks/nonlinear/` is crate
   `open_pipe_stress_nonlinear_benchmarks` with `fixture_inventory()` (~line
   1118), assembled inventories/observations, and
   `NonlinearRegressionCase.fixture_id` plus recorded expected
   states/residuals (~322–332); governed DEC-046 policy records exist as
   `*.dec046.json` files in the crate directory; README states the fixtures
   are software verification aids only. Sources:
   `validation/benchmarks/nonlinear/{Cargo.toml,src/lib.rs,README.md}` and the
   `*.dec046.json` listing.
10. **DEC-065 policy constraints.** `SOFTWARE_DECOMP.md` §12 row `DEC-065`
    settles binary `openpipestress-runner`; the five verbs; schema-first JSON
    stdin/`--input`; structured JSON stdout with optional explicit
    `--output`; single foreground local process; no daemon/network/telemetry/
    hidden filesystem mutation/private-data write/user-home scanning/direct
    SQL; and exit policy `0` completed-no-blocking, `1` blocking/validation
    failure, `2` usage/malformed/unsupported. Source:
    `execution/_Decomposition/SOFTWARE_DECOMP.md`, row `DEC-065` (~line 647).
11. **Check registry facts.** `software-workflow.json` defines checks
    `desktop-build`, `desktop-test`, `evidence-sweep`
    (`python3 tools/release/run_evidence_sweep.py --execute`), `piping-pytest`
    (`python3 -m pytest -q tests`), `harness-pytest` (cwd `../..`),
    `harness-self-check` (cwd `../..`); `always_checks` =
    `["harness-self-check"]`; path rules map `core/**`, `validation/**`,
    `tests/**` → `piping-pytest`+`evidence-sweep`, and `execution/**`,
    `docs/**`, `AGENTS.md`, `loop/**` → `harness-pytest`. The live runner CLI
    is `run_registered_checks.py <profile> [--check CHECK] --output OUTPUT
    [--timeout-seconds N]`. Sources: `software-workflow.json`; root
    `tools/software_workflow/run_registered_checks.py --help`; root
    `docs/SOFTWARE_WORKFLOW_PROFILE.md` exists.
12. **E1 procedure and frozen fixtures.**
    `docs/validation_manual/headless_runner_reproduction.md` documents the
    three-case E1 procedure; case 3 expects `run-benchmark` exit 1 with the
    stub diagnostic; the three inputs
    `validation/witness/inputs/tp_runner_015_final_cli_{solve,validation_blocking,benchmark_stub}_input.json`
    and generator `generate_tp_runner_015_inputs.py` exist; the benchmark stub
    input contains only a `request` object (operation `run_benchmark`), no
    payload. Its "Remaining E2 Work" section states `run-benchmark` /
    `run-regression` "remain structured stubs."
13. **DEL-09-04 E2 residual (the dependency this tranche discharges the
    DEL-10-05 side of).** DEL-09-04 `_STATUS.md ## Remaining` first bullet
    includes "runner benchmark/regression payload bindings still structured
    stubs (per-case reproduction runs through suite tests; see also
    DEL-10-05)"; the second bullet gates final public-benchmark release
    tolerances on the owner under the DEC-046 convention. Source:
    `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`.
14. **DEL-08-04 envelope references in runner code.**
    `core/runner/headless/src/lib.rs` references
    `schemas/results.schema.yaml` as the result-envelope `schema_ref`
    (~lines 350–357) and requires the DEL-08-04 wrapper identity for
    full-envelope payload validation (~line 588). This tranche does not alter
    that surface.
15. **Runner crate has no suite dependencies today.**
    `core/runner/headless/Cargo.toml` lists only
    `open_pipe_stress_canonical_json`, `open_pipe_stress_product_physics`,
    `serde`, `serde_json`, `sha2`; binding therefore requires new path
    dependencies (each suite crate has its own `Cargo.lock`, so lockfile
    updates are expected in-fence).
16. **Governance lane availability.** D-54/`DEC-087` (reasoned selection,
    superseding the exactly-one-outcome condition prospectively) is recorded
    in `execution/_Coordination/_DECISIONS/D-54_...md` and `SOFTWARE_DECOMP.md`
    §12; loop `Receipt-56` records "D-54/DEC-087 landing state reconciled" and
    the active workplan is `loop/WORKPLAN_2026-07-18b_piping_loop.md`;
    `Receipt-58` records the DEL-09-04 clean reproduction completed with
    DEL-09-04 remaining `IN_PROGRESS`. Source: `loop/LOOP_RECEIPTS.md` tail.
17. **CLI wrapper precedent.** The settled TP-RUNNER-015 CLI input wrapper is
    `request` + optional `solve.preview_model` + optional
    `rule_check_aggregate` (verb-named payload precedent), per DEL-10-05
    `ScopeOfWork.md` CLM-004 and the `CliInput` struct in the bin file; and
    TP-RUNNER-015 records the downstream verbs "return structured blocking
    diagnostics until downstream payload bindings exist" (DEL-10-05
    `MEMORY.md`, 2026-07-05 entry).
18. **Write-fence completeness claim.** Brief §5 enumerates every durable
    write the execution tranche may make (brief status block; runner crate
    files incl. Cargo.toml/Cargo.lock and new module(s); runner schema +
    contract test; bounded additive suite accessors; NEW-only witness
    input/generated files; DEL-10-05 `_STATUS.md`/`MEMORY.md`/one run record;
    DEL-09-04 `_STATUS.md`/`MEMORY.md` only under explicit HELP_HUMAN release,
    default untouched; exactly one sweep JSON; the R12 AgentRuns directory;
    `loop/LOOP_RECEIPTS.md` append-only) and §6 requires deterministic
    containment via `tools/software_workflow/validate_change_scope.py`; no
    task in §4 requires a write outside §5.
19. **No-new-normative-content claim.** Every acceptance predicate in brief §3
    derives from an accepted source: exit codes from `DEC-065`; per-case
    comparison bases from the suite crates' recorded values and the
    regression-evidence posture in their READMEs; scope from the DEL-10-05
    Remaining bullet and DEL-09-04 E2 residual; diagnostics naming from the
    existing `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING` pattern. The brief
    creates no tolerance, threshold, release criterion, or acceptance
    criterion, and requires fail-closed handling where a recorded basis cannot
    be reused.
20. **Stub retention claim.** After the tranche, the diagnostic
    `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` is required
    to remain for `export-results` only (brief §3.6), consistent with the
    Remaining `export-results` bullet staying open (claim 1 source).
21. **Repo state at authoring.** `git status` at intake showed only the
    untracked pre-existing R12 AgentRuns directory; HEAD was
    `96563e8e09b89908e13e6b2f1f1139aca3283855`. Source: live `git status`
    / `git rev-parse HEAD` at N1 execution.

## 6. Handoff State

- Accepted upstream basis: live tree at HEAD `96563e8e0` (claim 21); approved
  DAG-007; active workplan `WORKPLAN_2026-07-18b`.
- Derivative-package status: the brief and rationale are coordination
  artifacts derived from the cited live sources; they are not decomposition
  truth and carry no lifecycle effect.
- Closure verdict: N1 objective complete (brief + rationale + return
  authored); classification `ELIGIBLE_PENDING_VERIFICATION`; effect `HELD`.
- Next step (parent-owned): dispatch N2 fresh-context refutation against §5;
  on `COMMIT-SAFE`, HELP_HUMAN progresses the adoption chain and releases N3
  per the brief.
- Remaining blockers: none found for N2 dispatch. Known follow-on outside
  this tranche: E1 case-3 manual staleness and DEL-09-04 Remaining-wording
  update (§4 above).

No release-readiness, reproduction-acceptance, lifecycle, stage, or
compliance claim is made. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081).
