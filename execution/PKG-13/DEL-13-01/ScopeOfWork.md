---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-01
package_id: PKG-13
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-067]
package_objective_refs: [OBJ-014]
---

# Scope of Work — DEL-13-01 Design knowledge schema and provenance model

## Purpose and Objective Traceability

Design knowledge is what a project knows about a piping system that the piping itself does not say: where a line starts and ends, what it carries, where it may run and where it may not, where it can be supported, which equipment it meets, and what has to be reachable, sloped, drained or vented. This deliverable gives that knowledge one typed form. Every record says what it applies to, where it came from, who supplied it, whether it may be shared, and what is still assumed, so that design knowledge is explicit data with a source and never a hidden assumption or a value the product supplied. The constraint model (DEL-13-02) and the constraint engine (DEL-13-03) read these records; without them there is nothing recorded to check a route against.

It serves SOW-067 under OBJ-014 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-13, row DEL-13-01), and PRD §8.1 and §13.1, the format part of FR-KNOW-001, the record-level part of FR-MOD-006, and `docs/SPEC.md` §3.1. PKG-13 is outside v1.0, and no exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The design-knowledge format: its schema, the record kinds, and the provenance model every record carries.

## Deliverable Definition — Ontology

**Terms the format implements.**

- *Design knowledge.* Information the user or the user's organization supplies to inform model authoring and evaluation. It is project data. No part of it is a product default.
- *Design-knowledge document.* One versioned set of records for a project and a model: its identity, the project and model it informs, its records, the findings it carries, and its provenance.
- *Record.* One item of design knowledge, with a stable identifier, a kind, a name, its source notes, its unresolved assumptions and its provenance. The kinds cover the categories of SOW-067:
  - *endpoint*: where a line starts, ends or ties in, with its location and what it connects to;
  - *line data*: a line's identifier, its service and its attributes;
  - *routing corridor*: a region a route should follow, may follow, or must leave free;
  - *zone*: a region of a stated type, a no-go volume and a supportable zone among them;
  - *equipment interface*: the point where piping meets equipment, with its location and parameters;
  - *requirement*: a condition that must hold for named targets (access, slope, drain or vent), with its parameters;
  - *owner and project metadata*: descriptive data about the project, an area, a line or equipment.
- *Geometry.* The extent of a corridor or zone: a point, polyline, surface or volume given by coordinates and boundaries, or a reference to geometry held elsewhere.
- *Source note.* A statement of where a record's content came from or what design basis it rests on, attached to the record and marked public or private.
- *Unresolved assumption.* A statement taken as true without confirmation, attached to the records it affects. It is unresolved, resolved or rejected, with a note once it is resolved or rejected. The model's own assumptions and open issues are members of the model document and are DEL-02-01's.
- *Provenance record.* The seven fields of PRD §20.2 as DEL-01-02 defines them (source name, source location, source licence, contributor, contributor certification, redistribution status, review status), with a privacy classification that says whether the record is public, invented, private or suspected of being protected.
- *Quantity.* A value with its unit, its dimension and its own provenance. The dimension is one of the unit vocabulary's (DEL-02-02).
- *Reference.* A typed pointer to a model entity or to another record: the kind of object and its identifier.
- *Finding.* A diagnostic the document carries, with the fields of the diagnostic contract (DEL-00-06).

**Undecided.**

- How this format relates to the model document. The model document (DEL-02-01) is the canonical model and already carries lines, equipment interfaces and unresolved assumptions as typed records. Whether design knowledge becomes members of that document or stays a separate document that refers to it is not decided, and with it which surface and which operations enter design knowledge.
- The coordinate frame in which corridor and zone geometry is stated, and its tolerance.

**Boundary.** This deliverable defines the records and their provenance. Constraint records are DEL-13-02's, evaluating them is DEL-13-03's, and the physical-to-analytical transform is DEL-13-04's. The design-knowledge panel in the desktop is DEL-07-08's, and the operations that change design knowledge are DEL-16-01's. The paths that read a design-knowledge document for a surface lie in the code of the deliverable that owns that surface, and AC-009 constrains each of them. The provenance fields and what counts as protected content are DEL-01-02's; the unit and dimension vocabulary is DEL-02-02's. The format states no engineering value: no clearance, slope, span or access criterion, and no owner standard or code requirement is a source for it.

