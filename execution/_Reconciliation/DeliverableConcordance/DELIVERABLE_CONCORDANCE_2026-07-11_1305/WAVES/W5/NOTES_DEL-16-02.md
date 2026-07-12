# NOTES — DEL-16-02 Operation validation and diff preview (R2 W5)

DEL-16-02 / PKG-16, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained discovery pilot**. Ledger: 28 rows, 20 columns, RFC-4180
CRLF after normalization.

## Census and histograms

ClaimType: REQUIREMENT 8; ACCEPTANCE 8; EXCLUSION 4; DECLARED_STATE 6;
REMAINING_WORK 2. Disposition: ALIGNED 27; STALE_SETUP_SPECIFICATION 1.
AuthorityNeeded: NO 24 / OWNER 4. Selectability: YES 0 / NO 28. Confidence:
HIGH 25 / MEDIUM 3. SourceReliability: UNVERIFIED 22 / NOT_APPLICABLE 6.

The requirement census preserves REQ-16-02-001..008 one-for-one. Eight
acceptance rows preserve the eight distinct Specification verification
targets. The declaration census is the four-document kit plus `_STATUS.md`
and `MEMORY.md`; no deliverable-owned README exists. DECL-005 copies both
non-bootstrap residuals in status order and excludes the D-41 bootstrap.
`NormativeSource` paths are relative to the frozen DEL-16-02 folder unless a
repository path is explicit.

## Key judgments and self-flags

- The current slice is ALIGNED at its expressly bounded grain: canonical
  operation-schema validation, injected blocking constraint findings,
  deterministic preview, model-role/current-hash guards, unit/dimension
  vocabulary, target/mutation blocking, structured diagnostics, and negative
  professional boundaries are implemented and focused-test covered.
- REQ-003/ACC-002 remain ALIGNED at *constraint-diagnostic preservation*
  grain. DEL-13-03 live API integration is explicitly downstream/TBD and must
  not be inferred from injected test diagnostics.
- REQ-004 and REQ-006 remain ALIGNED at current supported-row/structured-slice
  grain. This does not establish final DEL-14 comparison/tolerance payload or
  canonical cross-package result-envelope completion.
- DECL-002 alone is STALE_SETUP_SPECIFICATION because it explicitly calls
  decomposition revision 0.7 the accepted current basis. Specification,
  Guidance, and Procedure accurately describe current behavior; their legacy
  DAG row IDs are bounded local derivative evidence, not asserted aggregate
  graph authority. MEMORY's revision-0.7 entry is dated history.
- REM-001 accurately records the gated center-of-gravity vector-payload design
  ruling. REM-002 accurately records the owner-review entry still required for
  corpus cases 66–75. Both are ALIGNED remaining work, not implementation
  failure or authorization to decide/apply them. Since both are gated, every
  row remains mechanically non-selectable.
- The three earlier PKG-02 review findings are human-dispositioned
  `ACCEPT_AS_IS` and `RESOLVED`; no mismatch rows are minted for them.
- No ACCEPTED_DIVERGENCE, PARTIALLY_IMPLEMENTED, VERIFIED_NOT_VALIDATED,
  UNKNOWN, AUTHORITY_CONFLICT, IMPLEMENTED_UNMAPPED, SECURITY marker, or
  lifecycle row is encoded.

## Implementation and evidence coverage

Primary owned/mapped surfaces inspected:
`core/model_operations/validation_preview/engine.py`,
`schemas/model_operation.schema.json`,
`tests/test_operation_validation_preview.py`,
`tests/test_model_operation_schema.py`, and the two invented operation/model
fixtures. Rust `core/model_operations/operation_applier`, Tauri commands, and
desktop operations/diff-preview/proposal/editor surfaces were treated as
downstream consumers/integration evidence, not duplicate ownership of the
DEL-16-02 validation-preview contract.

ALIGNED rows do not establish final constraint-engine API integration,
canonical diff/result-envelope completion, tolerance suitability, durable
acceptance, professional reliance, or release readiness.

## Verification and addendum-9 containment

Re-executed at the frozen SHA with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1602`, and cache suppression:

- `python3 -m pytest -p no:cacheprovider tests/test_model_operation_schema.py
  tests/test_operation_validation_preview.py -q` — **10 passed**.

No cargo was re-executed; recorded same-SHA Rust/desktop integration evidence
was inspected but not promoted above UNVERIFIED. No generator, compilation, or
`py_compile` ran. Lockless Cargo copy-out therefore did not apply. Frozen
tracked porcelain was empty before and after. Ignored-aware porcelain showed
exactly the same six allow-listed paths:

1. `projects/chirality-piping/.pytest_cache/`
2. `projects/chirality-piping/core/reporting/report_generator/Cargo.lock`
3. `projects/chirality-piping/core/reporting/result_export/Cargo.lock`
4. `projects/chirality-piping/core/reporting/state_comparison_handoff_sections/__pycache__/`
5. `projects/chirality-piping/tests/__pycache__/`
6. `projects/chirality-piping/validation/benchmarks/nonlinear/target/`

The frozen tree was neither cleaned nor modified. Writes were limited to this
CSV and notes in the dedicated D-41 worktree.

## Cross-ledger risks and fences

1. DEL-16-02 consumes DEL-16-01 schema authority and DEL-13-03/DEL-14-03/05
   contracts while feeding DEL-16-03/04 and desktop apply/proposal flows. R3
   must distinguish ownership from consumption/integration.
2. Injected constraint diagnostics are not proof of live constraint-engine
   integration. Preserve this grain without duplicating DEL-13-03 work.
3. Validation-preview no-mutation and the later Rust returned-document apply
   seam are complementary stages, not contradictory behavior.
4. The center-of-gravity design ruling and cases-66–75 review record are two
   governed residuals. Deduplicate them with Receipt-10/corpus surfaces and do
   not infer selectability from their presence.
5. Legacy DAG-002/DAG-006 labels are local evidence only; current authority is
   DAG-007 and no dependency rewrite is authorized.

No product, status, lifecycle, DAG, register, decision, dependency, review,
R4, or R5 surface changed. No release, certification, sealing, approval,
authentication, professional-reliance, code-compliance, legal-sufficiency, or
security-sufficiency assertion is made. Dispositions are agent judgments,
never human or engineering rulings.
