# Primitive Load Cases

This crate is the bounded implementation slice for `DEL-05-01`. It defines
code-neutral primitive load records and prepares deterministic mechanics
contributions for the current frame/support boundary.

## Scope

- Weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic,
  and occasional primitive load categories.
- Storage-neutral primitive load-case records that bind one primitive category
  to canonical model `LoadCase` metadata, provenance, payload, and hash refs.
- Explicit unit/dimension intent for load magnitudes.
- Boundary quantity records that carry canonical schema binding, explicit unit
  metadata, provenance reference, JCS payload reference, and payload-hash
  reference for persistence/result-envelope handoff.
- Deterministic findings for missing target references, invalid numeric input,
  invalid dimensions, and unsupported target/category combinations.
- Storage-neutral diagnostic records that map primitive-load validation and
  load-case assembly findings to code, class, severity, source, affected
  object, message, remediation, and provenance-reference fields for later
  result-envelope transport.
- Nodal force, element uniform load, and imposed-displacement contributions for
  later load-case algebra, stress recovery, GUI, and headless execution work.
- Lumped equivalent nodal conversion for explicit uniform `ForcePerLength`
  element loads when callers supply element span and node connectivity.

## Boundary

This crate does not provide protected code load combinations, design-code load
categories, wind/seismic procedures, pressure stress formulas, material or
section defaults, catalog data, rule-pack checks, result envelopes, or
professional/code-compliance claims. Inputs are explicit mechanics quantities
that upstream schema, unit, provenance, and solver boundaries must already
govern.

Boundary records bind to `schemas/model.schema.yaml#/$defs/LoadCase`,
`schemas/model.schema.yaml#/$defs/LoadRecord`,
`schemas/model.schema.yaml#/$defs/Result/values`, or
`schemas/results.schema.yaml#/$defs/QuantityResult` by explicit enum. Unit
metadata uses the accepted PKG-02 dimension vocabulary and rejects retired
aliases such as `temperature_difference`, `area_moment`, and generic
`stiffness`. No unit conversion constants are supplied by this crate.
Primitive load-case records enforce one primitive category per record; mixed
case algebra remains downstream `DEL-05-02` scope.
Diagnostic records preserve the local finding code and use local primitive-load
classes; they do not introduce a shared diagnostic enum or final result
envelope/API behavior.

The lumped equivalent nodal conversion is limited to translational/global
`ForcePerLength` element loads. Pressure, thermal, rotational, acceleration or
dynamic-style inputs, missing span/connectivity, invalid indices, and invalid
spans are reported as findings or construction errors rather than inferred.
`ForcePerLength` boundary metadata maps to the PKG-02 canonical
`force_per_length` dimension.

## Verification

The unit tests cover every primitive category, invalid/missing data findings,
diagnostic-record mapping, deterministic application behavior, boundary
metadata validation, retired dimension alias rejection, lumped equivalent nodal
conversion, and preservation of frame/support DOF assumptions.
