# PLAN — Development Roadmap

This document captures the development roadmap for the Chirality project execution system. It summarizes what has been codified, identifies future hardening candidates, and provides sequencing rationale.

## Control-Plane Boundary

- `docs/PLAN.md` is strategic: it explains priorities, rationale, and roadmap direction.
- Operational sequencing/blocker policy is governed by `execution/_Coordination/_COORDINATION.md`.
- Current run-state pointers and immediate queue belong in `execution/_Coordination/NEXT_INSTANCE_STATE.md`.

## Local-Only Source Policy

- Development guidance and execution evidence must come from files in this repository.
- Do not rely on non-local repositories or external clones as authoritative sources.
- `frontend/` is in-scope and present in this workspace. If any required runtime path is absent, treat deliverable documents under `execution/PKG-*/1_Working/DEL-*/` as the implementation contract and record a local blocker in coordination artifacts.

---

## 1. Completed: System Hardening

The working system has been codified into formal governance documents and aligned to the active agent framework.

### Governance Documents Written

| Document | Purpose | Status |
|----------|---------|--------|
| `docs/SPEC.md` | Physical structures, file formats, Dependencies.csv v3.1 schema, folder layout, validation checklist | Complete |
| `docs/TYPES.md` | Domain vocabulary, hierarchy, stable IDs, dependency vocabulary, agent roles, lifecycle states | Complete |
| `docs/DIRECTIVE.md` | Founding intent, design philosophy, professional responsibility model, scope, constraints | Complete |
| `docs/CONTRACT.md` | Invariant catalog (20 K-* invariants), change policy, enforcement map | Complete |
| `docs/PLAN.md` | This document | Complete |

### Governance Alignment

Current governance documents are internally aligned on the core model:
- Hierarchy is flat `package->deliverable` across `docs/TYPES.md`, `docs/SPEC.md`, and `docs/CONTRACT.md`
- Authoritative execution state is file-based (`_STATUS.md`, `_DEPENDENCIES.md`, `Dependencies.csv`)
- Invariant catalog is centralized in `docs/CONTRACT.md`
- Agent role boundaries and write scopes are defined by the active `AGENT_*.md` instruction suite

### Agent Instruction Hardening

| Change | Agents Affected |
|--------|-----------------|
| QA Contract sections added | PREPARATION, 4_DOCUMENTS, CHIRALITY_FRAMEWORK, CHIRALITY_LENS |
| Output Persistence notes added | ORCHESTRATOR, RECONCILIATION |

Agent instruction consistency: 92% → estimated 95%+ after hardening.

---

## 2. Existing Tooling

### Validation + Example Assets

- `examples/` provides concrete execution-root samples with package/deliverable structures and semantic artifacts (`_SEMANTIC.md`) for regression and conformance testing.
- `docs/harness/` documents SDK runtime validation and CI integration for harness behavior.
- `frontend/scripts/validate-harness-*.mjs` is now an explicit build target under the frontend baseline scope.

### DEL-03-05 Policy Rulings (2026-02-23)

- OI-001 key provisioning policy is resolved for current scope as of SCA-003 (2026-02-24): `ENV+UI` (UI key entry/local secure storage precedence; `ANTHROPIC_API_KEY` env fallback).
- DEL-03-05 provider completion path is explicitly SDK-first (`ADOPT_SDK_NOW`); direct HTTP provider paths are interim-only and not completion evidence.
- SDK-path implementation pass is now landed in `frontend/` with `@anthropic-ai/sdk` pinned to `0.78.0`; provider runtime preserves typed error taxonomy and streaming event contracts.

### Current Delivery Snapshot (2026-02-24)

- Core scope (`PKG-01..07`) is fully issued: 29/29 deliverables in `ISSUED`.
- Optional hardening (`PKG-08`) is scope-resolved by SCA-002:
  - `IN` and issued: `DEL-08-01`, `DEL-08-02`
  - `OUT` and retired: `DEL-08-03`, `DEL-08-04`, `DEL-08-05`, `DEL-08-06`, `DEL-08-07`
