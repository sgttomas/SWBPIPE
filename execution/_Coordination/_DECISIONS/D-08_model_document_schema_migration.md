# D-08 — Model-Document Schema Migration Policy

**Status:** AWAITING_RULING
**Prepared:** 2026-06-10 by TASK (Type 2), requested by WORKING_ITEMS (Type 1), tranche TP-APP-R2-EDITLOOP-001 decision-preparation subscope.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-08.
**Plan basis:** `plans/PLAN_2026-06-10_prd_completion.md` §2 row D-08; §3 Phase A item A2.
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

---

## 1. Decision statement and scope

**Decide:** the versioning and migration-ledger policy for persisted **model documents** — the editable model JSON payload stored per project row — extending (not replacing) the existing SQLite **store-schema** migration ledger.

The two layers must not be conflated:

| Layer | What it versions | Mechanism today | Status |
|---|---|---|---|
| **Store schema** | SQLite tables/columns of the local project store (`openpipestress-projects.sqlite3`, `apps/desktop/src-tauri/src/lib.rs:14`) | `PRAGMA user_version` integer ledger, v1–v8, applied on every store open (`lib.rs:140–318`) | Implemented (TP-MAC-80/81) |
| **Model-document schema** | The JSON payload inside each row's `model_json` column (and sibling JSON slots) | Documents carry a semver `schema_version` string, but **no policy** governs version checks, migration, or ledger records | **Undefined — this decision** |

D-08 blocks Phase A item A2 ("Store model documents inside the SQLite project envelope with schema version + migration ledger entries (per D-08)", `plans/PLAN_2026-06-10_prd_completion.md` §3 row A2) and sits on the plan's load-bearing chain A1→A2→A5 (§5).

---

## 2. Current state evidence

Line references are pinned to the committed baseline at preparation time (commit `6905e4e38`; `lib.rs` last changed by `ff4aac8f9`). `apps/desktop/src-tauri/src/lib.rs` is under concurrent uncommitted development in the sibling A1 tranche, so symbol names are the durable anchors if working-tree line numbers have drifted.

### 2.1 Store-schema ledger (implemented; the seam to extend)

- `STORE_SCHEMA_TARGET_VERSION: i64 = 8` and `STORE_MIGRATION_FRAMEWORK = "versioned_sqlite_user_version_migration_ledger"` — `apps/desktop/src-tauri/src/lib.rs:140–141`.
- Ledger entries are `StoreMigration { version, migration_id, apply }` (`lib.rs:143–147`); `store_migrations()` lists v1 (base `local_projects` + FTS tables) through v8, where v2–v8 each add one JSON column — v7 `model_hash_json`, v8 `project_envelope_hash_json` (`lib.rs:160–245`).
- `apply_store_migrations` runs on every store open, applies only steps with `version > user_version`, advances `PRAGMA user_version`, and derives staged status strings (`current_store_schema_v{n}_no_pending_migrations` / `migrated_on_open_store_schema_v{a}_to_v{b}`) (`lib.rs:274–318`). Steps are idempotent so pre-ledger stores reconcile from v0 without data loss (`lib.rs:280–282`).
- Evidence surfaces carry `migration_framework`, `migration_status`, `store_schema_version`, `store_schema_target_version`, `migrations_applied_on_open` (`lib.rs:48–53, 61–65`).
- The store is one database holding **many** project rows (`project_id TEXT PRIMARY KEY`, `model_json TEXT NOT NULL`, `lib.rs:251–257`; `list_local_projects`, `lib.rs:1280–1284`). `user_version` is a single integer per database file, not per document.
- Tranche history: DEL-02-05 `MEMORY.md` entries "TP-MAC-80 versioned store schema migration ledger" and "TP-MAC-81 full project-envelope hash persisted and verified on open" (`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/MEMORY.md`).

### 2.2 Model documents already carry a semver `schema_version`

- Canonical model schema requires top-level `schema_version` matching `^[0-9]+\.[0-9]+\.[0-9]+$` plus `project`, with `additionalProperties: false` (`schemas/model.schema.yaml:6–21`).
- The persisted preview model document declares `"schema_version": "0.1.0"` and `"document_kind": "openpipestress.product_preview.model"` (`fixtures/product_preview/invented_preview_model.json:2–3`).
- FACT (observable from the two files): the preview document carries top-level fields (`document_kind`, `data_boundary`) that `schemas/model.schema.yaml` does not admit, so today's editable preview document is **not** an instance of the canonical model schema; no dedicated preview-model schema file exists under `schemas/`. The ruling should say which document family the policy governs.
- Sibling persisted JSON slots also carry their own `schema_version`/document kinds (e.g., the agent-proposal document, `apps/desktop/src-tauri/src/lib.rs:1092–1094`), and `schemas/model_state.schema.json:9,18–21` requires semver `schema_version` on immutable model states.

