# NOTES — DEL-15-02 Target mapping and unsupported-behavior contract (R2 W5)

DEL-15-02 / PKG-15, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 31 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 12; ACCEPTANCE 6; EXCLUSION 4; DECLARED_STATE 6;
REMAINING_WORK 3. Disposition: ALIGNED 24; PARTIALLY_IMPLEMENTED 1;
STALE_SETUP_SPECIFICATION 3; REMAINING_STATE_MISMATCH 3. AuthorityNeeded:
NO 23 / OWNER 8. Selectability: YES 0 / NO 31. Confidence: HIGH 22 /
MEDIUM 9. SourceReliability: UNVERIFIED 25 / NOT_APPLICABLE 6.

The requirement census preserves DEL-15-02-R001..R012 one-for-one. Six
acceptance rows preserve V-001..V-006. The declared-state census is exactly
the four-document kit plus `_STATUS.md` and `MEMORY.md`; the owned module has
no README. The D-41 bootstrap is quoted only in DECL-005 `RecordedRemaining`
and excluded from residual/selectability analysis. `NormativeSource` paths are
relative to the frozen DEL-15-02 folder unless a repository path is explicit.

## Key judgments and self-flags

- The provider-neutral contract is implemented at schema/builder grain:
  mapping metadata, unsupported/approximate flags, assumption/warning refs,
  unit diagnostics, provenance, no-silent-default diagnostics, and negative
  professional-boundary behavior are directly exercised.
- REQ-011 is PARTIALLY_IMPLEMENTED. `PrivacyContext` records redaction and
  payload-boundary metadata and the strict schema excludes arbitrary payload
  fields, but the builder copies caller privacy context without enforcing
  redaction or diagnosing unsafe embedded-payload booleans. No runtime privacy
  or legal-sufficiency claim is inferred.
- REQ-009/ACC-002 remain ALIGNED on independent current schema/code/test
  grounds, but their load-bearing package-audit finding is
  `TECHNICALLY_ADDRESSED_PENDING_HUMAN`; addendum 13 therefore caps them at
  MEDIUM and routes OWNER.
- DECL-002..004 are STALE_SETUP_SPECIFICATION: Datasheet calls revision 0.7
  current, Guidance retains a live CT-001 conflict around approved DAG-006
  rows, and Procedure operationally treats DAG-006 mirror rows as current.
  Current authority is revision 0.8 / DAG-007. Specification is ALIGNED because
  its legacy row IDs are used as explicitly bounded local evidence rather than
  asserted aggregate authority; MEMORY's revision-0.7 entry is dated history.
- REM-001..003 preserve the three evidence-backed but unhomed human
  dispositions: DEL-15-02-PKG02-001 is technically addressed/pending human;
  RF-001 and RF-002 are technically addressed yet remain OPEN/TBD. Gate-5
  lifecycle approval advanced the deliverable at that time but explicitly did
  not disposition these findings. They are mismatch rows, not technical gaps.
- SECURITY rows use explicit-reason `NOT_APPLICABLE`; no convention-6
  owner-gated sufficiency marker is manufactured. No ACCEPTED_DIVERGENCE,
  UNKNOWN, AUTHORITY_CONFLICT, IMPLEMENTED_UNMAPPED, or lifecycle row exists.

## Implementation and evidence coverage

Primary owned/mapped surfaces inspected:
`schemas/target_mapping.schema.json`,
`core/handoff/target_mapping/{__init__.py,contract.py}`,
`tests/test_target_mapping_contract.py`, the canonical handoff package context,
and the shared external-prover authority-boundary helper. DEL-15-01/03 and
DEL-17-01/02 are upstream/downstream owners or target-specific consumers, not
duplicate authoritative owners of the provider-neutral DEL-15-02 contract.

ALIGNED contract rows do not establish target-specific field coverage,
commercial parser behavior, target equivalence, external solver correctness,
runtime redaction enforcement, professional reliance, or release readiness.

## Verification and addendum-9 containment

Re-executed at the frozen SHA with `PYTHONDONTWRITEBYTECODE=1` and external
`PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1502`:

- `python3 tests/test_target_mapping_contract.py` — PASS.
- `python3 tools/validation/validate_dependencies_schema.py <absolute
  DEL-15-02 Dependencies.csv>` — PASS, 29 columns / 18 rows.

An initial dependency-validator invocation used a duplicated project-relative
prefix after changing directory and returned file-not-found; the corrected
absolute-path invocation passed. No source or evidence result depends on the
failed path invocation.

No pytest cache, cargo, generator, compilation, or `py_compile` ran. Lockless
Cargo copy-out therefore did not apply. Frozen tracked porcelain was empty
before and after. Ignored-aware porcelain showed exactly the same six
allow-listed paths:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The frozen tree was neither cleaned nor modified. Writes were limited to this
CSV and notes in the dedicated D-41 worktree.

## Cross-ledger risks and fences

1. DEL-15-02 shares the target-mapping schema with DEL-15-01/03 and feeds
   DEL-17-01/02. R3 must preserve provider-neutral ownership and treat package
   assembly/target adapters as consumers.
2. Legacy DAG-002 row IDs and DAG-006 mirror language are derivative evidence,
   not current DAG authority. Deduplicate pointer drift without authorizing a
   dependency rewrite.
3. The three pending finding dispositions describe one technically addressed
   authority-label issue and two remediated document issues. Do not count them
   as three implementation defects or infer closure from lifecycle approval.
4. Privacy metadata shape is not runtime redaction enforcement; deduplicate
   this gap with PKG-12 redaction/export-control findings.
5. Generic unsupported/approximate flags do not prove target equivalence or
   target-specific coverage.

No product, status, lifecycle, DAG, register, decision, dependency, review,
R4, or R5 surface changed. No release, certification, sealing, approval,
authentication, professional-reliance, code-compliance, legal-sufficiency, or
security-sufficiency assertion is made. Dispositions are agent judgments,
never human or engineering rulings.
