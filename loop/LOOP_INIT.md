# Piping development loop

Resolve `REPO_ROOT` from the active checkout and set `WORKING_ROOT` to
`{REPO_ROOT}/projects/chirality-piping`. Paths below are relative to `WORKING_ROOT`.
Enter through `init/dev-loop-init-prompt.md` with the selected role and the
human's steering. This file owns the recurring development-loop procedure;
project `AGENTS.md` supplies standing responsibilities, boundaries and checks.
The dated `WORKPLAN_2026-07-*` files are historical evidence at their original
paths, not instructions for current work selection. The 2026-09-19 standing
procedure pointer records their supersession; preserve those historical bytes.

**Current work graph:** `execution/_Coordination/WorkGraphs/PIPING_LINTER_SCOPE_20260923/WORK_GRAPH.md`.

## Project pointers

- Purpose and scope: `docs/PRD.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md`,
  and the relevant adopted amendments or design specifications.
- Project dependency basis: `execution/_DAG/_LATEST.md`.
- Deliverables: relevant `execution/PKG-*/1_Working/DEL-*/` folders and their
  accepted production form, MEMORY.md, dependencies and lifecycle _STATUS.md.
  Piping also has ArchitectureBasis and bespoke contracts; use the actual form.
- Decisions and boundaries: project `AGENTS.md`, applicable entries in
  `execution/_Coordination/_DECISIONS/_REGISTER.md`, and relevant notices.
- Checks: `software-workflow.json` and the applicable project verification rules.
- Task Management register: `execution/_Coordination/_TaskManagement/REGISTER.csv`.

## 0. Read the steering and recover the situation

The init steering and subsequent human directions establish the purpose, phase,
priorities and limits. Read the selected graph, relevant deliverable contracts,
MEMORY run pointers, dependencies, implementation and evidence. Give a concise
reading of the intended outcome and proceed where the direction is clear.

Verify branch/worktree state, partial edits, active workers and prior integration
before repeating work. Recover the actual graph and its accepted basis when
continuing an undertaking. Preserve unrelated edits and transfer shared-file or
test-resource ownership explicitly. Missing or stale pointers require recovery
from their sources; they are not permission to restart completed work.

## 1. Construct or revise the work graph

Use `chirality-root:bundled:workflow:construct-local-work-graph` when no graph
exists or its route needs substantial revision. Relate the intended outcome to
the project DAG, deliverables and present work. Before the first DAG exists,
follow the phase steering and applicable dependency/cycle-resolution method.

Create the Git-tracked graph at
`execution/_Coordination/WorkGraphs/<undertaking>/WORK_GRAPH.md` and set this
file's current-work-graph pointer to that actual path. Commit it in the
undertaking's PR sequence early enough for handoff and keep it current across
sessions. Preserve historical graphs at their existing locations.

Plan substantive implementation and evidence through PRs, each carrying the
documentation, reconciliation and conditional Task Management work its slice
needs. Then plan one final bounded documentation/governance closeout stage:
reconciliation, conditional Task Management, one central loop receipt, terse
MEMORY entries and the final PR. Steps 2–6 supply the mechanics; no formal
reconciliation pass is required
after every node.

## 2. Advance implementation and evidence

Use Agent 0/1/2 responsibilities to maintain alignment, manage connected work and
execute bounded contributions. Managers integrate their children's returns and
advance independent ready work. Size concurrency to actual review and integration
capacity, with one owner for shared writes.

Implement, verify, validate where applicable, review, repair and integrate via
PRs under the graph and project requirements. Each PR carries the document,
reconciliation and governance consequences needed for that slice, including a
qualified Task Management transfer when needed under Step 4. Exercise meaningful connected behavior and
record the actual candidate and evidence. Prepare consequential decisions for
the human while unaffected work proceeds. Keep required production work in the
graph until its conditions are satisfied; an intermediate merge does not finish
the undertaking.

## 3. Perform the bounded documentation and governance closeout

After integrating the intended implementation and evidence work—normally the
penultimate merge—perform one planned documentation/governance closeout stage.
Use `chirality-root:bundled:workflow:bounded-reconciliation` for bounded
per-deliverable comparisons as needed within that stage. The closeout precedes
the final PR. Compare the delivered result and evidence with the actual
Scope of Work, dependency and governance records; apply warranted edits and
accepted decisions. Preserve future requirements and owning authority for scope,
lifecycle, protected criteria and issued baselines.

This closeout is bounded to the undertaking. A supported no-change result is
sufficient. If it finds missing required implementation or evidence, return that
work to the graph and repair it before completing closeout. Recheck affected
comparisons after a change; do not declare required work complete through a
report or transfer.

## 4. Route exceptional concerns

For a substantive PR or the final closeout, first resolve a concern through
authorized graph work,
a warranted document amendment, or the owning decision/scope-change route.
Work already allocated to an identified successor stays there. Ordinary future
requirements remain in their governing scope.

Only a material, evidenced concern without a current or identified successor
home qualifies for bounded Task Management intake. Give it to WORKING_ITEMS
selecting `chirality-root:bundled:workflow:task-management`, with the source,
significance, missing home and proposed treatment. Perform that workflow's
federation preflight; no general harvest is required. Promotion, disposition
and external assignment remain actual human acts. Retain the outcome or pending
intake in Task Management and link it from the originating PR/graph node and
final closeout as applicable. Routing does not
satisfy an unmet requirement or permit graph completion that depends on it.

## 5. Write the central receipt and affected MEMORY entries

Near final PR preparation, write one receipt for the undertaking at
`execution/_Coordination/AgentRuns/<RunID>/RECEIPT.md`, using the graph's stable
run identity. It is a concise, derivative account of what landed, affected
deliverables, actual PRs, checks and evidence, decisions or Task Management
transfers, and material limits. Use its result/checks/limits account for the
final PR description, adjusting links for the PR surface. Keep detailed logs at
their sources and link the graph; the receipt is neither a second execution
graph, a future-work list nor decision authority. A graph node or Task
Management invocation does not create another loop receipt.

Then add a terse entry to each affected deliverable's `MEMORY.md`: run/date,
what this run did there, and a link to the central receipt. Add the relevant PR,
decision, scope-change or transfer pointer when useful. MEMORY is a local run
index, not a decision record or future assignment. Preserve existing history.
Include the receipt and MEMORY changes in the final PR; use a stable run/branch
link until its PR URL exists, then bind that URL before final checks. Do not
claim a pending merge as complete. Root SPEC §9.8 still governs required
multi-agent execution provenance.

## 6. Complete the graph and merge the final PR

Complete the graph's promised work, bounded closeout, central receipt and
memory entries, and prepare the final PR with the integrated result and
evidence. Review and required checks must cover the actual final candidate;
resolve blocking findings and
obtain the decisions reserved to the human. Record readiness for final merge and
the PR URL in the candidate graph. Verify the actual merged state afterward
through Git or the PR service; do not assert a future merge SHA in the candidate or require a later completion-record commit.

The loop ends when the completed work graph's final PR merges under standing
Git authority. A review hold, unfinished required node or unmerged final PR
means it remains open. On interruption, retain the candidate, open checks,
active operations and next safe action in the graph. Final integration does not
itself issue a deliverable, accept a product or authorize release.
