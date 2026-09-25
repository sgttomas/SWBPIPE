# New-method UTF8 CSV — Python return

TASK `/root/physics_resume/physics_reader_join/physics_rust_export` implemented
only the Python0.3 stress-neutral package, its schema, its focused test file and
this evidence. Prior CR02 review/repair evidence remains unchanged. The original
two TypeScript Unicode failures remain byte-for-byte in
`ORIGINAL_TS_FAILURES.log`, with original location/hash in `EXECUTION_BASIS.json`.
No producer fixture, raw result/header/quality, JCS profile, semantic table,
source validator, other schema, p1/legacy builder, browser or native code was
changed. No delegation occurred.

The shared final contract is:

- For source-blocks-1, physics-1 and physics-source-1 only,
  `export_profile.csv_encoding = "utf-8"` and
  `export_profile.csv_row_order = "unicode_scalar_value_result_id"` are required.
- Their CSV checksum canonicalization is **`utf8_csv_record_lf_v1`**. This
  supersedes the initial proposed `normalized_utf8_lf_text` label.
- Record separators are LF. Quoted field CR, LF and CRLF are retained exactly;
  there is no generic `canonical_csv` whole-text cleanup on this new branch.
  Commas, double quotes and embedded CR/LF use ordinary quoted CSV fields with
  doubled quotes. IDs sort lexicographically by Unicode scalar value, matching
  Python string order, not UTF16 code-unit order.
- UTF8 encoding/decoding is strict, without a file-prefix BOM, unpaired
  surrogates or Unicode normalization. U+FEFF within an authored field is data,
  including at the beginning of an ID, and is retained. The exact emitted CSV
  bytes are hashed; received spelling is validated without replacing those
  independently hashed bytes.
- Precision-1 forbids both new profile fields and retains
  `normalized_ascii_lf_text`, its existing ASCII behavior, complete package
  checksum and all nine member byte hashes. Original830/828 semantics are not
  changed. CR02's five-key annotation/value-bit sidecar contract is retained.

The0.3 schema uses a local extended copy of the closed legacy ExportProfile so
the legacy schema is untouched; method branches require/forbid the new fields
and constrain both CSV checksum locations to the corresponding token. The
nine-member inventory is unchanged.

## Checks

`pure-utf8-01.json` / `.log`: **28 passed**,28 deselected. Checks quoting,
delimiters, quoted CR/LF, Unicode scalar ordering, composed/decomposed Unicode
without normalization, authored U+FEFF preservation, BOM/record-CR/surrogate
refusal, explicit labels, p1 byte identity and the existing CR02 bit vectors.
These are pure transport controls, not synthetic passing solver results.

`actual-unicode-01.json` / `.log`: **3 passed**,53 deselected, exit0 in13.66s.
The two actual `n05_unicode` modes use their maintained original request and raw
outputs, verify the captured request/mode hash domain, schema/CSV byte hashes,
all-nine-member readback, row order, unchanged metadata/value-bit sidecars,
malformed UTF8, BOM and bad encoding/checksum labels. The third check repeats
the p1 whole-packet/all-nine-member byte golden. Executables were explicitly
pinned to the existing checked-JSON and units binaries. No compile output
occurred in this actual run; final before/after input hash comparison had no
drift. Its raw-log SHA256 is
`2f3d1a537c30a3a9d3490712cb15f4ad8b7217a24f97a63edb980232641704d8`.
The counts overlap on the p1 golden and are not31 distinct tests.

## Recorded execution deviation

The initial pure Python run omitted `OPENPIPESTRESS_UNITS_BIN`. The current
`tests/conftest.py::pytest_sessionstart` consequently invoked
`tools/units/build_units_authority.py::build`, which ran:

```text
cargo build --locked --release --features cli --bin openpipestress_units --target-dir /private/tmp/piping-engine-integration-20260925/projects/chirality-piping/core/units/target/units-authority
```

Its cwd was `projects/chirality-piping/core/units`; the log reports4.71seconds.
This implicit build violated the no-build condition and was reported immediately
to the parent after it exited. The full compile log remains unaltered. The
resulting release binary hash is
`813a0dd5f6dcbe3454b0ea52a38463a5cd4e6b3554368d23f0133e8f3a31d8dc`.
The exact helper/fixture hashes and target are retained in `pure-utf8-01.json`.
No cache/log was deleted or rewritten to hide the build. The subsequent
parent-authorized actual run supplied **both** pinned executable variables,
preventing another auto-build. No no-build claim is made for the initial run.

## Frozen contribution

`SOURCE_MANIFEST.json` binds:

- package: `8b07faea35dacabfda35f4537d708a57ba585e918760c988b550f1d3f9414921`
- schema: `af3d229289c72349f2daa5b7826bd7f0be4924d6b854c6358d480c89d1d991dd`
- tests: `82abba8a3fb720f4a78fed36bd45b23f0e78dc8981bd410220ebe6ede4fab3f5`

No source/test changes followed the passing actual run. Parent owns matching
TypeScript integration and independent UTF8 review. This remains an unmerged
candidate, with no Current/native qualification or programme completion claim.
