---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-04
package_id: PKG-06
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-042]
package_objective_refs: [OBJ-002, OBJ-005]
---

# Scope of Work — DEL-06-04 Private rule-pack lifecycle and checksum handling

## Purpose and Objective Traceability

A user's rule pack stays the user's, and what was applied can be shown later. Every pack has an identity, a version, a source notice, a stated privacy and redistribution standing, and a checksum of its exact content. A pack the user creates is private unless its author records otherwise. A pack whose content has changed since its checksum was taken, whose rights are unstated, or that may hold protected content is flagged before it is used or shared. A report, an export or a run record can then name the pack that was applied, by identity, version, checksum and source note, without holding any of its formulas or values.

It serves SOW-042 under OBJ-002 and OBJ-005 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-06, row DEL-06-04, which keeps this deliverable to the local lifecycle and leaves access control to PKG-12), and `docs/PRD.md` FR-RULE-004, §17.2, §17.4, §10.6 step 4, §5.7, §19.2 and §20.3, with `docs/CONTRACT.md` OPS-K-RULE-3 and OPS-K-PRIV-1 and threat STM-010 of `docs/security/threat_model.md`. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It supports criterion 4 through the rule-pack reference a report carries (FR-REP-001).

- **OUT-001** — The private rule-pack lifecycle: the lifecycle record and its findings, the pack checksum and its verification, the local keeping of packs, and the audit reference by which other outputs cite a pack.

## Deliverable Definition — Ontology

**Terms the lifecycle implements.** The vocabularies of privacy class, redistribution status, review status and lifecycle status are those of the pack format (DEL-06-01).

- *Lifecycle record.* What the lifecycle reads from a pack: its identity and name, format version, rule-pack version, grammar version, lifecycle status, source notice, privacy class, redistribution status, review status, protected-content review, and checksum.
- *Private by default.* A pack the product creates for a user is classified as private user data with private-only redistribution. A pack is publicly exportable only when its privacy class is public, its redistribution status is public-permissive or invented example, its review is accepted and no protected content is suspected. Unknown rights are private.
- *Pack checksum.* SHA-256 over the RFC 8785 canonical form of the pack document with its checksum member excluded. The checksum record states the algorithm, the canonicalization, the payload it refers to, its scope, what was excluded and the value. No path, timestamp or session state is hashed.
- *Grammar binding.* The pack's grammar version is a member of the hashed content, so a change of grammar version changes the checksum.
- *Stale checksum.* A recorded value that differs from the value recomputed from the pack's content.
- *Lifecycle findings.* A missing identity, version, grammar version, source notice, redistribution status, review status or checksum; a grammar version that is malformed or not bound in the hashed content; a stale checksum; suspected protected content, which quarantines the pack; an attempted public export of a private pack; and a professional-boundary violation.
- *Blocked pack.* The lifecycle's verdict, returned with its findings, that a pack is not to be used until a finding is resolved. Suspected protected content always blocks.
- *Quarantined pack.* A pack with the protected-content finding. The finding stops the pack's use and escalates it for human or legal review, under the quarantine rule of the boundary policy (DEL-01-02). The pack is blocked, is not run and is not publicly exportable, and stays so until that review resolves the finding.
- *Audit reference.* What another output may hold about a pack: its identity, name, rule-pack version, grammar version, checksum, source notice, and privacy, redistribution and review standing, with a statement that a private pack's content is withheld. It holds no formula, table or value.
- *Local keeping.* Packs are kept in the user's local project store, by project and pack identity, and nowhere else: saved, opened, listed and deleted. A draft with findings can be saved. Opening a pack validates it again. A stored pack of a format version that is no longer current stays listed, opens with its refusal shown and can be deleted.
- *Manifest hash.* The checksum of a pack-related asset that is not JSON, taken over its exact bytes and recorded with a reference to the asset.

**Undecided.**

