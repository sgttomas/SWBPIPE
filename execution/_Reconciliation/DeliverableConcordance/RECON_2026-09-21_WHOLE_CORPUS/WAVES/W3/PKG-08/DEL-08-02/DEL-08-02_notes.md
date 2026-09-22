# DEL-08-02 Audit manifest and model hash — worker notes (W3, PKG-08, worker G1)

Forward ledger `DEL-08-02_forward.csv` (97 rows). It has 62 required keys,
plus the `.rNN` splits of CLM-004, CLM-006, CLM-011, CLM-025. CLM-011 has
no keyed requirement rows, so it had to be split to judge R1–R10. It also
has `.sNN` sub-claims SOW.s01, CLM-005.s01–s03, CLM-024.s01 and
CONTEXT#architecture-basis-injection.s01/.s02. The ledger is sealed in
`DEL-08-02_SEAL.txt`. The shared notebook is `../_WORKER_DEL-08-01_NOTES.md`.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/`.
- Audit manifest crate: `core/reporting/audit_manifest/` (Rust name
  `open_pipe_stress_audit_manifest`).
- The canonical input manifest is `apps/desktop/src/services/inputManifestService.ts`
  (document kind `openpipestress.current_session_input_manifest`).
- The shared RFC 8785 renderer is `core/serialization/canonical_json/`.

## Judgment calls

- **Canonicalization (FG-DEL-08-02-01), possible defect.**
  - The crate's `hash_canonical_json` hashes "project-local deterministic
    JSON": BTreeMap key order and caller-supplied number strings. It states it
    is not a JCS conformance claim, and its `Canonicalization` enum has no
    RFC 8785 value.
  - The product computes the model and input-manifest hashes with
    `canonicalSha256Hex`, whose sibling helpers label `rfc8785_jcs`. It then
    labels them `project_local_deterministic_json` in the package audit
    manifest, because the wire (`report_package/src/wire.rs::audit_canonicalization`)
    accepts no other JSON label.
  - The rows restating the JCS basis (CLM-004.r02, CLM-006.r02, CLM-011.r02,
    CLM-024.s01) are IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT ·
    PROJECT_BASELINE · BASELINE;RECORD · OWNER. MEDIUM confidence: the values
    are RFC 8785 but the recorded basis says otherwise.
  - This is an R4 code-change candidate, related to the W2 note on
    persistence hashing.
- **Versions (FG-DEL-08-02-02).** SolverVersionStamp records the solver
  name, version and build ref; the wire enforces the linked product-physics
  identity. No application/software version exists (R4, CLM-006.r03,
  CLM-025.r04, V-7).
- **Rule packs (FG-DEL-08-02-03).**
  - Engine capture and findings exist.
  - The product sends `rule_pack_refs: []` and `active_rule_packs: []`, and
    refuses packages when a rule check is active.
  - Recording claims (R5, CLM-004.r04, CLM-005.s02, CLM-006.r04, description)
    are PARTIALLY_IMPLEMENTED.
  - Boundary claims (CLM-004.r05, CLM-025.r03, V-5) are ALIGNED with
    `PRODUCT_CALLER: NONE`.
- **R9 status distinction.** AuditManifest has no analysis-status field;
  status lives in the result-export envelope and the package request. The row
  is PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE · LOCAL_DESIGN. The
  no-claim half holds.
- **Assets.** Asset hashing is separate and tested, but the product sends
  `assets: []`. Rows resting on it carry `PRODUCT_CALLER: NONE`.
  - V-4 (alter an asset, digest changes) has no located test, so it is
    PARTIALLY_IMPLEMENTED.
  - CLM-006.r05 lacks media type and inclusion-policy fields in
    AssetManifestEntry.
- **CLM-006.r01 input manifest** is ALIGNED. The input manifest carries model,
  unit, solver (mode and settings), load, rule-pack and external-asset bases
  with an rfc8785_jcs identity policy.

## Canonical departures

None.

## Convention friction

- CLM-011's requirement rows exist only as optional `.rNN` keys. Judging
  R1–R10 separately forced the all-or-none split. The parent is CONTAINER.
- The SPEC pointer drift is minted as `SOW.s01` on the SURFACE key (same
  practice as DEL-08-01). SPEC §4.5 now holds plugin contracts; solver
  determinism is §5.5.
- CLM-005 (bullets, no `.rNN` keys) is assessed directly for its setup-only
  bullet. The other bullets are `.s01`–`.s03` because they take different
  dispositions.

## UNKNOWN rows

None.

## Reverse pass (after sealing; no edits made)

- The routing notes agree with the forward findings:
  - RC-08-0179 records "Number strings are caller-supplied and not normalised
    to RFC 8785".
  - RC-08-0056 records "Always refuses unless rule-check status is
    inputs-incomplete".
- I answered the input manifest (RC-08-0092) CLAIMED_BY, because it matches
  "canonical input manifest" in the description. The forward rows would not
  change.
- The RFC 8785 crate (RC-08-0037) is PARTIAL: the H5 record sits in this
  folder, but the crate serves every hash seam. A verifier may prefer COVERS.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-08-01/02/03: **PASS, 0 findings**.

## Selectability

`SelectableUnderCurrentLoop` is NOT_APPLICABLE on every row (C9). Piping
selects work through owner-steered work graphs since 2026-09-19.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
