---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-01
package_id: PKG-12
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-029]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-01 Local-first storage and private data paths

## Purpose and Objective Traceability

This deliverable keeps a user's private engineering data on the user's own machine, in places the user controls. It says which kinds of place exist and what each may hold, it decides whether a described act of storage is admissible, and it fixes where the desktop puts the data it writes for itself. Its result is that the product works with no cloud service, writes nothing private into a public or remote place by default, and names every unresolved storage state instead of choosing a default for it.

It serves SOW-029 under OBJ-010 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-12, row DEL-12-01), PRD §20.3 (private data in user-controlled paths), PRD §20.4 (local-first operation) and PRD §7 non-goal 8 (no dependence on cloud services), within the local storage profile that DEL-00-04 states. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches criterion 10 (autosave) through DEL-02-05, whose recovery records and backup copies are written to places this deliverable classes.

- **OUT-001** — The storage policy, `docs/security/local_first_storage_policy.md`: the path classes, what each may hold, what a public repository path never holds, and the rule that storage is local and transmits nothing.
- **OUT-002** — The storage guard: the decision, made from metadata alone, on whether a described storage record is admissible in a stated context.
- **OUT-003** — The desktop's placement of its own data: the locations of the project store, its sidecars, recovery records, settings and caches.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Private data.* Private by default, whoever supplied it: project models and project-local assets; private rule packs, owner standards and company design bases; material libraries; component, section and vendor-derived library values; reports, exports and screenshots; diagnostics and support bundles; caches, recovery records and backups derived from a project; and any hash that can fingerprint private work. It stops being private only when the user intentionally exports, shares or contributes it with documented redistribution rights.
- *Path class.* A symbolic name for a kind of place. The policy and the guard speak in path classes and never in concrete paths. The classes are:

  | Path class | Holds | Boundary |
  |---|---|---|
  | `PUBLIC_REPO_ROOT` | Public source, schemas, documentation, invented examples and validation artifacts safe to redistribute | Never holds private data, credentials or protected standards content |
  | `USER_CHOSEN_PROJECT_PATH` | The user's project: the local project store and its recovery records in the local data directory, and the model documents, project packages and backup copies in folders the user chooses | Private by default; local |
  | `USER_PRIVATE_LIBRARY_ROOT` | The user's private rule packs, material and component libraries, owner and company design-basis data | Outside every public repository path; never committed by default |
  | `PROJECT_PRIVATE_ASSET_ROOT` | Project-scoped private attachments, local manifests and supporting files referenced in place | Distinct from the fields a project document persists |
  | `REPORT_EXPORT_TARGET` | A local report or export destination the user selects | May hold private values; what may be written there is DEL-12-02's decision |
  | `APP_CACHE_OR_SESSION_STATE` | Transient session and cache state | Never becomes an ungoverned durable store of private data |

  The guard's vocabulary may be finer: it may separate rule-pack roots from library roots and public examples from other public repository content, and it may hold classes of its own for diagnostic bundles, import staging and export staging. Each finer class falls under exactly one class above, and the policy says which. Which class above holds a diagnostic bundle, import staging or export staging is not decided. A secret reference is DEL-12-04's and is not a storage path class here. A class the guard does not know is blocked or reported in a finding, and is never treated as a known class.
- *Storage context.* Where a record is headed: a public or shared destination (the public repository, a public example or fixture, a public report, a shared model, a downstream tool); a local private destination (the project, a private library or rule-pack root, a private report, a private diagnostic, a local cache); a remote destination (cloud storage, cloud or network sync, a hosted database, a network share, a remote service); or direct SQL access to the store.
- *Storage record.* Metadata describing one thing to be stored: its kind, path class, privacy class, redistribution status, review status, source state, and whether the user stated intent to keep it locally and privately. A record that refers to a private rule pack also states the pack's identity, version, checksum and source notice. The guard reads the record and nothing else: no file, no store, no payload.
- *Decision.* For one record in one context, either *admitted as metadata only* or *blocked*, with a reason code and a diagnostic. A set of decisions comes with a safe manifest: the admitted metadata with every concrete path, payload and secret withheld.
- *Local data directory.* The per-user application data directory the operating system assigns to the desktop by its bundle identifier.

