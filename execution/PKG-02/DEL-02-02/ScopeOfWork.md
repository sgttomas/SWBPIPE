---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-02
package_id: PKG-02
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-025]
package_objective_refs: [OBJ-001, OBJ-012]
---

# Scope of Work — DEL-02-02 Unit system and dimensional-analysis core contract

## Purpose and Objective Traceability

No number in the product moves without a unit that has been checked. An engineer enters values in the units they work in, chooses the project's units, and sees results in SI or US customary units; every calculation receives its inputs on one consistent basis; and a value with a missing, unknown or dimensionally wrong unit is stopped with a named finding instead of being read as something it is not. A unit-conversion error is among the highest risks the PRD names, and this deliverable is where it is prevented once for the whole product.

It serves SOW-025 under OBJ-001 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-02, row DEL-02-02), and PRD §5.6 (unit safety), §5.5, §10.1 step 1 (the user creates a project and selects units), §12.9 (modal quantities published in units of the unit catalog) and §22.4 (unit-conversion regression). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It supports criteria 1 and 2, since every required analysis takes its quantities through the unit catalog.

- **OUT-001** — The unit system: the unit catalog and its conversions, the dimension vocabulary and its checks, the unit schema, and the project's unit selection and display units in the desktop.

## Deliverable Definition — Ontology

**Terms the unit system implements.**

- *Unit catalog.* A closed set of SI and US customary units. Each unit has a symbol, one dimension and a transform to its dimension's canonical unit. There are no user-defined units.
- *Dimension.* A named identifier with an exponent vector, such as `length`, `force`, `linear_stiffness` or `temperature_interval`. `schemas/units.schema.yaml` holds the one vocabulary. Dimensions that share an exponent vector stay distinct names where their meaning differs, as pressure and stress do.
- *Canonical basis.* The SI coherent units: m, kg, s, K, rad and those derived from them. Calculations run on this basis, and a quantity is converted to it once, where it enters a calculation.
- *Entered unit.* The unit the user typed. A stored quantity keeps its entered value and unit; conversion never rewrites it.
- *Conversion constants.* Public definitional values, held as binary64 with their derivation: 1 in = 25.4 mm; 1 ft = 0.3048 m; 1 lb = 0.453 592 37 kg; standard gravity 9.806 65 m/s²; 1 lbf = 1 lb × standard gravity; 1 psi = 1 lbf/in²; 1 ksi = 1000 psi; 1 bar = 100 000 Pa; 1 atm = 101 325 Pa; 180° = π rad; and the SI prefixes. Derived factors follow by arithmetic from these.
- *Absolute temperature and temperature interval.* Two dimensions. An absolute temperature converts affinely (K = °C + 273.15; °F = 1.8 × °C + 32). An interval converts by a factor only (1 °F interval = 5/9 K). Absolute minus absolute is an interval; absolute plus or minus an interval is absolute; interval plus or minus interval is an interval; absolute plus absolute is rejected. The canonical unit of both is K.
- *Pressure kind.* A pressure is absolute or gauge. Converting between kinds needs an explicit reference pressure with its provenance. No atmospheric value is assumed, and arithmetic across kinds without that conversion is rejected.
- *Quantity kind.* Whether a value is unit-bearing, absolute, interval, relative, or explicitly a ratio, percentage, coefficient or dimensionless number.
- *Conversion witness.* A check of a conversion against its definition, at one of two tolerances: an identity conversion is bit-exact and a single exact factor round-trips within one unit in the last place; a chained, derived or affine conversion round-trips within a relative error of 1e-12.
- *Project units.* One record in the model document: a map from seven dimension keys (length, force, moment, pressure, stress, temperature, angle) to catalog symbols. No unit-system name is stored; a preset is shorthand for a map. Node coordinates are numbers in the project's length unit; every other quantity carries its own unit, and the project units give the starting unit of a new entry.
- *Display unit.* The unit a value is shown in. It is session state and never touches the document.

**Undecided.** The unit identifier and alias namespace and the rule for an ambiguous symbol are not decided; until they are, a new selection takes a catalog symbol exactly, and a legacy `C` for degrees Celsius is read but not written. Which fields fall in each category of REQ-015's dimensionless classification, and the schema members that carry it, are not decided. The unit diagnostic code namespace is not decided. The treatment of angle and rotation beyond radian and degree conversion is not decided.

