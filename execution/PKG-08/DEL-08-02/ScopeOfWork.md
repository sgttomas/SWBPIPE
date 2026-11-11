---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-02
package_id: PKG-08
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-039]
package_objective_refs: [OBJ-007, OBJ-012]
---

# Scope of Work — DEL-08-02 Audit manifest and model hash

## Purpose and Objective Traceability

A reviewer holding a report, a result export or a saved run can tell exactly which model, which inputs, which solver build and which rule packs produced it, and can recompute every hash that says so with tools that are not SWBPIPE. This deliverable contributes what makes that possible: one canonical form for every JSON payload the product hashes, with the model hash computed on it; the audit manifest that records the identities of a run; and the content of the run's input manifest.

It serves SOW-039 under OBJ-007 and OBJ-012 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-08, row DEL-08-02), and PRD §15.1 and §15.2 (a model state records its hash; a run records its solver version and build identifier), §19.2 (a report states the model hash, the solver version and the rule-pack checksum) and §21.1 (the product may claim provenance, version, checksum and hash records). The release target's exit criterion 4 requires a shareable report "with a reproducible canonical hash"; DEL-08-01 owns that criterion and takes the hash basis from here.

- **OUT-001** — The audit manifest: the record of a run's model hash, input-manifest hash, solver identity, unit-system reference, rule-pack references and non-JSON assets, with the validator that reports what is missing or inconsistent in one (REQ-001, REQ-003 to REQ-009).
- **OUT-002** — The canonical JSON form and the SHA-256 hashing of JSON payloads on it, including the model hash, as one implementation that every hashing surface of the product uses (REQ-002, REQ-010).
- **OUT-003** — The content of the input manifest: what a run's input manifest must list for the run to be reproduced (REQ-012).

## Deliverable Definition — Ontology

A **hash record** states an algorithm, the canonicalization basis the digest was computed on, the payload it covers, and the digest. A record without all four identifies nothing.

The **canonical form** of a JSON payload is its RFC 8785 (JCS) serialization: object keys sorted by UTF-16 code units, numbers in the ECMAScript shortest round-trip form with negative zero written as zero, strings escaped as ECMAScript does, no insignificant whitespace, UTF-8. The **model hash** is SHA-256 over the canonical form of the model document.

The **input manifest** is the canonical list of what a run was given, enough to reproduce it: the model's canonical inputs, the unit basis, the solver identity and selected settings, the rule-pack references and the external asset references. What it must list is decided here; it is built for each solve and bound to the run by the analysis-run record, and the audit manifest records its reference and hash.

The **unit-system reference** identifies the project units record of the model the run solved: the map from dimension keys to unit symbols that the model document carries. It is not a unit-system name; the model stores none.

A **rule-pack reference** in the manifest is identity, version, checksum, source notice and redistribution status. It never holds rule content. A rule pack's own declared checksum is carried under the basis the pack declares and is not recomputed here.

An **asset** is anything a run depends on that is not JSON: it is identified by its own hash over its bytes, in its own manifest entry, and is never folded into a JSON payload hash.

A **manifest finding** is the validator's statement that a field is missing, duplicated, mislabelled or carries what it must not. It has a code, a severity and the subject it concerns.

Boundary:

- **CLM-001** — The rule that canonicalization precedes every hash, and the round-trip and versioning rules for persisted documents, are DEL-00-04's. This deliverable supplies the implementation of the canonical form and of the manifest. Building the input manifest for a solve, storing it and binding it to the run, and the analysis-run record, are DEL-14-02's; a rule pack's checksum definition is DEL-06-04's; the unit catalog and the project units are DEL-02-02's, and the shape of the project units record in the model document is DEL-02-01's.
- **CLM-002** — The report package that carries the manifest as a member, and the equalities it checks between the manifest and its other members, are DEL-08-01's. The desktop's assembly of a manifest for a package request sits in DEL-08-01's request builder; what a manifest must say is decided here.
- **CLM-003** — The manifest records identities. It stores no private payload, chooses no project container, and states nothing about the engineering adequacy of what it identifies.

## Completion and Reliance Basis — Epistemology

