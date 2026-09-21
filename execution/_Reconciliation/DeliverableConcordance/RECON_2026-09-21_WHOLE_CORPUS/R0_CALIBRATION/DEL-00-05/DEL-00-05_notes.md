# DEL-00-05 — R0 calibration notes

Worker: TASK (Type 2), run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`.
Forward ledger sealed at SHA-256
`de75117af723fa675c3feb143aba403bbc63e70a34a2a0ceb32bc4da80123bb3` (39 rows:
29 issued keys and 10 `.sNN` sub-claims). These notes do not edit it. These
dispositions are agent judgments. They are not owner rulings.

## Path aliases

- Evidence paths are repository-relative (`projects/chirality-piping/...`). I
  read them from the evidence checkout at `00115c719`.
- Aliases used in Notes: "session hooks" means the six
  `apps/desktop/src/features/workspace/*SessionState.ts` modules. "Operation
  engine" means `core/model_operations/operation_applier`, reached natively
  through the Tauri commands in `apps/desktop/src-tauri/src/lib.rs` and in the
  browser through its wasm32 build.
- Gate evidence (`GATE_EVIDENCE/PR834_CI`, `GATE_EVIDENCE/B4_4_SWEEP_9D55`) is
  not in the frozen tree, so the validator would reject it as a path. I cite it
  in Notes text only.
- Selectability: every row is `NOT_APPLICABLE`. Since 2026-09-19 Piping selects
  work through owner-steered work graphs, not `## Remaining` (C9).

## Judgment calls

1. **REQ-05-03, "preserve diagnostics".** Undo and redo clear stale mechanics
   results. They add an explicit solve-job event, and readiness status is then
   recomputed from the model. I read this as keeping diagnostic truth explicit
   and marked the row ALIGNED at MEDIUM confidence. If the owner means that
   undo should restore the earlier result diagnostics, the row becomes
   PARTIALLY_IMPLEMENTED.
2. **REQ-05-01.** The persisted envelope includes `selected_review_target` and
   `proposal`. I counted these as review context, not GUI selection, so the
   durable/transient split holds.
3. **REQ-05-05.** The desktop warning panel carries all six SPEC classes plus a
   `local_contract_class`. The Python DEL-07-04 contract
   (`core/gui/warnings/engine.py`) uses a local vocabulary with no nonlinear or
   IP class and no visible mapping. I treated that as a DEL-07-04/DEL-00-06
   matter, not a DEL-00-05 gap.
4. **Open holds.** The code has settled the state-library hold (React hooks,
   no library) and the undo-storage hold (in-memory whole-model snapshots, 25
   deep), but no ruling covers either. I marked them IMPLEMENTED_DIFFERENTLY /
   AUTHORITY_UNCLEAR. D-68 (2026-09-15) adopts, by reference to its plan, WCAG
   2.2 AA criteria for touched controls. That overtakes the accessibility hold:
   IMPLEMENTED_DIFFERENTLY / SCOPE_REDIRECTED_BY_RULING, flagged
   ADOPTED_BY_REFERENCE.
5. **Stale authority pointers.** The ArchitectureBasis surface, the
   currency-and-provenance block and resolved-decisions `.s01` cite
   SOFTWARE_DECOMP rev 0.9, DAG-007 and a §8.4 that does not exist. The frozen
   state is rev 0.12, with the decision log at §12, and DAG-010. The AB-00-05
   text is unchanged from rev 0.9 to 0.12, so the substance is aligned. I used
   STALE_REVIEW_OR_EVIDENCE / RECORD_DRIFT.
6. **`_CONTEXT.md`.** The banner discloses that this file is superseded setup
   context. I still rated each stale block STALE_SETUP_SPECIFICATION with
   AuthorityNeeded `NO`, except the surface row. The banner itself names rev
   0.9, so the surface row needs a repair (`OWNER`).
7. **`_STATUS.md`.** IN_PROGRESS matches D-40. SOFTWARE_DECOMP §13 calls PKG-00
   "retained reference context, not a production package awaiting issuance",
   which leaves IN_PROGRESS with no defined exit. I recorded this in Notes and
   made no lifecycle judgment (F-PIP-3).

## Convention friction

1. **No disposition for "substance aligned, pointer stale".** Stale revision
   and section pointers are forced into STALE_REVIEW_OR_EVIDENCE, which is
   about review evidence. *Fix:* add `STALE_REFERENCE`, or allow ALIGNED with a
   `POINTER_DRIFT` flag in Notes.
2. **No disposition for declared-state text overtaken by the code or a later
   ruling.** The open holds say OPEN, but the code or D-68 settled them.
   IMPLEMENTED_DIFFERENTLY suggests the code departed from a requirement,
   which is not what happened. *Fix:* add `DECLARED_OPEN_RESOLVED_IN_FACT`, or
   say in C6 that IMPLEMENTED_DIFFERENTLY covers declared-state rows.
3. **C1 sub-claims under blocks that also have numbered items.** C1 permits
   `.sNN` rows only where a block has "no numbered items".
   `AB#normative-requirements` has REQ items and also two unnumbered normative
   paragraphs (the OPS-K invariants and the boundary rules). Marking it
   COVERED_BY_CHILDREN without those paragraphs would have dropped them. I
   added `.s01` and `.s02`. *Fix:* allow `.sNN` for unnumbered normative
   residue in any block. Or have the extractor split table-plus-paragraph
   blocks.
4. **Declared-state container blocks with no numbered items.** Resolved
   decisions and open holds are tables or bullets, not REQ items, so the
   extractor issued no children. Every PKG-00 worker will split them by hand,
   inconsistently. *Fix:* the extractor should issue table rows and bullets in
   architecture-basis blocks as ITEM units.
5. **Evidence columns cannot hold gate evidence.** The validator checks each
   path against the frozen tree, and `GATE_EVIDENCE/` postdates the freeze.
   Suite pass status therefore ends up in Notes. *Fix:* add a
   `GateEvidenceRef` column, or allow a `GATE:` prefix token.
6. **Reverse pass for an architecture-basis deliverable.** CLAIMED_BY, PARTIAL
   and NOT_MINE assume the deliverable owns implementation. PKG-00 owns
   constraints: session-level mechanisms realize REQ-05-0x but belong to PKG-07.
   I used PARTIAL only for the mechanism itself (4 capabilities) and NOT_MINE
   for feature consumers. *Fix:* add a `CONSTRAINS <key>` answer for
   architecture-basis deliverables.
7. **Two dispositions are required on HISTORY rows.** C1 names only `MEMORY`
   as a single history unit. `STATUS#history` and `CONTEXT#preparation-notes`
   are also history, but must take a substantive disposition (I used ALIGNED).
   *Fix:* extend the MEMORY rule to any HISTORY-typed block.
8. **ADOPTED_BY_REFERENCE has no field.** A3 puts it in Notes, where it cannot
   be counted. *Fix:* a boolean column, or a reserved Notes prefix.

## Unit-grain observations

- The grain was mostly right. REQ-05-01..05 map one-to-one to the AB-00-05
  row, and every `_CONTEXT.md` block is a clean unit.
- Too coarse for architecture-basis tables and bullets (friction 3 and 4). I
  added 10 sub-claims to 29 keys.
- Too fine for `_CONTEXT.md` under a supersession banner. Twelve CONTEXT keys
  are all governed by one banner. A banner-aware surface row would have been
  enough, with blocks sampled.
- `STATUS#remaining` is empty, so there are no `/Rnn` units. The single block
  row carried it adequately.

## UNKNOWN rows

None. No row is UNKNOWN. The nearest to one are the three
AUTHORITY_UNCLEAR/owner rows (open-holds `.s01`–`.s03`). For each, the smallest
next check is to ask the owner whether the implemented choice closes the hold.

## Did the reverse pass change my view of the sealed ledger?

Only slightly.

- **Undo/redo.** CAP-WS-021 and CAP-WS-010 show that undo is also driven by the
  keyboard and the native menu. REQ-05-03's evidence cites only the session
  handlers and tests. This adds evidence and does not change the disposition.
- **UI preferences.** CAP-WS-028 is a third state category that REQ-05-01 does
  not name: UI preferences persisted in browser storage, neither project state
  nor transient. It is consistent with the split. If the owner wants that
  category covered, the AB-00-05 text should mention it. REQ-05-01 would stay
  ALIGNED.
- **Realized-artifacts table.** CAP-WS-012 confirms my note on
  `AB#realized-artifacts`. The table omits `features/workspace`, where the state
  architecture actually lives.
- **REQ-05-05.** No inventory capability gives GUI warning-class evidence
  beyond my forward citations. The pilot inventory has no missing-data or
  warning panel rows, so it could not test REQ-05-05.

Reverse answers: 4 PARTIAL, 92 NOT_MINE, 0 CLAIMED_BY.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
