---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-02
package_id: PKG-10
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-10-02 Import/export adapter framework

## Purpose and Objective Traceability

This deliverable is the shell every import and export adapter sits in. It says what an adapter is, what it must check before external data becomes model data or model data leaves the project, and it refuses an adapter that would skip a check. The result it contributes is that a new exchange format can be added to SWBPIPE without adding a new way around unit checks, provenance, the private-data boundary, diagnostics, the rule sandbox or the report controls.

It serves SOW-030 under OBJ-009 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-10, row DEL-10-02, and architecture-basis rows AB-00-03, AB-00-06 and AB-00-07), with `docs/PRD.md` §5.5, §5.6, §6.2 item 1, §11.7 (FR-HAND-005), §18.4, §20.2 to §20.5 and §21.2. The decomposition row leaves the formats themselves to other deliverables. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. Criterion 6, the MBF round trip, is owned by DEL-17-04 and DEL-07-12.

- **OUT-001** — The adapter framework: the adapter declaration contract `schemas/adapter_framework.schema.yaml`, and the validator, dispatch gate and operation-result builder in `core/adapters/framework/`.
- **OUT-002** — A sample adapter on invented data, `fixtures/adapters/invented/`, that shows a conforming declaration and exercises the framework's checks.

## Deliverable Definition — Ontology

**Terms the framework implements.**

- *Adapter.* A module that translates between an external form and the product's domain contracts: an importer or an exporter. An adapter translates at the edge of the application-service boundary and owns no validation, provenance policy or privacy policy.
- *Adapter declaration.* What an adapter states about itself: its identity, its capabilities, its provenance, its privacy context, its no-bypass controls and its validation plan. A declaration requests; it grants nothing.
- *Capability.* One of: import a model, export a model, import a library, export a library, import a rule-pack reference, export results, validate a payload, review a contribution.
- *Validation plan.* The checks an adapter commits to run on every payload: schema validation, unit validation, dimension validation, provenance validation, redistribution review, privacy classification and protected-content screening.
- *No-bypass controls.* The no-bypass constraints of the API boundary contract (DEL-10-01), each of which a declaration must affirm, with three more that apply to adapters: no network access, no choice of filesystem roots, and use of the public API boundary.
- *Finding.* One result of judging a declaration or a payload, with a code, a severity, a path, a message and a remediation. Its severity is information, warning, blocking or quarantine.
- *Outcome.* Accepted as a declaration, rejected, or quarantined. Quarantine is the outcome whenever protected content is suspected, and it takes precedence over every other finding.
- *Dispatch gate.* The one place a declaration could become a running adapter. It judges the declaration and then decides whether the adapter may be dispatched.
- *Operation result.* The envelope an adapter operation returns: diagnostics, privacy context, provenance, checksum and audit references, a reference to the result envelope it concerns, and the professional-boundary values.
- *Separate states.* A payload that parses is not thereby valid, cleared for redistribution, ready for a solve, ready for a rule check or accepted by anyone. The framework keeps these apart.

**Boundary.** The framework names no external format and parses no external file. A format enters the product through the deliverable the decomposition names for it: the export package and profile contracts and the named formats are PKG-17's (DEL-17-02 to DEL-17-09), the handoff package is PKG-15's, and the local FEA handoff contract is DEL-10-03's. The plugin manifest and the verification of a plugin's declaration are DEL-02-04's; that verification is a module inside this framework's package and calls the framework. The desktop panel that shows the framework's result is DEL-07-12's. The library import boundary is DEL-03-07's, and the export controls on private data are DEL-12-02's. Whether the product's built-in import and export adapters carry a declaration judged by this framework, or only adapters loaded as extensions do, is not decided.

The round trip a user sees, exporting a model in a named format and importing it back, is delivered by the format deliverables: MBF by DEL-17-04 with DEL-07-12, the others by DEL-17-03 and DEL-17-06 to DEL-17-08 (also cited as DEL-10-02:3). This deliverable's part of it is the checks the framework defines, which those adapters pass to the extent the question above brings them under the framework.

**Not decided.**

- The adapter execution model, the plugin runtime and the capability grants (also cited as DEL-10-02:1 and DEL-10-02:2). No rule-pack or report hook has a runtime.
- The redaction workflow an export adapter uses for private values.
- The format of a local FEA package.

## Completion and Reliance Basis — Epistemology

A declaration that is accepted shows what an adapter says about itself. It does not show what the adapter does. What an adapter does is observed where data crosses it, in AC-006 and AC-007. Those criteria apply to the adapters the framework governs: the sample adapter, and any other adapter as far as the undecided question of the Boundary brings it under the framework.

### Requirements

Requirement numbers 001 to 010 are those of the earlier `REQ-10-02-nn` series.

