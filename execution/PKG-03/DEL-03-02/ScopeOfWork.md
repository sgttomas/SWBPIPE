---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-02
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-018]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-02 Pipe section and component library schema

## Purpose and Objective Traceability

This deliverable is the product's definition of three private libraries: pipe sections, components and hangers. Each is a document in which a user holds dimensions, weights, stiffnesses and catalogue data they are entitled to use, with a unit, a source and a redistribution standing on every value. The public project ships the formats and invented examples, and no protected dimensional table, vendor catalogue or rating without documented redistribution rights.

It serves SOW-018 under OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-02), and PRD §18.2 and §18.3 with FR-RULE-006, FR-MOD-005 and the data boundary of §20.1 and §20.2. The hanger library is a user-imported library class, and its working load range is what hanger sizing reads (PRD §12.4). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable; it reaches exit criterion 1's hanger sizing through DEL-04-11.

- **OUT-001** — The pipe section and component library document formats, `schemas/section.schema.yaml` and `schemas/component.schema.yaml`, with invented example libraries.
- **OUT-002** — The hanger library document format, `schemas/hanger.schema.yaml`, in versions 1.0.0 and 1.1.0, with invented example libraries.

## Deliverable Definition — Ontology

**Terms the formats implement.**

- *Section record.* One pipe section: identifier, name, section type, privacy class, redistribution status, provenance and review status, with its dimensions and its properties. A dimension is an outside diameter, wall thickness, corrosion allowance, mill tolerance, nominal size or schedule label, insulation or lining thickness, or a user-defined dimension. A property is an area, inside diameter, second moment, section modulus, torsion constant, mass per length or contents volume per length, and states whether the user entered it or it was calculated.
- *Component record.* One component: identifier, name, type, privacy class, redistribution status, provenance and review status, with its fields. A field is a slot of a named kind holding a quantity or a text reference, the status of that value, the policy for whether it may appear in a public artifact, and what it is required for.
- *Component family contract.* For a family of component types, the statement of which field kinds are its geometry, which are user-entered modifiers, which are source metadata, and how mechanics and rule checks consume a record of that family. The mechanism is this deliverable's; the field kinds, completeness rule and diagnostics of the bend, branch, rigid and expansion-joint families are those of DEL-03-03 to DEL-03-06.
- *Hanger record.* One hanger: identifier, name and provenance, with its type, source reference, optional manufacturer reference, stiffness, installed, cold, hot and constant loads, travel range and movement limit. Its stiffness is a force per length on a named translational degree of freedom. Every quantity is a magnitude with its unit, dimension and its own provenance. A record may also carry a *working load range*: a minimum and a maximum force, each with its own provenance. The range is the member version 1.1.0 adds; the format admits it under either version.
- *Quantity, provenance, standing and completeness.* As in the material library (DEL-03-01): every numeric value carries its unit and dimension; provenance has the seven members of PRD §20.2; privacy, redistribution and review are closed vocabularies; in the section and component formats a required value that is absent is marked missing and named in a completeness finding with a diagnostic.

**Boundary.** These formats define documents. DEL-03-07 decides whether an imported library is accepted and applies the rules the formats leave to the import boundary, among them the refusal of a working load range in a 1.0.0 document and of a range whose ends differ in unit or whose minimum is not below its maximum. DEL-04-11 owns what the working load range means in sizing. DEL-03-03 to DEL-03-06 own what each component family means in a model and a solve. DEL-03-08 calculates section properties. DEL-07-03 owns the library editors and DEL-07-09 the palette coverage of them. No catalogue is bundled.

**Not decided.** Which public sources are accepted for redistributable section and component values.

## Completion and Reliance Basis — Epistemology

A document that validates is well formed. That establishes neither the correctness of its values nor the right to redistribute them, and a hanger record that validates is not thereby ready to solve or to size.

### Requirements

