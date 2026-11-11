---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-05
package_id: PKG-06
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-016]
package_objective_refs: [OBJ-005, OBJ-011]
---

# Scope of Work — DEL-06-05 Invented non-code example rule pack

## Purpose and Objective Traceability

A newcomer can open a complete rule pack, load it and see what a rule check does, without the project publishing a single real engineering criterion. The example is invented from end to end and says so wherever it appears. The result this deliverable contributes is a public demonstration of the rule-pack mechanism that cannot be mistaken for a design basis, a reading of any standard, or a result anyone may rely on.

It serves SOW-016 under OBJ-005 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-06, row DEL-06-05: an example only, with no proprietary content), and `docs/PRD.md` §8.5, §17.4, FR-RULE-007, §20.1, §21.2 and the MVP scope of §23, with `docs/CONTRACT.md` OPS-K-RULE-1. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The invented example rule pack, `examples/rule_packs/invented_demo.yaml`; the copy the desktop loads; and its notice, `docs/_Examples/rule_pack_notice.md`.

## Deliverable Definition — Ontology

**Terms the example implements.**

- *Invented.* Every label, description, category, quantity, unit, formula and value the example itself defines is made up for this repository. None corresponds to a code, a standard, an owner specification, a vendor's data or a real project. The member names, status tokens, dimensions and algorithm names it uses are the format's and the grammar's.
- *Example pack.* A valid rule pack in the current format (DEL-06-01) and a supported grammar version (DEL-06-02), classified as a public invented example with the invented-example redistribution status. It declares at least a solver-result input, a user-supplied input with a limit slot, a category, required load cases that name load cases of the bundled demo model, a declarative formula and a check that uses them.
- *Desktop copy.* The copy the desktop loads as its demonstration pack, `fixtures/product_preview/invented_demo_rule_pack.json`. It is the same bytes as the example.
- *Notice.* The statement that the example is software demonstration content only: not an engineering design basis, not an interpretation of any standard, and not suitable for professional reliance. The pack carries it in its source notice and its reliance notice, and the notice document states it for readers of the repository.
- *Checksum.* The example carries the pack checksum of its own content (DEL-06-04). A change to the content needs a new checksum.

**Undecided.** Whether the example must be able to reach an outcome within or exceeding its limit on the bundled demo model, or need only demonstrate the incomplete outcome.

**Boundary.** This deliverable is the example's content and its notice. The format the example conforms to is DEL-06-01's, the grammar DEL-06-02's, the run that evaluates it DEL-06-03's and the checksum definition DEL-06-04's. The panel that loads the example is DEL-07-04's, the example models that cite it are DEL-11-04's and the bundled demo model is DEL-02-01's. A real rule pack is the user's own, and none is published here.

## Completion and Reliance Basis — Epistemology

The example shows that the mechanism works on artificial content. It establishes nothing about any engineering criterion.

### Requirements

Requirement numbers are those of the earlier `DEL-06-05-REQ-0n` series.

- **REQ-001** — A public rule-pack example uses invented non-code values and clear non-engineering notices.
- **REQ-002** — The example holds no protected standards text, table, figure, copied formula, material allowable, SIF or flexibility table, protected dimensional table or proprietary commercial data.
- **REQ-003** — The example distinguishes a user-rule check from professional approval and claims no certification, sealing, authentication or code compliance for reliance (PRD §21.2).
- **REQ-004** — Code-specific values stay user-supplied or private. The example supplies no default for a value a rule check requires.
- **REQ-005** — The example carries its source, provenance and redistribution status, and its checksum binds its actual content under the canonical hash. No digest is invented, and none stays valid after the content changes.
- **REQ-006** — The example is declarative and not executable.
- **REQ-008** — The example is a pack like any other: it conforms to the schema, carries provenance and diagnostics, and passes the same validation and protected-content gate as any pack, with nothing special-casing it.

### Acceptance criteria

