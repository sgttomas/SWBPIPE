---
doc_id: CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-19
package_id: PKG-10
deliverable_id: DEL-10-05
decision_basis: DEC-065, DEC-025, DEC-046 (preserved gate), DEC-081, DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — DEL-10-05 Runner Benchmark/Regression Payload Bindings

**Status:** `EFFECTIVE — EXECUTION RELEASED BY HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)`

**Prepared by:** ORCHESTRATOR (N1) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12`

**Selected work item:** the DEL-10-05 `_STATUS.md ## Remaining` item
"Bind benchmark/regression runner payloads (E2 per-case reproduction currently
runs through suite tests) (source: TP-E2-VALMANUAL-001 residuals)" only. The
`export-results` binding (coupled to the DEL-08-01 report container) is OUT of
scope for this tranche.

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on the
D-52/`DEC-085` standing-approval overlay. Adoption remains the human owner's
conditional act under the standing rule; this document classifies and proposes
only. The adoption effect is `HELD` until independent refutation (N2) returns
`COMMIT-SAFE` and HELP_HUMAN progresses the chain. No execution writes are
authorized by this document in its current state.

## 1. Purpose and Accepted Basis

Purpose: bind the `run-benchmark` and `run-regression` downstream payloads in
the `openpipestress-runner` CLI so that per-case benchmark/regression
reproduction can execute through the runner instead of only through the
benchmark crates' suite tests, reusing the existing suites' fixture
inventories and recorded comparison values as regression evidence, with no
threshold or tolerance promotion and no lifecycle act.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(HEAD `96563e8e09b89908e13e6b2f1f1139aca3283855`):

- root `AGENTS.md` and project `AGENTS.md`;
- active committed-HEAD workplan `loop/WORKPLAN_2026-07-18b_piping_loop.md`
  and structurally valid `loop/LOOP_RECEIPTS.md` through cursor `Receipt-58`;
- `execution/_DAG/_LATEST.md` resolving to approved `DAG-007`
  (`approved_active_graph_authority`) and the DEL-10-05 rows in
  `execution/_DAG/DAG-007/DependencyEdges.csv`;
- DEL-10-05 `_STATUS.md` (IN_PROGRESS; the named Remaining item open),
  `MEMORY.md` (TP-RUNNER-015/TP-RUNNER-014 entries), `_CONTEXT.md`,
  `ScopeOfWork.md` (CLM-004 settled CLI conditions), `Dependencies.csv`, and
  run records `_run_records/WORKING_ITEMS_RUN_2026-07-05_TP-RUNNER-015.md`
  and `_run_records/WORKING_ITEMS_RUN_2026-07-04_TP-RUNNER-014.md`;
- runner surfaces: `core/runner/headless/src/bin/openpipestress-runner.rs`
  (stub arm at the
  `RunnerOperation::ExportResults | RunBenchmark | RunRegression` match arm,
  diagnostic `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`,
  exit 1), `core/runner/headless/src/lib.rs`, `core/runner/headless/Cargo.toml`,
  `schemas/headless_runner.schema.yaml`, and
  `tests/test_headless_runner_contract.py`;
- benchmark/regression suites:
  `validation/benchmarks/mechanics/` (DEL-09-01; `fixture_inventory()` at
  `src/lib.rs`; `MechanicsBenchmark.fixture_id`; recorded `expected_values`),
  `validation/benchmarks/stress/` (DEL-09-02; `fixture_inventory()`;
  `StressBenchmark.fixture_id`), and
  `validation/benchmarks/nonlinear/` (DEL-09-03; `fixture_inventory()`,
  assembled inventories and governed DEC-046 convergence-policy records;
  `NonlinearRegressionCase.fixture_id`), with their README claim posture:
  numerical comparison values are regression evidence for current solver
  behavior; release thresholds, final tolerance policy, CI gate policy, and
  professional reliance remain `TBD` pending human approval;
- E1 procedure and frozen fixtures:
  `docs/validation_manual/headless_runner_reproduction.md`,
  `validation/witness/inputs/generate_tp_runner_015_inputs.py`, and the three
  committed `validation/witness/inputs/tp_runner_015_final_cli_*_input.json`
  fixtures;
