---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-15-01
package_id: PKG-15
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-074]
package_objective_refs: [OBJ-017]
---

# Scope of Work — DEL-15-01 Canonical handoff package schema and manifest

## Purpose and Objective Traceability

This deliverable is the contract for what SWBPIPE hands to a downstream modelling tool or to an engineer who will check the design in a professional stress-analysis tool. It says what a handoff package is, what its manifest must state about the model it carries, and what makes a package whole, so that the receiver can tell which model they hold, in which units, with which library and rule-pack references, which warnings and which assumptions still open, and can recompute the model hash and the export hash, and check every member against the inventory, without SWBPIPE. The result it contributes is that a handoff loses nothing the receiver needs and claims nothing the software cannot claim.

It serves SOW-074 under OBJ-017 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-15, row DEL-15-01), and `docs/PRD.md` §5.8, §16.1, FR-HAND-001 and FR-HAND-002. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The handoff package contract: the schema `schemas/handoff_package.schema.json`, the form of a package and its members, the rules that tie a manifest to the model it describes, and the rules by which a package is verified.

## Deliverable Definition — Ontology

**Terms the contract defines.**

- *Handoff package.* A multi-member package in the sense of decision DEC-028: the model member and the manifest member, and any further member a later contract adds, each listed in the member inventory. Evidence binds to the members and to the manifest's hashes, never to the bytes of a container.
- *Model member.* The canonical model document (DEL-02-01) as its RFC 8785 canonical text, with no trailing newline.
- *Manifest.* One instance of the schema, as RFC 8785 canonical text. It describes the model member and embeds none of it.
- *Model hash.* SHA-256 of the model member's bytes.
- *Export hash.* SHA-256 of the RFC 8785 form of the manifest object with its `checksums` member removed.
- *Member inventory.* The manifest's `checksums`: one checksum for each member of the package and no other. The model member's is the model hash and the manifest member's is the export hash.
- *Checksum record.* Algorithm, canonicalization label, the payload it covers by reference and by scope, the digest, and its provenance. The canonicalization label names the byte basis the digest was computed on: `JCS` is RFC 8785 text; `deterministic_sorted_compact_json_payload_hash` is sorted-key compact JSON and is not RFC 8785; `JCS_compatible_json_payload_hash` is accepted from producers that still write it and asserts nothing about RFC 8785.
- *Units manifest.* The unit system reference, the dimensional basis, the unit of every unit key of the model's project units, each a unit-catalogue identifier (DEL-02-02), the model's unit diagnostics, and provenance. Node coordinates are numbers in the unit of the length key.
- *Entity manifest.* The stable identifiers of every entity of the model, by family: nodes, components, elements (the model's pipe segments), supports, materials, sections, lines, constraints, equipment interfaces, and load cases with combinations, with the basis state.
- *Derivation from the model member.* The rule that makes a manifest true of its model member, so that the expected manifest can be worked out from the model member alone:
  - the package identity is a function of the model member's project identifier and the model hash, so the same model member always gives the same identity;
  - for each family of the entity manifest, the manifest holds the identifier of every record of that family in the model member and no identifier the model member lacks, each under its family;
  - the unresolved assumptions are one record for each assumption or open issue of the model member whose status is unresolved, with its identifier, statement and affected objects, and none for one that is resolved or rejected;
  - the library references are one for each distinct library, by kind and identifier, that a provenance record of the model member names, and the rule-pack references are one for each rule-pack reference of the model member, with its identity and version;
  - every checksum names, by reference, the subject it hashes, and every reference to the basis state names the same saved state.
- *Library reference and rule-pack reference.* Identity, version, a checksum over the reference metadata, source notice, and redistribution, review and privacy classifications. A reference never carries library or rule content.
- *Unresolved assumption, warning, diagnostic.* Explicit records, each with its affected objects and provenance.
- *Basis state.* The saved model state (DEL-14-01) whose model hash is the package's model hash.
- *Reserved target surfaces.* The manifest's target-mapping metadata and unsupported-behaviour flags. This contract fixes their presence; their statuses and their content are DEL-15-02's.
- *Provenance record.* Source name, source location, source licence, contributor, contributor attestation, redistribution status, review classification and privacy classification.

**Boundary.**

- DEL-15-03 produces packages: the desktop's saving of a model document and of a handoff package, and the generic export step. This deliverable states what a correct package is.
- DEL-15-02 owns how internal entities map to target fields and the taxonomy of unsupported and approximate behaviour.
- DEL-15-04 owns external-prover metadata. A handoff package declares no prover status.
- DEL-02-01 owns the model document's format, DEL-02-02 the unit catalogue, DEL-14-01 saved states, DEL-14-02 analysis-run records, and DEL-08-02 the canonical JSON form and the hash record.
- PKG-17 owns every target format and, through DEL-17-02, the export-package contract of target exports: profiles, stable ID maps and loss reports. This contract is target-neutral.
- No commercial-tool parser, result ingestion, external solver invocation or professional acceptance record is part of a handoff package.

**Not decided.**

- The physical form of a package: folder or archive, its extension and the layout and names of its members, and how its members relate to the member roles of the project package and of DEL-17-02's export packages (decomposition DEC-028, OI-015). The schema states the container as undecided.
- Whether a package carries the target modelling-engine input file and the comparison references that PRD §16.1 lists, as further members or as references.
- Whether a package refers to the analysis runs recorded against its basis state, and to their result exports. The manifest has both lists and PRD §16.1 asks for neither; whether the run reference fits the run records of DEL-14-02 has not been established.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The handoff package is defined schema-first: the schema is the contract, and no producer or reader defines a field of its own (SOW-074; `docs/TYPES.md` §3).
- **REQ-002** — The schema is JSON Schema 2020-12 in strict JSON with a stable `$id` (decomposition DEC-010).
- **REQ-003** — A package carries the model hash as a checksum record that includes its algorithm, digest, canonicalization and provenance, and names the payload it covers (SOW-074; PRD §16.1; decomposition AB-00-04).
- **REQ-004** — A package carries a units manifest with the unit system, the dimensional basis, an entry for each unit, unit diagnostics and provenance, such that no missing unit passes silently (SOW-074; PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-005** — A package preserves the stable identifiers of the model's entities as entity records, so that the entity, load-case, support and restraint, and material and component manifests of PRD §16.1 can be read from it and nothing is coupled by position (SOW-074; `docs/TYPES.md` §2).
- **REQ-006** — A package carries library and rule-pack references by identity, version, checksum and source notice, and copies no protected or private payload (SOW-074; PRD §16.1, §20.1, §20.3).
- **REQ-007** — Unresolved assumptions, warnings and diagnostics are explicit records in the package, never silent defaults (SOW-074; PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-008** — The manifest reserves the target-mapping metadata SOW-074 requires; what a mapping says is DEL-15-02's.
- **REQ-009** — The manifest reserves the unsupported-target flags SOW-074 requires, which carry PRD §16.1's known target limitations; their taxonomy is DEL-15-02's.
- **REQ-010** — The package and manifest preserve provenance for every record and reference that reliance could rest on (PRD §20.2; `docs/CONTRACT.md` OPS-K-DATA-3).
- **REQ-011** — Nothing in the contract declares certification, sealing, approval, authentication, code compliance or professional acceptance, and a package declares no prover status (PRD §21.2; OBJ-017).
- **REQ-012** — The package follows the container strategy of decision DEC-028 (member inventory, manifest, per-member hashes) and is target-neutral: it names no commercial target's fields, formats or versions.
- **REQ-013** — A checksum's canonicalization label names the basis its digest was computed on. The sorted-key compact JSON basis is labelled `deterministic_sorted_compact_json_payload_hash` and is never presented as RFC 8785; `JCS_compatible_json_payload_hash` stays a valid value while any producer writes it (decomposition DEC-074, evidence requirement E1).
- **REQ-014** — A package includes the canonical model file and binds it by hash, and its manifest carries an export hash and the member inventory, so that a change to any byte of any member is detectable from the package alone (PRD §16.1; decomposition DEC-028).
- **REQ-015** — A package names the saved model state it is based on (PRD §16.2).
- **REQ-016** — The schema's examples and fixtures hold only invented data (PRD §20.1; `docs/IP_AND_DATA_BOUNDARY.md`).
- **REQ-017** — A warning or diagnostic in a package has the product's diagnostic form: code, class, severity, source, affected object, message, remediation and provenance, with a class that is one of the product's warning classes or maps to one (decomposition AB-00-06).

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12 with its stable `$id`. A fully populated invented manifest validates, and a manifest that lacks the model hash, the units manifest, the entity manifest, the library or rule-pack references, the unresolved assumptions, the warnings, the diagnostics, either reserved target surface, the checksums, the basis state, the privacy context, the provenance or the professional-boundary block is rejected. Every document the product presents as a handoff manifest validates against the schema.
- **AC-002** — For a conforming package, every JSON member is its own RFC 8785 text; the model hash is SHA-256 of the model member's bytes; the checksums are exactly the member inventory; and each checksum states its algorithm, canonicalization, payload, scope and provenance. A change to any byte of any member, a member the inventory does not list, or a rewritten or relabelled checksum fails verification. This carries commitment DEL-15-01:1 for the checksum.
- **AC-003** — A manifest is true of its model member: the package identifiers, the entity manifest, the unresolved assumptions and the identities of the library and rule-pack references are exactly those the derivation from the model member gives, each entity record can be traced to the entity of the model it names, every checksum names the subject it hashes, and the basis state references agree. A manifest that describes another model, or whose subject is forged, fails verification. This carries commitment DEL-15-01:1 for entity identity.
- **AC-004** — The units manifest names the unit system and the dimensional basis, has an entry, as a catalogue identifier, for every unit key of the model's project units, and carries the model's unit diagnostics and its provenance. A manifest that omits any of these is rejected, and a model whose units cannot be stated has no conforming package. This carries commitment DEL-15-01:1 for the units manifest.
- **AC-005** — A library or rule-pack reference carries identity, version, checksum, source notice, classifications and provenance and no content; an instance that marks a private, protected or commercial payload as embedded is rejected. Every kind of library a model document can reference has its own kind value in the schema.
- **AC-006** — Both `deterministic_sorted_compact_json_payload_hash` and `JCS_compatible_json_payload_hash` validate; the schema states that the sorted-key compact basis is not RFC 8785; and in every committed example package each digest recomputes on the basis its label names.
- **AC-007** — Every manifest carries the reserved target-mapping metadata and the unsupported-behaviour flags. A flag's status is one of DEL-15-02's closed statuses, and every flag requires human review.
- **AC-008** — No field, enumeration or constant of the schema can state certification, sealing, approval, authentication, code compliance, professional acceptance or a prover status. The professional-boundary block is fixed, and an instance that changes any of its values is rejected.
- **AC-009** — The schema names no commercial target's fields, formats or versions.
- **AC-010** — The schema, its fixtures and its example packages contain only invented data, each with provenance.
- **AC-011** — The schema's diagnostic record requires code, class, severity, source, affected object, message, remediation and provenance, and every class it admits is one of the product's six warning classes or a local class whose mapping to one of them the schema states.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- The manifest refers and never embeds. The model travels as its own member; libraries and rule packs travel as references.
- The hash of a JSON member is taken over its RFC 8785 text, and the hash of a member that is not JSON over its bytes. The export hash excludes the `checksums` member so that the inventory can sit inside the manifest it protects.
- Verification needs nothing but the package. It proves the package is internally consistent; it does not prove that a saved state, library or rule pack named in the package still exists or holds what its digest says, which only the store that holds them can confirm.
- The verifier used to examine packages shares no code with any producer.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the schema under a JSON Schema 2020-12 validator, the invented manifest's acceptance, and the rejection of each manifest with a required part removed or a fixed value changed, and the diagnostic record's required fields and classes): AC-001, AC-004, AC-005, AC-007, AC-008, AC-011.
- **VER-002** — Comparison with a named reference (RFC 8785 and SHA-256, computed by `tests/verifiers/handoff_package.py` and its canonical-JSON sibling, which import no product code): AC-002, AC-003 and AC-006 on the committed example packages, with one byte changed in each member, a member the inventory does not list, a forged subject and a relabelled checksum each required to fail.
- **VER-003** — Executable check (observes that the product's verifier and the independent verifier return the same verdict on the same bytes for every case of `fixtures/handoff/verifier_parity_cases.json`): AC-002, AC-003.
- **VER-004** — Judgment by a reviewer who did not write the schema (reads the schema's names, enumerations and descriptions for target-specific terms and for any wording PRD §21.2 prohibits, with the protected-content scan run over the schema and fixtures): AC-008, AC-009, AC-010.

## Governing Values and Decisions — Axiology

- **AX-001** — *The receiver needs nothing but the package.* Better is more of what a downstream engineer must know about the model stated in the package and derivable from its model member, and fewer placeholders. This is never bought by embedding private, protected or commercial content, or by naming a commercial target.
- **AX-002** — *Every label is true.* Better is more of the package recomputable by someone who has only the package and the public standards. A digest is never relabelled, a verification rule is never relaxed to admit a package, and a placeholder is never filled to look complete.
- **AX-003** — *The software claims a handoff, not a result.* A field that would let a package read as validated, approved or compliant is a defect, however useful to the receiver.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-074 OBJ-017 | REQ-001 REQ-002 REQ-007 REQ-010 REQ-015 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that is not product code. The required parts are an owner ruling: PRD §16.1 and FR-HAND-002 |
| OUT-001 | SOW-074 OBJ-017 | REQ-003 REQ-014 | AC-002 | VER-002 VER-003 | Outside source: RFC 8785 and SHA-256, recomputed by a verifier that imports no product code |
| OUT-001 | SOW-074 OBJ-017 | REQ-005 REQ-006 REQ-007 REQ-015 | AC-003 | VER-002 VER-003 | The derivation rule stated in the Ontology gives the expected manifest; a verifier that imports no product code applies it to the model member |
| OUT-001 | SOW-074 OBJ-017 | REQ-004 | AC-004 | VER-001 | Owner ruling: PRD §5.6 and `docs/CONTRACT.md` OPS-K-UNIT-1 state that a unit is never assumed; the unit identifiers are those of DEL-02-02's catalogue, which this contract does not write |
| OUT-001 | SOW-074 OBJ-017 | REQ-006 | AC-005 | VER-001 | Owner ruling: PRD §20.1 and §20.3 and `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-001 | SOW-074 OBJ-017 | REQ-013 | AC-006 | VER-002 | Outside source: RFC 8785 decides which digests may be labelled `JCS`; the label vocabulary is an owner ruling (decomposition DEC-074) |
| OUT-001 | SOW-074 OBJ-017 | REQ-008 REQ-009 | AC-007 | VER-001 | Owner ruling: SOW-074 requires the two surfaces and the decomposition gives their content to DEL-15-02, whose closed statuses a flag takes |
| OUT-001 | SOW-074 OBJ-017 | REQ-011 | AC-008 | VER-001 VER-004 | Owner ruling: PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-074 OBJ-017 | REQ-012 | AC-009 | VER-004 | Owner ruling: decomposition DEC-028 and OI-015; target-specific content waits on DEL-17-01 and DEL-17-02 |
| OUT-001 | SOW-074 OBJ-017 | REQ-016 | AC-010 | VER-004 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md`; protected-content scan and review |
| OUT-001 | SOW-074 OBJ-017 | REQ-017 | AC-011 | VER-001 | Owner ruling: decomposition AB-00-06 states the fields and the six classes; DEL-00-06 holds the vocabulary |
