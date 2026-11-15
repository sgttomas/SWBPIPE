---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-05
package_id: PKG-02
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-050, SOW-041]
package_objective_refs: [OBJ-001, OBJ-012]
---

# Scope of Work — DEL-02-05 Project persistence and round-trip serialization

## Purpose and Objective Traceability

An engineer's project is kept on their own machine and comes back exactly as it was saved. What is read back is the document that was written: no value, unit, reference, note or warning is lost and none is invented. A document in a format this build does not hold is refused by name and left untouched, and a crash costs at most one autosave interval of work.

This deliverable serves SOW-050 and SOW-041 under OBJ-001 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-02, row DEL-02-05). It carries `docs/PRD.md` FR-MOD-001 (project files serialize and deserialize without loss of model data, units, references, tags, notes or warnings), §5.2, §20.3 and §20.4, and exit criterion 10 of `docs/RELEASE_TARGET_v1.0.md`: autosave is on by default, and a crash loses at most one interval. The rules it implements are DEL-00-04's.

- **OUT-001** — Project persistence: the project store, save and open, the model-document version check, the persistence envelope and service contract, and the round trip.
- **OUT-002** — Autosave recovery records and backup copies.

## Deliverable Definition — Ontology

**Terms the persistence implements.**

- *Project store.* One local SQLite store in the application's local data folder, holding each project's model document with its revision, units, stored run, history and saved-package entries. The canonical JSON document is the domain truth and the store is a projection of it. The store has its own schema version, separate from any document's.
- *Model hash.* SHA-256 over the RFC 8785 canonical text of the model document.
- *Persistence envelope.* The schema-first form of a persisted project for the headless path, `schemas/project_persistence.schema.yaml`. Its slots are document metadata, project, units, model, loads, rule-pack references, provenance, diagnostics and version status, reproducibility, and optional review records.
- *Envelope hash.* SHA-256 over the canonical text of the envelope without the volatile storage summary and without the field that carries the hash, so that it can be recomputed from the restored envelope.
- *Model document file.* A model document opened from or saved to a file the user chooses. It is returned exactly as read.
- *Portable package.* The `.opsproj` multi-member archive. Its canonical JSON members carry the domain truth, each with its own hash in the manifest. The package and manifest contracts are DEL-17-02's.
- *Round trip.* Save followed by open, on any save path.
- *Autosave record.* One file in the application's autosave folder holding the exact canonical text of the session's current-version model document. Its name carries a slot derived from the project's identity, the time it was written and the model hash of its content. It is not a saved state, a run basis or a handoff basis, and it holds no queued change, proposal, review context, solve result or saved-package entry.
- *Autosave interval.* The time between autosave writes while a session holds unsaved edits: 5 minutes by default, with choices of 1, 2, 5, 10, 15 and 30 minutes, and off.
- *Recovery offer.* The explicit choice put to the user when a recovery record exists for the project being opened: open the autosave, keep it for later, or discard it. Keeping is the default.
- *Backup copy.* A copy of a project's stored document, written after a verified save to a local folder the user chose.

### CLM-003 — Model-document version contract

This is DEL-00-04's version rule applied to the model document.

- There is one current model-document version, named by DEL-02-01. It expresses a model with or without pressure: the pressure contract is optional, and its absence makes the document pressure-free.
- The check gives a `schema_version` exactly one state:

| State | When | Handling |
|---|---|---|
| `current` | The version is exactly the current one | Accepted; no version diagnostic |
| `retired_schema` | Any other parsed version not above the current one, including a leading-zero spelling of the current one | Refused by name with a diagnostic that says to re-author |
| `newer_than_supported` | A parsed version above the current one | Refused by name; never down-migrated |
| `unsupported_schema` | The version is missing, not a string, or unparseable | Refused by name; never coerced |

