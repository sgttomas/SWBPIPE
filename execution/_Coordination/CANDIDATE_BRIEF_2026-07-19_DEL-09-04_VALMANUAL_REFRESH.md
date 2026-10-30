---
doc_id: CB-2026-07-19-DEL-09-04-VALMANUAL-REFRESH-001
doc_kind: coordination.candidate_brief
status: adopted_effective_execution_released
prepared: 2026-07-19
package_id: PKG-09
deliverable_id: DEL-09-04
decision_basis: DEC-065, DEC-080, DEC-081, DEC-046 (preserved gate), DEC-085, DEC-087
agent_classification: classify_eligible
rule_activation: activate_owner_standing_approval
---

# CANDIDATE Brief — DEL-09-04 Validation-Manual Reproduction Page Refresh (Post-PR #287)

**Status:** `EFFECTIVE — EXECUTION RELEASED BY HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)`

**Prepared by:** ORCHESTRATOR (N1) for HELP_HUMAN

**Current run:** `HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13`

**Selected work item:** the DEL-09-04-owned follow-on recorded at R12 close
(`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/HANDOFF_STATE.md`):
refresh `docs/validation_manual/headless_runner_reproduction.md` to the
post-PR #287 runner behavior and strike only the landed
benchmark/regression-bindings dependency from the DEL-09-04 `_STATUS.md
## Remaining` first bullet. Docs + deliverable-state only. No reproduction
run, no lifecycle change, no threshold/tolerance content, no schema, code, or
test change, and no DEL-10-05 write.

This brief is authored under the D-54/`DEC-087` reasoned-selection lane on the
D-52/`DEC-085` standing-approval overlay. Adoption remains the human owner's
conditional act under the standing rule; this document classifies and proposes
only. The adoption effect is `HELD` until independent refutation (N2) returns
`COMMIT-SAFE` and HELP_HUMAN progresses the chain. No execution writes are
authorized by this document in its current state.

## 1. Purpose and Accepted Basis

Purpose: make the DEL-09-04 headless-runner reproduction page truthful again
on post-PR #287 sources. PR #287 (implementation commit
`60841413aca1753229df62997a4637d2179114db`, merge
`45ec0524d3b0c155392553a3b3e4190534ff0fe8`, Receipt-59) bound the
`run-benchmark` and `run-regression` payloads in `openpipestress-runner`
under `CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001`. The page still documents
the pre-binding stub expectation as current behavior; that expectation is now
historical, and the committed `del1005_payload_binding_*` witness family
provides the bound per-case reproduction path that the page should document.

The later executor must resolve paths from the active checkout:

```text
REPO_ROOT=$(git rev-parse --show-toplevel)
WORKING_ROOT=${REPO_ROOT}/projects/chirality-piping
```

All relative paths below are relative to `WORKING_ROOT` unless stated.

Accepted basis, verified against the live tree at brief preparation
(HEAD `45ec0524d3b0c155392553a3b3e4190534ff0fe8`, branch
`claude/piping-r13-valmanual-refresh`):

- root `AGENTS.md` and project `AGENTS.md`;
- active committed-HEAD workplan `loop/WORKPLAN_2026-07-18b_piping_loop.md`
  and structurally valid `loop/LOOP_RECEIPTS.md` through cursor `Receipt-59`;
