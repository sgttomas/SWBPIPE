---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-02
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-006]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-02 Straight pipe element

## Purpose and Objective Traceability

This deliverable is the straight pipe element: the one place where a straight run of pipe between two nodes becomes mechanics. From the member's section and material properties it gives the frame kernel an element stiffness, turns loads carried along the member into equivalent nodal loads, and, after the solve, recovers the internal forces and moments at the member's ends and at stations along it. Most of a piping model is straight pipe, so the displacements, support loads and member forces an engineer publishes rest on it.

It serves SOW-006 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-02), and PRD §12.3 "Straight Pipe Elements" with FR-SOL-003 and FR-SOL-007. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it; it reaches criterion 1 (the reference system solves) and criterion 2 (each analysis passes closed-form references) through DEL-04-07's route.

- **OUT-001** — The straight pipe element: its stiffness from explicit section properties, its equivalent nodal loads for member loads and free strain, its weight per length, and its recovery of end actions in local and global coordinates and of station resultants in the member's local axes.

## Deliverable Definition — Ontology

**Terms the element implements.**

- *Straight pipe element.* A two-node frame element along the chord between its nodes, with local axis x along the chord and y, z from the member's orientation reference.
- *Section properties.* Metal area, the two second moments of area and the torsion constant, with the moduli E and G: the element's whole description of the cross-section. For a member the model selects as Timoshenko, the shear area as well.
- *Member load.* A force per length over the whole span or a stated part of it, or a point force at a station, in local or global axes.
- *Equivalent nodal loads.* The end forces and moments that do the same work as a member load on the element's displacement shapes: for a full-span uniform load w on a member of length L, forces wL/2 and moments ±wL²/12 at the ends.
- *Free strain.* An axial strain the member would take if unrestrained (thermal, fit or pressure-induced). It enters as the equal and opposite end forces E·A·ε along the chord, which recovery removes again.
- *Weight per length.* The member's mass per length times the gravity the caller supplies. The mass per length is the caller's sum of the pipe, contents, insulation and component mass it distributes along the member.
- *End actions.* The forces and moments on the element at its two nodes, f = K·d − f_eq, in local coordinates and, rotated by the member's transform, in global coordinates.
- *Station resultants.* The internal axial force, two shears, torsion and two bending moments at a station along the member, in local axes: the end action at node i plus the loads carried between that end and the station.

**Boundary.** The element computes no section property, forms no load value, assembles no system and checks no stress:

- DEL-03-08 computes section and mass properties from the user's dimensions;
- DEL-05-01 forms the weight, thermal-strain and other primitive load values; DEL-05-05 decides which member loads a user can author and applies them to members;
- DEL-04-01 owns local axes, transforms, assembly and solve. The element's stiffness is evaluated with the two-node frame element formula held in the kernel's code, which the kernel's other straight members share; this deliverable remains accountable for that stiffness meeting REQ-002;
- DEL-05-03 recovers stresses from the resultants; the Timoshenko shear factor is its definition;
- DEL-04-07 maps model members onto elements, forms what pressure contributes to the solve on the exact route, maps element findings into the result envelope and publishes the rows.

