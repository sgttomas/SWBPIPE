---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-05
package_id: PKG-11
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-002]
---

# Scope of Work — DEL-11-05 Contributor tutorial and onboarding

## Purpose and Objective Traceability

Someone new to the project, a person or an agent, can make a first change the right way. From one short document they learn what to read first, how the work is divided into packages and deliverables, how to find the deliverable that owns what they want to change, what must never enter the public repository, what evidence a change carries and how it is handed over. They also learn what a contribution is not: maintainer acceptance is not engineering approval, and the project does not yet accept contributions from outside.

It serves SOW-033 under OBJ-001 and OBJ-002 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-11, row DEL-11-05: documentation only), and PRD §9.5, whose open-source contributor needs a contribution policy, tests and IP review gates, with the PRD's product principles, which keep external contribution intake closed until the owner activates it. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The contributor tutorial, `docs/contributor_guide/`.

## Deliverable Definition — Ontology

**Terms the tutorial implements.**

- *Contributor path.* The sequence the tutorial walks: read the repository instructions and the governing documents; find the deliverable that owns the change; read its scope and its needs; make the change inside that deliverable; run the checks the change calls for; hand the change over with its evidence.
- *Governing documents.* The repository instructions (`AGENTS.md`), the PRD, the invariant catalogue (`docs/CONTRACT.md`), the vocabulary (`docs/TYPES.md`), the technical specification (`docs/SPEC.md`), the IP and data boundary policy and the decomposition.
- *Work identity.* The flat hierarchy of the decomposition: a package holds deliverables, a deliverable has one scope and its declared needs, and every scope item, objective, package and deliverable has a stable identifier that survives renaming (`docs/TYPES.md` §1 and §2).
- *Roles.* The roles the repository instructions define, with what each may do. A TASK executes one bounded assignment and does not delegate.
- *Safe contribution checklist.* The questions a contributor answers before offering anything public: where the content came from, what rights they have in it, whether it holds protected or private content, and whether it claims more than the software may claim.
- *Handoff.* What a change is handed over with: what changed, why, what was checked and with what result, and what remains open.
- *Worked path.* One invented change taken through the contributor path from first reading to handoff, with a safe step and an unsafe step shown side by side.

**Boundary.** The tutorial is a path through what others define. The following belong elsewhere:

- the contributor workflow, the certification and the issue forms: DEL-01-03;
- the content boundary, the provenance record and the quarantine rule: DEL-01-02;
- the licence, the maintainer and the release policy: DEL-01-01;
- the architecture, the controls and the evidence a solver or rule-pack change needs: DEL-11-02;
- the roles, the working rules and the verification rules themselves: the repository instructions. Where the tutorial and a governing document differ, the governing document stands and the tutorial is the one to correct.

**Not decided.** The owner's contributor rules, the contributor legal instrument and the opening of external contribution intake. The tutorial presents each as undecided and teaches the certification workflow only as DEL-01-03 defines it.

## Completion and Reliance Basis — Epistemology

The tutorial is adequate when a newcomer who follows it reaches a correctly placed, checked and handed-over change without being told anything else, and when it says nothing a governing document does not.

### Requirements

REQ-001 to REQ-008 keep the numbers of the earlier `REQ-11-05` series. Number 009 of that series is not used.

