# Mechanics Stress Range Fixture

Fixture ID: `STRESS-RANGE-MECHANICS-ORIGINAL`

## Provenance

- Source: OpenPipeStress original stress recovery benchmark.
- License basis: project-original-public-content.
- Contributor: OpenPipeStress agentic development workflow.
- Certification: generated from elementary open mechanics; not copied from
  protected standards, code formulas, commercial software examples, or
  proprietary data.

## Inputs

The fixture compares two invented mechanics states using the same section
properties. Pressure inputs are omitted for this mechanics range case; omitted
pressure in both states produces no pressure range component.

| Quantity | State A | State B | Unit | Canonical dimension |
|---|---:|---:|---|---|
| Axial force | 60.0 | 180.0 | N | force |
| Bending moment about local y | -20.0 | 80.0 | N-m | moment |
| Bending moment about local z | 10.0 | 10.0 | N-m | moment |
| Torsional moment | 20.0 | 60.0 | N-m | moment |
| Area | 12.0 | 12.0 | m^2 | area |
| Section modulus about local y | 25.0 | 25.0 | m^3 | section_modulus |
| Section modulus about local z | 15.0 | 15.0 | m^3 | section_modulus |
| Torsion radius | 2.0 | 2.0 | m | length |
| Torsion constant | 80.0 | 80.0 | m^4 | second_moment_area |

## Expected Values

Stress range is the absolute difference between recovered mechanics components.
This is a component-by-component mechanics delta, not an equivalent stress,
fatigue equation, code stress range, or allowable comparison.

Axial range:

`abs((180.0 / 12.0) - (60.0 / 12.0)) = 10.0`

Bending-y range:

`abs((80.0 / 25.0) - (-20.0 / 25.0)) = 4.0`

Torsional shear range:

`abs((60.0 * 2.0 / 80.0) - (20.0 * 2.0 / 80.0)) = 1.0`

Bending-z range:

`abs((10.0 / 15.0) - (10.0 / 15.0)) = 0.0`

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `axial_normal_range` | 10.0 | Pa | stress |
| `bending_normal_y_range` | 4.0 | Pa | stress |
| `bending_normal_z_range` | 0.0 | Pa | stress |
| `torsional_shear_range` | 1.0 | Pa | stress |
| `pressure_hoop_range` | omitted | Pa | stress |
| `pressure_longitudinal_range` | omitted | Pa | stress |

## Boundary

This is a mechanics-only range comparison. It is not a fatigue assessment,
allowable comparison, design-code check, or professional approval.
