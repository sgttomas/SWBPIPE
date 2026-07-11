# NOTES — DEL-02-05 Project persistence and round-trip serialization (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen source tree
`main @ 551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`CLAIM_CONCORDANCE_DEL-02-05.csv` (37 rows, 20 columns, RFC-4180 clean, CRLF).

Run-level `NormativeSource` path alias (addendum 12, declared once): `Specification.md`
= `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/Specification.md`. Sibling
kit surfaces (`Datasheet.md`, `Guidance.md`, `Procedure.md`, `_STATUS.md`,
`MEMORY.md`, `_REVIEW.md`) are cited by basename from the same folder.

ClaimID form follows the brief/addendum-12 fixed form `<DEL-ID>-<TYPE>-NNN`
(e.g. `DEL-02-05-REQ-001`), NOT the deliverable's native requirement ID; the
native ID (`REQ-02-05-001` …) is carried in `NormativeSource`. This diverges
from the R0B exemplar (which used native IDs as ClaimID) — the exemplar predates
the adopted addenda; the binding brief governs.

## 1. Histograms (recount from CSV; reproduce exactly)

Disposition histogram (37 rows):
- ALIGNED — 27
- PARTIALLY_IMPLEMENTED — 6
- STALE_SETUP_SPECIFICATION — 4

ClaimType histogram (37 rows):
- REQUIREMENT — 26
- DECLARED_STATE — 6
- EXCLUSION — 3
- REMAINING_WORK — 2
- ACCEPTANCE — 0
- IMPLEMENTED_UNMAPPED — 0

Row-level breakdown:
- 26 REQUIREMENT rows (REQ-001..026): 20 ALIGNED, 6 PARTIALLY_IMPLEMENTED
  (REQ-001, REQ-006, REQ-010, REQ-020, REQ-021, REQ-022).
- 3 EXCLUSION rows (EXC-001..003): all ALIGNED.
- 6 DECLARED_STATE rows (DECL-001..006): 4 STALE_SETUP_SPECIFICATION
  (Specification, Datasheet, Guidance, Procedure), 2 ALIGNED (_STATUS.md,
  MEMORY.md).
- 2 REMAINING_WORK rows (REM-001 FR-001, REM-002 H2): both ALIGNED (residual
  valid/owned/bounded, per exemplar convention for live residual records).

## 2. Self-flagged rows

- **DEL-02-05-DECL-001 (Specification, STALE_SETUP_SPECIFICATION).** Judgment
  call. The Specification is mostly a durable requirements contract; the
  staleness is confined to its scope/out-of-scope TBD/PROPOSAL framing (physical
  container, migration framework, canonicalization library, diagnostic code
  prefixes) which post-dates rulings DEC-019/DEC-028/DEC-033 and the implemented
  JCS canonicalization/schema slice. Confidence MEDIUM: a reviewer could read
  the "remain TBD under SCA-001" as deliverable-scope deferral (still true) rather
  than drift. Requirement substance is dispositioned on the REQ rows, never here.
- **DEL-02-05-DECL-003 (Guidance, STALE_SETUP_SPECIFICATION).** Judgment call.
  Guidance is advisory and its principles remain sound; only the "keep container
  TBD until decision" / "record schema-layout decision before implementation"
  trade-off lines are overtaken (DEC-028, DEC-019, implementation proceeded).
  Consistent with the R0B DEL-07-05 C04 Guidance treatment. Confidence MEDIUM.
- **DEL-02-05-REQ-013 and DEL-02-05-REQ-025 (SECURITY class).** Convention-6
  SECURITY encoding applied: `ValidationEvidence = "NONE_FOUND - sufficiency
  review deferred, owner-gated"`, no VERIFIED_NOT_VALIDATED downgrade. These may
  be W1's first SECURITY-class rows — flagged for the Part C R2 reviewer
  spot-check of the convention-6 SECURITY encoding. Disposition ALIGNED because
  the local-first default is implemented and tested (test_local_first_storage_policy
  14 PASS); the end-to-end private-data sufficiency review is a declared PKG-12
  deferral with no named permitting ruling (ALIGNED, not ACCEPTED_DIVERGENCE, per
  addendum 11).
