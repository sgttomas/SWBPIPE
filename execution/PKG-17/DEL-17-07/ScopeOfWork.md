---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-07
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-07 Conservative PCF subset exporter

## Purpose and Objective Traceability

This deliverable is the PCF exporter: it writes a piping component file (PCF) of a model's routing and component context, for plant-design tools that read that format, together with an identity sidecar, a loss report and diagnostics. The result it contributes is that a user can take geometry and component context to a PCF-reading tool and see, entity by entity, what was written, what was left out, and what the receiving translator will decide for itself. PCF is a secondary path. A receiving translator fills many gaps from its own settings, so this exporter is deliberately narrow, and it is not the route by which SWBPIPE's results are checked against an external prover (that is DEL-17-04 and DEL-17-05).

It serves SOW-030 and SOW-074 under OBJ-009, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-07: useful for interoperability, not the first validation backbone, translator behaviour explicitly bounded), with `docs/PRD.md` §5.8, §16.1, FR-HAND-003 and FR-HAND-004 (Should) and FR-HAND-005 (Could). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The PCF subset exporter: the PCF profile, the writer that produces a PCF and its package from a model, the schema `schemas/pcf_export.schema.json`, the export a user reaches in the desktop, and the invented fixtures.

## Deliverable Definition — Ontology

**Terms the exporter implements.**

- *PCF profile.* The export profile of this target under the common contract (DEL-17-02), with target family `PCF`. It classifies every candidate family.
- *Candidate subset.* The families and attributes this exporter may carry and no others: straight pipe, elbows and bends, tees, reducers, flanges, valves, end connections, line numbers, nominal size, outside diameter and wall thickness where the profile supports them, basic specification and material labels, coordinates, component identifiers, and the identity sidecar.
- *Classification.* The profile's statement for one family: exported, omitted, approximated, delegated, unsupported or undecided. These are the six loss categories of the common contract.
- *Identity sidecar.* The stable ID map of this target, carried beside the PCF. It links each canonical SWBPIPE identifier to the PCF record written for it, or to the loss entry that says why none was.
- *Translator-dependent behaviour.* What a receiving translator decides from its own dialog, configuration or mapping database and not from the file. The public CAEPIPE PCF translator document (source `CAEPIPE-PCF`, `https://www.sstusa.com/pdfs/PCF.pdf`) records these for that translator:
  - dialog settings for piping code, specific gravity, node numbering, size filtering, vertical axis, flange handling and valve handling;
  - unit interpretation from the file's `UNITS-BORE` line;
  - fallback values for temperature, pressure, fluid density, weights, outside diameter, wall thickness, support type and material;
  - anchoring of free ends;
  - simulation of some components as rigid elements, equivalent pipes or concentrated masses.
- *Support outcome.* The class given to each support or restraint:
  - *reliably preserved*: the profile has source-confirmed attributes, mapping, units, coordinates, type and, where they apply, stiffness, gap and friction meaning, with identity traceable;
  - *delegated*: the receiving translator or the user's mapping database decides the final type or properties;
  - *undecided*: no admitted evidence settles whether the profile can preserve it;
  - *unsupported*: the profile cannot carry it or excludes it;
  - *blocked*: losing its meaning would make the export misleading for solver-ready use.

**Boundary.** This deliverable owns the PCF profile, writer, schema and fixtures.

- The common package, profile, stable ID map, manifest and loss report are DEL-17-02's. The admitted sources for PCF behaviour and the open questions about them are DEL-17-01's.
- The model document and its identifiers are DEL-02-01's. The refusal of an export when the project's unit map is incomplete, and exact length conversion, are DEL-02-02's.
- The target-neutral mapping contract is DEL-15-02's. The governed export route and the screening of private and protected values are DEL-12-02's. The desktop panel that presents this export is DEL-07-12's.
- A PCF that was written is export evidence only. It does not show that any downstream tool imported the model correctly or that the stress model is complete.

**Not decided.**

