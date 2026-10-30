# Axial Normal Stress Fixture

Fixture ID: `STRESS-AXIAL-NORMAL-ORIGINAL`

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- License basis: project-original-public-content.
- Contributor: OpenPipeStress agentic development workflow.
- Certification: generated from elementary open mechanics; not copied from
  protected standards, code formulas, commercial software examples, or
  proprietary data.

## Inputs

| Quantity | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Axial force | 120.0 | N | force |
| Area | 12.0 | m^2 | area |

## Expected Value

Axial normal stress is the supplied axial force divided by supplied area.

`120.0 / 12.0 = 10.0`

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `axial_normal` | 10.0 | Pa | stress |

## Boundary

This fixture verifies mechanics stress recovery only. It is not an allowable
comparison, code stress category, fatigue check, or professional approval.
