# MEMORY: DEL-17-04

## 2026-05-18 - SCA-004 PREPARATION scaffold
- Created minimum viable fileset from SOFTWARE_DECOMP revision 0.7.
- Populated local context, dependencies placeholder, references, semantic placeholder, and status.
- Production documents, semantic lensing, dependency extraction, implementation, release claims, and professional claims remain unperformed.

## 2026-05-18 - TP-EXPORT-003 ORCHESTRATOR population
- Populated the four-document kit for the CAEPIPE MBF profile/writer at contract/design level only.
- Generated `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and `Dependencies.csv`.
- Updated `_DEPENDENCIES.md` and `_STATUS.md` to `SEMANTIC_READY`.
- Remaining implementation/source TBDs: CAEPIPE version/profile, first MBF record-family subset, required fields, stable ID carrying strategy, blocking/diagnostic classification, invented fixtures, and writer tests.
- No MBF writer code, schema, parser, harness, CAEPIPE compatibility claim, release claim, code-compliance claim, or professional-acceptance claim was made.

## 2026-05-18 - TP-EXPORT-004R bounded TASK rerun
- Rebuilt `_SEMANTIC.md` as a full semantic-matrix artifact with canonical A/B and derived C/F/D/K/G/X/T/E matrices; appended `_STATUS.md` history while preserving `SEMANTIC_READY`.
- Rebuilt `_SEMANTIC_LENSING.md` with complete A/B/C/F/D/X/E lens coverage, lens-specific `NO_ITEMS` notes, and 12 warranted P3 items.
- Applied warranted P3 items only to `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`; added source/evidence slots, acceptance criteria, diagnostic classification guidance, pass-through-option rationale, and implementation readiness checks.
- Added three deliverable-local run records under `_run_records/` for `semantic-matrix-build`, `lens-register`, and `four-documents P3_ONLY`.
- Validation results: `validate_semantic_matrix.py` PASS; `validate_lens_register.py` PASS; `check_four_documents.sh` PASS; `check_min_viable_fileset.sh` PASS; `validate_dependencies_schema.py` PASS; final `git diff --check` PASS.
- Remaining TBDs: first CAEPIPE target version/profile, first MBF record-family and required-field subset, stable-ID direct carrying versus sidecar mapping, unsupported-entity blocking/diagnostic classification, invented fixtures, deterministic writer implementation, and loss-report tests.
- Boundary exclusions: no edits to coordination files, validators, schemas, code, DAG/blocker queues, `Dependencies.csv`, `_DEPENDENCIES.md`, `_REFERENCES.md`, or other deliverable folders; no CAEPIPE compatibility, release, code-compliance, formal validation, or professional-acceptance claim.

## 2026-05-18 - TP-EXPORT-004R parent fan-in review
- Parent reviewed sealed TASK outputs without authoring semantic or lensing content inline.
- Fan-in validation passed for `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, four-document kit, minimum viable fileset, `Dependencies.csv`, validator pytest coverage, validator py_compile, and `git diff --check`.
- Boundary scan found only negative guardrail language and no affirmative CAEPIPE compatibility, release, code-compliance, professional-acceptance, protected-data, proprietary-data, or reverse-engineering claim.

