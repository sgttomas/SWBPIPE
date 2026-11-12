---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-04
package_id: PKG-04
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-012]
package_objective_refs: [OBJ-003]
---

# Scope of Work — DEL-04-04 Nonlinear support active-set solver

## Purpose and Objective Traceability

At each iteration of the product's controlled iterative solve, each nonlinear support takes the state the mechanics admits for that iterate, so that a model containing such supports converges to an admissible support state or reports, with its active set, why it did not. This deliverable decides those states: whether a one-way support bears, a gap is closed, a pipe has lifted off its support, and whether a friction support sticks or slides. Given the displacements and reactions of one solve iteration it classifies each such support, says which have changed and whether the set has settled, and records the result so a reader can see which supports carried load. A nonlinear case is only as right as these states: a support taken as bearing when the pipe has lifted gives wrong loads everywhere.

It serves SOW-012 under OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-04, row DEL-04-04), and PRD §12.4 with FR-SOL-014 (gaps, one-way supports, lift-off and friction converge on validation cases and report diagnostics), the gapped-support and friction-support benchmarks of §22.2 and the nonlinear-convergence regression of §22.4. In `docs/RELEASE_TARGET_v1.0.md`, exit criterion 1 names this deliverable, with DEL-04-03, as owner of the supports of the reference system: its gap, lift-off and friction supports must solve on the exact route in the desktop without refusal. Under criterion 2 they pass closed-form or hand-calculation references frozen with an independent generator.

- **OUT-001** — The nonlinear support classifier: for one-way, gap, lift-off and friction supports, the state each takes from an iteration's displacements and reactions, the convergence verdict, and the record of states, changes and limitations.

## Deliverable Definition — Ontology

**Terms the classifier implements.**

- *Nonlinear support.* A support whose restraint depends on the solution: a one-way support (bears in one sense only), a gap (free until its clearance closes), a lift-off support (contact that can be lost) and a friction support (resists sliding up to a limit).
- *Nonlinear support record.* The support, its node and degree of freedom, its class with the sense in which it bears or closes, a gap's clearance and a friction support's coefficient.
- *State.* Active or inactive for a one-way, gap or lift-off support; for a friction support, inactive while its normal does not bear, otherwise sticking or sliding. In the system, an active one-way or lift-off support, a closed gap and a sticking friction support are each the rigid restraint DEL-04-03 defines on the support's degree of freedom; an inactive or open support is free; a sliding support is free in its tangential degree of freedom and carries the friction law's force.
- *State-switched test.* A support's governing quantity depends on its present state. An engaged support, a closed gap included, is judged on the sign of its reaction: it stays engaged while it bears. A released support is judged on its displacement towards the side it can bear against, or against its clearance: it engages when it would penetrate. Each is measured from the support's ground position, which an imposed support displacement moves, so re-engagement and lift-off are both observable.
- *Friction law.* A friction support whose normal bears is sticking while its tangential reaction satisfies |T| ≤ μ·N and sliding otherwise, N being the normal reaction of the current iterate. A sliding support carries a tangential force of magnitude μ·N opposing its motion; a sticking support holds its tangential degree of freedom. The law is static and history-free: each case is solved by itself, with no loading path, accumulated slip or carry-over between cases.
- *Convergence.* The set has converged when no support changes state and the residual is within the tolerance of its support class.
- *Admissibility confirmation.* A check, once the iteration has converged, that the converged state satisfies the state law: each support's deciding quantity (contact reaction, separation, stick margin μ·N − |T|, slip direction, and a friction normal derived from a reaction, which must be established as nonzero) lies strictly on its state's side, with its numerical error taken into account.
- *Active-set record.* The states of all nonlinear supports in a case, the supports that changed in the last iteration, the iteration settings, the convergence verdict, and the assumptions and limitations that apply.

**Boundary.** This deliverable classifies and records. It does not assemble or solve the nonlinear system.

- DEL-04-07 owns the assembled nonlinear iteration (assemble, solve, classify, update), its anti-chatter measures, the application of the friction force and of a closed gap's prescribed position in the system, and the running of the admissibility confirmation after convergence, with the publication of its outcome.
- DEL-04-01 assembles and solves each iteration.
- DEL-04-03 owns linear supports and the imposed movement of a support's ground.
- DEL-04-06 owns the diagnostic records a non-convergence is reported with.
- DEL-08-04 owns the export vocabulary for state codes, counts and flags.

What the admissibility confirmation requires is this deliverable's state law (REQ-013).

## Completion and Reliance Basis — Epistemology

A classified state is a statement about the discrete model under the friction law above. It is not a statement about the physical support. For a frictionless case with strict contact and a nonsingular reduced matrix the admissible state is unique, by linear complementarity theory; that does not guarantee convergence from every starting state. With friction, a converged state is one admissible state: it may depend on the starting states, and it says nothing about reachability through a loading history.

### Requirements

