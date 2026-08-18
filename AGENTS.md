---
doc_id: OPS-AGENTS
doc_kind: governance.agent_index
status: draft
created: 2026-04-30
---

# AGENTS — OpenPipeStress Project Instructions

This file records OpenPipeStress-specific instructions. Root `AGENTS.md` and
the canonical `agents/AGENT_*.md` packages govern runtime roles, selection,
delegation, and orchestration.

## Path Anchors

Executable prompts and bounded execution briefs must derive paths from the
active checkout:

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
- Set `WORKING_ROOT` to `{REPO_ROOT}/projects/chirality-piping`.
- Use `{REPO_ROOT}` and `{WORKING_ROOT}` in project-local instructions and
  briefs instead of machine-specific absolute paths.

## Knowledge-source reliability

The external engineering corpus `domains/piping-design/` (BM25 + dense retrieval
index, ~48.2k chunks) has **vetted, reliable prose / concept / design-guidance
content**, but its **extracted equation artifacts are NOT reliable** — they are
unreviewed `pdf2md`/OCR extractions pending the maintainer's manual equation
review, which writes a per-artifact JSON review status.

- Any retrieval consumer may cite `piping-design` for concepts, terminology,
  and approach, but must **never present an extracted equation from this corpus
  as authoritative**, and must surface each artifact's review status (cleared
  vs unverified).
- **Never use `piping-design` equation artifacts as references for a
  physics-model build** (solver / kernel / analytic-verification, including
  grounding the Phase-D engineering decisions D-16 / D-18 / D-19). Physics and
  equations come from the maintainer's vetted sources, not corpus extractions.

Per human directive 2026-06-18 (`DEC-043`).

## Agent Posture And Delegation

Root `AGENTS.md` and `agents/AGENT_*.md` define the roles; this section maps
them onto Piping work and fixes what a delegated run must leave behind.

| Agent | Type | Role in this project |
|---|---:|---|
| `HELP_HUMAN` | 0 | Default supervising entry for the standing development loop; runs Step 0, selects the tranche, supervises the selected Agent 1 managers, validates cross-manager fan-in, and routes closeout to `CHANGE`. |
| `WORKING_ITEMS` | 1 | One package-scoped instance per activated package; owns the tranche's work graph, dispatches Agent 2 children, validates fan-in, and returns package closure evidence. May execute single-deliverable work directly (see delegation posture below). |
| `TASK` | 2 | Executes one sealed implementation, review, diagnosis, test-planning, docs, or evidence sub-scope with explicit read/write bounds; the `software-*` skills are the default method packs. |
| `CHANGE` | 1 | Scoped Git/file-state closeout per the checklist below. |
| `EVALUATION` / `REVIEW` / `RESEARCH` / `AUDIT_*` | 1 / 2 | As defined at root; read-only unless the tranche's write scope names them. |

**Delegation posture (work-type conditioned).** Delegation is sized to the
target, not applied by default:

- *Single-manager path.* Tooling, tests, docs, evidence, and coordination
  tranches confined to one deliverable or one bounded integration scope may be
  executed directly by the `WORKING_ITEMS` instance under the bounded-execution
  evidence contract below; the receipt is the record and no AgentRuns package is
  owed.
- *Independent-review path (mandatory).* Any tranche that changes source under
  `core/**` (solver, mechanics, loads, stress recovery, model operations,
  reporting math) or `apps/desktop/src/**` product behavior dispatches at least
  one **fresh, read-only** `TASK + software-code-review` child over 100% of the
  frozen diff before the DEC-025 sweep and before push. The reviewer has no write
  target and is not the implementer's context. A `PASS` with no actionable finding
  is required to publish; findings are remediated by a child or the manager and
  re-reviewed. The sweep remains the deterministic gate; review is additional,
  not a substitute.
- *Multi-agent path.* Whenever children are dispatched (either path above,
  or any fan-out), the tranche uses a recorded work graph and freezes, under
  `execution/_Coordination/AgentRuns/<RUN_ID>/`: the activation or plan, the
  work graph, one sealed launch brief per child (role, skill, scope path, declared
  reads, allowed write targets, tools, acceptance criteria, exclusions), each
  child's return and status, the manager return, and a handoff state. Parentage,
  scopes, status, and returns are persisted per root `AGENTS.md`; siblings do not
  message or delegate directly. Terminal fan-out/fan-in for independent children;
  supervised many-to-many only when active findings may affect siblings.

**Execution attribution.** No durable Piping surface prescribes models (D-GOV-17
M1-D). Record which model actually ran each dispatched role in the AgentRuns
record and point to it from the receipt; when no AgentRuns record exists, the
receipt carries the minimum attribution directly. Any mid-wave substitution is
recorded where the wave's execution is recorded — never silently.

## Project-Wide Execution Discipline

