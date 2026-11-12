---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-04
package_id: PKG-12
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-040, SOW-029]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-04 Secret and private-library handling

## Purpose and Objective Traceability

This deliverable keeps two things from leaking: secrets, and the content of a user's private libraries. A credential is held anywhere in the product's artifacts only as an opaque reference, never as a value. A private library, rule pack or path is made known to the rest of the product by a reference record of metadata and never by its content. A release guard decides, for a stated destination, whether such a reference may appear and in what reduced form. Its result is that the product can use user-owned libraries and referenced credentials while no project, report, export, diagnostic, fixture or public artifact carries a secret or a private library's values.

It serves SOW-040 and SOW-029 under OBJ-010 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-12, row DEL-12-04, and open issue OI-010), PRD §20.3 (private rule packs, libraries, owner standards and design bases in user-controlled paths, excluded from telemetry, public examples and issue bundles), PRD §17.4 and FR-RULE-004 (private rule packs). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The handling contract, `docs/security/secret_private_library_handling.md`: what a reference record carries, what the release guard does in each context, what it always blocks, and what it never does.
- **OUT-002** — The reference records and the release guard: the metadata-only description of a private library, a private path, a secret field or a credential placeholder, and the decision on its release.

## Deliverable Definition — Ontology

**Terms the product implements.**

- *Private library.* A user-owned material, component or section library, a private rule pack, or an owner or company design basis.
- *Secret material.* A token, password, API key, licence key, certificate, private key, signing key, cookie or authorization header: anything usable as a credential.
- *Credential reference.* An opaque name for a secret held somewhere else. It identifies; it cannot be used to authenticate.
- *Reference record.* The metadata that stands for one private library, private path, secret field or credential placeholder:

  | Member | Purpose |
  |---|---|
  | reference identifier, record kind, label, version (for a library or rule pack) | Stable local identity, what kind of thing is referenced, and which edition of it |
  | storage locality | The path class the thing lives under, never a concrete path |
  | privacy class | Private by default for every private kind |
  | redistribution status, review status | The values DEL-01-02's policy defines |
  | source state and source note | A provenance summary without protected tables or formulas |
  | checksum and checksum status | Identifies the referenced content where a checksum exists |
  | value descriptor | An opaque placeholder or a path class; never content |
  | unresolved items | Every member that is absent, unknown or undecided, by name |

  A credential is a separate record from the library that needs it. Whether a reference may be transmitted is derived from the record by the guard and is not a stored flag.
- *Release context.* Where a reference would appear. Public: a public report, a public example, a public fixture, a shared model, a downstream tool. Local: a local private output, the private registry, a private diagnostic.
- *Release decision.* For one record in one context: released as metadata only, or blocked, with a reason code and a diagnostic. A set of decisions comes with a manifest of the metadata that may appear.

**Boundary.**

- DEL-03-07 decides whether a library is admitted and what the local library store keeps. DEL-06-04 keeps a rule pack's identity, checksum and privacy standing. DEL-02-05 owns the store. This deliverable describes and guards references to what they hold; it stores no library.
- DEL-12-01 owns where private libraries live. DEL-12-02 owns the redaction of values on an export route; this guard decides references, not values. DEL-12-03 owns the telemetry guard that rejects private-library and credential content.
- DEL-16-06 owns the one credential the product holds, the model-provider key of the embedded agent, and its custody in the operating system's credential store. REQ-004 applies to it.
- DEL-10-02 and DEL-00-07 own the adapter and plugin boundary and its grants. DEL-08-05 and the owner's release scan own the search for real secrets in a release.
- This deliverable is not a secret store, an encryption scheme, a key-management scheme or an integration with an external secret manager, and it requires no cloud service.

**Not decided.**

- Whether private rule packs and libraries are encrypted at rest, and with what key custody and recovery (PRD §27, question 11; decomposition OI-010).
- Which secret provider would hold a credential needed by an import or by private storage. Nothing in this scope requires such a credential.
- How a capability grant to a plugin or adapter would be recorded. The execution model and the grants are DEL-10-02's open selection.
- What a private-path reference or a credential reference (a secret-field reference or a credential placeholder) receives in a public report, a shared model or a downstream tool when its privacy class and redistribution status are known. AC-005 settles those contexts only for a private-library or rule-pack reference in a public report and for a reference whose status is unknown.
- What is done with material the guard blocks as suspected protected: where it is held and who may move or delete it. The guard blocks and reports; it moves nothing. This is commitment DEL-12-04:1.

## Completion and Reliance Basis — Epistemology

The deliverable is adequate when a secret value and a private library's content can be shown absent from everything that leaves the user's control, and when every uncertain reference is blocked or flagged by name.

### Requirements

