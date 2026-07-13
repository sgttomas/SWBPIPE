# Specification: DEL-03-06 Expansion joint component model

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-03-06-DECL-001`.

## Scope

This deliverable-local specification covers the implemented backend data-model slice for an expansion joint component model. Current evidence is limited to component-library schema slots, an invented public fixture record, completeness rules, diagnostics, and schema tests for manufacturer/user/private-library supplied stiffness, effective area, movement limits, and hardware data.

Exclusions:

- No public engineering value implementation.
- No manufacturer proprietary values.
- No invented expansion joint defaults.
- No certification, authentication, or compliance claims.
- No rule evaluator or global solver implementation; PKG-03 explicitly excludes those responsibilities.

## Requirements

| Req ID | Requirement | Basis |
|---|---|---|
| DEL-03-06-R-001 | The component model shall represent expansion joint stiffness data as supplied by a user, manufacturer, or lawful private library; current evidence implements `linear_stiffness` and `rotational_stiffness` slots without public values. | SOW-010 |
| DEL-03-06-R-002 | The component model shall represent effective area as supplied data with units and provenance; current evidence implements the `effective_area` slot without public values. | SOW-010; OPS-K-UNIT-1; OPS-K-DATA-3 |
| DEL-03-06-R-003 | The component model shall represent movement limits as supplied data with explicit missing-value handling; current evidence implements the `movement_limit` slot and missing-data diagnostic path while taxonomy remains `TBD`. | SOW-010; OPS-K-DATA-2 |
| DEL-03-06-R-004 | The component model shall represent hardware data without deriving defaults from protected or proprietary sources; current evidence implements `hardware_flag`/`hardware_reference` slots while hardware taxonomy remains `TBD`. | SOW-010; OPS-K-IP-1; OPS-K-IP-3 |
| DEL-03-06-R-005 | All expansion joint numeric values shall be unit-aware and dimensionally checked when persisted, imported, or used by downstream services. | OPS-K-UNIT-1; AB-00-04 |
| DEL-03-06-R-006 | Source, provenance, license/redistribution status, and review disposition shall be carried for component data where applicable. | OPS-K-IP-2; OPS-K-DATA-3 |
| DEL-03-06-R-007 | Missing solve-required or rule-check-required values shall produce explicit diagnostics or findings rather than silent defaults. | OPS-K-DATA-2; AB-00-06 |
| DEL-03-06-R-008 | The model shall preserve layer/API boundaries so adapters and plugins cannot bypass validation, provenance, units, diagnostics, or public/private data controls. | AB-00-02; AB-00-07 |
| DEL-03-06-R-009 | Validation tests shall cover schema/field presence, unit handling, missing-data diagnostics, provenance behavior, and protected-content guardrails where relevant. | AB-00-08 |

### Residual TBDs and gates

The implementation evidence technically addresses the stale generic-stiffness concern by using accepted `linear_stiffness` and `rotational_stiffness` dimensions. The following items remain unresolved and must not be treated as closed until human-approved or supported by later authoritative source material:

- `TBD`: exact per-axis stiffness field shape and solver degree-of-freedom mapping beyond the implemented dimensions.
- `TBD`: release-level required vs optional field classification beyond the current completeness rule.
- `TBD`: movement-limit validation classes.
- `TBD`: hardware flag/enumeration taxonomy.
- `TBD`: public expansion-joint source catalog policy and public fixture-value policy.
- `TBD`: dependency satisfaction, human disposition of `Review_Findings.csv`, and lifecycle closure.

## Standards

No accessible expansion joint manufacturer standard, code clause, or proprietary product data is introduced by the current evidence. Applicable standards and exact clause references remain `TBD`. Any later standards-derived or manufacturer-derived value must be supplied by the user or lawfully imported private data and must not be bundled as a public default.

## Verification

| Requirement | Current Verification Evidence |
|---|---|
| DEL-03-06-R-001 through R-004 | `schemas/component.schema.yaml` and `fixtures/component/invented_component_library_valid.json` define expansion-joint component type, family contract fields, invented record fields, and protected-value policy; `tests/test_component_section_schema.py` asserts this coverage. |
| DEL-03-06-R-005 | Tests assert component quantity dimensions include accepted `linear_stiffness` and `rotational_stiffness`, remain within accepted PKG-02 dimensions, and do not use retired dimensions. |
| DEL-03-06-R-006 | The invented fixture carries source, license/redistribution, contributor certification, and review status on schema-slot records. |
| DEL-03-06-R-007 | The invented fixture records an incomplete expansion-joint completeness finding and `EXPANSION_JOINT_STIFFNESS_DATA_MISSING` diagnostic. |
| DEL-03-06-R-008 | Current evidence is schema/fixture/test only; solver, adapter, GUI, persistence-service, and report bypass checks remain downstream package scope or separate authorized scope. |
| DEL-03-06-R-009 | `python3 -m pytest tests/test_component_section_schema.py` is the targeted validation for this reconciliation pass. |
| Residual TBDs | Human ruling or later sealed work must define remaining taxonomy, source/value policy, dependency closure, lifecycle state, and review dispositions before completeness or release claims. |

## Documentation

Current artifacts:

- `schemas/component.schema.yaml`
- `fixtures/component/invented_component_library_valid.json`
- `tests/test_component_section_schema.py`
- This DEL-03-06 evidence kit and run records.

Future authorized work may add schema/API notes if the model is exposed through persistence services, import/export, adapters, GUI services, or reports.
