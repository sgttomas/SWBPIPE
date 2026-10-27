# Library Import Provenance Checker

`provenance_checker.py` validates already-parsed material, section, and
component library payloads at the import boundary.

It checks:

- required provenance fields;
- redistribution and review disposition for public imports;
- private-only data handling;
- suspected protected-content quarantine;
- disposition members no verdict can read: a `redistribution_status` or
  `review_status` holding an array or an object, or a `provenance` written as
  an array, anywhere in the library metadata or a record, refused by name as
  `MODEL-DISPOSITION-RECORD-MALFORMED` (the desktop export screen's code);
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
Since W-17 (`00af9112c0`, `apps/desktop/src/features/hanger-selection/hangerSelection.ts`)
the selection writes only the short library reference into the model: the
support's `provenance_record` carries a `library_ref`, and the support's
`provenance` and the operation's `source_note` carry the library provenance
string `library:hanger:<library id>#<record id>`. Library metadata and the
record snapshot never enter the model, so library edits or deletion cannot
alter a selected support.

A model saved before `00af9112c0` may still carry the legacy form: the support's
`provenance` holds the canonical JSON text of
`{kind:"user_imported_hanger_selection",library_kind:"hanger",library_id,hanger_id,library_metadata,record_snapshot}`,
with no `library_ref` and no `library:` string. Nothing writes it now. The
library rule of route `DREP-PACKAGE-USER-RELEASE-010`, the shareable report
package (`apps/desktop/src-tauri/src/user_release_admission.rs`), refuses a
model carrying it as it refuses the W-17 marks (`REPORT-USER-RELEASE-LIBRARY-REFERENCED`,
rule `legacy_hanger_selection`; DEL-08-01 Design `report-user-release.md` §2.7).

PKG16/PKG07 own the resolver, preview, persistence and atomic configuration
replacement. On existing supports the whole before/after projection must
visibly clear old top-level `stiffness` and `nonlinear`, replace the selected
hanger, and preserve identity/label/node. Do not carry absent old quantities
into the new selection or copy variable stiffness onto constant selection.
Require explicit user-confirmed translational restraints and disclose existing
consumer non-consumption conditions. The native adapter must map `hanger` to
`hanger_library`. These integration obligations are outside this import crate.
