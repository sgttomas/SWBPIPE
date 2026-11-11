---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-03
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-021,SOW-077]
package_objective_refs: [OBJ-006]
---

# Scope of Work — DEL-07-03 Material, component, and rule-pack editors

## Purpose and Objective Traceability

This deliverable is where the engineer enters the engineering data a model depends on: materials and their temperature tables, sections, components, load cases with their loads and combinations, self-weight, private libraries and rule packs. Each editor asks for the value, its unit and its source, checks what it can, and queues the edit for review. None of them carries a table, a factor or a default of its own: the public product supplies the forms, and the user supplies every code-specific or proprietary value. The result this deliverable contributes is that every input an analysis needs can be entered, sourced and reviewed in the desktop before it is solved or checked.

It serves SOW-021 and SOW-077 under OBJ-006 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-03), and PRD §14.1, §10.6, §5.4, §5.5, §18.1 to §18.4 and FR-RULE-004 to FR-RULE-006. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It supports criterion 1, whose load cases and combinations, including the three-axis static seismic combinations, are entered here, and criterion 7, whose authoring time these editors share.

- **OUT-001** — The material, section, component, private-library and rule-pack editors.
- **OUT-002** — The load manager: load cases, their loads, states and pressure regions, combinations, condition use, and the self-weight plan.

## Deliverable Definition — Ontology

**Terms the editors implement.**

- *Editor.* A form or table that shows a record's members with their units and source, accepts the user's entries, and produces a structured operation. An editor holds a draft until the user queues it; the draft is transient.
- *Material editor.* Entry of a material's constitutive basis, elastic and shear modulus, Poisson's ratio, an optional material-level thermal expansion coefficient, temperature-dependent tables and expansion laws, each with its source.
- *Section and component editors.* Entry of pipe-section dimensions and of component geometry, weight, centre of gravity, stiffness and user modifiers, with the user's source for each.
- *Private library.* A user's own collection of material, section, component or hanger records, imported from a file, validated, and stored locally. A library is private unless the user states otherwise, and no library ships with the product.
- *Hanger selection.* The user's manual choice of a record from an imported hanger library for a support. It sizes nothing; sizing is DEL-04-11's.
- *Rule-pack manager.* The list of the project's local rule packs, with create, open, save and delete, the editors that compose a pack's declarations, check definitions and expressions, its validation findings and its checksum.
- *Rule-pack reference.* A rule pack as a model or a run names it: identity, version, checksum, source notice, redistribution status and required inputs.
- *Load manager.* The editor of load cases and combinations: a case's kind, primitive loads, wind exposure, analysis and reference states and pressure regions; a combination's basis and operands.
- *Combination operand.* A term of a combination: a load case or another combination, with a factor. The bases are mechanics, result-state subtraction, range envelope, absolute sum and square root of the sum of squares.
- *Condition use.* A load case's selection of operating-condition keys and set-level uses, which the engine resolves pipe by pipe.
- *Self-weight plan.* The engine's generated set of operations that add self-weight loads to a case for selected pipes under a gravity the user enters. The editor shows the plan and queues it; it computes no weight.

**Boundary.** These editors collect and present. The following belong elsewhere:

- support and restraint creation and editing, and generic editing in the tree, tables and inspector: DEL-07-02;
- the records themselves: materials DEL-03-01, sections and components DEL-03-02, bends DEL-03-03, branches DEL-03-04, rigid components DEL-03-05, expansion joints DEL-03-06, mass properties DEL-03-08;
- library import checks and provenance: DEL-03-07;
- load generation, case resolution and self-weight generation: DEL-05-01; combination algebra: DEL-05-02; user loads: DEL-05-05;
- the rule-pack format, the expression evaluator, the completeness check and rule-pack storage: DEL-06-01, DEL-06-02, DEL-06-03 and DEL-06-04;
- running a rule check and showing its outcome: DEL-07-04;
- validation and application of every operation: DEL-16-02 and DEL-16-06;
- coverage of the operation vocabulary: DEL-07-09.

## Completion and Reliance Basis — Epistemology

An editor establishes that a value was entered with a unit and a stated source. It establishes nothing about whether the value is right, or whether its source permits its use.

### Requirements

