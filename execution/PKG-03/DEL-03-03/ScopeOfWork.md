---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-03
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-007]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-03 Bend and elbow component model fields

## Purpose and Objective Traceability

This deliverable defines what a bend or elbow is in the product. A user places one where a line changes direction and gives it its geometry, its flexibility factors and its stress intensification factors, each with the source it came from. The definition says what those inputs are and what each does: the geometry and the flexibility factors make the bend a curved, more flexible member of the solved frame, and the intensification factors multiply the bending stress recovered at it. The software supplies no factor of its own.

It serves SOW-007 under OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-03), and PRD §12.3 (bend and elbow objects) with FR-SOL-010 and the user-entered SIF and flexibility-factor behaviours of §22.3. In `docs/RELEASE_TARGET_v1.0.md` it is an owning deliverable of exit criterion 1 for the reference system's realized bends: they solve on the exact pressure route in the desktop.

- **OUT-001** — The bend and elbow component definition: its members in the model document and in a component library record, and the behaviour of a bend in a solve and in stress recovery.

## Deliverable Definition — Ontology

**Terms the definition implements.**

- *Bend component.* A component of kind bend or elbow at a node. It names the pipe that spans the bend, and carries a bend radius, a bend angle, a plane orientation and a source reference for that geometry; flexibility factors and stress intensification factors with one source reference for those modifiers; and a declaration of how the solve consumes it. These members of the model document are this deliverable's definition.
- *Realized bend.* A bend declared as a curved member. Its pipe is solved as a circular arc between the pipe's two end nodes. The chord is the line between the nodes; the radius is the user's; the plane is spanned by the chord and the pipe's reference direction, and the arc bows toward that reference side. The included angle follows from chord and radius. An entered angle must agree with it.
- *Flexibility factors.* Dimensionless user factors, finite and above zero: an in-plane and an out-of-plane factor. On a realized bend the in-plane factor multiplies in-plane bending flexibility and the out-of-plane factor multiplies out-of-plane bending flexibility; torsional and axial flexibility are unchanged; a factor of 1 is the plain curved beam.
- *Stress intensification factors.* Dimensionless user factors, finite and above zero. On a realized bend they are directional and enter the intensified bending DEL-05-03 defines: the in-plane factor on the bending stress of the in-plane moment, which is the moment about the normal to the bend plane, and the out-of-plane factor on that of the out-of-plane moment, about the transverse axis in the plane. The two are entered both or neither. A torsional factor, where one applies, multiplies the torsional stress and does not enter intensified bending.
- *Geometry-only bend.* A bend declared as geometry only. Its pipe is solved as the straight chord, its flexibility factors are not applied, and its single intensification factor multiplies the resultant bending stress at the adjacent pipe end.
- *Library record of a bend.* A component library record of type bend or elbow, whose field kinds hold the centreline radius, included angle, tangent length, plane and end orientations, the intensification factors, the flexibility factors and the geometry source reference.

**Boundary.** This scope states the bend's definition and the criteria a bend meets. The arc element that realizes it is this definition and is examined under these criteria; its code is held with the frame kernel's (DEL-04-01), which assembles it into the global stiffness and solves it. DEL-04-07 composes bends into the product solve, admits them on each route, and owns pressure on an arc and the refusal of a geometry-only bend on the exact route. DEL-05-03 owns the base stresses the factors multiply and the definition of intensified bending on a realized arc. DEL-02-01 owns the model document and DEL-03-02 the component library format. DEL-07-03 owns the bend authoring forms, and the viewport's drawing of a bend is DEL-07-01's. Mitred bends are not modelled (PRD §7).

## Completion and Reliance Basis — Epistemology

A solved bend is a statement about a circular curved beam with the user's factors. It says nothing about whether those factors suit the fitting; that is the user's engineer's judgment.

### Requirements

