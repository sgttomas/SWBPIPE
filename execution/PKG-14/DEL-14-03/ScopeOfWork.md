---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-03
package_id: PKG-14
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-073, SOW-071]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-03 Model-state comparison engine

## Purpose and Objective Traceability

An engineer can ask what changed between two saved designs and get the same answer every time: which entities were added, which were removed, which changed and which were left alone, and for a changed entity, which fields differ and what each state holds. Entities are paired by their stable identifiers, or by a mapping the user made where an identifier changed, and by nothing else. The answer is a diagnostic aid for design iteration and review; it validates neither design.

It serves SOW-073 and SOW-071 under OBJ-016 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-14, row DEL-14-03), and `docs/PRD.md` §8.8, §10.5, §15.3 (the model inputs a comparison covers), §15.4, FR-CMP-003 (Must: added, removed, unchanged and modified entities are reported), FR-CMP-006 (Must: stable identifiers are the default mapping) and, for states, FR-CMP-007 (Should: manual mappings). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The model-state comparison: the comparison of two saved model states and its result.

## Deliverable Definition — Ontology

**Terms the product implements.** Comparison, side, mapping hierarchy, mapping record, unmatched record and the common comparison-result interface are defined by DEL-14-05 and used here as defined.

- *Compared entity.* Every addressable record of a state's model document, in its family: the nodes, elements, branches, bends, components, supports, loads, materials, sections, rule-pack references, constraints and unresolved assumptions of PRD §15.3, and the model-level members that are not a collection, solver settings among them; the order of each collection is compared too. Every member of the document belongs to exactly one of them.
- *Classification.* An entity only on the right is added. An entity only on the left is removed. A mapped pair with no differing field is unchanged, and a mapped pair with at least one is changed.
- *Unresolved.* A mapped pair that cannot be classified: a mapping that does not resolve to one entity on each side, or a changed unit-bearing field with missing unit or dimension metadata, with incompatible dimensions, or with different units and no governed normalization contract. It is reported as unresolved with a blocking diagnostic, never as changed or unchanged.
- *Field difference.* For a changed entity, the path of each field that differs and the value each side holds, as stored. Two field values are equal when their RFC 8785 canonical forms are equal.
- *Structural comparison.* The comparison subtracts nothing and gives no numeric difference of a unit-bearing value. It converts a unit only under a governed normalization contract.
- *State context.* What the result carries about each side beside the entities: the state's identity, name and hashes, and its warnings, unresolved assumptions, notes and external references.

**Undecided.**

- The governed normalization contract under which a changed field whose two sides are in different units could be classified. Until one is decided, such a field is unresolved.
- Whether a tolerance profile applies to any field of a state comparison (decomposition OI-014).

**Boundary.**

- **CLM-001** — Saved model states and their verified read are DEL-14-01's, and the model document and its stable identifiers DEL-02-01's. The unit catalog, which says what dimension a unit has, is DEL-02-02's.
- **CLM-002** — The mapping contract, the unmatched classifications and the common comparison-result interface are DEL-14-05's. Comparing the results of two runs, and every numeric difference in a common unit, is DEL-14-04's. Tolerance profiles are DEL-14-05's.
- **CLM-003** — The comparison view and the selection of two states in the desktop are DEL-07-08's; the CSV file of the comparison table is DEL-08-04's; the comparison sections of a report are DEL-08-06's.
- **CLM-004** — Outside the product: matching suggested from geometry or topology, external validation, and any statement that one state is acceptable.

## Completion and Reliance Basis — Epistemology

A comparison says how two stored documents differ. It does not say that a difference matters, and an unchanged entity is one whose stored fields are equal, not one that behaves the same.

### Requirements

