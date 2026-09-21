# Notes — DEL-00-04 Persistence and schema versioning architecture (W2, PKG-00)

Forward ledger: 43 rows (29 required keys, 8 `.rNN` rows, 6 `.sNN` sub-claims), sealed. Reverse: 387 capability rows.

## Path aliases

- `ArchitectureBasis.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md` in `NormativeSource` mean the files in this member's folder under `execution/PKG-00_Software Architecture Runway/1_Working/` (the folder names contain spaces, so they are never used in evidence columns).
- `core/…`, `apps/…`, `schemas/…`, `tests/…`, `fixtures/…` are project-root tokens (under `projects/chirality-piping/`). Project documents, registers, decisions and tools are always cited with the explicit `projects/chirality-piping/` prefix. `execution/_Coordination/WORKPLAN_*.md` tokens (DEL-00-02 only) are the repository-root workplans.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is suite-level pass evidence (A6); no per-test result is asserted and nothing was rerun.

## Judgment calls

- **Hash rows (FG-DEL-00-04-01; `resolved r03`, `realized r04`).** Both say canonical hashing is "implemented in `hashService.ts`" over "JCS-like sorted-key" JSON. Since 2026-06-11 (TP-H1-HASHUNIFY-001, TP-H5-JCSRENDER-001), `hashService.ts` routes through wasm to the Rust `canonical_json` crate, which renders RFC 8785 JCS. The TypeScript canonicalization is deleted. The rows are `STALE_REVIEW_OR_EVIDENCE` · `DOC_BEHIND_CODE`. The envelope-hash exclusion they describe still holds.
- **REQ-04-02.** Project records carry the store migration ledger and model-document migration status. Stored rule-pack and library records keep a versioned document but no migration status: `PARTIALLY_IMPLEMENTED`.
- **REQ-04-04 (F7).** The Python persistence service checks all six round-trip kinds but has no desktop product caller. The desktop store round-trips the model, results and analysis run and verifies the envelope hash on open, but its envelope holds no rule-pack references or diagnostics: `PARTIALLY_IMPLEMENTED`.
- **PDU-055 declaration (`purpose.s02`, CP-03).** `PARTIALLY_IMPLEMENTED` (FG-DEL-00-04-02). The delegation to an empty Remaining is not relied on (A4).
- **SCA-003 injection.** Split into `.s01` (storage terms, which match the code: `ALIGNED`) and `.s02`. `.s02` is the "Still TBD" list, whose migration framework and export packaging were ruled by DEC-019, DEC-028 and DEC-057; it is `STALE_SETUP_SPECIFICATION` (first present at `7bee9ae41`, F3).
- **DEC-051** relaxed runtime egress (the open-residency ruling). It did not change the storage terms in `resolved r01` or `.s01`, so those stay `ALIGNED`.
- **Possible defect outside this member (for the verifier and R3).** The Python reference service `core/project_persistence/service.py` and its tests use `SORTED_COMPACT_JSON` canonicalization; one test asserts "JCS" is absent. The persistence contract (DEL-02-05) and AB-00-04 call for JCS-compatible canonicalization. The product path uses RFC 8785 JCS. This belongs to DEL-02-05's ledger and does not change a DEL-00-04 disposition: REQ-04-03 is a definition claim, and the product implements JCS.

## Canonical departures

None. All seven CS rows inherit their assignment unchanged.

## Convention friction

- **Mixed-disposition blocks.** Purpose and open-holds blocks mix accurate statements with stale pins or open items, so they are split into `.sNN` sub-claims and the parent row is `CONTAINER` / `COVERED_BY_CHILDREN` (C1). Where a block has substance of its own besides a split-off sentence (normative-requirements blocks), the block is assessed directly and the sentence is a `.sNN`.
- **One SURFACE row, two surface-level defects.** When CP-04 rename residue takes the AB SURFACE row, the header revision pin (L6) is recorded on `<DEL>:AB.s01` (CP-02).
- **The AB bytes are hash-bound.** `CONSOLIDATION_MANIFEST.md` hashes each `ArchitectureBasis.md` and `tools/validation/validate_architecture_basis.py` checks it, so every AB catch-up also rehashes the manifest entry. The same validator requires `_CONTEXT.md` to keep the revision 0.7 setup pin (CS-01 rows), so a CS-01 catch-up must change the validator too.
- **Tier for REQ gaps.** REQs that restate an AB-00-0x row of SOFTWARE_DECOMP §8.1 (SCA-001 accepted baseline) take `PROJECT_BASELINE` when a gap remains; kit-only requirements take `LOCAL_DESIGN` (notebook item 5).

## UNKNOWN rows

- `DEL-00-04:AB#open-holds-and-routed-questions.s01` ("provider expansion"). Smallest check: find what "provider expansion" names in the D-41 R5 T1–T6 residual records. The same phrase appears with no referent in the pre-consolidation PDU-054 declaration. Otherwise ask the loop that authored the declaration, then dispose the row.

## Reverse pass

- 18 `CONSTRAINS`: store and document migrations, canonicalization and hash seams, the persistence envelope schema and Python reference service, the product save/open hash checks, storage capability, legacy-store carry-forward, private library and rule-pack storage, the import storability guard, and audit-manifest hashing. Two `COVERS`: the persistence contract that absorbed this member's anticipated document (RC-00-0132, owned by DEL-02-05) and the D-43 validator (RC-00-0285). 367 `NOT_MINE`. No `CLAIMED_BY`.
- The reverse pass did not change my view of any sealed row.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-00 ledgers (DEL-00-01 to DEL-00-04): **FAIL, 1 finding**. `DEL-00-03:AB` (CP-04, tier `LOCAL_DESIGN`) differs from `DEL-00-01:AB` and `DEL-00-02:AB` (CP-04, tier `PROJECT_BASELINE`). **Justified, not an error:** CP-04 itself sets different fields for the `.opsproj` identifier (tier `PROJECT_BASELINE`, persistence-compatibility obligation) and for other rename residue (default fields, `LOCAL_DESIGN`). `openpipestress-runner` is not one of the four identifiers CP-04 lists, so it takes the defaults. The sealed rows carry no `CANONICAL_DEPARTURE` marker, because neither row departs from CP-04.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Claim fence

Dispositions are agent judgments, not owner rulings. Nothing here states or implies a release, approval, compliance or certification claim; no protected standards, vendor or private data is quoted; no external-corpus equation artifact is used as evidence (DEC-043). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