## Completion and Reliance Basis — Epistemology

A document that conforms is well formed. Conformance establishes nothing about whether the knowledge is correct, complete or cleared for sharing: a provenance record states what its supplier declared.

### CLM-009 — Requirements

Requirement numbers 001 to 011 are those of the earlier `REQ-13-01-nnn` series.

- **REQ-001** — The format has a record for each category SOW-067 names: endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope, drain and vent requirements, and owner and project metadata (PRD FR-KNOW-001).
- **REQ-002** — Every record can carry source notes and unresolved assumptions, each tied to the record it affects (PRD §13.1; FR-MOD-006).
- **REQ-003** — The schema, its descriptions and every public example or fixture of design knowledge contain no protected standards text or tables, owner standards, code-specific values, proprietary vendor data or private project data (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1).
- **REQ-004** — A value in a record is identifiable as supplied by the user or the project, with its source. The format supplies no value (SOW-067; PRD §5.4).
- **REQ-005** — Every record carries the provenance record with all seven fields of PRD §20.2 and its privacy classification (PRD §13.1; `docs/SPEC.md` §3.1).
- **REQ-006** — Every unit-bearing value is a quantity with its unit and dimension. A value has no unit only where its field is explicitly dimensionless, a ratio, a percentage or a coefficient (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-007** — Missing design knowledge, a missing unit and an unresolved assumption are each stated in the document, as a finding or as a record. Nothing is defaulted, and an unknown is stated as unknown (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-008** — The physical model stays the editable source of truth. A design-knowledge document names the project and model it informs, and a record refers to each model entity it applies to by a typed reference to its identifier and never replaces or edits it (PRD §5.2; `docs/SPEC.md` §3).
- **REQ-009** — The format has no status, member or value by which software states approval, certification, sealing, authentication or code compliance (PRD §21.2).
- **REQ-010** — The schema is JSON Schema 2020-12 in strict JSON with a stable `$id`, and every document declares its schema version.
- **REQ-011** — No path that reads or writes design knowledge goes around schema validation, the unit and provenance checks, the private-data controls, protected-content screening, the result envelope, persistence hashes or the professional-boundary controls (`docs/SPEC.md` §1, §4.4, §4.5).

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12 in strict JSON with a stable `$id`. A document that omits its schema version, or carries a member the schema does not name, is refused.
- **AC-002** — For each category of REQ-001, a conforming document can state a record of that category with its identifier and its location, geometry or targets, and the record's kind tells it apart from every other category. A record with no kind or an unknown kind is refused.
- **AC-003** — A record without its provenance record, or whose provenance lacks any of the seven fields or the privacy classification, is refused. A source note and an unresolved assumption on a record are read back tied to that record's identifier, and an assumption carries its status.
- **AC-004** — A unit-bearing value without a unit or a dimension is refused, and the set of dimension identifiers the schema admits equals the unit schema's set.
- **AC-005** — The schema declares no default for any member. A document can state an unknown value, a missing unit and an unresolved assumption explicitly, and a document read back holds no value its author did not state.
- **AC-006** — A document names its project and its model, and every mention of a model entity in a record is a typed reference by identifier.
- **AC-007** — No member name, enumerated value or description in the schema, and no public example, states or implies approval, certification, sealing, authentication or code compliance.
- **AC-008** — The schema text and every public example or fixture of design knowledge contain no protected or private content, and each public example carries a provenance record that marks it invented or cleared.
- **AC-009** — Every design-knowledge document committed to the repository or bundled in the product validates against the schema, and each path that reads one refuses a document that does not. On such a path, a record with a missing or incompatible unit, missing provenance or unknown redistribution status is stopped with a finding that names it before it is shown, evaluated or applied, and private content is not written to a public destination. No path that reads or writes design knowledge leaves out a control REQ-011 names.

## Production and Verification Method — Praxeology

**Method choices that define the format.**

- One schema file at a stable `$id`. Records are closed: a member the schema does not name is refused.
- Provenance sits on the record it qualifies, and on each quantity, not in a register beside the data.
- The schema gives structure only. Every value comes from the user or the project.
- The schema declares no default, so that absence stays visible.

**How each criterion is examined.**

- **VER-001** — AC-001, executable check: observes the schema and constructed documents under a JSON Schema 2020-12 validator that imports no product code.
- **VER-002** — AC-002, AC-003, AC-005 and AC-006, executable check: observes, for constructed documents, each record kind accepted, each refusal, the notes and assumptions read back, and the schema's definitions for defaults and for references to model entities.
- **VER-003** — AC-004, comparison with a named reference: the dimension identifiers of `schemas/units.schema.yaml`, read from that file; with an executable check that observes the refusal of a quantity lacking a unit or dimension.
- **VER-004** — AC-007 and AC-008, judgment by a reviewer who did not write the schema, with the protected-content scan: examines the schema text and each public example with its provenance.
- **VER-005** — AC-009, executable check with judgment by a reviewer who did not write the paths: observes each committed and bundled design-knowledge document under the validator, and each reading path's refusal of constructed documents that do not conform or that hold a record lacking a unit, its provenance or a known redistribution status; the reviewer examines each path that reads or writes design knowledge against the controls REQ-011 names.

## Governing Values and Decisions — Axiology

- **AX-001** — *Explicit and sourced.* Better is fewer values in a design-knowledge document whose source, supplier or sharing status cannot be read from the record; the target is none. Provenance is never made optional to make entry easier.
- **AX-002** — *Slots, not values.* Better is a format in which more of what a project knows can be stated while the format itself states nothing. Coverage and realism never justify bundling an owner standard, a code value or private project data, in the schema or in an example.
- **AX-003** — *Unknown stays visible.* Better is fewer ways for missing knowledge to pass unmarked. A required member is never filled, and a default never added, so that a document validates.
- **AX-004** — *The model leads.* Design knowledge informs the physical model and points at it. Better is fewer mentions of a model entity in a record that are not a typed reference to it; the target is none. Where a record and the model disagree, the model governs and the disagreement is shown, never resolved by the format.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-067 OBJ-014 | REQ-010 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code |
| OUT-001 | SOW-067 OBJ-014 | REQ-001 | AC-002 | VER-002 | The categories are those of SOW-067 and PRD FR-KNOW-001; the documents are constructed, so the criterion states what is accepted and refused |
| OUT-001 | SOW-067 OBJ-014 | REQ-002 REQ-004 REQ-005 | AC-003 | VER-002 | PRD §20.2 lists the seven provenance fields and PRD §13.1 the public or private status; the documents are constructed |
| OUT-001 | SOW-067 OBJ-014 | REQ-006 | AC-004 | VER-003 | The unit schema's dimension vocabulary (DEL-02-02), read from its file. A list of dimensions kept in the check itself is no reference |
| OUT-001 | SOW-067 OBJ-014 | REQ-004 REQ-007 | AC-005 | VER-002 | The criterion states the expected result (PRD §5.5: nothing is defaulted); the document read back equals the document written |
| OUT-001 | SOW-067 OBJ-014 | REQ-008 | AC-006 | VER-002 | PRD §5.2 makes the physical model the source of truth; the criterion states the reference form the schema must use |
| OUT-001 | SOW-067 OBJ-014 | REQ-009 | AC-007 | VER-004 | PRD §21.2 lists the prohibited claims; the reviewer compares against it and `docs/claims_registry.md` |
| OUT-001 | SOW-067 OBJ-014 | REQ-003 | AC-008 | VER-004 | PRD §20.1 and `docs/IP_AND_DATA_BOUNDARY.md` state what a public artifact may contain; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-067 OBJ-014 | REQ-011 | AC-009 | VER-005 | The schema is the reference for each document, applied by a validator that imports no product code. The stopped documents are constructed, so the criterion states what is stopped; the controls a path keeps are those `docs/SPEC.md` §1, §4.4 and §4.5 name, and the reviewer compares each path against that list |