- **REQ-001** — A private library or rule pack is stored outside public example directories by default and is marked private in its metadata (PRD §20.3; FR-RULE-004; `docs/IP_AND_DATA_BOUNDARY.md` §6).
- **REQ-002** — Private rule packs, component libraries, material data, project files and calculation results are not transmitted by telemetry by default (PRD §20.3, §20.5).
- **REQ-003** — A private library's registry record includes its privacy class, a provenance or source summary, its redistribution status, its review status, and its checksum status where a checksum is available (`docs/IP_AND_DATA_BOUNDARY.md` §4).
- **REQ-004** — Project, registry, report and test artifacts hold opaque credential references only, never a usable credential value.
- **REQ-005** — Export, report, issue-report and shared-model paths warn before exposing private data and redact or omit protected and private values by default (SOW-040; PRD §20.3).
- **REQ-006** — Where protected standards data or proprietary source content is suspected in a private-library import or a public contribution path, ingestion or publication stops and the matter goes to quarantine and human review (`docs/IP_AND_DATA_BOUNDARY.md` §5; `docs/CONTRACT.md` OPS-K-IP-3).
- **REQ-007** — A plugin or adapter is denied private-library, private-rule-pack, filesystem and network access unless an explicit grant exists.
- **REQ-008** — No plugin, adapter, import, export or private storage path goes around schema validation, unit checks, provenance checks, privacy controls, protected-content screening, diagnostics, checksums or report controls.
- **REQ-009** — A report or an exported result may identify a private rule pack or library by name, version, checksum and source note, without exposing protected formulas, private values or proprietary source content (`docs/CONTRACT.md` OPS-K-RULE-3).
- **REQ-010** — Missing provenance, an unknown redistribution status and an uncertain privacy class are each an explicit diagnostic or finding, never a silent default (`docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-DATA-3).

### Acceptance criteria

- **AC-001** — The contract states the members of a reference record, the guard's behaviour in each release context, everything it blocks in every context, and what it never does. It contains no real credential, no usable credential example, no real private path and no private or protected content.
- **AC-002** — A reference record built for any private kind is private by default and carries every member of the record definition. A member that is absent, unknown or undecided is listed among the record's unresolved items and yields a finding; none is defaulted to a public value.
- **AC-003** — A record that carries secret material in any member is blocked in every release context, and no decision, diagnostic or manifest the guard returns contains the secret's value. A secret-field reference and a credential placeholder hold only an opaque reference.
- **AC-004** — No project document, library or rule-pack store record, report, export, diagnostic, log, fixture or test of the product contains a usable credential value, the model-provider credential included.
- **AC-005** — In a public fixture or example a private reference is blocked. In a public report a private-library or rule-pack reference is released as metadata only (its label, version, checksum and source note) with a warning, and nothing of its content. In a public, shared or downstream context a private reference whose redistribution status or privacy class is unknown or undecided is blocked. In a local context private metadata is released only on the user's explicit intent, with a warning.
- **AC-006** — In every release context the guard blocks a record that carries a private-library payload, a private-path payload or any payload marker; a cloud or network reference; an external secret-manager reference; a direct-SQL marker; a storage-bypass marker; or a review status of rejected or quarantined or a source suspected protected. A concrete path in a released record is reduced to its path class. A blocked record causes no file to be written or moved.
- **AC-007** — The same records in the same context give the same decisions and manifest, and evaluating them reads no referenced file and opens no network connection.
- **AC-008** — Every diagnostic carries a code, class, severity, source, affected object, message and remediation; identifies the reference and the reason and never a value, a path or content; uses the provenance or IP-boundary warning class for missing provenance and unknown redistribution; and states or implies no approval, certification or code compliance.
- **AC-009** — A telemetry event attempt that names a private-library, private-rule-pack, private-path, secret or credential-reference field is rejected by the product's telemetry guard under every configuration. This is observed on the telemetry guard and on the product's routes, not on reference records. This carries commitment DEL-12-04:3.
- **AC-010** — A plugin or adapter declaration that asks for private-library, private-rule-pack, filesystem or network access is refused or blocked without a grant, and no adapter runs. This is observed on the adapter gate.
- **AC-011** — By default every private library and rule pack the product stores is in the local library store under a private path class, is marked private, and none is under a public example directory.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- *References over secrets.* What the product writes down about a credential is its name. The value has one home outside the product's artifacts.
- *Metadata only.* A record describes; it never contains. The guard reads the record and never the thing referenced.
- *Private unless shown public.* A private kind is private by default, and an unresolved member blocks or warns according to the context. Nothing is promoted on missing data.
- *A checksum identifies, it does not permit.* A matching checksum says which content is meant. It never makes that content public.
- *Invented fixtures.* Tests use sentinel markers and symbolic path classes, never realistic tokens, keys or paths.

**How each criterion is examined.**

- **VER-001** — AC-001, judgment by a named role (a reviewer who did not write the contract): reads the contract against PRD §20.3, `docs/IP_AND_DATA_BOUNDARY.md` §4 to §6 and this scope; reports each member, context rule or block that is missing, added or weakened, and any credential-like example, real path or private content.
- **VER-002** — AC-002, AC-003, AC-005 and AC-006, executable check: builds invented reference records of every kind, with each member present, absent and unknown and with each marker, submits them to the guard in every release context with and without intent, and observes each decision, reason code, unresolved list and manifest.
- **VER-003** — AC-007 and AC-008, executable check: repeats an evaluation and compares the outputs; observes that the guard's module imports no file, database or network facility; and observes the fields and text of every diagnostic returned for records seeded with sentinel secrets, paths and content.
- **VER-004** — AC-004, executable check with judgment by a named role (a reviewer who did not write the code that holds the credential): searches project documents, store records, reports, exports, diagnostics, logs, fixtures and tests written during a session in which a model-provider credential is configured for a sentinel credential value, and the reviewer examines every place the credential is read or written.
- **VER-005** — AC-009 and AC-010, executable check: submits telemetry event attempts naming each private-library and credential field to the telemetry guard under each configuration, and adapter declarations requesting each kind of access to the adapter gate, and observes the rejection and that nothing runs.
- **VER-006** — AC-011, executable check: imports an invented private library and saves an invented private rule pack, and observes where each is stored, its path class and its privacy marking.

## Governing Values and Decisions — Axiology

- **AX-001** — *A secret has one home.* Better is fewer artifacts and code paths in which a credential value can exist; the target is the operating system's credential store alone. Convenience of configuration never buys a credential in a project file, a report, a log or an example.
- **AX-002** — *Uncertain means private.* Better is a larger share of references with unresolved provenance, redistribution or privacy that end blocked or flagged by name. Realism of an example or completeness of a report never buys a promotion to public.
- **AX-003** — *Enough metadata to act, no content.* Better is a reference record that lets the product validate, warn, hash, redact and route with fewer members. It is never bought by carrying a value, a formula or a concrete path.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-040 SOW-029 OBJ-010 | REQ-003 REQ-004 REQ-005 | AC-001 | VER-001 | PRD §20.3 and `docs/IP_AND_DATA_BOUNDARY.md` §4 to §6; the record's members are defined by this scope; a reviewer who did not write the contract compares it with them |
| OUT-002 | SOW-040 SOW-029 OBJ-010 | REQ-001 REQ-003 REQ-010 | AC-002 | VER-002 | This scope's definition of the reference record says what each invented record must carry and list; expected results are written from it, not taken from the product |
| OUT-002 | SOW-040 OBJ-010 | REQ-004 | AC-003 | VER-002 | The criterion determines the result: the sentinel secret is known to the check and must be absent from every output |
| OUT-002 | SOW-040 SOW-029 OBJ-010 | REQ-004 | AC-004 | VER-004 | The criterion determines the result: the sentinel credential value is known to the check and must be found in none of the product's artifacts |
| OUT-002 | SOW-040 OBJ-010 | REQ-005 REQ-009 | AC-005 | VER-002 | This scope's rule per release context says what each invented record must receive |
| OUT-002 | SOW-040 SOW-029 OBJ-010 | REQ-006 REQ-008 | AC-006 | VER-002 | This scope's list of what every context blocks; `docs/IP_AND_DATA_BOUNDARY.md` §5 requires suspected content to stop |
| OUT-002 | SOW-029 OBJ-010 | REQ-008 | AC-007 | VER-003 | The criterion determines the result: equal outputs, and no file or network facility in the guard |
| OUT-002 | SOW-040 OBJ-010 | REQ-010 | AC-008 | VER-003 | DEL-00-06's diagnostic contract fixes the fields and classes; PRD §21.2 lists the prohibited claims |
| OUT-002 | SOW-040 SOW-029 OBJ-010 | REQ-002 | AC-009 | VER-005 | PRD §20.3 and §20.5 exclude this content from telemetry; the decision is DEL-12-03's guard's, which this deliverable did not write |
| OUT-002 | SOW-029 OBJ-010 | REQ-007 REQ-008 | AC-010 | VER-005 | No adapter execution model or grant is selected, so the expected result of every request is that nothing is granted and nothing runs; the decision is DEL-10-02's gate's |
| OUT-002 | SOW-029 OBJ-010 | REQ-001 | AC-011 | VER-006 | PRD §20.3 and FR-RULE-004 say where private libraries and rule packs live and how they are marked; the location is observed in the local store |
