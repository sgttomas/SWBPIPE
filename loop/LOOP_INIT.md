# Piping development loop

Resolve `REPO_ROOT` from the active checkout and set `WORKING_ROOT` to
`{REPO_ROOT}/projects/chirality-piping`. Paths below are relative to
`WORKING_ROOT` unless they begin with `{REPO_ROOT}`.

This file binds Piping to Root `AGENTS.md`, the active role,
`loop/PROJECT_GUIDANCE.md`, the bundled workflows and the manuals. It states only what is
specific to Piping and stated in none of them. The human's steering selects
the undertaking, and its work graph carries the state.

## Entry reading

1. Take the current editions from
   `{REPO_ROOT}/docs/alignment-manual/README.md`. Read the Agent User
   Manual's headings to three levels (`grep -nE '^#{1,3} '` on its
   Markdown), then read the Field Book in full.
2. Read `loop/PROJECT_GUIDANCE.md` in full before you write anything.
3. Read the work graph of the undertaking the steering names. If it has
   none, construct one.

## When to read further

Go to a section:
- when you are about to make a choice it addresses;
- when something contradicts what you expected;
- before you prepare a decision for the human;
- when you enter unfamiliar work.

Read the section, not the chapter. Instructions and the human's decisions
govern; the manuals explain. Record what you read in the run evidence, as
Root `AGENTS.md` requires. If you are unsure whether a section
matters, read it.

## Methods

Load each workflow when it is needed, as
`chirality-root:bundled:workflow:<name>`:
- `construct-local-work-graph`: Piping adopts its graph, closeout, receipt
  and MEMORY conventions;
- `coordinated-knowledge-work`, for coordination and delegation, applied in
  proportion to the work;
- `bounded-reconciliation`, for the closeout comparisons;
- `task-management`: Piping permits bounded intake, through WORKING_ITEMS, of
  a material, evidenced concern that has no current or identified successor
  home;
- `scope-change`, `dependency-extract`, `audit-dep-closure` and
  `project-dag`, when a change reaches the decomposition, the registers or
  the DAG.

## Piping records

- **Basis.** `docs/PRD.md`.
- **Decomposition.** `execution/_Decomposition/SOFTWARE_DECOMP.md`, at the
  revision named by `execution/_Decomposition/_LATEST.md`. Its revision notes
  record each amendment that changed it, and `execution/_ScopeChange/_LATEST.md`
  selects the latest accepted one.
- **DAG.** `execution/_DAG/_LATEST.md` names the accepted version and how its
  currency is decided.
- **Deliverables.** `execution/PKG-*/1_Working/DEL-*/`: the production
  contract (`ScopeOfWork.md`, or an accepted bespoke form such as PKG-00's
  `ArchitectureBasis.md`), `_DEPENDENCIES.md`, `Dependencies.csv`,
  `_STATUS.md` and `MEMORY.md`.
- **Decisions.** `execution/_Coordination/_DECISIONS/_REGISTER.md` tracks
  decision packets and points to each ruling; its header says how to record
  one. Codified rulings are the `DEC` entries in the decomposition's §12.
  Notices are `execution/_Coordination/NOTICE_*.md`, and run records are in
  `execution/_Coordination/AgentRuns/<RunID>/`.
- **Checks.** `software-workflow.json`, as `loop/PROJECT_GUIDANCE.md` describes
  under "Software checks".
- **Task Management.** `execution/_Coordination/_TaskManagement/REGISTER.csv`.
- **Work graphs.** `execution/_Coordination/WorkGraphs/`.

## Conventions

- **MEMORY.** Add a terse Runs entry to each affected deliverable's
  `MEMORY.md`, and create the file the first time it is needed.
- **LOOP_RECEIPTS.** `loop/LOOP_RECEIPTS.md` is a closed historical ledger
  that ends at Receipt 162. Do not append to it, and do not use it as the
  receipt or the recovery cursor of a new undertaking.

## Standing constraints

- Piping's stage gate is the target stage and the exit criteria recorded in
  `execution/_Coordination/_COORDINATION.md` under "Current Target Stage".
  Assess against those criteria, not against a `docs/PRD.md` §24 milestone
  read by its label; only the owner's approved update advances the target.