- **REQ-001** — The audit manifest identifies the exact model input payload used for a solve or report run (also cited as DEL-08-02-R1). Source: SOW-039.
- **REQ-002** — Every JSON payload hash is SHA-256 over the payload's RFC 8785 canonical form (also cited as DEL-08-02-R2). Source: SOW-039; decomposition AB-00-04; DEL-00-04/REQ-003.
- **REQ-003** — A non-JSON or binary asset is represented by its own manifest entry with a hash over its bytes, its provenance and its inclusion status, and never enters a JSON payload hash (also cited as DEL-08-02-R3). Source: SOW-039; decomposition AB-00-04.
- **REQ-004** — The manifest records the software version, the solver version and the build identifier that produced the run (also cited as DEL-08-02-R4). Source: PRD §15.2, §19.2.
- **REQ-005** — Where a rule pack takes part in a run or a report, the manifest records its name or identifier, version, checksum, source notice and redistribution status (also cited as DEL-08-02-R5). Source: SOW-039; PRD §19.2.
- **REQ-006** — The manifest records the unit-system reference of the run, so that replay depends on no unit default (also cited as DEL-08-02-R6). Source: OBJ-012; PRD §5.6.
- **REQ-007** — A missing model hash, input-manifest hash, solver version, build identifier, unit-system reference, rule-pack checksum, rule-pack source notice, asset hash or asset provenance is a finding of the validator. Nothing is filled in to make a manifest pass (also cited as DEL-08-02-R7). Source: PRD §5.5.
- **REQ-008** — A manifest contains no protected standards text, protected table, proprietary formula, private rule-pack payload or private project data. A private payload is named by reference and hash only (also cited as DEL-08-02-R8). Source: PRD §19.4, §20.1.
- **REQ-009** — The manifest keeps mechanics solved, user-rule checked and human acceptance apart and carries no status that claims certification, approval or code compliance (also cited as DEL-08-02-R9). Source: PRD §21.2.
- **REQ-010** — Every surface of the product that hashes JSON (the model editor, the desktop hash service, the headless runner, the result export, the report package) uses this one canonical form, so that equal values give equal digests whichever path produced the JSON text. Source: decomposition AB-00-04; OBJ-012.
- **REQ-011** — A hash record names the basis its digest was actually computed on. A digest is never relabelled with a basis it was not computed on, and a slot refuses a basis it does not admit. A model-hash, input-manifest-hash or other JSON payload slot admits only SHA-256 over the RFC 8785 canonical form. A rule-pack checksum slot also admits the basis the pack declares. An asset slot admits only a hash over the asset's own bytes. Source: REQ-002, REQ-003; PRD §21.1.
- **REQ-012** — A run's input manifest lists the model's canonical inputs, the unit basis, the solver identity and selected settings, each participating rule pack by identifier, version, checksum and source notice, and each external asset by reference and hash, so that the run can be reproduced from what it names with no default supplied. It records rule packs and assets by reference and provenance and copies no protected or private payload into a public artifact. Source: SOW-039.

- **AC-001** — For every JSON payload the product hashes, the recorded digest equals SHA-256 of the payload's RFC 8785 canonical form as computed by an implementation that shares no code with the product, and the canonical form of each example in RFC 8785 equals the published one. Verify with VER-001.
- **AC-002** — Two JSON texts that differ only in key order, whitespace or number spelling of equal values have the same model hash. A change to any value the model carries changes it. The same value reaches the same digest through the desktop, the headless runner and the model editor. Verify with VER-002.
- **AC-003** — A manifest built for a run names the model and the input manifest by hash, the software version, solver version and build identifier, the unit-system reference, and each participating rule pack by identifier, version, checksum, source notice and redistribution status. Each hash record states its algorithm, basis and payload, and each digest recomputes from the payload it names. Verify with VER-003.
- **AC-004** — Removing each field REQ-007 lists from a complete manifest produces a finding that names the field and its subject, and no default value appears in its place. A duplicate reference, a hash attached to a subject other than the one it names, and a basis the slot does not admit are each a finding. Verify with VER-004.
- **AC-005** — Adding or altering a non-JSON asset changes that asset's entry and leaves the model hash unchanged. Verify with VER-004.
- **AC-006** — No manifest, fixture or example holds rule content, private project values or protected content; a private payload offered for inclusion without redaction is a blocking finding; and no manifest field or status states certification, approval, acceptance or code compliance. A manifest holds no human acceptance record, and nothing in it presents a solve or a rule-check outcome as acceptance. Verify with VER-005.
- **AC-007** — The input manifest built for an invented run lists the model's canonical inputs, the unit basis, the solver identity and selected settings, each participating rule pack by identifier, version, checksum and source notice, and each external asset by reference and hash, each equal to the run's own; and the hash the audit manifest records for it recomputes from it. Verify with VER-006.

## Production and Verification Method — Praxeology

