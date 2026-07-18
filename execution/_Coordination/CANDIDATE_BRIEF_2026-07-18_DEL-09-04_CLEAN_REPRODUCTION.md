---
doc_id: CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001
doc_kind: coordination.candidate_brief
status: owner_standing_approval_activated_ready_for_durable_landing
prepared: 2026-07-18
package_id: PKG-09
deliverable_id: DEL-09-04
decision_basis: DEC-080, DEC-085
owner_standing_approval: activated_s5_commit_safe_awaiting_durable_landing
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval_ready_for_landing
---

# CANDIDATE Brief — DEL-09-04 Actor-Neutral Clean-Checkout Reproduction

**Status:** `OWNER STANDING APPROVAL ACTIVATED — READY FOR DURABLE LANDING; EXECUTION HOLD`

**Prepared by:** ORCHESTRATOR for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260718-DEL0904-R1`

**Selected work item:** the DEL-09-04 `_STATUS.md ## Remaining` actor-neutral
clean-checkout reproduction item only

The owner's current direction, recorded in D-52/`DEC-085`, is the conditional
class-level approval and adoption rule itself. HELPS_HUMANS classified this
brief `CLASSIFY_ELIGIBLE` and recorded `ACTIVATE_OWNER_STANDING_APPROVAL`.
Adoption remains the human owner's act under that rule; the agent does not
adopt or rule, and no separate owner case selection is claimed. The adoption
effect remains held until the owner approval plus agent
classification/activation are durably SHA-bound. The local verifier and actual
DEC-083 S5 sibling review are both `COMMIT-SAFE`; no correction remains. Only
after durable landing may a new managed execution node release the bounded
work below. Adoption does not accept a reproduction result,
advance a lifecycle or target stage, or perform any other owner act.

## 1. Purpose and Accepted Basis

Purpose: produce one actor-neutral, local-only reproduction of the three
documented E1 headless-runner examples from a clean checkout of a pinned source
commit, recording the environment, tool versions, exact commands, exit codes,
and output hashes in the DEC-080 evidence home.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

Accepted basis, verified against the live tree at brief preparation:

- root `AGENTS.md` and project `AGENTS.md`;
- `loop/WORKPLAN_2026-07-17_piping_loop.md` and structurally valid
  `loop/LOOP_RECEIPTS.md` through the latest applicable `Receipt-55` cursor;
- `execution/_DAG/_LATEST.md` resolving to approved `DAG-007`, its
  `APPROVAL_RECORD.md`, and the active DEL-09-04 execution-upstream rows;
- `software-workflow.json` under the ratified root
  `docs/SOFTWARE_WORKFLOW_PROFILE.md` contract;
- DEL-09-04 `_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`,
  dependency registers, consolidated `ScopeOfWork.md` successor to the former
  four-document kit, and the relevant 2026-07-05/2026-07-10/2026-07-12 local
  run records;
- `docs/_Registers/Deliverables.csv` row DEL-09-04 and
  `docs/_Registers/ScopeLedger.csv` row SOW-027;
- applicable invariants in `docs/CONTRACT.md`, especially OPS-K-IP-1..3,
  OPS-K-DATA-1..3, OPS-K-AUTH-1..2, OPS-K-PRIV-1..2, OPS-K-AGENT-1..4,
  OPS-K-UNIT-1, and OPS-K-MECH-2;
- amended `docs/PRD.md` §§22.1, 22.5, and 24 R6;
- `D-47_prd_validation_posture_and_relocation.md` §§5 and 8 and its
  codification as `DEC-080` in `SOFTWARE_DECOMP.md` §12;
- the documented E1 procedure at
  `docs/validation_manual/headless_runner_reproduction.md` and the live
  fixtures, generator, runner manifest, and contract test cited by it.

Live selection facts:

- DEL-09-04 is `IN_PROGRESS`, has the named open Remaining item, and is a
  canonical DAG-007 node.
