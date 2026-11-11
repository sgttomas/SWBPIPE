---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-02
package_id: PKG-13
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-068, SOW-067]
package_objective_refs: [OBJ-014, OBJ-018]
---

# Scope of Work — DEL-13-02 Constraint entity and provenance model

## Purpose and Objective Traceability

A constraint is a condition the piping design has to respect, recorded as data: these points must connect, this route must stay out of that volume, this clearance must be kept, a support may stand only in this zone, this line must slope, drain or vent, or a value some check needs has not been supplied. This deliverable gives each such condition one typed record that says what kind it is, what it applies to, which design knowledge it rests on, which values it uses, and who or what stated it: the user, the project, an import, an agent, or a source document. With it a constraint can be reviewed, traced to its origin and checked by the constraint engine. Without it, constraints live in people's heads, in report prose or in agent text, where nothing can check them and nobody can tell whose they are.

It serves SOW-068 and SOW-067 under OBJ-014 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-13, row DEL-13-02), and PRD §8.2 and §13.2, FR-KNOW-004 (each constraint identifies its source), and FR-KNOW-002 and FR-KNOW-003 as the record their findings attach to. PKG-13 is outside v1.0 (rulings OD-1 and OD-3, `docs/COMPLETENESS_MAP.md` §4.2), and no exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The constraint format: its schema, the constraint kinds, the record of a constraint with its origin and provenance, and the classes of finding a constraint document carries.

## Deliverable Definition — Ontology

**Terms the format implements.**

- *Constraint.* One recorded condition on the design, with a stable identifier, a kind, a name, a state, its origin, the model entities it applies to, the design-knowledge records it rests on, its parameters, its unresolved assumptions and its provenance.
- *Constraint kind.* One of the categories of SOW-068: connectivity, route (a no-go volume included), clearance, support zone, slope, drain and vent, and missing required data.
- *State.* Whether a constraint is proposed, in force, not in force, superseded, or cannot be evaluated because data is missing.
- *Origin.* Who or what stated the constraint: the user, the project, an import, an agent, or a source document. An origin that is not known is stated as unknown.
- *Parameter.* A named value a constraint uses: a text, a flag, a quantity or a list of references, with its own provenance. A quantity has its value, unit, dimension and provenance, as in DEL-13-01.
- *Constraint set.* The constraints of one project and one model, with the design-knowledge documents they rest on, the findings raised against the set, and its provenance. It is a versioned document.
- *Finding class.* The class of a finding a constraint document carries. There is one for each category of SOW-068 (connectivity conflict, route conflict, clearance conflict, support-zone conflict, slope, drain and vent conflict, missing constraint data), with classes for a provenance warning, a unit warning, a data-boundary warning and a schema failure. Each is one of the six warning classes of the diagnostic contract or a local class with a stated mapping to them (DEL-00-06).
- *Provenance record.* As DEL-13-01 states it: the seven fields of PRD §20.2 with a privacy classification.

**Undecided.**

- How this format relates to the model document. The model document (DEL-02-01) is the canonical model and already carries constraints as typed records that nothing evaluates. Whether the constraint of this format is that member, a definition that member takes, or a separate document that refers to the model is not decided.
- PRD FR-KNOW-004 also names a project standard and an owner requirement as sources of a constraint. Whether each is a distinct origin is not decided.

**Boundary.** This deliverable defines the record of a constraint. It evaluates nothing: finding a violated constraint is DEL-13-03's. The design-knowledge records a constraint rests on are DEL-13-01's, and the model document is DEL-02-01's. Showing constraints and their findings in the desktop is DEL-07-08's, and the operations that create or change a constraint are DEL-16-01's. The format bundles no value: no limit, owner standard, code requirement or acceptance rule is written into the schema or a public example. A parameter's value is supplied by the user or the project, including a value the user takes from an owner's requirement or a code and supplies privately with its provenance.

## Completion and Reliance Basis — Epistemology

A constraint record states a condition that someone recorded. It does not establish that the condition is the right one, that the set is complete, or that a design which satisfies every recorded constraint is acceptable.

### CLM-010 — Requirements

Requirement numbers 001 to 010 are those of the earlier `R-13-02-nnn` series.