Choices that define the product:

- The canonical form is RFC 8785. Integers beyond 2^53 are outside the interoperable JSON range it assumes; the checked profile refuses them, and a payload that must be hashed stays inside that range.
- There is one implementation of the canonical form. Other languages reach it through a binding or are checked against it; none carries a second serializer.
- The manifest records and validates. It computes no hash of a payload it does not hold, and a caller's digest is carried with the basis the caller computed it on.
- A rule pack's declared checksum is carried as declared. Its definition and its verification belong to the rule-pack lifecycle.

- **VER-001** — Comparison with a named reference (RFC 8785's own published examples, and SHA-256 and canonical bytes from a JCS implementation that imports no product code): canonicalize and hash an invented corpus covering key order, number forms, string escapes, nesting and the boundary of the safe-integer range, and compare bytes and digests.
- **VER-002** — Executable check (observes digests for reordered, respaced and respelled texts of equal values, for a one-value change, and for the same value sent through each hashing surface): the expected relations are equality and inequality, which the criterion itself determines.
- **VER-003** — Executable check (observes the fields of a manifest built from an invented run and recomputes each digest from its named payload with the reference implementation of VER-001).
- **VER-004** — Executable check (observes the validator's findings for each removed field, each duplicate, each mismatched subject and each refused basis, and the model hash before and after an asset change).
- **VER-005** — Executable check (observes the blocking finding for an unredacted private payload, the absence of any authority status in the manifest's vocabulary, and the absence of any acceptance record or of a solve or rule-check outcome presented as one) and judgment by a reviewer who did not write the fixtures (reads every public manifest fixture and example for rule content, private values and protected content).
- **VER-006** — Executable check (observes the members of the input manifest built for an invented run against REQ-012's list and the run's own inputs, and recomputes its hash with the reference implementation of VER-001).

## Governing Values and Decisions — Axiology

- **AX-001** — *One value, one digest.* Better is fewer ways for equal values to hash differently: fewer serializers, fewer bases, fewer hashing paths outside the one implementation. That may never be gained by loosening the canonical form away from RFC 8785 or by relabelling a digest.
- **AX-002** — *Identify, do not carry.* Better is a manifest from which more of a run can be re-identified and recomputed by an outside party. That may never be gained by copying private or protected payloads into it; where the two conflict, the manifest keeps the reference and the hash and states that the payload is private.
- **AX-003** — *Missing is a finding.* Better is fewer manifest fields that can be absent without a named finding. A field is never defaulted to make a manifest complete.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-002 | SOW-039 OBJ-012 | REQ-002, REQ-010 | AC-001 | VER-001 | Outside source: RFC 8785 with its published examples, and digests from a JCS and SHA-256 implementation that imports no product code. The project's own hash corpus, whose expected values the product generated, only guards against change. |
| OUT-002 | SOW-039 OBJ-012 | REQ-002, REQ-010, REQ-011 | AC-002 | VER-002 | The criterion determines the expected result: equal values give equal digests and a changed value a different one, on every hashing surface. |
| OUT-001 | SOW-039 OBJ-007 | REQ-001, REQ-004, REQ-005, REQ-006, REQ-011 | AC-003 | VER-003 | What a manifest must name comes from SOW-039 and PRD §15.2 and §19.2. Each digest is recomputed by the outside implementation of VER-001 from the payload the record names. |
| OUT-001 | SOW-039 OBJ-007 | REQ-007, REQ-011 | AC-004 | VER-004 | The list of fields whose absence must be reported comes from SOW-039 and PRD §5.5; each case is constructed by removing one field from a complete invented manifest. |
| OUT-001 | SOW-039 OBJ-012 | REQ-003 | AC-005 | VER-004 | The separation of asset hashes from JSON payload hashes comes from the decomposition's architecture basis AB-00-04; the unchanged model hash is recomputed by the outside implementation. |
| OUT-001 | SOW-039 OBJ-007 | REQ-008, REQ-009, CLM-003 | AC-006 | VER-005 | The prohibited content comes from PRD §19.4 and §20.1 and the prohibited claims from PRD §21.2; the fixtures are read by someone who did not write them. |
| OUT-003 | SOW-039 OBJ-012 | REQ-012, CLM-001 | AC-007 | VER-006 | What an input manifest must list comes from SOW-039. Each member is compared with the invented run's own inputs, which the check writes, and the hash is recomputed by the outside implementation of VER-001. That the manifest lists what the run was given does not show that a replay from it reproduces the result. |
