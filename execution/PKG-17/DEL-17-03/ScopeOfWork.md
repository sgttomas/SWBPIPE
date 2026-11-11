---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-03
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017]
---

# Scope of Work — DEL-17-03 Native open JSON export package

## Purpose and Objective Traceability

This deliverable is SWBPIPE's own target-neutral export of a model: the model's canonical data as deterministic JSON, packaged with its manifest, stable ID map, loss report, validation report and diagnostics. The result it contributes is that a user can take a model out of SWBPIPE in a documented, vendor-free form in which identity, units, references, warnings, unresolved assumptions and losses are all visible, and that every other target adapter has one project-owned package to read instead of the application's internals.

It serves SOW-030 and SOW-074 under OBJ-009 and OBJ-017 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-03: the first-wave export target and the canonical adapter input), with `docs/PRD.md` §5.8, §11.7 (FR-HAND-001 and FR-HAND-002, both Must) and §16.1. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The native JSON export package: its schema `schemas/native_json_export.schema.json`, the builder and writer that produce a package from a model, and the invented public fixture.

## Deliverable Definition — Ontology

**Terms the package implements.**

- *Member.* One of six parts of a package:
  - `manifest`: the source model reference and its hash basis, the export profile, the member inventory with a hash for each member, diagnostics and boundary notes;
  - `model_payload`: the model's canonical data, with its units manifest;
  - `stable_id_map`: canonical SWBPIPE identifiers mapped to package-local records;
  - `loss_report`: what was exported, omitted, approximated, delegated or unsupported, and what is undecided;
  - `validation_report`: the package-shape and source-boundary checks made on this package, and nothing about the solver;
  - `diagnostics`: the warnings and blocking messages of export preparation.
- *Native profile.* The export profile of this target under the common contract. It declares the profile identifier and version, the source model identity and hash basis, the unit and coordinate policy, the identifier families included, the member path policy, the timestamp policy, the loss-category policy and the boundary notes.
- *Computed hash.* A hash this package computes over one of its own JSON members: SHA-256 over the member serialized with sorted keys, compact separators and ASCII escaping, labelled `deterministic_sorted_compact_json_payload_hash`.
- *Source checksum.* A checksum of the source model that the caller supplies with its own declared algorithm, canonicalization and digest. The package carries it unchanged beside its computed hashes.

**Boundary.** This deliverable owns the native package, its schema, builder, writer and fixture.

- The definitions of package, profile, stable ID map, manifest, loss report and loss category are DEL-17-02's.
- Every target-specific format is its own deliverable's: MBF (DEL-17-04), stress-neutral (DEL-17-06), PCF (DEL-17-07), review geometry (DEL-17-08), further targets (DEL-17-09). No member of the native package names a commercial target.
- The model document, its identifiers and its validator are DEL-02-01's. The project's portable package (`.opsproj`) is DEL-02-05's. The handoff package and the handoff export workflow are DEL-15-01's and DEL-15-03's.
- The governed export route and the screening of private and protected values are DEL-12-02's.
- No desktop, API, command-line or project-store entry to the export belongs to this deliverable. The panel that presents a native package in the desktop is DEL-07-12's.

**Not decided.**

- Which desktop, API or command-line entry exposes the native export, which deliverable owns that entry, and how a stored project is bound to it.
- The hash basis. REQ-006 gives this package a project-local computed hash that is not RFC 8785, while the common contract (DEL-17-02/REQ-007, DEL-17-02/AC-006) and decisions DEC-010 and DEC-028 require the project's canonical JSON basis for member hashes. Which governs for the native package is the owner's to settle.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — A native package has a manifest, a model payload, a stable ID map, a loss report, a validation report and diagnostics.
- **REQ-002** — The package preserves canonical SWBPIPE identity, directly in its JSON records or through its package-local ID map.
- **REQ-003** — The manifest identifies the source model reference, the export profile, the members, the hash basis and the boundary notes.
- **REQ-004** — The loss report uses the six categories of the common contract.
- **REQ-005** — The validation report holds package-shape and source-boundary evidence only and states neither solver validation nor professional acceptance.
- **REQ-006** — The package is deterministic, key-sorted, compact JSON. A hash it computes is a computed hash as defined above and carries that label; no runtime timestamp is hidden in a hash input; a source checksum keeps its declared canonicalization and digest or is refused as malformed; and the label asserts no RFC 8785 conformance.
- **REQ-007** — The public fixture is invented, and the package's shape, hashes, diagnostics, privacy and boundary flags and payload screening are each examined on it.
- **REQ-008** — Target-specific and adapter-specific behaviour stays outside the native package.

### Acceptance criteria

