# Notes — DEL-14-01 Immutable model state records (W3, PKG-14)

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-01_Immutable model state records/` (the path has spaces; tokens resolve at the freeze).
- Project-root tokens (`schemas/`, `core/`, `apps/`, `tests/`, `fixtures/`) resolve under `projects/chirality-piping/`.
- Parity records are cited at the repository-root path `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`, where they exist at the freeze.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` is the suite-level gate record. No per-test status is asserted beyond it ("not rerun").

## Judgment calls

- **Table splitting.** Only the Requirements table (CLM-011) is split into
  `.r01`-`.r10`. CLM-018 includes its single optional `.r01` row. Other
  tables (CLM-003, CLM-004, CLM-006, CLM-012, CLM-013, CLM-027, CLM-029) are
  assessed as blocks. Where parts differ, the most substantive gap sets the
  disposition and the Notes name the other gaps.
- **JCS hash basis (FG-DEL-14-01-01).** The model-state schema fixes
  `sorted_compact_json_payload_hashes`, and the Python persistence service
  emits `SORTED_COMPACT_JSON`. The D-41 R5 T2A repair (DEC-074 E1) relabelled
  it as explicitly not RFC 8785/JCS. DEC-010, AB-00-04 and SPEC 3.2/4.4 still
  state a JCS-compatible basis. Rows requiring JCS are therefore
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · BASELINE;RECORD`.
  DEC-074 E1 selected evidence requirements and did not amend DEC-010. This
  follows the W2 JCS-scope reading (RESOLUTIONS, FG-DEL-02-05-03 terms).
- **No product save path (FG-DEL-14-01-02, F7).** No desktop or Rust path
  creates, saves or lists named model-state records. Desktop panels use
  `state:TBD` or `not generated` placeholders, and the 0.2 run record
  synthesises `state:<model_ref>:preview`. R1 ("the product shall save") is
  `PARTIALLY_IMPLEMENTED`. Contract-level rows are `ALIGNED` with
  `PRODUCT_CALLER: NONE`.
- **External-reference screening (FG-DEL-14-01-03).** R9 and the CLM-027
  trade-off are `PARTIALLY_IMPLEMENTED` at `INVARIANT` tier (IP_DATA;SECURITY).
  Remaining R01 points to R9 through `OPEN_ACTION`.
- **Protected-content review (FG-DEL-14-01-04).** CLM-013, CLM-020 and CLM-021
  require a protected-content/private-data review record. None was located, so
  they are `UNKNOWN`, following the W1 DEL-07-02 CLM-026 resolution. R10, the
  prohibition itself, is `ALIGNED` on inspection: the only model-state
  examples are invented inline test records.
- **Remaining R02 (Phase G).** `ALIGNED` with `OPEN_ACTION` to CLM-011.r01.
  The product slice for this deliverable has not landed, so the open action
  is real. This is not CP-06.
- **MEMORY.** The Boundary Decisions, Verification and Remaining TBDs
  sections are read as part of the dated 2026-05-04 entry, which makes them
  history. This is recorded with `GAP_WORDING_CHECKED`.

## Canonical departures

None. CS rows inherit. CS-04 has `.s01` (PKG-00 SEMANTIC_READY), `.s02`
(Still TBD items since ruled) and `.s03` (Resolved Baseline JCS clause).

## Convention friction

- A CP-04 SURFACE row is the only place to record the model-state schema's
  `$id` and title carrying the former name. That is code-side residue, but it
  is not one of the four identifiers CP-04 lists, so the default variant was
  used.
- C1 allows unsplit blocks, which forces one disposition on tables with
  mixed gaps. Notes carry the secondary gaps.

## Smallest checks for UNKNOWN rows

- CLM-013, CLM-020, CLM-021: locate a protected-content/private-data review
  record for model-state examples and fixtures (inline invented records in
  `tests/test_project_persistence_service.py`, and
  `fixtures/persistence/`), or run and record that review.

## Reverse pass

- Claimed: RC-14-0289 (model-state schema).
- F5-specific NOT_MINE reasons are given for 13 overlaps. These are the
  desktop `lib.rs` capabilities (cited only for `create_local_project`), the
  comparison engine, the report sections, the ComparisonPanel, the registers
  and the persistence fixture.
- The reverse pass confirmed the forward view. No product capability saves or
  lists model-state records; RC-14-0005 (project create/open/save) stores
  model and run envelopes only. The sealed ledger is unchanged.

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-14 worker ledgers
(DEL-14-01, DEL-14-02, DEL-14-03): **PASS, 0 consistency findings**. No
flagged pair needed a justification.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Claim fence

These notes and ledgers make no release, approval, compliance or
certification claim. Dispositions are agent judgments, not owner rulings. No
protected standards text, vendor or private data is quoted, and no external
equation artifact is used as evidence (DEC-043).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