- **REQ-001** — The tutorial starts the contributor at the repository instructions and the governing documents, before any implementation or documentation work (`AGENTS.md`; `docs/README.md`).
- **REQ-002** — The tutorial explains the flat hierarchy of packages and deliverables and the stable identifiers (`docs/TYPES.md` §1 and §2; the decomposition).
- **REQ-003** — The tutorial describes the responsibilities of the Type 1 and Type 2 roles as the repository instructions define them, and presents a TASK's authority as its one bounded assignment and nothing wider (`AGENTS.md`).
- **REQ-004** — The tutorial requires the contributor to keep protected standards content, vendor intellectual property and private data out of the public repository, and to supply provenance for public data (`docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3, OPS-K-DATA-1 to OPS-K-DATA-3, OPS-K-PRIV-1; PRD §20.1, §20.2).
- **REQ-005** — The tutorial keeps the rule-pack boundary: a public example uses invented non-code values, and code-specific data stays the user's (`docs/CONTRACT.md` OPS-K-RULE-1, OPS-K-RULE-3; SOW-033).
- **REQ-006** — The tutorial keeps the professional boundary. It claims no certification, code compliance, approval, sealing or reliance for the software, an agent, a contributor or a maintainer, and it says that accepting a contribution is not approving engineering work (PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-007** — The tutorial tells the contributor that the architecture-basis rows of the decomposition constrain a change, to be cited by identifier, and that a basis row is context for the work and decides nothing beyond what it states.
- **REQ-008** — The tutorial states what a handoff carries as the repository instructions define it: what changed, why, what was checked and with what result, what remains open (including any warning), and the route to review (`AGENTS.md`).
- **REQ-010** — The tutorial states that external contribution intake is closed, names the project licence as DEL-01-01 records it, and names no contributor legal instrument as adopted (PRD product principles; DEL-01-03:1, DEL-01-03/REQ-004).
- **REQ-011** — The tutorial includes the safe contribution checklist and the worked path.
- **REQ-012** — The tutorial tells the contributor that an unknown value, source or legal point is written as `TBD` and never invented; that a conflict or gap is surfaced and not silently resolved; and that every change keeps the schema, unit, provenance and diagnostic checks, with no adapter, plugin or public-API change going around them, the rule sandbox, the report controls or the data boundary. For a change to the solver or the rule-pack machinery it sends the contributor to the developer guide for the layer, the controls and the evidence (`docs/CONTRACT.md` OPS-K-AGENT-1, OPS-K-AGENT-2, OPS-K-DATA-2, OPS-K-UNIT-1; decomposition AB-00-07).

### Acceptance criteria

- **AC-001** — The tutorial presents every step of the contributor path in order, names every governing document with a link that resolves, and includes the safe contribution checklist and a worked path with one safe and one unsafe step.
- **AC-002** — The hierarchy, the identifier forms and the roles the tutorial describes are those of `docs/TYPES.md` §1 and §2 and of the repository instructions, and nothing in the tutorial gives a TASK authority beyond its one bounded assignment.
- **AC-003** — A newcomer who did not write the tutorial, given an invented change, reaches from the tutorial alone: the deliverable that owns the change, the criteria of that deliverable the change bears on, the checks to run, and a handoff with every item of REQ-008. Each agrees with the repository instructions and the owning scope.
- **AC-004** — The tutorial states each boundary of REQ-004 and REQ-005 and what to do with content that may be protected, which is DEL-01-02's quarantine route. Its own text and examples hold nothing of the classes PRD §20.1 keeps out of the public repository, and its examples carry no engineering value.
- **AC-005** — The tutorial says that maintainer acceptance is not professional engineering approval. No passage states or implies a claim PRD §21.2 prohibits.
- **AC-006** — The tutorial states that external contribution intake is closed, invites no pull request, patch or data contribution from outside, names MIT as the project licence without drawing a legal conclusion, and presents the contributor rules and the legal instrument as undecided.
- **AC-007** — The tutorial's account of the architecture-basis rows names where they are found and how to cite one, and its account of the certification workflow is DEL-01-03's, with nothing added.
- **AC-008** — The tutorial states the `TBD` rule, the surface-conflicts rule and the keep-the-checks rule as `docs/CONTRACT.md` and the decomposition's AB-00-07 state them, and links the developer guide for solver and rule-pack changes.

## Production and Verification Method — Praxeology

**Method choices that define the tutorial.**

- *A path, not a policy.* The tutorial orders what already exists into the sequence a newcomer needs. It states no rule of its own; each rule is one sentence and a link to the document that owns it.
- *Boundaries before mechanics.* The data and authority boundaries come before any instruction to change a file.
- *Short enough to read first.* The tutorial is the first thing read, so it is kept to what the first change needs; depth is in the developer guide and the governing documents.
- *Show one change end to end.* The worked path uses an invented change with placeholder names and no engineering value.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the tutorial's headings and links): the steps of the contributor path, the checklist and the worked path are present, and every link resolves.
- **VER-002** — Comparison with a named reference (the repository instructions, `docs/TYPES.md` §1 and §2, `docs/CONTRACT.md`, the decomposition's architecture-basis table and DEL-01-03's workflow): compare the tutorial's statements of the hierarchy, the identifiers, the roles, the handoff, the `TBD`, surface-conflicts and keep-the-checks rules, the architecture-basis rows and the certification workflow with each, and report every statement that differs or that the source does not make, each of those rules the tutorial omits, and a missing link to the developer guide.
- **VER-003** — Judgment by a named role (a newcomer, person or agent, who did not write the tutorial and has not worked in the repository): take one invented change through the tutorial and record the owning deliverable found, the criteria identified, the checks chosen and the handoff written. The reviewer of VER-002 compares each with the repository instructions and the owning scope.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the tutorial): read it against PRD §20.1 and §21.2, the boundary policy of DEL-01-02, and DEL-01-03's statement of contribution intake and the legal instrument (DEL-01-03:1, DEL-01-03/REQ-004), and report each protected or private item, each example with an engineering value, each missing boundary, each statement that can be read as approval or compliance, any invitation to contribute from outside and any instrument presented as adopted.

## Governing Values and Decisions — Axiology

- **AX-001** — *A predictable first hour.* Better is less time for a newcomer to reach a correctly placed, checked and handed-over first change, measured by the trial. That time is never reduced by leaving out a boundary, a stop rule or the reading that comes first.
- **AX-002** — *Say it once, where it lives.* Better is fewer rules the tutorial states in its own words, measured by the count of statements that differ from their governing document, which is to be zero. Convenience never justifies a second version of a rule.
- **AX-003** — *Review is not approval.* Where wording could let a contributor read maintainer review, a passing check or a merged change as approval of engineering work, the wording that claims less is the better one.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-011 | AC-001 | VER-001 | The contributor path and the governing documents are this scope's, from SOW-033 and the decomposition row. The check reads the tutorial's headings and links against them |
| OUT-001 | SOW-033 OBJ-001 | REQ-002 REQ-003 | AC-002 | VER-002 | Outside source: `docs/TYPES.md` §1 and §2 and the repository instructions, read from those documents and not from the tutorial |
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-008 | AC-003 | VER-003 | The invented change says what must be found; the expected answers are the repository instructions' and the owning scope's, and they are reached by someone who did not write the tutorial |
| OUT-001 | SOW-033 OBJ-002 | REQ-004 REQ-005 | AC-004 | VER-004 | PRD §20.1 lists what the public repository may not contain and the quarantine route is DEL-01-02's policy. The reviewer reads the tutorial's text and examples against them |
| OUT-001 | SOW-033 OBJ-002 | REQ-006 | AC-005 | VER-004 | PRD §21.2 lists the prohibited claims. The reviewer reads every passage against that list |
| OUT-001 | SOW-033 OBJ-002 | REQ-010 | AC-006 | VER-004 | DEL-01-03:1 states that intake is closed and DEL-01-03/REQ-004 leaves the legal instrument unadopted; the licence is the one DEL-01-01 records. The reviewer reads the tutorial against them |
| OUT-001 | SOW-033 OBJ-001 | REQ-007 | AC-007 | VER-002 | Outside source: the decomposition's architecture-basis table and DEL-01-03's workflow, neither written by the tutorial |
| OUT-001 | SOW-033 OBJ-002 | REQ-012 | AC-008 | VER-002 | `docs/CONTRACT.md` OPS-K-AGENT-1, OPS-K-AGENT-2, OPS-K-DATA-2 and OPS-K-UNIT-1 and the decomposition's AB-00-07, read from the catalogue and the decomposition and not from the tutorial |