- Its active `EXECUTION / UPSTREAM` rows target DEL-00-01, DEL-00-02,
  DEL-00-06, DEL-00-08, DEL-09-01, DEL-09-02, DEL-09-03, and DEL-01-04; all
  eight rows are `SATISFIED` in DAG-007. No active execution-upstream blocker
  was found.
- DEC-080 removed the earlier stage-gate parking and fixed the immutable
  derivative evidence home as
  `validation/evidence/reproduction/<run-id>/`.
- D-45 and D-06b remain open owner gates, but neither governs this bounded
  reproduction item.

## 2. Objective

From one local clean checkout of one pinned `SOURCE_COMMIT`, run the three E1
examples documented in `headless_runner_reproduction.md`:

1. `solve` — expected exit `0` and a completed, diagnostic-clean result with
   a non-empty `runner_result.result_refs` list;
2. `validate-input` — expected exit `1`, diagnostic
   `HEADLESS_RUNNER_LOAD_BASIS_MISSING`, and no solver result;
3. `run-benchmark` — expected exit `1`, diagnostic
   `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`.

Record the required provenance and outputs in a unique immutable bundle. A
successful run may close only the exact DEL-09-04 Remaining bullet named by
this brief. It does not close the broader E2 residuals.

## 3. Bounded Tasks

### 3.1 Freeze the execution basis

- Begin on a new local `codex/` branch from a clean integration checkout.
- Record `SOURCE_COMMIT=$(git rev-parse HEAD)` before any evidence write.
- Freeze one `RUN_ID` before the first durable write, using
  `REPRO_DEL0904_<UTC-YYYYMMDDTHHMMSSZ>_<SOURCE_COMMIT-12>`.
- Record the adopted brief commit or content hash in the managed execution
  run record.
- Stop if the active DAG pointer, DEL-09-04 lifecycle/Remaining item, DEC-080,
  procedure path, or software workflow profile has changed materially since
  this brief. Do not silently reinterpret the scope.

### 3.2 Preflight without provisioning

Resolve and record the existing tools only:

```text
git --version
python3 --version
rustc --version --verbose
cargo --version --verbose
uname -a
```

On Darwin, also record `sw_vers` when already available. Record the absolute
executable paths returned by `command -v`. Do not install, update, download,
or activate any toolchain or dependency. If an existing tool or offline Cargo
cache is insufficient, finalize the run as blocked/failed evidence and stop.

### 3.3 Create a clean, local-only checkout

- Allocate a task-specific temporary directory with `mktemp -d`.
- Create a local filesystem clone from the resolved `REPO_ROOT` only:
  `git clone --no-hardlinks --no-checkout "$REPO_ROOT" "$REPRO_CHECKOUT"`.
  A URL, fetch, pull, submodule download, or other network source is forbidden.
- Run `git -C "$REPRO_CHECKOUT" checkout --detach "$SOURCE_COMMIT"`.
- Prove the source identity and initial cleanliness with `git rev-parse HEAD`,
  `git status --porcelain=v1 --untracked-files=all`,
  `git diff --exit-code`, and `git diff --cached --exit-code`. The two status
  outputs must be empty and both diff checks must exit `0`.
- Set `REPRO_PROJECT="$REPRO_CHECKOUT/projects/chirality-piping"` and keep
  `CARGO_TARGET_DIR` under the temporary root, outside the checkout.

The temporary clone, Cargo target directory, and scratch logs are ephemeral
execution state. They are removed after their evidence has been copied and
hashed. They are not durable project outputs.

### 3.4 Execute the documented E1 procedure offline

Run from `REPRO_PROJECT`. Preserve each command as an argv/cwd record; capture
stdout, stderr, and the actual exit code separately. Do not place expected
nonzero commands in a shell chain that would either skip them or permit a
failed mandatory assertion to fall through.

