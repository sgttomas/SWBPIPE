# CANDIDATE — DEL-08-04 route-A complete result schema and migration design

Identifier: `K8-ROUTE-A-DESIGN-20260908-V1`. Status: implementation-ready candidate; not adopted, public, or implemented. Accepted source is `779dedb8670625b36af07b89fc5557470e47c50e`. This record consumes the accepted W10 V1 repair brief and controlling V2 clarification. W10 V3/V4 changed evidence representation only; the original hashes remain resolved by `PORTABILITY_RELOCATION_V3.json` and `WITNESS_RELOCATION_V4.json`.

## Result

The route-A candidate is schema version `0.2.0`, represented completely by `{RUN_ROOT}/instances/K8/evidence/RESULTS_SCHEMA_0_2_0_CANDIDATE.json`. It makes every actual native `ResultItem` carryable, including rows the current `0.1.0` mapper discloses instead of exporting. It preserves the current top-level result fields for consumer migration and adds two required fields:

- `source_record`: the exact native `ResultItem` JSON object, retaining `id`, `kind`, `value`, `unit`, `entity_ref`, optional `basis_ref`, optional `source_result_refs`, and optional five-string metadata without normalization;
- `classification`: a closed transport classification that says whether the existing canonical category is preserved, source-specific metadata is standardized, source semantics remain unstandardized, pressure meaning remains pending P5/PKG04-05, or the value is only a solver observation.

This split is the core safety property. `source_record` is controlling if a normalized label disagrees. The broad `family` and `dimension` fields remain available to current-shaped consumers, but they cannot erase or override the native source record. The new `semantic_status` field prevents broad family labels from being read as a new mechanics ruling.

