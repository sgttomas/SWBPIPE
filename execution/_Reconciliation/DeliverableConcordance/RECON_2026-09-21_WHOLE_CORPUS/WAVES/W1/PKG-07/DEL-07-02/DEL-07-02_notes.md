# DEL-07-02 notes — W1 PKG-07 (worker G1)

TASK (Type 2) worker, run HELP-HUMAN-PIPING-20260921-RECONCILIATION, brief
`R2-WORKER_brief.md` (SHA-256 `06224add…c0ea0ef9`, verified). Frozen state
`00115c71931bcae79909602d653740d3bb72dfa1`; code and documents were read only
from the evidence checkout. These are agent dispositions, not owner rulings.

- Forward ledger `DEL-07-02_forward.csv`: 111 rows. They cover 83 required
  keys; the `.rNN` rows of CLM-004, CLM-010, CLM-018 and CLM-025; and three
  `.sNN` sub-claims (CLM-009.s01, CONTEXT#architecture-basis-injection.s01 and
  .s02). SHA-256
  `8a0bb3ffee6e2f3876053e3582571f2223932d257a27ac3fe80e1a2c359d4ff1`, sealed
  2026-09-21T20:40:41Z. Not edited after sealing.
- Validator v2: PASS (forward, and forward+reverse against
  `ROUTING/PKG-07_capabilities.csv`).
- Selectability: `SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row
  (C9). Since 2026-09-19 Piping selects work through owner-steered work graphs,
  not `## Remaining`.

## Path aliases

- Deliverable-folder files (ScopeOfWork.md, _STATUS.md, _CONTEXT.md, MEMORY.md,
  `_run_records/`) sit under a path containing spaces. They are cited in
  `ContextRefs` only, because evidence columns may not hold spaces.
- Code is cited from the project root (`apps/…`, `core/…`, `tests/…`,
  `schemas/…`). Main surfaces:
  - `apps/desktop/src/features/model-tree/ModelTree.tsx` and
    `PropertyInspector.tsx`;
  - `features/workspace/table/*`, the shared Node/Material/Section tables;
  - `features/workspace/selectionState.ts`, `modelIndex.ts` and
    `services/operationService.ts`;
  - `core/gui/model_tree/engine.py`.
- Parity records are root-level: `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`.
- `tools/validation/validate_claims_language.py` is the root tool (no project
  copy).
- Suite status is cited as `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`
  and was not rerun. Per-test pass status is never asserted.

## Judgment calls

- **R0 named repairs carried.**
  - Matrix OUT-001 → CP-09 STALE_REVIEW_OR_EVIDENCE/EVIDENCE_OVERTAKEN. It
    cites CHANGE-P2 parity (PASS, production hash `112fee6a…`; frozen SOW
    `298be604…`; the SOW changed afterwards under DEC-081 D-48 Wave 2).
  - VER-001 cites the same parity.
  - R03's tier and baseline class are aligned with DEL-07-09 R03
    (LOCAL_DESIGN/NONE: the merge, not a ruling, overtook it).
  - R11 is harmonized with DEL-07-09 R09 (CP-08).
  - No reverse answer cites a Remaining key.
- **Splits.** The CLM-004, CLM-010, CLM-018 and CLM-025 tables were split
  because their rows take different dispositions. CLM-003, CLM-005, CLM-008,
  CLM-017, CLM-020, CLM-033 and CLM-036 were assessed as whole blocks: every
  row takes one disposition.
- **State-library hold (CP-10)** on CLM-004.r03 and CLM-010.r02.
  - DEL-00-05's ArchitectureBasis still records the GUI component/state-library
    choice as OPEN and routed to a human ruling.
  - The code holds transient state in bespoke React context and session
    modules, and `package.json` adds no library.
  - Rows that only state principles ("keep unresolved choices TBD", CLM-031)
    are ALIGNED.
- **CLM-034 boundary.** Disposition: IMPLEMENTED_UNDOCUMENTED ·
  SCOPE_GREW_BY_DIRECTION · PROJECT_BASELINE · OWNER.
  - The creation forms and the direct/review table editing (PRs #832-#834,
    merged 2026-09-20/21) landed under DEL-07-02 briefs and merged PRs (A2).
  - The SCA-009 annex lands the material editor surface (row 3) and the section
    editor surface (row 17) in DEL-07-03.
  - The owner must place the line. MEDIUM confidence.
