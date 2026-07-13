# NOTES — DEL-17-05 CAEPIPE external run harness and CSV parser (R2 W5)

DEL-17-05 / PKG-17, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 37 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 16; ACCEPTANCE 8; EXCLUSION 5; DECLARED_STATE 6;
REMAINING_WORK 2. Disposition: ALIGNED 26; PARTIALLY_IMPLEMENTED 2;
VERIFIED_NOT_VALIDATED 1; STALE_SETUP_SPECIFICATION 6;
REMAINING_STATE_MISMATCH 2. AuthorityNeeded: NO 26 / OWNER 9 /
ENGINEERING 2. Selectability: YES 0 / NO 37. Confidence: HIGH 32 / MEDIUM 5.
SourceReliability: UNVERIFIED 31 / NOT_APPLICABLE 6.

The requirement census preserves all 16 Specification requirements one-for-one.
The acceptance census preserves VER-001..008 one-for-one. Five exclusion rows
separate the setup-era code/schema/parser/fixture and GUI/API exclusions from the
still-active executable/license, release/compatibility, and professional/formal-
validation boundaries. The declaration census is exactly the four-document kit
plus `_STATUS.md` and `MEMORY.md`; no owned README exists. Both open review
findings are represented as remaining-work rows. The D-41 bootstrap appears only
as excluded declaration context in DECL-005; its `RemainingSource` and gate
metadata are `NONE_RECORDED` and it is excluded from residual/selectability
analysis.

## Key judgments and self-flags

- REQ-014 is PARTIALLY_IMPLEMENTED. The package can accept explicit privacy
  metadata and blocks protected payloads, but its default privacy value is
  `invented_public_example`; the generic parser/package path does not prove that
  a user-provided CAEPIPE CSV defaults to private/user-controlled handling.
- REQ-015 is PARTIALLY_IMPLEMENTED. Unit values are preserved, desktop evidence
  carries one no-conversion witness per parsed row, and unknown/unmapped parser
  states are diagnosed. Explicit coordinate-uncertainty and loss-report-
  uncertainty preservation is not represented or tested.
- ACC-006 is VERIFIED_NOT_VALIDATED. The record shape and acknowledgement
  guardrails support an opt-in external-run record, but no user-owned CAEPIPE
  executable was invoked and the first invocation profile remains TBD. This is
  recorded as `NONE_FOUND` validation evidence and is not compatibility,
  solver-validation, or target-support evidence.
- EXC-001/002 and DECL-001..004 are STALE_SETUP_SPECIFICATION. The active
  documents still describe a Phase A four-document-only tranche and future
  code/schema/parser/fixture/GUI work, while the frozen tree contains the Python
  package, JSON Schema, invented fixtures, focused tests, and desktop parser-only
  preview. Their conservative commercial, target, IP, and professional boundaries
  remain applicable; dated Phase A history is not rewritten.
- REM-001 and REM-002 are REMAINING_STATE_MISMATCH. RF-001 and RF-002 remain
  OPEN and evidence-backed, while `_STATUS.md` homes only the D-41 bootstrap.
  As unhomed review findings, both retain their review source but use
  `RecordedRemaining=NONE_RECORDED` and `GateOrStageConstraint=NONE_RECORDED`.
  REM-002 also records `NONE_FOUND` target/live validation evidence. The pilot
  does not close, waive, or convert either finding into a ruling.
- REQ-006 is ALIGNED at package-reference grain: a DEL-17-04 MBF export-package
  reference is the manifest/ID-map/loss-report aggregation seam. Confidence is
  MEDIUM because no live user-owned run exercises the linkage.
- REQ-011 is ALIGNED across adjacent owned surfaces: the Python package binds
  CSV/MBF/run identity and the desktop packet adds model-state, analysis-run,
  result, and source-model references. This does not assert that the desktop
  preview is runtime-validated against the Python JSON Schema.
- No ACCEPTED_DIVERGENCE, UNKNOWN, AUTHORITY_CONFLICT, IMPLEMENTED_UNMAPPED,
  SECURITY marker, lifecycle row, release ruling, or professional/engineering
  approval is encoded.

