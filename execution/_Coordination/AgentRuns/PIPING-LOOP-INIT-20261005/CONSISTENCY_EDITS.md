# Consistency edits for the Piping LOOP_INIT change

Prepared by LM, a Type 2 TASK, for HELP_HUMAN (ROOT), run
`PIPING-LOOP-INIT-20261005`. No file was edited. These are the minimal edits
that keep other instructions true once `P/loop/LOOP_INIT.md` is replaced by
`LOOP_INIT_PROPOSED.md`.

**Method.**
- Each "Before" block is the exact current text, with its line breaks, at
  NUM HEAD `6c2cb5a4eb`. Each was checked to occur exactly once in its file
  by a byte-level count. Replace it with the "After" block.
- Sources were found by searching `P` (its root files, `init/`, `loop/` and
  `docs/`) and `NUM` (outside `execution/`, `plans/`, archives and evidence)
  for `LOOP_INIT`, "owns the recurring", "recurrent development procedure" and
  "pointer index live", and by reading
  `P/execution/_Coordination/_COORDINATION.md`, which the draft names.
- No validator reads the phrases changed here. The newest WORKPLAN edit (A7)
  passes the structural-duplication function of
  `NUM/tools/validation/validate_instruction_entrypoints.py`, applied in a
  scratch copy.

## A. Required: made untrue by the change

### A1. `P/AGENTS.md`, lines 11–13

The file's opening says LOOP_INIT is the procedure.

Before:
```text
Root `AGENTS.md` and the selected `agents/AGENT_*.md` package govern agent roles
and delegation. This file holds Piping-specific constraints. The recurrent
development procedure is `loop/LOOP_INIT.md`; the init prompt enters it.
```

After:
```text
Root `AGENTS.md` and the selected `agents/AGENT_*.md` package govern agent roles
and delegation. This file holds Piping-specific constraints. The init prompt
enters the development loop at `loop/LOOP_INIT.md`, which binds Piping to the
shared workflows and manuals that carry its recurring procedure.
```

### A2. `P/AGENTS.md`, lines 38–39

LOOP_INIT will no longer describe the local work graph; it adopts the method
that does.

Before:
```text
the old loop procedure does not waive those boundaries. It supersedes F-PIP-5's
deliverable-only work-selection procedure with the local work graph described in `loop/LOOP_INIT.md`.
```

After:
```text
the old loop procedure does not waive those boundaries. It supersedes F-PIP-5's
deliverable-only work-selection procedure with the local work graph that `loop/LOOP_INIT.md` adopts.
```

### A3. `P/AGENTS.md`, lines 60–62

Before:
```text
`loop/LOOP_INIT.md` owns the recurring development procedure. Its named
workflows provide bounded methods; other historical plans and coordination
records do not supply alternate loop mechanics. The development init prompt
```

After:
```text
`loop/LOOP_INIT.md` binds the recurring development procedure to its named
workflows and the shared manuals; other historical plans and coordination
records do not supply alternate loop mechanics. The development init prompt
```

**Checked in `P/AGENTS.md`, no change.**
- Lines 64–66, "Keep LOOP_INIT evergreen, with no undertaking-specific graph
  pointer or execution state.": still true.
- Lines 86–87, "Record one central loop receipt and terse MEMORY run rows near
  final PR preparation under LOOP_INIT.": still true, since LOOP_INIT adopts
  construct's receipt and MEMORY conventions and holds the MEMORY convention.
- No sentence cites LOOP_INIT's numbered steps.

### A4. `NUM/init/dev-loop-init-prompt.md`, §5, lines 156–158

The Root launcher's Piping paragraph. "fences" were never in LOOP_INIT (they
are in project `AGENTS.md`), and "receipt references" lead to the closed
ledger. The tagged launcher block above it is unchanged, so it still
byte-matches `P/init/dev-loop-init-prompt.md`.

Before:
```text
The recurrent procedure, fences, and pointer index live in
`projects/chirality-piping/loop/LOOP_INIT.md`; follow its live discovery
pointers and receipt references. This supersedes the older
```

After:
```text
`projects/chirality-piping/loop/LOOP_INIT.md` binds the loop to the shared
workflows and manuals and holds its entry reading and record pointers; the
project `AGENTS.md` holds its fences. This supersedes the older
```

