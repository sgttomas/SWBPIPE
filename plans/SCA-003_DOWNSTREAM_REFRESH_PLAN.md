---
doc_id: PLAN-SCA-003-DOWNSTREAM-REFRESH
doc_kind: plan.workflow
status: draft_for_orchestrator_handoff
created: 2026-05-17
scope_change: SCA-003
accepted_revision: "0.6"
execution_boundary: planning_only_no_type2_dispatch
---

# SCA-003 Downstream Refresh Plan

Plan identifier: SCA-003_DOWNSTREAM_REFRESH.

## Purpose

SCA-003 selected the MVP local storage profile for OpenPipeStress: a local-only SQLite project store/index with rebuildable retrieval sidecars, while preserving canonical JSON/JCS as the domain and interchange truth. The decomposition has already accepted this decision as revision `0.6`; the remaining work is derivative closure across schemas, docs, tests, persistence code, desktop packaging, and save/open UX.

This plan is a handoff artifact for a future ORCHESTRATOR/TASK session. It does not dispatch Type 2 work, mutate schemas, change code, promote lifecycle state, or claim implementation closure.

## Accepted Basis

| Surface | Role | SCA-003 status |
|---|---|---|
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Current decomposition working surface | Accepted at revision `0.6` with SCA-003 local storage profile incorporated. |
| `execution/_ScopeChange/SCA-003_2026-05-17_1658/` | SCOPE_CHANGE execution snapshot | Accepted scope-change record and audit trail for SCA-003. |
| `execution/_ScopeChange/SCA-003_2026-05-17_1658/Handoff_State.md` | Handoff state | Records SCA-003 as accepted with derivative closure still open. |
| `execution/_ScopeChange/SCA-003_2026-05-17_1658/RUN_SUMMARY.md` | Run summary | Records focused validation and accepted amendment summary. |
| `docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md` | Docs-side authority | Establishes SCA-003 as the accepted docs authority for the MVP local storage profile. |
| `docs/_Registers/ScopeLedger.csv` | Scope companion register | Carries SCA-003 scope-ledger propagation for storage-related scope items. |
| `docs/_Registers/Deliverables.csv` | Deliverable companion register | Carries SCA-003 propagation for affected deliverables. |
| `docs/_Registers/ContextBudgetQA.csv` | Context-budget companion register | Carries SCA-003 context-budget propagation for affected task surfaces. |
| Affected `_CONTEXT.md` files | Dispatch context surfaces | Updated to preserve SCA-003 constraints for future bounded TASK work. |

The affected `_CONTEXT.md` files include the persistence, schema, API/plugin, adapter, packaging, and local-first security task contexts updated by SCA-003. A future ORCHESTRATOR inventory should verify this list against the accepted snapshot before dispatching derivative work.

## SCA-003 Decision Baseline

Future downstream work must preserve these accepted decisions:

- SQLite is the MVP local project store/index substrate.
- Canonical JSON/JCS remains the domain, hash, and interchange truth.
- SQLite tables are storage/projection implementation details, not public contracts.
- SQLite FTS5/BM25 retrieval sidecars are rebuildable, local-only, and non-authoritative.
- NumPy sidecars are optional only if later evidence justifies the packaging and maintenance cost.
- Large files are referenced by path/URI plus hash, size, classification, and verification metadata by default.
- Large files are not copied into a project package by default.
- No hosted DB, daemon, required network, cloud sync, telemetry path, or direct plugin/adapter SQL access is authorized.
- All storage mutation still flows through application-service command/query/job boundaries.
- DB migrations and product schema migrations are separate downstream concerns and must not be collapsed into one compatibility mechanism.

## Stale Surfaces

### DIRECT_REFRESH_REQUIRED

These surfaces directly implement or govern the storage decision and should receive bounded downstream TASK briefs.