## 2026-05-23 - CAEPIPE MBF export foundation implementation
- Implemented the first bounded CAEPIPE MBF export foundation: `core/handoff/caepipe_mbf/`, `schemas/caepipe_mbf_export.schema.json`, `fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json`, and `tests/test_caepipe_mbf_export_package.py`.
- The builder emits deterministic ASCII MBF text for an invented smoke subset, manifest/member hashes, sidecar stable-ID map, mandatory loss report, validation report, diagnostics, provenance, privacy, and professional-boundary flags.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` to distinguish this bounded foundation from full CAEPIPE target coverage and to preserve sidecar-first stable IDs while direct MBF ID carrying remains `TBD-17-01-003`.
- Validation passed: JSON parse checks for schema and fixture; `python3 -m py_compile core/handoff/caepipe_mbf/*.py tests/test_caepipe_mbf_export_package.py`; `pytest tests/test_caepipe_mbf_export_package.py`; adjacent `pytest tests/test_native_json_export_package.py tests/test_handoff_export_workflow.py`; four-document, minimum-fileset, dependency-schema, semantic-matrix, and lens-register checks for DEL-17-04.
- Remaining TBDs: CAEPIPE target version/profile, definitive MBF record-family and required-field subset, direct MBF stable-ID carrier, broader unsupported-entity classification, external execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and any target-specific compatibility claims remain future guarded work.
- Boundary exclusions preserved: no lifecycle change, candidate promotion, coordination pointer update, commit, release claim, CAEPIPE compatibility claim, solver-validation claim, code-compliance claim, professional-acceptance claim, external CAEPIPE execution, CSV parser, PCF exporter, GLB/glTF exporter, commercial solver behavior, proprietary example, protected standards content, or reverse-engineering was introduced.

## 2026-06-03 - TP-PKG17-LIFECYCLE-DISPOSITION-001
- Human-approved lifecycle disposition set local `_STATUS.md` to `IN_PROGRESS` for the current committed-evidence posture.
- Evidence basis: `DEV-001_BLOCKER_QUEUE.csv` records this deliverable as committed and unblocked; `TP-INTEGRATED-VERIFY-002_2026-05-31` passed executed checks; `TP-LIFECYCLE-READINESS-AUDIT-001_2026-05-31` identified PKG-17 lifecycle disposition as human-gated.
- Run record: `_run_records/TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-06-03.md`; closeout snapshot: `execution/_Aggregation/TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-06-03/`.
- Non-claims preserved: no product implementation change, DAG change, DEV-001 evidence edit, release claim, target compatibility claim, code-compliance claim, or professional-reliance claim.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-17-04`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - Unsupported-entity diagnostic/loss-report tranche

- Implemented four-document-anchored DEL-17-04 refinement for unsupported CAEPIPE MBF entities.
- Added blocking diagnostics for malformed unsupported entity refs, missing matching `unsupported` loss-report entries, and `info`-severity unsupported loss entries.
- Regenerated the invented CAEPIPE MBF fixture so its loss report covers exported, omitted, approximated, delegated, unsupported, and `tbd` categories.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` to record the selected `TBD-17-04-004` policy: explicit unsupported behavior is warning-level by default unless separately marked blocking; missing or understated coverage blocks.
- Validation passed: py_compile for CAEPIPE MBF code/tests; JSON parse checks for schema and fixture; `pytest tests/test_caepipe_mbf_export_package.py` (12 passed); adjacent native JSON and handoff workflow tests (12 passed); four-document, minimum-fileset, dependency-schema, semantic-matrix, and lens-register checks; boundary scan reviewed; targeted `git diff --check` passed.
- Remaining TBDs preserved: CAEPIPE target version/profile, definitive MBF record-family and required-field subset, direct MBF stable-ID carrier, external execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and target-specific compatibility claims remain future guarded work.
- Boundary exclusions preserved: no edits to lifecycle status, coordination/DAG files, PKG-01, release claims, CAEPIPE compatibility claims, solver-validation claims, code-compliance claims, professional-acceptance claims, commercial solver behavior, proprietary examples, protected standards content, or reverse-engineering.

## 2026-06-04 - Profile-basis guardrail tranche

- Implemented DEL-17-04 profile-basis guardrails for the CAEPIPE MBF export foundation.
- Added blocking diagnostics for blank or weakened target-version basis, record-subset basis, and missing carried TBD references while source-confirmed CAEPIPE version/profile and MBF record-family closure remain unresolved.
- Tightened `schemas/caepipe_mbf_export.schema.json` so `target_version_basis` and `record_subset_basis` are non-empty and `carried_tbd_refs` must include `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`.
- Updated focused tests in `tests/test_caepipe_mbf_export_package.py`; the invented fixture still matched the deterministic builder output, so no fixture checksum churn was introduced.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` to record the profile-basis guardrail without closing CAEPIPE target/version, MBF subset, or direct stable-ID TBDs.
- Validation passed: py_compile for CAEPIPE MBF code/tests; `pytest tests/test_caepipe_mbf_export_package.py` (15 passed); adjacent native JSON and handoff workflow tests (12 passed); JSON parse checks for schema and fixture; `Dependencies.csv` schema validation; four-document, minimum-fileset, semantic-matrix, and lens-register checks.
- Remaining TBDs preserved: CAEPIPE target version/profile, definitive MBF record-family and required-field subset, direct MBF stable-ID carrier, external execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and target-specific compatibility claims remain future guarded work.
- Boundary exclusions preserved: no lifecycle status edit, coordination/DAG edit, candidate promotion, release claim, CAEPIPE compatibility claim, solver-validation claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, proprietary example, protected standards content, or reverse-engineering.

## 2026-06-04 - Write-package manifest parity tranche

- Implemented DEL-17-04 writer parity for the CAEPIPE MBF export foundation.
- Changed `write_caepipe_mbf_export_package` to write manifest-declared package members instead of a separate hard-coded sidecar list, with unsafe path and unknown-role fail-fast checks.
- Strengthened writer tests so every manifest-declared path is written; JSON members are canonicalized; MBF text remains ASCII; declared non-recursive member hashes match written content; and manifest JSON content matches the package manifest.
- Validation passed: py_compile for CAEPIPE MBF code/tests; `pytest tests/test_caepipe_mbf_export_package.py` (15 passed); adjacent native JSON and handoff workflow tests (12 passed); JSON parse checks for schema and fixture; `Dependencies.csv` schema validation; four-document, minimum-fileset, semantic-matrix, and lens-register checks; final targeted `git diff --check`.
- Remaining TBDs preserved: CAEPIPE target version/profile, definitive MBF record-family and required-field subset, direct MBF stable-ID carrier, external execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and target-specific compatibility claims remain future guarded work.
- Boundary exclusions preserved: no lifecycle status edit, coordination/DAG edit, candidate promotion, schema/fixture churn, release claim, CAEPIPE compatibility claim, solver-validation claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, proprietary example, protected standards content, or reverse-engineering.

## 2026-06-04 - Source-basis normalization tranche

- Normalized DEL-17-04 CAEPIPE MBF emitted `source_basis_refs` so the builder and invented fixture no longer list `DEL-17-03` as CAEPIPE target/source authority.
- Preserved `DEL-17-03` as historical implementation-pattern evidence only; current emitted source-basis refs are `DEL-17-01`, `DEL-17-02`, `CAEPIPE-IMPORT-MBF`, and `CAEPIPE-EXPORT-MBF`.
- Added focused regression coverage that blocks reintroducing `DEL-17-03` into default source-basis refs and confirms manifest/profile source-basis parity.
- Validation passed: `pytest tests/test_caepipe_mbf_export_package.py` (15 passed); adjacent native JSON and handoff workflow tests (12 passed); JSON parse checks for schema and fixture; `Dependencies.csv` schema validation; py_compile for CAEPIPE MBF code/tests; four-document, minimum-fileset, semantic-matrix, and lens-register checks; targeted `git diff --check`.
- Remaining TBDs preserved: CAEPIPE target version/profile, definitive MBF record-family and required-field subset, direct MBF stable-ID carrier, external execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and target-specific compatibility claims remain future guarded work.
- Boundary exclusions preserved: no lifecycle status edit, coordination/DAG edit, candidate promotion, dependency authority edit, release claim, CAEPIPE compatibility claim, solver-validation claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, proprietary example, protected standards content, or reverse-engineering.

## 2026-06-04 - Source-basis guardrail tranche

- Implemented DEL-17-04 guardrails for caller-supplied CAEPIPE MBF profile `source_basis_refs`.
- Added blocking diagnostics when profile source-basis refs omit `DEL-17-01`, `DEL-17-02`, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, or include `DEL-17-03` as CAEPIPE target/source authority.
- Tightened `schemas/caepipe_mbf_export.schema.json` so both export-profile and manifest source-basis refs carry the required authority set and reject `DEL-17-03`.
- Added focused regression coverage for missing source-basis refs and unsafe `DEL-17-03` source-basis promotion.
- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` to record `DEL-17-04-REQ-011` and the source-basis guardrail treatment.
- Validation passed: py_compile for CAEPIPE MBF code/tests; JSON parse checks for schema and fixture; `pytest tests/test_caepipe_mbf_export_package.py` (17 passed); adjacent native JSON and handoff workflow tests (29 total passed); `Dependencies.csv` schema validation; four-document, minimum-fileset, semantic-matrix, and lens-register checks; targeted `git diff --check`.
- Boundary-language scan hits were reviewed as negative guardrail language, schema/property text, historical run evidence, or explicit no-claim boundary wording; no affirmative CAEPIPE compatibility, release, code-compliance, professional-acceptance, protected-content, proprietary-data, or reverse-engineering claim was introduced.
- Remaining TBDs preserved: CAEPIPE target version/profile, definitive MBF record-family and required-field subset, direct MBF stable-ID carrier, external execution, CSV parsing, runtime/API/GUI integration, lifecycle/acceptance decisions, and target-specific compatibility claims remain future guarded work.
- Boundary exclusions preserved: no lifecycle status edit, coordination/DAG edit, candidate promotion, dependency authority edit, release claim, CAEPIPE compatibility claim, solver-validation claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, proprietary example, protected standards content, or reverse-engineering.

## 2026-06-12 - TP-UNITS-B2-EXPORTDISCLOSURE-001 export unit-system disclosure

- Added required `unit_system_disclosure` to CAEPIPE MBF export packages.
- The Python builder now emits and checksums `unit_system_disclosure.json`
  with DEC-018 unit-system ref, entered-unit storage convention, source model
  units, smoke-subset target export units, conversion policy, false
  export-time conversion flag, and protected/private-content false flags.
- The strict schema, invented fixture, desktop MBF panel, and App regression
  now cover the disclosure and manifest member.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
  corresponding DEL-02-02, DEL-17-06, and DEL-17-07 run records;
  `apps/desktop/SMOKE.md` TP-MAC-139.
- Validation: focused export-package tests passed 32/32; repository Python
  tests passed 356/356; full desktop Vitest 216/216; desktop build passed;
  Playwright R2 smoke 2/2.
- Boundaries preserved: no CAEPIPE compatibility claim, target import
  round-trip, external execution, solver-validation claim, code-compliance
  claim, professional-acceptance claim, commercial solver behavior,
  protected standards content, private data, or reverse-engineering.

## 2026-06-04 - TP-PKG17-CHECKING-TRANSITION-001

- Explicit human approval set local `_STATUS.md` to `CHECKING` after formal review fan-in found no blockers for DEL-17-04.
- Refreshed active dependency authority wording so DAG-006 is current graph authority and historical DAG-005 wording is provenance only.
- Closed the DAG-authority review warning in `Review_Findings.csv`; open target-support TBDs remain non-blocking for CHECKING.
- Boundary exclusions preserved: no DAG artifact edit, candidate promotion, DEV-001 evidence edit, release claim, target compatibility claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, protected standards content, or external-validation claim.
