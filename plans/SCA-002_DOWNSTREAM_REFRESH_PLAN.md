---
doc_id: PLAN-SCA-002-DOWNSTREAM-REFRESH
doc_kind: plan.workflow
status: approved_for_orchestrator_planning
created: 2026-05-03
scope_change: SCA-002
accepted_revision: "0.5"
---

# SCA-002 Downstream Refresh Plan

## Purpose

This plan records the follow-on workflow after corrected `SCA-002` is accepted as the revision `0.5` decomposition basis.

SCA-002 adds physical-model, design-knowledge, operation, state/run/comparison, handoff, external-prover-boundary, and GUI comparison scope. It updates decomposition truth and companion registers only. It intentionally leaves downstream coordination, DAG, dependency, lifecycle, implementation-evidence, dispatch, and package-local production artifacts stale until owning workflows refresh them.

## Accepted Basis

| Surface | Role | State |
|---|---|---|
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | working surface | Accepted revision `0.5` |
| `docs/_Registers/ScopeLedger.csv` | authoritative companion register | 76 scope rows |
| `docs/_Registers/Deliverables.csv` | authoritative companion register | 92 deliverable rows |
| `docs/_Registers/ContextBudgetQA.csv` | authoritative companion register | 92 context-budget rows |
| `docs/_ScopeChange/SCA-002_2026-05-02_1854/Authority.md` | docs-side authority | Accepted design basis |
| `execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md` | acceptance evidence | Human accepted for downstream refresh planning |

## Stale Downstream Surfaces

The following surfaces remain stale relative to revision `0.5` and must not be used for new Type 2 dispatch until refreshed or explicitly scoped by a later human decision:

- `execution/_DAG/DAG-001/` and approval record, which are tied to revision `0.4`;
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.*`;
- `execution/_Coordination/DEV-001_DISPATCH_*.md`;
- deliverable-local `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, dependency mirrors, and production documents;
- lifecycle state files;
- dependency registers;
- implementation evidence registers.

Existing committed implementation evidence remains historical evidence. It is not invalidated by SCA-002, but it cannot be assumed complete against revision `0.5` until reconciliation maps it against the accepted scope.

## Quarantined Reference Corpus

`docs/_ScopeChange/chirality-app-docs/` is classified as a quarantined reference corpus for SCA-002.

It may be read for governance perspective on agentic implementation, write quarantine, provenance, human gates, and professional-responsibility controls. It is not accepted OpenPipeStress product scope, implementation basis, runtime architecture, UI requirement, dependency authority, or dispatch authority.

No Type 2 dispatch, DAG edge, package-local context, production artifact, implementation brief, GUI/runtime work, SDK/provider integration, harness API, Electron/desktop packaging work, lifecycle transition, or dependency-register change may be generated from this corpus under SCA-002.

Any promotion of Chirality app/harness material into OpenPipeStress development requires a later explicit scope change or architecture decision. Until then, downstream workflows should treat the corpus as read-only perspective material and keep it separate from development execution.

## Workflow Phases

### Phase 0 - Acceptance And Bootstrap Alignment

Status: completed by SCA-002 acceptance recording.

Required outputs:

- SCA-002 acceptance record exists.
- `_LATEST.md` pointers identify the accepted revision `0.5` SCA/decomposition basis.
- bootstrap and next-instance prompt surfaces point to `execution/_Decomposition/SOFTWARE_DECOMP.md`.
- downstream refresh is explicitly authorized for planning only.

### Phase 1 - ORCHESTRATOR Inventory

`ORCHESTRATOR` should scan and report:

- register counts and context-envelope counts;
- deliverables added by SCA-002;
- pre-existing deliverables whose scope/objective mapping changed;
- package-local folders missing for `PKG-13` through `PKG-16`;
- missing deliverable folders/contexts for new deliverables;
- existing deliverable-local contexts that reference the old decomposition path or revision;
- stale dispatch briefs;
- DAG node and edge coverage gaps relative to the 92-deliverable register;
- dependency/local-register surfaces requiring reconciliation;
- package-local production documents that require refresh or explicit deferral.
- quarantined Chirality app corpus presence/status, without promoting it into development scope.

Expected high-level delta:

- prior `DAG-001` represented 73 deliverable nodes from revision `0.4`;
- accepted revision `0.5` has 92 deliverables;
- 19 deliverables must be introduced into refreshed coordination surfaces.

### Phase 2 - Reconciliation Before Graph Refresh

`RECONCILIATION` should compare current DEV-001 state against revision `0.5`.

Required outputs:

- list of old dispatch briefs that must not be reused;
- list of deliverable-local contexts requiring regeneration;
- mapping of committed implementation evidence to still-valid revision `0.5` deliverables;
- list of scope additions that create new dependency assumptions;
- list of candidate dependency questions introduced by state/run/comparison, handoff, operation, and GUI comparison work.
- request for a later compatibility memo before any Chirality app/harness concept is promoted, covering dependency authority, DAG usage, sealed context, write scopes, professional reliance, protected-data boundaries, and runtime assumptions.

