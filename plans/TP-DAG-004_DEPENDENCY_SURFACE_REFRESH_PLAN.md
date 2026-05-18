---
doc_id: TP-DAG-004-DEPENDENCY-SURFACE-REFRESH-PLAN
doc_kind: tranche.plan
status: prepared_pending_dispatch
created: 2026-05-11
tranche: TP-DAG-004
prepared_by: current_implementation_agent
predecessor_tranche: TP-DAG-003
dispatch_matrix: plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv
graph_authority: execution/_DAG/DAG-002/
latest_preliminary_graph: execution/_DAG/DAG-003/
---

# TP-DAG-004 Dependency Surface Refresh Plan

## Purpose

Prepare the dependency-surface refresh/reconciliation tranche required before
any approval request for `DAG-003` or any Type 2 dispatch from a refreshed DAG.

`DAG-003` remains a preliminary carry-forward snapshot. The approved graph
authority remains `execution/_DAG/DAG-002/` revision `0.5` until a later
dependency-refreshed graph is explicitly approved.

## Trigger

`TP-DAG-003` refreshed graph packaging and node metadata from the current
register and DEL-local `_STATUS.md` / `MEMORY.md` signals, but it did not first
refresh DEL-local `_DEPENDENCIES.md` / `Dependencies.csv` surfaces. The
coordination handoff therefore requires a dependency-surface refresh and
reconciliation tranche before graph approval or refreshed-graph dispatch.

## Current Surface Audit

Audit commands run on 2026-05-11:

```bash
python3 tools/coordination/analyze_dep_closure.py execution
tools/evaluation/check_dependency_schema.sh execution
python3 tools/validation/validate_dependencies_schema.py execution/_DAG/DAG-003/DependencyEdges.csv
python3 tools/coordination/audit_dag.py --nodes execution/_DAG/DAG-003/DeliverableNodes.csv --edges execution/_DAG/DAG-003/DependencyEdges.csv --strict
```

Findings:

- `docs/_Registers/Deliverables.csv` and `execution/PKG-*/1_Working/DEL-*`
  currently resolve to 92 deliverables.
- Lifecycle signals are 84 `CHECKING` non-`PKG-00` deliverables and 8
  `SEMANTIC_READY` `PKG-00` architecture-basis deliverables.
- Local dependency coverage is 84 `Dependencies.csv` files, all schema-valid
  v3.1 with 867 total rows, 859 active execution edges, 8 candidate rows, 0
  local anchor rows, 0 orphans, 0 active SCCs, and 0 bidirectional pairs.
- All 84 non-`PKG-00` local surfaces are synchronized `DAG-002` mirrors, not
  dependency-extract evidence refreshed after `TP-RECON-01`.
- All 84 non-`PKG-00` deliverables have `TP-RECON-01` drift signals in
  `_STATUS.md` and/or `MEMORY.md`, with state preserved as `CHECKING`.
- The 8 `PKG-00` deliverables remain architecture-basis context surfaces and
  are exempt from local `Dependencies.csv` mirror dispatch in this tranche.
- `DAG-003/DependencyEdges.csv` remains schema-valid and acyclic for active
  edges, but this does not make it approval-ready.

## Scope

In scope:

- Use `plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv` as the dispatch
  source for 84 one-deliverable refresh runs.
- Dispatch canonical `TASK` in generic shell mode with
  `TaskSkill: dependency-extract`.
- Run exactly one `DeliverableID` per TASK invocation.
- Set `CONSUMER_CONTEXT=RECONCILIATION`.
- Limit writes to the assigned deliverable's `_DEPENDENCIES.md` and
  `Dependencies.csv`.
- Preserve uncertainty, candidate rows, retired rows, conflicts, and unresolved
  target questions as evidence. Do not silently promote them into active graph
  authority.
- After refresh runs complete, audit all refreshed local registers and
  reconcile them against `DAG-002` and preliminary `DAG-003` in a later graph
  proposal.

Out of scope:

- approving `DAG-003`;
- creating `DAG-003/APPROVAL_RECORD.md`;
- dispatching product implementation work;
- changing lifecycle state or marking any deliverable `ISSUED`;
- editing `MEMORY.md`, `_STATUS.md`, source documents, schemas, tests, code,
  aggregate DAG files, blocker queues, or archived coordination files;
- promoting candidate edges;
- making schedule, priority, staffing, release, certification, code-compliance,
  sealing, or professional acceptance claims.

## Dispatch Rules

Each TASK dispatch must receive:

- one `DeliverableID`;
- the parent `PackageID`;
- scope items, objectives, and context envelope from the dispatch matrix;
- applicable invariants from `docs/CONTRACT.md` and
  `docs/IP_AND_DATA_BOUNDARY.md`;
- acceptance criteria from this plan and the assigned deliverable's
  `_CONTEXT.md`;
- explicit write scope limited to `_DEPENDENCIES.md` and `Dependencies.csv`.

Dispatch waves in the matrix are package-grouping convenience only. They are
not authorization to run a multi-deliverable TASK. Each row remains a separate
TASK brief.

