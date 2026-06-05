# Datasheet: DEL-03-06 Expansion joint component model

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-03-06 |
| Package ID | PKG-03 |
| Package | Piping Components, Materials, and Library Data Model |
| Type | BACKEND_FEATURE_SLICE |
| Scope Item | SOW-010 |
| Objective | OBJ-004 |
| Evidence Status | Implemented schema, invented fixture, and test evidence for expansion-joint component slots; no public engineering values or lifecycle promotion |

## Attributes

The implemented component-library evidence represents expansion joints as supplied-data component records. The current public repository evidence defines schema slots, an invented fixture record, completeness findings, and diagnostics; it does not supply engineering values.

| Attribute Category | Required Treatment | Source |
|---|---|---|
| Stiffness values | Implemented as `linear_stiffness` and `rotational_stiffness` component field kinds and accepted quantity dimensions. Public fixture values remain missing and private/user/manufacturer supplied. Exact per-axis DOF/tensor mapping for downstream solver consumption remains `TBD`. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010; docs/CONTRACT.md#OPS-K-DATA-1 |
| Effective area | Implemented as an `effective_area` field kind with provenance and missing-value handling in the invented fixture. Pressure/thrust usage semantics remain downstream solver scope. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010 |
| Movement limits | Implemented as a `movement_limit` field kind with explicit missing-value diagnostics. Movement-limit class taxonomy and dimensional validation categories remain `TBD`. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010; docs/CONTRACT.md#OPS-K-DATA-2 |
| Hardware data | Implemented as `hardware_flag`/`hardware_reference` schema slots and fixture contract evidence, with `hardware_reference` present on the invented expansion-joint record. Hardware flag/enumeration taxonomy remains `TBD`. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010 |
| Provenance | Implemented through source, license/redistribution status, contributor certification, and review status fields on schema-slot fixture records. Public source-catalog policy, fixture-value policy, and human review disposition remain open. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; docs/CONTRACT.md#OPS-K-IP-2; docs/CONTRACT.md#OPS-K-DATA-3 |

### Preserved TBDs and Gates

- `TBD`: exact per-axis stiffness field shape and solver degree-of-freedom mapping beyond the implemented `linear_stiffness` and `rotational_stiffness` dimensions.
- `TBD`: release-level required vs optional classification beyond the current schema completeness rule.
- `TBD`: movement-limit classes and dimensional validation categories.
- `TBD`: hardware flag/enumeration taxonomy.
- `TBD`: accepted public expansion-joint source catalog and public fixture-value policy.
- `TBD`: dependency satisfaction, human disposition of review findings, and lifecycle closure.

## Conditions

- Public repository artifacts must not include manufacturer proprietary values, protected standards text, protected examples, or copied data tables.
- Missing solve-required or rule-check-required expansion joint values must remain explicit findings, never silent defaults.
- Current schema dimensions classify stiffness as `linear_stiffness` and `rotational_stiffness`; any supplied numeric quantity must remain unit-aware and dimensionally checked before mechanics or rule-check use.
- Outputs may support review but must not claim certification, authentication, or code compliance.

## Construction

Current construction is bounded to the component-library schema, invented fixture, and schema validation tests. The implementation evidence includes `expansion_joint` as a component type, expansion-joint family contract fields, an invented expansion-joint record with missing supplied values, a blocking completeness finding, and expansion-joint diagnostic codes.

Current model partitions:

| Partition | Content |
|---|---|
| Identity | Component ID, component type, library/source reference, provenance metadata |
| Mechanical inputs | `linear_stiffness`, `rotational_stiffness`, `effective_area`, `movement_limit`, `hardware_flag`/`hardware_reference`, and `manufacturer_reference` slots |
| Unit metadata | Component quantity dimensions for linear and rotational stiffness plus area-compatible supplied values |
| Validation state | Missing data diagnostics, provenance warnings, assumption warnings |
| Persistence hooks | Schema version, strict schema shape, fixture provenance, review status, redistribution status, and open decision records |

## References

- `_CONTEXT.md` for deliverable identity, architecture basis, and package scope.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-010, OBJ-004, PKG-03, and AB-00-01/02/04/06/07/08.
- `docs/CONTRACT.md` for OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-AGENT-1..4.
- `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, and `tests/test_component_section_schema.py` for current implementation evidence.