- **REQ-001** — The format represents a constraint of each category SOW-068 names: connectivity, route conflict, clearance conflict, support-zone conflict, slope, drain and vent conflict, and missing required data.
- **REQ-002** — A constraint can be tied to the design knowledge it rests on, for each category SOW-067 names: endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope, drain and vent requirements, and owner and project metadata.
- **REQ-003** — A constraint record identifies its origin where it is known: the user, the project, an import, an agent, or a source document (decomposition row DEL-13-02; SOW-068 under PRD FR-KNOW-004).
- **REQ-004** — A value that a solve or a rule check requires and that is missing is recorded as a finding with its provenance, never as a default (`docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-005** — A unit-bearing value a constraint uses is a quantity in the unit system's vocabulary, with its unit and dimension. Dimensionless is never the fallback for a missing unit (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1; also cited as DEL-13-02:2).
- **REQ-006** — The schema, its descriptions and every public example contain no protected standards text, tables or criteria, proprietary catalogue data, private owner standards, private project data or code-specific values (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1).
- **REQ-007** — The format defines no status by which software states certification, sealing, authentication, professional approval or code-compliance acceptance (PRD §21.2).
- **REQ-008** — The schema is JSON Schema 2020-12 in strict JSON.
- **REQ-009** — A constraint takes part in the physical model through typed references to model entities, the findings raised against it, its unresolved assumptions and its trace links (PRD §5.2, §8.2).
- **REQ-010** — The schema has a stable `$id`, and every document declares its schema version in semantic form.
- **REQ-011** — The consumers of constraint data, namely the constraint engine, the transform and the desktop surfaces that show constraints or block on them, are demonstrated against actual constraint documents, and the documents they read conform to this schema (also cited as DEL-13-02:1).

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12 in strict JSON with a stable `$id`. A document that omits its schema version, or carries a member the schema does not name, is refused.
- **AC-002** — For each category of REQ-001, a conforming document can state a constraint of that kind with its targets, and a finding of that category's class. A constraint with no kind or an unknown kind is refused.
- **AC-003** — A constraint names the design-knowledge records it rests on and the model entities it applies to by typed reference, a record of each category of REQ-002 can be named, and a constraint's unresolved assumptions are read back tied to it.
- **AC-004** — Each origin of REQ-003 can be stated and is told apart from the others. A constraint with no origin, or without its provenance record, is refused, and an unknown origin is stated as unknown and never omitted.
- **AC-005** — A conforming document can state a missing required value as a missing-data constraint and as a finding. The schema declares no default for any member. Every finding a document carries has the fields of the diagnostic contract and a class that is one of the six warning classes or a local class whose mapping to them the schema states.
- **AC-006** — A quantity without a unit or a dimension is refused, and the set of dimension identifiers the schema admits equals the unit schema's set.
- **AC-007** — No member name, enumerated value or description in the schema, and no public example, states or implies certification, sealing, authentication, approval or code compliance.
- **AC-008** — The schema text and every public example contain no protected or private content and no limit or criterion value, and each public example carries a provenance record that marks it invented or cleared.
- **AC-009** — Every constraint document that the constraint engine, the transform or a desktop surface reads in the product validates against this schema.

## Production and Verification Method — Praxeology

**Method choices that define the format.**

- One schema file at a stable `$id`. Records are closed: a member the schema does not name is refused.
- The record of a constraint is kept apart from its evaluation. The format holds what was stated and what was found; it holds no rule for finding it.
- Origin and provenance sit on the constraint, and provenance on each parameter and quantity.
- The schema declares no default.

**How each criterion is examined.**

- **VER-001** — AC-001, executable check: observes the schema and constructed documents under a JSON Schema 2020-12 validator that imports no product code.
- **VER-002** — AC-002, AC-003, AC-004 and AC-005, executable check: observes, for constructed documents, each kind, origin and finding class accepted, each refusal, the references and assumptions read back, and the schema's definitions for defaults and class mappings.
- **VER-003** — AC-006, comparison with a named reference: the dimension identifiers of `schemas/units.schema.yaml`, read from that file; with an executable check that observes the refusal of a quantity lacking a unit or dimension.
- **VER-004** — AC-007 and AC-008, judgment by a reviewer who did not write the schema, with the protected-content scan: examines the schema text and each public example with its provenance.
- **VER-005** — AC-009, executable check: observes, under the validator, the constraint documents each consumer reads when it runs in the product.

## Governing Values and Decisions — Axiology

- **AX-001** — *A finding, never a hidden default.* Better is fewer conditions and fewer missing values that exist only outside the record; the target is none. A value is never supplied so that a constraint can be stated or evaluated.
- **AX-002** — *Origin kept.* Better is fewer constraints whose origin a reviewer cannot read. An agent's proposal is never recorded as the user's or the project's, and an unknown origin stays marked unknown.
- **AX-003** — *Named kinds, the user's values.* Better is a format in which more of a project's recorded conditions fall under a named kind while the schema and its public examples state no limit. A kind beyond those of SOW-068 enters by scope change. Realism in an example never justifies a real owner's or a code's value.
- **AX-004** — *Record, do not judge.* The format says what a constraint is and what was found against it. It never carries a status that reads as acceptance, and a finding is review support, not a verdict.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-068 OBJ-014 | REQ-008 REQ-010 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code |
| OUT-001 | SOW-068 OBJ-014 | REQ-001 | AC-002 | VER-002 | Owner ruling: the categories are those of SOW-068; the documents are constructed, so the criterion states what is accepted and refused |
| OUT-001 | SOW-067 SOW-068 OBJ-014 | REQ-002 REQ-009 | AC-003 | VER-002 | Owner ruling: the design-knowledge categories are those of SOW-067; the documents are constructed |
| OUT-001 | SOW-068 OBJ-014 OBJ-018 | REQ-003 | AC-004 | VER-002 | Owner ruling: PRD FR-KNOW-004 states that each constraint can identify its source, the origins are those of decomposition row DEL-13-02 with the import of FR-KNOW-004, and PRD §20.2 lists the provenance fields; the documents are constructed |
| OUT-001 | SOW-068 OBJ-014 | REQ-004 | AC-005 | VER-002 | The criterion states the expected result (PRD §5.5: nothing is defaulted); the diagnostic fields and the six classes are those of the architecture basis AB-00-06 in the decomposition |
| OUT-001 | SOW-068 OBJ-014 | REQ-005 | AC-006 | VER-003 | The unit schema's dimension vocabulary (DEL-02-02; owner ruling DEC-018), read from its file. A list of dimensions kept in the check itself is no reference |
| OUT-001 | SOW-067 OBJ-018 | REQ-007 | AC-007 | VER-004 | Owner ruling: PRD §21.2 lists the prohibited claims; the reviewer compares against it and `docs/claims_registry.md` |
| OUT-001 | SOW-067 OBJ-018 | REQ-006 | AC-008 | VER-004 | Owner ruling: PRD §20.1 and `docs/IP_AND_DATA_BOUNDARY.md` state what a public artifact may contain; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-068 OBJ-014 | REQ-011 | AC-009 | VER-005 | The schema is the reference for each document, applied by a validator that imports no product code; the documents are those the consumers read in the product, not ones written for the check |