## Orchestration and Aggregation Model

This tranche may be executed as a fan-out / fan-in workflow under any parent
agent acting in the orchestration role. The parent does not need to be the
repo-defined `ORCHESTRATOR` persona, but it must preserve the same governance
posture:

- define the governed tranche and current wave;
- launch bounded subagents with exactly one matrix row each;
- give each subagent only the assigned deliverable folder, the governing
  documents, the dependency-extract skill contract, and explicit write scope;
- keep aggregate DAG files, lifecycle files, source documents, and blocker
  queues outside worker write scope;
- collect only compact worker closeout reports: status, changed paths, row
  counts, validation result, warnings, conflicts, and failed-input notes;
- integrate results after each wave before launching the next wave if context,
  tool limits, or conflict volume make full fan-out impractical.

Recommended fan-out cap is 6 to 12 active workers at a time, adjusted to the
runtime limit. Workers should not read each other's deliverable folders or
attempt cross-deliverable reconciliation. Cross-deliverable reasoning belongs
to the parent fan-in step.

After worker closeout, aggregation may be done by the parent session directly
or by a bounded `AGGREGATION` run. Use an `AGGREGATION` agent when the parent
needs a durable immutable rollup under `execution/_Aggregation/`, such as a
refresh coverage index, conflict register, per-wave validation summary, or
source-to-edge provenance index. Direct parent aggregation is sufficient when
the closeout is only a short matrix-status update and deterministic validation
output.

Any aggregation step remains read-only on deliverables and writes only to its
own tool-root snapshot or to a later explicitly authorized coordination
closeout. Aggregation does not approve graph changes, promote candidate edges,
or mutate local dependency registers.

## Canonical TASK Brief

Use this sealed brief shape for each row of the matrix:

```yaml
TaskSkill: dependency-extract
ScopePath: <DeliverablePath>
DeliverableID: <DeliverableID>
PackageID: <PackageID>
SCOPE: <DeliverableID>
RUN_ROOT: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution
DECOMPOSITION_PATH: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md
MODE: UPDATE
STRICTNESS: CONSERVATIVE
CONSUMER_CONTEXT: RECONCILIATION
AllowedWriteTargets:
  - <DeliverablePath>/_DEPENDENCIES.md
  - <DeliverablePath>/Dependencies.csv
ExpectedOutputs:
  - Updated deliverable-local Dependencies.csv
  - Updated deliverable-local _DEPENDENCIES.md
  - TASK run record under the assigned deliverable _run_records directory
```

Custom instructions for every run:

- Read `AGENTS.md`, `docs/CONTRACT.md`, `docs/IP_AND_DATA_BOUNDARY.md`,
  `agents/AGENT_TASK.md`, and `skills/dependency-extract/*`.
- Read only the assigned deliverable folder plus
  `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- Treat existing local `Dependencies.csv` rows as prior evidence from the
  approved `DAG-002` mirror. Preserve conflicts and uncertainty in the local
  run notes; do not change aggregate graph authority.
- Do not edit source docs, `MEMORY.md`, `_STATUS.md`, code, schemas, tests, or
  aggregate DAG artifacts.
- Do not infer protected standards content, private data, engineering default
  values, or professional/code-compliance claims.

## Acceptance Criteria

The tranche is ready for graph reconciliation when:

- all 84 matrix rows have completed or have explicit failed-input records;
- every completed `Dependencies.csv` validates with
  `tools/validation/validate_dependencies_schema.py`;
- every completed `_DEPENDENCIES.md` records defaults, source paths, warnings,
  and run history;
- no writes occurred outside assigned dependency artifacts and TASK run
  records;
- `PKG-00` exemptions remain documented and no `PKG-00` local CSV dispatch is
  invented;
- a post-refresh audit records schema coverage, orphan checks, SCC checks,
  candidate treatment, and unresolved conflicts;
- fan-in aggregation records completed rows, failed rows, changed paths,
  validation status, conflict/candidate notes, and any rows requiring later
  reconciliation;
- a later graph proposal reconciles refreshed local evidence against
  `DAG-002` and preliminary `DAG-003` without silently promoting candidates.

## Verification

After dispatch completion, run:

```bash
python3 tools/coordination/analyze_dep_closure.py execution
tools/evaluation/check_dependency_schema.sh execution
python3 tools/validation/validate_dependencies_schema.py execution/_DAG/DAG-003/DependencyEdges.csv
python3 tools/coordination/audit_dag.py --nodes execution/_DAG/DAG-003/DeliverableNodes.csv --edges execution/_DAG/DAG-003/DependencyEdges.csv --strict
git diff --check
git status --short
```

Scope check:

```bash
git status --short -- plans/TP-DAG-004_DEPENDENCY_SURFACE_REFRESH_PLAN.md plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv execution/_Coordination/_COORDINATION.md execution/_Coordination/NEXT_INSTANCE_PROMPT.md
```

## Handoff

No Type 2 dependency-extract run has been dispatched by this plan preparation.
The next parent session should start from this plan and dispatch matrix, then
run one canonical TASK per matrix row as governed dependency-surface refresh
work.