**Boundary.** This deliverable defines units, dimensions and conversions and checks them. The shape of a quantity and of the project units record in the model document is DEL-02-01's. Each consumer applies the catalog at its own boundary and owns that boundary: the product solve (DEL-04-07), the rule evaluator (DEL-06-02), imports and exports (PKG-10, PKG-17), reports (PKG-08) and result display (DEL-07-05). The operation engine that applies a unit change is DEL-16-06's. Tolerances for comparing solver results with references are the governed tolerances of the reference classes (DEL-09-01) and are not conversion tolerances. It defines no engineering value, and no code table, allowable or dimensional table is a source for it.

## Completion and Reliance Basis — Epistemology

A unit check establishes that a value is dimensionally what its field requires. It establishes nothing about whether the value is right.

### CLM-014 — Requirements

Requirement numbers 001 to 016 are those of the earlier `U-0nn` series.

- **REQ-001** — Every calculation, formula, imported value, export, schema and rule evaluation is unit-aware and dimensionally checked.
- **REQ-002** — The domain core owns unit enforcement. Adapters, plugins and import and export paths check units through it and cannot go around it.
- **REQ-003** — A unit-bearing value that crosses a schema, service, import, export, persistence, solver, load, stress, report or rule-pack boundary carries its unit, unless its field is explicitly classified as dimensionless.
- **REQ-004** — Every unit has one stable dimension from the one vocabulary. A change to the vocabulary is a change to the unit schema.
- **REQ-005** — Addition, subtraction, comparison and conversion reject incompatible dimensions. Multiplication, division and powers give an explicit derived dimension where the vocabulary has one.
- **REQ-006** — Conversion is deterministic. An unknown unit, a missing unit, an ambiguous or incompatible conversion and a non-finite value each produce a named finding and no converted value.
- **REQ-007** — Each conversion is a record with its factor or offset representation, provenance, redistribution status and review status. A record of suspected protected or proprietary origin is quarantined and escalated, and is not used.
- **REQ-008** — A persisted quantity keeps its entered value and unit through every round trip, and nothing converts a stored value.
- **REQ-009** — The public schema for quantities, units, dimensions and unit findings is JSON Schema 2020-12.
- **REQ-010** — A unit finding carries a code, class, severity, source, affected object, message, remediation and provenance.
- **REQ-011** — A unit check is never presented as code compliance, certification, approval, sealing or authentication (PRD §21.2).
- **REQ-012** — The unit system is examined for dimension compatibility, rejection of incompatible units, deterministic conversion, serialization round trips, schema validation, import and export paths, rule-pack unit mismatch, and hash stability of unit-bearing payloads (also cited as DEL-02-02:1).
- **REQ-013** — An unsupported dimension, an unresolved constant or an unsupported conversion is a visible finding. No engineering value is invented to fill it.
- **REQ-014** — Examples and fixtures for unit behaviour use invented, public-domain or permissively licensed data.
- **REQ-015** — A value is unitless only where its field is explicitly classified as dimensionless, a ratio, a percentage or a coefficient. A missing unit on a unit-bearing field is a finding and is never read as dimensionless.
- **REQ-016** — Unit identifiers resolve deterministically, and an ambiguous symbol is rejected (also cited as DEL-02-02:3).
- **REQ-017** — The dimension vocabulary is stated once, in the unit schema. Every consumer reads it from there or from the catalog, and none keeps a copy of its own.
- **REQ-018** — The catalog has named dimensions, each with display units from definitional factors, for the dynamics quantities: frequency, angular frequency, rotation per length, length per rotation, rotational generalized mass, mass-length per rotation and time squared (also cited as DEL-02-02:6).
- **REQ-019** — Unit authority reaches every boundary of REQ-001: the model's coordinates and quantities entering the solve, rule inputs, imports, exports, reports and comparisons are each normalized and checked through the catalog, not through a conversion of the consumer's own (also cited as DEL-02-02:2).
- **REQ-020** — A new project is created with the units the user selects. It carries all seven keys, each an exact catalog symbol of the key's dimension, chosen through an SI preset, a US customary preset, or any catalog symbol per key.
- **REQ-021** — After creation a project unit changes only through a structured operation, one key at a time. The length key cannot change once the document has a node. The other keys can change at any time. No stored value is converted, a key cannot be removed, and each refusal is named and leaves the document unchanged.
- **REQ-022** — The starting unit of a new entry comes from the project units, directly or by a composition checked against the catalog, such as force per length. Where the key is missing, or the catalog lacks the composed unit, the field has no default and says why; no other unit is substituted, and a unit already stored stays as stored.
- **REQ-023** — The desktop refuses, by name, to open a document without the project units map. An export that takes a unit from the project units is refused by name when that key is missing and writes no substitute. A length an export converts is converted exactly through the unit's definition.
- **REQ-024** — A displayed value can be shown in another catalog unit of its dimension. Display conversion changes no stored value and no model hash. A value shown in a unit other than its stored unit cannot be taken for the stored value.

