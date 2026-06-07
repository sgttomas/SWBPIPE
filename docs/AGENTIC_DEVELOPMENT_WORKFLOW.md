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
| 8 | Hand off for review or human decision without changing lifecycle state unless that gate was explicitly authorized. | Changed paths, validation results, data-boundary notes, warnings, and open `TBD`s. |

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