- **REQ-001** — A bend or elbow is identified as its own component family, distinct from every other (SOW-007).
- **REQ-002** — A bend accepts the user's bend radius, bend angle and bend plane orientation (SOW-007; PRD §12.3, FR-SOL-010).
- **REQ-003** — A bend accepts the user's in-plane, out-of-plane and, where applicable, torsional stress intensification factors (SOW-007; PRD §12.3).
- **REQ-004** — A bend accepts the user's flexibility factors (SOW-007; PRD §12.3).
- **REQ-005** — The geometry and each local modifier carry the user's source and provenance (PRD §12.3; `docs/CONTRACT.md` OPS-K-DATA-3).
- **REQ-006** — The product ships no SIF or flexibility table, equation, catalogue value, standard example or default, and computes no factor. Every factor is the user's (SOW-007; PRD §12.3; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1).
- **REQ-007** — A missing or inconsistent value a solve or a rule check requires is an explicit, named finding. Nothing is defaulted and no entered geometry is adjusted to fit (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-008** — Geometry values are unit-aware and dimensionally checked; factors are dimensionless (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-009** — Example data for bends is invented, non-code and labelled as such (PRD FR-RULE-007).
- **REQ-010** — A bend is a member of the 3D centreline frame model. Its definition assumes no shell or solid element (`docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-011** — A bend's values reach a solve only through the product's validation: no path that creates, changes or imports a bend bypasses the unit, provenance and completeness checks (the decomposition's architecture basis for DEL-00-02 and DEL-00-07).
- **REQ-012** — A realized bend's flexibility factors enter the global stiffness through an arc-consistent curved element. Scaling a straight chord by the factor is not the realization, and a factor that is in the stiffness is not applied again to stress.
- **REQ-013** — A realized bend is a solved member on the pressure-free route and on the exact route (release target exit criterion 1).

### Acceptance criteria

- **AC-001** — A bend or elbow accepts every input of REQ-002 to REQ-005, in the model document and in a component library record. Each geometry value carries a unit of its dimension and each factor is dimensionless; on every path that creates, changes or imports a bend, a value of the wrong dimension is rejected, and the rejection names it. A library record that lacks a required bend value validates only with that value marked missing and a bend completeness finding.
- **AC-002** — A realized bend with flexibility factors of 1 has the end flexibility of a circular curved beam in closed form, for loading in and out of its plane, at the analytic tolerance. With an in-plane factor k_in and an out-of-plane factor k_out, its end flexibility equals the closed-form curved-beam end flexibility in which the in-plane bending strain-energy term is multiplied by k_in and the out-of-plane bending strain-energy term by k_out, and the torsional and axial terms are unchanged. Equal factors on the two planes are the case k_in = k_out.
- **AC-003** — In a thermal expansion loop whose bends carry known flexibility, the solved displacements and anchor reactions reproduce an independent reference. As the factor increases, the anchor reaction decreases monotonically, and the sparse and dense solver modes agree.
- **AC-004** — On a realized bend, the intensified bending published at each station of the arc is formed as DEL-05-03 defines it: the in-plane factor on the bending stress of the in-plane moment, and the out-of-plane factor on that of the out-of-plane moment. The flexibility factor does not enter it. On a geometry-only bend, the intensified bending at the adjacent pipe end equals the user's single factor times the resultant base bending stress.
- **AC-005** — Each of these blocks the solve with a named finding on the component, and no value is substituted: a realized bend without a radius or a flexibility factor; a radius too small to span the chord; an entered angle that disagrees with the angle the chord and radius imply; a bend plane that cannot be formed; one directional factor without the other; a factor that is not finite, dimensionless and above zero; directional factors on a bend that is not realized. A geometry-only bend on the pressure-free route states that it is solved as its chord and that its flexibility factor is not applied.
- **AC-006** — The realized bends of the invented reference system solve on the exact route in the desktop, and none is refused.
- **AC-007** — The bend's sources, schemas, fixtures and tests contain nothing REQ-006 excludes, and every example factor is invented and labelled so.

## Production and Verification Method — Praxeology

**Method choices that define the bend.**

- A realized bend is a two-node circular-arc element formed from its end nodes, the user's radius and the pipe's reference direction. No absolute arc centre enters its definition.
- The element is built from the curved beam's end flexibility in closed form, with the flexibility factors on the two bending terms only. It is Euler–Bernoulli under either shear model.
- A realized bend's stress factors are the directional pair; its single scalar factor is not applied to an arc.
- Geometry that is over-determined is checked, not reconciled: chord, radius and angle must agree as entered.

**How each criterion is examined.**

- **VER-001** — Executable check: observes acceptance of a complete bend in the model document and in a library record, the named rejection of a mis-dimensioned value, and the missing-value finding on an incomplete library record.
- **VER-002** — Comparison with a named reference: the element's end flexibility and stiffness against closed-form curved-beam values under `validation/references/t4_i6`, produced without product code, for factors of 1 and for other factors on the in-plane and out-of-plane bending terms.
- **VER-003** — Comparison with a named reference, with an executable check: a thermal expansion loop against a closed-form or hand-calculation reference frozen under `validation/references/` with an independent generator; the check also observes the reaction across a sweep of the factor and the sparse and dense results side by side.
- **VER-004** — Executable check: observes the intensified bending published at each station of a realized arc against the value DEL-05-03's definition gives from the published bending stress rows of that station and the user's directional factors, and the intensified bending at a geometry-only bend's adjacent pipe end against the user's single factor times the resultant of the bending stress rows published there.
- **VER-005** — Executable check: observes each case of AC-005 refused with its named finding and no solved result, and the statement on a geometry-only bend.
- **VER-006** — Comparison with a named reference: the reference system (`validation/references/reference_system`) solved in the desktop on the exact route.
- **VER-007** — Executable check, with judgment by an independent reviewer: a protected-content scan over the bend's sources, schemas, fixtures and tests, which is a proxy; a reviewer who did not write them reads them against `docs/IP_AND_DATA_BOUNDARY.md` §3.

## Governing Values and Decisions — Axiology

- **AX-001** — *The user's factors, never ours.* Better is fewer places where a factor could originate in the software; the target is none. No convenience, example or default justifies one.
- **AX-002** — *Flexibility is solved.* Better is closer agreement between a solved bend and the closed-form curved beam with the user's factors. This is never bought by loosening a tolerance or by substituting a scaled straight chord for the arc.
- **AX-003** — *Refuse, do not repair.* Better is fewer inconsistent bends that reach a solve. This is never bought by altering what the user entered.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-007 OBJ-004 | REQ-001 REQ-002 REQ-003 REQ-004 REQ-005 REQ-008 REQ-011 | AC-001 | VER-001 | PRD §12.3 lists the inputs; the criterion states the expected acceptance and rejections |
| OUT-001 | SOW-007 OBJ-004 | REQ-010 REQ-012 | AC-002 | VER-002 | Outside source: the closed-form end flexibility of a circular curved beam, through references that import no product code. The tolerance is the governed analytic tolerance |
| OUT-001 | SOW-007 OBJ-004 | REQ-012 | AC-003 | VER-003 | Outside source: a closed-form or hand-calculation loop reference with an independent generator. The sweep and the agreement of solver modes are the product's own output and only guard against change |
| OUT-001 | SOW-007 OBJ-004 | REQ-003 REQ-012 | AC-004 | VER-004 | The criterion states the expected value from the user's factors (PRD §22.3) and the published base bending stresses; the combination on a realized arc is DEL-05-03's definition of intensified bending. Correctness of the base stress rests on DEL-05-03's references |
| OUT-001 | SOW-007 OBJ-004 | REQ-007 | AC-005 | VER-005 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state the expected refusal; the geometric conditions follow from the definition of a circular arc |
| OUT-001 | SOW-007 OBJ-004 | REQ-013 | AC-006 | VER-006 | Exit criterion 1 fixes the reference system's content; its expected results are the independent references of `validation/references/reference_system` |
| OUT-001 | SOW-007 OBJ-004 | REQ-006 REQ-009 | AC-007 | VER-007 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md`; PRD §12.3 and §20.1 |
