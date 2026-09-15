# Compatibility contract and test basis — frozen V1

Status: `READY_FOR_WRITER_PENDING_ROOT_RELEASE`  
Role: `WORKING_ITEMS` / Type 1  
Parent: `HELP_HUMAN`  
Actual model/effort: `gpt-5.6-sol` / `high`  
Mechanism: delegated-harness-native; role and Type-2 non-delegation are instruction-asserted  
Baseline: `eff9a58dd712ff9673fa26b9fa809a2f725f6f97`  
Scope: DEL-14-02 / PKG-14 / SOW-072 / OBJ-016, plus the explicitly authorized DEL-17-06 / PKG-17 / SOW-046,074 / OBJ-007,017,018 consumer integration and localized PKG-07, PKG-08 and PKG-02 callers.

This derivative contract freezes the writer basis before product effect. It consumes the accepted results snapshot and D-67; it does not change decomposition 0.12, SCA-009, DAG-010/R5, lifecycle state, dependency satisfaction, release status or professional reliance. Root owns the combined acceptance, native witnesses, complete review/sweep, receipt and publication.

## 1. Version dispatch and legacy preservation

Readers shall dispatch on the exact top-level `schema_version`; unknown versions fail closed with an inspectable finding. New analysis and stress-neutral records use `0.2.0`. No new `0.1.0` record may be emitted by the current path.

For analysis records, preserve two distinct historical `0.1.0` profiles:

1. `analysis_run_python_sorted_compact_v0_1`: the full strict Python envelope and its sorted-key, compact, ASCII-escaped JSON projection (`json.dumps(sort_keys=True,separators=(",",":"),ensure_ascii=True)`).
2. `analysis_run_desktop_locale_v0_1`: the abbreviated desktop envelope and its recursive `Object.entries(...).sort((a,b) => a[0].localeCompare(b[0]))`, then `JSON.stringify`, projections. Preserve any received `rfc8785_jcs` label even though that label does not prove JCS bytes.

The baseline strict analysis schema bytes and strict stress-neutral schema bytes must be copied unchanged to version-named `*.v0.1.schema.*` files before the unversioned schema paths become exact dispatchers. The existing 0.1 stress-neutral Python producer, readers and three fixture files remain byte-identical. Desktop 0.1 analysis verification may be attempted only when the exact profile and required preimage/numeric representation are established. Missing preimage, ambiguous profile, or lost numeric-type evidence yields `unverifiable`; a recomputed preservation checksum never verifies or rewrites an old checksum. A known unequal digest remains `mismatch`.

## 2. Checked canonicalization profile

`openpipestress_jcs_ijson_v1` is additive around the existing Rust canonicalizer. Existing `canonical_json`, model hashes, native project hashes, input-manifest hashes and public operation behavior remain unchanged.

The checked profile accepts JSON null/boolean/string/array/object values, finite binary64 numbers, and integral values in `[-9007199254740991, 9007199254740991]`. It rejects duplicate object member names at every depth, malformed UTF-8, lone surrogate escapes, non-finite values, and unsafe integers. TypeScript rejects `undefined`, `bigint`, symbols, functions, non-finite numbers, unsafe integers and non-string-keyed/non-plain data before `JSON.stringify`. Python rejects non-string mapping keys, non-finite floats, unsafe integers and non-JSON values before transport. Negative zero is accepted and canonicalizes to `0`; `1` and `1.0` canonicalize to the same bytes.

The existing WASM exports stay unchanged. Add bounded checked exports named `canonical_json_checked_v1` and `canonical_sha256_hex_checked_v1`; only the new 0.2 record/package paths call them. The private executable is feature-gated in the canonicalization crate and resolved by an explicit configured path or a single documented build output path. It never searches `PATH`, invokes Cargo at runtime, uses a shell, or falls back to Python serialization.

The executable consumes one UTF-8 JSON document on stdin:

```json
{"protocol_version":"1.0.0","profile":"openpipestress_jcs_ijson_v1","items":[{"id":"caller-stable-id","json_text":"{\"a\":1}"}]}
```

It returns one UTF-8 JSON document on stdout with the same protocol/profile and ordered items of `{id,canonical_json}`. Duplicate item IDs, empty IDs, malformed requests or any invalid item reject the whole batch with nonzero status; stdout must not contain a partial success. Python computes SHA-256 over the returned canonical UTF-8 bytes. Setup builds the binary explicitly with the checked-CLI feature; absence or protocol/profile mismatch is an error.

## 3. Analysis record 0.2.0

The Python and desktop producers emit the same strict schema and field meanings. The received mechanics result is never mutated. Every source row appears once in `analysis_run.result_refs`, including physical quantities, reviews, basis/disclosure rows and diagnostics/work rows. Each reference carries:

- its exact source row reference and a separate `result_row` checksum over the complete received row;
- category, nullable physical source dimension, family and semantic-contract `{id,sha256}` derived from `fixtures/results/semantic_contract_v0_2.json` (`4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da`);
- explicit interpretation status/findings when physical meaning is unavailable;
- received metadata unchanged in a source-annotation carrier, separate from derived canonical metadata.

