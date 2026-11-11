---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-03
package_id: PKG-10
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-031, SOW-049]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-10-03 Local FEA handoff data contract

## Purpose and Objective Traceability

SWBPIPE analyses a piping system as a centerline frame. Some local details, such as a nozzle, a lug, a trunnion or a branch, need shell or solid analysis that a frame model cannot give. This deliverable defines what the user hands to whoever does that analysis: a package that identifies the local region, ties it to the global model and its results, and carries the units, assumptions and limitations needed to reconstruct the local problem. It also states the advisory criteria for when the global analysis is enough and when a local handoff is worth considering. The result it contributes is that a local-detail problem can leave SWBPIPE traceably, and that the software never presents the decision to hand off, or the local analysis, as its own engineering conclusion.

It serves SOW-031 and SOW-049 under OBJ-009 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-10, row DEL-10-03: guidance only, no external FEA implementation), with `docs/PRD.md`'s product principles (shell and solid FEA remain a local handoff), §4.3, §6.2 item 6, §7 item 9 and §21.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The local FEA handoff data contract, `schemas/local_fea_handoff.schema.yaml`.
- **OUT-002** — The handoff criteria guidance, `docs/local_analysis/local_fea_handoff_guidance.md`: the advisory labels and the factors that point to each.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Global analysis.* The 3D centerline frame model, which is the product's normal analysis of a system. *Local analysis.* Optional shell or solid analysis of one detail, done outside the product.
- *Handoff package.* The record a user hands over. Its parts are: its identity, including the product version that wrote it; references to the source model and results with their hashes; the global model context (the coordinate frame, the solve status, and the materials, sections and components of the region by reference); the local region; the units manifest; the entity identifiers; the transfer basis; the unit-preservation witnesses; the guidance assessment; assumptions; unsupported-behaviour flags; warnings and diagnostics; the privacy context; provenance; the professional-boundary values; and reproducibility metadata, including rule-pack references that expose no private values.
- *Local region.* The selected entities, their geometry and local-detail references, the coordinate system, how the region was selected (by the user, suggested by a diagnostic, or selected in review) and why.
- *Transfer basis.* What the local analysis takes from the global one: load cases, displacement, force and moment results, boundary conditions and cut boundaries, with their sign conventions. The package refers to result rows; it copies no result value. *Candidate rows* are the result rows of one kind (force, stress) that the region's transfer basis could refer to; the *governing row* of a kind is the row the result summary names as that kind's governing maximum, where it names one.
- *Units manifest.* The named unit of each of coordinate, force, moment, displacement, rotation, stress and temperature, with the unit system it rests on.
- *Unit-preservation witness.* A record, for each result the transfer basis refers to, that the referenced value keeps its source unit and that nothing was converted.
- *Guidance label.* One of five advisory labels, or the explicit undecided value `TBD` where no classification has been made:
  - `global_centerline_expected_sufficient_for_screening`: the frame results appear suitable for ordinary global review;
  - `local_detail_review_consider`: a local feature, assumption or uncertainty should be looked at before relying on global results alone;
  - `local_shell_solid_handoff_consider`: the detail may need shell or solid treatment outside the global solver;
  - `global_to_local_transfer_inputs_incomplete`: the package lacks a reference, unit, result basis, hash, assumption or boundary-transfer item it needs;
  - `human_review_required`: a competent person must review the context, assumptions, diagnostics, limitations and local-analysis setup. This label is on every assessment.
- *Unsupported-behaviour flag.* An explicit statement of something the handoff does not do or does only approximately: no mesh generated, no external solver invoked, no target format selected, boundary transfer needing review, a local-detail assumption unresolved, or an approximate global-to-local transfer. The flags are the handoff's loss report.

**Boundary.** This deliverable is a data contract and guidance. It builds no shell or solid analysis, generates no mesh, runs no external solver and writes no external tool's format. It gives no code-specific local-stress acceptance rule.

