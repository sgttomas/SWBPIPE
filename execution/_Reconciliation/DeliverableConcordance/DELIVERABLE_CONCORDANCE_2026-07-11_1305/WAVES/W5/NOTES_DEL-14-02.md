# NOTES — DEL-14-02 Analysis run records (R2 W5)

DEL-14-02 / PKG-14, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 28 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 11; ACCEPTANCE 7; EXCLUSION 4; DECLARED_STATE 6;
REMAINING_WORK 0. Disposition: ALIGNED 19; PARTIALLY_IMPLEMENTED 5;
STALE_SETUP_SPECIFICATION 4. AuthorityNeeded: NO 24 / OWNER 4.
Selectability: YES 0 / NO 28. Confidence: HIGH 19 / MEDIUM 9.
SourceReliability: UNVERIFIED 22 / NOT_APPLICABLE 6.

The requirement census preserves DEL-14-02-R001..R011 one-for-one. The seven
acceptance rows preserve the seven distinct `Specification.md ## Verification`
targets. The declared-state census is exactly the four-document kit plus
`_STATUS.md` and `MEMORY.md`; no deliverable-owned README exists. The D-41
bootstrap is quoted verbatim only in DECL-005 `RecordedRemaining` and excluded
from residual and selectability analysis. `NormativeSource` paths are relative
to the frozen DEL-14-02 folder unless a repository path is written explicitly.

## Key judgments and self-flags

- REQ-005/ACC-005 are PARTIALLY_IMPLEMENTED: the builder and persistence
  history preserve one explicit `unit_system_ref`, and the schema has no
  defaults, but the focused tests do not prove that missing/ambiguous unit
  metadata on every unit-bearing referenced result becomes a diagnostic.
- REQ-006 is PARTIALLY_IMPLEMENTED: diagnostic structures, blocking classes,
  upstream-diagnostic propagation, and core-field validation exist, but the
  focused negative test does not cover every solve-required and
  rule-check-required input named by the requirement.
- REQ-007/ACC-004 are PARTIALLY_IMPLEMENTED: payload scope and deterministic
  hashes are present and mutation-sensitive, but `canonical_json` is stable
  sorted compact `json.dumps`, not a demonstrated full RFC-8785/JCS
  implementation despite emitted `canonicalization: JCS` metadata. This is a
  bounded reproducibility-contract gap, not an engineering or external-
  validation ruling.
- DECL-001..004 are STALE_SETUP_SPECIFICATION. Specification, Datasheet,
  Guidance, and Procedure retain setup/future statements that implementation,
  exact schema design, or tests are absent/TBD even though the frozen tree has
  the detailed schema, builder, persistence binding, and focused tests.
  Procedure also calls revision 0.7 the current basis while the accepted
  decomposition is revision 0.8. Surviving technical and boundary guidance is
  not rejected.
- DECL-005 is current and bootstrap-only. DECL-006 is ALIGNED: its revision-
  0.7/DAG-006 entries are dated historical records protected by addendum 1,
  while its implementation head matches the frozen slice.
- SECURITY rows use explicit-reason `NOT_APPLICABLE`, not the convention-6
  sufficiency marker. The rows are bounded schema-content/redaction checks;
  no named owner-gated security-sufficiency deferral exists.
- No REMAINING_WORK row is minted: `Review_Findings.csv` is header-only,
  `_REVIEW.md` records PASS/no findings, and the sole `_STATUS.md` item is the
  excluded bootstrap. No ACCEPTED_DIVERGENCE, UNKNOWN, AUTHORITY_CONFLICT,
  IMPLEMENTED_UNMAPPED, or lifecycle-reassessment row is encoded.

## Implementation and evidence coverage

Primary mapped surfaces inspected: `schemas/analysis_run.schema.json`,
`core/analysis_runs/{__init__.py,records.py}`,
`tests/test_analysis_run_schema.py`, `tests/test_analysis_run_records.py`,
`core/project_persistence`, `apps/desktop/src/services/previewService.ts`, and
the product-preview integration assertions. R1's broader shared-surface
attributions (desktop shell/report/result-export/rule-check/run-audit and
shared result schemas/crates) were treated as consumers or adjacent evidence,
not duplicate authoritative ownership of the DEL-14-02 run-record contract.

Contract-grain ALIGNED rows do not establish public API transport, release-
grade solver provenance, comprehensive commercial-prover ingestion, external
validation, professional reliance, public-export enforcement breadth, or
comparison/handoff completion.

## Verification and addendum-9 containment

Re-executed at the frozen SHA with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1402`, and no cache provider:

- `python3 -m json.tool schemas/analysis_run.schema.json` — PASS.
- `python3 tests/test_analysis_run_schema.py` — PASS.
- `python3 -m pytest -p no:cacheprovider tests/test_analysis_run_schema.py
  tests/test_analysis_run_records.py -q` — **9 passed** (the structural script
  is not a pytest test module; the nine collected tests are the record tests).

No cargo, generator, compilation, or `py_compile` ran. Lockless Cargo copy-out
therefore did not apply. Frozen tracked porcelain was empty before and after.
Ignored-aware porcelain showed exactly the same six allow-listed pre-existing
paths before and after:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The frozen tree was neither cleaned nor modified. Writes were limited to this
CSV and notes in the dedicated D-41 worktree.

## Cross-ledger risks and fences

1. DEL-14-02 shares model-state, status, result, audit, persistence, preview,
   report, and handoff consumers with DEL-14-01/03/04/05, PKG-08, PKG-10, and
   PKG-15. R3 must distinguish shared consumption from duplicate ownership.
2. The JCS-label-versus-implementation finding overlaps PKG-08 audit/hash and
   persistence surfaces; deduplicate the underlying canonicalization species
   rather than counting both REQ-007 and ACC-004 as separate gaps.
3. Unit-reference evidence is not numeric mechanics validation. Do not infer
   unit-conversion correctness or engineering suitability from the ALIGNED
   schema/run-basis rows.
4. The schema's `unresolved_tbd` default still says `physical project
   container` while `RunContractStatus` fixes the SCA-003 container profile;
   carry this as an R3 cross-surface currentness observation, not a new
   deliverable claim or authorization to repair product code.
5. Setup-document staleness is surface-local. It does not invalidate the
   implemented schema/tests, and it authorizes no R5 document rewrite.

No product, status, lifecycle, DAG, register, decision, dependency, review,
R4, or R5 surface changed. No release, certification, sealing, approval,
authentication, professional-reliance, code-compliance, legal-sufficiency, or
security-sufficiency assertion is made. Dispositions are agent judgments,
never human or engineering rulings.
