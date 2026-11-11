---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-01
package_id: PKG-01
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-001, SOW-048]
package_objective_refs: [OBJ-001, OBJ-002]
---

# Scope of Work — DEL-01-01 Project governance baseline

## Purpose and Objective Traceability

This deliverable is the project's public statement of how it is governed. A reader of the public repository learns from it what licence the software is offered under, who maintains it and who may release it, what a public release must disclose, and which invariants every other part of the product is held to. It also tells that reader that maintaining or releasing the software is not professional approval of anyone's piping calculation.

It serves SOW-001 and SOW-048 under OBJ-001 and OBJ-002 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-01, row DEL-01-01: a single governance surface, policy language only), and the PRD's product principles, §4.2, §5.9 and §21.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches the release target through criterion 15, where DEL-10-04 records the licence consistently in the build: the licence it records is the one stated here.

- **OUT-001** — The governance baseline: the project licence text at the repository root, the invariant catalogue `docs/CONTRACT.md`, and the documentation index `docs/README.md` that leads a reader to the governance documents.
- **OUT-002** — The maintainer and release policy, `governance/MAINTAINERS.md`: who may accept content into the repository, who may release, what review a public contribution must pass before it is accepted, and what a public release must disclose.

## Deliverable Definition — Ontology

**Terms the baseline implements.**

- *Project licence.* MIT (DEC-112). The licence text is the MIT License with the project's one copyright notice.
- *Human project authority.* The owner. The owner is the sole developer, the sole maintainer and the sole release authority; the maintainer quorum is one (DEC-027).
- *Maintainer authority* and *release authority.* Authority to accept content into the repository, and authority to publish a release. Both are project governance. Neither is professional engineering approval of a calculation made with the software.
- *Invariant.* A rule of `docs/CONTRACT.md` with a stable identifier (`OPS-K-<area>-<n>`), a statement and the point at which it is enforced. A retired invariant stays listed as retired.
- *Release disclosure.* What a public release states about itself: its scope, its validation status, its known limitations, its data-boundary constraints and its professional-responsibility limitations.
- *Undecided value.* A governance value the owner has not decided. It is written as undecided.

**Boundary.** This deliverable states who decides and under what licence. The following belong elsewhere:

- what may and may not enter the public repository, and the reviewer's checklist: DEL-01-02;
- the contributor's side of a contribution, and whether intake is open: DEL-01-03;
- what the product may claim, and the report notice: DEL-01-04;
- recording the licence in package manifests and build artefacts, signing and notarizing, and the release notes template that carries the disclosures: DEL-10-04;
- the release quality gates: DEL-09-05;
- the contributor guide: DEL-11-05.

The meaning of an invariant outside the governance rows (OPS-K-GOV-1 to OPS-K-GOV-4) comes from the PRD and is enforced by the deliverable its enforcement point names. This deliverable keeps the catalogue: its identifiers, its enforcement map and its retired list.

**Not decided.**

- The owner's contributor rules: who may contribute and on what terms, beyond what the owner has recorded (sole-maintainer review, quorum one and closed external intake; DEC-027, DEC-079). OUT-002 is written from them and is not drafted without them.
- The legal-review process for licence and intellectual-property questions.
- Release maturity wording. No release carries a maturity label until the owner decides one.

## Completion and Reliance Basis — Epistemology

The baseline is policy language. It is adequate when a reader can find each governance value in one place, each value is one the owner decided, and nothing in it reads as legal advice or professional approval.

### Requirements

