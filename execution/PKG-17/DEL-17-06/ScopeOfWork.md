---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-06
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-046, SOW-074]
package_objective_refs: [OBJ-007, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-06 Stress-neutral CSV/JSON package

## Purpose and Objective Traceability

This deliverable is the stress-neutral package: a project-controlled, vendor-neutral package of a solved result in two synchronized forms, CSV for inspection in a spreadsheet and JSON for tooling. The result it contributes is that a reviewer or a downstream tool can read SWBPIPE's results row by row, each row with its identity, unit, dimension and source run, and can compare them over time or across target exporters, in a format that belongs to no vendor and that encodes no solver's hidden defaults and no code's acceptance criteria.

It serves SOW-046 and SOW-074 under OBJ-007, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-06: the result and export assurance target that follows the export package contracts and the run and result records), with `docs/PRD.md` §5.6, §5.7, §11.7 (FR-HAND-002) and §16.1. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it; CSV export of the desktop's results tables (exit criterion 5) is DEL-08-04's.

- **OUT-001** — The stress-neutral package: its schema `schemas/stress_neutral_export.v0.3.schema.json`, the builders that produce a package from a result and its run record, the reader and writer of its materialized members, the export a user reaches in the desktop, and the invented fixture.

## Deliverable Definition — Ontology

**Terms the package implements.**

- *Member.* One of nine files of a package: `manifest.json`, `stress_neutral_results.csv`, `result_rows.json`, `unit_system_disclosure.json`, `unit_preservation_witnesses.json`, `stable_id_map.json`, `loss_report.json`, `validation_report.json` and `diagnostics.json`.
- *Result row.* One solved value. Its columns, in both forms, are `result_id`, `canonical_ref`, `row_kind`, `result_family`, `load_case_ref`, `station_ref`, `component_ref`, `value`, `unit`, `dimension` and `correlation_status`.
- *Source.* What a package is built from: a result export envelope of the product (DEL-08-04) and the analysis-run record that binds it to its model state and settings (DEL-14-02). The package is a projection of its source and adds no result of its own.
- *Unit-preservation witness.* The record, per row, that the value and unit in the package are the source's value and unit. Where the source's meaning for a row is unknown, missing or contradictory, or the row is diagnostic work, the witness is withheld under a named code and the row keeps its raw value.
- *Received source checksum.* The checksum the source carried. The package preserves it unchanged, apart from the checksums it computes itself.
- **CLM-018** — *Hash labels.* A checksum of a JSON member, and the package checksum, is SHA-256 over the member's canonical form under the checked profile `openpipestress_jcs_ijson_v1`. A checksum of a CSV member is over its normalized text and is labelled separately: `normalized_ascii_lf_text`, or `utf8_csv_record_lf_v1` for a member that is not ASCII. The package checksum covers the whole package except its own carrier.
- **CLM-042** — *One current version.* Stress-neutral 0.3.0 is the only version. A 0.1 or 0.2 package is refused by name with the instruction to export again; there is no migration, and the bytes of an older package are never relabelled or verified under the current labels.

**Boundary.** This deliverable owns the stress-neutral schema, builders, reader, writer and fixture.

- The result export document, its envelope and the CSV export of the desktop's results tables are DEL-08-04's. The analysis-run record is DEL-14-02's. Comparison mapping, tolerance profiles and comparison export are DEL-14-05's.
- The canonical JSON form and its hashing are DEL-08-02's. The verdict on whether a result may be read at all, and at what standing, is the product result reader's; this package takes it from the run record (DEL-14-02) and forms none of its own.
- The package, profile, identity-map and loss-report definitions are DEL-17-02's. The admitted sources for target behaviour are DEL-17-01's. The governed export route and the screening of private and protected values are DEL-12-02's. Parsing an external prover's CSV is DEL-17-05's. The desktop panel that presents this export is DEL-07-12's; the builder and validator the panel calls are this deliverable's.
- The package is not a vendor format and not a solver input deck. A human acceptance of a result is recorded outside it; the package may carry a neutral reference to such a record and its hash basis, and nothing more.

**Not decided.**

