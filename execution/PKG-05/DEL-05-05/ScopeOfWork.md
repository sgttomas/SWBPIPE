---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-05
package_id: PKG-05
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-052, SOW-013]
package_objective_refs: [OBJ-003, OBJ-012]
---

# Scope of Work — DEL-05-05 Concentrated and distributed user load application

## Purpose and Objective Traceability

This deliverable lets an engineer load the model with something that is not one of the piping load categories: a force or a moment at a node, or a load spread along a pipe. Equipment reactions, attached weights and any other load the engineer has worked out arrive this way. The deliverable defines these user loads, applies each one to the frame model at the place, in the direction and with the magnitude the engineer entered, and keeps each applied load identifiable to what reads the solve afterwards, so that a report or a later check can say which load produced what. A user load is a plain mechanical load. It carries no factor, no combination and no code meaning.

It serves SOW-052 and, as context, SOW-013, under OBJ-003 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-05, row DEL-05-05: general user loads kept separate from code-specific combinations), and PRD §12.2 (distributed and concentrated load handling) and §12.5 with FR-SOL-006. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 1 through DEL-04-07, whose one route applies every concentrated force, concentrated moment and distributed load on both routes or refuses it by name.

- **OUT-001** — User load application: the three user-load kinds, their application to nodes and members of the frame model, their findings, and the record of each applied load for the consumers of the solve.

## Deliverable Definition — Ontology

**Terms the deliverable implements.**

- *User load.* A load the user authors on a load case in one of three kinds:
  - *concentrated force*: at a node, along global X, Y or Z, a force;
  - *concentrated moment*: at a node, about global X, Y or Z, a moment;
  - *distributed load*: on a pipe, along global X, Y or Z, a force per length of the member, uniform over the whole span or over the part between two fractions of its length.
- *Global direction.* A direction is a global axis whatever the member's orientation. A distributed load is per length of the member, not per length of its projection.
- *Distributed load on a realized arc.* A force per unit arc length over the whole arc.
- *Point force on a member.* A force at a station along a member, the station given as a fraction of its length.
- *Applied-load record.* What each applied user load carries for the consumers of the solve (stress recovery, reports, exports, the desktop and the headless runner): its identity, kind, target, dimension and provenance.
- *Finding.* A user load that cannot be applied, reported with a code, the load's identity, a message and a remediation.

**Boundary.** This deliverable decides which loads a user can author and what each means. The following belong elsewhere:

- the load case, its categories, the preparation of a nodal or member load into solver contributions and the order of those contributions: DEL-05-01. A user load is a primitive load of its case, and this scope adds no second preparation;
- the equivalent nodal loads of a member load and the recovery of member actions along the span, for Euler–Bernoulli and Timoshenko members: DEL-04-02. The frame's degrees of freedom and the curved-bend element: DEL-04-01;
- composing loads into the product solve on both routes: DEL-04-07. Combinations of cases: DEL-05-02. Stresses: DEL-05-03;
- the load-case panel in which a user enters a load: DEL-07-03. The model-document members: DEL-02-01. Units: DEL-02-02.

**Not decided.**

- Whether a point force on a member is a product capability. A concentrated load is entered at a node; REQ-008 stands as written until the owner decides.
- Whether the applied-load record is part of the published result, and whether it also carries the load's direction and its magnitude with unit.
- Whether the user-load crate (`core/loads/user_loads`), which applies user loads by a route of its own and is linked only by the mechanics benchmark suite, stays part of this deliverable, becomes the product's route or is retired. Until the owner decides, the product solve applies a user load only through DEL-05-01's preparation.

## Completion and Reliance Basis — Epistemology

An applied user load is a statement about the user's input. Its correctness is examined against closed-form mechanics, on a member in any orientation.

### Requirements

