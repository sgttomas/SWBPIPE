# Bending Normal Stress Fixture

Fixture ID: `STRESS-BENDING-NORMAL-ORIGINAL`

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
| Bending moment about local y | 50.0 | N-m | moment |
| Section modulus about local y | 25.0 | m^3 | section_modulus |
| Bending moment about local z | -30.0 | N-m | moment |
| Section modulus about local z | 15.0 | m^3 | section_modulus |

## Expected Values

Bending normal stress components are supplied bending moments divided by the
corresponding supplied section moduli.

`50.0 / 25.0 = 2.0`

`-30.0 / 15.0 = -2.0`

| Result | Value | Unit | Canonical dimension |
|---|---:|---|---|
| `bending_normal_y` | 2.0 | Pa | stress |
| `bending_normal_z` | -2.0 | Pa | stress |

## Boundary

The sign convention follows the current stress-recovery API inputs. This
fixture does not encode design-code stress categories, stress indices,
allowables, or professional approval.
