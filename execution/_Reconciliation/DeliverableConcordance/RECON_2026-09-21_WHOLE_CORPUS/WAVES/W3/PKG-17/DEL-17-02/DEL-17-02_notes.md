# DEL-17-02 notes — W3 PKG-17, worker G1

Forward: `DEL-17-02_forward.csv` (160 rows). It has 130 required keys, the
`.rNN` rows of the split blocks CLM-003, CLM-006, CLM-007, CLM-008 and CLM-044,
and one `.s01`. CLM-005, CLM-009, CLM-011 and CLM-012 are assessed directly and
not split. Sealed per `DEL-17-02_SEAL.txt`.
Reverse: `DEL-17-02_reverse.csv` (320 capabilities).

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/`.
- The exporters are cited project-relative: `core/handoff/{native_json,caepipe_mbf,pcf_export,review_geometry,stress_neutral,export_adapter_sdk,caepipe_external}/…`,
  the matching `schemas/*export*.schema.json`, `fixtures/*/invented/*_package.json`
  and `tests/test_*`.
- `validate_semantic_matrix.py`, `validate_lens_register.py` and
  `check_four_documents.sh` exist only at the repository root (CLM-032
  `ROOT_DOC:`).

## Judgment calls

- **Contract-only scope.** DEC-076 confirms DEL-17-02 as "contract owner
  only". The requirement rows are about export packages, so they are judged
  against every PKG-17 exporter's builder, schema and invented fixture (C6(b)).
- **REQ-007 hash basis (PROJECT_BASELINE, OWNER).** Every exporter hashes
  `json.dumps(sort_keys, compact, ensure_ascii)` bytes. The project itself says
  this is not RFC 8785/JCS (`core/analysis_runs/records.py`, stress-neutral
  `canonical_json`). DEC-028 binds the native package to REQ-007. The row is
  IMPLEMENTED_DIFFERENTLY with cause POSSIBLE_DEFECT.
  - *Verifier note:* the MBF package still labels these bytes
    `JCS_compatible_json_payload_hash`. That is a possible mislabel for
    DEL-17-04, which owns the MBF package.
- **Profile field coverage (FG-DEL-17-02-04).** This covers CLM-009 and
  REQ-020/021/022. `entity_coverage` appears in no profile. The target version
  basis uses three field names and is absent from native and stress-neutral.
  `coordinate_policy` is absent from MBF and stress-neutral. The stable-ID
  policy is called `identity_policy` in three exporters. All rows are
  PARTIALLY_IMPLEMENTED · PARTIAL_SLICE.
- **Loss-category token (FG-DEL-17-02-05).** Native JSON uses `TBD`; the
  contract and the other schemas use `tbd`. CLM-012, REQ-054 and CLM-043 are
  IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT.
- **Other partial rows.**
  - REQ-014: no timestamp policy is declared (CP-11).
  - REQ-035: only MBF uses the carrier vocabulary.
  - REQ-040: PCF and stress-neutral manifests have no `source_basis_refs`.
  - REQ-042: there is no manifest field-status indicator.
  - REQ-053: tbd and unsupported entries only warn, and all packages are
    foundation-status (CP-11).
  - CLM-010: ID-map family coverage is narrow.
  - CLM-024/DEL-17-05: the harness package has no manifest or profile object.
- **CONTEXT#anticipated-artifacts.** The four "schemas" exist only as
  contract field tables. Concrete schemas are per target, owned by DEL-17-03..09.
  The row is PARTIALLY_IMPLEMENTED · OWNERSHIP_ELSEWHERE · OWNER, MEDIUM.
- **Stale records.**
  - CLM-033 ("Current validator-path evidence", 2026-07-12): EVIDENCE_OVERTAKEN.
    The four-document files were replaced on 2026-07-14.
  - VER-001 and output OUT-001: CP-09. The parity PASS hash differs from the
    frozen SOW, which had a 2026-07-16 DEC-081 edit.
  - STATUS: CP-05. Last Updated 2026-07-12 is older than the 2026-07-16
    history entry.
  - CLM-003.r04: STALE_REVIEW_OR_EVIDENCE. The SEMANTIC_READY tranche state is
    a lifecycle or review state (F3).
- **DAG pointers (FG-DEL-17-02-06).** CLM-006.r03 and CLM-007.r08 name DAG-005
  or DAG-006 as the active graph (CP-02). The freeze's approved graph is
  DAG-010.
- **The PLAN-EXPORT-INTEROP finding (FG-DEL-17-02-01)** is shared with
  DEL-17-01. It covers CLM-007.r07, REQ-026 and CLM-041, all origin 7ddd4faf9,
  so all STALE_REVIEW_OR_EVIDENCE.

## Canonical departures

None.

## Convention friction

- Several requirement tables have `.rNN` keys, but their items would differ
  only by exporter field. I assessed CLM-009 directly (allowed when the block
  has its own substance) rather than writing 12 `.rNN` rows.

## UNKNOWN rows

None.

## Reverse pass

- COVERS (29): each PKG-17 exporter's builder, schema, fixture and desktop
  panel under the contract. Also the export review manifest, and
  `exportUnitDisclosure.ts`, which DEC-076 names as consuming DEL-17-02's
  export-unit contract. No CLAIMED_BY, because the scope is contract-only.
- Where the ledger cites the path, NOT_MINE reasons are specific (F5). This
  applies to analysis-run records, model schema, dependency validator,
  registers, CONTRACT, IP boundary, and the harness fixtures, schema and panel.
  Target-mapping, handoff and canonical-hash capabilities are NOT_MINE with
  specific reasons.
- **Did later work change my view of anything sealed?** Yes, one point.
  DEL-17-03's history, read after this ledger was sealed, shows that the
  non-JCS label narrowing was applied under DEC-074 R5 T2A E1/PDU-002 (commit
  `101dcb420`). If E1 is read as adopting the sorted-compact basis for
  exports, REQ-007's cause would be SCOPE_REDIRECTED_BY_RULING rather than
  POSSIBLE_DEFECT. The disposition, tier and OWNER need would stay. The E1 text
  is in an excluded July PROPOSED_* file and was not read. TBD-17-01-006
  (FG-DEL-17-02-07) has the same DEC-074 O11/E7 caveat as DEL-17-01.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-17-01/02/03: **PASS, 0 findings.**

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19 Piping selects work through owner-steered work graphs.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No row
states or implies release, approval, compliance or certification. These
dispositions are agent judgments, not owner rulings.