- parent plan
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/ORCHESTRATION_PLAN.md`
  (plan version 1, posture TERMINAL_FAN_OUT_IN, serialized N1→N2→N3→N4);
- `execution/_DAG/_LATEST.md` resolving to approved `DAG-007`
  (`approved_active_graph_authority`) and the DEL-09-04 rows in
  `execution/_DAG/DAG-007/DependencyEdges.csv`;
- DEL-09-04 deliverable folder (under
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
  `_STATUS.md` (`IN_PROGRESS`; two Remaining bullets), `MEMORY.md` (R11
  clean-reproduction entry newest), `_CONTEXT.md` (Type `DOC_UPDATE`),
  `ScopeOfWork.md`, `Dependencies.csv`, and
  `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R11.md`
  (reproduction `PASS` at pinned source commit
  `23eeaabc904064e2297690e391df153dea116ff0`, bundle
  `validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`,
  `INTERNALLY_VERIFIED`);
- the page itself: `docs/validation_manual/headless_runner_reproduction.md`
  (stub-expectation rows at lines 43, 59, and 95–99 of the live file);
- the R12 chain:
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`
  (adopted, executed), the N3 implementation return
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/instances/N3/RETURN.md`
  (overall `PASS`, 9/9 predicates, N4 v2 `COMMIT-SAFE`), and
  `.../HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/HANDOFF_STATE.md`
  (this tranche is the recorded follow-on);
- runner ground truth (live source):
  `core/runner/headless/src/bin/openpipestress-runner.rs` —
  `RunnerOperation::RunBenchmark` and `RunnerOperation::RunRegression` route
  through `execute_suite_verb` and the binding module;
  `RunnerOperation::ExportResults` alone retains the stub diagnostic
  `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD`;
  payload-missing diagnostics `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` /
  `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`; and
  `core/runner/headless/src/benchmark_binding.rs` (per-case reporting,
  fail-closed blocking codes including
  `HEADLESS_RUNNER_{BENCHMARK,REGRESSION}_CASE_COMPARISON_BASIS_NOT_REUSABLE`);
- committed witness evidence for the bound path: the generator
  `validation/witness/inputs/generate_del1005_payload_binding_inputs.py`, the
  five `validation/witness/inputs/del1005_payload_binding_*_input.json`
  fixtures, and the five
  `validation/witness/generated/del1005_payload_binding_*.json` witnesses
  (contents verified at preparation; facts enumerated in §3 and in the N1
  return);
- frozen E1 surfaces (read-only): the three
  `validation/witness/inputs/tp_runner_015_final_cli_*_input.json` fixtures,
  `validation/witness/inputs/generate_tp_runner_015_inputs.py`, and the three
  committed `validation/witness/generated/tp_runner_015_final_cli_*.json`
  witnesses;
- governance: `DEC-065` (local CLI/process policy and exit-code policy
  0/1/2 — exit 0 only with no blocking diagnostics), `DEC-080` (evidence home
  `validation/evidence/reproduction/<run-id>/`; actor-neutral clean-checkout
  criterion), `DEC-081` (claims taxonomy), `DEC-046` (governed convergence
  tolerance records; promotion owner-gated and untouched here), D-52/`DEC-085`
  (`execution/_Coordination/_DECISIONS/D-52_four_lens_standing_approval_overlay.md`),
  D-54/`DEC-087`
  (`execution/_Coordination/_DECISIONS/D-54_reasoned_discretion_standing_approval_refinement.md`),
  and the prior adopted DEL-09-04 brief
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
  (its §8 rerun triggers drive the rerun-consequence statement this page must
  carry);
- `software-workflow.json` under the ratified root
  `docs/SOFTWARE_WORKFLOW_PROFILE.md` contract, and the root tools
  `tools/software_workflow/run_registered_checks.py` and
  `tools/software_workflow/validate_change_scope.py`.

## 2. Live Selection Facts

- DEL-09-04 is `IN_PROGRESS` with exactly two `## Remaining` bullets. The
  first (E2 residuals) currently reads, in part: "runner benchmark/regression
  payload bindings still structured stubs (per-case reproduction runs through
  suite tests; see also DEL-10-05); MAINTAINER_REVIEWED case-page promotion
  and GUI-workflow validation evidence open". The bindings clause is now
  contradicted by the live tree (PR #287 landed; DEL-10-05 closed its
  bindings Remaining bullet at R12). The second bullet (owner-gated DEC-046
  tolerance promotion) is out of scope and must remain byte-identical.
- DAG-007 posture for DEL-09-04
  (`execution/_DAG/DAG-007/DependencyEdges.csv`, rows 766–777): three ANCHOR
  rows (`DEL-09-04-A001` SOW-027, `A002` OBJ-008, `A003` OBJ-011,
  `SatisfactionStatus NOT_APPLICABLE`); six EXECUTION UPSTREAM rows, all
  `SATISFIED` — `DAG-002-E0286/E0287/E0288/E0289` (architecture-basis
  constraints AB-00-01/02/06/08), `DAG-002-E0543/E0544/E0545` (benchmark
  suite prerequisites DEL-09-01/09-02/09-03), `DAG-002-E0546`
  (professional-boundary constraint DEL-01-04); and one DOWNSTREAM ENABLES
  row `DEL-09-04-E001` (DEL-09-05), `SATISFIED`. No `TBD` or blocking
  execution-upstream row exists for DEL-09-04 in the approved graph; row
  Notes cite superseded DAG-006 row IDs as provenance, which is historical
  packaging, not a live conflict.
