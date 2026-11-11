---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-01
package_id: PKG-02
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-041, SOW-065]
package_objective_refs: [OBJ-001, OBJ-012, OBJ-014]
---

# Scope of Work — DEL-02-01 Canonical domain model schema

## Purpose and Objective Traceability

There is one model document, and it says what the piping model is. Every tool that edits, solves, stores, exports or reports a model reads and writes that same typed, versioned document, so no entity or value exists only in a screen, an agent message or a format of its own. A person can inspect the document, a reader can refuse it by name, and two tools given the same document mean the same model.

It serves SOW-041 and SOW-065 under OBJ-001, OBJ-012 and OBJ-014 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-02, row DEL-02-01), and PRD §5.2 (the schema-backed physical model is the source of truth), §8.2, §5.5 (no silent engineering defaults), §11.1 FR-MOD-001 to FR-MOD-006 and §20.1 to §20.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It supports criterion 1, since every required analysis is solved from one model document, and criterion 6, since an import returns the same entity identities.

- **OUT-001** — The model document format: its schema, its one current version, the document validator, and the format rules every reader and writer follows.

## Deliverable Definition — Ontology

**Terms the format implements.**

- *Model document.* The JSON document of kind `openpipestress.product_preview.model` that the desktop opens, edits and saves and the solver reads. The desktop's model document is the canonical model (owner ruling OD-4, `docs/COMPLETENESS_MAP.md` §4.2). `schemas/model.schema.yaml` describes it.
- *Current version.* The model document has one current version, 0.9.0. A change to what a document can contain, including a new optional member, is a version step of at least the minor level (DEC-033).
- *Format and desktop profile.* The format is what every reader requires: version, kind, project, analysis status, nodes, pipe segments and supports. The desktop profile is the format plus the members the desktop requires to open and save a document. A headless solve request needs only the format.
- *Identifier.* A non-blank string that names one record of a family. Records refer to each other by identifier.
- *Quantity.* A value with its entered unit, `{value, unit}`. Node coordinates alone are bare numbers in the project's length unit. Coordinates and directions are in one implicit global right-handed Cartesian frame; the document declares no other coordinate system.
- *Closed record.* A record that admits only the members the schema names.
- *Solve input and unread member.* A solve input is a member some solve reads. An unread member is model data no solve reads: tags, notes, lines, unresolved assumptions, constraints, equipment interfaces, rule-pack references and provenance records.
- *Document validator.* The one check of the unread members' rules. A blocking finding refuses the document or the operation that would produce it; a warning names a condition and never blocks. A quarantined disposition is a provenance record or rule-pack reference whose redistribution status says protected content is suspected or whose review status is quarantined, the dispositions the library import quarantines (DEL-03-07).
- *Model hash.* The SHA-256 of the document's RFC 8785 canonical text. It is computed, never stored in the document.

### CLM-006 — What the document contains

- **Root:** the version, the document kind and the data-boundary statement. No migration or compatibility status is stored.
- **Project:** identity, name, the project units, rule-pack references, model settings, tags and notes.
- **Model objects:** nodes with coordinates; pipe segments (the elements) with their end nodes, material, section and operating-condition set; sections; materials; components; supports, whose restraints express the restrained degrees of freedom, with an optional direction on a single translational restraint and the snubber family; load cases with their loads, condition use and test condition; combinations, including difference bases, the absolute-sum and SRSS bases and combinations that take earlier combinations as operands; operating-condition sets with keyed conditions; response spectra; the pressure contract; and the load and reference-state group, defined in `schemas/load_reference_state.schema.json`.
- **Model data no solve reads:** typed lines, constraints and equipment interfaces as records without evaluation; unresolved assumptions and open issues; tags and notes; provenance strings and structured provenance records.
- **Diagnostics** carried by the document, and the professional-boundary statement.
- **Referenced, not carried:** Result and Report records are named by reference to the result and report schemas. Results, input manifests, run and report records, storage policy and report settings belong to their own formats.