- **REQ-001** — The baseline states the project licence, MIT, and carries its text. Stating the licence is not legal advice and makes no legal conclusion (SOW-001; SOW-048; `docs/CONTRACT.md` OPS-K-GOV-1).
- **REQ-002** — Governance language keeps project governance approval distinct from professional engineering approval: accepting a contribution or publishing a release is never described as certifying, sealing, authenticating, endorsing or approving engineering work, or as establishing code compliance (PRD §5.9, §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-003** — The maintainer policy makes review for protected content, provenance, redistribution rights and private data a condition of accepting a public contribution. It names the gates and refers to the documents that define them (DEL-01-02 and DEL-01-03); it does not restate them (`docs/CONTRACT.md` OPS-K-GOV-4, OPS-K-IP-1 to OPS-K-IP-3; PRD §20.2).
- **REQ-004** — The release policy requires every public release to carry the five release disclosures (`docs/CONTRACT.md` OPS-K-GOV-3; PRD product principles).
- **REQ-005** — Maintainer authority, release authority and every other public-governance decision is recorded in the baseline before it is treated as project policy (`docs/CONTRACT.md` OPS-K-GOV-2). The recorded values are those of DEC-027.
- **REQ-006** — Every governance value in the baseline traces to a decision the owner made. The baseline carries no value an agent proposed and the owner did not adopt (`docs/CONTRACT.md` OPS-K-GOV-2).
- **REQ-007** — An undecided governance value is written as undecided. The baseline invents no legal conclusion, maintainer identity, quorum rule, signing process or compliance claim (`docs/CONTRACT.md` OPS-K-AGENT-1).
- **REQ-008** — Invariant identifiers are stable and never reused, a retired identifier stays on the retired list, and every invariant names its enforcement point (decomposition row DEL-01-01: single governance surface).
- **REQ-009** — From the documentation index a reader reaches the invariant catalogue, the IP and data boundary policy and the claims registry.

### Acceptance criteria

- **AC-001** — Every copy of the licence text in the repository is the MIT License with the same copyright notice, and every project document that names the project licence names MIT and draws no legal conclusion from it.
- **AC-002** — The maintainer policy names, as conditions of accepting a public contribution, review of its source and provenance, its redistribution status, the contributor's certification, protected-content risk, private-data risk, the review disposition and quarantine where it applies, and points to DEL-01-02's and DEL-01-03's documents for each.
- **AC-003** — The release policy names all five release disclosures, and states release maturity wording as undecided.
- **AC-004** — The maintainer, quorum and release-authority values in the baseline are exactly those the owner recorded: one human project authority who is sole maintainer and sole release authority, quorum one. Every other authority value appears as undecided.
- **AC-005** — No baseline document reproduces protected standards text, tables, figures or examples, or proprietary data.
- **AC-006** — Every `OPS-K-` identifier cited anywhere in the repository resolves to exactly one row of the catalogue or of its retired list, no identifier is defined twice, and every row names an enforcement point.
- **AC-007** — No baseline document describes maintainer acceptance or a release as certification, sealing, authentication, endorsement, professional approval or code compliance.
- **AC-008** — Every link in the documentation index resolves, and the index links the invariant catalogue, the IP and data boundary policy and the claims registry.

## Production and Verification Method — Praxeology

**Method choices that define the baseline.**

- *Policy language only.* The baseline contains no software and enforces nothing itself; each invariant names where it is enforced.
- *One place per value.* A governance value is stated in one document. Other documents link to it.
- *The catalogue is a table.* One row per invariant: identifier, statement, enforcement. An enforcement map lists the invariants checked at each enforcement point, and a retired list keeps withdrawn identifiers.
- *Decided or undecided.* A value is written as decided only where the decomposition's decision register holds the owner's decision for it; otherwise it is written as undecided.

**How each criterion is examined.**

- **VER-001** — Comparison with a named reference (the MIT License text published under the SPDX identifier `MIT`): compare each copy of the licence text in the repository with the reference apart from the copyright line, compare the copies with each other including the copyright line, and search the project's documents for a licence name, confirming each names MIT.
- **VER-002** — Judgment by a named role (an independent reviewer who did not write the text): read the maintainer and release policy and the governance rows of the catalogue against PRD §20.1, §20.2 and §21.2 and `docs/CONTRACT.md` OPS-K-GOV-1 to OPS-K-GOV-4, and the decided values against the decision register (DEC-027, DEC-079, DEC-112); report each missing gate, each missing disclosure, each value with no owner decision behind it, each undecided value written as decided, each approval-like description of maintaining or releasing, and any protected or proprietary content.
- **VER-003** — Executable check (observes that identifiers resolve, not what the invariants mean): collect every `OPS-K-` identifier cited in any tracked file, and test that each resolves to exactly one catalogue or retired row, that no identifier is defined twice and that every row has an enforcement entry.
- **VER-004** — Executable check (observes link targets): resolve every link in `docs/README.md` and test that the three governance documents of REQ-009 are among them.

## Governing Values and Decisions — Axiology

- **AX-001** — *The owner decides.* Better is fewer undecided governance values, each closed by a recorded owner decision. A value is never closed by an agent's draft, an invented value or a guess at what the owner would choose.
- **AX-002** — *Say it once.* Better is a shorter baseline in which each governance value is stated in one place and linked from the others, measured by the number of places a value is stated. This is never bought by leaving a reader unable to find the licence, who decides, or what a release must disclose.
- **AX-003** — *Governance is not engineering approval.* Where wording could be read either way, the reading that claims less is the better one. Clarity about what maintainers and releases do is never bought with a word that implies professional approval or a legal conclusion.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-001 SOW-048 OBJ-001 | REQ-001 | AC-001 | VER-001 | Outside source: the MIT License text published under SPDX identifier `MIT`. That the licence is MIT is an owner ruling (DEC-112) |
| OUT-002 | SOW-048 OBJ-002 | REQ-003 | AC-002 | VER-002 | Owner ruling: PRD §20.2 and `docs/CONTRACT.md` OPS-K-GOV-4 list the gates; the reviewer compares the policy with them |
| OUT-002 | SOW-048 OBJ-001 | REQ-004 REQ-007 | AC-003 | VER-002 | Owner ruling: `docs/CONTRACT.md` OPS-K-GOV-3 lists the disclosures |
| OUT-002 | SOW-048 OBJ-001 | REQ-005 REQ-006 REQ-007 | AC-004 | VER-002 | Owner ruling: DEC-027 gives the values; the comparison is with the decision register, not with the baseline's own text |
| OUT-001 | SOW-001 OBJ-002 | REQ-007 | AC-005 | VER-002 | Owner ruling: PRD §20.1 lists what the public repository must not contain |
| OUT-001 | SOW-048 OBJ-001 | REQ-008 | AC-006 | VER-003 | The criterion states the expected result: one defining row per cited identifier and an enforcement entry per row |
| OUT-001 | SOW-048 OBJ-001 | REQ-002 | AC-007 | VER-002 | Owner ruling: PRD §21.2 lists the prohibited claims |
| OUT-001 | SOW-001 OBJ-001 | REQ-009 | AC-008 | VER-004 | The criterion states the expected result: each link resolves to a file in the repository |
