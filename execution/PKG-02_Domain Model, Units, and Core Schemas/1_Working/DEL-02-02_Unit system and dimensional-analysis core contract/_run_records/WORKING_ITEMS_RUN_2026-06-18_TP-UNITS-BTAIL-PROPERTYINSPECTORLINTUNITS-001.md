---
agent: WORKING_ITEMS
tranche_id: TP-UNITS-BTAIL-PROPERTYINSPECTORLINTUNITS-001
smoke_id: TP-MAC-257
date: 2026-06-18
supporting_deliverable: DEL-02-02
status: PASS
---

# WORKING_ITEMS Run - Property Inspector Report-Lint Unit Inventory

## Scope

Supporting unit-contract evidence. The Property Inspector already reports
unit-validation statuses for unit-bearing and dimensionless operation intents;
this tranche records that existing surface in the report-lint public unit
inventory.

## Evidence

- Report-lint public unit-policy target count increased from 39 to 40.
- The added target ref is
  `apps/desktop/src/features/model-tree/PropertyInspector.tsx` with
  `unit_policy_surface_id=property-inspector-unit-validation-surface`.
- Lint conversion remains false and target-format conversion-witness targets
  remain two.
- Focused/full desktop validation passed as recorded in the DEL-08-05 primary
  run record and `apps/desktop/SMOKE.md` TP-MAC-257.

## Boundary

No DEC-018 catalog constant, schema dimension enum, unit-conversion API,
operation schema, operation validation/application behavior, protected
standards content, private data, lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance claim changed.
