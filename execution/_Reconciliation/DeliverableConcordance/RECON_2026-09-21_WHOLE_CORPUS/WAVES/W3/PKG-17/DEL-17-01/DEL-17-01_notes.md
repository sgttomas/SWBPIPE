# DEL-17-01 notes — W3 PKG-17, worker G1

Forward: `DEL-17-01_forward.csv` (96 rows: 87 required keys, the eight
`CLM-004.rNN` rows, and one `.s01` sub-claim). Sealed per `DEL-17-01_SEAL.txt`.
Reverse: `DEL-17-01_reverse.csv` (320 capabilities).

## Path aliases

- `D1` = `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/`
  (evidence tokens contain spaces and resolve at the freeze).
- SOW parity records live at the repository root, under
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…` (EVIDENCE_MAP).
- `tools/validation/check_four_documents.sh` and
  `check_min_viable_fileset.sh` exist only at the repository root. The project
  `tools/validation/` holds only `validate_architecture_basis.py` and
  `validate_dependencies_schema.py` (CLM-023 carries `ROOT_DOC:`).

## Judgment calls

- **PLAN-EXPORT-INTEROP is absent (FG-DEL-17-01-01, 8 rows).**
  `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` was deleted by `349a2ab33`
  (2026-06-03, "Retire DEV-001 and archive legacy coordination files"). The
  deletion kept no archive copy. A domain copy was removed by `511f1e1ba`
  (2026-08-20). The SCA-004 snapshot does not hold it either.
  - The rows that name the plan as a current source take the CP-08 fields.
    These are CLM-004.r01, F-17-01-001..006 (their Source Basis columns) and
    CLM-027 bullet 2.
  - The disposition class follows the F3 origin of the row's own text.
    CLM-004.r01 dates from 7bee9ae41, so it is STALE_SETUP_SPECIFICATION. The
    finding Source Basis columns and CLM-027 date from 7ddd4faf9 (2026-05-23),
    so they are STALE_REVIEW_OR_EVIDENCE.
  - Each finding's substance holds. Downstream exporters implement it (MBF
    first, optional user-owned harness, parsed CSV as handoff evidence,
    conservative PCF, review-only glTF, explicit loss categories).
- **DAG-005 is admitted without being listed (FG-DEL-17-01-02).** REQ-001 and
  CLM-019 require sources to be listed in `_REFERENCES.md` or the PKG-17
  reference index. DAG-005 is admitted in the register and cited by
  F-17-01-006, but appears in neither list. Both rows are PARTIALLY_IMPLEMENTED
  at MEDIUM confidence. REQ-001's "listed in" clause can be read as applying
  only to "otherwise admitted" sources; under that reading REQ-001 is aligned.
- **Four-document residue (FG-DEL-17-01-03, CP-01).** This covers the
  CLM-015 acceptance bullet, CLM-022 step 1 and the CLM-023 validation
  commands. The check script requires Datasheet, Specification, Guidance and
  Procedure.md, so it would now fail. CLM-024 is a past-tense record of the
  2026-05-18 P3 pass, so it is treated as accurate history (ALIGNED, MEDIUM).
- **The TBD register is mostly accurate.** TBD-17-01-001..003 are carried
  open by the MBF code and schema (const pins). TBD-004 and TBD-005 are kept
  TBD by the DEL-17-05 and DEL-17-07 SOWs. TBD-17-01-006 is STALE (DOC_BEHIND_CODE,
  MEDIUM): DEL-17-08 selected the first JSON glTF profile's identity policy
  (extras plus an authoritative sidecar), and the register does not record it.
- **External URLs were not fetched.** Rows that rest on vendor pages are
  MEDIUM confidence. Reachability is CLM-023's manual-review item.
- **Rename residue (CP-04).** Recorded once on the SOW SURFACE row. The
  frontmatter pin (revision 0.8 at `e8f59a63`) is split to `SOW.s01` (CP-02).
- **Architecture Basis Injection** names only SCA-003 and SCA-004, while
  SOFTWARE_DECOMP rev 0.12 says the basis is amended through SCA-008. It has
  no revision pin, so CS-04 does not apply; the row is judged as CP-02.

## Canonical departures

- CLM-004.r01 departs from CP-08's disposition class under F3 (origin
  7bee9ae41 gives STALE_SETUP_SPECIFICATION). CP-08's cause, tier, layer and
  authority are kept. The row's Notes record this.

## Convention friction

- CP-08 is written for evidence files. It is applied here to a missing
  *source* (a governing plan). F3's pin exception does not cover file pointers,
  so the origin test decides the class. As a result, one finding group spans
  two classes.
- DEL-17-01 is a DOC_UPDATE deliverable. The "implementation" evidence is the
  source-basis documents, plus downstream code as evidence that they are
  consumed.

## UNKNOWN rows

None.

## Reverse pass

- COVERS (16): exporter code, schemas, fixtures and panels that carry
  DEL-17-01 source IDs or TBD IDs (MBF, harness run and fixtures, PCF builder,
  fixture and panel, review geometry, SDK builder, fixture and panel).
  NOT_MINE for everything else. Rows whose entry points the ledger cites
  (registers, CONTRACT, SPEC, IP boundary, harness schema and panel, PCF
  schema, SDK schema) give reasons specific to the capability (F5).
- **Did the reverse pass change my view of anything sealed?** One point.
  DEL-17-08's SOW cites DEC-074 O11/E7 for its selected glTF profile. If that
  option did rule the identity policy, TBD-17-01-006's cause would be
  SCOPE_REDIRECTED_BY_RULING rather than DOC_BEHIND_CODE. The option text is
  in an excluded July PROPOSED_* file and was not read, so the verifier should
  weigh it. Nothing else changed.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-17-01/02/03: **PASS, 0 findings.**

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19 Piping selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No row
states or implies release, approval, compliance or certification. These
dispositions are agent judgments, not owner rulings.
