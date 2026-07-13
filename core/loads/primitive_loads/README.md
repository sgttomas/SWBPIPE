# Primitive Load Cases

This crate is the bounded implementation slice for `DEL-05-01`. It defines
code-neutral primitive load records and prepares deterministic mechanics
contributions for the current frame/support boundary.

## Scope

- Weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic,
  and occasional primitive load categories.
- Stable category and load-dimension metadata helpers for boundary records,
  load-case mapping, diagnostics, and tests.
- Storage-neutral primitive load-case records that bind one primitive category
  to canonical model `LoadCase` metadata, provenance, payload, and hash refs.
- Explicit unit/dimension intent for load magnitudes.
- Boundary quantity records that carry canonical schema binding, explicit unit
  metadata, provenance reference, JCS payload reference, and payload-hash
  reference for persistence/result-envelope handoff. `TBD` dimensions are
  rejected for concrete quantity metadata; unit conversion constants remain
  outside this crate.
- Deterministic findings for missing target references, invalid numeric input,
  missing load or source identifiers, invalid dimensions, and unsupported
  target/category combinations.
- Storage-neutral diagnostic records that map primitive-load validation and
  load-case assembly findings to code, class, severity, source, affected
  object, message, remediation, and provenance-reference fields for later
  result-envelope transport.
- Nodal force, element uniform load, and imposed-displacement contributions for
  later load-case algebra, stress recovery, GUI, and headless execution work.
- Lumped equivalent nodal conversion for explicit uniform `ForcePerLength`
  element loads when callers supply element span and node connectivity.
- Equivalent-static mechanics preparation for wind, seismic, and occasional
  loads only when callers supply an explicit basis/provenance reference.

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

Equivalent-static preparation is limited to wind, seismic, and occasional
primitive categories as explicit mechanics inputs. The helper validates the
caller-supplied basis/provenance reference and then reuses the primitive
mechanics validation path. It does not generate dynamic procedures, response
parameters, code factors, environmental defaults, or conversion constants.

Solver load-vector assembly sorts accepted contributions by node, global DOF,
and source ID, sums repeated node/DOF values deterministically, and returns no
partial vector when any contribution has a missing source ID, invalid node/DOF,
node/DOF mismatch, non-finite value, or non-finite assembled sum.

## Verification

The unit tests cover every primitive category, invalid/missing data findings,
diagnostic-record mapping, deterministic application behavior, boundary
metadata validation, retired dimension alias rejection, equivalent-static input
boundaries, lumped equivalent nodal conversion, deterministic solver load-vector
assembly, and preservation of frame/support DOF assumptions.

## D-41 R5 T7 PDU-054 current declaration

Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The primitive-load slice now includes explicit loads and the DEC-068 equivalent-static wind, seismic, and occasional generators. Dynamic loading and code-prescribed generation are outside this bounded slice unless separately implemented and evidenced.