- DEL-09-04 `_STATUS.md ## Remaining` (the E2 residual naming the runner
  benchmark/regression payload stubs, cross-referencing DEL-10-05) and the
  DEL-08-04 result-envelope references in `core/runner/headless/src/lib.rs`
  (`schemas/results.schema.yaml`, `DEL-08-04` wrapper identity);
- governance: `DEC-065` (D-33 O-A final local CLI/process policy, including
  the exit-code policy 0/1/2), `DEC-064`, `DEC-046` (governed convergence
  tolerance records; promotion owner-gated), `DEC-025` (five-surface local
  evidence sweep as the pre-push gate for code-touching branches), `DEC-081`
  (claims taxonomy), D-52/`DEC-085`, D-54/`DEC-087` (landed state reconciled
  at Receipt 56), and the structural model brief
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`;
- `software-workflow.json` under the ratified root
  `docs/SOFTWARE_WORKFLOW_PROFILE.md` contract, and the root tools
  `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts

- DEL-10-05 is `IN_PROGRESS` with two Remaining items; this brief selects only
  the benchmark/regression payload-binding item. The `export-results` item
  stays open and untouched.
- DAG-007 active `EXECUTION / UPSTREAM` rows for DEL-10-05
  (`execution/_DAG/DAG-007/DependencyEdges.csv`):
  - `DEP-10-05-E001` (DEL-00-03, INTERFACE) — `SATISFIED`;
  - `DEP-10-05-E002` (DEL-00-06, INTERFACE) — `SATISFIED`;
  - `DEP-10-05-E003` (DEL-08-04, INTERFACE) — `TBD`;
  - `DEP-10-05-E004` (DEL-10-04, PREREQUISITE) — `TBD`;
  - `DEP-10-05-E005` (DEL-02-02, CONSTRAINT) — `TBD`;
  - `DEP-10-05-E006` (DEL-02-05, INTERFACE) — `TBD`;
  - `DEP-10-05-E007` (DEL-08-02, INTERFACE) — `TBD`;
  - `DEP-10-05-E008` (DEL-04-06, PREREQUISITE) — `TBD`;
  - `DEV-001-STAGE2-DEL-10-05-PKG02-001` (DEL-02-01, INTERFACE) — `SATISFIED`;
  - `DEV-001-STAGE2-DEL-10-05-PKG02-003` (DEL-02-03, PREREQUISITE) —
    `SATISFIED`.
  The six `TBD` rows are the same active rows that the recorded 2026-06-07
  human-approved REVIEW ruling accepted as deferred for the bounded
  runner-contract boundary (DEL-10-05 `_STATUS.md` history, 2026-06-07). This
  tranche stays inside that bounded runner surface and does not resolve,
  promote, or depend on resolving any `TBD` row. No active execution-upstream
  blocker specific to this tranche was found.
- The three benchmark suites exist as standalone crates with their own
  `Cargo.lock` files and expose per-case inventories keyed by stable
  `fixture_id` strings; the headless runner crate currently has no dependency
  on any of them (`core/runner/headless/Cargo.toml`).
- DEL-09-04 completed the actor-neutral clean-checkout reproduction
  (Receipt 58); its remaining E2 residual explicitly names the runner
  benchmark/regression payload stubs and points at DEL-10-05. This tranche
  discharges the DEL-10-05 side of that dependency; DEL-09-04 surfaces are
  not edited by default (see §5 and §7).
- D-54/`DEC-087` reasoned-selection lane is landed and reconciled
  (Receipt 56); the active workplan is `loop/WORKPLAN_2026-07-18b_piping_loop.md`.

## 3. Objective and Acceptance Predicates

Bind `run-benchmark` and `run-regression` in `openpipestress-runner` to the
existing suite inventories so that all of the following hold on the
implementation head:

1. **Benchmark binding.** `openpipestress-runner run-benchmark` with a valid
   schema-first request (operation `run_benchmark`) plus a benchmark payload
   naming a supported benchmark suite (`mechanics` = DEL-09-01,
   `stress` = DEL-09-02) and an explicit `cases` list of `fixture_id` values
   executes exactly those cases, and exits `0` when every requested case
   executes and matches its recorded comparison values.
