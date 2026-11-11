---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-01
package_id: PKG-16
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-069]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-01 Structured model operation schema

## Purpose and Objective Traceability

This deliverable is the data contract for a change to the model. A change a person makes in the desktop, a change the embedded agent proposes and a change read from a proposal file are the same kind of record: a structured model operation, with one published shape and a closed taxonomy. The result it contributes is that validation, preview, acceptance, application and the operation history all work on that one record, so a change can be checked, shown, decided, applied and reviewed later without any of them guessing what it means, and nothing reaches the model that is not such a record.

It serves SOW-069 under OBJ-015 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-16, row DEL-16-01), with `docs/PRD.md` §5.3, §10.3 step 3 and FR-AGENT-001. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 17, under which every change goes through the one operation route, through DEL-16-06.

- **OUT-001** — The operation contract: the published schema of an operation and of an operation batch, the taxonomy of operation kinds, change kinds and target object types, and invented example records.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Operation.* One proposed change to one target. It carries an identifier, an operation kind, a target, a change, an author type, a source, a rationale and a status.
- *Target.* The model record the operation acts on, named by object type and reference. A target is a record of the model document or a record the operation creates.
- *Change.* What the operation does to its target: a change kind, the part of the target it changes, the new value, and the unit and dimension of a unit-bearing value.
- *Before value.* The value the author saw when writing the operation, where a record states one. It is one way for validation to tell that the model has moved since; the model basis is the required way.
- *Batch.* An ordered, non-empty list of operations under one identifier, written against one model state. A later operation may name a record an earlier one in the same batch creates.
- *Model basis.* The model state a batch was written against, named by reference and by hash. The hash is SHA-256 over the RFC 8785 canonical form of the model document.
- *Taxonomy.* The closed sets of operation kinds, change kinds and target object types, and the pairings among them that are admitted. Together they add, move, modify, delete and reconnect model entities, and act on constraints, loads, supports and design knowledge.
- *Author type.* Whether a person or something else wrote the operation. It is asserted metadata; it is not an identity.
- *Source.* Where the operation came from.
- *Assumption record.* An unresolved assumption attached to an operation or batch: an identifier, a statement and the records it affects.
- *Status.* The state of a proposal: proposed, schema-validated, blocked by diagnostics, ready for the person's review, or rejected. No status says a change was accepted as engineering work.
- *Boundary records.* Two fixed records every operation carries. The mutation-route record says that structured operations are the only route and that direct mutation is not allowed. The professional-boundary record says that human review is required and that the software makes no compliance, certification, sealing, approval or authentication claim.

**Boundary.** This deliverable defines the record. What is done with a record belongs elsewhere:

- validating an operation, previewing it and naming its refusals: DEL-16-02;
- the person's decision on it and the operation history: DEL-16-03;
- what an agent's rationale must hold and must never claim: DEL-16-04;
- applying it, and the embedded agent's capability catalog, which admits a subset of this taxonomy: DEL-16-06;
- the model records an operation names and the form of each record it creates: DEL-02-01;
- the unit catalog and the dimension vocabulary: DEL-02-02;
- the diagnostic record: DEL-00-06; the canonical form and the model hash: DEL-08-02;
- the commands a person sees and how the palette organises them: DEL-07-09. That vocabulary binds to the implemented operation taxonomy, not to the published schema's operation-kind list;
- stamping the author of an imported proposal: DEL-16-03. Where the intake panel sits in the desktop: DEL-07-09.

What a valid value means for a change kind is decided by the deliverable that owns the records it changes. This deliverable states the kind, its target types and the form of its value.

**Not decided.**

