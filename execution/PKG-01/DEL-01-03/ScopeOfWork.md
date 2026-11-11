---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-03
package_id: PKG-01
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-028, SOW-048]
package_objective_refs: [OBJ-002]
---

# Scope of Work — DEL-01-03 Contributor certification workflow

## Purpose and Objective Traceability

This deliverable is the contributor's path into the public repository. It says what a person offering content must state about where it came from and what rights they have in it, how an offer is routed on that evidence, when it is rejected or quarantined, and what record its disposition leaves. It also provides the public issue forms, the one public channel that stays open while contribution intake is closed, which keep protected and private data out of the tracker.

It serves SOW-028 and SOW-048 under OBJ-002 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-01, row DEL-01-03), and PRD §9.5, §20.2 and §20.3 and the PRD's product principles, which keep external contribution intake closed until the owner activates it with the required contributor and legal arrangements. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches the release target through criterion 16: the issue forms of the public repository are the ones defined here.

- **OUT-001** — The contributor workflow, `CONTRIBUTING.md`, and the contributor certification template, `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`: the attestation, the certification record, routing by evidence state, rejection and quarantine, and the disposition record.
- **OUT-002** — The public issue forms in `.github/ISSUE_TEMPLATE/`: forms for a defect report, a question and documentation feedback, with blank issues disabled.

## Deliverable Definition — Ontology

**Terms the workflow implements.**

- *Contribution.* Anything offered for the public repository that can carry data: data records, examples, report templates, schemas with example payloads, benchmark cases, import fixtures and documentation excerpts.
- *Contributor certification.* The contributor's attestation, made for repository-governance review, that the contribution is their original work or is offered with documented redistribution rights; that they have identified its sources, licences and provenance; and that it copies no protected content and no private project or rule-pack content unless redistribution rights are documented. It is not a legal instrument and is not legal clearance.
- *Certification record.* The contributor's side of the two-sided record of a contribution: the provenance record DEL-01-02 defines, a description of the contribution, a protected-content screen and a private-data screen. At disposition the reviewer and the disposition notes are added. The reviewer's side is DEL-01-02's checklist.
- *Evidence state and route.* A contribution is routed by the state of its evidence:

  | Evidence state | Route |
  |---|---|
  | Record complete, redistribution status `public_permissive` | Maintainer review |
  | A required field missing or undecided | Pending or rejected; the reviewer does not fill it in |
  | Redistribution status `unknown` | Not accepted as public data |
  | Redistribution status `private_only` | Kept out of every public surface |
  | Redistribution status `protected_suspected`, or a screen that finds suspected content | Stop; quarantine under DEL-01-02's rule, recording metadata and not the suspected content; escalate for human or legal review |

- *Disposition.* The review outcome, one of `pending`, `accepted`, `rejected`, `quarantined`, recorded with the reviewer, the date, the evidence reviewed, the reasons and the limits. It is a repository governance review only.
- *Contributor legal instrument.* The legal terms under which the project receives a contribution, such as a contributor licence agreement or a developer certificate of origin. It is separate from the certification, which records where content came from and whether it may be public.
- *Intake* and *activation.* Intake is the acceptance of external contributions. Activation is the owner's decision to open it.

**Boundary.** The workflow is the contributor's side. The following belong elsewhere:

- who reviews and accepts, and the quorum: DEL-01-01;
- what may be public, the provenance record, the quarantine rule and the reviewer's checklist: DEL-01-02;
- the provenance check on library import: DEL-03-07;
- the contributor guide that teaches this workflow: DEL-11-05;
- the public repository itself: DEL-10-04.

**Not decided.**

- The owner's contributor rules. OUT-001 is written from them and is not drafted without them.
- The contributor legal instrument, and with it the final wording of the certification.
- Whether automated protected-content and provenance checks run at every contribution review or only before a release.

## Completion and Reliance Basis — Epistemology

