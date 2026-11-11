---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-03
package_id: PKG-09
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-026]
package_objective_refs: [OBJ-008]
---

# Scope of Work — DEL-09-03 Nonlinear support regression suite

## Purpose and Objective Traceability

This deliverable is the standing guard on nonlinear support behaviour. A one-way support, a gap, a lift-off support and a friction support each change state as loads change, and the solver must find the right state, settle within its governed limits, say so plainly when it cannot, and give the same answer every time. The suite keeps small invented cases whose settled states and values are worked out by hand, runs each through the product's nonlinear solve, and compares states exactly and values at the governed tolerance, so that a change to the solver that moves any of them is caught.

It serves SOW-026 under OBJ-008 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-09, row DEL-09-03), the gapped-support and friction-support benchmarks of PRD §22.2, the nonlinear convergence tests of PRD §22.4, and the PRD §24 R5 exit criterion that nonlinear cases converge or report actionable diagnostics. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable; it guards the gap, lift-off and friction behaviour that exit criterion 1 requires of the reference system's supports.

- **OUT-001** — The nonlinear support regression suite: invented cases for active-set, gap, lift-off, friction, convergence and non-convergence behaviour, each with its hand-derived expected support states and values, its provenance record and the governed convergence limits it is judged by, and the comparison of the product's nonlinear solve with them, repeated to show that the result does not change.

## Deliverable Definition — Ontology

**Terms the suite implements.**

- *Regression case.* One invented model with nonlinear supports, its loads and units, the observations it expects, their derivation, its family and its provenance record.
- *Family.* Active set (a one-way support that bears or releases), gap (closure and opening), lift-off, friction (sticking and sliding), and non-convergence (a case built not to settle). Mixed cases combine several supports, classes and degrees of freedom.
- *Support state.* Active or inactive, gap open or closed, in contact or lifted, sticking or sliding. The states and the law that selects them are the product's (DEL-04-04); the suite uses them and defines none.
- *Expected observation.* For a case: the settled state of each support, the displacements and reactions, the iteration count, the convergence verdict and the diagnostics.
- *Convergence limits.* Convergence is judged per class: one-way, gap, lift-off, friction, and cases with several supports and degrees of freedom. The residual is the number of supports whose state changed in the final iteration. It must be zero, within at most four iterations (DEC-046).
- *Governed record.* The machine-readable record of the convergence limits and the cases each applies to, kept with the suite.
- *Independent reference.* The settled states and values obtained by hand calculation with invented inputs, importing and running no product code.

**Boundary.** The suite observes the nonlinear solve and implements none of it.

