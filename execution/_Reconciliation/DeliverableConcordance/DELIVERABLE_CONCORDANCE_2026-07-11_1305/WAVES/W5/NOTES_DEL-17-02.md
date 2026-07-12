# NOTES — DEL-17-02 Export package, profile, and stable ID map contracts (R2 W5)

DEL-17-02 / PKG-17, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 63 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 44; ACCEPTANCE 6; EXCLUSION 6; DECLARED_STATE 6;
REMAINING_WORK 1. Disposition: ALIGNED 59; STALE_SETUP_SPECIFICATION 3;
STALE_REVIEW_OR_EVIDENCE 1. AuthorityNeeded: NO 59 / OWNER 4. Selectability:
YES 0 / NO 63. Confidence: HIGH 57 / MEDIUM 6. SourceReliability:
UNVERIFIED 57 / NOT_APPLICABLE 6.

The requirement census preserves all 44 non-contiguous source IDs one-for-one
through contiguous run-local REQ-001..044 IDs; each `NormativeSource` records
the original DEL-17-02 requirement number. Six acceptance rows preserve the
six bullet criteria. Six exclusion rows retain distinct implementation,
runtime/UI/API, adapter/writer, commercial/license, protected/private, and
professional/product-claim boundaries. The declaration census is exactly the
four-document kit plus `_STATUS.md` and `MEMORY.md`; no owned README exists.
The D-41 bootstrap appears only in DECL-005 and is excluded from residual and
selectability analysis.

## Key judgments and self-flags

- All 44 requirements are ALIGNED at **contract-definition grain**. DEL-17-02
  expressly owns the human-readable common contract, not concrete schemas,
  exporters, parsers, or target writers. Multiple separately owned downstream
  packages now cite/consume its package/profile/manifest/ID-map/loss-report
  rules; that is corroboration, not duplicate DEL-17-02 implementation.
- Contract-grain ALIGNED does not prove downstream JCS fidelity, target
  compatibility, target-field coverage, solver validity, parser completeness,
  release readiness, or professional reliance. Each downstream owner retains
  its own implementation/evidence judgment.
- DECL-002 is STALE on lifecycle and authority pointers (SEMANTIC_READY and
  rev-0.7/DAG-005/006 versus current IN_PROGRESS and rev-0.8/DAG-007).
  DECL-003 currently instructs use of DAG-006 as a project source reference.
  DECL-004 begins by treating DAG-006 as active authority. These are
  STALE_SETUP_SPECIFICATION surface findings; their substantive export rules
  remain applicable.
- RF-001 is STALE_REVIEW_OR_EVIDENCE, not merely an unhomed open finding. The
  Procedure explicitly says to run from repository root, all five named
  `tools/validation` paths exist there, and all five passed in this pilot.
  The finding remains OPEN/HumanDisposition=TBD and absent from status, so its
  formal disposition routes OWNER; this pilot does not close it.
- Specification's tranche-time statement that downstream deliverables were not
  populated is historical scoped acceptance evidence, not a current absence
  declaration. Later downstream implementation does not make the DEL-17-02
  ownership exclusion false.
- No ACCEPTED_DIVERGENCE, PARTIALLY_IMPLEMENTED, VERIFIED_NOT_VALIDATED,
  REMAINING_STATE_MISMATCH, UNKNOWN, AUTHORITY_CONFLICT,
  IMPLEMENTED_UNMAPPED, SECURITY marker, or lifecycle row is encoded.

## Implementation and evidence coverage

Primary owned surfaces inspected are the four documents, semantic/lens
artifacts, dependency register, review finding, status, memory, and run
records. Downstream corroboration sampled:
`schemas/{native_json_export,caepipe_mbf_export,stress_neutral_export,
pcf_export,review_geometry_export,export_adapter_sdk}.schema.json` and their
corresponding `core/handoff/**` package/test surfaces. R1 maps only shared
`schemas/model.schema.yaml` to DEL-17-02; it remains upstream canonical model
vocabulary, not an owned export-contract implementation.

## Verification and addendum-9 containment

Executed from frozen repository root with `PYTHONDONTWRITEBYTECODE=1` and
external `PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1702`:

- `tools/validation/check_four_documents.sh <DEL-17-02>` — PASS.
- `tools/validation/check_min_viable_fileset.sh <DEL-17-02>` — PASS.
- `python3 tools/validation/validate_dependencies_schema.py <Dependencies>` —
  PASS, 29 columns / 22 rows.
- `python3 tools/validation/validate_semantic_matrix.py <DEL-17-02>` — PASS.
- `python3 tools/validation/validate_lens_register.py <DEL-17-02>` — PASS.

No pytest, cargo, generator, compilation, or `py_compile` ran. Lockless Cargo
copy-out therefore did not apply. Frozen tracked porcelain was empty before
and after. Ignored-aware porcelain showed exactly the same six allow-listed
paths:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The frozen tree was neither cleaned nor modified. Writes were limited to this
CSV and notes in the dedicated D-41 worktree.

## Cross-ledger risks and fences

1. DEL-17-02 is the common contract owner; DEL-17-03..09 implement target
   packages. R3 must distinguish normative consumption from duplicate code or
   schema ownership.
2. Loss-report/category/ID-map requirements repeat across every downstream
   package. Aggregate the common contract species once and retain target-local
   evidence separately.
3. Stable sorted JSON or manifest hash metadata is not automatically proof of
   full JCS/RFC-8785 fidelity; join the PKG-08/14/15 canonicalization risk.
4. DAG-005/006 and migration-era evidence are historical derivative records,
   not current graph authority. No dependency/register rewrite is authorized.
5. The stale RF-001 premise and stale Procedure DAG pointer are distinct:
   owner review may disposition the finding, while document refresh remains
   an unauthorized R5 candidate.

No product, status, lifecycle, DAG, register, decision, dependency, review,
R4, or R5 surface changed. No release, compatibility, certification, sealing,
approval, authentication, professional-reliance, code-compliance, legal-
sufficiency, target-support, or security-sufficiency assertion is made.
Dispositions are agent judgments, never human or engineering rulings.