- Exporting a local FEA submodel is owed work at Could priority, to be implemented as its own work item with implementing deliverables that a decomposition revision scopes; this contract is its authority (also cited as DEL-10-03:1). It is distinct from the handoff package of PKG-15. Deferring it again is the owner's decision.
- The desktop panel that shows a package is DEL-07-12's. The builder it calls, with the unit rule of REQ-013, is this deliverable's, and what it builds conforms to this contract.
- The global solver and what its frame model represents are DEL-04-01's; stress recovery is DEL-05-03's; unit symbols are DEL-02-02's; entity identity is DEL-02-01's.
- An export adapter that writes a package for a particular tool would be subject to the API boundary contract (DEL-10-01) and the adapter framework (DEL-10-02).

**Not decided.**

- The external FEA formats a package may be written in, and which tools.
- How global results map onto shell or solid boundary conditions for any particular solver.
- Whether a package may carry result values converted to its declared units. Routine engineering unit conversion is permitted as a solution; a package that used it would need a witness for each converted value.

## Completion and Reliance Basis — Epistemology

The contract is adequate when an FEA specialist holding a package can identify the region, the global results it rests on, their units and the stated limitations without access to the project, and when nothing in the package or the guidance reads as a decision the software made.

### Requirements

Requirement numbers 001 to 011 are those of the earlier `DEL-10-03-REQ-nn` series.

- **REQ-001** — The contract treats the global centerline analysis as the normal method and local shell or solid FEA as an optional specialized handoff (SOW-031; PRD §7 item 9).
- **REQ-002** — The contract has a slot for each part of the handoff package listed above.
- **REQ-003** — Every coordinate, force, moment, displacement, rotation, stress and temperature the package names or refers to is unit-aware and dimensionally explicit, and so is every local-detail assumption the user supplies (PRD §5.6).
- **REQ-004** — The package carries source and provenance, redistribution status, privacy classification and review status wherever engineering reliance or public contribution could be affected (PRD §20.2, §20.3).
- **REQ-005** — A missing solve-required value, a missing rule-check value, missing provenance, an unresolved local-detail assumption and an unmade format or tool decision each appear as a diagnostic or as an explicit undecided value, and never as a default (PRD §5.5).
- **REQ-006** — A guidance label is advisory. No label and no part of the package asserts code compliance, professional approval, certification, sealing, endorsement or project-specific acceptability, or compatibility with, or validation by, an external solver (PRD §4.3, §21.2).
- **REQ-007** — The contract, the guidance and their examples embed no protected standards text, tables or formulas, no material allowables, no SIF or flexibility tables, no protected dimensional tables, no proprietary vendor data and no private project or rule-pack values (PRD §20.1).
- **REQ-008** — A handoff is exported only through the schema-first command, query and job boundary, and no adapter or plugin that writes one goes around domain validation, unit checks, diagnostics, provenance checks, privacy controls, protected-content screening or report controls (decomposition AB-00-07).
- **REQ-009** — The external format, the target solver adapter, mesh generation and external solver invocation stay undecided in the contract.
- **REQ-010** — The guidance states criteria that distinguish expected sufficiency of the global analysis from a recommended local-detail review or local FEA handoff, and leaves the engineering decision with a competent person (SOW-049).
- **REQ-011** — A diagnostic in a package carries code, class, severity, source, affected object, message, remediation and provenance, for a finding that blocks the handoff and for an assumption, provenance, privacy or IP-boundary finding alike (decomposition AB-00-06).
- **REQ-013** — The result units a package declares are the units of the result rows it refers to, and nothing is converted. Where a governing row exists, its unit decides and only rows in that unit are referred to; a candidate row in another unit is left out and named in a diagnostic with its identifier and unit. Where no governing row decides and the candidate rows disagree, the package is refused. Where no row of a kind is referred to, the declared unit is the project's unit for that kind, and a project that lacks it is refused.
- **REQ-014** — A package whose region lacks stable model identity, or whose units, result basis or hashes are missing, is not presented as usable: it carries the incomplete-inputs label and a blocking diagnostic.
- **REQ-015** — Every hash a package carries is SHA-256 over the RFC 8785 canonical form of the JSON it names, so that the same package gives the same hash (decomposition AB-00-04).

