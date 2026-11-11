---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-04
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-008]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-04 Branch connection component model fields

## Purpose and Objective Traceability

This deliverable defines what a branch connection is in the product: the component at the node where a branch pipe leaves a header. A user gives it the header and branch it joins, its sizes, angle and type, its reinforcement data, and their own header-side and branch-side stress intensification factors and flexibility factor, each with the source it came from. The definition says what those inputs are and what they do: the three pipes meet as members of the solved frame, a supplied flexibility factor enters that frame's stiffness, and the header and branch factors multiply the bending stress recovered at their own pipe ends. The software supplies no factor of its own.

It serves SOW-008 under OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-04), and PRD §12.3 (branch connection objects) with FR-SOL-011, the branch assembly benchmark of §22.2 and the user-entered SIF multiplier and flexibility-factor behaviours of §22.3. In `docs/RELEASE_TARGET_v1.0.md` it is an owning deliverable of exit criterion 1 for the reference system's tee with user modifiers.

- **OUT-001** — The branch connection component definition: its members in the model document and in a component library record, and the behaviour of a branch in a solve and in stress recovery.

## Deliverable Definition — Ontology

**Terms the definition implements.**

- *Branch component.* A component of kind branch, tee or branch connection at the intersection node. It names the header pipe and the branch pipe, which meet at that node and differ, and carries the run size, header size, connection angle, connection type, reinforcement area, a reinforcement reference and a source reference for that geometry. These members of the model document, and the local modifiers, are this deliverable's definition.
- *Local modifiers.* The user's header-side stress intensification factor, branch-side stress intensification factor and flexibility factor, with one source reference. Each is dimensionless, finite and above zero.
- *Header side and branch side.* The header side is the end of the header pipe at the intersection node; the branch side is the end of the branch pipe there. The header factor multiplies the resultant bending stress at the header side and the branch factor that at the branch side.
- *The joint.* In the solved frame the pipes of a branch with no supplied flexibility factor share the intersection node's six degrees of freedom. Where the user supplies a flexibility factor, a branch element carries it into the global stiffness (DEC-070).
- *Library record of a branch.* A component library record of type branch, whose field kinds hold the run and header sizes, connection angle and type, reinforcement area and reference, the intensification and flexibility factors, and the geometry source reference.

**Boundary.** This scope states the branch's definition and the criteria a branch meets. DEL-04-07 composes it into the product solve, admits it on each route, and owns the pressure junction formed where pressure regions meet at its node. DEL-05-03 owns the base stresses the factors multiply and the two section bases they are published on. DEL-02-01 owns the model document and DEL-03-02 the component library format. DEL-07-03 owns the creation form. Local checks of a branch against a design code are evaluated only through the user's rule packs (PRD §12.7); a first-class nozzle-flexibility object is not modelled (PRD §7).

**Not decided.** How a branch's single flexibility factor acts in the branch element that carries it: which member, side and bending axes it makes more flexible.

## Completion and Reliance Basis — Epistemology

A branch result is a statement about three frame members meeting at a node, with the user's flexibility factor in the stiffness where one is supplied and the user's intensification factors applied to recovered stress. It is not a local stress analysis of the fitting and not a code check.

### Requirements