- **REQ-001** — The framework operates through schema-first command, query and job result envelopes, and no adapter goes around the public API and plugin boundary (decomposition AB-00-03, AB-00-07).
- **REQ-002** — Every import path validates units and dimensions before external data is accepted into domain workflows, and every import or export reports each unit conversion it performs, deterministically (PRD §5.6).
- **REQ-003** — Every import path captures the source and provenance, the licence or redistribution status and the review disposition of data records that may be reused or contributed publicly (PRD §20.2).
- **REQ-004** — An imported record with a missing required field, a missing or inconsistent unit, missing provenance, unclear redistribution status, suspected protected origin or a value outside a user-defined reasonableness range produces a diagnostic and never a silent default. Suspected protected content is quarantined (PRD §5.5, §18.4, §20.2).
- **REQ-005** — Every export path checks the private or public status of the data before it writes a payload that may leave the local project (PRD §20.3).
- **REQ-006** — An adapter's diagnostics carry code, class, severity, source, affected object, message, remediation and provenance (decomposition AB-00-06).
- **REQ-007** — A hook that connects an adapter to rule packs keeps rule-pack sandboxing, unit awareness, versioning, checksums and the public or private status of the pack (PRD §17.3; decomposition AB-00-07).
- **REQ-008** — A hook that connects an adapter to reports keeps report warnings, assumptions, provenance, limitations, protected-content controls and the professional-responsibility notice.
- **REQ-009** — The framework itself selects no external format, no proprietary tool behaviour and no commercial integration default. A format is admitted only where the decomposition names it or the owner has selected it, and its redistribution rights are documented.
- **REQ-010** — Every adapter result keeps mechanics solved, user rule checked and professional approval as distinct states, and states no approval, certification or code compliance (PRD §4.3, §21.2).
- **REQ-011** — No adapter is dispatched from a declaration until an execution model, a plugin runtime and the capability grants have been selected and stated in this scope. Until then the gate dispatches nothing: an invalid or quarantined declaration stops at its outcome, and an accepted declaration is blocked with a finding that names the missing selection. Acceptance of a declaration confers no authority to run. Reading a persisted record is not adapter dispatch.
- **REQ-012** — A malformed declaration, including one that is not strict JSON, is judged with findings and never by an exception, and judging a declaration reads no file, opens no connection and runs no adapter code (decomposition AB-00-03, AB-00-07).
- **REQ-013** — The framework comes with a sample adapter that passes an invented external record through every check of the validation plan. Public sample adapters and fixtures use invented data only, with invented provenance marked as such.

### Acceptance criteria

- **AC-001** — The declaration contract is valid JSON Schema 2020-12 in strict JSON, and the sample invented declaration validates against it and is accepted.
- **AC-002** — A declaration is refused, each time with a finding that carries code, severity, path, message and remediation, when it: turns off any no-bypass control; omits a check of the validation plan; lacks any of the seven provenance members or carries a redistribution status that is not cleared; exposes SQL, a database handle, a table name or a direct store write; allows telemetry or private-data transmission by default; uses a status that states compliance, certification, sealing or approval; or requests a capability outside the list above.
- **AC-003** — A declaration or payload in which protected content is suspected has the quarantine outcome whatever else is wrong with it, and no finding quotes the content it concerns.
- **AC-004** — The dispatch gate dispatches no adapter. A refused or quarantined declaration returns its outcome, an accepted one returns a blocked outcome with a finding naming the unselected execution model, and the gate takes no executor, callback or other argument through which code could run.
- **AC-005** — An operation result validates against the declaration contract, each of its diagnostics has the eight members of REQ-006, its privacy context is local and private unless reviewed as public, and its professional-boundary values state that human review is required and that the software makes no compliance, certification, sealing or approval claim.
- **AC-006** — For every importing adapter the framework governs, an invented external record with each fault of REQ-004 is stopped at the adapter with a diagnostic that names the item, and none of it enters the model. Each unit conversion the adapter performs is reported, and the same input gives the same report.
- **AC-007** — For every exporting adapter the framework governs: a payload holding a protected-suspected value is quarantined and nothing is written; a payload holding a private value bound for a destination outside the local project leaves only as the export control of DEL-12-02 decides, which is stopped, redacted, or written with an explicit warning under the user's stated intent; an export never drops a warning, a limitation or the professional-responsibility notice of the result it exports; and each unit conversion the adapter performs is reported, the same payload giving the same report.
- **AC-008** — A malformed declaration, including one that is not strict JSON, returns findings and raises nothing, and judging any declaration touches no file and no network and runs no adapter code.
- **AC-009** — The sample adapter, its declaration and every fixture are invented, carry their provenance note and contain no protected or private content.
- **AC-010** — Each item of the not-decided list appears in the declaration contract as undecided, and a declaration that settles one of them without a recorded selection is refused.
- **AC-011** — The sample adapter is an importer: it translates an invented external record into the domain contracts through every check of the validation plan, and it is among the importing adapters AC-006 is examined on. It is examined where data crosses it and is not dispatched through the gate.

## Production and Verification Method — Praxeology

**Method choices that define the framework.**

