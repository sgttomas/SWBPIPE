# NOTES — DEL-17-08 GLB/glTF review geometry export (R2 W5)

DEL-17-08 / PKG-17, IN_PROGRESS; frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Role/model: **GPT-5
deliverable-grained owning pilot**. Ledger: 49 rows, 20 columns, RFC-4180 CRLF
after normalization.

## Census and histograms

ClaimType: REQUIREMENT 26; ACCEPTANCE 11; EXCLUSION 5; DECLARED_STATE 6;
REMAINING_WORK 1. Disposition: ALIGNED 29; PARTIALLY_IMPLEMENTED 12;
STALE_SETUP_SPECIFICATION 7; ACCEPTED_DIVERGENCE 1. AuthorityNeeded: NO 29 /
OWNER 15 / ENGINEERING 5. Selectability: YES 0 / NO 49. Confidence: HIGH 36 /
MEDIUM 13. SourceReliability: UNVERIFIED 43 / NOT_APPLICABLE 6.

The requirement census preserves all 26 non-contiguous source IDs one-for-one
through contiguous run-local REQ-001..026 IDs; every `NormativeSource` records
the original DEL-17-08 number. Eleven acceptance rows preserve all five Phase A
Verification checks and all six Future Requirement-to-Check Map slots. Five
exclusion rows separate setup-era writer/schema/fixture and GUI/API exclusions
from still-active release/compatibility, solver/analysis, and professional/
compliance boundaries. The declaration census is exactly the four-document kit
plus `_STATUS.md` and `MEMORY.md`; no owned README exists. RF-002 is the sole
open residual. The D-41 bootstrap appears only as excluded declaration context
in DECL-005; its `RemainingSource` and gate metadata are `NONE_RECORDED` and it
is excluded from residual/selectability analysis.

## Key judgments and self-flags

- REQ-009 is PARTIALLY_IMPLEMENTED. The schema-backed Python package accepts
  only geometry already normalized to glTF +Y-up/right-handed meters and does
  not carry separate source-basis/applied-transform provenance. The desktop
  packet records DEC-018 source units and a z-up-to-y-up transform, but the two
  paths are not shown as one runtime-schema-validated route.
- REQ-015 is PARTIALLY_IMPLEMENTED. The sidecar distinguishes `canonical_ref`
  from one `gltf_ref`, but it does not explicitly distinguish node, mesh,
  primitive, generated-name, and sidecar-row identifiers as required.
- REQ-018 is PARTIALLY_IMPLEMENTED. Manifest source/model/profile/member/boundary
  fields exist, but source-basis refs live only inside an in-memory package
  profile that is neither embedded directly in the manifest nor written as a
  hashed package member.
- REQ-021 is PARTIALLY_IMPLEMENTED. Output is deterministic, generator text is
  fixed, and timestamps are omitted, but no profile/manifest field declares
  fixed-generator and omitted-timestamp policy.
- REQ-023/024 and ACC-010 are PARTIALLY_IMPLEMENTED. Python v1 selects
  centerline segments and records one omitted support; it has no complete
  canonical-family coverage register. The desktop preview also emits node,
  support, and component markers, so selected/deferred family state is not
  reconciled in one profile.
- REQ-026 is PARTIALLY_IMPLEMENTED. Tests cover units, coordinate basis,
  duplicate IDs, zero length, unresolved nodes, missing geometry, missing ID
  map, and missing loss report. Impossible-bend and emitted-entity-to-map
  completeness checks are absent.
- ACC-004 is PARTIALLY_IMPLEMENTED because the mixed setup-era TBD check still
  labels concrete JSON glTF filename/profile/schema/identity/fixture decisions
  unresolved. ACC-006 lacks a 26-requirement implementation traceability
  matrix. ACC-008 lacks a bound application-service/API writer seam. ACC-011
  lacks binary GLB validation and a rendered visual-comparison method.
- ACC-005, EXC-001/002, and DECL-001..004 are
  STALE_SETUP_SPECIFICATION. Active documents still say Phase A produces no
  writer, schema, fixture, GUI, or implementation even though these artifacts
  exist. Substantive visual-review, privacy, compatibility, solver-fidelity, and
  professional boundaries remain applicable.
- REM-001 is ACCEPTED_DIVERGENCE, not closure. RF-002 remains OPEN, but the
  named human-approved CHECKING transition explicitly accepted its Phase A
  wording warning as nonblocking. As an unhomed review finding, it retains its
  review `RemainingSource` but uses `RecordedRemaining=NONE_RECORDED` and
  `GateOrStageConstraint=NONE_RECORDED`. This permits bounded deferral only.
  RF-001 is RESOLVED and is not a residual; dated DAG-005/006 references remain
  history.
