---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-04
package_id: PKG-13
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-066]
package_objective_refs: [OBJ-014]
---

# Scope of Work — DEL-13-04 Physical-to-analytical transformation contract

## Purpose and Objective Traceability

The engineer edits a physical model; the solver solves an analytical one. This deliverable is the contract for getting from the first to the second. The same physical model always gives the same analytical model. Everything in the physical model either appears in the analytical model with a link back to where it came from, or is reported as omitted, unsupported or incomplete by a warning that names it. Nothing is filled in on the way. With the contract, a result can be traced to the design data it came from, and a reader can see what the idealization left out. Without it, the analytical model is an unexplained by-product of whatever code feeds the solver.

It serves SOW-066 under OBJ-014 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-13, row DEL-13-04), and PRD §8.3, FR-MOD-007, §5.2 and §5.5, with §12.1 for the frame target. PKG-13 is outside v1.0, and no exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. In the product, FR-MOD-007 is carried by the product solve's derivation of its built model from the model document, which is DEL-04-07's.

- **OUT-001** — The transform contract: the transform, the transform result with its versioned schema, the transform reference format, and the internal solver-boundary adapter.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Physical model.* The editable design model and the source of truth: geometry, components, materials, sections, supports, loads and cases, with its unresolved assumptions and its references to design knowledge and constraints. It is richer than the analytical model.
- *Analytical model.* The idealized model derived from a physical model for solving (PRD §8.3): centerline nodes with six degrees of freedom, two-node elements with their material and section properties, SIF and flexibility data, restraints, loads, load cases and combinations, and the other quantities the solve requires. It names the physical model it was derived from. It is a derived view and is never edited.
- *Model role.* What a model is: the physical source of truth, or an analytical solver model.
- *Transform reference format.* The format in which the transform reads a physical model and writes an analytical one, `schemas/physical_transform_model.schema.yaml`: a project of models with roles, typed load records (nodal force, nodal moment, element point force, element uniform distributed force) and traceability links. It is not the model document.
- *Transform.* The derivation of an analytical model from one physical model under stated settings. The contract version is part of the result's identity.
- *Transformation warning.* A finding raised when physical data cannot be represented analytically: a record type the analytical model has no element for, a required member that is missing, a reference that does not resolve, a quantity without its unit, or a dimension outside the unit vocabulary. It names the physical record it concerns.
- *Omission.* A physical record that is not carried into the analytical model. An omission always has a transformation warning.
- *Traceability link.* A directed link from a physical record to the analytical record derived from it, or to the warning that explains why none was derived. A link can also pair the path of one carried value in the source with its path in the target; such a pair is given only for a quantity carried with complete unit metadata, and never for an omitted record.
- *Transform result.* The analytical model, the findings, the traceability links, and a statement of whether any finding is blocking. Its schema is this deliverable's.
- *Solver-boundary adapter.* The internal step that turns an analytical model into the records frame assembly takes: node degrees of freedom, straight-pipe connectivity and orientation, property bindings, support targets and load applications. Each load application it emits carries an identity, a hash of its payload, and the chain of links back to its source record.

**Undecided.**

- Which document the contract's physical model is. The model document (DEL-02-01) is the canonical model, and the product solve (DEL-04-07) derives its analytical model from it. The transform here reads its own reference format, and nothing derives that format from the model document. Whether this contract comes to govern the product solve's derivation, or stays a separate transform over its own format, is not decided.
- The final set of transformation-loss classes and their severities (decomposition open issue OI-012).
- The wire form of the transform result: its envelope, fields and version.

**Boundary.** The contract derives and accounts; it solves nothing. Frame assembly and the linear solve are DEL-04-01's; what a support means is DEL-04-03's and DEL-04-04's; what a load means is DEL-05-01's. The product solve and its derivation of a built model are DEL-04-07's. The model document is DEL-02-01's and the unit vocabulary is DEL-02-02's. Design knowledge and constraints are carried as references only: their formats are DEL-13-01's and DEL-13-02's, and the transform does not evaluate a constraint (DEL-13-03). Handoff, target mapping and export read the result and are PKG-15's and PKG-17's. Shell and solid analysis is a separate handoff path and never a target of this transform.

## Completion and Reliance Basis — Epistemology

A transform result establishes that the analytical model follows from the physical model by the contract's rules. It does not establish that a centerline and frame idealization is suitable for the physical system: that suitability has no independent validation here, and agreement of carried section properties with a reference establishes those values and nothing more (also cited as DEL-13-04:3).

### CLM-017 — Requirements

