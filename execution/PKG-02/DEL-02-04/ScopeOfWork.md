---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-04
package_id: PKG-02
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-038]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-02-04 Plugin and extension domain contracts

## Purpose and Objective Traceability

The product can be extended without an extension becoming a way around it. A plugin or adapter may translate data in and out, add checks, or connect a rule pack, but only through boundaries that keep units, provenance, diagnostics, report controls, rule sandboxing and the public/private data boundary in force, and it holds no capability it was not explicitly granted. This deliverable states that contract: what an extension may declare, what it may never switch off, and how a declaration is judged before anything is trusted.

It serves SOW-038 under OBJ-009 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-02, row DEL-02-04, and the architecture basis AB-00-07). The current PRD has no clause on plugins: the PRD passage SOW-038 cites has no counterpart in it. The contract rests on the PRD clauses an extension must not weaken: §5.5, §5.6, §17.3 (an expression evaluator without filesystem or network access), §20.1 to §20.5 and §21.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The plugin and extension domain contract: the manifest schema, the domain contract document, and the verification of a declaration against them.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Plugin and adapter.* Code outside the product core that translates at a boundary. Neither owns the domain core, the solver, project storage, provenance policy, privacy policy or professional-boundary language.
- *Extension point.* A named place where an extension may attach. The families are schema categories, not a registry of approved points: import adapter, export adapter, report or output extension, validation hook, diagnostic provider and rule-pack integration hook.
- *Manifest.* What a plugin declares about itself. A manifest requests; it grants nothing.
- *Entrypoint.* One declared attachment: its extension point, the domain surface it touches, and its mode, which is a command, a query, a job or metadata only. Every entrypoint requires the governed boundary.
- *Governed boundary.* An application-service command, query or job, or an equivalent that keeps schema validation, unit checks where values are dimensional, diagnostics and result envelopes, provenance and data-boundary checks, report controls, and audit and reproducibility metadata.
- *Permission.* A capability a manifest requests. Every capability is denied by default; how a capability is granted, consented to and revoked is undecided. The candidate classes are governed project mutation, read-only query, private-library access, filesystem path access, network access, report and export generation, background job execution and rule-pack evaluator integration.
- *No-bypass controls.* The controls no manifest can turn off: units, provenance, privacy, rule sandboxing, the analysis boundary, persistence, schema validation, the diagnostic envelope, checksums, protected content, the report boundary, the solver boundary and human acceptance.
- *Quarantine.* The disposition of content suspected to be protected or proprietary: it is held, classified and escalated for human review, and is not imported into a public artifact.
- *Declaration verification.* The judgment of a manifest, an adapter declaration and an operation result against the contract. It reads data only: it loads no plugin, dispatches no adapter, reads no file and opens no connection.

### CLM-007 — Parts of the contract

- **The manifest schema**, `schemas/plugin_manifest.schema.yaml`, with a slot for each concept: plugin identity, version and review status; extension-point declarations; schema and envelope version; capability requests; provenance and data-boundary declaration; privacy declaration; diagnostics compatibility; checksums; sandbox declaration; the no-bypass controls; and the professional-boundary statement.
- **The domain contract document**, `docs/architecture/extension_domain_contracts.md`: what an extension may touch in the domain model and which controls it can never bypass.
- **Declaration verification**, which returns its findings in a result envelope.

**Undecided.** The transport for a public extension API; the approved extension-point registry and who owns it; the permission names, the sandbox mechanism, the route by which a capability is granted, and consent and revocation; which import and export formats a plugin may bind to; and whether a plugin may ever receive telemetry-facing or private engineering data. The default for the last is no, and telemetry-facing data can reach a plugin only under a telemetry design approved as DEL-12-03 requires.

**Boundary.** This deliverable is a contract at the domain and API level. It builds no plugin loader, no runtime and no transport. The adapter framework and its declarations are DEL-10-02's; the import and export formats are PKG-17's; the API boundary contract is DEL-10-01's; the rule evaluator's sandbox is DEL-06-02's; threat modelling of extensions is DEL-12-05's; private-data controls are PKG-12's. The embedded agent is not a plugin under this contract; it is DEL-16-06's.

