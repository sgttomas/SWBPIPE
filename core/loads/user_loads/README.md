# User Load Application

This crate is the bounded implementation slice for `DEL-05-05`. It prepares
explicit concentrated forces, concentrated moments, and distributed user loads
for the current frame-solver boundary.

## Scope

- Concentrated force inputs applied to node translational degrees of freedom.
- Concentrated moment inputs applied to node rotational degrees of freedom.
- Uniform distributed user-load inputs applied to element translational
  directions over an explicit normalized span.
- Model-load and result-hook boundary records that require explicit unit
  metadata, provenance reference, canonical schema binding, JCS payload
  reference, and payload-hash reference.
- Deterministic findings for missing targets, invalid dimensions, unsupported
  target/load combinations, non-finite inputs, invalid spans, and non-positive
  element lengths where a length is supplied.
- Traceable contribution records and result-recovery hooks for later stress
  recovery, reporting, export, GUI, and headless execution work.

## Boundary

This crate does not provide design-code load combinations, public default
factors, wind/seismic procedures, material or section defaults, protected
standards content, proprietary project data, rule-pack checks, result-envelope
integration, or professional/code-compliance claims. Inputs are explicit
mechanics quantities that upstream schema, unit, provenance, and solver
boundaries must already govern.

User-load input and contribution records bind to the canonical
`schemas/model.schema.yaml#/$defs/LoadRecord` surface. Recovery hooks bind to
model result values or `schemas/results.schema.yaml#/$defs/QuantityResult`.
The crate stores payload/hash references for deterministic round-trip handoff
but does not compute hashes, define conversion constants, or invent missing
unit metadata. Distributed `ForcePerLength` quantities carry the PKG-02
canonical `force_per_length` dimension.

## Verification

The unit tests cover concentrated force, concentrated moment, uniform
distributed load behavior, invalid/missing input findings, dimension checks,
deterministic contribution ordering, unit-bearing model-load boundary records,
result-recovery hook schema binding, and preservation of the
no-default/no-compliance boundary.
