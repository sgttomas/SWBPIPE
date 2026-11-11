---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-02
package_id: PKG-09
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-026]
package_objective_refs: [OBJ-008]
---

# Scope of Work — DEL-09-02 Stress recovery benchmark suite

## Purpose and Objective Traceability

This deliverable is the standing check that stress recovery turns member forces, moments and pressure into the right fundamental stresses. It keeps a suite of small invented cases whose stresses are worked out by hand from elementary mechanics, recovers the same stresses with the product, and compares the two at the governed tolerance. Fundamental stresses are among the results an engineer publishes from SWBPIPE, so a wrong sign, axis or section term must be caught here before it reaches a report.

It serves SOW-026 under OBJ-008 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-09, row DEL-09-02) and PRD §22.1, §22.3 and §12.7. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable; it guards the fundamental stresses that the release target's published-results ruling lets the user's engineer issue.

- **OUT-001** — The stress recovery benchmark suite: invented cases for axial, bending, torsional, pressure-derived and stress-range behaviour, each with its hand-calculated expected stresses, its derivation, its provenance record, and the comparison of the product's recovered stresses with them.

## Deliverable Definition — Ontology

**Terms the suite implements.**

- *Benchmark case.* One invented set of resultants or loads, section properties and, where pressure acts, a pressurized straight pipe, with the stresses it expects, the derivation of each, and its provenance record.
- *Behaviour.* The five behaviours the suite covers, as PRD §22.3 words them: axial stress from axial force; bending stress from moment; torsional shear from torque; pressure-derived hoop stress and pressure-derived longitudinal stress; stress range from two states.
- *Stress component.* A named fundamental stress with its sign convention. The names and signs are those of the product's stress recovery (DEL-05-03); the suite uses them and defines none.
- *Stress range.* The component-wise absolute difference of the recovered components of two result states, as DEL-05-03 defines it. It is a mechanics difference. It is not a fatigue check, an allowable comparison or a code stress category.
- *Independent reference.* Expected stresses obtained from a closed form by hand calculation with invented inputs, importing and running no product code.
- *Calculation witness.* A hand calculation kept in machine-readable form. Where a case has one it is the source of record, and its human-readable renderings are generated from it.
- *Governed tolerance.* The tolerance of the case's reference class (DEC-026): for the analytic class, 1e-9 relative with an absolute floor per quantity kind near zero.

**Boundary.** The suite observes stress recovery and implements none of it.

- Stress recovery, its component names, signs and section bases are DEL-05-03's. Section properties are DEL-03-08's. The straight-pipe element that turns a case's member loads into end and station resultants is DEL-04-02's. The solve entry and the exact pressure route are DEL-04-07's. Whether each is correct is a criterion of its own deliverable; this deliverable owns the cases, their references and the standing comparison.
- The suite contains no code stress equation, allowable, stress category, stress index, SIF or flexibility table and no fatigue criterion. Code-specific stress formulas are evaluated only through user rule packs (PRD §12.7).
- Pressure cases are solved only through the current exact pressure contract.
- Mechanics benchmarks are DEL-09-01's, nonlinear support cases DEL-09-03's, the manual pages DEL-09-04's, and which suites gate a release DEL-09-05's.
- The user-entered SIF multiplier and flexibility-factor behaviours of PRD §22.3 are examined by DEL-03-03 and DEL-03-04; this suite covers the other six items of that list, as the five behaviours above.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The suite covers axial, bending, torsion, pressure and stress-range behaviour (PRD §22.3; `docs/VALIDATION_STRATEGY.md` §2).
- **REQ-002** — Every case and derivation is original, public-domain or permissively licensed, and carries the full provenance record of `docs/IP_AND_DATA_BOUNDARY.md` (source name, location and licence, contributor and contributor certification, redistribution status and review disposition), as SOW-026 and `docs/CONTRACT.md` OPS-K-IP-2 require.
- **REQ-003** — The suite excludes protected standards text and examples, copied code formulas, protected tables, proprietary data, material allowables and code stress equations (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3, OPS-K-RULE-1).
- **REQ-004** — The suite verifies mechanics stress recovery only. It decides no code compliance and no fatigue acceptability, and no case, report or label states or implies certification, sealing or professional approval (PRD §21.2).
- **REQ-005** — Inputs, expected stresses, comparisons and diagnostics are unit-aware and dimensionally checked through the project's unit catalog (DEC-018; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-006** — A case that lacks a source, a unit, a sign convention, the pair of states of a range, a result field or a tolerance basis says so in a finding. Nothing is defaulted (`docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-007** — Comparison uses the governed tolerance of the case's reference class. A case may tighten the governed value and never loosens it (DEC-026). The absolute floors of quantity kinds that have not been measured are not decided, as DEL-09-01/REQ-006 states.
- **REQ-008** — The suite reaches stress recovery and the solve only through their public interfaces, and a run keeps the diagnostics and result-envelope content the product returned and reports, per case, each expected stress beside the recovered stress (decomposition AB-00-02, AB-00-06). A case can be run by name through the headless runner's benchmark workflow.