- **DEL-02-05-REQ-020 (PARTIALLY_IMPLEMENTED).** Judgment call on
  PARTIALLY_IMPLEMENTED vs ALIGNED-with-declared-TBD. The JSON payload +
  input-manifest identification is implemented/tested; the "referenced non-JSON /
  binary manifest" clause is tied to the .opsproj container (FR-001 residual)
  whose implementation is owned outside DEL-02-05. Marked PARTIALLY_IMPLEMENTED
  and carried the FR-001 RecordedRemaining because the requirement clause is not
  fully realized for this deliverable.
- **DEL-02-05-REM-002 ClaimClass=MECHANICS.** The H2 residual's core is a
  numeric/hash-integrity + version-literal correctness edge; per addendum-7
  ("numeric/unit correctness = MECHANICS") I classed the residual MECHANICS
  rather than WORKFLOW. The corresponding requirement rows it touches keep their
  own classes (SCHEMA/WORKFLOW/REPORTING).

## 3. Zero-count census decisions (no ACCEPTANCE, no IMPLEMENTED_UNMAPPED)

- **ACCEPTANCE = 0.** The Specification "Verification" table is a requirement-ID
  -> verification-approach mapping with no independent acceptance IDs or distinct
  pass/fail criteria — it restates the requirements. Per the brief ("verification
  tables that merely restate requirements do NOT get mirrored ACCEPTANCE rows")
  no ACCEPTANCE rows are minted. This contrasts with the R0B DEL-07-05 exemplar,
  whose Specification carried named VER-07-05-NNN acceptance activities (Document
  review, Boundary review, Dependency-register validation, Semantic setup review,
  Future implementation test) — distinct acceptance grain that DEL-02-05 lacks.
- **IMPLEMENTED_UNMAPPED = 0.** Every material surface in DEL-02-05's orbit
  (schemas/project_persistence.schema.yaml SURF-197, core/project_persistence
  SURF-105, projectService SURF-058, hashService SURF-053, src-tauri SURF-005,
  fixtures/persistence SURF-158, canonical_json RUST-23) is already attributed to
  DEL-02-05 in R1 IMPLEMENTATION_SURFACES.csv — mapped, not unmapped. The
  `.opsproj` multi-member container (`core/reporting/report_package`,
  TP-E3-CONTAINER-001 / TP-E5-PACKAGING-001) is DEC-028/DEC-057/DEC-061-owned by
  PKG-08 (DEL-08-02/04/06) and PKG-17 (DEL-17-02): it is referenced by DEL-02-05's
  FR-001 residual but is outside DEL-02-05's ownership boundary, so no
  IMPLEMENTED_UNMAPPED row is minted for it (orientation confirmed against
  report_package/src/lib.rs header and the R1 surfaces index).

## 4. Evidence-execution log

Re-executed side-effect-free at the frozen SHA (guards: `PYTHONDONTWRITEBYTECODE=1`,
`pytest -p no:cacheprovider`, external `TMPDIR`, external `CARGO_TARGET_DIR`;
`git -C FROZEN status --porcelain` empty BEFORE and AFTER every run; HEAD verified
`551f84ef6…`):
- `tests/test_persistence_schema.py` — 2 PASS
- `tests/test_project_persistence_service.py` — 15 PASS
- `tests/test_model_schema.py` — 4 PASS
- `tests/test_physical_to_analytical_transform.py` — 13 PASS
- `tests/security/test_local_first_storage_policy.py` — 14 PASS
  (aggregate pytest batch: 48 passed)
- `core/serialization/canonical_json` — `cargo test` 8 PASS (RUST-23)

Verified by direct read/grep at the frozen SHA (side-effect-free):
- Stale `"0.1.0"` version-check literals present (H2 residual): ExportReviewPanel.tsx
  L269/L1135, ProjectValidationPanel.tsx L255/L580/L582, ReportPanel.tsx L657/L798/L833.
- `evaluateModelDocumentLocal` browser mirror present in projectService.ts (L68)
  distinct from the Rust authority src-tauri/src/model_document_migration.rs.
- `migrate_project_store` (store-schema migrate) present in service.py L360; the
  schema service_operations enum lists `migrate_project`; no explicit
  project-document migrate operation beyond the store level (FR-001 residual).
- All named implementation surfaces exist; DecisionBasis ruling records
  D-01/D-08/D-09/D-06 all present under execution/_Coordination/_DECISIONS/;
  DEC-033 present in SOFTWARE_DECOMP section 12 (and its ruling text explicitly
  names the H2 unification item as unchanged/open — corroborates REM-002).

Cited as recorded (not re-executed at frozen SHA 551f84ef6):
- Desktop suites (Vitest VT-02/14/19, Playwright PW-01) via the DEC-025 sweep
  SWEEP_20260711T040758Z_e648462f1d05.json at commit e648462f1 (ancestor of the
  frozen SHA). R1 VERIFICATION_INDEX records these CONTENT_IDENTICAL over
  apps/desktop/ to the frozen SHA; I did not independently run those path diffs,
  so I cite the R1 index binding rather than asserting the addendum-10
  content-identical string myself.
- `apps/desktop/src-tauri` crate (62 tests, incl. model-document migration): R1
  RUST-37 records LatestRecordedExecution = NONE_FOUND (crate is outside the
  DEC-025 sweep cargo roots `core, validation/benchmarks`); recorded-only.

## 5. Convention friction notes

- **ClaimID form vs exemplar.** Resolved in favour of the brief's explicit
  `<DEL-ID>-<TYPE>-NNN` form; the R0B exemplar's native-ID ClaimIDs predate the
  addenda. Native requirement IDs preserved in NormativeSource.
- **Overtaken-TBD vs still-open-deferral.** Convention 4's widened
  STALE_SETUP_SPECIFICATION ("no longer describes the frozen implemented slice")
  is clear for future-tense artifact prose (Datasheet DECL-002, HIGH) but
  genuinely ambiguous where a doc declares something "TBD" that has since been
  ruled at *program* level yet remains out of the *deliverable's* build scope
  (Specification/Guidance). I read program-level rulings + implemented slice as
  drift of the surface declaration and dispositioned STALE at MEDIUM confidence,
  routing R5 repair to OWNER. Flagged above.
- **RecordedRemaining spread.** The two non-bootstrap residuals each touch
  several requirements. Per addendum "copied verbatim only on rows the residual's
  claim touches," I copied FR-001 verbatim onto REQ-001/020/021/022 (+ REM-001,
  + _STATUS.md DECL-005) and H2 verbatim onto REQ-006/010 (+ REM-002,
  + DECL-005), and cross-referenced (not verbatim) on REQ-005/012 in RemainingWork
  to avoid over-spreading. A reviewer may prefer wider or narrower spread.