### Acceptance criteria

- **AC-001** — For every unit of the catalog, the dynamics units of REQ-018 included, conversion to the canonical unit and back agrees with the value formed from the unit's public definition, at the witness tolerance of its kind (also cited as DEL-02-02:5).
- **AC-002** — Each operation of REQ-005 gives the stated dimension or a rejection for compatible and incompatible operands, and the temperature and pressure-kind rules hold: an absolute temperature difference is an interval, absolute plus absolute is rejected, and a change of pressure kind without a reference pressure is rejected.
- **AC-003** — An unknown symbol, a symbol of another dimension, a missing unit, an ambiguous symbol and a non-finite value each give their named finding and no value; a missing unit on a unit-bearing field is never accepted as dimensionless; and each finding carries the fields of REQ-010.
- **AC-004** — Every catalog unit has a dimension, a transform kind, a factor or offset representation, provenance and a review status, and no catalog entry, fixture or example comes from protected or proprietary data.
- **AC-005** — The dimension identifiers of the unit schema, the model schema, the catalog and every consumer are one set, and no consumer's source holds a literal copy of it.
- **AC-006** — At each boundary of REQ-019, a quantity with an unknown unit or a unit of the wrong dimension is refused with a unit finding, and a valid quantity reaches the calculation as the catalog's canonical value.
- **AC-007** — A quantity's entered value and unit are the same after save and reopen, after export and import where the format carries them, and in the hashed payload; no round trip converts a stored value.
- **AC-008** — The unit schema is valid JSON Schema 2020-12, the unit fixture validates against it, and a fixture record without a required unit yields a missing-unit finding.
- **AC-009** — A project created with each preset, and with a custom selection, carries the seven keys with exact catalog symbols of the right dimensions; a symbol that is unknown, an alias or of another dimension is refused for a new selection.
- **AC-010** — Each refusal of REQ-021 is raised by name and leaves the document unchanged; a permitted change alters one symbol and no stored value; and its inverse restores the prior document.
- **AC-011** — With a key present, a new entry starts in the stated unit. With the key missing or the composed unit absent from the catalog, the entry has no default and the notice names the key or unit. A document without the units map is refused at open by name, and each export of REQ-023 is refused by name with no substitute unit written.
- **AC-012** — A value shown in another unit equals the value formed from the units' public definitions, the document and its model hash are unchanged by any display-unit choice, an exported length equals its exact conversion, and a value shown in another unit is distinguishable from the stored value and unit.
- **AC-013** — No unit finding, message or schema text states or implies compliance, certification or approval.

## Production and Verification Method — Praxeology

**Method choices that define the unit system.**

- Calculations run in the SI coherent basis. Conversion happens once, where a quantity enters a calculation, and the calculation receives canonical numbers.
- A factor is formed from the definitional constants by arithmetic and held as binary64 with its derivation. No third-party unit library is the source of a factor.
- One catalog, in the domain core. Other languages and the desktop read it; they do not restate it.
- The project units are one map of seven keys. A composed default is formed from the keys and checked against the catalog before it is offered.
- A unit change after creation is a structured operation, reviewed and undone like any other edit.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-012, comparison with a named reference: the public definitions of the units, written out by a reference that imports no product code, compared at the witness tolerances; with an executable check that observes the document and model hash before and after a display-unit choice, and how a value shown in another unit is set apart from the stored value and unit.
- **VER-002** — AC-002 and AC-003, executable check: observes the resulting dimension or rejection of each operation, and the finding raised for each constructed bad input with its fields.
- **VER-003** — AC-004 and AC-013, judgment by a reviewer who did not write the catalog, with the protected-content scan: examines each catalog record's provenance and review status, the fixtures and the finding texts.
- **VER-004** — AC-005 and AC-008, executable check: observes the identifier sets of the schemas, the catalog and each consumer, and the unit schema and fixture under a JSON Schema 2020-12 validator that imports no product code.
- **VER-005** — AC-006 and AC-007, executable check: observes, at each boundary, the refusal of constructed bad quantities, the canonical value a calculation receives, and the entered value and unit after each round trip.
- **VER-006** — AC-009, AC-010 and AC-011, executable check: observes the created document's units, each named refusal and the unchanged document, each default and notice, the open refusal and each export refusal.

