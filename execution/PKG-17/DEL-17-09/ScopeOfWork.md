---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-09
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-09 Export adapter SDK and additional targets

## Purpose and Objective Traceability

This deliverable is the contract by which a further export target is admitted to SWBPIPE: what someone proposing a target must bring, the states a target passes through, and the checklist it is examined against. The result it contributes is that the set of export targets can grow, by the project or by a contributor, without any target weakening what the first ones keep: an admitted source for every statement about the target, identity that survives the export, a loss report on every export, no access the user did not grant, and no claim the software cannot make. A target's name is not evidence, and a record in this contract runs nothing.

It serves SOW-030, SOW-074 and SOW-075 under OBJ-009, OBJ-017 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-09: adapter extensibility after the common profile and package contracts), with `docs/PRD.md` §5.8, §16.2, §19.3 and FR-HAND-005 (Could). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The export adapter admission contract: the schema `schemas/export_adapter_sdk.schema.json`, the builder that forms a registry of adapter and target records with their checklists and reports what is wrong with one, and the invented template a contributor starts from.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Adapter record.* The declaration of one export adapter: the target it writes, its target version basis, unit policy, coordinate policy, stable-ID policy, source-basis references, loss-report policy, diagnostics policy and boundary notes. These are the fields of an export profile under the common contract (DEL-17-02), restated for a target not yet built.
- *Target record.* The registry's entry for one target: its name, its admission state, the sources offered for it and their standing, its unresolved questions, and the statement that no support is claimed.
- *Admission state.* Where a target stands. A state says what evidence exists; none is a state of release, compatibility or acceptance.
  - *candidate*: named for possible review, nothing claimed;
  - *source-basis pending*: no public or rights-cleared source is admitted yet, and all target behaviour is undecided;
  - *source-basis admitted*: sources are recorded with their boundaries;
  - *contract-ready*: the adapter record is complete against its sources, and a writer for the target may be built in a deliverable of its own;
  - *rejected* or *quarantined*: the sources offered are protected, proprietary, private or of unclear redistribution standing.
- *Source record.* One source offered for a target: its location, its licence or redistribution basis, the use it is admitted for, its boundary notes, the target behaviour it bears on and its review standing. Admitted sources are public official documentation, public specifications, project-owned material and user-provided material with rights.
- *Checklist.* The examination of one target before admission, one entry per topic: source-basis completeness; target version basis; units and coordinate conventions; stable ID mapping and sidecars; package member inventory; manifest and hash policy; loss-report categories; diagnostics and severity routing; privacy and protected-content screening; ownership of external execution where there is any; and absence of prohibited wording. An entry states what was examined, the evidence, and a status of pass, fail, undecided or not applicable.
- *Denied grant.* A kind of access an adapter might ask for: filesystem, network, process, private data, storage, rule pack, solver or report control. A record may list what an adapter would need; listing grants nothing.
- **CLM-018** — *Computed hash.* A checksum this contract's packages compute over a JSON record is SHA-256 over the record serialized with sorted keys, compact separators and ASCII escaping, labelled `deterministic_sorted_compact_json_payload_hash`, and the label asserts no RFC 8785 conformance.

**Boundary.** This deliverable owns the admission contract, its schema, builder, template and fixture. It admits no target and writes no target format.

- The package, profile, stable ID map, manifest and loss report an adapter must produce are DEL-17-02's. The register of admitted sources is DEL-17-01's. Each built target is its own deliverable's (DEL-17-03 to DEL-17-08).
- The generic adapter declaration, its validator and the dispatch gate are DEL-10-02's. The plugin manifest and its no-bypass controls are DEL-02-04's. The public API boundary, with its permissions and grant states, is DEL-10-01's. This contract adds what an export target must show and decides nothing those contracts leave undecided.
- External-run evidence and the external-prover record are DEL-17-05's and DEL-15-04's. The desktop panel that presents the registry is DEL-07-12's.

**Not decided.**

