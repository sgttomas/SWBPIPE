# Specification: DEL-05-01 Primitive load case engine

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-05-01-DECL-001`.

## Scope

This deliverable specifies the implemented evidence boundary for the primitive load case engine under `core/loads/primitive_loads`. The slice defines code-neutral primitive load categories, storage-neutral primitive load-case records, boundary quantity metadata, deterministic mechanics preparation, equivalent-static mechanics handling, lumped equivalent nodal conversion, straight-pipe axial-effect helpers, solver load-vector assembly helpers, and diagnostic bridge records for later result-envelope transport.

This deliverable does not define code-specific load combinations, bundle coefficients or default magnitudes, set jurisdictional factors, implement rule-pack evaluation, implement final result-envelope/API integration, or claim engineering compliance. Mixed-category load-case algebra and user-defined combinations remain downstream DEL-05-02 scope.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-05-01-001 | The deliverable shall preserve primitive load category identity for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads. | SOW-013; DEL-05-01; `PrimitiveLoadCategory` |
| REQ-05-01-002 | Primitive load records shall remain compatible with the 3D centerline/frame mechanics analysis boundary through explicit node, element, and support targets. | OPS-K-MECH-1; `LoadTarget`; `LoadDirection` |
| REQ-05-01-003 | The primitive load engine shall not encode code-specific load combinations, allowables, factors, coefficients, procedure generators, default environmental values, or certification claims. | PKG-05 exclusion; OPS-K-DATA-1; OPS-K-IP-1; OPS-K-MECH-2 |
| REQ-05-01-004 | Unit-bearing load inputs shall preserve explicit dimension intent and reject invalid or retired boundary metadata where implemented. | OPS-K-UNIT-1; `LoadDimension`; `CanonicalDimension`; `BoundaryMetadataError` |
| REQ-05-01-005 | Missing or invalid solve-required load data shall become explicit findings/errors rather than silent defaults. | OPS-K-DATA-2; `FindingCode`; `BoundaryMetadataError` |
| REQ-05-01-006 | Load validation and load-case assembly findings shall be convertible to diagnostic records carrying code, class, blocking severity, source, affected object, message, remediation, and provenance reference. | AB-00-06; `LoadDiagnosticRecord` |
| REQ-05-01-007 | Primitive load-case records shall bind one primitive category to canonical model `LoadCase` metadata, provenance, payload reference, payload-hash reference, and sorted load IDs; mixed-category algebra remains DEL-05-02 scope. | SOW-013; SOW-014; DEL-05-02; `PrimitiveLoadCaseRecord`; `_DEPENDENCIES.md` |
| REQ-05-01-008 | Wind, seismic, and occasional equivalent-static handling shall accept only explicit equivalent mechanics loads with caller-supplied basis/provenance refs unless a later sealed scope authorizes dynamic procedure generation. | SOW-013 note; `_CONTEXT.md`; `README.md`; `EquivalentStaticMechanicsBasis` |
| REQ-05-01-009 | Deterministic primitive-load tests shall cover category representation, supported/unsupported target and dimension boundaries, boundary metadata, equivalent-static handling, load-case records, diagnostic bridge records, lumping, axial effects, and solver-vector assembly behavior before release use. | OPS-K-SOLVER-1; `src/lib.rs` tests |
| REQ-05-01-010 | Primitive load inputs, load-case records, boundary quantity records, and diagnostic records shall retain provenance references sufficient to distinguish explicit user/lawful source inputs from unresolved `TBD` state. | OPS-K-DATA-1; OPS-K-DATA-2; AB-00-06 |
| REQ-05-01-011 | Lumped equivalent nodal conversion shall remain limited to explicit translational/global `ForcePerLength` element loads with caller-supplied element span and node connectivity. | `prepare_lumped_nodal_loads`; `ElementLoadSpan` |
| REQ-05-01-012 | Straight-pipe axial-effect helpers shall remain mechanics-only helpers for thermal axial force and pressure thrust from caller-supplied properties and shall not introduce pressure stress formulas or material/default catalogs. | `prepare_straight_pipe_axial_effects`; PKG-05 exclusions |

## Standards

No external protected standard text is introduced by this deliverable. Governing local standards are the project invariant catalog, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification evidence / approach |
|---|---|
| REQ-05-01-001 | Unit tests and enum inspection confirm all eight primitive categories exist and map to primitive load-case kinds without invented coefficients. |
| REQ-05-01-002 | Unit tests cover node, element, and support target preparation for accepted mechanics boundaries. |
| REQ-05-01-003 | Documentation and crate boundary review confirm absence of code combinations, allowables, protected standard values, and professional/compliance claims. |
| REQ-05-01-004 | Unit tests cover canonical dimension parsing, `force_per_length`, dimension mismatch, and retired alias rejection. |
| REQ-05-01-005 | Unit tests cover missing targets, missing magnitudes, invalid targets, invalid dimensions, invalid directions, invalid spans/connectivity, invalid physical properties, non-finite inputs, and blocked outputs. |
| REQ-05-01-006 | Unit tests cover conversion from primitive-load findings and load-case assembly findings into diagnostic records with provenance. |
| REQ-05-01-007 | Unit tests cover load-case schema binding, schema load-type mapping, sorted load IDs, missing metadata, duplicate IDs, empty loads, and mixed-category rejection. |
| REQ-05-01-008 | Unit tests cover wind/seismic/occasional explicit equivalent mechanics loads, required equivalent-static basis/provenance refs, and rejection of acceleration/dynamic-placeholder dimensions. |
| REQ-05-01-009 | Current crate evidence is the focused 40-test primitive-load suite; broader release gates remain under project validation/review authority. |
| REQ-05-01-010 | Unit tests cover missing provenance rejection for boundary records and diagnostic records. |
| REQ-05-01-011 | Unit tests cover lumped nodal splitting, accepted translational directions, unsupported pressure/thermal/dynamic inputs, missing/invalid spans, invalid indices, and unchanged base `prepare_loads` behavior. |
| REQ-05-01-012 | Unit tests cover thermal and pressure axial force preparation, finding fan-in, missing/invalid properties, non-finite magnitude, and overflowing computed force rejection. |

## Remaining TBDs

- Canonical unit conversion constants and production unit conversion policy.
- Final result-envelope/API integration and application-service command/query surface.
- Production tolerance policy and release thresholds.
- Wind/seismic dynamic treatment, occasional-event mapping, and any future lawful procedure generators.
- Broader material/property sourcing policy, default prohibition enforcement, and human acceptance gates.
- Professional reliance and code-compliance acceptance remain human/project authority decisions, not crate behavior.

## Documentation

Required local deliverable artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `MEMORY.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`. Implementation evidence is in `core/loads/primitive_loads/README.md` and `core/loads/primitive_loads/src/lib.rs`.
