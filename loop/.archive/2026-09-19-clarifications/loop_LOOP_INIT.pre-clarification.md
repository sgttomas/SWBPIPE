# Piping development — recurrent loop

This file holds the recurrent procedure and discovery guidance. Current work
and status belong in the owning run; priorities and phase transitions come from
the owner's steering. Historical dated `WORKPLAN_*.md` files remain at their
cited paths but are not selected or executed as loop instructions.

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and `WORKING_ROOT` as
`{REPO_ROOT}/projects/chirality-piping`. Read root `AGENTS.md`, the active role
instructions and `{WORKING_ROOT}/AGENTS.md` for project-specific constraints.

## Discovery and steering

Use the owner's assignment to identify the owning run. Read its work graph,
latest handoff, recorded directions and applicable plan/specifications. Inspect
named branch and worktree state, including unmerged work, before proposing new
implementation. When no run is identified, validate `loop/LOOP_RECEIPTS.md` with
`tools/validation/validate_piping_loop_receipts.py --repo-root .`, then use its
latest applicable pointers to discover the current run records and coordination
notices. A receipt is navigation and history, not an instruction to repeat its
completed work. Report the proposed continuation; do not restart historical
work by default. A failed receipt check blocks reliance on that cursor, not
independent work with a verified basis.

Apply the owner's current directions and any per-run steer, including an
owner-supplied handoff prompt, the launcher's steer or the owning run's direction
record. Keep specific priorities, lane and worktree identities,
active handoff paths, pause conditions and next actions in those state or
steering records, not in these recurrent instructions. Distinguish a recorded
owner direction from the previous agent's recommendation.

Return a short orientation and the proposed next bounded work. Apply the
project's discovery, strategy, testing, review and integration requirements.
Continue under existing authority and an applicable approved strategy; bring
the owner only consequential choices or unresolved authority conflicts. A
handoff's procedural habit does not create a new owner gate.

## Stable source pointers

Resolve these paths relative to `WORKING_ROOT` as needed for the assignment:

- Owner decisions: `execution/_Coordination/_DECISIONS/_REGISTER.md` and its
  cited rulings; coordination notices and run records under
  `execution/_Coordination/`.
