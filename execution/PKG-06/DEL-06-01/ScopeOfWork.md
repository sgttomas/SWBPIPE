---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-01
package_id: PKG-06
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-016, SOW-042]
package_objective_refs: [OBJ-005]
---

# Scope of Work — DEL-06-01 Rule-pack schema

## Purpose and Objective Traceability

A user can write their own design basis down as data. A rule pack says what each check needs, which of the user's own stress categories it belongs to, which load cases it is judged on, what is computed, what it is compared with and where every value came from. The result this deliverable contributes is the format that makes such a pack complete, versioned and source-noted, so that solver results can be evaluated against a basis the user owns while the public project holds no code content at all.

It serves SOW-016 and SOW-042 under OBJ-005 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-06, row DEL-06-01), and `docs/PRD.md` §8.5, §10.6 steps 2 and 3, §17.1, §17.2, §17.4, FR-RULE-001, §5.4 and §5.5, with `docs/CONTRACT.md` OPS-K-RULE-3 and OPS-K-DATA-3 and `docs/SPEC.md` §7. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it; there, rule packs are a design aid in v1.0.

- **OUT-001** — The rule-pack document format: `schemas/rule_pack.schema.yaml`, its one current version, the format validation of a pack document, and the basis plan a rule-check run follows.

## Deliverable Definition — Ontology

**Terms the format implements.**

- *Rule pack.* A JSON document the user supplies. It holds structure, declarations and references; the public schema holds none of the user's values.
- *Format version.* One version is current. A pack declares it at the top level and in its metadata, and the two must agree. There is no reader for any other version and no migration: the user re-authors an older pack. The grammar version a pack also declares is separate and is DEL-06-02's.
- *Contents.* Identity (id, name, rule-pack version, lifecycle status, source notice); classification (privacy class and redistribution status); a provenance record; required inputs; value slots; formula declarations; stress categories; required load cases; load-case mappings; check definitions; diagnostics, which are the pack's warnings and blocking conditions; a checksum record; and a professional-boundary record. Together these are the items PRD §17.2 lists.
- *Required input.* A quantity a check needs: its id, its source kind (solver result, model input, user-supplied rule value, private library value or owner design basis), its dimension and unit, whether a rule check requires it, and its provenance and redistribution status. A private-library input holds a reference to a library, a record and a slot, and the value is never in the pack. A solver-result input holds a row selector.
- *Value slot.* A limit, allowable, coefficient or threshold the user supplies when a check is run. The pack holds the slot, its kind, its dimension and unit, and its provenance.
- *Formula declaration.* A declarative expression in DEL-06-02's grammar. A formula is data and never executable code.
- *Stress category.* The user's own name and description. The format gives a category no built-in meaning and has no fixed list.
- *Required load case.* A named reference to one load case or one combination of the model, by the model's own identifier. No two required load cases name the same one.
- *Load-case mapping.* The required load cases assigned to a category. Every category has exactly one mapping, every required load case is in at least one mapping, and every required load case is reachable from at least one check through that check's category.
- *Check.* One category, the required inputs and value slots it reads, one formula, and an acceptability relation: one of the four ordering relations, less-than-or-equal when absent.
- *Basis and basis plan.* A basis is a load case or combination a check is evaluated on. The basis plan lists, for each check, the required load cases mapped to its category, in the mapping's order. A check has one outcome for each of its bases, and an outcome is identified by its check and its basis.
- *Row selector.* What a solver-result input uses to name one row of a solved result within a basis: a result kind and an entity, and optionally a location, a component, a section basis and a tie rod. An optional member that is absent matches only a row without that member. A tie rod is named by its exact identifier. A row without a basis is never selected, and neither is an operand row published under a combination (the held-status subtrahend), which shares the combination's kind, entity, location and component, so that a selector on that combination selects its result.
- *Binding lives in the pack.* The mapping from the user's categories to a model's load cases and combinations is authored in the pack and bound by the pack's checksum. Nothing is added to the model document for it.
- *Provenance record.* Source name, type, location and licence, contributor and contributor certification, redistribution status, protected-content review and review status.

**Boundary.** This deliverable defines the document and checks that a document is well formed. The expression grammar, its encoding in the pack and the unit consistency of a formula are DEL-06-02's. Binding inputs to data and running checks are DEL-06-03's. The checksum's computation and verification, the classification default of a new pack and the keeping of packs are DEL-06-04's. The invented example is DEL-06-05's. The editor is DEL-07-03's, and the model's reference to a pack is DEL-02-01's. The format carries the user's own category names, descriptions and mappings; the repository adds no code-standard text, table, allowable or category definition.