| Surface | Likely owner | Required refresh |
|---|---|---|
| `docs/architecture/persistence_contract.md` | `DEL-00-04`, `DEL-02-05` | Replace physical-container TBD language with the SQLite local storage profile while preserving JSON Schema 2020-12, canonical JSON/JCS hashes, deterministic round-trip behavior, no silent defaults, and no bypass of application-service validation. |
| `docs/security/local_first_storage_policy.md` | `DEL-12-01` | Update local-first policy to state that storage is local-only, offline-capable, free, bundled with the desktop app, and has no hosted DB, daemon, required network, cloud sync, telemetry path, or repo-default write behavior. |
| `tests/security/test_local_first_storage_policy.py` | `DEL-12-01` | Align policy tests with the accepted SQLite local storage profile and the no-network/no-telemetry/no-private-path-leak requirements. |
| `schemas/project_persistence.schema.yaml` | `DEL-02-05` | Add the concrete physical-container profile, external artifact reference fields, and sidecar rebuildability expectations without making SQL DDL the public schema. |
| Persistence service code and tests | `DEL-02-05` | Implement create/open/save/validate/version-check/migrate behavior behind application services; preserve canonical bytes/hashes and test DB migration separately from product schema migration. |
| Desktop save/open UX | GUI workflow deliverables, `DEL-10-04` | Replace storage TBD or preview-only behavior with user-visible local project create/open/save flows only after the storage service contract exists. |
| Desktop packaging and SQLite capability | `DEL-10-04` | Verify macOS packaging can bundle the required SQLite/FTS5 capability offline, with no daemon, hosted DB, required network, cloud sync, or telemetry path. |
| Plugin/API/adapter boundaries | `DEL-02-04`, `DEL-10-01`, `DEL-10-02` | Prohibit direct SQL access by plugins, adapters, and external API consumers; expose only schema-governed application-service operations and canonical artifacts. |

### REVIEW_REQUIRED

These surfaces may contain terminology, assumptions, or storage-adjacent language that should be reviewed before editing. They should not be bulk-edited without a specific stale text finding.

| Surface | Review focus |
|---|---|
| `docs/TYPES.md` | Confirm type language still treats JSON Schema and canonical payloads as the domain contract, with SQLite only as local storage/projection. |
| `docs/SPEC.md` | Review product behavior references for project persistence, save/open semantics, offline posture, and external file references. |
| User-guide references | Ensure end-user storage language says local-only and offline-capable without promising unimplemented UX. |
| `docs/security/threat_model.md` | Review for SQLite-specific local storage risks: file permissions, local backups, private path leakage, DB dumps, logs, and sidecar caches. |
| `docs/security/telemetry_policy.md` | Confirm the storage path does not imply telemetry, cloud sync, or network-dependent persistence. |
| State/run/comparison/handoff/operation-audit docs and contexts | Review whether PKG-14, PKG-15, and PKG-16 persistence language depends on the old physical-container TBD. |

### HISTORICAL_OR_DEFERRED

These surfaces may contain old storage language, but they should be classified rather than bulk-edited. Historical records should remain stable unless a future CHANGE or audit task explicitly authorizes correction.

| Surface | Classification guidance |
|---|---|
| Old run records under `execution/**/_run_records/` | Historical evidence; do not rewrite for SCA-003 unless a record is incorrectly treated as current dispatch authority. |
| Package `MEMORY.md` files | Usually historical local memory; classify stale storage notes before editing. |
| Semantic audit artifacts | Preserve audit trail; update only via a bounded audit refresh if an artifact is actively consumed. |
| Archived coordination folders | Historical; do not bulk-edit. |
| Broad old SCA-001 physical-container TBD context text | Classify as historical or stale according to whether it is directly consumed by storage implementation. |

## Workflow Phases

### 1. Acceptance and Integrity Verification

Verify that SCA-003 remains the accepted decomposition basis before planning implementation work.

Required checks:

- Confirm `execution/_Decomposition/SOFTWARE_DECOMP.md` declares revision `0.6`.
- Confirm the latest scope-change pointers resolve to `SCA-003_2026-05-17_1658`.
- Confirm `execution/_ScopeChange/SCA-003_2026-05-17_1658/Handoff_State.md`, `execution/_ScopeChange/SCA-003_2026-05-17_1658/RUN_SUMMARY.md`, and `docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md` exist and agree on the accepted local SQLite storage profile.
- Confirm the three companion registers carry the SCA-003 propagation entries expected by the accepted snapshot.

