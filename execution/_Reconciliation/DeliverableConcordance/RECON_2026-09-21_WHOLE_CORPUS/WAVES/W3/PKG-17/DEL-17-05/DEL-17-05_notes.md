# DEL-17-05 — W3 notes (PKG-17, worker G2)

Forward ledger sealed at SHA-256
`39b8be76395157ccefa6f9f0c66edd581017046c097b184bd6fa6eaf048a9587`
(129 rows: 104 required keys, the CLM-004, CLM-006 and CLM-007 `.rNN`
splits, and the sub-claims `SOW.s01`, `CLM-017.s01`, `CLM-029.s01` and
`CLM-030.s01`). DEL-17-05 was an R0 pilot. Its R0 forward ledger and notes were
read before encoding. The rows were encoded afresh, not copied, and carry the
§4 named repairs:
- OUT-001 cites the CHANGE-P4 parity and is STALE_REVIEW_OR_EVIDENCE;
- REQ-001 and REQ-004 are re-disposed under C6(a) as PARTIALLY_IMPLEMENTED;
- CLM-019 takes tier INVARIANT.

Dispositions are agent judgments, not owner rulings.

## Path aliases

- As DEL-17-04: full tokens in evidence columns; deliverable-local paths
  resolve at the freeze.
- Gate evidence: the B4.4 sweep summary (the log lists
  `tests/test_caepipe_external_run_package.py`) and
  `GATE:GATE_EVIDENCE/PR834_CI/HOSTED_SUMMARY.json` for public-CI
  non-failure. Not rerun.

## Judgment calls

- **The harness has no process launch.** `run.py` builds records for skipped,
  parser-only and attempted runs; nothing launches, captures or discovers
  output. Requirements whose governed behaviour is the live run are
  PARTIALLY_IMPLEMENTED or DOCUMENTED_UNIMPLEMENTED with cause
  DEFERRED_BY_RULING. The ruling is DEC-080: external-prover activation is
  owner-gated on lawful tool access and "nothing activates". These rows take
  BaselineClass RULED_CRITERION and AuthorityNeeded OWNER (FG-DEL-17-05-03).
  Rows that are met only by absence carry CP-11: REQ-001, REQ-012, CLM-004.r01
  and r08, CLM-006.r01, CLM-027 and the Context Envelope.
- **Missing record fields (FG-DEL-17-05-06, PARTIAL_SLICE).** The closed schema
  has no field for:
  - the MBF profile ID or TBD;
  - environment context (OS, permissions, launch context);
  - the source model ID;
  - the coverage-register version;
  - explicit manifest, ID-map or loss-report links.
- **Parser coverage (FG-DEL-17-05-07).** Unknown sections and unmapped rows
  produce warnings. Missing sections are not diagnosed, and expected columns are
  not enforced. Coordinate and loss-report uncertainty are not handled.
- **REQ-016.** The DEL-17-04 binding is a substring test for `del-17-04`, and
  `REQUIRED_MBF_REF` is unused. Acknowledgement blocking holds, so the row is
  PARTIALLY_IMPLEMENTED.
- **Phase A text** (CLM-011, 017, 023, 025, 029, 030, 034, and the CONF items'
  impacted-section pointers) is origin text (7bee9ae41). It is
  STALE_SETUP_SPECIFICATION with REPRESENTATION_MIGRATED (CP-01), or with
  DOC_BEHIND_CODE where code advanced (CLM-011, which also contradicts the
  mounted desktop panel). CLM-003's "Current phase" field is a phase state, so
  it is STALE_REVIEW_OR_EVIDENCE under F3.
- **PDU-055 declarations** (CLM-002/010/021/033) are CP-03 rows with CP-02
  fields: they pin revision 0.8 and DAG-007. Their delegation to Remaining is
  not relied on.
- **CLM-007.r01** (the export plan) is CP-08, because the plan is absent at the
  freeze. The public URL rows match the DEL-17-01 register and were not fetched
  (network is limited to GitHub reads).
- **Remaining.** R01 is ALIGNED with `OPEN_ACTION: DEL-17-05:SOW#CLM-019`
  (VERIFIED_NOT_VALIDATED). R02 (RF-001 and RF-002 dispositions) has no
  governing SOW row, so it carries the gap itself (DOCUMENTED_UNIMPLEMENTED ·
  AuthorityNeeded REVIEW).
- **"O10"** in CLM-019 and CLM-043 is not defined in any governing record
  found; DEC-080 is cited as the governing gate.
- **Architecture Basis Injection** has the same body as DEL-17-04 and the same
  judgment (ALIGNED, MEDIUM).

## Canonical departures

None. CS rows inherit their assigned values; CP-11 rows share one profile, which
batch mode compares by disposition.

## Convention friction

- The sub-claims split a Phase A list from a future-implementation list inside
  one block. These parts take different dispositions (C1), which the grain does
  not otherwise express.
- `VER-001` exists twice, under CLM-016 and under the praxeology section, with
  different content (keys are parent-qualified).

## UNKNOWN rows

None.

## Reverse pass

Four of 320 capabilities answered other than NOT_MINE: RC-17-0149, -0216 and
-0218 CLAIMED_BY; RC-17-0141 (desktop External prover run panel) PARTIAL. F5
reasons are specific for RC-17-0108, -0117, -0142, -0231 and -0314. The R0
reverse file (96 pilot capabilities, all NOT_MINE) was read only after sealing;
the new inventory now has capabilities for `core/handoff/caepipe_external`,
which answers the R0 review's F7 gap. The reverse pass did not change my view of
any sealed row.

## Batch consistency

`--batch` over the three G2 forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
