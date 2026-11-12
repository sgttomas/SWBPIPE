---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-10-01
package_id: PKG-10
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030]
package_objective_refs: [OBJ-009]
---

# Scope of Work — DEL-10-01 Public API and plugin boundary

## Purpose and Objective Traceability

This deliverable is the contract for everything that asks the product to do something from outside it: automation, a plugin, an adapter. It says which operations may be asked for, the form a request and its result take, and which controls no caller can switch off. The result it contributes is that SWBPIPE can be integrated with other tools and extended, and every such integration still passes through unit checks, provenance, diagnostics, the rule sandbox, the private-data controls, the report controls and the professional boundary.

It serves SOW-030 under OBJ-009 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-10, row DEL-10-01, and architecture-basis rows AB-00-03, AB-00-06 and AB-00-07). SOW-030 takes its list of public operation families from a PRD clause that the current `docs/PRD.md` no longer carries as a section; the contract rests on the current PRD's §4.3, §5.3 to §5.6, §5.8, §6.2 item 1, §17.3, §18.4, §20.1 to §20.5 and §21.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The public API and plugin boundary contract: the machine-readable contract `api/api_boundary_contract.yaml` (request and result envelopes, the operation registry, the export-interoperability registry) and its companion document `docs/architecture/plugin_boundary.md`.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Outside caller.* Automation, a plugin or an adapter: anything that is not a first-party module calling another through an internal seam.
- *Operation.* One thing an outside caller may ask for. It has an identifier, a boundary area, a mutability and a set of no-bypass constraints.
- *Operation family.* The seven kinds of public operation listed in the PRD clause SOW-030 cites (v0.1 §19.3): model creation, import and export; load-case definition; solve execution; result extraction; rule-pack evaluation; report generation; and validation-test execution.
- *Command, query, job.* The three request kinds of DEL-00-03. A mutating operation is a command, a read-only operation is a query, and a long-running solve, report, export or validation run is a job with progress, cancellation and reproducibility metadata.
- *Request envelope and result envelope.* The schema-first wrappers every operation uses. A result envelope carries an outcome, diagnostics, provenance, privacy context and checksums. No operation reports its outcome as a bare success or failure.
- *Operation registry.* The list of registered operations, each with the guards it requires.
- *No-bypass constraints.* What every operation must keep in force: domain and schema validation; unit validation; provenance; privacy classification; the result envelope; diagnostics; hash controls; persistence only through application services; report controls; the human-acceptance boundary; the rule sandbox; and the bans on arbitrary code, on compliance claims, on bundled protected content and on transmitting private data by default. This is the set stated once for the package; the adapter framework (DEL-10-02) requires each of them of an adapter.
- *Permission.* A capability a caller requests. Every permission is denied by default, and the contract defines no way to hold one.
- *Export-interoperability concepts.* Export profile, stable ID map, loss report, external-run evidence and adapter SDK, and the target families that use them (stress-neutral export, conservative PCF export, review-geometry export). The contract registers them as boundary areas. Their writers and formats belong to PKG-17.

**Boundary.** This deliverable is a contract. It builds no server, endpoint, transport, plugin loader or runtime.

- DEL-00-07 owns the map of which seams are public and the rule that adapters and plugins bypass nothing. This contract is where that rule is stated for each public operation.
- DEL-02-04 owns the plugin manifest and what an extension may touch in the domain.
- DEL-10-02 owns the adapter framework; the deliverables of PKG-17 own each export format; DEL-10-03 owns the local FEA handoff contract.
- DEL-10-05 owns the command-line runner, the one automation entry the product has.
- DEL-06-02 owns the rule evaluator's sandbox; DEL-05-04 owns the analysis-status vocabulary; DEL-00-06 owns the diagnostic and envelope shape.

**Not decided.**