The workflow is adequate when a contributor can tell what they must state, a maintainer can tell where each offer goes, and every accepted contribution leaves a two-sided record. While intake is closed, it is adequate when no public surface invites a contribution and the issue forms keep protected and private data out.

### Requirements

- **REQ-001** — The workflow requires the contributor to supply, for every public data record, its source, source location, licence or an explicit statement that the licence is undecided, the contributor's identity, the certification, the redistribution status and the review status (PRD §20.2; `docs/CONTRACT.md` OPS-K-IP-2).
- **REQ-002** — The workflow rejects or quarantines a contribution that contains, or appears to contain, protected content as DEL-01-02 defines it, proprietary vendor data without redistribution rights, or private user, project or rule-pack data (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3).
- **REQ-003** — Suspected protected content is routed to human or legal review. It is never paraphrased, summarized or reformatted into public data (`docs/CONTRACT.md` OPS-K-IP-3).
- **REQ-004** — The workflow states the contributor legal instrument as undecided and adopts none. Review authority and quorum are those DEL-01-01 records.
- **REQ-005** — The workflow keeps contribution governance separate from professional engineering approval. Neither the certification nor a disposition claims legal clearance or a certification of rights, or makes any claim PRD §21.2 prohibits: certifying, sealing, approving or authenticating engineering work, declaring code compliance for professional reliance, or replacing the engineer of record or project-specific professional review (PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-006** — Every reviewed contribution has a recorded disposition, with one of the four status values, that states it is a project governance review (`docs/CONTRACT.md` OPS-K-GOV-4).
- **REQ-007** — The workflow keeps the boundary between public and private data visible to the adapters, importers, rule packs, reports and release gates that later act on contributed content: a contribution's record carries its redistribution status and its review status in the values DEL-01-02 defines.
- **REQ-008** — The workflow and the certification template are at the paths OUT-001 names, and present the contributor legal instrument, its legal sufficiency and the opening of intake as decisions that are the owner's.

### Delivery commitments

- **DEL-01-03:1** — External contribution intake is closed. Opening it is an owner act recorded as a new entry in the decision register, with a contributor legal instrument adopted on legal advice. An activation uses the sole-maintainer review and the protected-content screening defined above, or supersedes them. This scope opens no intake and adopts no instrument.
- **DEL-01-03:2** — Before public release, the owner or counsel reviews the project's public-facing language, including the public issue forms, the contribution policy and the disclaimers. DEL-01-02's policy is part of that review.
- **DEL-01-03:3** — The project's issue forms are at the root `.github/ISSUE_TEMPLATE/` of the public repository `sgttomas/SWBPIPE`, so that GitHub applies them to every issue opened there.

### Acceptance criteria

- **AC-001** — The certification template requires every field of DEL-01-02's provenance record, the contributor certification, a description of the contribution, the protected-content screen and the private-data screen, and has places for the reviewer and the disposition notes. The certification has the contributor attest each statement of the contributor certification term: original work or documented redistribution rights; sources, licences and provenance identified; and no protected content and no private project or rule-pack content copied without documented rights. The workflow sends no contribution with a required field missing or undecided to maintainer review.
- **AC-002** — For each of the five evidence states the workflow names the one route in the table above, and for suspected content it says to stop, to record metadata without copying the suspected content, and to escalate.
- **AC-003** — The disposition record carries one of `pending`, `accepted`, `rejected` or `quarantined`, the reviewer, the date, the evidence reviewed and the reasons, and states that it is a repository governance review.
- **AC-004** — Neither the certification wording nor the disposition wording states or implies legal clearance, a certification of rights, or any claim PRD §21.2 prohibits; and the workflow names no contributor legal instrument as adopted.
- **AC-005** — `CONTRIBUTING.md` and every issue form state that external contributions are not accepted, and neither asks for a pull request, a patch or a data contribution.
- **AC-006** — Every issue form carries the data-boundary caution (no protected standards content and no private project data in a report, its screenshots, logs or attachments) with a confirmation the reporter must give; no issue can be opened without passing through a form; and the forms are at the root `.github/ISSUE_TEMPLATE/` of the public repository.
- **AC-007** — Before public release, a review of the public-facing language by the owner or counsel is recorded, and it covers the issue forms and the contribution policy.

## Production and Verification Method — Praxeology

**Method choices that define the workflow.**

- *Route on evidence, not on merit.* A contribution's usefulness plays no part in its route; only the state of its record does.
- *Two-sided record.* With a quorum of one there is no second reviewer. The control is the pair of records: the contributor's certification and the reviewer's checklist, holding the same evidence fields.
- *Record metadata, not content.* A quarantine or rejection record names the contribution and the concern. It never reproduces the suspected content.
- *Forms, not blank issues.* Every public report passes through a form that carries the data-boundary caution.

**How each criterion is examined.**

- **VER-001** — Judgment by a named role (an independent reviewer who did not write the text): read the workflow and the certification template against DEL-01-02's provenance record and quarantine rule, PRD §20.2 and §21.2, and DEC-027 and DEC-079; report each missing field, each attestation of the contributor certification term the certification omits, each evidence state without a single route, each claim of clearance or approval, any instrument presented as adopted and any invitation to contribute.
- **VER-002** — Judgment by a named role (an independent reviewer acting first as contributor and then as maintainer): take five invented contributions, one in each evidence state, through the workflow and the template and record the route and the disposition record each produces. The expected routes are those of the table.
- **VER-003** — Executable check (observes the form files, a proxy for what GitHub shows): each file in `.github/ISSUE_TEMPLATE/` parses as an issue form; the configuration disables blank issues; and every form contains the contributions-closed statement, the data-boundary caution and a required confirmation.

## Governing Values and Decisions — Axiology

- **AX-001** — *Evidence before acceptance.* Better is a higher share of accepted public contributions with a complete two-sided record, and every suspected submission rejected or quarantined (PRD §25.3). Speed of acceptance and the usefulness of the content are never reasons to accept an incomplete record.
- **AX-002** — *Light for the lawful contributor.* Better is fewer fields and fewer steps for a contribution whose record is complete. This is never bought by dropping a provenance field or the certification, or by a reviewer supplying a value the contributor left out.
- **AX-003** — *Governance only.* Where the certification or a disposition could be read as more than a repository governance record, the wording that claims less is the better one.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-028 OBJ-002 | REQ-001 REQ-007 REQ-008 | AC-001 | VER-001 | PRD §20.2 lists the provenance fields, and the two screens are those of the certification record. The attestations are those this scope's contributor certification term states. The comparison is with DEL-01-02's record, not with the template's own text |
| OUT-001 | SOW-028 OBJ-002 | REQ-002 REQ-003 | AC-002 | VER-002 | The evidence-state table says what must happen to each invented contribution; the routes are reached by a reviewer who did not write the workflow |
| OUT-001 | SOW-028 SOW-048 OBJ-002 | REQ-006 | AC-003 | VER-002 | `docs/CONTRACT.md` OPS-K-GOV-4 and OPS-K-IP-2 require a review disposition; the four values are DEL-01-02's |
| OUT-001 | SOW-048 OBJ-002 | REQ-004 REQ-005 | AC-004 | VER-001 | PRD §21.2 lists the prohibited claims; DEL-01-03:1 leaves the instrument unadopted |
| OUT-001 | SOW-048 OBJ-002 | REQ-004 REQ-008, DEL-01-03:1 | AC-005 | VER-001 VER-003 | DEL-01-03:1 states that intake is closed |
| OUT-002 | SOW-028 OBJ-002 | REQ-002, DEL-01-03:3 | AC-006 | VER-003 | PRD §20.1 and §20.3 say what a public report may not contain; the repository's location is the release target's criterion 16 |
| OUT-002 | SOW-048 OBJ-002 | DEL-01-03:2 | AC-007 | HUMAN_REVIEW: the owner or counsel reviews the public-facing language before public release | The owner's or counsel's own review; nothing the product produces stands in for it |
