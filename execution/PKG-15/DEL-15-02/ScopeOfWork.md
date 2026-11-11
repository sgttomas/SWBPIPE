---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-15-02
package_id: PKG-15
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-074]
package_objective_refs: [OBJ-017]
---

# Scope of Work — DEL-15-02 Target mapping and unsupported-behavior contract

## Purpose and Objective Traceability

This deliverable is the contract by which an export says how SWBPIPE's entities and fields map onto a target, and where the target cannot carry them exactly. A handoff is only as good as what it admits to losing: a support the target has no element for, a load the target can only approximate, a unit it stores differently. The contract gives each such fact one record, tied to the model it was exported from, so that the engineer who receives the export reads the loss and no assumption disappears in transit. The result it contributes is that no export approximates in silence.

It serves SOW-074 under OBJ-017 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-15, row DEL-15-02), and `docs/PRD.md` §5.8, §16.1 (known target limitations), FR-HAND-003 and FR-HAND-004. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The target-mapping contract: the schema `schemas/target_mapping.schema.json`, the mapping record, the behaviour flag with its closed statuses, the source context that binds a mapping to a handoff package, and the builder that forms a contract and reports what is wrong with one.

## Deliverable Definition — Ontology

**Terms the contract defines.**

- *Target-mapping contract.* One record set for one export to one target: its identity, the kind of target and a reference to it, the source context, the mapping records, the unsupported and approximate behaviour flags, the diagnostics, provenance and the professional-boundary block.
- *Target kind.* One of: local FEA, external prover, commercial tool by reference, generic downstream modelling, or undecided. A kind is a category. The contract names no product.
- *Mapping record.* One internal entity, field, unit, result, load case, boundary condition or metadata item and where it goes in the target: a stable mapping identifier, the kind, the source reference, the target reference, the kind of value, its unit metadata, a mapping status, the assumptions and warnings it rests on, and provenance.
- *Mapping status.* Mapped, unsupported, approximate, requires human review, or undecided.
- *Behaviour flag.* One behaviour the target does not carry exactly: a label, a status (unsupported, approximate, requires human review, not implemented, or undecided), the target reference, the affected objects, the assumptions and warnings it rests on, and provenance. Every flag requires human review.
- *Source context.* What ties the mapping to a handoff package (DEL-15-01): the package's model hash, a reference to its units manifest, its entity, library, rule-pack, unresolved-assumption and warning references, and its privacy context.
- *Disclosure.* A mapping that is not exact says why: an unsupported or approximate mapping names the assumptions or warnings it rests on.

**Boundary.**

- This contract is target-neutral. Concrete mappings, field coverage and taxonomy extensions of a named target wait on DEL-17-01's account of the target and DEL-17-02's export contracts (decomposition OI-015).
- DEL-15-01 owns the handoff package and reserves the manifest's target surfaces; this deliverable owns what a mapping and a flag say.
- DEL-15-03 assembles a package and a mapping into an export and checks that they agree.
- DEL-15-04 owns the screen for wording that claims authority, which this contract applies to behaviour labels.
- DEL-12-02 owns redaction and the export contexts. This contract carries a privacy context and refuses an unsafe one; it redacts nothing.
- A mapping is disclosure. It does not show that a target reproduced the model, and it resolves no gap.

**Not decided.**