### 2.3 The app already declares the gap as an explicit TBD

- Validation preflight emits `model_document_migration_status: "model_document_migrations_not_defined_tbd"` (`apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx:467`), asserted by tests (`apps/desktop/src/App.test.tsx:1177–1185`).
- `open_local_project` returns the stored model with **no** model-document version check or migration step (`apps/desktop/src-tauri/src/lib.rs:1242–1277`).

### 2.4 What DEL-00-04 already specifies (architecture; state CHECKING since 2026-06-04 per its `_STATUS.md`)

Path: `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-04_Persistence and schema versioning architecture/`

- REQ-04-01: persistence is deterministic, versioned, unit-aware, provenance-preserving; REQ-04-02: **every persisted artifact declares schema version and migration status**; REQ-04-03: canonicalization is a prerequisite for reproducible hashes (algorithm then-TBD) (`Specification.md`).
- `Datasheet.md` TBD slots include **migration framework**, physical project file format, storage backend — routed to human ruling, not silently selected.

### 2.5 What DEL-02-05 already specifies (persistence contract; state CHECKING since 2026-06-03 per its `_STATUS.md`)

Path: `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/`

- REQ-02-05-006: schema/version metadata + migration status sufficient to detect **unsupported, stale, or failed** migrations; framework TBD. REQ-02-05-021: migration status semantics and diagnostics; framework/tooling TBD. REQ-02-05-023: deterministic `MIGRATION`-class diagnostics (`Specification.md`).
- Service contract defines an explicit **"Migrate project"** operation taking "Source project, target schema version, migration policy" (`Specification.md`, operations table), and behavior rows for supported/unsupported/stale/failed cases.
- `Datasheet.md` "Migration mechanism" row: migration-aware persistence required; framework/tooling TBD.

### 2.6 The persistence envelope schema already reserves the record shape

`schemas/project_persistence.schema.yaml` (the DEL-02-05 contract; referenced by `docs/SPEC.md` §4.4):

- Top level requires `schema_version`, `project`, `hash`, **`migration`** (`:8–13`).
- `MigrationStatus` requires `status` ∈ {`current`, `migration_needed`, `stale`, `migrated`, `unsupported_schema`, `failed`, `newer_than_supported`, `TBD`}, plus semver `source_schema_version` / `target_schema_version`; it separates `db_migration_status` from `product_schema_migration_status` and enumerates `migration_framework` ∈ {`application_service_separate_db_and_product_schema`, `TBD`} (`:264–332`; `migration_framework` at `:318`). The envelope's model payload itself delegates to `schemas/model.schema.yaml` by `$ref` (`:333–341`).

### 2.7 Hash evidence chain that migration will touch

- Model and envelope hashes are SHA-256 over JCS-like sorted-key canonical JSON (`apps/desktop/src/services/hashService.ts:13–19, 36–43, 58–72`); the envelope hash excludes the volatile storage summary and its own carrier field so it is recomputable from a restored envelope.
- Both hashes are persisted per row (store v7/v8 columns, `lib.rs:222–243`) and **recomputed and compared on open**, yielding `verified_match` / `mismatch_review_required` (`apps/desktop/src/App.tsx:286–303, 958–1017`).

### 2.8 Governing documents

- `docs/SPEC.md` §4.4: persistence envelope is versioned and schema-governed; SCA-003 selects the local SQLite store as MVP substrate; migrate flows must return structured outputs and diagnostics; "product and DB migration mechanics … remain separate `TBD` decisions."
- `docs/PRD.md` §10 FR-001 (Must): "Create, open, save, and version projects — Project file round-trips without loss of model, units, loads, rule-pack references, or provenance metadata."

---

## 3. Open questions awaiting ruling

