# Brief LM: map piping's LOOP_INIT and draft its binding form

TASK (Type 2) for HELP_HUMAN (ROOT), run `PIPING-LOOP-INIT-20261005`. You return to ROOT and do not delegate. You make no Git writes. Write only the outputs below.

**Placeholders.**
- `WT` = the T3 worktree root.
- `NUM` = `WT/numerics`, the checkout to read and write.
- `RUN` = `NUM/projects/chirality-piping/execution/_Coordination/AgentRuns/PIPING-LOOP-INIT-20261005`.
- `P` = `NUM/projects/chirality-piping`.
- `V4RUN` = `NUM/projects/chirality-app-v4/execution/_Coordination/AgentRuns/APP-V4-GRAPH-CLOSURE-20261004`.

## The basis

**The owner's decisions:**
- `RUN/OWNER_DECISIONS.md`.
- `V4RUN/OWNER_DECISIONS.md`, the sections "How LOOP_INIT steers agents through the guidance" and "What LOOP_INIT is, and what an agent reads at entry". Their test: a sentence stays in LOOP_INIT only if it is specific to the project, instructs, and is stated nowhere else. Also: no routers; no new record type for current state; no edition pins; the entry reading is the Agent User Manual's headings to three levels plus the Field Book in full, then the work graph.

**The precedent:**
- App v4's change: `V4RUN/LOOP_INIT_MAPPING.md` (the method and classes, and destination quotes for the legacy text), `V4RUN/reviews/MR-LOOPINIT.md` (the review, and what it caught) and `NUM/projects/chirality-app-v4/loop/LOOP_INIT.md` (the result).
- Piping's current text, `P/loop/LOOP_INIT.md`, is App v3's legacy text, nearly word for word: `diff` it with `NUM/projects/chirality-app-dev/loop/LOOP_INIT.md`. Most of its sentences have an App v4 mapping row, but **re-verify every destination quote against the files at NUM HEAD.** Don't copy rows on trust.

**The general layers:**
- Root `AGENTS.md`;
- `NUM/agents/AGENT_HELP_HUMAN.md`;
- the bundled workflows `construct-local-work-graph`, `coordinated-knowledge-work`, `bounded-reconciliation`, `task-management`, `scope-change`, `dependency-extract`, `audit-dep-closure` and `project-dag`, in `NUM/workflows/` (check the names against `workflows/index.json`);
- the manuals, at the current editions named by `NUM/docs/alignment-manual/README.md`. The Agent User Manual's §15 is "Enter Piping development".

**Piping's own instructions:** `P/AGENTS.md`. It already states the fences (F-PIP-1 to F-PIP-4), the knowledge-source rule, review, DEC-025 for product-code merges, evidence, and the records rules. Do not repeat it in LOOP_INIT.

## Outputs, all in `RUN/`

1. **`LOOP_INIT_MAPPING.md`.** A table of every sentence or clause of `P/loop/LOOP_INIT.md`, with line numbers, class (a, b, c or d) and destination.
   - **The classes:** (a) carried elsewhere, with a verbatim quote; (b) piping-specific, so it stays; (c) general and found nowhere else, with a proposed home and text; (d) obsolete or superseded, with the reason.
   - **Quotes:** check each one verbatim with `grep -F` after joining wrapped lines, and say how you checked.
   - **Then list:**
     - the draft lines not in the current file, with their sources;
     - any class (c) additions needed elsewhere;
     - any gaps you met.
2. **`LOOP_INIT_PROPOSED.md`.** The full proposed `P/loop/LOOP_INIT.md`, in App v4's shape and voice, but with **piping's** facts. The content:
   - the opening: `REPO_ROOT`/`WORKING_ROOT`, and the binding sentence, which must also name piping's project `AGENTS.md`;
   - Entry reading: the manual headings and the Field Book; then project `AGENTS.md`, if your mapping shows it belongs here; then the work graph the steer names, or construct one;
   - When to read further;
   - Methods, with piping's adoption or permission lines where these are piping-specific. The task-management intake condition is an example.
   - Piping records. These are pointers that actually exist:
     - `docs/PRD.md`;
     - `execution/_Decomposition/SOFTWARE_DECOMP.md`, with adopted amendments and design specifications (check `execution/_ScopeChange/`);
     - `execution/_DAG/_LATEST.md`;
     - deliverables under `execution/PKG-*/1_Working/DEL-*/`, including Architecture Basis and bespoke contracts;
     - decisions in `execution/_Coordination/_DECISIONS/_REGISTER.md` and run records;
     - `software-workflow.json`;
     - the Task Management register `execution/_Coordination/_TaskManagement/REGISTER.csv`;
     - `execution/_Coordination/WorkGraphs/`.
   - Conventions:
     - `loop/LOOP_RECEIPTS.md` is a historical ledger ending at Receipt 162. Check that, and say what it now means for agents.
     - MEMORY.
   - Standing constraints, only those that are piping-specific and stated nowhere else. AUM §5 says "The project loop states who assesses the position"; check how that applies to piping.
   - **Nothing machine-local, and no current state.** No host paths, undertaking names, IDs or dates of current work.
3. **`CONSISTENCY_EDITS.md`.** The exact minimal edits, as before and after text, needed elsewhere so that instructions stay true once LOOP_INIT changes. At least:
   - the sentences in `P/AGENTS.md` that say LOOP_INIT "owns" or "describes" the recurring procedure, or that cite its steps;
   - the Root launcher's piping paragraph in `NUM/init/dev-loop-init-prompt.md` §5, which says "The recurrent procedure, fences, and pointer index live in" LOOP_INIT;
   - `P/init/dev-loop-init-prompt.md`, only if a change is needed.
   
   Do not propose a wholesale revision of AGENTS.md. List, but do not edit, the stale citations you find in the manuals (for example AUM §15's "[Piping loop §§3–6]"), since manual revisions are out of scope.

## Rules

- Read whole files only where needed. Read sections, not chapters.
- Check every path in your draft with `test -e` (relative to `P` or `NUM`), and every workflow name against `workflows/index.json`.
- Use no machine-absolute paths in your outputs. Write `P/…`, `NUM/…`, `{REPO_ROOT}`, `{WORKING_ROOT}`.
- No cargo, tests, network, installs, or writes outside `RUN/`.
- **Time box:** 75 minutes.
- **End your turn** with the counts by class, the draft's length (lines and approximate tokens) against the current file's, the paths and names checked, any open question for ROOT, and the sha256 of each output.