- Dependency closure is acyclic on the full graph (`SCC=0`) with blocker-subset sequencing unchanged and acyclic.

### Desktop Frontend (`frontend/`) — Baseline Completed

Frontend baseline scope added by SCA-001 has been implemented and issued:

1. `DEL-01-03` — workspace bootstrap + packaging baseline (`frontend/`, scripts, bundle resources)
2. `DEL-03-07` — harness API baseline (`/api/harness/session/*`, `/api/harness/turn`)
3. `DEL-02-05` — workflow shell baseline (PORTAL/PIPELINE, file tree, chat wiring)
4. `DEL-07-03` — validation/runbook baseline (deterministic validation artifacts)

Remaining frontend work is now maintenance/hardening under issued deliverables, not baseline creation.

### Matrix Navigation + Pipeline Taxonomy

The desktop UI uses a 3x4 matrix to route operator intent into WORKBENCH or PIPELINE.

- Columns (shared): `GUIDING`, `APPLYING`, `JUDGING`, `REVIEWING`
- Rows:
  - `NORMATIVE` -> opens `WORKBENCH`
  - `OPERATIVE` -> opens `PIPELINE`
  - `EVALUATIVE` -> opens `WORKBENCH`

Matrix cells:

| Row | Guiding | Applying | Judging | Reviewing |
|-----|---------|----------|---------|-----------|
| `NORMATIVE` | `HELP` | `ORCHESTRATE` | `WORKING_ITEMS` | `AGGREGATE` |
| `OPERATIVE` | `DECOMP*` | `PREP*` | `TASK*` | `AUDIT*` |
| `EVALUATIVE` | `AGENTS` | `DEPENDENCIES` | `CHANGE` | `RECONCILING` |

PIPELINE category model:

- `DECOMP*`
- `PREP*`
- `TASK*`
- `AUDIT*`

Option policy:

- Requested but unsupported variants remain visible as disabled entries.
- Disabled entries are intentionally non-selectable and rendered as "coming soon".

`TASK*` selector model:

- Uses split selectors instead of one mixed list:
  - `Task Agent` (static options)
  - `Scope` selectors (dynamic options)
- Dynamic scope sources:
  - Deliverables scanned from the selected working root
  - Knowledge types scanned from canonical deliverable file types
- Knowledge-type scope is shown only when a knowledge decomposition marker is found in `_Decomposition`.

---

## 3. Hardening Scope Status (Post SCA-002)

SCA-002 (2026-02-24) resolved all PKG-08 TBD scope items.

| Candidate | Deliverable | Status | Notes |
|---|---|---|---|
| `_REFERENCES.md` content hashes + verification | `DEL-08-01` | IN / ISSUED | Implemented under `execution/_Scripts/references_hash_tool.py` with test coverage and control-plane integration. |
| `Dependencies.csv` v3.1 schema linter | `DEL-08-02` | IN / ISSUED | Implemented under `execution/_Scripts/validate_dependencies.py` with test coverage. |
| Folder structure validator | `DEL-08-03` | OUT / RETIRED | Removed from active scope by SCA-002. |
| On-demand dependency graph generator | `DEL-08-04` | OUT / RETIRED | Removed from active scope by SCA-002. |
| Deliverable-level lock mechanism | `DEL-08-05` | OUT / RETIRED | Removed from active scope by SCA-002. |
| Unified run record persistence | `DEL-08-06` | OUT / RETIRED | Removed from active scope by SCA-002. |
| Staleness propagation tooling | `DEL-08-07` | OUT / RETIRED | Removed from active scope by SCA-002. |

---

## 4. Sequencing Rationale

Current sequencing priority is maintenance and coherence, not new PKG-08 expansion:

1. Keep full-graph closure acyclic while preserving blocker-subset execution semantics.
2. Keep DEL-08-01/08-02 tooling green in CI/local workflows.
3. Treat DEL-08-03..08-07 as out-of-scope unless a future scope change explicitly reactivates them.

---

EOF