- The operation engine and the headless persistence service name the three refusals `MODEL_SCHEMA_VERSION_RETIRED`, `MODEL_SCHEMA_VERSION_NEWER_THAN_SUPPORTED` and `MODEL_SCHEMA_VERSION_UNSUPPORTED`.
- There is no stale, migrated or failed state. No model document is migrated, upgraded, down-migrated or coerced, and the product keeps no model-document migration code. The project store's own schema keeps its own forward-only rule (REQ-027).
- A document that is not current is refused for editing and for solving. Opening or saving never rewrites its stored bytes.
- A project or saved model state that holds a retired version is refused by name on open. Recorded runs keep their own format and their existing read and compare behaviour, with no model-document version gate added. Run history is listed by the active project's identity, so the runs of a retired project are reached through a current project of the same identity, and comparison keeps its eligibility rule.
- Every surface that shows or acts on a version state takes it from one evaluator.

**Undecided.**

- Which members the project document occupies in the portable package, and how payloads that are not JSON are partitioned between the hashed payload and the manifest.
- How enforcement of the private-data boundary is divided between save and open, export safeguards and PKG-12's controls.
- Automatic rotation of backup copies. It needs a stated model of what else may write to the folder and a promise the product can support; until then the product deletes no backup.
- Which volumes a backup folder may be on, beyond a local folder that supports cloning.

**Boundary.** The document format and its validator are DEL-02-01's, the unit catalog DEL-02-02's, the status vocabulary DEL-05-04's and the human acceptance record's binding DEL-02-03's. The canonical JSON implementation and the audit manifest are DEL-08-02's. Saved model states and recorded runs are DEL-14-01's and DEL-14-02's; the report package's contents are DEL-08-01's; storage paths and the local-first controls are DEL-12-01's; the operation engine is DEL-16-06's; the open and save surfaces in the desktop are DEL-07-11's. This deliverable defines no second shape for an object another schema owns. What an operation history record holds is DEL-16-03's; the store keeps it (REQ-037). What a saved-package entry holds is DEL-08-01's; the store keeps it (REQ-039). Recovery records are written through the atomic file writer in DEL-08-01's report-package code, whose outcome DEL-08-01 states only for a package save (DEL-08-01/REQ-018); REQ-033 holds for recovery records whichever writer writes them.

## Completion and Reliance Basis — Epistemology

A round trip shows that what was stored comes back; it does not show that what was stored was right. A hash establishes identity of content, not its correctness, and a fixture that validates establishes the shape of the contract, not durable storage.

### CLM-014 — Requirements

Requirement numbers 001 to 026 are those of the earlier `REQ-02-05-nnn` series.

