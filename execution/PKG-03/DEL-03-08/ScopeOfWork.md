---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-08
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-051,SOW-018]
package_objective_refs: [OBJ-004, OBJ-012]
---

# Scope of Work — DEL-03-08 Pipe section property and mass-property calculator

## Purpose and Objective Traceability

This deliverable is the product's definition of a pipe's section and mass properties, and the calculator that computes them. From an outside diameter, a wall thickness and, where the user gives them, a corrosion allowance, a mill tolerance, an insulation thickness and densities, it gives the metal area, second moment, section modulus, torsion constant, bore volume and mass per length that stiffness, self-weight and stress recovery are built on. It is the reference the solve's own section and mass computation is held to, so that every number downstream starts from the user's dimensions and from no table.

It serves SOW-051 and SOW-018 under OBJ-004 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-08), and PRD §18.2 with the unit-safety principle of §5.6 and the no-silent-defaults principle of §5.5. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable; it reaches exit criteria 1 and 2 through DEL-04-07, whose section and mass adapter is compared with it.

- **OUT-001** — The pipe section and mass property calculator: its definitions, its inputs and outputs with their units and provenance, and its refusals.
- **OUT-002** — The reference contract for the product's section and mass adapter: the definitions and values that adapter is compared with.

## Deliverable Definition — Ontology

**Terms the calculator implements.**

- *Inputs.* Outside diameter and wall thickness are required. Corrosion allowance, mill tolerance, insulation thickness, and material, contents and insulation densities are optional. Each is a quantity: a magnitude with its unit, its dimension and its provenance.
- *Effective wall.* The wall thickness less the corrosion allowance and less the mill tolerance, each where given. An absent allowance or tolerance means no reduction; it is not a zero the calculator supplied.
- *Section.* A circular annulus with outside diameter D and inside diameter d, where d is D less twice the effective wall.
- *Section properties.* Metal area π(D² − d²)/4. Second moment of area π(D⁴ − d⁴)/64, the same about every diameter. Section modulus, the second moment divided by D/2. Torsion constant π(D⁴ − d⁴)/32. Contents volume per length πd²/4.
- *Mass per length.* Metal: metal area times material density. Contents: bore area times contents density. Insulation: the area of the annulus between D and D plus twice the insulation thickness, times insulation density. Each part is given only when its density, and for insulation its thickness, is given; the total is the sum of the parts that are given.
- *Finding.* A blocking refusal with code, class, severity, the field it concerns, source, affected object, message, remediation and provenance. A calculation with a finding returns no property.

**Boundary.** This deliverable computes the properties of the annulus it is given. Which wall a published result is computed on (the nominal wall, or the wall reduced by mill tolerance and corrosion allowance) is the definition of the two section bases, which DEL-05-03 owns. The product's own section and mass computation inside the solve is DEL-04-07's adapter; DEL-04-07 owns its conformance to this contract and the assessment of any difference. DEL-05-01 generates self-weight from mass per length. DEL-02-02 owns units and conversion. DEL-03-02 owns the section library format, and no dimensional table belongs to either. The calculator contains no solver, no stress formula and no rule check.

**Not decided.** Whether this contract takes each property on the wall the solve takes it on. Here every property and every mass is computed on one effective wall. On the exact route the solve takes stiffness and metal mass on the nominal wall and contents mass on the bore reduced by mill tolerance and corrosion allowance, under the section bases DEL-05-03 defines. Until it is decided, AC-002 keeps the difference visible and this contract is unchanged.

## Completion and Reliance Basis — Epistemology

A calculated property is geometry and arithmetic on the user's numbers. It carries no judgment that the dimensions are right for a pipe or that a tolerance or allowance is appropriate.

### Requirements

