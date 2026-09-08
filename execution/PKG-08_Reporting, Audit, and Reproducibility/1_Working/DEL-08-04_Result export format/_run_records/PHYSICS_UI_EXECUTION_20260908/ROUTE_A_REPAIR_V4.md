# CANDIDATE — DEL-08-04 route-A repair V4

Identifier: `K8-ROUTE-A-REPAIR-20260908-V4`. Status: additive repaired candidate; not adopted or implemented. It succeeds frozen K8 V3 manifest SHA-256 `24d368585f1b1a7f01d79dd6201ae44fa0bfb20bc0aea72d41e5ca2c5ffcdf4c` and consumes RK V3 backcheck SHA-256 `50ad5912f72acfca1f06e2868a448e8b60c51d8f817b1629390fe88d6bff5aee`. V1 through V3 bytes remain unchanged.

V4 changes only the validator-totality method reopened by RK-V3-001. The V3 API/version ingress, immutable partial 0.1 reader, complete 0.2 reader, public compatibility candidates, and pressure sequencing remain unchanged and are not re-decided here.

## Central validation phases

`semantic_validator_v4.py` uses four ordered phases:

1. A non-recursive host-domain preflight proves that the value is finite, acyclic strict JSON composed only of null, boolean, number, string, array and object, with string-only object keys. Non-finite numbers retain `RESULT_EXPORT_NON_FINITE_NUMBER`; every other host-domain violation returns `RESULT_EXPORT_SCHEMA_INVALID`.
2. The complete Draft 2020-12 candidate schema runs once. Its complete nested error tree is centrally partitioned for structural/type failures before any semantic relation lookup. The structural class includes type, required-property, container-size/property and additional-property failures, plus enum/const failures whose instance JSON type differs from the expected type.
3. Only after that structural/type class is empty may exact version dispatch and the existing V2 semantic relations run. Thus wrong-shaped kind/unit/classification/reference values return schema-invalid without reaching `.get`, tuple hashing or string methods, while a well-typed unknown `(kind, unit)` pair retains `RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED`.
4. Remaining full-schema errors return `RESULT_EXPORT_SCHEMA_INVALID`. No catch-all exception handler hides programming faults.

The same function gates in-memory writer input and strict-parsed reader input. The writer then strictly serializes, reparses and revalidates.

## Whole-class proof

The V4 adversarial suite derives 52 paths for every field consumed by version or semantic relations, including nested references, metadata and classification. It substitutes each with all six JSON value classes, creating 312 cases and 624 writer/reader boundary calls. Every substitution with the wrong JSON type returns `RESULT_EXPORT_SCHEMA_INVALID`; zero exceptions escape.

Named regressions cover `source_record.kind` as array, object and number; `source_record.unit` as array and object; a writer-only non-string object key; global duplicates; all primary mirrors; reference projection; false owner; well-typed unknown pair; NaN and both infinities; duplicate JSON members; exact version ingress; and P5-pending preserve-only status. The clean 830-row witness round-trips with exact `source_record` equality.

The V4 schema changes only its candidate revision annotation and retains the 45 source-pair branches and owner bindings. No public schema, adapter, consumer or physics implementation changed.
