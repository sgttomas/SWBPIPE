# CR02 — retained source value bits

Parent selected this bounded correction after independent review found31 actual
mixed-source negative-zero values changed to positive-zero during JCS
materialization/readback while source-bound validation still passed.
`ORIGINAL_STRESS_TRANSPORT_PROBE.json` and `ORIGINAL_probe_stress_transport.py`
preserve the exact independent failure evidence; `EXECUTION_BASIS.json` records
its original location/hash and all three owned preimages. That failure is not
erased or reclassified as a passing original implementation.

Only non-precision annotations change. The final closed annotation has five keys:

- `source_row_index`: original raw row index.
- `source_result_id`: exact original result ID.
- `source_row`: original row properties, with its numeric `value` in canonical
  JCS form. In particular, either received zero sign is represented as +0.
- `source_row_sha256`: the existing checked-JCS row checksum. JCS intentionally
  maps both zero signs to the same canonical number; this digest alone does not
  preserve the received zero sign.
- `source_value_bits`: exactly16 lowercase hexadecimal digits encoding the
  actual received finite IEEE754 binary64 value, captured before serialization.

For a nonzero canonical number, the sidecar must match its exact binary64 bits.
For a zero canonical number, the annotation value must be +0 and the sidecar may
be only `0000000000000000` or `8000000000000000`. Nonfinite bit patterns,
malformed/uppercase/short strings, booleans, other decoded values, and a negative
zero in `source_row.value` itself are rejected. This leaves one explicit place
for the received sign and avoids claiming raw JSON alone is bit-preserving.

The original source-bound comparison now includes the recomputed sidecar string,
so coherently rehashing a zero-sign substitution cannot match the independently
supplied raw source. Standalone transport may carry either internally consistent
zero sign, but its whole-package checksum binds that choice; the packet is not
producer authentication. Other row fields, metadata, absent/null properties and
source result references retain their existing treatment. No reconstructed fake
raw envelope is used.

No change is made to JCS, the shared checked profile, precision-1 annotations
(which remain absent), precision-1 field/member bytes, semantic table identity,
physics/numerical criteria, or the nine-member inventory. The sidecar travels in
existing manifest package metadata and participates in the complete-package hash.
It is not a tenth member and does not invent a new row-value physical oracle.

This correction supersedes the four-key annotation description in the prior
`CONTRACT.md`; that earlier record remains historical. Matching TypeScript is
owned by the manager. Actual-source/full-transport tests require the separately
owned CR01/CR03 validators to reach a stable boundary; pure bit-vector checks do
not invoke those moving validation paths.
