---
doc_id: OPS-README
doc_kind: governance.index
status: current_index
created: 2026-04-30
---

# OpenPipeStress Agentic Development Docs

This `docs/` package is a seed governance and decomposition set for agentic development of **OpenPipeStress**: a source-available noncommercial, code-neutral piping flexibility and stress-analysis platform.

The central project stance is:

> **Open the mechanics; protect the standards; empower the engineer.**

This package is written from the perspective of `SOFTWARE_DECOMP`: it converts the product intent and PRD into a structured, flat package/deliverable decomposition suitable for downstream agent execution.

## Document map

| File | Role | Use |
|---|---|---|
| `../INIT.md` | Bootstrap | Required reading order and agent startup constraints; maintained only at the repository root. |
| `../AGENTS.md` | Agent index | Project-specific use of Chirality agent roles. |
| `INTENT.md` | Directional intent | Why this product exists and what must remain true. |
| `PRD.md` | Product requirements | Product capabilities, users, non-goals, and release strategy. |
| `PLAN.md` | Strategic roadmap (non-governing) | Definition of "complete per the PRD", current milestone position, layer-relation map, roadmap posture and risks; routes to the authorities and to the active `plans/PLAN_*.md` for tranche selection. |
| `DIRECTIVE.md` | Founding directive | Design philosophy, professional boundary, stop rules, and scope constraints. |
| `TYPES.md` | Vocabulary and identity | Canonical terms, IDs, statuses, and software/piping domain types. |
| `CONTRACT.md` | Invariant catalog | Binding project invariants for legal/data boundary, solver, rule packs, reports, and agents. |
| `SPEC.md` | Technical specification | Architecture, schemas, solver mechanics, GUI, reports, V&V, and agentic execution mechanics. |
| `IP_AND_DATA_BOUNDARY.md` | Data/IP policy | Public/private data rules, contributor certification, provenance, and quarantine policy. |
| `VALIDATION_STRATEGY.md` | Verification and validation | Benchmark and release-quality strategy. |
| `AGENTIC_DEVELOPMENT_WORKFLOW.md` | Agentic workflow | Authority surfaces, phase-aware coordination loops, `TASK` discipline, and handoff expectations. |
| `user_guide/index.md` | User guide | Draft user-facing workflow skeleton, limitations, data boundaries, and current `TBD`s. |
| `../execution/_Decomposition/SOFTWARE_DECOMP.md` | Working surface | SSOW, objectives, packages, deliverables, scope ledger summary, telemetry, open issues. |
| `_Registers/ScopeLedger.csv` | Authoritative companion register | Machine-readable scope-to-package/deliverable mapping. |
| `_Registers/Deliverables.csv` | Authoritative companion register | Machine-readable deliverable catalog. |
| `_Registers/ContextBudgetQA.csv` | Authoritative companion register | Context-envelope and sizing audit. |
| `../governance/MAINTAINERS.md` | Maintainer policy skeleton | Draft public-governance surface for maintainer, contribution, release, and open governance questions. |

## Status

The current decomposition authority is
`execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`, coordinated by
approved `DAG-007` under `execution/_DAG/DAG-007/`. `DAG-007` is the current
canonical dependency type-system rectification graph authority. `DAG-001`
through `DAG-006` remain immutable historical snapshots; `DAG-006` preserves
the prior approved legacy-enum graph context. Revision `0.7` carries the accepted
architecture runway, design-engine and persistence amendments, and
export-format interoperability scope into the active working surface.

The project license selected by human governance decision is
`PolyForm-Noncommercial-1.0.0`; see `../LICENSE.md`.

This docs package remains a governance and coordination index. It is not an
approved engineering work product, not a legal opinion, not a release record,
and not a claim of code compliance or professional reliance.

## How agents should use this package

1. Start from the root `INIT.md`, then follow `execution/_Coordination/_COORDINATION.md` for authority intake, work selection, execution discipline, validation, and handoff.
2. Use `execution/_Decomposition/SOFTWARE_DECOMP.md` as the authoritative working surface for packages and deliverables.
3. Use `_Registers/*.csv` for machine-checkable mappings.
4. Treat every deliverable as bounded: no agent may expand scope silently.
5. Unknown values become `TBD`; suspected protected data is quarantined and escalated.
6. Generated outputs remain drafts until a human accepts them at the appropriate gate.

## Foundational references used to create this package

- Conversation intent captured in `INTENT.md`.
- Product requirements captured in `PRD.md`.
- Chirality bootstrap and agent framework examples supplied by the user.
- M. W. Kellogg, *Design of Piping Systems* (1956), as a historical technical reference for classical piping flexibility analysis.
