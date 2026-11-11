---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-05
package_id: PKG-14
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-073]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-05 Comparison mapping, tolerance, and export contracts

## Purpose and Objective Traceability

Comparison in SWBPIPE has one set of contracts. They say how two entities or results correspond when their identifiers differ, how something with no counterpart is classified, what a tolerance is and how it is applied, and what a comparison says when it leaves the application as JSON, CSV or a report section. The two comparison engines produce to these contracts and the exports and report sections read them, so that a comparison means the same thing wherever it appears, carries its units and its evidence with it, and never rests on a tolerance the user did not choose.

It serves SOW-073 under OBJ-016 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-14, row DEL-14-05), and `docs/PRD.md` §8.8, §15.4 (entity mapping), §15.5 (tolerance profiles), FR-CMP-005 and FR-CMP-007 (Should: map, unmap and mark entities intentionally unmatched; map entities whose identifiers or topology changed), FR-CMP-008 (Should: absolute and relative tolerances by result category) and FR-CMP-010 (Should: a comparison exports as CSV, JSON and a report section). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The comparison contracts: mapping records and unmatched classifications, tolerance profiles, the common comparison-result interface, and the export semantics of a comparison as JSON, CSV and a report section.

## Deliverable Definition — Ontology

**Terms the contracts define.** DEL-14-03 and DEL-14-04 take these definitions from here.

- *Comparison.* A deterministic difference between two model states, two analysis runs, or both. It is a diagnostic aid for design iteration and review, and never validation or acceptance.
- *Side.* Left or right. Each side is a participant.
- *Participant.* One side of a comparison, named by reference and hash: its model state and unit system, and, where the side is a run, its analysis run and its result document.
- *Mapping hierarchy.* Two things correspond, in this order: by equal stable identifier; else by a manual mapping the user made; else they are unmatched. No correspondence is inferred from position, name, proximity or apparent similarity.
- *Mapping record.* The statement that one thing on the left corresponds to one on the right: what kind of thing is mapped (an entity, a result, a diagnostic, a setting or a load basis), whether the match is automatic or manual, both references, the evidence for it, and its review state and provenance (`schemas/comparison_mapping.schema.json`).
- *Unmatched record.* The statement that something has no counterpart: which side it is on (present on one side only, that is added or deleted, PRD §15.4) and, separately, whether the user marked it intentionally unmatched (PRD FR-CMP-005).
- *Tolerance profile.* A named set of tolerance rules a user defines (`schemas/comparison_tolerance.schema.json`). It has no numeric default.
- *Tolerance rule.* For one result category and dimension: an absolute tolerance, a relative tolerance, or both with the stated way they combine; the unit the tolerance is expressed in; and what happens when unit metadata is missing. The categories are at least those of PRD §15.5: displacement, rotation, force, moment, reaction, stress, stress ratio, temperature, pressure, and support gap or movement.
- *Common comparison-result interface.* The one versioned schema for the result of a state comparison or a run comparison: the participants, the mappings used, the unmatched records, each difference with its units, the tolerance profile referred to and the classification it gave, diagnostics, assumptions, hashes, the professional-boundary notice and provenance.
- *Export semantics.* What each exported form of a comparison must hold. The JSON form is the comparison-result document. The CSV form is one row per compared or unmatched record, with a stable row identifier, stated columns, a deterministic order, and a stated delimiter and encoding. A report section refers to a comparison by identity and hash and keeps its diagnostics, statuses, limitations and notice.

**Undecided.**

- The fields and the version of the common comparison-result interface. Until a version is accepted, no producer or export claims conformance to it.
- Any unmatched classification beyond one-side-only and intentionally unmatched.
- Default tolerance values, and the workflow by which a user reviews and confirms mappings (decomposition OI-014). Until a value is decided, the product supplies none.

**Boundary.**

- **CLM-001** — Computing a state comparison is DEL-14-03's and computing a run comparison DEL-14-04's. They produce the comparison-result document and consume mapping records and tolerance profiles. Nothing here compares, and the determinism of a comparison is theirs to show (also cited as DEL-14-05:3).
- **CLM-002** — Writing a comparison table of the desktop to a CSV file, and mapping the comparison-result interface into the result export document, are DEL-08-04's. The comparison sections of a report, and their layout, are DEL-08-06's and DEL-08-01's. The mapping editor and the comparison views are DEL-07-08's.
- **CLM-003** — The unit catalog and the dimension vocabulary are DEL-02-02's. State and run records are DEL-14-01's and DEL-14-02's; the contracts refer to them by identity and hash and define no second shape for them.
- **CLM-004** — Outside this deliverable: comprehensive ingestion of a commercial prover's results (PKG-14), determining external validation, and supplying any tolerance or acceptance rule from a code or an owner's standard. A user may put such a value in a private tolerance profile of their own (PRD §5.4); the product never supplies one. A participant is named by reference and hash, so the contracts do not assume that a side is a SWBPIPE run (PRD FR-CMP-011, §15.6, not an MVP requirement).

