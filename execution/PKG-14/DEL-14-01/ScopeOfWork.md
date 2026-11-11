---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-01
package_id: PKG-14
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-071]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-01 Immutable model state records

## Purpose and Objective Traceability

An engineer can fix a design at a point in time, give it a name, and come back to exactly that design later. A saved model state is what the rest of design iteration stands on: a run is bound to it, two states are compared, and a report or a handoff refers to it by identity and hash. What a state holds never changes after it is saved; a change to the model makes a new state. Names, tags and notes carry the review context that other tools force into a status lifecycle, and no state says or implies that a design was approved.

It serves SOW-071 under OBJ-016 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-14, row DEL-14-01), and `docs/PRD.md` §5.7 (state-based auditability), §8.6, §15.1, §16.3 (names, tags and notes instead of a forced prover status), FR-CMP-001 (Must: a user can save, reopen, tag and compare model snapshots without mutation) and the state level of FR-MOD-006 (Must: assumptions and open issues recorded at state level). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The model state record: its schema, and the saving, listing, reading and reopening of states in the project store, with their labels.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Model state.* A named, immutable snapshot of a project's model at a point in time, with its record.
- *Snapshot.* The RFC 8785 canonical text of the model document as it was when the state was saved. It holds everything PRD §15.1 asks a state to record about the model itself: units, the entity graph, materials and components with their library references, load definitions and constraints.
- *Model hash.* SHA-256 over the snapshot, as DEL-02-01 defines it. It identifies the content of a state.
- *State record.* The schema-governed record of a state (`schemas/model_state.schema.json`): its identity, name, kind, time of creation, the project it belongs to, its parent states, tags, notes, external references, unresolved assumptions, warnings, analysis statuses, hashes, immutability policy, professional boundary and provenance.
- *Labels.* A state's name, tags, notes and state-level assumptions. Labels are the one part of a state a user can edit after saving. They are kept apart from the immutable record and outside its hash, so renaming, tagging or annotating a state does not make a new state.
- *State-level assumption.* An assumption or open issue recorded against the state itself, beside those the model document already carries.
- *External reference.* A reference a user enters to something outside the project (a document, a drawing, a model file, a review record), with its type, label, target, privacy class and provenance.
- *Parent state.* The saved state a session was opened from. A state saved from that session names it.
- *Hash record.* A digest with its algorithm, the basis it was computed on, the payload it covers and that payload's scope.

**Undecided.**

- How a state's payloads that are not JSON, and the targets of its external references, are partitioned for hashing.
- Whether a saved state that nothing refers to may be deleted.
- How saved states and recorded runs travel in the portable package. The architecture makes the package's canonical JSON members the domain truth and the project store a projection of them; until this is decided, a project's states and recorded runs exist only in its local project store.

**Boundary.**

- **CLM-001** — The model document, its validator, its stable identifiers and the definition of the model hash are DEL-02-01's. The canonical JSON implementation and the form of a hash record are DEL-08-02's. The project store, its schema version and the model-document version check are DEL-02-05's. The analysis-status vocabulary is DEL-05-04's and the diagnostic record is DEL-00-06's.
- **CLM-002** — Run records are DEL-14-02's and the comparison of two states is DEL-14-03's. The History panel and the interaction of saving, opening and labelling a state in the desktop are DEL-07-08's. The state sections of a report are DEL-08-06's, and the classification of what an export may expose is DEL-12-02's.
- **CLM-003** — Outside the product: a formal prover-status lifecycle, an automatic record of professional acceptance, and comprehensive ingestion of a commercial prover's outputs.

## Completion and Reliance Basis — Epistemology

A state's hash establishes that the content read is the content saved. It says nothing about whether the design is right, and a record that validates against the schema establishes the record's shape, not that anything was stored.

### Requirements

