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