## Completion and Reliance Basis — Epistemology

A record that validates against these contracts is well formed. It is not thereby right: whether a mapping is the correct correspondence is the user's judgment, and whether a tolerance is suitable is the user's engineering decision.

### Requirements

- **REQ-001** — The contracts support the deterministic comparison of two model states, two analysis runs, or both, on stable identifiers (also cited as DEL-14-05-R001). Source: SOW-073.
- **REQ-002** — The mapping contract represents a manual mapping wherever stable identifiers do not align, with both references and the evidence for the match (also cited as DEL-14-05-R002). Source: SOW-073; PRD §15.4, FR-CMP-007.
- **REQ-003** — The contract represents an unmatched entity or result explicitly, with its classification, including one a user marked intentionally unmatched (also cited as DEL-14-05-R003). Source: PRD §15.4, FR-CMP-005.
- **REQ-004** — The tolerance contract expresses absolute and relative tolerances by result category, for differences in a common unit. It supplies no default engineering value: a rule without a value is stated as having none (also cited as DEL-14-05-R004). Source: SOW-073; PRD §15.5, §5.5, FR-CMP-008.
- **REQ-005** — Every unit-bearing value in a comparison result or its export carries its unit and dimension. A value without them is a blocking finding, and no export writes it as a bare number (also cited as DEL-14-05-R005). Source: PRD §5.6.
- **REQ-006** — The JSON form of a comparison is a schema-first result document in JSON Schema 2020-12 (also cited as DEL-14-05-R006). Source: decomposition AB-00-03.
- **REQ-007** — The CSV form of a comparison is defined by its export semantics, which are stated without embedding protected standards data, proprietary engineering values, private project data or a private rule pack's payload; an export names a private rule pack by identity, version and checksum only (also cited as DEL-14-05-R007). Source: SOW-073; PRD §20.1.
- **REQ-008** — A report section of a comparison preserves its diagnostics, provenance, hashes or audit-manifest reference, analysis statuses, limitations and professional-boundary notice, and reaches the report through the reporting deliverables with those preserved (also cited as DEL-14-05-R008 and DEL-14-05:4). Source: PRD FR-REP-004, §19.2.
- **REQ-009** — No contract, field, status or example claims certification, sealing, approval, authentication, external validation or code compliance (also cited as DEL-14-05-R009). Source: SOW-073; PRD §21.2.
- **REQ-010** — Public artifacts of the contracts bundle no protected standards text or tables, no code-specific acceptance criteria, no proprietary values and no private user data (also cited as DEL-14-05-R010). Source: PRD §20.1.
- **REQ-011** — The contracts refer to model states, analysis runs, result documents and units as their owners define them (also cited as DEL-14-05-R011). Source: SOW-073.
- **REQ-012** — There is one common comparison-result interface with one authoritative versioned schema. DEL-14-03 and DEL-14-04 produce it and DEL-08-04 maps it; it binds mappings, the account of what is unmatched, tolerance references, units and diagnostics, and it selects no numeric tolerance and no report layout (also cited as DEL-14-05-SCA011-REQ-001, DEL-14-05-SCA011-VER-001 and DEL-14-05:2). Source: decomposition, SCA-011 ownership amendment.
- **REQ-013** — What the product emits conforms to the contracts: the mappings the engines produce and consume, the tolerance profiles a user defines, and the JSON and CSV a comparison is exported as, with their stable identifiers, units, diagnostics and provenance (also cited as DEL-14-05:1). Source: SOW-073; PRD FR-CMP-010.

### Acceptance criteria