- **REQ-001** — The product saves named immutable model states, and a user can reopen one as the working model of a new session (also cited as DEL-14-01-R1 and, for this deliverable's part, DEL-14-01:3). Source: SOW-071; PRD FR-CMP-001.
- **REQ-002** — A state record carries the state's name, tags, notes, the external references the user entered, unresolved assumptions, warnings, the time of creation and deterministic hashes, and the snapshot carries the rest of PRD §15.1 (also cited as DEL-14-01-R2). Source: SOW-071; PRD §15.1.
- **REQ-003** — A saved state is read-only. Nothing the product does changes a saved snapshot or the immutable part of its record, and a change to the model makes a new state. Historical states remain reproducible: a saved state that a recorded run or another state refers to is never removed (also cited as DEL-14-01-R3). Source: PRD §15.1.
- **REQ-004** — Every JSON payload hash of a state is SHA-256 over the payload's RFC 8785 canonical form (also cited as DEL-14-01-R4). Source: decomposition AB-00-04.
- **REQ-005** — Each hash record of a state names the payload it covers, that payload's scope and the basis its digest was actually computed on (also cited as DEL-14-01-R5). Source: SOW-071; PRD §21.1.
- **REQ-006** — Saving and reading a state inserts no engineering default. Units, provenance, rule-pack values, material and component data, SIF and flexibility inputs, allowables and load-basis values come back as saved (also cited as DEL-14-01-R6). Source: PRD §5.5.
- **REQ-007** — Unresolved assumptions and warnings are kept as visible review evidence: those of the model at the time of saving, and those recorded against the state itself (also cited as DEL-14-01-R7). Source: PRD §15.1, FR-MOD-006.
- **REQ-008** — No field, status, kind or label vocabulary of a state is a professional approval, certification, sealing, authentication or code-compliance status, and no prover-status lifecycle is enforced on states (also cited as DEL-14-01-R8). Source: PRD §16.3, §21.2.
- **REQ-009** — An external reference is an explicit record. Wherever one is entered or consumed it passes schema validation, the privacy controls, protected-content screening and the professional-boundary checks, and it goes around none of them (also cited as DEL-14-01-R9 and DEL-14-01:2). Source: PRD §15.1, §20.1, §20.3, §21.2.
- **REQ-010** — Public examples and fixtures of states hold only invented or cleared public data, with their provenance (also cited as DEL-14-01-R10). Source: PRD §20.1.
- **REQ-011** — States are kept in the product's project store and round-trip there: a state saved and read back returns the same snapshot, record and labels, and its hashes verify (also cited as DEL-14-01:1). Source: SOW-071; decomposition AB-00-04.
- **REQ-012** — A stored state whose snapshot, record or identity no longer agrees with its recorded hashes is refused by name on every read and is never presented as a valid state. Source: SOW-071 (deterministic hashes); PRD §15.1.
- **REQ-013** — A state's labels can be edited after it is saved without changing its identity, its snapshot or any hash of its record. Source: PRD FR-CMP-001, §16.3.
- **REQ-014** — A state saved from a session that was opened from a saved state names that state as its parent. Source: PRD §15.1 (new states are created from existing states).

### Acceptance criteria

- **AC-001** — A model saved as a state and read back returns a snapshot byte-identical to the canonical text that was saved, the same model hash, and a record that validates against the state schema with every member of REQ-002 present.
- **AC-002** — Every attempt to change a saved snapshot or the immutable part of its record, through any route the product offers, is refused by name and leaves the stored bytes as they were. Editing a reopened state and saving gives a second state that names the first as its parent, and the first is unchanged. Removing a state that a recorded run or a child state refers to is refused and says what refers to it.
- **AC-003** — Each hash in a state record is reproduced, from the payload the record names, by an implementation of RFC 8785 and SHA-256 that imports no product code. Each hash record states its algorithm, basis, payload and scope.
- **AC-004** — A state saved from a model that carries unresolved assumptions, validator warnings and values with units and provenance returns each of them as saved, with nothing added and nothing dropped. Assumptions recorded against the state appear with the model's, and each says which it is.
- **AC-005** — Renaming a state and changing its tags, notes and state-level assumptions leave its identity, snapshot, model hash and record hashes unchanged, and the next read returns the edited labels.
- **AC-006** — A stored state altered outside the product, in its snapshot, its record, a hash or an identity field, is refused by name when it is read, opened or compared.
- **AC-007** — A state with an external reference returns the reference as entered. A reference without its type, target, privacy class or provenance is refused with a named finding, and a reference classed private or suspected protected does not appear in a public export or report of the state. A reference's label and target are examined by the professional-boundary check of every report or export that carries them, as the rest of that output's content is.
- **AC-008** — The state schema is valid JSON Schema 2020-12. It admits no approval, certification, sealing, authentication or compliance value in any status, kind or field; its analysis statuses are those of DEL-05-04's vocabulary; and every public fixture and example of a state is invented and carries its provenance.
- **AC-009** — Listing a project's states returns every saved state with its identity, name and model hash. A stored state that does not verify is named as such in the listing, neither dropped nor listed as valid. Opening a listed state gives a working model whose model hash equals the state's.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- The snapshot is stored as the canonical text of the model document and returned as stored. Nothing re-serializes, repairs, migrates or normalizes a saved state.
- Labels are stored apart from the record. The record's hashes cover the snapshot and the immutable fields and nothing a user may edit.
- A state's hashes are verified on every read, before anything of it is shown or used.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the snapshot, model hash, record and labels read back from the project store against the document and labels given to save, for populated and empty-label states).
- **VER-002** — Executable check (observes the store after attempted updates and replacements through each service, after a save from a reopened state, and after a removal attempt on a state with a recorded run and with a child state).
- **VER-003** — Comparison with a named reference (RFC 8785 and SHA-256, computed by an implementation that imports no product code).
- **VER-004** — Executable check (observes label edits, and reads of states whose stored snapshot, record, hash and identity fields were each altered in turn).
- **VER-005** — Executable check (observes external references that are complete and that lack each required member, the public export and report of a state holding a private reference, and the professional-boundary check's findings on a report of a state holding a reference worded as a prohibited claim).
- **VER-006** — Comparison with a named reference (the JSON Schema 2020-12 specification, applied by a validator that imports no product code) with judgment by a reviewer who did not write the schema, who examines its vocabularies and the public fixtures with the protected-content scan.
- **VER-007** — Executable check (observes the listing of a project after several saves and with one stored state altered, and the model hash of the working model after each listed state is opened).

## Governing Values and Decisions — Axiology

- **AX-001** — *The saved design is the design read back.* Better is every saved state reading back byte-identical, on every machine and build that can read it. Convenience of opening a state never justifies repairing, migrating or normalizing it.
- **AX-002** — *The user's words over a status lifecycle.* Better is more of a reviewer's context carried in names, tags, notes and references the user chose. It may never be bought with a label vocabulary that reads as approval, validation or acceptance.
- **AX-003** — *Refuse over presenting doubtful content.* A state that does not verify is named and refused. Better is fewer refusals of states that were saved correctly, never by weakening what is verified.
- **AX-004** — *Assumptions stay visible.* Better is fewer assumptions and warnings lost between the model and the record of its state; the target is none.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-071 OBJ-016 | REQ-001 REQ-002 REQ-011 | AC-001 | VER-001 | The criterion fixes the expected result: what is read back is what was written. The snapshot and model hash are compared with the canonical text given to save, and the record with the JSON Schema 2020-12 specification under a validator that imports no product code |
| OUT-001 | SOW-071 OBJ-016 | REQ-003 REQ-014 | AC-002 | VER-002 | Owner ruling: PRD §15.1 makes a state immutable after creation and historical states reproducible. The stored bytes are compared with a copy taken before each attempt |
| OUT-001 | SOW-071 OBJ-016 | REQ-004 REQ-005 | AC-003 | VER-003 | Outside source: RFC 8785 and SHA-256 |
| OUT-001 | SOW-071 OBJ-016 | REQ-006 REQ-007 | AC-004 | VER-001 | The inputs are constructed with known assumptions, warnings, units and provenance, so the criterion states what must come back; the comparison is with the document and labels given to save |
| OUT-001 | SOW-071 OBJ-016 | REQ-013 | AC-005 | VER-004 | Owner ruling: PRD FR-CMP-001 lets a user tag a snapshot without mutation. The hashes after the edit are compared with those recorded before it |
| OUT-001 | SOW-071 OBJ-016 | REQ-012 | AC-006 | VER-004 | Each alteration is made deliberately to a known state, so the criterion states that it is refused; nothing is compared with the product's earlier output |
| OUT-001 | SOW-071 OBJ-016 | REQ-009 CLM-002 | AC-007 | VER-005 | Owner ruling: PRD §15.1, §20.1, §20.3 and §21.2. The references are constructed, so the criterion states which are refused, which are withheld and which the professional-boundary check examines |
| OUT-001 | SOW-071 OBJ-016 | REQ-008 REQ-010 CLM-003 | AC-008 | VER-006 | Outside source for the schema's validity: the JSON Schema 2020-12 specification. Owner ruling for its content: PRD §16.3, §20.1 and §21.2, judged by a reviewer against those clauses |
| OUT-001 | SOW-071 OBJ-016 | REQ-001 REQ-011 REQ-012 | AC-009 | VER-007 | The criterion fixes the expected result: the states saved are the states listed, compared with the saves made, and the model hash of an opened state is compared with the hash recorded when it was saved. The altered state is constructed, so the criterion states that it is named |
