# MEMORY - DEL-02-05 Project Persistence and Round-Trip Serialization

## Session 2026-05-01

Human project authority authorized one bounded DAG item of ORCHESTRATOR's
choosing. ORCHESTRATOR selected `DEL-02-05 - Project persistence and
round-trip serialization` because `DEL-02-01`, `DEL-02-02`, and `DEL-02-03`
are completed predecessors and persistence anchors downstream storage, hashes,
reports, CLI/headless runs, GUI editing, and private-data handling.

Fresh dispatch brief:

- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-05.md`

Approved write scope:

- `schemas/project_persistence.schema.yaml`
- `docs/architecture/persistence_contract.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_persistence_schema.py`
- this `MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implemented/tightened contract:

- Added explicit persistence validation profile coverage.
- Added application-service `PersistenceOperation` records for create, open,
  save, validate, version-check, and migrate flows.
- Added checksum payload-scope and payload-partition status fields.
- Added external hash-bound `HumanAcceptanceRef` records that invalidate on
  bound-hash changes and do not imply software approval or code compliance.
- Added round-trip normalization rules that prohibit silent engineering
  defaults.
- Preserved the physical project package/container, migration framework,
  canonical JSON library choice, and non-JSON/binary hash partition as `TBD`.

Guardrails:

- No lifecycle state transition was made.
- No blocker queue refresh was run.
- No `DAG-001`, candidate-edge, `Dependencies.csv`, or `_DEPENDENCIES.md`
  mutation occurred.
- No protected standards text, protected tables, proprietary engineering
  values, private data, or automatic compliance/certification/sealing claims
  were introduced.

## 2026-05-11 TP-RECON-01 Reconciliation

- Evidence sources: `plans/TP-RECON-01_DISPATCH_MATRIX.csv` row for
  `DEL-02-05`, `AGENTS.md`, `docs/CONTRACT.md`,
  `docs/IP_AND_DATA_BOUNDARY.md`, TP-MAC-06 through TP-MAC-10, TP-PER-01,
  commit `742016e`, current DEL files, archived DEV-001 evidence/status
  rows, and current persistence service/schema/fixture/tests.
- Implemented persistence evidence: TP-PER-01 and
  `core/project_persistence/service.py` provide schema-shaped project
  envelopes, deterministic canonical JSON/hash helpers, round-trip validation,
  run-history refs, private-data/provenance/professional-boundary diagnostics,
  telemetry-off default, and `physical_container.status = "TBD"`;
  `schemas/project_persistence.schema.yaml`,
  `fixtures/persistence/invented_persisted_preview_project.json`,
  `tests/test_persistence_schema.py`, and
  `tests/test_project_persistence_service.py` corroborate the contract.
- Implemented TP preview evidence: TP-MAC-06 through TP-MAC-10 closeouts
  record bounded preview additions for thermal axial load, midspan recovery,
  explicit user mechanics combinations, pressure thrust, and fixed station-grid
  recovery; the persistence fixture and service tests carry representative
  run-history refs for midspan, quarter-station shear, and combination results.
- Verification recorded: archived evidence maps `DEL-02-05` to committed
  implementation evidence `742016e` and lifecycle `CHECKING`; TP-PER-01 and
  TP-MAC-06 through TP-MAC-10 closeouts record final verification commands,
  while current persistence tests cover schema shape, stable hashes,
  round-trip equality, mutation detection, and missing-boundary diagnostics.
- Deferred boundaries: durable physical project containers, desktop save/open
  UX, final CLI syntax, migrations, external storage/execution, private data
  workflows beyond schema markers, protected rule/code content, allowables,
  SIF/flexibility factors, release claims, and professional acceptance remain
  outside this deliverable.

## 2026-05-16 PKG-02 Foundation-Slice Hardening

Scope executed:

- Converted `tests/test_persistence_schema.py` from script-only assertions into
  pytest-collected contract and fixture tests while preserving direct
  `python3` execution.
- Added focused persistence-service fixture assertions in
  `tests/test_project_persistence_service.py`.

Evidence:

