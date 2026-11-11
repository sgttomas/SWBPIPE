---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-05
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-009]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items

## Purpose and Objective Traceability

This deliverable defines what a valve, flange, reducer, rigid placeholder or specialty item is in the product. A user places one on a line and gives it its length and end sizes, its weight and centre of gravity, how stiff it is against the pipe it sits in, and for a reducer how far its two centrelines are offset, each with the source it came from. The definition says what those inputs are and what each does in a solve: the component's body is a member of the frame, stiffer or softer than the pipe by the user's factor; its weight acts at its centre of gravity; and a reducer's eccentricity is the geometry of its two ends. The software supplies no dimension, weight or stiffness of its own.

It serves SOW-009 under OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-05), and PRD §12.3 (rigid and semi-rigid components; reducers) with FR-SOL-004, FR-SOL-012 and the rigid link benchmark of §22.2. In `docs/RELEASE_TARGET_v1.0.md` it is an owning deliverable of exit criterion 1 for the reference system's eccentric reducer, valves and flanges.

- **OUT-001** — The rigid-family component definition: its members in the model document and in a component library record, and the behaviour of its body, weight and reducer offset in a solve and in published results.

## Deliverable Definition — Ontology

**Terms the definition implements.**

- *Rigid-family component.* A component of kind valve, flange, reducer, rigid or specialty at a node. It maps one straight member that ends at that node; that member is the component's *body*. The rigid-family members of a component in the model document are this deliverable's definition.
- *Consumption declaration.* Each rigid-family component declares how the solve consumes it. Its scale and its weight are applied only when it declares mechanics geometry only. A component declared not consumed by the solve solves as its pipe, with its pipe weight, and its offset is not read. An absent declaration is not assumed to be either.
- *Stiffness scale.* A dimensionless user factor s, finite and above zero, with a source reference. The body is solved with s times the stiffness of its mapped pipe section, in every stiffness term. Any finite scale above zero is accepted; there is no engineering bound. A scale above 1 makes a rigid body, below 1 a semi-rigid one. The scale changes elastic response only: the body's free thermal growth is unchanged, and its elastic strain under pressure is s times smaller.
- *Stiffness values.* Optional user-entered linear and rotational stiffnesses. They are recorded with their source and checked finite and positive; one uniform scale realizes the component, and they are not applied.
- *Reference-section result.* A stress published on a scaled body. It is the stress of the mapped pipe section under the body's forces. It is not a stress of the valve, flange or reducer body, and the product supplies none. The stress published along an eccentric reducer's chord with no scale applied has the same meaning, though it is an ordinary pipe row.
- *Weight and centre of gravity.* The weight is a force, the component's total. The centre of gravity is a vector in the model's global X, Y and Z axes, measured from the component's node, in model length units. The weight acts along gravity at the node together with the moment of that force about the node. Where a component's weight is applied, its body's own distributed self-weight is not.
- *Eccentric offset.* On a reducer only: a vector in the model's global X, Y and Z axes, from the centreline at the component's node to the centreline at the other end of its body, in model length units. The positions of the nodes govern the mechanics; the declared offset is checked against them and never places a node. A zero vector declares a concentric reducer. An absent offset declares nothing.
- *Orientation.* A rigid or semi-rigid component's orientation is one of its inputs (PRD §12.3). The centre of gravity and the eccentric offset are in global axes and do not depend on it.
- *Reducer local modifiers.* Optional user-supplied local modifiers on a reducer, each with its source (PRD §12.3).
- *Library record of a rigid component.* A component library record of type valve, flange, reducer, rigid or specialty, whose field kinds hold the body length, end sizes and end connection references, weight, centre of gravity, linear and rotational stiffness, and the stiffness-behaviour and source references.

**Boundary.** This scope states the family's definition and the criteria it meets. DEL-04-07 composes it into the product solve, admits it on each route, and owns the pressure step where a reducer changes the bore between pressure regions. DEL-05-01 generates self-weight loads, into which the component's weight enters as this definition states. DEL-05-03 owns stress recovery and the case's stress maximum and headline. DEL-02-01 owns the model document and DEL-03-02 the component library format. DEL-07-03 and DEL-07-02 own the creation and inspector forms. A directional stiffness set is an expansion joint's (DEL-03-06). Stepped-reducer element formulations are not provided (PRD §7).

