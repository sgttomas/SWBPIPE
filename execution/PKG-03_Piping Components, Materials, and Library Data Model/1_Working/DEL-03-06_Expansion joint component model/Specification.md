# Specification: DEL-03-06 Expansion joint component model

## Scope

This deliverable-local setup specification covers the backend feature slice for an expansion joint component model. It is limited to the data model expectations needed to support manufacturer/user-supplied stiffness, effective area, movement limits, and hardware data.

Exclusions:

- No product implementation in this setup pass.
- No manufacturer proprietary values.
- No invented expansion joint defaults.
- No certification, authentication, or compliance claims.
- No rule evaluator or global solver implementation; PKG-03 explicitly excludes those responsibilities.

## Requirements

| Req ID | Requirement | Basis |
|---|---|---|
| DEL-03-06-R-001 | The future component model shall represent expansion joint stiffness data supplied by a user, manufacturer, or lawful private library. | SOW-010 |
| DEL-03-06-R-002 | The future component model shall represent effective area as supplied data with units and provenance. | SOW-010; OPS-K-UNIT-1; OPS-K-DATA-3 |
| DEL-03-06-R-003 | The future component model shall represent movement limits as supplied data with explicit missing-value handling. | SOW-010; OPS-K-DATA-2 |
| DEL-03-06-R-004 | The future component model shall represent hardware data without deriving defaults from protected or proprietary sources. | SOW-010; OPS-K-IP-1; OPS-K-IP-3 |
| DEL-03-06-R-005 | All expansion joint numeric values shall be unit-aware and dimensionally checked when persisted, imported, or used by downstream services. | OPS-K-UNIT-1; AB-00-04 |
| DEL-03-06-R-006 | Source, provenance, license/redistribution status, and review disposition shall be carried for component data where applicable. | OPS-K-IP-2; OPS-K-DATA-3 |
| DEL-03-06-R-007 | Missing solve-required or rule-check-required values shall produce explicit diagnostics or findings rather than silent defaults. | OPS-K-DATA-2; AB-00-06 |
| DEL-03-06-R-008 | The model shall preserve layer/API boundaries so adapters and plugins cannot bypass validation, provenance, units, diagnostics, or public/private data controls. | AB-00-02; AB-00-07 |
| DEL-03-06-R-009 | Validation tests shall cover schema/field presence, unit handling, missing-data diagnostics, provenance behavior, and protected-content guardrails where relevant. | AB-00-08 |

### Implementation-level TBDs

The semantic lensing pass identified these unresolved inputs. They are not requirements until human-approved or supported by later authoritative source material:

- `TBD`: stiffness field shape and degree-of-freedom mapping.
- `TBD`: required vs optional field classification.
- `TBD`: movement-limit validation classes.
- `TBD`: hardware flag/enumeration taxonomy.
- `TBD`: concrete acceptance criteria for unit/dimension validation and missing-data diagnostics.

## Standards

No accessible expansion joint manufacturer standard, code clause, or proprietary product data was introduced in the sealed setup brief. Applicable standards and exact clause references are therefore `TBD`. Any later standards-derived or manufacturer-derived value must be supplied by the user or lawfully imported private data and must not be bundled as a public default.

## Verification

| Requirement | Setup Verification Approach |
|---|---|
| DEL-03-06-R-001 through R-004 | Future schema/model tests confirm required categories exist while value contents remain supplied/TBD. |
| DEL-03-06-R-005 | Future unit tests and schema validation confirm dimensional typing and conversion behavior. |
| DEL-03-06-R-006 | Future provenance tests confirm source/license/review fields are present and preserved. |
| DEL-03-06-R-007 | Future validation tests confirm missing required data yields diagnostics/finding records. |
| DEL-03-06-R-008 | Future architecture/API tests confirm no bypass of validation/provenance/diagnostics boundaries. |
| DEL-03-06-R-009 | Future test-gate records confirm layered validation coverage. |
| Lens TBDs | Future human ruling or implementation brief defines exact taxonomy before code claims completeness. |

## Documentation

Anticipated artifacts for future product work:

- Expansion joint model.
- Validation tests.
- Schema/API notes if the model is exposed through persistence, import/export, adapters, or GUI services.
