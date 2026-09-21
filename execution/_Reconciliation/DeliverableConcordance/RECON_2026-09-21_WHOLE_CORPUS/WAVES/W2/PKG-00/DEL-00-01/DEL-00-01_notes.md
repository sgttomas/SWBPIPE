# Notes — DEL-00-01 Architecture decision record baseline (W2, PKG-00)

Forward ledger: 35 rows (28 required keys, 7 `.sNN` sub-claims), sealed. Reverse: 387 capability rows.

## Path aliases

- `ArchitectureBasis.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md` in `NormativeSource` mean the files in this member's folder under `execution/PKG-00_Software Architecture Runway/1_Working/` (the folder names contain spaces, so they are never used in evidence columns).
- `core/…`, `apps/…`, `schemas/…`, `tests/…`, `fixtures/…` are project-root tokens (under `projects/chirality-piping/`). Project documents, registers, decisions and tools are always cited with the explicit `projects/chirality-piping/` prefix. `execution/_Coordination/WORKPLAN_*.md` tokens (DEL-00-02 only) are the repository-root workplans.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is suite-level pass evidence (A6); no per-test result is asserted and nothing was rerun.

## Judgment calls

- **ADR process versus decision log (FG-DEL-00-01-01; `AB#purpose.s03`, `REQ-01-04`).** The ADR surface holds only ADR-0001 (2026-06-11). Architecture rulings after that date (DEC-022, 023, 025, 028, 057, 065, 089) exist only as decision-log rows, which have no affected-packages or reconsideration-trigger fields. Disposed `PARTIALLY_IMPLEMENTED` · `PROJECT_BASELINE` (AB-00-01 is accepted baseline) · `OWNER`, because whether decision-log rows satisfy AB-00-01 is a choice, not a catch-up. MEDIUM confidence.
- **Open TBD list (`open-holds.s01`).** It lists rule grammar (ruled by D-02, DEC-022), CI provider (D-05, DEC-025; later DEC-059, DEC-093) and coverage thresholds (DEC-024, DEC-060) as open. The text was first written 2026-07-15, after those rulings, so the cause is `RECORD_DRIFT`, not `SCOPE_REDIRECTED_BY_RULING`. The DEC-012 status column in the decision log is also not updated for these; that is outside this member.
- **D-06b (`open-holds.s02`).** D-06b was ruled on 2026-07-25 (DEC-089), after the text: `SCOPE_REDIRECTED_BY_RULING`. The unsigned v0.1 posture still holds, so only the record clause is stale (tier `LOCAL_DESIGN`).
- **`.opsproj` (CP-04)** is recorded once, on the AB SURFACE row, with the `.opsproj` variant fields and the DEC-101 persistence-compatibility note. The resolved-decisions block is assessed directly as `ALIGNED` on its substance.
- **REQ-01-05 (IP boundary)** is `ALIGNED` on an agent reading of the three ADR-surface files; the requirement's "Human review" evidence column names an acceptance method, and no human review record for the ADR surface was sought beyond the 2026-05-11 PKG-00 lock review (which predates the surface).
- **ADR index residue (observation, not a keyed claim).** `docs/architecture/adr/index.md` still says "Per DEL-00-01 Specification REQ-01-01" (four-document residue) and says the pre-surface decision log runs to DEC-019. Both belong to the index file, not to this member's issued keys.

## Canonical departures

None. All seven CS rows inherit their assignment unchanged.

## Convention friction

- **Mixed-disposition blocks.** Purpose and open-holds blocks mix accurate statements with stale pins or open items, so they are split into `.sNN` sub-claims and the parent row is `CONTAINER` / `COVERED_BY_CHILDREN` (C1). Where a block has substance of its own besides a split-off sentence (normative-requirements blocks), the block is assessed directly and the sentence is a `.sNN`.
- **One SURFACE row, two surface-level defects.** When CP-04 rename residue takes the AB SURFACE row, the header revision pin (L6) is recorded on `<DEL>:AB.s01` (CP-02).
- **The AB bytes are hash-bound.** `CONSOLIDATION_MANIFEST.md` hashes each `ArchitectureBasis.md` and `tools/validation/validate_architecture_basis.py` checks it, so every AB catch-up also rehashes the manifest entry. The same validator requires `_CONTEXT.md` to keep the revision 0.7 setup pin (CS-01 rows), so a CS-01 catch-up must change the validator too.
- **Tier for REQ gaps.** REQs that restate an AB-00-0x row of SOFTWARE_DECOMP §8.1 (SCA-001 accepted baseline) take `PROJECT_BASELINE` when a gap remains; kit-only requirements take `LOCAL_DESIGN` (notebook item 5).

## UNKNOWN rows

None.

## Reverse pass

- One `CLAIMED_BY` (RC-00-0099, the ADR surface), five `COVERS` (ADR-0001's wasm engine seam: RC-00-0002, 0155, 0188, 0220; the D-43 validator RC-00-0285), three `CONSTRAINS` (DEC-009/DEC-057 realizations: RC-00-0011, 0138, 0357), 378 `NOT_MINE`. Overlapping NOT_MINE reasons name the cited path and why it is cited (F5).
- The reverse pass did not change my view of any sealed row. RC-00-0136 (hosted desktop browser-test workflow, D-65/DEC-093) confirms the CI-provider part of `open-holds.s01`. RC-00-0010 (DEC-053 sparse default-promotion observation) is consistent with the resolved-decisions row, which cites DEC-050 only for adoption timing.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-00 ledgers (DEL-00-01 to DEL-00-04): **FAIL, 1 finding**. `DEL-00-03:AB` (CP-04, tier `LOCAL_DESIGN`) differs from `DEL-00-01:AB` and `DEL-00-02:AB` (CP-04, tier `PROJECT_BASELINE`). **Justified, not an error:** CP-04 itself sets different fields for the `.opsproj` identifier (tier `PROJECT_BASELINE`, persistence-compatibility obligation) and for other rename residue (default fields, `LOCAL_DESIGN`). `openpipestress-runner` is not one of the four identifiers CP-04 lists, so it takes the defaults. The sealed rows carry no `CANONICAL_DEPARTURE` marker, because neither row departs from CP-04.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Claim fence

Dispositions are agent judgments, not owner rulings. Nothing here states or implies a release, approval, compliance or certification claim; no protected standards, vendor or private data is quoted; no external-corpus equation artifact is used as evidence (DEC-043). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
