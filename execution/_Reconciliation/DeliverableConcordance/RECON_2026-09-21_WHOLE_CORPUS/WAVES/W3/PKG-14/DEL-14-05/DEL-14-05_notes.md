# DEL-14-05 notes — Comparison mapping, tolerance, and export contracts (W3, PKG-14, worker G2)

Forward ledger sealed in `DEL-14-05_SEAL.txt` (109 body rows; 60 required keys;
.rNN splits for CLM-005, CLM-006, CLM-011, CLM-013, CLM-018, CLM-020;
sub-claims CLM-026.s01 and CONTEXT#architecture-basis-injection.s01/.s02).
Not an R0 pilot. Frozen state `00115c719`.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/`.
- Contracts: `schemas/comparison_mapping.schema.json` (MappingRecord, UnmatchedRecord, Csv/JsonExportContract, ReportSectionExportRef, ComparisonContractStatus) and `schemas/comparison_tolerance.schema.json` (ToleranceProfile/Rule, UnitMetadataPolicy). Tests: `tests/test_comparison_contracts.py` (structural) and the DEL-14-04 suite (`tests/test_analysis_run_comparison.py`), which schema-validates an automatic MappingRecord instance.
- Parity record (CP-09): root `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P4/checks/DEL-14-05/parity.md`.
- Gate: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (not rerun).

## Judgment calls

1. **Contract claims.** The schemas are the deliverable's artifacts. The only consumers are the test-only DEL-14-04 engine and a contract reference in the Python handoff exporter. ALIGNED contract rows carry `PRODUCT_CALLER: NONE` (F7).
2. **FG-DEL-14-05-01, report-section export.** ReportSectionExportRef requires only hash, provenance and notice flags. Units, analysis statuses and limitations have no slot, and rendering is `reserved_reference_only_not_implemented`. The governing row is R008 (CLM-011.r08). CLM-005.r04, CLM-006.r06, CLM-013.r08, AC-001 and CLM-019 (step 9) share the finding: PARTIALLY_IMPLEMENTED, LOCAL_DESIGN, MEDIUM. STATUS R02 is ALIGNED with `OPEN_ACTION: CLM-011.r08`.
3. **FG-DEL-14-05-02, comparison exporters.** Deliverables.csv and SOW CLM-010 name "comparison exporters", but no code emits `comparison_review_csv_v1` or `comparison_review_json_v1`, and no instance-level export validation or CSV round trip exists. Rows CLM-010, CONTEXT#anticipated-artifacts and CLM-013.r05/.r06/.r07 are PARTIALLY_IMPLEMENTED at MEDIUM. The decomposition row itself says "export semantics", and that wording tension is recorded in the notes.
4. **FG-DEL-14-05-03, CP-10 unmatched-classification enum (owner item).** The SOW held the values TBD unless human-approved or source-defined, but the schema fixes an eight-value enum and no approval record was located. Rows CLM-006.r02, CLM-011.r03 and CLM-013.r03 are IMPLEMENTED_DIFFERENTLY, AUTHORITY_UNCLEAR, PROJECT_BASELINE, OWNER, MEDIUM.
5. **FG-DEL-14-05-04, validation.** STATUS R01 keeps tolerance suitability and independent validation open. No governing row in this ledger carries that work, so under F2 the Remaining row takes the gap itself: VERIFIED_NOT_VALIDATED, INVARIANT, VALIDATION, ENGINEERING. F2 names DOCUMENTED_UNIMPLEMENTED and PARTIALLY_IMPLEMENTED as examples, but the validation-gap class is the one that describes this subject.
6. **Stale TBD wording.** Condition, construction, verification and consideration rows whose substance is an overtaken setup TBD are STALE_SETUP_SPECIFICATION, DOC_BEHIND_CODE: exact CSV fields, exact JSON schema, mapping-schema details, JSON/CSV contract fields, several "Current status: TBD" verification rows, and CLM-026.s01. The F3 origin is `7bee9ae41` for each phrase. Requirement rows whose substance is met, with only a stale TBD rider (R006, R007), are ALIGNED, with the rider noted.
7. **OI-014 tolerance defaults.** CLM-005.r01 and R004 are ALIGNED because the condition and requirement are accurate and hold: the contract defines no defaults. DEL-14-04's OQ-14-04-001 is disposed differently (FG-03 of DEL-14-04) because its subject is the undecided default set itself, not the no-default guard. The batch check did not flag the pair because their bodies differ.
8. **CP-01.** The CLM-021 records list names the four retired documents. It is STALE_SETUP_SPECIFICATION, REPRESENTATION_MIGRATED.
9. **CP-05.** `_STATUS.md` Last Updated reads 2026-07-12, but the newest history entry is 2026-07-16.
10. **CP-04, rename residue (owner item).** The residue is recorded once on the SOW SURFACE: CLM-012 names three "OpenPipeStress" boundary sources. CLM-012 is judged ALIGNED on its substance. Also observed for R3, in the deliverable's own schemas and not keyed: `$id https://openpipestress.org/schemas/comparison_{mapping,tolerance}.schema.json` and both titles are active identifiers carrying the former name. The MEMORY mention ("OpenPipeStress DEV-001 revision 0.5") refers to a past state and is not residue.
11. **Pins.** The D-41 declarations (CLM-002/009/016/023) are CP-03 rows with CP-02 fields. CP-02 also applies to CLM-007 (DAG-006 approval record), CLM-018.r02 (revision 0.7), CLM-018.r04, CLM-020.r07 and CLM-026 (DAG-006, from `1f4de36d9`).
12. **CONTEXT ABI.** No Rust-core row was created for this deliverable, because its artifacts are language-neutral JSON schemas. `.s01` covers SEMANTIC_READY and `.s02` covers the ruled Still-TBD items; both are consistent with DEL-14-04.

## Canonical departures

None. All 9 CS assignments are inherited unchanged.

## Convention friction

- CLM-029 contains "source gaps" as setup wording, so its Notes carry GAP_WORDING_CHECKED.
- The register ("comparison exporters") and the decomposition ("export semantics") differ in whether exporter code is expected. I treated the register as governing too and recorded the tension, but did not settle it.

## UNKNOWN rows

None.

## Reverse pass

322 capabilities were answered:
- CLAIMED_BY: RC-14-0186 (mapping/export schema) and RC-14-0265 (tolerance schema).
- COVERS: RC-14-0078 and RC-14-0150. These are DEL-14-04 engine functions that consume or emit DEL-14-05 contracts.
- NOT_MINE with specific reasons: the report-section builders (DEL-08-06), the handoff export workflow, the governance documents, the desktop Comparison panel, the model-state engine, and the upstream record and unit schemas.

The routing notes ("Only tests reference the schema file by name" for the tolerance schema; "Referenced by core/handoff/exporter/workflow.py" for the mapping schema) agree with the sealed treatment. The reverse pass did not change my view of any sealed row.

## Batch consistency

`--batch` over both G2 forward files: PASS, 0 consistency findings.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
