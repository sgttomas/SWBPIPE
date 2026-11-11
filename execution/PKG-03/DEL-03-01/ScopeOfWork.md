---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-01
package_id: PKG-03
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-017]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-01 Material library schema with provenance

## Purpose and Objective Traceability

This deliverable is the product's definition of a private material library: the document in which a user holds their own materials, with each property at the temperatures they have data for, their own allowables, and for every value its unit, its source and whether it may be redistributed. It is what lets the product work with code-specific and proprietary material data while the public project ships none.

It serves SOW-017 under OBJ-004 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-03, row DEL-03-01), and PRD §18.1 with FR-RULE-005, FR-MOD-005 and the data boundary of §20.1 to §20.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The material library document format, `schemas/material.schema.yaml`, with an invented example library that exercises it.

## Deliverable Definition — Ontology

**Terms the format implements.**

- *Material library.* One document: the library's identity (identifier, name, scope, privacy class, provenance, review status) and its material records, with the property definitions and completeness rules the records are read against. It states the version of the format it is written in.
- *Material record.* One material: identifier, name, family, an optional specification or grade label, privacy class, redistribution status, its property values, its allowable slots, its completeness findings, provenance and review status.
- *Property value.* One property of one material: its kind (among them density, elastic modulus, shear modulus, Poisson's ratio, thermal expansion coefficient, yield strength, tensile strength and allowable stress, with a user-defined kind for anything else), the value as a magnitude with its unit, dimension and quantity kind, what the value is required for, and its own provenance and review status.
- *Temperature-dependent property.* The same property held at more than one temperature, each value stated with the temperature it applies at. The format holds the user's points only: it defines no curve and no value between or beyond them.
- *Allowable slot.* A place for the user's own allowable: its kind, an optional value, an optional reference to the rule pack that reads it, and the policy for whether a value may appear in a public artifact. The format supplies the slot and never a value.
- *Provenance.* Seven members on the library, on each record, on each value and on each diagnostic: source name, source location, source licence, contributor, contributor certification, redistribution status and review status (PRD §20.2).
- *Standing vocabularies.* Privacy class (public schema only, public permissive, private user data, quarantine metadata only); redistribution status (public permissive, private only, unknown, protected suspected, rejected); review status (pending, accepted, rejected, quarantined).
- *Completeness finding.* The record of a value that a mechanics solve, a rule check or a report requires and the record does not hold. The value is marked missing; the finding names it and carries a diagnostic.

**Boundary.** This deliverable defines the document. DEL-03-07 decides whether an imported library is accepted, held for review, quarantined or refused, and gates the local library store. DEL-07-03 owns the editor in which a user authors a library. DEL-02-02 owns units and dimensions. DEL-02-01 owns the model's own material entity, and DEL-05-02 owns how a solve selects or interpolates a material's temperature points. Pipe section, component and hanger libraries are DEL-03-02's. Applying a library record to a model entity, reporting a library reference that no longer resolves and choosing a hanger record are DEL-07-03's. The format carries no engineering value of its own.

**Not decided.** Which public sources are accepted for redistributable material values.

## Completion and Reliance Basis — Epistemology

A document that validates is well formed and complete in the stated members. That says nothing about whether its values are right or lawful to redistribute.

### Requirements

- **REQ-001** — A material record can hold every item PRD §18.1 lists: name, specification or grade label, density, Poisson's ratio, elastic modulus against temperature, thermal expansion data, user-supplied allowable stress, user-supplied yield and ultimate strength, corrosion allowance, mill tolerance, source and provenance, verification status and private or public status. Every numeric value carries its magnitude, unit, dimension and quantity kind (SOW-017; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-002** — The format has slots for allowables and bundles none. No public artifact of this deliverable carries a protected or code-specific allowable, material table or proprietary value (SOW-017; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1; PRD §20.1).
- **REQ-003** — Provenance is required on the library, on each material record, on each property value, on each allowable slot and on each diagnostic (`docs/CONTRACT.md` OPS-K-DATA-3, OPS-K-IP-2; PRD FR-MOD-005).
- **REQ-004** — The library and each record state their privacy class and redistribution status (`docs/CONTRACT.md` OPS-K-IP-2, OPS-K-PRIV-1; PRD §18.1).
- **REQ-005** — A value that a solve or a rule check requires and the record lacks is an explicit completeness finding with a diagnostic. The format defines no default for any member (`docs/CONTRACT.md` OPS-K-DATA-2; PRD §5.5).
- **REQ-006** — Content suspected of being protected can be recorded as such and held: the redistribution, value and review vocabularies each have a state for it, and a diagnostic names it (`docs/CONTRACT.md` OPS-K-IP-3; PRD §20.2).
- **REQ-007** — A material library document is versioned and schema-governed, and survives storage unchanged: what is read back is what was saved, units and provenance included (the decomposition's architecture basis for DEL-00-04).
- **REQ-008** — An invented example is labelled as invented in its provenance and value status and is never presented as engineering data (PRD FR-RULE-007, §20.1).
- **REQ-009** — Every dimension a material property may carry is a dimension of the project's unit catalog.

### Acceptance criteria

- **AC-001** — A library document that gives a material every item of REQ-001, each numeric value with its unit, dimension, quantity kind and provenance, validates against the format. A document that omits a required identity, unit, dimension, provenance, redistribution or review member, or carries a member the format does not define, is rejected, and the rejection names the member.
- **AC-002** — A material record holds one property at two or more temperatures. Each value states the temperature it applies at, with its unit, and a reader recovers every temperature-and-value pair from the library document alone.
- **AC-003** — The format defines no default for any member. A record can mark a solve-required or rule-check-required property as missing, with a completeness finding that names it and a blocking diagnostic carrying code, class, severity, source, affected reference, message, remediation and provenance, and such a document validates.
- **AC-004** — The schema and the example library contain no protected standards text, table or allowable and no proprietary value. Every value in the example is invented, or public-domain or permissively licensed with its provenance recorded and its review status accepted, or omitted, and is labelled so where it appears.
- **AC-005** — Every dimension the format admits for a material property is a dimension of the unit catalog.
- **AC-006** — A valid material library document saved to the local library store and read back equals the original in canonical JSON form (RFC 8785), with its format version, units and provenance intact.
- **AC-007** — A library document can mark the library, a record or a single value as suspected protected content, give it a quarantined standing and attach the diagnostic that names it, and such a document validates. Whether it is admitted is DEL-03-07's decision.

## Production and Verification Method — Praxeology

**Method choices that define the format.**

- The format is a JSON Schema 2020-12 document that closes every object: a member it does not define is an error.
- Standing, privacy and value states are closed vocabularies. An unresolved state is itself a named value, never an absent member.
- Values and provenance travel together: a value's source is on the value, not inferred from its record or library.
- Public example values are invented or cleared for redistribution; where neither is available the example omits the value and shows the missing-value finding.

**How each criterion is examined.**

- **VER-001** — Executable check: validates the schema itself, a complete document, and one document per removed or unknown member; observes acceptance of the first and the named rejection of each of the others, the absence of any default in the schema, the validity of a document that marks a required property as missing with its completeness finding and diagnostic members, and the validity of a document that marks suspected protected content at each level.
- **VER-002** — Executable check: authors a record with one property at several temperatures and observes that a reader recovers each temperature-and-value pair, with units, from the document alone.
- **VER-003** — Executable check, with judgment by an independent reviewer: a protected-content scan observes that listed protected terms are absent from the schema and the example, which is a proxy; a reviewer who did not write the example reads it against `docs/IP_AND_DATA_BOUNDARY.md` §2 to §4.
- **VER-004** — Comparison with a named reference: the format's dimension vocabulary against the dimension list of the unit catalog, `schemas/units.schema.yaml`.
- **VER-005** — Executable check: saves a valid library through the local library store, reads it back and compares the canonical forms of the two documents.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing protected ships.* Better is fewer values in this deliverable's public artifacts whose redistribution is not documented and accepted. A more realistic example never justifies a value with an uncleared source.
- **AX-002** — *Explicit over convenient.* Better is fewer ways a value can leave a library without its unit, its source and its standing. This is never bought with a default, an inferred unit, or a rule that stops a user keeping an incomplete private library; an absent value stays absent and is named.
- **AX-003** — *The library is the user's.* Better is more of what PRD §18.1 lists that a user can record as their own data. The format never gains coverage by supplying a value itself.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-017 OBJ-004 | REQ-001 REQ-003 REQ-004 | AC-001 | VER-001 | PRD §18.1 and §20.2 list the members; the criterion states the expected acceptance and rejections |
| OUT-001 | SOW-017 OBJ-004 | REQ-001 | AC-002 | VER-002 | SOW-017 and PRD §18.1 require properties against temperature; the criterion states the expected pairs |
| OUT-001 | SOW-017 OBJ-004 | REQ-005 | AC-003 | VER-001 | PRD §5.5 and `docs/CONTRACT.md` OPS-K-DATA-2 state the expected finding; the diagnostic fields are the decomposition's architecture basis for DEL-00-06 |
| OUT-001 | SOW-017 OBJ-004 | REQ-002 REQ-008 | AC-004 | VER-003 | Protected-content scan and independent review under `docs/IP_AND_DATA_BOUNDARY.md`; PRD §20.1 |
| OUT-001 | SOW-017 OBJ-004 | REQ-009 | AC-005 | VER-004 | A reference this deliverable does not write: the unit catalog's dimension list (DEL-02-02) |
| OUT-001 | SOW-017 OBJ-004 | REQ-007 | AC-006 | VER-005 | The criterion states the expected result: the saved document itself, in the canonical form an outside standard defines (RFC 8785) |
| OUT-001 | SOW-017 OBJ-004 | REQ-006 | AC-007 | VER-001 | PRD §20.2 (unclear or suspicious data is quarantined) and `docs/CONTRACT.md` OPS-K-IP-3 |
