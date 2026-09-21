# DEL-07-02 calibration notes (R0)

Worker: TASK (Type 2), run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`.
Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. These are agent
calibration observations, not owner rulings. Standard claim fence applies
(F-PIP-2; claims taxonomy per DEC-081).

Sealed forward ledger: `DEL-07-02_forward.csv`, SHA-256
`aadaa5b7c04e5c85277328f574d89a2667c9e9a989e1b3c73f6431da8774fe1e`, 88 rows
(83 issued keys plus `CLM-004.s01` to `.s05`). It was not edited after sealing.

Selectability: every row is `NOT_APPLICABLE` (C9). Since 2026-09-19, Piping
selects work through owner-steered work graphs, so no row is selectable under
`## Remaining`.

## Path aliases

- `KIT` = `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/`.
- `SRC` = `projects/chirality-piping/apps/desktop/src/`. The main surfaces
  are `features/model-tree/ModelTree.tsx`, `features/model-tree/PropertyInspector.tsx`,
  `services/operationService.ts`, `features/workspace/selectionState.ts`, `features/workspace/modelIndex.ts`
  and `features/viewport/viewportSelection.ts`.
- `GATE` = the run folder's `GATE_EVIDENCE/PR834_CI` and `GATE_EVIDENCE/B4_4_SWEEP_9D55`.
  These are cited only in `ContextRefs`: they are not in the frozen tree, so
  the validator's path check would reject them in the evidence columns.
- `tools/validation/validate_claims_language.py` is a root-relative path.
  Every other path in the ledger is relative to the repository root, under
  `projects/chirality-piping/`.

## Judgment calls

- **CLM-004 split.** CLM-004 (the Attributes table) holds five distinct claims,
  so I split it into sub-claims. Other multi-row tables (CLM-005, CLM-008,
  CLM-010, CLM-017, CLM-018, CLM-025, CLM-033) are assessed as one row each,
  because their rows share a single disposition or a single dominant gap.
- **Blocks assessed directly.** CLM-020 and CLM-027 are assessed as blocks,
  not containers: their only child items (AC-001, VER-001) do not carry the
  block's own content.
- **CLM-003 marked non-normative.** CLM-003 (the identification table) is
  `NON_NORMATIVE` under C1, even though its delegation row is normative. That
  delegation is carried by CLM-007 and CLM-014.
- **Stale currentness declarations.** The PDU-055 declarations (CLM-002 and
  CLM-012) and the CLM-009 references are `STALE_REVIEW_OR_EVIDENCE` with
  cause `EVIDENCE_OVERTAKEN`. They cite decomposition revision 0.7/0.8 and
  DAG-007, while the frozen state is revision 0.12 and DAG-010.
  `STALE_SETUP_SPECIFICATION` is kept for real setup-era wording (the TBD
  slots, `_CONTEXT.md`).
- **Remaining items.** Per A4, each Remaining item is dispositioned by whether
  its text matches the frozen evidence. Landed-state narrative that matches
  (R01, R09, R10) is `ALIGNED`, even though it holds no open work. The
  mismatches:
  - R02 calls the criteria prospective, but D-72 ruled them.
  - R03 describes a pre-merge gate, but PR #789 is merged.
  - R08's landed baseline stops before the D-68 box-select gestures.
  - R11 cites a file that does not exist anywhere in the frozen tree.
- **Verification hooks.** Requirement verification hooks that still say
  "future tests" or "once contracts are accepted" did not downgrade an
  otherwise `ALIGNED` requirement. I recorded them in `Notes`.
- **RQ-006 reads.** The requirement says tree and inspector reads use a
  governed query or result-envelope boundary. In the code they read the
  session model that the project and operation services deliver, not a
  per-read query. I treated that as within the boundary (`ALIGNED`, MEDIUM
  confidence).
- **Ownership questions.** Creation forms and grid bulk editing are hosted by
  DEL-07-02 but are not described in its SOW. CLM-034 records this as
  `IMPLEMENTED_UNDOCUMENTED` with cause `AUTHORITY_UNCLEAR`, not
  `AUTHORITY_CONFLICT`: the sources are silent rather than in conflict, and
  the DEL-07-03 SOW does not reference these surfaces.
- **Adopted by reference.** D-68 adopts an approved plan by reference, so it
  is flagged `ADOPTED_BY_REFERENCE` in the SOW surface row's notes.

## Convention friction

1. **No divergence layer for documentation-only staleness.** C5 layers cover
   claims, validation, IP, baseline, security and lifecycle. Setup-era text
   and superseded authority pointers get `NONE`, which reads as "no
   divergence". Smallest fix: add a `DOCUMENTATION` layer, or define `NONE`
   as "no protected layer affected".
2. **`UNKNOWN` forces a cause tag.** The validator requires a `CauseTag` on
   every non-aligned row, including `UNKNOWN`, but no tag means "evidence not
   yet located". Three rows needed `OTHER` plus an explanation (RQ-008, R04,
   matrix OUT-001). Smallest fix: add `EVIDENCE_NOT_LOCATED`, or exempt
   `UNKNOWN` from the cause tag.
