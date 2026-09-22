# Piping development loop

Resolve `REPO_ROOT` from the active checkout. `WORKING_ROOT` is
`{REPO_ROOT}/projects/chirality-piping`; paths below are relative to it.
Work as HELP_HUMAN within the owner's assignment and applicable project
instructions. This file locates the work and describes how to continue it.

**Current work graph:** `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/WORK_GRAPH.json`

## Project pointers

- Purpose and scope: `docs/PRD.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md`,
  and applicable amendments or design specifications cited by the graph.
- Phase dependency basis: `execution/_DAG/_LATEST.md`.
- Deliverables: relevant `execution/PKG-*/1_Working/DEL-*/` folders, especially
  `ScopeOfWork.md`, `_STATUS.md`, `MEMORY.md`, and dependency records.
- Decisions and constraints: project `AGENTS.md`, applicable entries in
  `execution/_Coordination/_DECISIONS/_REGISTER.md`, and routed notices.
- Verification: `software-workflow.json`; use the affected engineering,
  native-host and product checks identified by project `AGENTS.md`.

Follow references when they matter to the assignment. Historical plans, receipts
and run folders can supply evidence and help locate unfinished work.

## 0. Orient and recover

Read the current graph and the owner's latest direction. Follow any phase cursor
linked by the graph or its handoff; existing runs retain their selected method,
pinned basis and owner gates until their own adoption. Check its objective,
basis, ready work, holds and recovery information against the actual branch,
working tree and referenced evidence, including unmerged work in named worktrees.
An interrupted node may have produced work after its last graph update; inspect
that work before repeating or claiming it. Preserve unrelated changes.
Before reassigning active work, files or shared test resources, verify that the
prior workers/checks have stopped or explicitly transfer their ownership.

A missing or contradictory graph target needs recovery; it does not mean no
graph exists. If the owner names another graph, establish which undertaking is
being continued and update the pointer when the selection is settled. An owner
pause remains a pause until resumed. Report the next useful work concisely.

Before using a historical receipt as a recovery cursor, run the repository's
`python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`. A failed
cursor cannot be relied on; independent work with a verified basis can continue.

## 1. Construct or revise the local graph

When the pointer is `none`, use `construct-local-work-graph` from the bundled
`chirality-root` library to interpret the init steering and subsequent chat,
clarify material gaps, and select a route through the phase DAG. Develop the
relevant deliverables and current code into sufficiently detailed work scope.
Save its graph, replace the pointer above with the project-relative path, and
begin following it within the agreed scope. If an objective cannot be inferred,
establish it with the owner. A completed graph does not start another phase.

For an existing graph, use the same method when the work needs restructuring.
Keep the graph's identity and useful completed results; update dependencies,
verification and reconciliation work as discoveries require. A graph translates
the phase DAG into executable work; changes to the accepted project basis follow
their owning decision path.

## 2. Organize and advance ready work

Use Agent 0/1/2 responsibilities to coordinate, manage and execute bounded work.
Each assignment has a clear outcome, applicable basis, write scope and checks.
Run independent work concurrently; serialize shared writes and test resources.
Follow work dependencies and existing holds. Bring consequential choices beyond
the assignment to the owner while independent authorized work continues.

## 3. Execute, verify and record the result

Build, investigate, test, review and repair until the node's completion conditions
are supported. Apply project checks and independent review to the actual
candidate. Record what changed, the evidence, limitations and remaining work in
the graph or its linked result. Keep supporting evidence in the undertaking
folder or an appropriate existing artifact; avoid copying it into several logs.
Update graph state at meaningful changes so another session can continue.

## 4. Reconcile bounded results

Traverse planned `bounded-reconciliation` nodes from the bundled
`chirality-root` library as coherent results become available. These assignments
reconcile the actual contents of affected deliverable folders with the code
and evidence: scope descriptions and requirements, Remaining items, dependencies
and supporting references as warranted. Update the named sections within the
assignment, preserving future requirements and unresolved departures.
Implementation and reconciliation completion remain separately visible.

Reconciliation can proceed alongside independent implementation. A dependent
node that needs an updated deliverable record waits for that reconciliation;
unrelated work need not wait. A larger scope or dependency conflict becomes
explicit follow-on work or an owner decision.

## 5. Continue, pause or complete

Continue from the graph while ready authorized work remains. At a meaningful
checkpoint or transfer, refresh its recovery information: source revision and
uncommitted work, outstanding checks, evidence locations, blockers or owner
decisions, and the next safe action. Keep necessary evidence recoverable outside
temporary worktrees. A separate session handoff or receipt is unnecessary when
the graph already contains the continuation facts.

Complete the undertaking only when its completion conditions, including planned
reconciliation, are met or remaining scope has an explicit agreed disposition.
Retain the completed graph and its evidence references. Leave it selected until
another undertaking is chosen; completion is not authority for a new phase,
deliverable issuance or release.