Expected output: a short ORCHESTRATOR verification note. No Type 2 dispatch and no implementation edits.

### 2. ORCHESTRATOR Stale-Surface Inventory

Create a current inventory of stale storage surfaces and classify each surface as `DIRECT_REFRESH_REQUIRED`, `REVIEW_REQUIRED`, or `HISTORICAL_OR_DEFERRED`.

The inventory should identify:

- Current direct implementation targets.
- Current docs/security/schema targets.
- Old historical artifacts that mention physical-container TBD but should not be rewritten.
- Any unexpected stale surfaces not covered by this plan.

Expected output: a bounded dispatch inventory suitable for TASK brief creation.

### 3. RECONCILIATION Storage-TBD Classification Memo

Ask RECONCILIATION to decide which remaining physical-container TBD references are current stale obligations and which are historical/deferred artifacts.

The memo should:

- Prevent accidental bulk editing of historical records.
- Identify any live contradictions with SCA-003.
- Identify terminology that must be standardized before TASK work begins.
- Preserve the distinction between SQLite as storage/projection and canonical JSON/JCS as domain truth.

Expected output: storage-TBD classification memo with references to concrete files and line-level findings where practical.

### 4. Docs/Security Contract Refresh

Dispatch bounded TASK work for documentation and local-first security surfaces after the inventory and reconciliation memo are accepted.

Minimum scope:

- Refresh `docs/architecture/persistence_contract.md`.
- Refresh `docs/security/local_first_storage_policy.md`.
- Update related policy tests only where the policy text creates explicit test obligations.
- Preserve no hosted DB, no daemon, no required network, no cloud sync, no telemetry path, and no direct plugin/adapter SQL access.

Expected output: docs/security contract patch plus focused tests or test-plan notes.

### 5. Schema Contract Refresh

Dispatch bounded TASK work for `DEL-02-05`.

Minimum scope:

- Update `schemas/project_persistence.schema.yaml` to describe the SQLite physical-container profile without making SQL DDL public contract.
- Add external artifact reference representation for path/URI, hash, size, classification, and verification metadata.
- Record sidecars as rebuildable and hash-neutral.
- State that DB migrations and product schema migrations are separate downstream concerns.
- Preserve JSON Schema 2020-12, canonical JSON/JCS hashes, deterministic round-trip behavior, and no silent defaults.

Expected output: schema patch and round-trip/hash/migration test expectations.

### 6. Persistence Service and Tests

Dispatch bounded TASK work only after the docs/security and schema contract updates are reviewed.

Minimum scope:

- Implement a SQLite local project store/index behind application services.
- Store canonical project/model/state/run payloads as schema-governed JSON with JCS-compatible hashes.
- Use SQLite tables as payload containers and indexed projections, not public interchange contracts.
- Keep FTS5/BM25 sidecars rebuildable and non-authoritative.
- Keep NumPy sidecars out of the mandatory MVP unless a later benchmark-backed task authorizes them.
- Add diagnostics for missing, moved, stale, private, or hash-mismatched external file references.
- Test that sidecar deletion and rebuild do not change canonical project hashes.
- Test DB migrations separately from product schema migrations.

Expected output: persistence service implementation and focused tests.

### 7. Desktop Packaging and UX Follow-Through

Dispatch bounded TASK work for packaging and user workflows after the persistence service contract exists.

Minimum scope:

- Verify macOS app packaging bundles or otherwise provides SQLite/FTS5 capability locally and offline.
- Ensure app behavior does not require a daemon, hosted DB, required network, cloud sync, or telemetry path.
- Implement create/open/save UX against application-service storage APIs.
- Keep plugin/API/adapter consumers away from direct SQL access.
- Ensure large-file references are visible and diagnosable without silently copying source files into the project package.

