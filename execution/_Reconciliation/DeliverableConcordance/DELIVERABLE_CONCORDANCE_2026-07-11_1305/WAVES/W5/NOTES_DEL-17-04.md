# NOTES — DEL-17-04 CAEPIPE MBF export profile and deterministic writer (R2 W5)

Frozen evidence: `main` @
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Deliverable:
PKG-17 / DEL-17-04, lifecycle `IN_PROGRESS`. Role/model: **GPT-5
deliverable-grained owning pilot**.

Unqualified deliverable documents refer to
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/`
at the frozen SHA; other paths are relative to `projects/chirality-piping/`.

## Census and histograms

The exact adopted 20-column ledger contains **39 rows**.

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 11 |
| ACCEPTANCE | 16 |
| EXCLUSION | 1 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 5 |
| **Total** | **39** |

All 39 dispositions are **ALIGNED** and all confidence values are HIGH.
AuthorityNeeded is NO 34 / OWNER 5. SourceReliability is UNVERIFIED 33 /
NOT_APPLICABLE 6. All 39 rows are mechanically non-selectable.

The 16 ACCEPTANCE rows preserve six Verification Requirements plus ten
Acceptance Criteria rather than merging their distinct normative claims. The
declaration census is the four-document kit plus `_STATUS.md` and `MEMORY.md`;
no deliverable-owned README exists.

Five REMAINING_WORK rows preserve Guidance TBD-001..004 plus open formal
review row RF-002. RF-001 is resolved and is used as authority-currentness
evidence, not counted as remaining. TBD-004 records an implemented foundation
classification with later specialization still open. RF-002 overlaps the
three source/profile TBDs and wider excluded/downstream work; that overlap is
explicitly flagged for R3 deduplication.

## Self-flags for package fan-in

- **ACC-008/009 and REM-001/002 — unresolved acceptance wording.** ALIGNED
  means target-version and record-subset uncertainty is correctly preserved;
  it does not mean either target decision is closed.
- **REM-004 — selected foundation policy versus open specialization.** The
  current warning/blocking policy is implemented and tested. The Guidance TBD
  remains as a later entity-specific source-confirmed profile question.
- **REM-005 — overlap row.** RF-002 is an open informational aggregate, not a
  fifth independent product defect. R3 should deduplicate its version/subset/
  stable-ID portions against REM-001..003 and route external run/parser to
  DEL-17-05, runtime/API/GUI to their owners, and lifecycle to human authority.
- **DECL-001/002 — later extension grain.** The four-document kit accurately
  defines the core Python package contract but does not exhaust June desktop
  unit-disclosure/conversion-witness extensions; MEMORY records those dated
  additions. No stale core declaration was found.
- **ACC-001/007 and source-basis controls — external-source grain.** D-41
  verified frozen refs and guards, not live vendor-page content or CAEPIPE
  compatibility.
- The sole `_STATUS.md` remaining item is the byte-exact D-41 bootstrap. It is
  present only on DECL-005 and excluded from residual, gate, and selectability
  analysis.

## Verification and addendum-9 containment

Re-executed in the frozen worktree with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/d41-del1704-pycache`, and pytest cache disabled:

- `python3 -m pytest -q -p no:cacheprovider projects/chirality-piping/tests/test_caepipe_mbf_export_package.py`
  — PASS, 17 tests.
- `projects/chirality-piping/tools/validation/validate_dependencies_schema.py <DEL-17-04>/Dependencies.csv`
  — VALID, 29 columns, 17 rows.
- root `tools/validation/check_four_documents.sh <DEL-17-04>` — PASS.
- root `tools/validation/check_min_viable_fileset.sh <DEL-17-04>` — PASS.
- root semantic-matrix and lens-register validators — VALID.

The current dependency row count is 17; the June 4 formal review's four-row
count is historical pre-refresh evidence. No Cargo command or in-tree
`py_compile` ran. Ignored-aware porcelain before and after contained exactly
the six known allow-listed paths and no seventh path. Tracked porcelain
remained empty; the frozen worktree was not modified or cleaned.

## Cross-ledger and R3 risks

1. **Source authority versus writer ownership:** DEL-17-01 owns admitted
   CAEPIPE source/TBD authority; DEL-17-02 owns common export contracts;
   DEL-17-04 owns the bounded MBF profile/package builder.
2. **Target-support posture:** deterministic invented MBF text, hashes,
   sidecars, manifests, loss reports, and tests do not establish CAEPIPE
   import success, version coverage, compatibility, or solver validation.
3. **TBD duplication:** DEL-17-04 TBD-001..003 restate DEL-17-01 TBD-001..003
   at target-profile grain. Preserve lineage without multiplying defects.
4. **Python versus desktop grain:** the Python contract and desktop MBF panel
   share ownership, but desktop conversion witnesses do not close Python
   target-version/subset/direct-ID TBDs.
5. **Unit evidence:** disclosed units and conversion witnesses are metadata
   and deterministic conversion evidence, not engineering validation of the
   exported model or target interpretation.
6. **Review/lifecycle drift:** `_REVIEW.md` retains its then-current CHECKING
   wording while frozen `_STATUS.md` is IN_PROGRESS under the later lifecycle
   correction. The review is evidence, not current lifecycle authority.
7. **Dependency state:** pending/TBD dependency satisfaction cells authorize
   no dependency or DAG changes.

## Fences

All dispositions are agent judgments, never human, owner, vendor,
engineering, professional, security, legal, validation, compatibility, or
release rulings. Writes were limited to this CSV and notes. No product,
deliverable, lifecycle, status, dependency, DAG, register, review, decision,
R4, or R5 surface changed. No approval, certification, sealing,
authentication, code-compliance, professional reliance, legal clearance,
security assurance, target compatibility, solver validation, or external
validation claim is made.
