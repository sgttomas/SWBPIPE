# Stress Recovery

This crate is the bounded implementation slice for `DEL-05-03`. It recovers
code-neutral mechanics stresses from explicit element force resultants, section
properties, and optional pressure basis inputs.

## Scope

- Axial normal stress from supplied axial force and area.
- Bending normal stress components from supplied bending moments and section
  moduli.
- Torsional shear stress from supplied torque, torsion radius, and torsion
  constant.
- Thin-wall pressure membrane components from explicit pressure, radius, and
  wall thickness inputs.
- Mechanics-only component stress ranges from two recovered stress states.
- Neutral construction of force resultants from solver element-end resultants,
  with finite-input validation and no code stress interpretation.
- Input unit metadata validation for force resultants, section properties, and
  pressure-basis quantities using the accepted PKG-02 canonical dimension
  vocabulary.
- Recovered stress boundary records that bind explicit stress-unit metadata to
  the canonical result/result-envelope schema surface with payload and
  payload-hash references.
- Deterministic findings for missing inputs, non-finite values, non-positive
  properties, missing or incompatible unit metadata, non-finite recovered
  outputs, incomplete range states, incomplete status, and status-boundary
  violations.

## Boundary

This crate does not implement design-code stress equations, allowables, stress
indices, SIF/flexibility tables, rule-pack checks, protected standards content,
pipe tables, unit conversion constants, report rendering, GUI behavior, local
FEA handoff, or professional/code-compliance claims.

Inputs are already parsed mechanics quantities from governed solver, section,
load, unit, and status boundaries. Unknown conversion, result-envelope,
persistence, tolerance, rule-check, and benchmark-publication details remain
`TBD`.

The `recover_stresses_with_unit_metadata` boundary checks require unit and
unit-system references for every present stress input quantity before recovery.
Recovered component records can then bind to the canonical model result value
or schema-first result-envelope quantity surface. This crate records payload
and hash references for deterministic handoff; it does not compute hashes,
choose conversion constants, set production tolerances, or make compliance
claims.

## Verification

The unit tests cover axial, bending, torsion, pressure membrane components,
combined normal/shear summaries, mechanics-only stress ranges, element-end
resultant input construction, missing inputs, invalid numeric values, unit
metadata validation, result boundary records, non-finite recovered values, and
analysis-status propagation without human approval or code-compliance claims.