## Implementation and evidence coverage

Primary mapped surfaces inspected:

- `core/handoff/caepipe_external/{__init__.py,run.py}`;
- `schemas/caepipe_external_run.schema.json`;
- `fixtures/caepipe_external/invented/{caepipe_results.csv,
  parser_only_run_package.json,skipped_run_package.json}`;
- `tests/test_caepipe_external_run_package.py`;
- `apps/desktop/src/features/caepipe-external/
  CaepipeExternalHarnessPanel.tsx` and its focused `App.test.tsx` assertions;
- the complete deliverable-local truth set, dependency/semantic surfaces,
  review findings, and durable run records.

The Python slice builds deterministic skipped/parser-only/external-attempt
evidence records, parses two invented fixture-confirmed sections, records
canonical versus weak/unmapped correlation, emits boundary diagnostics, and
writes metadata/parser/diagnostic/checksum sidecars. The desktop slice renders a
parser-only download preview and adds DEC-018 unit disclosure/witness evidence.
Neither surface discovers or invokes CAEPIPE by default.

## Verification and addendum-9 containment

Executed from the frozen repository with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1705`, and pytest cache disabled:

- `python3 -m pytest -p no:cacheprovider -q
  tests/test_caepipe_external_run_package.py` — PASS, 8 passed in 0.18s.
- `python3 tools/validation/validate_dependencies_schema.py <Dependencies>` —
  PASS, 29 columns / 19 rows.
- `tools/validation/check_four_documents.sh <DEL-17-05>` — PASS.
- `tools/validation/check_min_viable_fileset.sh <DEL-17-05>` — PASS.
- semantic-matrix and lens-register validators — PASS.

The first four-document invocation was attempted from the project subdirectory
using a repository-root-relative tool path and returned `no such file or
directory`; rerunning the documented validator from the frozen repository root
passed. This was an invocation-location correction, not a product or evidence
failure.

No cargo, generator, compilation, or `py_compile` ran. Lockless Cargo copy-out
therefore did not apply. Frozen tracked porcelain was empty before execution.
Ignored-aware porcelain showed exactly the six allow-listed paths:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

Final containment rechecks confirmed the same tracked-clean and ignored-aware
state. The frozen tree was neither cleaned nor modified. Writes were limited to
this CSV and notes in the dedicated D-41 worktree.

## Cross-ledger risks and R3 fences

1. DEL-17-04 owns the MBF package/writer contract; DEL-17-05 consumes its package
   reference and must not be counted as a second MBF writer or compatibility
   proof.
2. DEL-17-02 owns common package/profile/ID-map/loss-report vocabulary. R3 should
   aggregate that contract once while retaining DEL-17-05 parser correlation and
   run-record evidence as target-local behavior.
3. Python schema-backed records and the desktop parser-only preview are adjacent
   owned surfaces, not proof of one schema-validated runtime route; preserve the
   desktop `shape_aligned_not_runtime_json_schema_validated` fence.
4. User-provided CSV privacy defaults and coordinate/loss-report uncertainty are
   concrete implementation gaps. Do not generalize invented-fixture privacy or
   unit witnesses into broad CAEPIPE-output handling sufficiency.
5. Live invocation profile, first supported target/version/MBF profile, source-
   confirmed CSV coverage, and target interpretation remain TBD. A future live
   run would remain regression/handoff evidence only.
6. The active Phase A wording is current-document staleness, while dated run
   records and MEMORY entries are protected historical evidence. Any refresh is
   an R5/owner action, not R2 correction.
7. DAG-005/006 references in dated evidence are provenance; DAG-007 is current
   graph authority. No dependency/register rewrite is authorized.

No product, status, lifecycle, DAG, register, decision, dependency, review, R4,
or R5 surface changed. No release, compatibility, certification, sealing,
approval, authentication, professional-reliance, code-compliance, legal-
sufficiency, target-support, solver-validation, or security-sufficiency assertion
is made. Dispositions are agent judgments, never human or engineering rulings.