- **REQ-001** — A project can be created, opened, saved and version-checked.
- **REQ-002** — A project round trip loses nothing: model data, units, loads, references, rule-pack references, tags, notes, warnings and provenance return as saved (`docs/PRD.md` FR-MOD-001).
- **REQ-003** — The envelope carries the model document as DEL-02-01 defines it and references the product's other schemas for the objects they own.
- **REQ-004** — The persistence envelope is JSON Schema 2020-12.
- **REQ-005** — A persisted JSON payload is deterministic and hashed as SHA-256 over its RFC 8785 canonical text. A serializer that is not RFC 8785 is labelled as what it is, and its output is never called canonical or used where the canonical hash is required.
- **REQ-006** — Every persisted document carries its schema version, and the version check gives it exactly one state of CLM-003.
- **REQ-007** — Every persisted engineering value carries its unit or refers to the project's declared units. Nothing is given a unit by default.
- **REQ-008** — Provenance is preserved for materials, components, SIF and flexibility values, allowables, rule-pack values and other data an engineer relies on. Missing provenance is a finding, never accepted as complete.
- **REQ-009** — A rule pack is persisted as a reference with its version, checksum and source note. No public schema, fixture or test of this deliverable embeds a protected formula, allowable or code-specific value.
- **REQ-010** — The store is reached only through application services. No adapter or plugin has SQL access, and none goes around unit, provenance, diagnostic, sandbox, envelope, report or data-boundary checks.
- **REQ-011** — A failure of open, save or validation, and every version refusal, is a structured diagnostic in a result envelope: code, class, severity, source, affected object, message, remediation, and provenance where it applies.
- **REQ-012** — A persisted project carries the reproducibility metadata reports and audit manifests need: the model hash and compatibility with the input manifest.
- **REQ-013** — Private project, material, component and rule-pack data stay on the user's machine. Persistence transmits nothing.
- **REQ-014** — Every public fixture and example is invented, original, public-domain or permissively licensed, and a data record in one states its provenance and redistribution status.
- **REQ-015** — Reserializing is deterministic: the same document gives the same canonical text and the same hash on every save, every engine and every machine.
- **REQ-016** — No persisted status, field, diagnostic or example claims professional approval, certification, sealing, authentication or automatic code compliance (`docs/PRD.md` §21.2).
- **REQ-017** — The envelope has a slot for each of: document and schema metadata, project identity, units, the model, loads, rule-pack references, provenance and redistribution metadata, diagnostics and version status, reproducibility metadata, and optional review records.
- **REQ-018** — After parse, validate, serialize and parse again, the content of each category is semantically equal: stable identifiers, object types, references, values and their units, load cases and their records, rule-pack references, provenance and reproducibility metadata. Only fields documented as volatile may differ, and no engineering default is inserted.
- **REQ-019** — Round-tripped unit metadata distinguishes an explicit unit, a reference to the declared units, a missing unit and an incompatible one. The last two are findings.
- **REQ-020** — A hash record names the payload it covers. A member that is not JSON is hashed by its bytes in a manifest, and a partition no hash covers is stated as uncovered.
- **REQ-021** — An older, a newer and an unsupported document are each refused by name, the older with the re-author diagnostic. The compatibility window is the current version only, for model documents and for the portable package.
- **REQ-022** — Create, open, save, validate and version check are commands and queries that return result envelopes.
- **REQ-023** — Persistence diagnostics cover schema, version, unit metadata, provenance, rule-pack reference, data-boundary, private-data and professional-boundary failures, each with a deterministic class.
- **REQ-024** — A persisted rule-pack reference identifies whether the pack is private or public, its version, checksum, source note and redistribution status. A missing or private reference is a diagnostic, and no protected rule content appears in a public artifact.
- **REQ-025** — Create, open and save keep the local-first, private-by-default boundary.
- **REQ-026** — A review record, where one is stored, is bound to the hashes of the model, rule pack and report it concerns. It does not survive a change to that content, and it implies no software certification or automatic code compliance.
- **REQ-027** — The store's own schema changes forward only, each step in one transaction that is rolled back whole on failure. A build that meets a store newer than it knows refuses the store before writing anything.
- **REQ-028** — No open or save destroys stored work. A refused document stays as stored. A stored run too large to display is withheld at open and kept, and a save that would replace it with nothing is refused.
- **REQ-029** — A stored document refused at open can be exported to a file the user chooses, exactly as the store holds it. The export opens the store read-only and adopts nothing into a session.
- **REQ-030** — One evaluator classifies a model-document version, compiled for both the native and the web engine, so that no surface carries its own copy of the rule (also cited as DEL-02-05:3).
- **REQ-031** — A project round-trips through the portable package: its model, units, loads, rule-pack references and provenance return as saved. The reader accepts only the current package version, refuses an older, a newer and an unrecognised version by name, and verifies every member against the hash the manifest gives for it (also cited as DEL-02-05:1 and DEL-02-05:2).
- **REQ-032** — Autosave is on by default. While a session holds unsaved edits, a recovery record of the session's document is written every interval, so that a crash loses at most one interval of work. Turning autosave off shows a standing notice.
- **REQ-033** — No recovery record on disk is partial. A record that reads back as a different hash is removed; one whose read-back cannot be checked is kept and reported as unverified. Writing one never opens or writes the project store and never changes the saved project. An older record is removed only after a newer one is verified and durable; after a save lands, only records whose hash equals the saved model's are discarded.
- **REQ-034** — A recovery is offered and never applied automatically. The offer compares times by this machine's clock and never hides a record.
- **REQ-035** — A backup copy is made only when the user has turned backups on and chosen a folder. It is taken from the stored document after a verified save, verified by read-back, and deletes nothing. A refused, an unverified and an unknown outcome are each reported as what they are, and a backup never delays or undoes a save.
- **REQ-036** — A save never replaces a stored revision of a project that the session did not open or write and the user did not confirm. A save in place after another window or process saved the project, a replacement of a revision that changed after the user was shown it, and the creation of a project under an identity already stored without the user's confirmation of the stored revision are each refused by name and write nothing. A creation or a save that replaces exactly the revision the user confirmed is permitted. Of two writers at once, the second sees the first's revision and is refused.
- **REQ-037** — The store keeps a project's operation history records with the project: a save and reopen return every record unchanged and in order.
- **REQ-038** — On Windows, autosave and backup keep REQ-033 and REQ-035, or the build refuses them by name. Their checks are bound to the open file and folder rather than to path names, so that a link, a junction, a swapped folder or a file replaced during the operation is not followed or written through; a direct copy or a rename of a named source does not stand in for a backup's all-or-none publication; and a sync whose outcome cannot be confirmed is reported as unknown. Windows is not a release platform for v1.0; this is what a Windows build must meet before autosave and backup are enabled on it.
- **REQ-039** — The store keeps a project's saved-package entries as DEL-08-01/REQ-023 defines them: it returns them unchanged and in order when the project is opened, and removes them with the project. Adding or removing an entry changes no other record of the project. The table that holds them is added by a forward-only store migration (REQ-027).

