# RK independent successor backcheck — K8 V2

Verdict: `CHANGES_REQUIRED`.

Review basis: K8 `MANIFEST_V2.json` SHA-256 `ee4fcf91cf7eb818b2f85403c4819322df24d6070c94f62b7aeaee70e875725a`, 14/14 bound outputs present and byte-matched. The V1 manifest remains SHA-256 `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65` and its 12 outputs remain unchanged.

## Actionable findings

### RK-V2-001 — HIGH — the mandatory reader/writer validator can throw on schema-invalid JSON instead of returning a stable refusal

`semantic_validator_v2.py` calls relation validation before Draft 2020-12 validation. Although line 126 conditionally builds `envelope`, line 127 calls `document.get(...)` unconditionally; parser-valid `[]` and `null` therefore raise `AttributeError`. A version-valid document with `classification: null` reaches line 203 and raises the same exception. `validate_writer([])` also throws. These are required boundary inputs, not a schema-only concern: the candidate says every strict-parsed reader input and in-memory writer document passes the mandatory gate and that schema failure returns `RESULT_EXPORT_SCHEMA_INVALID` rather than escaping as an exception.

Required repair: make the validator total over every strict JSON value and every writer input. Perform a non-throwing structural/type guard before relation access, or make every relation walk type-safe, and return a defined stable error for invalid shapes. `RESULT_EXPORT_SCHEMA_INVALID` should cover parser-valid structural failures; reserve version refusal for object documents whose requested/outer/inner version is missing or unregistered. Add both-boundary adversarial probes for a top-level array, top-level null, and null/scalar nested relation nodes such as `classification`, then prove no malformed strict-JSON shape escapes the validator.

This reopens `RK-001`. The V2 checks do close its substantive well-shaped-document cases: global duplicate IDs, ID/value/unit/metadata/reference equality, ordered source-result references, finite numbers, exact kind/unit registry lookup, owner equality, and row-level status dominance.

### RK-V2-002 — MEDIUM — the promised absent-version refusal cannot traverse the proposed typed wire boundaries, and the 0.1.0 read dispatch remains underspecified

The proposed exact API uses `requested_schema_version: String`, and the proposed Tauri command accepts `ResultExportRequest` directly, while promising that an absent version returns `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED` in `ResultExportOutcome::Refused`. At a serialized Tauri boundary, ordinary Serde deserialization rejects an absent required `String` before `export_result_document` can run. The same contradiction is concrete at the existing report boundary: current `ReportPackageRequest` derives `Deserialize` with required fields, and `openpipestress-runner` converts raw JSON to that DTO before `assemble_wire_request`; adding another required `String` means an omitted version follows the existing generic `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID` path rather than the proposed structured result-export refusal.

The read registry also advertises `0.1.0` and `0.2.0`, while the V2 packet defines only `validate_result_export_0_2_0`. The design requires a version-specific schema and semantic validator after dispatch, so the registered 0.1.0 handler and its exact validation/migration behavior must be named or the version must not be in the registry.

Required repair: define the serialized request and outcome DTOs, including the enum/tag representation; make the ingress version field nullable/optional or provide an explicit custom/raw-value deserialization layer that can issue `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED`; and route report CLI/Tauri failures through the same stable refusal shape. Bind every advertised read version to a named schema and semantic validator, including the immutable 0.1.0 path. Add direct writer API, Tauri DTO, and report-package CLI probes for absent, empty, and unknown versions.

This leaves `RK-002` open. The V2 packet otherwise makes useful concrete choices for ownership, cache identity, one-document desktop/report consumption, no local canonical regeneration, and native-payload availability on refusal.

## Closed first-pass finding

`RK-003` is closed. All three pressure mappings remain owned by `PKG04_05_MECHANICS` and carry P5-pending statuses. The row-status rule makes them `preserve_only`, even in a `mechanics` result set. The design permits exact lossless native pressure transport now and correctly gates only a later status/meaning standardization on PKG04/05 acceptance. It does not invent or adopt pressure mechanics.

## Positive evidence

- Isolated reruns of `build_candidate_v2.py` and `adversarial_tests_v2.py` passed. Their four regenerated outputs match the frozen schema, mapping, coverage, and adversarial-record hashes byte-for-byte.
- The candidate schema is Draft 2020-12-valid; all 45 kind/unit branches require a branch-specific `owner_semantics` constant; unknown pairs and false owner/status relabels are rejected.
- The nonlinear witness contains 830 rows and preserves `source_record` JSON values 830/830. This is additive to the unchanged V1 evidence that covered all 2,348 current rows with native totals 786/830/813 and omissions 9/33/39.
- Checksum payload references, algorithms, values, and canonicalization labels remain distinct; V2 only preserves the V1 checksum payload and adds explanatory conformance claims.
- The candidate/API/compatibility status remains explicitly unadopted. No public schema, producer, consumer, pressure model, lifecycle record, or source implementation was changed.

## Review limit

This was a bounded design/code-evidence backcheck of the 14-output V2 successor. I ran focused isolated Python/schema probes and inspected the named current producer, report DTO/CLI ingress, Tauri bridge, and desktop consumers. I did not run the full harness, product tests, Cargo, or native builds, and I did not treat schema coverage as physical approval.