```text
python3 validation/witness/inputs/generate_tp_runner_015_inputs.py

CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<temporary-target> \
  cargo run --offline --manifest-path core/runner/headless/Cargo.toml \
  --bin openpipestress-runner -- solve \
  --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json \
  --output <temporary-output>/tp_runner_015_solve.json

CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<temporary-target> \
  cargo run --offline --manifest-path core/runner/headless/Cargo.toml \
  --bin openpipestress-runner -- validate-input \
  --input validation/witness/inputs/tp_runner_015_final_cli_validation_blocking_input.json \
  --output <temporary-output>/tp_runner_015_validation_blocking.json

CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<temporary-target> \
  cargo run --offline --manifest-path core/runner/headless/Cargo.toml \
  --bin openpipestress-runner -- run-benchmark \
  --input validation/witness/inputs/tp_runner_015_final_cli_benchmark_stub_input.json \
  --output <temporary-output>/tp_runner_015_benchmark_stub.json
```

The environment assignment and `--offline` flag are mandatory. Cargo may
compile from an already-present local cache; it may not retrieve missing
material. The generator must exit `0`, and a scoped `git diff --exit-code`
must prove that it reproduced the three committed input fixtures byte-for-byte.

Run the two documented review checks under the same offline/no-install
posture and record them as separate commands:

```text
CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<temporary-target> \
  cargo test --offline --manifest-path core/runner/headless/Cargo.toml
python3 tests/test_headless_runner_contract.py
```

After all five runtime/review commands, repeat the clean-checkout status and
tracked-diff checks. Ignored build metadata is not evidence and must not be
copied into the repository.

### 3.5 Validate outputs and build the immutable bundle

Use Python's standard JSON parser or another already-present local tool to
assert the exact objective predicates in §2 against the three output files.
Do not compare a fixed `result_refs` count: the 2026-07-10 run recorded 830
against an older committed witness count of 822, while the governed E1
acceptance predicate is non-empty.

The bundle at `validation/evidence/reproduction/<RUN_ID>/` must contain at
least:

```text
README.md
manifest.json
environment.txt
source_commit.txt
procedure.sha256
git-status-before.txt
git-status-after.txt
commands.jsonl
validation_summary.json
SHA256SUMS.txt
stdout/<command-id>.txt
stderr/<command-id>.txt
exit_codes/<command-id>.txt
outputs/tp_runner_015_solve.json
outputs/tp_runner_015_validation_blocking.json
outputs/tp_runner_015_benchmark_stub.json
checks/<check-id>.json-or-txt
```

`manifest.json` must identify the bundle as a derivative package; cite
`SOURCE_COMMIT`, the adopted brief, DEC-080, and the procedure path plus
SHA-256; record run ID, UTC timestamps, cwd, argv, environment overrides,
expected and actual exit codes, input hashes, output hashes, and validation
predicates; and state `overall_status` as `PASS`, `FAIL`, or `BLOCKED`.
`README.md` must use the `INTERNALLY_VERIFIED` evidence label only when the
criteria pass and must say: "Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081)." It must not use `PROVER_CORRELATED` or
`ENGINEER_ACCEPTED`.

Compute `SHA256SUMS.txt` only after bundle contents other than that checksum
file are final. Verify the checksum file. Once finalized, never overwrite or
reuse the run directory; any rerun receives a new `RUN_ID`.

### 3.6 Update local state and close out

On `PASS` only:

- remove only the exact clean-checkout reproduction bullet from DEL-09-04
  `_STATUS.md ## Remaining`;
- append one `_STATUS.md ## History` entry citing the adopted brief,
  DEC-080, bundle path, source commit, and `INTERNALLY_VERIFIED` result while
  preserving lifecycle `IN_PROGRESS` and every other Remaining item;
- append DEL-09-04 `MEMORY.md` with the same evidence pointer, material
  observations, and preserved boundaries;
- write one new DEL-09-04 `_run_records/WORKING_ITEMS_RUN_*.md` execution
  record;
- append one versioned minimal loop receipt after all checks pass, then rerun
  the receipt validator.