### Acceptance criteria

- **AC-001** — Every case states its inputs and expected stresses with a unit and dimension of the project's unit catalog, names each stress component and its sign convention, and has a provenance record. A case missing any item of REQ-006 fails with a finding that names what is missing, and no comparison proceeds on a default.
- **AC-002** — The suite holds at least one case for each of: axial stress from axial force, bending stress from moment, torsional shear from torque, pressure-derived hoop stress, pressure-derived longitudinal stress, and stress range from two states. A suite missing one fails.
- **AC-003** — For every case, the stresses the product recovers agree with the hand-calculated values at the governed tolerance, component by component and with the stated sign; the pressure cases are solved on the exact pressure route; and a stress range equals the component-wise absolute difference of the two states' recovered components. No case compares at a tolerance looser than the governed one.
- **AC-004** — Every expected stress has a derivation that imports and runs no product code. A calculation witness evaluates its closed forms without product code, and its committed renderings equal what is generated from it. No case, derivation or fixture contains anything REQ-003 excludes.
- **AC-005** — The output of a complete run validates as a result envelope: each stress value carries its unit, its component name and the link to the recovery it came from, with the diagnostics and the mechanics-only status the product returned. A run reports, for each case, each expected stress beside the recovered stress with its unit and the outcome of the comparison. Run by name through the headless runner, no expected stress is reported unobserved.
- **AC-006** — No case, report, label or output states or implies certification, code compliance, fatigue acceptability or approval, and no stress range is presented as a fatigue or allowable check.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- Cases are small, deterministic and synthetic: invented resultants, or invented member loads on a single invented annulus, so that each expected stress follows from stated closed forms.
- A case's expected stresses are frozen with the hand calculation that produced them.
- Where a hand calculation is kept as a calculation witness, comparisons and reviews bind to the witness, never to a manually edited rendering (`docs/VALIDATION_STRATEGY.md` §2).
- The pressure cases drive the product's exact-pressure solve of a straight pipe; they do not call a pressure formula of their own in the product.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002 and AC-005, executable check: observes each case's units against the unit catalog, its component names, sign conventions and provenance record, the behaviour coverage of the inventory, the finding raised for each missing item, and the validation of a run's output as a result envelope, including a run through the headless runner.
- **VER-002** — AC-003, comparison with named references: the hand calculations in `validation/hand_calcs/stress`, which import no product code, at the governed tolerance.
- **VER-003** — AC-004 and AC-006, judgment by a reviewer who did not write the case: examines each derivation and witness for independence from product code, each provenance record, and the suite's texts against REQ-003 and `docs/claims_registry.md`; the protected-content scan runs over the suite's sources, notes and fixtures, and the witness's renderings are regenerated and compared.

## Governing Values and Decisions — Axiology

- **AX-001** — *Independence.* Better is a larger share of expected stresses that a reviewer can recompute by hand from the stated inputs; the target is all of them. An expected stress is never taken from the product or changed to make a comparison pass.
- **AX-002** — *Mechanics only.* Better is fewer places where a reader could take a benchmark for a code check. A case is never made more useful by adding a code stress category, an allowable or a fatigue criterion.
- **AX-003** — *Closer agreement.* Better is a smaller largest deviation from the hand calculations and a tighter case tolerance. It is never bought by loosening a governed value or dropping a component.
- **AX-004** — *Clean sources.* Better is an original invented case. Coverage is never bought with a protected, proprietary or unprovenanced source.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-026 OBJ-008 | REQ-005 REQ-006 | AC-001 | VER-001 | Owner ruling: DEC-018 selects the unit catalog and `docs/CONTRACT.md` OPS-K-DATA-2 states the expected finding; units are compared with the catalog and component names with DEL-05-03's, neither written by the suite |
| OUT-001 | SOW-026 OBJ-008 | REQ-001 | AC-002 | VER-001 | Owner ruling: PRD §22.3 and `docs/VALIDATION_STRATEGY.md` §2 state the required behaviours |
| OUT-001 | SOW-026 OBJ-008 | REQ-001 REQ-007 | AC-003 | VER-002 | Outside source: closed-form stress mechanics by hand calculation in `validation/hand_calcs/stress`, which imports no product code; the tolerance is an owner ruling (DEC-026) |
| OUT-001 | SOW-026 OBJ-008 | REQ-002 REQ-003 | AC-004 | VER-003 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md` and `docs/CONTRACT.md` OPS-K-IP-1 state what a public case may contain; independence is judged by a reviewer who did not write the case |
| OUT-001 | SOW-026 OBJ-008 | REQ-008 | AC-005 | VER-001 | Owner ruling: the decomposition's AB-00-06 names what an envelope carries; the output is validated by the result-export validator, which the suite did not write, and the expected stresses it reports are the hand-calculated values of AC-003 |
| OUT-001 | SOW-026 OBJ-008 | REQ-004 | AC-006 | VER-003 | Owner ruling: PRD §21.2 and `docs/claims_registry.md` |
