# Unit-normalized sparse comparison design

**Status:** `DESIGNED / NON-AUTHORITATIVE / NOT IMPLEMENTED`

This design replaces the ambiguous mixed-unit labels `solution-DOF units` and
`generalized-force units` with quantity-kind rows. It selects no public
numeric value and changes no sparse implementation or accepted DEC-050/053
record.

## Required raw observations

For every sparse/dense fixture and repeat solve, emit these maxima separately:

| row class | component mapping | raw delta unit |
| --- | --- | --- |
| solution translation | `ux`, `uy`, `uz` | `m` |
| solution rotation | `rx`, `ry`, `rz` | `rad` |
| equilibrium residual force | equations paired with `ux`, `uy`, `uz` | `N` |
| equilibrium residual moment | equations paired with `rx`, `ry`, `rz` | `N-m` |
| repeat-solution translation | `ux`, `uy`, `uz` | `m` |
| repeat-solution rotation | `rx`, `ry`, `rz` | `rad` |
| pivot sign | factorization pivot | exact `count` |

Each row must preserve fixture ID, DOF ID, component, dense reference, sparse
observation, repeat observation where applicable, raw delta, and unit. A
missing or non-finite member fails closed.

## Comparison algebra

For a continuous kind `k`, future owner-ruled members `rtol_k` and `atol_k`
would be applied componentwise and inclusively:

```text
abs(sparse_i - dense_i) <= max(atol_k, rtol_k * abs(dense_i))
```

The dimensionless normalized ratio is:

```text
q_i = abs(sparse_i - dense_i) / max(atol_k, rtol_k * abs(dense_i))
```

If both numerator and denominator are zero, define `q_i = 0`; if the
denominator is zero and the numerator is nonzero, define `q_i = +infinity`.
The fixture passes only when the maximum `q_i` for every required continuous
kind is at most one, repeat rows pass the same kind-specific predicate, and
the nonpositive-pivot count is exactly zero.

## Why current sparse evidence cannot be backfilled

The immutable nine-row DEC-053 observation stores one
`max_abs_sparse_dense_solution_delta` across translation and rotation DOFs,
and one `max_abs_sparse_residual` across force and moment equations. The raw
component vectors and winning component kinds are not recorded. The observed
aggregate maxima—`5.342535303043405e-10 solution-DOF units` and
`1.0058283805847168e-7 generalized-force units`—therefore cannot be converted
lawfully into `m`, `rad`, `N`, and `N-m` values.

The existing dimensionless relative parity maximum
`7.060341894958857e-11`, zero repeat deltas, and zero nonpositive pivots remain
valid committed observations. They do not prove admission under this new
unit-normalized design until a separately authorized instrument emits the raw
kind-separated rows.

## Follow-up boundary

A future implementation tranche would add observation fields and regenerate a
new derivative sparse capture; it must not rewrite the accepted DEC-053
snapshot. Numeric `rtol`/`atol` members remain owner-gated and `TBD` here.
