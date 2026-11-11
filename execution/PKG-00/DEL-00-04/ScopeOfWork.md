---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-04
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-059]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-04 Persistence and schema versioning architecture

## Purpose and Objective Traceability

This deliverable is the rule set for everything SWBPIPE writes to disk. It says what the truth is (canonical JSON documents governed by schemas), how each document states which version it is, how a document is hashed so that two machines agree, and what a save and reopen must preserve. The result it contributes is that a project saved today opens to the same model, units, loads, references and provenance; that anyone can recompute its hash; and that an old or foreign document is refused by name and never half-read.

It serves SOW-059 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-04, and architecture-basis row AB-00-04 in §8.1), with `docs/PRD.md` FR-MOD-001, §5.2, §5.7 and §20.4 and `docs/SPEC.md` §4.4. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 4 (a report with a reproducible canonical hash) and criterion 10 (autosave) through DEL-08-02 and DEL-02-05.

- **OUT-001** — The persistence rules: the domain truth, schema governance, the version declaration, the canonical hash basis, the round-trip requirement and the local storage profile.

## Deliverable Definition — Ontology

**Terms the architecture implements.**

- *Domain truth.* The schema-governed canonical JSON document. The local store and every index are projections of it.
- *Schema governance.* Every kind of persisted document has a schema, in the dialect the baseline names (DEL-00-01).
- *Version declaration.* Each persisted document carries its schema version. Each kind of document, and the project package, has one current version. A reader accepts that version and refuses every other by name: an older one with a diagnostic that says to re-author, a newer, missing or unparseable one as such. Nothing is migrated, upgraded, down-migrated or coerced. The names of the version statuses are DEL-02-05's.
- *Store version.* The local store's own table layout is versioned apart from the documents, by SQLite's `user_version`.
- *Version numbering.* Any change to what a document can contain, including an added optional field, raises at least the minor version. A patch number is for a change that does not alter the document's shape.
- *Canonical hash.* SHA-256 over the RFC 8785 (JCS) canonical form of a JSON payload. A hash record names the payload it covers. A hash carried inside a document leaves out its own field and the volatile storage fields, so it can be recomputed from the restored document. A member that is not JSON is hashed by its bytes in a manifest.
- *Round trip.* Save followed by open. It returns semantically equal model content, unit metadata, loads, rule-pack and library references, diagnostics and provenance; only fields documented as volatile may differ; and it inserts no engineering default.
- *Local storage profile.* A local SQLite project store and index. Retrieval sidecars are rebuildable and never authoritative. Large external files are referenced in place by path or URI with a hash and metadata. There is no hosted database, daemon, required network, cloud sync or telemetry path. The store is changed only through application services, and no plugin or adapter has SQL access.
- *Project package.* A multi-member archive whose canonical JSON members carry the domain truth. Evidence binds to the members and their per-member hashes and not to the bytes of the archive.

**Boundary.** This deliverable states the rules; it owns no schema, reader, serializer or table. Its criteria are observed on the code of the deliverables below, and a shortfall is repaired there.

- DEL-02-05 owns the persistence envelope, the project store, save and open, the version check and its status names, and the store's own schema changes.
- DEL-08-02 owns the canonical JSON implementation, the model hash and the audit manifest.
- DEL-02-01 owns the model document schema; DEL-17-02 owns the package and manifest contracts.
- DEL-12-01 owns storage paths and the local-first controls.
- DEL-00-03 owns the service boundary through which the store is changed.

**Not yet defined.** How a stored rule-pack or library reference declares its version status, which is part of REQ-002. How payloads that are not JSON are partitioned in hash and round-trip manifests, which is part of REQ-003.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Persistence is deterministic, versioned, unit-aware, provenance-preserving, schema-governed and round-trip testable (decomposition AB-00-04).
- **REQ-002** — Every persisted document declares its schema version and is given a version status on reading. A reader accepts only the current version and refuses every other by name. Opening a document never rewrites it.
- **REQ-003** — Canonicalization comes before every hash and audit manifest: a JSON payload is hashed as SHA-256 over its RFC 8785 canonical form.
- **REQ-004** — Round trips are checked for models, units, loads, rule-pack references, diagnostics and provenance metadata (`docs/PRD.md` FR-MOD-001).
- **REQ-005** — Public schemas are separate from private, user-owned code data and proprietary libraries (`docs/PRD.md` §5.4).
- **REQ-006** — Storage follows the local storage profile (decomposition AB-00-04).
- **REQ-007** — Any change to what a persisted document can contain, including an added optional member, raises at least the minor version of its schema; a patch increment leaves the document's shape unchanged.