- Project DAG: the accepted snapshot named by `execution/_DAG/_LATEST.md`.
- Deliverable basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` and relevant
  deliverable specifications, contracts and Remaining entries.
- Product requirements and checks: `docs/PRD.md`, `software-workflow.json`.
- Session continuity: `loop/LOOP_RECEIPTS.md` and the owning run's work graph
  and handoff. Receipts and notices are pointers and evidence, not authority.

## Organise the undertaking

Begin with the owner's assignment, the owning run's work graph, latest handoff,
recorded owner directions and relevant specifications. Verify their material
claims against actual code, branch heads, worktrees and evidence before relying
on them. Inspect named unmerged lanes: work absent from main may already exist
there. Preserve unrelated changes and parked work. Do not recreate existing work.

Give the owner a concise orientation: what exists, what remains, discrepancies
that matter and the next bounded work. A material conflict in authority, scope
or candidate identity blocks the affected action. Report minor documentary
drift and correct it within scope without stopping independent work. Repository
state establishes what exists; it does not overrule the owner's direction.

Maintain a work graph for the undertaking under
`execution/_Coordination/AgentRuns/<RUN_ID>/`. Choose its structure to suit the
work. Record each slice's state, dependencies and blockers, named semantic
changes, departures from specification, deferred work and when it retires, and
evidence pointers. Derive handoffs from this graph. Keep one canonical copy of
evidence in the owning run, with hash references where needed.

The project DAG describes project-wide deliverables and dependencies. Its
structure is expected to remain relatively stable while being rebuilt for each
project phase under owner direction. The owner steers phase transitions; agents
do not infer or initiate them from apparent progress.

The session work graph translates that broader structure into executable work
at the detail needed for the undertaking. It replaces the session workplan;
no separate workplan or dated workplan selector is required. Maintain traceable
links to the current phase's accepted DAG and relevant deliverables, marking
uncertain mappings as provisional. Record newly discovered dependencies,
departures and gaps for the next owner-directed reconciliation. The session
graph does not silently amend the project DAG or close deliverable commitments.

Keep run evidence in its owning run rather than guessing a deliverable home.
Existing deliverables, decisions and approved dependency snapshots remain valid
in their own domains. Reconcile scope and status, and rebuild the project DAG,
when the owner directs that phase work. No DAG rebuild is required at session
entry or merely because a work graph changes.

Discover enough to propose a delegation and model strategy before substantial
implementation or agent launches. Explain what Agent 0 will do, delegated
responsibilities, model/effort choices, concurrent work, write scopes and
integration/review ownership. Use recorded experience where it helps. Obtain
the owner's agreement to the strategy, then proceed within it; revisit material
changes. An already approved applicable strategy satisfies this step. Read-only
discovery, preparation of the proposal and bounded decision packages may proceed
before that agreement. Cost estimates are not required.

## Delegation and coordination

Choose agent types by responsibility and delegation needs, not task complexity
or reasoning level. Use managers when coordinated implementation and repair
cycles benefit from them; dispatch bounded specialists directly where useful.
No package-by-package manager roster or separate Git-closeout role is mandatory.
Keep canonical role instructions at root authoritative rather than recreating a
role hierarchy here.

Seal each delegated brief before launch. It identifies the objective, relevant
context and hashes, parent, role, tools, write scope, constraints, expected
output and checks. The parent launches its children, observes progress and
validates their returns. Retain returns verbatim with hashes and actual model
and reasoning settings, plus findings, substitutions and dispositions. Record
host enforcement limits honestly. Type 2 instances do not delegate.

Parallelise work with clear dependencies and disjoint writes. Give shared files
one integration owner. Serialize access to native/browser state and other shared
test resources when concurrent use would invalidate results. Intervene when
repeated repair attempts yield no new evidence; change the diagnosis or split
the problem instead of repeating an unproductive approach.

## Decisions

Use engineering judgment inside the authorised scope. Established exemplars
and consistency in ontology, epistemology, praxeology and axiology inform that
judgment; they are guides, not mandatory approval gates. Distinguish verified
exemplar behaviour from inference. Present consequential conflicts or choices
requiring owner judgment as a bounded package with evidence and a recommendation.

Disclose departures from specification, their rationale and how to reverse them.
A departure that conflicts with an explicit owner decision or protected criterion
requires an owner decision before it takes effect. Do not manufacture approvals
from silence. Record the owner's words verbatim and distinguish them from agent
interpretations and decisions.

## Build and test through user workflows

Implement bounded slices with explicit observable outcomes. Maintain the work
graph while implementing, testing, fixing and integrating; record discovered
gaps and exclusions instead of implying whole-project completion.

Exercise changed user workflows as soon as they are operable. Use actual
pointer/keyboard interaction, native computer use and suitable automation to
test what users can accomplish, alongside focused code tests. Do not wait for
the final appearance pass to find interaction and state defects.

As connected functionality becomes available, extend a reusable scenario set
covering authoring, selecting, editing, review/apply, solving, inspecting results,
undo/redo, save and reopen. Include interruptions, cancellation, invalid input,
recovery, keyboard use and relevant window sizes. Check the resulting model,
history and result designation as well as visible feedback. Exercise human and
equivalent typed operation routes where semantic equivalence is involved.

Record candidate and environment, actions, expected and observed outcomes,
failures and evidence. Fix defects within authority, add useful regression
coverage and repeat affected scenarios. Repeat connected journeys after the
visual pass. Report unavailable or blocked scenarios explicitly. Scale the
scenario set to the slice; it is not a demand to run every scenario on every
edit. Agent-driven behavioural testing does not substitute for independent
practitioner usability work or lift its holds.

## Reproducible evidence

For empirical claims, retain sufficient non-secret evidence for another
person or agent to check and, where feasible, reproduce the result: input and
candidate identities, relevant tool versions and environment, commands and
arguments, expected and observed outcomes, and the necessary raw outputs.
Keep a bounded rerun method and hash references to canonical evidence. Scale
this record to the claim; ordinary documentation edits do not require a product
qualification package. If required evidence cannot be retained or reproduced,
state the limitation and withhold the unsupported claim. Never preserve secrets
or private user models to satisfy an evidence requirement.

## Protected checks, review and integration

Never weaken a test or move a tolerance, oracle or limit to obtain a pass.
When a protected check and the design disagree, preserve the check, bring the
owner the measured conflict and a recommendation, and block the affected
acceptance or merge. Check delegated changes for this failure mode as well.

Every slice receives independent review of its complete frozen diff before
merge, by a fresh-context reviewer who did not write it. Give the reviewer
the requirements, source diff and evidence, with a brief to seek defects and
unsupported claims. Same-model review must not be described as model diversity.
Fix actionable findings and obtain backchecks before merge. Any subsequent
candidate changes require review coverage; earlier review alone cannot cover
new bytes. Reviews remain independent of the implementer's self-checks.

Software work uses `software-workflow.json` under the root
`docs/SOFTWARE_WORKFLOW_PROFILE.md` contract. Preserve the registered DEC-025
evidence sweep and applicable practitioner-harness, self-check and receipt
validation obligations. Run the evidence sweep on a clean candidate before
every merge touching product code. Bind evidence to the actual candidate and
record skipped, unavailable or failed checks explicitly. Required CI must pass
on the candidate that merges. Never alter a check merely to accommodate this
procedure revision; surface an actual contradiction for resolution.

Use the host capabilities actually available. Where a required command needs
host permission, use the host's supported approval mechanism. Where execution
is unavailable or declined, record the command and outstanding verification.
An execution denial or unavailable tool is not a pass; CI is not a substitute
for a required native witness unless its owning criterion allows that.

Use scoped branches and PRs. Preserve existing unmerged work; check upstream,
candidate identity and the index before integration. Stage only authorised
files and evidence. Standing Git authority applies to branch creation, commit,
push, PR and merge; do not invent a second per-merge permission gate. Integrate
upstream changes without discarding others' work, review the resulting candidate
and rerun checks whose applicability changed. Escalate substantive conflicts
that exceed existing authority. Do not force-push another contributor's branch.

Keep run state, evidence and a concise handoff current at closure or pause.
Existing loop receipts remain historical evidence with their append-only and
validation rules; use a short reference to the run rather than duplicate its
work graph or numerical evidence. Never rewrite old verdicts, returns or records
to make them agree with a new procedure.

A merged slice is not an accepted deliverable. Report observed test results
without claiming usability, conformance, performance or owner acceptance.
Explicit holds remain until lifted by the owner. Source-control closeout does
not establish lifecycle issuance or engineering validation.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
