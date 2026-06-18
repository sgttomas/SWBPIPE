---
run_id: WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-MATLIBFIELDUNITS-001
tranche_id: TP-UNITS-BTAIL-MATLIBFIELDUNITS-001
smoke_id: TP-MAC-197
agent: WORKING_ITEMS
primary_deliverable: DEL-07-03
supporting_deliverables:
  - DEL-03-01
  - DEL-02-02
status: passed
created: 2026-06-17
---

# TP-UNITS-BTAIL-MATLIBFIELDUNITS-001 — Material Library Property Unit Helper

## Scope

Bounded Phase B-tail app unit-entry tranche while C5.7 remains
human-execution gated. The selected gap was broader app unit entry/pickers:
material-library import drafts had no structured unit-bearing property helper.

Write scope:

- `apps/desktop/src/features/library/LibraryManagerPanel.tsx`
- `apps/desktop/src/features/library/LibraryManagerPanel.test.tsx`
- `apps/desktop/e2e/r2-smoke.spec.ts`
- `apps/desktop/SMOKE.md`
- this run record and the DEL-03-01/DEL-02-02 companion records
- deliverable `MEMORY.md` files for DEL-07-03, DEL-03-01, and DEL-02-02
- `plans/PLAN_COMPLETION_LOG.md`
- `plans/PLAN_2026-06-17_prd_completion.md`

## Changes

- Added a material-property unit helper to the Private Library Manager when
  the selected import kind is `material` and a draft exists.
- The helper can draft one private material `properties[]` quantity with
  explicit `magnitude`, `unit_ref`, `dimension_id`,
  `quantity_kind=unit_bearing`, `unit_required=true`, and
  `missing_unit_behavior=diagnostic_blocking`.
- Browser preview keeps the selected property default unit ref visible as
  model metadata and records that no fallback catalog is synthesized.
- Desktop/Tauri mode uses the DEC-018 catalog route and filters choices by
  compatible material-property dimension.
- The helper updates only the in-memory JSON draft. Existing validate/save
  actions still route through the local-only backend and DEC-036 refusal
  behavior.

## Validation

Passed:

- `npm test --workspace apps/desktop -- LibraryManagerPanel.test.tsx` — 13/13
  tests.
- `npm test --workspace apps/desktop` — 18/18 files, 395/395 tests.
- `npm run build --workspace apps/desktop` — passed with existing Vite
  large-chunk warning.
- `npm run test:e2e --workspace apps/desktop -- -g "library manager loads"` —
  2/2 Playwright tests.

## Boundary Review

No material engineering allowables, public material values, material schema
enum change, DEC-018 catalog constant change, validation/storage rule change,
protected standards content, private project payload, network/telemetry path,
lifecycle transition, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance claim changed.

## Residuals

Phase B-tail still has broader app unit-entry surfaces and target-format
conversion witnesses outside the already covered boundaries. C5.7 remains
human-execution gated.