The schema uses JSON Schema Draft 2020-12 conditionals and exact `const` values to bind each observed `(kind, unit)` pair to one transport family/family/dimension/status tuple. The applicable primary specification is the [JSON Schema Draft 2020-12 core specification](https://json-schema.org/draft/2020-12/draft-bhutton-json-schema-01).

## Exact observed coverage

W10's complete row inventory contains 2,348 currently emitted rows: 2,010 schema-valid identity rows and 338 rows with one or more invalid metadata fields. Those 338 rows account for 518 metadata-field violations. Each accepted actual document also contains two invalid `rfc8785_jcs` checksum-label occurrences, producing W10's full-document error totals of 160/212/152.

The current binder discloses omitted native rows one-for-one. K8 reads those exact disclosure IDs from the accepted documents rather than rebuilding W10's million-byte inventories:

| Scenario | Current emitted | Exact omitted | Native total | Accepted document SHA-256 |
|---|---:|---:|---:|---|
| linear | 777 | 9 | 786 | `c150b35c4ca35bc6145834510a33cf22269b540998fe7919fb177b8edfa686d3` |
| nonlinear | 797 | 33 | 830 | `ad121d3859d1159edae80f20175e649462a6d2c112a069c70743b87febf30be9` |
| zero pressure | 774 | 39 | 813 | `ecdfbb7e5cf1cb22a4e1776a31115d9991dde11a3f58f839d5e77e4744b0632d` |

The exact omitted result IDs are frozen under `actual_documents.*.omitted_result_ids` in `COVERAGE_VALIDATION.json`. They comprise solver-mode rows; component stiffness; hanger stiffness/travel; constant-effort travel; nonlinear active-set counts, flags, states and residual observations; and, in the zero-pressure witness, 30 explicitly disclosed longitudinal-pressure-stress rows. The candidate's 45-row `SOURCE_KIND_MAPPING.csv` is the complete distinct `(kind, unit)` mapping across the full 830-row native witness plus all three documents' disclosures.

The candidate validation migrates the full accepted nonlinear native witness in memory. All 830 `source_record` objects equal the original native JSON values; `result_id`, magnitude and unit are equal; source metadata is equal when present; and the resulting complete document passes the candidate schema. No transformed giant witness is duplicated in the repository.

## Source-family variants and semantic boundary

| `source_family` | Candidate behavior | Meaning boundary |
|---|---|---|
| `element_response` | Preserve accepted force/moment/stress family and dimension while carrying the entire native row. | Existing accepted category only; no new recovery equation or sign convention. |
| `nodal_kinematic` | Add exact displacement/rotation component coverage while preserving the native component, frame, location, basis and sign strings. | Axis/component metadata is transport structure; mechanics meaning remains PKG04/05-owned. |
| `support_response` | Carry resultants and nonlinear final displacement/reaction rows with exact support, DOF, policy, final-state and source-link strings. | Does not equate combined nodal reaction with isolated spring/friction action. |
| `pressure_response` | Preserve current hoop/longitudinal/thrust source rows and mark P5-pending semantics explicitly. | No new wall/effective-force, pressure-reference, closure-transfer, boundary or constitutive meaning. P5/PKG04-05 owns the mechanics decision. |
| `user_review` | Carry component multiplier, component stiffness, hanger and constant-effort review values with exact provenance-bearing metadata. | Review inputs/echoes remain review evidence; they are not silently promoted to solved reactions or code results. |
| `solver_observation` | Carry solver mode, counts, flags, states, deltas and residual observations under a distinct family. | Transport/diagnostic evidence only. `N*m` free-DOF work residual is `energy`, never moment. |

`semantic_status=source_specific_unstandardized*` is deliberate. Complete canonical coverage means every native value is represented and traceable. It does not require PKG08 to invent normalized mechanics meaning. PKG04/05 owns mechanics/result interpretation; PKG13 owns physical-to-analytical/source bridge meaning; PKG10 owns adapter behavior; PKG14 owns run/manifest provenance; PKG08 owns this schema and export vocabulary.

## Version and negotiation contract

The candidate uses exact outer and inner `schema_version: "0.2.0"` constants and a required `version_contract`:

```json
{
  "schema_id": "https://openpipestress.org/schemas/results/0.2.0/results.schema.json",
  "document_version": "0.2.0",
  "minimum_reader_version": "0.2.0",
  "unknown_version_policy": "reject_unsupported_schema_version"
}
```

The implementation contract is:

1. A writer accepts an explicit requested result-schema version at the adapter boundary. Absence uses the Owner-selected default; it never guesses from consumer fields.
2. The writer emits exactly one version per result document. Root and nested versions must match exactly.
3. A supported-version table is closed. A request for an unimplemented version returns structured `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED`, includes requested and supported versions, emits no canonical result document, leaves the complete native mechanics envelope accessible, and does not rewrite numerical or status fields.
4. A reader validates the version before reading any result row. Unsupported versions are refused for interpretation, report generation, adapter conversion, hashing as a governed result document, or UI rendering.
5. An optional forward-compatibility store may retain an unsupported document as opaque bytes with its original hash, but it cannot parse, render, compare, convert, or call it valid. That behavior is a separate Owner choice in the audit-D05 packet.

## Checksum and canonicalization boundary

The candidate adds `canonicalization_claim` to each checksum. `rfc8785_jcs` remains a legacy implementation label paired with `implementation_label_only_conformance_not_established`; it is not rewritten to `JCS`. `JCS` is reserved for a separately evidenced `rfc8785_conformant` claim. RFC 8785 defines JCS over I-JSON-compatible data, ECMAScript primitive serialization and recursive property sorting; renaming a project label does not prove those conditions. See [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html).

For migration, every referenced checksum's `algorithm`, `payload_ref`, and `value` stays byte-for-byte equal. Existing canonical serializer algorithms and the model/mechanics digest payloads stay unchanged. The new enclosing `0.2.0` result document has different bytes and therefore receives a new enclosing-document digest wherever one is produced; an old enclosing digest must never be relabeled as covering new bytes.

## Current consumer impact and assigned work

| Surface | Observed current behavior | Required migration | Owner |
|---|---|---|---|
| `schemas/results.schema.yaml` and `tests/test_results_schema.py` | One unversioned schema path, semver pattern, closed five-field metadata enums. | Install adopted versioned schema path, exact version test, source-record equality tests, all 45 pair checks and unknown-version refusal. Keep `0.1.0` immutable as historical schema. | PKG08 |
| `core/reporting/result_export/src/lib.rs` | Typed `ResultFamily`, `DimensionId`, `ResultMetadata`; complete-string checks; one serializer. | Add versioned DTOs/dispatch, `source_record`, `classification`, new transport categories and checksum claim. Validate before serialization. | PKG08 |
| `core/runner/headless/src/result_envelope_binding.rs` | Bounded kind/unit table exports 777/797/774 and discloses 9/33/39; fixed `0.1.0`. | PKG10 adapter requests adopted version and maps every native row exactly once. No source row may be dropped, duplicated or inferred. Mechanics-specific mapping must consume PKG04/05 and PKG13 accepted inputs. | PKG10 with PKG04/05 and PKG13 inputs |
| `core/runner/headless/src/lib.rs` and runner CLI | Library-only `result_envelope_document` is skipped by serde; CLI solve selects runner/mechanics envelopes; `export-results` consumes a report-package request. | Preserve current serialization until the adopted public route says otherwise. Add structured version request/refusal at the chosen public boundary; no implicit solve-output exposure. | PKG10 |
| `core/reporting/report_package/src/wire.rs`, `src/lib.rs`, tests | DTO converts directly to the single Rust envelope type and packages result documents. | Dual-read selected legacy/current versions during migration; package the original versioned document; reject unsupported versions before report generation. | PKG08 reporting, coordinated through root |
| Desktop `ResultExportPanel.tsx` | Independent ad hoc `0.1.0` producer maps every native row heuristically, defaults unmatched kinds to `ratio`, and copies raw metadata into closed enum slots. | Replace heuristic producer with adopted versioned adapter output. Remove fallback-to-ratio. Display source-specific and transport-only statuses without converting them to mechanics conclusions. | consuming package routed by root; PKG10 adapter contract |
| Desktop `reportPackageRequest.ts` | Independent `1.0.0`-labeled result-envelope shape and heuristic family/metadata projection. | Consume the adopted result document rather than regenerate one; preserve version and exact source records. | consuming package routed by root; PKG08 contract |
| Desktop results interpretation and result views | Read native `MechanicsResult`, not the library-only canonical field. | Continue native viewing; when consuming canonical output, branch on supported version and render `semantic_status`. Unknown versions fail closed or remain opaque per Owner ruling. | consuming package routed by root |
| Adapter/handoff/native-package references | Mostly carry `result_envelope_ref` or schema path without parsing rows. | Add schema ID/version to refs; preserve original version; reject conversion where target adapter has no explicit version capability. | PKG10 adapter; PKG14 manifest/provenance |

Search found no app/tool/test read of `PreviewRunnerOutput.result_envelope_document` outside its Rust producer/tests. This supports W10's finding of no known urgent current library consumer; it does not prove no external Rust library consumer exists.

## Implementation order after Owner ruling

1. PKG08 freezes the adopted schema version and version registry; `0.1.0` remains immutable.
2. PKG04/05 and P5 supply accepted semantics for every pressure-pending row; PKG13 supplies accepted source/bridge mapping where a normalized reference crosses the physical-to-analytical boundary. Unresolved rows remain explicitly source-specific and still travel losslessly.
3. PKG08 implements versioned Rust DTOs/validation/serialization and exact source-record equality checks.
4. PKG10 implements version request negotiation and the exhaustive 45-pair adapter. It removes the current omitted-row path only for the adopted `0.2.0` writer.
5. PKG14 binds manifest and enclosing-document digest behavior without changing referenced model/mechanics payload digests.
6. Each in-repo consumer either declares support for `0.2.0` or returns the structured unsupported outcome. Independent desktop generators are retired or made explicit legacy-only fixtures.

No step may commence from this candidate alone. Public compatibility adoption is audit D05/Owner-gated.

## Executable acceptance checks

An implementation is ready for review only when all checks pass:

- Draft 2020-12 meta-schema check passes; outer version, inner version, schema ID and negotiated version are exact and equal.
- The three current source scenarios yield exactly 786/830/813 `0.2.0` values. Their current 777/797/774 emitted subsets retain identical `result_id`, magnitude, unit, object/basis/station refs, metadata, diagnostics and source links.
- All exact omitted IDs in `COVERAGE_VALIDATION.json` appear once in `0.2.0`; no current or omitted native ID appears twice.
- Every emitted row's `source_record` equals its native `ResultItem` JSON value, including field absence versus presence and source-result-ref order.
- The 45 observed `(kind, unit)` pairs map exactly once; an unregistered pair returns `RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED` and preserves the native envelope.
- All 518 previously invalid metadata occurrences validate only because they are explicitly native strings plus a separate classification; none is coerced to an unrelated enum or `TBD`.
- Pressure rows retain their exact native metadata/source refs and P5-pending status until the owning mechanics decision is adopted.
- Referenced checksum `algorithm`, `payload_ref`, `value`, native row values and native serializer outputs are unchanged. Legacy `rfc8785_jcs` never produces an RFC-conformance claim without separate JCS vectors/conformance evidence.
- Readers reject or opaquely quarantine unsupported versions according to the Owner-selected policy before interpretation. No fallback version, heuristic family, or default ratio is allowed.
- Report, desktop, headless, adapter and manifest tests exercise supported `0.1.0`, supported `0.2.0`, requested unsupported version and future unknown version.
- The final source diff receives the project-required fresh read-only review and DEC-025 checks before publication.

## Candidate evidence

- `{RUN_ROOT}/instances/K8/evidence/RESULTS_SCHEMA_0_2_0_CANDIDATE.json`
- `{RUN_ROOT}/instances/K8/evidence/SOURCE_KIND_MAPPING.csv`
- `{RUN_ROOT}/instances/K8/evidence/COVERAGE_VALIDATION.json`
- `{RUN_ROOT}/instances/K8/evidence/build_candidate.py`
- `PUBLIC_COMPATIBILITY_DECISION.md` in this run record

These are derivative design evidence, not decomposition truth or an accepted schema.
