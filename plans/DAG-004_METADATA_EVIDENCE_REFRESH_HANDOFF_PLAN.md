---
doc_id: PLAN-DAG-004-METADATA-EVIDENCE-REFRESH-HANDOFF
doc_kind: plan.workflow
status: draft_for_next_session_handoff
created: 2026-05-17
target_artifact: DAG-004
basis_scope_change: SCA-003
accepted_revision: "0.6"
execution_boundary: planning_only_no_dag_materialization
---

# DAG-004 Metadata/Evidence Refresh Handoff Plan

## Purpose

Create a bounded handoff for a future session to materialize `DAG-004` as a metadata/evidence reconciliation against accepted `SCA-003` decomposition revision `0.6`.

This plan does not authorize broad dependency redesign. The intended default is to preserve the `DAG-003` node/edge topology and refresh only the DAG artifact metadata, evidence notes, and review packet language that became stale after SCA-003 resolved the MVP local storage profile. If the audit finds actual dependency deltas, the next session must isolate them in a review worklist instead of silently folding them into the graph.

## Correct Agent Routing

Use these roles for the next session:

| Role | Use In This Refresh |
|---|---|
| `ORCHESTRATOR` | Overall coordination owner if the next session is running a project-control pass. It frames the run, confirms the coordination representation, and routes setup-time dependency work when needed. |
| `RECONCILIATION` | Best human-facing owner for dependency governance. It should interpret the DAG-004 request, manage the dependency evidence question, and decide whether Type 2 audit evidence is enough or a CHANGE handoff is needed. |
| `AUDIT_DEP_CLOSURE` | Read-only Type 2 audit agent for cross-deliverable dependency closure over deliverable-local `Dependencies.csv`. Use it if the next session needs fresh closure evidence from local registers. It does not edit DAG files. |
| `CHANGE` | File-state owner for applying approved repo edits, including creation of `execution/_DAG/DAG-004/`. CHANGE should preserve dirty unrelated files and report git state. |
| `TASK` + `dependency-extract` | Only if the audit finds that deliverable-local dependency surfaces are stale or incomplete enough to require re-extraction. This is not part of the default DAG-004 metadata refresh. |

Practical default: the next agent can act as ORCHESTRATOR/RECONCILIATION for planning and evidence review, then perform or hand off CHANGE-style file edits to create the DAG-004 proposal artifact. Do not treat `AUDIT_DEP_CLOSURE` as the writer of DAG artifacts.

## Required Reading List

Before materializing `DAG-004`, read these files intentionally:

### Agent Instructions

- `agents/AGENT_ORCHESTRATOR.md` — dependency tracking modes, setup-time dependency extraction, and coordination boundaries.
- `agents/AGENT_RECONCILIATION.md` — dependency governance, toolbelt rules, and handoff discipline.
- `agents/AGENT_AUDIT_DEP_CLOSURE.md` — read-only dependency closure audit scope and outputs.
- `agents/AGENT_CHANGE.md` — repo file-state ownership, approval gates, and separation from dependency governance.
- `agents/AGENT_SCOPE_CHANGE.md` — SCOPE_CHANGE handoff boundaries; useful background because SCA-003 is the accepted amendment source.

### Current Authority And Registers