**Not decided.** Whether the formation bounds that consumers read from the element are a commitment of this deliverable. Whether a point force on a member is a product capability, and so whether a member selected as Timoshenko refuses one; it is the same question as DEL-05-05/REQ-008, and AC-005 stands as written until that is decided.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The element is a member of the 3D centerline frame model and does not bypass the frame kernel: its transforms, assembly and solve are the kernel's, and its stiffness is the one straight-member frame formula the kernel assembles (SOW-006; `docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-002** — The element's 12×12 stiffness carries axial stiffness EA/L, bending stiffness about both local transverse axes and torsional stiffness GJ/L in the member's local coordinate system, formed from open mechanics and the caller's section and material values only (PRD §12.3, FR-SOL-003; `docs/CONTRACT.md` OPS-K-DATA-1).
- **REQ-003** — Section properties are explicit inputs. A missing, non-finite or non-positive solve-required property is a finding that names it; nothing is defaulted (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-004** — The element gives the weight per length of the member from the caller's mass per length and gravity, covering distributed pipe, contents, insulation and component weight; the equivalent nodal loads of full-span and partial-span distributed loads and of point forces; and the axial free-strain effect E·A·ε through which thermal strain enters. It applies no load of its own (PRD §12.2, §12.3).
- **REQ-005** — The element recovers end forces and moments in local and global coordinates, and resultants at the ends and at optional stations along the member in local axes, with their units, as mechanics quantities for stress recovery. It evaluates no stress rule (PRD §12.2, §12.3, FR-SOL-003, FR-SOL-007; `docs/CONTRACT.md` OPS-K-MECH-2).
- **REQ-006** — Element output is deterministic: the same inputs give bit-identical stiffness, load vectors and recovered actions on every run, so every element check is deterministic (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-007** — Element findings carry a code, the affected member and a message, and state or imply no approval, certification or compliance (PRD FR-SOL-015, §12.8, §21.2).
- **REQ-008** — All element inputs, intermediate values and outputs are unit-aware and dimensionally checked (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-009** — The element forms no pressure load. Pressure reaches it only as the free strain or member load its caller forms, and the member's pressure metadata passes to stress recovery unchanged (PRD §12.3).
- **REQ-010** — Element sources, tests and fixtures hold only invented or rights-cleared values: no protected dimensional table, material value or standards content (`docs/CONTRACT.md` OPS-K-IP-1; PRD §20.1).

### Acceptance criteria

- **AC-001** — On a single element and on multi-element spans, displacements and recovered end actions, in local and in global coordinates, agree with closed-form beam mechanics at the analytic tolerance (1e-9 relative, with a stated absolute floor near zero): axial extension, torsion, tip force and tip moment on a cantilever, a simply supported pipe under its own weight, a fixed-fixed member under free strain, and an inclined member.
- **AC-002** — The equivalent nodal loads of a full-span uniform load are wL/2 and ±wL²/12; those of a partial-span load and of a point force equal the fixed-end actions of the same load on a fixed-fixed beam; and for a Timoshenko member they equal the exact Timoshenko fixed-end actions.
- **AC-003** — The resultants at any station satisfy statics with the end resultants and the loads between them, including partial-span loads and free strain, and the station at each end equals the end resultant.
- **AC-004** — The weight per length equals mass per length times the supplied gravity, and no weight, thermal or other load appears in a solve unless the caller supplied it.
- **AC-005** — A missing or invalid section property, a zero-length member, a station outside the span or an inverted load span is refused with a finding that names the member and the input. A point member force on a member selected as Timoshenko is refused by name.
- **AC-006** — A section property, load or recovered quantity whose dimension is incompatible with its use is rejected with a finding that names it, and the element assumes no unit.
- **AC-007** — No element output or message states or implies stress acceptability, code compliance or approval.
- **AC-008** — Repeated runs give bit-identical stiffness, load vectors and resultants.
- **AC-009** — The element's sources, tests and fixtures contain only invented or rights-cleared values.
- **AC-010** — A member's local and global stiffness equal the frame kernel's frame element for the same section, material and orientation, bit for bit, and the element forms no stiffness term, transform, assembly or solve of its own.
- **AC-011** — The element forms no pressure load, and the pressure metadata of a member reaches stress recovery with the same values it was given.

## Production and Verification Method — Praxeology

**Method choices that define the element.**

- The element's stiffness is the frame kernel's two-node frame element evaluated with the member's section: Euler–Bernoulli, or the exact static Timoshenko element where the model selects it. The element adds no stiffness term of its own.
- Equivalent nodal loads are the work-consistent loads of the selected beam theory. A lumped approximation is not used.
- End actions are f = K·d − f_eq: recovery removes the equivalent loads and free-strain forces the element added, so a recovered action is the member's internal action and not the nodal force. A station resultant is the end action at node i plus the integrals of the loads along the member to the station.

**How each criterion is examined.**

- **VER-001** — AC-001 to AC-003, comparison with named references: closed-form beam solutions and the frozen packages `validation/references/w9_benchmarks` (simply supported pipe under weight), `t4_u8` and `pf_timoshenko` (Timoshenko members) and `w16_timoshenko_partial_wind` (partial-span loads), whose generators import no product code.
- **VER-002** — AC-002 and AC-003, executable check: observes the equivalent nodal loads and the station resultants against the fixed-end actions and statics the criterion states.
- **VER-003** — AC-004 to AC-006 and AC-008, executable check: observes each invalid or mis-dimensioned input refused with its named finding, the weight per length, and repeat-run bit identity.
- **VER-004** — AC-007 and AC-009, judgment by a reviewer who did not write the code: examines element messages against `docs/claims_registry.md`, and the protected-content scan runs over the element's sources, tests and fixtures.
- **VER-005** — AC-010, executable check with judgment by a reviewer who did not write the code: the check observes the element's local and global stiffness against the kernel frame element's for the same section, and the reviewer examines that the element forms no stiffness term, transform, assembly or solve of its own.
- **VER-006** — AC-011, executable check: observes that a pressurized member adds no element load beyond what its caller formed, and that the member's pressure metadata at stress recovery equals the authored values.

## Governing Values and Decisions — Axiology

- **AX-001** — *Exact for the beam theory selected.* Better is a smaller deviation from the closed forms and independent references, for end and station actions and for load equivalents. This is never bought by loosening a tolerance, altering a reference, or adding a correction that the selected theory does not contain.
- **AX-002** — *Nothing assumed.* Better is more missing or incompatible inputs refused by name and fewer values the element could supply by itself; the target is none. Convenience for the caller never justifies a default section, material, weight or gravity, and outputs stay mechanics only.
- **AX-003** — *One stiffness formula.* The element stays a thin adapter onto the kernel's frame element. A second stiffness derivation here, even a faster one, is a defect.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-006 OBJ-003 | REQ-002 REQ-005 | AC-001 | VER-001 | Outside source: closed-form beam mechanics and `validation/references/w9_benchmarks`, `t4_u8` and `pf_timoshenko`, generated without product code |
| OUT-001 | SOW-006 OBJ-003 | REQ-004 | AC-002 | VER-001 VER-002 | Outside source: fixed-end actions in closed form (wL/2 and ±wL²/12 for a full-span load), and `validation/references/w16_timoshenko_partial_wind`, whose values are not computed through the product's element |
| OUT-001 | SOW-006 OBJ-003 | REQ-005 | AC-003 | VER-001 VER-002 | Outside source: statics in closed form |
| OUT-001 | SOW-006 OBJ-003 | REQ-004 | AC-004 | VER-003 | The criterion states the expected value; PRD §5.5 requires that the element supply no load |
| OUT-001 | SOW-006 OBJ-003 | REQ-003 REQ-007 | AC-005 | VER-003 | PRD §5.5 and §12.8 and `docs/CONTRACT.md` OPS-K-DATA-2 state the expected refusals. The refusal of a point force on a Timoshenko member rests on no ruling: it is undecided (Ontology) |
| OUT-001 | SOW-006 OBJ-003 | REQ-008 | AC-006 | VER-003 | PRD §5.6 and `docs/CONTRACT.md` OPS-K-UNIT-1 state the expected refusal |
| OUT-001 | SOW-006 OBJ-003 | REQ-007 | AC-007 | VER-004 | The reviewer's reading against PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-006 OBJ-003 | REQ-006 | AC-008 | VER-003 | The product's own earlier output, which only guards against change |
| OUT-001 | SOW-006 OBJ-003 | REQ-010 | AC-009 | VER-004 | Protected-content scan and the reviewer's reading under `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-001 | SOW-006 OBJ-003 | REQ-001 | AC-010 | VER-005 | SOW-006 and `docs/CONTRACT.md` OPS-K-MECH-1 place the element inside the frame kernel's boundary. The comparison is against the kernel's frame element, which this deliverable does not write |
| OUT-001 | SOW-006 OBJ-003 | REQ-009 | AC-011 | VER-006 | PRD §12.3 and §12.5 state what is carried; the comparison is against the authored input |
