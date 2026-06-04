# NEXT INSTANCE PROMPT

## Current Authority

- Read `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/_COORDINATION.md` and follow the Development Loop directions.
- Discover current state from authoritative surfaces named in `_COORDINATION.md`: `_DAG/_LATEST.md`, approved DAG artifacts, deliverable-local `_STATUS.md`, `MEMORY.md`, `_run_records/**`, dependency/review files, and current aggregation or review artifacts as needed.
- Treat blockers or dirty git state outside this project/write scope as external-scope noise; record and bypass, do not fix.

## Governing Imperatives

1. `SOFTWARE_DECOMP` says what must be built and why.
2. `DAG-006` says what depends on what, using approved active edges.
3. Deliverable-local `_STATUS.md` files say the current lifecycle state for
   work selection.
4. `_COORDINATION.md` says how to execute work: bounded tranches, human
   approval, local-status discovery, DAG-guided context selection, `TASK`
   workers, fan-in, validation, and evidence records.

## Next Action

1. Enter through the coordination workflow:
   - read this prompt;
   - read `_COORDINATION.md`;
   - perform the baseline authority intake defined in `_COORDINATION.md`;
   - add execution or review intake documents only as required by the selected tranche type;
   - run  `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary`.
2. Use the Local Status And DAG-Guided Development Loop in `_COORDINATION.md`
   as the governing workflow. If a human-approved implementation or review
   tranche is already active, continue that tranche within its write bounds.
   Otherwise, use the loop to propose exactly one next bounded tranche.
3. Take stock of the project through deliverable-local `_STATUS.md` files:
   normal development usually selects from `IN_PROGRESS`; human-directed review
   gates select from `CHECKING`; `ISSUED` is not work-selection scope; and
   `SEMANTIC_READY` is architecture/preparation basis unless specifically
   directed.
4. For the selected deliverable, inspect the local deliverable folder first,
   then use `DAG-006` only to discover upstream/downstream context to read.
5. Record `git status --short` before coordination-sensitive planning or
   execution.
6. Inspect in detail any execution files, or other core implementation code
   files, according to your need when deciding what to do next.
7. Propose the next bounded tranche only unless the human has already approved
   execution.