- The page's three stale surfaces on the live file: the Fixture Set table row
  3 (line 43: `run-benchmark` on the frozen stub input expected to emit the
  stub diagnostic), the Reproduction Procedure third runner command
  (line 59), and the Remaining E2 Work paragraph (lines 95–99:
  "`run-benchmark` / `run-regression` remain structured stubs" and "a clean
  environment demonstration record" — the latter also landed at R11,
  Receipt-58).
- Post-#287 behavior of the frozen E1 case 3: the frozen input
  `tp_runner_015_final_cli_benchmark_stub_input.json` contains only a
  `request` object (no `benchmark` payload), so on post-#287 sources the same
  command exits 1 with `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` — not the
  stub diagnostic recorded in its committed witness. The committed witness
  and the R11 bundle remain truthful for their pinned pre-#287 commits.
- The five committed del1005 witnesses carry the bound-path facts the
  refreshed page may assert (per-file enumeration in §3.2 and the N1 return).
- Receipt cursor is `Receipt-59` (R12); the R12 HANDOFF_STATE names this
  page-plus-Remaining refresh as the natural next selection and records that
  any post-refresh E1 reproduction rerun needs a new run ID.

## 3. Objective and Acceptance Predicates

Refresh the page and DEL-09-04 state so that all of the following hold on the
execution head:

1. **Witness-anchored assertions.** Every command shape, expected exit code,
   expected diagnostic code, and expected per-case count that the refreshed
   page asserts is verified against committed witness evidence in the live
   tree (the five `del1005_payload_binding_*` input/generated pairs, the
   preserved R12 N3 evidence, and the DEC-065 exit policy: exit 0 only when
   no blocking diagnostic is present, exit 1 with blocking diagnostics, exit
   2 reserved for usage/malformed input) or against a live offline run of the
   documented command at the execution head. No asserted value may originate
   in this tranche.
2. **Bound-path documentation.** The page documents the bound
   `run-benchmark`/`run-regression` per-case reproduction path using exactly
   the committed fixtures and witnesses, including at least:
   - `del1005_payload_binding_benchmark_single_case_input.json` —
     `run-benchmark`, suite `mechanics` (DEL-09-01), one case
     `MECH-TP-PHYS-004-LOAD-TO-RESULTANT`; expected exit 0; witness reports
     1/1 `executed_and_matched`, no diagnostics;
   - `del1005_payload_binding_benchmark_multi_case_input.json` —
     `run-benchmark`, suite `stress` (DEL-09-02), three named cases; expected
     exit 0; witness reports 3/3 `executed_and_matched`, no diagnostics;
   - `del1005_payload_binding_regression_full_suite_input.json` —
     `run-regression`, suite `nonlinear` (DEL-09-03), omitted `cases` list;
     expected exit 0; witness reports whole-suite default applied, 5/5
     `executed_and_matched`, no diagnostics;
   - `del1005_payload_binding_benchmark_payload_missing_input.json` —
     expected exit 1 with `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`;
   - `del1005_payload_binding_regression_payload_missing_input.json` —
     expected exit 1 with `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING`;
   plus the generator
   `generate_del1005_payload_binding_inputs.py` as the deterministic input
   source and the five committed generated witnesses as the recorded outputs.
3. **Fail-closed semantics described truthfully.** The page describes the
   blocked-case behavior as it exists: cases whose recorded comparison basis
   is not reusable through a crate-encoded predicate fail closed with
   `HEADLESS_RUNNER_{BENCHMARK,REGRESSION}_CASE_COMPARISON_BASIS_NOT_REUSABLE`
   (on the R12 implementation head: mechanics whole-suite 11/21 matched + 10
   blocked; stress whole-suite 12/15 matched + 3 blocked; a whole-suite run
   containing blocked cases exits 1), described strictly as regression
   evidence for current solver behavior — not as a defect, not as a release
   judgment, and not as new acceptance criteria. Release thresholds, final
   tolerance policy, CI gate policy, and professional reliance remain `TBD`,
   owner-gated.
