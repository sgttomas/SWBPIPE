# NEXT INSTANCE PROMPT

## Required Reading

Read, in order:

1. `agents/AGENT_WORKING_ITEMS.md`
2. `agents/AGENT_TASK.md`
3. `docs/CONTRACT.md`
4. `docs/SPEC.md`
5. `docs/TYPES.md`
6. `docs/IP_AND_DATA_BOUNDARY.md`
7. `execution/_Coordination/_COORDINATION.md`
8. `execution/_DAG/_LATEST.md`
9. `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`

## Current State

`DAG-005` is the approved active graph authority. Its active edge set is authoritative for coordination. Candidate rows remain non-gating.

`DEV-001` remains the current development path. 

Use deliverable-local `MEMORY.md`, `_STATUS.md`, `_DEPENDENCIES.md`, and `Dependencies.csv` as local state evidence. Archived coordination files are historical evidence only unless explicitly referenced by an approved current document.

First check `git status`.

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

There is currently no recommended candidate direction because we paused development at a point where development plans concluded.  There is still much to do, however.

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
- Preserve lifecycle states.
- Do not broaden scope just because physical model work remains incomplete; recommend `SCOPE_CHANGE` only if the proposed work exceeds SOW-064 through SOW-076 or changes approved authority boundaries.
- Each proposed `TASK` brief must include proper run-record and deliverable `MEMORY.md` closeout requirements.
- Use fan-out, subagent delegation, and fan-in only after human approval, with the main instance acting as orchestrator and reviewer.