---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-06
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-010]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-06 Expansion joint component model

## Purpose and Objective Traceability

This deliverable defines what an expansion joint is in the product: a component that takes the place of a span of pipe and connects its two ends through a flexible element. Its stiffness, effective pressure area, movement limits and hardware come from the manufacturer or the user, each with its source. The definition says what those inputs are and what they do in a solve: the joint resists the relative movement of its two ends with the user's stiffness and nothing else, its tie rods are separate elastic rods whose forces are published, and its effective area is what pressure acts on. The software supplies no joint data of its own.

It serves SOW-010 under OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-06), and PRD §12.3 (expansion joints) with FR-SOL-013. In `docs/RELEASE_TARGET_v1.0.md` it is an owning deliverable of exit criterion 1 for the reference system's expansion joint with tie rods.

- **OUT-001** — The expansion joint component definition: its members in the model document and in a component library record, and the behaviour of the joint and its tie rods in a solve and in published results.

## Deliverable Definition — Ontology

**Terms the definition implements.**

- *Expansion joint component.* A component of kind expansion joint at a node, naming the span of pipe it replaces, with an effective area, movement limit, hardware reference and manufacturer reference, and the connector that gives it its mechanics. These members of the model document, the objective connector among them, are this deliverable's definition.
- *Objective connector.* A two-ended element between attachments at the two end nodes of the replaced span. Each attachment has its node, that node's initial axes and a local offset. The connector has its own frame, and a symmetric 6×6 stiffness in the order three translations then three rotations, entered as work coefficients with a translation scale and a rotation scale. It resists only the relative movement of its attachments: a rigid-body movement of both produces no force. The axial, lateral, angular and torsional stiffnesses of PRD §12.3 are terms of this stiffness in the connector frame.
- *Stiffness provenance.* The stiffness carries a source reference, the restraints under which it was measured, and a reference for any change of basis applied to it.
- *Reference state.* Stress-free, or prestressed with an entered reference load, at an installed reference temperature. The stiffness is that of the installed joint and does not vary with temperature.
- *Replaced span.* The pipe the joint stands in for. It contributes no pipe stiffness, no member results, no self-weight and no thermal load of its own.
- *Hardware.* Untied, or explicit elastic tie rods. A tie rod is a linear elastic axial rod between the joint's two ends, acting in tension and compression, with rigid offsets from each end, a stiffness, a reference extension and a source reference. Hinge, gimbal and limit-rod hardware is recorded as a reference and has no mechanics (PRD §7).
- *Pressure model.* Unpressurized, or a constant effective area with its source reference, acted on by the internal pressure.
- *Library record of an expansion joint.* A component library record of type expansion joint, whose field kinds hold linear and rotational stiffness, effective area, movement limit, hardware and manufacturer references.

**Boundary.** This scope states the joint's definition and the criteria it meets. The connector and tie-rod elements are held with the frame kernel's code (DEL-04-01). DEL-04-07 composes the joint into the product solve, applies the pressure thrust that follows from the effective area on the load side, and owns the route on which a joint is admitted and where in a pressure region it may sit. The published connector evidence record is read by the product's result reader; which deliverable's criteria govern that reader is not decided. DEL-02-01 owns the model document and DEL-03-02 the component library format. DEL-07-03 owns the creation form.

**Not decided.** Whether a solve compares the joint's movement with the entered movement limits, and what classes of movement limit and of hardware the product distinguishes. Until it is decided, a movement limit and a hardware reference are recorded, sourced user inputs that change no result.

## Completion and Reliance Basis — Epistemology

A joint result is a statement about a linear connector with the user's stiffness. It says nothing about whether the joint's movements, cycles or hardware are acceptable; that is the manufacturer's data and the user's engineer's judgment.

### Requirements