2. **Regression binding.** `openpipestress-runner run-regression` with a valid
   request (operation `run_regression`) plus a regression payload naming the
   `nonlinear` suite (DEL-09-03) behaves identically in form: requested cases
   execute against the crate's recorded expected states/residual records, and
   exit is `0` only when all requested cases execute and match.
3. **Whole-suite default.** An omitted or empty `cases` list selects the named
   suite's full `fixture_inventory()` case set.
4. **Per-case reporting.** The structured JSON output reports, for every
   requested case: the `fixture_id`, family, the recorded (expected) values,
   the observed values, observed-vs-recorded deltas, and a per-case
   match/fail status expressed ONLY in the existing regression-evidence claim
   posture (regression evidence for current solver behavior; release
   thresholds, final tolerance policy, CI gate policy, and professional
   reliance remain `TBD`, owner-gated). No new numeric tolerance constant and
   no release-threshold vocabulary is introduced; comparison predicates must
   reuse the recorded comparison basis already encoded in the suite crates. A
   case whose recorded basis cannot be reused fails closed with a structured
   blocking diagnostic; the executor must not invent a tolerance for it.
5. **Exit policy (DEC-065).** Exit `0` only when all requested cases execute
   and match recorded regression evidence; exit `1` with structured blocking
   diagnostics for any mismatch, unknown `fixture_id`, unsupported suite
   name, missing payload, or execution failure; exit `2` remains reserved for
   usage, unreadable input, malformed JSON, or unsupported operation. No
   silent partial skip: every requested case appears in the output as
   executed-and-matched, executed-and-mismatched, or blocked.
6. **Missing payload diagnostics.** `run-benchmark`/`run-regression` without
   their payload object exit `1` with new payload-missing blocking
   diagnostics following the existing
   `HEADLESS_RUNNER_SOLVE_PAYLOAD_MISSING` naming pattern. The stub diagnostic
   `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` is emitted for
   `export-results` ONLY after this tranche.
7. **DEC-065 policy preservation.** Output remains schema-first structured
   JSON on stdout (optional explicit `--output`), single foreground local
   process, no network, daemon, telemetry, hidden filesystem mutation,
   repository-default private-data write, user-home scanning, or direct
   SQL/SQLite bypass; the `policy` block continues to report `DEC-065`.
8. **No collateral behavior change.** `solve` and `validate-input` behavior,
   the frozen E1 input fixtures, the TP-RUNNER-015 generator, and the three
   committed `validation/witness/generated/tp_runner_015_final_cli_*.json`
   witnesses are unchanged byte-for-byte.
9. **Evidence.** New witness evidence (new files only) demonstrates at least:
   one passing per-case benchmark run, one passing whole-suite or multi-case
   run, one passing regression run, and one payload-missing blocking case per
   bound verb; and the full validation plan in §6 passes.

A successful run may close only the exact DEL-10-05 Remaining
benchmark/regression payload-binding bullet. It does not close the
`export-results` bullet, any DEL-09-04 Remaining item, or Phase E row E2.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several implementation shapes were defensible, the selection below was
made under D-54/`DEC-087` reasoned selection; the four-lens analysis and the
materially rejected alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape:

- **Payload field naming** follows the existing verb-named `solve` wrapper
  precedent in the settled TP-RUNNER-015 CLI input wrapper: optional
  `benchmark` and `regression` objects beside `request`.
- **Case addressing** is `suite` (string selector) plus optional `cases`
  (list of suite-local `fixture_id` strings); omitted/empty means the full
  inventory. `run-benchmark` accepts the benchmark suites
  (`mechanics`, `stress`); `run-regression` accepts the regression suite
  (`nonlinear`). Verb-to-suite mapping mirrors the accepted deliverable
  ontology (DEL-09-01/DEL-09-02 benchmark suites; DEL-09-03 nonlinear
  regression suite).
- **Module layout**: one new bounded binding module inside the existing
  headless crate (for example `core/runner/headless/src/benchmark_binding.rs`)
  reached from the lib/bin split, with new path dependencies from
  `core/runner/headless/Cargo.toml` on the three suite crates. The suite
  crates remain the single source of fixture identity and recorded values.

