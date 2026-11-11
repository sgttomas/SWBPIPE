---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-01
package_id: PKG-09
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-026]
package_objective_refs: [OBJ-008]
---

# Scope of Work — DEL-09-01 Mechanics benchmark suite

## Purpose and Objective Traceability

This deliverable is the standing check that the solver computes frame and piping mechanics correctly. It keeps a suite of small invented models whose answers are worked out without the product (by closed form, by hand calculation, or by a generator that runs no product code), solves each one with the product, and compares the two at the governed tolerance. It also keeps the one invented reference system on which the release is examined: a single piping model that holds every component, support and analysis the first publication-ready release must handle, with the independent checks that any solve of it must satisfy.

It serves SOW-026 under OBJ-008 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-09, row DEL-09-01) and PRD §22.1, §22.2 and §22.4. In `docs/RELEASE_TARGET_v1.0.md`, exit criterion 2 names this deliverable for the reference system: the system passes closed-form or hand-calculation references frozen in `validation/references/` with an independent generator. Criteria 1, 6, 7 and 17 are examined on the reference system this deliverable maintains; their owners are other deliverables.

- **OUT-001** — The mechanics benchmark suite: invented models for cantilevers, frames, thermal growth, imposed displacement, stiffness transforms and the other solver benchmarks of PRD §22.2, each with its independently derived expected results, its derivation, its provenance record, and the comparison of the product's solve with them.
- **OUT-002** — The reference system of exit criterion 1: its specification, its generated model document, its run sheet, its named key results and its independent references.

## Deliverable Definition — Ontology

**Terms the suite implements.**

- *Benchmark case.* One invented model with its inputs and units, the result fields it expects, the derivation of each expected value, its family and its provenance record.
- *Family.* The five required families are cantilever, frame, thermal growth, imposed displacement and stiffness transform. Of the solver benchmarks PRD §22.2 lists, this suite carries twelve: cantilever with tip force; cantilever with tip moment; fixed-fixed thermal beam; simply supported pipe under weight; inclined-member coordinate transformation; L-shaped thermal expansion frame; U-loop thermal expansion; torsion-only element; rigid link; imposed anchor displacement; linear spring support; branch assembly. The gapped-support and friction-support cases are DEL-09-03's.
- *Independent reference.* Expected values obtained from a closed form, a hand calculation or a generator that imports, reads and runs no product code, frozen together with what produced them.
- *Governed tolerance.* The tolerance of a case's reference class. For the analytic class it is 1e-9 relative, with an absolute floor per quantity kind that applies near zero.
- *Provenance record.* For each case: the full provenance record of `docs/IP_AND_DATA_BOUNDARY.md` (source name, location and licence, contributor and contributor certification, redistribution status and review disposition), with the case's assumptions, units and expected result fields.
- *Provenance index.* One list of every case with its provenance record.

**Terms the reference system implements.**

- *Reference system.* One invented piping system that runs from equipment nozzle A to equipment nozzle B with a branch to nozzle C. It contains pressure, realized bends, a tee with user modifiers, an eccentric reducer, a valve and flanges, and an expansion joint with tie rods; linear restraints, a gap, a lift-off support, a friction support, a variable spring and a constant-effort support. Its analyses are sustained, operating, expansion with its basis, hydrotest, wind, three-axis static seismic with SRSS and absolute-sum combination, seismic anchor movement, modal, response spectrum and hanger sizing. It is authored for the exact pressure route. Every value is invented and its section sizes are deliberately non-standard; it is a test object, not a design.
- *Specification.* The single statement of the system's geometry, stable identities, materials, sections, components, supports, pressure regions, load cases, combinations, analysis requests, key results and pending ledger. Entities have stable identities so that a round trip can compare them.
- *Fixture.* The model document produced by the generator from the specification, with the record of what was authored.
- *Key result.* A named result of the system that consumers read and report: anchor reactions; the largest translation; spring and constant-effort loads; nonlinear support status; tee intensified stress rows; tie-rod and joint pressure forces; the hydrotest marking with held supports; frequencies and mass participation; spectrum reactions and displacements; the hanger table.
- *Reference check.* A statement any correct solve of the system must satisfy without an invented expected value: vertical reactions sum to the total weight; the horizontal reaction resultant is zero and the reaction moment balances the weight's moment; the expansion combination equals operating minus sustained on every signed component row; SRSS and absolute-sum combinations equal their definitions over the operand rows; the anchor-movement difference is self-equilibrated; the wind and seismic reaction differences equal minus the generated load resultants; the spectrum directional combination equals its recomputation from the published per-direction results.
- *Pending ledger.* The list of each family or analysis of exit criterion 1 that the fixture does not yet contain or the exact route does not yet solve, each with the refusal or absence a solve of the full system shows.
- *Run sheet.* The ordered entry of the system from a blank project through the desktop's surfaces, with its entry counts and time estimate.