- The runtime: how an adapter is loaded and run, the permission taxonomy, the approval of sandbox capabilities, and who owns a grant record (DEL-10-01, DEL-10-02).
- How REQ-007's eleven distinctions become checklist categories, whether mechanics readiness and rule-check readiness are categories of their own, the reviewer's role, the form of a signoff and the approval artifact. These are the owner's to select. Until the owner selects them, no checklist category or record field is added for them, and no existing entry or flag is read as supplying one. A record's flag that human review is required is a boundary flag; it names no reviewer and records no acceptance.
- Who admits a source for a new target, and where that ruling is recorded.
- The hash basis. CLM-018 gives this contract a project-local computed hash that is not RFC 8785, while the common contract (DEL-17-02/REQ-007) requires the project's canonical JSON basis. Which governs here is the owner's to settle.
- Which further targets are proposed. None is named by this scope.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The contract uses DEL-17-02's package, profile, stable ID map, manifest and loss report as they are, and redefines none of them incompatibly.
- **REQ-002** — A further target has its own admitted source basis before any behaviour of that target is described as supported, compatible, validated or ready for use.
- **REQ-003** — Target behaviour that is unknown, unsupported, version-sensitive or without sufficient source is undecided, unsupported, omitted, approximated or delegated, and never a support claim.
- **REQ-004** — An adapter record declares its target version basis, unit policy, coordinate policy, stable-ID policy, source-basis references, loss-report policy, diagnostics policy and boundary notes.
- **REQ-005** — A target preserves canonical identity directly or through a sidecar map where its artifacts cannot carry SWBPIPE identifiers.
- **REQ-006** — Every target adapter produces a loss report on every export, including one that lost nothing.
- **REQ-007** — The examination of an adapter distinguishes syntactic readiness, schema validation, unit and dimensional validation, provenance completeness, redistribution review, privacy classification, protected-content screening, mechanics readiness, rule-check readiness, export review and human review.
- **REQ-008** — No adapter or target record grants filesystem, network, process, private-data, report-control, solver, rule-pack or storage access by default.
- **REQ-009** — Public templates and target examples use invented or rights-cleared data only, and hold no protected standards data, proprietary commercial example, copied vendor file or private project data.
- **REQ-010** — External execution, where a target needs it, is optional, owned by the user, bound by the user's licence, and recorded as non-authoritative external-run evidence.
- **REQ-011** — No output, manifest, diagnostic, document or checklist of an adapter states certification, sealing, approval, authentication, endorsement, code compliance, professional acceptance, formal solver validation, release readiness or vendor compatibility (`docs/PRD.md` §21.2).
- **REQ-012** — A record authorises nothing at run time: no loader, grant, endpoint, external execution or target writer follows from a record's presence or state.
- **REQ-013** — Every adapter record carries source-basis references to DEL-17-01 and DEL-17-02, and a target in the state source-basis admitted carries source evidence specific to that target beside them.
- **REQ-014** — When a runtime loader is selected (DEL-10-01, DEL-10-02), an admission package is bound to it and carries complete provenance detail; until then the declaration gate dispatches nothing.

### Acceptance criteria