- *Declare, then check, then gate.* An adapter is judged first on what it declares, then on what it does to data, and it runs only past a gate. The three are separate, so that none stands in for another.
- *Format-neutral.* The framework's contract and validator name no format. Format-specific behaviour lives in the format's own deliverable.
- *Quarantine first.* Suspected protected content decides the outcome before any other check is considered.
- *Deny by default.* The gate has no path to execution that a caller can supply.
- *One result shape.* An adapter operation returns the product's diagnostic and result-envelope shape and no shape of its own.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the validator's outcome and findings): presents the sample declaration and invented declarations that each carry one fault of AC-002, AC-003, AC-008 and AC-010, and reads the outcome, the finding codes and their members.
- **VER-002** — Executable check (observes the dispatch gate): presents accepted, refused and quarantined declarations and reads the outcome and the dispatched flag; inspects the gate's signature for any argument that could carry code.
- **VER-003** — Executable check (observes each adapter the framework governs at the point data crosses it): presents invented external records, sound and with each fault of REQ-004, to every such importing adapter, the sample adapter among them, and private or protected-suspected payloads to every such exporting adapter, and reads the diagnostics, the conversion reports and what reached the model or the output.
- **VER-004** — Executable check (observes the operation-result builder's output): validates the built envelope against the declaration contract and reads its diagnostic members, its privacy context and its professional-boundary values.
- **VER-005** — Judgment by a named role (an independent reviewer who did not write the framework): reads the contract, the sample adapter and the fixtures against decomposition rows AB-00-06 and AB-00-07 and PRD §20.1 and §21.2, and reports any protected or private content, any prohibited claim, and any undecided item settled by implication.

## Governing Values and Decisions — Axiology

- **AX-001** — *Narrow shell, mandatory checks.* Better is more of the import and export paths under the framework passing every check of the validation plan, measured by the adapters and faults for which AC-006 and AC-007 hold. Support for another format is never a reason to admit a path that skips a check.
- **AX-002** — *A finding, not a default.* Better is fewer external values that reach the model without a unit, a source or a redistribution status, which is none. User convenience never fills a missing value.
- **AX-003** — *Auditable exchange.* Better is an exported or imported artifact that can be traced: its diagnostics, hashes and warning state travel with it. Speed of export is never bought by leaving them behind.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 OBJ-009 | REQ-001 REQ-013 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 meta-schema decides the contract's validity. That the sample is accepted is defined by this scope |
| OUT-001 | SOW-030 OBJ-009 | REQ-001 REQ-003 REQ-006 REQ-007 REQ-008 REQ-010 | AC-002 | VER-001 | Decomposition AB-00-07 and PRD §20.2 to §20.5 and §21.2 say what a declaration may not do; each invented faulty declaration states the refusal it must receive |
| OUT-001 | SOW-030 OBJ-009 | REQ-004 | AC-003 | VER-001 | PRD §20.2 requires quarantine of suspected protected content; the precedence is defined by this scope |
| OUT-001 | SOW-030 OBJ-009 | REQ-011 | AC-004 | VER-002 | No execution model, runtime or grant is selected, so the expected result of every dispatch attempt is that nothing runs |
| OUT-001 | SOW-030 OBJ-009 | REQ-006 REQ-010 | AC-005 | VER-004 | Decomposition AB-00-06 lists the diagnostic members, and PRD §21.2 the prohibited claims |
| OUT-001 | SOW-030 OBJ-009 | REQ-002 REQ-003 REQ-004 | AC-006 | VER-003 | PRD §18.4 lists the faults an import must flag, and §5.6 requires each conversion to be reported deterministically. Each invented record states what must happen to it, and the comparison is against what reached the model, which the adapter under test did not report. A converted value is expected from the unit definitions of DEL-02-02's catalog, and determinism is one run compared with another |
| OUT-001 | SOW-030 OBJ-009 | REQ-002 REQ-005 REQ-008 | AC-007 | VER-003 | PRD §20.2 and §20.3, and §5.6 for conversions. What happens to a private value is decided by DEL-12-02's export control, not by the adapter. Each invented payload states what must happen to it, and the comparison is against what was written |
| OUT-001 | SOW-030 OBJ-009 | REQ-012 | AC-008 | VER-001 | Decomposition AB-00-03 (an outcome is an envelope, not an exception) and AB-00-07 (no adapter goes around the sandbox); each invented malformed declaration states the refusal it must receive |
| OUT-002 | SOW-030 OBJ-009 | REQ-013 | AC-009 | VER-005 | PRD §20.1 lists what public content may not contain; the reviewer compares |
| OUT-001 | SOW-030 OBJ-009 | REQ-009 REQ-011 | AC-010 | VER-001 VER-005 | The not-decided list of this scope names the selections not yet made; the check and the reviewer compare the contract with it |
| OUT-002 | SOW-030 OBJ-009 | REQ-013 | AC-011 | VER-003 | The validation plan of this scope says which checks the sample passes. The invented record states what must reach the domain contracts, and the comparison is against what reached them, which the sample adapter did not report |
