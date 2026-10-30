# Torsional Shear Stress Fixture

Fixture ID: `STRESS-TORSIONAL-SHEAR-ORIGINAL`

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
| Torsional moment | 40.0 | N-m | moment |
| Torsion radius | 2.0 | m | length |
| Torsion constant | 80.0 | m^4 | second_moment_area |

## Expected Value

Torsional shear stress is supplied torque times supplied radius divided by
supplied torsion constant.

`40.0 * 2.0 / 80.0 = 1.0`

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `torsional_shear` | 1.0 | Pa | stress |

## Boundary

This fixture verifies a mechanics shear stress component only. It is not a code
allowable comparison, fatigue criterion, or professional approval.
