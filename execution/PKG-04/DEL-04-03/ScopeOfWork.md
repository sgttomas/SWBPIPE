---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-03
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-011]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-03 Linear support and restraint models

## Purpose and Objective Traceability

This deliverable defines how a piping model is held: what each linear support and restraint does to the frame's degrees of freedom, and how a movement imposed at a support enters the solve. Anchors, guides, line stops, vertical supports, directional restraints (on a global axis or skewed), translational and rotational springs, variable spring hangers and constant-effort supports each become restrained degrees of freedom, stiffness terms, prescribed values or forces that the frame kernel applies. Support loads are among the results an engineer publishes, so the support law has to be exact and nothing in it may be assumed.

It serves SOW-011 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-03), and PRD §12.4 and §12.2 with FR-SOL-001, FR-SOL-005 and the imposed-displacement part of FR-SOL-006; the variable spring's preload comes from SOW-083. In `docs/RELEASE_TARGET_v1.0.md`, exit criterion 1 names this deliverable, with DEL-04-04, as owner of the supports of the reference system: its linear restraints, its variable spring and its constant-effort support must solve on the exact route in the desktop without refusal. Under criterion 2 each of those passes closed-form or hand-calculation references from an independent generator, and under criterion 3 every refusal that remains is listed in the release limitations.

- **OUT-001** — The linear support model, by which the solver models and solves anchors, guides, line stops, vertical supports, directional restraints including skewed ones, translational and rotational springs, a variable spring hanger as its stiffness plus its cold load, a constant-effort support and imposed displacements on the six-degree-of-freedom frame model: for each, the restrained degrees of freedom, stiffness terms, prescribed values and forces it contributes to a solve, with the findings for what a support record lacks.

## Deliverable Definition — Ontology

**Terms the support model implements.**

- *Support record.* A support at a node, of one family, that names the degrees of freedom it acts on. The families of this deliverable are anchor, guide, line stop, vertical support, directional restraint, translational and rotational spring, variable spring hanger and constant-effort support. A snubber's meaning is DEL-04-10's; a nonlinear support's is DEL-04-04's.
- *Rigid restraint.* A named degree of freedom held at zero, or at a prescribed value. It is an exact constraint, never a stiff spring.
- *Spring.* A translational or rotational stiffness k on a named degree of freedom, between the pipe and a base. It acts on the pipe with −k(u − g), where g is the base's displacement, zero unless a movement is imposed there.
- *Variable spring hanger.* A spring of the user's entered rate on its declared translational degree of freedom, plus its entered cold (preset) load as a constant force. A spring held in a test case is DEL-05-01's held support; the rest of the preload law (the combination gate, recovery and diagnostics) is DEL-04-11/REQ-014's.
- *Constant-effort support.* A support of zero stiffness that applies the user's entered constant force along the positive direction of its translational degree of freedom. A record with no entered force applies none and is reported.
- *Skewed restraint.* A restraint or spring that acts along an authored global direction instead of a global axis. The direction is any finite nonzero global vector, stored as authored and never normalized; a direction that lies exactly along an axis is that axis.
- *Imposed displacement.* A prescribed translation or rotation at a support's degree of freedom in a load case, such as an anchor or nozzle movement. It is the user's input; the product supplies no movement.
- *Support finding.* A named statement that a support record lacks or misstates something a solve requires.

**Boundary.** The support model applies nothing to a matrix and iterates on nothing:

- DEL-04-01 reduces the system for restrained and prescribed degrees of freedom and for a restraint along a direction; this deliverable decides which restraints a model may carry;
- DEL-04-04 decides the state of one-way, gap, lift-off and friction supports;
- DEL-04-06 maps support findings and their codes into the diagnostic envelope; the findings and codes are this deliverable's;
- DEL-04-07 integrates this model into the product solve on both routes, forms the load a moved spring base adds, and publishes support reactions;
- DEL-05-01 owns the load case that carries an imposed displacement; DEL-02-01 owns the support record's shape in the model document;
- DEL-04-11 sizes hangers and states the rest of the preload law.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The model covers anchors, guides, line stops, vertical supports, springs and imposed displacements (SOW-011), directional restraints and translational and rotational springs (PRD §12.4, FR-SOL-005), and the variable spring hanger and constant-effort support from user-entered data (PRD §12.4; DEC-049).
- **REQ-002** — Every support acts on the six degrees of freedom of a frame node through the frame kernel's degree-of-freedom mapping (`docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-003** — The model is mechanics only: it decides no rule acceptability and no suitability of a support, and no output or text of it states or implies approval or code compliance (`docs/CONTRACT.md` OPS-K-MECH-2; PRD §21.2).
- **REQ-004** — Stiffnesses, displacements, rotations and forces keep their dimension; a quantity whose dimension does not match its use is rejected, and the model converts nothing (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-005** — A support record that lacks a solve-required value (a degree of freedom, a spring's stiffness, an imposed value, a constant-effort support's force, a variable spring hanger's rate or cold load) is a finding. No stiffness, load or movement is defaulted (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-006** — Examples and fixtures use invented values only: no protected standards content and no vendor catalogue data (`docs/CONTRACT.md` OPS-K-IP-1).
- **REQ-007** — Each support finding carries a code, the support's identity and a message, in a deterministic order.
- **REQ-008** — Supports reach a solve only through the kernel's boundary-condition interface. No adapter or caller edits a stiffness to imitate a support.
- **REQ-009** — Support output is deterministic: the same model gives the same boundary data and findings, in the same order, on every run, so every support check is deterministic (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-010** — *Skewed restraints.* (a) A single-degree-of-freedom translational restraint or spring may act along an authored finite nonzero global direction, stored as authored and never normalized; a snubber likewise, under DEL-04-10. (b) A skewed rigid restraint is enforced exactly, dᵀu = 0 to rounding, never by a penalty stiffness. (c) A skewed restraint may share its node with axis restraints and springs; the node's restraints are reduced together, and each reaction is attributed to its support. (d) A skewed spring contributes the stiffness k·d dᵀ/(dᵀd). A case the support model does not admit is refused by name, never solved approximately. A case the model cannot represent exactly or attribute uniquely is always refused: a direction outside the range the exact form can represent, a skewed record on a node with no element, and a set of restraints at a node whose rows are dependent. Which further cases are admitted is not decided: two or more skewed rigid restraints at one node, imposed motion at or along a skewed record, skewed records in a model with nonlinear supports, and skewed records on the exact route.
- **REQ-012** — For each model, the support model yields the complete boundary data of a solve: the restrained degrees of freedom, the spring terms and the prescribed values, the same in the sparse and dense solve modes.
- **REQ-013** — *Imposed displacement.* An imposed displacement at a rigid support is solved as a prescribed value of the partitioned system, K_ff·u_f = f_f − K_fc·g_c with u_c = g_c, on both solve routes; it is not also added as a force. At a spring it moves the spring's base, adding the load k·g. In a model with nonlinear supports it enters the nonlinear solve the same way, and a movement at a nonlinear support's own degree of freedom moves that support's ground. Where imposed motion at a skewed restraint is admitted (REQ-010), an imposed displacement g moves the restraint's ground along the authored direction, by g along d/|d|.
- **REQ-014** — *Hanger families.* A constant-effort support contributes no stiffness and acts as its user-entered force along the positive axis of its declared translational degree of freedom; a record with no entered force applies none and is reported by a support finding; none is defaulted. A variable spring hanger is modelled as a spring of its user-entered rate on its declared translational degree of freedom plus its user-entered cold (preset) load as a constant force (PRD §12.4; SOW-083). No rate, load, catalogue value or sizing result is defaulted (DEC-049).

### Acceptance criteria

- **AC-001** — For each of anchor, guide, line stop, vertical support, directional restraint on a global axis, translational spring and rotational spring, a solved model held by that support gives, on both solve routes, displacements and support reactions that agree with closed-form mechanics at the analytic tolerance (1e-9 relative, with its governed absolute floor near zero): a restrained degree of freedom is exactly zero, a spring's reaction is −k·u, and reactions balance the applied loads.
- **AC-002** — An imposed displacement at an anchor, at a restraint, at a spring's base and at a nonlinear support's ground reproduces the closed-form displaced solution, superposes with the other loads of its case, and adds no force of its own, on both routes.
- **AC-003** — A constant-effort support reduces the reactions elsewhere by exactly its entered force and adds no stiffness, on both solve routes; a record with no entered force applies no force and is reported with its code and support; and the support's published action equals the force the solve applied.
- **AC-004** — A skewed rigid restraint leaves dᵀu = 0 with motion perpendicular to d free; a skewed restraint and the axis restraints and springs at its node hold together, each with its own attributed reaction; a skewed spring adds the stiffness k·d dᵀ/(dᵀd); and each reaction along a direction matches the independent reference. Each case REQ-010 does not admit is refused with its own code and names the support.
- **AC-005** — A support record that lacks a required value, repeats a restraint or names a node outside the model is reported as a finding with its code and support, and no solve proceeds on an assumed value.
- **AC-006** — A stiffness, displacement, rotation or force whose dimension does not match its use is rejected.
- **AC-007** — The boundary data of a model are the same in both solve modes and on repeated runs, and results agree between the modes at the analytic tolerance.
- **AC-008** — No support output or message states or implies suitability, approval or compliance, and the support model's sources, tests and fixtures contain nothing that REQ-006 excludes.
- **AC-009** — A model held by a variable spring hanger with a cold load gives, on both solve routes, displacements and a support force that agree with the closed-form spring-plus-preload solution at the analytic tolerance. The rest of the preload law is examined under DEL-04-11/AC-014.

## Production and Verification Method — Praxeology

**Method choices that define the support model.**

- A rigid restraint is imposed by reducing the system, never by a penalty stiffness.
- A skewed direction is used through an exact form of the authored vector, so that the constraint holds exactly in the reduced system and the reaction along the direction can be attributed without ambiguity.
- A constant-effort support's force acts along the positive axis of its degree of freedom; the user's sign convention is the model's.
- The support model records the caller's unit basis and converts nothing.

**How each criterion is examined.**

- **VER-001** — AC-001 to AC-004 and AC-009, comparison with named references: closed-form support mechanics, a closed-form spring-plus-preload hand calculation, and the frozen packages `validation/references/w9_benchmarks` (imposed anchor displacement), `w16_imposed_motion`, `w16_nonlinear_motion`, `rt_a3_constant_effort` and `skew_restraints`, whose generators import no product code.
- **VER-002** — AC-004 to AC-007, executable check: observes each finding and each skew refusal with its code and named support, each dimension rejection, and the boundary data in both modes and on repeated runs.
- **VER-003** — AC-008, judgment by a reviewer who did not write the code: examines support texts against `docs/claims_registry.md`, and the protected-content scan runs over the support model's sources, tests and fixtures.

## Governing Values and Decisions — Axiology

- **AX-001** — *Exact constraints.* Better is a restraint that holds exactly and a reaction closer to the independent reference. This is never bought with a penalty stiffness or a tolerance on the constraint.
- **AX-002** — *More of what users author is admitted.* Better is fewer refusals of authored supports. An admission is never bought by solving a case the model cannot represent exactly or attribute uniquely: such a case stays refused by name.
- **AX-003** — *Nothing assumed.* Better is fewer values the model could supply by itself; the target is none. No stiffness, load, direction or movement is invented for the user's convenience.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-011 OBJ-003 | REQ-001 REQ-002 REQ-008 | AC-001 | VER-001 | Outside source: closed-form support mechanics and equilibrium, by hand calculation |
| OUT-001 | SOW-011 OBJ-003 | REQ-013 | AC-002 | VER-001 | Outside source: `validation/references/w9_benchmarks`, `w16_imposed_motion` and `w16_nonlinear_motion`, generated without product code |
| OUT-001 | SOW-011 OBJ-003 | REQ-014 | AC-003 | VER-001 | Outside source: `validation/references/rt_a3_constant_effort`, an analytical reference generated without product code; the positive-axis force convention is this scope's method choice (Praxeology). PRD §5.5 requires that a record with no entered force be reported |
| OUT-001 | SOW-011 OBJ-003 | REQ-010 | AC-004 | VER-001 VER-002 | Outside source: `validation/references/skew_restraints`, generated without product code; the refusals are those REQ-010 names |
| OUT-001 | SOW-011 OBJ-003 | REQ-005 REQ-007 | AC-005 | VER-002 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state the expected findings |
| OUT-001 | SOW-011 OBJ-003 | REQ-004 | AC-006 | VER-002 | PRD §5.6 and `docs/CONTRACT.md` OPS-K-UNIT-1 state the expected rejection |
| OUT-001 | SOW-011 OBJ-003 | REQ-009 REQ-012 | AC-007 | VER-002 | The product's own other mode and earlier output, which only guard against change; correctness rests on AC-001 to AC-004 |
| OUT-001 | SOW-011 OBJ-003 | REQ-003 REQ-006 | AC-008 | VER-003 | The reviewer's reading against PRD §21.2 and `docs/claims_registry.md`, and the protected-content scan under `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-001 | SOW-011 OBJ-003 | REQ-014 | AC-009 | VER-001 | Outside source: a closed-form spring-plus-preload hand calculation, frozen with a generator that imports no product code |