1. **Where the model-document schema version lives.** In-document `schema_version` (already present) as the single authority; a store column; or both with a defined precedence. Related: which document families are covered — `model_json` only, or all persisted JSON slots (`proposal_json`, `mechanics_result_json`, `analysis_run_json`, …) that carry their own `schema_version`.
2. **Migration trigger.** Migrate-on-open (eager rewrite), migrate-in-memory-on-open + persist-on-save, or only via the explicit "Migrate project" service operation that DEL-02-05 already defines.
3. **Ledger record shape.** Mirror of the Rust `StoreMigration {version, migration_id, apply}` chain keyed by semver; what a per-document applied-migration record contains (source/target versions, migration ids, timestamps, pre/post hashes) and where it is persisted (envelope `migration` block, store column, both).
4. **Forward compatibility.** Posture for `newer_than_supported` documents (schema enum already names the state): refuse open, or open read-only with diagnostics. Lossy down-migration is presumed prohibited — confirm.
5. **Backward compatibility window.** How many prior versions remain migratable; semantics of `stale` vs `unsupported_schema`; whether the pre-1.0 (`0.x`) window is exempt from compatibility guarantees.
6. **Rejection vs auto-migrate posture.** Whether any migration may occur without visible evidence. ASSUMPTION: silent auto-migration conflicts with the project's no-silent-mutation posture (`docs/SPEC.md` §4.4 round-trip rules; DEL-02-05 REQ-02-05-019 pattern of findings-not-defaults), so at minimum migration must be evidenced; whether it additionally requires explicit user action is the ruling.
7. **Hash-evidence interaction.** A migration rewrites canonical bytes, so persisted `model_hash`/`project_envelope_hash` change; absent a policy, a post-migration open would report `mismatch_review_required` (§2.7). Must the ledger record pre- and post-migration hashes, and is the pre-migration document retained (and for how long)?
8. **SemVer semantics.** What major/minor/patch mean for model documents (breaking vs additive vs editorial), and whether same-major reads are accepted without a migration record. `TBD` until ruled.

---

## 4. Options

### Option A — Extend the single store `user_version` ledger to cover model-document migrations

One integer ledger; bumping `user_version` implies rewriting every row's `model_json` in the same open-time migration pass.

- For: one mechanism, one evidence surface; no new record shape.
- Against: `user_version` is per-database, not per-document (§2.1), so all projects are force-migrated in bulk on open — maximal blast radius, hard rollback; conflates DDL with payload semantics, contradicting the schema's separated `db_migration_status` / `product_schema_migration_status` (§2.6); document version becomes implicit and non-portable, which collides with D-09 (a single-file container leaving the store loses its version context); silent bulk rewrite sits poorly with the evidenced-mutation posture (§3 Q6).

### Option B — Two-track versioning: store ledger for DDL; per-document semver transform chain for model documents

Keep `user_version` exclusively for tables/columns. Model documents are versioned by their in-document `schema_version`; a registered, ordered chain of pure document transforms (`source_semver → target_semver`, each with a stable `migration_id`, mirroring `StoreMigration`) upgrades a document. On open, the app compares document version to the supported target, migrates **in memory**, surfaces `MigrationStatus` (`migrated`, source/target versions, ids), and persists only on the user's save — at which point hashes are recomputed through the existing create/save path (§2.7).

- For: matches the schema's reserved shape and framework enum verbatim (§2.6); per-document versions survive export/import (D-09-compatible); stored bytes untouched until a user action, so stored hash evidence stays internally consistent; window policy expressible per version (`stale`/`unsupported_schema`/`newer_than_supported`).
- Against: two mechanisms to maintain; "migrated-but-unsaved" UI state needs honest labeling; document save now also records migration evidence (ledger entry incl. pre/post hashes) — more record-keeping.

### Option C — No implicit mutation: explicit migrate operation only

Open never alters the document, not even in memory beyond validation: out-of-date documents open as `migration_needed` (read-only/blocked for editing) until the user invokes the staged "Migrate project" operation (already defined by DEL-02-05 §2.5), which performs the transform, records a full ledger entry with pre/post hashes, and retains the pre-migration document.

- For: strongest human-decides and audit posture; cleanest hash story (every hash transition tied to an explicit recorded operation).
- Against: friction on every schema bump for every project; pre-1.0 churn (Phase A will iterate the document schema) makes routine edits annoying; risks users rubber-stamping the migrate dialog, eroding the intended deliberateness.

### Option D — Perpetual read-compatibility (lazy upcast readers, no rewrite)

Never migrate stored documents; maintain readers that upcast every supported historical version at parse time forever.

- For: zero rewrite risk; stored bytes and hashes never change.
- Against: reader complexity grows without bound; "no loss on round-trip" (FR-001) becomes ambiguous when old documents re-serialize through new writers anyway; weak forward-compatibility story; no migration evidence trail, sitting awkwardly with DEL-00-04 REQ-04-02's declared-migration-status requirement (§2.4).

