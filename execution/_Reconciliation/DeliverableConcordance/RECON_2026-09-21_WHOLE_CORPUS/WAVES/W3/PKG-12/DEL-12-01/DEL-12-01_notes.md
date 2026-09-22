# DEL-12-01 — W3 notes (worker G1)

Forward SHA-256 in `DEL-12-01_SEAL.txt`. 138 rows: 93 required keys, 36 optional `.rNN` rows (CLM-003, CLM-004, CLM-020, CLM-024, CLM-031 split) and 9 `.sNN` sub-claims.

## Path aliases

- `D` = `projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths/`.
- SOW short paths (`core/security/local_first_storage/`, `tests/security/...`) resolve under `projects/chirality-piping/`; `docs/security/local_first_storage_policy.md` is cited in project form.
- "Project store" = `apps/desktop/src-tauri/src/lib.rs` (`app_store_path`, `open_project_store`, `store_migrations`), which is DEL-02-05 behaviour.
- `validate_enum.py` exists only at the repository root (ROOT_DOC).

## Judgment calls

- **OS root selected in code without a ruling (FG-DEL-12-01-01, CP-10, OWNER).** The store resolves under the Tauri app-local data directory; docs/SPEC.md §4.4 still lists OS roots and application data directories as TBD and no ruling was found. Seven rows carry it (CLM-003.r06, CLM-004.r03, LFSP-REQ-005, CLM-013.s02, CLM-024.r03, CLM-031.r01, LFSP-OI-002). **Owner item for R4.**
- **Container and migration TBDs overtaken by DEC-028 / DEC-019 (FG-02)**: SCOPE_REDIRECTED_BY_RULING, LOCAL_DESIGN catch-up.
- **Runtime storage declared deferred though it exists (FG-03)**: the SQLite store has existed since the initial migration (migration ledger since 2026-06-10) with round-trip and migration tests; the 2026-08-21 tranche bound report/export routes. DOC_BEHIND_CODE. STATUS R01 is REMAINING_STATE_MISMATCH at MEDIUM: the 2026-08-21 record still calls the LFSP-REQ-011 families open, so records and code disagree about whether DEL-02-05 store tests count.
- **Path-class vocabulary (FG-04)**: the SOW's six classes differ from the product policy and guard (eleven classes, only USER_PRIVATE_LIBRARY_ROOT shared). IMPLEMENTED_DIFFERENTLY · DOC_BEHIND_CODE on CLM-005, CLM-019, CLM-027, CLM-032.
- **Deliverable-scoped negatives** ("this deliverable implements no runtime storage, chooses no roots", CLM-003.r08, CLM-009) are read as statements about DEL-12-01's own artifacts, which hold; the product behaviour is recorded on the FG rows.
- **D-24/DEC-051** (owner-configured model-provider transmission) is treated as the governance approval that the cloud-exception clauses anticipate, not as cloud storage.
- **INVARIANT partials**: LFSP-REQ-008 (provenance/redistribution across store and adapter boundaries not shown end to end) and LFSP-REQ-010 (plugin no-bypass holds only by absence, CP-11).

## Canonical departures

- CLM-007 and CLM-017 (INIT.md pointer): CP-02 with the F3 setup-origin class, CANONICAL_DEPARTURE written.
- No departures from CS assignments.

## Convention friction

- CP-02 lists STALE_REVIEW_OR_EVIDENCE while F3 sends setup-era pointer text to STALE_SETUP_SPECIFICATION; the F3 exception names only pins, review states and metadata. I applied F3 to file pointers.
- Surface rows: F1 makes a whole-file claim non-aligned whenever any block diverges; I scoped `STATUS` and `CONTEXT` surface claims to the file header and said so in ClaimSummary.

## UNKNOWN rows

None.

## Reverse pass

Claimed: RC-12-0182 (guard), RC-12-0318 (route admission), RC-12-0078 (policy). Partial: RC-12-0265 (TypeScript local-first mirror in the redaction module), RC-12-0062 (report-package save evidence check). Covers: store capabilities (0004, 0165, 0243, 0347), 0071, 0192, 0097, 0188. The reverse pass did not change my view of any sealed row; it confirmed that the store capabilities belong to persistence work, consistent with FG-01/FG-03 treating them as another deliverable's behaviour. Every NOT_MINE whose EntryPoints overlap a cited path names the path's role in this ledger (F5).

## Batch consistency

`validate_ledger_v2.py --batch` over the three PKG-12 G1 ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining`.

## Process disclosure

Before the manager told me the write boundary for scratch, I kept builder scripts and two SOW text dumps in the session scratchpad (shared). One of my files there (`_scratch_lib.py`) was overwritten by another worker's same-named file; I left their version untouched and moved my own scratch into my deliverable folders, then deleted it. No file I do not own was modified by me. Forward ledgers were built by script from my own judgments; the R0 pilot rows were read (DEL-12-03 only) but not copied.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). These dispositions are agent judgments, not owner rulings.