- **AC-001** — The mapping schema and the tolerance schema are valid JSON Schema 2020-12. Invented mapping, unmatched and tolerance records validate, and a record lacking a reference, its evidence, its provenance, or the unit and dimension of a tolerance rule is refused.
- **AC-002** — A manual mapping names one thing on each side by stable reference, with its evidence and review state. An unmatched record names its side and its classification, and a user's mark of intentionally unmatched is distinct from every other classification. The contract admits no mapping that names neither an identifier match nor a user's mapping.
- **AC-003** — A tolerance rule for each category of PRD §15.5 can be written with an absolute value, a relative value, or both with their stated combination, in a unit of the category's dimension. A rule with no numeric value validates and is marked as having none, and no schema default, example or fixture supplies a number in its place.
- **AC-004** — The common comparison-result interface exists at one accepted, named version. An invented state comparison and an invented run comparison validate against it, and a result with a difference lacking unit metadata, a difference that names no mapping, or a boundary field set to claim approval is refused.
- **AC-005** — The JSON export of an invented comparison is its comparison-result document. Its CSV export has one row per compared and unmatched record with a stable row identifier, the stated columns and order, and the same identifiers, values and units as the JSON. A report-section reference to it carries its identity, hashes, diagnostics, statuses, limitations and notice.
- **AC-006** — A comparison made in the product yields mappings, a tolerance profile reference and exports that each validate against these contracts, with the stable identifiers of both sides unchanged.
- **AC-007** — No contract, example or fixture holds protected or private content or a code-derived tolerance, and none states or implies certification, approval, validation or compliance.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- The contracts are schemas first. An engine, an export or a view conforms to them and none defines its own shape for the same thing.
- A definition is stated once, here, and referred to by the deliverables that use it.
- A tolerance is always the user's. The contract has a place for a missing value so that nothing needs to invent one.
- Every exported form derives from the one comparison-result document, so that the forms cannot disagree.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference (the JSON Schema 2020-12 specification, applied by a validator that imports no product code) with an executable check (observes invented valid records and records constructed with each required member missing).
- **VER-002** — Executable check (observes constructed mapping and unmatched records of each kind against the mapping schema, including a mapping without an identifier match or a user's mapping).
- **VER-003** — Executable check (observes a rule for each PRD §15.5 category with each admitted combination of values, a rule with no value, and a scan of the schema, examples and fixtures for a supplied numeric tolerance).
- **VER-004** — Comparison with a named reference (the accepted version of the comparison-result schema, under a validator that imports no product code) with an executable check (observes invented state and run comparisons and each constructed invalid witness).
- **VER-005** — Executable check (observes the JSON and CSV exports and the report-section reference of one invented comparison, and compares identifiers, values and units between them).
- **VER-006** — Executable check (observes the mappings, tolerance reference and exports of a comparison made through the product's own comparison path against the schemas).
- **VER-007** — Judgment by a reviewer who did not write the contracts, who examines them, their examples and fixtures against PRD §20.1 and §21.2 with the protected-content scan.

## Governing Values and Decisions — Axiology

- **AX-001** — *Explicit correspondence over inferred correspondence.* Better is more of the entities a user cares about paired by an identifier or by the user's own mapping. It may never be gained by guessing a match the user did not make.
- **AX-002** — *No tolerance the user did not choose.* Better is more comparisons classified against a profile the user defined. A default, a silent threshold or a value the product supplies from a code is never the way to get there.
- **AX-003** — *One meaning in every form.* Better is fewer differences between what a comparison says on screen, in JSON, in CSV and in a report; the target is none. A form that drops units, diagnostics or the notice to look simpler is not acceptable.
- **AX-004** — *Evidence, not verdict.* Better is a larger share of differences shown with their size, unit and the tolerance rule that classified them. A comparison shows what differs and by how much. Wording or structure that reads as validation or acceptance is never traded for clarity.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-073 OBJ-016 | REQ-001 REQ-006 REQ-011 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification. The invalid records are constructed, so the criterion states which are refused |
| OUT-001 | SOW-073 OBJ-016 | REQ-002 REQ-003 | AC-002 | VER-002 | Owner ruling: the mapping hierarchy of PRD §15.4 and FR-CMP-005. The records are constructed against it, not taken from an engine's output |
| OUT-001 | SOW-073 OBJ-016 | REQ-004 | AC-003 | VER-003 | Owner ruling: the categories of PRD §15.5 and the no-default rule of PRD §5.5; the scan compares against those clauses |
| OUT-001 | SOW-073 OBJ-016 | REQ-005 REQ-012 | AC-004 | VER-004 | The reference is the comparison-result schema at its accepted version, which is undecided (see Undecided); once a version is accepted the schema is compared with the JSON Schema 2020-12 specification and the invalid witnesses are constructed |
| OUT-001 | SOW-073 OBJ-016 | REQ-006 REQ-007 REQ-008 | AC-005 | VER-005 | The criterion fixes the expected result: every form holds the same identifiers, values and units. The forms are compared with each other and with the invented comparison they were made from |
| OUT-001 | SOW-073 OBJ-016 | REQ-013 CLM-001 CLM-002 | AC-006 | VER-006 | Outside source: the schemas, applied by a validator that imports no product code to what the product emitted |
| OUT-001 | SOW-073 OBJ-016 | REQ-009 REQ-010 CLM-004 | AC-007 | VER-007 | Owner ruling: PRD §20.1 and §21.2, judged by a reviewer against those clauses |