**Undecided.** No deliverable currently commits that a published result row states the load case or combination it belongs to, its kind and its entity, and where it has them its location, component, section basis and tie-rod identity, nor that an operand row published under a combination is marked as one. The row selector and AC-006 rely on both.

## Completion and Reliance Basis — Epistemology

A pack that validates is well formed. Validation says nothing about whether the user's criteria are right.

### Requirements

Requirement numbers are those of the earlier `REQ-06-01-0nn` series.

- **REQ-001** — The schema uses JSON Schema 2020-12.
- **REQ-002** — The schema includes rule-pack identity, schema version, rule-pack version, lifecycle and status, and source-note metadata. One schema version is current; a document declaring any other version is refused by name, without migration.
- **REQ-003** — The schema requires provenance metadata for rule-pack values, allowable slots, formulas and source bases.
- **REQ-004** — The schema records the public or private classification and the redistribution status of each rule pack and of each public-example candidate.
- **REQ-005** — The schema includes checksum metadata identifying the algorithm, the hash scope, the hash value and the canonicalization basis.
- **REQ-006** — The schema supports required-input declarations with units, dimensions, source and provenance requirements, and missing-value findings.
- **REQ-007** — The schema supports formula declarations without embedding protected standards text, protected formulas, copied code equations or arbitrary executable code.
- **REQ-008** — The schema supports user-supplied allowable slots with units, provenance, source notes, redistribution status and completeness status.
- **REQ-009** — The schema represents the outcomes of a user-rule check, within the user's limit, exceeding it or incomplete, and keeps a user-rule outcome distinct from professional approval.
- **REQ-010** — The schema supports structured diagnostics for rule-check blocking, provenance warning, unit mismatch, protected-content warning, evaluator error and redistribution warning.
- **REQ-011** — A public example uses invented non-code values and carries an explicit non-engineering notice, and the format enforces that standing on a pack classified as a public example (also cited as DEL-06-01:1).
- **REQ-012** — The format carries no state that asserts engineering authority, approval, certification or code compliance. Its professional-boundary record holds each such claim false and human review required (PRD §21.2).
- **REQ-013** — The schema supports user-defined stress categories, required load cases that reference model load cases or combinations, and mappings that assign required load cases to each category. Each check names one category and has one outcome for each load case or combination mapped to it. A mapped load case or combination that is absent from the model, has no result rows, or lacks a row the check needs makes that outcome incomplete without evaluating the check on it; the check's other outcomes are reported independently.

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12 in strict JSON. A pack that declares every item of PRD §17.2 validates against it. A pack that lacks a required record or member, the checksum record's algorithm, canonicalization, scope and value among them, or that carries a member the format does not define, is rejected, and the rejection names the member.
- **AC-002** — A pack whose format version is absent, is not the current one, or differs between the top level and the metadata is refused by a finding that names the version found and the version supported. The pack is reported as blocked, and nothing rewrites or migrates it.
- **AC-003** — A pack, required input, value slot, formula, stress category or required load case that lacks its provenance record, any member of it, or its redistribution status is rejected by a named finding, and no value is supplied in its place.
- **AC-004** — Each of these is a blocking named finding at validation: a reference to a category, required load case, input, slot or formula that is not declared; a duplicate identifier; two required load cases naming the same load case or combination; a category with no mapping or more than one; a required load case in no mapping or reachable from no check; and a solver-result input without a row selector.
- **AC-005** — For a valid pack the basis plan lists, for each check, exactly the required load cases mapped to its category, each once and in the mapping's order.
- **AC-006** — For every row that has a basis in every solved result the product publishes, an operand row published under a combination aside, a selector exists that selects that row and no other within its basis, two tie rods of one connector included.
- **AC-007** — A diagnostic a pack declares carries a code, class, severity, source, whether it blocks the rule check, the affected reference, a message, a remediation and provenance; and the declarable classes cover the six conditions of REQ-010.
- **AC-008** — A pack classified as a public invented example validates only with the invented-example redistribution status, a completed protected-content review, an accepted review status and a source notice. A pack that claims a public classification with any other standing is rejected by a named finding.
- **AC-009** — A formula declaration admits only declarative forms, its code-execution member can only be false, and a formula given as an expression tree must name the grammar.
- **AC-010** — The schema text, its descriptions and its fixtures contain no protected standards text, table, formula or allowable and no proprietary or private value. Every fixture value is invented.
- **AC-011** — The statuses a check can declare are analysis statuses that software may emit, and none is human approval. The format has no member or value that asserts approval, certification or code compliance, and a professional-boundary record that asserts one does not validate.

