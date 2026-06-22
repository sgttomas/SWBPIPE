# Assembled Multi-Support Multi-DOF Observation Fixture

## Purpose

This note documents `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL`, an
invented assembled nonlinear depth fixture that exercises two nonlinear
supports on two translational tip DOFs in one frame solve. It is observation
evidence only and is intentionally outside the accepted current assembled
validation seed.

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
| One-way support DOF | Ux | label | dimensionless |
| Gap support DOF | Uy | label | dimensionless |
| Gap clearance | 0.0002 | mm | length |
| Initial one-way state | active | label | dimensionless |
| Initial gap state | inactive | label | dimensionless |
| Observation policy reference | TP-R4-D9-MULTISUPPORT-OBS-TBD | label | dimensionless |

## Expected Values

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| First-iteration changed support count | 2 | count | dimensionless |
| Final changed support count | 0 | count | dimensionless |
| Expected iteration count | 2 | count | dimensionless |
| Final one-way state | inactive | label | dimensionless |
| Final gap state | active | label | dimensionless |
| Expected convergence flag | true | label | dimensionless |
| Expected diagnostic | TOLERANCE_POLICY_TBD | label | dimensionless |

This fixture proves that the assembled loop can carry simultaneous nonlinear
state changes on more than one support and more than one translational DOF. It
does not promote non-seed force, displacement, reaction-delta, energy, sparse
default, external validation, release, or CI thresholds.

Tolerance policy: `TBD`.