- **AC-001** — The example validates against the rule-pack schema and passes pack validation with no blocking finding, in the current format and a supported grammar version, and its formula is a declarative expression.
- **AC-002** — The example is classified as a public invented example with the invented-example redistribution status; each of its provenance records states an invented origin, a completed protected-content review and an accepted review status; and its source notice, its reliance notice and the notice document each say that it is demonstration content only, not an engineering design basis and not suitable for professional reliance.
- **AC-003** — The example and the notice quote or reproduce no standard, clause, equation, table, allowable, material property, SIF or flexibility value, and hold no owner, vendor or project data. Every label, description, unit and value the example itself defines is invented; the member names, tokens and dimensions are the format's and the grammar's.
- **AC-004** — The example's recorded checksum equals the checksum recomputed from its content, and every copy of the example the product bundles is byte-identical to it.
- **AC-005** — Loaded as shipped, with no value supplied, a run of the example on the bundled demo model reports each of its checks incomplete on each of its load cases and names each unsupplied input. The example holds no limit value.
- **AC-006** — The statuses the example declares are among rule inputs incomplete, user-rule checked, user-rule failed and human review required, and no status, text or notice of the example states or implies compliance, certification or approval.

## Production and Verification Method — Praxeology

**Method choices that define the example.**

- One file is the example. Any other place the product needs it holds a copy of the same bytes.
- The labels are artificial on their face: a demonstration quantity in a demonstration unit, in a category called invented.
- The notice travels with the content: in the pack's own members and beside it in the repository.

**How each criterion is examined.** The example is content, so the owner's definitions say what must hold and each comparison is against something the example's author did not write.

- **VER-001** — AC-001 and AC-005, executable check: validates the example under a JSON Schema 2020-12 validator that imports no product code and under pack validation, and runs it as shipped on the bundled demo model, observing the outcome and the findings.
- **VER-002** — AC-002, AC-003 and AC-006, judgment by a reviewer who did not write the example, with the protected-content scan: examines every name, value, status and notice of the example and of the notice document.
- **VER-003** — AC-004, comparison with a named reference: RFC 8785 canonicalization and SHA-256 computed by an implementation that shares no code with the product; with an executable check that compares the bundled copies byte for byte.

## Governing Values and Decisions — Axiology

- **AX-001** — *Plainly invented.* Better is an example that fewer readers could mistake for an engineering criterion: more of its names and values artificial on their face. It is never made more realistic by resembling the categories, limits or formulas of a real code.
- **AX-002** — *More of the mechanism on show.* Better is an example that exercises more of the format a newcomer needs to see: provenance, units, a missing-input finding, an outcome. This is never bought with a value that looks like engineering data, and never by shortening or softening the notice.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-016 OBJ-005 | REQ-006 REQ-008 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification applied without product code. The format and validation are DEL-06-01's, which the example's author did not write |
| OUT-001 | SOW-016 OBJ-005 OBJ-011 | REQ-001 REQ-003 REQ-005 | AC-002 | VER-002 | Owner ruling: PRD §17.4 and FR-RULE-007 and `docs/CONTRACT.md` OPS-K-RULE-1 say what a public example must state |
| OUT-001 | SOW-016 OBJ-011 | REQ-001 REQ-002 | AC-003 | VER-002 | Owner ruling: PRD §20.1. The reviewer compares against sources the product did not write |
| OUT-001 | SOW-016 OBJ-005 | REQ-005 | AC-004 | VER-003 | Outside source: RFC 8785 and SHA-256, computed without product code. The equality of the copies is fixed by the criterion |
| OUT-001 | SOW-016 OBJ-005 | REQ-004 | AC-005 | VER-001 | Owner ruling: PRD §5.5 and §17.5, nothing is defaulted and no outcome is reported without the required values. The run is DEL-06-03's |
| OUT-001 | SOW-016 OBJ-011 | REQ-003 | AC-006 | VER-002 | Owner ruling: PRD §21.2 and `docs/claims_registry.md` |
