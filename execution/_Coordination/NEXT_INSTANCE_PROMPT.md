# NEXT INSTANCE PROMPT

Read `/Users/ryan/ai-env/projects/chirality/agents/AGENT_WORKING_ITEMS.md`

Act in the `WORKING_ITEMS` persona for
`/Users/ryan/ai-env/projects/chirality/projects/chirality-piping`.

## Current Authority

- Read
  `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/_COORDINATION.md`
  and follow the active-surface, state-tracking, and Integrated Verification
  and Tranche Selection Loop directions.
- Read
  `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/NEXT_INSTANCE_STATE.md`
  if present, and treat it as compact resume state only. When it summarizes
  project truth, verify against the authoritative artifact it cites.
- Treat blockers or dirty git state outside this project/write scope as
  external-scope noise; record and bypass, do not fix.

## Governing Imperatives

1. `SOFTWARE_DECOMP` says what must be built and why.
2. `DAG-005` says what depends on what, using approved active edges.
3. `DEV-001` says what is currently unblocked or blocked for implementation,
   based on committed evidence.
4. `_COORDINATION.md` says how to execute work: bounded tranches, human
   approval, integrated verification snapshots, gap-to-tranche selection,
   `TASK` workers, fan-in, validation, and evidence records.

## Next Action

1. Enter through the coordination workflow:
   - read this prompt;
   - read `_COORDINATION.md`;
   - read `NEXT_INSTANCE_STATE.md` if present;
   - read `execution/_Decomposition/SOFTWARE_DECOMP.md`;
   - read `execution/_DAG/_LATEST.md`, `DAG-005/APPROVAL_RECORD.md`, and the
     needed `DAG-005` node/edge/wave artifacts;
   - read `DEV-001_BLOCKER_QUEUE.md/.csv` and
     `DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
2. Use the Integrated Verification and Tranche Selection Loop in
   `_COORDINATION.md` as the governing workflow. If a human-approved
   implementation tranche is already active, continue that tranche within its
   write bounds. Otherwise, use the loop to propose exactly one next bounded
   tranche.
3. Take stock of the project through the lens of the `execution/` folders,
   surveying packages and deliverables against `SOFTWARE_DECOMP`, `DAG-005`,
   and `DEV-001`.
4. For coordination-sensitive planning, run
   `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
   and record `git status --short` before proposing the tranche.
5. If current readiness gaps are not already pinned to one bounded owner,
   propose a read-only integrated verification snapshot under
   `execution/_Aggregation/TP-INTEGRATED-VERIFY-###_YYYY-MM-DD/` using the
   commands and closeout files specified in `_COORDINATION.md`.
6. Inspect in detail any execution files, or other core implementation code
   files, according to your need when deciding what to do next.
7. Propose the next bounded tranche only; do not implement until approved.