- **REQ-001** — A branch accepts the inputs PRD §12.3 lists: header association, branch association, intersection node, branch angle, reinforcement data, header-side modifiers, branch-side modifiers and provenance for local data (SOW-008; PRD FR-SOL-011).
- **REQ-002** — Branch factors, reinforcement values and other code-specific values are the user's or lawfully imported private data. The product ships no branch, SIF, flexibility, dimensional or reinforcement table, equation or default, and computes no factor (`docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-IP-1).
- **REQ-003** — A missing value a solve or a rule check requires is an explicit, named finding. Nothing is defaulted (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-004** — The geometry and each local modifier carry the user's source and provenance (PRD §12.3; `docs/CONTRACT.md` OPS-K-DATA-3).
- **REQ-005** — Sizes, the connection angle and the reinforcement area are unit-aware and dimensionally checked; factors are dimensionless (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-006** — A branch's values reach a solve only through the product's validation: unit, provenance and completeness checks are not bypassed on any path that creates or changes a branch (the decomposition's architecture basis for DEL-00-07; `docs/CONTRACT.md` OPS-K-IP-3).
- **REQ-008** — The user's header-side and branch-side factors multiply the resultant bending stress recovered at their own sides (PRD §22.3).
- **REQ-009** — A branch with user modifiers is solved on the pressure-free route and on the exact route (release target exit criterion 1; PRD §22.2, branch assembly case).
- **REQ-010** — Where the user supplies a branch flexibility factor, it enters the global stiffness through a branch element that carries it. A factor that is in the stiffness is not applied again to stress (DEC-070; PRD §22.3).

### Acceptance criteria

- **AC-001** — (a) In the model document, a branch accepts every input of REQ-001. (b) A library record of a branch accepts the run and header sizes, connection angle and type, reinforcement area and reference, the header-side intensification factor, the branch-side intensification factor, the flexibility factor and their sources. In both, each numeric value has its unit and each factor is dimensionless, and in a library record a missing value is marked with a branch completeness finding.
- **AC-002** — On every product path that creates or changes a branch, each of the run size, header size, connection angle and reinforcement area is accepted with a unit of its dimension and brought to the model's basis, is rejected with a finding that names it when its dimension is wrong, and when absent is reported as missing with nothing substituted.
- **AC-003** — Each published intensified bending stress at a branch equals the user's factor for that side times the resultant bending stress hypot(σ_y, σ_z) formed from the bending stress rows published at the same member end and section basis: the header factor at the header side, the branch factor at the branch side. On the exact route this holds on each section basis.
- **AC-004** — A header with a branch solves with the intersection node in equilibrium under its loads, and its displacements, member end actions and reactions reproduce an independent reference at the analytic tolerance.
- **AC-005** — Each of these raises a named finding on the component: header and branch pipes that are the same pipe or do not meet at the component's node as a branch; a missing header pipe, branch pipe, size, angle, type, reinforcement reference or geometry source; a missing or invalid header factor, branch factor, flexibility factor or modifier source reference. No intensified stress is published at a side whose own factor is missing or invalid, or whose factor has no source reference.
- **AC-006** — The tee of the invented reference system, with its user modifiers, solves on the exact route in the desktop and is not refused.
- **AC-007** — The branch's sources, schemas, fixtures and tests contain nothing REQ-002 excludes, and every example value is invented and labelled so.
- **AC-008** — A branch with a supplied flexibility factor solves with that factor in its stiffness. Its displacements and end actions reproduce an independent known-flexibility reference.

## Production and Verification Method — Praxeology

**Method choices that define the branch.**

- A branch is a record at its node. With no supplied flexibility factor it adds no element: the header and branch pipes are ordinary frame members that meet at the node.
- Each side's factor is applied at that side's pipe end, by association of the component with the pipes it names. A pipe the component does not name receives no factor.
- A realized bend that a branch names carries no branch factor.
- Example values are invented; where a real value would be needed, the example omits it and shows the missing-value finding.

**How each criterion is examined.**

- **VER-001** — Executable check: observes acceptance of a branch with every input of REQ-001 in the model document and of a library record with every member of AC-001 (b), each with its unit, and the missing-value finding on an incomplete library record.
- **VER-002** — Executable check: for each of the four quantities of AC-002, on each creating and changing path, observes the accepted, rejected and absent outcomes with their named findings.
- **VER-003** — Executable check: observes each published intensified stress against the user's factor times the resultant of the bending stress rows published at the same member end and section basis, on both routes and on each section basis.
- **VER-004** — Comparison with a named reference: a branch assembly against the exact-arithmetic frame references under `validation/references/rt_a3_tee`, generated without product code.
- **VER-005** — Executable check: observes each case of AC-005 with its named finding, and the absence of intensified rows at a side whose factor or source reference is missing or invalid.
- **VER-006** — Comparison with a named reference: the reference system (`validation/references/reference_system`) solved in the desktop on the exact route.
- **VER-007** — Executable check, with judgment by an independent reviewer: a protected-content scan over the branch's sources, schemas, fixtures and tests, which is a proxy; a reviewer who did not write them reads them against `docs/IP_AND_DATA_BOUNDARY.md` §3.
- **VER-008** — Comparison with a named reference: a branch with a supplied flexibility factor against a closed-form or hand-calculated known-flexibility reference frozen under `validation/references/` with an independent generator; displacements and end actions are compared.

## Governing Values and Decisions — Axiology

- **AX-001** — *The user's factors, never ours.* Better is fewer places where a branch factor, size or reinforcement value could originate in the software; the target is none. No convenience, example or default justifies one.
- **AX-002** — *Say what was applied.* Better is fewer branch inputs that a result leaves unaccounted for: each is either applied and shown, or stated as not applied. This is never bought by applying a factor whose effect has not been defined.
- **AX-003** — *Mechanics only.* Better is a clearer line between the stress the frame recovers and any judgment of the fitting. No output of a branch states or implies a code check.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-008 OBJ-004 | REQ-001 REQ-004 | AC-001 | VER-001 | Owner ruling: PRD §12.3 lists the inputs; the criterion states the expected acceptance |
| OUT-001 | SOW-008 OBJ-004 | REQ-005 REQ-006 | AC-002 | VER-002 | Owner ruling: `docs/CONTRACT.md` OPS-K-UNIT-1 and PRD §5.6 state the expected acceptance, rejection and finding |
| OUT-001 | SOW-008 OBJ-004 | REQ-008 | AC-003 | VER-003 | The criterion states the expected value: the user's factor times the resultant of the published bending stresses (PRD §22.3). Correctness of the base stress rests on DEL-05-03's references |
| OUT-001 | SOW-008 OBJ-004 | REQ-009 | AC-004 | VER-004 | Outside source: exact-arithmetic frame references for a branch assembly, from a generator that imports no product code (PRD §22.2). The tolerance is an owner ruling (DEC-026) |
| OUT-001 | SOW-008 OBJ-004 | REQ-003 | AC-005 | VER-005 | Owner ruling: PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state the expected findings |
| OUT-001 | SOW-008 OBJ-004 | REQ-009 | AC-006 | VER-006 | Owner ruling for the reference system's content (exit criterion 1); its expected results are the independent references of `validation/references/reference_system` |
| OUT-001 | SOW-008 OBJ-004 | REQ-002 | AC-007 | VER-007 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md`; owner ruling PRD §20.1 |
| OUT-001 | SOW-008 OBJ-004 | REQ-010 | AC-008 | VER-008 | Outside source: a closed-form or hand-calculated reference with an independent generator (DEC-070; PRD §22.3) |