- **REQ-001** — The engine compares two saved, immutable model states (also cited as REQ-14-03-001). Source: SOW-071; PRD §15.3.
- **REQ-002** — Stable identifiers are the primary basis of matching and are applied automatically (also cited as REQ-14-03-002). Source: SOW-073; PRD FR-CMP-006.
- **REQ-003** — Where two entities correspond and their identifiers differ, they are compared as a pair only under an explicit mapping record (also cited as REQ-14-03-003). Source: SOW-073; PRD §15.4, FR-CMP-007.
- **REQ-004** — Every entity of both states is classified as added, removed, changed or unchanged, or reported as unresolved (also cited as REQ-14-03-004). Source: decomposition row DEL-14-03; PRD FR-CMP-003.
- **REQ-005** — The same two states, mappings and settings give the same result, in content and in order (also cited as REQ-14-03-005). Source: SOW-073.
- **REQ-006** — The result keeps each state's context: unresolved assumptions, warnings, notes, external references and hashes are not dropped (also cited as REQ-14-03-006). Source: SOW-071.
- **REQ-007** — A unit-bearing value is never compared as a bare number. A changed unit-bearing field carries its unit and dimension on both sides. Missing metadata, incompatible dimensions, and different units with no governed normalization contract are each reported unresolved with a blocking diagnostic. The comparison gives no numeric difference of a unit-bearing value (also cited as REQ-14-03-007). Source: PRD §5.6.
- **REQ-008** — The result carries diagnostics, with provenance, for a missing or unresolvable mapping, a mapping of a category the comparison does not support or one that pairs entities of different families, an unresolved assumption of either state, and the limits of what the comparison establishes (also cited as REQ-14-03-008). Source: decomposition AB-00-06.
- **REQ-009** — No output claims certification, sealing, authentication, professional approval, code compliance or external validation (also cited as REQ-14-03-009). Source: SOW-073; PRD §21.2.
- **REQ-010** — Fixtures and examples of state comparison are invented and hold no protected standards text, proprietary value or private project data (also cited as REQ-14-03-010). Source: PRD §20.1.
- **REQ-011** — The result is produced against DEL-14-05's common comparison-result interface, with the stable identifiers and mappings of its source unchanged and no conversion applied (also cited as DEL-14-03-SCA011-REQ-001, DEL-14-03-SCA011-VER-001 and DEL-14-03:2). Source: decomposition, SCA-011 ownership amendment.
- **REQ-012** — In the product, a user compares two saved states of a project and receives this comparison of their stored snapshots, with its mappings, its unit and missing-data diagnostics and each state's context (also cited as DEL-14-03:1). Source: PRD §10.5, FR-CMP-003.
- **REQ-013** — The comparison is complete: every member of both model documents takes part, and a member the comparison cannot assign to an entity refuses the comparison by name. Nothing is skipped. Source: PRD §15.3.

### Acceptance criteria