---

## 5. Recommendation — `PROPOSAL`

Adopt **Option B**, hardened with Option C's evidence guarantees:

1. **Authority of version:** the in-document `schema_version` (semver) is the sole authority for model-document schema version; `PRAGMA user_version` remains DDL-only. Set `migration_framework = "application_service_separate_db_and_product_schema"` (already the schema's named non-TBD value, §2.6).
2. **Trigger:** migrate-on-open **in memory**, persist-on-save. Never rewrite stored bytes as a side effect of open. The staged "Migrate project" service operation remains available for explicit/bulk use.
3. **Visibility:** opening an out-of-date document always surfaces `MigrationStatus` (`source_schema_version`, `target_schema_version`, `status=migrated`, applied `migration_id`s) in the validation preflight, replacing `model_document_migrations_not_defined_tbd` (§2.3); saving a migrated document writes a per-document ledger record including pre- and post-migration `model_hash` values, preserving hash-chain continuity (§3 Q7).
4. **Compatibility posture:** `newer_than_supported` → refuse to open for editing; report the structured diagnostic (read-only inspection MAY be offered; no down-migration). Older-than-window → `unsupported_schema`, refuse with diagnostic, never coerce. Within window → migratable. Window size: `TBD — human ruling` (ASSUMPTION: while `schema_version` is `0.x`, a narrow window such as current-minus-one is adequate; this is a preview-era convenience, not a compatibility promise).
5. **Scope:** govern `model_json` now; carry the same record shape for the sibling JSON slots as they become editable (each already carries `schema_version`, §2.2). Confirmation of this scope is part of the ruling.

Rationale: Option B is the only option simultaneously consistent with the already-specified record shape and framework enum (§2.6), DEL-00-04 REQ-04-02 (§2.4), the per-document portability D-09 will need, and the existing hash-verification-on-open behavior (§2.7). Options A and D each contradict at least one accepted artifact; Option C's full friction is disproportionate during pre-1.0 schema churn but its evidence discipline is retained.

This recommendation is a `PROPOSAL` only. It confers no authority and changes no state.

---

## 6. Downstream impact map

| Surface | Impact of this ruling |
|---|---|
| **Phase A2 persistence tranche** | A2 is specified as "schema version + migration ledger entries (per D-08)" (`plans/PLAN_2026-06-10_prd_completion.md` §3 A2); A2 sits on the critical chain A1→A2→A5 (§5). The ruling defines A2's acceptance shape. |
| **FR-001 round-trip acceptance** | "Round-trips without loss" (`docs/PRD.md` §10 FR-001) must be restated as: lossless at constant version; across versions, migration is recorded, evidenced, and loss-free for required content (`schemas/project_persistence.schema.yaml` `RoundTripManifest`/`MigrationStatus`). |
| **Project-envelope hash verification** | Verify-on-open (§2.7) will flag any unevidenced payload rewrite as `mismatch_review_required`; the ruling's pre/post-hash ledger records are what keep migration and hash evidence mutually consistent (TP-MAC-81 series, DEL-02-05 `MEMORY.md`). |
| **D-09 container format** | Per-document semver + in-envelope migration records are container-portable; a store-level integer is not. D-09 (single-file container; `_REGISTER.md` row D-09) should inherit whichever record shape D-08 fixes. |
| **Validation preflight** | Closes `model_document_migration_status=model_document_migrations_not_defined_tbd` (`ProjectValidationPanel.tsx:467`) with derived statuses, mirroring how TP-MAC-80 closed the store-side TBD. |
| **DEL-00-04 / DEL-02-05 reviews** | Both deliverables are in CHECKING with "migration framework" as a declared TBD slot (§2.4, §2.5); the ruling resolves that slot for their formal review. No lifecycle state is changed by this packet. |

---

## 7. Authority and ruling record

Only the **human project authority** rules on D-08. Agents prepared this packet and may not certify, approve, or adopt it (`docs/CONTRACT.md` K-AUTH-1 analog per project invariants cited in DEL-00-04 `Specification.md` OPS-K-AUTH-1).

Per existing decision practice, the accepted ruling is recorded as a `DEC`/`SCA` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` (or its successor register), after which the dispatching persona updates `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-08 from `AWAITING_RULING` to `RULED` with a pointer (`_REGISTER.md` header, lines 1–15). This packet does not edit the register and does not resolve the decision.
