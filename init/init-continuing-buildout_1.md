# NEXT INSTANCE PROMPT

## Required Reading

Read, in order:

1. `AGENTS.md`
2. `agents/AGENT_WORKING_ITEMS.md`
3. `agents/AGENT_TASK.md`
4. `docs/CONTRACT.md`
5. `docs/SPEC.md`
6. `docs/TYPES.md`
7. `docs/IP_AND_DATA_BOUNDARY.md`
8. `execution/_Coordination/_COORDINATION.md`
9. `execution/_DAG/_LATEST.md`
10. `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
11. `execution/_DAG/DAG-004/APPROVAL_RECORD.md`
12. `execution/_DAG/DAG-004/DAG_Audit.json`
13. Relevant deliverable-local state files for any proposed tranche:
    - `_CONTEXT.md`
    - `_STATUS.md`
    - `_REFERENCES.md`
    - `_DEPENDENCIES.md`
    - `Dependencies.csv`
    - `MEMORY.md`

## Current State

`DAG-004` is the approved active graph authority. Its active edge set is authoritative for coordination. Candidate rows remain non-gating.

`DEV-001` remains the current development path. The blocker queue has been recomputed from approved `DAG-004`: 92 unblocked, 0 blocked, 861 active edges included, 11 candidate edges excluded.

Use deliverable-local `MEMORY.md`, `_STATUS.md`, `_DEPENDENCIES.md`, and `Dependencies.csv` as local state evidence. Archived coordination files are historical evidence only unless explicitly referenced by an approved current document.

First check `git status`.

Important: the prior instance approved DAG-004 as active graph authority and updated the root DEV-001 blocker queue to DAG-004. If those changes are still uncommitted, recommend committing them before starting another tranche, but do not commit without explicit human approval.

Known recent governance direction:

- DAG-004 approval covered the active edge set and SCA-003 metadata/evidence refresh only.
- Candidate edges remain non-gating.
- No lifecycle changes, candidate promotion, Type 2 dispatch, PREPARATION work, release claims, or professional/code-compliance claims were authorized by DAG-004 approval.
- Physical model work is in scope and not DAG-blocked, but still likely needs vital product-completion work beyond existing schema/contracts/fixtures.

## Operating Loop

Proceed session by session through bounded tranches:

1. Assess current state from approved coordination evidence and relevant deliverable-local files.
2. Propose one bounded tranche with:
   - objective
   - rationale
   - deliverables involved
   - write scope
   - explicit exclusions
   - validation plan
   - fan-out/fan-in plan if useful
   - closeout criteria
3. Wait for human approval or redirection.
4. Only after approval, dispatch canonical `TASK` workers where useful.
5. For each `TASK` brief, include:
   - `DeliverableID`
   - `PackageID`
   - `DeliverablePath`
   - required local reading
   - acceptance criteria
   - explicit write scope
   - validation commands
   - run-record closeout requirement
   - deliverable `MEMORY.md` update requirement
6. Fan in outputs, review/audit results, and record concise closeout evidence.
7. Require explicit human approval for lifecycle changes, candidate promotion, commits, release claims, acceptance records, or professional/code-compliance claims.

## Recommended Candidate Direction

Start by assessing whether the next bounded tranche should target physical model completion.

The current decomposition already scopes the physical model as the editable schema-backed source of truth, richer than the analytical solver model, with physical-to-analytical transformation, design knowledge, constraints, model operations, immutable states, comparisons, and handoff workflows.

The likely next agenda is not a scope change by default. It is to identify a bounded implementation tranche that advances physical model completeness inside existing scope, especially around one of:

- physical model entity coverage and canonical schema use;
- physical-to-analytical transformation coverage across components, supports, loads, and diagnostics;
- model-operation application and auditability;
- persistence/state/run integration for physical model records;
- GUI design-authoring workflow readiness;
- constraint validation maturity for practical design authoring.

If review shows another path is more urgent under DAG-004, propose that instead and explain why.

## Action

Act in the `WORKING_ITEMS` persona.

Do not execute implementation yet. Do not dispatch subagents yet. Do not change lifecycle state, promote candidates, commit, or make release/professional claims.

Your first output should be a proposed next bounded development tranche, including:

- objective
- why this tranche now
- deliverables and files likely involved
- explicit write bounds
- explicit non-goals
- validation plan
- closeout evidence requirements
- proposed `TASK` briefs if fan-out is appropriate
- review/fan-in plan

Planning constraints:

- Treat PKG-02 as the accepted foundation contract if encountered.
- Treat DAG-004 as the active graph authority.
- Do not reopen DEV-001 finding resolution or mark findings resolved unless explicitly asked.
- Preserve lifecycle states.
- Do not broaden scope just because physical model work remains incomplete; recommend `SCOPE_CHANGE` only if the proposed work exceeds SOW-064 through SOW-076 or changes approved authority boundaries.
- Each proposed `TASK` brief must include proper run-record and deliverable `MEMORY.md` closeout requirements.
- Use fan-out, subagent delegation, and fan-in only after human approval, with the main instance acting as orchestrator and reviewer.