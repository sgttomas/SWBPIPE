---
doc_id: OPS-AGENTS
doc_kind: governance.agent_index
status: active
created: 2026-04-30
revised: 2026-09-19
---

# AGENTS — SWBPIPE Project Instructions

Root `AGENTS.md` and the selected `agents/AGENT_*.md` package govern agent roles
and delegation. This file holds Piping-specific constraints. The recurrent
development procedure is `loop/LOOP_INIT.md`; the init prompt enters it.
The owner's handoff prompt or current directions supply session steering.
An agent's plan, handoff or interpretation does not create owner authority.

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

## Project boundaries

The domain, privacy, claims, lifecycle and scope-gated integration boundaries
in F-PIP-1 through F-PIP-4 remain subject to their owning rulings. Their retained
definitions are in `loop/WORKPLAN_2026-07-18b_piping_loop.md`, under "Standing
constraints — fences"; later owner rulings govern explicit amendments. Replacing
the old loop procedure does not waive those boundaries. It supersedes F-PIP-5's
deliverable-only work-selection procedure with the local work graph described in `loop/LOOP_INIT.md`.
Do not edit historical fence definitions or infer release/issuance authority.

Write only inside the authorised scope, including required work evidence. Root
governance, agents, workflows, skills and tools need explicit authority for that
work. Do not alter other projects or stage, revert or repair unrelated state.
For authorised dependency-register work, use the canonical type system and the
approved snapshot named by `execution/_DAG/_LATEST.md`; preserve legacy labels
as provenance rather than re-emitting them as current enums.

## Execution attribution

Keep model allocation in the approved run strategy and actual execution records,
not these standing instructions (D-GOV-17 M1-D). A model or harness change must
not silently change role, authority or scope. Reuse transcript parsers, launch
scripts and attribution conventions only when they work in the current host.
Preserve their evidence purpose using available tools; attribute commits
truthfully. Do not copy another harness's model or co-author identity.

## Work graph continuity and bounded reconciliation

`loop/LOOP_INIT.md` names the current local work graph. This pointer is
navigation state; scope, node state and recovery facts belong in the graph.
Use the graph across sessions, including its pauses and owner decisions.
Changing the pointer does not authorize a new undertaking or lift a hold.

Plan bounded reconciliation within authorized development work: compare a
stable code result and its evidence with the affected deliverable contents,
including ScopeOfWork.md, Remaining items and dependency records. Apply warranted
document updates within the assignment and preserve unresolved differences.
Do not infer that implemented behavior replaces intended requirements. Lifecycle,
accepted scope and formal dependency-basis changes retain their owning decision paths.
Reconciliation completion is distinct from implementation completion.

Keep concise results and recoverable evidence with the undertaking or their
owning deliverables. A graph or linked result can supply the work record;
separate per-session narratives, handoffs and loop receipts are not required
for this development loop. This supersedes its routine receipt-append rule;
other explicitly selected workflows retain their own output contracts.
Existing receipt bytes remain frozen and their validators retain the historical
integrity check; no ledger rewrite or validator change is part of this adoption.
Existing records remain historical evidence. Operational attribution required
by Root remains truthful and available without being repeated in work tables.

Retain bounded launch instructions and returns with their actual context and
source identity as required by Root. A small read-only launch message can be
the brief; no elaborate duplicate brief is required. Preserve the actual role,
model/effort, substitutions, enforcement limits, return and its disposition in
the linked execution evidence. Work tables focus on the resulting work.

## Decisions and shared work

Use judgment within the authorized assignment. Disclose departures from
specification, their rationale and reversal method. An explicit ruling, adopted
requirement or protected criterion requires its owning decision before reversal.
Preserve material owner directions faithfully in the graph or a linked record,
distinguishing proposals and interpretations from actual decisions.

Give concurrent assignments disjoint write scopes or one integration owner.
Serialize shared native/browser resources when concurrency would invalidate
results. Type 2 returns to its parent and does not delegate. Change the diagnosis
when repeated repair attempts provide no new evidence.

## Software checks

Software work uses `software-workflow.json` under the root
`docs/SOFTWARE_WORKFLOW_PROFILE.md` contract. The profile registers checks;
it does not grant authority or replace the review, evidence and owner-held
boundaries below.

Every proposed mergeable slice receives fresh-context independent review of its
complete frozen diff by a reviewer who did not implement it. This includes
instruction and owner-direction changes. Repair actionable findings and backcheck
the correction and its consequences; later candidate changes need review coverage.
Scale the review to what the change can break. A read-only assignment is not
itself a mergeable slice, and same-model review is not model diversity.

Never weaken a protected test, tolerance, oracle or limit to obtain a pass.
Bring a measured conflict with a protected criterion to the owner and block the
affected acceptance or merge while independent work continues.

Preserve the registered DEC-025 evidence sweep and applicable practitioner-
harness, self-check and historical-receipt validation obligations. Run the sweep
on a clean candidate before every merge touching product code; required CI and
review must cover the actual merging revision. Record failed, skipped or
unavailable checks; repeat checks invalidated by later changes. Historical
receipt validation protects existing records and does not require new entries.

Exercise affected user workflows as they become operable alongside code tests,
including relevant editing, review/apply, solving, result inspection, undo/redo,
save/reopen, interruption and recovery paths. Check model/history/result state as
well as visible behavior; exercise equivalent typed routes when their equivalence
is claimed. Use native application evidence for native-host behavior. Record the
candidate, environment, expected and observed outcomes; repair defects and repeat
affected scenarios. An unavailable required native witness remains outstanding;
CI or browser evidence is not a substitute unless the owning criterion allows it.
Agent-driven checks do not lift independent practitioner-usability holds.

For empirical claims, retain the non-secret inputs, candidate identities,
commands, environment/tool versions and raw outputs needed to check or reproduce
the result, with a bounded rerun method and hash references where needed. Scale
evidence to the claim. If required evidence is unavailable, withhold the claim.
Use one canonical copy with references from the graph or deliverable.

Use the host's actual permissions and supported approval mechanism. Record the
exact unavailable command and outstanding verification; denial is not a pass.
Use the `chirality-change` skill for scoped Git closeout under standing authority.
Preserve unrelated and unmerged work, verify upstream/candidate/index, stage only
owned files, and recheck integration changes. Source integration, observed checks,
engineering acceptance, lifecycle issuance and release remain distinct.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