- Pytest now collects and passes
  `tests/test_persistence_schema.py tests/test_project_persistence_service.py`.
- Direct execution with `python3 tests/test_persistence_schema.py` passes.
- Fixture checks cover deterministic hash scopes, round-trip manifest behavior,
  run-history references, private-data defaults, provenance manifest entries,
  telemetry-off default, and professional-boundary flags.

Boundaries preserved:

- Physical project container, durable open/save UX, migration framework,
  non-JSON partitioning, and desktop persistence workflows remain `TBD`.
- Existing deterministic hash and mutation-detection behavior remains intact.
- Lifecycle remains `IN_PROGRESS`; no lifecycle transition was made.
- No dependency register, DAG, blocker queue, or candidate-edge edits.

## 2026-05-17 TP-PHYS-012-C Transformed Physical Source Persistence Guard

- Added a persistence-service test that transforms the populated canonical
  physical source fixture into an analytical solver-model payload, validates
  the payload against `schemas/model.schema.yaml`, embeds it with
  `build_project_persistence_envelope`, and validates the persistence envelope
  with delegated `model.schema.yaml` resolution.
- Added a negative guard proving a transformed payload with noncanonical
  `area_moment` dimension is rejected by delegated persistence schema
  validation even when persistence-service hashes are recomputed.
- Verification passed: `python3 -m pytest tests/test_project_persistence_service.py`
  with 8 tests passed; `python3 -m pytest tests/test_physical_to_analytical_transform.py`
  with 8 tests passed; `python3 tests/test_model_schema.py` passed;
  `python3 tests/test_persistence_schema.py` passed.
- No persistence schema, persistence service implementation, physical project
  container decision, migration framework, lifecycle/status, dependency
  register, DAG, blocker queue, candidate row, review finding disposition,
  product preview runtime, CLI/API, protected standards content, owner
  criteria, private/proprietary data, code-compliance claim, professional
  reliance claim, release readiness claim, or human acceptance status was
  changed or introduced.

## 2026-06-03 - TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001 CHECKING transition
- Human approval accepted non-resolving DEV-001 evidence commits as migration-caused aberrations and approved lifecycle advancement to `CHECKING` for formal review.
- Evidence basis: `TP-CODE-EVIDENCE-AUDIT-001_2026-06-03` found current source/schema/fixture/test evidence and passing targeted/full-gate checks; `TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001_2026-06-03` reconciled the migration-era commit-pointer gap.
- Local `_STATUS.md`, DEV-001 blocker queue lifecycle displays, and DAG-005 deliverable display surfaces were aligned to `CHECKING` where applicable.
- Boundary preserved: this is review-readiness only; no `ISSUED`, release-readiness, external compatibility, code-compliance, protected-IP/private-data, or professional-engineering authentication claim is made.

## 2026-06-04 - TP-AUTHORITY-REFRESH-0_7-DAG006

