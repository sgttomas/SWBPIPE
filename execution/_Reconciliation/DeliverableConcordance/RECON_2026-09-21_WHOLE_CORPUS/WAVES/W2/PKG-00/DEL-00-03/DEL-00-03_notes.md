# Notes — DEL-00-03 Application service command-query-job model (W2, PKG-00)

Forward ledger: 37 rows (28 required keys, 4 `.rNN` rows, 5 `.sNN` sub-claims), sealed. Reverse: 387 capability rows.

## Path aliases

- `ArchitectureBasis.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md` in `NormativeSource` mean the files in this member's folder under `execution/PKG-00_Software Architecture Runway/1_Working/` (the folder names contain spaces, so they are never used in evidence columns).
- `core/…`, `apps/…`, `schemas/…`, `tests/…`, `fixtures/…` are project-root tokens (under `projects/chirality-piping/`). Project documents, registers, decisions and tools are always cited with the explicit `projects/chirality-piping/` prefix. `execution/_Coordination/WORKPLAN_*.md` tokens (DEL-00-02 only) are the repository-root workplans.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is suite-level pass evidence (A6); no per-test result is asserted and nothing was rerun.

## Judgment calls

- **REQ-03-02 envelopes.** Operation validate/apply, runner verbs and the solve job return envelopes with diagnostics. Desktop storage commands for projects, rule packs and libraries return typed receipts, or `Result<_, String>` error text with no diagnostics array. Disposed `PARTIALLY_IMPLEMENTED` · `PROJECT_BASELINE` (AB-00-03, AB-00-06). None of them makes a bare success claim, so a reader who reads the requirement as "no raw success claims" only would call it aligned (MEDIUM confidence).
- **REQ-03-03 transactions.** Atomic units exist (operation apply leaves the input unchanged on rejection, atomic batches roll back, the store uses SQLite transactions, report packages save atomically). No document, schema or API contract defines transaction boundaries for solve runs or adapter calls. `PARTIALLY_IMPLEMENTED`.
- **REQ-03-04 job control.** Solve is a job with start, poll and cancel plus progress, and analysis runs carry reproducibility metadata. Report generation and result export run as synchronous commands with no job cancellation or progress. `PARTIALLY_IMPLEMENTED`.
- **PDU-054 declaration (`purpose.s02`, CP-03) and `open-holds.s02`.** Both say the contract is implemented and put residuals in an empty `## Remaining`. A4: the delegation is not relied on. The subject is partly implemented (the REQ gaps above), so both rows are `PARTIALLY_IMPLEMENTED` (FG-DEL-00-03-01).
- **Realized-artifact pointers (FG-DEL-00-03-02).** `r02` lists `viewport_editor` under `core/gui/*/engine.py`, but that directory is a Rust crate and never had `engine.py`. `r04` calls `analysis_run.schema.json` a progress contract, but it carries reproducibility only (progress lives in runner JobState and the desktop solve job). Both are `STALE_REVIEW_OR_EVIDENCE` · `RECORD_DRIFT`. The Python GUI engines have no desktop product caller.
- **`openpipestress-runner` (CP-04 default fields)** is recorded on the AB SURFACE row. Crate `open_pipe_stress_headless_runner` also carries the former name. The resolved-decisions block (D-33 verbs, exit codes 0/1/2) is `ALIGNED` on substance.

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

- 27 `CONSTRAINS`: service seams, envelopes, job control, atomic units, the status distinction, and the report and export commands that show the REQ-03-04 gap. One `COVERS` (the D-43 validator, RC-00-0285). 359 `NOT_MINE`. No `CLAIMED_BY`: the member says it implements nothing, and the evidence agrees.
- The reverse pass did not change my view of any sealed row. RC-00-0120 (the Python solve-execution timeline with cancelling and cancelled states) is a no-caller engine and does not change REQ-03-04's product reading.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-00 ledgers (DEL-00-01 to DEL-00-04): **FAIL, 1 finding**. `DEL-00-03:AB` (CP-04, tier `LOCAL_DESIGN`) differs from `DEL-00-01:AB` and `DEL-00-02:AB` (CP-04, tier `PROJECT_BASELINE`). **Justified, not an error:** CP-04 itself sets different fields for the `.opsproj` identifier (tier `PROJECT_BASELINE`, persistence-compatibility obligation) and for other rename residue (default fields, `LOCAL_DESIGN`). `openpipestress-runner` is not one of the four identifiers CP-04 lists, so it takes the defaults. The sealed rows carry no `CANONICAL_DEPARTURE` marker, because neither row departs from CP-04.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Claim fence

Dispositions are agent judgments, not owner rulings. Nothing here states or implies a release, approval, compliance or certification claim; no protected standards, vendor or private data is quoted; no external-corpus equation artifact is used as evidence (DEC-043). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
