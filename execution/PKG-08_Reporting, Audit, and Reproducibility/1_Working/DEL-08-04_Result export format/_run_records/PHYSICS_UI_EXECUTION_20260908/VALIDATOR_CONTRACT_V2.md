# Candidate result-export 0.2.0 validator contract V2

Status: implementation contract candidate; not adopted or implemented. This contract resolves RK-001 and is executable in `instances/K8/evidence/semantic_validator_v2.py`.

## Required order and invocation

Readers perform strict UTF-8 JSON parse, exact version dispatch, finite and relation validation, Draft 2020-12 validation, then interpretation. Writers perform finite and relation validation, Draft 2020-12 validation, strict serialization with non-finite output disabled, strict reparse, and the same reader validation. Both call the version-specific `validate_result_export_0_2_0` behavior. A document that fails any stage cannot be emitted, governed-hashed as a result export, reported, converted, compared, or rendered as canonical.

The runtime validator enforces:

- `result_id` is unique across all result sets;
- `result_id == source_record.id`;
- `magnitude == source_record.value` with both finite;
- `unit == source_record.unit`;
- metadata presence/absence and JSON value equality;
- `object_ref == {"ref_type":"preview_entity","ref_id":source_record.entity_ref}`;
- `basis_ref == source_record.basis_ref` when present, otherwise the enclosing result set's `basis_ref`;
- top-level `source_result_refs` has the same presence and ordered JSON value as `source_record.source_result_refs`;
- top-level family/dimension and all five classification fields equal the registered 45-pair mapping;
- row `semantic_status`, not enclosing `set_type`, controls whether a row is standardized or preserve-only.

JSON object member names must also be unique. `NaN`, `Infinity`, `-Infinity`, host-language non-finite numbers and any serializer mode that emits them are forbidden.

## Stable failure payload and taxonomy

Each failure is represented as `{code, json_pointer, result_id, message}`. `result_id` is null when the failure is document-wide. Error order follows document order; consumers may display `message`, but integrations branch only on `code`.

| Code | Required trigger |
|---|---|
| `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED` | requested version absent or empty at writer/API boundary |
| `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` | requested, outer or inner version is not in the applicable registry |
| `RESULT_EXPORT_JSON_INVALID` | payload is not strict UTF-8 JSON |
| `RESULT_EXPORT_JSON_DUPLICATE_MEMBER` | a JSON object repeats a member name |
| `RESULT_EXPORT_NON_FINITE_NUMBER` | either mirror or any other document number is non-finite |
| `RESULT_EXPORT_SCHEMA_INVALID` | Draft 2020-12 validation fails after version dispatch and pre-schema guards |
| `RESULT_EXPORT_DUPLICATE_RESULT_ID` | a result ID appears more than once across any sets |
| `RESULT_EXPORT_SOURCE_RECORD_ID_MISMATCH` | top ID differs from native ID |
| `RESULT_EXPORT_SOURCE_RECORD_VALUE_MISMATCH` | magnitude differs from native value |
| `RESULT_EXPORT_SOURCE_RECORD_UNIT_MISMATCH` | unit differs from native unit |
| `RESULT_EXPORT_SOURCE_RECORD_METADATA_MISMATCH` | metadata value or presence differs |
| `RESULT_EXPORT_SOURCE_RECORD_REFERENCE_MISMATCH` | object, basis or ordered source-result reference projection differs |
| `RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED` | native pair has no exact registry row |
| `RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH` | owner differs from the pair registry |
| `RESULT_EXPORT_CLASSIFICATION_MISMATCH` | family, dimension or other classification field differs from the pair registry |

The first failure is sufficient to block. Implementations may return all errors in this stable order: version, non-finite, duplicate ID, ID, value, unit, metadata, reference, native pair, owner, classification, schema.

## Boundary tests

Each invariant must be mutated independently at the in-memory writer and serialized reader boundaries. The suite must include duplicate IDs across different sets, metadata removal as well as value disagreement, source-result-ref reordering, finite mirror disagreement, paired `NaN`/infinity injection, unknown unit for a known kind, false P5 owner, and a pending row inside `set_type=mechanics`. A clean 830-row witness must pass writer serialization, strict reader parse, schema validation and semantic validation with 830/830 exact source records.