**Boundary.** This deliverable owns what a document may contain and whether a given document is well formed. It owns no meaning beyond that:

- Physical validity, admission and what each solve input means belong to the solve (DEL-04-07) and to each member's owner: components (PKG-03), linear supports and direction (DEL-04-03), nonlinear supports (DEL-04-04), loads, condition resolution and the test condition (DEL-05-01), combinations (DEL-05-02), the load and reference state (DEL-05-05), modal data (DEL-04-09), snubbers and spectra (DEL-04-10), and the default node-proximity tolerance (DEL-04-06).
- The unit catalog is DEL-02-02's. The version check, the project store and round-trip persistence are DEL-02-05's. The analysis status vocabulary is DEL-02-03's.
- The operation engine that applies changes is DEL-16-06's; this deliverable defines the change kinds that write its own model-data and model-settings members and operating-condition sets with their assignment and use. The desktop surfaces that edit them are DEL-07-02's.
- Constraint evaluation, design-knowledge entry and candidate generation are PKG-13's and are not in v1.0. The physical-to-analytical transform and its own model shape are DEL-13-04's. State- and run-level assumptions, tags and notes are DEL-14-01's and DEL-14-02's.
- It implements no numerical solving and no library population.

## Completion and Reliance Basis — Epistemology

The format describes structure. A document that conforms is well formed; that establishes nothing about the physical system it describes.

### CLM-011 — Requirements

Requirement numbers 001 to 012 are those of the earlier `REQ-02-01-nn` series.

