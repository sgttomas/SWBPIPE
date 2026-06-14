---
run-id: WORKING_ITEMS_RUN_2026-06-13_TP-C3-IMPORTCMD-001
timestamp: 2026-06-13T19:35:00-0600
completed: 2026-06-13T20:15:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C3-IMPORTCMD-001 — library-import validation seam (Phase C3 command + service slice)

## Tranche and authority basis

- Tranche: completion-plan Phase C item **C3** (private library management GUI),
  second sub-slice — the **desktop seam** that exposes the C3 foundation crate
  through the app boundary. Selected per the `_COORDINATION.md` Application
  Integration And Issuance Loop step 3.1 (earliest unblocked item on the
  R3/Phase C dependency spine: C2 done → C3 → C4). The prior slice's run record
  (`TP-C3-IMPORTVALIDATE-001`) named this verbatim as its "C3-next (seam)"
  hand-off. This mirrors the C2 ordering (`TP-C2-RPLIFE-001` backend seam after
  the `rule_pack_document` crate) and the B2 units seam
  (`TP-UNITS-B2-CATALOGCMD-001` / `-FRONTENDSVC-001`).
- Authority: **DEL-03-07** (Public-private library import provenance checker;
  CHECKING). The runtime contract is the crate
  `open_pipe_stress_library_import_document` (landed `TP-C3-IMPORTVALIDATE-001`),
  itself the port of `core/library_import/provenance_checker.py`. PRD §13.5
  (import warnings → findings surface), §14.6 (library manager — later slice),
  FR-022 (private libraries). IP boundary (no protected tables/constants;
  private data never redistributed), OPS-K-AUTH-1 (software findings only),
  OPS-K-PRIV-1 (local-only).
- Regression gate (loop step 3.2): full desktop Vitest suite green at the start
  HEAD once the wasm engine was prebuilt (the suite needs
  `npm run build:wasm`; DEC-020). No regression; this is new in-stage C3 scope.

## Scope boundary (what this slice is / is not)

This slice is the **command + typed frontend service seam only**. In scope:
a `validate_library_import` Tauri command wrapping the crate, and a typed
`libraryImportService.ts` route with the explicit browser-unavailable seam.
**Not** in scope (the next C3 slices, hand-offs below): local private-library
persistence/CRUD, the import-wizard GUI, a workspace section/list panel, the
PRD §13.5 blocking-vs-advisory *display*, rule-pack ↔ library reference wiring.
No persistence and no file parsing here — validation is a pure function over an
already-parsed payload.

## Changes

1. **`apps/desktop/src-tauri/src/lib.rs`** — new `validate_library_import`
   `#[tauri::command]` and its `library_import_validation_value` helper. The
   helper calls the crate's `validate_library_import_tokens(payload,
   library_kind, intended_visibility)` — which returns `Err` for an unsupported
   `library_kind`/`intended_visibility` token (the seam rejects, never guesses)
   — and projects the `ImportValidationResult` into a stable JSON envelope:
   `outcome`, `library_kind`, `intended_visibility`, `accepted`,
   `has_blocking_findings` (severity `blocking`/`quarantine`), the ordered
   `findings`, the PKG-02 `diagnostics` projection, and a fixed
   `professional_boundary_notice` (software-findings-only language; no legal /
   certification / sealing / authentication / code-compliance determination).
   Command registered in the `generate_handler!` list. Crate imported as
   `library_import_document`.
2. **`apps/desktop/src-tauri/Cargo.toml`** — added the path dependency on
   `open_pipe_stress_library_import_document`.
3. **`apps/desktop/src/services/libraryImportService.ts` (new)** — typed seam:
   `LibraryKind` / `IntendedVisibility` / `LibraryImportFinding` /
   `LibraryImportValidation` mirrors; `validateLibraryImport(payload, kind,
   visibility)` returning a discriminated route (`tauri_backend` |
   `unavailable_browser_preview`) — browser preview returns an explicit
   `LIBRARY-IMPORT-BACKEND-DESKTOP-ONLY` diagnostic and never calls `invoke`,
   exactly like the units and rule-pack seams; and a pure
   `partitionLibraryImportFindings` projection that splits findings along the
   PRD §13.5 blocking-vs-advisory axis once, at the seam (blocking =
   `blocking`/`quarantine`; advisory = `review_required`), so the later UI slice
   does not re-derive severity semantics.
4. **`apps/desktop/src/services/libraryImportService.test.ts` (new)** — Vitest:
   browser-unavailable route (no `invoke`, diagnostic asserted); desktop route
   asserting the `invoke("validate_library_import", { payload, libraryKind,
   intendedVisibility })` camelCase contract and the validation shape;
   unsupported-token rejection propagated from a rejected `invoke`; and the
   §13.5 partition (ordered blocking vs advisory, and empty for an accepted
   result).
5. **Rust command tests (3, in `lib.rs`)** — accepted private payload
   (`PRIVATE_LOCAL_ONLY`, accepted, no blocking findings, boundary notice
   present, no compliance language); blocked empty payload (`REJECTED`,
   `has_blocking_findings`, `IMPORT_LIBRARY_METADATA_MISSING` +
   `IMPORT_RECORD_SET_MISSING` codes, PKG-02 `import_boundary` diagnostic
   present); unsupported `library_kind` and `intended_visibility` tokens both
   rejected with the offending token in the error.
6. **`apps/desktop/SMOKE.md`** — TP-MAC-153 backend-seam entry (no UI change;
   GUI evidence rides the later C3 wizard slice per the H4 posture).

## Validation

- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` — **40/40**
  (37 prior + 3 new library-import command tests). `cargo fmt --check` clean.