- Whether the package carries model tables beside the result rows (node, element, component, restraint and support, equipment and nozzle, material, section, load and design case), and their fields. Material and section tables, if carried, are identity and provenance only.
- How the package expresses comparison tolerances and comparison outcomes. Until DEL-14-05's contracts are applied, a comparison made with this package is diagnostic.
- Whether a diagnostic-work row can be given a unit-preservation witness, or stays withheld for good. The case in hand is the sparse invented fixture, whose rows `result:nonlinear-support:free-dof-work-residual` and `result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual` have theirs withheld. Every raw row is retained either way.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The package supports review, debugging, community adapter development and comparison across target exporters, and does not present itself as a vendor format.
- **REQ-002** — The package provides CSV for inspection and JSON for structured exchange.
- **REQ-003** — The package preserves canonical SWBPIPE identity, in row and object fields or through its identity map.
- **REQ-004** — The package has a manifest that records the source model and run, the member inventory, the hashes, diagnostics and boundary notes.
- **REQ-005** — The package has a loss report even when the export succeeds, in the common contract's categories.
- **REQ-006** — Every result value carries its unit and dimension, or the export blocks with a diagnostic.
- **REQ-007** — The package carries the diagnostics, warnings, unresolved assumptions, provenance, reproducibility references and professional-boundary notice that its source holds.
- **REQ-008** — The package copies no protected standards text or table, proprietary formula, code-specific allowable, SIF or flexibility value, private rule-pack content, private project data or proprietary commercial example into a public artifact.
- **REQ-009** — The package states no code compliance, certification, sealing, professional approval, formal validation, engineering acceptance or release readiness.
- **REQ-010** — Target-specific or version-sensitive behaviour that no admitted source (DEL-17-01) settles is marked undecided and never presented as supported.
- **REQ-011** — The stress-neutral profile carries source-basis references to DEL-08-04, DEL-14-02, DEL-14-05 and DEL-17-02, and a package without them is not accepted.

### Acceptance criteria

- **AC-001** — A package built from the result export document of an invented solved result (DEL-08-04) and its run record validates against the 0.3.0 schema and has the nine members. For every row, the CSV and the JSON hold the same identifier, canonical reference, family, load case, value, unit and dimension, and the manifest's inventory, the identity map and the loss report describe the same rows in both forms.
- **AC-002** — Every row has a unit and a dimension. A source value without a unit, or with a unit of another dimension, blocks the export with a diagnostic naming the row. Each row's value and unit equal the source's, and its witness says so or is withheld under its named code; no value is converted, rounded or re-spelled on the way.
- **AC-003** — The manifest names the source model, the analysis run and the four source-basis references, and a package missing any of them is refused. Every row maps through the identity map to a canonical identifier or is listed as omitted; the loss report is present when nothing was lost; and the diagnostics, warnings, unresolved assumptions, provenance, reproducibility references and boundary notice of the source appear in the package unchanged.
- **AC-004** — Each JSON member's checksum and the package checksum equal SHA-256 over the canonical form as computed by an implementation that imports no product code. Each CSV member's checksum is over its labelled normalized text. The received source checksum is present, unchanged and distinct from the computed ones. Changing any value changes the checksums that cover it, and no current package carries the retired sorted-compact label.
- **AC-005** — Every reader of a stress-neutral package refuses a 0.1 or 0.2 package by name and reads nothing from it.
- **AC-006** — Writing the nine members of a package to a directory and reading them back gives an equal package that validates. A directory with a missing, added or altered member is refused.
- **AC-007** — A row whose meaning the source leaves unknown, missing or contradictory keeps its received value and unit, has its unit-preservation witness withheld under its named code, and has no family, unit or dimension inferred for it. A source the product reader refuses, or one without an admitted run record, yields a named refusal and no package. Every row the source holds is retained in the package.
- **AC-008** — For the same result and run record, the package the desktop saves and the package the headless builder gives have members equal by canonical value, and both validate against the schema.
- **AC-009** — The fixture and every example are invented and carry their provenance. A rule pack appears by identifier, version and checksum only. No member states a claim REQ-009 names or compatibility with a target tool, or presents the package as a vendor format or a solver input deck; every statement of target-specific or version-sensitive behaviour is marked undecided or cites the admitted source that settles it; and no public artifact holds content REQ-008 names.

## Production and Verification Method — Praxeology

**Method choices that define the package.**

- The package projects its source; it computes no result and decides no admission.
- The two forms are built from one set of rows, so that neither can drift from the other.
- Row order is not identity and a spreadsheet header is not a unit: every row carries its identifier, unit and dimension itself.
- A gap is stated, as a withheld witness, a loss entry or a diagnostic, and never filled by a guess.

**How each criterion is examined.**