Software work uses `software-workflow.json` under the root
`../../docs/SOFTWARE_WORKFLOW_PROFILE.md` contract. The profile registers
project checks; it does not expand project authority or replace the evidence
and owner gates below.

Dependency-register work is governed by the canonical v3.1 type system. New or
refreshed dependency rows must emit only canonical core enum values; legacy
labels from historical DAGs are read-only migration inputs and must be preserved
as provenance notes rather than re-emitted. The current approved dependency
graph authority is the immutable snapshot named by
`execution/_DAG/_LATEST.md`; consumers must resolve that committed pointer
rather than pinning a DAG identifier in this instruction. At this packet's
2026-07-25 basis, the pointer names `DAG-008`; earlier DAGs remain immutable
historical or superseded snapshots.

Agents may write inside the selected tranche scope, required evidence and
coordination artifacts, and project-local plans created under the issue-plan
rule below. Do not write root governance files, root agent instructions, root
skills, or root tools unless the human explicitly directs that governance
work.

Host-capability execution. Some registered checks and evidence surfaces cannot run inside the session's sandbox because the sandbox denies process spawning, Mach bootstrap, network, or keychain access — DEC-025 surfaces annotated execution_capability: host (Playwright/Chromium) are the standing example. For such a surface, the executing agent requests sandbox escalation for the exact command (for example npm run build:wasm && PLAYWRIGHT_WORKERS=1 ../../node_modules/.bin/playwright test …, or tools/release/run_evidence_sweep.py --execute --only-capability host) and runs it itself in the session; the human approves or declines per command. A sandbox denial is not a substrate limitation and is not a reason to park, hand off, waive, or infer a pass. Park with HOST_RERUN_REQUIRED only when the escalation request itself is declined, and record the exact command that was declined. Escalated runs are recorded in the run record with the command, the capability annotation, and the result, and remain bound to the same commit-hash and clean-worktree rules as sandboxed runs. Root harness CI is additional evidence where it exists; it does not replace a required host surface unless the owning decision says so.

There may be other agents working in this monorepo with disjoint write scopes.
Treat unrelated dirty files outside the selected tranche scope as external
state. Do not fix, stage, revert, or interpret them unless the human directs
that work.

## Closeout And Git Discipline

When a tranche is complete, validated, and project closeout rules allow it,
perform the following scoped Git/file-state closeout checklist:

- inspect root git status;
- confirm validation evidence and skipped-check notes;
- stage only files in the selected tranche write scope, required
  evidence/coordination artifacts, and any explicitly created governance-issue
  plan;
- do not stage unrelated dirty files;
- commit the scoped tranche when validation and staging are clean;
- fetch or otherwise verify upstream state before pushing;
- push only when the upstream branch can be fast-forwarded to the local scoped
  commit.

Branch creation is routine. Creating the tranche's task branch (and, when
isolation is warranted, its worktree lane) from a verified clean basis on the
integration branch — clean status, no in-progress Git operation, base SHA
recorded — is an ordinary Step 0/Step 4 act and requires no `APPROVE:` token,
whether performed by the loop session or by a dispatched CHANGE instance.
CHANGE reports the branch name and base SHA and proceeds. Basing a lane on a
dirty worktree, or any switch that would discard or carry uncommitted work,
remains non-routine under root `AGENT_CHANGE.md`. Owner direction 2026-07-19
(recorded in App `loop/LOOP_RECEIPTS.md` Receipt-74), applied to this loop by
owner direction 2026-08-15; scoped commit/push closeout follows the checklist
above and likewise needs no per-run `APPROVE:` token.

If the remote branch has advanced and local `HEAD` does not already include
it, stop and surface the conflict. Do not merge, rebase, force push, or judge
another agent's work during ordinary closeout.

Git closeout is source-control hygiene. It is not lifecycle issuance, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance acceptance.

## Issue-Plan Rule

If concrete, actionable issues are discovered that would improve the agentic
development loop, project governance, agents, or skills, record them in at
most one timestamped plan under `{WORKING_ROOT}/plans/` for the session. Do
not create a plan merely to satisfy this instruction when no actionable issue
was found.

## Bounded-execution evidence contract

Every bounded deliverable execution must bind:

- one `DeliverableID` or an otherwise authorized bounded integration scope;
- the parent `PackageID`;
- scope items and objectives from `_Registers/Deliverables.csv`;
- applicable invariants from `CONTRACT.md`;
- acceptance criteria from `_CONTEXT.md` or the sealed brief;
- explicit write scope.
- for delegated executions, the sealed brief's path under
  `execution/_Coordination/AgentRuns/<RUN_ID>/` and the reviewer's return when
  the independent-review path applies.

If a bounded execution would cross package boundaries or require protected
data, stop and return the condition to the governing authority.
