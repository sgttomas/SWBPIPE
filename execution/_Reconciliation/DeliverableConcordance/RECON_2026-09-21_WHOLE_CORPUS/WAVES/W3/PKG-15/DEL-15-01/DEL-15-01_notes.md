# Notes — DEL-15-01 (W3 PKG-15, worker G1)

Agent judgments, not owner rulings.

## Path aliases

- Project-root evidence tokens (`core/…`, `apps/…`, `schemas/…`, `tests/…`)
  resolve under `projects/chirality-piping/`. Deliverable-local files are cited
  with the full `projects/chirality-piping/execution/PKG-15_Handoff and External
  Prover Workflow/1_Working/<DEL folder>/…` path (spaces resolve at the freeze).
- Parity records and D-41 run records are cited at the repository root
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…`) and the
  project (`projects/chirality-piping/execution/_Coordination/…`) as they exist.
- `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` stands for the gate sweep's
  `pytest -q tests` run (1,138 passed, no failures); tests are not rerun (A6).
- `ScopeOfWork.md lines a-b` in DeclaredState refers to the frozen SOW.

## Judgment calls

- The deliverable is an API contract. Claims about the schema are satisfied by
  `schemas/handoff_package.schema.json` (strict 2020-12, fixture, focused
  tests) and carry `PRODUCT_CALLER: NONE`: only tests use the schema, and the
  desktop HandoffPanel emits its own technical-preview shape. That product-side
  question is judged under DEL-15-03.
- FG-DEL-15-01-01 (IMPLEMENTED_DIFFERENTLY · OTHER · LOCAL_DESIGN, MEDIUM):
  SOW slot and requirement text names properties the schema never had
  (model_hash provenance; units_manifest entries/diagnostics; entity_ids record
  fields). CauseTag OTHER because the text post-dates and misdescribes the
  schema; it is neither code advancing nor a rename.
- Splits: CLM-006 (.r01-.r10; parent assessed on its intro sentence) and
  CLM-011 (.r01-.r13; parent CONTAINER).
- STATUS R01 (Phase H program item) is PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  PROJECT_BASELINE · OWNER: accurate and open, and no governing row in this
  ledger carries product-level Phase H integration.

## Canonical treatment and convention friction

- Keyed CS rows inherit CS-01/02/03/04/06/07 without departure.
- CS-04: `.s01` (PKG-00 at SEMANTIC_READY; all PKG-00 members IN_PROGRESS at
  the freeze) and `.s02` (Still TBD keeps the physical project package/container,
  superseded by DEC-017/SCA-003, and the import/export format list, decomposed
  under SCA-004/OI-004) are added on every deliverable.
- D-41 R5 T7 PDU-055 declarations: `CanonicalSituation=CP-03`, fields per CP-02
  (revision 0.8 / DAG-007 pins). Remaining delegation not relied on (A4).
- Text naming revision 0.7/0.8 or DAG-002/006/007 as the approved or current
  basis: CP-02. A row ID such as `DAG-002-E0805` used as an identifier is not a
  finding.
- Four-document residue (first present at `7bee9ae41` by `git log -S`): CP-01
  as STALE_SETUP_SPECIFICATION.
- CP-04 rename residue recorded once on each SOW SURFACE row (default
  variant); items judged on substance.
- CP-05 on each STATUS SURFACE (Last Updated 2026-07-12; history entry
  2026-07-16). CP-09 on VER-001 and the matrix OUT-001 (PASS parity records of
  2026-07-14 bind a SOW hash older than the D-48 Wave 2 edits of 2026-07-16/17).
- Remaining "obtain human dispositions" items are accurate and open with no
  governing row: DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · LOCAL_DESIGN ·
  RECORD · REVIEW (F2 second branch).
- Friction: CP-02 versus F1 for a single stale pointer inside an otherwise
  accurate block. I followed F1 (block not aligned) and named the accurate
  parts in Notes. Layers for pure product-implementation gaps have no exact
  vocabulary entry; I used RECORD.
- The OI-015 citation for the handoff package container (DEL-15-01/02 CLM-004)
  is treated as a stale pointer: OI-015 has never named a container in the
  frozen history.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row. Reverse files
answer all 290 routed capabilities; F5-specific NOT_MINE reasons are written
for every capability whose EntryPoints hit a path this ledger cites.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-15 forward ledgers: PASS,
0 findings.

## Selectability

SelectableUnderCurrentLoop is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
