# CANDIDATE — DEL-08-04 route-A repair V3

Identifier: `K8-ROUTE-A-REPAIR-20260908-V3`. Status: additive repaired candidate; not adopted or implemented. It succeeds frozen K8 V2 manifest SHA-256 `ee4fcf91cf7eb818b2f85403c4819322df24d6070c94f62b7aeaee70e875725a` and consumes RK V2 backcheck SHA-256 `59ab40141c09864d1a77ad45107adaead71bd060a03d1e74299412739c6d28de`. V1 and V2 bytes remain unchanged.

## Total validation

`semantic_validator_v3.py` makes the mandatory gate total over every host value and strict-JSON value. A non-object top level, null/scalar `result_envelope`, non-array `result_sets` or `values`, scalar result row, and null/scalar `source_record`, `classification`, or `object_ref` return `RESULT_EXPORT_SCHEMA_INVALID`; none can raise through relation access. Object documents whose outer or inner version is absent or unsupported return `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED`. Well-shaped relation failures retain their specific stable codes from V2. Finite-number checks still precede general schema validation.

Both writer and reader use the same shape guard. The writer validates before strict serialization, then reparses and validates again. The reader strict-parses before validation. `adversarial_tests_v3.py` covers top-level null, array, string, number and boolean plus nine nested null/scalar shapes at both boundaries, and observes zero uncaught exceptions.

## Exact wire ingress and outcome

The external function is:

```rust
pub fn export_result_document_wire(raw: serde_json::Value) -> ResultExportWireOutcome;
```

It reads `requested_schema_version` from the raw object before constructing a typed request. Missing, null or empty returns `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED`; an unknown string returns `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED`; a non-string returns `RESULT_EXPORT_SCHEMA_INVALID`. Only a supported non-empty string constructs the internal `ResultExportRequest`, whose version field is the closed enum `SupportedResultExportWriteVersion`.

The JSON outcome is an internally tagged enum:

```rust
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ResultExportWireOutcome {
    Document { schema_version: String, document: serde_json::Value },
    Refused { refusal: ResultExportRefusal },
}
```

`ResultExportRefusal` retains `{code, requested_version, supported_versions, native_payload_available}`. `requested_version` is null for absent/null input. The Tauri command accepts `serde_json::Value` and returns this enum, so Serde cannot reject the missing version before normalization.

The existing CLI already holds `export_results` as raw `Value`. `execute_export_results` must call `normalize_report_result_schema_version` on required field `requested_result_schema_version` before deserializing `ReportPackageRequest`. A version refusal sets new optional `CliOutput.result_export_outcome` to the same `refused` variant, exits 1, emits no report package, and does not use `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`. Other malformed report fields retain the existing generic payload-invalid path. The typed `ReportPackageRequest` may use `Option<String>` for the already-normalized field or a custom deserializer that accepts missing/null; it must not preempt normalization.

Direct writer, Tauri raw command and report CLI use the same normalizer behavior. The V3 executable tests cover absent, null, empty, unknown and supported cases for all three boundary contracts.

## Complete dual-read dispatch for candidate K-A

The recommended K-A candidate retains bounded dual-read. Each advertised version has an exact handler:

| Version | Immutable schema | Validator | Capability |
|---|---|---|---|
| `0.1.0` | `schemas/results/0.1.0/results.schema.json` | `validate_result_export_0_1_0_legacy` | read/package the original partial document only |
| `0.2.0` | `schemas/results/0.2.0/results.schema.json` | `validate_result_export_0_2_0` | complete lossless route-A document |

Before changing the current schema alias, implementation must freeze the present `schemas/results.schema.yaml` content as the immutable 0.1 schema. The legacy validator performs strict JSON parsing, exact outer/inner `0.1.0`, finite-number validation, immutable-schema validation and global `result_id` uniqueness. Because 0.1 has no `source_record`, it cannot claim source equality or complete native coverage. It does not synthesize omitted rows, upgrade to 0.2, or infer pressure/user/observation semantics. Report packaging preserves the original 0.1 bytes/version and labels its capability `legacy_partial_read_only_no_upgrade`.

If the Owner later declines dual-read, `0.1.0` must be removed from `RESULT_EXPORT_READ_VERSIONS` and existing 0.1 documents will return unsupported; that is a material amendment to candidate K-A, not an implementation shortcut.

RK-V2-003 remains closed: pending pressure rows travel losslessly as `preserve_only`; only standardization or changed mechanics meaning remains PKG04/05-gated.
