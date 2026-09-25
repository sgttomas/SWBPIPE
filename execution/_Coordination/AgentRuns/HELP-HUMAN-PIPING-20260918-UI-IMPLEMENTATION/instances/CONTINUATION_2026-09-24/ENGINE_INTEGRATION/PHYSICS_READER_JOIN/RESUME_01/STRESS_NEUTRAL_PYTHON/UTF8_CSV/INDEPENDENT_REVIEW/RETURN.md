# Frozen three-file UTF8 review

**The UTF8 delta is clear within its exact three-file scope.** No remaining wire-contract finding. The separately reported dense diagnostic unitless/ratio witness-mapping defect remains open and requires its own frozen repair/backcheck; this return does not clear that defect or overall integration.

Reviewed the full Python package, schema and test deltas and affected rendering, validation, materialization and reopening calls. New methods require `csv_encoding=utf-8`, `csv_row_order=unicode_scalar_value_result_id`, and CSV checksum token `utf8_csv_record_lf_v1`. CSV record separators are LF, while quoted CR, LF and CRLF remain data. Strict decoding/encoding refuses file-prefix BOM, malformed/overlong UTF8 and surrogates without Unicode normalization. Authored U+FEFF inside fields, combining spellings, whitespace, quotes and commas are retained. Python scalar-value ordering correctly differs from UTF16 ordering for supplementary characters. The validator compares received spelling rather than silently rerendering or normalizing independently hashed bytes.

The local schema profile is exactly the legacy closed profile plus the two selected fields; its local reference shape is identical. Method branches require the new fields and checksum token for the three new methods, and forbid them for p1. Both CSV checksum locations remain constrained; the nine-member inventory and prior exact-bit sidecar/source bindings are preserved.

Independent bounded probes passed: Unicode scalar ordering and exact quoted field data; actual n05_unicode sparse81-row and dense82-row full nine-member materialization/reopening with source and analysis supplied; original request/mode receipt-domain binding; exact CSV byte digest; all source value-bit sidecars and metadata; five malformed-wire refusals in each mode; and unchanged p1 whole-package/all-nine-member goldens. [Probe results](UTF8_PROBES.json) bind actual source and member hashes. No new producer or native solve was performed.

Final frozen SHA256 values:

| File under `projects/chirality-piping/` | SHA256 |
|---|---|
| `core/handoff/stress_neutral/package_v0_3.py` | `8b07faea35dacabfda35f4537d708a57ba585e918760c988b550f1d3f9414921` |
| `schemas/stress_neutral_export.v0.3.schema.json` | `af3d229289c72349f2daa5b7826bd7f0be4924d6b854c6358d480c89d1d991dd` |
| `tests/test_stress_neutral_physics_source.py` | `82abba8a3fb720f4a78fed36bd45b23f0e78dc8981bd410220ebe6ede4fab3f5` |

Every preimage, diff reconstruction and initially reviewed current hash matched. The author's actual Unicode/p1 run passed3 with53 deselected, and all13 bound source/helper/fixture hashes plus raw log hash were checked. Its earlier pure28-pass run used an earlier test-file hash; that scope is preserved. That run also accidentally invoked a4.71-second units release build because the units environment variable was omitted. The deviation, command, binary hash and unaltered compile log are retained and are not described as no-build evidence. The reviewer supplied both authority paths and used direct Python imports, without pytest session startup or builds. [Checked basis](CHECKED_BASIS.json) preserves all exact evidence identities and any subsequent source drift separately.

TASK `/root/physics_resume/joined_consumer_review` retained actual parent `/root/physics_resume`; prior full role/Root/Piping/loop/software-code-review basis applies. Only UTF8_CSV/INDEPENDENT_REVIEW was written. No delegation, product edit, build, broad suite, new solver/native/browser/network execution or Git mutation. Original failure logs remain unchanged. TS Support mapping, final frontend supplement and the newly reported diagnostic witness delta are excluded. No native/current, whole-candidate CI, external compatibility or engineering qualification follows.