- The transport of a public API, its endpoint syntax and any OpenAPI binding.
- The API version and its stability level.
- The plugin runtime: how a plugin is loaded, signed and isolated.
- The permission names, and how a grant is given and persists. The names the contract and its document list are candidates.
- Code-generation tooling for the contract.
- Which of the model-creation, load-case-definition and persisted-report families are offered to outside callers.

## Completion and Reliance Basis — Epistemology

The contract is adequate when an implementer of a public operation, or the author of a plugin or adapter, can tell from it alone what form the request and result take and which controls must hold, and cannot find in it a permitted way around one.

### Requirements

Requirement numbers 001 to 014 are those of the earlier `DEL-10-01-REQ-nn` series.

- **REQ-001** — The public API boundary is schema-first and uses command, query, job, diagnostic and result-envelope concepts. No nontrivial operation returns a raw success or failure (decomposition AB-00-03).
- **REQ-002** — The contract classifies the exposed operations into the seven operation families (SOW-030).
- **REQ-003** — A mutating operation of the API or of a plugin goes through an application-service command, or an equivalent boundary that keeps schema validation, unit checks, provenance checks, diagnostic and result envelopes, report controls and reproducibility metadata. No public surface exposes SQL, a database handle, a table name or a direct write to the project store (decomposition AB-00-02, AB-00-04, AB-00-07).
- **REQ-004** — A long-running solve, report, export or validation operation is a job with progress, cancellation, reproducibility metadata and a final result envelope (decomposition AB-00-03).
- **REQ-005** — A model import or export operation validates units, dimensional consistency, provenance, redistribution status, public or private classification and protected-content risk, with diagnostics, before data enters core workflows or a public artifact. Data suspected of protected or proprietary content is stopped and quarantined and never enters a public artifact; a missing unit on dimensional data is a finding, never a default (PRD §5.5, §18.4, §20.2).
- **REQ-006** — The contract and its document embed no protected standards text, tables, examples or formulas, no material allowables, no SIF or flexibility tables, no protected dimensional tables, no proprietary vendor data and no private project or rule data (PRD §20.1).
- **REQ-007** — A path by which an operation contributes public data requires the seven provenance members of PRD §20.2: source name, source location or identifier, source licence, contributor, contributor certification, redistribution status and review status. An unknown value is stated as unknown or blocks the contribution.
- **REQ-008** — A hook that faces rule packs keeps evaluation sandboxed, deterministic and unit-aware. It permits no arbitrary code and no way around the check that required inputs are complete (PRD §17.3).
- **REQ-009** — Where an envelope refers to rule-pack data or results, it carries the rule pack's identity, version, checksum, source notice, redistribution status and private or public marking.
- **REQ-010** — A result envelope carries DEL-00-06's outcome kind (invalid input, incomplete model, mechanics result, rule-check result, human review needed) and DEL-05-04's automatic analysis statuses, so that invalid input, an incomplete model, mechanics solved, incomplete rule inputs, a user rule check evaluated or failed, and human review required are told apart. Human approval appears only as a reference to a human acceptance record that the software did not generate. No status states code compliance (PRD §4.3).
- **REQ-011** — No output of the API, a plugin, an adapter or a report surface claims to certify, seal, approve, authenticate, endorse or declare code compliance for professional reliance (PRD §21.2).
- **REQ-012** — The plugin boundary is denied by default. No capability, private-data access, filesystem, network or process access, bypass of a report or export control, or integration with the rule evaluator exists without a grant. Until a permission and sandbox design states how a grant is given and persists, the contract defines no grant (decomposition AB-00-07; PRD §20.4, §20.5).
- **REQ-013** — What is not decided stays undecided in the contract. Each item of the list above appears as undecided, and no schema value, registry entry or sentence settles one by implication.
- **REQ-014** — A JSON payload that is hashed for a manifest, a job, a model snapshot, a result or a reproducibility record is hashed as SHA-256 over its RFC 8785 canonical form. A non-JSON asset is bound by a manifest hash (decomposition AB-00-04). The owner's disposition of dependency finding `PKG10-DEL1001-PKG02-W001`, on this contract's use of the persistence hash basis, is not decided (also cited as DEL-10-01:2).
- **REQ-016** — Where a model-creation, load-case-definition or persisted-report family is offered to outside callers, it has its own registered entry, and an invalid-input outcome has its own encoding (also cited as DEL-10-01:1). A backend workflow step is not by itself a public operation family.
- **REQ-017** — The export-interoperability concepts and target families are registered as boundary areas under the same no-bypass constraints as every other operation. Registration selects no writer, format, endpoint or runtime (SOW-030).