### Acceptance criteria

- **AC-001** — A project saved and reopened on each save path returns a document with the same model hash, and every category of REQ-018 compares equal.
- **AC-002** — For every version state of CLM-003, each implementation of the check gives the same state and refuses every state but `current` by name, and no stored byte of the refused document changes. Every desktop surface that shows or acts on a version state takes it from the one evaluator of REQ-030, and no other code in the native or the web engine classifies a model-document version.
- **AC-003** — A project and a saved model state holding a retired version are each refused by name on open, and the recorded runs of that project are still listed, read and compared through a current project of the same identity.
- **AC-004** — The model hash and the envelope hash are reproduced by an implementation of RFC 8785 and SHA-256 that imports no product code. Each is unchanged by key order, changes with any change of content, and the envelope hash is recomputed from a restored envelope.
- **AC-005** — The persistence envelope schema is valid JSON Schema 2020-12 with every slot of REQ-017. The public invented project validates against it, and an envelope lacking a required slot, a unit or a version is refused with a named diagnostic.
- **AC-006** — Each failure of REQ-023 produces a diagnostic with the fields of REQ-011. A missing or incompatible unit and missing provenance are findings, and no default is inserted.
- **AC-007** — A failed store migration leaves the store as it was; a newer store is refused before any write; a withheld run survives open and a refused save; and the export of a refused document is byte-identical to the stored text with the store unchanged.
- **AC-008** — Persistence opens no network connection. Its public fixtures are invented and carry no protected or private content, and no status, field or diagnostic claims approval or compliance.
- **AC-009** — The package reader accepts a package of the current version and refuses an older, a newer and an unrecognised version, a malformed container or manifest, and a member whose hash differs from the manifest, each by name.
- **AC-010** — A project written to a portable package and read back returns the same model hash, with units, loads, rule-pack references and provenance equal.
- **AC-011** — On a new installation autosave is on at the default interval, and turning it off shows the standing notice for as long as it is off. With autosave on and unsaved edits, a session ended without warning leaves a recovery record no older than one interval before the end. On reopening, the offer is made, nothing is applied until the user chooses, and choosing the autosave gives a document with the model hash the record's name carries. The store is unchanged by the whole sequence.
- **AC-012** — Under a full disk, an interrupted rename and two writers at once, no record on disk is partial, every record either verifies against its name or was reported unverified when it was written, and no older record was removed without a verified, durable newer one.
- **AC-013** — A backup copy equals the stored document byte for byte; no file in the backup folder is deleted; a refused, an unverified and an unknown outcome are each reported distinctly; and the save completes whatever the backup's outcome.
- **AC-014** — A save in place after another session saved the same project, a replacement of a revision that changed after the user confirmed it, the creation of a project under a stored identity without confirmation, and the second of two concurrent writers are each refused by name with the stored revision unchanged; a save over the revision the session opened or last wrote, and a creation or save that replaces the revision the user confirmed, succeed.
- **AC-015** — A project saved with accepted and rejected operation batches in its history and reopened returns every history record byte-identical and in its order.
- **AC-016** — On a Windows build, autosave and backup are either refused by name, or, under a swapped parent or destination folder, a link or junction in the path, a file replaced while locked, a sharing violation, a full disk, an interrupted publication and a read-back mismatch, leave no partial recovery record, never publish a partial or unverified copy as a successful backup, retain and report a failed backup's temporary, partial or unverified outputs and delete none of them, write and delete nothing outside their folders, and report an outcome that cannot be confirmed as unknown.
- **AC-017** — A project saved with saved-package entries and reopened returns every entry byte-identical and in its order. Each addition of an entry and each removal leaves the project's model document, stored run, history, saved states and recorded runs byte-identical. Removing the project removes its entries. A store of the version before the entries' table opens with that table added and every earlier record unchanged, and a failed migration leaves the store as it was.