- **RQ-002, CLM-018.r07 and CLM-025.r05** → PARTIALLY_IMPLEMENTED ·
  OWNERSHIP_ELSEWHERE. Rule-pack and private-library reference fields, and
  checksum display, live in `features/rule-packs` and
  `features/secret-private-library`.
- **RQ-005 is ALIGNED (MEDIUM).** The inspector ProvenanceBlock does not cover
  material or section selections; their provenance shows as the Provenance
  column of the tables.
- **RQ-006 reads.** Tree and inspector reads use the session model returned by
  the project and operation services, not per-read queries. I treated this as
  within the result-envelope boundary.
- **AC-001 → PARTIALLY_IMPLEMENTED/DOC_BEHIND_CODE.** The contract no longer
  describes the implemented editing boundary.
- **Remaining.**
  - R01 is the ROOT FINAL_ACCEPTANCE foundation item, with open residuals →
    ALIGNED.
  - R08's landed baseline omits the D-68 foundation and the table editing →
    REMAINING_STATE_MISMATCH/DOC_BEHIND_CODE.
  - R09 and R10 are landed narrative → CP-06 with NO_OPEN_ACTION.
- **Rename residue.** None of DEL-07-02's surfaces carries the former name.
  `core/gui/model_tree/engine.py` (PROVENANCE source_name) does. The SOW does
  not name that file, so it is recorded on CLM-019 as context for the R4
  rename class, not as a CP-04 row.
- **Post-freeze-of-docs code.** The table editing of 2026-09-20/21 has no
  STATUS or MEMORY entry. This is noted on the STATUS SURFACE (CP-05) and
  MEMORY rows.
- **ADOPTED_BY_REFERENCE** is flagged on the SOW SURFACE row, because D-68
  adopts its plan by reference.

## Canonical departures

None. All seven CS assignments are inherited unchanged. The CS-04 row covers
the pin only; its PKG-00 readiness and Still-TBD parts are the `.s01` and
`.s02` rows that CS-04 prescribes.

## Convention friction

1. Evidence columns cannot hold the space-bearing deliverable paths, so
   deliverable-local evidence (run records, _REVIEW.md) can only be cited in
   ContextRefs.
2. CP-03 blocks that are not pins (PDU-054 declarations) have no fields of
   their own. I left `CanonicalSituation` empty on them, so the batch CP-03
   group stays homogeneous.
3. CP-10's example ("state library") meets rows that state the hold as a
   requirement on documents (DEL-07-01 REQ-03, VER-07). I dispose
   declared-state rows as CP-10 and judge document-obligation rows on the
   documents; the notebook records this rule.

## Smallest next check for each UNKNOWN row

- `SOW#CLM-015/DEL-07-02-RQ-008`, `SOW#CLM-018.r08` and `SOW#CLM-025.r06`:
  locate, or have an authorized reviewer run, a protected-content and
  fixture-provenance review over the desktop preview fixture, the e2e fixtures
  and the screenshots cited in run records. A3 discovery found none.
- `STATUS#remaining/R04`: locate a native re-observation of short-panel
  scrolling and the inspector AX omission on the post-SWBPIPE-B shell. The
  latest records are FINAL_ACCEPTANCE.json and the V103 archive, both earlier.

## Reverse pass and its effect on the sealed ledger

Answers: 2 CLAIMED_BY, 13 PARTIAL, 1 COVERS, 9 UNKEYED, 462 NOT_MINE.

The reverse pass left the sealed dispositions in place. It widened one
finding and exposed one:

- **Wider than CLM-034 states.** Annex row 24 puts the display-unit switching
  surface in DEL-07-02. Annex row 5 puts force/moment/displacement load GUI in
  "DEL-07-02 (inspector forms)", but it is implemented in the load-case manager
  panel. Rows 14, 18 and 21 add expansion-joint creation, removal affordances
  and boundary GUI. No DEL-07-02 key covers any of these, so they are answered
  UNKEYED. The sealed forward ledger records only the material/section side of
  the boundary (CLM-034) and does not name the display-unit surface. I would
  add that to the CLM-034 notes; no disposition changes.
- **Hosted but not owned.** The inspector hosts MaterialTemperatureForm and
  SectionAssignment, which the annex lands in DEL-07-03. They are answered
  NOT_MINE with that reason.
- **No inventory gap.** The Python contract module (`core/gui/model_tree`) now
  has an inventory row (UNKEYED, nearest CLM-019).

## Batch consistency

`validate_ledger_v2.py --batch` over the DEL-07-02, DEL-07-09 and DEL-07-01
forward ledgers: PASS, 0 findings.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No
release, approval, compliance or certification claim is stated or implied.