4. **Frozen E1 procedure preserved.** The three original tp_runner_015 cases
   remain on the page as the frozen E1 procedure, with a dated 2026-07-19
   note stating that case 3's stub expectation is historical for sources at
   or after PR #287 (`60841413a`, merged at `45ec0524d`): on such sources the
   same frozen command exits 1 with
   `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` because the frozen input
   carries no `benchmark` payload; the committed
   `tp_runner_015_final_cli_benchmark_stub.json` witness and prior
   reproduction bundles (including
   `REPRO_DEL0904_20260719T202023Z_23eeaabc9040`) remain truthful for their
   pinned commits. The three frozen input fixtures, their generator, and the
   three committed generated witnesses stay byte-identical.
5. **Rerun-trigger consequence stated.** The page states that, per
   `CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8, any new clean-checkout
   reproduction runs from a post-#287 source commit under a fresh run ID and
   new immutable bundle; completed bundles are not edited, reinterpreted, or
   invalidated.
6. **Remaining E2 truthfulness.** The page's Remaining E2 Work paragraph is
   refreshed to the live tree: still open — public-benchmark threshold
   disposition (owner-gated), MAINTAINER_REVIEWED case-page promotion,
   GUI-workflow validation evidence, and the `export-results` binding (the
   only remaining structured stub, DEL-10-05); landed — the
   benchmark/regression bindings (PR #287) and the clean-checkout
   demonstration (R11 bundle, cited, `INTERNALLY_VERIFIED`, owner acceptance
   preserved).
7. **Claims calibration.** All refreshed language stays inside the
   regression-evidence claim posture (DEC-081); no new tolerance, threshold,
   acceptance criterion, or normative content is introduced anywhere in the
   tranche; the page continues to state that validation acceptance and
   professional judgment remain with the responsible engineer.
8. **Bounded state update.** DEL-09-04 `_STATUS.md`: only the first Remaining
   bullet is edited, and only to strike the landed bindings dependency
   (recording that the bindings landed via DEL-10-05 / PR #287) while keeping
   MAINTAINER_REVIEWED case-page promotion and GUI-workflow validation
   evidence open in that bullet; the second (owner-gated tolerance) bullet is
   byte-identical; `Current State: IN_PROGRESS` unchanged; exactly one new
   History entry and updated `Last Updated`. `MEMORY.md`: exactly one new
   entry (newest-first). Exactly one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_VALMANUAL_REFRESH_R13.md`.
9. **Non-acts.** No reproduction run is executed and no reproduction bundle
   or witness file is created or modified; no lifecycle, stage, promotion, or
   acceptance act; no DEL-10-05, code, schema, test, tool, or `validation/**`
   write; and the §6 validation plan passes.

A successful run edits only the surfaces in §5. It does not close any
DEL-09-04 Remaining bullet, does not close Phase E row E2, and does not
promote any evidence posture.

## 4. Selected Design (D-54 Reasoned Selection) and Bounded Tasks

