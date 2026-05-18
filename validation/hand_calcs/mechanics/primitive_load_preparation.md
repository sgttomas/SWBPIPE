# MECH-PRIMITIVE-LOAD-PREP

## Purpose

Primitive-load benchmark for accumulated nodal mechanics loads, an element
uniform weight contribution, and an imposed displacement contribution.

## Provenance

- Source: OpenPipeStress original mechanics benchmark.
- Redistribution: project-original-public-content.
- Contributor certification: generated from elementary open mechanics, not
  copied from protected standards, commercial software examples, or proprietary
  data.

## Invented Inputs

| Item | Value | Unit | Canonical dimension |
|---|---:|---|---|
| Node `1`, global `Y` load A | 8.0 | N | force |
| Node `1`, global `Y` load B | -3.0 | N | force |
| Element `0`, global `Z` weight | 1.25 | N/m | TBD |
| Node `2`, `Uz` imposed displacement | -0.02 | m | length |

## Expected Values

Accumulated nodal force at node `1`, global `Y`:

```text
F_y = 8.0 + (-3.0)
    = 5.0
```

The element weight and imposed displacement remain separate solver-boundary
contributions:

```text
uniform_weight_force_per_length = 1.25
imposed_uz_displacement = -0.02
```

Tolerance policy: `TBD`.