- **REQ-001** — The schema is a JSON Schema 2020-12 document in strict JSON syntax with a stable `$id`.
- **REQ-002** — The schema defines or references a record for Project, Model, Node, Element, Material, Component, Load and LoadCase, Result and Report, and for every family in CLM-006. Lines, nodes, elements, supports, components, loads, materials, constraints and equipment interfaces are typed entities (FR-MOD-002).
- **REQ-003** — Every addressable record has a stable identifier, and records refer to each other by identifier, never by position. An identifier persists across edits unless the user creates a new entity (FR-MOD-003).
- **REQ-004** — Every dimensional value carries its unit, except node coordinates, which are in the project's length unit. A missing or incompatible unit is a named finding, never a default.
- **REQ-005** — Materials, sections, components with their SIF and flexibility modifiers, supports, loads, operating-condition sets and rule-pack references can record their source. Materials, sections, components and supports can also carry a structured provenance record with the fields of PRD §20.2 and a verification note (FR-MOD-005).
- **REQ-006** — The schema, its descriptions, and its fixtures and examples contain no protected standards text, tables, formulas, allowables, SIF or flexibility tables, protected dimensional tables, proprietary catalogue data or private user data, and supply no code value as a default.
- **REQ-007** — The document keeps mechanics status, rule-check status and professional acceptance as separate fields and has no automatic code-compliance, certified or approved state.
- **REQ-008** — The document's diagnostics, and the Result and Report records it references, carry a code, class, severity, source, affected object, message, remediation and provenance.
- **REQ-009** — A document serializes deterministically, and a round trip returns equal content: model data, units, references, tags, notes and warnings (FR-MOD-001). Hashes are taken over the RFC 8785 canonical text.
- **REQ-010** — Every path by which a document enters the product applies the same format and validator checks: file open, project load, the headless runner, every operation apply, and the native package, PCF, CAEPIPE MBF and library import boundaries. No adapter or plugin admits a document that fails them.
- **REQ-011** — Public examples and fixtures use invented, public-domain or permissively licensed data, each with a provenance manifest. Code-specific examples stay private.
- **REQ-012** — Conformance is examined in layers, each against its own criterion: schema, units, provenance, protected content, deterministic round trip, and Result and Report compatibility. No layer stands in for another.
- **REQ-013** — The current version is named once per language, and production code names a model-document version only through that constant.
- **REQ-014** — The current version carries every member of CLM-006 in one version (also cited as DEL-02-01:2 and DEL-02-01:3).
- **REQ-015** — Nothing is defaulted or inferred. Every member beyond the format's required ones is optional and its absence means none. An authored null is refused, not read as absence. The exceptions are the members on which a null is read as absence: the label and provenance of a node and of a pipe, a section's mill tolerance, densities and insulation, a load case's modulus basis, and a support's family and nonlinear behaviour. Whether these should refuse a null too is undecided, and no other member admits one. The product writes no member the user did not author, and clearing a member removes it. Each pipe stores its own condition-set reference; inheritance along a run belongs to authoring tools, and no reader infers a set from topology.
- **REQ-016** — A member the document carries is read with its defined meaning, or refused or withheld by name wherever a solve does not support it. It is never dropped. A case's condition use means exactly the per-case records it resolves to: a document solves as its resolved twin, except for the identities bound to the authored document, and a missing set, condition or value refuses the case by name.
- **REQ-017** — The unread members change no solve. A document that carries them solves exactly as the same document without them, and nothing evaluates lines, constraints or equipment interfaces.
- **REQ-018** — The schema describes exactly the persisted contract: every member any reader decodes or any writer emits. Every record is closed, except the data-boundary statement, an open map of strings, and a section's provenance, a string or an object. Every document the desktop writes satisfies the desktop profile.
- **REQ-019** — The document validator runs wherever a document enters or changes: native file open, the webview open path, project load, and every operation apply and validate-only preview. It refuses a malformed member by a named code and its JSON path, reports a missing subject or a quarantined disposition as a warning, and never repairs, defaults or normalizes a document. A document with a blocking finding is not saved to the project store.
- **REQ-020** — Tags and notes are allowed on the project and on every model object and model-data record, and no tag vocabulary is enforced (FR-MOD-004). Assumptions and open issues are recorded at project, line, component and support level, with a resolution note once resolved or rejected (FR-MOD-006). A reference from an assumption, constraint or equipment interface never blocks deleting the entity it names; the missing subject is a warning. A pipe belongs to at most one line. A rule-pack reference carries the pack's identity and its stamped checksum only; rule content never enters the document.
- **REQ-021** — Model settings are an optional record of model-level settings: the node-proximity tolerance, a positive finite length in a length unit of the unit catalog, and the shear model. An absent setting takes the default its owning deliverable defines, and the default is never written into the document.
- **REQ-022** — This deliverable's model-data and model-settings members, and operating-condition sets with their assignment and use, are created, changed and removed only through structured operations. Each operation refuses by name, changes nothing else in the document, and has an inverse that restores the prior member or its absence.
- **REQ-023** — The physical model in the document is the source of truth for editable piping design data: geometry, components, supports, equipment interfaces, loads, constraints, provenance, unresolved assumptions and warnings. It does not replace the PKG-13 to PKG-16 contracts. The model can carry physical-to-analytical traceability and typed references to design knowledge, model operations, model states, analysis runs, comparisons, handoff packages and external-prover metadata. Whether those typed references are still intended is undecided: the owner's reading of CLM-006 places result, run and report records in their own formats.
- **REQ-024** — The domain examples include typed `LoadRecord` fixtures that conform to their schema, covered by a provenance manifest (also cited as DEL-02-01:1). Which `LoadRecord` definition they conform to is undecided: the model schema's, or that of the physical-to-analytical transform's model shape (DEL-13-04), which the domain examples use.

### Acceptance criteria