### Acceptance criteria

- **AC-001** — The contract is valid JSON Schema 2020-12 in strict JSON. An operation record validates only when it carries a category, a registered identifier, a boundary area, a mutability and its no-bypass constraints, and a record that names a transport is refused.
- **AC-002** — The contract represents each of the seven operation families of REQ-002, and every registered operation belongs to one of them. A model-creation, load-case-definition or persisted-report family that is offered to outside callers has its own registered entry, and the invalid-input outcome has its own encoding. Every registered operation is exactly one of command, query or job, and every long-running operation is a job whose envelope has progress, cancellation and reproducibility members.
- **AC-003** — Every registered operation names the guards it requires. The contract enumerates the no-bypass constraints of the Ontology, and every registered operation keeps each of them in force. Every mutating operation and every operation that touches stored data is constrained to application services, and nothing in the contract or its document offers SQL, a database handle, a table name or a direct store write.
- **AC-004** — The result envelope requires diagnostics with code, class, severity, source, affected object, message, remediation and provenance. Its status values are the six automatic statuses of DEL-05-04, and it carries an outcome kind with DEL-00-06's five values. It has no value for compliance, certification or approval, and human acceptance appears only as a reference marked as not generated by the software.
- **AC-005** — The contract requires an import or export operation to validate units, dimensional consistency, provenance, redistribution status and public or private classification and to return a diagnostic for each gap, a missing unit included. A path that contributes public data requires the seven provenance members of PRD §20.2. Suspected protected content ends in quarantine and reaches no public artifact. An envelope that refers to a rule pack requires its identity, version, checksum, source notice, redistribution status and private or public marking.
- **AC-006** — Every permission is denied by default, and while how a grant is given and persists is undecided, it is recorded as undecided and no permission can be in a granted state. Telemetry is off by default and is never enabled without the user's explicit action, and no operation transmits private data by default.
- **AC-007** — Each item of the not-decided list appears in the contract as undecided, and neither the contract nor its document names a transport, an endpoint, a runtime, a loader or a granted permission.
- **AC-008** — The contract declares the hash of a JSON payload as SHA-256 over RFC 8785 canonical JSON, and the checksum of any example it carries is reproduced by an implementation that imports no product code.
- **AC-009** — The contract, its document and any example contain no protected or private content and no claim of compliance, certification, sealing or approval.
- **AC-010** — Every rule the companion document states is carried by a schema constraint or a registry entry of the contract, and the document decides nothing the contract leaves undecided.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- *One file, strict JSON.* The contract is a single JSON Schema 2020-12 document written in strict JSON, so that it parses without a YAML dependency and validates with any conforming implementation.
- *Transport-neutral.* Envelopes and the registry are specified without a transport. The same contract serves an in-process call, a command-line request or a later network binding.
- *Registry beside schema.* The schema fixes the shape of a record; the registry lists the operations and their guards. An operation exists publicly only when it is registered.
- *Borrowed vocabularies.* Diagnostic fields, analysis statuses and request kinds are those of DEL-00-06, DEL-05-04 and DEL-00-03. The contract defines none of its own where one of those exists.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the contract file): parses it as strict JSON, validates it against the JSON Schema 2020-12 meta-schema, and validates invented operation records that should pass and that should be refused.
- **VER-002** — Executable check (observes the operation registry and the envelope definitions): counts the operations of each family, the request kind, guards and no-bypass constraints of each operation, the envelope's required members, outcome kinds and status values, what an import, export or contribution operation must validate, the permission and telemetry defaults and the undecided items.
- **VER-003** — Comparison with a named reference: the checksum of each example against SHA-256 over RFC 8785 canonical JSON computed by an implementation that imports no product code.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the contract): reads the contract and its document against decomposition rows AB-00-03, AB-00-06 and AB-00-07, DEL-00-07's boundary map and the PRD clauses above, and reports each rule that is missing, added or weakened, each disagreement between contract and document, each undecided item settled by implication, and any protected content or prohibited claim.

