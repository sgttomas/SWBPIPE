# Candidate result-export validator contract V3

Status: successor to frozen V2; not adopted or implemented.

The validator is a total function over all host values. It returns a list of stable errors and never exposes a type, key, parse, serialization, overflow, or relation-access exception. A structural shape guard precedes relation access. Non-object or wrong-shaped parsed JSON returns `RESULT_EXPORT_SCHEMA_INVALID`. Exact version dispatch applies only after an object envelope is available. Non-finite checks precede Draft 2020-12 validation because host validators may accept non-JSON float values.

V2's stable relation taxonomy remains unchanged for well-shaped documents. V3 adds executable totality cases for top-level null/array/scalars and nested null/scalar relation nodes. Writer and reader behavior must agree for the same JSON-shaped value.

Raw version normalization also precedes typed DTO construction. Missing, null and empty version fields return `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED`; unknown versions return `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED`; wrong field types return `RESULT_EXPORT_SCHEMA_INVALID`. Tauri accepts raw `Value`. The report CLI inspects the raw `export_results` value before `ReportPackageRequest` deserialization and publishes the same tagged refusal outcome.

The candidate read registry is closed and handler-bound: 0.1 uses `validate_result_export_0_1_0_legacy` with the immutable versioned 0.1 schema and partial/no-upgrade limits; 0.2 uses `validate_result_export_0_2_0` with the V3 schema and full source-record relations.
