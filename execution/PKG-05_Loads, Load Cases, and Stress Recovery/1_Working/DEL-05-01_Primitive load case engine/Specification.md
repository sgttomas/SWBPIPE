# Specification: DEL-05-01 Primitive load case engine

## Scope

This deliverable specifies the setup boundary for primitive load definitions covering weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional categories. It does not implement a load engine, define code-specific load combinations, bundle coefficients or default magnitudes, set jurisdictional factors, or claim engineering compliance.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-05-01-001 | The deliverable shall preserve primitive load category definitions for weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional loads. | SOW-013; DEL-05-01 |
| REQ-05-01-002 | Primitive load definitions shall remain compatible with the 3D centerline/frame mechanics analysis boundary. | OPS-K-MECH-1 |
| REQ-05-01-003 | The load case engine setup shall not encode code-specific load combinations, allowables, factors, coefficients, or certification claims. | PKG-05 exclusion; OPS-K-DATA-1; OPS-K-IP-1; OPS-K-MECH-2 |
| REQ-05-01-004 | Unit-bearing load inputs shall preserve unit awareness and dimensional checks. | OPS-K-UNIT-1 |
| REQ-05-01-005 | Missing solve-required load values shall become explicit findings rather than silent defaults. | OPS-K-DATA-2 |
| REQ-05-01-006 | Load validation and application results shall be able to carry diagnostic fields for code, class, severity, source, affected object, message, remediation, and provenance when findings are emitted. | AB-00-06 |
| REQ-05-01-007 | Future implementation shall separate primitive load definitions from downstream load-case algebra and user-defined combinations. | SOW-013; SOW-014; DEL-05-02 |
| REQ-05-01-008 | Dynamic treatment for wind, seismic, or occasional categories shall remain TBD unless sealed by a later implementation brief. | SOW-013 note; `_CONTEXT.md` |
| REQ-05-01-009 | Future implementation acceptance shall include deterministic load application tests before release use. | OPS-K-SOLVER-1; anticipated artifacts |
| REQ-05-01-010 | Primitive load inputs shall retain provenance sufficient to distinguish user-supplied values, lawful imports, and unresolved TBDs. | OPS-K-DATA-1; OPS-K-DATA-2; AB-00-06 |

## Standards

No external protected standard text is introduced by this setup. Governing local standards are the project invariant catalog, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-05-01-001 | Schema/model inspection confirming all primitive categories are represented as categories without invented coefficients. |
| REQ-05-01-002 | Mechanics integration review confirming load definitions target the centerline/frame analysis model. |
| REQ-05-01-003 | Protected-content and rule-boundary review for absence of code-specific combinations, factors, allowables, and certification wording. |
| REQ-05-01-004 | Unit tests or schema tests for dimensional validity of unit-bearing load inputs. |
| REQ-05-01-005 | Invalid/missing input tests proving explicit findings are emitted. |
| REQ-05-01-006 | Diagnostic/result-envelope tests when load validation emits findings. |
| REQ-05-01-007 | Interface review against DEL-05-02 scope before adding algebra or combination behavior. |
| REQ-05-01-008 | Human ruling or sealed brief required before dynamic-specific implementation details are treated as in scope. |
| REQ-05-01-009 | Deterministic fixture tests for load application behavior before release use. |
| REQ-05-01-010 | Provenance checks showing source and unresolved-state handling for load inputs. |

## Documentation

Required local setup artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`.