## Production and Verification Method — Praxeology

**Method choices that define the persistence.**

- The document is stored and returned as written. No reader repairs, defaults or normalizes it.
- One evaluator and one validator: every entry point calls the same check, so a rule has one place to be wrong.
- A write is made beside its target and moved into place only when complete and verified.
- The headless path may use a simpler serializer, on the condition that it is labelled truthfully and never stands in for the canonical hash.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-003, executable check: saves and reopens projects in the store and through the envelope, compares model hashes and each category, and opens a project and a saved state holding a retired version.
- **VER-002** — AC-002, executable check: runs one table of version cases through every implementation of the check, compares the stored bytes before and after, and searches the native and web engines' source for any other classification of a model-document version.
- **VER-003** — AC-004, comparison with a named reference: RFC 8785 and SHA-256, computed by an implementation that imports no product code.
- **VER-004** — AC-005 and AC-006, executable check: observes the schema, the invented project and constructed invalid envelopes under a JSON Schema 2020-12 validator that imports no product code, and the diagnostics of the persistence service.
- **VER-005** — AC-007, executable check: forces a migration to fail, presents a newer store, an oversize stored run and a refused stored document, and compares the store and the exported bytes.
- **VER-006** — AC-008, judgment by a reviewer who did not write the persistence, with the protected-content scan and an executable check that the persistence code has no network client.
- **VER-007** — AC-009 and AC-010, executable check: reads constructed packages of each kind and round-trips a project through a package.
- **VER-008** — AC-011 and AC-012, executable check: observes the autosave setting and the notice on a fresh settings folder, ends a session without warning, and injects a full disk, an interrupted rename and a concurrent writer, then inspects the autosave folder, what each write reported, the offer and the store.
- **VER-009** — AC-013, executable check: makes a backup in a folder that already holds files, with each outcome forced in turn, and compares the copy, the folder and the save.
- **VER-010** — AC-014, executable check: saves one stored project from two sessions in turn and at once, replaces a revision that changed after confirmation, creates a project under a stored identity without confirmation and with confirmation of the stored revision, and compares the stored revision before and after each.
- **VER-011** — AC-015, executable check: saves a project whose history holds accepted and rejected batches, reopens it, and compares each history record read back with the one saved.
- **VER-012** — AC-016, executable check on a Windows build: injects each fault the criterion names into autosave and backup and inspects the folders, everything outside them and what each operation reported.
- **VER-013** — AC-017, executable check: writes entries to a stored project, reopens it and compares each entry read back with the one written; compares the project's other records before and after each entry is added and each is removed; removes the project and reads the entries; and compares every earlier record before and after a successful migration, and the store before and after a forced migration failure.

## Governing Values and Decisions — Axiology

