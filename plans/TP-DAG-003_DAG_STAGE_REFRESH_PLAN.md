---
doc_id: TP-DAG-003-DAG-STAGE-REFRESH-PLAN
doc_kind: tranche.plan
status: implemented_preliminary_resequenced
created: 2026-05-11
tranche: TP-DAG-003
prepared_by: current_implementation_agent
governing_instruction_basis:
  - agents/AGENT_ORCHESTRATOR.md
  - plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md
  - execution/_DAG/DAG-002/APPROVAL_RECORD.md
  - execution/_DAG/DAG-002/PROPOSAL_RECORD.md
target_graph: execution/_DAG/DAG-003/
approval_status: preliminary_not_approval_ready_pending_dependency_surface_refresh
---

# TP-DAG-003 DAG Stage Refresh Plan

## Purpose

Create `DAG-003` as a governed stage refresh of the OpenPipeStress SOFTWARE
deliverable execution DAG. As revised after implementation review, this tranche
is a preliminary carry-forward snapshot from the approved `DAG-002` baseline,
not the final current-state-resolved software-construction DAG.

`DAG-003` remains useful as validated graph packaging and node/status metadata
evidence, but it must not be approved or used for Type 2 dispatch until a
dependency-surface refresh and graph reconciliation tranche runs first.

`DAG-003` is not Type 2 dispatch authority, not a professional acceptance
artifact, and not a lifecycle promotion.

## Follow-On Status

`TP-DAG-004` subsequently prepared the dependency-surface refresh plan and
dispatch matrix required by this resequencing finding:

- `plans/TP-DAG-004_DEPENDENCY_SURFACE_REFRESH_PLAN.md`
- `plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv`

That follow-on preparation does not approve `DAG-003`, does not run
dependency-extract, and does not change the approved graph authority. The next
required action remains execution of the one-deliverable `TASK` +
`dependency-extract` refresh runs, followed by audit and graph reconciliation.

## Resequencing Finding

Implementation review found that the original sequence was too narrow for a
DAG intended to guide creation of working software. The generated `DAG-003`
correctly carries forward the approved `DAG-002` edge set and refreshes node
metadata from `_STATUS.md` and `MEMORY.md`, but it does not first refresh
deliverable-local dependency surfaces.

The correct order for an approval-ready current-state DAG is:

1. Audit existing DEL-local `_DEPENDENCIES.md` and `Dependencies.csv` files.
2. Use DEL-local `_STATUS.md` and `MEMORY.md` as selection and reconciliation
   signals for drift, implemented evidence, deferred scope, and changed
   interfaces.
3. Dispatch canonical `TASK` shell runs with `TaskSkill: dependency-extract`
   for each deliverable whose local dependency surface requires refresh. Each
   run must be one deliverable, with write scope limited to that deliverable's
   `_DEPENDENCIES.md` and `Dependencies.csv`.
4. Audit refreshed local dependency registers with the dependency-audit tooling.
5. Reconcile local dependency evidence against the aggregate `DAG-002` /
   preliminary `DAG-003` edge set, preserving conflicts and candidate rows
   rather than silently promoting them.
6. Generate a new approval-ready graph proposal from the reconciled dependency
   basis.

Therefore the current `DAG-003` artifact set should be treated as a preliminary
baseline for the dependency refresh/reconciliation workflow, not as an
approval-ready graph.

## Instruction Authority

The governing instruction set for this tranche is the coordination/DAG lineage:

- `agents/AGENT_ORCHESTRATOR.md` for project-level coordination artifacts,
  dependency mode rules, candidate-edge labeling, and advisory blocker
  boundaries.
- `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md` for the canonical
  deliverable-level DAG artifact contract, node register fields, edge register
  conventions, graph audit, cycle report, and topological-wave semantics.
- `execution/_DAG/DAG-002/APPROVAL_RECORD.md` for the approved revision `0.5`
  active edge set baseline and candidate-row treatment.
- `execution/_DAG/DAG-002/PROPOSAL_RECORD.md` for the prior proposal packaging
  pattern.

`agents/AGENT_DOMAIN_HYPERGRAPH.md` is not the correct authority for this work:
it governs DOMAIN `CAT`/`KTY` hypergraph snapshots under
`_Aggregation/Hypergraph/`, not the SOFTWARE execution DAG.

`agents/AGENT_TASK.md` is required for the prerequisite dependency-surface
refresh because `skills/dependency-extract` is a canonical `TASK` skill.
`agents/AGENT_DELIVERABLE_TASK.md` is not the right profile for that refresh
unless a later brief also authorizes broader deliverable-local document work.
For dependency surfaces, use the generic `TASK` shell with
`TaskSkill: dependency-extract`, one deliverable per run, and explicit write
scope limited to `_DEPENDENCIES.md` and `Dependencies.csv`.

## Scope

This tranche produced the preliminary carry-forward artifact set by:

1. Creating `execution/_DAG/DAG-003/`.
2. Refreshing `DeliverableNodes.csv` from `docs/_Registers/Deliverables.csv`.
3. Refreshing node lifecycle/status metadata from DEL-local `_STATUS.md` and
   `MEMORY.md`, including TP-RECON-01 reconciled history where present.