## Production and Verification Method — Praxeology

**Method choices that define the format.**

- The schema is the contract, written in strict JSON. Format rules that a schema cannot express, such as reference integrity and reachability, are findings of the same validation.
- One current version. A change to what a strict reader accepts is a new format version, and older packs are refused, not converted.
- A pack names the model's own identifiers. The format is specific to no code and to no solver result version beyond the row selector's members.
- The basis plan is computed in one place, and the validation and the run both read it from there.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-009, executable check: validates the schema, a complete constructed pack and packs with each required member removed or an undefined member added, under a JSON Schema 2020-12 validator that imports no product code.
- **VER-002** — AC-002, AC-003, AC-004 and AC-008, executable check: observes the findings and the blocked standing that validation returns for constructed packs with each defect, and the pack's bytes before and after.
- **VER-003** — AC-005, executable check: observes the basis plan of constructed packs with shared load cases, several categories and reordered mappings.
- **VER-004** — AC-006, executable check: for every solved result committed as a fixture or witness, counts the rows each row's own selector selects within its basis, operand rows under a combination excluded, and observes that no selector selects an operand row.
- **VER-005** — AC-007, executable check: observes the members of declared diagnostics against the diagnostic contract's field list, and the declarable classes against the conditions of REQ-010.
- **VER-006** — AC-010 and AC-011, judgment by a reviewer who did not write the schema, with the protected-content scan: examines the schema text, descriptions and fixtures, and the status and boundary vocabularies.

## Governing Values and Decisions — Axiology

- **AX-001** — *Neutral structure.* Better is fewer schema members, values, descriptions or fixture contents that presume the categories, terms or limits of any one code; the target is none. A category, limit, formula or term of a real standard never enters the schema, a description or a fixture to make the format easier to use or an example more realistic.
- **AX-002** — *Refuse, do not repair.* Better is fewer packs that validate and then cannot run: more defects caught as named findings at validation. A pack is never migrated, defaulted or normalized to make it validate.
- **AX-003** — *Everything has a source.* Better is fewer members a user can rely on that carry no provenance. A provenance or redistribution requirement is never made optional to ease authoring.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-016 SOW-042 OBJ-005 | REQ-001 REQ-002 REQ-005 REQ-006 REQ-008 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code. The list of contents is PRD §17.2 |
| OUT-001 | SOW-042 OBJ-005 | REQ-002 | AC-002 | VER-002 | The packs are constructed, so the criterion states the expected finding |
| OUT-001 | SOW-016 SOW-042 OBJ-005 | REQ-003 REQ-004 REQ-006 REQ-008 | AC-003 | VER-002 | PRD §20.2 names the provenance fields and `docs/CONTRACT.md` OPS-K-DATA-3 and OPS-K-RULE-3 require them. The packs are constructed |
| OUT-001 | SOW-016 OBJ-005 | REQ-013 | AC-004 | VER-002 | PRD §17.2 lists the records. The packs are constructed, so the criterion states each expected finding |
| OUT-001 | SOW-016 OBJ-005 | REQ-013 | AC-005 | VER-003 | The criterion states the expected plan for each constructed pack |
| OUT-001 | SOW-016 OBJ-005 | REQ-006 REQ-013 | AC-006 | VER-004 | The criterion states the expected count, one. The rows are solved results published by other deliverables, which this one did not write. The count covers the committed fixtures and witnesses only: a row kind or member that no committed result holds is not established by it |
| OUT-001 | SOW-016 OBJ-005 | REQ-010 | AC-007 | VER-005 | The decomposition's AB-00-06 names the diagnostic fields; the conditions are REQ-010's |
| OUT-001 | SOW-016 SOW-042 OBJ-005 | REQ-004 REQ-011 | AC-008 | VER-002 | PRD §17.4 and FR-RULE-007 and `docs/CONTRACT.md` OPS-K-RULE-1. The packs are constructed |
| OUT-001 | SOW-016 OBJ-005 | REQ-007 | AC-009 | VER-001 | PRD §17.3 and `docs/CONTRACT.md` OPS-K-RULE-2 exclude executable code; the comparison is by a validator that imports no product code |
| OUT-001 | SOW-016 OBJ-005 | REQ-007 REQ-011 | AC-010 | VER-006 | PRD §20.1 states what a public artifact may contain. The reviewer compares against sources the product did not write |
| OUT-001 | SOW-016 OBJ-005 | REQ-009 REQ-012 | AC-011 | VER-006 | The reviewer compares with PRD §21.2, `docs/claims_registry.md` and the analysis status vocabulary of DEL-02-03 |