### A5. `P/execution/_Coordination/_COORDINATION.md`, lines 3–4

The header note says LOOP_INIT is the procedure.

Before:
```text
> **Current procedure:** `loop/LOOP_INIT.md` is the sole recurring development
> procedure. Read the historical rules and phase decisions below for their
```

After:
```text
> **Current procedure:** `loop/LOOP_INIT.md` is the sole development-loop entry;
> it binds Piping to the shared workflows and manuals that carry the recurring
> procedure. Read the historical rules and phase decisions below for their
```

### A6. `P/docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, lines 18–19

The banner says LOOP_INIT "owns" the procedure. This file is a DEL-11-05
document; its `_STATUS.md` reads IN_PROGRESS, and BR §1 forbids edits only to
CHECKING or ISSUED deliverables or those inside an active concordance run's
frozen scope. I did not establish whether
`execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`
is active or covers DEL-11-05; ROOT should confirm. The banner was added by an
earlier instruction tranche (commit `29decb9fee`).

Before:
```text
> **Loop procedure reference:** `loop/LOOP_INIT.md` owns the recurring development
> procedure. Older agent names, workplan selectors and development step tables
```

After:
```text
> **Loop procedure reference:** `loop/LOOP_INIT.md` binds the recurring development
> procedure to the shared workflows and manuals. Older agent names, workplan
> selectors and development step tables
```

### A7. `P/loop/WORKPLAN_2026-09-19_piping_loop.md`, lines 6–10

The newest WORKPLAN, which `P/README.md` sends agents to and the entrypoint
validator checks. Its first sentence becomes untrue. Its clause "with
LOOP_INIT pointing to the selected graph" has been untrue since the evergreen
change of 2026-09-23. The file says "Current navigation below follows the
subsequently adopted shared loop instructions", so its navigation is
maintained; the owner-adopted retirement text (lines 14–19) is unchanged.

Before:
```text
The current reusable development procedure is `loop/LOOP_INIT.md`;
`projects/chirality-piping/AGENTS.md` supplies standing responsibilities and
constraints. Current local development graphs are Git-tracked at
`execution/_Coordination/WorkGraphs/<undertaking>/WORK_GRAPH.md`, relative to the
project, with LOOP_INIT pointing to the selected graph. AgentRuns holds linked
```

After:
```text
The current development-loop entry is `loop/LOOP_INIT.md`, which binds Piping
to the shared workflows and manuals that carry the reusable procedure;
`projects/chirality-piping/AGENTS.md` supplies standing responsibilities and
constraints. Current local development graphs are Git-tracked at
`execution/_Coordination/WorkGraphs/<undertaking>/WORK_GRAPH.md`, relative to the
project, and the human's steering selects the graph. AgentRuns holds linked
```

**Checked elsewhere, no change.**
- `P/init/dev-loop-init-prompt.md`: "Read `{WORKING_ROOT}/loop/LOOP_INIT.md`
  and follow it within the owner's steering and live authority." stays true.
  No change is needed. Any change would also have to be made, byte for byte,
  to the Root launcher's §5 block.
- `P/init/taskmgmt-init-prompt.md`: "App/Piping development continues through
  its development init prompt and `loop/LOOP_INIT.md`." stays true.
- `NUM/init/dev-loop-init-prompt.md`, lines 29–30 (the catalog entry): stays
  true.
- `NUM/docs/SPEC.md`, the App/Piping graph paragraph: "LOOP_INIT remains
  evergreen and carries no undertaking-specific pointer or execution state."
  stays true.
- `NUM/workflows/construct-local-work-graph/resources/work-graph-template.md`:
  its two LOOP_INIT mentions stay true.

## B. Recommended: already stale, found in passing

These were untrue before this change and stay untrue after it. They route
entry through retired surfaces. Include them only if ROOT agrees (mapping Q5).

### B1. `P/README.md`, line 43

Before:
```text
1. Start from `init/dev-loop-init-prompt.md` → `loop/LOOP_INIT.md` → the newest `loop/WORKPLAN_*.md` (the development loop instructions: discovery, work selection from deliverable folders, execution discipline, validation, and handoff). `execution/_Coordination/_COORDINATION.md` remains the ruled-record surface (current target stage).
```