4. Carrying forward `execution/_DAG/DAG-002/DependencyEdges.csv` as the active
   edge basis, preserving candidate-edge semantics as non-gating.
5. Generating the standard DAG artifact set:
   - `DeliverableNodes.csv`
   - `DependencyEdges.csv`
   - `dag.json`
   - `DAG_Audit.md`
   - `DAG_Audit.json`
   - `TopologicalWaves.md`
   - `Cycle_Report.md`
   - `PROPOSAL_RECORD.md`
   - `_LATEST.md`
   - `DAG-003_APPROVAL_REVIEW_PACKET.md`
6. Generating advisory blocker/readiness outputs from `DAG-003` active edges
   only and placing them inside `execution/_DAG/DAG-003/`.
7. Updating `execution/_DAG/_LATEST.md` after validation, marking `DAG-003` as
   preliminary and not approval-ready pending dependency-surface refresh.
8. Updating `_Coordination` minimally to point future agents to this plan and
   the `DAG-003` artifact folder.

## Boundaries

This tranche did not:

- create `execution/_DAG/DAG-003/APPROVAL_RECORD.md`;
- approve `DAG-003` or any active edge set;
- promote candidate edges;
- change lifecycle state or mark any deliverable `ISSUED`;
- dispatch Type 2 work;
- run `PREPARATION`;
- edit deliverable-local `MEMORY.md`, `_STATUS.md`, `_DEPENDENCIES.md`, or
  `Dependencies.csv`;
- refresh deliverable-local dependency mirrors;
- claim that DEL-local dependency surfaces are current;
- claim that the current `DAG-003` topology has been resolved from refreshed
  local dependency evidence;
- make schedule, staffing, priority, implementation-readiness, code compliance,
  certification, sealing, or professional acceptance claims.

## Inputs

| Input | Use |
|---|---|
| `docs/_Registers/Deliverables.csv` | Node/register source |
| `execution/_DAG/DAG-002/DependencyEdges.csv` | Starting active/candidate edge basis |
| `execution/_DAG/DAG-002/APPROVAL_RECORD.md` | Approved active edge and candidate boundary |
| `execution/_DAG/DAG-002/PROPOSAL_RECORD.md` | Proposal packaging precedent |
| DEL-local `_STATUS.md` | Lifecycle state refresh |
| DEL-local `MEMORY.md` | Implementation-history metadata signal only |
| DEL-local `_DEPENDENCIES.md` and `Dependencies.csv` | Deferred prerequisite for approval-ready graph resolution |
| `plans/TP-RECON-01_DELIVERABLE_HISTORY_RECONCILIATION_PLAN.md` | History reconciliation source context |
| `tools/validation/validate_dependencies_schema.py` | Edge schema validation |
| `tools/coordination/audit_dag.py` | Graph integrity audit |
| `tools/coordination/build_dev001_blocker_queue.py` | Advisory queue generation inside DAG-003 only |

## Implementation Method

1. Inspect `DAG-002` artifact schemas and current register columns.
2. Build `DAG-003/DeliverableNodes.csv` by joining deliverable register rows
   with discovered deliverable folders and status metadata. Report missing
   folders/status files in `DAG_Audit.md`; do not guess missing facts.
3. Build `DAG-003/DependencyEdges.csv` by carrying forward `DAG-002` edge rows
   with `DAG-003-*` edge IDs and proposal notes. Preserve `ACTIVE`,
   `CANDIDATE`, and retired/proposal statuses exactly as non-promoted graph
   dispositions.
4. Generate `dag.json` from `ACTIVE` edges only for topology and include
   candidate rows as non-gating metadata.
5. Generate `TopologicalWaves.md` from active edges only. Waves are dependency
   order evidence, not schedule, staffing, priority, or readiness.
6. Generate `Cycle_Report.md` for active edges and candidate-layer warnings.
7. Run deterministic schema/audit/JSON checks.
8. Generate an approval review packet that is now explicitly marked deferred
   pending dependency-surface refresh, without creating an approval record.
9. Generate advisory blocker/readiness outputs under `execution/_DAG/DAG-003/`
   only after graph validation passes.
10. Update `execution/_DAG/_LATEST.md` and minimal coordination pointers.

## Diagnostics

`DAG_Audit.md` and `DAG_Audit.json` must report:

- node count and register coverage;
- active, candidate, and non-active edge counts;
- invalid edge endpoints;
- self-dependencies;
- duplicate active directed edges;
- active-edge cycle status;
- candidate-layer SCC warnings;
- deliverable folders missing from expected execution paths;
- status files missing or unparsable;
- explicit statement that `DAG-003` is preliminary and not approval-ready
  pending dependency-surface refresh.

## Acceptance Criteria

The tranche is complete when:

