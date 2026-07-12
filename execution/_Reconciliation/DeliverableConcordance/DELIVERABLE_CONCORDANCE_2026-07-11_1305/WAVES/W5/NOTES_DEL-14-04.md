# NOTES — DEL-14-04 Analysis-run comparison engine (R2 W5)

DEL-14-04 / PKG-14, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 24 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 9; ACCEPTANCE 6; EXCLUSION 3; DECLARED_STATE 6;
REMAINING_WORK 0. Disposition: ALIGNED 14; PARTIALLY_IMPLEMENTED 4;
VERIFIED_NOT_VALIDATED 2; STALE_SETUP_SPECIFICATION 4. AuthorityNeeded:
NO 18 / OWNER 4 / ENGINEERING 2. Selectability: YES 0 / NO 24.
Confidence: HIGH 16 / MEDIUM 8. SourceReliability: UNVERIFIED 18 /
NOT_APPLICABLE 6.

The requirement census preserves R-14-04-001..009 one-for-one. The six
acceptance rows preserve the six distinct `Specification.md ## Verification`
items. The declaration census is exactly the four-document kit plus
`_STATUS.md` and `MEMORY.md`; the implementation module has no owned README.
The D-41 bootstrap is quoted verbatim only in DECL-005 `RecordedRemaining` and
excluded from residual and selectability analysis. `NormativeSource` paths are
relative to the frozen DEL-14-04 folder unless a repository path is explicit.

## Key judgments and self-flags

- REQ-001/ACC-002 are PARTIALLY_IMPLEMENTED: the engine deterministically
  sorts and consumes caller-supplied mapping records, recognizes comparable
  `automatic_match`/`manual_match` status tokens, and tests manual/unresolved
  cases, but it does not derive automatic stable-ID matches. The missing path
  is absent implementation, not merely absent test coverage.
- REQ-003/ACC-003 are VERIFIED_NOT_VALIDATED. Explicit unit/dimension guards,
  caller-supplied conversions, negative diagnostics, and the DEC-026 mixed
  kPa/Pa plus lbf/N corpus all pass. Those are agent-generated verification
  fixtures, not an independent engineering validation/suitability basis.
- REQ-004 is PARTIALLY_IMPLEMENTED: generic result families, `object_refs`,
  settings, and carried diagnostics exist, but focused evidence does not cover
  every named node/element/support/terminal/stress-location category.
- REQ-007 is PARTIALLY_IMPLEMENTED: structured run/result/mapping/tolerance
  input and a deterministic comparison dictionary exist, but no canonical
  comparison-result/export schema validation was found. DEL-14-05's contract
  schemas explicitly say their comparison/result-delta engines are outside
  those schema contracts.
- DECL-001..004 are STALE_SETUP_SPECIFICATION. Specification, Datasheet,
  Guidance, and Procedure still call implemented engine/test/fixture/contract
  surfaces future or TBD; the first, second, and fourth also carry current
  revision-0.7/DAG-006 framing against revision 0.8. Genuine open default,
  canonical export-shape, and broader category-coverage questions remain.
- DECL-005 is current and bootstrap-only. DECL-006 is ALIGNED; its revision-
  0.7/DAG-006 entry is dated historical context, while its implementation,
  evidence-hardening, DEC-026, and desktop-unit entries match the frozen tree.
- No REMAINING_WORK row is minted: `Review_Findings.csv` is header-only,
  `_REVIEW.md` records PASS/no findings, and the only status item is the
  bootstrap. No ACCEPTED_DIVERGENCE, UNKNOWN, AUTHORITY_CONFLICT,
  IMPLEMENTED_UNMAPPED, SECURITY marker, or lifecycle-reassessment row exists.

## Implementation and evidence coverage

Primary owned/mapped surfaces inspected:
`core/comparison/analysis_run/engine.py`,
`tests/test_analysis_run_comparison.py`,
`tests/test_comparison_contracts.py`, DEL-14-05 mapping/tolerance schemas,
DEL-14-02 run-record contracts, `apps/desktop/src/services/previewService.ts`,
and `apps/desktop/src/features/comparison`. Shared preview/desktop/report
surfaces were treated as consumers or adjacent evidence, not duplicate
authoritative ownership of the comparison engine.

Contract-grain ALIGNED rows do not establish default tolerance suitability,
engineering validation, complete category coverage, comprehensive external-
prover ingestion, external validation, professional reliance, release
readiness, or canonical export completion.

## Verification and addendum-9 containment

Re-executed at the frozen SHA with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1404`, and cache suppression:

- `python3 -m pytest -p no:cacheprovider tests/test_analysis_run_schema.py
  tests/test_analysis_run_records.py tests/test_comparison_contracts.py
  tests/test_analysis_run_comparison.py -q` — **19 passed**.

No cargo, generator, compilation, or `py_compile` ran. Lockless Cargo copy-out
therefore did not apply. Frozen tracked porcelain was empty before and after.
Ignored-aware porcelain showed exactly the same six allow-listed paths:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The frozen tree was neither cleaned nor modified. Writes were limited to this
CSV and notes in the dedicated D-41 worktree.

## Cross-ledger risks and fences

1. DEL-14-04 consumes DEL-14-02 run records and DEL-14-05 mapping/tolerance/
   export contracts. R3 must distinguish shared consumption from duplicate
   ownership and deduplicate repeated unit/tolerance findings.
2. The engine accepts caller-supplied conversion factors and profiles; this is
   not a source of unit catalog authority or tolerance suitability. DEC-018/
   DEC-026 and DEL-14-05 remain the governing boundaries.
3. The missing canonical comparison-output schema overlaps reporting/export
   consumers in PKG-08 and PKG-15; aggregate the interface species once.
4. The desktop comparison packet records equal-unit/no-conversion behavior,
   while the core engine supports caller-supplied mixed-unit conversion. These
   are different grains, not a contradiction; avoid flattening them in R3.
5. Setup-document staleness is surface-local and authorizes no R5 rewrite.

No product, status, lifecycle, DAG, register, decision, dependency, review,
R4, or R5 surface changed. No release, certification, sealing, approval,
authentication, professional-reliance, code-compliance, legal-sufficiency, or
security-sufficiency assertion is made. Dispositions are agent judgments,
never human or engineering rulings.