Where several documentation shapes were defensible, the selection below was
made under D-54/`DEC-087` reasoned selection; the four-lens analysis and the
materially rejected alternatives are recorded in
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/CURRENT_CANDIDATE_RATIONALE.md`.
That fact alone is not a referral condition.

Selected shape:

- **Restructure, do not append-only and do not erase.** The page keeps its
  frontmatter identity (`OPS-VALIDATION-MANUAL-HEADLESS-RUNNER-REPRODUCTION`,
  `draft_evidence`) and gains a clear two-part body: (a) the frozen E1
  tp_runner_015 procedure, byte-preserved in substance, with the dated
  historical note on case 3; (b) a new bound-path section documenting the
  five del1005 cases, their generator, commands, expected exits, diagnostics,
  per-case counts, whole-suite default, and fail-closed semantics. Stale
  normative-looking rows are corrected in place rather than left beside a
  contradicting note.
- **Expected values are quoted from committed witnesses only.** The refresh
  describes landed behavior; it defines nothing. Exit-code expectations
  derive from the committed diagnostics plus the DEC-065 exit policy and the
  preserved R12 evidence.
- **The rerun consequence is stated on the page**, so the next reproduction
  run cannot inherit a stale expectation.

Bounded tasks for the later executor (N3):

### 4.1 Freeze the execution basis

- Begin on the integration checkout; record the base commit before any
  durable write; verify the tree is clean apart from this run's lawful
  pre-existing state (the R13 AgentRuns directory and this brief).
- Record the adopted brief commit or content hash in the run record.
- Stop if the DAG pointer, DEL-09-04 lifecycle/Remaining text, the runner
  bin/binding sources, the del1005 witness family, or
  `software-workflow.json` has changed materially since this brief. Do not
  silently reinterpret scope.

### 4.2 Refresh the page

Edit `docs/validation_manual/headless_runner_reproduction.md` to satisfy
§3.1–§3.7, including: correcting the Fixture Set table and Reproduction
Procedure so the frozen E1 case 3 row is labeled historical with the dated
note; adding the bound-path section (fixtures, generator command, runner
commands with `--input`/`--output` shape, expected exits 0/0/0/1/1, expected
diagnostics, per-case counts, whole-suite default, blocked-case fail-closed
description); listing the five committed generated witnesses; refreshing the
Remaining E2 Work paragraph; and adding the rerun-trigger consequence
statement. Keep the Review Checks section truthful (the cargo test and
contract-test commands remain valid post-#287).

### 4.3 Update DEL-09-04 state

Apply the §3.8 `_STATUS.md`, `MEMORY.md`, and run-record writes. The struck
dependency is recorded as landed (DEL-10-05 / PR #287 / Receipt-59), not
deleted from history.

### 4.4 Validate and close out

Run the §6 plan in order, each failure halting closeout. On success only:
append one versioned minimal receipt to `loop/LOOP_RECEIPTS.md`, rerun the
receipt validator, and return with enumerated refutable claims. On failure or
block: leave `_STATUS.md`/`MEMORY.md` unchanged if the failure precedes their
edit (or record the truthful partial state if it does not), record truthful
failed/blocked evidence in the run record, and return to HELP_HUMAN.

## 5. Exact Write Fence for the Later Execution

While the adoption effect is held: no execution writes are authorized.

After the adoption chain becomes effective, durable writes are limited to
(paths relative to `WORKING_ROOT` unless noted):

1. this candidate brief, only for the governed standing-approval /
   classification / activation status record or a later superseding
   hold/rejection record;
2. `docs/validation_manual/headless_runner_reproduction.md`;
3. DEL-09-04 only (under
   `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/`):
   `_STATUS.md`, `MEMORY.md`, and one new
   `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_VALMANUAL_REFRESH_R13.md`;
4. the parent managed-run directory
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/**`;
5. `loop/LOOP_RECEIPTS.md`, append-only, for one new versioned minimal
   receipt.

**Profile determination (no sweep artifact).** This tranche writes only
`docs/**`, `execution/**`, and `loop/**` paths. Under the live
`software-workflow.json` path rules, those paths select `harness-pytest`
(rule: `execution/**`, `docs/**`, `AGENTS.md`, `loop/**`) plus the always-on
`harness-self-check`. The `piping-pytest` and `evidence-sweep` checks are
selected only by `core/**`, `validation/**`, or `tests/**` writes, none of
which this fence permits; therefore no evidence-sweep run is required, no new
`validation/evidence/sweeps/SWEEP_*.json` is authorized, and the fence
contains no `validation/**` path — which in turn keeps the determination
self-consistent. The DEC-025 five-surface sweep remains the pre-push gate for
code-touching branches; this tranche touches no code.

Ephemeral writes are limited to scratch logs and, if the optional §6 offline
spot-run is exercised, task-local build/output directories outside durable
project paths.

No other project file is writable. In particular, do not write or fix:

- `docs/validation_manual/index.md`, `docs/validation_manual/cases/**`, or
  any other `docs/**` file;
- any `validation/**` path: no witness input/generated file, no
  reproduction bundle, no sweep artifact, no benchmark crate file;
- any `core/**`, `schemas/**`, `tests/**`, or tool file;
- DEL-10-05 or any other deliverable/package folder;
- dependency registers, DAGs, decomposition, decision registers/packets,
  PRD/PLAN, claims registry, or loop workplans;
- root governance, `_DomainEngines/**`, app-dev, or any external path;
- completed immutable evidence bundles under
  `validation/evidence/reproduction/**`.

