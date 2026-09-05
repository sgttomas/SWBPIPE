# Library Import Provenance Checker

`provenance_checker.py` validates already-parsed material, section, and
component library payloads at the import boundary.

It checks:

- required provenance fields;
- redistribution and review disposition for public imports;
- private-only data handling;
- suspected protected-content quarantine;
- unit and value-level provenance metadata for imported numeric values.
- PKG-02-style diagnostic envelope projection for import findings.

It does not parse external file formats and does not make legal conclusions
about licenses or redistribution rights. Unresolved rights remain review
findings for the human project authority.

## Hanger library boundary (SCA-009 row 23)

`library_kind="hanger"` accepts the standalone `schemas/hanger.schema.yaml`
1.0.0 contract: `hanger_library` metadata and `hanger_records`. Identities
`library_id` and `hanger_id` are explicit nonblank strings; repeated record IDs
reject. Each record carries `name`, `hanger`, and all seven provenance fields.
Every quantity uses `{magnitude, unit, dimension, provenance}` with full
value-level provenance. The input is inspected without mutation or defaults.

Both Python and Rust interpret the same schema source using a bounded walker
for its authored keywords, failing closed on unsupported schema keywords.
This is not a general JSON Schema implementation. Standard Draft 2020-12
validation is also tested. Runtime additionally rejects nonfinite quantities,
duplicate IDs, and unacceptable dispositions. Missing optional engineering
values remain absent: a successful import does not establish solve readiness.

The explicit supported unit vocabulary matches `core/units`: force `N`, `lbf`;
length `m`, `mm`, `in`, `ft`; force-per-length `N/m`, `lbf/ft`, `lbf/in`.
Quantities must be positive; stiffness DOF is explicitly `UX`, `UY`, or `UZ`.
There is no inferred orientation, unit conversion, catalog sizing, ranking,
source scraping, vendor parser, or selectable bundled catalog. Fixtures are
invented non-engineering test data only. Unknown source/license is retained as
private review metadata, not legal acceptance. Every nested provenance object
is checked for quarantine and visibility, even in an otherwise malformed
wrapper. Item-level dispositions cannot conceal rejected/protected provenance.

Downstream manual selection must revalidate library + record identity and map
only explicit `magnitude` to product `value`, retaining the original units.
Persist canonical `HangerSelectionEvidence` JSON in operation `source_note` and
support `provenance`: `{kind:"user_imported_hanger_selection",
library_kind:"hanger",library_id,hanger_id,library_metadata,record_snapshot}`.
Both snapshots include all original metadata and per-value provenance. Library
edits/deletion must not alter already-selected support snapshots.

PKG16/PKG07 own the resolver, preview, persistence and atomic configuration
replacement. On existing supports the whole before/after projection must
visibly clear old top-level `stiffness` and `nonlinear`, replace the selected
hanger, and preserve identity/label/node. Do not carry absent old quantities
into the new selection or copy variable stiffness onto constant selection.
Require explicit user-confirmed translational restraints and disclose existing
consumer non-consumption conditions. The native adapter must map `hanger` to
`hanger_library`. These integration obligations are outside this import crate.
