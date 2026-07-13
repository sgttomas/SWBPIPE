# MEMORY: DEL-17-05

## 2026-07-12 - D-41 R5 T6 PDU-050 hold

- Cache-disabled tests verify schema, parser, skipped-without-executable,
  privacy/boundary diagnostics, user-responsibility gates, metadata, and
  invented fixtures only.
- No user-owned executable or selected target/MBF/invocation profile was
  available. Optional live CAEPIPE execution remains O10 profile-gated, so
  `DEL-17-05-ACC-006` stays `VERIFIED_NOT_VALIDATED`.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T6-PDU050-HOLD.md`.

## 2026-05-18 - SCA-004 PREPARATION scaffold
- Created minimum viable fileset from SOFTWARE_DECOMP revision 0.7.
- Populated local context, dependencies placeholder, references, semantic placeholder, and status.
- Production documents, semantic lensing, dependency extraction, implementation, release claims, and professional claims remain unperformed.

## 2026-05-18 - TP-EXPORT-005 ORCHESTRATOR population
- Populated the four-document kit, regenerated `_SEMANTIC.md`, generated `_SEMANTIC_LENSING.md`, applied Pass 3 warranted edits, and refreshed `Dependencies.csv` plus `_DEPENDENCIES.md`.
- Run records added for sealed phases: `TASK_RUN_2026-05-18_1156.md`, `TASK_RUN_2026-05-18_1203.md`, `TASK_RUN_2026-05-18_1211.md`, `TASK_RUN_2026-05-18_1218.md`, and `TASK_RUN_2026-05-18_1226.md`.
- Parent validation passed: four-document check, minimum fileset check, semantic matrix validator, lens register validator, P3 disposition validator, dependency schema validator, `git diff --check`, and boundary-language scan.
- Remaining TBDs include CAEPIPE invocation profile, parser coverage, first MBF profile details, and target compatibility behavior pending source evidence.
- Boundary exclusions preserved: no implementation code, schemas, runtime harness, bundled executable, license-bypass path, compatibility claim, release claim, code-compliance claim, or professional-acceptance claim.

## 2026-05-24 - DEL-17-05 external-run evidence foundation
- Added bounded implementation foundation for skipped and parser-only CAEPIPE external-run evidence.
- Added deterministic module, schema, invented public CSV/JSON fixtures, and focused tests.
- Public tests do not invoke CAEPIPE and do not require an executable, license, private payload, or commercial output.
- Remaining TBDs preserved: live invocation profile, source-confirmed CSV section coverage, target version/profile details, and any compatibility or validation interpretation.
- Boundary exclusions preserved: no lifecycle promotion, DAG change, bundled executable, license-bypass path, release claim, compatibility claim, code-compliance claim, solver-validation claim, or professional-acceptance claim.

## 2026-06-03 - TP-PKG17-LIFECYCLE-DISPOSITION-001
- Human-approved lifecycle disposition set local `_STATUS.md` to `IN_PROGRESS` for the current committed-evidence posture.
- Evidence basis: `DEV-001_BLOCKER_QUEUE.csv` records this deliverable as committed and unblocked; `TP-INTEGRATED-VERIFY-002_2026-05-31` passed executed checks; `TP-LIFECYCLE-READINESS-AUDIT-001_2026-05-31` identified PKG-17 lifecycle disposition as human-gated.
- Run record: `_run_records/TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-06-03.md`; closeout snapshot: `execution/_Aggregation/TP-PKG17-LIFECYCLE-DISPOSITION-001_2026-06-03/`.
- Non-claims preserved: no product implementation change, DAG change, DEV-001 evidence edit, release claim, target compatibility claim, code-compliance claim, or professional-reliance claim.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-17-05`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-04 - TP-PKG17-SIBLING-GUARDRAIL-HARDENING-001

- Added a DEL-17-04 MBF-package binding guardrail for attempted CAEPIPE external-run evidence.
- Added blocking diagnostics for attempted external execution when license responsibility or user-owned environment responsibility is not acknowledged.
- Added focused regression coverage for unsafe MBF refs and missing responsibility acknowledgements.
- Boundary exclusions preserved: no lifecycle promotion, DAG change, executable bundling, license-bypass path, compatibility claim, release claim, code-compliance claim, solver-validation claim, or professional-acceptance claim.

## 2026-06-04 - TP-PKG17-CHECKING-TRANSITION-001

- Explicit human approval set local `_STATUS.md` to `CHECKING` after formal review fan-in found no blockers for DEL-17-05.
- Refreshed active dependency authority wording so DAG-006 is current graph authority and historical DAG-005 wording is provenance only.
- Narrowed the review warning to remaining Phase A/historical-context wording; open live-execution and parser-coverage TBDs remain non-blocking for CHECKING.
- Boundary exclusions preserved: no DAG artifact edit, candidate promotion, DEV-001 evidence edit, executable bundling, license-bypass path, release claim, target compatibility claim, code-compliance claim, professional-acceptance claim, commercial solver behavior, protected data, or external-validation claim.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.

## 2026-06-17 - TP-UNITS-BTAIL-CAEPIPEEXTERNALUNITWITNESS-001 CAEPIPE parser unit witnesses

- Primary role for the bounded Phase B-tail CAEPIPE external parser unit
  witness tranche. The desktop CAEPIPE External Harness panel now emits
  DEC-018 unit-system disclosure and one unit-preservation witness per parsed
  invented CSV row.
- The parser package preserves row value, unit, and inferred dimension with
  `conversion_performed=false`; visible UI evidence shows the DEC-018
  disclosure and `count=3` witnesses.
- Validation passed: focused App Vitest 55/55, focused R2 Playwright smoke
  2/2, full desktop Vitest 397/397, desktop production build with the
  existing Vite large-chunk warning, and `git diff --check`.
- Boundary preserved: no external CAEPIPE execution, executable/license/path
  requirement, target compatibility claim, solver validation claim, schema
  contract change, unit conversion API, protected standards content, private
  payload, lifecycle state transition, release-readiness claim, professional
  approval, certification, sealing, authentication, or code-compliance claim
  changed.
## 2026-07-12 - D-41 R5 T3 PDU-016 private default

- CAEPIPE user-output evidence now defaults to `private_user_controlled`, local-only, telemetry-disabled handling; partial caller overrides cannot erase those defaults.
- Focused negative/no-bypass evidence is recorded in `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T3-PDU016.md`.
- This is one selected seam only, not a whole-product security/privacy claim; lifecycle remains `IN_PROGRESS`.

## 2026-07-12 - D-41 R5 T4 PDU-010/PDU-038 conformance evidence

- Added an executable parser-boundary witness covering the current invented
  `NODE_DISPLACEMENTS`/metres and `ELEMENT_FORCES`/newtons rows, target IDs,
  stable IDs, source CSV reference, and parser-only/no-execution status.
- This is fixture-level parser verification. Invocation layout, live output
  discovery, broader section coverage, comparison semantics, target
  compatibility, and external validation remain unresolved and unclaimed.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T4-PDU010-PDU038.md`.
  Lifecycle remains `IN_PROGRESS`; the D-41 bootstrap remains for T7.