Local branch/commit metadata is ordinary closeout mechanics after the
adoption chain becomes effective. This brief does not authorize a push, pull,
fetch, PR creation, self-merge, or any other network/external state change.

## 6. Evidence and Validation Plan

Registered checks (per the §5 profile determination), run through the root
tool from `REPO_ROOT`, each as its own halting step, persisting normalized
JSON into the R13 AgentRuns directory:

```text
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check harness-pytest --output <out>
python3 tools/software_workflow/run_registered_checks.py \
  "$WORKING_ROOT/software-workflow.json" --check harness-self-check --output <out>
```

`piping-pytest` and `evidence-sweep` are not selected by this tranche's
changed paths (§5); if the executor's final changed-path set were ever to
include a `core/**`, `validation/**`, or `tests/**` path, that is a fence
violation and a failure, not a trigger to add checks.

Page-assertion verification (mandatory, before closeout): for each command
the refreshed page asserts, cross-check the expected exit code and diagnostic
against the committed del1005 witnesses — parse the five
`validation/witness/generated/del1005_payload_binding_*.json` files and
confirm: the three success witnesses carry `diagnostics: null`/empty with
`suite_run` counts (1/1 mechanics; 3/3 stress; 5/5 nonlinear with
`whole_suite_default_applied: true`) implying exit 0 under the DEC-065 exit
policy; the two payload-missing witnesses carry exactly the blocking codes
`HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` /
`HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING` implying exit 1; and the exit
codes 0/0/0/1/1 match the preserved R12 N3 evidence (N3 RETURN §3 row 6 and
the DEL-10-05 run record). Confirm the historical-note claim by inspecting
the live bin source (stub diagnostic confined to the `ExportResults` arm;
payload-missing arm for `run-benchmark`) and the frozen stub input (no
`benchmark` key). A live offline spot-run of the five documented commands is
permitted but optional; if run, outputs are ephemeral only, no durable write,
and no provisioning (a missing local build prerequisite makes the spot-run
skipped, not blocked, because the committed-witness cross-check is the
mandatory path).

Also require, in sequence, every failure stopping subsequent state-changing
closeout:

- `python3 tools/validation/validate_claims_language.py --repo-root .` from
  `REPO_ROOT`;
- `python3 tools/validation/validate_path_anchors.py . --text` from
  `REPO_ROOT`;
- `git diff --check` from `REPO_ROOT`;
- byte-identity of the seven frozen E1 surfaces (three inputs, generator,
  three generated witnesses): `git status --porcelain` over those paths is
  empty;
- byte-identity of the DEL-09-04 second Remaining bullet (owner-gated
  tolerance) against the pre-edit text;
- changed-path containment against the §5 fence using
  `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
  --base <base-commit> --allowed <each §5 path>` from `REPO_ROOT`, persisting
  its JSON stdout in the R13 run directory;
- `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`
  from `REPO_ROOT`, after the receipt append.

No check may install missing dependencies, use the network, or update a
toolchain. A missing local prerequisite for a mandatory check is a truthful
blocked result, not permission to provision it.

## 7. Defect and Failure Disposition

- Fail closed. Any §3 predicate that does not hold, any §6 check failure, any
  frozen-surface byte drift, any unexpected changed path, or any discovery
  that a page assertion cannot be anchored to committed witness evidence is a
  failure that stops closeout; record truthful evidence and return to
  HELP_HUMAN.
- No scope drift: no reproduction run, no new witness or bundle, no
  threshold/tolerance content (DEC-046 promotion is owner-gated and
  untouched), no MAINTAINER_REVIEWED promotion, no GUI-workflow evidence, no
  lifecycle/stage/release/acceptance act, no DEL-10-05 or code/schema/test
  write.
- A repair need outside the §5 fence — including any defect discovered in the
  del1005 witnesses, the runner behavior, `docs/validation_manual/index.md`,
  or the case pages — is reported and returned to HELP_HUMAN for a new lawful
  selection, not fixed here.
- If the checkout is not clean before execution (beyond this run's lawful
  pre-existing state), stop and return the condition; do not stash, reset, or
  interpret unrelated work.

## 8. Rerun Triggers

A rerun (new execution record, same governed brief unless superseded) is
required when any of these changes after the implementation base commit:

