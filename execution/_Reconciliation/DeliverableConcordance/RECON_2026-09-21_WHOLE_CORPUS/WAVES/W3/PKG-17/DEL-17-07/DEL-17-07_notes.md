# DEL-17-07 notes — Conservative PCF subset exporter (W3, PKG-17, worker G3)

Forward ledger sealed: `DEL-17-07_SEAL.txt` (SHA-256
`b5fce3cdad0804dc14f84fe2960384a547fed5c76129be7fba6ee36dc55a0757`). 102 rows,
all required keys, no `.rNN` blocks split, no `.sNN` sub-claims. Read at the
freeze `00115c71931bcae79909602d653740d3bb72dfa1`. Not an R0 pilot.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/`
  (cited with spaces; resolves at the freeze).
- Python engine: `core/handoff/pcf_export/package.py`; schema
  `schemas/pcf_export.schema.json`; fixtures `fixtures/pcf_export/invented/`;
  tests `tests/test_pcf_export_package.py`.
- Product surface: `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx`,
  rendered in the Exports dock section of `apps/desktop/src/App.tsx`. It is a
  separate TypeScript packet builder; it does not call the Python engine.
- Suite evidence: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` (not rerun).
- SOW references `PLAN-EXPORT-INTEROP` = `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`,
  deleted by `349a2ab33` (2026-06-03) and absent at the freeze.

## Judgment calls

- **FG-DEL-17-07-01 (possible defect, MEDIUM).** Two silent fallbacks: the Python
  renderer writes 0 for an absent node coordinate (`_coord` defaults), and the
  desktop packet converts any section quantity unit other than `m` with factor 1
  (treated as millimetres). Route drafts accept a free-text length unit. Rows:
  CLM-006, REQ-021, CLM-030, CLM-043.
- **FG-DEL-17-07-02.** No per-family classification record exists in the profile
  or schema; only straight pipe is in scope. Rows: CLM-005, REQ-013, CLM-028,
  CLM-029.
- **FG-DEL-17-07-03 (IP_DATA, needs review).** No fixture provenance record exists
  to the SOW template. Separately, the fixture pairs a nominal-size label with an
  OD and wall thickness that coincide with a published standard dimensional-table
  entry (values deliberately not restated). The boundary forbids copying protected
  dimensional tables; whether one entry counts is unsettled. REQ-041 and CLM-045
  are `UNKNOWN · AUTHORITY_UNCLEAR · INVARIANT`. The provenance gap rows (REQ-040,
  CLM-007, CLM-025, CLM-031, CLM-034, AC-001, VER-001) are `PARTIALLY_IMPLEMENTED ·
  INVARIANT · IP_DATA;RECORD`.
- **Latent writer defect.** `write_pcf_export_package` writes `model.pcf` and five
  JSON members but not `unit_system_disclosure.json`, which the manifest lists. It
  is latent because the route withholds materialization for the fixture (tested).
  Recorded on CLM-030.
- **Architecture Basis Injection.** The injected baseline says "Rust core/application
  services" (DEC-009). The export is Python core plus a TypeScript desktop builder.
  No ruling was found, so the row is `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR ·
  PROJECT_BASELINE · BASELINE;RECORD · OWNER`. This is likely cross-cutting.
- REQ-034 (component approximations) is CP-11 because no component is mapped.
- Deleted-plan pointers that are origin text use `STALE_SETUP_SPECIFICATION ·
  BASIS_POINTER_STALE` (F3 over CP-02's class) and carry no CS code.
- The CLM-008 DEL-15-02 row is stale because DEL-15-02 is now named "Target
  mapping and unsupported-behavior contract".
- R01 (RF-001 human disposition) is accurate and open. No governing row carries
  it, so F2's second branch applies: `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED ·
  REVIEW`.

## Canonical departures

None. CS-01/03/06 rows inherit their assignments. CP-01, CP-02 (for CP-03
blocks), CP-04, CP-05, CP-09 and CP-11 are applied as written.

## Convention friction

- CP-02 names "section reference" pointers `STALE_REVIEW_OR_EVIDENCE`. F3 sends
  origin-text pointers to `STALE_SETUP_SPECIFICATION`. I followed F3 and left
  CanonicalSituation empty on those rows.
- Rename residue in active code that the SOW does not name is not a CP-04 row.
  Examples: `ENGINE_PROVENANCE` source_name "OpenPipeStress ...", schema `$id`
  host `openpipestress.org`, desktop download filenames `openpipestress-preview-*`,
  `document_kind` `openpipestress.technical_preview.*`, and "OPS-" identifier
  prefixes. They are recorded here for R3.

## Smallest checks for UNKNOWN rows

- REQ-041 and CLM-045: a maintainer reviews whether the OD and wall-thickness
  values paired with the fixture's nominal-size label in
  `fixtures/pcf_export/invented/*` (also asserted in the test) come from a
  protected dimensional table. The maintainer then records a disposition or
  replaces them with clearly invented values.

## Reverse pass

Claimed: RC-17-0260 (Python builder), RC-17-0078 (schema), RC-17-0053
(fixtures), RC-17-0247 (desktop PCF panel). The paths below overlap my citations,
so each NOT_MINE gives a specific reason:
- RC-17-0093: shared redaction route control;
- RC-17-0108: planning registers;
- RC-17-0142: user guide.

**The reverse pass changed my view of one sealed row (not edited):** CONTEXT
architecture-basis-injection Notes say "JCS-labelled hashing hold". That is an
error I would correct. The Python PCF package labels its checksums
`JCS_compatible_json_payload_hash`, but it serializes with sorted-key, compact,
ASCII-escaped `json.dumps`, which is not RFC 8785/JCS. DEL-17-09's D-41 T2A repair
narrowed exactly this label for its own package. The row's disposition stands; the
Notes clause should say the JCS label is unsupported.

## Batch consistency

`--batch` over DEL-17-07/08/09: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping selects
work through owner-steered work graphs, not `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