- The classifier, its state and friction law and its verdict are DEL-04-04's. The assembled nonlinear iteration is DEL-04-07's. The diagnostics are DEL-04-06's. Whether each is correct is a criterion of its own deliverable; this deliverable owns the cases, their references, the governed record and the standing comparison.
- The exact references for the product route's converged active set (`validation/references/kd5_nonlinear`, `rt_a3_nonlinear`) belong to the deliverables whose criteria they ground.
- Comparison of nonlinear results with an external prover is DEL-17-05's.
- Mechanics benchmarks are DEL-09-01's, stress recovery benchmarks DEL-09-02's, the manual pages DEL-09-04's, and which suites gate a release DEL-09-05's.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The suite covers active-set, gap, friction and lift-off behaviour, convergence and non-convergence, and regression of each (SOW-026, SOW-012; PRD §11.4 FR-SOL-014, §22.2, §22.4, §24 R5).
- **REQ-002** — Every case and derivation is public, original or permissively licensed, with a provenance record (SOW-026; `docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3, OPS-K-DATA-1 to OPS-K-DATA-3).
- **REQ-003** — The suite contains no protected standards data or example, no code-derived table, no proprietary component value and no commercial benchmark case or commercial software output without redistribution rights.
- **REQ-004** — A run observes, per case, each support's state, gap, lift-off and friction state, the limits applied, the iteration count, and any non-convergence warning or failure the product returned (`docs/CONTRACT.md` OPS-K-SOLVER-2; decomposition AB-00-06). A case can be run by name through the headless runner's regression workflow.
- **REQ-005** — A regression comparison is deterministic: the same model, units and solver version give the same observations on every run (OBJ-008; `docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-006** — Case definitions, inputs and expected observations are unit-aware and dimensionally checked through the project's unit catalog (DEC-018; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-007** — Every converging case is judged by the convergence limits of its class. A case may tighten a limit and never loosens one; loosening a limit or raising the iteration cap is an owner decision (DEC-046). Limits on the other residuals (the force and moment equilibrium of free degrees of freedom, residual work, and the change of displacement and reaction between iterations) are governed only for the cases they were measured on. For any other case they are not decided, and a comparison that needs one reports that in a named diagnostic.
- **REQ-008** — A regression outcome is software verification evidence only. No case, report or label states or implies code compliance, certification, sealing, approval or professional reliance (PRD §21.2).

### Acceptance criteria

- **AC-001** — Every case states its inputs and expected observations with a unit and dimension of the project's unit catalog and has a provenance record. A run reports, per case, each support's state, the gap, lift-off and friction state, the limits applied, the iteration count, the verdict and the diagnostics; run by name through the headless runner, no expected observation is reported unobserved.
- **AC-002** — The suite holds at least one case in each of the active-set, gap, lift-off, friction and non-convergence families, the gap and friction families holding the gapped-support and friction-support benchmarks of PRD §22.2; a suite missing one fails.
- **AC-003** — For every converging case, the settled state of every support equals the hand-derived state exactly, and the displacements and reactions agree with the hand calculation at the governed tolerance of its reference class (DEC-026).
- **AC-004** — Every converging case meets the convergence limits of its class: no support changes state in the final iteration, within the iteration cap. A case built not to settle is reported as not converged, with the supports still changing and a failure-level diagnostic, and is never reported as converged. No case applies a limit looser than the governed record.
- **AC-005** — Repeated runs of each case give identical support states, iteration counts, values and diagnostics.
- **AC-006** — Every expected observation has a derivation that imports and runs no product code, and no case, derivation or fixture contains anything REQ-003 excludes.
- **AC-007** — No case, report, label or output states or implies code compliance, certification, approval or professional reliance.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- Cases are small and invented, so that each settled state can be reasoned out by hand before the product is run.
- The suite examines states and diagnostics as well as magnitudes: a case passes only if the supports end in the expected states.
- The convergence limits are read from the governed record, not written into a case.
- Each case is run more than once in a check, so that a result that depends on run order or history fails.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002 and AC-004, executable check: observes each case's units against the unit catalog and its provenance record, the family coverage of the inventory, each run's reported states, iteration count, verdict and diagnostics against the governed record's limits, the outcome of the non-convergence case, and a run through the headless runner.
- **VER-002** — AC-003, comparison with named references: the hand calculations in `validation/hand_calcs/nonlinear`, which import no product code; support states are compared exactly and values at the governed tolerance.
- **VER-003** — AC-005, executable check: runs each case repeatedly and compares every observation of each run with the first.
- **VER-004** — AC-006 and AC-007, judgment by a reviewer who did not write the case: examines each derivation for independence from product code, each provenance record, and the suite's texts against REQ-003 and `docs/claims_registry.md`; the protected-content scan runs over the suite's sources, notes and fixtures.

## Governing Values and Decisions — Axiology

- **AX-001** — *Intended behaviour, not incidental behaviour.* Better is a larger share of expected observations derived by hand before the product is run; the target is all of them. An expected state or value is never copied from the product's output to freeze whatever it does today.
- **AX-002** — *Sharper detection.* Better is a case that fails on a smaller change of state, iteration count or value. Detection is never bought with a broad acceptance band or a limit looser than the governed record.
- **AX-003** — *Honest non-convergence.* Better is fewer cases, other than those built not to settle, that end unsettled, and a non-convergence reported with more of its cause: the supports still changing and their states. That count is never reduced by removing a non-convergence case, raising a cap, loosening a limit or reporting an unsettled case as converged.
- **AX-004** — *Clean sources.* Better is an original invented case that tests mechanics and diagnostics directly. Coverage is never bought with a protected example or a commercial program's result.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-026 OBJ-008 | REQ-004 REQ-006 | AC-001 | VER-001 | Owner ruling: DEC-018 selects the unit catalog and `docs/CONTRACT.md` OPS-K-SOLVER-2 names what a nonlinear case reports; units are compared with the catalog, which the suite did not write |
| OUT-001 | SOW-026 OBJ-008 | REQ-001 | AC-002 | VER-001 | Owner ruling: PRD FR-SOL-014 and §24 R5 name the support families and require that a case converges or reports why it did not, and §22.2 names the gapped-support and friction-support benchmarks |
| OUT-001 | SOW-026 OBJ-008 | REQ-001 | AC-003 | VER-002 | Outside source: hand calculation of each settled state and its values in `validation/hand_calcs/nonlinear`, which imports no product code; the tolerance is an owner ruling (DEC-026) |
| OUT-001 | SOW-026 OBJ-008 | REQ-007 | AC-004 | VER-001 | Owner ruling: DEC-046 states the convergence limits, and the case definition says which case must not settle |
| OUT-001 | SOW-026 OBJ-008 | REQ-005 | AC-005 | VER-003 | The product's own earlier output, which only guards against change; correctness rests on AC-003 |
| OUT-001 | SOW-026 OBJ-008 | REQ-002 REQ-003 | AC-006 | VER-004 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md` states what a public case may contain; independence is judged by a reviewer who did not write the case |
| OUT-001 | SOW-026 OBJ-008 | REQ-008 | AC-007 | VER-004 | Owner ruling: PRD §21.2 and `docs/claims_registry.md` |