- **AC-001** — The schema is valid JSON Schema 2020-12 in strict JSON. Every committed document at the current version validates against the format; every desktop-profile document, the bundled demo included, also validates against the desktop profile; and every document the desktop or the operation engine emits on each authoring path validates.
- **AC-002** — Each family of REQ-002 and CLM-006 has a named definition, every record is closed apart from the two stated exceptions, and every addressable record has an identifier that other records name.
- **AC-003** — A document carrying every member comes back with equal content and the same model hash through each path: save and reopen in the project store; save and open as a file; the headless persistence envelope; and undo and redo of each operation of REQ-022.
- **AC-004** — A document carrying every unread member solves end to end with results equal to those of the same document without them.
- **AC-005** — For each validator rule, a malformed member is refused with its code and JSON path at file open, at project load and at apply; a missing subject and a quarantined disposition are reported as warnings and block nothing; and the document's bytes are unchanged by the check.
- **AC-006** — For each optional member, absence leaves the document's meaning unchanged, and an authored null is refused by name on every member but the exceptions REQ-015 names. Each member a solve does not support is refused or withheld by its named code through the desktop and the headless entry, on both the pressure-free and the exact route, and none is dropped.
- **AC-007** — A document whose cases use operating conditions gives the same results as its resolved twin, and a case naming a missing set, condition or value is refused by name.
- **AC-008** — No production code compares or prints a model-document version except through the one constant of its language.
- **AC-009** — Every dimensional member of the schema except node coordinates is a quantity with a unit, a quantity without a unit is refused, and a node-proximity tolerance that is not finite, not positive or not in a length unit is refused by name.
- **AC-010** — Each family of REQ-005 accepts its source, the structured provenance record carries every field of PRD §20.2, each public fixture and domain example is covered by a provenance manifest, and each typed `LoadRecord` fixture of the domain examples validates against the `LoadRecord` definition of REQ-024.
- **AC-011** — The schema text, fixtures and examples contain no protected or private content and make no claim of compliance, certification or approval, and the document has no state that asserts them.
- **AC-012** — At each entry path of REQ-010, a document that fails the format or the validator is refused and is not adopted.
- **AC-013** — Each operation of REQ-022 applies its change and nothing else, refuses each invalid request by name, and is inverted exactly; undo in the desktop restores the prior document and its model hash.
- **AC-014** — The document's diagnostic record carries the fields of REQ-008, and the Result and Report references resolve to definitions that carry them.

## Production and Verification Method — Praxeology

**Method choices that define the format.**

- One schema file with reusable definitions at a stable `$id`; the load and reference-state group keeps its own schema file and is referenced.
- The schema is the description. Judgment at run time is the validator's and the readers', and the conformance checks keep the three in agreement.
- Conformance is by rule over every committed document, found by its document kind, including documents embedded in requests and envelopes. An exclusion is a named entry with its reason: a retired-version document is a refusal control or a hash-bound frozen input.
- The desktop and the operation engine each write a golden document per authoring path under a fixed clock; a fresh emission that differs from its golden fails.
- One validator serves native and webview code, so open and apply judge a document identically.
- Members are stored explicitly on the record they describe.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002 and AC-014, executable check: observes the schema, every committed and emitted document and the named definitions under a JSON Schema 2020-12 validator that imports no product code.
- **VER-002** — AC-003, executable check: observes the content and the SHA-256 of the RFC 8785 text before and after each round-trip path.
- **VER-003** — AC-004 and AC-007, executable check: observes the published results of a document and of its twin, and the refusal of a case with a missing set, condition or value.
- **VER-004** — AC-005 and AC-012, executable check: observes, for constructed malformed documents, the code, JSON path and unchanged bytes at each entry path and at apply.
- **VER-005** — AC-006 and AC-009, executable check: observes, for constructed documents, the refusal of each authored null, each unit failure and each unsupported member on each route.
- **VER-006** — AC-008, executable check: observes production source for version literals outside the one constant of each language.
- **VER-007** — AC-010 and AC-011, judgment by a reviewer who wrote neither the schema nor the fixtures, with the protected-content scan and an executable check: the reviewer examines the schema text, each fixture's provenance manifest and the status vocabulary; the check observes each typed `LoadRecord` fixture of the domain examples under its `LoadRecord` definition with a validator that imports no product code.
- **VER-008** — AC-013, executable check: observes each operation's applied document, refusals and inverse, and the desktop session's undo.

## Governing Values and Decisions — Axiology