- Whether the published schema is the form the operation route checks, or a proposal and interchange schema beside a separately defined engine record form, is not decided.
- Whether an operation may bind to a hash of part of the model in place of the whole document. Until that is decided the basis is the hash of the whole model document.
- The granularity of a change: how a change names the member of its target it sets, and which pairings of operation kind, change kind and target type the taxonomy admits. Until that is decided a change names its kind, its target type and the form of its value, and the taxonomy's families stay owed.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — A change by a person in the desktop, a change proposed by an agent and a change read from a proposal file are represented as structured operations before they can alter the model. The contract admits no record that means "set the model directly" (SOW-069; PRD §5.3).
- **REQ-002** — The contract defines operations that add, move, modify, delete and reconnect model entities, and operations on constraints, loads, supports and design knowledge (decomposition row DEL-16-01). Each change kind states its target types and the form of its value. Design knowledge itself is deferred to v0.2; its operations are defined here and are applied when it arrives.
- **REQ-003** — A record carries what validation, preview, acceptance, application and history each need: identity, kind, target, the change with its new value, the model basis, author type and source, rationale, assumptions and status (SOW-069; SOW-070). The target and the kind are enough for the preview to show the value before and after (PRD §10.3).
- **REQ-004** — No status, field or enumerated value of the contract can state acceptance of engineering work, certification, sealing, authentication, approval or code compliance (PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-005** — The schema and its examples hold invented values only: no protected standards text or tables, no proprietary catalogue values, no private project data and no value offered as an engineering default (PRD §5.4, §5.5; `docs/IP_AND_DATA_BOUNDARY.md`).
- **REQ-006** — The schema is JSON Schema 2020-12, the dialect of the project's public schemas.
- **REQ-007** — A record names model objects by reference and defines no second shape for them. The content of a record an operation creates is carried as a value in the model document's own form (PRD §5.2).
- **REQ-008** — The contract supplies no default. A unit-bearing value carries its unit and dimension, and a missing one is a finding and never an assumed unit (PRD §5.5, §5.6; `docs/CONTRACT.md` OPS-K-DATA-2 and OPS-K-DATA-3).
- **REQ-009** — The status vocabulary describes a proposal and nothing later: it has no value for applied, approved or accepted work.
- **REQ-010** — A batch is bound to the model state it was written against by reference and by hash.
- **REQ-011** — The published schema, the interactive operation vocabulary that DEL-07-09 holds and the operations the operation route applies are reconciled: each kind in one is matched in the others or is a stated difference. Every operation the route applies keeps the unit, model-basis, hash and diagnostic guarantees this contract requires. The schema's operation-kind list is not taken as the complete taxonomy, and no member enters the published schema only because the engine's record carries it (SOW-069).
- **REQ-012** — A proposal written outside the application enters only as a batch in this contract's form (FR-AGENT-001). Offline proposal intake stays in the product.

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12 in strict JSON. Every object it defines is closed, so an undeclared member fails validation, and it declares no default value.
- **AC-002** — Each kind of the published schema, each normative item of the interactive operation vocabulary and each kind of operation the operation route applies is matched to the others or named as a difference with its reason. None is left unaccounted for, and the schema's operation-kind list is not used as the complete list. On every operation the route applies, whether it came from a desktop editor, the embedded agent or offline intake, a unit-bearing value carries its unit and dimension, the batch is bound to its model basis by hash, and a refusal is a named diagnostic.
- **AC-003** — For each of add, move, modify, delete and reconnect, and for each of constraints, loads, supports and design knowledge, the taxonomy has at least one kind and an invented example of it validates. Each change kind states its target types and the form of its value. A record whose operation kind, change kind or target type is outside the taxonomy fails validation.
- **AC-004** — A record that lacks its identifier, kind, target, change, author type or status fails validation, as does a change that sets a value and lacks the new value, and a record by an author other than a person that lacks a rationale or a source. Every record names its target and kind so that the preview can show the value before and after. A record can carry assumption records, each with an identifier, a statement and the records it affects, and an assumption record missing any of these fails validation. A unit-bearing value without a unit or a dimension fails validation or yields a missing-unit diagnostic, and is never read as dimensionless.
- **AC-005** — The status, kind and boundary vocabularies contain no value that states acceptance of engineering work, approval, certification, sealing, authentication or code compliance. A record carrying such a status fails validation. The professional-boundary record admits only "human review required" as true and the five claims as false, and the mutation-route record admits only structured operations with direct mutation not allowed.
- **AC-006** — The schema text and every example hold invented values, each example carries a provenance record that marks it invented, and none holds protected standards content, a proprietary catalogue value, private project data or a value presented as an engineering default.
- **AC-007** — A target is named by object type and reference, and a batch by its model basis. No definition in the schema restates a record that the model document's schema defines.
- **AC-008** — A batch names its model basis by reference and by a hash record that states the algorithm, the canonical form and the payload it covers. A batch without a basis fails validation.
- **AC-009** — A proposal file that is not a batch in this contract's form is refused at intake by name, and nothing from it is queued.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- Schema first. The published schema defines the contract's record. It is not derived from the code that reads it.
- Closed and without defaults. An object admits only its declared members, and nothing is filled in for the author.
- Examples are public and invented. They show each kind and are not engineering data.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-005, AC-007 and AC-008, executable check: observes the schema document. It validates the schema against the JSON Schema 2020-12 metaschema, walks every object definition for closure and for the absence of defaults, reads the status, kind and boundary vocabularies against the prohibited claims of PRD §21.2, compares definition names with the model document's schema, and validates examples altered to carry an authority status, a changed boundary value or no basis, expecting each to fail.
- **VER-002** — AC-002, comparison between three separately written lists (the published schema's kinds, the vocabulary's normative items and the kinds the operation engine admits), with an executable check: the comparison reports every item with no match and no stated difference, and the check observes, on the operations each desktop editor, the embedded agent's host and offline intake emit, the unit and dimension of each unit-bearing value, the basis hash of the batch and the named diagnostic of a refused one.
- **VER-003** — AC-003 and AC-004, executable check: observes the examples and their alterations. It validates one example of each family, then the same examples with each required member removed, with an assumption record missing each of its members, with a kind or target type outside the taxonomy and with a unit or dimension removed.
- **VER-004** — AC-006, judgment by a named role (a reviewer who did not write the examples): reads the schema text and every example for protected, proprietary or private content and for values that read as defaults, with an executable check that each example's provenance record marks it invented.
- **VER-005** — AC-009, executable check: observes the review queue after offline intake is given a well-formed batch and files that are not one.

## Governing Values and Decisions — Axiology

- **AX-001** — *One shape.* Better is fewer places where the form of an operation is defined and fewer translations between an author and the engine; the target is one definition and no translation. That is never reached by loosening the schema to admit undeclared members.
- **AX-002** — *Refuse, do not guess.* Better is a larger share of malformed or incomplete records turned away by the contract itself, with a reason that names the member. A record is never completed with a default value, unit or author to let it through.
- **AX-003** — *Readable by the person deciding.* Better is a larger share of an operation's meaning that can be read from the record alone: its target, its new value with its unit, and why. Completeness never comes from a field that states acceptance, approval or compliance.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-069 OBJ-015 | REQ-006 REQ-008 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 metaschema. Closure and the absence of defaults are stated by the criterion |
| OUT-001 | SOW-069 OBJ-015 | REQ-001 REQ-011 | AC-002 | VER-002 | The three lists compared are written separately, none generated from another: the published schema, DEL-07-09's vocabulary and the engine's admitted kinds. The emitted operations of each route are observed, not their declared form |
| OUT-001 | SOW-069 OBJ-015 | REQ-002 | AC-003 | VER-003 | The families are those decomposition row DEL-16-01 lists; the invented examples and their altered copies state what must validate and what must fail |
| OUT-001 | SOW-069 OBJ-015 | REQ-003 REQ-008 | AC-004 | VER-003 | The altered records state what must fail |
| OUT-001 | SOW-069 OBJ-015 | REQ-004 REQ-009 | AC-005 | VER-001 | The vocabularies are read against the prohibited claims PRD §21.2 lists; the altered records state what must fail |
| OUT-001 | SOW-069 OBJ-015 | REQ-005 | AC-006 | VER-004 | PRD §5.4 and `docs/IP_AND_DATA_BOUNDARY.md`, read by a reviewer who did not write the examples |
| OUT-001 | SOW-069 OBJ-015 | REQ-007 | AC-007 | VER-001 | The comparison is against the model document's schema, which DEL-02-01 owns |
| OUT-001 | SOW-069 OBJ-015 | REQ-010 | AC-008 | VER-001 | Decomposition AB-00-04 fixes the hash basis. The altered records state what must fail |
| OUT-001 | SOW-069 OBJ-015 | REQ-012 | AC-009 | VER-005 | The files that are not batches state what must be refused |