- The target version basis: which PCF target profile, or which translator version, is the first one supported (DEL-17-01, TBD-17-01-005 and CQ-17-01-006). Until an admitted source or the owner selects it, the classifications of the candidate subset are candidates and no family is recorded as supported on the strength of translator behaviour.
- Whether PCF can carry canonical identity directly. Until a source confirms it, identity travels in the sidecar.
- A reference for PCF syntax. No PCF format specification is admitted; the only admitted source is `CAEPIPE-PCF`, which describes one translator's reading of the format.
- The source-basis reference `PLAN-EXPORT-INTEROP`, which REQ-015 requires a profile to carry. The plan it names is not in the repository; the candidate subset it listed is stated above.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The profile rests on DEL-17-01 for every statement about PCF target behaviour.
- **REQ-002** — The profile and package follow DEL-17-02 for the package, profile, stable ID map, manifest and loss report.
- **REQ-003** — Target behaviour that no admitted public or project-owned source supports is undecided, delegated, unsupported or reported as loss.
- **REQ-004** — The exporter relies on no hidden translator default for behaviour that affects stress or identity.
- **REQ-010** — The profile declares target family `PCF`, a profile identifier and version, the target version basis, its source-basis references and boundary notes.
- **REQ-011** — The profile declares its unit policy, coordinate policy, stable-ID policy, line and component identity policy and loss-report policy.
- **REQ-012** — The target version basis stays undecided until an admitted source or the owner selects a first PCF target profile.
- **REQ-013** — The profile gives each candidate family one classification.
- **REQ-014** — The exporter carries nothing outside the candidate subset; the subset changes only where review of an admitted source narrows or expands it.
- **REQ-015** — The profile carries source-basis references to DEL-17-01, DEL-17-02, `CAEPIPE-PCF` and `PLAN-EXPORT-INTEROP`, and a package without them is not accepted.
- **REQ-020** — The writer emits only content the profile classifies as exported, from model data that carries its units and provenance.
- **REQ-021** — The writer gives a diagnostic for a missing or ambiguous unit, coordinate, nominal size, outside diameter, wall thickness, material or specification label, component identity, end connection or support meaning, wherever the profile needs it.
- **REQ-022** — The writer invents no pressure, temperature, fluid density, component weight, outside diameter, wall thickness, material property, support type, equipment connection or boundary condition.
- **REQ-023** — Canonical identity is preserved in the sidecar, or in the PCF itself only where a source confirms the format carries it.
- **REQ-024** — An identifier the target generates is recorded apart from canonical SWBPIPE identity.
- **REQ-030** — Every export attempt produces a loss report.
- **REQ-031** — A loss entry has the members the common contract gives it: affected canonical identifiers, category, severity, the target artifact if any, reason, source-basis reference and downstream implication.
- **REQ-032** — Supports and restraints are listed separately, each with its support outcome.
- **REQ-033** — Free ends, equipment nozzles, adjacent pipelines and boundary conditions are blocked, undecided, delegated or reported as loss wherever the receiving path would otherwise depend on translator anchoring or on connection detail PCF lacks.
- **REQ-034** — A component that a receiving translator turns into a rigid element, an equivalent pipe, a concentrated mass or another approximation is recorded as approximated or delegated, and not as exported.
- **REQ-040** — Fixtures are invented, or otherwise safe to redistribute, with recorded provenance.
- **REQ-041** — Fixtures copy no proprietary plant model, vendor example, protected standards text or table, material allowable, SIF or flexibility value, owner criterion or commercial catalogue value.
- **REQ-042** — Public fixtures carry no code-specific compliance meaning and imply no professional acceptance.

### Acceptance criteria

AC-001 to AC-008 together carry commitment DEL-17-07:1.

- **AC-001** — A profile validates only when it declares everything REQ-010 and REQ-011 name, carries the four source-basis references, and classifies every family of the candidate subset. While the target version basis is undecided the profile says so, and a package that records a target version or a family as supported without an admitted source for it is refused.
- **AC-002** — For an invented model within the subset, the PCF holds one record for each entity classified as exported and no other record. Every coordinate and dimension in the file equals the model's value converted exactly to the unit the file declares, the file declares only units the profile's unit policy states, and two exports of the same model are byte-identical.
- **AC-003** — For a model that lacks any item REQ-021 names, the export gives a diagnostic naming the entity and the item, and the PCF holds no substituted value, placeholder or default unit in its place. No quantity REQ-022 names appears in a PCF unless the model holds it with its unit and the profile classifies it as exported.
- **AC-004** — Every record of the PCF maps through the sidecar to exactly one canonical identifier, every entity of the model appears in the sidecar as written or in the loss report as not written, and no target-generated identifier appears in a field that holds canonical identity.
- **AC-005** — A loss report accompanies every attempt, including one that lost nothing and one that was blocked, and each entry has the members of REQ-031. Each support and restraint has an entry with one support outcome. Each free end, nozzle connection, adjacent-pipeline connection and boundary condition is blocked or reported. Each component of a kind `CAEPIPE-PCF` says the translator simulates is recorded as approximated or delegated. Each translator-dependent behaviour is recorded as delegated, undecided or blocked, as the responses in Praxeology assign, and none appears as a value the exporter chose.
- **AC-006** — Every keyword and attribute the writer emits is one an admitted source documents, and the profile cites the source for each.
- **AC-007** — The package for an invented model validates against the schema and holds the PCF with its hash, the manifest, the sidecar, the loss report and the diagnostics, with hashes on the basis DEL-17-02/REQ-007 requires. The package a user saves from the desktop for the session's model is that package, and is refused, with nothing written, when the project's unit map is incomplete.
- **AC-008** — Every fixture is invented and carries its provenance and redistribution standing; positive fixtures cover each exported family, and negative fixtures cover an unsupported support, an ambiguous unit, a missing outside diameter or wall thickness, an unsupported component and an unresolved boundary condition. No member of a package and no fixture states PCF completeness, compatibility with a named tool, solver validation, code compliance or professional acceptance.