Known unit or component contradictions block creation. Missing optional physical metadata discloses; it is not invented. Diagnostic work remains diagnostic work and never becomes a moment or governing ratio. The raw producer DTO does not gain a dimension assertion.

Checksums are distinct:

- `received_result`: checked canonical bytes of the complete received mechanics envelope;
- `result_row`: checked canonical bytes of each complete received row;
- `analysis_run_record`: checked canonical bytes of the complete 0.2 envelope after removing exactly the checksum object whose `payload_scope` is `analysis_run_record` from `analysis_run.hashes`. Every other field and checksum remains in the projection.

An immutable revision is identified by the tuple `(mechanics run_id, analysis_run_record digest)`. This tuple need not create a circular embedded field. Changing rule status makes a new record/digest while preserving the received solve and its hashes exactly.

Current qualification requires a fresh successful solve plus exact current model and established current input-manifest evidence. Saved old or new records reopen Historical because persistence does not contain the historical manifest payload. Do not invent it. Exact saved preimages/profile evidence may still verify historical hashes. Save-without-change preserves attachments and bytes. Model edits, undo/redo transitions, cancellation, stale/busy guards and late responses cannot restore Current evidence. Result export, stress-neutral export, query, comparison, rule-check and report routes use one designation decision.

The input-manifest document remains `1.0.0` and retains its established hash/profile; 0.2 records reference that evidence rather than relabel or recompute it.

## 4. Stress-neutral 0.2.0

The 0.2 schema is strict and implements D-67/CLM-042. It repairs the ten frozen construction families: root, `stable_id_map`, manifest, diagnostics, privacy, received source checksums, professional boundary, export profile, validation report and loss report. Preserve all 830 received rows; produce 828 eligible unit-preservation witnesses and withhold exactly the two diagnostic-work witnesses with explicit findings. Presentation-only fields stay outside the exported payload.

Received checksum objects are retained byte-for-value as source evidence in a dedicated `received_source_checksums` representation. They are never inserted into the computed member-checksum array and never relabeled.

The declared materialized members are exactly:

1. `manifest.json`
2. `stress_neutral_results.csv`
3. `result_rows.json`
4. `unit_system_disclosure.json`
5. `unit_preservation_witnesses.json`
6. `stable_id_map.json`
7. `loss_report.json`
8. `validation_report.json`
9. `diagnostics.json`

For JSON members, the member projection is exactly the value written to that member file, hashed with `openpipestress_jcs_ijson_v1`. CSV remains normalized ASCII/LF text and is hashed over the exact emitted bytes. The manifest seed is the established manifest construction before its own checksum entry, including source refs, received checksum evidence, assumptions/reproducibility refs, export-profile ref, boundary notes, computed member checksums other than manifest, and diagnostics. The package projection is the complete assembled package after hash decoration with only the dedicated package-self-checksum carrier removed. No other omission is allowed.

Asynchronous completion must be generation-bound: replacing the source invalidates any pending result. Validate the final decorated payload against the 0.2 schema and reverify all declared member bytes/checksums immediately before download. `schema_conformant` and `validation_ready` are separate; blocked engineering/package diagnostics do not masquerade as a schema failure or a ready package. Controlled-export checks remain in force.

## 5. Frozen acceptance matrix

Focused acceptance must cover:

- checked canonical parity across Rust, WASM/TypeScript and Python for Unicode UTF-16 key order and escaping, `1`/`1.0`, negative zero, exponent boundaries, smallest/largest finite binary64, unsafe integers, non-finite values, duplicate keys and malformed Unicode;
- both legacy analysis profiles: exact available preimages, absent preimages, ambiguous/lost numeric type, true mismatch, immutable received labels and no JCS substitution for desktop locale projections;
- the complete 60-signature/47-kind semantic inventory, the nine former admission failures, four legacy interpretation mismatches, all-row accounting, contradictions, missing metadata and raw-row/result preservation;
- Python/desktop 0.2 schema parity, checksum projection tamper tests, immutable rule revisions and mixed 0.1/0.2 persistence round trips;
- Current/Historical agreement for query/export/report/comparison/rule-check/stress-neutral routes and the existing busy/stale/cancel/history/late-response guards;
- all ten stress-neutral construction families, 830 rows, 828 witnesses, two withheld work witnesses, exact nine-member inventory, emitted payload schema validation, missing/tampered member rejection and source replacement during pending hashing.

Use `/private/tmp/piping-foundation-20260914-venv/bin/python`. Focused pure Cargo/Python/TypeScript runs use isolated Rust target directories. No native build, GUI/browser action, full DEC-025 sweep, push or merge belongs to this manager. Raw failures are preserved under this instance. After two unsuccessful cycles without new evidence, return a fresh-diagnosis request rather than repeat the same approach.

## 6. Read-only adopted amendments and claim fence

The writer must not edit `D-67_RULING_2026-09-14.md`, `_DECISIONS/_REGISTER.md`, DEL-17-06 `ScopeOfWork.md`, or DEL-17-06 `_CONTEXT.md`. Their frozen hashes are in `LEGACY_EXPECTATIONS_V1.json`. Five active DEL-14-02 and twelve active DEL-17-06 dependency rows remain pending. This undertaking supplies bounded source and verification evidence only.
