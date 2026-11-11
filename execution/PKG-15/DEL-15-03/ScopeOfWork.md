---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-15-03
package_id: PKG-15
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-074]
package_objective_refs: [OBJ-017]
---

# Scope of Work — DEL-15-03 Downstream modeling export workflow

## Purpose and Objective Traceability

This deliverable is the export path: the acts by which an engineer takes a model out of SWBPIPE to rebuild or check it elsewhere. The engineer saves the canonical model document as a file, saves a handoff package that carries the model with the manifest DEL-15-01 defines, and, for a downstream target, joins a package to a target mapping so that everything the target cannot carry is stated. The path is generic: it knows no commercial tool's format. The result it contributes is that the model an engineer hands on is exactly the model they were working on, identified by its hash, with its units, references, warnings and open assumptions beside it, and that a failed or refused export leaves nothing half-written.

It serves SOW-074 under OBJ-017 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-15, row DEL-15-03), and `docs/PRD.md` §5.8, §16.1, §16.2, FR-HAND-001 and FR-HAND-002, with FR-HAND-003 and FR-HAND-004 for the target step. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The target-neutral export step: a handoff package and a DEL-15-02 target mapping assembled into one controlled export record for a downstream target, with every unsupported or approximate behaviour stated.
- **OUT-002** — The canonical model export: saving the session's model document as a file.
- **OUT-003** — The handoff package export: producing a package that conforms to DEL-15-01 from the session's model, and publishing it to a location the user chooses.

## Deliverable Definition — Ontology

**Terms the export path implements.**

- *Canonical model file.* The session's model document written as its RFC 8785 canonical text with no trailing newline, so that SHA-256 of the file is the session's model hash. It is a model document, not a package.
- *Handoff package.* As DEL-15-01 defines it: the model member, which is the canonical model file's bytes, and the manifest.
- *Basis state.* The saved model state the package is based on: an existing state of the project whose model hash is the session's, or a new state of kind handoff basis that the user confirms and that is saved before the package is published.
- *Publication.* Making a package appear under its name. It is atomic: the name holds the previous package, or nothing, until it holds the complete new one.
- *Saved.* The outcome reported only when the bytes under the name have been read back and are the bytes the export verified.
- *Refusal.* An export that writes nothing and returns named diagnostics. Cancelling is not a refusal.
- *Blocking disposition.* A provenance, library or rule-pack record marked as suspected protected content, quarantined or rejected.
- *Export record.* The output of the target step: the package's content, the mapping records, the unsupported-target records and the diagnostics, returned through the export control for the downstream-tool context (DEL-12-02).
- *Unsupported-target record.* One statement of a behaviour the target does not carry exactly, taken from the manifest's flags, from the mapping's unsupported and approximate flags, or from the target's declared unsupported capabilities. Each requires human review.
- *Invented target.* A made-up target description with declared capabilities, used to exercise the target step without any commercial tool.

**Boundary.**

- DEL-15-01 states what a correct package is and how one is verified; this deliverable produces packages that meet it.
- DEL-15-02 owns mapping records and the taxonomy of unsupported and approximate behaviour; the target step carries them and adds none.
- DEL-15-04 owns the screen for wording that claims authority.
- DEL-02-01 owns the model document's format and validator; DEL-14-01 the saved states; DEL-12-02 the export contexts, the route catalogue and the native disposition screen; DEL-08-02 the canonical JSON form.
- The model document save writes its file through the atomic file writer of the report package save, which is DEL-08-01's. No deliverable currently commits to providing that writer to other callers; REQ-012 holds this deliverable's saves to all or nothing whichever writer they use.
- DEL-07-11 owns the File menu and the dispatch of its commands; DEL-07-12 owns the Exports panels, whose handoff summary is a preview and not a handoff package.
- PKG-17 owns every target format: MBF, PCF, stress-neutral, native JSON and review geometry. No commercial-tool parser, result ingestion or external solver invocation is on this path.

**Not decided.**