- **REQ-001** — A section record can hold every item PRD §18.2 lists: size label, outside diameter, wall thickness, schedule or designation text, corrosion allowance, mill tolerance, lining, insulation or coating data, and source and provenance, with its redistribution status (SOW-018).
- **REQ-002** — A component record can hold every item PRD §18.3 lists: type, manufacturer, model, size or rating label, end connection, face-to-face or end-to-end length, weight, centre of gravity, stiffness behaviour, effective pressure area and movement limits where they apply, source and provenance, and redistribution status (SOW-018; PRD FR-RULE-006).
- **REQ-003** — No public artifact of this deliverable carries a protected dimensional table, standards data, proprietary catalogue value, material allowable, SIF or flexibility table, or code-derived value (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1; PRD §18.2, §20.1).
- **REQ-004** — In the section and component formats a required value that a record lacks is an explicit value status, completeness finding and diagnostic. In the hanger format a missing required member makes the document invalid, and an absent optional value stays absent. No format defines a default (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-005** — Every value that an engineer may rely on carries provenance, with its source and its licence or redistribution status (`docs/CONTRACT.md` OPS-K-DATA-3, OPS-K-IP-2).
- **REQ-006** — Every dimensional, weight, mass, centre-of-gravity, stiffness and property field carries a unit and a dimension (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-007** — Content suspected of being protected can be recorded as such and held, in the redistribution, value and review vocabularies and by a diagnostic; it is never accepted as public content, which DEL-03-07 decides (`docs/CONTRACT.md` OPS-K-IP-3; PRD §20.2).
- **REQ-008** — Each format is a closed JSON Schema 2020-12 document: a member it does not define is an error.
- **REQ-009** — A library document is versioned and schema-governed, and survives storage unchanged, units and provenance included (the decomposition's architecture basis for DEL-00-04).
- **REQ-011** — A diagnostic for an invalid, incomplete, private, unknown-source or suspected-protected record carries code, class, severity, source, affected object, message, remediation and provenance (the decomposition's architecture basis for DEL-00-06).
- **REQ-012** — The formats' vocabularies agree with the model: every dimension is a dimension of the unit catalog, and every concrete component family the formats admit is a component kind the model document admits; a type that names no family (other, or undecided) is not a model kind.
- **REQ-014** — The hanger format admits versions 1.0.0 and 1.1.0. A record may carry a working load range, and no other new member, which the format admits under either version. A 1.0.0 document without it remains valid unchanged and needs no migration (PRD §12.4).

### Acceptance criteria

- **AC-001** — A section library that gives a record every item of REQ-001, and a component library that gives a record every item of REQ-002, each numeric value with its unit, dimension and provenance, validate against their formats. A document that omits a required identity, unit, dimension, provenance, redistribution or review member, or carries a member the format does not define, is rejected, and the rejection names the member.
- **AC-002** — A hanger library written as 1.0.0 without a working load range validates exactly as it did before 1.1.0 existed. A 1.1.0 record whose working load range has a minimum and a maximum, each a force with its own unit, dimension and provenance, validates. A range with a missing end or any other member, a record without a member the format requires, and a quantity without its unit, dimension or provenance, are rejected.
- **AC-003** — No format defines a default for any member. In the section and component formats a record can mark a required dimension, property or field as missing, with a completeness finding that names it and a blocking diagnostic with the members of REQ-011, and such a document validates.
- **AC-004** — The three schemas and their example libraries contain nothing REQ-003 excludes. Every value in a hanger example is invented or omitted; every value in a section or component example is invented, public-domain or permissively licensed with documented provenance, or omitted, and is labelled so where it appears.
- **AC-005** — Every dimension the formats admit is a dimension of the unit catalog; every concrete component family the formats admit is a component kind the model document admits, and a type that names no family (other, or undecided) is not a model kind.
- **AC-006** — A valid section library, component library and hanger library, each saved to the local library store and read back, equal their originals in canonical JSON form (RFC 8785), with format version, units and provenance intact.
- **AC-007** — A library document can mark the library, a record or a single value as suspected protected content, give it a quarantined standing and attach the diagnostic that names it, and such a document validates. Whether it is admitted is DEL-03-07's decision.

## Production and Verification Method — Praxeology

**Method choices that define the formats.**

- One record-and-slot mechanism serves every component family. A family adds field kinds, a completeness rule and diagnostic codes; it adds no new record shape.
- In the component format, stiffness is held as linear stiffness and rotational stiffness, two separate dimensions; there is no generic stiffness dimension.
- The hanger format is a standalone schema, read as the same source by every validator of it, so that the rule set cannot drift between implementations.
- A new hanger member enters by a new format version; the refusal of that member in an older-version document is applied at the import boundary.
- Hanger example values are invented. Section and component example values are invented or cleared for redistribution; where neither is available the example omits the value and shows the missing-value finding.

**How each criterion is examined.**

- **VER-001** — Executable check: validates the section and component schemas themselves, complete documents, and one document per removed or unknown member; observes acceptance, each named rejection, the absence of any default, the validity of a document that marks a required value as missing with its completeness finding and diagnostic members, and the validity of a document that marks suspected protected content at each level.
- **VER-002** — Executable check: validates hanger libraries in each version, valid and invalid, against the hanger schema with a standard JSON Schema 2020-12 validator; observes that the 1.0.0 cases without a working load range give the verdicts they gave under the 1.0.0-only format, that a record lacking a required member is rejected, and that the range cases are accepted or rejected as the criterion states.
- **VER-003** — Executable check, with judgment by an independent reviewer: a protected-content scan observes that listed protected terms are absent from the schemas and examples, which is a proxy; a reviewer who did not write the examples reads them against `docs/IP_AND_DATA_BOUNDARY.md` §2 to §4.
- **VER-004** — Comparison with named references: the formats' dimension vocabulary against the unit catalog, `schemas/units.schema.yaml`, and the component types against the component kinds of `schemas/model.schema.yaml`.
- **VER-005** — Executable check: saves a valid library of each kind through the local library store, reads it back and compares the canonical forms.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing protected ships.* Better is fewer values in these public artifacts whose redistribution rights are not documented. A common-looking dimension or catalogue weight is treated as protected until its source is cleared; realism never justifies one.
- **AX-002** — *Explicit over convenient.* Better is fewer ways a value can leave a library without its unit, source and standing. This is never bought with a default, and never by refusing a user an incomplete private library.
- **AX-003** — *Old documents keep working.* Better is fewer stored libraries that a format change invalidates. Compatibility is never bought by letting a stored older-version document be admitted with a member its version did not define.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-018 OBJ-004 | REQ-001 REQ-002 REQ-005 REQ-006 REQ-008 | AC-001 | VER-001 | PRD §18.2, §18.3 and §20.2 list the members; the criterion states the expected acceptance and rejections |
| OUT-002 | SOW-018 OBJ-004 | REQ-004 REQ-014 | AC-002 | VER-002 | PRD §12.4 for the format and its range; the 1.0.0 verdicts are the format's own earlier output, which only guards against change |
| OUT-001 | SOW-018 OBJ-004 | REQ-004 REQ-011 | AC-003 | VER-001 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2; the diagnostic members are the decomposition's architecture basis for DEL-00-06 |
| OUT-001 | SOW-018 OBJ-004 | REQ-003 | AC-004 | VER-003 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md`; PRD §18.2 and §20.1, and for hanger examples the release target's hanger scope (invented example records only) |
| OUT-001 | SOW-018 OBJ-004 | REQ-012 | AC-005 | VER-004 | References this deliverable does not write: the unit catalog (DEL-02-02) and the model document's component kinds (DEL-02-01) |
| OUT-001 | SOW-018 OBJ-004 | REQ-009 | AC-006 | VER-005 | The criterion states the expected result: the saved document itself, in the canonical form an outside standard defines (RFC 8785) |
| OUT-001 | SOW-018 OBJ-004 | REQ-007 | AC-007 | VER-001 | PRD §20.2 and `docs/CONTRACT.md` OPS-K-IP-3 |