On `FAIL` or `BLOCKED`, keep the Remaining bullet and lifecycle unchanged.
Write a truthful failed/blocked bundle and local run record if execution began;
do not implement a repair in this tranche.

## 4. Evidence and Validation Plan

### 4.1 Reproduction evidence

The bundle proves, by recorded content rather than narrative alone:

- exact clean source commit and empty initial checkout status;
- environment and tool versions;
- procedure identity and hash;
- exact argv/cwd/environment for each command;
- actual exit codes, including both expected exit-1 diagnostic cases;
- output JSON predicate checks and SHA-256 hashes;
- generator determinism and final tracked-source cleanliness;
- derivative-package status and claim-calibrated result.

### 4.2 Profile-driven and loop closeout checks

Because this tranche writes `validation/**`, `execution/**`, and `loop/**`,
the project profile independently selects these registered checks:

- `piping-pytest`;
- `evidence-sweep`;
- `harness-pytest`;
- always-check `harness-self-check`.

Run them through `tools/software_workflow/run_registered_checks.py` and retain
normalized results in the reproduction bundle. The one invocation of
`evidence-sweep` may create exactly one new tool-generated
`validation/evidence/sweeps/SWEEP_*.json`; snapshot that directory before and
after and stop if the delta is not exactly one new sweep artifact.

These are root tools, not WORKING_ROOT tools. Run their actual CLI from
`REPO_ROOT` after `BUNDLE` has resolved to the absolute reproduction-bundle
path:

```text
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check piping-pytest \
  --output "$BUNDLE/checks/piping-pytest.json"
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check evidence-sweep \
  --output "$BUNDLE/checks/evidence-sweep.json"
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check harness-pytest \
  --output "$BUNDLE/checks/harness-pytest.json"
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check harness-self-check \
  --output "$BUNDLE/checks/harness-self-check.json"
```

The CLI shape above was checked against the live `--help` surface. Run each
mandatory check as its own halting step; do not chain a failed check into a
later state-changing command.

DEC-025 judgment: this evidence/state tranche does not touch code, so the
workplan's code-touching pre-push rule does not independently trigger the
DEC-025 sweep. Nevertheless, `software-workflow.json` selects the same
`evidence-sweep` command because the authorized bundle is under
`validation/**`; it remains mandatory on that distinct profile basis. Running
the sweep does not create stage, lifecycle, release, or acceptance authority.

Also require, in sequence, with every failure stopping subsequent
state-changing closeout:

- `python3 tools/validation/validate_claims_language.py --repo-root .`;
- `python3 tools/validation/validate_path_anchors.py . --text`;
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`
  after the receipt append;
- `git diff --check`;
- JSON parsing for every new `.json` file;
- changed-path containment against the write fence, using
  `tools/software_workflow/validate_change_scope.py` or an equivalent
  deterministic comparison that includes untracked files.

For the deterministic containment check, run from `REPO_ROOT` after freezing
the exact repo-relative values `SWEEP_FILE`, `RUN_RECORD`, and
`EXECUTION_RUN_ID`:

```text
python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" \
  --base "$SOURCE_COMMIT" \
  --allowed projects/chirality-piping/execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md \
  --allowed "projects/chirality-piping/validation/evidence/reproduction/$RUN_ID" \
  --allowed "projects/chirality-piping/validation/evidence/sweeps/$SWEEP_FILE" \
  --allowed "projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md" \
  --allowed "projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/MEMORY.md" \
  --allowed "projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/$RUN_RECORD" \
  --allowed "projects/chirality-piping/execution/_Coordination/AgentRuns/$EXECUTION_RUN_ID" \
  --allowed projects/chirality-piping/loop/LOOP_RECEIPTS.md
