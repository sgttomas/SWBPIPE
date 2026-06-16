# WORKING ITEMS RUN - TP-UNITS-B2-IMPORTRT-001

Date: 2026-06-16
Agent: WORKING_ITEMS
Deliverable: DEL-02-02 Unit system and dimensional-analysis core contract
Role: supporting evidence record for DEL-02-05 primary tranche
Lifecycle state: CHECKING (unchanged)

## Scope

Record the unit-system side of the local project unit round-trip evidence
slice. The primary implementation record is in DEL-02-05 because the changed
runtime surface is local project persistence.

## Evidence Summary

- Local project create/save/open summaries now expose deterministic unit
  metadata signatures:
  - `unit_round_trip_status`
  - `unit_round_trip_checked_ref_count`
  - `unit_round_trip_signature`
- The signature is collected from explicit stored unit refs in project units,
  material quantities, section quantities, pipe segment section quantities, and
  primitive-load magnitudes.
- Project Storage Audit and Project Validation Preflight display/export the
  same unit evidence.

## Validation

- Focused desktop Vitest: 61/61.
- Full desktop Vitest: 386/386.
- Desktop build: PASS, existing Vite chunk-size warning only.
- Rust formatting/package check: PASS.
- Focused Tauri local-project store regression: PASS.
- In-app Browser fallback: PASS for `New blank` -> `Save local` -> `Open local`
  visible unit round-trip rows.
- Direct pre-sweep Playwright execution initially failed before assertions
  because the Chromium binary was missing and browser install timed out against
  the Playwright CDN. This was superseded by the passing DEC-025 sweep:
  Playwright dev-server e2e 10/10 and production-dist e2e 1/1.

## Boundary Review

This supporting slice does not change DEC-018 unit catalog semantics, conversion
constants, solver normalization, report calculation behavior, protected content
policy, private-data policy, release-readiness status, professional approval,
certification, sealing, authentication, or code-compliance posture.

## Pointers

- Primary run record:
  `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-16_TP-UNITS-B2-IMPORTRT-001.md`
- Smoke entry: `apps/desktop/SMOKE.md` TP-MAC-173.
- Completion log: `plans/PLAN_COMPLETION_LOG.md`.
