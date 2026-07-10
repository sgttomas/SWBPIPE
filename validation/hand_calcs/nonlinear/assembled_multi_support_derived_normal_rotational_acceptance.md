# Assembled Multi-Support Derived-Normal Friction/Rotational Acceptance Fixture

## Purpose

This note documents
`NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL`, an
invented assembled nonlinear validation fixture that exercises derived-normal
friction sliding on `Ux` and rotational lift-off release on `Rz` in one frame
solve. It is a narrow non-seed acceptance companion in the multi-support
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
| Normal-source force | -100.0 | N | force |
| Tip rotational moment | 2.0 | N-m | moment |
| Friction support DOF | Ux | label | dimensionless |
| Lift-off support DOF | Rz | label | dimensionless |
| Normal source support DOF | Uy | label | dimensionless |
| Friction coefficient | 0.06 | ratio | dimensionless |
| Derived normal reaction | 100.0 | N | force |
| Friction limit | 6.0 | N | force |
| Initial friction state | sticking | label | dimensionless |
| Initial lift-off state | active | label | dimensionless |
| Active-set policy reference | DEC-046-CV-B-multisupport-active-set-count-validation-v1 | label | dimensionless |
| Free-DOF force/moment policy reference | DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1 | label | dimensionless |
| Free-DOF work policy reference | DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1 | label | dimensionless |
| Displacement/reaction delta policy reference | DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1 | label | dimensionless |

## Expected Values

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| First-iteration changed support count | 2 | count | dimensionless |
| Final changed support count | 0 | count | dimensionless |
| Expected iteration count | 2 | count | dimensionless |
| Free-DOF force residual | 0.0 | N | force |
| Free-DOF moment residual | 0.0 | N-m | moment |
| Final free-DOF work residual | 0.0 | N-m | moment |
| Multi-support translation delta threshold | 100.0 | mm | displacement |
| Multi-support rotation delta threshold | 0.005 | rad | rotation |
| Multi-support force-reaction delta threshold | 10.0 | N | force |
| Multi-support moment-reaction delta threshold | 3.0 | N-m | moment |
| Final friction state | sliding | label | dimensionless |
| Final lift-off state | inactive | label | dimensionless |
| Expected convergence flag | true | label | dimensionless |
| Expected diagnostic | none | label | dimensionless |

This fixture broadens the accepted multi-support / multi-DOF evidence set under
`DEC-046` to a combined derived-normal friction and rotational lift-off
transition. The companion also carries the fixture-set general-energy residual
policy and displacement/reaction delta threshold policy as observed-envelope
records. It does not define sparse default, product-preview, release, external
validation, total strain-energy, modal-energy, or CI thresholds.

Displacement/reaction delta policy: `DEC-046-CV-B-multisupport-displacement-reaction-delta-threshold-validation-v1`.

Tolerance policy: `DEC-046-CV-B-multisupport-active-set-count-validation-v1`.
Free-DOF force/moment residual policy: `DEC-046-CV-B-multisupport-free-dof-force-moment-residual-validation-v1`.
Free-DOF work residual policy: `DEC-046-CV-B-multisupport-free-dof-work-residual-validation-v1`.