- **REQ-002** — Materials, sections, components, load cases, rule packs and private libraries are each editable and reviewable in the desktop (SOW-021; PRD §14.1).
- **REQ-003** — The material editor records a source for every material and offers no allowable, code table or material value of its own (PRD §18.1, §5.4).
- **REQ-004** — The section and component editors treat dimensions, weights, centres of gravity, stress-intensification and flexibility factors, stiffnesses and catalogue values as the user's data, each with its source (PRD §18.2, §18.3).
- **REQ-005** — The load manager enters load cases, their primitive loads, states and pressure regions with units. Code-specific load combinations are the user's: the product supplies combination bases and no combination (SOW-077; DEC-111).
- **REQ-007** — A rule-pack reference shows its identity, version, checksum, source notice, redistribution status, required inputs and the findings for inputs that are missing.
- **REQ-008** — Every edit is a structured operation that is validated, queued for review and applied on the user's Apply. No editor writes the model, a library or a rule pack by another route (PRD §5.3).
- **REQ-009** — Drafts, selection and validation display are transient and are not written to the model document.
- **REQ-010** — A value an editor needs and lacks is reported in place, with its class, before the edit is queued. No editor supplies an engineering default: a blank optional quantity is absent, and a quantity with a value and no unit, or a unit and no value, is refused (PRD §5.5).
- **REQ-011** — Private materials, components, libraries and rule packs stay in user-controlled local storage and are marked private by default. No editor transmits them or writes them into a public example (PRD §20.3, FR-RULE-004).
- **REQ-012** — Editor texts distinguish entered data, readiness to solve, readiness for a user-rule check and human review, and none states or implies code compliance (PRD §21.2).
- **REQ-014** — *Rule-pack manager.* The user creates, opens, saves and deletes local rule packs, composes required inputs, load-case mappings, expressions, limits and warnings, sees the pack's validation findings for units, references and required fields, and generates and stamps its checksum (PRD §10.6).
- **REQ-015** — *Private libraries.* The user imports material, section, component and hanger libraries from files, sees every import finding of PRD §18.4, stores an accepted library locally, applies a record's values to a model entity, which keeps a reference to the library record they came from, and chooses a hanger record for a support by hand. A library reference in the model that no longer matches its library is reported (PRD FR-RULE-005, FR-RULE-006; DEC-103).
- **REQ-016** — *Combinations.* A combination is entered on any of the five bases, with ordered operands that are load cases or combinations, each operand, factor and envelope mode entered by the user. A blank means not chosen, and queueing names what is missing. Self-reference, a cycle, a repeated operand and a combination operand of a declared operating comparison are prevented in place with the reason. A combination another combination names cannot be deleted, and the editor lists what names it.
- **REQ-017** — *Condition use.* For each load case the user selects condition keys and set-level uses, and the editor shows the engine's per-pipe resolution and its refusals by name. An unticked use is absent, never false. "Operating" is offered among case kinds. A difference basis is offered only on a result-state subtraction and is written only when the user sets it; the operating-minus-sustained template prefills neither case.
- **REQ-018** — *Self-weight.* The user chooses pipes and enters gravity; the editor shows the engine's plan with what it includes and excludes, and queues it as one batch only while the plan matches the current model.
- **REQ-020** — *Bend realization.* An intent that inserts a bend at a corner is applied only through batch review, after a preview of that exact batch on the current model.

### Acceptance criteria

- **AC-001** — For each of materials, sections, components, load cases, rule packs and libraries: a complete invented record entered through its editor queues an operation, changes nothing until Apply, and after Apply is stored with every entered value, unit and source exactly as entered. A record without a source is refused by name. No field of any editor is prefilled with an engineering value, and no editor text is a statement REQ-012 excludes.
- **AC-002** — For each route of OUT-002 (load case, primitive load, state, pressure region, combination, condition use, self-weight plan), valid and invalid entries are exercised through the operation engine: a valid entry produces the operations of its kind and nothing else; an entry with a wrong dimension, a missing unit or a missing required member is refused in place by name and queues nothing.
- **AC-003** — A blank optional quantity is stored as absent; a half-entered quantity is refused before queueing; and the canonical hash of the model is unchanged by opening, drafting in or cancelling any editor.
- **AC-004** — A rule pack composed in the manager with invented inputs, mappings, expressions and limits validates, is saved privately, reopens with the same content and checksum, and reports each unit, reference and required-field fault by name when one is introduced. A rule-pack reference shows the seven items of REQ-007.
- **AC-005** — An invented library file with each fault of PRD §18.4 shows the corresponding finding and is not stored; a clean one is stored locally, marked private, and its records can be applied to the model. A model reference to a changed or deleted library record is reported. Choosing a hanger record writes that record's values to the support and nothing computed.
- **AC-006** — Combinations on each of the five bases, with load-case and combination operands, are stored with exactly the operands, factors, order and mode entered, and pass the engine's own check before they are queued. Each of the four prevented shapes is refused in place. Deleting a referenced combination is refused and lists the referencing paths.
- **AC-007** — For an invented model with condition sets, the per-pipe resolution shown for a case equals the engine's, a pipe the engine refuses is named, an unticked use is absent from the stored record, and a difference basis appears in the stored combination only when the user set it.
- **AC-008** — The self-weight plan shown equals the engine's plan for the chosen pipes and gravity; a plan made on an earlier model cannot be queued; and the batch applied is the plan.
- **AC-010** — A bend-insertion intent cannot be applied before its preview, and a preview of an earlier batch or an earlier model does not unlock it.
- **AC-011** — Private library and rule-pack data are written only under the user's local storage paths, appear in no export, report or issue bundle unless the user exports them, and no network request carries them.