- Whether stored private packs are encrypted, and the default (PRD §27, question 11; the decomposition's OI-010). The decision is the owner's and its implementation PKG-12's.
- How pack-related assets that are not JSON are partitioned under manifest hashes.

**Boundary.** This deliverable keeps packs, identifies them and flags their standing. The pack format is DEL-06-01's. The project store that holds the packs is DEL-02-05's, under the storage profile REQ-011 states. Storage location policy, encryption, access control, permission persistence, secrets, and the exclusion of private data from telemetry and issue bundles are PKG-12's. A model's reference to a pack and its resolution are DEL-02-01's and DEL-14-02's, the audit manifest is DEL-08-02's and the report is DEL-08-01's. Findings for a missing rule value or unit are DEL-06-01's at validation and DEL-06-03's at the run; carrying them through this lifecycle, and any execution of a pack on behalf of a plugin or adapter, are outside v1.0, as `docs/RELEASE_TARGET_v1.0.md` states for rule packs (also cited as DEL-06-04:2 and DEL-06-04:3).

## Completion and Reliance Basis — Epistemology

A checksum establishes which content was applied. It establishes nothing about whether that content is correct, lawful to share or fit for use.

### Requirements

Requirement numbers 001 to 012 are those of the earlier `R-06-04-0nn` series.

- **REQ-001** — A rule-pack lifecycle record includes the pack's stable identity, human-readable name, version, source and provenance notice, redistribution and privacy status, and checksum metadata.
- **REQ-002** — A private rule pack is marked private by default unless a public-permissive redistribution basis is explicitly recorded (FR-RULE-004).
- **REQ-003** — Public project artifacts contain no private rule-pack content, protected standards text, protected table, copied code formula, proprietary vendor data, owner standard or company design basis.
- **REQ-004** — The hash of a JSON rule-pack payload is taken over its canonical JSON form (RFC 8785) and recorded with its algorithm, canonicalization, payload reference and value.
- **REQ-005** — A pack-related asset that is not JSON is represented by a manifest hash with an explicit payload reference.
- **REQ-006** — A checksum identifies what was hashed and does not depend on an environment-local path, a timestamp, session state or any other volatile field.
- **REQ-007** — The lifecycle emits a diagnostic for a missing source notice, a missing redistribution status, a stale or missing checksum, suspected protected content and an attempted public export of private content.
- **REQ-008** — Reports and exports can reference a private pack's identity, name, version, checksum and source note without exposing its formulas or values (also cited as DEL-06-04:1).
- **REQ-009** — No lifecycle status states or implies professional certification, sealing, approval, endorsement or automatic code compliance (PRD §21.2).
- **REQ-010** — A stored pack keeps the unit metadata of every numeric input and value: what is opened is what was saved (also cited as DEL-06-04:4).
- **REQ-011** — Storage follows the project persistence contract (`docs/architecture/persistence_contract.md`; decomposition AB-00-04): packs are kept in the user's local project store, whose domain truth is the canonical JSON payload and whose index is a rebuildable projection, with no hosted database, daemon, required network or direct plugin or adapter SQL access; a pack that travels in the portable project package travels as a canonical-JSON member of it. The encryption default, access-control policy, permission persistence and secret handling stay with PKG-12.
- **REQ-012** — No adapter or plugin places a pack in the local store or in a run around validation, the privacy standing, the protected-content check, the checksum or the report controls.
- **REQ-013** — The grammar version a pack declares is bound by its checksum.

### Acceptance criteria

- **AC-001** — For a pack document, the recorded checksum equals SHA-256 of the RFC 8785 canonical form of the document without its checksum member, as computed by an implementation that shares no code with the product. Reordering members or changing whitespace leaves it unchanged, and changing any other member, the grammar version included, changes it.
- **AC-002** — A pack whose recorded checksum is absent, or differs from the recomputed value, yields the missing or the stale finding. A pack refused for its format version yields the stale finding only when its recorded checksum differs from the value recomputed from its content; the refusal alone yields none.
- **AC-003** — The same pack saved at two times, under two paths and in two sessions has the same checksum, and its checksum record names what was hashed and what was excluded.
- **AC-004** — Each lifecycle finding is raised by name for a constructed record with that defect and for no other record: a missing identity, version, grammar version, source notice, redistribution status or review status; a malformed grammar version; a grammar version absent from the hashed content. A record with suspected protected content is quarantined: it yields the protected-content finding, which marks the pack for human or legal review, is reported as blocked and is refused for public export, and stays so until that review resolves the finding. No missing member is defaulted.
- **AC-005** — A pack the product creates for a user is classified private with private-only redistribution. A pack is reported publicly exportable only under the four conditions of the definition above, and a public export requested for any other pack is refused with a named finding.
- **AC-006** — A pack saved to the local store and opened is equal to what was saved in canonical form, units included, and its checksum verifies. A draft with findings saves, and a pack opened from the local store is validated again with its findings returned. The list shows each stored pack's identity, version, lifecycle status and privacy class. A deleted pack is gone, and a stored pack of another format version remains listed, opens with its refusal and can be deleted. Saving writes to the local store only and transmits nothing.
- **AC-007** — The audit reference of a private pack that holds sentinel formula text, table rows and values contains none of them, and holds only the members of its definition. A report or export request made for a run that used a local pack carries that pack's audit reference, with a checksum equal to the one recomputed from the stored pack.
- **AC-008** — No lifecycle status, finding or record states or implies approval, certification, sealing or code compliance, and a pack whose boundary record asserts one yields the boundary finding.
- **AC-009** — A pack-related asset that is not JSON is covered by a checksum record that names the asset and whose value equals the hash of the asset's exact bytes.
- **AC-010** — Every fixture and example of the lifecycle is invented, and no public artifact of it holds private pack content or protected content.
- **AC-011** — No adapter or plugin path puts a pack into the local store or into a run around validation, the privacy standing, the protected-content check, the checksum or the report controls.

## Production and Verification Method — Praxeology

**Method choices that define the lifecycle.**

- The canonical JSON document is the pack; the local store is where it is kept. The checksum is taken over the canonical document and never over stored bytes or a file.
- The checksum member is the one exclusion from the hash, because it holds the value.
- A finding is raised and shown; a stale or missing checksum is never repaired silently on open.
- An audit reference is built from the lifecycle record alone, so it cannot carry content the record does not hold.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-003, comparison with a named reference: RFC 8785 canonicalization and SHA-256 computed by an implementation that shares no code with the product, over invented packs; with an executable check that observes the checksum across reordering, whitespace, single-member changes, save times, paths and sessions.
- **VER-002** — AC-002, AC-004, AC-005 and AC-008, executable check: observes the findings and the exportable standing for constructed records with each defect and each combination of privacy, redistribution and review standing, and the classification of a newly created pack.
- **VER-003** — AC-006, executable check: saves, opens, lists and deletes constructed packs in a local store, compares the opened document with the saved one, observes the findings returned on opening, and observes that nothing is written or sent elsewhere.
- **VER-004** — AC-007, executable check: builds the audit reference and a report and an export request for a run with a private pack that carries sentinel content, and searches each for the sentinels and for the recomputed checksum.
- **VER-005** — AC-009, comparison with a named reference: the hash of an invented asset's bytes computed by a tool that shares no code with the product.
- **VER-006** — AC-010 and AC-011, judgment by a reviewer who did not write the lifecycle, with the protected-content scan and an executable check of the repository's module graph: examines the fixtures and examples, and every adapter or plugin path by which a pack could reach the store or a run.

## Governing Values and Decisions — Axiology

- **AX-001** — *Private unless shown otherwise.* Better is fewer ways a pack's content can leave the user's machine without the user's explicit act; the target is none. Ease of sharing never weakens the default, and unknown rights are never treated as public.
- **AX-002** — *The checksum means the content.* Better is a checksum that identifies more exactly the content that was applied: fewer members outside the hash and fewer records left unverified. What is hashed is never narrowed to keep a checksum stable, and a checksum is never presented as evidence that a pack is correct.
- **AX-003** — *Cite, do not copy.* Better is a reference that lets a reader identify the applied pack more exactly. It never does so by placing a formula, a table or a value in a report, an export or a public artifact.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-042 OBJ-005 | REQ-004 REQ-013 | AC-001 | VER-001 | Outside source: RFC 8785 and SHA-256, computed without product code. The one exclusion and the grammar binding are those REQ-004 and REQ-013 state, under the canonical-JSON hashing of the decomposition's AB-00-04 |
| OUT-001 | SOW-042 OBJ-005 | REQ-007 | AC-002 | VER-002 | The records are constructed, so the criterion states the expected finding; the recomputed value rests on the grounds of AC-001 |
| OUT-001 | SOW-042 OBJ-005 | REQ-006 | AC-003 | VER-001 | The criterion fixes the expected result: equality across times, paths and sessions |
| OUT-001 | SOW-042 OBJ-002 OBJ-005 | REQ-001 REQ-007 | AC-004 | VER-002 | SOW-042 and `docs/CONTRACT.md` OPS-K-RULE-3 name what a pack must carry, and OPS-K-IP-3 the quarantine and escalation. The records are constructed. The check observes the finding, its review marking and the blocked standing; the review that resolves a quarantine is a person's and is not examined here |
| OUT-001 | SOW-042 OBJ-002 | REQ-002 | AC-005 | VER-002 | PRD FR-RULE-004 and §17.4. The records are constructed, so the criterion states the expected standing |
| OUT-001 | SOW-042 OBJ-002 | REQ-010 REQ-011 | AC-006 | VER-003 | The criterion fixes the expected result: the pack read back equals the pack written. Local-only keeping is PRD §20.3 |
| OUT-001 | SOW-042 OBJ-002 OBJ-005 | REQ-008 | AC-007 | VER-004 | PRD §19.2 and §19.4 state what a report cites and may not contain. The sentinel content is constructed, and the checksum rests on the grounds of AC-001 |
| OUT-001 | SOW-042 OBJ-005 | REQ-009 | AC-008 | VER-002 | PRD §21.2 and `docs/claims_registry.md` |
| OUT-001 | SOW-042 OBJ-005 | REQ-005 | AC-009 | VER-005 | Outside source: the hash algorithm applied to the asset's bytes without product code |
| OUT-001 | SOW-042 OBJ-002 | REQ-003 | AC-010 | VER-006 | PRD §20.1. The reviewer compares against sources the product did not write |
| OUT-001 | SOW-042 OBJ-002 | REQ-012 | AC-011 | VER-006 | The decomposition's AB-00-07, no bypass. The comparison is the repository's module graph |