- How this contract relates to DEL-17-02's stable ID map and loss report. Target exporters record identity and loss in those; nothing yet states whether a named target's concrete mappings are recorded in this contract or only in the target exporter's ID map and loss report, or whether one is derived from the other.
- The taxonomy and field coverage of each named target (decomposition OI-015).

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The contract carries target-mapping metadata for handoff exports: how internal entities map to target fields (SOW-074; PRD FR-HAND-003).
- **REQ-002** — The contract carries unsupported-target flags (SOW-074; PRD FR-HAND-004).
- **REQ-003** — The contract discloses approximate behaviour where the target cannot represent the source exactly (decomposition row DEL-15-02).
- **REQ-004** — The contract keeps the handoff's unresolved assumptions and warnings attached to the mappings and flags they bear on (SOW-074).
- **REQ-005** — A unit-bearing mapped value keeps its unit and dimension metadata (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-006** — Records that reliance could rest on carry source, provenance, redistribution and privacy status and review status (PRD §20.2; `docs/CONTRACT.md` OPS-K-IP-2, OPS-K-DATA-3).
- **REQ-007** — A missing or unsupported value is never replaced by a default (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-008** — The contract, its fixtures and its examples embed no protected standards text or tables, proprietary values, private formulas or private rule-pack payloads (PRD §20.1).
- **REQ-009** — The contract creates no status of professional approval, certification, sealing, authentication or code compliance, and a mapping never states that a target is equivalent or validated (PRD §21.2).
- **REQ-010** — The contract fits the handoff package: its source context is expressed in the package's terms and names the package's model hash and units manifest.
- **REQ-011** — The contract keeps the redaction and export-control boundary: it carries the privacy context of its source and exposes nothing that context withholds (PRD §20.3).
- **REQ-012** — The schema and the builder are target-neutral: they name no commercial tool, format or field. Concrete mappings, field coverage, taxonomy extensions and package realization of a named target wait on DEL-17-01 and DEL-17-02 (decomposition OI-015).
- **REQ-013** — The builder emits a blocking privacy diagnostic when redaction is not affirmed or an embedded private, protected or commercial payload is reported. Carrying privacy metadata is not enforcement.
- **REQ-014** — A diagnostic of the contract has the product's diagnostic form: code, class, severity, source, affected object, message, remediation and provenance (decomposition AB-00-06).

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12. A contract built for an invented target validates, and a contract that lacks its identity, target kind, source context, mapping records, either flag list, provenance or the professional-boundary block is rejected. Every document the product presents as a target-mapping contract validates against the schema.
- **AC-002** — A mapping record supplied without its identifier, kind, source reference, target reference, status or provenance, and a flag supplied without its label, status, target reference or provenance, are each a finding that names the record and the missing part, and the builder supplies none of them. A mapping whose kind is outside the closed set is a finding that names it. A mapping that is unsupported or approximate and names no assumption or warning is a finding that names it.
- **AC-003** — A unit-bearing mapping without unit metadata is a blocking finding that names the mapping, and no unit is supplied.
- **AC-004** — Every flag has a status from the closed set, at least one affected object and human review required. A flag with another status, with no affected object, or with human review switched off is a finding.
- **AC-005** — A contract without its source context, or whose source context lacks the model hash, the units manifest reference or any other required reference, is a blocking finding that names what is missing. Nothing is filled in.
- **AC-006** — A source whose privacy context does not affirm redaction, or reports an embedded private, protected or commercial payload, yields a blocking privacy finding. The builder never turns an unsafe context into a safe one.
- **AC-007** — A behaviour label that uses wording of approval, certification, validation or compliance is a blocking finding. The professional-boundary block is fixed, and no contract the builder emits contains such a claim.
- **AC-008** — Building twice from the same inputs gives an identical contract. Mapping records and flags are identified by their stable identifiers, not by position (`docs/TYPES.md` §8, `Reference`), so the same records and flags supplied in another order give the same contract.
- **AC-009** — For an export to a target, the contract's mapping records cover every entity and field the export wrote, its flags are exactly the behaviours the target does not carry exactly, and its source context names the model hash and units manifest of the package that was exported. A contract that validates but was not checked against an actual export establishes nothing about support for that target. This carries commitment DEL-15-02:1.
- **AC-010** — The schema and the builder name no commercial tool, format or field.
- **AC-011** — The schema, its fixtures and its examples contain only invented data.
- **AC-012** — The schema's diagnostic record requires code, class, severity, source, affected object, message, remediation and provenance.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- Loss is a record, not an absence. Something the target cannot carry appears as a mapping with a status or as a flag; a missing mapping is itself a finding.
- A mapping belongs to one exported model. It carries that package's model hash and units reference, so that a mapping cannot be attached to another model unnoticed.
- The builder reports and does not repair. It returns the contract with its findings, sorted and deterministic, and changes no input to make it pass.
- Statuses are closed vocabularies. A target-specific behaviour is expressed through a label and the closed status, never through a new status.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the schema under a JSON Schema 2020-12 validator, the acceptance of a contract built for an invented target, the rejection of each contract with a required part removed, and the diagnostic record's required fields): AC-001, AC-012.
- **VER-002** — Executable check (observes the builder's findings for invented inputs with each fault the criteria name, and the identity of two contracts built from the same inputs in different orders): AC-002 to AC-008.
- **VER-003** — Comparison with a named reference (the target's own description of what it can carry: for the invented target its declared capabilities, for a named target the account DEL-17-01 holds of it; and the package that was exported): AC-009.
- **VER-004** — Judgment by a reviewer who did not write the code (reads the schema, the builder and its messages for named commercial tools and for wording PRD §21.2 prohibits, with the protected-content scan run over them and the fixtures): AC-007, AC-010, AC-011.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing is lost quietly.* Better is a larger share of what an export does not carry exactly appearing as a status or a flag the engineer reads; the target is all of it. A cleaner-looking mapping never justifies leaving out an unsupported item, and a flag never drops the assumptions it rests on.
- **AX-002** — *Disclosure, not assurance.* Better is fewer readings of a mapping as proof that the target matches the model. No status, label or message says equivalent, validated or approved, however complete the mapping is.
- **AX-003** — *Neutral until a target is known.* The contract gains no field, status or label for one tool's convenience. What a named target needs is expressed within the closed vocabularies.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-074 OBJ-017 | REQ-001 REQ-002 REQ-003 REQ-006 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that is not product code. The required parts are those of SOW-074 and PRD FR-HAND-003 and FR-HAND-004 |
| OUT-001 | SOW-074 OBJ-017 | REQ-001 REQ-002 REQ-003 REQ-004 REQ-006 REQ-007 | AC-002 | VER-002 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state that nothing unsupported passes silently; the invented inputs carry the faults |
| OUT-001 | SOW-074 OBJ-017 | REQ-005 | AC-003 | VER-002 | The builder's findings on an invented unit-bearing mapping without unit metadata; the criterion states the expected blocking finding (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1) |
| OUT-001 | SOW-074 OBJ-017 | REQ-002 REQ-003 REQ-004 | AC-004 | VER-002 | PRD FR-HAND-004; the closed statuses are the criterion's own statement |
| OUT-001 | SOW-074 OBJ-017 | REQ-007 REQ-010 | AC-005 | VER-002 | PRD §5.5; the source context is DEL-15-01's package, which this contract does not write |
| OUT-001 | SOW-074 OBJ-017 | REQ-011 REQ-013 | AC-006 | VER-002 | The builder's findings on invented sources whose privacy context does not affirm redaction or reports an embedded payload; the criterion states the expected blocking finding (PRD §20.3) |
| OUT-001 | SOW-074 OBJ-017 | REQ-009 | AC-007 | VER-002 VER-004 | The builder's findings on invented labels carrying each prohibited term, and the reviewer's reading of the builder's output and messages against PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-074 OBJ-017 | REQ-001 | AC-008 | VER-002 | Two builds from the same inputs, and from the same records and flags in another order, compared with each other. This shows that the builder is repeatable and that supplied order changes nothing (PRD §3 asks for reproducibility); it does not show that the contract is correct |
| OUT-001 | SOW-074 OBJ-017 | REQ-001 REQ-002 REQ-003 REQ-010 | AC-009 | VER-003 | Outside source: the target's own description of what it carries, and the exported package. Neither is written by this contract's builder |
| OUT-001 | SOW-074 OBJ-017 | REQ-012 | AC-010 | VER-004 | The reviewer's reading of the schema and the builder for any commercial tool, format or field; the contract stays target-neutral under decomposition OI-015 until DEL-17-01 and DEL-17-02 describe a named target |
| OUT-001 | SOW-074 OBJ-017 | REQ-008 | AC-011 | VER-004 | `docs/IP_AND_DATA_BOUNDARY.md`; protected-content scan and review |
| OUT-001 | SOW-074 OBJ-017 | REQ-014 | AC-012 | VER-001 | Validator results on a diagnostic with each field removed; the fields are those decomposition AB-00-06 lists and DEL-00-06 defines |