## Production and Verification Method — Praxeology

**Method choices that define the editors.**

- An editor builds an operation and the engine decides. A rule the engine has is not restated in the editor; where an editor refuses early, it uses the engine's check and shows the engine's code.
- Absence is a value. A blank optional member is stored as absent, never as zero, false or a default.
- Templates write structure, not numbers. A template may create the records a common pattern needs; every case, factor and basis in them is chosen by the user.
- Code-specific content enters only as the user's data, in a material, a library or a rule pack the user owns.
- Private by default: a library or rule pack is local and private until the user says otherwise.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the queued operations, the stored records after Apply and the model hash): enters invented records through each editor of OUT-001 with the operation engine, for AC-001 and AC-003.
- **VER-002** — Executable check (observes the operations produced and the refusals shown for valid and invalid entries on each route of OUT-002): for AC-002 and AC-010.
- **VER-003** — Executable check (observes saved and reopened rule packs, their findings and checksums, and stored libraries and their findings): for AC-004 and AC-005.
- **VER-004** — Comparison with a named reference (the operation engine's check of a combination, DEL-05-01's case resolution and its self-weight plan, each evaluated by the engine on the same invented model): compares what the editor shows, stores and queues, for AC-006, AC-007 and AC-008.
- **VER-005** — Judgment by a named role (an independent security reviewer the owner arranges): examines where private library and rule-pack data are written and whether any path sends them out, for AC-011; with an executable check that observes file writes and network requests during editor use.
- **VER-006** — Judgment by a named role (an independent reviewer who did not write the code): examines editor texts against PRD §21.2 and `docs/claims_registry.md`, and every field for a prefilled engineering value, for AC-001.

## Governing Values and Decisions — Axiology

- **AX-001** — *Complete and sourced before fast.* Better is fewer inputs an analysis needs that the desktop cannot enter, and fewer that can be entered without a source, measured against the reference system's records. Entry is never made faster by a prefilled value, an inferred unit or a default basis.
- **AX-002** — *Fewer steps for repeated data.* Better is less repeated typing for data that genuinely repeats across pipes, supports and cases, measured by the entries the reference system's loads take. No repetition control writes to the model directly or repeats a value the user did not enter.
- **AX-003** — *Keep the user's data the user's.* Nothing that would make sharing or support easier justifies moving a private library or rule pack out of the user's storage, or bundling an example that is not invented.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-021 OBJ-006 | REQ-002 REQ-003 REQ-004 REQ-008 REQ-012 | AC-001 | VER-001 VER-006 | The entered record is the expected stored record; validity is judged by the document validator and the operation engine, which the editors did not write. The record contents are PRD §18.1 to §18.3; the excluded statements are an owner ruling, PRD §21.2 |
| OUT-002 | SOW-021 SOW-077 OBJ-006 | REQ-005 REQ-008 | AC-002 | VER-002 | The criterion states the expected operations and refusals; the refusal codes are the operation engine's and the unit system's |
| OUT-001 | SOW-021 OBJ-006 | REQ-009 REQ-010 | AC-003 | VER-001 | Owner ruling: PRD §5.5. Absence and the unchanged hash are observed on the stored document |
| OUT-001 | SOW-021 OBJ-006 | REQ-007 REQ-014 | AC-004 | VER-003 | Owner ruling: PRD §10.6. Validation findings and the checksum are DEL-06-01's and DEL-06-04's, not the manager's |
| OUT-001 | SOW-021 OBJ-006 | REQ-015 | AC-005 | VER-003 | Owner ruling: PRD §18.4 lists the import findings; the findings themselves are DEL-03-07's |
| OUT-002 | SOW-077 OBJ-006 | REQ-016 | AC-006 | VER-004 | The entered combination is the expected stored record; the prevented shapes and the delete refusal are the operation engine's rules, which the editor did not write. The combined values are DEL-05-02's to establish |
| OUT-002 | SOW-077 OBJ-006 | REQ-017 | AC-007 | VER-004 | The engine's resolution is the reference and the editor only displays it: this guards against the display diverging and claims no correctness of the resolution, which is DEL-05-01's |
| OUT-002 | SOW-077 OBJ-006 | REQ-018 | AC-008 | VER-004 | The engine's plan is the reference and the editor only displays and queues it; the weights' correctness is DEL-05-01's and DEL-03-08's |
| OUT-001 | SOW-021 OBJ-006 | REQ-020 | AC-010 | VER-002 | Owner ruling under the scope delegation: a bend moves the corner to points the user did not type, so the preview precedes Apply |
| OUT-001 | SOW-021 OBJ-006 | REQ-011 | AC-011 | VER-005 | Owner ruling: PRD §20.3 and FR-RULE-004. The storage paths are DEL-12-01's; the examination is by a reviewer who did not write the code |
