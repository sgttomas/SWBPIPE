# DEL-17-09 notes — Export adapter SDK and additional targets (W3, PKG-17, worker G3)

Forward ledger sealed: `DEL-17-09_SEAL.txt` (SHA-256
`a47829ee89450e0e4a1bb1881ac6f023bd48b50285c78c768de115059beb4461`). 91 rows,
all required keys, no `.rNN` blocks split, no `.sNN` sub-claims. Read at the
freeze `00115c71931bcae79909602d653740d3bb72dfa1`. Not an R0 pilot.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/`.
- Python engine: `core/handoff/export_adapter_sdk/package.py`. Schema:
  `schemas/export_adapter_sdk.schema.json`. Fixtures:
  `fixtures/export_adapter_sdk/invented/`. Tests: `tests/test_export_adapter_sdk.py`.
- Product surface: `apps/desktop/src/features/export-adapter-sdk/ExportAdapterSdkPanel.tsx`.
  This is a separate TypeScript preview. It is the only place an adapter template
  record exists. Its package hash comes from the shared
  `apps/desktop/src/services/hashService.ts`.

## Judgment calls

- **The SOW's exclusion of code, schemas, manifests and tests** (CLM-012, CLM-005,
  CLM-006, REQ-012 and the CLM-015/REQ-012 check) was overtaken by the 2026-05-28
  foundation. These rows are `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE`, with
  origin text at `7bee9ae41`. Runtime loaders, endpoints, sample adapters and target
  writers are still absent, as the SOW states.
- **REQ-007 versus the owner holds.** REQ-007 enumerates eleven validation
  categories. The builder implements nine, and R02/R03 hold "the exact taxonomy"
  for owner selection. The records are in tension. REQ-007 is
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · OWNER_HOLD · OWNER`. R02 is `ALIGNED`
  with `OPEN_ACTION` pointing to REQ-007.
- **R03** (reviewer role, signoff format, approval artifact) has no governing row
  carrying it, so it takes the gap itself (`DOCUMENTED_UNIMPLEMENTED · OWNER_HOLD`).
  **R01** (runtime loader binding) likewise takes the gap
  (`DOCUMENTED_UNIMPLEMENTED · NOT_STARTED`). CLM-030 names DEL-10-01 and DEL-10-02
  as candidate owners.
- **FG-DEL-17-09-01.** The contract has no external-execution policy or
  run-evidence field. Rows: REQ-010 (CP-11), CLM-027, AC-001, VER-001.
- **FG-DEL-17-09-02.** The checklist taxonomy differs from REQ-007 and from the
  CLM-029 topics (no package-inventory, manifest/hash-policy or external-execution
  category).
- **CLM-040 admission states.** The code has candidate, source-basis pending,
  source-basis admitted, rejected, quarantined and tbd. It lacks contract-ready and
  implementation-gated. `PARTIALLY_IMPLEMENTED` (MEDIUM).
- **CLM-018 is UNKNOWN.** The Python package uses the sorted-compact label and makes
  no JCS claim (tested). The desktop SDK panel's package hash, from the shared
  engine, is labelled `rfc8785_jcs`. The clause covers "all DEL-17-09-produced JSON
  checksum records" but ties the label to the Python serializer.
- **Architecture Basis Injection.** The shared treatment applies (Python core plus
  TypeScript builder against DEC-009). This deliverable adds a second departure: its
  checksums deliberately carry a non-JCS label, against the injected "JCS-compatible
  hash basis".
- REQ-005 and REQ-006 are judged as contract-level obligations, since the SOW's
  subject is the admission contract. Their ALIGNED rows carry
  `PRODUCT_CALLER: NONE` for the Python builder.

## Canonical departures

None.

## Convention friction

- Procedure steps for future target admission (CLM-025, CLM-026) have never been
  exercised, because no target is admitted. I judged them against the record slots
  the contract provides. CLM-026 carries GAP_WORDING_CHECKED. A verifier may prefer
  CP-11.
- Code strings with the former name that the SOW does not name: `ENGINE_PROVENANCE`
  source_name, contributor, and the desktop `document_kind`. They are recorded here
  for R3.

## Smallest checks for UNKNOWN rows

- CLM-018: the owner or maintainer confirms whether the desktop adapter SDK preview
  package hash (shared hash service, `canonicalization: rfc8785_jcs`) falls under
  CLM-018. If it does, relabel it or amend the clause. If it does not, record the
  scope.

## Reverse pass

Claimed: RC-17-0034 (Python builder), RC-17-0268 (schema), RC-17-0202 (fixtures),
RC-17-0299 (desktop SDK panel). These paths overlap my citations, so each NOT_MINE
is specific:
- RC-17-0023: shared hash service;
- RC-17-0108: registers.

The reverse pass did not change my view of any sealed DEL-17-09 row. Reading this
deliverable's T2A label repair exposed the matching JCS overclaim in the DEL-17-07
and DEL-17-08 packages, which is recorded in their notes.

## Batch consistency

`--batch` over DEL-17-07/08/09: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
