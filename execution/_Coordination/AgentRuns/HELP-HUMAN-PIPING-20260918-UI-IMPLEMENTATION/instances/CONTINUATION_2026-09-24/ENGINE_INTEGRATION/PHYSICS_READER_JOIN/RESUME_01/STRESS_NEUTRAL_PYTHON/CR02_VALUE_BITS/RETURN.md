# CR02 repaired — Python stress-neutral value-bit transport

The confirmed failure is preserved unchanged in
`ORIGINAL_STRESS_TRANSPORT_PROBE.json`:31 negative-zero row values in the actual
mixed sparse source became positive-zero after materialization/reopening, while
validation against the supplied original source still passed. The original probe
script, origin hashes, and all three preimages remain in this folder. The earlier
four-key packet result is historical evidence, not a successful bit-retention
claim.

The parent-selected repair adds required `source_value_bits` to every non-p1
annotation. `source_row.value` is now explicitly canonical numeric data (+0 for
either received zero sign); the sidecar records the actual received finite
IEEE754 binary64 bits before serialization. The five-key contract, decode rules,
source comparison and hash boundary are detailed in `CONTRACT_DELTA.md`.

Standalone validation rejects malformed/nonfinite bit patterns, a decoded value
that differs from the canonical number, and noncanonical negative zero in the
annotation numeric field. Whole-package hashing binds the sidecar. Validation
with an independently supplied raw source recomputes the original bit strings,
so a coherently rehashed +0/-0 substitution no longer matches that source. This
is source consistency, not fresh Current or producer authentication.

No JCS/shared-profile change, precision-1 field or byte change, additional member,
numeric oracle relaxation, producer result/header/quality edit, other validator
edit, Cargo/npm build, native/browser action or delegation occurred. Other raw
row properties, metadata, absent/null distinctions and source references retain
their existing representation. Scope is the original owned Python package,
stress-neutral schema, focused test file and this evidence only.

Observed checks:

- `pure-values-01.json` / `.log`: **19 passed**,26 deselected, exit0. Covers ±0,
  normal and minimum-subnormal transport vectors, malformed/nonfinite/mismatched
  sidecars, and exact pre-change p1 whole-packet/all-nine-member byte hashes.
- `ACTUAL_ANNOTATION_ONLY.json`: both actual mixed modes preserve every sidecar
  through checked JCS annotation serialization; each has31 negative-zero rows.
  Sparse has191 rows/114 positive zeros; dense193 rows/115 positive zeros. This
  narrower probe is explicitly not presented as the complete package test.
- `mixed-roundtrip-01.json` / `.log`: **3 passed**,42 deselected, exit0 in41.80s.
  Covers actual sparse/dense full nine-member materialization and reopening,
  retained zero sidecars, canonical+0 JSON numbers, schema conformance, missing
  sidecar refusal, checksum refusal before rehash, original-source refusal after
  coherent sign-flip rehash, and the p1 golden again. The p1 check is intentionally
  repeated across stages; these counts do not claim22 distinct tests.

The full actual check ran only after the CR01/CR03 validator owner declared a
stable window. Its command, all helper/fixture/owned-file hashes and both
executables are recorded: checked JSON
`bc1343ef45704c0ec65c1c7ac244c50e349af49de3a9b850dc11578bb2fc6c54`,
units authority
`fe28924fb3c0ba61e8789b65771c512da890ab4950406042cdb2c8bff55de464`.
The final input-hash comparison shows **no drift**. Raw actual-run log SHA256 is
`7bd0b8d1370ddb448bddf153af8833a71cd569b76dd764c55668735416a7aef1`.

Frozen contribution (`SOURCE_MANIFEST.json`):

- package: `1e833ff9efd2ec72482a84cb1fa622615e777ef412555b3a31d5470fe0ada0ab`
- schema: `3e6b89643324ba1b22efa0918e8a511500e199ca367250bc30574c948ba9a994`
- tests: `e12b2b243f48a3361a89443d837d8e33c31abf9f26d623b098757fef2f092c97`

No source/test edits followed the passing full check. The parent owns TypeScript
agreement and independent reviewer backcheck; no independent review or whole
programme completion is claimed by this implementation return.
