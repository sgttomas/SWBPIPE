# WORKING_ITEMS Run Record - TP-R4-D2-BRANCHSTRESS-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Deliverable: DEL-05-03 - Fundamental stress recovery module
Package: PKG-05 - Loads, Load Cases, and Stress Recovery
Tranche: TP-R4-D2-BRANCHSTRESS-001
Target stage: R4 / Phase D

## Scope

Landed the stress-recovery evidence side of the D2 branch connection
app-absorption slice under `DEC-045`. Branch components with complete
user-entered header/branch SIF values, user-entered flexibility factor, source
reference, and pipe-side mapping now emit side-specific
`component_user_stress_multiplier_review` rows at mapped pipe endpoints.

The base straight-pipe frame model, base pipe stress rows, and load-combination
algebra remain unchanged. Branch modifiers are applied only in review-labeled
stress-recovery rows under `mechanics_geometry_only`.

## Implemented Evidence

- Generalized component multiplier emission from bend-only to component-family
  / component-side metadata.
- Added branch-side selection: header SIF applies to the mapped header pipe and
  branch SIF applies to the mapped branch pipe.
- Added metadata basis fields for `component_family`, `component_side`,
  user-entered SIF/flexibility, source reference, and solver consumption.
- Regenerated the invented mechanics fixture to 749 result rows, including
  12 component multiplier evidence rows including combinations.

## Validation

- `cargo test --manifest-path core/product_physics/Cargo.toml` - passed 33/33
  unit tests.
- `npm test --workspace apps/desktop -- App.test.tsx` - passed 57/57 tests.
- `npm test --workspace apps/desktop` - passed 19/19 files, 406/406 tests.
- `npm run build:desktop` - passed; retained the existing Vite chunk-size
  warning.
- `npm run test:e2e:desktop` - passed 18/18 Playwright tests.

## Boundary

No protected standards values, code-derived branch factors, private data,
frame-stiffness behavior change, lifecycle state changes, release-readiness
claims, professional approval, certification, sealing, authentication, or
code-compliance claims were introduced.

## Residual

The branch connection stress-recovery review rows are landed for the invented
preview path. The PRD §16.2 branch-assembly benchmark remains D9 validation
evidence.