### Acceptance criteria

- **AC-001** — The contract is valid JSON Schema 2020-12 in strict JSON, and a complete invented package validates against it.
- **AC-002** — A package lacking any part of the handoff package is refused, and so is a units manifest that omits any of the seven named units.
- **AC-003** — Each result a transfer basis refers to has a unit-preservation witness that names its source unit and states that no conversion was performed, and a package whose witness states a conversion is refused. A package built from results in mixed units follows REQ-013: the declared unit equals the unit of every row referred to, each omitted row is named in a diagnostic, and the disagreeing and missing-project-unit cases are refused by name.
- **AC-004** — A guidance assessment without `human_review_required` is refused, as is a package or assessment in which any approval, compliance, certification, sealing or authentication value is other than false, which carries any value stating that a target solver accepts or validated the package, or which uses a value other than the five labels and `TBD`.
- **AC-005** — The contract fixes the global analysis as the centerline frame model and the local analysis as optional, and carries the external format, the target solver adapter, mesh generation and solver invocation as undecided. A package that names one of them is refused.
- **AC-006** — A package in which the region identity, a unit, the result basis or a hash is present only as the explicit undecided value carries `global_to_local_transfer_inputs_incomplete` and a blocking diagnostic with the eight members of REQ-011, and no value in it was supplied by default. Missing provenance appears as a provenance diagnostic naming the item.
- **AC-007** — The guidance gives, for each of the five labels, its meaning, the factors that point to it and what it does not decide. The factors for considering a local review include local geometry or restraint behaviour the frame model does not represent, local load introduction, attachment, nozzle and equipment-interface concerns, a localized discontinuity and an unresolved assumption. The factors for expecting the global analysis to suffice are a question limited to system flexibility, displacements, reactions, member forces and moments and open-mechanics stress recovery, with complete solve-required data, consistent units and visible assumptions. The guidance says that every label is advisory and cites no code clause.
- **AC-008** — The contract, the guidance and every example are invented or original, and contain no protected or private content and no claim of compliance, certification or approval.
- **AC-009** — A handoff package reaches a file only as the result of an export operation that returns a result envelope with the package's diagnostics, after schema validation, unit checks, provenance checks, privacy controls and protected-content screening; a package that fails one of them is not written. A declaration of an adapter or plugin that would write a package without those checks is refused by the adapter framework (DEL-10-02).
- **AC-010** — Every hash in a package equals SHA-256 over the RFC 8785 canonical form of what it names, as computed by an implementation that imports no product code, and building the same package twice gives the same hashes.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- *Reference, do not copy.* A package refers to model entities and result rows by identifier and hash. It bundles no project model and no result table.
- *Target-neutral.* The contract describes the local problem in the product's own terms and selects no solver's fields, mesh settings or exchange format.
- *Units by witness.* A package does not assert its units; it shows, for each referenced result, that the source unit was kept.
- *Labels with reasons.* An assessment gives labels together with references to the inputs and rationale behind them, never a pass or a fail.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the contract and packages validated against it): validates the contract against the JSON Schema 2020-12 meta-schema, then a complete invented package and invented packages that each carry one fault of AC-002, AC-004, AC-005 and AC-006, and reads which are accepted and refused.
- **VER-002** — Executable check (observes packages built by the desktop from invented projects): builds a package from results in one unit, in mixed units with a governing row, in mixed units without one, and from a project lacking a unit, and reads the declared units, the witnesses, the diagnostics and the refusals.
- **VER-003** — Judgment by a named role (a reviewer competent in piping stress analysis and local shell or solid analysis, who did not write the guidance): reads the guidance's factors for each label and judges whether they are sound screening criteria for the stated purpose.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the contract or the guidance): reads both, and the examples, against SOW-031, SOW-049 and PRD §20.1 and §21.2, and reports each missing label, meaning or factor, any protected or private content, any cited code clause and any statement that reads as a decision or an approval.
- **VER-005** — Executable check (observes how a package reaches a file): exports invented packages that each fail one of the checks of AC-009 and reads the result envelope returned and whether a file was written; presents the adapter framework with a declaration of a handoff writer that turns one of those checks off and reads the refusal.
- **VER-006** — Comparison with a named reference: each hash in an invented package against SHA-256 over RFC 8785 canonical JSON computed by an implementation that imports no product code; and the hashes of two builds of one package against each other.

