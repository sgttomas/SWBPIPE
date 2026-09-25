# Pure stress-neutral method projection contract

`stress-neutral` retains stress results in a solver/vendor-neutral packet. It
makes no code, supplier compatibility, Current, solver-validation or professional
acceptance claim. Parent clarified that no stress-exclusion policy was selected.
No row family is silently removed.

Existing precision-1 output shape and nine member bytes are preserved, alongside
unchanged legacy builders. Added0.3 method branches explicitly bind table identity
and SHA256, producer, numerical-quality and formulation metadata. Physics-1 carries
`contract_evidence`; source-blocks-1 carries `source_block_recovery`;
physics-source-1 carries both. Foreign, missing and unknown method namespaces are
not downgraded. Schema oneOf branches pair every exact ID/hash/profile/namespace.

All three added methods carry `source_annotations` in original raw source order:

```json
{
  "source_row_index": 0,
  "source_result_id": "actual-id",
  "source_row": { "id": "actual-id", "kind": "actual-kind", "value": 1, "unit": "N", "entity_ref": "actual-entity" },
  "source_row_sha256": "checked-JCS-digest-of-the-entire-original-row"
}
```

The displayed shape is documentation, not a fabricated passing result fixture.
`source_row` is an exact deepcopy of the supplied actual raw row, retaining all
fields and absent/null distinctions, metadata component/frame/location/basis/sign,
source result references, value and unit. Rows are not reconstructed from
normalized packet rows or CSV. The existing checked JCS transport/hash profile
continues to define serialized numeric identity, including its established
negative-zero normalization; this tranche introduces no new numeric encoding.

Normalized `result_rows` retain their existing sorted order and all received IDs,
values and units. Annotations have a bijection to these rows by ID. Six support
force/moment components keep reaction family with force/moment dimensions from
the explicit table; magnitudes remain distinct rows. Ordinary and retained-source
stress extrema keep their original metadata basis and table signature. No stress
or maximum is recomputed from rounded projected actions.

All nine members remain `manifest.json`, CSV, result rows, unit-system disclosure,
unit witnesses, stable-ID map, loss report, validation report and diagnostics.
Evidence namespaces and annotations reside in transported package metadata within
manifest.json and are protected by the complete-package checksum. Existing member
checksums and manifest seed remain intact. Source row hashes bind the original
row bytes under checked JCS; the actual received source hash remains separate.

Source-bound build/validation uses the full actual raw validator and actual
analysis-record builder/bindings. Supplied source namespace values and annotation
rows must match the raw source exactly. Standalone transport inspection calls
method-specific transport-metadata validation (including receipt hashes/physical
case crosshashes as applicable) and validates retained-row hashes/projection
relationships. It never constructs a fake mechanics envelope from derivative
rows and never supplies an invocation context to mint numerical eligibility.
A coherently rehashed new packet can be internally consistent without matching a
supplied original source; an explicit negative control keeps that distinction.

The NUM source-block packet extension was inspected read-only as an API/delta
reference. It is not imported wholesale or asserted independently reviewed here.
The new exact-row annotation contract differs from its older flat annotation
shape; parent coordinates the matching TypeScript projection. There is no
silent compatibility fallback for that unregistered intermediate packet shape.

The closed ordinary physical shape in the stress-neutral schema is copied
unchanged from the existing published result schema's three Physics definitions;
a regression test checks exact equality. Source and composite receipt shapes
reference their owning standalone schemas. Parent-owned metadata validator APIs
and actual-source fixtures remain separate dependencies.