- `npm test` (desktop Vitest, wasm prebuilt) — **313/313** across 15 files
  (4 new in `libraryImportService.test.ts`). No regression.
- `npm run build` (desktop, `tsc -b && vite build`) — clean (the >500 kB
  chunk-size note is pre-existing and informational).
- DEC-025 five-surface evidence sweep run at the committed HEAD; summary
  committed alongside this tranche. The sweep's cargo surface runs
  `cargo test` per crate (40/40 here).
- Evidence posture (H4): a backend command + typed service seam with **no
  user-visible desktop surface** in this slice (no component, no workspace
  section). The owed evidence is the Rust command tests and the Vitest service
  test (both green); Playwright/Vitest UI evidence rides the later C3 wizard
  slice, exactly as `TP-C2-RPLIFE-001` recorded for the rule-pack backend seam.
- Pre-existing clippy debt (recorded, out of tranche scope): `cargo clippy
  --all-targets -D warnings` on src-tauri reports 10 errors on the clean HEAD
  before this tranche (`too_many_arguments`, an `assert_eq!` literal-bool, and
  `len_zero` nits in pre-existing tests). `clippy -D warnings` is not part of
  the established gate (the DEC-025 sweep runs `cargo test`, not clippy), so
  these were never green; this slice adds none of them (its functions take 3
  args and its asserts compare `Value`s, not bools or `len()`). Left untouched
  per the entry-prompt rule against unrelated hardening; flagged here for a
  future hardening-lane sweep if the project later gates clippy.

## Pre-commit adversarial review and dispositions

Self-review across four lenses (seam contract fidelity; service/test
non-vacuousness; boundary/IP; scope discipline):

- **Contract fidelity.** The JS↔Rust argument contract (`payload`,
  `libraryKind`/`library_kind`, `intendedVisibility`/`intended_visibility`) is
  the Tauri v2 camelCase↔snake_case mapping already proven by
  `validate_rule_pack`'s `publicExportRequested`; the Vitest asserts the exact
  `invoke` call shape, and the Rust tests assert the envelope keys, so a rename
  on either side reddens a test. `has_blocking_findings` and the partition
  helper agree on the severity set (`blocking`/`quarantine` block;
  `review_required` advises) — pinned on both sides.
- **Non-vacuousness.** The blocked-payload Rust test asserts specific finding
  **codes** and the PKG-02 diagnostic `class`, not just rejection; the partition
  Vitest asserts ordered code lists, so a misclassification reddens it.
- **Boundary.** The command emits software findings only; the boundary notice
  is asserted to contain "software findings only" and to **not** contain
  "certified". No persistence, no file parsing, no network, no protected/private
  data. No lifecycle change; DEL-03-07 stays CHECKING.
- **Scope.** Persistence, wizard GUI, workspace section, §13.5 display, and
  rule-pack↔library wiring are deliberately deferred (hand-offs below). The
  partition helper is a pure data projection of the crate's own severity field
  (not invented display), kept because it is the typed seam's natural shape and
  is fully tested.

## Residuals and hand-offs (next Phase C / C3 work)

- **C3 persistence:** local-only private-library store (CRUD) for imported
  material/section/component libraries — private-by-default, never committed —
  mirroring the C2 `local_rule_packs` v10 store migration and save/open/list/
  delete commands.
- **C3 UI:** import-wizard panel (file/payload → kind → visibility → validate →
  §13.5 blocking-vs-advisory warnings via `partitionLibraryImportFindings` →
  confirm) + a workspace section + list panel, replicating the C2 RulePack
  manager pattern; boundary notice; rule-pack ↔ library reference wiring.
- **C4:** end-to-end rule checks on authored models consume imported libraries.
- Parity discipline (carried from `TP-C3-IMPORTVALIDATE-001`): if
  `provenance_checker.py` or the shared fixtures change, the crate's
  `tests/provenance_parity.rs` must move with them; this seam re-exposes the
  crate verbatim and adds no new validation semantics.
- Pre-existing src-tauri clippy debt (above) for a future clippy-gating sweep.
