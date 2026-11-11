---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-15-04
package_id: PKG-15
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-075]
package_objective_refs: [OBJ-017, OBJ-018]
---

# Scope of Work — DEL-15-04 External prover boundary metadata

## Purpose and Objective Traceability

This deliverable is the record in which an engineer keeps the context of work done outside SWBPIPE: which external tool a model was taken to, what was sent, where the external files are, and what the engineer wants to remember about it. An engineer validates a design in a professional stress-analysis tool and accepts it on their own authority; SWBPIPE helps them keep track of that and never speaks for it. So the record is free-form where the engineer writes (names, tags, notes, references, attachments) and closed where the software could be mistaken for an authority: it has no status lifecycle, no acceptance field and no way for the software to say that an external check was done. The result it contributes is that the external-prover loop of PRD §16.2 can be followed across model states without the product acquiring an approval state.

It serves SOW-075 under OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-15, row DEL-15-04), and `docs/PRD.md` §4.2, §4.3, §16.2, §16.3, §21.2 and §21.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The external-prover metadata record: the schema `schemas/external_prover_metadata.schema.json` and the builder that forms a record and reports what is wrong with one.
- **OUT-002** — The authority-wording screen: the one check, shared by the handoff and export records that need it, that finds wording of approval, certification, sealing, authentication, compliance, external validation, acceptance or prover status in a value.

## Deliverable Definition — Ontology

**Terms the record defines.**

- *External-prover metadata record.* One record of external-workflow context, with its identity, the groups below, diagnostics, provenance and the professional-boundary block. Its contract status is fixed as workflow metadata without authority.
- *Name, tag, note.* The engineer's own labels and remarks. No vocabulary is enforced on them.
- *External reference.* A pointer to something outside the product: its kind, a location, an optional checksum, and provenance.
- *Attachment.* A reference to an artifact (a file location, a hash, a note). An attachment never contains the artifact.
- *Context link.* A reference by identity to a handoff package, a target mapping, an export record or a saved model state. A link carries no content of what it names.
- *Assumption, warning, unsupported-target flag.* Review evidence carried with the record, each with the objects it affects.
- *Proposed authority claim.* Something offered to the record as a status of approval, acceptance, certification, validation or lifecycle stage. The record keeps it only as a rejected claim, with a blocking finding.
- *Professional-boundary block.* Fixed statements: human review is required; the record is metadata only; no external tool was invoked and no commercial result was ingested; the software makes no compliance, certification, sealing, approval or authentication claim and creates no reliance record and no external-validation record.
- *Human acceptance reference.* A pointer to an acceptance made by a person outside the software. The record has none; REQ-007 governs one if it is added.

**Boundary.**

- A handoff package (DEL-15-01), a target mapping (DEL-15-02) and an export (DEL-15-03) are linked from this record and never copied into it.
- DEL-14-01 owns saved model states and DEL-02-01 the model document; the names, tags and notes of states and models are theirs. This record is the context of an external workflow, not a label on a state.
- DEL-01-04 owns the wording of the product's boundary statements. This deliverable owns the screen that finds prohibited wording in handoff and export records.
- DEL-07-12 owns the desktop panel that presents this metadata.
- DEL-17-05 owns the correlation of SWBPIPE results with an external tool's results, and what a prover-correlation comparison identifies (PRD §22.5) is DEL-17-05's record. This record ingests no result and compares nothing.
- No formal prover-status lifecycle, automatic professional acceptance record or commercial-tool result ingestion exists in the product (decomposition DEC-016).

**Not decided.**