## Completion and Reliance Basis — Epistemology

A declaration that verifies shows that what a plugin says about itself is within the contract. It does not show what the plugin does. Behaviour at run time can be examined only where a plugin runtime exists, and no declaration check stands in for that examination.

### CLM-012 — Requirements

Requirement numbers 001 to 018 are those of the earlier `DEL-02-04-REQ-nn` series.

- **REQ-001** — An extension attaches only at a declared extension point, and no extension point lets it go around governance, validation, diagnostics, unit safety, report controls or the public/private data boundary.
- **REQ-002** — Data entering or leaving through an extension is unit-aware wherever it is dimensional or numerical. Missing or incompatible unit metadata is reported and never defaulted.
- **REQ-003** — An extension preserves source and provenance for material, component, rule, load, allowable, SIF and flexibility, and report-affecting values.
- **REQ-004** — A path by which an extension contributes public data requires the source, its location, the licence or redistribution status, the contributor's certification and the review disposition. A missing value is stated as unknown or the contribution is rejected.
- **REQ-005** — No extension redistributes publicly any protected standards text, tables, examples, copied code formulas, protected dimensional data, proprietary vendor data held without rights, or private rule, project or library data.
- **REQ-006** — Content suspected to be protected or proprietary is quarantined and escalated for human review instead of entering a public artifact.
- **REQ-007** — An extension's diagnostics use the product's diagnostic and result envelope: code, class, severity, source, affected object, message, remediation and provenance.
- **REQ-008** — No output or report of an extension claims certification, sealing, approval, authentication, endorsement or code compliance for reliance (PRD §21.2).
- **REQ-009** — A mutating operation of an extension goes through a governed boundary. An extension has no direct path to the domain core, the solver, report controls or project storage: no SQL, no database handle, no table name and no direct store write.
- **REQ-010** — A read-only or long-running operation of an extension keeps the distinction between command, query and job, with cancellation, progress and reproducibility metadata where they apply.
- **REQ-011** — The manifest and every interchange contract of an extension are JSON Schema 2020-12 with schema-first envelopes.
- **REQ-012** — A hash of a manifest, a provenance record or a reproducibility artifact is taken over canonical JSON with JCS-compatible canonicalization.
- **REQ-013** — A hook that faces rule packs is sandboxed, deterministic and unit-aware, and can run no arbitrary code and reach no filesystem or network.
- **REQ-014** — A declaration is verified in layers: schema conformance, unit safety, provenance, diagnostics, protected content, and regression of adapter and plugin behaviour.
- **REQ-015** — What is undecided stays visible in the contract. The transport, plugin bindings to import and export formats, the sandbox mechanism and permission grants are carried as undecided, and nothing in the contract settles them by implication.
- **REQ-016** — The manifest has a slot for every concept of CLM-007.
- **REQ-017** — The contract grants no capability by default. Registry entries, permission names, the sandbox mechanism and the granting route take effect only once decided.
- **REQ-018** — By default an extension receives no telemetry-facing data and transmits no private project, rule-pack, component, material or result data. An extension receives telemetry-facing data only under an approved telemetry design (DEL-12-03), and only once that exposure is verified.
- **REQ-019** — Nothing in the product loads, dispatches or executes a plugin, and a manifest in any state confers nothing (also cited as DEL-02-04:2). A plugin runtime, with its transport, capability granting and permission persistence, enters only by a change of scope.

### Acceptance criteria

