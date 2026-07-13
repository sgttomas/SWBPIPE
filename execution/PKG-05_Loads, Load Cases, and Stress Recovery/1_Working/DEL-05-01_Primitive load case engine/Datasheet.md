# Datasheet: DEL-05-01 Primitive load case engine

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-05-01-DECL-002`.

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-05-01 |
| Name | Primitive load case engine |
| Package | PKG-05 Loads, Load Cases, and Stress Recovery |
| Type | BACKEND_FEATURE_SLICE |
| Scope items | SOW-013 |
| Objectives | OBJ-003 |
| Context envelope | M |

## Evidence Summary

| Evidence surface | Current evidence |
|---|---|
| Implementation crate | `core/loads/primitive_loads` |
| Crate scope | Code-neutral primitive load records and deterministic mechanics-boundary preparation for weight, pressure, thermal, imposed displacement, hydrotest, wind, seismic, and occasional categories. |
| Readme evidence | `core/loads/primitive_loads/README.md` documents implemented categories, storage-neutral primitive load-case records, boundary quantity metadata, diagnostic bridge records, lumping, and exclusions. |
| Rust API evidence | `core/loads/primitive_loads/src/lib.rs` defines primitive category/type records, load quantities, target/direction records, boundary metadata, load-case records, diagnostic records, preparation helpers, lumping, axial-effect helpers, and solver-vector assembly helpers. |
| Local memory evidence | `MEMORY.md` records implementation, verification, and preserved boundaries from the original crate implementation through the 2026-06-05 foundational hardening tranche and parent fan-in. |

## Attributes

| Attribute | Current value | Source |
|---|---|---|
| Primary subject | Primitive load definitions and mechanics-boundary preparation for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional categories. | `_CONTEXT.md`; `core/loads/primitive_loads/README.md`; `src/lib.rs` |
| Analysis model boundary | Primitive loads are prepared for a 3D centerline/frame mechanics model through nodal, element-uniform, imposed-displacement, lumped-equivalent nodal, axial-effect, and solver-vector assembly surfaces. | `docs/CONTRACT.md` OPS-K-MECH-1; `src/lib.rs` |
| Mechanics/rule boundary | The crate defines mechanics load categories and deterministic findings; it does not evaluate user-rule acceptability, code compliance, professional judgment, code combinations, or allowables. | `docs/CONTRACT.md` OPS-K-MECH-2; `core/loads/primitive_loads/README.md` |
| Unit boundary | Load magnitudes carry explicit dimensions; boundary metadata maps accepted PKG-02 dimension vocabulary including `force_per_length` and rejects retired aliases. No conversion constants are supplied. | OPS-K-UNIT-1; `CanonicalDimension`; `QuantityUnitMetadata` |
| Boundary metadata | `BoundaryRecordRef`, `BoundaryQuantityRecord`, and `PrimitiveLoadCaseRecord` carry schema binding, target/basis refs, JCS payload refs, payload-hash refs, unit metadata, provenance refs, and canonical round-trip key material. | `src/lib.rs` |
| Load-case record boundary | `PrimitiveLoadCaseRecord` binds one primitive category to canonical model `LoadCase` metadata, sorted load IDs, provenance, payload refs, and hash refs. Mixed-category algebra remains downstream DEL-05-02 scope. | `PrimitiveLoadCaseRecord`; `_DEPENDENCIES.md` |
| Diagnostic bridge | `LoadDiagnosticRecord` maps primitive-load validation and load-case assembly findings to code, class, blocking severity, source, affected object, message, remediation, and provenance ref for later result-envelope transport. | `LoadDiagnosticRecord`; `diagnostic_records_from_load_findings`; `diagnostic_records_from_load_case_assembly_findings` |
| Missing values | Missing solve-required load targets, magnitudes, spans, properties, or provenance become explicit findings/errors rather than silent defaults. | OPS-K-DATA-2; `FindingCode`; `BoundaryMetadataError` |
| Dynamic scope | Wind, seismic, and occasional loads are represented only as explicit equivalent mechanics loads; equivalent-static preparation requires caller-supplied basis/provenance refs. Dynamic procedure generation, response spectra, code coefficients, and environmental defaults remain `TBD` and out of this deliverable. | `_CONTEXT.md`; `README.md`; tests in `src/lib.rs` |

## Primitive Load Category Register

| Category | Implemented mechanics evidence | Preserved boundary / open item |
|---|---|---|
| Weight | Element uniform `ForcePerLength` loads; eligible for lumped equivalent nodal conversion when caller supplies span/connectivity. | Mass-source, gravity-vector, density provenance, and unit-conversion policy remain `TBD`. |
| Pressure | Element pressure or equivalent line-load records; straight-pipe pressure thrust helper computes `p * internal_area` from caller-supplied properties. | Pressure stress formulas, code stress categories, pressure design rules, and property/default sourcing remain outside this deliverable. |
| Thermal | Element temperature-change records; straight-pipe thermal axial helper computes `E * A * alpha * DeltaT` from caller-supplied properties. | Reference temperature, material property provenance, expansion policy, and broader thermal behavior remain bounded to future sealed scopes. |
| Imposed displacement | Support-target imposed displacement records preserve translational displacement or rotational DOF boundary. | Support/restraint implementation and boundary-condition ownership remain interface-dependent. |
| Hydrotest | Element pressure or equivalent line-load category is represented in primitive mechanics preparation. | Hydrotest procedure defaults, test-fluid assumptions, and code requirements are not invented. |
| Wind | Explicit equivalent nodal force or element `ForcePerLength` mechanics loads are accepted. | Wind coefficients, profiles, exposure rules, and code factors are user/rule-supplied `TBD`. |
| Seismic | Explicit equivalent nodal force or element `ForcePerLength` mechanics loads are accepted. | Dynamic treatment, response parameters, seismic code procedure generation, and acceleration-style placeholders remain out of scope. |
| Occasional | Explicit equivalent nodal force/moment or element `ForcePerLength` mechanics loads are accepted. | Event definition and user/rule mapping remain downstream/user-governed. |

## Implemented Surfaces

| Surface | Current status |
|---|---|
| Primitive category model | Implemented by `PrimitiveLoadCategory` and `PrimitiveLoadCaseKind`. |
| Primitive load records | Implemented by `PrimitiveLoad`, `LoadTarget`, `LoadDirection`, and `LoadQuantity`. |
| Boundary quantity metadata | Implemented by `CanonicalDimension`, `QuantityUnitMetadata`, `BoundaryRecordRef`, and `BoundaryQuantityRecord`. |
| Primitive load-case records | Implemented by `PrimitiveLoadCaseRecord`; validates model `LoadCase` schema binding, non-empty refs, non-empty unique load IDs, and single-category membership. |
| Load preparation | Implemented by `prepare_loads` for nodal, element-uniform, and imposed-displacement contributions with deterministic sorting. |
| Equivalent-static preparation | Implemented by `EquivalentStaticMechanicsBasis` and `prepare_equivalent_static_loads` for wind, seismic, and occasional explicit mechanics loads with caller-supplied basis/provenance refs. |
| Lumped equivalent nodal conversion | Implemented by `prepare_lumped_nodal_loads` for explicit translational/global `ForcePerLength` element loads with caller-supplied spans/connectivity. |
| Axial effects | Implemented by `prepare_straight_pipe_axial_effects` for thermal axial force and pressure thrust from caller-supplied properties. |
| Solver load-vector assembly | Implemented by `assemble_solver_load_vector`; sorts/sums valid nodal contributions and returns findings instead of partial vectors when invalid data exists. |
| Diagnostic bridge | Implemented by diagnostic conversion helpers for primitive-load and load-case assembly findings. |

## Verification Evidence

| Evidence | Result |
|---|---|
| Historical run records | `MEMORY.md` records successful cargo formatting/testing tranches for primitive implementation, lumping, solver-vector assembly, axial effects, boundary dimension update, load-case records, and diagnostic bridge. |
| Current crate tests | `src/lib.rs` contains 40 tests for all primitive categories, boundary metadata, retired dimension aliases, equivalent-static handling, load-case records, deterministic sorting, lumping, axial effects, diagnostic records, missing/invalid inputs, and solver-vector assembly findings. |
| Current validation evidence | `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked` passed with 40 tests in the 2026-06-05 foundational hardening tranche; doc-alignment closeout should still run diff hygiene for touched files. |

## Conditions

- Primitive load categories remain separate from code-specific load combinations and allowables. Source: `_CONTEXT.md`; PKG-05 exclusions; SOW-014 note.
- The downstream load-case algebra/user-combination interface remains DEL-05-02. Source: `Specification.md` REQ-05-01-007; `_DEPENDENCIES.md`.
- The crate exposes storage-neutral boundary records and diagnostic records, not the final application-service result envelope/API.
- Unknown coefficients, default magnitudes, conversion constants, jurisdictional values, and professional acceptance remain `TBD`. Source: OPS-K-AGENT-1; OPS-K-DATA-1; OPS-K-IP-1.
- Agent-authored deliverable evidence remains draft/proposal material until accepted by the human workflow authority. Source: OPS-K-AGENT-4.

## References

- `_CONTEXT.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `MEMORY.md`
- `core/loads/primitive_loads/README.md`
- `core/loads/primitive_loads/src/lib.rs`
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7
- `docs/_Registers/Deliverables.csv` row DEL-05-01
- `docs/_Registers/ScopeLedger.csv` row SOW-013
- `docs/_Registers/ContextBudgetQA.csv` row DEL-05-01
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