Expected output: packaging validation, UX implementation, and focused tests or manual QA notes.

### 8. REVIEW/AUDIT/CHANGE Closure

After bounded TASK patches land, run closure through the evaluative agents.

Minimum closure sequence:

- REVIEW checks each deliverable against SCA-003, acceptance criteria, tests, privacy policy, and no-bypass constraints.
- RECONCILIATION checks terminology and cross-package consistency.
- AUDIT agents check decomposition coverage, dependency closure, governance conformance, and epistemic integrity as appropriate.
- CHANGE records the derivative closure state, snapshots any approved updates, and stages or commits only if the human project authority requests it.

Expected output: derivative closure record, not a new scope change unless implementation reveals an architectural contradiction.

## Dispatch Tranche Recommendation

### Tranche A: Contracts Before Code

Dispatch first:

- `DEL-00-04`: persistence and schema versioning architecture.
- `DEL-12-01`: local-first storage and private data paths.
- `DEL-02-05`: project persistence and round-trip serialization.

Purpose: lock the contract, security posture, and persistence schema before implementation.

### Tranche B: Implementation Spine

Dispatch after Tranche A review:

- Persistence service and tests for SQLite-backed local project storage.
- `DEL-10-04`: desktop build, packaging, and CI/CD pipeline SQLite/FTS5 capability.
- `DEL-10-01`: public API boundary no-SQL checks.
- `DEL-10-02`: import/export adapter no-SQL checks.
- GUI save/open work once storage APIs exist.

Purpose: implement storage behind application services and verify that packaging/UX can consume it locally and offline.

### Tranche C: Product Workflow Alignment

Dispatch after Tranche B stabilizes:

- State/run/comparison persistence alignment.
- Handoff manifest persistence alignment.
- Model operation audit-trail persistence alignment.
- Reporting and diagnostics references affected by external artifact handling.

Purpose: align higher-level workflows with the accepted local storage substrate without exposing SQL as product contract.

## Required Downstream Acceptance Criteria

Derivative work is not closed until the refreshed surfaces demonstrate:

- Round-trip save/open preserves canonical JSON/JCS hashes and semantic equality.
- SQLite storage and indexed projections do not replace JSON Schema public contracts.
- DB migrations and product schema migrations are tested separately.
- FTS5/BM25 sidecars can be deleted and rebuilt without changing project hashes.
- NumPy sidecars remain optional and non-authoritative unless later evidence explicitly promotes them.
- Large-file references detect missing, moved, stale, private, and hash-mismatched files.
- Large files are not silently copied into the project package by default.
- No private data is written into public repo paths by default.
- No network, hosted DB, daemon, cloud sync, or telemetry path is required.
- Plugins, adapters, and external API consumers cannot bypass application services with direct SQL.

## Non-Goals

- No Type 2 dispatch from this plan.
- No schema mutation by this plan.
- No code mutation by this plan.
- No lifecycle promotion.
- No DAG approval claim.
- No professional approval, certification, or code-compliance claim.
- No hosted/cloud storage assumption.
- No bulk editing of historical artifacts.
- No commitment to mandatory NumPy retrieval sidecars for MVP.

## Open Decisions for Downstream Work

These decisions should be handled by bounded downstream TASK or ORCHESTRATOR work, not by this plan:

- Exact Rust/desktop SQLite dependency and packaging strategy.
- Exact proof that SQLite FTS5 is available in the bundled macOS app build.
- Exact project package shape, file extension, and user-visible folder/file model.
- DB migration tool or migration table convention.
- External artifact verification cadence and user-facing repair workflow.
- Criteria for later promoting NumPy sidecars from optional cache to implemented local cache.
- Portable export workflow for optionally copying selected external artifacts after user review and redaction.

## Recommended Next Action

Ask ORCHESTRATOR to begin Phase 1 and Phase 2 from this plan: verify accepted SCA-003 integrity, inventory stale storage surfaces, and request a RECONCILIATION storage-TBD classification memo. Only after those outputs are accepted should bounded TASK briefs be created for Tranche A.