## Production and Verification Method — Praxeology

**Method choices that define the exporter.**

- The subset is narrow by choice. It is never widened because a receiving translator might infer the rest.
- A hidden default is a reportable condition, with four responses:
  - the model holds the value and the profile carries it: export it;
  - the receiving path may infer it from its own configuration: record it as delegated;
  - it affects solver-ready reading and no source confirms it: block it or mark it undecided;
  - the target represents a component by an approximation: record the approximation and the canonical identifiers affected.
- Supports and restraints are classified apart from geometry and components.
- The PCF and its sidecar travel together; the sidecar is the authority for identity.

**How each criterion is examined.**

- **VER-001** — Executable check (observes profiles with each declaration, source-basis reference and family classification removed, and a package that claims a target version without a source).
- **VER-002** — Executable check (observes the PCF of an invented model against the model's entities, coordinates and dimensions, the unit lines against the profile, and two exports for byte equality).
- **VER-003** — Executable check (observes exports of invented models each lacking one item of REQ-021, and the PCF text for any quantity of REQ-022 the model does not hold).
- **VER-004** — Executable check (observes the PCF's records, the sidecar and the loss report against the identifiers of the invented model).
- **VER-005** — Comparison with a named reference (the translator-dependent behaviours and simulated component kinds listed in `CAEPIPE-PCF`) and executable check (observes the loss report of an invented model with supports, free ends, a nozzle connection and components of the simulated kinds).
- **VER-006** — Comparison with a named reference (the keywords and attributes documented in the admitted sources the profile cites): examines every keyword and attribute of an exported PCF.
- **VER-007** — Executable check (observes the package of an invented model against the schema and the members' hashes, the package the desktop saves, and the desktop export with an incomplete unit map).
- **VER-008** — Executable check (the project's protected-content scan over the schema and fixtures) and judgment by a reviewer who did not write the fixtures: examines provenance, coverage of the positive and negative cases, and every member for the claims AC-008 excludes.

## Governing Values and Decisions — Axiology

- **AX-001** — *Narrow and stated.* Better is a larger share of a model's entities with a classification that an admitted source supports. Coverage is never raised by relying on what a translator would infer.
- **AX-002** — *No hidden default.* Better is fewer values in a PCF that the model did not hold. A cleaner file never justifies supplying one.
- **AX-003** — *Identity survives the format.* Better is more of the model's entities traceable from the PCF back to their canonical identifiers. Convenience for a reader that keeps only the PCF never moves identity out of the sidecar into a field no source confirms.
- **AX-004** — *Caution on supports.* Where a support's meaning may be lost, blocking is preferred to an export that reads as solver-ready.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-001 REQ-002 REQ-010 REQ-011 REQ-012 REQ-013 REQ-015 | AC-001 | VER-001 | Owner ruling: the decomposition row bounds translator behaviour and the common contract names what a profile declares; the seeded omissions state what must be refused |
| OUT-001 | SOW-030 OBJ-009 | REQ-014 REQ-020 | AC-002 | VER-002 | The invented model, which the exporter did not write, and the public definitions of the units; byte equality of two exports guards against change only |
| OUT-001 | SOW-074 OBJ-017 OBJ-018 | REQ-004 REQ-021 REQ-022 | AC-003 | VER-003 | Owner ruling: `docs/PRD.md` §5.5 (no silent engineering default); the seeded omissions state what must be diagnosed |
| OUT-001 | SOW-074 OBJ-017 | REQ-023 REQ-024 | AC-004 | VER-004 | Owner ruling: the common contract's stable ID map; the identifiers compared are the invented model's own |
| OUT-001 | SOW-074 OBJ-017 OBJ-018 | REQ-003 REQ-030 REQ-031 REQ-032 REQ-033 REQ-034 | AC-005 | VER-005 | Outside source: `CAEPIPE-PCF`, for which behaviours are translator-dependent; owner ruling: the common contract's loss categories |
| OUT-001 | SOW-030 OBJ-009 | REQ-001 REQ-003 | AC-006 | VER-006 | Outside source: `CAEPIPE-PCF`, for the keywords it documents. No independent grounds for whether the file is otherwise well-formed PCF: no format specification and no reference reader is admitted |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-002 REQ-030 | AC-007 | VER-007 | Outside source: RFC 8785 and SHA-256 for the hashes, and a JSON Schema 2020-12 validator for the package; owner ruling: `docs/PRD.md` FR-HAND-003 and FR-HAND-004 |
| OUT-001 | SOW-074 OBJ-018 | REQ-040 REQ-041 REQ-042 | AC-008 | VER-008 | Owner ruling: `docs/IP_AND_DATA_BOUNDARY.md` and `docs/PRD.md` §21.2; the reviewer did not write the fixtures |
