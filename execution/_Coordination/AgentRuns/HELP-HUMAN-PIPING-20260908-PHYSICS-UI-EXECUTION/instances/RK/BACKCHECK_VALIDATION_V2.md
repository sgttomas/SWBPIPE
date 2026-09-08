# RK K8 V2 backcheck validation

## Release and integrity

- Pre-work release record: `BACKCHECK_RELEASE_V2.md`, SHA-256 `810523fd4093993a2cff36746602e7c252e0bbd87d55f6d2ee7671adf48bc44d`.
- K8 V2 manifest: expected and observed SHA-256 `ee4fcf91cf7eb818b2f85403c4819322df24d6070c94f62b7aeaee70e875725a`.
- Manifest outputs: 14/14 expected hashes matched.
- K8 V1 manifest: SHA-256 remained `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65`; all original V1 outputs remained byte-identical.

## Isolated reproduction

I copied the K8 evidence directory to `{TASK_TEMP}/rk-k8v2-backcheck` and ran from the released source checkout:

```text
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v2-backcheck/build_candidate_v2.py
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v2-backcheck/adversarial_tests_v2.py
```

Both commands passed. Regenerated hashes:

| Artifact | Regenerated SHA-256 | Frozen match |
|---|---|---|
| `RESULTS_SCHEMA_0_2_0_CANDIDATE_V2.json` | `fd7f8202b7b04b18cf457c8c07724fa92c51dae2138222055f1ee22b385b9e32` | yes |
| `SOURCE_KIND_MAPPING_V2.csv` | `4f6075b3fc78af2e56b5eb2bf47c521be544315fd15c661d2ae4989d43ca4751` | yes |
| `COVERAGE_VALIDATION_V2.json` | `c47c8e4b74d6a0966bdf56df43f1e75930c4378aa326d98d6ab1aa957af903e5` | yes |
| `ADVERSARIAL_VALIDATION_V2.json` | `223ac364b8448365d8194f53616676501e0b3d68a3337b9c72fef7f5a4e3b473` | yes |

The clean witness passed writer serialization and strict reader reparse with 830 rows and exact native `source_record` equality 830/830. The released 13-case suite passed duplicate-ID, mirror, metadata presence, object/basis/reference ordering, NaN/infinity, false owner, unsupported pair, row-status dominance, and round-trip probes.

## Independent focused probes

| Probe | Writer | Reader | Expected contract result |
|---|---|---|---|
| top-level `[]` | `AttributeError: 'list' object has no attribute 'get'` | same | stable validation error, no exception |
| top-level `null` | n/a | `AttributeError: 'NoneType' object has no attribute 'get'` | stable validation error, no exception |
| valid-version document with `classification: null` | not repeated after reader failure | `AttributeError: 'NoneType' object has no attribute 'get'` | `RESULT_EXPORT_SCHEMA_INVALID`, no exception |
| false registered owner | `RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH` | same | pass |
| altered registered status | `RESULT_EXPORT_CLASSIFICATION_MISMATCH` | same | pass |
| outer version `9.9.9` | `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` | same | pass |
| inner version `9.9.9` | `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` | same | pass |

The exception path is caused by semantic relation access preceding schema validation in `validate_document` and by unguarded `.get` calls in `validate_semantics`.

## Conceptual V1-to-V2 delta checked

- Added a top-level `QuantityResult.source_result_refs` compatibility projection; the runtime validator enforces exact presence and ordered JSON equality to `source_record.source_result_refs`.
- Added `owner_semantics` to the 45-row mapping and required the pair-specific owner constant in all 45 schema branches.
- Added one mandatory semantic validator at writer and reader boundaries, strict JSON parsing/serialization, stable diagnostic codes, and well-shaped adversarial cases.
- Added exact candidate API, registry, ownership, migration, Tauri, desktop cache, report consumer, and pressure sequencing records.
- Preserved V1 evidence and candidate-only status.

## Current-consumer evidence checked

- `core/reporting/report_package/src/wire.rs` currently derives `Deserialize` for `ReportPackageRequest` with ordinary required `String` members.
- `core/runner/headless/src/bin/openpipestress-runner.rs` deserializes raw `export_results` JSON to `ReportPackageRequest` before calling `assemble_wire_request`; DTO failure becomes `HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID`.
- `apps/desktop/src-tauri/src/lib.rs` exposes typed request parameters at the Tauri command boundary.
- `apps/desktop/src/features/result-export/ResultExportPanel.tsx` currently has its own `buildResultExportPacket`, and `apps/desktop/src/features/report/reportPackageRequest.ts` independently constructs the current result envelope. The V2 single-document migration names both actual consumers correctly.

No source was modified and no public compatibility or physics decision was made.

## Path-anchor check

After replacing all literal host/temp paths in the RK successor records with repository-relative paths and `{TASK_TEMP}`, I ran:

```text
python tools/validation/validate_path_anchors.py .
```

The validator reported four pre-existing findings outside `instances/RK/**`, in `instances/M9/HANDOFF.md`, `instances/M9/RUNTIME_SUMMARY.json`, `instances/RM/REVIEW.md`, and `instances/RP/LAUNCH_BRIEF.md`. It reported no RK finding. Those sibling paths were read-only and remain untouched.
