# Assembled Multi-Support Cascade Gap/Lift-Off Acceptance Fixture

## Purpose

This note documents
`NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`, an invented
assembled nonlinear validation fixture that exercises a sequential active-set
cascade in one frame solve. A rotational lift-off support on `Rz` releases in
the first iteration, which changes the next transverse `Uy` gap classification;
the third iteration has no support-state changes. It is a narrow non-seed
acceptance companion in the multi-support validation fixture set.

## Provenance

- Source: OpenPipeStress original nonlinear support regression fixture.
- Redistribution: project-original-public-content.
- Contributor assertion: generated from invented support states; not copied
  from protected standards, commercial software examples, proprietary data,
  private data, or real project records.

## Invented Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Tip transverse force | 1.0 | N | force |
| Gap support DOF | Uy | label | dimensionless |
| Lift-off support DOF | Rz | label | dimensionless |
| Gap clearance | 0.002 | mm | length |
| Initial gap state | inactive | label | dimensionless |
| Initial lift-off state | active | label | dimensionless |
| First-iteration changed support | Rz lift-off | label | dimensionless |
| Second-iteration changed support | Uy gap | label | dimensionless |
| Active-set policy reference | DEC-046-CV-B-multisupport-active-set-count-validation-v1 | label | dimensionless |
| Free-DOF force/moment policy reference | DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1 | label | dimensionless |
| Free-DOF work policy reference | DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1 | label | dimensionless |
| Displacement/reaction delta policy reference | DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1 | label | dimensionless |

## Expected Values

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| First-iteration changed support count | 1 | count | dimensionless |
| Second-iteration changed support count | 1 | count | dimensionless |
| Final changed support count | 0 | count | dimensionless |
| Expected iteration count | 3 | count | dimensionless |
| Free-DOF force residual | 0.0 | N | force |
| Free-DOF moment residual | 0.0 | N-m | moment |
| Final free-DOF work residual | 0.0 | N-m | moment |
| Multi-support translation delta threshold | 100.0 | mm | displacement |
| Multi-support rotation delta threshold | 0.005 | rad | rotation |
| Multi-support force-reaction delta threshold | 10.0 | N | force |
| Multi-support moment-reaction delta threshold | 3.0 | N-m | moment |
| Final gap state | active | label | dimensionless |
| Final lift-off state | inactive | label | dimensionless |
| Expected convergence flag | true | label | dimensionless |
| Expected diagnostic | none | label | dimensionless |

This fixture broadens the accepted multi-support / multi-DOF evidence set under
`DEC-046` to a measured sequential active-set cascade within the existing
max-iteration cap. The companion also carries the fixture-set displacement/reaction delta threshold policy as an observed-envelope record. It does not define general energy, sparse default, product-preview, release, external validation,
or CI thresholds.

Displacement/reaction delta policy: `DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1`.

Tolerance policy: `DEC-046-CV-B-multisupport-active-set-count-validation-v1`.
Free-DOF force/moment residual policy: `DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1`.
