---
doc_id: DEL-10-04-MEMORY
doc_kind: execution.memory
status: draft
created: 2026-05-04
deliverable_id: DEL-10-04
package_id: PKG-10
tranche: DEV-001_REV05_TRANCHE_C
revision: 0.5
---

# MEMORY - DEL-10-04 Build, Packaging, And CI/CD Pipeline

## 2026-07-12 - D-41 R5 T1 root workspace-manifest attribution (PDU-077)

- Owner ruling `DEC-074` option O3 attributes the existing root
  `projects/chirality-piping/package.json` to DEL-10-04 as
  implementation/build evidence. The independent receiving-scope audit found
  no contradiction with DEL-10-04's accepted build/package/CI scope.
- Read-only inspection records the `apps/desktop` workspace and the existing
  `dev:desktop`, `build:desktop`, `build:wasm:desktop`, `test:desktop`,
  `test:e2e:desktop`, and `test:e2e:dist:desktop` entrypoints, plus
  `generate:product-preview-mechanics`, which invokes the existing Rust
  example and writes the invented product-preview mechanics fixture.
- The manifest was not edited. This clarification does not activate CI,
  change build behavior, move shared `core/product_preview` governance,
  broaden fixture generation, or resolve provider/release/signing policy.
- The D-42 build-readiness panel attribution conflict is separate and remains
  unresolved and untouched.
- All genuine `_STATUS.md ## Remaining` items, including the exact D-41
  program item, are preserved pending other applicable R5 currentness
  backchecks. Lifecycle remains `IN_PROGRESS`.
- Evidence: `execution/_Coordination/_DECISIONS/D-41_R4_RULING_2026-07-12.md`
  (`DEC-074` O3); D-41 PDU-077; root manifest SHA-256
  `7e719791e3ffdc7b57eddb2bb32d682705bf945af5b7207ffc699a2a45648656`;
  `_run_records/TASK_RUN_2026-07-12_D41-R5-T1-PDU-077-root-package-json.md`.

## 2026-07-10 - TP-E5-EXPORTPIPE-001 DEC-059 sanitized-export pipeline build

- Built the DEC-059/D-05b prerequisite export pipeline
  (`tools/release/export_public_openpipestress.py`, stdlib-only) with
  self-tests (`tests/test_export_public_openpipestress.py`, 20 tests):
  fail-toward-exclusion allowlist staging, sanitize pass,
  `path/size_bytes/sha256` manifest bound to the canonical commit,
  mechanical boundary check, DEL-08-05 protected-content lint gate via the
  engine's `protected_content_lint_cli` (consumed as-is; same DEC-058
  check (b) surface as `run_release_candidate_scan.py`), export
  record/report emission, and `--verify` drift detection. Plan-only by
  default; `--execute` writes only to an explicit caller-named target
  directory outside the source root.