- **AX-001** — *One document, exactly described.* Better is fewer members that a reader decodes or a writer emits without the schema naming them, and fewer committed documents no rule classifies; the target is none. A closed record is never opened, and a rule never weakened, to let a document through.
- **AX-002** — *Say nothing the user did not.* Better is fewer values the product supplies on the user's behalf. A required field is required only for what it serves: a value needed to solve, to check a rule or to report is asked for there, and its absence is a named finding. A default is never written to make a solve or an export go through.
- **AX-003** — *Carry before meaning, refuse by name.* The format may carry a member before any solve reads it, so that a version step is not needed for each consumer. Better is fewer carried members without a named refusal or a visible statement that the solve does not use them. A result is never computed while ignoring a member that would change it.
- **AX-004** — *Open structure, private values.* The schema is public and structural; code-specific and proprietary values are the user's. Realism in an example never justifies protected or private content.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-041 OBJ-001 | REQ-001 REQ-012 REQ-018 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code. The goldens are the product's own earlier output and only guard against change |
| OUT-001 | SOW-041 SOW-065 OBJ-001 OBJ-014 | CLM-006 REQ-002 REQ-003 REQ-014 REQ-018 REQ-023 | AC-002 | VER-001 | Owner ruling: the families are those of PRD FR-MOD-002 and CLM-006, and the criterion states what must be found |
| OUT-001 | SOW-041 OBJ-012 | REQ-009 | AC-003 | VER-002 | The criterion fixes the expected result: the document read back equals the document written. The hash form is an outside source, RFC 8785 with SHA-256 |
| OUT-001 | SOW-065 OBJ-012 OBJ-014 | REQ-017 | AC-004 | VER-003 | The criterion states the expected relation, equal results. Both sides are the product's own output, so it establishes that the members are unread and claims no numerical correctness |
| OUT-001 | SOW-041 SOW-065 OBJ-014 | REQ-019 REQ-020 | AC-005 | VER-004 | The documents are constructed, so the criterion and the rules of REQ-020 state the expected finding for each |
| OUT-001 | SOW-065 OBJ-012 | REQ-014 REQ-015 REQ-016 | AC-006 | VER-005 | Owner ruling: PRD §5.5 and the one-version rule state what must happen; the documents are constructed |
| OUT-001 | SOW-065 OBJ-012 | REQ-016 | AC-007 | VER-003 | The criterion states the expected relation between a document and its twin. Both sides are the product's own output: it guards the equivalence only, and the resolved values' numerical grounds are DEL-05-01's independent references |
| OUT-001 | SOW-041 OBJ-012 | REQ-013 | AC-008 | VER-006 | The criterion states the expected result: no literal is found |
| OUT-001 | SOW-041 OBJ-012 | REQ-004 REQ-021 | AC-009 | VER-005 | Owner ruling: PRD §5.6 and REQ-021 state what is refused; unit and dimension membership come from the unit catalog (DEL-02-02) |
| OUT-001 | SOW-065 OBJ-014 | REQ-005 REQ-011 REQ-024 | AC-010 | VER-007 | Owner ruling: PRD §20.2 lists the fields a public record carries. Outside source for the load-record fixtures: the JSON Schema 2020-12 specification, applied by a validator that imports no product code |
| OUT-001 | SOW-041 OBJ-001 | REQ-006 REQ-007 | AC-011 | VER-007 | Owner ruling: PRD §20.1 and §21.2 and `docs/IP_AND_DATA_BOUNDARY.md`; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-041 OBJ-012 | REQ-010 | AC-012 | VER-004 | The documents are constructed, so the criterion states the expected refusal at each entry path |
| OUT-001 | SOW-065 OBJ-012 OBJ-014 | REQ-022 | AC-013 | VER-008 | The criterion fixes the expected result: the stated change and its exact inverse. Hash equality after undo uses RFC 8785 with SHA-256 |
| OUT-001 | SOW-041 OBJ-001 | REQ-008 | AC-014 | VER-001 | Owner ruling: the diagnostic fields are those of the architecture basis AB-00-06 in the decomposition |