Bounded tasks for the later executor (N3):

### 4.1 Freeze the execution basis

- Begin on a clean local working branch from the integration checkout; record
  the base commit before any durable write.
- Record the adopted brief commit or content hash in the managed execution
  run record under the R12 AgentRuns directory.
- Stop if the active DAG pointer, DEL-10-05 lifecycle/Remaining item, DEC-065,
  the suite crates' claim posture, or `software-workflow.json` has changed
  materially since this brief. Do not silently reinterpret scope.

### 4.2 Implement the bindings

- Extend the CLI input wrapper with the optional `benchmark` and `regression`
  payload objects and implement per-case execution/comparison through the new
  binding module, reusing each suite's `fixture_inventory()` (and, for
  `nonlinear`, its recorded expected-state/residual and governed DEC-046
  policy records as already encoded) as the only source of recorded values.
- Add the payload-missing, unknown-case, and unsupported-suite blocking
  diagnostics; restrict the stub diagnostic to `export-results`.
- Update `core/runner/headless` unit tests (including the current
  `downstream_operation_verbs_are_stable_but_stubbed` test, which must be
  narrowed to `export-results`) and add per-verb binding tests.
- Update `schemas/headless_runner.schema.yaml` and
  `tests/test_headless_runner_contract.py` only as required to keep the
  schema/contract surface true to the bound behavior; do not weaken existing
  checks.
- If minimal additive accessor changes inside a suite crate are strictly
  required to reuse its recorded comparison basis, they must be additive and
  non-breaking, must not change any recorded fixture value, expected value,
  policy record, or README claim posture, and must keep that crate's tests
  passing unchanged in meaning.

### 4.3 Produce witness evidence

- Add NEW witness input fixtures (and, if scripted, one NEW generator file)
  under `validation/witness/inputs/`, and NEW generated witness outputs under
  `validation/witness/generated/`, covering the §3.9 cases. Do not edit the
  frozen E1 three inputs, their generator, or the three committed
  TP-RUNNER-015 generated witnesses.

### 4.4 Update local state and close out

On success only:

- remove only the exact benchmark/regression payload-binding bullet from
  DEL-10-05 `_STATUS.md ## Remaining`, preserving the `export-results` bullet
  and lifecycle `IN_PROGRESS`;
- append one `_STATUS.md ## History` entry and one `MEMORY.md` entry citing
  this brief, the evidence paths, and preserved boundaries;
- write one new DEL-10-05 `_run_records/WORKING_ITEMS_RUN_*.md`;
- leave DEL-09-04 surfaces untouched by default (§7 governs the only
  exception);
- append one versioned minimal loop receipt after all checks pass, then rerun
  the receipt validator.