- the runner bin/binding sources, verbs, diagnostics, or exit policy
  (`DEC-065`);
- the del1005 witness family (generator, inputs, or generated witnesses) or
  the frozen E1 surfaces;
- the DEL-09-04 Remaining scope, lifecycle state, or the page itself through
  another lane;
- applicable active DAG-007 rows or the approved DAG pointer;
- `software-workflow.json` or the root software workflow profile;
- a prior failed/blocked result after the underlying condition is resolved.

Consequence carried by this tranche (and stated on the page): the refreshed
page changes the documented reproduction surface, so any subsequent
clean-checkout reproduction executes from a post-#287 source commit under a
fresh run ID and a new immutable bundle per
`CB-2026-07-18-DEL-09-04-CLEAN-REPRO-001` §8; completed bundles remain
truthful for their pinned commits and are never edited.

A material governance change (new decision touching the runner or manual
surface, supersession of the D-52/D-54 lanes) returns the brief itself to
HELP_HUMAN before any rerun.

## 9. Exclusions and Preserved Gates

This brief does not authorize:

- any reproduction run, reproduction acceptance, evidence-posture promotion,
  or new/edited reproduction bundle;
- MAINTAINER_REVIEWED case-page promotion, GUI-workflow validation evidence,
  or closure of any DEL-09-04 Remaining bullet;
- promotion of release thresholds, final tolerance policy, CI gate policy, or
  any DEC-046 tolerance record (owner-gated);
- new tolerance constants, acceptance criteria, or normative content of any
  kind; the page describes landed behavior with witness-committed values
  only;
- the `export-results` binding or any DEL-10-05, code, schema, test, or tool
  work;
- edits to suite fixture values, recorded expected values, policy JSON
  records, READMEs, hand-calcs, the frozen E1 fixtures/generator/witnesses,
  or the del1005 witness family;
- lifecycle transition, stage or milestone advancement, issuance, release,
  packaging, signing, publication, push, PR merge, hosted CI, network or
  external service use, or any external commitment;
- professional approval, certification, sealing, authentication, or
  code-compliance claims.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 10. Owner Adoption by Standing Approval — Attribution and Effect Record

```text
OwnerStandingApproval: DEC-085 / D-52 §2, as prospectively refined by DEC-087 / D-54 §1
AgentClassification: CLASSIFY_ELIGIBLE (N1, `instances/N1/RETURN.md`)
RuleActivation: ACTIVATE_OWNER_STANDING_APPROVAL
ClassifiedBy: ORCHESTRATOR (Agent 1), HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13 / N1
AgentJudgment: SELECT_AND_ADVANCE (D-54 §3.3; selected shape per §4)
SelectedOutcome: refresh the reproduction page to post-#287 bound behavior per §3–§4 within the §5 fence, witness-anchored values only
JudgedBy: ORCHESTRATOR (Agent 1), HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13 / N1
AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL (DEC-085 / D-52 §2, durably SHA-bound at governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47)
OwnerCaseSelection: NONE
RejectedAlternatives: recorded in the rationale artifact (page structure; run-regression documentation scope; same-tranche reproduction rerun; index.md touch; full-bullet closure)
RationaleArtifact: execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13/CURRENT_CANDIDATE_RATIONALE.md
IndependentVerifier: COMMIT-SAFE — `instances/N2/RETURN.md` (20/20 claims confirmed; independent 10-class re-screen pass)
EffectStatus: EFFECTIVE — EXECUTION RELEASED BY HELP_HUMAN FAN-IN DISPOSITION (N2 COMMIT-SAFE)
PreservedGates: reproduction acceptance and evidence-posture promotion; MAINTAINER_REVIEWED case-page promotion; GUI-workflow validation evidence; DEC-046 threshold/tolerance promotion; export-results binding (DEL-10-05); lifecycle/stage/issuance/release/acceptance; prover activation/correlation; publication/external action; merge authority; D-45; D-06b; F-PIP-1..5
```

Adoption is the owner's conditional act under the standing rule; the agent
classifies, selects among defensible documentation shapes under D-54, and
proposes. HELP_HUMAN — not N1 — progresses `EffectStatus` only after the
independent refutation returns `COMMIT-SAFE` and the governed record is
durably bound per K-AUTH-2 / D-GOV-04. No execution is released by this
document in its current state.
