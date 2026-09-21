# DEL-03-04 — worker notes (W2, PKG-03, group G2)

Branch connection component model fields. Forward ledger: 69 rows (67 required
keys, 2 `.sNN` sub-claims; no optional `.rNN` block split). Frozen state
`00115c719`. Agent judgments only; nothing here is an owner ruling.

## Path aliases

- Project-root tokens (`schemas/…`, `fixtures/…`, `tests/…`, `core/…`,
  `apps/…`) resolve under `projects/chirality-piping/`; project documents are
  cited as `projects/chirality-piping/…`.
- SOW-STAGE2 parity and claim-map records are repository-root AgentRuns paths
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`), as in
  `EVIDENCE_MAP.csv`.
- Tests are cited as `<file>::<test name>`.
- Deliverable-local records (`_STATUS.md`, `MEMORY.md`, `Review_Findings.csv`,
  `_run_records/…`) sit under a folder name with spaces and commas. Part D
  forbids spaces in evidence columns, so they are cited in `ContextRefs` and
  described in Notes.
- Suite-level pass status: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`
  (the sweep log shows the component schema test module executed inside the
  1,138-test Python PASS). No per-test status is asserted beyond it (A6; not
  rerun).

## Judgment calls

1. **Product-path slice absent from the SOW (FG-DEL-03-04-01).** The
   2026-06-21 TP-R4-D2-BRANCHSTRESS-001 tranche (DEC-045) landed branch inputs,
   unit checks and diagnostics in `core/product_physics`, tee creation in the
   operation applier, and desktop creation/inspector fields. The SOW, migrated
   2026-07-14, still names only the schema, fixture and schema test. Recorded
   on the SOW SURFACE row and on the rows whose text limits or TBD-lists the
   evidence (CLM-006 GUI TBD, CLM-010 scope excluding product code, CLM-014
   verification TBDs). Neutral evidence lists (CLM-003, CLM-012, CLM-015) stay
   `ALIGNED`.
2. **RQ-002 → UNKNOWN.** The code side holds (no defaults; fixture values
   unset; C-120 values invented and labelled). The requirement's own
   verification names a protected-content and fixture review that was not
   located; the schema test's seven-term keyword denylist is not a review. W1
   resolution for DEL-07-02 CLM-026 followed (INVARIANT, IP_DATA;RECORD,
   REVIEW).
3. **RQ-005 → PARTIALLY_IMPLEMENTED, INVARIANT, VALIDATION.** Branch unit
   checks exist in product validation and the applier, but the verification
   column requires branch tests for accepted, rejected and absent units; none
   were found. OPS-K-UNIT-1 is a contract invariant and the gap touches its
   verification (F8). The layer vocabulary has no units layer; VALIDATION is
   the nearest.
4. **RQ-006 → PARTIALLY_IMPLEMENTED, INVARIANT, IP_DATA, REVIEW.** Bounded
   strict-schema no-bypass evidence (PDU-019) exists; the formal review is held
   and product-path branch records carry no privacy class. Remaining R01 points
   `OPEN_ACTION` here (F2).
5. **R01 flagged `AdoptedByReference=YES`**: the hold's source is DEC-074 O7,
   whose adopted text is in an excluded `PROPOSED_*` file and stays unread
   (R0 review §7).
6. **VER-001** takes the CP-09 treatment (parity overtaken:
   `AnyPassMatchesFrozen=NO`; the 2026-07-16 DEC-081 edit postdates all PASS
   records), as does the matrix OUT-001 row. The purpose-section OUT-001 is
   judged on substance (`ALIGNED`).
7. **CONTEXT** surface, `.s01` (PKG-00 SEMANTIC_READY statement, overtaken by
   D-40/D-43) and `.s02` (Still-TBD list partly settled) as recorded in the
   carry-forward notebook; both `.sNN` rows are `STALE_SETUP_SPECIFICATION`
   (origin 7bee9ae41). Context Envelope note is pure sizing → NON_NORMATIVE.
8. **Origins (F3)** by `git log -S` on the deliverable folder: CLM-006,
   CLM-010, CLM-014, CLM-020 text from 1b62eb5b8 (2026-06-04) →
   `STALE_REVIEW_OR_EVIDENCE`; CLM-021 four-document line and CLM-022 records
   list from 7bee9ae41 → `STALE_SETUP_SPECIFICATION`.
9. **Observation, no row:** `schemas/component.schema.yaml` `$id` and `title`
   still carry the former product name. This SOW does not name that identifier,
   so no CP-04 row is raised here; R3 may cluster it with other active-code
   residue.

## Canonical departures

None. CS-01, CS-02, CS-04, CS-06 (OK) and CS-07 rows are inherited unchanged.

## Convention friction

- The no-spaces rule for evidence columns prevents citing deliverable-local
  review and run records as evidence; the REVIEWED rating on CLM-030 rests on
  `Review_Findings.csv` and MEMORY cited in `ContextRefs`.
- CP-09 is written for OUT-001 and parity rows; applying it to VER-001 is an
  extension, disclosed in the row.
- No divergence layer fits unit/dimension invariants.
- **Custody note for the verifier:** my scratch builder files in this folder
  (`_scratch_*`) disappeared at about 16:57 local time, after this ledger was
  sealed, without my action. The sealed forward file was re-hashed afterwards
  and matches the seal (`c1e22696…`).

## Smallest checks for UNKNOWN rows

- `SOW#CLM-011/DEL-03-04-RQ-002`: search review returns, `Review_Findings.csv`
  and the D-41 records for a protected-content and fixture review disposition
  covering the branch fixture record and preview `component:C-120`; if none
  exists, run that review.

## Reverse pass

376 capabilities answered: PARTIAL 3 (component schema RC-03-0073, component
fixtures RC-03-0109, component-symbol insertion RC-03-0294), COVERS 4
(library-import provenance RC-03-0072/0107, preview fixtures RC-03-0167,
single-field edit rules RC-03-0321), NOT_MINE 369. Every capability whose entry
points hit a path this ledger cites has a capability-specific reason (F5). The
reverse pass did not change my view of any sealed row. It showed no PKG-03
capability for the product-physics branch modifier path, which is consistent
with that path being solver-owned while the branch field contract is this
deliverable's.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-03-04, DEL-03-05 and DEL-03-06:
PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