**Boundary.**

- DEL-02-05 owns the project store, save and open, recovery records and backup copies: the code that reads and writes at the places this deliverable classes. DEL-00-04 owns the persistence rules those writes obey. This deliverable's criteria on placement are observed on that code, and a shortfall is repaired there.
- DEL-12-02 owns what may leave through an export or a report, including the admission of each export route. Its route admission applies this guard to the route's own metadata.
- DEL-12-04 owns credential references and the handling of private-library references, and states the open question of whether stored private data is encrypted at rest. A secret is never a storage record's content here.
- DEL-12-03 owns telemetry. Storage is never a telemetry or synchronization path.
- DEL-16-06 owns the model-provider channel of the embedded agent. When the user configures a provider, the user's own private data may be sent to it (`docs/CONTRACT.md` OPS-K-PRIV-1). That channel is not storage.
- The guard opens no store, writes no file, chooses no operating-system path and is not a storage service.

## Completion and Reliance Basis — Epistemology

The policy, the guard and the placement are adequate when a user's private data can be shown to stay local and under the user's control by default, and when every storage state the product cannot resolve is reported by name. They are a product boundary. They are not an encryption, security-certification or legal claim.

### Requirements

- **REQ-001** — Storage is local-first by default for private project, rule-pack, material, component and design-basis data: the data stays under the user's control on the user's machine, and modeling, solving, rule evaluation, reporting, comparison and export work with no cloud service (SOW-029; PRD §20.3, §20.4; `docs/CONTRACT.md` OPS-K-PRIV-1).
- **REQ-002** — A public repository path is never the default durable location of private project models, private rule packs, private material or component libraries, owner standards, reports, diagnostics, caches, fingerprinting hashes, credentials or proprietary data (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` §3, §6).
- **REQ-003** — Cloud storage, cloud or network sync, a hosted database, a storage daemon and a required network are not part of the product. Any of them enters only by a scope change the owner approves (SOW-029).
- **REQ-004** — Storage conventions agree with the persistence rules of DEL-00-04: versioned, unit-aware, provenance-preserving, schema-governed, round-trip testable persistence, canonical JSON as the domain truth, the SQLite store as a projection, rebuildable sidecars, and large files referenced in place.
- **REQ-005** — The project store is the local SQLite profile and the project package the multi-member archive that DEL-00-04 states. This deliverable places them and DEL-02-05 reads and writes them. The guard performs no input or output.
- **REQ-006** — The policy and the guard use path classes. Neither contains, requires or emits a real user path, a secret path or an environment-specific location.
- **REQ-007** — A stored reference to a private rule pack keeps its identity, version, checksum, source notice and redistribution status. No public artifact carries the pack's formulas, text, allowables or values (`docs/CONTRACT.md` OPS-K-RULE-3).
- **REQ-008** — Material, component, section, rule-pack, report and project data keep their provenance and their redistribution and privacy status across every storage, import and export boundary (`docs/CONTRACT.md` OPS-K-IP-2, OPS-K-DATA-3).
- **REQ-009** — A storage diagnostic names the state (unresolved, private, review required, missing, moved, hash-mismatched, blocked), the affected record and a remediation. It prints no private value, concrete path or secret, and claims no certification, approval or code compliance (PRD §21.2).
- **REQ-010** — No adapter, import, export, plugin or private-library path stores data around the path classes, the guard, the application-service boundary of the store, or the unit, provenance and public/private checks.
- **REQ-011** — Storage behaviour is examined in six families: private-path resolution, prevention of repository leakage, round-trip serialization, migration status, provenance preservation, and the report and export boundary.
- **REQ-012** — A missing storage choice, an unresolved location and an unknown context, kind, class or status are each an explicit finding or a block. None is resolved by a silent default (`docs/CONTRACT.md` OPS-K-DATA-2).

### Acceptance criteria

- **AC-001** — The policy states each path class with what it holds and its boundary, the local-first default, the list of what a public repository path never holds, and that remote storage is absent and enters only by an approved scope change. For each finer class the guard uses it names the path class that class falls under, or says that this is not decided. It contains no concrete user path, no secret and no protected content.
- **AC-002** — For a record of any private kind headed to a public or shared context, the guard blocks. For a record whose privacy class or redistribution status is absent or unknown headed to such a context, it blocks. Public metadata with a public redistribution status (public permissive, or the status the product's schemas give an invented example) is admitted as metadata only.
- **AC-003** — The guard blocks every record in a remote context or with a cloud or network reference; every record in a direct-SQL context or with a direct-SQL marker; every record that carries secret material or a payload; and every record whose review status is quarantined or rejected or whose source is suspected protected. It refuses every record in a context it does not know. Each block and each refusal names its reason. A record whose kind or path class the guard does not know is blocked or carries a finding that names what is unknown, and no rule treats the unknown member as a known one.
- **AC-004** — In a local private context the guard admits private metadata only when the record states the user's explicit intent, and blocks without it. An admitted record with a concrete path has the path reduced to its path class.
- **AC-005** — No guard output (decision, diagnostic or safe manifest) contains a concrete path, a payload or a secret. The same records in the same context give the same decisions and the same manifest, and evaluating them reads no file, opens no store and opens no network connection.
- **AC-006** — Every location to which the desktop writes project-derived data falls under exactly one path class. By default each is inside the local data directory or a folder the user chose, and none is inside a source checkout, a public example directory or a remote location. A project store left under the product's former bundle identifier is carried forward to the current local data directory and never overwrites a store already there.
- **AC-007** — With networking unavailable, creating, saving, opening, autosaving and backing up a project, storing a library or a rule pack, solving, evaluating rules, rendering a report, comparing and exporting all complete. None of those operations opens a network connection.
- **AC-008** — Every storage diagnostic carries a code, class, severity, source, affected object, message and remediation; none contains a private value, a concrete path or a secret; none states or implies certification, approval or code compliance.
- **AC-009** — The safe manifest of an admitted record keeps its privacy class, redistribution status, review status and source state. In a local private context, the manifest entry of an admitted reference to a private rule pack keeps the identity, version, checksum, source notice and redistribution status the record states. In a public or shared context no guard output for such a reference carries any of the pack's formulas, text, allowables or values.
- **AC-010** — No statement of the policy contradicts a persistence rule of DEL-00-04, and the policy selects no storage substrate, container or migration behaviour of its own.
- **AC-011** — Each of the six families of REQ-011 has an examination that observes product behaviour: for every writer of private data the path class of its destination; an attempt to store private data under a public repository path; a round trip; a store whose migration status is not current; the provenance and status members before and after storage; and the path class and admission of a report or export destination. This carries commitment DEL-12-01:1.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *Classes, not paths.* Policy and guard name kinds of place. A concrete location is decided where data is written and is never an example in a document.
- *Metadata only.* The guard decides from what a record says about itself. It never reads the thing described, so it can run anywhere and can leak nothing.
- *Block first.* The guard decides in a fixed order: context, then remote and SQL markers, then secret and payload, then review and protected status, then privacy and redistribution, then intent. An earlier block is never softened by a later rule.
- *One status vocabulary.* Privacy class, redistribution status and review status are the values DEL-01-02's policy defines and the product's schemas carry. This deliverable adds none.
- *Placement by the operating system.* The desktop's own data goes where the operating system assigns per-user application data. No platform path is written into the product.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-010, judgment by a named role (a reviewer who did not write the policy): reads the policy against PRD §20.3 and §20.4, `docs/IP_AND_DATA_BOUNDARY.md` §3 and §6 and DEL-00-04's persistence rules; reports each class, prohibition or rule that is missing, added or contradicted, and any concrete path, secret or protected content.
- **VER-002** — AC-002, AC-003 and AC-004, executable check: submits invented records of every kind, privacy class, redistribution status, review status, marker and intent to the guard in every context and observes the decision and reason code of each.
- **VER-003** — AC-005, AC-008 and AC-009, executable check: observes every decision, diagnostic and manifest the guard returns for seeded concrete paths, payloads and secrets; repeats an evaluation and compares the outputs; observes that the guard's module imports no file, database or network facility; and observes the members of a private rule-pack reference in the manifest of a local private context and the absence of seeded pack content from every output of a public or shared context.
- **VER-004** — AC-006, executable check: observes the destination of each desktop write of project-derived data against the path classes, the local data directory and the user's chosen folders, including the carry-forward of a store from a former identifier with and without a store already present.
- **VER-005** — AC-007, executable check: runs each named workflow in the desktop with networking unavailable and with network connections observed.
- **VER-006** — AC-011, judgment by a named role (a reviewer who did not write the examinations), with executable checks: maps each family of REQ-011 to the examinations that observe it and runs them; a family whose only evidence is another deliverable's adjacent fixture is reported as unexamined.

## Governing Values and Decisions — Axiology

- **AX-001** — *Private data stays local.* Better is fewer places and fewer operations through which private data can reach a public or remote location without the user's explicit act; the target is none. Convenience of sharing or synchronizing never buys a default remote write.
- **AX-002** — *Unresolved is said, not guessed.* Better is a larger share of unknown or unsupported storage states that end in a named finding the user can act on. A block is never loosened to a warning, and a default is never supplied, to let an operation through.
- **AX-003** — *Describe without exposing.* Better is less private detail (paths, payloads, hashes that fingerprint private work) in guard outputs and diagnostics for the same ability to locate and fix the problem. A finding is never suppressed to achieve it.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-029 OBJ-010 | REQ-001 REQ-002 REQ-003 REQ-006 | AC-001 | VER-001 | PRD §20.3 and §20.4, DEL-00-04's local storage profile and `docs/IP_AND_DATA_BOUNDARY.md` say what must be local and what a public path may not hold; a reviewer who did not write the policy compares it with them. The path classes are defined by this scope |
| OUT-001 | SOW-029 OBJ-010 | REQ-004 REQ-005 | AC-010 | VER-001 | DEL-00-04's persistence rules, which this deliverable did not write; the reviewer compares the policy with them |
| OUT-002 | SOW-029 OBJ-010 | REQ-002 REQ-008 REQ-012 | AC-002 | VER-002 | This scope's decision rule says what each invented record must decide; the expected decisions are written from the rule, not taken from the guard's output |
| OUT-002 | SOW-029 OBJ-010 | REQ-003 REQ-010 REQ-012 | AC-003 | VER-002 | This scope's decision rule and the exclusions of DEL-00-04's local storage profile (hosted database, cloud sync, direct SQL) say what must block; REQ-012 says an unknown kind or path class ends in a block or a named finding |
| OUT-002 | SOW-029 OBJ-010 | REQ-001 REQ-006 | AC-004 | VER-002 | This scope's decision rule says when intent is required |
| OUT-002 | SOW-029 OBJ-010 | REQ-005 REQ-006 REQ-009 | AC-005 | VER-003 | The criterion determines the result: the seeded path, payload and secret are known to the check and must be absent from every output |
| OUT-003 | SOW-029 OBJ-010 | REQ-001 REQ-002 REQ-011 | AC-006 | VER-004 | The operating system's per-user data directory and the folder the user chose are known to the check independently of the product; the path classes are defined by this scope |
| OUT-003 | SOW-029 OBJ-010 | REQ-001 REQ-003 | AC-007 | VER-005 | PRD §20.4 lists the workflows. Completion and the absence of connections are observed from outside the product |
| OUT-002 | SOW-029 OBJ-010 | REQ-009 | AC-008 | VER-003 | DEL-00-06's diagnostic contract fixes the fields; PRD §21.2 lists the prohibited claims |
| OUT-002 | SOW-029 OBJ-010 | REQ-007 REQ-008 | AC-009 | VER-003 | The criterion determines the result: the members the record states are known to the check and must be in the local private manifest, and the seeded pack content must be absent from every public or shared output. The persisted reference and its round trip are DEL-02-05's |
| OUT-003 | SOW-029 OBJ-010 | REQ-010 REQ-011 | AC-011 | VER-006 | REQ-011 lists the six families; each scenario says what must happen, and a reviewer who did not write the examinations judges the coverage |