After:
```text
1. Start from `init/dev-loop-init-prompt.md` → `loop/LOOP_INIT.md`, which binds the development loop to the shared workflows and manuals; `AGENTS.md` holds the standing constraints. `execution/_Coordination/_COORDINATION.md` remains the ruled-record surface (current target stage).
```

### B2. `P/docs/README.md`, line 58

The same sentence as B1, with the same replacement.

Before:
```text
1. Start from `init/dev-loop-init-prompt.md` → `loop/LOOP_INIT.md` → the newest `loop/WORKPLAN_*.md` (the development loop instructions: discovery, work selection from deliverable folders, execution discipline, validation, and handoff). `execution/_Coordination/_COORDINATION.md` remains the ruled-record surface (current target stage).
```

After:
```text
1. Start from `init/dev-loop-init-prompt.md` → `loop/LOOP_INIT.md`, which binds the development loop to the shared workflows and manuals; `AGENTS.md` holds the standing constraints. `execution/_Coordination/_COORDINATION.md` remains the ruled-record surface (current target stage).
```

### B3. `P/execution/_Coordination/_COORDINATION.md`, lines 103–104

The "Pointers" list. LOOP_INIT will name COORD for the target stage, so agents
will read this file.

Before:
```text
- Session entry: `init/dev-loop-init-prompt.md` → `loop/LOOP_INIT.md` → the newest
  `loop/WORKPLAN_*.md` → `loop/LOOP_RECEIPTS.md`.
```

After:
```text
- Session entry: `init/dev-loop-init-prompt.md` → `loop/LOOP_INIT.md`.
```

## C. Listed only: outside this tranche

### C1. Agent User Manual citations of the Piping loop

`NUM/docs/alignment-manual/CHIRALITY_AGENT_USER_MANUAL_v3.md`. The link
`[piping-loop]` (line 849) resolves to `P/loop/LOOP_INIT.md`. Manual revisions
are out of scope; the 2026-09-23 notice records that "The owner retains
authorship of the manual/guide update."

Section numbers that will not exist:
- line 104 (§2), "[Piping loop §§0–1]";
- line 524 (§13), "[Piping loop §5]";
- line 613 (§15), "[Piping loop §§3–6]";
- line 673 (§18), "[Piping loop §0]";
- line 690 (§18), "[Piping loop §6]".

Unnumbered citations whose content will no longer be in the loop file (the
statement stays true; the citation should name the workflow, FB section or
AUM section that now carries it):
- line 102 (§2), recovery of five things;
- line 387 (§9), walking the route before dispatch;
- line 682 (§18), receipt-cursor validation (LOOP_INIT keeps only the
  ledger's status);
- line 769 (§20), the stale continuation summary.

Already stale before this change:
- line 104 (§2): "Both checked headers say “none selected for a successor
  undertaking.”" Piping's file has had no such header since 2026-09-23.
- line 601 (§15): "At this basis, the published Piping loop says “none
  selected for a successor undertaking.”"
- line 605 (§15): "Recover the graph selected by the live loop, then its
  linked resume procedure, phase events, handoff, and later owner
  directions." The live loop selects no graph.

Still accurate: lines 62, 347 and 603.

### C2. Consolidated v8

`NUM/docs/alignment-manual/Project_Management_for_Human_Agent_Teams_Consolidated_v8.md`
still tells loops to point LOOP_INIT at the graph, against the evergreen rule
(stale since 2026-09-23, not caused by this change):
- line 1678: "keep it current, and point LOOP_INIT to its actual location."
- line 1782: "and LOOP_INIT must point there."

### C3. `construct-local-work-graph` (needs `create-workflow`)

`NUM/workflows/construct-local-work-graph/WORKFLOW.md`, line 11. Root AGENTS
requires `create-workflow` for any workflow revision, so this is not proposed
for this tranche.

Before:
```text
The human's steering selects the undertaking; `LOOP_INIT.md` supplies the
evergreen procedure for recovering, constructing and following its graph.
```

Proposed for a later `create-workflow` revision (App v4's A4, still unapplied):
```text
The human's steering selects the undertaking; `LOOP_INIT.md` supplies, or
points to, the evergreen procedure for recovering, constructing and following
its graph.
```

### C4. Historical records

`NUM/docs/alignment-manual/MANUAL_REVIEW_v1.md` (line 68) and the tranche
manifests under `NUM/docs/governance_harness/tranche_manifests/` describe the
loop as it was. They are historical; no action.