- **AC-001** — For invented pairs of states constructed with a known entity added, one removed, one changed in a known field and the rest untouched, the comparison reports exactly those classifications, and for the changed entity exactly the paths of the differing fields with both stored values.
- **AC-002** — Repeating a comparison gives a byte-identical result. Listing the entities of a collection in another order in either state changes the classification of none of them; the change of order is itself reported as a difference.
- **AC-003** — Two states compare with every entity unchanged exactly when their model hashes are equal.
- **AC-004** — Two entities with different identifiers are compared as a pair when an explicit mapping record names both, and are reported removed and added without one. A mapping that names an absent entity is reported unresolved with a diagnostic, and an identifier repeated within one family of one state refuses the comparison by name.
- **AC-005** — A changed field whose two sides hold quantities in different units, a unit-bearing field without unit or dimension metadata, and one whose two sides have incompatible dimensions are each reported unresolved with a blocking diagnostic and no numeric difference. Quantities in the same unit and dimension are reported changed, with both values as stored.
- **AC-006** — The result carries each state's identity, name, hashes, warnings, unresolved assumptions, notes and external references as the state's record holds them.
- **AC-007** — A model document with a member the comparison does not assign is refused by name, and the product compares the two stored snapshots of the states selected, read through DEL-14-01's verified read.
- **AC-008** — A comparison result validates against the accepted version of the common comparison-result interface, a result made invalid in each required member is refused, and the identifiers and mappings of the source survive a round trip through its exported forms.
- **AC-009** — Every implementation of the comparison in the product gives the same classifications, summaries and field paths for the same pairs.
- **AC-010** — No result, label or message states or implies approval, validation or compliance, and every fixture and example is invented and carries its provenance.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- Matching is by identifier or explicit mapping only. Position, name and proximity pair nothing.
- The comparison is structural. It reads stored values and reports every stored difference.
- The comparison fails closed: a document it cannot fully account for is refused, never partly compared.
- Rows are produced in a fixed order that does not depend on the order of the input.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the classifications, field paths and values for invented pairs constructed with each kind of difference).
- **VER-002** — Executable check (observes repeated comparisons byte for byte, and comparisons of the same pair with collections reordered).
- **VER-003** — Comparison with a named reference (the model hashes of the two states, as DEL-02-01 defines them and an implementation of RFC 8785 and SHA-256 that imports no product code computes them) for identical pairs, pairs that differ only in member order, and pairs with single mutations.
- **VER-004** — Executable check (observes pairs with explicit mappings, with a mapping to an absent entity, and with a repeated identifier).
- **VER-005** — Executable check (observes changed fields constructed with different units, missing unit metadata, incompatible dimensions and equal units).
- **VER-006** — Executable check (observes the state context of the result against the two state records, a document with an unassigned member, and the snapshots the product's comparison read).
- **VER-007** — Comparison with a named reference (the accepted version of the comparison-result schema, under a validator that imports no product code) with an executable check (observes constructed invalid results and an export round trip).
- **VER-008** — Executable check (observes each implementation on one committed set of invented pairs and compares their classifications, summaries and field paths; it shows that the implementations agree and nothing more).
- **VER-009** — Judgment by a reviewer who did not write the comparison, who examines its outputs, wording and fixtures against PRD §20.1 and §21.2 with the protected-content scan.

## Governing Values and Decisions — Axiology

- **AX-001** — *Identity over resemblance.* Better is more of a design's real correspondences captured by stable identifiers and the user's mappings. It may never be gained by inferring that two entities are the same.
- **AX-002** — *Exact over forgiving.* Better is a comparison that reports every stored difference; the target is that equal hashes and no differences always coincide. A difference is never hidden because it looks insignificant.
- **AX-003** — *Unresolved over guessed.* Where a pair cannot be classified honestly it is reported unresolved with the reason. Better is fewer unresolved pairs, never by treating a missing unit or mapping as if it were present.
- **AX-004** — *Context travels with the difference.* Better is fewer assumptions and warnings of either state absent from the result; the target is none.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-073 SOW-071 OBJ-016 | REQ-001 REQ-002 REQ-004 | AC-001 | VER-001 | The pairs are constructed with known differences, so the criterion fixes the expected classifications and field paths; nothing is taken from the engine's earlier output |
| OUT-001 | SOW-073 OBJ-016 | REQ-002 REQ-005 | AC-002 | VER-002 | The criterion fixes the expected result: the second run equals the first, and order changes no classification |
| OUT-001 | SOW-073 SOW-071 OBJ-016 | REQ-004 REQ-013 | AC-003 | VER-003 | Outside source: the model hash, which is RFC 8785 and SHA-256 over the stored snapshot and is not computed by the comparison |
| OUT-001 | SOW-073 OBJ-016 | REQ-003 REQ-008 | AC-004 | VER-004 | Owner ruling: the mapping hierarchy of PRD §15.4, as DEL-14-05 states it. The mappings are constructed, so the criterion states the expected pairing |
| OUT-001 | SOW-073 OBJ-016 | REQ-007 | AC-005 | VER-005 | The fields are constructed with a known unit condition each, so the criterion states the expected report; a unit's dimension comes from DEL-02-02's catalog |
| OUT-001 | SOW-071 OBJ-016 | REQ-006 | AC-006 | VER-006 | The expected context is the two state records, which DEL-14-01 produces and the comparison did not write |
| OUT-001 | SOW-073 OBJ-016 | REQ-012 REQ-013 CLM-001 | AC-007 | VER-006 | The unassigned member is constructed, so the criterion states the refusal; the snapshots compared are checked against the states' stored model hashes |
| OUT-001 | SOW-073 OBJ-016 | REQ-011 CLM-002 | AC-008 | VER-007 | The reference is the accepted version of DEL-14-05's comparison-result schema, which this deliverable does not own; DEL-14-05 states its version as undecided |
| OUT-001 | SOW-073 OBJ-016 | REQ-005 | AC-009 | VER-008 | The product's own output on committed pairs, compared between its implementations. This guards against the implementations drifting apart and is not grounds for correctness; AC-001 and AC-003 carry those |
| OUT-001 | SOW-073 OBJ-016 | REQ-009 REQ-010 CLM-004 | AC-010 | VER-009 | Owner ruling: PRD §20.1 and §21.2, judged by a reviewer against those clauses |