- Where a record is kept and how an engineer enters one. Saved states and the model document already hold the engineer's names, tags, notes and external references, and no document or store is named as the home of this record.
- Whether the engineer's own free text is screened. The screen is applied to names, tags and notes, so a note in the engineer's words that an external check was completed is a blocking finding, while PRD §4.3 allows such a statement as a user-supplied external record and PRD §16.3 promises unforced names, tags and notes.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The data model supports external-prover workflow metadata (SOW-075).
- **REQ-002** — The record supports flexible names, tags, notes, external references and attachments (decomposition row DEL-15-04; PRD §16.3).
- **REQ-003** — The record forces no formal prover-status lifecycle (SOW-075; PRD §16.3; decomposition DEC-016).
- **REQ-004** — The record creates no automatic professional acceptance record (SOW-075; PRD §21.3).
- **REQ-005** — The record emits no approval, certification, code-compliance, sealing, authentication, endorsement or professional-reliance status, and no equivalent of one (PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-006** — External-prover metadata is diagnostic and handoff support. It is never evidence that an external verification was sufficient (decomposition DEC-015; PRD §4.3).
- **REQ-007** — A reference to a human acceptance, if the record ever carries one, is external, owned by a human actor and bound to the hashes of the payloads that were reviewed, and does not survive a change to them (PRD §21.3; `docs/CONTRACT.md` OPS-K-AUTH-2).
- **REQ-008** — Fixtures, examples and sample metadata copy no protected standards text or tables, proprietary formulas or values, private project data, private rule-pack or library content, real secrets or commercial software examples (PRD §20.1).
- **REQ-009** — A record that becomes a public data contribution carries source, provenance, licence and redistribution status, contributor certification and review disposition (PRD §20.2; `docs/CONTRACT.md` OPS-K-IP-2).
- **REQ-010** — The record is a schema-first contract in JSON Schema 2020-12, its diagnostics have the product's diagnostic form, and any hash it carries names the basis it was computed on (decomposition DEC-010).

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12. A record with invented names, tags, notes, external references, attachments and context links validates, and so does a record with any of those groups empty. Every document the product presents as external-prover metadata validates against the schema.
- **AC-002** — The schema has no field that takes a prover status, a lifecycle stage or an acceptance, and its records are closed: a record with such a field added is rejected.
- **AC-003** — A proposed authority claim appears in the record only as a rejected claim and produces a blocking finding, and no record the builder emits presents the claim as a status.
- **AC-004** — The professional-boundary block is fixed. A record in which any software-authority statement is true, or which says an external tool was invoked or a commercial result ingested, is rejected by the schema and is a blocking finding of the builder.
- **AC-005** — An attachment that embeds its payload is a blocking finding. A record contains references, hashes and notes only.
- **AC-006** — The authority-wording screen finds each prohibited term in a string, a list and a nested record, whatever its letter case, and finds none in a value without one. Applied to a proposed authority claim of this record, a hit is a blocking finding that names the field.
- **AC-007** — Every external reference, attachment, link, assumption, warning and flag carries provenance, and a flag without an affected object is a finding. Provenance names the source, its location, licence, contributor, contributor certification, redistribution status and review status (PRD §20.2); a field not yet known is stated as such and is never omitted.
- **AC-008** — The same inputs give an identical record.
- **AC-009** — The record ingests nothing: building one reads no external file, invokes no tool and parses no result.
- **AC-010** — The schema, its fixtures and its examples contain only invented data and name no commercial tool's result format.
- **AC-011** — If the schema gains a human acceptance reference, it requires a named human actor and the hashes of the reviewed payloads, the builder never fills it, and a record whose bound hashes no longer match is a finding.
- **AC-012** — The schema's diagnostic record requires code, class, severity, source, affected object, message, remediation and provenance, and every class it admits is one of the product's six warning classes or a local class whose mapping to one of them the schema states.

## Production and Verification Method — Praxeology

**Method choices that define the record.**

- Free where the engineer writes, closed where the software states. The descriptive groups take any content; the contract status and the boundary block are constants.
- A claim is rejected visibly. An offered status is kept as a rejected claim with its finding, so that the refusal can be seen, and is not silently dropped.
- References, never payloads. External artifacts are named by location and hash.
- One screen for authority wording, defined here and used by every handoff and export record that screens for it, so that the product has one list of prohibited terms.
- The builder reports and does not repair: it returns the record with its findings, sorted and deterministic.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the schema under a JSON Schema 2020-12 validator, the acceptance of invented records, and the rejection of each record with a status field added or a boundary statement changed, and the diagnostic record's required fields and classes): AC-001, AC-002, AC-004, AC-012.
- **VER-002** — Executable check (observes the builder's findings for invented inputs with each fault the criteria name, the rejected-claim record, and the identity of two records built from the same inputs): AC-003, AC-004, AC-005, AC-007, AC-008.
- **VER-003** — Executable check (observes the screen's result for each term of PRD §21.2's prohibited claims and of the prover-status vocabulary, in each value shape, and for clean values): AC-006.
- **VER-004** — Judgment by a reviewer who did not write the code (reads the schema, the builder and its messages against PRD §21.2 and `docs/claims_registry.md`, confirms that the builder reaches no file, tool or network, and runs the protected-content scan over the schema and fixtures): AC-002, AC-009, AC-010, AC-011.

## Governing Values and Decisions — Axiology

- **AX-001** — *The engineer's words, the engineer's authority.* Better is more of the external workflow's context that an engineer can record in their own terms. This never extends to a field, status or default by which the software itself says that something was approved, validated or accepted.
- **AX-002** — *No lifecycle by accretion.* A fixed vocabulary of stages, however convenient for sorting and filtering, is a prover-status lifecycle and is not added. Order among records comes from the engineer's names and tags and from the model states they link to.
- **AX-003** — *A link is not an ingestion.* Better is more precise reference to external artifacts, by location and hash. Reading, parsing or summarizing an external result is outside the record however much it would save the engineer.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-001 REQ-002 REQ-010 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that is not product code. The groups are an owner ruling: SOW-075 and PRD §16.3 |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-003 REQ-004 REQ-005 | AC-002 | VER-001 VER-004 | Owner ruling: PRD §16.3 and §21.2 and decomposition DEC-016 |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-004 REQ-005 REQ-006 | AC-003 | VER-002 | Owner ruling: PRD §4.3 and §21.2 and decomposition DEC-015 |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-005 REQ-006 | AC-004 | VER-001 VER-002 | Owner ruling: PRD §21.2 and decomposition DEC-016 |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-002 REQ-008 | AC-005 | VER-002 | Owner ruling: PRD §20.1 and §20.3 |
| OUT-002 | SOW-075 OBJ-017 OBJ-018 | REQ-005 | AC-006 | VER-003 | Owner ruling: PRD §21.2 lists the prohibited claims and PRD §16.3 excludes a prover status; the expected hits are those terms |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-002 REQ-009 | AC-007 | VER-002 | Owner ruling: PRD §20.2; the invented inputs carry the faults |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-010 | AC-008 | VER-002 | The product's own earlier output, which only guards against change |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-006 | AC-009 | VER-004 | Owner ruling: SOW-075 and decomposition DEC-016 exclude result ingestion; judged by a reviewer who did not write the code |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-008 | AC-010 | VER-004 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md`; protected-content scan and review |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-007 | AC-011 | VER-004 | Owner ruling: PRD §21.3 and `docs/CONTRACT.md` OPS-K-AUTH-2 |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-010 | AC-012 | VER-001 | Owner ruling: decomposition AB-00-06 states the fields and the six classes; DEL-00-06 holds the vocabulary |