- **SourceReliability.** Per addendum 6, all DECLARED_STATE prose rows use
  NOT_APPLICABLE; all technical (REQUIREMENT/EXCLUSION/REMAINING_WORK) rows use
  UNVERIFIED (project-original agent-generated, agent-audited, human disposition
  pending — the deliverable is IN_PROGRESS with no human acceptance of the cited
  evidence). No REVIEWED used: the DEC rulings govern *decisions*, not the cited
  test/code evidence, and `_REVIEW.md` is a mechanical AGENT_CHECK.
- **SelectableUnderCurrentLoop** is mechanical only (convention 6): YES on the
  six requirement rows plus two REMAINING_WORK rows plus the _STATUS.md surface
  row that carry an ungated residual; NO elsewhere. The owner suspension is a
  run-level caveat, not encoded per-row. The `(gated: D-41)` bootstrap item is
  recorded verbatim only in the _STATUS.md surface row (DECL-005) and excluded
  from all residual/gate/selectability analysis (addendum 2).

## 6. Boundary-compliance statement

- Discovery was read-only outside the two output files
  (`CLAIM_CONCORDANCE_DEL-02-05.csv`, `NOTES_DEL-02-05.md`). No lifecycle
  transition applied; no LIFECYCLE_REASSESSMENT_REQUIRED used (none warranted).
  No DAG mutation, no cross-project edit, no edit to any `_STATUS.md`, register,
  or product file.
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim appears anywhere in the outputs (F-PIP-1..5). The
  agent dispositions are agent judgments, never owner/engineering rulings;
  authority is routed via `AuthorityNeeded` (OWNER on the four STALE declared-
  state R5-repair rows; NO elsewhere). No DEFERRED_AGENT_WORKFLOW disposition was
  needed (the D-41 concordance bootstrap item is excluded from analysis per
  addendum 2, not dispositioned).
- Frozen evidence tree verified clean: `git status --porcelain` empty before and
  after every re-execution; HEAD held at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
  Build artifacts redirected outside the frozen tree (external TMPDIR and
  CARGO_TARGET_DIR); no writes into the frozen tree on any path.
- Note: the run's scratchpad is shared across sibling pilots; a unique generator
  filename was used to avoid clobbering another deliverable's scratch script. No
  effect on outputs.