- **AX-001** — *The user's work is never lost or silently changed.* Better is less work lost to a crash, with one interval as the limit, and no stored byte changed by anything but a save the user asked for. A refusal that leaves the work untouched is always preferred to an open that alters it.
- **AX-002** — *Refuse by name over coercion.* A document the build does not hold is named and refused. Convenience of opening old work never justifies migrating, defaulting or guessing.
- **AX-003** — *Determinism over convenience.* The same document gives the same bytes and the same hash everywhere. A faster or simpler serializer is acceptable only where it cannot be mistaken for the canonical one.
- **AX-004** — *Local and private by default.* Nothing leaves the machine through persistence, and a copy is made only where the user chose.
- **AX-005** — *Truthful outcomes.* An outcome that is not known is reported as unknown. Better is fewer outcomes reported without verification; the target is none.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-050 OBJ-001 OBJ-012 | CLM-014 REQ-001 REQ-002 REQ-003 REQ-015 REQ-018 | AC-001 | VER-001 | The criterion fixes the expected result: what is read back is what was written. The model hash of the reopened document is recomputed and compared with the hash of the document given to save |
| OUT-001 | SOW-050 OBJ-012 | CLM-003 REQ-006 REQ-021 REQ-030 | AC-002 | VER-002 | CLM-003 states one current version and nothing migrated. The table of cases and expected states is written from CLM-003, not from an implementation's output |
| OUT-001 | SOW-050 OBJ-012 | CLM-003 REQ-021 | AC-003 | VER-001 | CLM-003 states what happens to retired projects, saved states and recorded runs; the criterion states the expected refusals and the runs still reached |
| OUT-001 | SOW-041 OBJ-012 | REQ-005 REQ-012 REQ-015 REQ-020 | AC-004 | VER-003 | Outside source: RFC 8785 and SHA-256 |
| OUT-001 | SOW-041 OBJ-012 | REQ-004 REQ-007 REQ-009 REQ-017 REQ-024 | AC-005 | VER-004 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code. The envelopes are constructed, so the criterion states which are refused |
| OUT-001 | SOW-050 OBJ-001 | REQ-007 REQ-008 REQ-011 REQ-019 REQ-022 REQ-023 | AC-006 | VER-004 | The inputs are constructed with a known defect each, so the criterion states the expected finding; the diagnostic fields are DEL-00-06's |
| OUT-001 | SOW-050 OBJ-001 | REQ-027 REQ-028 REQ-029 | AC-007 | VER-005 | The criterion fixes the expected result: the store and the exported bytes are compared with a copy taken before the operation |
| OUT-001 | SOW-050 OBJ-001 | REQ-010 REQ-013 REQ-014 REQ-016 REQ-025 REQ-026 | AC-008 | VER-006 | `docs/PRD.md` §20.1, §20.3, §20.4 and §21.2 and `docs/IP_AND_DATA_BOUNDARY.md`; the reviewer compares against sources the product did not write |
| OUT-001 | SOW-050 OBJ-012 | REQ-021 REQ-031 | AC-009 | VER-007 | The package holds one version (REQ-031). The packages are constructed, so the criterion states which are refused; member hashes are recomputed with SHA-256 |
| OUT-001 | SOW-050 OBJ-001 | REQ-002 REQ-031 | AC-010 | VER-007 | The criterion fixes the expected result: the model hash read from the package equals the hash of the project written |
| OUT-002 | SOW-050 OBJ-001 | REQ-032 REQ-034 | AC-011 | VER-008 | Exit criterion 10 of `docs/RELEASE_TARGET_v1.0.md` sets autosave on by default and the one-interval limit. The recovered document's hash is recomputed and compared with the hash of the session's document before the end |
| OUT-002 | SOW-050 OBJ-001 | REQ-033 | AC-012 | VER-008 | The faults are injected, so the criterion states the expected result; each record is verified by recomputing its hash against its name, and an unverified record is matched to the report made when it was written |
| OUT-002 | SOW-050 OBJ-001 | REQ-035 | AC-013 | VER-009 | The criterion fixes the expected result: the copy is compared byte for byte with the stored document, and the folder with a listing taken before |
| OUT-001 | SOW-050 OBJ-001 | REQ-028 REQ-036 | AC-014 | VER-010 | The criterion fixes the expected result: the stored revision is compared with a copy taken before each write, and each refusal is provoked by the check |
| OUT-001 | SOW-050 OBJ-012 | REQ-037 | AC-015 | VER-011 | The criterion fixes the expected result: the records read back are compared with those saved. What a record holds is DEL-16-03's |
| OUT-002 | SOW-050 OBJ-001 | REQ-038 | AC-016 | VER-012 | The faults are injected, so the criterion states the expected result; the folders and their surroundings are compared with listings taken before. Evidence comes from a Windows build only; a macOS run says nothing about it |
| OUT-001 | SOW-050 OBJ-012 | REQ-027 REQ-039 | AC-017 | VER-013 | The criterion fixes the expected result: the entries read back are compared with those written, and the other records with copies taken before. What an entry holds is DEL-08-01's |