- `execution/_Decomposition/SOFTWARE_DECOMP.md` — current decomposition revision `0.6`; especially `DEC-017`, `AB-00-04`, `SOW-050`, `SOW-059`, and `OI-011`.
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ContextBudgetQA.csv`
- `docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md`
- `execution/_ScopeChange/SCA-003_2026-05-17_1658/Handoff_State.md`
- `execution/_ScopeChange/SCA-003_2026-05-17_1658/RUN_SUMMARY.md`
- `plans/SCA-003_DOWNSTREAM_REFRESH_PLAN.md`

### Existing DAG Basis

- `execution/_DAG/_LATEST.md` — confirms `DAG-003` remains approved authority.
- `execution/_DAG/DAG-003/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-003/PROPOSAL_RECORD.md`
- `execution/_DAG/DAG-003/DAG-003_APPROVAL_REVIEW_PACKET.md`
- `execution/_DAG/DAG-003/DeliverableNodes.csv`
- `execution/_DAG/DAG-003/DependencyEdges.csv`
- `execution/_DAG/DAG-003/dag.json`
- `execution/_DAG/DAG-003/DAG_Audit.md`
- `execution/_DAG/DAG-003/DAG_Audit.json`
- `execution/_DAG/DAG-003/Cycle_Report.md`
- `execution/_DAG/DAG-003/TopologicalWaves.md`
- `execution/_DAG/DAG-003/DAG-003_DEPENDENCY_RECONCILIATION.md`
- `execution/_DAG/DAG-003/DAG-003_DEPENDENCY_RECONCILIATION.csv`
- `execution/_DAG/DAG-003/DAG-003_DEPENDENCY_RECONCILIATION_SUMMARY.json`

### Prior DAG Plans And Tools

- `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`
- `plans/TP-DAG-003_DAG_STAGE_REFRESH_PLAN.md`
- `plans/TP-DAG-004_DEPENDENCY_SURFACE_REFRESH_PLAN.md`
- `plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv`
- `tools/validation/validate_dependencies_schema.py`
- `tools/coordination/audit_dag.py`
- `tools/coordination/analyze_dep_closure.py`
- `tools/coordination/build_dev001_blocker_queue.py`
- `tools/evaluation/check_dependency_schema.sh`

## Handoff Prompt For Next Session

Use this prompt to start the next agent session:

```text
You are operating in /Users/ryan/ai-env/projects/chirality/projects/chirality-piping.

Goal: create a bounded DAG-004 metadata/evidence refresh proposal against SCA-003 revision 0.6.

Read first:
- plans/DAG-004_METADATA_EVIDENCE_REFRESH_HANDOFF_PLAN.md
- agents/AGENT_ORCHESTRATOR.md
- agents/AGENT_RECONCILIATION.md
- agents/AGENT_AUDIT_DEP_CLOSURE.md
- agents/AGENT_CHANGE.md
- execution/_Decomposition/SOFTWARE_DECOMP.md
- docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md
- execution/_ScopeChange/SCA-003_2026-05-17_1658/Handoff_State.md
- execution/_ScopeChange/SCA-003_2026-05-17_1658/RUN_SUMMARY.md
- execution/_DAG/_LATEST.md
- execution/_DAG/DAG-003/APPROVAL_RECORD.md
- execution/_DAG/DAG-003/PROPOSAL_RECORD.md
- execution/_DAG/DAG-003/DAG-003_APPROVAL_REVIEW_PACKET.md
- execution/_DAG/DAG-003/DeliverableNodes.csv
- execution/_DAG/DAG-003/DependencyEdges.csv
- execution/_DAG/DAG-003/DAG_Audit.md
- execution/_DAG/DAG-003/DAG_Audit.json

Boundaries:
- DAG-003 remains approved authority until explicit later human approval.
- Do not update execution/_DAG/_LATEST.md to make DAG-004 authoritative.
- Do not create an approval record claiming DAG-004 is approved.
- Do not bulk-edit historical DAG-001/DAG-002/DAG-003 records.
- Do not redesign the dependency graph unless audit evidence finds actual dependency deltas.
- If dependency deltas are found, isolate them in the DAG-004 review packet as proposed edge-delta items requiring approval.
- Preserve unrelated dirty files; report them and leave them alone.

Default implementation:
1. Inventory DAG-003 against SCA-003 revision 0.6 and current registers.
2. Create execution/_DAG/DAG-004/ using the DAG-003 artifact shape.
3. Refresh DAG artifact names, headings, paths, doc_ids, and metadata from DAG-003 to DAG-004.
4. Update basis language from revision 0.5-dependency-refreshed to SCA-003 revision 0.6.
5. Replace current DAG evidence language that says the physical storage/container remains unresolved with SCA-003 wording:
   SQLite local project store/index substrate; canonical JSON/JCS remains truth; sidecars are rebuildable/non-authoritative; no hosted DB, daemon, required network, cloud sync, telemetry path, or direct plugin/adapter SQL.
