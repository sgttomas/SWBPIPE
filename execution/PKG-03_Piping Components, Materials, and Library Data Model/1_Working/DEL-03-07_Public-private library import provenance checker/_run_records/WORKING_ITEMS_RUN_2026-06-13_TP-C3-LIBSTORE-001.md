---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C3-LIBSTORE-001
timestamp: 2026-06-13T20:25:00-0600
completed: 2026-06-13T21:20:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-LIBSTORE-001 — local-only private-library persistence (Phase C3 store slice)

## Tranche and authority basis

- Tranche: completion-plan Phase C item **C3** (private library management GUI),
  third sub-slice — **local-only private-library persistence (CRUD)**. Selected
  per the `_COORDINATION.md` Application Integration And Issuance Loop step 3.1
  (earliest unblocked R3/Phase C dependency-spine item: C2 done → C3 → C4),
  continuing the C3 sequence after the foundation crate
  (`TP-C3-IMPORTVALIDATE-001`) and the validation seam (`TP-C3-IMPORTCMD-001`).
  Human-approved continuation of the C3 work. Mirrors the C2 store half
  (`TP-C2-RPLIFE-001`: `local_rule_packs` v10 migration + save/open/list/delete).
- Authority: **DEL-03-07** (Public/private library import provenance checker;
  CHECKING) for the import-boundary verdict the store honors; PRD §13
  (material/section/component libraries), §13.5 (import warnings), §12.4 /
  §14.6 (private-by-default library management), FR-022 (private libraries).
  IP boundary (DEL-01-02 / IP_AND_DATA_BOUNDARY.md: protected content
  quarantined and never used; private data never redistributed), OPS-K-PRIV-1
  (local-only), OPS-K-AUTH-1 (software findings only), DEC-019/DEC-028 store
  posture (SQLite store is a local projection, never committed).
- Regression gate (loop step 3.2): full desktop Vitest green at the start HEAD
  (`417e14c4f`, the TP-C3-IMPORTCMD-001 evidence commit) with the wasm engine
  prebuilt; src-tauri 40/40. No regression; this is new in-stage C3 scope.

## Scope boundary (what this slice is / is not)

In scope: the **store layer** — a `local_libraries` table (store v11 migration)
and `save_local_library` / `open_local_library` / `list_local_libraries` /
`delete_local_library` Tauri commands, with the matching typed frontend service
routes. **Not** in scope (the next C3 slices): the import-wizard GUI, a
workspace section / list panel, the §13.5 blocking-vs-advisory *display*, and
rule-pack ↔ library reference wiring. No file parsing — the store persists an
already-parsed, already-validated import document.

## Persistence policy (labeled design decision — surfaced for human review)

The private library store is **private-by-default and admits only accepted
imports.** `save_local_library` re-validates every document at the import
boundary with `intended_visibility = "private"` and stores it **only when the
verdict is accepted (`PRIVATE_LOCAL_ONLY`)**. A blocked (`REJECTED`) or
suspected-protected (`QUARANTINE`) import is **refused**: the command returns
`stored: false` with the full validation findings so the caller can surface
them, but nothing is written. The gate is the named helper
`library_import_is_storable`.

This is the conservative IP-boundary posture, derived from the authorities (the
DEL-03-07 checker's `accepted` verdict + the IP boundary's rule that suspected
protected content is quarantined and never used). It **differs deliberately from
the rule-pack store**, which keeps a user's own in-progress draft saveable: a
rule-pack draft is the user's private authoring with no external-IP risk,
whereas a library *import* carries external provenance/redistribution/
protected-content risk that the checker exists to gate. `PROPOSAL` for the human
project authority: if an audit trail of *refused* imports is wanted instead
(store-with-quarantine-flag), that is a small follow-up; the conservative
refuse-to-store default ships here because it cannot leak suspected-protected
content into storage. No human decision is blocked by shipping this default.

