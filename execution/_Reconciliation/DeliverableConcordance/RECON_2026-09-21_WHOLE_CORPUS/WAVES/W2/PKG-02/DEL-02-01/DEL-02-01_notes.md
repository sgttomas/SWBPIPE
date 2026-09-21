# DEL-02-01 notes — Canonical domain model schema (W2, PKG-02)

Forward ledger: 83 rows (77 required keys, 6 sub-claims), sealed in
`DEL-02-01_SEAL.txt`. Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`.
Every disposition here is an agent judgment, not an owner ruling.

## Path aliases

- `DEL02-01/` in `NormativeSource` means
  `projects/chirality-piping/execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-01_Canonical domain model schema/`.
  The folder name contains spaces, so it never appears in evidence columns.
- Project-root evidence tokens (`schemas/…`, `tests/…`, `fixtures/…`,
  `core/…`, `apps/…`) resolve under `projects/chirality-piping/`. Project
  documents are cited explicitly (`projects/chirality-piping/docs/…`).
- Parity records are root `execution/_Coordination/AgentRuns/SOW-PACKAGE-BATCH-ADOPTION-20260714-01/…`.

## Judgment calls

- **Schema claims have product callers.** The persistence envelope `$ref`s
  the model schema, desktop persistence stores `{schema_version, project}`
  model payloads, and the operation applier reuses the `Id` pattern. So F7's
  `PRODUCT_CALLER: NONE` marker did not apply to any aligned row.
- **REQ-02-01-09 (hash compatibility) ALIGNED with `GAP_WORDING_CHECKED`.**
  The schema's `Checksum` offers JCS and the desktop product hashes model
  payloads with RFC 8785. The Python persistence reference service labels
  its envelope hashes `SORTED_COMPACT_JSON`, not JCS. I treated that as a
  DEL-02-05 persistence-hash matter, not a limit on this schema's payload
  compatibility. The same fact is a finding in DEL-02-02 (FG-DEL-02-02-01),
  where the claim itself restates the Python hash basis. A verifier may read
  REQ-02-01-09 more broadly.
- **REQ-02-01-10 (no bypass at adapters) UNKNOWN.** Contract-level no-bypass
  controls exist, but I did not trace every product import path.
- **CLM-019 PARTIALLY_IMPLEMENTED.** I found two unmet verification
  expectations:
  - no provenance manifest for `fixtures/domain/`;
  - the stored minimal and physical fixtures predate the typed `LoadRecord`
    shape. `tests/test_model_schema.py` normalises them in memory before
    JSON Schema validation, so the committed bytes are not validated as they
    are. The minimal fixture is unchanged since `7bee9ae41`.
- **CLM-005 split into three sub-claims.**
  - `.s01`: the schema layout and fixture organisation were settled in code
    with no ruling located. This is CP-10: IMPLEMENTED_DIFFERENTLY ·
    AUTHORITY_UNCLEAR · OWNER.
  - `.s02`: the `$id` TBD was overtaken by the DEC-101 identity act, but the
    schema still declares `https://openpipestress.org/…`. This is CP-04 with
    default fields.
  - `.s03`: the container and migration TBD was ruled by DEC-017, DEC-019
    and DEC-028.
- **C-02-01-001 (objective conflict).** The conflict has dissolved: the
  context and register now list OBJ-012 and OBJ-014. I recorded the stale
  row as STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING (SCA-002
  revision 0.5 refresh).
- **Architecture basis injection.** The PKG-00 SEMANTIC_READY statement is
  `.s01`, because every PKG-00 deliverable is IN_PROGRESS. The Still-TBD
  list is `.s02`, because the rule grammar (DEC-022) and the container
  (DEC-017, DEC-028) have been ruled. The revision-0.5 supplemental criteria
  split off `.s01` for the DAG-006 pointer.
- **F3 origin checks.** I ran `git log -S` for every stale string. All
  setup-era text dates from `7bee9ae41`. The DAG-006 clause dates from
  `1f4de36d9` (2026-06-03) and stays as a CP-02 pin.

## Canonical departures

None. The CS rows inherit CS-01, CS-02, CS-04, CS-06-OK and CS-07. On
CLM-005.s02 and the SOW SURFACE row, CP-04 keeps its
STALE_REVIEW_OR_EVIDENCE class rather than taking F3's setup class. The
residue arises from the 2026-09-18 ruling, not from setup-era specification.
This is recorded in the row Notes and is not a departure.

## Convention friction

- Deliverable folders and PKG-00 status paths contain spaces. They cannot be
  evidence tokens, so they appear only in NormativeSource (by alias) or in
  ContextRefs.
- CP-04 ("record once on the SURFACE row") and a sub-claim about an active
  identifier (`$id`) overlap. I kept the SOW-text residue on the SURFACE row
  and gave the `$id` its own sub-claim, because the TBD bullet names it.

## Smallest check for each UNKNOWN row

- `SOW#CLM-011/REQ-02-01-10`: trace each product import path (native package
  open, PCF and CAEPIPE import, library import). Confirm that model-schema
  validation runs before data is accepted.

## Reverse pass and the sealed forward ledger

The reverse pass (254 capabilities) gave 2 CLAIMED_BY, 1 PARTIAL, 2 COVERS
and 249 NOT_MINE. It did not change my view of any sealed DEL-02-01 row. The
routing note for RC-02-0165 (a verification-only trace-gap fixture) agrees
with the ledger.

## Batch consistency

`validate_ledger_v2.py --batch` over the three sealed PKG-02 W2 ledgers:
**PASS, 0 findings.**

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Piping
selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
