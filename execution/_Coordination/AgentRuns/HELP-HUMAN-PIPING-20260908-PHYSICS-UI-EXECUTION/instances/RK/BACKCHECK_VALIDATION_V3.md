# RK K8 V3 backcheck validation

## Release and integrity

- Pre-work release record: `instances/RK/BACKCHECK_RELEASE_V3.md`.
- K8 V3 manifest: expected and observed SHA-256 `24d368585f1b1a7f01d79dd6201ae44fa0bfb20bc0aea72d41e5ca2c5ffcdf4c`.
- Manifest outputs: 16/16 expected hashes matched.
- K8 V2 manifest remained `ee4fcf91cf7eb818b2f85403c4819322df24d6070c94f62b7aeaee70e875725a`.
- K8 V1 manifest remained `a6ad4f8ab2881801a2ebabb483e5c22e4cafaa029e07cc398bf420013b576a65`.

## Isolated reproduction

The K8 evidence directory was copied to `{TASK_TEMP}/rk-k8v3-backcheck`, then these focused commands were run from the released checkout:

```text
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v3-backcheck/build_candidate_v3.py
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v3-backcheck/adversarial_tests_v3.py
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v3-backcheck/adversarial_tests_v2.py
```

All three passed. The regenerated V3 hashes match the frozen artifacts:

| Artifact | SHA-256 |
|---|---|
| `RESULTS_SCHEMA_0_2_0_CANDIDATE_V3.json` | `e7f2af0188c2a193da13627560cafe91c8c922ee89554566a945213348b62f96` |
| `SOURCE_KIND_MAPPING_V3.csv` | `4f6075b3fc78af2e56b5eb2bf47c521be544315fd15c661d2ae4989d43ca4751` |
| `COVERAGE_VALIDATION_V3.json` | `3c5186f4fcda57c441f51b6f7815b0e046f3a6714ed37a8bf1b405bcd3c35fea` |
| `ADVERSARIAL_VALIDATION_V3.json` | `97243b9fdd5db56d7a9211ebb5ce90c188b8eeef2ce54c6288fdc7da541f3adc` |

The released V3 suite passes five top-level JSON types, nine selected nested container types, 15 labeled ingress cases, unknown outer/inner versions, and the clean 830-row round trip. The unchanged V2 suite passes all 13 relation checks.

## Independent type and refusal probes

| Probe | Writer result | Reader result | Contract result |
|---|---|---|---|
| `source_record.kind: []` | uncaught `TypeError: unhashable type: 'list'` | same | `RESULT_EXPORT_SCHEMA_INVALID` |
| `source_record.unit: {}` | uncaught `TypeError: unhashable type: 'dict'` | same | `RESULT_EXPORT_SCHEMA_INVALID` |
| `source_record.kind: 7` | `RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED` | same | `RESULT_EXPORT_SCHEMA_INVALID` |
| `classification.semantic_status: 7` | `RESULT_EXPORT_CLASSIFICATION_MISMATCH` | same | `RESULT_EXPORT_SCHEMA_INVALID` before relation classification |
| non-string host object key | uncaught `AttributeError` in writer numeric walk | n/a: not a strict JSON object | stable writer rejection, no exception |

Raw version normalization independently returned:

- `RESULT_EXPORT_SCHEMA_VERSION_REQUIRED` for absent, null, and empty values;
- `RESULT_EXPORT_SCHEMA_INVALID` for boolean and array values;
- `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` for `9.9.9`;
- success for registered `0.2.0`, and for the report registry's `0.1.0`/`0.2.0` entries.

The three boundary labels in `adversarial_tests_v3.py` call the same normalizer, as the design requires. Actual production Tauri and CLI routing remains an implementation-stage obligation; the V3 design now states the raw-before-typed order, tagged outcome, exit behavior, and legacy handler binding precisely enough for that later implementation.

## Schema, mapping, and pressure regression

- V3 schema content equals V2 after excluding `$comment` and `description`.
- V3 and V2 mapping CSVs are byte-identical: 45 rows and 45 unique pairs.
- All 45 schema branches require the pair-specific `owner_semantics` constant.
- The three pressure rows remain P5-pending, owned by `PKG04_05_MECHANICS`, and interpreted as `preserve_only` until a later accepted semantics change.

No source or author artifact was modified.

## Final-freeze hygiene

- `python tools/validation/validate_candidate_whitespace.py`: PASS; zero skipped untracked binary/symlink paths.
- `python tools/validation/validate_path_anchors.py .`: no RK finding. Its sole remaining finding is outside RK in `instances/R/BACKCHECK_VALIDATION_V2.md`; that sibling record is read-only for this review and remains untouched.