- Real-tree dry run into a temp directory: 892 files staged, boundary and
  lint gates pass (1 reviewed expected boundary finding, 5 reviewed
  expected blocking lint findings on invented lint fixtures +
  docs/TYPES.md prohibition prose, 1 provenance-review warning),
  deterministic across consecutive runs.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E5-EXPORTPIPE-001.md`.
- Boundary preserved (DEC-059 hard fences): no public repository created,
  nothing pushed/published, no CI activated, no `.github/workflows` file
  added, no linter/scan-surface edits, no release-readiness or clearance
  claim; the export act stays owner-gated behind the three DEC-059
  prerequisites (G1-G7 pipeline; D-20/DEC-058 owner-signed green scan
  before any push; D-06/DEC-057 repo location/naming).

## 2026-06-18 - TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001 supporting export-review classification

- Supporting role for Export Safety Review matrix cleanup: the
  `build_package_readiness` export row is explicitly classified as
  non-unit-bearing metadata because it records script, shell, readiness-profile,
  and release-decision status without quantities, units, dimensions, or target
  conversion.
- Evidence:
  `_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-EXPORTREVNONUNITBOUNDARY-001.md`;
  `apps/desktop/SMOKE.md` TP-MAC-271; completion log entry; primary DEL-12-02
  run record and supporting DEL-12-03/DEL-02-02 run records.
- Boundary preserved: no build script, package manifest, Tauri config, CI
  provider, release matrix, signing, notarization, publishing, installer,
  binary package, private payload, protected standards content, lifecycle
  transition, release-readiness claim, professional approval, certification,
  sealing, authentication, or code-compliance claim changed.

## Scope Executed

Implemented the provider-neutral DEL-10-04 release/build skeleton within the
sealed Tranche C brief write scope:

- created `docs/BUILD_AND_RELEASE.md`;
- created `docs/RELEASE_NOTES_TEMPLATE.md`;
- created `tools/release/check_release_readiness.py`;
- created `tests/test_release_readiness_script.py`;
- created sealed brief
  `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-10-04.md`;
- recorded this deliverable memory and a local run note.

This implementation did not create live CI workflow files, package-manager
manifests, signing or publishing configuration, release binaries, installers,
notarization artifacts, dependency-register edits, aggregate DAG changes, or
candidate-edge promotions.

Post-implementation closeout moved `DEL-10-04` to `CHECKING` with
`WORKING_TREE` evidence. A later CHANGE gate committed the implementation and
closeout as `daaff87`, then promoted the `DEL-10-04` evidence row to
`COMMITTED` using that implementation commit. The queue remained 73 unblocked /
19 blocked after promotion.

## Evidence Basis

The implementation was grounded in these project artifacts:

- `execution/_Coordination/DEV-001_REV05_TRANCHE_C_PROPOSAL.md`;
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-10-04.md`;
- `docs/CONTRACT.md`;
- `docs/IP_AND_DATA_BOUNDARY.md`;
- `docs/PROFESSIONAL_BOUNDARY.md`;
- `docs/VALIDATION_STRATEGY.md`;
- `docs/RELEASE_QUALITY_GATES.md`;
- `docs/_Registers/Deliverables.csv` row `DEL-10-04`;
- `docs/_Registers/ScopeLedger.csv` row `SOW-032`;
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`;
- approved `DAG-002` readiness evidence for `DEL-10-04`.

## Boundary Controls Applied

- Kept the CI provider, workflow path, coverage thresholds, performance
  thresholds, release matrix, installer formats, signing, notarization,
  attestation, publishing, and maintainer quorum as `TBD`.
- Made the readiness script provider-neutral, local-only, and dry-run by
  default.
- Avoided `shell=True` and external-service integration in the readiness
  script.
- Avoided protected standards text, protected tables, proprietary examples,
  private project data, private rule-pack data, private library data, real
  secrets, signing keys, and publication credentials.
- Framed release evidence as software-quality evidence only, not professional
  engineering acceptance or code-compliance proof.

## Implementation Notes

`tools/release/check_release_readiness.py` discovers crate-local Cargo
manifests under `core/` and `validation/benchmarks/` because the current
repository has no root Cargo workspace. It also preserves the absence of a root
JavaScript package or live desktop shell package manifest rather than inventing
one.

The script exposes these profiles:

- `skeleton`;
- `python`;
- `security`;
- `cargo`;
- `all`.

Only `skeleton` was executed during this implementation. Broader profile
execution is available for future review but may take longer and remains a
maintainer choice.

## Validation

Checks run on 2026-05-04:

| Check | Outcome |
|---|---|
| `python3 tools/release/check_release_readiness.py --profile skeleton` | Pass; dry-run listed 23 discovered Cargo manifests and 2 planned local checks. |
| `python3 tools/release/check_release_readiness.py --profile skeleton --execute` | Pass; `DAG-002` dependency schema validation passed and focused script tests passed. |
| `python3 -m pytest -q tests/test_release_readiness_script.py` | Pass; 3 tests passed. |
| `git diff --check` | Pass. |
| trailing-whitespace scan over changed DEL-10-04/coordination files | Pass; no matches. |
| focused protected-content/private-data scan | Reviewed; matches were boundary/handoff wording only, not copied protected data, tables, formulas, examples, or proprietary values. |
| focused private-data and real-secret scan | Reviewed; matches were boundary wording and historical handoff text only, not credentials, tokens, passwords, keys, or private data. |
| focused authority-overclaim scan | Reviewed; matches were negative boundary statements, human-gate wording, or historical handoff text only, not software certification, sealing, endorsement, approval, authentication, or compliance claims. |
| post-implementation closeout queue rebuild | Pass; 73 unblocked / 19 blocked with 56 evidence records and 55 committed evidence records. |
| post-commit evidence promotion queue rebuild | Pass; 73 unblocked / 19 blocked with 56 evidence records and 56 committed evidence records. |

## Remaining TBDs

- CI provider and hosted workflow location.
- Final supported OS/architecture release matrix.
- Installer/package formats.
- Signing, notarization, checksum publication, release attestation, and
  publishing policy.
- Coverage, performance, tolerance, and permitted-variance thresholds.
- Maintainer quorum and release authority.
- Final package/container format for desktop project files.

## 2026-05-11 TP-RECON-01 Reconciliation

TP-RECON-01 reconciled DEL-10-04 history from the archived coordination
bundle into this deliverable-local memory. The archived implementation
evidence rows record DEL-10-04 as `COMMITTED` at `daaff87` with subject
`docs: add provider-neutral release skeleton`; the lifecycle snapshot records
the deliverable as `CHECKING`.

Evidence reconciled:

- DEV-001 revision 0.5 Tranche C selected the provider-neutral
  release/packaging skeleton lane after the Tranche B promotion made
  DEL-10-04 newly unblocked.
- The sealed brief authorized local build/release documentation, a release
  notes template, a local dry-run capable readiness script, focused tests,
  deliverable memory, and run notes.
- The Tranche C closeout records the implementation and closeout commit
  `daaff87`, promotion of DEL-10-04 evidence from `WORKING_TREE` to
  `COMMITTED`, and a blocker queue rebuilt to 73 unblocked / 19 blocked.
- The TP-MAC-01 desktop assembly plan and TP-MAC-01-A sealed brief later use
  DEL-10-04 only as the provider-neutral desktop build-route and local
  dev/build-discipline basis for a scaffold; worker launch was not authorized
  by those archived TP-MAC-01 records.

Boundaries preserved by this reconciliation:

- Current state remains `CHECKING`; no release lifecycle terminal state is
  recorded.
- CI provider, workflow path, release matrix, thresholds, signing,
  notarization, attestation, publishing, maintainer quorum, and release
  authority remain `TBD`.
- No live CI workflow, release/signing/publishing surface, protected standards
  content, proprietary data, private project data, private rule-pack data,
  secrets, or professional reliance claim is added by this reconciliation.

## 2026-05-19 - TP-VERIFY-014B release-readiness command path audit

TP-VERIFY-014B audited the current release-readiness command/path evidence for
parent fan-in into `DEL-09-05`.

Findings:
- `docs/BUILD_AND_RELEASE.md` and `tests/test_release_readiness_script.py`
  still reference project-local `tools/release/check_release_readiness.py`.
- `tools/` is currently absent inside `chirality-piping/`.
- A matching script exists under the parent Chirality tool root at
  `/Users/ryan/ai-env/projects/chirality/tools/release/check_release_readiness.py`.

Classification: current gap until a later approved tranche either restores the
project-local script path or records the parent tool-root convention as
governed project execution evidence.

Local run record:
- `_run_records/TASK_RUN_2026-05-19_TP-VERIFY-014B.md`

Boundary: this audit changed only this `MEMORY.md` and the local run record. It
did not change lifecycle state, CI workflows, release records, candidate rows,
blocker queues, implementation evidence, professional-boundary decisions, or
code-compliance decisions.

## 2026-05-20 - TP-VERIFY-015 release-readiness command path closeout

TP-VERIFY-015 closed the project-local release-readiness command path gap
identified by TP-VERIFY-014.

Changes:
- Restored project-local `tools/release/check_release_readiness.py`.
- Added project-local `tools/validation/validate_dependencies_schema.py`,
  because the skeleton execute profile depends on it.
- Updated the restored release-readiness script to derive the approved
  dependency graph from `execution/_DAG/_LATEST.md`; it currently resolves to
  `DAG-005`.

Validation:
- `python3 tools/release/check_release_readiness.py --profile skeleton` passed.
- `python3 tools/release/check_release_readiness.py --profile skeleton --execute`
  passed.
- `python3 tools/validation/validate_dependencies_schema.py execution/_DAG/DAG-005/DependencyEdges.csv`
  passed with 29 columns and 988 data rows.
- `python3 -m pytest -q tests/test_release_readiness_script.py` passed with
  4 tests.

Local run record:
- `_run_records/TASK_RUN_2026-05-20_TP-VERIFY-015.md`

Boundary: this tranche changed only the project-local release readiness helper
surface, this `MEMORY.md`, the local run record, and the `DEL-09-05` fan-in
closeout. It did not change lifecycle state, CI workflows, release records,
candidate rows, blocker queues, implementation evidence, professional-boundary
decisions, or code-compliance decisions.

## 2026-05-31 - TP-VERIFY-017 coordination readiness command correction

TP-VERIFY-017 corrected the DEL-10-04 release-readiness coordination
regression command surface.

Changes:
- Updated `tools/release/check_release_readiness.py` so the `python` and `all`
  profiles run `python3 -m pytest -q tests/test_coordination_maintenance.py`
  instead of collecting `tools/coordination`.
- Added focused coverage in `tests/test_release_readiness_script.py` confirming
  the `python` and `all` profiles include the intended coordination
  maintenance test command and exclude the old `tools/coordination` target.

Validation:
- `python3 -m pytest -q tests/test_release_readiness_script.py tests/test_coordination_maintenance.py`
  passed with 9 tests.
- `git diff --check` passed.

Local run record:
- `_run_records/TASK_RUN_2026-05-31_TP-VERIFY-017_DEL-10-04.md`

Boundary: this tranche changed only the DEL-10-04 release-readiness command
surface, the focused script test, this `MEMORY.md`, and the local run record.
It did not change lifecycle state, DAG artifacts, blocker queues,
implementation evidence, release records, CI workflows, professional-boundary
decisions, or code-compliance decisions.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-10-04`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-11 - TP-SWEEP-001 (DEC-025 evidence sweep + F-4 rider)

