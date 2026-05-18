---
amendment_id: SCA-002
doc_kind: scope_change.acceptance_record
package_role: snapshot / handoff artifact
created: 2026-05-03
status: accepted_for_downstream_refresh_planning
accepted_by: human_project_authority
---

# SCA-002 Acceptance Record

## Human Approval

On 2026-05-03, the human project authority approved corrected SCA-002 for acceptance recording and downstream refresh planning:

> APPROVE: proceed with SCA-002 for acceptance recording and downstream refresh planning

## Acceptance Scope

This acceptance confirms `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5` as the accepted decomposition basis for downstream refresh planning.

Accepted SCA-002 truth surfaces:

- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ContextBudgetQA.csv`
- `docs/_ScopeChange/SCA-002_2026-05-02_1854/Authority.md`
- `execution/_ScopeChange/SCA-002_2026-05-02_1854/`

## Boundaries

This approval authorizes acceptance recording and ORCHESTRATOR downstream refresh planning. It does not itself:

- refresh `DAG-001`;
- approve a new DAG;
- update deliverable-local production documents;
- update deliverable-local dependency registers;
- update lifecycle states;
- update implementation evidence;
- create or reuse Type 2 dispatch briefs;
- certify engineering results, code compliance, professional approval, sealing, or reliance.

## Post-Acceptance Quarantine Clarification

The later review of `docs/_ScopeChange/chirality-app-docs/` classified that folder as a quarantined reference corpus for SCA-002. It may inform governance perspective, but it is not accepted product scope, implementation basis, runtime architecture, UI requirement, dependency authority, or dispatch authority.

Promotion of any Chirality app/harness material requires a later explicit scope change or architecture decision.

## Next Authorized Workflow

`ORCHESTRATOR` may plan the revision `0.5` downstream refresh from `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.

Execution of the refresh remains gated by the plan's own approval points.
