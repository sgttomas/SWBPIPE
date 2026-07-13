# Datasheet: DEL-03-08 Pipe section property and mass-property calculator

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-08 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Deliverable type | BACKEND_FEATURE_SLICE |
| Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 |
| Scope items | SOW-051, SOW-018 |
| Objectives | OBJ-004, OBJ-012 |
| Context envelope | M |
| Lifecycle state | `IN_PROGRESS`; implementation evidence exists, but this is not ISSUED, accepted, released, or approved for professional reliance |

## Attributes

| Attribute | Value |
|---|---|
| Calculator purpose | Calculate pipe section and mass-property outputs from user-entered dimensions and material data with unit checks. |
| Permitted data source posture | User-entered or lawfully imported private/project data only. |
| Public bundled data posture | No protected pipe dimensional tables, material allowables, contents defaults, insulation defaults, corrosion allowances, or proprietary component data. |
| Unit posture | Inputs and outputs carry explicit unit and dimension metadata. Mixed units are rejected; approved conversion constants and public unit catalog remain `TBD`. |
| Provenance posture | Calculator inputs require provenance metadata, and derived outputs state that they were calculated from user-entered dimensions. Exact private-library record linkage remains `TBD`. |
| Solver boundary | This deliverable prepares section and mass properties; it does not implement the global solver, code compliance, or rule-pack evaluation. |

## Conditions

Implementation evidence exists in `core/section_properties/calculator.py` for user-entered pipe outside diameter, wall thickness, optional corrosion allowance, optional insulation thickness, and optional material, contents, and insulation density inputs. The calculator derives inside diameter, metal/cross-section area, second moment of area, section modulus, torsional constant, contents volume per length, and optional mass-per-length contributors when matching density inputs are supplied.

The implementation rejects missing required dimensions, missing provenance, incompatible dimensions, mixed units, and non-physical geometry through blocking diagnostics. It does not provide pipe schedule tables, material defaults, unit conversion constants, protected dimensional tables, contents defaults, insulation defaults, corrosion defaults, SIF/flexibility values, code-specific values, or proprietary catalog values.

Exact allowed public unit catalog, conversion constants, accepted schema field placement, dependency satisfaction, fixture-value policy, public source catalog, lifecycle disposition, and downstream solver/GUI/report integration remain `TBD`.

Mass-property tests use invented synthetic values in `tests/test_section_properties.py`. No protected published pipe tables, material tables, or vendor proprietary data may be encoded as test data.

## Construction

| Construction item | Status |
|---|---|
| Section property calculator artifact | Implemented at `core/section_properties/calculator.py`. |
| Calculator README | Implemented at `core/section_properties/README.md`. |
| Mass property tests | Implemented in `tests/test_section_properties.py` using invented synthetic values; formal fixture-value policy remains `TBD`. |
| Schema-like input mapping | Implemented through `quantity_from_mapping`; accepted schema field placement and dependency satisfaction remain `TBD`. |
| Diagnostic envelope fields | Implemented with code, diagnostic class, severity, field, source, affected object, message, remediation, and provenance; downstream result-envelope mapping remains `TBD`. |
| Private library linkage | Provenance is required at the calculator boundary; exact private-library record linkage remains `TBD`. |

## References

- `_CONTEXT.md` for deliverable identity, scope, objectives, artifact list, and architecture basis injection.
- `_REFERENCES.md` for governing local references.
- `docs/_Registers/Deliverables.csv` row DEL-03-08.
- `docs/_Registers/ScopeLedger.csv` rows SOW-051 and SOW-018.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-03-08.
- `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-SOLVER-1, and OPS-K-AGENT-1..4.
- `core/section_properties/calculator.py` and `core/section_properties/README.md` for implemented calculator behavior.
- `tests/test_section_properties.py` for current unit and negative-test evidence.
- `fixtures/results/invented/tp_phys_015_section_property_stress_evidence_envelope.json` for invented downstream section-property evidence context.
- `Review_Findings.csv` for human-gated technical finding status; statuses remain `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`.

## Open Questions

Current implementation-form note: `PipeSectionInput` accepts an optional
dimensional `mill_tolerance`, and the calculator uses `wall_thickness -
corrosion_allowance - mill_tolerance` as effective wall. The input is
provenance-stamped and validated like the other dimensional slots; a fractional
or catalog-derived tolerance policy remains `TBD`.

| Question | Needed from |
|---|---|
| Which approved unit catalog and conversion constants, if any, may this calculator call? | DEL-02-02 / human architecture ruling |
| Which schema record owns calculator inputs, outputs, and library linkage? | DEL-03-02 / schema owner |
| What downstream result-envelope mapping is required before solver, persistence, GUI, or report integration? | PKG-02 / PKG-08 / human architecture ruling |
| What public source catalog and fixture-value policy is acceptable for section and mass-property tests? | Validation/QA owner |
| When may the technically addressed review findings move beyond human-gated `TBD` disposition? | Human project authority |

## D-41 R5 T2B PDU-047 Evidence State

| Path | Inputs | Bound outputs |
|---|---|---|
| Production `calculate_pipe_section_properties` → TP-PHYS-015 formal oracle → governed result envelope | Invented OD 2.0 m; wall 0.25 m | Area `m^2`/`area`; section modulus `m^3`/`section_modulus`; torsional constant `m^4`/`second_moment_area`, checked with existing witness tolerances. |
