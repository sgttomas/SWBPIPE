# NEXT INSTANCE PROMPT

## Required Reading
Read:
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

## Current State

`DAG-003` is approved graph authority. Its active edge set is authoritative for coordination. Candidate rows remain non-gating.

`DEV-001` remains the current development path. The blocker queue has been recomputed from approved `DAG-003`: 92 unblocked, 0 blocked.

Use deliverable-local `MEMORY.md`, `_STATUS.md`, `_DEPENDENCIES.md`, and `Dependencies.csv` as local state evidence. Archived coordination files are historical evidence only.

First check `git status`. TP-PHYS-008 axial-effect work and the follow-up diagnostics correction were implemented and validated in the prior instance; if still uncommitted, recommend committing before starting another tranche, but do not commit without explicit human approval.

## Operating Loop

Proceed session by session through bounded tranches:

1. Propose one tranche with objective, scope, write bounds, validation, and closeout criteria.
2. Wait for human approval or redirection.
3. Dispatch canonical `TASK` workers where useful. For deliverable-local work, include `DeliverablePath`; `TASK` must read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, and primary deliverable artifacts before acting.
4. Fan in outputs, run review/audit, and record concise closeout evidence.
5. Require explicit human approval for lifecycle changes, candidate promotion, commits, release claims, acceptance records, or professional/code compliance claims.

## Recommended Candidate Tranche

We have reached a natural inflection point where one major area of development has come to resolution and another one could open up in a new direction.  You can assess and propose according to what you see fit.

## Action

Act in the `WORKING_ITEMS` persona: propose the next bounded development tranche, with objective, scope, validation, closeout criteria, bounded `TASK` briefs where useful, and explicit write scopes. Do not execute, dispatch subagents, change lifecycle state, promote candidates, commit, or make release/professional claims until human approval.

Planning constraints:
- Treat PKG-02 as the accepted foundation contract if encountered.
- Do not reopen DEV-001 finding resolution or mark findings resolved unless explicitly asked.
- Preserve lifecycle states.
- Require each proposed TASK brief to include proper run-record and deliverable `MEMORY.md` closeout requirements.

plan your execution using fan-out, subagent delegation, and fan-in, all with you as the orchestator managing and reviewing the tasks you assign to 'TASK' agents with the appopriate init task briefs.