- WORKING_ITEMS/TASK workforce current-authority refresh applied to active deliverable-local surfaces for `DEL-02-05`.
- Current authority basis is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7` plus approved `execution/_DAG/DAG-006/` active graph authority.
- Historical run records, historical DAG row IDs, review dispositions, lifecycle `_STATUS.md`, aggregate DAG artifacts, candidate edges, repo-level governance files, schemas, code, and tools were intentionally not changed by this refresh.
- Preserved historical references remain evidence of earlier work, not current authority claims.

## 2026-06-10 - TP-MAC-79 validation-preflight model-hash evidence

- Desktop app tranche: the Project Validation Preflight panel now reports the
  canonical model-hash evidence chain instead of stale TBDs. The
  `reproducibility_metadata` round-trip category, packet summary, validation
  profile, visible `Model hash evidence` line, and staged diagnostics carry
  the model-hash status (`computed_not_persisted` →
  `persisted_open_verification_not_run` → `verified_on_open` /
  `mismatch_review_required` / `recompute_unavailable_review_required`).
- The hash basis is unchanged: WebCrypto SHA-256 over JCS-like sorted-key
  model JSON (DEL-08-02 evidence service from TP-MAC-75/78); this tranche
  only consumes that evidence. The full project-envelope hash remains an
  explicit TBD via the new
  `validation_profile.project_envelope_hash_status` field; migration
  framework and physical container remain TBD.
- Evidence: `{ScopePath}/_run_records/TASK_RUN_2026-06-10_1755.md`;
  `apps/desktop/SMOKE.md` TP-MAC-79 (browser smoke marker
  `2026-06-10T23:51:19.823Z`, zero console errors); Vitest 13/13, desktop
  build, cargo lib 3/3, pytest 340/340 all green.
- Boundary preserved: review-only local technical-preview reproducibility
  signal, model-payload scope only; no lifecycle, release, certification,
  sealing, authentication, professional-approval, or code-compliance claim.

## 2026-06-10 - TP-MAC-80 versioned store schema migration ledger

- Desktop app tranche: the local SQLite project store now initializes through
  a versioned `PRAGMA user_version` migration ledger
  (`versioned_sqlite_user_version_migration_ledger`,
  `STORE_SCHEMA_TARGET_VERSION=7`: v1 base project/FTS tables, v2–v7 the six
  snapshot JSON columns) instead of ad-hoc unversioned setup, and the
  hard-coded `migration_status="current"` is replaced by derived staged
  statuses (`current_store_schema_v7_no_pending_migrations` /
  `migrated_on_open_store_schema_v{before}_to_v{after}`). Migration steps are
  idempotent, so legacy pre-ledger stores reconcile from v0 without data loss
  (cargo-tested, including row preservation with defaulted columns).
- Evidence surfaces: `StorageCapability` and `LocalProjectSummary` carry
  `migration_framework`, `migration_status`, `store_schema_version`,
  `store_schema_target_version`, `migrations_applied_on_open`; the validation
  preflight packet gains a `store_migration` block, profile fields, a visible
  `Store migration evidence` line, a staged `migrate` service operation
  (closing `not_run_migration_framework_tbd`), and staged `MIGRATION`-class
  info diagnostics. Browser fallback reports honest non-SQL parity values.
- Authority basis: DEL-00-04 REQ-04-01/REQ-04-02 (versioned persistence;
  declared schema version and migration status). Model *document* schema
  migrations remain an explicit TBD
  (`model_document_migration_status=model_document_migrations_not_defined_tbd`),
  as do the full project-envelope hash and physical project container.
- Evidence: `{ScopePath}/_run_records/TASK_RUN_2026-06-10_1935.md`;
  `apps/desktop/SMOKE.md` TP-MAC-80 (browser smoke marker
  `2026-06-11T01:27:57.689Z`, zero console errors); Vitest 13/13, desktop
  build, cargo lib 5/5 (2 new migration tests), pytest 340/340 all green.
- Boundary preserved: local store maintenance evidence only; no destructive
  migrations; no lifecycle, release, certification, sealing, authentication,
  professional-approval, or code-compliance claim.

## 2026-06-10 - TP-MAC-81 full project-envelope hash persisted and verified on open

- Desktop app tranche: the canonical reproducibility hash chain
  (TP-MAC-75/78/79 model-payload scope) now extends to the full persisted
  project envelope, closing the validation-profile
  `project_envelope_hash_status=
  model_payload_scope_only_full_project_envelope_hash_tbd` seam.
  `computeProjectEnvelopeHash` hashes the canonical JSON of `{model,
  editor_intents, proposal, selected_review_target, mechanics_result,
  analysis_run, model_hash}` — excluding the volatile storage summary and
  the envelope-hash carrier field so the hash is recomputable from a
  restored envelope. Computed at create/save, persisted, and verified on
  open (`verification_basis=recomputed_on_open_from_restored_envelope_payload`).
- Store schema: ledger migration v8 (`store-v8-project-envelope-hash-column`,
  `STORE_SCHEMA_TARGET_VERSION=8`) adds `project_envelope_hash_json`;
  existing v7 stores stage `migrated_on_open_store_schema_v7_to_v8` — the
  first real staged use of the TP-MAC-80 migration ledger, cargo-tested via
  `store_migration_ledger_stages_v7_store_to_v8_applying_only_pending_migration`
  (only the pending v8 step applies; rows preserved).
- Evidence surfaces: `LocalProjectSummary` gains
  `persisted_project_envelope_hash_count/ref`; the validation preflight
  gains an `Envelope hash evidence` line
  (`data-testid="project-validation-envelope-hash"`), packet blocks
  `project_envelope_hash` / `project_envelope_hash_integrity`, profile
  `project_envelope_hash_scope=
  persisted_envelope_payload_excluding_storage_summary_and_hash_carrier`,
  and staged REPRODUCIBILITY diagnostics
  (`PROJECT-VALIDATION-ENVELOPE-HASH-NOT-COMPUTED` / `-REVIEW-ONLY` /
  `-MISMATCH`).
- Evidence: `{ScopePath}/_run_records/TASK_RUN_2026-06-10_2015.md`;
  `apps/desktop/SMOKE.md` TP-MAC-81 (fresh-server browser smoke marker
  `2026-06-11T02:11:26.627Z`, `verified_match` over an envelope including a
  persisted mechanics result and analysis run, zero console
  errors/warnings); Vitest 13/13, desktop build, cargo lib 6/6, pytest
  340/340 all green.
- Boundary preserved: the envelope hash is a local technical-preview
  review-reproducibility signal only; model *document* schema migrations and
  the physical project container remain explicit TBDs; no lifecycle,
  release, certification, sealing, authentication, professional-approval, or
  code-compliance claim.

## 2026-06-10 - TP-APP-R2-PERSIST-001 model-document schema migration seam (A2, DEC-019)

- The human project authority ruled decision D-08 (`DEC-019`): two-track
  versioning with in-document semver as the sole model-document version
  authority, migrate-in-memory-on-open / persist-on-save, explicit-operation
  evidence guarantees. The desktop app now implements that policy:
  `apps/desktop/src-tauri/src/model_document_migration.rs` (transform-chain
  registry, refusal semantics for newer/unsupported/failed documents), store
  schema v9 evidence-only `model_migration_ledger_json` column with
  pre/post-hash ledger records, and DEC-019 evidence in the validation
  preflight replacing the `model_document_migrations_not_defined_tbd` marker.
- This deliverable's reserved `MigrationStatus` shape and
  `application_service_separate_db_and_product_schema` framework enum from
  `schemas/project_persistence.schema.yaml` are now exercised by running
  code. The staged explicit "Migrate project" operation, sibling JSON-slot
  coverage, read-only inspection of newer documents, and the
  backward-compatibility window size remain open (window size is an explicit
  `TBD — human ruling` carried in the DEC-019 packet).
- Evidence: run record
  `_run_records/WORKING_ITEMS_RUN_2026-06-10_model_document_schema_migration.md`;
  smoke `apps/desktop/SMOKE.md` TP-MAC-83; suites 24/24 src-tauri, 25/25
  Vitest, 342/342 pytest, build green.
- No lifecycle change: `_STATUS.md` remains `CHECKING`; this is
  app-integration absorption of design authority, not issuance.

## 2026-06-11 - TP-APP-R2-PERSISTEDSOLVE-001 persisted edited-load solve regression

- WORKING_ITEMS app-integration tranche added Tauri backend regression
  evidence that a structured edited load magnitude survives local SQLite
  persistence and reload before solve.
- The test applies `op:test-load-L-100-Y-magnitude` to change `load:L-100`
  `primitive_loads.1.magnitude.value` from `350` to `425` N, persists the
  edited model, reloads `project:edited-load-roundtrip`, and verifies the
  restored model/hash carrier before solving.
- Evidence is recorded in
  `_run_records/WORKING_ITEMS_RUN_2026-06-11_persisted_edited_load_solve_regression.md`
  and `apps/desktop/SMOKE.md` TP-MAC-107. Validation passed:
  src-tauri format check and src-tauri Rust tests 27/27.
- This did not complete packaged-Tauri GUI smoke, change schema migration
  policy, create a physical container ruling, alter lifecycle state, or make
  release, professional, certification, sealing, authentication, or
  code-compliance claims.