- **AC-001** — The manifest schema is valid JSON Schema 2020-12 in strict JSON, and the public invented manifest validates against it (also cited as DEL-02-04:1). A manifest is refused when it turns off any no-bypass control; allows direct SQL, a database handle, table-name coupling or a direct store write; allows arbitrary code execution or process spawning; enables telemetry or private-data transmission by default; omits a slot of CLM-007; or names a transport.
- **AC-002** — Only a complete, valid manifest and adapter declaration verify. A missing, unknown or dimensionally incompatible unit; missing or uncleared provenance; a disabled control; and an incompatible diagnostic envelope each produce a finding with the fields of REQ-007. Suspected protected content quarantines, and the quarantine takes precedence over every other finding. Malformed or hostile input produces bounded findings and never an exception, and verification dispatches nothing.
- **AC-003** — The manifest checksum of the invented manifest is the SHA-256 of the RFC 8785 canonical text of the manifest with its own checksum value left out: it is reproduced by an independent implementation, is unchanged by key order, and changes with any other change of content.
- **AC-004** — No code path in the product loads, dispatches or runs a plugin, and a manifest that requests capabilities holds none.
- **AC-005** — Every entrypoint a manifest declares names a command, query, job or metadata-only mode and requires the governed boundary, and a persistence-facing entrypoint admits application-service access only.
- **AC-006** — The contract's schema, document and fixtures contain no protected or private content and no claim of compliance, certification or approval, and each undecided item of REQ-015 appears as undecided.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- The contract is schema-first: what an extension may be is what the manifest schema admits. A control that must hold is a constant in the schema, not a value a manifest supplies.
- Deny by default: a capability exists only after an explicit grant.
- Verification is pure. It judges already-loaded data, takes its own bounded copy of what the caller supplies before reading it, and fails closed.
- Fixtures are invented.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-005, executable check: observes the schema, the invented manifest and constructed invalid manifests under a JSON Schema 2020-12 validator that imports no product code.
- **VER-002** — AC-002, executable check: observes the findings and result envelope for constructed valid, invalid, malformed and hostile declarations, and that nothing is dispatched.
- **VER-003** — AC-003, comparison with a named reference: RFC 8785 canonicalization and SHA-256, computed by an implementation that imports no product code.
- **VER-004** — AC-004, judgment by a reviewer who did not write the contract, with an executable check: the reviewer examines the product for any path that loads, dispatches or runs a plugin, and the check observes that a manifest requesting capabilities is granted none.
- **VER-005** — AC-006, judgment by a reviewer who did not write the contract, with the protected-content scan: examines the schema, document and fixtures.

## Governing Values and Decisions — Axiology

- **AX-001** — *No way around.* Better is fewer controls an extension could weaken; the target is none. Stricter validation and an explicit diagnostic are preferred to broader extension authority, and extensibility is never bought with a bypass.
- **AX-002** — *Least privilege.* Better is fewer capabilities held without an explicit grant, starting from none. Ease of writing a plugin never justifies a default grant.
- **AX-003** — *Private stays under the user's control.* Flexibility of import and export never moves data out of private control without provenance, redistribution status, unit checks and the protected-content gate.
- **AX-004** — *Diagnostics that can be audited.* A structured finding with its source and remediation is preferred to a terse failure, since an extension's failures are part of the record a reviewer needs.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-038 OBJ-009 | CLM-007 REQ-001 REQ-009 REQ-011 REQ-013 REQ-016 REQ-017 REQ-018 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code. Which manifests must be refused is stated by the criterion from SOW-038 and the architecture basis AB-00-07 |
| OUT-001 | SOW-038 OBJ-009 | REQ-002 REQ-003 REQ-004 REQ-005 REQ-006 REQ-007 REQ-014 | AC-002 | VER-002 | The declarations are constructed, so the criterion states the expected finding for each; unit and dimension membership come from the unit catalog (DEL-02-02) |
| OUT-001 | SOW-038 OBJ-009 | REQ-012 | AC-003 | VER-003 | Outside source: RFC 8785 and SHA-256, computed without product code; that the hashed form is SHA-256 over RFC 8785 is DEL-00-04's rule |
| OUT-001 | SOW-038 OBJ-009 | REQ-017 REQ-019 | AC-004 | VER-004 | The criterion states the expected result: nothing is found. The reviewer examines the product's source, not a record the product wrote |
| OUT-001 | SOW-038 OBJ-009 | REQ-009 REQ-010 | AC-005 | VER-001 | The manifests are constructed, so the criterion states which are valid. It examines the declaration only; behaviour at run time has no examination while no runtime exists |
| OUT-001 | SOW-038 OBJ-009 | REQ-005 REQ-008 REQ-015 | AC-006 | VER-005 | PRD §20.1 and §21.2 and `docs/IP_AND_DATA_BOUNDARY.md`; the reviewer compares against sources the product did not write |
