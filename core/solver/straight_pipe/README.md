# Straight Pipe Element

This crate is the bounded implementation slice for `DEL-04-02`. It adapts explicit straight-pipe section and mass properties into the existing 3D frame kernel, exposes weight hooks for later load-case work, and recovers local mechanical element forces.

## Scope

- Straight two-node pipe element construction.
- Explicit section-property integration through caller-supplied area, second moments, and torsion constant.
- Explicit mass-per-length and weight-per-length hooks.
- Local element displacement and force recovery using the frame-kernel stiffness and orientation conventions.
- Direct local end-resultant recovery for pipe end `I` or `J` using the
  solver force-vector components without sign reinterpretation or stress-rule
  classification.

## Boundary

This crate does not provide pipe dimension tables, material defaults, code-specific values, stress checks, load combinations, or professional/code-compliance claims. Inputs are mechanics quantities that upstream schemas, unit contracts, provenance checks, and section-property calculations must already govern.

The external solver-facing boundary is represented by `StraightPipeBoundaryMetadata`. It binds the straight-pipe numeric fields to a caller-supplied frame-kernel unit basis, mass-per-length and gravity unit metadata, a `TBD` dimension marker for force-per-length until PKG-02 accepts a more specific vocabulary term, and canonical analytical/source model references. The metadata layer records identifiers only; it does not convert units or supply engineering defaults.

## Verification

The unit tests cover section-property validation, frame-kernel stiffness integration, weight hooks, axial local force recovery, direct end-resultant recovery, transverse bending recovery, and displacement-length validation.
