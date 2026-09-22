# DEL-17-08 notes — GLB/glTF review geometry export (W3, PKG-17, worker G3)

Forward ledger sealed: `DEL-17-08_SEAL.txt` (SHA-256
`490381cfe063811b93f66beef6f97632e85758f6c6504bb367be5fd5e938bcab`). 110 rows,
all required keys, no `.rNN` blocks split, no `.sNN` sub-claims. Read at the
freeze `00115c71931bcae79909602d653740d3bb72dfa1`. Not an R0 pilot.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/`.
- Python engine: `core/handoff/review_geometry/package.py`. Schema:
  `schemas/review_geometry_export.schema.json`. Fixtures:
  `fixtures/review_geometry/invented/`. Tests:
  `tests/test_review_geometry_export_package.py`.
- Product surface: `apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx`,
  a separate TypeScript glTF builder. It applies a declared Z-up to Y-up rotation.
  The Python path accepts only input already in the target basis.
- `DEC-074 O11/E7`: the SOW cites it for the selected JSON line profile. Its option
  text is in the July run's excluded `PROPOSED_*` file and was not read (authority
  map note on DEC-074). Rows that lean on it say so.

## Judgment calls

- **GLB not produced.** The SOW itself places GLB outside the selected profile. I
  could not read the O11 text, so OUT-001 and CONTEXT description are
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE` (MEDIUM), not `DEFERRED_BY_RULING`.
- **REQ-043.** The impossible-bend diagnostic does not exist. Every other listed
  diagnostic exists and is tested. `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`. R02 is
  `ALIGNED` with `OPEN_ACTION` pointing to REQ-043 (F2). CLM-022 is `ALIGNED` as an
  accurate declaration and carries GAP_WORDING_CHECKED.
- **R01** (PDU-031 generator/timestamp policy) is accurate. REQ-034 accurately
  declares the hold and is `ALIGNED`, so R01 takes the gap itself:
  `DOCUMENTED_UNIMPLEMENTED · NOT_STARTED · OWNER_HOLD · OWNER`.
- **REQ-042.** The desktop preview records the centerline simplification as
  `approximated`. The Python fixture package has only `exported` and `omitted`
  entries. `PARTIALLY_IMPLEMENTED` (MEDIUM).
- **REQ-021.** The extras location is recorded. No stripping or consumer-risk
  policy field exists, although the authoritative sidecar mitigates the risk.
  `PARTIALLY_IMPLEMENTED` (MEDIUM).
- REQ-015 (extensions) and REQ-033 (binary hashes) are `ALIGNED`, not CP-11.
  - REQ-015 is a prohibition on undeclared extension use, which the emitted asset
    satisfies.
  - For REQ-033, every emitted member is hashed.
- FG-DEL-17-08-02: no per-family coverage classification exists. Rows: REQ-040,
  X-001, CLM-029.
- Architecture Basis Injection: same shared treatment as DEL-17-07 and DEL-17-09.
- Many setup-era "Phase A" and "remains TBD" rows are overtaken by the selected
  profile. They are `STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE`, with origin
  checked by `git log -S` at `7bee9ae41`. CLM-003's "Lifecycle role" is a
  lifecycle/review state and stays `STALE_REVIEW_OR_EVIDENCE` (F3 exception).
  CLM-021 and X-003 are later declarations (CP-01 `STALE_REVIEW_OR_EVIDENCE`).

## Canonical departures

None.

## Convention friction

- Several CLM-020 slot rows have two gaps: a substantive gap and CP-01 four-document
  pointers. Per C7, the substantive cause is the CauseTag and CP-01 is noted.
- These active identifiers carry the former name:
  - `GLTF_GENERATOR` "OpenPipeStress DEL-17-08 review geometry exporter 0.1.0"
    (pinned by a test, and the "fixed versioned generator" REQ-034 relies on);
  - the glTF `extras.openpipestress` identity key;
  - the scene name;
  - `ENGINE_PROVENANCE`;
  - desktop `document_kind` values and download filenames.
  They are recorded on the SURFACE row and here for R3. Renaming the generator
  string touches a test-enforced output.
- Not a claim row: the Python renderer also writes 0.0 for an absent node
  coordinate, the same pattern as DEL-17-07 FG-01. No DEL-17-08 claim lists absent
  coordinates.

## Smallest checks for UNKNOWN rows

None (no UNKNOWN rows).

## Reverse pass

Claimed: RC-17-0062 (Python builder), RC-17-0177 (schema), RC-17-0303 (fixtures),
RC-17-0296 (desktop panel). These paths overlap my citations, so each NOT_MINE is
specific:
- RC-17-0226: API boundary; C-002 cites it only;
- RC-17-0263: model schema, used as an input source;
- RC-17-0108: registers.

**The reverse pass changed my view of one sealed row (not edited):** the
CONTEXT architecture-basis-injection Notes say "JCS-labelled hashing hold". That
is an error I would correct. `_checksum` and `_source_model_hash` label records
`JCS_compatible_json_payload_hash`, but serialization is sorted-key, compact,
ASCII-escaped `json.dumps`, not RFC 8785. The label overclaims, as DEL-17-09's T2A
repair recognised for its own package.

## Batch consistency

`--batch` over DEL-17-07/08/09: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
