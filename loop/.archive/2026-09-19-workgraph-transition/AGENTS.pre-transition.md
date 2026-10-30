---
doc_id: OPS-AGENTS
doc_kind: governance.agent_index
status: active
created: 2026-04-30
revised: 2026-09-19
---

# AGENTS — SWBPIPE Project Instructions

Accepted by the owner on 2026-09-19; these project instructions replace the
previous standing development-loop procedure.
Root `AGENTS.md` and the selected `agents/AGENT_*.md` package govern agent roles
and delegation. Applicable owner directions govern the work; an agent's plan,
handoff or interpretation does not create owner authority.

## Paths and knowledge sources

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`. Set `WORKING_ROOT` to
`{REPO_ROOT}/projects/chirality-piping`. Use these anchors or repository-relative
paths in durable instructions and briefs, not machine-specific absolute paths.

The external engineering corpus `domains/piping-design/` has vetted prose,
concepts and design guidance, but its extracted equation artifacts are
unreviewed `pdf2md`/OCR extractions pending the maintainer's manual review.
Consumers may cite the prose for concepts, terminology and approach. Never
present an extracted equation as authoritative; report the artifact's review
status. Never use those equation artifacts as references for physics-model
builds, solver/kernel work or analytical verification. Use the maintainer's
vetted engineering sources. This preserves DEC-043.

## Discover and organise the work

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

This graph is built from the code and the undertaking. It is not the repository's
deliverable DAG. Record tentative deliverable bindings as unverified; do not
guess a deliverable home for evidence or infer lifecycle completion from merged
code. Reconcile deliverable scope and status when the owner directs that work.
Existing deliverables, decisions and approved dependency snapshots remain valid
in their own domains; changing the planning surface does not amend them.

Discover enough to propose a delegation and model strategy before substantial
implementation or agent launches. Explain what Agent 0 will do, delegated
responsibilities, model/effort choices, concurrent work, write scopes and
integration/review ownership. Use recorded experience where it helps. Obtain
the owner's agreement to the strategy, then proceed within it; revisit material
changes. An already approved applicable strategy satisfies this step. Read-only
discovery, preparation of the proposal and bounded decision packages may proceed
before that agreement. Cost estimates are not required.

## Delegation and evidence

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

Keep model allocation in the approved run strategy and actual execution records,
not these standing instructions (D-GOV-17 M1-D). A model or harness change must
not silently change role, authority or scope. Reuse transcript parsers, launch
scripts and attribution conventions only when they work in the current host.
Preserve their evidence purpose using available tools; attribute commits
truthfully. Do not copy another harness's model or co-author identity.

Parallelise work with clear dependencies and disjoint writes. Give shared files
one integration owner. Serialize access to native/browser state and other shared
test resources when concurrent use would invalidate results. Intervene when
repeated repair attempts yield no new evidence; change the diagnosis or split
the problem instead of repeating an unproductive approach.

## Decisions and boundaries

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

The domain, privacy, claims, lifecycle and scope-gated integration boundaries
in F-PIP-1 through F-PIP-4 remain subject to their owning rulings. Their retained
definitions are in `loop/WORKPLAN_2026-07-18b_piping_loop.md`, under "Standing
constraints — fences"; later owner rulings govern explicit amendments. Replacing
the old loop procedure does not waive those boundaries. It supersedes F-PIP-5's
deliverable-only work-selection procedure with the run graph described above.
Do not edit historical fence definitions or infer release/issuance authority.

Write only inside the authorised scope, including required run evidence. Root
governance, agents, workflows, skills and tools need explicit authority for that
work. Do not alter other projects or stage, revert or repair unrelated state.
For authorised dependency-register work, use the canonical type system and the
approved snapshot named by `execution/_DAG/_LATEST.md`; preserve legacy labels
as provenance rather than re-emitting them as current enums.

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