- SECURITY/governance negative-boundary rows use explicit-reason
  `NOT_APPLICABLE`; there are zero convention-6 SECURITY markers and no
  owner-gated security-sufficiency deferral.
- No VERIFIED_NOT_VALIDATED, REMAINING_STATE_MISMATCH, UNKNOWN,
  AUTHORITY_CONFLICT, IMPLEMENTED_UNMAPPED, lifecycle row, compatibility ruling,
  or professional/engineering approval is encoded.

## Implementation and evidence coverage

Primary mapped surfaces inspected:

- `core/handoff/review_geometry/{__init__.py,package.py}`;
- `schemas/review_geometry_export.schema.json`;
- `fixtures/review_geometry/invented/{source_centerline_payload.json,
  review_geometry_export_package.json,model.gltf}`;
- `tests/test_review_geometry_export_package.py`;
- `apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx` and its
  focused `App.test.tsx` evidence;
- the complete deliverable-local truth set, dependency/semantic artifacts,
  review findings, and durable run records.

The Python implementation deterministically emits JSON `model.gltf` using
embedded-buffer LINES primitives for canonical centerline elements, direct
identity `extras`, an authoritative sidecar ID map, manifest, loss report,
validation report, diagnostics, hashes, privacy/provenance, and professional
boundaries. It does not emit binary GLB, surface/tube geometry, or viewer-
compatibility evidence. The desktop preview adds visible unit/coordinate
witnesses and more marker families, but is adjacent evidence rather than proof
of Python schema/writer integration.

## Verification and addendum-9 containment

Executed from the frozen tree with `PYTHONDONTWRITEBYTECODE=1`, external
`PYTHONPYCACHEPREFIX=/tmp/chirality-pyc-del1708`, and pytest cache disabled:

- `python3 -m pytest -p no:cacheprovider -q
  tests/test_review_geometry_export_package.py` — PASS, 10 passed in 0.15s.
- `python3 tools/validation/validate_dependencies_schema.py <Dependencies>` —
  PASS, 29 columns / 19 rows.
- four-document and minimum-fileset checks — PASS.
- semantic-matrix and lens-register validators — PASS.

No Cargo command, generator, compilation, or `py_compile` ran. Lockless Cargo
copy-out therefore did not apply. Frozen tracked porcelain was empty before
execution. Ignored-aware porcelain showed exactly the six allow-listed paths:

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

1. DEL-17-02 owns common package/profile/manifest/ID-map/loss-report vocabulary;
   DEL-17-08 owns the target-local JSON glTF geometry realization. Shared fields
   are contract consumption, not duplicate ownership.
2. DEL-17-01 and GLTF-2.0 are source authorities, not compatibility evidence.
   A structurally valid invented `model.gltf` does not establish viewer support.
3. Python centerline-only coverage and desktop node/support/component markers
   must not be merged into a single asserted coverage profile without an
   explicit binding and reconciliation.
4. Unit disclosure plus `conversion_performed=false` is not numeric transform
   validation. The Python accepted-target-basis path and desktop z-up-to-y-up
   transform remain distinct evidence grains.
5. Binary GLB remains absent. Do not infer GLB container correctness from JSON
   glTF structure or from the deliverable title.
6. Stable sorted compact Python JSON is labeled
   `JCS_compatible_json_payload_hash`; no RFC-8785 conformance vectors were found.
   R3 should join this with the shared PKG-14/17 canonicalization risk without
   converting deterministic hashes into full JCS proof.
7. Exact equality to an invented `model.gltf` fixture is deterministic structural
   evidence, not rendered visual QA, geometry fidelity, or viewer compatibility.
8. Documentation cleanup rows and RF-002 are one setup-wording species, not
   multiple product gaps. Dated Phase A and DAG records remain protected history.
9. DAG-007 is current graph authority. Dependency satisfaction observations
   authorize no DAG, register, or dependency mutation.

No product, status, lifecycle, DAG, register, decision, dependency, review, R4,
or R5 surface changed. No release, GLB/viewer compatibility, solver/analysis
fidelity, formal validation, certification, sealing, approval, authentication,
professional reliance, code compliance, legal sufficiency, target support, or
security sufficiency assertion is made. Dispositions are agent judgments, never
human or engineering rulings.
