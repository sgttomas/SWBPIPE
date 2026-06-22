# Assembled Multi-Support Three-DOF Acceptance Fixture

## Purpose

This note documents `NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL`,
an invented assembled nonlinear validation fixture that exercises one-way
release on `Ux`, gap closure on `Uy`, and friction sliding on `Uz` in one
frame solve. It is a narrow non-seed acceptance companion in the multi-support
validation fixture set.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Tip axial force | 10.0 | N | force |
| Tip transverse force | 1.0 | N | force |
| Tip vertical force | 6.0 | N | force |
| One-way support DOF | Ux | label | dimensionless |
| Gap support DOF | Uy | label | dimensionless |
| Friction support DOF | Uz | label | dimensionless |
| Friction coefficient | 0.30 | ratio | dimensionless |
| Explicit normal reaction | 10.0 | N | force |
| Friction limit | 3.0 | N | force |
| Gap clearance | 0.0002 | mm | length |
| Initial one-way state | active | label | dimensionless |
| Initial gap state | inactive | label | dimensionless |
| Initial friction state | sticking | label | dimensionless |
| Active-set policy reference | DEC-046-CV-B-multisupport-active-set-count-validation-v1 | label | dimensionless |
| Free-DOF force/moment policy reference | DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1 | label | dimensionless |
| Free-DOF work policy reference | DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1 | label | dimensionless |
| Displacement/reaction delta policy reference | DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1 | label | dimensionless |

## Expected Values

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| First-iteration changed support count | 3 | count | dimensionless |
| Final changed support count | 0 | count | dimensionless |
| Expected iteration count | 2 | count | dimensionless |
| Free-DOF force residual | 0.0 | N | force |
| Free-DOF moment residual | 0.0 | N-m | moment |
| Final free-DOF work residual | 0.0 | N-m | moment |
| Multi-support translation delta threshold | 100.0 | mm | displacement |
| Multi-support rotation delta threshold | 0.005 | rad | rotation |
| Multi-support force-reaction delta threshold | 10.0 | N | force |
| Multi-support moment-reaction delta threshold | 3.0 | N-m | moment |
| Final one-way state | inactive | label | dimensionless |
| Final gap state | active | label | dimensionless |
| Final friction state | sliding | label | dimensionless |
| Expected convergence flag | true | label | dimensionless |
| Expected diagnostic | none | label | dimensionless |

This fixture broadens the accepted multi-support / multi-DOF evidence set under
`DEC-046` from two-support Ux/Uy companions to a three-support, three-translation
companion. It preserves the same narrow active-set, free-DOF, and displacement/reaction delta threshold scope. It does not define general energy, sparse default,
product-preview, release, external validation, or CI thresholds.

Displacement/reaction delta policy: `DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1`.

Tolerance policy: `DEC-046-CV-B-multisupport-active-set-count-validation-v1`.
Free-DOF force/moment residual policy: `DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1`.
