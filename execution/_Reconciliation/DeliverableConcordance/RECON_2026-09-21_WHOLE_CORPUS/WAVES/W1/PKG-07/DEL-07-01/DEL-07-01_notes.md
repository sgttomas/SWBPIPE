# DEL-07-01 notes — W1 PKG-07 (worker G1)

TASK (Type 2) worker, run HELP-HUMAN-PIPING-20260921-RECONCILIATION. Frozen
state `00115c719`; code and documents were read only from the evidence
checkout. These are agent dispositions, not owner rulings. DEL-07-01 was not an
R0 pilot.

- Forward ledger `DEL-07-01_forward.csv`: 152 rows. They cover 83 required
  keys, the `.rNN` rows of CLM-003, 004, 005, 006, 007, 012, 019, 022 and 028,
  and two `.sNN` rows (CONTEXT#architecture-basis-injection.s01/.s02). SHA-256
  `60a41ea639af65acfac4b5373ec000efc7ae19e9a647c9a19164077af26c90c9`, sealed
  2026-09-21T20:50:32Z. Not edited after sealing.
- Validator v2: PASS (forward, and forward+reverse).
- Selectability: `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row
  (C9); Piping selects work through owner-steered work graphs.

## Path aliases

- Deliverable files are cited in ContextRefs only, because their path has
  spaces.
- Viewport code:
  - `apps/desktop/src/features/viewport/PipeViewport.tsx`,
    `viewportRouting.ts`, `routeDraft.ts`, `viewportSelection.ts` and
    `viewportVisibility.ts`;
  - `features/component-creation/componentIntent.ts`;
  - `schemas/viewport_editor.schema.yaml`;
  - `core/gui/viewport_editor/`.
- Parity: root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…/DEL-07-01`.
- `tools/validation/check_four_documents.sh` and `validate_enum.py` are root
  tools. `validate_dependencies_schema.py` is cited in its project copy.

## Judgment calls

- **SOW SURFACE → CP-04.** CLM-012.r01 names "OpenPipeStress governance and
  invariant documents". The finding is recorded once, on the SURFACE row, and
  the item row is judged on its substance (ALIGNED).
  - The setup-era staleness of the surface is carried on the block rows.
  - Active code not named by the SOW also carries the former name:
    `viewport_editor.schema.yaml` (`$id` and title) and the viewport_editor
    crate (Cargo description, fixture source names). These are noted as R4
    code-change context.
- **Setup-era origin text** is preserved verbatim by the SOW migration and is
  assessed as current claims → STALE_SETUP_SPECIFICATION/DOC_BEHIND_CODE. It
  covers:
  - CLM-003.r07/.r08, CLM-005.r06, the CLM-006 prose and CLM-010;
  - REQ-10, VER-05, CLM-018, CLM-019.r02/.r06 and CLM-022.r05/.r06.
  - Four-document residue → CP-01 (REPRESENTATION_MIGRATED): VER-01, CLM-014,
    CLM-020, CLM-022.r01 and CLM-023.
  - The PDU-055 blanket supersession does not change these rows (C1).
- **CLM-015 (D-41 PDU-008 boundary) → STALE_REVIEW_OR_EVIDENCE/DOC_BEHIND_CODE.**
  "Dedicated bend authoring and full component-symbol geometry authoring
  remain absent" is overtaken: bend and multi-kind symbol creation landed on
  2026-08-21, and route capture on 2026-09-09. `CanonicalSituation` is left
  empty to keep the CP-03 group homogeneous.
- **CP-10** on CLM-004.r05, CLM-012.r05 and CLM-028.r03 (declared-state rows on
  the library hold).
  - REQ-03 and VER-07 oblige the setup documents to keep the TBD, which they
    do, so they are ALIGNED and point to the CP-10 rows.
  - CLM-012.r04 (dependency versions TBD) is setup residue, because DEC-012
    leaves versions to implementation.
- **REQ-02 → PARTIALLY_IMPLEMENTED/PARTIAL_SLICE.** There are no reaction
  arrows or stress-ratio colour maps in the viewport, and valves, flanges and
  reducers share one box glyph. The claim permits deferral, but none is
  recorded (C6(a)).
- **REQ-07 and CLM-006.r05 → PARTIALLY_IMPLEMENTED/OWNERSHIP_ELSEWHERE.** The
  viewport shows only invalid-geometry exclusions. Classed warnings render in
  DiagnosticsPanel and the DEL-07-04 missing-data panel.
- **Stale reference pointers → CP-02.** CLM-007.r03 cites INIT.md, which was
  removed. CLM-007.r09 cites PRD FR-003/FR-013, which are no longer in the PRD.
- **Remaining.**
  - R01 → CP-06 with NO_OPEN_ACTION (PR #794 at `362dcffc`; RUNTIME_REPORT).
  - R02 → ALIGNED.
  - R03 → CP-07 RULED_CRITERION.
  - R04 → CP-07 NONE.
  - R05 → UNKNOWN.
  - R06 → REMAINING_STATE_MISMATCH/DOC_BEHIND_CODE, because its baseline omits
    the D-68 foundation and the SWBPIPE canvas slices.
  - R07 → ALIGNED.
- **STATUS SURFACE → CP-05.** Last Updated is 2026-09-18, but History has
  entries from 2026-09-19. The C3 dimmed-presentation commits of 2026-09-20
  are unrecorded.

## Canonical departures

None. Seven CS assignments are inherited unchanged. The CS-04 pin row is
supplemented by `.s01`/`.s02`, as CS-04 prescribes.

## Convention friction

1. The CP-04 rule ("record on the SURFACE row") collides with the SURFACE
   row's own whole-file staleness. Only one disposition fits. I gave the
   SURFACE row CP-04 and carried the setup-era staleness on the blocks.
2. The setup-kit verification rows (VER-02/03/04, CLM-022.r02/r03) define
   methods and assert no result. I judged them ALIGNED as methods, since
   no-rerun rules prevent confirming a result.

## Smallest next check for each UNKNOWN row

- `STATUS#remaining/R05`: locate a native re-observation of short-panel
  scrolling and the inspector AX omission on the post-SWBPIPE-B shell. The
  latest records (FINAL_ACCEPTANCE.json, the V103 archive) predate it.

## Reverse pass and its effect on the sealed ledger

Answers: 22 CLAIMED_BY, 7 PARTIAL, 14 UNKEYED, 444 NOT_MINE.

The reverse pass left the sealed dispositions in place. It exposed
undocumented ownership the forward ledger only partly states:

- the geometry tools (annex rows 19/20);
- boundary authoring (row 21);
- authored-coordinate measurement;
- the viewport_editor schema, crate and fixture;
- the UI performance benchmark harness, hit oracles and UI diagnostics, which
  STATUS R03 records but no SOW key covers.

These are answered UNKEYED. The forward SOW SURFACE row mentions only the
schema and crate, as rename context. I would add a note that these landed
capabilities have no SOW key; no disposition changes.

## Batch consistency

`--batch` over the three forward ledgers: PASS, 0 findings. R03 shares its
body with DEL-07-02 R02, R04 with 07-02 R03 and 07-09 R03, and R05 with 07-02
R04 and 07-09 R04. Each group carries identical fields.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No
release, approval, compliance or certification claim is stated or implied.
