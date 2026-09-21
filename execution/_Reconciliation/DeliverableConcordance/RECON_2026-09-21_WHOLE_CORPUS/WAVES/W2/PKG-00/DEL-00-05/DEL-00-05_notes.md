# DEL-00-05 notes — GUI state and interaction architecture

Wave W2, package PKG-00, worker G2. Forward ledger 38 rows (29 required keys,
3 `.rNN` rows for the split resolved-decisions block, 6 `.sNN` sub-claims).
Reverse file 387 answers. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.

## Path aliases

- `AB` = `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-05_GUI state and interaction architecture/ArchitectureBasis.md` (project-relative). Deliverable files contain spaces, so they are cited only in `NormativeSource` and `ContextRefs`, never in evidence columns.
- `apps/desktop/src/features/workspace/` holds the six `*SessionState.ts` hooks, `selectionState.ts`, `sessionModel.ts` and `workspaceSession.ts`.
- `SOFTWARE_DECOMP.md` line anchors: L6 revision, L246 PKG-00 row for this member, L442 AB-00-05, L447 §8.2, L587 §12, L600 DEC-009, L665 DEC-074, L704 §13.

## R0 calibration repairs carried

- `AuthorityNeeded` harmonized: every `_CONTEXT.md` block and every pointer row is `NO` (CP-02/CS rows); `OWNER` only where a choice is needed (the CP-10 holds and the accessibility hold).
- The three open-hold rows go to the owner as sealed: `.s01` state library and `.s02` undo storage (CP-10, OWNER), `.s03` accessibility (OWNER). `.s03` is now STALE_REVIEW_OR_EVIDENCE · SCOPE_REDIRECTED_BY_RULING rather than IMPLEMENTED_DIFFERENTLY, because C6(d) reserves IMPLEMENTED_DIFFERENTLY for code departures and CP-10; the hold text was overtaken by D-68.
- The §8.4 pointer (R0 review F6) is judged on the resolved-decisions block and on `.r01`, whose record cell repeats it. Under the v2 keys the block has `.rNN` keys, so the R0 `.s01`-`.s04` sub-claims were replaced by `.r01`-`.r03` plus the block row.
- Surface pin rows re-mapped from R0's RECORD_DRIFT/PROJECT_BASELINE/OWNER to CP-02 (BASIS_POINTER_STALE, LOCAL_DESIGN, NO), and `DivergenceLayers` to the amended vocabulary.
- `STATUS#remaining` is pre-typed NON_NORMATIVE in extractor v2 (R0 had it DECLARED_STATE · ALIGNED); kept as pre-typed.

## Judgment calls

- REQ-05-03 "preserve diagnostics": read as keeping diagnostic truth explicit across an undo/redo (results cleared, solve-job event recorded, readiness recomputed). If the owner means restoring prior result diagnostics, the row would become PARTIALLY_IMPLEMENTED. Confidence MEDIUM.
- REQ-05-05: the product panel keeps all six classes with a local contract class beside each; the DEL-07-04 Python contract's own vocabulary is that deliverable's and DEL-00-06's matter (GAP_WORDING_CHECKED on the row).
- `.r01` (DEC-009 stack): substance accurate, but under F1 it takes the §8.4 pointer disposition because its own record cell cites the missing section.
- realized-artifacts: not split; all three `.rNN` rows would share ALIGNED.
- Header pin: the AB SURFACE row carries it (no rename residue in this file).
- The AB file is hash-bound in the package CONSOLIDATION_MANIFEST and the validator compares that hash, so every AB text repair also re-records the manifest hash. Recorded in RemainingWork on the pointer rows; still LOCAL_DESIGN · NO under CP-02.
- CS-01: the validator deliberately requires the setup 0.7 pin to stay under the banner; the inherited stale disposition is kept and the note says the repair is a label, not a value change.

## Canonical departures

None. All seven CS assignments inherited unchanged; CP-02, CP-10 applied as written.

## Convention friction

- F1 versus C1's common-defect rule: record cells that repeat a broken pointer make otherwise-accurate items non-aligned (`.r01`). Applied F1 as ruled.
- CP-02 says `AuthorityNeeded NO`, but the manifest hash binding means an AB edit also touches a package-level D-43 artifact. Whether that needs a decision is for the verifier or owner; flagged here, not departed from.

## UNKNOWN rows

None.

## Reverse pass

21 CONSTRAINS, 2 COVERS, 364 NOT_MINE. PKG-00 owns no implementation, so realized state/selection/undo/command-route capabilities are CONSTRAINS with the REQ key. The R0 reverse file (read after sealing) had UI preferences (CAP-WS-028) as NOT_MINE with a gap note; here RC-00-0176 is CONSTRAINS REQ-05-01 (chrome state kept out of the project file, consistent with DEC-109). The reverse pass did not change my view of any sealed row.

## Batch consistency

`validate_ledger_v2.py --batch` over the four W2 PKG-00 G2 ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): Piping selects work through owner-steered work graphs since 2026-09-19.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
