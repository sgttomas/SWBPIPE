---
doc_id: OPS-AGENTS
doc_kind: governance.agent_index
status: draft
created: 2026-04-30
---

# AGENTS — OpenPipeStress Agent Index

This file maps the general Chirality agent framework onto OpenPipeStress development. It does not redefine the canonical agent framework; it specifies how the existing agent roles should be used for this project.

## Path Anchors

Executable prompts and TASK briefs must derive paths from the active checkout:

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
- Set `WORKING_ROOT` to `{REPO_ROOT}/projects/chirality-piping`.
- Use `{REPO_ROOT}` and `{WORKING_ROOT}` in project-local instructions and
  briefs instead of machine-specific absolute paths.

## Agent posture

| Row | Purpose in OpenPipeStress |
|---|---|
| Normative | Define constraints: data boundary, solver architecture, validation, professional responsibility. |
| Operative | Execute bounded deliverables: code, tests, schemas, GUI slices, docs, reports. |
| Evaluative | Review, reconcile, audit, and decide whether outputs are acceptable for the next stage. |

## Primary agents

| Agent | Type | Role in this project |
|---|---:|---|
| `HELP_HUMAN` | 0 | Optional supervising entry; derives the cross-package graph, launches package managers, brokers notices, and validates cross-package fan-in. |
| `SOFTWARE_DECOMP` | 1 | Maintains `_Decomposition/SOFTWARE_DECOMP.md`, scope ledger, packages, deliverables, context budget, and open issues. |
| `WORKING_ITEMS` | 1 | One package-scoped instance per activated package; derives the intra-package graph, dispatches Agent 2 work across deliverables, validates fan-in, and returns package closure evidence. |
| `PREPARATION` | 2 | Scaffolds package and deliverable folders from the decomposition. |
| `TASK` | 2 | Executes one sealed deliverable using an appropriate skill/profile. |
| `REVIEW` | 1 | Reviews deliverables against scope, tests, data boundary, and acceptance criteria. |
| `RECONCILIATION` | 1 | Runs activated deliverable-corpus concordance through claim-level discovery, package waves, synthesis, decision routing, repair, and backcheck. |
| `EVALUATION` | 1 | Orchestrates generic read-only audits and cross-surface assessment outside deliverable-corpus concordance. |
| `CHANGE` | 1 | Performs final Git/file-state closeout for validated tranches: scoped status review, staging, commit, and fast-forward-safe push. |
| `RESEARCH` | 1 | Evidence-grounded, read-only inquiry over accepted decompositions, source/standards catalogs, and retrieval indexes; returns cited findings without changing project state. |
| `AUDIT_*` | 2 | Runs bounded checks for decomposition coverage, governance conformance, dependency closure, and epistemic integrity. |

## Knowledge-source reliability

The external engineering corpus `domains/piping-design/` (BM25 + dense retrieval
index, ~48.2k chunks) has **vetted, reliable prose / concept / design-guidance
content**, but its **extracted equation artifacts are NOT reliable** — they are
unreviewed `pdf2md`/OCR extractions pending the maintainer's manual equation
review, which writes a per-artifact JSON review status.

- `RESEARCH`/`RESEARCHER` and any retrieval consumer (including a future embedded
  design agent) may cite `piping-design` for concepts, terminology, and approach,
  but must **never present an extracted equation from this corpus as
  authoritative**, and must surface each artifact's review status (cleared vs
  unverified).
- **Never use `piping-design` equation artifacts as references for a
  physics-model build** (solver / kernel / analytic-verification, including
  grounding the Phase-D engineering decisions D-16 / D-18 / D-19). Physics and
  equations come from the maintainer's vetted sources, not corpus extractions.

Per human directive 2026-06-18 (`DEC-043`).

## Project-Wide Execution Discipline

Use a recorded work graph for every multi-agent tranche. The human may
prescribe the graph or delegate selection to HELP_HUMAN or a directly invoked
WORKING_ITEMS instance. Terminal fan-out/fan-in is appropriate for independent
children; supervised many-to-many agency is appropriate when active findings
may affect siblings. Mixed sequential/concurrent stages are allowed.

HELP_HUMAN owns cross-package dependencies and shared-surface ownership. Each
WORKING_ITEMS instance owns exactly one package and coordinates its
deliverable-scoped Agent 2 children. Shared reads are allowed. Concurrent
writes must be disjoint; overlapping writes are serialized against an
accepted predecessor or assigned to one integration owner. Agent 1 and Agent 2
siblings do not message or delegate directly; coordination flows through the
parent and preserves claim status and evidence.

Software package activations use `software-workflow.json` under the root
`../../docs/SOFTWARE_WORKFLOW_PROFILE.md` contract. WORKING_ITEMS selects the
appropriate `software-*` TASK skill and registered checks; the profile does not
expand project authority or replace the evidence and owner gates below.

Dependency-register work is governed by the canonical v3.1 type system. New or
refreshed dependency rows must emit only canonical core enum values; legacy
labels from historical DAGs are read-only migration inputs and must be preserved
as provenance notes rather than re-emitted. `DAG-007` is the current approved
canonical type-system rectification graph authority; `DAG-001` through
`DAG-006` remain immutable historical snapshots.

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
the package `WORKING_ITEMS` instance returns a closeout handoff to HELP_HUMAN
or the human, which invokes `CHANGE` as a separate Agent 1 for final
Git/file-state review. `CHANGE` should commit and push the validated
tranche as the ordinary terminal action when git state allows closeout;
per-run `APPROVE:` tokens are not required for scoped closeout commit/push.

If a `CHANGE` agent/subagent is unavailable, perform the `CHANGE` closeout
checklist inline:

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

## Project-specific TASK skill profiles

These are proposed profile labels for `TASK`; they may be implemented as skills or as explicit sealed briefs.

| Profile | Typical deliverables |
|---|---|
| `solver-core` | PKG-04 and PKG-05 solver/load/stress deliverables. |
| `domain-schema` | PKG-02, PKG-03, PKG-06 schemas and data models. |
| `rule-pack-engine` | PKG-06 evaluator and completeness-check deliverables. |
| `gui-workflow` | PKG-07 GUI deliverables. |
| `report-audit` | PKG-08 report and reproducibility deliverables. |
| `validation-qa` | PKG-09 verification/validation deliverables. |
| `interop-build` | PKG-10 API, plugin, packaging, and FEA handoff deliverables. |
| `docs-education` | PKG-11 documentation and invented-example deliverables. |
| `security-privacy` | PKG-12 private-data and telemetry deliverables. |
| `ip-governance` | PKG-01 and data-boundary deliverables. |

## Dispatch rule

Every Type 2 execution must receive:

- one `DeliverableID`;
- the parent `PackageID`;
- scope items and objectives from `_Registers/Deliverables.csv`;
- applicable invariants from `CONTRACT.md`;
- acceptance criteria from `_CONTEXT.md` or the sealed brief;
- explicit write scope.

If a requested task crosses package boundaries or requires protected data, stop and escalate to `SOFTWARE_DECOMP` or the human project authority.
