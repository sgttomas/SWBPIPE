---
doc_id: OPS-AGENTIC-DEVELOPMENT-WORKFLOW
doc_kind: guide.agentic_workflow
status: draft
created: 2026-06-07
deliverable_id: DEL-11-05
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: governed_by
    to: OPS-TYPES
  - rel: explains
    to: OPS-AGENTS
  - rel: explains
    to: OPS-COORDINATION
---

# Agentic Development Workflow

This document is a contributor workflow map for OpenPipeStress. It explains
how to move through existing authority surfaces; it does not replace
`AGENTS.md`, `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/TYPES.md`,
`docs/IP_AND_DATA_BOUNDARY.md`, `execution/_Coordination/_COORDINATION.md`,
approved DAG artifacts, deliverable-local lifecycle files, review records, or
human project-authority decisions.

If this map conflicts with a governing document, an assigned sealed brief, an
approved DAG record, a deliverable-local `_STATUS.md`, a review disposition, or
a human project-authority instruction, stop and surface the conflict.

This workflow is project guidance only. It is not legal advice, professional
engineering approval, certification, sealing, standards-body endorsement,
release authorization, lifecycle approval, or a code-compliance determination.

## Authority Map

| Surface | What it owns | Boundary |
|---|---|---|
| `docs/DIRECTIVE.md` | Founding intent, product boundaries, and stop rules. | Does not fill unresolved implementation or governance decisions. |
| `docs/CONTRACT.md` | Invariant catalog for IP, data, units, solver, rule, report, privacy, governance, and agent behavior. | Invariants become enforceable through downstream schemas, tests, review, and human gates. |
| `docs/TYPES.md` | Stable IDs, package/deliverable hierarchy, lifecycle vocabulary, epistemic labels, and professional-boundary terms. | Lifecycle status is not a professional engineering status. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data, protected-content, provenance, and quarantine policy. | Human/legal review remains required when rights are uncertain. |
| `AGENTS.md` | Project mapping of Chirality roles to OpenPipeStress work. | It does not redefine the canonical agent framework. |
| `execution/_Coordination/_COORDINATION.md` | Current entry protocol, authority intake, work-selection loop, bounded execution discipline, validation, and handoff rules. | Handoff prose is not substitute authority. |
| `execution/_DAG/_LATEST.md` and approved DAG records | Active dependency graph authority and approved edge context. | DAG approval does not dispatch Type 2 work, change lifecycle state, promote candidates, or create release/professional claims. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` and `docs/_Registers/*.csv` | Package, deliverable, scope, objective, and context-budget truth. | Do not renumber, reinterpret, or silently expand scope. |
| Deliverable-local `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, and `_run_records/**` | Selected deliverable context, lifecycle state, references, dependency evidence, working memory, and execution evidence. | Local records do not supersede decomposition, DAG, review, or human approval authority. |
| Human project authority and formal review/change records | Binding decisions, lifecycle gates, acceptance, release decisions, and scope amendments. | Software, agents, and deterministic tools provide evidence only. |

## Role Map

| Role | Function | Authority |
|---|---|---|
| Human project authority | Resolves ambiguity, accepts gates, records binding governance decisions, and authorizes lifecycle/release/scope changes. | Binding when recorded in the proper authority surface. |
| Type 1 agents such as `SOFTWARE_DECOMP`, `WORKING_ITEMS`, `REVIEW`, `RECONCILIATION`, and `CHANGE` | Decompose, route, reconcile, review, and manage approved change surfaces. | Draft/proposal authority unless a human gate records acceptance. |
| Type 2 agents such as `TASK`, `PREPARATION`, and `AUDIT_*` | Execute one sealed deliverable or bounded check with explicit scope. | Draft/proposal authority only. |
| Deterministic tools | Validate schemas, tests, dependency records, links, scans, hashes, and status discovery. | Evidence only; tools do not judge acceptance. |

`TASK` is the generic bounded-task shell. It normalizes the effective brief,
honors explicit write authorization, loads a declared skill only when one
exists, executes inside the allowed boundary, and writes a durable run record
under the selected deliverable's `_run_records/`. `ScopePath`,
`DeliverablePath`, and profile labels do not grant write authority by
themselves.

## Phase-Aware Coordination

Agentic coordination should change when the dominant project uncertainty
changes. Deliverables, DAG records, review states, application code, and
issuance records are not interchangeable work surfaces. Each phase below names
the normal work unit and the question the coordination loop is trying to
answer.

| Phase | Dominant question | Normal work unit | Coordination emphasis |
|---|---|---|---|
| 1. Intent externalization | What are we trying to define? | Package or deliverable scaffold. | Authority intake, deliverable identity, scope boundaries, acceptance criteria, and open questions. |
| 2. Dependency resolution | What depends on what? | Dependency edge, blocker, or clarifying deliverable note. | DAG semantics, cycle checks, edge amendments, blockers, and graph validation. |
| 3. Deliverable maturation | Is this deliverable sufficiently defined and evidenced? | One deliverable or a small dependency-linked set. | Local memory, issue burn-down, acceptance evidence, validation, and bounded improvement. |
| 4. `CHECKING` transition | Is this ready for formal checking? | Review candidate. | Review packet completeness, residual issue policy, status update authority, and audit trail. |
| 5. Application integration | What bounded app capability should become real next? | App slice spanning UI, core, fixtures, tests, and evidence as needed. | Working desktop application, app/code/test intake, cross-deliverable context, fan-in, and preview boundary. |
| 6. App feedback and gap closure | What does the application reveal that documents did not? | App-discovered gap or failed workflow. | Smoke evidence, gap classification, routing to code/schema/fixture/review, and before/after validation. |
| 7. Issuance | Is this deliverable formally accepted as authoritative? | Human-approved issuance candidate. | Final evidence, immutable records, human gate, frozen `ISSUED` authority, and formal change path. |

The stable heuristic is:

```text
Externalize intent as deliverables.
Resolve dependencies into a usable graph.
Mature deliverables until they are reviewable.
Move mature deliverables into CHECKING.
Shift ordinary work from deliverables to product integration.
Use the working application to expose remaining gaps.
Issue deliverables through a separate human-gated lifecycle path.
```

The important transition is from deliverable production to product
integration. Mature deliverables define intended behavior, but the working
application tests whether those intentions compose. Once the DAG is coherent
and deliverables are in `CHECKING`, ordinary work should default to bounded app
slices. Formal issuance remains separate from ordinary development.

Every agent instance must be able to operate from its local instructions,
authority surfaces, current state, write scope, validation expectations, and
handoff rules. It must not require awareness of other instantiations, prior
sessions, or higher-level recursion.

## Coordination Surface Specs

Use these specs when changing the development loop. In this repository,
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md` currently performs the
`NEXT_SESSION_PROMPT.md` role. A separate `NEXT_INSTANCE_STATE.md` is a
possible future split for compact mutable handoff state, but it must not become
substitute authority unless the coordination record explicitly adopts it.

### Phase 1: Intent Externalization

`_COORDINATION.md` should define project authority, package and deliverable
naming rules, allowed scaffolding write scopes, `TASK` brief requirements, and
the loop for selecting missing or underspecified deliverables. It should require
purpose, scope, assumptions, acceptance criteria, and open questions for each
deliverable.

`NEXT_INSTANCE_STATE.md`, if used, should record phase
`INTENT_EXTERNALIZATION`, packages and deliverables created or missing,
incomplete contexts, unresolved project assumptions, the last scaffold check,
and dirty git state.

`NEXT_SESSION_PROMPT.md` should instruct the next session to read authority,
inspect the deliverable register and missing contexts, select one bounded
scaffold or enrichment tranche, and avoid dependency closure unless explicitly
requested.

The init prompt should remain minimal and end with a phase clause like:

```text
Consider one bounded deliverable-scaffolding or intent-clarification tranche.
Spawn TASK agents only for independent deliverables with explicit briefs.
```

### Phase 2: Dependency Resolution

`_COORDINATION.md` should define dependency relation semantics, the source of
truth for dependency records, edge amendment rules, cycle detection, blocker
handling, and the criteria for a usable DAG.

`NEXT_INSTANCE_STATE.md`, if used, should record phase
`DEPENDENCY_RESOLUTION`, the active DAG identifier, known cycles, missing or
ambiguous edges, deliverables blocked by dependency issues, the last DAG
validation result, and open human questions.

`NEXT_SESSION_PROMPT.md` should instruct the next session to read the
deliverable register, dependency records, and DAG artifacts, run dependency
validation, select one bounded dependency-resolution tranche, and avoid
maturing deliverables beyond what is needed for graph closure.

The init prompt phase clause should be:

```text
Consider one bounded dependency-resolution tranche. Spawn TASK agents only for
separable edge-analysis or deliverable-clarification briefs.
```

### Phase 3: Deliverable Maturation

`_COORDINATION.md` should define maturity criteria, local memory and run-record
expectations, acceptance-evidence requirements, DAG-guided work selection, and
rules for reducing blockers and `TBD`s.

`NEXT_INSTANCE_STATE.md`, if used, should record phase
`DELIVERABLE_MATURATION`, the active DAG pointer, ready and blocked
deliverables, deliverables with high `TBD` or open-issue burden, the last
completed tranche, and validation evidence.

`NEXT_SESSION_PROMPT.md` should instruct the next session to run status and DAG
discovery, select one bounded deliverable-maturation tranche, prefer
deliverables whose dependencies are mature enough, spawn `TASK` agents only for
independent deliverable-local improvements, and record evidence and residual
blockers.

The init prompt phase clause should be:

```text
Consider one bounded deliverable-maturation tranche. Spawn TASK agents only for
explicit deliverable-local scopes with clear acceptance criteria.
```

### Phase 4: CHECKING Transition

`_COORDINATION.md` should define `CHECKING` entry criteria, review packet
requirements, allowed residual issues, status-update authority, and review
evidence expectations.

`NEXT_INSTANCE_STATE.md`, if used, should record phase
`CHECKING_TRANSITION`, deliverables eligible for `CHECKING`, deliverables
already in `CHECKING`, rejected candidates needing remediation, last review
results, and current status counts.

`NEXT_SESSION_PROMPT.md` should instruct the next session to run status
summary, select review candidates, inspect deliverable-local context, memory,
evidence, and dependency status, change status only when criteria are met, and
record review rationale.

The init prompt phase clause should be:

```text
Consider one bounded review tranche to move mature deliverables toward
CHECKING. Spawn TASK agents only for separable review or remediation briefs.
```

### Phase 5: Application Integration

`_COORDINATION.md` should define application integration as the default loop,
name app/code/test/fixture/build surfaces, state when cross-deliverable work is
allowed, define the technical-preview boundary, and name validation commands
for app tranches.

`NEXT_INSTANCE_STATE.md`, if used, should record phase
`APPLICATION_INTEGRATION`, current app status, working or failing build and
test commands, active app gaps, relevant `CHECKING` deliverables used as
design context, known `ISSUED` deliverables, dirty git state, and any active
tranche.

`NEXT_SESSION_PROMPT.md` should instruct the next session to inspect
app/build/test surfaces first, use the DAG only to discover related
deliverable context, select one bounded app-integration tranche, spawn `TASK`
agents only for separable subscopes with disjoint write scopes, and preserve
the technical-preview limits.

The init prompt phase clause should be:

```text
Consider one bounded app-integration tranche toward the working application.
Spawn TASK agents only for separable subscopes with explicit briefs and
disjoint write scopes.
```

### Phase 6: App Feedback And Gap Closure

`_COORDINATION.md` should define the app-driven feedback loop, smoke-test and
workflow evidence, gap categories, routing rules for each gap type, and the
conditions under which an app finding reopens deliverable work.

`NEXT_INSTANCE_STATE.md`, if used, should record phase
`APP_FEEDBACK_AND_GAP_CLOSURE`, last smoke and test results, known
app-discovered gaps, gap classification, fixes in progress, deliverables
affected by app findings, and validation commands still failing.

`NEXT_SESSION_PROMPT.md` should instruct the next session to run or inspect
latest app validation, select one bounded gap-closure tranche, prefer fixes
that unblock visible workflows, update deliverable review artifacts only when
the app reveals a real deliverable issue, and record before/after evidence.

The init prompt phase clause should be:

```text
Consider one bounded app-feedback or gap-closure tranche. Spawn TASK agents
only when the gap can be split into independent code, fixture, schema, or
review scopes.
```

### Phase 7: Issuance

`_COORDINATION.md` should define `ISSUED` criteria, the human approval gate,
immutable evidence requirements, the change path for already-issued
deliverables, final review and closure records, and boundary language for
release, professional, legal, certification, and code-compliance claims.

`NEXT_INSTANCE_STATE.md`, if used, should record phase `ISSUANCE`,
deliverables in `CHECKING`, deliverables eligible for `ISSUED`, deliverables
already `ISSUED`, human approvals received or pending, final blockers, and the
last issuance decision record.

`NEXT_SESSION_PROMPT.md` should instruct the next session not to issue anything
without human approval, inspect final review packets and evidence, confirm
dependency closure and app-related findings, update status to `ISSUED` only
through the approved lifecycle path, and treat `ISSUED` deliverables as frozen
authority unless a formal change path opens them.

The init prompt phase clause should be:

```text
Consider one human-approved issuance tranche. Do not select ISSUED
deliverables for ordinary work; reopen them only through the formal change
path.
```

## Default Development Loop

| Step | Action | Evidence |
|---|---|---|
| 1 | Perform authority intake from the coordination record, directive, contract, types, IP/data policy, approved DAG pointer, approved DAG record, and relevant decomposition/register rows. | Notes in the run record or final handoff. |
| 2 | Confirm the assigned `DeliverableID`, `PackageID`, scope items, objectives, acceptance criteria, and allowed write scope. | Sealed brief and deliverable `_CONTEXT.md`. |
| 3 | Inspect the current worktree and the deliverable-local packet before editing. Treat unrelated changes as other-worker or user work. | `git status --short`; local file reads. |
| 4 | Use DAG context only to discover relevant upstream/downstream context. Inspect local artifacts where needed. | Approved DAG edge context plus cited local files. |
| 5 | Execute only the bounded task. Keep unknowns, unsettled policy decisions, missing engineering values, and unsupported source claims as `TBD`. | Changed files inside the allowed write scope. |
| 6 | Run focused validation appropriate to the changed surface. | Commands, pass/fail/deferred results, and warnings. |
| 7 | Update deliverable-local evidence surfaces explicitly authorized by the brief. | `MEMORY.md`, `_run_records/**`, or other approved evidence files. |
| 8 | Hand off for review or human decision without changing lifecycle state unless that gate was explicitly authorized; for completed validated development tranches, autonomously hand off to `CHANGE` for final Git/file-state review, scoped commit, and push. | Changed paths, validation results, data-boundary notes, warnings, open `TBD`s, and `CHANGE` closeout result. |

## Bounded TASK Checklist

Every Type 2 execution needs an explicit work contract:

- one `DeliverableID` and parent `PackageID`;
- scope items and objectives from the registers or selected deliverable packet;
- applicable invariants from `docs/CONTRACT.md`;
- acceptance criteria from the sealed brief or `_CONTEXT.md`;
- explicit allowed write targets;
- exclusions and human-gated decisions that must not be touched;
- validation commands or evidence expectations.

When `ApplyEdits: true` and `AllowedWriteTargets` are present, non-run-record
writes stay inside those targets. The run record is always written under
`{ScopePath}/_run_records/`. If the write boundary is ambiguous, stop and
surface the ambiguity rather than broadening the task.

## Data And Claim Boundaries

Public work may include open mechanics, schemas, blank templates, workflow
docs, invented examples, original examples, and permissively licensed material
with documented provenance. Public work must not include protected standards
text, tables, figures, examples, copied code-derived formulas, proprietary
vendor data without rights, private project models, owner standards, private
rule packs, company design bases, credentials, or real secrets.

Maintainer acceptance of a repository contribution is project governance only.
It is not professional engineering approval of a piping calculation. Software
and agents do not certify, seal, authenticate, approve, or declare engineering
code compliance for reliance.

The project license has been selected as `PolyForm-Noncommercial-1.0.0`.
The final contributor legal mechanism, maintainer roster/quorum, release
authority, legal-review authority, security contact, release-label vocabulary,
human-acceptance workflow, jurisdiction-specific professional-practice wording,
CI provider/coverage thresholds, and exact dependency versions remain `TBD`
unless recorded by the human project authority or qualified reviewer in the
proper governance surface.

## Stop And Route

Stop and escalate through the assigned review, `CHANGE`, `SOFTWARE_DECOMP`, or
human project-authority path when:

- the requested work crosses package or deliverable boundaries;
- a source appears protected, proprietary, private, or unclear for public use;
- a required engineering value, source citation, governance decision, or legal
  conclusion is missing;
- two sources conflict on scope, authority, or safety-relevant meaning;
- a change would imply release readiness, lifecycle transition, professional
  reliance, code compliance, certification, sealing, authentication, or
  standards-body endorsement;
- validation cannot be run or fails and the brief does not authorize a
  deferral.

## Handoff Minimum

When no narrower template is provided, close a bounded run with:

```text
Deliverable: DEL-XX-YY / PKG-XX
Changed paths:
- path

Commands and results:
- command -> result

Boundary review:
- protected-content/proprietary/private-data/secret scan result
- professional/legal/release/lifecycle claim review result

Warnings and open TBDs:
- item or none
```

The handoff is evidence for review. It does not change decomposition truth,
DAG authority, lifecycle state, release status, maintainer authority, legal
policy, or engineering reliance status by itself.

For completed validated development tranches, the handoff must include or be
followed by an autonomous `CHANGE` agent/subagent closeout. `CHANGE` stages
only tranche-scoped files, commits, and pushes as the ordinary terminal action
when validation and git state allow it. Per-run `APPROVE:` tokens are not
required for scoped closeout commit/push; destructive actions, merges,
ambiguous staging, or unresolved dirty-state conflicts still require explicit
human approval or ruling.
