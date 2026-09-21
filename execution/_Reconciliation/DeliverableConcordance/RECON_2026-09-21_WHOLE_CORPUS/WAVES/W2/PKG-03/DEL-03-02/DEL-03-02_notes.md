# DEL-03-02 notes — Pipe section and component library schema

Worker: W2 PKG-03 G1 (TASK). Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Forward ledger 85 rows (75 required keys, 9 optional `.rNN` for CLM-003, 1
`.sNN`); sealed before the routing file was read.

## Path aliases

- `DEL0302/` = `projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-02_Pipe section and component library schema/`.
  Cited in `ContextRefs` only (the path has spaces).
- Parity records: `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/WORKING-P1-PKG03/manager-validation/DEL-03-02/` (root-level).

## Judgment calls

- **Rename (CP-04).** The SOW prose no longer names the former product, but
  both schemas it names carry it in `$id` and title. It is recorded once on
  the SURFACE row.
- **Four-document residue (FG-DEL-03-02-02).** The kit front matter preserved
  as CLM-001/008/015/022 is metadata, so SRE. CLM-019 step 6 is initial-migration
  text, so STALE_SETUP_SPECIFICATION. CLM-020 and CLM-021 date from 2026-06-04,
  so SRE.
- **Review state (FG-DEL-03-02-01).** All three PKG-02 findings are
  `ACCEPT_AS_IS` / `RESOLVED`. CLM-014 still lists "human review disposition"
  as open: RECORD_DRIFT, MEDIUM because of the hedge "or governed outside this
  reconciliation scope".
- **Round-trip.** REQ-09, REQ-12 and CLM-013/REQ-12 explicitly place
  round-trip outside their own claim, so they are ALIGNED with
  GAP_WORDING_CHECKED. VER-001 asks for deterministic persistence
  compatibility, and none is evidenced for library documents, so it is
  PARTIALLY_IMPLEMENTED (FG-DEL-03-02-03), as in DEL-03-01.
- **CLM-018 last bullet** ("human-owned dependency declarations are not
  tracked") is ALIGNED: `_DEPENDENCIES.md` still records no declared
  dependencies. Its 13 register rows are extracted, not human-declared.
- **CF-001** is ALIGNED as a conflict record. Both files do cite 0.7. Their
  staleness against 0.12 sits on CS-01 and the pin rows.
- **REQ-10 / CLM-013 REQ-11** use the library-import seam and its tests (owned
  by DEL-03-07) as corroboration. They are MEDIUM.
- **F7.** The claims are about schemas, fixtures and tests. The schemas are
  consumed by the import seam and the Library Manager.

## Canonical departures

None.

## Convention friction

As in DEL-03-01: sub-claims were used sparingly, and review evidence sits in
`ContextRefs`. The OUT-001 matrix row names CLM-008, which after migration is
the specification front matter. The traceability intent is unaffected, so the
row stays ALIGNED under CP-09.

## UNKNOWN rows

None.

## Reverse pass

- CLAIMED_BY: RC-03-0073 (component schema), RC-03-0326 (section schema) and
  RC-03-0109 (component fixtures).
- CONSTRAINS: RC-03-0072 and RC-03-0107 (the library-import seam, via REQ-10
  and AB-00-07).
- NOT_MINE with specific reasons for the F5 overlaps: hanger import in the
  cited checker file (RC-03-0198), the units schema and the registers.
- NOT_MINE for section-property calculation, which is SOW-051 (DEL-03-08):
  RC-03-0175 and 0338.

The reverse pass did not change my view of any sealed row.

## Batch consistency

PASS, 0 findings, across the three G1 ledgers. The REQ-09 difference from
DEL-03-01 REQ-03-01-007 is justified as recorded in the DEL-03-01 notes.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