On failure or block: keep the Remaining bullet and lifecycle unchanged, record
truthful failed/blocked evidence in the run record, and return to HELP_HUMAN.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to
(paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed standing-approval /
   classification / activation status record or a later superseding
   hold/rejection record;
2. runner crate files: `core/runner/headless/Cargo.toml`,
   `core/runner/headless/Cargo.lock`, `core/runner/headless/src/lib.rs`,
   `core/runner/headless/src/bin/openpipestress-runner.rs`, and new bounded
   binding module file(s) under `core/runner/headless/src/`;
3. `schemas/headless_runner.schema.yaml` and
   `tests/test_headless_runner_contract.py`, only as required by §4.2;
4. minimal additive non-breaking accessor changes (plus resulting
   `Cargo.lock` updates) inside
   `validation/benchmarks/mechanics/`, `validation/benchmarks/stress/`,
   and/or `validation/benchmarks/nonlinear/`, only under the §4.2 conditions;
   no recorded value, fixture, policy JSON, or README edits;
5. NEW files only under `validation/witness/inputs/` and
   `validation/witness/generated/`; the three frozen
   `tp_runner_015_final_cli_*_input.json` fixtures,
   `generate_tp_runner_015_inputs.py`, and the three committed
   `tp_runner_015_final_cli_*.json` generated witnesses are read-only;
6. DEL-10-05 only:
   `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/`
   `_STATUS.md`, `MEMORY.md`, and one new
   `_run_records/WORKING_ITEMS_RUN_*.md`;
7. DEL-09-04 `_STATUS.md` and `MEMORY.md` ONLY if the tranche fully lands and
   HELP_HUMAN explicitly releases the cross-reference update at fan-in;
   default is DO NOT touch DEL-09-04 in this tranche — its Remaining wording
   stays as-is and any update is follow-on work;
8. exactly one new tool-emitted
   `validation/evidence/sweeps/SWEEP_*.json` from the single mandatory
   evidence-sweep run, verified as a one-file before/after delta;
9. the parent managed-run directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/**`;
10. `loop/LOOP_RECEIPTS.md`, append-only, for one new versioned minimal
    receipt.

Ephemeral writes are limited to task-local Cargo target/build directories and
scratch logs outside durable project paths.

No other project file is writable. In particular, do not write or fix:

- `docs/validation_manual/**` (including
  `headless_runner_reproduction.md` — see §7 for the recorded staleness
  consequence), `docs/**`, `plans/**`, schemas other than
  `schemas/headless_runner.schema.yaml`, or any tool;
- suite fixture values, recorded expected values, DEC-046 policy JSON
  records, hand-calc notes, or suite READMEs;
- any other deliverable or package, including DEL-08-01, DEL-08-04, DEL-09-01,
  DEL-09-02, DEL-09-03 deliverable folders, and (by default) DEL-09-04;
- dependency registers, DAGs, decomposition, decision registers/packets,
  coordination stage records, PRD/PLAN, claims registry, or loop workplans;
- root governance, `_DomainEngines/**`, app-dev, PEC, or any external path;
- completed immutable evidence bundles under
  `validation/evidence/reproduction/**`.

Local branch/commit metadata is ordinary closeout mechanics after the adoption
chain becomes effective. This brief does not authorize a push, pull, fetch, PR
creation, self-merge, or any other network/external state change.

## 6. Evidence and Validation Plan

Because this tranche writes `core/**`, `validation/**`, `tests/**`,
`schemas/**`, `execution/**`, and `loop/**`, the project profile
(`software-workflow.json`) selects `piping-pytest` and `evidence-sweep`
(core/validation/tests paths), `harness-pytest` (execution/loop paths), and
always-check `harness-self-check`. Run them through the root tool from
`REPO_ROOT`, each as its own halting step, persisting normalized JSON into the
R12 AgentRuns directory:

```text
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check piping-pytest --output <out>
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check evidence-sweep --output <out>
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check harness-pytest --output <out>
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check harness-self-check --output <out>
```

The CLI shape (`profile` positional, `--check`, mandatory `--output`) was
checked against the live `--help` surface. The one `evidence-sweep` invocation
is also the DEC-025 five-surface pre-push gate for this code-touching branch;
it may create exactly one new `validation/evidence/sweeps/SWEEP_*.json`
(snapshot the directory before and after; stop if the delta is not exactly one
new artifact). Running the sweep creates no stage, lifecycle, release, or
acceptance authority.

Also require, from `WORKING_ROOT` unless noted, in sequence, every failure
stopping subsequent state-changing closeout:

- `cargo fmt --manifest-path core/runner/headless/Cargo.toml --check`;
- `cargo test --manifest-path core/runner/headless/Cargo.toml`;
- `cargo test --manifest-path validation/benchmarks/<suite>/Cargo.toml` for
  every suite crate touched under §5 item 4 (skip untouched crates);
- `python3 tests/test_headless_runner_contract.py`;
- `python3 tools/validation/validate_claims_language.py --repo-root .` from
  `REPO_ROOT`;
- `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check`;
- JSON parsing for every new `.json` file;
- changed-path containment against the §5 fence using
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT` (the live
  tool includes untracked non-ignored paths when no `--path` arguments are
  given; persist its JSON stdout in the R12 run directory);
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`
  from `REPO_ROOT`, after the receipt append.

No check may install missing dependencies, use the network, or update a
toolchain. A missing local prerequisite is a truthful blocked result, not
permission to provision it.

## 7. Defect and Failure Disposition

- Fail closed. Any acceptance predicate in §3 that does not hold, any check
  failure in §6, any fixture-byte drift in the frozen E1 surfaces, any
  unexpected changed path, or any suite-crate recorded-value change is a
  failure that stops closeout; keep the Remaining bullet open, record the
  truthful evidence, and return to HELP_HUMAN.
- No scope drift: no `export-results` binding, no threshold/tolerance
  creation or promotion (DEC-046 promotion is owner-gated and untouched), no
  schema/vocabulary reconciliation beyond §4.2, no persisted-project input
  surface, no CI activation, and no lifecycle/stage/release/acceptance act.
- A repair need outside the §5 fence — including the known consequence that
  `docs/validation_manual/headless_runner_reproduction.md` E1 case 3
  (run-benchmark stub expectation) becomes a historical expectation once the
  binding lands, and any DEL-09-04 Remaining-wording update — returns to
  HELP_HUMAN for a new lawful selection. The completed immutable DEL-09-04
  reproduction bundle records pre-binding behavior at its pinned source
  commit and is not edited, reinterpreted, or invalidated.
- If two suites conflict about a recorded value's meaning, or a comparison
  basis cannot be reused without inventing a tolerance, the affected cases
  fail closed with structured diagnostics; the conflict is reported, not
  resolved here.
- If the checkout is not clean before execution, stop and return the
  condition; do not stash, reset, or interpret unrelated work.

## 8. Rerun Triggers

A rerun (new execution record, same governed brief unless superseded) is
required when any of these changes after the implementation base commit:

- `DEC-065` policy, the runner crate surfaces, the runner schema, or the
  contract test;
- any bound suite crate's inventory, recorded values, claim posture, or
  governed DEC-046 policy records;
- the DEL-10-05 Remaining scope or lifecycle state;
- applicable active DAG-007 rows or the approved DAG pointer;
- `software-workflow.json` or the root software workflow profile;
- a prior failed/blocked result after the underlying condition is resolved.

A material governance change (new decision touching the runner surface,
supersession of D-52/D-54 lanes) returns the brief itself to HELP_HUMAN before
any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- the `export-results` payload binding or any DEL-08-01 report-container work;
- promotion of release thresholds, final tolerance policy, CI gate policy, or
  any DEC-046 tolerance record (owner-gated);
- new tolerance constants, acceptance criteria, or normative content of any
  kind; comparison bases are reused, never created;
- edits to suite fixture values, recorded expected values, policy JSON
  records, READMEs, hand-calcs, or the frozen E1 fixtures/generator/witnesses;
- validation-manual edits, MAINTAINER_REVIEWED case-page promotion, GUI
  workflow evidence, or closure of any DEL-09-04 Remaining item;
- reproduction acceptance, evidence-posture promotion, external-prover
  procurement/activation/correlation, or `PROVER_CORRELATED` /
  `ENGINEER_ACCEPTED` status;
- lifecycle transition, stage or milestone advancement, issuance, release,
  packaging, signing, publication, push, PR merge, hosted CI, network or
  external service use, or any external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (by N1, `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N1/RETURN.md`)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: ORCHESTRATOR (Agent 1), HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12 / N1
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected design per §4)
SelectedOutcome: bind run-benchmark/run-regression per §3–§4 within the §5 fence
JudgedBy: ORCHESTRATOR (Agent 1), HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12 / N1
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (payload naming, case addressing, verb-to-suite mapping, module layout)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N2/RETURN.md` (21/21 claims confirmed; independent D-52 §4.1 re-screen pass)
EffectStatus: EFFECTIVE — EXECUTION RELEASED BY HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)
PreservedGates: export-results binding; DEC-046 threshold/tolerance promotion; lifecycle/stage/issuance/release/acceptance; reproduction acceptance and evidence-posture promotion; prover activation/correlation; publication/external action; merge authority; D-45; D-06b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the agent
classifies, selects among defensible implementation shapes under D-54, and
proposes. HELP_HUMAN — not N1 — progresses `EffectStatus` only after the
independent refutation returns `COMMIT-SAFE` and the governed record is
durably bound per K-AUTH-2 / D-GOV-04. No execution is released by this
document in its current state.