- **REQ-001** — A user can author concentrated forces, concentrated moments and distributed loads as explicit inputs of a load case, and each is applied in the solve or refused by name (SOW-052; PRD §12.5).
- **REQ-002** — User loads stay separate from code-specific combinations: a user load has no factor, no combination and no default magnitude of its own, and none is supplied for it (the decomposition's row; `docs/CONTRACT.md` OPS-K-DATA-1).
- **REQ-003** — Load input, application and the applied-load record are unit-aware and dimensionally checked: a force for a concentrated force, a moment for a concentrated moment, a force per length for a distributed load (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-004** — User loads act on the 3D centerline frame model: a concentrated load on a node's degrees of freedom and a distributed load on a member (PRD §12.1; `docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-005** — Application is deterministic: the same case gives the same applied loads and findings, in the same order, bit for bit, so every check of it is deterministic (PRD §12.2; `docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-006** — Each applied user load carries an applied-load record that result and report consumers can read, and each finding carries code, class, severity, source, affected object, message, remediation and provenance. Neither states or implies approval or compliance (decomposition AB-00-03, AB-00-06; PRD §5.7, §21.2).
- **REQ-007** — On a straight member, a distributed load over the whole span or a valid part of it enters the solve as the member's work-consistent equivalent nodal loads in global axes, never as a lumped approximation.
- **REQ-008** — On a straight member, a point force at a station enters the solve as its equivalent nodal loads. A moment at a station and a station fraction outside the member are refused as findings.
- **REQ-009** — A load along global X, Y or Z is applied as that global force vector on a member of any orientation.
- **REQ-010** — A member's user loads add to the thermal and pressure axial effects that DEL-05-01 prepared for the member. This deliverable forms no axial effect of its own.

### Acceptance criteria

- **AC-001** — A concentrated force and a concentrated moment at the free end of a cantilever, and a distributed load over a simply supported span, give displacements, rotations and reactions that agree with closed-form beam mechanics at the analytic tolerance (1e-9 relative, with a stated absolute floor near zero), on both routes.
- **AC-002** — A distributed load over part of a span gives the fixed-end actions of that load on a fixed-fixed beam as its equivalent nodal loads, and the solved model agrees with the closed form. A member along global Y or Z, or inclined, gives the same global result as the same member along X with the load rotated with it. A distributed load over a whole realized arc is in equilibrium with the reactions it produces.
- **AC-003** — A point force at a station on a straight member gives the fixed-end actions of that force, and the solved model agrees with the closed form. A moment at a station, or a station fraction outside 0 to 1, is a finding.
- **AC-004** — A user load with a missing target, magnitude or unit, a dimension that does not fit its kind, a node or pipe the model does not have, a non-finite value, or an inverted or out-of-range span is reported as a finding with its code and the load's identity, and its case gives the solve no partial set of loads. A load the route cannot apply is refused by name, never dropped.
- **AC-005** — Each applied user load has one applied-load record with all of its fields, which a result or report consumer can read, and repeated runs give bit-identical records and findings in the same order.
- **AC-006** — The deliverable's sources, tests and fixtures hold no factor, combination, allowable or default magnitude and only invented values, and no message or record states or implies approval or compliance.
- **AC-007** — A straight member with a distributed user load and a temperature change gives the superposition of the two closed-form results, with the thermal effect counted once.

## Production and Verification Method — Praxeology

**Method choices that define the application.**

- A user load is a primitive load of its case under its own kind. In the product solve it reaches the solver through DEL-05-01's preparation and DEL-04-02's equivalent loads, and by no second path.
- Member loads are applied work-consistently for the member's beam theory. Lumping is not used.
- A direction is resolved to a global vector before it meets a member's local axes.

**How each criterion is examined.**

- **VER-001** — Comparison with named references (closed-form beam mechanics and the hand calculations `validation/hand_calcs/mechanics/tp_phys_004_load_to_resultant.md`, `tp_phys_005_oriented_load_to_resultant.md` and `tp_phys_006_partial_span_load_to_resultant.md`): each user-load kind solved on both routes, in the orientations and spans of AC-001 to AC-003, with statics checked for the arc.
- **VER-002** — Executable check: observes each finding of AC-004 with its code and the load's identity, the absence of loads for a blocked case, the applied-load records of AC-005 and bit-identical output on repeated runs.
- **VER-003** — Judgment by a named role (an independent reviewer who did not write the code): examines the deliverable's sources, tests, fixtures and messages against PRD §20.1 and §21.2, with the protected-content scan's result for the same files.
- **VER-004** — Comparison with a named reference (`validation/hand_calcs/mechanics/tp_phys_009_combined_load_axial_effects.md` and the closed forms it uses): a member carrying a distributed user load and a temperature change.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing assumed.* Better is fewer values the deliverable could supply by itself; the target is none. No magnitude, direction, span or unit is supplied to make a load applicable.
- **AX-002** — *More of what users author is applied.* Better is fewer named refusals of user loads on either route. A refusal is never removed by applying a load approximately or by moving it to a nearby node.
- **AX-003** — *Closer to the closed form.* Better is tighter agreement of an applied member load with its closed form; a tolerance is only tightened. Speed never justifies a lumped member load.
- **AX-004** — *A general load stays general.* Where a user load could be given a code meaning, the reading that claims less is the better one. Convenience never justifies a built-in factor or combination.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-052 OBJ-003 | REQ-001 REQ-004 | AC-001 | VER-001 | Outside source: closed-form beam mechanics and `validation/hand_calcs/mechanics/tp_phys_004_load_to_resultant.md`, which import no product code |
| OUT-001 | SOW-052 OBJ-003 | REQ-007 REQ-009 | AC-002 | VER-001 | Outside source: fixed-end actions of a partial uniform load, statics of the arc, and `tp_phys_005_oriented_load_to_resultant.md` and `tp_phys_006_partial_span_load_to_resultant.md`. The rotated twin compares the product with its own output and only guards against change |
| OUT-001 | SOW-052 OBJ-003 | REQ-008 | AC-003 | VER-001 | Outside source: fixed-end actions of a point force on a fixed-fixed beam |
| OUT-001 | SOW-052 OBJ-012 | REQ-003 REQ-006 | AC-004 | VER-002 | The criterion states the expected finding for each defect; PRD §5.5 and §5.6 say that none is defaulted |
| OUT-001 | SOW-052 OBJ-012 | REQ-005 REQ-006 | AC-005 | VER-002 | The criterion states the expected fields; repeated runs compare the product with its own earlier output and only guard against change |
| OUT-001 | SOW-052 SOW-013 OBJ-003 | REQ-002 | AC-006 | VER-003 | Owner ruling: PRD §20.1 and §21.2 list what must not appear; the reviewer compares the files with them |
| OUT-001 | SOW-013 OBJ-003 | REQ-010 | AC-007 | VER-004 | Outside source: superposition of the closed forms in `tp_phys_009_combined_load_axial_effects.md` |