Requirement numbers 001 to 012 are those of the earlier `DEL-13-04-REQ-nnn` series.

- **REQ-001** — The transform derives a solver-ready analytical model from the physical model deterministically (SOW-066; PRD FR-MOD-007).
- **REQ-002** — A transformation warning is recorded whenever physical design data cannot be represented analytically (SOW-066).
- **REQ-003** — The physical model stays the editable source of truth, and the analytical model is a derived view for the solver (PRD §5.2, §8.2, §8.3).
- **REQ-004** — Traceability runs from each physical source record to the analytical record, omission, assumption or warning that is its outcome (OBJ-014).
- **REQ-005** — A unit-bearing value that crosses the transform carries its unit and dimension, unless its field is explicitly dimensionless (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-006** — A missing value the solve requires is an explicit finding and is never defaulted by the transform (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-007** — The analytical model is a 3D centerline and frame model (PRD §12.1; `docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-008** — The contract, its reference format and its fixtures contain no protected standards content, code-specific default, protected table, copied formula, proprietary catalogue value or owner standard (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1).
- **REQ-009** — The contract and its findings claim no certification, sealing, approval, authentication, engineering acceptance or code compliance (PRD §21.2).
- **REQ-010** — Every solver-facing behaviour of the contract is examined by a deterministic check (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-012** — The solver-boundary adapter accepts only a model of the analytical role. It is internal: it is not a public API, a desktop or runtime path, or an external-prover path, and its output is no statement of solver acceptance.
- **REQ-013** — This deliverable owns the transform result as a production contract with one authoritative versioned schema, which preserves the physical source identity, the analytical derivation, units, findings and traceability. Where an application service returns the result, it is carried in a result envelope (also cited as DEL-13-04-SCA011-REQ-001, DEL-13-04:1 and DEL-13-04:2).

### Acceptance criteria

- **AC-001** — The same physical model and settings give the same transform result on every run: the same analytical model, findings and traceability links, in the same order.
- **AC-002** — The physical model is unchanged by the transform. The analytical model declares the analytical role and names the physical model it was derived from, and no output of the transform has the physical role.
- **AC-003** — Every record of the physical model has an outcome: either an analytical record with a link from its source record, or an omission with a transformation warning that names it and a link from the record to that warning. An unsupported component is omitted with its linked warning and no value-level link (also cited as DEL-13-04:4).
- **AC-004** — Each kind of unrepresentable data yields its named finding and no analytical record for the item: an unsupported element, component or support type; a missing required member; an unresolved node, material, section, support-target or load-target reference. The result is marked blocking wherever the missing item is needed to solve.
- **AC-005** — A quantity without its unit, or with a dimension outside the unit vocabulary, yields a named finding. No quantity is carried without its unit and dimension, and the set of dimension identifiers the reference format admits equals the unit schema's set.
- **AC-006** — For a physical model that lacks a value the solve requires, the transform gives a finding, and the analytical model holds no value the physical model did not state.
- **AC-007** — The analytical model is a centerline frame model: every structural element is a two-node line element between centerline nodes. It holds no shell or solid element.
- **AC-008** — The adapter refuses a model that is not of the analytical role. A missing property, a non-finite quantity, a wrong dimension, an unresolved target, an unordered load span, a load record outside the typed kinds, and an orientation reference that is zero or parallel to the element's axis each yield a named finding and no emitted record for the item; nothing is inferred. The same analytical model gives the same emitted records; an element's axis is the unit vector from its start node to its end node; and each load application's payload hash is the canonical hash of its payload. No application service, desktop command or exported interface reaches the adapter.
- **AC-009** — No finding, record or schema text of the contract states or implies certification, sealing, approval, authentication, acceptance or code compliance.
- **AC-010** — The contract's source, reference format and fixtures contain no protected or private content, and every fixture is invented and carries its provenance.
- **AC-011** — The transform reference format is valid JSON Schema 2020-12 in strict JSON. Every committed document in that format validates against it, and so does every analytical model the transform derives.
- **AC-012** — The transform result, serialized, validates against this deliverable's one versioned schema and carries the identity of the physical model, the analytical model, the findings and the traceability links. Where an application service returns it, it is carried in a result envelope with its outcome kind.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- The transform is a function of the physical model and its settings and of nothing else. It reads the physical model and never writes to it.
- Omit and report, never approximate. A record the analytical model cannot represent is left out with a warning; it is not forced into a solver element.
- Records, findings and links are emitted in a stable order.
- A payload hash is the canonical hash of the persistence rules: SHA-256 over the RFC 8785 form of the payload.
- The adapter is an internal step of the contract and is reached only from within it.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002, AC-003, AC-004, AC-005, AC-006 and AC-007, executable check: observes, for constructed physical models, the transform result across repeated runs, the physical model before and after, the outcome of every source record, each named finding, the carried quantities, and the kinds of record in the analytical model; with the reference format's dimension identifiers compared with those of `schemas/units.schema.yaml`, read from that file.
- **VER-002** — AC-008, executable check with comparison against named references: observes the adapter's refusals, findings and emitted records for constructed analytical models, and the code paths that reach it; element axes are compared with vector geometry worked without product code, and payload hashes with SHA-256 over the RFC 8785 form computed by an implementation that shares no code with the product.
- **VER-003** — AC-009 and AC-010, judgment by a reviewer who did not write the contract, with the protected-content scan: examines the finding texts, the reference format and each fixture with its provenance.
- **VER-004** — AC-011 and AC-012, executable check: observes the reference format, the committed documents, the derived analytical models and the serialized transform result under a JSON Schema 2020-12 validator that imports no product code, and, where an application service returns the result, the envelope and outcome kind it is carried in.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing lost unseen.* Better is fewer physical records that end with neither a derived record nor a warning; the target is none. A record is never approximated without a warning to make the analytical model look more complete.
- **AX-002** — *Same in, same out.* Better is a result that depends on nothing but the physical model, the settings and the contract version. Determinism is never given up for speed or for a more convenient ordering.
- **AX-003** — *Say what is missing.* Better is fewer values in an analytical model that the physical model did not state; the target is none. A value is never supplied so that a model becomes solvable.
- **AX-004** — *The physical model leads.* Better is fewer ways an analytical value can change other than by deriving it again from the physical model; the target is none. The analytical model is never edited, written back or treated as the design.
- **AX-005** — *A frame model, and said to be one.* Better is more of what the frame idealization cannot hold carried by named warnings and links, not dropped. This is never bought by adding a non-frame element to the target or by claiming fidelity beyond the idealization.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-066 OBJ-014 | REQ-001 REQ-010 | AC-001 | VER-001 | The criterion fixes the expected relation: equal results on every run. It compares the product with itself and guards reproducibility only |
| OUT-001 | SOW-066 OBJ-014 | REQ-003 | AC-002 | VER-001 | The criterion fixes the expected result: the physical model read back equals the one supplied, and the role and source reference are stated by PRD §5.2 and §8.3 |
| OUT-001 | SOW-066 OBJ-014 | REQ-002 REQ-004 | AC-003 | VER-001 | The physical models are constructed, so the criterion states the expected outcome of each record; the comparison is against the constructed source, which the product did not write |
| OUT-001 | SOW-066 OBJ-014 | REQ-002 REQ-006 | AC-004 | VER-001 | The inputs are constructed, so the criterion states the expected finding (SOW-066; PRD §5.5). Which loss classes are final awaits OI-012 |
| OUT-001 | SOW-066 OBJ-014 | REQ-005 | AC-005 | VER-001 | The unit schema's dimension vocabulary (DEL-02-02), read from its file; the quantities are constructed |
| OUT-001 | SOW-066 OBJ-014 | REQ-006 | AC-006 | VER-001 | The criterion states the expected result (PRD §5.5: nothing is defaulted); the comparison is against the constructed physical model |
| OUT-001 | SOW-066 OBJ-014 | REQ-007 | AC-007 | VER-001 | PRD §12.1 and `docs/CONTRACT.md` OPS-K-MECH-1 state the frame target. It establishes the model's form, not its suitability |
| OUT-001 | SOW-066 OBJ-014 | REQ-010 REQ-012 | AC-008 | VER-002 | Outside source: vector geometry for element axes and RFC 8785 with SHA-256 for payload hashes, each worked without product code; the refusals are fixed by the criterion on constructed inputs |
| OUT-001 | SOW-066 OBJ-014 | REQ-009 | AC-009 | VER-003 | PRD §21.2 lists the prohibited claims; the reviewer compares against it and `docs/claims_registry.md` |
| OUT-001 | SOW-066 OBJ-014 | REQ-008 | AC-010 | VER-003 | PRD §20.1 and `docs/IP_AND_DATA_BOUNDARY.md` state what a public artifact may contain; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-066 OBJ-014 | REQ-003 REQ-013 | AC-011 | VER-004 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code |
| OUT-001 | SOW-066 OBJ-014 | REQ-013 | AC-012 | VER-004 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code, against the schema this deliverable publishes. The schema's fields await the undecided wire form |