## Governing Values and Decisions — Axiology

- **AX-001** — *Enough to reconstruct, no more.* Better is a package from which the local problem can be set up with fewer questions back to the analyst, measured by the parts of the package a specialist had to ask for. This is never bought by exporting private project data the handoff does not need.
- **AX-002** — *Reasons over verdicts.* Better is guidance that helps an engineer decide sooner, measured by whether each label comes with factors a competent reader can check against the model. A label never becomes a pass, a fail or an acceptance.
- **AX-003** — *Stated limits.* Better is fewer things a recipient could assume the handoff did and it did not: every omission and approximation is flagged. A cleaner-looking package is never preferred to a flag.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-031 OBJ-009 | REQ-002 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 meta-schema decides the contract's validity |
| OUT-001 | SOW-031 OBJ-009 | REQ-002 REQ-003 REQ-004 | AC-002 | VER-001 | The parts of a package are defined by this scope from SOW-031; each invented faulty package states the refusal it must receive |
| OUT-001 | SOW-031 OBJ-009 | REQ-003 REQ-013 | AC-003 | VER-001 VER-002 | The criterion itself determines the expected result: the declared unit must equal the unit of each referenced row, read from the source results and not from the package. The unit symbols are DEL-02-02's catalog |
| OUT-001 | SOW-049 OBJ-009 | REQ-006 | AC-004 | VER-001 | PRD §4.3 and §21.2 list the prohibited claims, and SOW-049 makes the criteria guidance only |
| OUT-001 | SOW-031 OBJ-009 | REQ-001 REQ-009 | AC-005 | VER-001 | SOW-031 and the decomposition row make local FEA optional and leave formats and tools unselected |
| OUT-001 | SOW-031 OBJ-009 | REQ-005 REQ-011 REQ-014 | AC-006 | VER-001 | PRD §5.5 forbids silent defaults and decomposition AB-00-06 lists the diagnostic members; each invented incomplete package states what it must carry |
| OUT-002 | SOW-049 OBJ-009 | REQ-010 | AC-007 | VER-003 VER-004 | SOW-049 requires the criteria. Whether the factors are sound engineering screening criteria is the competent reviewer's judgment, not a result the product computes |
| OUT-001 | SOW-031 OBJ-009 | REQ-007 | AC-008 | VER-004 | PRD §20.1 and §21.2 list the prohibited content and claims; the reviewer compares |
| OUT-002 | SOW-049 OBJ-009 | REQ-007 | AC-008 | VER-004 | PRD §20.1 and §21.2 list the prohibited content and claims; the reviewer compares |
| OUT-001 | SOW-031 OBJ-009 | REQ-008 | AC-009 | VER-005 | Decomposition AB-00-07 says what no writer may go around. Each seeded bypass states the refusal it must receive, and whether a file was written is read from the file system, not from the export's own report |
| OUT-001 | SOW-031 OBJ-009 | REQ-015 | AC-010 | VER-006 | Outside source: RFC 8785 and SHA-256, computed by an implementation that imports no product code. Determinism is one build compared with another |
