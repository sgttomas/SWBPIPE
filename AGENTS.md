---
doc_id: OPS-AGENTS
doc_kind: governance.agent_index
status: draft
created: 2026-04-30
---

# AGENTS — OpenPipeStress Agent Index

This file maps the general Chirality agent framework onto OpenPipeStress development. It does not redefine the canonical agent framework; it specifies how the existing agent roles should be used for this project.

## Agent posture

| Row | Purpose in OpenPipeStress |
|---|---|
| Normative | Define constraints: data boundary, solver architecture, validation, professional responsibility. |
| Operative | Execute bounded deliverables: code, tests, schemas, GUI slices, docs, reports. |
| Evaluative | Review, reconcile, audit, and decide whether outputs are acceptable for the next stage. |

## Primary agents

| Agent | Type | Role in this project |
|---|---:|---|
| `SOFTWARE_DECOMP` | 1 | Maintains `_Decomposition/SOFTWARE_DECOMP.md`, scope ledger, packages, deliverables, context budget, and open issues. |
| `PREPARATION` | 2 | Scaffolds package and deliverable folders from the decomposition. |
| `TASK` | 2 | Executes one sealed deliverable using an appropriate skill/profile. |
| `REVIEW` | 1 | Reviews deliverables against scope, tests, data boundary, and acceptance criteria. |
| `RECONCILIATION` | 1 | Detects cross-package conflicts, overlaps, stale assumptions, and inconsistent terminology. |
| `CHANGE` | 1 | Manages decomposition amendments, git state, snapshots, and human approval records. |
| `AUDIT_*` | 2 | Runs bounded checks for decomposition coverage, governance conformance, dependency closure, and epistemic integrity. |

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
