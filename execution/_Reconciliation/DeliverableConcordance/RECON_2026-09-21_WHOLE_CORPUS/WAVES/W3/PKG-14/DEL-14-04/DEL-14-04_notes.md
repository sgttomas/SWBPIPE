# DEL-14-04 notes — Analysis-run comparison engine (W3, PKG-14, worker G2)

Forward ledger sealed in `DEL-14-04_SEAL.txt` (110 body rows; 81 required keys;
.rNN splits for CLM-004, CLM-006, CLM-008, CLM-015; sub-claims CLM-005.s01,
CLM-017.s01/.s02, CONTEXT#architecture-basis-injection.s01-.s03). Not an R0
pilot. Frozen state `00115c719`.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine/` (spaces resolve at the freeze).
- Engine `core/comparison/analysis_run/engine.py`; tests `tests/test_analysis_run_comparison.py` (11 tests) and `tests/test_comparison_contracts.py`; input contracts `schemas/comparison_mapping.schema.json`, `schemas/comparison_tolerance.schema.json`, `schemas/analysis_run.schema.json`.
- Parity record (CP-09): root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P4/checks/DEL-14-04/parity.md`; SOW migration commit `0e69c0786`.
- Gate: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (1,138 Python tests passed; the sweep log lists this test file). Per-test status is not asserted beyond that record; nothing was rerun.

## Judgment calls

1. **No product caller (F7).** The engine is reached only from tests. The desktop Comparison panel is a separate TypeScript single-run preview (`apps/desktop/src/services/previewService.ts`), and the desktop report projection throws on non-empty comparison input. Every ALIGNED engine row carries `PRODUCT_CALLER: NONE`. The deliverable is a backend feature slice, so claims about the engine are satisfied by the engine.
2. **FG-DEL-14-04-01, PDU-011 output schema.** `AnalysisRunComparison.to_dict()` has no accepted schema. The governing row is R-14-04-007. Declaration rows that describe the hold (CLM-008.r01, CLM-017.s01, OQ-14-04-002) and procedure step 6 (CLM-023) take the gap disposition under C6(b) and F1: PARTIALLY_IMPLEMENTED, PARTIAL_SLICE, PROJECT_BASELINE, BASELINE, OWNER. The guard rows CLM-026 and CLM-037 are complied with, so they are ALIGNED with GAP_WORDING_CHECKED.
3. **FG-DEL-14-04-02, PDU-047 validation.** CLM-008.r02 and CLM-017.s02 are VERIFIED_NOT_VALIDATED, INVARIANT, VALIDATION, ENGINEERING. No validation evidence exists, and the section-property oracle is not a basis (A5). STATUS R02 is ALIGNED with `OPEN_ACTION: CLM-017.s02`, and STATUS R01 is ALIGNED with `OPEN_ACTION: R-14-04-007` (F2).
4. **FG-DEL-14-04-03, OI-014.** OQ-14-04-001 is DOCUMENTED_UNIMPLEMENTED with cause DEFERRED_BY_RULING. OI-014 is an open issue recorded in decomposition revision 0.12, and a human product decision is needed. R-14-04-005 is ALIGNED because the engine does use tolerance profiles; defaults are not an element of that requirement.
5. **FG-DEL-14-04-04, Rust-core basis versus the Python engine (possible authority item).** DEC-009 adopts Rust core/application services, but the engine is Python under `core/`. No ruling was located that permits Python domain engines; DEC-025 only registers pytest as a gate surface. Rows CLM-004.r07 and CONTEXT ABI.s02 are IMPLEMENTED_DIFFERENTLY, AUTHORITY_UNCLEAR, PROJECT_BASELINE, OWNER, MEDIUM. The pattern probably spans the corpus, so R3 may cluster it.
6. **FG-DEL-14-04-05, entity categories.** CLM-004.r05 and R-14-04-004 name nodes, elements, supports and terminals. The engine compares any `object_ref` generically, but the tests use only `PipeElement` refs, so those rows are PARTIALLY_IMPLEMENTED at MEDIUM. The R6 N3 closure covered the seven result families, not entity kinds. CLM-024, whose scope-coverage check names no entity kinds, is ALIGNED.
7. **CLM-006.r03, unit normalization.** Normalization uses only caller-supplied factors. It does not consume the DEC-018 catalog or `units.schema.yaml`, and the "catalog TBD" status is overtaken. The row is PARTIALLY_IMPLEMENTED, LOCAL_DESIGN (F8), because the no-silent-units boundary holds. MEDIUM.
8. **R-14-04-008, protected content.** ALIGNED at MEDIUM. The frozen engine and test bytes carry only invented, labelled fixture values. The last recorded scoped protected-content scan (May 2026) predates later edits, which have software-review records.
9. **CLM-016, documentation.** PARTIALLY_IMPLEMENTED. Only the MEMORY notes and the module docstring document the engine; nothing covers the delta output structure, the ordering basis or fixture provenance.
10. **Stale TBD wording.** Status and condition rows whose substance is an overtaken setup TBD are STALE_SETUP_SPECIFICATION, DOC_BEHIND_CODE. F3 origin is `7bee9ae41`, checked by `git log -S` for each phrase. Requirement rows whose substance is met but carry a stale TBD rider (R-14-04-002, R-14-04-009) are ALIGNED, with the rider noted.
11. **Pins.** The four D-41 declarations (CLM-002/011/020/029) are CP-03 rows with CP-02 fields (revision 0.8, DAG-007). CLM-007, CLM-014, CLM-022 and CLM-005.s01 are CP-02 (revision 0.7, DAG-006, from `1f4de36d9`, 2026-06-03). The SOW frontmatter pin `@e8f59a633` is revision 0.8.
12. **Rename residue.** The keyed DEL-14-04 surfaces have none. The following are observed for R3 but not keyed: engine.py line 1 docstring ("OpenPipeStress reviews"), the engine.py provenance contributor string "OpenPipeStress comparison engine" (an active identifier emitted into MappingRecord provenance), and `_REFERENCES.md`, which is not a keyed surface.

## Canonical departures

None. All 8 CS assignments are inherited unchanged. The pattern rows use the CP-02, CP-03 and CP-09 treatments as written. CP-03 is written in `CanonicalSituation` with the CP-02 field set, as CP-03 directs.

## Convention friction

- CP-09 was also applied to VER-001, because its "review source parity" element is the overtaken parity record.
- The engine's diagnostic code `ARC-UNIT-MISSING` trips the F4 scan; CLM-005 carries GAP_WORDING_CHECKED for it.
- CS-04 allows `.sNN` rows for divergent ABI parts. Three were used: SEMANTIC_READY, which takes CS-05-style fields; the Rust baseline, which is FG-04; and the ruled Still-TBD items.

## UNKNOWN rows

None.

## Reverse pass

322 capabilities were answered:
- CLAIMED_BY: RC-14-0078 (exact-ID mapping producer) and RC-14-0150 (compare_analysis_runs).
- COVERS: RC-14-0186 (mapping schema), RC-14-0265 (tolerance schema) and RC-14-0256 (analysis-run schema). The engine consumes these contracts but does not own them.
- NOT_MINE with specific reasons: every overlapping path (the registers, SPEC.md), plus the adjacent comparison, report and desktop capabilities.

The routing notes ("Python only, reached from tests"; "Display only") agree with the sealed F7 treatment. The reverse pass did not change my view of any sealed row.

## Batch consistency

`--batch` over the DEL-14-04 and DEL-14-05 forward files: PASS, 0 consistency findings.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Piping selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
