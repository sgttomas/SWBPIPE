---
doc_id: WORKING_ITEMS_RUN_2026-06-17_TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001
doc_kind: execution.run_record
status: completed
created: 2026-06-17
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001
smoke_id: TP-MAC-219
deliverable_id: DEL-09-05
package_id: PKG-09
---

# WORKING_ITEMS Run Record - TP-UNITS-BTAIL-VALIDATIONEVIDUNITS-001

## Scope

Bounded Phase B-tail validation-evidence unit-policy visibility slice while
C5.7 remains human-execution gated.

Primary deliverable context:

- `DEL-09-05` release quality gate checklist.
- Supporting unit context: `DEL-02-02` accepted DEC-018 unit-system basis.

## Changes

- Added `unit_policy_evidence` to the Validation Evidence export packet in
  `apps/desktop/src/features/validation-evidence/ValidationEvidencePanel.tsx`.
- Exposed the same evidence in the browser row
  `data-testid="validation-evidence-unit-policy"`.
- Extended App Vitest assertions to check the rendered line and exported JSON
  packet.
- Extended the focused R2 Playwright smoke path to check the browser-visible
  row in both desktop and compact projects.
- Recorded evidence in `apps/desktop/SMOKE.md` TP-MAC-219,
  `plans/PLAN_COMPLETION_LOG.md`, and the active completion plan.

## Unit Policy Evidence

The packet records:

- `unit-system:dec-018-si-dual-display`;
- `storage_convention=entered_units_preserved`;
- sorted project units from the invented preview model;
- `unit_bearing_record_count=18`;
- the manual section `unit_and_schema_verification`;
- `conversion_policy=validation_evidence_inventory_records_unit_context_without_conversion`;
- `conversion_performed=false`;
- `release_gate_threshold_policy=TBD`.

## Validation

Passed:

- `npm test --workspace apps/desktop -- src/App.test.tsx -t "renders the engineering workspace"` - 1/1 focused App test.
- `npm run test:e2e --workspace apps/desktop -- r2-smoke.spec.ts:548` - 2/2 focused Playwright tests.
- `npm test --workspace apps/desktop` - 18/18 files, 399/399 tests.
- `npm run build --workspace apps/desktop` - passed with the existing Vite large-chunk warning.

## Boundary Review

This tranche changed validation-evidence review metadata only. It did not
change release thresholds, release authorization, persistence behavior, solver
behavior, unit conversion APIs, DEC-018 catalog constants, protected standards
content, private payload handling, lifecycle state, release-readiness claims,
professional approval, certification, sealing, authentication, or
code-compliance claims.