- **AC-001** — A registry built from the invented template validates against the schema. An adapter record that lacks any declaration REQ-004 names, or either reference REQ-013 names, is refused with a finding that names what is missing, and a record that redefines a member of the common contract is refused.
- **AC-002** — A target record validates only in one of the admission states. A target stands at source-basis admitted or contract-ready only with at least one source record specific to it whose standing is admitted; a target whose offered source is protected, proprietary, private or of unclear standing stands at rejected or quarantined. Every target record below contract-ready states that no support is claimed, and every behaviour without a source is recorded under one of REQ-003's classes.
- **AC-003** — A checklist validates only with an entry for every topic of the checklist, each with its status and evidence references. An entry's status speaks only for what that entry examined: passing it establishes nothing about a distinction of REQ-007 that it did not examine. A target reaches contract-ready only when every entry required before admission has passed.
- **AC-004** — Every denied grant is recorded as denied in every registry. A record that marks a grant as given, or that requests an access outside the eight kinds, is refused. In no state does a record cause an adapter to be loaded, run or given access: offered to the adapter framework's gate (DEL-10-02), an adapter whose target is contract-ready is dispatched no more than any other. An admission package offered to a selected runtime loader is admitted only when it is bound to that loader and carries complete provenance detail.
- **AC-005** — Each export target the product builds (native JSON, CAEPIPE MBF, stress-neutral, PCF and review geometry) has an adapter record and a target record that validate against this contract, with the sources its own scope cites. The registry the builder forms for these targets is complete and valid against the schema, so that a consumer can show it without generating its own.
- **AC-006** — Each computed hash of a registry carries its label and equals SHA-256 over the serialization CLM-018 names, as recomputed without product code; changing any value of a record changes its hash; and two builds from the same inputs are byte-identical.
- **AC-007** — The template and fixture are invented and carry their provenance. A record that describes external execution marks it optional, user-owned and non-authoritative, and one that does not is refused. No record, finding or example states a claim REQ-011 names, and a record that carries one is refused with a finding naming the field.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- Admission is by evidence. A target moves forward only by adding a source or passing a checklist entry, never by being named, requested or popular.
- Fewer targets with strong sources are preferred to many targets listed as candidates and read as supported.
- Every export is treated as lossy until its loss report says otherwise; the manifest, ID map, diagnostics and loss report are part of the export and not extras.
- Options of a target are passed through as that target's configuration and never become checking logic of SWBPIPE.
- The contract is declarative. It holds no code that loads or runs an adapter.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the registry built from the invented template against the schema, and adapter records with each declaration and each reference removed and with a redefined contract member).
- **VER-002** — Executable check (observes target records in each admission state, with and without a specific admitted source, with a source of each refused standing, and with an unsourced behaviour claimed as supported).
- **VER-003** — Executable check (observes checklists with each topic removed, and a target moved to contract-ready with a required entry failed or undecided) and judgment by a reviewer who did not write the contract: examines whether any entry, status or admission state reads as establishing a distinction of REQ-007 that the entry did not examine.
- **VER-004** — Executable check (observes registries with a grant marked as given and with an unknown access kind, the outcome of the adapter framework's gate for a contract-ready target's adapter, and a selected runtime loader's response to an admission package that is unbound or lacks provenance detail).
- **VER-005** — Executable check (observes the records of the product's own export targets against the schema, the sources they cite against each target's scope and DEL-17-01's register, and the registry the builder forms for them against the schema).
- **VER-006** — Comparison with a named reference (a recomputation of SHA-256 over sorted-key compact JSON written without product imports) and executable check (observes two builds and a mutated record).
- **VER-007** — Executable check (the project's protected-content scan over the schema, template and fixture, and records seeded with a prohibited claim and with an external execution marked as required) and judgment by a reviewer who did not write them: examines provenance and wording.

## Governing Values and Decisions — Axiology

- **AX-001** — *Evidence admits, names do not.* Better is a larger share of every listed target's behaviour tied to an admitted source or to a stated classification. A longer list of targets is never bought with a weaker source.
- **AX-002** — *The same controls for every adapter.* Better is fewer ways in which a contributed adapter is treated differently from the product's own. Convenience for a contributor never removes a check, a loss report or a boundary note.
- **AX-003** — *Declare, do not run.* Better is fewer record fields whose value could be read as permission to load, run or access anything. Extensibility is never reached by letting a declaration grant access.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-001 REQ-004 REQ-005 REQ-006 REQ-013 | AC-001 | VER-001 | The schema is examined with a JSON Schema 2020-12 validator; records seeded with each omission are read against what the common contract (DEL-17-02) says a profile declares, and the expected result for each is refusal |
| OUT-001 | SOW-030 OBJ-009 OBJ-018 | REQ-002 REQ-003 REQ-013 | AC-002 | VER-002 | Records seeded with each source standing, read against `docs/IP_AND_DATA_BOUNDARY.md` on which sources may be admitted; the seeded records state what must be refused |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-007 | AC-003 | VER-003 | The seeded checklists, and the reading of a reviewer who did not write the contract against the distinctions `docs/SPEC.md` §4.5 names |
| OUT-001 | SOW-030 SOW-075 OBJ-009 OBJ-018 | REQ-008 REQ-012 REQ-014 | AC-004 | VER-004 | The seeded registries, read against `docs/SPEC.md` §4.5 (denied by default); the gate's outcome is DEL-10-02's, which this contract did not write |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-001 REQ-002 REQ-004 | AC-005 | VER-005 | The decomposition assigns each built target to its deliverable, whose scope names its sources; the records are compared against those scopes, DEL-17-01's register and the schema, none of which this contract wrote |
| OUT-001 | SOW-074 OBJ-017 | CLM-018 | AC-006 | VER-006 | Outside source: SHA-256 over a serialization any JSON library reproduces, recomputed without product code; byte equality of two builds guards against change only |
| OUT-001 | SOW-075 OBJ-017 OBJ-018 | REQ-009 REQ-010 REQ-011 | AC-007 | VER-007 | The scan and the seeded records, which show only that what was seeded is refused, and the reviewer's reading against `docs/PRD.md` §16.2 and §21.2 and `docs/IP_AND_DATA_BOUNDARY.md` |
