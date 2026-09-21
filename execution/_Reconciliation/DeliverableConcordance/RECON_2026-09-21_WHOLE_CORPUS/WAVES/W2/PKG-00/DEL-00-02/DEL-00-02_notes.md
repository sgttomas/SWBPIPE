# Notes — DEL-00-02 Repository and module boundary architecture (W2, PKG-00)

Forward ledger: 43 rows (29 required keys, 6 `.rNN` rows, 8 `.sNN` sub-claims), sealed. Reverse: 387 capability rows.

## Path aliases

- `ArchitectureBasis.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md` in `NormativeSource` mean the files in this member's folder under `execution/PKG-00_Software Architecture Runway/1_Working/` (the folder names contain spaces, so they are never used in evidence columns).
- `core/…`, `apps/…`, `schemas/…`, `tests/…`, `fixtures/…` are project-root tokens (under `projects/chirality-piping/`). Project documents, registers, decisions and tools are always cited with the explicit `projects/chirality-piping/` prefix. `execution/_Coordination/WORKPLAN_*.md` tokens (DEL-00-02 only) are the repository-root workplans.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is suite-level pass evidence (A6); no per-test result is asserted and nothing was rerun.

## Judgment calls

- **Layer map and module ownership (FG-DEL-00-02-01).** The member's purpose says it holds a layer map and module ownership table. Neither exists in the AB, and the pre-consolidation kit (git `33fd81a85^`) listed them only as expected Datasheet content. SOFTWARE_DECOMP AB-00-02 names the eleven layers without per-layer responsibilities; SPEC §1 gives a five-layer overview. So `REQ-02-01`, `purpose.s01`, `normative-requirements.s01` (the "satisfied by reference" sentence) and `CONTEXT#description` are `PARTIALLY_IMPLEMENTED` · `PROJECT_BASELINE`. MEDIUM confidence: a reader could accept SPEC §1 plus the implemented tree as enough.
- **REQ-02-05 / PDU-007 (FG-DEL-00-02-03).** `PARTIALLY_IMPLEMENTED` · `LOCAL_DESIGN` (kit-only requirement) · `REVIEW`. The only executable check is the D-43 kit-hygiene validator. `STATUS#remaining/R01` is `ALIGNED` with `OPEN_ACTION` pointing to REQ-02-05 (F2). `open-holds.s01` restates the same hold and takes REQ-02-05's disposition (C6(b)).
- **Package manager (`open-holds.s03`, CP-10).** The project `package.json` declares an npm workspace with a committed lockfile, which settles the "monorepo/package-manager selection" the member records as open with no ruling. `IMPLEMENTED_DIFFERENTLY` · `AUTHORITY_UNCLEAR` · `PROJECT_BASELINE` · `OWNER`. No code-lint configuration was found, so the lint-tooling part stays open as declared.
- **Record hosting (`open-holds.s02`).** `UNKNOWN` · `AUTHORITY_UNCLEAR` · `REVIEW`: governing sources are silent on where implementation run records belong.
- **Module-boundary decision recording (`resolved r06`, FG-DEL-00-02-04).** Same finding as DEL-00-01 FG-DEL-00-01-01 (only ADR-0001 exists; later module-boundary rulings such as DEC-094 are decision-log rows only).
- **Layout deferral (`resolved r02`, realized-artifacts).** `ALIGNED`: the Deliverables register (governing companion) records `module_boundaries.md` as a permitted deferral under the SOW-057 note. The AB also cites a July-run W1 notes file for the deferral; that file is an excluded input and was not read.
- **`.opsproj` (CP-04)** is recorded on the AB SURFACE row; `resolved r05` is judged on substance.

## Canonical departures

None. All seven CS rows inherit their assignment unchanged.

## Convention friction

- **Mixed-disposition blocks.** Purpose and open-holds blocks mix accurate statements with stale pins or open items, so they are split into `.sNN` sub-claims and the parent row is `CONTAINER` / `COVERED_BY_CHILDREN` (C1). Where a block has substance of its own besides a split-off sentence (normative-requirements blocks), the block is assessed directly and the sentence is a `.sNN`.
- **One SURFACE row, two surface-level defects.** When CP-04 rename residue takes the AB SURFACE row, the header revision pin (L6) is recorded on `<DEL>:AB.s01` (CP-02).
- **The AB bytes are hash-bound.** `CONSOLIDATION_MANIFEST.md` hashes each `ArchitectureBasis.md` and `tools/validation/validate_architecture_basis.py` checks it, so every AB catch-up also rehashes the manifest entry. The same validator requires `_CONTEXT.md` to keep the revision 0.7 setup pin (CS-01 rows), so a CS-01 catch-up must change the validator too.
- **Tier for REQ gaps.** REQs that restate an AB-00-0x row of SOFTWARE_DECOMP §8.1 (SCA-001 accepted baseline) take `PROJECT_BASELINE` when a gap remains; kit-only requirements take `LOCAL_DESIGN` (notebook item 5).
- **Directory tokens and F5.** `resolved r02` cites the module tree (`core/`, `apps/desktop`, `api`, `schemas/`, `tests/`) as layout evidence. That makes 274 capabilities overlap a cited path. Their `NOT_MINE` reasons are generated per capability: each names the capability, the module it lives in, the cited path and why it is cited. They share a structure but not content; the verifier may judge whether that meets F5.

## UNKNOWN rows

- `DEL-00-02:AB#open-holds-and-routed-questions.s02`. Smallest check: search for a REVIEW disposition on TP-SEAM-WASM-001 record hosting (the 2026-07-15 and 2026-07-25 workplans still list it as routed). If none exists, route the question to REVIEW.

## Reverse pass

- Six `CONSTRAINS`: the API boundary contract, plugin boundary, plugin manifest, extension contracts and adapter framework under REQ-02-03 (RC-00-0019, 0381, 0072, 0184, 0061), and the npm workspace under `open-holds.s03` (RC-00-0357). Four `COVERS`: the wasm module-boundary seam whose run records this member hosts (RC-00-0002, 0188, 0220) and the D-43 validator under REQ-02-05 (RC-00-0285). 377 `NOT_MINE`.
- **Changed view (sealed; not edited).** The Notes of `resolved-decisions…r04` say dense "stays default" under DEC-050. RC-00-0010 records a DEC-053 observation that promotes sparse solving to the interactive default, so that Notes clause may be out of date. The row's disposition (`ALIGNED`, on the DEC-023 strategy) is unaffected. The verifier should correct the Notes wording if confirmed.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-00 ledgers (DEL-00-01 to DEL-00-04): **FAIL, 1 finding**. `DEL-00-03:AB` (CP-04, tier `LOCAL_DESIGN`) differs from `DEL-00-01:AB` and `DEL-00-02:AB` (CP-04, tier `PROJECT_BASELINE`). **Justified, not an error:** CP-04 itself sets different fields for the `.opsproj` identifier (tier `PROJECT_BASELINE`, persistence-compatibility obligation) and for other rename residue (default fields, `LOCAL_DESIGN`). `openpipestress-runner` is not one of the four identifiers CP-04 lists, so it takes the defaults. The sealed rows carry no `CANONICAL_DEPARTURE` marker, because neither row departs from CP-04.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Claim fence

Dispositions are agent judgments, not owner rulings. Nothing here states or implies a release, approval, compliance or certification claim; no protected standards, vendor or private data is quoted; no external-corpus equation artifact is used as evidence (DEC-043). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
