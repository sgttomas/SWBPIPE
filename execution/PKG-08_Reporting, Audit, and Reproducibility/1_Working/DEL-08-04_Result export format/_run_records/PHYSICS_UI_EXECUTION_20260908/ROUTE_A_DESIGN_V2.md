# CANDIDATE — DEL-08-04 route-A design V2

Identifier: `K8-ROUTE-A-DESIGN-20260908-V2`. Status: repaired implementation-ready candidate; not adopted, public, or implemented. It succeeds `ROUTE_A_DESIGN.md` SHA-256 `1c7697d9f4d1e701bd10003d963e79189167f11f3b80f029d5e64e52e37255f7` without changing that frozen file. It resolves RK first-pass findings `RK-001` through `RK-003` from `instances/RK/RETURN.md` SHA-256 `1e6deabb8b0ec0971b342ba0db47e5b80aad970ec2d36c408060d9a068e90e57`.

The public compatibility choices remain candidates. The recommended combination remains `K-A + K-U1`; this record does not claim that the Owner selected it.

## V2 schema and mandatory runtime gate

`RESULTS_SCHEMA_0_2_0_CANDIDATE_V2.json` is candidate revision K8-V2 of public schema version `0.2.0`. It retains all V1 coverage and binds `classification.owner_semantics` in every one of the 45 `(source_record.kind, source_record.unit)` branches. A P5-pending pressure row therefore cannot be schema-valid with `PKG08_TRANSPORT_ONLY` ownership.

JSON Schema cannot express uniqueness of a property across arrays or equality between independently located fields. Draft 2020-12 also validates an in-memory data model; a permissive host can supply non-finite floats even though `NaN` and infinities are not JSON number tokens. Schema validation alone is therefore insufficient.

The adopted implementation must provide one version-specific semantic validator named `validate_result_export_0_2_0`. Both boundaries must call the same validator:

1. **Writer:** after DTO construction and before emission, governed hashing, report inclusion, adapter conversion, or UI handoff. It validates the in-memory document, serializes with a strict finite JSON serializer, reparses with a strict JSON parser, and validates the reparsed value.
2. **Reader:** after strict JSON parsing and exact version dispatch, before interpretation, governed hashing, report generation, conversion, comparison, or UI rendering.

Validation success is required; diagnostics do not downgrade these failures to warnings. `semantic_validator_v2.py` is the executable reference behavior and `VALIDATOR_CONTRACT_V2.md` is the implementation contract.

## Exact candidate public API under K-A + K-U1

If the Owner selects `K-A + K-U1`, the implementation target is one PKG10 writer API, shared by headless and desktop:

```rust
pub const RESULT_EXPORT_WRITE_VERSIONS: &[&str] = &["0.2.0"];
pub const RESULT_EXPORT_READ_VERSIONS: &[&str] = &["0.1.0", "0.2.0"];

pub struct ResultExportRequest {
    pub requested_schema_version: String,
    pub runner_request: RunnerRequest,
    pub runner_result: RunnerResult,
    pub mechanics_envelope: MechanicsEnvelope,
}

pub enum ResultExportOutcome {
    Document(serde_json::Value),
    Refused(ResultExportRefusal),
}

pub struct ResultExportRefusal {
    pub code: String,
    pub requested_version: String,
    pub supported_versions: Vec<String>,
    pub native_payload_available: bool,
}

pub fn export_result_document(request: ResultExportRequest) -> ResultExportOutcome;
```

These names are the proposed public contract, not present source. The implementation replaces direct calls to `result_envelope_binding::build_result_export_document` with `export_result_document`; the current function may remain private only as an immutable `0.1.0` migration helper. `requested_schema_version` is required and has no absence/default rule. Missing or empty input yields `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED`. An unregistered version yields `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` with the exact request, the closed registry in ascending semantic-version order, `native_payload_available=true`, and no result document. PKG08 owns the read/write registries and version-specific validator; PKG10 owns dispatch and native-to-document adapter behavior.