6. Keep remaining TBDs that SCA-003 did not resolve, including migration implementation, release packaging/signing, encryption/key management, final CLI syntax, cloud exception workflow, and external adapter formats.
7. Validate the generated DAG-004 artifact with schema/audit/json/stale-text checks.

Validation commands:
python3 tools/validation/validate_dependencies_schema.py execution/_DAG/DAG-004/DependencyEdges.csv
python3 tools/coordination/audit_dag.py --nodes execution/_DAG/DAG-004/DeliverableNodes.csv --edges execution/_DAG/DAG-004/DependencyEdges.csv --strict
python3 -m json.tool execution/_DAG/DAG-004/dag.json
python3 -m json.tool execution/_DAG/DAG-004/DAG_Audit.json
python3 -m pytest -q tools/coordination/test_dag_control_plane.py tools/coordination/test_dev001_blocker_queue.py
rg -n "exact storage mechanism remains TBD|physical project package/container remains TBD|physical container and migration framework remain TBD|0.5-dependency-refreshed" execution/_DAG/DAG-004
git diff --check -- execution/_DAG/DAG-004
git status --short

Final response must report files changed, whether DAG topology was preserved, validation results, any proposed edge deltas, remaining intentionally deferred TBDs, and current git status.
```

## Execution Outline For DAG-004

1. **Inventory and classify.**
   - Compare `DAG-003` nodes/edges against current registers and `SOFTWARE_DECOMP.md` revision `0.6`.
   - Scan DAG-003 current evidence for SCA-003-sensitive stale language.
   - Classify each finding as metadata/evidence refresh, actual dependency delta, or historical/non-current text.

2. **Create proposal artifact.**
   - Create `execution/_DAG/DAG-004/` from the DAG-003 artifact structure.
   - Preserve the standard DAG files: nodes, edges, machine graph, audit reports, topological waves, cycle report, reconciliation files, blocker queue, Mermaid view, proposal record, and review packet.
   - Rename DAG-specific filenames and headings from `DAG-003` to `DAG-004`.

3. **Refresh basis and evidence.**
   - Update DAG-004 metadata to `SCA-003` revision `0.6`.
   - Cite the SCA-003 authority files listed above.
   - Refresh storage evidence for affected surfaces including `DEL-00-04`, `DEL-02-04`, `DEL-02-05`, `DEL-10-01`, `DEL-10-02`, `DEL-10-04`, `DEL-10-05`, `DEL-12-01`, `DEL-12-05`, `DEL-14-01`, `DEL-14-02`, and storage-relevant `DEL-16-*` rows.

4. **Audit topology.**
   - If node/edge counts and classifications remain unchanged, state that DAG-004 preserves DAG-003 topology.
   - If actual dependency changes appear necessary, do not silently edit the edge set. Add a review-packet section listing proposed edge deltas and required approval.

5. **Prepare review boundary.**
   - `DAG-004` should be `proposed_pending_review`.
   - It should not include an approval record claiming acceptance.
   - It should not authorize Type 2 dispatch, lifecycle promotion, candidate promotion, or professional approval.

## Acceptance Criteria

- `execution/_DAG/DAG-004/` exists as a proposal/review artifact when the next session executes this plan.
- `DAG-003` remains the approved authority until later explicit approval.
- DAG-004 current evidence no longer says the physical project storage/container remains unresolved where SCA-003 resolved it.
- DAG-004 explicitly cites SCA-003 revision `0.6` and the SCA-003 authority files.
- Graph topology is preserved unless an actual dependency delta is found and isolated for review.
- Validation commands pass, or failures are reported with concrete file/row evidence.

## Non-Goals

- No broad edge redesign.
- No Type 2 implementation dispatch.
- No lifecycle promotion.
- No approval record for DAG-004 unless separately authorized by the human.
- No update to `execution/_DAG/_LATEST.md` that would make DAG-004 authoritative.
- No bulk edits to historical DAG records or SCA snapshots.