- **VER-001** — Executable check (observes a package built from the result export document of an invented solved result, and that the builder's input is that document): schema validity, the nine members, and row-by-row equality of the CSV and JSON forms with the manifest, identity map and loss report.
- **VER-002** — Executable check (observes each row against its source value and unit, the witnesses and their withholding codes, and the builder's response to a value without a unit and to a unit of another dimension).
- **VER-003** — Executable check (observes the manifest's references, the identity map's coverage, the loss report, and the source's diagnostics and notices against the package's; and a package with each source-basis reference removed).
- **VER-004** — Comparison with a named reference (an RFC 8785 canonicalizer that imports no product code) and executable check (observes fixed byte and digest vectors, a mutated member, the CSV labels and the received source checksum).
- **VER-005** — Executable check (observes each reader given a 0.1 and a 0.2 package).
- **VER-006** — Executable check (observes a package written to a directory and read back, and directories with a missing, added and altered member).
- **VER-007** — Executable check (observes a source holding a row of unknown meaning, whose row, withheld witness and code are read against the source and whose source rows are counted against package rows; and a source the product's reader refuses and one without an admitted run record, each of which gives its named refusal and no package).
- **VER-008** — Executable check (observes the desktop's package and the headless builder's package for the same invented result and run record, member by member).
- **VER-009** — Executable check (the project's protected-content scan over the schema, fixtures and examples) and judgment by a reviewer who did not write them: examines provenance, rule-pack references and every member for the claims REQ-009 excludes, for a statement of target compatibility or of the package as a vendor format, and for target-specific behaviour neither marked undecided nor citing an admitted source.

## Governing Values and Decisions — Axiology

- **AX-001** — *Two forms, one content.* Better is more of the package's content checked equal between CSV and JSON. A convenience of one form, such as a rounded number or a friendlier header, never changes a value, a unit or an identifier.
- **AX-002** — *Explicit gaps.* Better is fewer withheld witnesses and fewer unknown families, reached by the source stating its meaning. A gap is never closed by guessing what a row means.
- **AX-003** — *Stress-neutral.* Better is fewer fields that depend on a receiving solver or a code. No target default and no acceptance criterion enters the vocabulary to make an adapter's work easier.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-046 SOW-074 OBJ-007 OBJ-017 | REQ-001 REQ-002 REQ-004 | AC-001 | VER-001 | The schema is examined with a JSON Schema 2020-12 validator, the members against the common contract (DEL-17-02), and the two forms are compared with each other and with the invented source |
| OUT-001 | SOW-046 OBJ-007 | REQ-006 | AC-002 | VER-002 | The source result's own values and units, which the package did not write; the seeded unit faults state what must block |
| OUT-001 | SOW-074 OBJ-017 | REQ-003 REQ-005 REQ-007 REQ-011 | AC-003 | VER-003 | The invented source envelope and run record, against which the manifest, identity map, loss report and carried notices are compared; `docs/PRD.md` §5.7 and FR-HAND-002 state what must be carried |
| OUT-001 | SOW-074 OBJ-017 | CLM-018 | AC-004 | VER-004 | Outside source: RFC 8785 and SHA-256, through a canonicalizer that imports no product code; the fixed vectors guard against change only |
| OUT-001 | SOW-046 OBJ-007 | CLM-042 | AC-005 | VER-005 | A 0.1 and a 0.2 package given to each reader; the expected result is the named refusal CLM-042 states, with nothing read |
| OUT-001 | SOW-046 SOW-074 OBJ-007 OBJ-017 | REQ-004 CLM-018 | AC-006 | VER-006 | The package before writing is the expected result; the seeded member faults state what must be refused |
| OUT-001 | SOW-046 OBJ-007 OBJ-018 | REQ-007 | AC-007 | VER-007 | The source's own rows and the product reader's verdict, neither written by this package |
| OUT-001 | SOW-046 OBJ-007 | REQ-002 REQ-004 | AC-008 | VER-008 | The product's own two builders compared with each other, which shows they agree and not that either is correct; correctness rests on the rows of AC-002 and AC-003 |
| OUT-001 | SOW-074 OBJ-018 | REQ-001 REQ-008 REQ-009 REQ-010 | AC-009 | VER-009 | The scan, whose markers are synthetic and so show only that it finds what it is seeded with, and the reading of a reviewer who did not write the members against `docs/IP_AND_DATA_BOUNDARY.md` and `docs/PRD.md` §21.2 |