### Acceptance criteria

- **AC-001** — Saving the same project state twice, on any supported machine, produces byte-identical canonical documents and equal hashes.
- **AC-002** — Every kind of persisted document has a schema that requires its version field. Opening a document whose version is not current is refused with a diagnostic that names the version and its status, and the document's bytes are unchanged afterward.
- **AC-003** — For every hashed JSON payload, the recorded hash equals SHA-256 of the payload's RFC 8785 canonical form as computed by an implementation that shares no code with the product. Each hash record names its payload, and a hash carried inside a document is recomputable from the restored document.
- **AC-004** — A save and reopen of a project populated with models, units, loads, rule-pack and library references, diagnostics and provenance returns semantically equal content. The only differing fields are those documented as volatile; reopening inserts no value that was not in the saved document, and no engineering default. This carries commitment DEL-00-04:1.
- **AC-005** — Public schemas define structure only. No public schema, fixture or example embeds private rule data, proprietary library content or code-specific values.
- **AC-006** — Deleting every retrieval sidecar (the rebuildable full-text index) and reopening the project loses no model, result or record. No plugin or adapter opens the store or issues SQL, and every change to the store is made through an application service. Authoring, solving and reporting work with the network disabled.
- **AC-007** — For each change to a persisted-document schema, the declared current version rises by at least a minor step whenever the set of valid documents changes.

## Production and Verification Method — Praxeology

**Method choices that define the architecture.**

- The document is canonical JSON and the store is rebuilt from it, never the reverse.
- One current version and no migration: an older document is re-authored, and no compatibility code is kept for a retired version.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-004, executable check: saves a populated project, reopens it and compares the content semantically, then saves again and compares the bytes and hashes.
- **VER-002** — AC-002, executable check: opens documents with an older, a newer, a missing and a malformed version and observes the named refusal and the unchanged source bytes.
- **VER-003** — AC-003, comparison with a named reference: the test vectors of RFC 8785 and a JCS and SHA-256 computation that imports no product code.
- **VER-004** — AC-005 and AC-006, executable check with judgment by a reviewer who did not write the schemas: deletes the retrieval sidecars and reopens, searches plugin and adapter modules for store access, runs the core workflows offline, and the reviewer examines the public schemas, fixtures and examples with the protected-content scan.
- **VER-005** — AC-007, executable check with judgment by a reviewer who did not make the change: compares each persisted-document schema's changes with its declared current version, and the reviewer judges whether a change the check cannot classify alters the set of valid documents.

## Governing Values and Decisions — Axiology

- **AX-001** — *The document is the truth.* Better is less state that exists only in the store or a sidecar; the target is none. An index may make a query faster and is never made authoritative to do so.
- **AX-002** — *Refuse, do not guess.* Better is more malformed, foreign and out-of-version inputs refused by name. A reader never half-opens or silently rewrites a document to be helpful.
- **AX-003** — *Hashes anyone can recompute.* Better is fewer fields left out of a hash, each documented. The hash basis never depends on a platform or an implementation.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-059 OBJ-013 | REQ-001 | AC-001 | VER-001 | The criterion states the expected result: two saves of one state are byte-identical. It needs no reference |
| OUT-001 | SOW-059 OBJ-013 | REQ-002 | AC-002 | VER-002 | Owner ruling: one current version, every other refused by name and nothing migrated |
| OUT-001 | SOW-059 OBJ-013 | REQ-003 | AC-003 | VER-003 | Outside source: RFC 8785 and its test vectors, and SHA-256 (FIPS 180-4), computed without product code |
| OUT-001 | SOW-059 OBJ-013 | REQ-004 | AC-004 | VER-001 | Owner ruling: `docs/PRD.md` FR-MOD-001 states what must survive; the comparison is the authored input against the reopened document |
| OUT-001 | SOW-059 OBJ-013 | REQ-005 | AC-005 | VER-004 | Owner ruling: `docs/PRD.md` §5.4 and `docs/IP_AND_DATA_BOUNDARY.md`; protected-content scan and reviewer |
| OUT-001 | SOW-059 OBJ-013 | REQ-006 | AC-006 | VER-004 | Owner ruling: decomposition AB-00-04 states the profile; the deletion and offline scenarios state what must happen |
| OUT-001 | SOW-059 OBJ-013 | REQ-007 | AC-007 | VER-005 | Owner ruling: additive changes raise the minor version (decomposition DEC-033, DEC-038); the comparison is each schema's change against its declared version |