The first serialized bridge is one Tauri command backed by that same Rust API:

```rust
#[tauri::command]
fn export_result_document(
    request: ResultExportRequest,
) -> ResultExportOutcome;
```

The desktop stores the returned `Document` once per `(run_id, requested_schema_version, native-result-envelope hash)`. `ResultExportPanel.tsx` receives that document and removes its local `buildResultExportPacket`, kind heuristics, and fallback to `ratio`. `buildReportPackageRequest` receives the same document as a new required argument `resultExportDocument`; it removes its independent `resultEnvelope` construction and puts the supplied document's `result_envelope` into `result_envelopes` unchanged.

At the report reader boundary, `ReportPackageRequest` adds required `requested_result_schema_version: String`. `assemble_wire_request` first requires every supplied envelope to equal that version, dispatches through `RESULT_EXPORT_READ_VERSIONS`, runs the version-specific schema and semantic validator, and only then converts or packages it. `openpipestress-runner export-results` carries this existing `ReportPackageRequest` field unchanged; it does not run another result writer. A mismatch or unknown version returns the same structured refusal fields and no report package.

The current library-only `PreviewRunnerOutput.result_envelope_document` receives the `Document` outcome. A `Refused` outcome leaves it `None`, appends the stable blocking code and refusal fields to the runner diagnostic, and leaves `mechanics_envelope` available. This preserves the native payload while refusing a false canonical result.

No desktop or report-package code regenerates a canonical document after this migration. Native desktop result views may continue reading `MechanicsResult`; any view labeled canonical must use the returned document and its row-level status.

## Row-level interpretation rule

`ResultSet.set_type` groups rows and does not confer standardized mechanics meaning. Every consumer must branch on `QuantityResult.classification.semantic_status`:

| Status | Permitted interpretation |
|---|---|
| `existing_canonical_category` | standardized category already accepted; no new mechanics meaning |
| `source_specific_metadata_standardized` | standardized transport metadata/category only |
| `source_specific_unstandardized` | preserve, identify, display as unstandardized; no mechanics inference |
| `existing_pressure_source_label_preserved_pending_P5` | preserve exact source label and row; no standardized pressure inference |
| `source_specific_unstandardized_pending_P5` | preserve exact source row; no standardized pressure inference |
| `transport_observation_only` | diagnostic/transport observation only |

Thus a pending, source-specific, user-review, or solver-observation row remains `preserve_only` even inside a current-shaped `set_type=mechanics` set. A consumer that cannot display that distinction must refuse interpretation rather than fall back to family, dimension, set type, or `ratio`.

## Pressure sequencing clarification

Lossless transport of every current pressure row is permitted in the candidate now: its exact `source_record`, native value, unit, metadata, basis, entity and ordered source-result references travel with a P5-pending status. PKG04/05 acceptance gates only changing that pending/source-specific status or assigning standardized wall force, effective force, closure-transfer, boundary, constitutive, or other mechanics meaning. It does not gate exact-preservation transport. This sequencing statement is a candidate design clarification and does not adopt a pressure model.

Implementation order after the compatibility Owner ruling is therefore:

1. PKG08 adopts a version choice, schema, registry, validator and stable error payload.
2. PKG10 implements the single adapter API and exact-preservation `0.2.0` writer; pending pressure rows remain `preserve_only`.
3. Desktop and report-package consumers use the one returned document and both reader boundaries validate it.
4. PKG14 binds the enclosing-document digest without changing existing referenced checksum payloads or algorithms.
5. A later PKG04/05-accepted pressure decision may revise pressure status or meaning through its own governed change and version analysis.

## Closure checks

K8-V2 acceptance requires the candidate builder and adversarial test to pass, the V1 manifest to remain byte-identical, and RK to backcheck the V2 manifest. No production source may be changed from this candidate alone.