**Boundary.** The suite observes the solver and implements none of it.

- Whether a mechanics capability is correct is a criterion of the deliverable that owns it (DEL-04-01, DEL-04-02, DEL-04-03, DEL-05-01, DEL-05-05, DEL-04-07). This deliverable owns the cases, their references and the standing comparison.
- Stress recovery benchmarks are DEL-09-02's. Nonlinear support cases are DEL-09-03's. The manual pages that present cases are DEL-09-04's. Which suites and cases gate a release is DEL-09-05's.
- For the reference system, this deliverable owns the specification, the fixture, the run sheet, the key-result names and the reference checks. Solving every family on the exact route without refusal belongs to the owners exit criterion 1 names; each analysis's own closed-form references belong to that analysis's deliverable; the MBF round trip is DEL-17-04's; entry within the authoring time is DEL-07-02's and DEL-07-06's; building it by the embedded agent is DEL-16-06's. Correlation with an external prover is DEL-17-05's.
- Reference packages under `validation/references/` belong to the deliverables whose criteria they ground. This deliverable holds `w9_benchmarks`, `w16_imposed_motion`, `t3_r1` and `reference_system`.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The suite is separate from the solver. It reaches the solver only through the solver's public interfaces and changes no production behaviour (decomposition AB-00-02).
- **REQ-002** — The suite covers the cantilever, frame, thermal-growth, imposed-displacement and stiffness-transform families (`docs/VALIDATION_STRATEGY.md` §2).
- **REQ-003** — Every case and every derivation is original, public-domain or permissively licensed, with a provenance record. No protected standards example, code text or table, proprietary commercial benchmark or vendor or private project data enters the suite (SOW-026; `docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3). A case is admitted only when its provenance record carries an accepted review disposition. Content whose redistribution rights or protected status is uncertain is quarantined for the human or legal review of `docs/IP_AND_DATA_BOUNDARY.md`; the independent reviewer's judgment of VER-004 does not stand in for that review.
- **REQ-004** — Benchmark inputs and expected values are unit-aware and dimensionally checked: each carries a unit of the project's unit catalog and is checked through that catalog (`docs/CONTRACT.md` OPS-K-UNIT-1). A unit declared only inside a case does not meet this requirement.
- **REQ-005** — A run keeps what the solve returned: per case, each expected value beside the observed value with its unit, the solver version, the diagnostics and status of the solve, and the case's assumptions, limitations and provenance (decomposition AB-00-06). A case can be run by name through the headless runner's benchmark workflow, and an expected value that a run cannot observe is reported as unobserved and fails its case.
- **REQ-006** — Comparison uses the governed tolerance of the case's reference class. A case may tighten the governed value and never loosens it. The absolute floors of quantity kinds that have not been measured are not decided; a comparison that needs one reports that in a named diagnostic and does not pass on an invented floor.
- **REQ-007** — The suite is verification of software behaviour. No case, report or label states or implies certification, code compliance, professional approval or suitability for a project (PRD §21.2).
- **REQ-008** — A provenance index lists every case with its provenance record.
- **REQ-009** — *Numerical quality.* The suite holds cases that probe the solver's numerical behaviour: mechanisms and unrestrained bodies, weak and stiff springs, long chains, cancellation, extreme stiffness ratios, skewed and non-finite inputs. Their expected outcomes come from exact-arithmetic references. A case the solver cannot resolve must be refused or reported unresolved, and is never counted as a pass (SOW-026; PRD §12.8).
- **REQ-010** — The suite includes each of the twelve PRD §22.2 solver benchmarks named above, each with an independent reference (PRD §22.2).
- **REQ-011** — *Reference system.* One invented system holds every family and analysis exit criterion 1 lists, as defined above. The specification is its single statement: geometry, identities and invented inputs change in the specification first, and the fixture is the generator's output from it.
- **REQ-012** — A key result has an expected value only from a closed-form or hand-calculation reference with an independent generator, from an exact identity between published quantities, or from a solve of the system frozen with its basis recorded. A value frozen from a solve only guards against change. No expected value is entered otherwise, and none is changed to silence a failure.
- **REQ-013** — Every family or analysis of exit criterion 1 is either in the fixture and solved on the exact route, or in the pending ledger. A pending family keeps its place and identities in the specification, and no family is dropped from the system to make a criterion pass. At release, every row still pending has an entry in `docs/RELEASE_LIMITATIONS_v1.0.md` (exit criterion 3).
- **REQ-014** — The run sheet enters the system from a blank project in an order a person can follow, states its entry counts and time estimate, and lists every difference between what it enters and the fixture (exit criterion 7).

### Acceptance criteria

- **AC-001** — Every case states its inputs and expected values with a unit and dimension of the project's unit catalog, and a case with a missing unit, a dimension mismatch or no provenance record fails. A run reports, per case, every expected value beside its observed value with units, the solver version, the diagnostics and status the solve returned, and the case's assumptions and limitations; run by name through the headless runner, no expected value is reported unobserved.
- **AC-002** — The committed fixture equals the generator's output and validates as the current model document. Each family and analysis exit criterion 1 lists is either in the fixture and solved on the exact route without refusal, or recorded in the pending ledger with the refusal or absence a solve shows, and the generator's pending flags agree with that ledger. No expected key-result value is recorded without its basis.
- **AC-003** — The suite holds at least one case in each of the five families and each of the twelve PRD §22.2 benchmarks; a suite missing a family or a benchmark fails.
- **AC-004** — For every case, the displacements, rotations, reactions and member end actions the product solves agree with the case's independent reference at the governed tolerance, in both solver modes. No case compares at a tolerance looser than the governed one.
- **AC-005** — Every expected value has a derivation that imports, reads and runs no product code. Every case has a provenance record with an accepted review disposition, the provenance index lists every case, and no case, derivation or fixture contains protected or proprietary content.
- **AC-006** — The suite's code lives in validation and test locations and links the solver only through its public interfaces; the product builds and solves identically with the suite absent.
- **AC-007** — Each numerical-quality case ends as its exact reference requires: a row within the predicate passes, a mechanism or inadmissible input is refused, and a case beyond the solver's range ends unresolved with no rows. Every negative control fails the predicate it was built to fail, and no unresolved or uncovered row is counted as a pass.
- **AC-008** — No case, report, label or output of the suite or the reference system states or implies certification, code compliance, approval or suitability for a project.
- **AC-009** — On the solved fixture, in both solver modes, every reference check that its cases admit holds against the independent references, and no key-result value frozen from a solve is presented as evidence that the value is correct.
- **AC-010** — The run sheet names every entity and value of the specification that the desktop can enter, in an order that can be followed, with its counts and estimate; every difference between the model it yields and the fixture is listed in it.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- Cases start with models a reviewer can check by hand; a case is broadened only while its derivation stays reviewable.
- A case's expected values are frozen with the derivation or generator that produced them, so a change to either is visible.
- The reference system's weight loads are the product's own self-weight generation for each case, captured; the fixture generator computes no weight. The independent reference recomputes the total weight and its moment from the specification's inputs without that capture.
- Occasional effects on the reference system are cases that include the sustained loads, and each occasional term is a difference from the sustained case, because with nonlinear supports every static case has its own support status.
- The generator keeps one pending flag per ledger row, and the fixture is regenerated when a row changes.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-003 and AC-006, executable check: observes each case's units against the unit catalog, its provenance record, the family and benchmark coverage of the inventory, the per-value report of a run through the headless runner, and the suite's dependencies on the solver.
- **VER-002** — AC-004, comparison with named references: the hand calculations in `validation/hand_calcs/mechanics` and the frozen packages `validation/references/w9_benchmarks` and `w16_imposed_motion`, whose generators import no product code, at the governed tolerance in both solver modes.
- **VER-003** — AC-007, comparison with named references: the 100-digit references of `validation/benchmarks/numerical_integrity`, the exact references of `validation/references/t3_r1`, and expectations frozen before any product comparison for the product-route regressions; the check observes each row's outcome class and each negative control.
- **VER-004** — AC-005 and AC-008, judgment by a reviewer who did not write the case: examines each derivation for independence from product code, each provenance record with its review disposition and the index, and the suite's texts against `docs/claims_registry.md`; the protected-content scan runs over the suite's sources, notes and fixtures.
- **VER-005** — AC-002, AC-009 and AC-010, executable check with a comparison: regenerates the fixture and compares it with the committed file, validates it against the model schema, compares the specification's ledger with the generator's flags, solves the fixture through the headless runner in both modes and tests the reference checks against `validation/references/reference_system`; the run sheet is compared with the specification and the fixture.

## Governing Values and Decisions — Axiology

- **AX-001** — *Independence.* Better is a larger share of expected values that rest on a reference sharing nothing with the product; the target is all of them. An expected value is never taken from the product to fill a gap or changed to make a comparison pass.
- **AX-002** — *Closer agreement.* Better is a smaller largest deviation from the references and a tighter case tolerance. It is never bought by loosening a governed value, dropping a row or tuning the product to a case.
- **AX-003** — *Fewer pending rows.* Better is fewer rows in the reference system's pending ledger; the target is none. A row is never cleared by removing its family from the system or by recording an expected value without its basis.
- **AX-004** — *Clean sources.* Better is an original invented case, even where a protected example is familiar. Coverage is never bought with a protected, proprietary or unprovenanced source.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-026 OBJ-008 | REQ-004 REQ-005 | AC-001 | VER-001 | Units are compared with the project's unit catalog, and a run's report with the content the decomposition's AB-00-06 says a run keeps; the suite wrote neither |
| OUT-002 | SOW-026 OBJ-008 | REQ-011 REQ-012 REQ-013 | AC-002 | VER-005 | Exit criterion 1 lists the families and analyses and the specification states the system; the committed fixture is compared with the generator's output and the model schema |
| OUT-001 | SOW-026 OBJ-008 | REQ-002 REQ-010 | AC-003 | VER-001 | The suite's inventory is compared with the five families of `docs/VALIDATION_STRATEGY.md` §2 and the benchmarks of PRD §22.2 |
| OUT-001 | SOW-026 OBJ-008 | REQ-006 REQ-010 | AC-004 | VER-002 | Outside source: closed-form frame mechanics by hand calculation and the reference packages `w9_benchmarks` and `w16_imposed_motion`, generated without product code; the tolerance is the governed value of the case's reference class |
| OUT-001 | SOW-026 OBJ-008 | REQ-003 REQ-008 | AC-005 | VER-004 | Each case and record is compared with what `docs/IP_AND_DATA_BOUNDARY.md` allows a public case to contain, and independence is judged by a reviewer who did not write the case. That judgment is evidence for the criterion; it is not the review disposition the policy requires to admit content |
| OUT-001 | SOW-026 OBJ-008 | REQ-001 | AC-006 | VER-001 | The decomposition's AB-00-02 states the separation; the product's build and results without the suite are the comparison |
| OUT-001 | SOW-026 OBJ-008 | REQ-009 | AC-007 | VER-003 | Outside source: exact-arithmetic and 100-digit references generated without product code, and expectations frozen before product comparison |
| OUT-001 | SOW-026 OBJ-008 | REQ-007 | AC-008 | VER-004 | The suite's texts are compared with PRD §21.2 and `docs/claims_registry.md` by a reviewer who did not write them |
| OUT-002 | SOW-026 OBJ-008 | REQ-012 | AC-009 | VER-005 | Outside source: total weight and its moment recomputed from the specification by a generator that imports no product code, and exact identities between published rows; a key-result value frozen from a solve is the product's own earlier output and only guards against change |
| OUT-002 | SOW-026 OBJ-008 | REQ-014 | AC-010 | VER-005 | Exit criterion 7 and the specification state what must be entered; the run sheet's model is compared with the fixture |