```

With no `--path` arguments, the live tool includes both Git diff paths and
untracked, non-ignored paths. Persist its JSON stdout in the bundle without
changing the CLI semantics (for example, by ordinary stdout redirection to a
predeclared `checks/change-scope.json` file).

No check may install missing dependencies. A missing local prerequisite is a
truthful blocked result, not permission to provision it.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to:

1. this candidate brief, only for the governed standing-approval and agent-
   classification/activation record or a later superseding hold/rejection
   record;
2. exactly one new
   `validation/evidence/reproduction/<RUN_ID>/**` bundle;
3. exactly one new tool-emitted
   `validation/evidence/sweeps/SWEEP_*.json` from the single mandatory
   profile-run evidence sweep, verified as a one-file before/after delta;
4. DEL-09-04 only:
   - `_STATUS.md`;
   - `MEMORY.md`;
   - one new `_run_records/WORKING_ITEMS_RUN_*.md`;
5. one bounded managed-execution record under
   `execution/_Coordination/AgentRuns/<EXECUTION_RUN_ID>/**`;
6. `loop/LOOP_RECEIPTS.md`, append-only, for one new versioned minimal receipt.

Ephemeral writes are limited to one task-specific temporary local clone,
Cargo target directory, outputs, and logs under the `mktemp -d` root. Cleanup
may remove only that resolved temporary root after its absolute path has been
validated as the task-created directory.

No other project file is writable. In particular, do not write or fix:

- `core/**`, `tests/**`, fixture/generator inputs, committed witness outputs,
  the validation manual pages, schemas, scripts, or tools;
- any other deliverable or package, including DEL-10-05;
- dependency registers, DAGs, decomposition, decision registers/packets,
  coordination stage records, PRD/PLAN, claims registry, or loop workplan;
- root governance, `_DomainEngines/**`, app-dev, PEC, or any external path.

Local branch/commit metadata is ordinary closeout mechanics after the adoption
chain becomes effective.
This brief does not authorize a push, pull, fetch, PR creation, self-merge, or
other network/external state change.

## 6. Acceptance Criteria

The tranche passes only when all of the following are true:

1. The owner standing approval, agent `CLASSIFY_ELIGIBLE` classification and
   `ACTIVATE_OWNER_STANDING_APPROVAL` record, local `COMMIT-SAFE`, satisfactory
   DEC-083 S5 review, and durable SHA binding were all recorded before
   execution.
2. The clean local checkout is pinned to the recorded `SOURCE_COMMIT`, with
   empty initial status and zero tracked/cached diff.
3. No network, installation, dependency update, or external service was used.
4. The generator exits `0` and reproduces the three committed input fixtures
   byte-for-byte.
5. `solve` exits `0`; job state is `COMPLETED`; request/result diagnostics are
   empty; `result_refs` is non-empty.
6. `validate-input` exits `1`; it contains
   `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; no solver result is emitted.
7. `run-benchmark` exits `1`; it contains
   `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`.
8. Both documented review checks exit `0`.
9. Final tracked source status remains clean; all required environment,
   command, exit, output, predicate, and hash records exist; checksum
   verification passes.
10. The required profile checks, claims lint, path-anchor check, receipt
    validator, JSON parsing, whitespace check, and containment check all pass.
11. The status update closes only this reproduction bullet, preserves
    lifecycle `IN_PROGRESS` and every other Remaining item, and makes no
    broader claim.
12. The receipt and run record cite the unique bundle and source commit and
    report the result without calling it owner acceptance, stage exit, release
    readiness, prover correlation, or professional acceptance.

## 7. Defect and Failure Disposition

- Expected exit `1` for the two diagnostic cases is evidence only if the
  matching predicate passes. Any different exit or diagnostic is `FAIL`.
- Missing offline dependencies/tools are `BLOCKED`; do not install or fetch.
- Output-schema drift, changed fixture bytes, test failure, hash mismatch,
  profile-check failure, non-clean source state, or an unexpected changed path
  is `FAIL` and stops closeout.
- Preserve the unique failed/blocked bundle and record the observed evidence;
  keep the Remaining item open. A rerun uses a new ID and cites the failed run.
- Do not repair runner code, fixtures, tests, docs, governance, or another
  deliverable under this brief. Return any repair need to HELP_HUMAN for a new
  lawful selection and owner gate where applicable.
- If the checkout is not clean before execution, do not stash, reset, revert,
  or interpret unrelated work; stop and return the condition.

## 8. Rerun Triggers

A new run and new immutable bundle are required when any of these changes
after the source commit used here:

- the E1 procedure, its generator, any of the three input fixtures, the
  headless-runner manifest/source/dependencies, or the contract test;
- DEC-080, PRD §§22/24, DEL-09-04 acceptance/Remaining scope, or the evidence
  home;
- applicable active DAG-007 rows or the approved DAG pointer;
- `software-workflow.json` or the root software workflow profile;
- any toolchain/environment change when evidence is being compared across
  runs;
- a prior `FAIL`/`BLOCKED` result after the underlying condition is resolved.

Normal later edits elsewhere do not mutate or overwrite a completed bundle.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- code, fixture, test, documentation, schema, tool, dependency, or workflow
  fixes;
- network access, dependency/tool installation, hosted CI, cloud/daemon/
  telemetry operation, publication, push, PR creation, merge, or release;
- public-benchmark tolerance or threshold promotion;
- `MAINTAINER_REVIEWED` case-page promotion, GUI workflow evidence, or the
  remaining E2 benchmark/regression runner payload bindings;
- external-prover procurement or activation, comparison, or
  `PROVER_CORRELATED` status;
- reproduction acceptance, `ENGINEER_ACCEPTED` status, professional
  judgment, or third-party/publication-era reproduction;
- target-stage advancement, lifecycle transition, issuance, signing,
  notarization, or any D-06b act;
- temperature-indexed shear-modulus disposition or any D-45 act.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval and Agent Classification

**Owner standing-approval status:** `ACTIVATED — S5_COMMIT_SAFE / READY_FOR_DURABLE_LANDING`

**Owner:** Ryan Tufts

**Owner act/date:** conditional class-level standing approval and adoption
rule, 2026-07-18

**Governed pointer:** D-52 §2 / `DEC-085`; verbatim text SHA-256
`a31a551af14b10381087cb9150f3ef6e9a1339d866d4e1b8a221e68cca36f7b2`

**Agent classification:** `CLASSIFY_ELIGIBLE`

**Rule activation:** `ACTIVATE_OWNER_STANDING_APPROVAL`

**Classified/activated by:** HELPS_HUMANS,
`HELP-HUMAN-PIPING-20260718-STANDING-APPROVAL-R2` /
`HELPS-HUMANS-PIPING-STANDING-APPROVAL-01`

**Adoption authority:** `HUMAN_OWNER_BY_STANDING_APPROVAL`

**Owner adoption result:** `CONDITIONAL_ADOPTION_TRIGGERED_BY_ELIGIBILITY_CLASSIFICATION — READY_FOR_DURABLE_LANDING`

**Owner case-specific selection:** `NONE`

**Rationale:** D-52 §5 and
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-STANDING-APPROVAL-R2/CURRENT_CANDIDATE_RATIONALE.md`

**Independent reviews:** local verifier-01 `BLOCK` preserved; corrected local
verifier-02 `COMMIT-SAFE`; actual DEC-083 S5 sibling review `COMMIT-SAFE` with
no correction required, recorded in the R2 run.

**Effect status:** `READY_FOR_DURABLE_LANDING — EXECUTION HELD UNTIL K-AUTH-2 / D-GOV-04 BINDING`

**Preserved gates:** reproduction-result acceptance; evidence-posture
promotion; lifecycle/stage/issuance/release; prover activation/correlation;
publication/external action; merge authority; D-45; D-06b; F-PIP-1..5.

`READY_FOR_DURABLE_LANDING` is not `EFFECTIVE`. No downstream execution node
may be released before durable K-AUTH-2/D-GOV-04 landing.