No file-state promotion, lifecycle change, or dependency edge approval is implied by this reconciliation.

### Phase 3 - DAG Refresh Proposal

`ORCHESTRATOR` should propose a new graph snapshot rather than mutating the approved `DAG-001` record in place.

Recommended target:

- create `execution/_DAG/DAG-002/` as the revision `0.5` coordination graph proposal;
- preserve `DAG-001` as historical approved graph for revision `0.4`;
- include all 92 accepted deliverables unless explicitly excluded by human ruling;
- keep `PKG-00` architecture-basis treatment consistent with SCA-001 unless a later human ruling changes it;
- keep candidate edges non-gating until reconciliation and human approval;
- run cycle, coverage, and schema checks before seeking approval.

Human approval is required before any new blocker queue or dispatch readiness state can use the refreshed graph.

### Phase 4 - PREPARATION Scaffold Plan

After ORCHESTRATOR and human approval of the scaffold tranche, `PREPARATION` should create or refresh package/deliverable control surfaces.

Primary targets:

- `PKG-13` through `PKG-16` package folders;
- `DEL-07-08`;
- `DEL-08-06`;
- `DEL-13-01` through `DEL-16-04`;
- revised contexts for existing deliverables whose direct scope changed, including at least `DEL-01-04` and `DEL-02-01`;
- current decomposition path/revision references in generated contexts.

Preparation should not edit production deliverable documents except where a sealed preparation brief explicitly defines the allowed control-surface write set.

### Phase 5 - Dependency And Blocker Refresh

After a human-approved refreshed graph exists:

- regenerate aggregate dependency graph artifacts for revision `0.5`;
- regenerate blocker queue from approved active edges only;
- refresh local dependency mirrors only if the governing workflow still uses them as synchronized evidence;
- keep candidate edges out of blocker computation;
- record graph approval conditions in a new approval record.

### Phase 6 - Downstream Production Refresh

Type 2 work resumes only from sealed briefs generated against revision `0.5`.

Each Type 2 execution must receive:

- one `DeliverableID`;
- one `PackageID`;
- scope items and objectives from `docs/_Registers/Deliverables.csv`;
- applicable invariants from `docs/CONTRACT.md`;
- acceptance criteria from refreshed context or sealed brief;
- explicit write scope;
- SCA-002 constraints where relevant;
- SCA-001 architecture-basis injection where relevant.

No old `DEV-001_DISPATCH_*.md` brief may be reused as an implementation brief after SCA-002 acceptance.

### Phase 7 - REVIEW, AUDIT, CHANGE

Required closing checks:

- `REVIEW` verifies refreshed contexts and any production changes against revision `0.5`;
- `AUDIT_*` checks decomposition/register coverage, graph coverage, protected-data boundaries, stale-reference risks, and dependency closure;
- `CHANGE` manages commits and final file-state records after human approval.

## Initial Tranche Recommendation

Use a conservative tranche order:

1. acceptance/bootstrap pointer alignment;
2. ORCHESTRATOR inventory and reconciliation report;
3. graph proposal for revision `0.5`;
4. preparation scaffold for new packages/deliverables;
5. context refresh for existing deliverables with changed scope;
6. blocker queue regeneration after graph approval;
7. one pilot Type 2 brief generated from revision `0.5`.

## Non-Goals

This plan does not:

- execute Type 2 deliverables;
- approve `DAG-002` or any refreshed graph;
- mutate `DAG-001`;
- update implementation evidence;
- mark lifecycle states as issued/complete;
- generate professional approval, certification, code compliance, or sealed engineering reliance.
- integrate or implement the quarantined Chirality app/harness corpus.

## Open Decisions

| ID | Decision | Owner |
|---|---|---|
| PLAN-SCA002-OD-001 | Confirm the graph refresh target name, with `DAG-002` recommended. | Human / ORCHESTRATOR |
| PLAN-SCA002-OD-002 | Decide whether to refresh all deliverable contexts in one PREPARATION tranche or split by package. | Human / ORCHESTRATOR |
| PLAN-SCA002-OD-003 | Decide which committed implementation evidence needs targeted review against SCA-002 before remaining usable. | RECONCILIATION / REVIEW |
| PLAN-SCA002-OD-004 | Decide the first revision `0.5` pilot Type 2 deliverable only after graph and context refresh approval. | Human |
| PLAN-SCA002-OD-005 | Decide whether to open a future scope change for Chirality app / agent harness integration. | Human / SOFTWARE_DECOMP / SCOPE_CHANGE |

## Recommended Next Action

Run `ORCHESTRATOR` against this plan to produce the Phase 1 inventory and Phase 2 reconciliation request, including the quarantined reference-corpus classification. Do not dispatch implementation work until the refreshed coordination surfaces are accepted.