## Governing Values and Decisions — Axiology

- **AX-001** — *A small, fully guarded surface.* Better is fewer public operations and permissions, each with every guard it needs, measured by the number of registered operations that lack a guard or a no-bypass constraint, which is zero. Ease of integration is never a reason to register an operation without them.
- **AX-002** — *Undecided stays visible.* Better is a contract an implementer can build against today with fewer places where an unmade choice is assumed, measured by the undecided items that some schema value or sentence settles by implication. This is never bought by inventing endpoint syntax, permission grants or a runtime.
- **AX-003** — *One vocabulary.* Better is fewer terms that this contract defines for itself where a neighbour already defines them, measured by enumerated values with no stated mapping to DEL-00-06, DEL-05-04 or DEL-00-03. A local value is never allowed to weaken the meaning of the one it maps to.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 OBJ-009 | REQ-001 | AC-001 | VER-001 | Outside source: the JSON Schema 2020-12 meta-schema decides validity. The pass and refuse cases are defined by this scope |
| OUT-001 | SOW-030 OBJ-009 | REQ-002 REQ-004 REQ-016 REQ-017 | AC-002 | VER-002 | The seven families of the PRD clause SOW-030 cites, which the current PRD does not carry, and decomposition AB-00-03 for the request kinds. The check counts the registry against them |
| OUT-001 | SOW-030 OBJ-009 | REQ-003 | AC-003 | VER-002 VER-004 | Decomposition AB-00-02, AB-00-04 and AB-00-07 say what must hold; a reviewer who did not write the contract reads it against them |
| OUT-001 | SOW-030 OBJ-009 | REQ-010 REQ-011 | AC-004 | VER-002 | PRD §4.3 and §21.2 list what software may and may not say. The field, outcome-kind and status sets are DEL-00-06's and DEL-05-04's, not this contract's |
| OUT-001 | SOW-030 OBJ-009 | REQ-005 REQ-007 REQ-008 REQ-009 | AC-005 | VER-002 | PRD §18.4 lists the import warnings and §20.2 the provenance members of a public data record and the quarantine of suspect data |
| OUT-001 | SOW-030 OBJ-009 | REQ-012 | AC-006 | VER-002 | Decomposition AB-00-07 and PRD §20.4 and §20.5 say what is denied by default; the check reads each permission and default against them |
| OUT-001 | SOW-030 OBJ-009 | REQ-013 | AC-007 | VER-002 VER-004 | The not-decided list of this scope names the choices not yet made; the check and the reviewer compare the contract with it |
| OUT-001 | SOW-030 OBJ-009 | REQ-014 | AC-008 | VER-003 | Outside source: RFC 8785 and SHA-256, computed by an implementation that imports no product code |
| OUT-001 | SOW-030 OBJ-009 | REQ-006 REQ-011 | AC-009 | VER-004 | PRD §20.1 and §21.2 list the prohibited content and claims; the reviewer compares |
| OUT-001 | SOW-030 OBJ-009 | REQ-001 REQ-013 | AC-010 | VER-004 | The contract says what the document may state; the comparison is made by a reviewer who wrote neither |
