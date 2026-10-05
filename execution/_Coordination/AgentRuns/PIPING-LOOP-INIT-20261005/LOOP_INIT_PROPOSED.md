# Proposed LOOP_INIT.md (draft for HELP_HUMAN)

Drafted by LM, a Type 2 TASK, for HELP_HUMAN (ROOT), run
`PIPING-LOOP-INIT-20261005`. It is a proposal: the live `P/loop/LOOP_INIT.md`
is unchanged. The basis is `RUN/OWNER_DECISIONS.md` and the two sections of
`V4RUN/OWNER_DECISIONS.md` on LOOP_INIT. `LOOP_INIT_MAPPING.md` accounts for
every sentence of the current file, and `CONSISTENCY_EDITS.md` gives the edits
needed elsewhere.

Drafting notes. They are not part of the proposed text.
- Shape and voice follow `NUM/projects/chirality-app-v4/loop/LOOP_INIT.md`.
  The facts are Piping's. The opening names project `AGENTS.md`, and entry
  step 2 reads it (mapping N3).
- Every path was checked with `test -e` relative to `P`, or `NUM` for the
  alignment-manual README. Every workflow name was checked in
  `NUM/workflows/index.json` (bundled, `chirality-root`).
- No host path, undertaking name, current ID or date of current work appears.
  "Receipt 162" names where a closed historical ledger ends, as the brief
  asks; "PKG-00" names where a bespoke production form lives.
- Open choices for ROOT: the MEMORY creation clause (mapping Q1),
  `coordinated-knowledge-work` (Q2), and the last sentence of "When to read
  further" (Q3).
- `_structural_duplication_findings` from
  `NUM/tools/validation/validate_instruction_entrypoints.py`, applied to this
  text in a scratch copy, returned no findings.
- The proposed text runs from the next heading to the end of the file. It is
  87 lines and 3,875 characters, about 1,020 tokens (characters / 3.8). The
  current file is 146 lines and 8,475 characters, about 2,230 tokens.

---

# Piping development loop

Resolve `REPO_ROOT` from the active checkout and set `WORKING_ROOT` to
`{REPO_ROOT}/projects/chirality-piping`. Paths below are relative to
`WORKING_ROOT` unless they begin with `{REPO_ROOT}`.

This file binds Piping to Root `AGENTS.md`, the active role, project
`AGENTS.md`, the bundled workflows and the manuals. It states only what is
specific to Piping and stated in none of them. The human's steering selects
the undertaking, and its work graph carries the state.

## Entry reading

1. Take the current editions from
   `{REPO_ROOT}/docs/alignment-manual/README.md`. Read the Agent User
   Manual's headings to three levels (`grep -nE '^#{1,3} '` on its
   Markdown), then read the Field Book in full.
2. Read project `AGENTS.md` in full before you write anything.
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
Root `AGENTS.md` requires. If you are unsure whether something matters, ask
the human.

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
  revision named by `execution/_Decomposition/_LATEST.md`. It records each
  adopted amendment, and `execution/_ScopeChange/_LATEST.md` selects the
  accepted scope change.
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
- **Checks.** `software-workflow.json`, as project `AGENTS.md` describes
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

- Piping's stage gates are the exit criteria of the release milestones in
  `docs/PRD.md` §24. `execution/_Coordination/_COORDINATION.md`, under
  "Current Target Stage", records the target stage; only the owner's
  approved update advances it.