**Not decided.** How a component's orientation is given: whether it is the direction of the mapped member or a member the component declares. Which local modifiers a reducer carries and what each multiplies.

## Completion and Reliance Basis — Epistemology

A result on a rigid-family component is a statement about an equivalent pipe of the mapped section with the user's scale, weight and geometry. It says nothing about the component's own body.

### Requirements

- **REQ-001** — Valves, flanges, reducers, rigid placeholders and specialty items, among them strainers, instruments and equipment stubs, are represented as the rigid family (SOW-009; PRD §12.3).
- **REQ-002** — Dimensions, weights, centres of gravity and stiffnesses are the user's or lawfully imported private data. The product ships no catalogue or vendor value and no default (`docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-IP-1).
- **REQ-003** — Dimensional, weight, centre-of-gravity and stiffness values are unit-aware and dimensionally checked. Linear stiffness and rotational stiffness are separate dimensions (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-004** — Component data that can enter a library or a reusable example carries its source and redistribution status (`docs/CONTRACT.md` OPS-K-IP-2, OPS-K-DATA-3).
- **REQ-005** — A required value that is absent is an explicit, named finding. Nothing is defaulted (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-006** — A rigid-family component is a member of the 3D centreline frame model. Its definition requires no shell or solid element (`docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-007** — A component's values reach a solve only through the product's validation: no path that creates, changes or imports a component bypasses the unit, provenance and completeness checks (the decomposition's architecture basis for DEL-00-02 and DEL-00-07).
- **REQ-008** — A component's members persist with the model document and are read back unchanged (the decomposition's architecture basis for DEL-00-04).
- **REQ-009** — Public examples of rigid-family components are invented or omit their values, and say so.
- **REQ-010** — The centre of gravity has one convention: global X, Y and Z axes, measured from the component's node, in model length units. No other axes, origin, sign or component-local frame is inferred.
- **REQ-011** — A reusable public example carries its source, redistribution status and review disposition on every value.
- **REQ-012** — The eccentric offset has one convention: global X, Y and Z axes, from the centreline at the component's node to the centreline at the other end of its body, in model length units (PRD §12.3).
- **REQ-013** — Rigid and semi-rigid behaviour is realized by the user's stiffness scale on the body, on the pressure-free route and on the exact route (PRD FR-SOL-004, FR-SOL-012).
- **REQ-014** — The component's weight acts at its centre of gravity (PRD §12.3).
- **REQ-015** — A stress on a scaled body is identified as a reference-section result wherever it is published, and never stands as a stress of the component's body or as the case's governing stress (PRD §21.1). On an eccentric reducer with no scale applied, the chord's rows are ordinary pipe rows, and that they are reference-section stresses is disclosed in the result's information, the result views and the report.
- **REQ-016** — The body length, each end size and the centre-of-gravity vector each carry their own unit.
- **REQ-017** — The reference system's eccentric reducer, valves and flanges solve on the exact route (release target exit criterion 1).
- **REQ-018** — A large scale can cost precision in a body's recovered forces. From a scale of 1e5, the lowest decade at which the rigid-link reference first has a row outside the analytic tolerance, a published result states that the scaled body's forces may carry reduced precision. The threshold is a heuristic measured on one benchmark, not an accuracy bound or an engineering limit; another model may lose digits at a smaller scale, and the warning does not replace the solve's own qualification screens.

### Acceptance criteria

- **AC-001** — A rigid-family component accepts the inputs PRD §12.3 lists for rigid and semi-rigid components and for reducers (length or offsets, weight, centre of gravity, orientation, rigid or semi-rigid behaviour, optional stiffness values and source; for a reducer, upstream and downstream sections, length, weight, concentric or eccentric geometry with its offset vector, and optional user-supplied local modifiers) in the model document and in a component library record, each numeric value with its unit. A library record that lacks a required value validates only with that value marked missing and a rigid-component completeness finding.
- **AC-002** — A body with a large scale below 1e5 transfers forces and moments as a rigid link: the displacements, member end actions and reactions of a rigid link case reproduce an independent reference at the analytic tolerance on both routes and in both solver modes. At a scale of 1e5 or more the published result carries the precision warning of REQ-018, once for the body, and no accuracy is asserted. A body with a scale below 1 reproduces the reference for a member of that scaled stiffness, and in a thermal case the body's free growth is the same at every scale.
- **AC-003** — With a weight and a centre-of-gravity vector that is non-zero along each axis in turn, the reactions of a supported model equal those for the weight as a force along gravity at the component's node with the moment of that force about the node, at the analytic tolerance on both routes. The body's distributed self-weight is not applied as well.
- **AC-004** — An axial force through a reducer whose end centrelines are separated transversely produces, in the upstream run, the moment of that force about the upstream centreline, and the solved case reproduces an independent reference at the analytic tolerance on both routes. On a reducer that is not declared not consumed by the solve, a declared offset that agrees with the node geometry is accepted and changes no result, and a declared offset that disagrees with it, is not transverse to the run or cannot be compared because the run axis is unresolved is refused by name. An offset given on a component that is not a reducer is refused by name. No node is moved.
- **AC-005** — A component that declares mechanics geometry only, or declares nothing, and whose scale or weight is not applied states the first condition that was not met, and its body solves as its pipe. A component declared not consumed by the solve solves as its pipe. A body mapped onto a realized bend or onto a span an expansion joint replaces, and one member mapped by two components, are each refused by name. Each applied scale is stated on the result with its member and value.
- **AC-006** — Every stress published on a scaled body is marked as a reference-section result of a component body; it is excluded from the case's stress maximum and headline; and a user rule bound to it reports its inputs as incomplete for that reason. A reader that shares no code with the producer reads the same marks from the published result.
- **AC-007** — When a rigid-family component is created in the inspector or in the viewport, the body length, each end size and the centre-of-gravity vector each take a unit of their own, which starts as the project's length unit or unselected where there is none. Choosing a unit for one changes no other quantity, and the entered values and units reach the model unchanged.
- **AC-008** — On every path that creates, changes or imports a rigid-family component, a value of the wrong dimension is rejected with a finding that names it and an absent required value is reported as missing. A component saved with the model and reopened carries the same members, values and units.
- **AC-009** — The eccentric reducer, valves and flanges of the invented reference system solve on the exact route in the desktop, and none is refused.
- **AC-010** — The family's sources, schemas, fixtures and tests contain no vendor or catalogue dimension, weight, centre of gravity or stiffness and no protected table. Every example value is invented or omitted, is labelled so, and carries its source, redistribution status and review disposition.
- **AC-011** — An eccentric reducer solved with no scale applied publishes its chord's rows as ordinary pipe rows, and its result information, the result views that show those rows and the report each state that they are stresses of the reference pipe section along the chord and not of the reducer.

## Production and Verification Method — Praxeology

**Method choices that define the family.**

- There is no rigid element and no rigid offset link. The body is the mapped straight member, formed with the user's scale on its elastic and shear moduli.
- A reducer's body runs between its two centreline nodes, so the step's statics come from the actual geometry and no load is added for the offset.
- The weight enters as a nodal force and nodal moments at the component's node, which is statically exact for the body.
- Declared data that the geometry already determines is checked against the geometry and never used in its place.
- Results on a scaled body stay published with their values; they are identified, not withheld.

**How each criterion is examined.**

- **VER-001** — Executable check: observes acceptance of a component of each kind carrying every input AC-001 lists, in the model document and in a library record, and the missing-value finding on an incomplete library record.
- **VER-002** — Comparison with a named reference, with an executable check: rigid link and semi-rigid cases, with a thermal case, against `validation/references/w15_rigid_scaling`, generated without product code, on both routes and in both solver modes; the check observes the precision warning on each case at a scale of 1e5 or more and its absence below.
- **VER-003** — Comparison with a named reference: component weight cases with an offset along each axis against `validation/references/w15_component_weight`, generated without product code.
- **VER-004** — Comparison with a named reference, with an executable check: the eccentric step cases against `validation/references/w15c_eccentric_reducer`, generated without product code; the check observes each accepted and refused offset of AC-004 and that node positions are unchanged.
- **VER-005** — Executable check: observes the stated condition for each unapplied scale and weight on a component that declares mechanics geometry only or nothing, the pipe solution of a component declared not consumed, each named mapping refusal, and the statement of each applied scale.
- **VER-006** — Executable check, with a comparison: observes the marks, the exclusion from maximum and headline and the rule outcome on a model whose scaled body would otherwise govern; a verifier that imports no product code reads the same published result and reaches the same marks.
- **VER-007** — Executable check: drives the creation forms in the inspector and the viewport and observes each quantity's unit, its independence from the others, and the values that reach the model.
- **VER-008** — Executable check: for each creating, changing and importing path observes the rejection and the missing-value finding, and compares a component before save and after reopen.
- **VER-009** — Comparison with a named reference: the reference system (`validation/references/reference_system`) solved in the desktop on the exact route.
- **VER-010** — Executable check, with judgment by an independent reviewer: a protected-content scan over the family's sources, schemas, fixtures and tests, which is a proxy; a reviewer who did not write the examples reads them against `docs/IP_AND_DATA_BOUNDARY.md` §2 to §4.
- **VER-011** — Executable check: solves an unscaled eccentric reducer and observes ordinary pipe rows on its chord and the reference-section statement in the result information, in the result views and in a generated report.

## Governing Values and Decisions — Axiology

- **AX-001** — *The user's data, never ours.* Better is fewer places where a dimension, weight, centre of gravity or stiffness could originate in the software; the target is none. No convenience or example justifies a catalogue value.
- **AX-002** — *Honest about the body.* Better is fewer results a reader could take for a stress of the component itself. This is never bought by withholding the forces or values the frame did compute.
- **AX-003** — *The geometry governs.* Better is fewer models in which declared component data and the node geometry disagree and solve anyway. This is never bought by moving a node or by using the declaration in place of the geometry.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-009 OBJ-004 | REQ-001 REQ-003 REQ-004 | AC-001 | VER-001 | PRD §12.3 lists the inputs; the criterion states the expected acceptance |
| OUT-001 | SOW-009 OBJ-004 | REQ-006 REQ-013 REQ-018 | AC-002 | VER-002 | Outside source: closed-form rigid link and scaled-member references from a generator that imports no product code (PRD §22.2). The tolerance is the governed analytic tolerance. The 1e5 threshold is measured on this benchmark only; the evidence does not establish where another model loses precision |
| OUT-001 | SOW-009 OBJ-004 | REQ-010 REQ-014 | AC-003 | VER-003 | Outside source: statics of a force and its moment about the node, from a generator that imports no product code. The convention is REQ-010's definition, which the reference applies and does not establish |
| OUT-001 | SOW-009 OBJ-004 | REQ-012 | AC-004 | VER-004 | Outside source: closed-form statics of the eccentric step, from a generator that imports no product code. The convention is REQ-012's definition, which the reference applies and does not establish; the refusals are those the criterion names |
| OUT-001 | SOW-009 OBJ-004 | REQ-005 REQ-013 | AC-005 | VER-005 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state that nothing is dropped silently; the criterion names the expected statements and refusals |
| OUT-001 | SOW-009 OBJ-004 | REQ-015 | AC-006 | VER-006 | The criterion states the expected marks, which follow from the body's definition as an equivalent pipe. The second reader shares no code with the producer; agreement between them shows consistent publication, not correct stress |
| OUT-001 | SOW-009 OBJ-004 | REQ-016 | AC-007 | VER-007 | The criterion states the expected units and values: what the user entered |
| OUT-001 | SOW-009 OBJ-004 | REQ-007 REQ-008 | AC-008 | VER-008 | `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2 state the expected rejection and finding; the reopened component is compared with the saved one |
| OUT-001 | SOW-009 OBJ-004 | REQ-017 | AC-009 | VER-009 | Exit criterion 1 fixes the reference system's content; its expected results are the independent references of `validation/references/reference_system` |
| OUT-001 | SOW-009 OBJ-004 | REQ-002 REQ-009 REQ-011 | AC-010 | VER-010 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md`; PRD §20.1 |
| OUT-001 | SOW-009 OBJ-004 | REQ-015 | AC-011 | VER-011 | The criterion states the expected rows and statements, which follow from the definition of a reference-section result; PRD §21.1 |