- **REQ-001** — Nonlinear supports act on the degrees of freedom of the 3D frame model; no local contact model replaces it (`docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-002** — The classifier covers one-way restraints, gaps, lift-off and friction, for use by a controlled iterative method (SOW-012; PRD §12.4).
- **REQ-003** — Every nonlinear case reports its convergence state, each support's state, and any unresolved non-convergence (`docs/CONTRACT.md` OPS-K-SOLVER-2).
- **REQ-004** — Classification is deterministic: the same trial facts and starting states give the same states, changes and verdict on every run, so every classifier check is deterministic (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-005** — A nonlinear support that lacks a solve-required value (a gap's clearance, a friction coefficient, a normal or tangential reaction, a trial state) is a blocking finding. No value, tolerance or coefficient is defaulted (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-006** — Clearances, displacements, forces and friction quantities keep their dimension; a mismatch is rejected (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-007** — The classifier computes mechanics only; no state, verdict or text implies acceptability, compliance or approval (`docs/CONTRACT.md` OPS-K-MECH-2; PRD §21.2).
- **REQ-008** — The active-set record carries what a report needs without parsing message text: the iteration settings, each support's state, the supports that changed, the diagnostics, and the assumptions and limitations.
- **REQ-009** — Classification follows the state-switched test and the friction law defined above.
- **REQ-010** — Convergence is judged per support class (gap, one-way, lift-off, friction) against the governed relative residual tolerance, absolute residual floor and iteration cap of the class, which DEL-09-03/REQ-007 holds and restricts. The classifier sets and relaxes none of them.
- **REQ-011** — Friction on a rotational degree of freedom is refused by name: a force limit μ·N cannot be applied as a moment.
- **REQ-012** — The four classes converge on the validation cases and report diagnostics, and the reference system's gap, lift-off and friction supports are admitted and converge (PRD FR-SOL-014, §22.2; release target exit criterion 1).
- **REQ-013** — A converged state is presented as confirmed only when the admissibility confirmation establishes it for every nonlinear support. A weakly active support, a deciding quantity that cannot be established on its state's side, or a case on which no confirmation runs leaves the state unconfirmed: the case's standing says so with a named reason, and the state is never presented as confirmed. A confirmed state with friction is one admissible state, not the only one.

**Undecided.**

- Convergence tolerances and caps for classes and model sizes beyond the measured fixture set are not yet set. Until one is measured and adopted, the affected case states that its tolerance policy is undecided.
- No friction model with history across load steps or cases is selected; one enters only by a change of scope made under the delegation in force. It matters for differences between cases: on an invented bar whose sustained force slides it and whose operating case adds cooling, operating minus sustained gives an axial-force change of 0 N as solved, against +512 N under a sequential history that sticks from the sustained position.

### Acceptance criteria

- **AC-001** — For each class and each transition (a one-way support that bears and one that releases; a gap that stays open, closes and re-opens; lift-off and re-engagement; friction that sticks, slides in each direction, and slides with a normal that changes between iterates), the state the classifier returns from the trial facts of the hand-calculation and frozen reference cases, including the gapped-support and friction-support cases of PRD §22.2, equals the reference state exactly. Agreement of the converged displacements and reactions with those references is DEL-04-07/AC-002.
- **AC-002** — The classifier's inputs are an iteration's displacements and reactions and its outputs are states, changes and a verdict. It holds no stiffness, assembles nothing and solves nothing, and the assembled iteration reaches it only through that interface.
- **AC-003** — Repeated runs from the same starting states give identical states and records. On a frictionless case with strict contact and a nonsingular reduced matrix, every run that converges reaches the one admissible state.
- **AC-004** — A case that does not settle within its cap, or whose states cycle, is reported as not converged with the supports still changing and each support's state; it is never published as converged.
- **AC-005** — A missing clearance, friction coefficient, normal or tangential reaction or trial state, a non-finite input, a negative clearance, friction coefficient or tolerance, a dimension mismatch, and friction on a rotational degree of freedom are each refused with a blocking finding that names the support. No value is defaulted.
- **AC-006** — The active-set record of a converged and of a non-converged case contains every field of REQ-008, and a report can state each support's state from the record alone.
- **AC-007** — Contact at exact touch, loss of contact, and friction beside a retained spring classify as the hand calculation requires.
- **AC-008** — No classifier output or message states or implies acceptability, compliance or approval.
- **AC-009** — The gap, lift-off and friction supports of the release target's reference system are admitted on the exact route, and each takes and reports the state of the reference system's references in the converged solve DEL-04-07's iteration reaches.
- **AC-010** — Each converged state of the confirmation references is confirmed. A converged state with a weakly active support, or with a deciding quantity moved to the wrong side of its state, is not presented as confirmed, and its standing names the reason; no case on which the confirmation did not run is presented as confirmed.

## Production and Verification Method — Praxeology

**Method choices that define the classifier.**

- The nonlinear solve is an active-set fixed-point iteration: assemble, solve, classify, update the active set. The classifier is the "classify" step: it takes an iterate's trial facts and never assembles or solves.
- Classification switches its governing quantity with the support's state.
- A sliding support carries a bounded tangential force, not a released degree of freedom, with its normal taken from the current iterate.
- Tolerances are class-tiered and tighten-only; an unmeasured entry stays undecided instead of borrowing another class's value, and the classifier sets no tolerance or default of its own.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-007, comparison with named references: each class and transition is classified from the trial facts of the hand calculations under `validation/hand_calcs/nonlinear` and of the frozen packages `validation/references/rt_a3_nonlinear` (the §22.2 cases), `kd5_nonlinear` (an exact-rational reference) and `w16_nonlinear_motion`, derived without product code, and the returned states are compared with the reference states exactly.
- **VER-002** — AC-002, executable check with judgment by a reviewer who did not write the code: the check observes the classifier's dependencies and its use by the assembled iteration; the reviewer examines whether the classifier forms or solves any system.
- **VER-003** — AC-003 to AC-006, executable check: observes repeat-run identity, convergence of the frictionless uniqueness cases from different starting states, the non-convergence report at the cap, a negative test for each missing-data refusal with its named support, and the record's fields.
- **VER-004** — AC-008, judgment by a reviewer who did not write the code: examines classifier texts against `docs/claims_registry.md`.
- **VER-005** — AC-009: the reference-system run on the exact route, with its support states compared against the reference system's frozen references.
- **VER-006** — AC-010, comparison with a named reference and executable check: the converged states and deciding quantities of `validation/references/kd5_nonlinear`, derived in exact rational arithmetic without product code, are confirmed; the check observes each committed mutation (a weakly active support, a deciding quantity on the wrong side) left unconfirmed with its named reason, and no unconfirmed case presented as confirmed.

## Governing Values and Decisions — Axiology

- **AX-001** — *The right state before a converged one.* Better is exact agreement of support states with the state law and the independent references on more support configurations, with fewer refusals. Convergence is never bought by loosening a tolerance, raising a cap, or accepting a state that violates its contact condition.
- **AX-002** — *Fewer iterations on practical models.* Better is fewer iterations to the same states. This never changes a converged state or hides a case that cycles.
- **AX-003** — *Say what was assumed, assume nothing.* A reader must be able to see each support's state and the friction law's limits in the result; a shorter record that drops them is worse. No state is admitted without being reported, a missing clearance, coefficient or reaction always fails closed, and no tolerance, coefficient or default is invented.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-012 OBJ-003 | REQ-001 REQ-002 REQ-009 REQ-012 | AC-001 | VER-001 | Outside source: hand calculations in `validation/hand_calcs/nonlinear` and the closed-form and exact-rational references `validation/references/rt_a3_nonlinear` and `kd5_nonlinear`, which import no product code |
| OUT-001 | SOW-012 OBJ-003 | REQ-002 | AC-002 | VER-002 | The method keeps assembly and the solve outside the classifier; the reviewer judges that the code keeps to it |
| OUT-001 | SOW-012 OBJ-003 | REQ-004 | AC-003 | VER-003 | Outside source for uniqueness: linear complementarity theory for the strictly complementary, nonsingular frictionless problem. Repeat-run identity is the product's own earlier output and only guards against change |
| OUT-001 | SOW-012 OBJ-003 | REQ-003 REQ-010 | AC-004 | VER-003 | DEL-09-03/REQ-007 holds the cap behaviour and `docs/CONTRACT.md` OPS-K-SOLVER-2 the report |
| OUT-001 | SOW-012 OBJ-003 | REQ-005 REQ-006 REQ-011 | AC-005 | VER-003 | PRD §5.5, `docs/CONTRACT.md` OPS-K-DATA-2 and OPS-K-UNIT-1 state the expected refusals, with hand-derived expected findings |
| OUT-001 | SOW-012 OBJ-003 | REQ-008 | AC-006 | VER-003 | The criterion states the expected fields (`docs/CONTRACT.md` OPS-K-SOLVER-2) |
| OUT-001 | SOW-012 OBJ-003 | REQ-009 | AC-007 | VER-001 | Outside source: hand calculations in `validation/hand_calcs/nonlinear` and `validation/references/w16_nonlinear_motion`, derived without product code |
| OUT-001 | SOW-012 OBJ-003 | REQ-007 | AC-008 | VER-004 | The reviewer's reading against PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-012 OBJ-003 | REQ-012 | AC-009 | VER-005 | Release target exit criterion 1 requires the admission; the expected states come from the reference system's frozen references, generated without product code |
| OUT-001 | SOW-012 OBJ-003 | REQ-013 | AC-010 | VER-006 | Outside source: `validation/references/kd5_nonlinear`, exact-rational states and deciding quantities derived without product code. The confirmation concerns the discrete model: with friction it establishes one admissible state, not uniqueness or reachability through a loading history |