**Ruled 2026-06-13 (`DEC-036`):** the human project authority accepted the
implemented refuse-to-store default ("I accept your actions on how to handle
persistence of non-accepted imports. You can close out that matter
accordingly"). The store-with-quarantine-flag audit-trail variant is not
pursued unless a later ruling reopens it; this matter is closed.

## Changes

1. **`apps/desktop/src-tauri/src/lib.rs` — store v11 migration.**
   `STORE_SCHEMA_TARGET_VERSION` 10 → 11; new `store-v11-local-libraries-table`
   migration creates `local_libraries (project_id, library_kind, library_id,
   document_json, created_at_unix, updated_at_unix, PRIMARY KEY(project_id,
   library_kind, library_id))`. The migration framework is unchanged
   (idempotent, user_version-ledgered); the three migration-ledger evidence
   tests were updated for the new step (count 10 → 11, status strings v10 → v11).
2. **`apps/desktop/src-tauri/src/lib.rs` — persistence helpers + 4 commands.**
   `library_metadata_key` (kind → `<kind>_library`, rejects unsupported kinds),
   `library_id_from_document` (extracts `<kind>_library.library_id`, non-empty),
   `library_document_string` (name/privacy_class for the index),
   `library_import_is_storable` (the accept-gate), and
   `upsert_library`/`load_library`/`list_libraries`/`delete_library`. Commands
   `save_local_library` (validate → gate → store-or-refuse, returns
   `LocalLibrarySaveResult { stored, ... , validation }`), `open_local_library`
   (re-validates on read), `list_local_libraries`, `delete_local_library`. All
   registered in `generate_handler!`. The store re-uses the
   `library_import_validation_value` helper from the prior slice.
3. **`apps/desktop/src/services/libraryImportService.ts` — persistence routes.**
   `LocalLibraryIndexEntry` / `LocalLibraryEnvelope` / `LocalLibrarySaveResult`
   (with `stored: boolean` and nullable timestamps) / `LocalLibraryDeleteReceipt`
   types and `saveLocalLibrary` / `openLocalLibrary` / `listLocalLibraries` /
   `deleteLocalLibrary` routes, each with the shared browser-unavailable seam
   (refactored `validateLibraryImport` to share the `libraryImportUnavailable`
   helper).
4. **Tests.** Rust (3 new): `local_library_store_round_trips_accepted_import_per_project`
   (upsert/load/list/delete, per-project isolation, scoped delete leaves the
   other project's row); `local_library_store_gate_admits_accepted_and_refuses_quarantined_imports`
   (accept-gate true for `PRIVATE_LOCAL_ONLY`, false for `QUARANTINE`, with the
   `IMPORT_PROTECTED_CONTENT_SUSPECTED` code asserted);
   `library_id_extraction_requires_metadata_and_supported_kind`. Vitest (4 new):
   persistence browser-unavailable route; save invoke contract + `stored` flag;
   a refused (not-stored, null timestamps, blocking validation) pass-through;
   and the open/list/delete camelCase invoke contracts.
5. **`apps/desktop/SMOKE.md`** — TP-MAC-154 store-seam entry (no UI change; GUI
   evidence rides the later C3 wizard slice per the H4 posture).

## Validation

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` — **43/43**
  (40 prior + 3 new; 3 migration-ledger tests updated for v11). `cargo fmt
  --check` clean.
- `npm test` (desktop Vitest, wasm prebuilt) — **317/317** across 15 files
  (4 new). No regression.
- `npm run build` (desktop, `tsc -b && vite build`) — clean.
- DEC-025 five-surface evidence sweep at the committed HEAD; summary committed
  alongside this tranche.
- Evidence posture (H4): a store + service seam with **no user-visible desktop
  surface** in this slice (no component, no workspace section). The owed evidence
  is the Rust store/gate tests and the Vitest service tests (both green);
  Playwright/Vitest UI evidence rides the later C3 wizard slice, as
  `TP-C2-RPLIFE-001` recorded for the rule-pack store.
- Pre-existing src-tauri clippy debt (recorded in the TP-C3-IMPORTCMD-001 run
  record; never a gate) is unchanged by this slice and left untouched.

## Pre-commit adversarial review and dispositions

Self-review across four lenses (store correctness; IP-boundary/policy;
test non-vacuousness; scope):

- **Store correctness.** The PK `(project_id, library_kind, library_id)` keys
  per-project per-kind; the round-trip test proves upsert/load, per-project
  isolation, and that a scoped delete leaves a second project's row intact.
  `open`/`save` re-derive validation rather than trusting a stored verdict, so
  a future checker change re-gates stored documents on next open.
- **IP boundary / policy.** The accept-gate is asserted in Rust (quarantine →
  not storable, with the protected-content code present). Suspected-protected
  content cannot reach the table. The store is local SQLite only, never
  committed or transmitted; the documented policy divergence from the rule-pack
  store is principled and surfaced above for human review.
- **Test non-vacuousness.** The gate test asserts the specific QUARANTINE
  outcome and finding code (not just a boolean); the Vitest asserts exact
  `invoke` argument objects (camelCase contract) and the `stored`/null-timestamp
  pass-through, so a rename or a dropped field reddens a test.
- **Scope.** No GUI, no wizard, no §13.5 display, no rule-pack↔library wiring
  (hand-offs below). The accept-gate helper and the `stored` flag are the store
  layer's own contract, fully tested.

## Residuals and hand-offs (next Phase C / C3 work)

- **C3 UI:** import-wizard panel (file/payload → kind → visibility → validate via
  `validateLibraryImport` → §13.5 blocking-vs-advisory warnings via
  `partitionLibraryImportFindings` → confirm → `saveLocalLibrary`) + a workspace
  section + list panel (over `listLocalLibraries`) replicating the C2 RulePack
  manager pattern; boundary notice; the refused-save surfacing (`stored:false` +
  findings).
- **C3 wiring:** rule-pack ↔ library reference wiring.
- **C4:** end-to-end rule checks on authored models consume imported libraries.
- **Policy follow-up (PROPOSAL, human-gated):** if a stored audit trail of
  *refused* imports is wanted, add a quarantine-flag store path; the current
  default refuses-and-surfaces.
- Parity discipline (carried from `TP-C3-IMPORTVALIDATE-001`): the store re-uses
  the crate verbatim via the validation seam and adds no new validation
  semantics; the crate ↔ Python parity guard remains the cross-language anchor.
