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

If a bounded execution would cross package boundaries or require protected
data, stop and return the condition to the governing authority.