- **AC-001** — A package built from an invented model validates against the schema and has the six members. The manifest names the source model reference and its hash basis, the profile, the boundary notes and every member with its hash. A build that lacks a member, a stable ID map or a loss report returns a blocking diagnostic and no package is written.
- **AC-002** — For an invented model that has library references, rule-pack references, warnings and unresolved assumptions, the package carries the model hash, the units manifest, every entity identifier, each library and rule-pack reference by identity only, the warnings and the unresolved assumptions.
- **AC-003** — Every canonical identifier of the exported model appears in the model payload or in the stable ID map. The loss report is present when nothing was lost, uses only the six categories, and names every entity the package did not carry.
- **AC-004** — Two builds from the same inputs are byte-identical. Each computed hash equals SHA-256 over the member's serialization on the hash basis that governs (not decided, above: this package's sorted-key, compact, ASCII-escaped form, or the RFC 8785 form of DEL-17-02/AC-006), recomputed without product code, and carries that basis's label; changing any value of a member changes its hash; and no hash input contains a timestamp. A source checksum supplied with a different label comes out with its algorithm, canonicalization and digest unchanged, and a malformed one is refused.
- **AC-005** — The validation report lists only package-shape and source-boundary checks. A payload that carries a value classed private, or content suspected to be protected, is stopped before any file is written. No member states compatibility with a target, solver validation, code compliance or professional acceptance.
- **AC-006** — The fixture is invented and carries its provenance, and the schema admits no field whose meaning depends on a particular target.

## Production and Verification Method — Praxeology

**Method choices that define the package.**

- The package is the least target-specific export path: it says what the model holds and decides nothing for a target.
- An explicit omission is preferred to a default; a missing value is reported and never supplied.
- Serialization is deterministic by construction: sorted keys, compact separators, no runtime value in any hashed member.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the package built from an invented model, and the builder's response to a missing member, ID map and loss report): schema validity, the six members, the manifest's inventory against the members, and the blocking diagnostics.
- **VER-002** — Executable check (observes the package built from an invented model that has library references, rule-pack references, warnings and unresolved assumptions): its model hash, units manifest, entity identifiers, references, warnings and assumptions against the model's own.
- **VER-003** — Executable check (observes the identifiers of an invented model against the payload, the ID map and the loss report of its package).
- **VER-004** — Comparison with a named reference (a recomputation of SHA-256 over the member's serialization on the governing hash basis, written without product imports) and executable check (observes two builds, a mutated member, a timestamped input and a differently labelled source checksum).
- **VER-005** — Executable check (the project's protected-content scan, and a payload seeded with a private value and with a protected-content marker) and judgment by a reviewer who did not write the package: examines the validation report's checks and every member for the claims REQ-005 excludes.
- **VER-006** — Judgment by a reviewer who did not write the fixture: examines the fixture's provenance and the schema's fields for target-specific meaning.

## Governing Values and Decisions — Axiology

- **AX-001** — *Traceable before convenient.* Better is more of the model's content present with its identity, units and provenance, and fewer entities in the loss report. Coverage is never raised by writing a default for a value the model does not hold.
- **AX-002** — *Target-neutral.* Better is fewer fields whose meaning depends on a receiving tool. A target's need is met in that target's deliverable and never by adding its semantics here.
- **AX-003** — *Deterministic.* Better is fewer sources of byte variation between two exports of one model. Determinism is never obtained by hiding a runtime value inside a hashed member.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-001 REQ-003 | AC-001 | VER-001 | Owner ruling: the common export contract (DEL-17-02) and the decomposition row name the package's members and manifest; the schema is examined with a JSON Schema 2020-12 validator that imports no product code, and the seeded omissions state what must block |
| OUT-001 | SOW-074 OBJ-017 | REQ-001 REQ-002 REQ-003 | AC-002 | VER-002 | Owner ruling: `docs/PRD.md` FR-HAND-001 and FR-HAND-002; the comparison is against the invented model, which the exporter did not write |
| OUT-001 | SOW-074 OBJ-017 | REQ-002 REQ-004 | AC-003 | VER-003 | Owner ruling: the common contract's loss categories; the identifiers compared are the invented model's own |
| OUT-001 | SOW-074 OBJ-017 | REQ-006 | AC-004 | VER-004 | Outside source: SHA-256 over the governing serialization (sorted-key compact JSON that any JSON library reproduces, or RFC 8785), recomputed without product code; which basis governs awaits the owner's ruling; byte equality of two builds guards against change only |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-005 REQ-007 | AC-005 | VER-005 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md` and `docs/PRD.md` §21.2; the seeded values state what must be stopped |
| OUT-001 | SOW-030 OBJ-009 | REQ-007 REQ-008 | AC-006 | VER-006 | Owner ruling: the decomposition makes this package the target-neutral adapter input; the reviewer did not write the fixture |