- **REQ-001** — The calculator uses only explicit values the user entered or lawfully imported: dimensions, densities, corrosion allowance, mill tolerance and insulation (SOW-051; `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-IP-1).
- **REQ-002** — A missing required value is refused with a named finding. The calculator supplies no default, and an absent optional value contributes nothing (SOW-051; `docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-003** — Every input, intermediate and output is unit-aware and dimensionally checked. A unit is checked against the unit catalog's dimension for that input, and inputs of one dimension given in different units are converted through the unit catalog or refused by name; magnitudes in different units are never combined as bare numbers (SOW-051; `docs/CONTRACT.md` OPS-K-UNIT-1; PRD §5.6).
- **REQ-004** — Provenance and redistribution status travel with the values: every input must carry its provenance, and every output carries the provenance and redistribution status of the inputs it was derived from, with the statement that it was calculated (SOW-018; `docs/CONTRACT.md` OPS-K-DATA-3).
- **REQ-005** — No public artifact of this deliverable encodes a protected pipe dimensional table, protected material data or proprietary data. Example and test values are invented (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3; SOW-051).
- **REQ-006** — Calculator outputs are mechanics quantities only. They carry no code-compliance, certification or approval meaning (PRD §21.2).
- **REQ-007** — The calculator is separate from the global solver and from rule evaluation (the decomposition's PKG-03 exclusion).
- **REQ-008** — The definitions above are the contract the product's section and mass adapter is compared with: the same explicit inputs give the same properties, with their units.

### Acceptance criteria

- **AC-001** — For a circular pipe, the calculated metal area, second moment, section modulus, torsion constant, contents volume per length and each mass per length equal the closed-form annulus values of the definitions above, for the nominal wall and for walls reduced by a corrosion allowance, by a mill tolerance and by both. Each output carries its unit and dimension. With no density given, no mass is returned; with some given, the total is the sum of exactly those parts.
- **AC-002** — For every case of a shared set of outside diameter, wall, mill tolerance, corrosion allowance and densities, the calculator returns the area, second moment, torsion constant and mass per length with their units, and the product's adapter is compared with those values case by case. A case in which the two differ is kept with the size of the difference; neither side is altered to agree.
- **AC-003** — Each of these is refused with a named blocking finding and returns no property: a missing outside diameter or wall thickness; a value that is not finite; an outside diameter, wall or density that is not positive; a negative allowance, tolerance or insulation thickness; an allowance and tolerance that together leave no positive effective wall; a wall that leaves a negative inside diameter.
- **AC-004** — An input whose unit is not a unit of its dimension in the unit catalog is refused with a finding that names the input. Inputs of one dimension in different units give either the result for the converted values or a named refusal, and never a result computed from the unconverted magnitudes.
- **AC-005** — An input without provenance is refused with a finding that names it. Every returned property names the inputs it was derived from and carries their provenance and redistribution status, and states that it was calculated.
- **AC-006** — The calculator's sources, tests and fixtures contain no protected dimensional or material table and no proprietary value; every example value is invented. No output or message states or implies code compliance, certification or approval.
- **AC-007** — The calculator depends on no solver or rule-evaluation module, and neither depends on it for anything but these properties.

## Production and Verification Method — Praxeology

**Method choices that define the calculator.**

- The section is the exact circular annulus in closed form. There is no table lookup and no schedule.
- Reductions of the wall are absolute thicknesses entered by the user. A percentage or catalogue tolerance is not interpreted.
- Optional contributors are present or absent. Absence is never read as zero, and presence is never assumed.
- A calculation either returns every requested property or returns findings and nothing else.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference: the calculator's outputs against closed-form annulus values evaluated by the formal calculation witness under `validation/witness`, which runs no product code, and against closed-form values stated in the test for the reduced-wall and mass cases.
- **VER-002** — Comparison with a named reference: the calculator and the product's adapter on a shared set of dimensions, corrosion allowance, mill tolerance, densities and units, with every difference recorded with its size.
- **VER-003** — Executable check: observes each refusal of AC-003 with its named finding and the absence of any returned property.
- **VER-004** — Executable check: observes the refusal of a unit outside its dimension, and for mixed units either the converted result or the named refusal.
- **VER-005** — Executable check: observes the refusal of an input without provenance, and on each output the named inputs with their provenance and redistribution status.
- **VER-006** — Executable check, with judgment by an independent reviewer: a protected-content scan over the calculator's sources, tests and fixtures, which is a proxy; a reviewer who did not write them reads the values against `docs/IP_AND_DATA_BOUNDARY.md` §3 and the output texts against `docs/claims_registry.md`; a dependency check observes what the calculator imports and what imports it.

## Governing Values and Decisions — Axiology

- **AX-001** — *From the user's dimensions only.* Better is fewer inputs the calculator could obtain from anywhere but the user; the target is none. Convenience never justifies a schedule, a default density or an assumed allowance.
- **AX-002** — *Exact over approximate.* Better is a smaller deviation from the closed-form annulus. This is never bought by loosening a tolerance.
- **AX-003** — *One definition.* Better is fewer differences between this contract and what the solve computes, each one that remains stated with its size. Agreement is never reached by changing this contract only to match what the adapter computes.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-051 OBJ-004 OBJ-012 | REQ-001 | AC-001 | VER-001 | Outside source: closed-form geometry of a circular annulus, evaluated by a calculation witness that runs no product code |
| OUT-002 | SOW-051 OBJ-004 OBJ-012 | REQ-008 | AC-002 | VER-002 | Two implementations of the product compared with each other, which only guards against drift. Correctness rests on AC-001; on the comparison alone there are no independent grounds |
| OUT-001 | SOW-051 OBJ-004 OBJ-012 | REQ-002 | AC-003 | VER-003 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state the expected refusal; the geometric conditions follow from the definition of an annulus |
| OUT-001 | SOW-051 OBJ-012 | REQ-003 | AC-004 | VER-004 | A reference this deliverable does not write: the unit catalog's dimensions and conversion constants (DEL-02-02) |
| OUT-001 | SOW-018 OBJ-004 | REQ-004 | AC-005 | VER-005 | The criterion states the expected content: the provenance that was given on the inputs |
| OUT-001 | SOW-051 OBJ-004 | REQ-005 REQ-006 | AC-006 | VER-006 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md` and `docs/claims_registry.md` |
| OUT-001 | SOW-051 OBJ-004 | REQ-007 | AC-007 | VER-006 | The criterion states the expected dependencies (the decomposition's PKG-03 exclusion) |