- Where the target step runs. Nothing in the PRD or the decomposition says whether it is a command a user runs, or a step each target exporter of PKG-17 runs for its own target.
- The physical form of a package (see DEL-15-01).

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The export path generates schema-compliant handoff packages (SOW-074; PRD FR-HAND-001).
- **REQ-002** — What the path exports includes the model hash, the units manifest, the entity identifiers, the library and rule-pack references, the unresolved assumptions, the warnings, the target-mapping metadata and the unsupported-target flags (SOW-074; PRD FR-HAND-002, §16.1).
- **REQ-003** — The path is generic. It implements no target-specific commercial-tool parser or writer and is exercised on an invented target (SOW-074; decomposition OI-015).
- **REQ-004** — Unit metadata crosses the export boundary unchanged. A unit the export cannot state is a named refusal, never an assumed unit (PRD §5.6; `docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-005** — The path preserves diagnostics, warnings, unresolved assumptions, provenance and hashes, and hides no missing data or boundary risk. A library or rule-pack reference that does not resolve in the local store stays listed and carries a warning that names it (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-006** — Nothing the path writes or says claims certification, sealing, approval, authentication, professional acceptance or code compliance (PRD §21.2).
- **REQ-007** — Fixtures and examples use invented or cleared data and copy no protected standards content, commercial software example or proprietary data (PRD §20.1).
- **REQ-008** — The path respects the schema-first contracts. A checksum it did not compute is carried with its algorithm, scope, digest and canonicalization label unchanged, and carrying one asserts no recomputation, validity or RFC 8785 conformance. A digest the path computes is labelled with the basis it was computed on (decomposition DEC-010, DEC-074).
- **REQ-009** — The path takes handoff, mapping, export-control and model context only through their contracts, and no export leaves by a route that goes around the export control (decomposition AB-00-07).
- **REQ-010** — Missing target behaviour, missing mapping support and unsupported features are explicit findings or unsupported-target records, never defaults (SOW-074; PRD FR-HAND-004).
- **REQ-011** — A user can save the canonical model document whenever a model is open, with or without a solve and on either solve route. The file's SHA-256 is the session's model hash, and the file reopens to the same model (PRD FR-HAND-001, FR-MOD-001). The desktop's model document is the canonical model.
- **REQ-012** — An export is all or nothing. A failure, a refusal or a cancellation leaves the destination as it was, and no export changes the session's model, its hash or its solve standing (decomposition DEC-028; PRD §5.3).
- **REQ-013** — An export of the model is a local, private save made on the user's explicit intent, with no network use. It is refused while the model, a referenced library or a referenced rule pack carries a blocking disposition, and the refusal names the record (PRD §20.1, §20.3, §20.4; `docs/IP_AND_DATA_BOUNDARY.md`).
- **REQ-014** — Every handoff package is based on a saved model state with the package's model hash (PRD §16.2).

### Acceptance criteria

- **AC-001** — A package saved from an invented model that carries every kind of content validates against the handoff schema and passes DEL-15-01's verification, and so does a package saved from the bundled demonstration model.
- **AC-002** — A saved canonical model file is its own RFC 8785 text, its SHA-256 equals the model hash the session shows, and opening it gives a model with equal content and the same hash. The save is offered with no solved result, on both solve routes.
- **AC-003** — In a saved package the manifest is built from the session's model and store and from nothing else: every entity identifier, unresolved assumption, unit, and library and rule-pack reference of the model is present, and no value is a fixed placeholder standing in for one the model has. A model whose project units do not yield every unit of the manifest is refused by name.
- **AC-004** — A failure injected at each step of publication leaves the destination holding the previous package, byte for byte, or nothing if there was none. Saved is reported only when the published members read back equal to the verified bytes; where that cannot be confirmed the export is refused by name and the previous package is not deleted.
- **AC-005** — After a save, a refusal and a cancellation alike, the session's model, model hash and solve standing are what they were.
- **AC-006** — An export without the user's explicit intent is refused. A model that carries a blocking disposition in a provenance record, a rule-pack reference or a referenced library record is refused with a diagnostic that names the record, and nothing is written. A record marked private or of unknown standing is exported. No export opens a network connection.
- **AC-007** — A rule-pack reference that a solve of the same model would exclude stays listed in the package with a warning that names it, and a library reference the store does not hold is exported with a warning. A store that cannot be read refuses the export.
- **AC-008** — A package's basis state exists in the project store and its model hash is the package's. Where no saved state has that hash, a state is saved first, on the user's confirmation, and the package names it.
- **AC-009** — Given a handoff package, a target mapping and an invented target, the export record carries the package's model hash, units manifest, entity identifiers, references, unresolved assumptions and warnings equal to the package's, the manifest's target-mapping metadata and the mapping's records equal to their inputs, the provenance of the package, the mapping and each reference, and one unsupported-target record for every flag of the manifest, every unsupported and approximate flag of the mapping and every unsupported capability the target declares.
- **AC-010** — A missing or invalid package, a missing mapping, a missing target description, a mapping whose model hash or units reference differs from the package's, and a unit-bearing mapping without unit metadata each produce a blocking finding that names the item, and no value is supplied in its place. Each finding carries code, class, severity, source, affected object, message, remediation and provenance.
- **AC-011** — Every checksum the target step receives leaves it with the same algorithm, scope, digest and label.
- **AC-012** — The export record is returned only through the export control for the downstream-tool context, together with that control's decisions and findings: a value the context redacts or omits is named in the findings returned with the record. A package that reports an embedded private, protected or commercial payload, or a redaction that is not affirmed, is blocked.
- **AC-013** — No output or message of the path states or implies approval, certification, validation by an external tool or code compliance, and a target description that carries such wording is a blocking finding.
- **AC-014** — The path's sources and fixtures name no commercial tool's format and hold only invented data.
- **AC-015** — The same package, mapping and target give an identical export record.

## Production and Verification Method — Praxeology

**Method choices that define the export path.**

- The exported model is the hashed model. The saved bytes are the text the session's model hash is taken over, so no second serialization exists to drift from it.
- The side that writes decides. The native save re-establishes, on the exact bytes it is about to write, every condition the request claims: the canonical form, the hash, the document's validity and the disposition screen.
- The state is saved before the package is published. A publication that then fails leaves a state that is harmless and can be reused.
- A package is verified in memory before it is written and again after it is published, by DEL-15-01's rules.
- The target step assembles and checks; it interprets no target and executes no referenced contract.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference (the handoff schema under a JSON Schema 2020-12 validator, and DEL-15-01's verification run by `tests/verifiers/handoff_package.py`, which imports no product code): AC-001 and AC-003 on packages the product produced.
- **VER-002** — Comparison with a named reference (RFC 8785 and SHA-256 computed outside the product): AC-002 on a saved file, with the file reopened through the product's open path.
- **VER-003** — Executable check (observes the destination's bytes, recorded by the test before the save, after a failure injected at each publication step, and the session's model, hash and solve standing after each outcome): AC-004, AC-005.
- **VER-004** — Executable check (observes refusal or admission, the named record and the absence of any written file for invented models with each disposition, with and without intent, and with an unreadable store; observes the warnings for unresolved references against the resolution cases of `fixtures/handoff/rule_pack_ref_resolution_vectors.json`; and observes that no network connection is opened during a save, a refusal or a cancellation): AC-006, AC-007, AC-008.
- **VER-005** — Executable check (observes the export record built from the committed invented package, an invented mapping and `fixtures/handoff/invented_target_fixture.json`, the blocking findings for each invalid input the criteria name, the findings returned with a record from which the control redacts or omits a value, and the record's identity across repeated builds): AC-009 to AC-012, AC-015.
- **VER-006** — Executable check on the built application (observes Save Model Document and Save Handoff run through the native menu, the file reopened with an equal hash and the package verified): AC-001, AC-002.
- **VER-007** — Judgment by a reviewer who did not write the code (reads the path's messages against `docs/claims_registry.md` and its sources and fixtures for commercial formats, with the protected-content scan run over them): AC-013, AC-014.

## Governing Values and Decisions — Axiology

- **AX-001** — *What is handed on is what was modelled.* Better is fewer ways for an exported model or manifest to differ from the session's model; the target is none. Convenience never justifies a second serialization, a placeholder value or an export of a model the document validator refuses.
- **AX-002** — *Whole or absent.* Better is fewer states of the destination, after any failure, that are neither the previous package nor the new one. Speed and simplicity of publication never justify reporting Saved for bytes that were not confirmed, or deleting a previous package that could not be verified.
- **AX-003** — *Loss is stated.* Better is more of what a target cannot carry appearing as a record the engineer reads. An export is never made to look complete by dropping a warning, defaulting a value or approximating in silence.
- **AX-004** — *The user's data leaves only by the user's act.* A blocking disposition refuses every export, and no ease of sharing justifies an export without stated intent or by a route outside the export control.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-003 | SOW-074 OBJ-017 | REQ-001 REQ-002 | AC-001 | VER-001 VER-006 | Outside source: the JSON Schema 2020-12 specification and DEL-15-01's verification rules, applied by a verifier that imports no product code |
| OUT-002 | SOW-074 OBJ-017 | REQ-011 | AC-002 | VER-002 VER-006 | Outside source: RFC 8785 and SHA-256, computed by an implementation that shares no code with the product |
| OUT-003 | SOW-074 OBJ-017 | REQ-002 REQ-004 REQ-005 | AC-003 | VER-001 | DEL-15-01's derivation rule gives the expected manifest content; a verifier that imports no product code applies it to the model member. The unit refusal is the criterion's own expected result (PRD §5.6) |
| OUT-003 | SOW-074 OBJ-017 | REQ-012 | AC-004 | VER-003 | The scenario states what must remain; the comparison is against the destination's bytes as the test recorded them before the save |
| OUT-002 | SOW-074 OBJ-017 | REQ-012 | AC-005 | VER-003 | The scenario states the expected result: the session's state recorded before the export |
| OUT-003 | SOW-074 OBJ-017 | REQ-012 | AC-005 | VER-003 | The scenario states the expected result: the session's state recorded before the export |
| OUT-002 | SOW-074 OBJ-017 | REQ-009 REQ-013 | AC-006 | VER-004 | Refusal or admission, the named record and the written files observed on invented models with each disposition, with and without intent; the expected outcome is what `docs/IP_AND_DATA_BOUNDARY.md` and PRD §20.1, §20.3 and §20.4 say may leave and on whose act |
| OUT-003 | SOW-074 OBJ-017 | REQ-009 REQ-013 | AC-006 | VER-004 | Refusal or admission, the named record and the written files observed on invented models with each disposition, with and without intent; the expected outcome is what `docs/IP_AND_DATA_BOUNDARY.md` and PRD §20.1, §20.3 and §20.4 say may leave and on whose act |
| OUT-003 | SOW-074 OBJ-017 | REQ-005 | AC-007 | VER-004 | PRD §5.5 requires missing data to be shown. The expected exclusions are those DEL-14-02's resolution rule gives for the shared invented cases, which this path does not compute |
| OUT-003 | SOW-074 OBJ-017 | REQ-014 | AC-008 | VER-004 | The criterion states the expected result; the state is read from the project store through DEL-14-01's service |
| OUT-001 | SOW-074 OBJ-017 | REQ-002 REQ-003 REQ-005 REQ-010 | AC-009 | VER-005 | The scenario states the expected result: the invented package, mapping and target are the inputs, and the expected records are their flags and declared capabilities, which the product did not write |
| OUT-001 | SOW-074 OBJ-017 | REQ-004 REQ-010 | AC-010 | VER-005 | The findings observed for each invalid invented input; the criterion states the expected blocking finding (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2) |
| OUT-001 | SOW-074 OBJ-017 | REQ-008 | AC-011 | VER-005 | The supplied checksum is the reference: the input is compared with the output |
| OUT-001 | SOW-074 OBJ-017 | REQ-005 REQ-009 | AC-012 | VER-005 | The record and findings observed on invented inputs whose metadata the downstream-tool context includes, redacts and blocks; the expected decisions are those DEL-12-02's context rule gives, which this path does not compute (PRD §20.3), and every redacted or omitted value is named in the findings (PRD §5.5) |
| OUT-002 | SOW-074 OBJ-017 | REQ-006 | AC-013 | VER-005 VER-007 | The finding observed on an invented target description that carries prohibited wording, and the reviewer's reading of the path's outputs and messages against PRD §21.2 and `docs/claims_registry.md` |
| OUT-003 | SOW-074 OBJ-017 | REQ-006 | AC-013 | VER-005 VER-007 | The finding observed on an invented target description that carries prohibited wording, and the reviewer's reading of the path's outputs and messages against PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-074 OBJ-017 | REQ-006 | AC-013 | VER-005 VER-007 | The finding observed on an invented target description that carries prohibited wording, and the reviewer's reading of the path's outputs and messages against PRD §21.2 and `docs/claims_registry.md` |
| OUT-002 | SOW-074 OBJ-017 | REQ-003 REQ-007 | AC-014 | VER-007 | The protected-content scan and the reviewer's reading of the sources and fixtures for commercial formats, against decomposition OI-015 and `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-003 | SOW-074 OBJ-017 | REQ-003 REQ-007 | AC-014 | VER-007 | The protected-content scan and the reviewer's reading of the sources and fixtures for commercial formats, against decomposition OI-015 and `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-001 | SOW-074 OBJ-017 | REQ-003 REQ-007 | AC-014 | VER-007 | The protected-content scan and the reviewer's reading of the sources and fixtures for commercial formats, against decomposition OI-015 and `docs/IP_AND_DATA_BOUNDARY.md` |
| OUT-001 | SOW-074 OBJ-017 | REQ-001 | AC-015 | VER-005 | Two builds from the same package, mapping and target, compared with each other. This shows that the step is repeatable (PRD §3 asks for reproducibility); it does not show that the record is correct |
