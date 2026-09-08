# RK K8 V4 backcheck validation

## Integrity and reproduction

- Release record: `instances/RK/BACKCHECK_RELEASE_V4.md`.
- K8 V4 manifest observed at required SHA-256 `a285a7aee0f276919c16db28fb74926257735b9b41dd9c7cc66b554dba1a0941`.
- V4 outputs: 18/18 manifest hashes matched.
- V3 predecessor: manifest SHA-256 `24d368585f1b1a7f01d79dd6201ae44fa0bfb20bc0aea72d41e5ca2c5ffcdf4c`; outputs 16/16 matched.
- Released validator and adversarial-test hashes matched `1ef2e0f7730f3f2590a7e72167f1ce1f893e550e99478a0702eb62cf572f93f9` and `6ced8f265facdee383700cbaafde3da744e0857a2fae0cc80a203a7fa1ee3334`.

The evidence directory was copied to `{TASK_TEMP}/rk-k8v4-backcheck`. Focused commands:

```text
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v4-backcheck/build_candidate_v4.py
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v4-backcheck/adversarial_tests_v4.py
uv run --with jsonschema python3 {TASK_TEMP}/rk-k8v4-backcheck/adversarial_tests_v3.py
```

All passed. Regenerated outputs matched:

| Artifact | SHA-256 |
|---|---|
| `RESULTS_SCHEMA_0_2_0_CANDIDATE_V4.json` | `398b08671e75f850859a9ccac3b5f071b46d5d5b84f96819398fcc2a2c879252` |
| `SOURCE_KIND_MAPPING_V4.csv` | `4f6075b3fc78af2e56b5eb2bf47c521be544315fd15c661d2ae4989d43ca4751` |
| `COVERAGE_VALIDATION_V4.json` | `234f43284e0dd4961a9e19beaa717031bd6f22eca67ddc387e184aaba34c6e8c` |

## Released matrix

The V4 suite independently reported:

- 52 relation-consumed fields;
- null, boolean, number, string, array, and object substitutions;
- 312 substitution cases and 624 writer/reader calls;
- 830-row exact source-record round trip;
- zero uncaught exceptions and zero test writes;
- retained duplicate, mirror, metadata, reference, owner, unknown-pair, finite-number, duplicate-member, version-ingress, and pressure regressions.

## Independent probes

| Probe | Writer | Reader |
|---|---|---|
| kind array | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| unit object | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| numeric kind | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| owner array | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| metadata component object | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| object-ref ID array | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| wrong kind type plus mirror mismatch | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| well-typed unknown pair | `RESULT_EXPORT_NATIVE_KIND_UNIT_UNSUPPORTED` | same |
| mirror ID mismatch | `RESULT_EXPORT_SOURCE_RECORD_ID_MISMATCH` | same |
| false PKG04/05 owner | `RESULT_EXPORT_CLASSIFICATION_OWNER_MISMATCH` | same |
| well-typed unknown version | `RESULT_EXPORT_SCHEMA_VERSION_UNSUPPORTED` | same |
| unknown version plus nested wrong type | `RESULT_EXPORT_SCHEMA_INVALID` | same |
| non-string host object key | `RESULT_EXPORT_SCHEMA_INVALID` | reader not applicable |

Ingress probes for a non-object, array, non-string host key, wrong-typed version, and supported version returned structured schema-invalid or success results with no exception.

## Frozen design retention

- V4 schema equals V3 after excluding `$comment` and `description`.
- V4 and V3 mapping files are byte-identical: 45 rows and 45 unique pairs.
- V3 outputs remain 16/16 hash-exact, preserving the closed raw/optional version ingress, tagged Tauri/CLI refusal mapping, and legacy 0.1 reader contract.
- The three pressure rows retain their exact P5-pending status and `PKG04_05_MECHANICS` ownership; lossless transport remains `preserve_only` and standardization remains owner-gated.

No source, author artifact, public contract, or physics semantics was modified.