## Governing Values and Decisions — Axiology

- **AX-001** — *Refuse rather than guess.* Better is fewer boundaries where a number passes without a checked unit; the target is none. A permissive parse, an assumed unit or a fallback conversion is never accepted to make an import, an entry or an export easier.
- **AX-002** — *Exactness, stated.* Better is a conversion closer to its definition, with the rounding that remains stated. A witness tolerance is only tightened. A factor is never tuned to agree with another program, and a tolerance never loosened to pass.
- **AX-003** — *The user's units stay the user's.* Better is fewer places where a stored value is rewritten or a displayed value could be taken for the stored one. A stored value is never converted for convenience.
- **AX-004** — *A small catalog with sources.* A unit is in the catalog only with a public definition and reviewed provenance. Breadth never justifies a factor without a source, and a missing unit is a visible gap, not a reason to substitute another.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-025 OBJ-012 | REQ-006 REQ-018 | AC-001 | VER-001 | Outside source: the public definitions of the units (the international inch, foot and pound, standard gravity, the standard atmosphere, the Celsius and Fahrenheit scales, the SI prefixes), evaluated without product code. The tolerances are the witness tolerances the ontology states |
| OUT-001 | SOW-025 OBJ-012 | REQ-004 REQ-005 | AC-002 | VER-002 | Outside source: dimensional analysis, exponent-vector arithmetic. The temperature and pressure-kind rules are those the ontology states |
| OUT-001 | SOW-025 OBJ-001 OBJ-012 | REQ-006 REQ-010 REQ-013 REQ-015 REQ-016 | AC-003 | VER-002 | The inputs are constructed, so the criterion states the expected finding (PRD §5.5 and §5.6); the diagnostic contract (DEL-00-06) states the required fields. Which symbols are ambiguous awaits the undecided alias rule |
| OUT-001 | SOW-025 OBJ-001 | REQ-007 REQ-014 | AC-004 | VER-003 | The catalog's sources are the public definitions the ontology names, and PRD §20.1 and §20.2 state what a public record may contain; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-025 OBJ-012 | REQ-004 REQ-017 | AC-005 | VER-004 | The criterion states the expected result: one set, with the unit schema as its statement |
| OUT-001 | SOW-025 OBJ-012 | REQ-001 REQ-002 REQ-003 REQ-019 | AC-006 | VER-005 | The quantities are constructed, so the criterion states the expected refusal; the canonical value expected at each boundary comes from the unit definitions |
| OUT-001 | SOW-025 OBJ-012 | REQ-008 REQ-012 | AC-007 | VER-005 | The criterion fixes the expected result: the quantity read back equals the quantity written |
| OUT-001 | SOW-025 OBJ-001 | REQ-009 REQ-010 | AC-008 | VER-004 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code |
| OUT-001 | SOW-025 OBJ-001 | REQ-020 | AC-009 | VER-006 | PRD §10.1 says the user selects units, and the catalog holds SI and US customary units. The selections are constructed, so the criterion states the expected keys and each refusal |
| OUT-001 | SOW-025 OBJ-012 | REQ-021 | AC-010 | VER-006 | The documents are constructed, so the criterion states each expected refusal and the unchanged document |
| OUT-001 | SOW-025 OBJ-012 | REQ-022 REQ-023 | AC-011 | VER-006 | The criterion states the expected default, notice and refusal (PRD §5.5: nothing is assumed) |
| OUT-001 | SOW-025 OBJ-012 | REQ-023 REQ-024 | AC-012 | VER-001 | Outside source: the public definitions of the units. The unchanged document and hash are fixed by the criterion |
| OUT-001 | SOW-025 OBJ-001 | REQ-011 | AC-013 | VER-003 | PRD §21.2 and `docs/claims_registry.md` state the prohibited claims; the reviewer reads the finding texts and schema against them |
