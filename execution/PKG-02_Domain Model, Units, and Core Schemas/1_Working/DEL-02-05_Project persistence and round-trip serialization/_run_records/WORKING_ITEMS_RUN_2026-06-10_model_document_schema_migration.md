# WORKING_ITEMS RUN — 2026-06-10 — Model-document schema migration seam (TP-APP-R2-PERSIST-001)

- **Tranche:** `TP-APP-R2-PERSIST-001` — completion plan Phase A2, unblocked
  this session by the human ruling on decision D-08 (`DEC-019` in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, accepting
  `execution/_Coordination/_DECISIONS/D-08_model_document_schema_migration.md`
  as proposed).
- **Run by:** WORKING_ITEMS (Type 1 persona) working locally inside the
  sealed app tranche; one parallel TASK worker prepared the D-10
  report-rendering decision packet in a disjoint write scope.
- **Authority basis:** DEC-019 policy; DEL-02-05 and DEL-00-04 contracts as
  mature design authority (`CHECKING`); the `MigrationStatus` record shape
  and framework enum already reserved by
  `schemas/project_persistence.schema.yaml`. No lifecycle change.

## What landed

1. **New application-service module**
   `apps/desktop/src-tauri/src/model_document_migration.rs` implementing the
   accepted policy: in-document semver `schema_version` is the sole
   model-document version authority (`PRAGMA user_version` stays DDL-only);
   ordered transform-chain registry (`source → target` with stable
   `migration_id`s) evaluated on open; migrate-in-memory-on-open with
   persist-on-save; `newer_than_supported` / `unsupported_schema` / `failed`
   documents refused for editing with structured diagnostics — no coercion,
   no down-migration, no invented historical versions (the published chain
   is empty because `0.1.0` is the only version ever published; chain
   mechanics are proven with injected test chains).
2. **Store schema v9** (`store-v9-model-document-migration-ledger-column`):
   evidence-only `model_migration_ledger_json` column; per-document ledger
   records carry source/target versions, applied migration ids, pre- and
   post-migration model hashes, framework, and a no-destructive-rewrite
   marker. Persisting an open-time-migrated document appends its record
   (`prepare_model_document_for_persist` in `lib.rs`, used by create and
   save).
3. **Open/create/save integration:** `open_local_project` evaluates the
   restored document and returns migration status + ledger in the envelope
   (or refuses); `create_local_project` / `save_local_project` evaluate
   before persisting and thread the open-time status back for ledger
   evidence.
4. **Frontend:** envelope types extended; browser-preview mirror evaluation
   in `projectService.ts` (status-only, no chain, refusals throw); App state
   threads `model_document_migration` + ledger through create/open/save and
   into the Project Validation Preflight, whose hardcoded
   `model_document_migrations_not_defined_tbd` marker is replaced by real
   DEC-019 evidence (new `project-validation-model-document-migration`
   line; `migration_scope` relabeled to
   `local_store_schema_ddl_only_model_document_schema_tracked_separately_per_dec_019`).

## Scope notes (visible limits)

- Backward-compatibility window size remains `TBD — human ruling` (DEC-019
  packet recommendation §4 note); with an empty chain every non-current
  older version currently refuses as `unsupported_schema`, which is the
  conservative posture.
- Sibling persisted JSON slots (`proposal_json`, `mechanics_result_json`,
  `analysis_run_json`, …) carry their own `schema_version` but are not yet
  governed by this seam; DEC-019 names extending the same record shape as
  they become editable.
- The staged "Migrate project" explicit service operation defined by
  DEL-02-05 remains a later implementation; this tranche delivers the
  open/save path.
- Read-only inspection of `newer_than_supported` documents (policy "MAY") is
  not yet offered; they refuse with diagnostics.

## Evidence

- `cargo test` `apps/desktop/src-tauri`: **24/24** (7 migration-module + 3
  persist-path tests new; store ledger tests now pin target version 9 and
  the v7→v9 staged path).
- `npm test --workspace apps/desktop`: **25/25** (new projectService
  browser-engine suite; R2 flow test asserts the new validation line after
  save).
- `npm run build --workspace apps/desktop`: pass.
- `python3 -m pytest -q tests`: **342/342** (no Python surfaces changed).
- Browser smoke: `apps/desktop/SMOKE.md` → **TP-MAC-83** (marker replaced;
  persistence-state transition observed across create/save/open; console
  clean).

## Boundary review

Local-only; stored bytes never rewritten as a side effect of open; no
destructive migration; no down-migration; refused documents are findings,
not silent coercions; ledger records carry no professional, certification,
sealing, authentication, approval, release, or code-compliance claims. Git
closeout is source-control hygiene only and is not lifecycle issuance.

## Affected surfaces

`apps/desktop/src-tauri/src/{lib.rs,model_document_migration.rs}`,
`apps/desktop/src/{App.tsx,App.test.tsx,types.ts}`,
`apps/desktop/src/services/{projectService.ts,projectService.test.ts}`,
`apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx`,
`apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx`,
`apps/desktop/SMOKE.md`, this run record, `MEMORY.md` (this deliverable),
plus session governance fan-in: DEC-018/DEC-019 ruling records, decision
register rows D-01/D-08 → RULED and D-10 → AWAITING_RULING, completion-plan
row updates.
