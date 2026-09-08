# RK independent final successor backcheck — K8 V3

Verdict: `CHANGES_REQUIRED`.

Review basis: K8 `MANIFEST_V3.json` SHA-256 `24d368585f1b1a7f01d79dd6201ae44fa0bfb20bc0aea72d41e5ca2c5ffcdf4c`, with 16/16 bound outputs present and byte-matched. K8 V1 and V2 manifests remain unchanged at their frozen hashes.

## Actionable finding

### RK-V3-001 — HIGH — the V3 validator is still not total over the values its contract claims to accept

The V3 guard checks the container types for `result_envelope`, result sets, rows, `source_record`, `classification`, and `object_ref`, but it does not guard the types of relation fields before calling the V2 semantic validator. A strict-JSON document with `source_record.kind: []` reaches the mapping lookup with an unhashable list and raises `TypeError` at both writer and reader boundaries. `source_record.unit: {}` fails the same way. A numeric `source_record.kind` does not raise, but it returns `RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED` instead of the contract's `RESULT_EXPORT_SCHEMA_INVALID` for a wrong-shaped field. The stronger host-value claim also fails: adding a non-string mapping key to an otherwise valid writer document raises `AttributeError` in the numeric walker before strict serialization can reject it.

This is the same validation-order boundary identified in `RK-V2-001`. The V3 top-level and nine nested probes close the exact crashes reported in V2, but do not establish the advertised total function or schema-first type behavior.

Required repair: place a non-throwing schema/type phase before every relation lookup, or extend the pre-relation guard to all accessed field types and make the host-value walker total. Parser-valid field-type violations must return `RESULT_EXPORT_SCHEMA_INVALID`; no unhashable mapping key or host object key may escape. Add both-boundary cases for array/object `source_record.kind` and `source_record.unit`, plus a writer-only non-string object-key case. Retain the specific mirror/owner codes only after the fields needed for those relations pass their structural types.

## Prior-finding disposition

- `RK-V2-001`: remains open through `RK-V3-001`; the named V2 malformed cases are fixed, but totality is not.
- `RK-V2-002`: closed at the candidate-design boundary. V3 moves version normalization ahead of typed DTO construction, defines the internally tagged wire outcome and Tauri/report CLI mapping, makes wrong version-field types schema-invalid, and binds both advertised read versions. The 0.1 handler is explicitly immutable, partial, read-only, and unable to infer or upgrade omitted semantics.
- `RK-V2-003`: remains closed. The three pressure mappings retain P5-pending status and PKG04/05 ownership; exact transport remains `preserve_only`, while status or mechanics-meaning standardization remains PKG04/05-gated.

## Positive evidence

- The V3 manifest and all 16 output hashes match.
- Isolated V3 builder and adversarial runs pass and regenerate the schema, mapping, coverage record, and adversarial record byte-for-byte. The V2 adversarial suite also passes unchanged.
- The 830-row witness passes writer/reader round trip with 830/830 exact `source_record` equality.
- The V3 schema differs from V2 only in candidate revision comment/description. It retains 45 discriminatory kind/unit branches, 45 required owner constants, exact unknown-pair refusal, and the V2 checksum/source-record protections.
- Direct and report version normalization returns the specified codes for absent, null, empty, wrong-type, unknown, and supported version values.
- Candidate K-A+K-U1 remains unselected. No public schema, producer, consumer, pressure model, lifecycle state, or production source was adopted or changed.

## Review limit

This review covered only the 16 V3 successor outputs and the two requested repair closures, with regression confirmation of pressure sequencing. It used focused isolated Python/schema probes. It did not rerun the original broad program review, Cargo, the full harness, product/native builds, or physical validation.