- **REQ-001** — The joint's axial, lateral, angular and torsional stiffness are data supplied by a user, a manufacturer or a lawful private library (SOW-010; PRD §12.3).
- **REQ-002** — The effective pressure area is supplied data with a unit and provenance (SOW-010; PRD §12.3).
- **REQ-003** — Movement limits are supplied data, and an absent limit is an explicit finding (SOW-010; PRD §12.3).
- **REQ-004** — Hardware data is supplied data. Nothing about a joint's hardware is derived or defaulted (SOW-010).
- **REQ-005** — Every numeric value of a joint is unit-aware and dimensionally checked (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-006** — Manufacturer data carries its source, licence or redistribution status and review disposition (PRD §12.3; `docs/CONTRACT.md` OPS-K-DATA-3, OPS-K-IP-2).
- **REQ-007** — A missing value a solve or a rule check requires is an explicit, named finding. Nothing is defaulted (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-008** — A joint's values reach a solve, a saved model and a report only through the product's validation; no adapter, persistence, interface or report path bypasses the unit, provenance and completeness checks (the decomposition's architecture basis for DEL-00-02 and DEL-00-07).
- **REQ-010** — A joint is solved only as an objective connector that replaces its span. A joint given in any other form is refused by name; it is neither converted nor solved as pipe without saying so.
- **REQ-011** — Elastic tie rods are explicit axial rod elements, and each rod's force is published (PRD §12.3).
- **REQ-012** — The reference system's expansion joint with tie rods solves on the exact route (release target exit criterion 1).
- **REQ-013** — The product ships no manufacturer value and no joint default. Every example is invented or omits its values (SOW-010; `docs/CONTRACT.md` OPS-K-IP-1).

### Acceptance criteria

- **AC-001** — An expansion joint accepts the inputs PRD §12.3 lists (axial, lateral, angular and torsional stiffness, effective pressure area, movement limits, elastic tie rods, hinge, gimbal or limit-rod metadata, and manufacturer data provenance) in the model document, each numeric value with its unit, and a component library record of an expansion joint holds its stiffness, effective area, movement limit, hardware and manufacturer references. A library record that lacks a required value validates only with that value marked missing and an expansion-joint completeness finding. Each manufacturer-sourced value carries its source reference, and in a library record its redistribution status and review disposition as well. A value without its source reference is reported as missing it, a library record that omits any of the three is rejected, and none is assumed.
- **AC-002** — The forces and moments a joint applies to its two ends equal its stiffness acting on the relative movement of its attachments, offsets included, less its reference load, and a rigid-body movement of both ends produces none. Element and system cases reproduce independent references at the analytic tolerance in both solver modes.
- **AC-003** — For a joint with explicit elastic tie rods, each rod's published force equals its stiffness times its extension measured from its reference extension, and the solved system reproduces an independent reference at the analytic tolerance, with the joint pressurized and unpressurized.
- **AC-004** — The replaced span publishes no member result and receives no self-weight or thermal load. A load on that span which a case applies is refused by name; a stored load that no solved case applies is admitted.
- **AC-005** — Each of these is refused by name and nothing is solved in its place: a joint with no connector, or given only by stiffness rates; a stiffness that is not symmetric and positive definite; a hardware kind, rod law or pressure model the definition does not have. A joint the user has marked as not consumed by the solve is, on the pressure-free route, analysed as its pipe with that disclosed on every result, and on the exact route refused.
- **AC-006** — On every path that creates or changes a joint, a value of the wrong dimension is rejected with a finding that names it and an absent required value is reported as missing. A joint saved with the model and reopened carries the same members, values and units, and a report that presents a joint's values presents them with the units and sources that were entered.
- **AC-007** — The expansion joint with tie rods of the invented reference system solves on the exact route in the desktop and is not refused.
- **AC-008** — The joint's sources, schemas, fixtures and tests contain no manufacturer value and no protected content, and every example value is invented or omitted and labelled so.

## Production and Verification Method — Praxeology

**Method choices that define the joint.**

- The joint's relative movement is taken on a symmetric midpoint basis with small rotations, so that neither end is privileged.
- Whether the entered stiffness is positive definite is decided exactly, on the entered coefficients and on the values the solve uses.
- A joint replaces its span. It is never placed in parallel or in series with the pipe it stands in for.
- Tie rods are elements of their own between the joint's ends; they are not folded into the joint's stiffness.
- Pressure thrust is a load, formed from the effective area on the load side, and is not part of the joint's stiffness (DEC-045).

**How each criterion is examined.**

- **VER-001** — Executable check: observes acceptance of a complete joint in the model document and in a library record, the missing-value finding on an incomplete library record, and for a manufacturer-sourced value the source reference it carries, the redistribution status and review disposition it carries in a library record, the finding for an absent source reference and the rejection of a library record that omits any of the three.
- **VER-002** — Comparison with a named reference: connector element and system cases against `validation/references/t4_i12`, generated without product code, in both solver modes.
- **VER-003** — Comparison with a named reference: tied joint cases, pressurized and unpressurized, against `validation/references/t4_u5b`, generated without product code; each published rod force is compared with its reference.
- **VER-004** — Executable check: observes the absence of results and loads on a replaced span, the refusal of an applied load on it, the admission of an unapplied one, each named refusal of AC-005, and the disclosure on an unconsumed joint.
- **VER-005** — Executable check: for each creating and changing path observes the rejection and the missing-value finding, compares a joint before save and after reopen, and observes the units and sources of a joint's values in a generated report.
- **VER-006** — Comparison with a named reference: the reference system (`validation/references/reference_system`) solved in the desktop on the exact route.
- **VER-007** — Executable check, with judgment by an independent reviewer: a protected-content scan over the joint's sources, schemas, fixtures and tests, which is a proxy; a reviewer who did not write the examples reads them against `docs/IP_AND_DATA_BOUNDARY.md` §2 to §4.

## Governing Values and Decisions — Axiology

- **AX-001** — *The manufacturer's data, never ours.* Better is fewer places where a stiffness, area, limit or hardware property could originate in the software; the target is none. No example or convenience justifies an invented default.
- **AX-002** — *Never as pipe by accident.* Better is fewer joints that a solve treats as the pipe they replace without the result saying so. A refusal that asks the user to re-enter a joint is preferred to a silent approximation, however inconvenient.
- **AX-003** — *What the hardware carries is shown.* Better is more of a joint's load path that a result publishes element by element. This is never bought by reporting a force the solve did not compute.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-010 OBJ-004 | REQ-001 REQ-002 REQ-003 REQ-004 REQ-005 REQ-006 | AC-001 | VER-001 | Owner ruling: PRD §12.3 lists the inputs and PRD §20.2 the provenance members; the criterion states the expected acceptance and findings |
| OUT-001 | SOW-010 OBJ-004 | REQ-010 | AC-002 | VER-002 | Outside source: independently derived connector references from a generator that imports no product code. The tolerance is an owner ruling (DEC-026) |
| OUT-001 | SOW-010 OBJ-004 | REQ-011 | AC-003 | VER-003 | Outside source: independently derived tied-joint references from a generator that imports no product code; owner ruling PRD §12.3 for the published rod force |
| OUT-001 | SOW-010 OBJ-004 | REQ-010 | AC-004 | VER-004 | The criterion states the expected absences and refusals, which follow from the definition of a replaced span |
| OUT-001 | SOW-010 OBJ-004 | REQ-007 REQ-010 | AC-005 | VER-004 | Owner ruling: PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state that nothing is assumed silently; the criterion names the expected refusals |
| OUT-001 | SOW-010 OBJ-004 | REQ-008 | AC-006 | VER-005 | Owner ruling: `docs/CONTRACT.md` OPS-K-UNIT-1 and OPS-K-DATA-2 state the expected rejection and finding; the reopened joint and the report are compared with what was entered |
| OUT-001 | SOW-010 OBJ-004 | REQ-012 | AC-007 | VER-006 | Owner ruling for the reference system's content (exit criterion 1); its expected results are the independent references of `validation/references/reference_system` |
| OUT-001 | SOW-010 OBJ-004 | REQ-013 | AC-008 | VER-007 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md`; owner ruling PRD §20.1 |