- `plans/TP-DAG-003_DAG_STAGE_REFRESH_PLAN.md` exists and records this boundary;
- `execution/_DAG/DAG-003/` contains the standard artifact set;
- all nodes from `docs/_Registers/Deliverables.csv` are represented;
- all carried-forward active/candidate edge endpoints are valid deliverables;
- candidate rows remain non-gating;
- active edges are acyclic or cycles are explicitly reported;
- advisory blocker/readiness outputs, if generated, live under
  `execution/_DAG/DAG-003/`;
- `execution/_DAG/_LATEST.md` points to `DAG-003` as preliminary and not
  approval-ready pending dependency-surface refresh;
- `_Coordination` updates are limited to pointers and do not recreate broad
  state logs;
- no `DAG-003/APPROVAL_RECORD.md` exists.

## Verification

Run:

```bash
python3 tools/validation/validate_dependencies_schema.py execution/_DAG/DAG-003/DependencyEdges.csv
python3 tools/coordination/audit_dag.py --nodes execution/_DAG/DAG-003/DeliverableNodes.csv --edges execution/_DAG/DAG-003/DependencyEdges.csv --strict
python3 -m json.tool execution/_DAG/DAG-003/dag.json
python3 -m json.tool execution/_DAG/DAG-003/DAG_Audit.json
python3 -m pytest tools/coordination/test_dag_control_plane.py tools/coordination/test_dev001_blocker_queue.py
git diff --check
git status --short
```

Scope check:

```bash
git status --short -- plans/TP-DAG-003_DAG_STAGE_REFRESH_PLAN.md execution/_DAG/DAG-003 execution/_DAG/_LATEST.md execution/_Coordination/_COORDINATION.md execution/_Coordination/NEXT_INSTANCE_PROMPT.md
```

## Assumptions

- `DAG-002` revision `0.5` remains the approved active edge baseline until a
  later dependency-refreshed graph is explicitly accepted.
- `DAG-003` refreshes metadata and proposal packaging; it does not re-author
  topology beyond carrying forward approved/candidate `DAG-002` rows unless a
  validation issue requires a documented diagnostic.
- DEL-local status and memory files are metadata inputs and drift signals, but
  they do not directly mutate topology without dependency extraction and
  reconciliation.
- No deliverable-local mirror refresh was authorized in this tranche.
- Root `_Coordination/` should remain minimal.

## Closeout Template

## Closeout

Closed on 2026-05-11, then resequenced after implementation review.

Generated artifacts:

- `execution/_DAG/DAG-003/DeliverableNodes.csv`
- `execution/_DAG/DAG-003/DependencyEdges.csv`
- `execution/_DAG/DAG-003/dag.json`
- `execution/_DAG/DAG-003/DAG_Audit.md`
- `execution/_DAG/DAG-003/DAG_Audit.json`
- `execution/_DAG/DAG-003/TopologicalWaves.md`
- `execution/_DAG/DAG-003/Cycle_Report.md`
- `execution/_DAG/DAG-003/PROPOSAL_RECORD.md`
- `execution/_DAG/DAG-003/_LATEST.md`
- `execution/_DAG/DAG-003/DAG-003_APPROVAL_REVIEW_PACKET.md`
- `execution/_DAG/DAG-003/DEV-001_BLOCKER_QUEUE.md`
- `execution/_DAG/DAG-003/DEV-001_BLOCKER_QUEUE.csv`
- `execution/_DAG/DAG-003/Node_Metadata_Audit.csv`
- `execution/_DAG/DAG-003/DAG-003_Mermaid.md`

Counts:

- Deliverable nodes: 92
- Packages represented: 17
- Active edges: 859
- Candidate edges: 8
- Retired rows: 1
- Active SCCs: 0
- Active plus candidate SCC warnings: 3
- Topological waves: 15
- Advisory blocker/readiness rows: 92 unblocked, 0 blocked, computed locally
  under `execution/_DAG/DAG-003/` from active edges only. This is advisory over
  carried-forward topology only and is not approval-ready software-construction
  readiness.

Verification passed:

```bash
python3 tools/validation/validate_dependencies_schema.py execution/_DAG/DAG-003/DependencyEdges.csv
python3 tools/coordination/audit_dag.py --nodes execution/_DAG/DAG-003/DeliverableNodes.csv --edges execution/_DAG/DAG-003/DependencyEdges.csv --strict
python3 -m json.tool execution/_DAG/DAG-003/dag.json
python3 -m json.tool execution/_DAG/DAG-003/DAG_Audit.json
python3 -m pytest tools/coordination/test_dag_control_plane.py tools/coordination/test_dev001_blocker_queue.py
```

Coordination updates:

- `execution/_DAG/_LATEST.md` now points to `DAG-003` as a preliminary
  validated artifact that is not approval-ready pending dependency-surface
  refresh.
- `execution/_Coordination/_COORDINATION.md` points to this tranche plan and
  `execution/_DAG/DAG-003/`.
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` remains short and notes
  that `DAG-003` is not approved dispatch authority and should not be approved
  before dependency-surface refresh/reconciliation.

No `execution/_DAG/DAG-003/APPROVAL_RECORD.md` was created. `DAG-003` remains
preliminary and not approval-ready. The next required tranche should refresh
and audit DEL-local dependency surfaces, then reconcile them into a later graph
proposal before any approval request.