- The D-05 ruling (`DEC-025`, SOFTWARE_DECOMP §12) is implemented: hosted CI
  deferred; `tools/release/run_evidence_sweep.py` is the deterministic
  five-surface local evidence entrypoint and the commit-bound pre-push/fan-in
  merge gate for parallel agent branches. Summary artifacts land in
  `validation/evidence/sweeps/SWEEP_<utc>_<commit12>[-dirty].json`.
- F-4 rider landed: `apps/desktop/scripts/build-wasm-engine.mjs` now writes
  wasm-bindgen glue to a sibling staging dir and renames it into place.
- Documentation: `docs/BUILD_AND_RELEASE.md` §2/§3/§5.1/§7/§9 and
  `docs/RELEASE_QUALITY_GATES.md` §10 updated; §9's CI-provider TBD closed
  with the `D-05b` follow-up pointer (public sanitized-export CI, with D-06).
- Focused tests: `tests/test_evidence_sweep.py` (11 tests).
- Local run record:
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_TP-SWEEP-001_evidence_sweep.md`
- Boundary: development-evidence tooling only; no hosted workflow file; no
  lifecycle, release, professional, certification, or code-compliance claims.

## 2026-06-12 - TP-R2VERIFY-FIX-002 sweep git-state hardening

- Repair of R2 verification finding F-2
  (`plans/VERIFICATION_2026-06-12_r2_exit_chain.md`): a failed git capture in
  `run_evidence_sweep.py` could read as a clean working tree.
  `collect_git_state` now records `status_capture_failed`,
  `working_tree_dirty` becomes `null` on capture failure, filenames gain
  `-gitunverified`, and `--execute` exits nonzero when the summary cannot bind
  to a verified git state (DEC-025 commit-binding contract). `schema_version`
  bumped 1 → 2.
- Validation: `tests/test_evidence_sweep.py` 16/16 (two new cases plus the
  pinned schema bump).
- Evidence: `_run_records/WORKING_ITEMS_RUN_2026-06-12_sweep_git_state_hardening.md`.
- Boundary unchanged: tooling evidence only; gate semantics per DEC-025.

## 2026-06-17 - Lifecycle Housekeeping

- Housekeeping lifecycle reset: `_STATUS.md` current state set to `IN_PROGRESS` to reflect current code development in progress. This does not change review, issuance, release readiness, professional approval, certification, sealing, authentication, or code-compliance status.
