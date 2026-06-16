# WORKING ITEMS RUN - TP-UNITS-B2-IMPORTRT-001

Date: 2026-06-16
Agent: WORKING_ITEMS
Deliverable: DEL-02-05 Project persistence and round-trip serialization
Tranche: TP-UNITS-B2-IMPORTRT-001 local project unit round-trip evidence
Lifecycle state: CHECKING (unchanged)

## Scope

Add explicit unit metadata round-trip evidence to local project create/save/open
summaries so the desktop application can show and test that restored local
project envelopes preserve unit refs. This is a Phase B2 import/open/save unit
I/O slice selected from the completion plan residuals.

## Changes

- Extended `LocalProjectSummary` with:
  - `unit_round_trip_status`
  - `unit_round_trip_checked_ref_count`
  - `unit_round_trip_signature`
- Added browser-preview and Tauri SQLite summary collectors for explicit unit
  refs in:
  - `project.units`
  - material elastic/shear/thermal quantities
  - section quantity properties
  - pipe segment section quantities
  - primitive-load magnitudes
- Exposed the evidence in Project Storage Audit and Project Validation
  Preflight visible rows and JSON packets.
- Extended unit, app, backend, and e2e assertions for the new evidence.

## Validation

- `npm test --workspace apps/desktop -- projectService.test.ts App.test.tsx`
  - PASS: 2 files, 61 tests.
- `npm test --workspace apps/desktop`
  - PASS: 18 files, 386 tests.
- `npm run build --workspace apps/desktop`
  - PASS; existing Vite chunk-size warning only.
- `rustfmt apps/desktop/src-tauri/src/lib.rs --edition 2021`
  - PASS.
- `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check -p openpipestress-desktop`
  - PASS.
- `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml local_project_store_uses_sqlite_fts5_and_round_trips_model_snapshot --lib`
  - PASS: 1 test, 60 filtered.
- In-app Browser fallback at `http://127.0.0.1:5173/`
  - PASS: `New blank` -> `Save local` -> `Open local` showed
    `unit_metadata_preserved_in_local_project_envelope`, `checked_refs=6`,
    and the six blank-project unit refs in both `project-storage-unit-round-trip`
    and `project-validation-unit-round-trip`.
- DEC-025 sweep at committed tranche HEAD
  - PASS: cargo crate sweep, repository pytest, desktop Vitest, Playwright
    dev-server e2e 10/10, Playwright production-dist e2e 1/1, and desktop
    production build.

## Skipped / Blocked Evidence

- Direct pre-sweep `npx playwright test apps/desktop/e2e/r2-smoke.spec.ts`
  initially failed before executing assertions because the local Chromium
  executable was absent from the cache and a manual browser install timed out
  against the Playwright CDN. This was superseded by the passing DEC-025
  Playwright dev-server and production-dist lanes above.

## Boundary Review

This tranche changes persistence and validation evidence only. It does not add
unit conversion, target import compatibility, solver behavior, protected
standards content, private project data, network access, telemetry,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claims.

## Residuals

- B2 still owns broader app unit entry/pickers outside the named covered forms
  and non-PCF target-format conversion witnesses.
- B3 still owns broader mixed-unit round-trip, conversion-witness,
  incompatible-unit rejection, and D-04/DEC-026 tolerance corpus outside the
  named witnesses.