3. **A4 versus mis-filed Remaining items.** A4 judges only whether an item's
   text matches the evidence. It gives no way to say "this Remaining item is
   landed history, not open work" (R01, R09, R10, R11). That is a real R6
   finding that `ALIGNED` hides. Smallest fix: allow
   `REMAINING_STATE_MISMATCH` with `RECORD_DRIFT` when an item has no open
   action, or add a boolean `OpenAction` column for `STATUS#remaining` rows.
4. **Container rule ignores tables beside items.** The extractor issues
   `ITEM` children (AC-001, VER-001) inside blocks whose table substance
   those items do not carry. C1 only covers "fully carried" or "no numbered
   items". Smallest fix: state that such a block is assessed directly and is
   not a `CONTAINER`.
5. **Currentness declarations re-delegate to Remaining.** The PDU-055
   declarations say the surviving residuals are "those recorded in `_STATUS.md
   ## Remaining`". Under A4, that makes a SOW claim point at a non-authority.
   There is no convention for a governing surface that delegates to Remaining.
   Smallest fix: under A4, treat such a declaration as `DECLARED_STATE` whose
   residual clause cannot be relied on; say this in C1 or A4.
6. **Missing `STALE_REVIEW_OR_EVIDENCE` versus `STALE_SETUP_SPECIFICATION`
   guidance.** Declarations dated 2026-07 are not setup-era, but their
   authority is overtaken. I used `STALE_REVIEW_OR_EVIDENCE`; another worker
   could reasonably choose the other value. Smallest fix: give a one-line rule
   for each (setup-era origin versus later declaration or reference
   overtaken).
7. **Gate evidence has no citable path.** Gate evidence lives outside the
   frozen tree, so it cannot go in `VerificationEvidence`. Smallest fix: let
   the validator accept a `GATE:` token prefix resolved against the run
   folder.
8. **Reverse pass: `PARTIAL` is overloaded.** `PARTIAL` means both "my claim
   covers part of this capability" and "my key covers it only through a
   landed-state Remaining narrative". Several `PARTIAL` answers point at R01
   or R05 only because no SOW key exists. Smallest fix: add an answer
   `ONLY_IN_DECLARED_STATE`, or require naming the closest SOW key plus a
   gap note.

## Unit-grain observations

- The `CLM-NNN` block grain fit the prose blocks. It was too coarse for
  multi-claim tables such as CLM-004, which needed sub-claims, and too fine
  for heading-only blocks. Four blocks (CLM-001, CLM-011, CLM-021, CLM-028)
  are pure headings that the extractor could fold into their section.
- Duplicated declarations (CLM-002/CLM-012 and CLM-027/CLM-037) and the
  duplicated conflict tables (CLM-020/CLM-036) double the row count without
  adding information. Grouping duplicates would help verifiers.
- The sub-claim `UnitKind` value `SUBCLAIM` is accepted by the validator but
  not named in Part D. Part D should list it.
- A single `MEMORY` row for 989 lines is right for history, but it hides that
  the top entry repeats the stale R03 wording.

## Smallest next check for each UNKNOWN row

- `SOW#CLM-015/DEL-07-02-RQ-008`: locate or run a protected-content and
  fixture-provenance review over the desktop preview fixture, the e2e
  snapshots, and the run-record screenshots DEL-07-02 cites.
- `STATUS#remaining/R04`: read the V99 and V103 native inspector observation
  records to confirm the AX omission is still open at the freeze.
- `SOW#output-and-evaluation-matrix/OUT-001`: have a worker allowed to read
  the D-41 concordance ledgers confirm whether their DEL-07-02 rows are the
  expected claim map and parity report.

## Effect of the reverse pass on the sealed forward ledger

The reverse pass leaves the sealed dispositions in place. It strengthens
three points and exposes one gap:

- **The CLM-034 ownership finding is broader than sealed.** Four capabilities
  sit in DEL-07-02 code with no SOW key: grid bulk editing (CAP-TREE-005/006/007),
  creation forms (CAP-TREE-012), deletion intents (CAP-TREE-013) and schema-slot
  entry (CAP-TREE-011). CLM-034 and the SOW surface row already flag the gap,
  but it covers more capabilities than the sealed notes list.
- **Inspector hosts panels it does not own.** CAP-TREE-016: the inspector
  hosts material-temperature, wind-exposure and section-assignment panels
  that no DEL-07-02 key mentions. That is a hosting-versus-ownership
  question the sealed ledger does not raise.
- **Selection ownership overlap.** CAP-WS-022: R01 describes shared typed
  selection as landed in this deliverable, while CLM-014 gives the
  selection-model architecture to DEL-00-05. I sealed R01 as `ALIGNED` on its
  text; in hindsight it also carries an ownership overlap worth an R4 note.
  The row stays as sealed.
- **No inventory row for the Python contract module.** The inventory has no
  row for `core/gui/model_tree/engine.py`, which the sealed CLM-019 row names
  as an undocumented DEL-07-02 artifact. R3 should check whether that module
  is in the unmapped set.
