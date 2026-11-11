---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-03
package_id: PKG-11
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-003]
---

# Scope of Work — DEL-11-03 Theory notes: classical to modern centerline analysis

## Purpose and Objective Traceability

An engineer who is about to issue SWBPIPE's results under their own responsibility can read how each result was computed: what the method is, what it assumes, and where it stops being valid. The theory notes explain why routine piping analysis is done on a 3D centerline/frame model, and then, analysis by analysis, the method the product uses. They are written from open mechanics, cite only sources the public project may cite, and copy nothing from a standard.

It serves SOW-033 under OBJ-001 and OBJ-003 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-11, row DEL-11-03: cite public or permissive sources, copy no formula from standards), and the PRD's product principles, §5.4, §20.1, §21 and §22.1. In `docs/RELEASE_TARGET_v1.0.md` it owns exit criterion 13 together with DEL-11-01: the user guide and the theory notes cover every analysis of criterion 1, with its inputs, method, assumptions and limitations, and state that SWBPIPE does not determine code compliance. The notes' share is the method, its assumptions, the limits of the method, and what each input means to the method. What the user enters and does is the guide's share.

- **OUT-001** — The theory notes, `docs/theory/`: the centerline note and the notes on the analyses.

## Deliverable Definition — Ontology

**Terms the notes implement.**

- *Centerline note.* The note that explains the model itself: the centerline idea, its reading as a 3D frame, what the global model is good for, what it does not decide, the separate layer of user-rule checks, verification and validation, the handoff to local FEA, and how to read a result.
- *Required analysis.* Each analysis of exit criterion 1: sustained, operating, expansion with its basis, hydrotest, wind, three-axis static seismic with SRSS and absolute-sum combination, seismic anchor movement, modal, response spectrum and hanger sizing.
- *Analysis coverage.* For one required analysis, under headings a link can reach: the inputs as the method uses them; the method, step by step; the assumptions it rests on; and its limitations, which are the conditions under which its result is not valid or is not produced.
- *Method owner.* The deliverable whose scope defines an analysis. A note explains that method and defines none. Where a note and the method owner's scope differ, the scope stands and the note is the one to correct.
- *Mechanics statement.* A relation or formula in a note, stated with the assumptions it rests on and supported by an accepted source.
- *Source record.* The entry in a note's source notes for one source: title; locator; the section used; licence or redistribution status; source type; the claims it may support; disposition; and review notes on protected content.
- *Disposition.* `ACCEPTED`: the source may support the claims its record names and no others. `TBD`: unresolved, supports nothing. `REJECTED`: never used for public prose. `QUARANTINED`: appears protected, private or proprietary and is held for human or legal review.
- *Continuity of terms.* Classical flexibility analysis, the centerline model and the 3D frame model are three levels of one abstraction: the historical family of ideas, the geometric idealisation, and its computational form. A rule check is the separate evaluation of the user's own acceptability rules on mechanics results.
- *Lineage.* The PRD's product principles state the technical lineage: classical centerline flexibility analysis, refined through modern open methods, with no mandate to reproduce any historical text. The notes state the lineage on that authority and make no more detailed historical claim without an accepted source.

**Boundary.** The notes explain. The following belong elsewhere:

- the definition of each method, its admitted domain and its refusals: the method owner (DEL-04-07 for the solve, DEL-05-01 for loads and cases, DEL-05-02 for combinations, DEL-04-09 for modal, DEL-04-10 for response spectrum, DEL-04-11 for hanger sizing);
- the independent references and benchmark results that examine the product: the method owner and DEL-09-01;
- what the user enters and does: DEL-11-01;
- the local FEA handoff contract: DEL-10-03;
- which content may be public, and the quarantine rule: DEL-01-02.

**Not decided.** Three source selections, each the owner's to select or discharge:

- a public or permissive source for the detailed history of piping flexibility practice;
- a public or permissive source for formula-level 3D frame theory;
- a public or permissive source for local shell and solid FEA practice, should the notes go beyond stating the handoff boundary.

Also not decided: whether a relation the notes derive in the open from stated assumptions, with no external source, meets REQ-004. As REQ-004 is written, it does not.

## Completion and Reliance Basis — Epistemology

The notes are adequate when a reader competent in structural mechanics can follow each method from its assumptions to its result, can tell where it stops being valid, and can trace every historical and mechanics claim to a source the project may cite.

### Requirements

REQ-001 to REQ-011 keep the numbers of the earlier `REQ-11-03` series. Numbers 009 and 012 of that series are not used.

- **REQ-001** — The notes explain centerline analysis as mechanics, for education and review. They assert no code compliance, certification, sealing, approval or professional reliance (PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-002** — The notes distinguish the mechanics solve from user-supplied rule checks and from human professional acceptance (`docs/CONTRACT.md` OPS-K-MECH-2, OPS-K-DATA-1).
- **REQ-003** — The notes present routine global piping analysis as a 3D centerline/frame model and local shell or solid FEA as a handoff, not a substitute (`docs/CONTRACT.md` OPS-K-MECH-1; PRD product principles).
- **REQ-004** — Every historical claim and every mechanics claim cites an accepted source (decomposition row DEL-11-03; `docs/CONTRACT.md` OPS-K-IP-2).
- **REQ-005** — The notes neither copy nor paraphrase protected standards text, examples, figures or tables, code-specific formulas, material allowables, stress-intensification or flexibility tables, protected dimensional tables or proprietary commercial data (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-IP-3; PRD §20.1).
- **REQ-006** — Every quantity in a note has its unit or dimension, and every relation states its sign convention and frame where the result depends on them. No note relies on an unstated default (`docs/CONTRACT.md` OPS-K-UNIT-1, OPS-K-DATA-2).
- **REQ-007** — A claim without source support is not made. Where a subject the notes should cover lacks a source, the note says the subject is unsupported and leaves it out; it invents no history, section reference or citation (`docs/CONTRACT.md` OPS-K-AGENT-1, OPS-K-AGENT-2).
- **REQ-008** — Any example in a note uses invented values, is marked as educational, and corresponds to no standard's or vendor's example (`docs/CONTRACT.md` OPS-K-RULE-1; SOW-033).
- **REQ-010** — Every cited source has a source record with every field.
- **REQ-011** — Every source record has one of the four dispositions, and only an `ACCEPTED` source supports a claim, within the claims its record names.
- **REQ-013** — The notes give analysis coverage for every required analysis (exit criterion 13).
- **REQ-014** — The notes state, in the release target's words, that SWBPIPE does not determine code compliance (exit criterion 13; PRD §24, V1).
- **REQ-015** — Each note's account of a method agrees with the method owner's scope: the same steps, the same assumptions, the same conditions of refusal.

### Delivery commitments

- **DEL-11-03:1** — Until the owner selects or discharges each of the three source selections, the notes make no detailed historical claim, hold no formula-level frame theory, and treat local FEA only as a boundary.

### Acceptance criteria

- **AC-001** — The centerline note covers each of its subjects: purpose and boundary; the lineage; the centerline and frame abstraction with nodes, elements, local and global frames, supports, loads and resultants; what the global model is good for and what it does not decide; the rule-check layer; verification and validation; the local FEA handoff; reading results; and source notes.
- **AC-002** — For each of the ten required analyses, analysis coverage exists with all four parts, every link to it resolves, and a reader can tell from the notes' own contents which note covers which analysis.
- **AC-003** — Every step, assumption and condition of refusal a note gives for an analysis is the method owner's, and no assumption or limit the method owner states for that analysis is missing from the note.
- **AC-004** — Every mechanics statement in the notes is correct for the assumptions stated beside it: a reader competent in structural mechanics, without reading product code, finds each relation in its cited source and follows it from those assumptions.
- **AC-005** — Every source in the source notes has all eight fields and one disposition. Every historical claim and every mechanics claim cites an `ACCEPTED` source whose record names that claim. No `TBD`, `REJECTED` or `QUARANTINED` source supports any sentence. The three undecided source selections appear as undecided, and the notes hold no detailed historical claim, no formula-level frame theory and no account of local FEA beyond the handoff boundary.
- **AC-006** — The notes contain nothing of the classes REQ-005 names, no formula taken from a standard, and no example value that is not invented.
- **AC-007** — The notes state that SWBPIPE does not determine code compliance. No passage states or implies a claim PRD §21.2 prohibits, mechanics results, user-rule outcomes and human acceptance are presented as three separate things, and the global model is nowhere presented as a substitute for local analysis.
- **AC-008** — Every quantity in the notes carries a unit or dimension, and every relation whose result depends on a sign convention or a frame states it.

## Production and Verification Method — Praxeology

**Method choices that define the notes.**

- *Explain from assumptions.* Each method is presented as assumptions, then steps, then the result and its limits, so that a reader can check it without the product.
- *One note per method owner's subject.* The centerline note carries the model; each analysis is covered where its method is explained, and a reader finds it from the notes' contents.
- *Every relation cited.* A relation is stated with its assumptions and the citation of the accepted source that supports it. A relation without one is left out.
- *Sources are admitted before they are used.* A source enters the source notes with its record and a disposition before any sentence rests on it.
- *The scope governs, the note follows.* A note cites the method owner's scope for the definition and points to the independent references that examine the product. It does not present the product's own output as evidence that a method is right.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the notes' headings, links and source tables): the centerline note's subjects have their headings; each required analysis has its four coverage headings; every link into and within the notes resolves; every source table row has all eight fields and one of the four dispositions; and the code-compliance statement is present.
- **VER-002** — Comparison with a named reference (the method owner's scope for each analysis): compare each note's steps, assumptions and conditions of refusal with that scope, and report every statement that differs, that the scope does not make, or that the scope makes and the note omits.
- **VER-003** — Judgment by a named role (an independent reviewer competent in structural mechanics who did not write the notes and does not read product code): look up each relation in its cited source, follow it from the assumptions stated beside it, check units, sign conventions and frames, and report each relation that is not in its source, does not follow, or lacks a unit or convention the result depends on.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the notes): read the notes against PRD §20.1 and §21.2 and the boundary policy of DEL-01-02; open each cited source and confirm its licence status and that it supports the claim it is cited for; and report each protected item, each formula or example that corresponds to a standard's or a vendor's, each claim resting on a source that is not accepted or does not support it, each mechanics or historical claim with no source, each undecided source selection presented as settled, each passage of the three kinds DEL-11-03:1 holds back, and each statement that can be read as a prohibited claim.

## Governing Values and Decisions — Axiology

- **AX-001** — *Checkable by the reader.* Better is a larger share of mechanics statements a competent reader can verify from the note alone, by derivation or by opening a cited source. Readability never removes an assumption a result depends on.
- **AX-002** — *Clean sources before fuller history.* Better is fewer claims without an accepted source, which is to be none. A richer historical or practical account is never bought with a source of unclear licence, a paraphrase of a standard or an invented citation; the subject is left out instead.
- **AX-003** — *Explain the method that is built.* Better is fewer differences between a note and its method owner's scope, which is to be none. An explanation is never made tidier by describing a method the product does not use.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 OBJ-003 | REQ-001 REQ-002 REQ-003 | AC-001 | VER-001 | Owner ruling: the subjects are this deliverable's accepted commitment under SOW-033 and the PRD's product principles. The check reads the note against the list |
| OUT-001 | SOW-033 OBJ-003 | REQ-013 | AC-002 | VER-001 | Owner ruling: exit criterion 1 lists the analyses and exit criterion 13 lists what is covered for each. The list is read from the release target, not from the notes |
| OUT-001 | SOW-033 OBJ-003 | REQ-015 | AC-003 | VER-002 | Outside source: the method owner's scope for each analysis, which the notes do not write |
| OUT-001 | SOW-033 OBJ-003 | REQ-004 REQ-006 | AC-004 | VER-003 | Outside source: the cited public sources and closed-form classical mechanics. The reviewer looks up each relation, follows it from its assumptions and imports no product code or product output |
| OUT-001 | SOW-033 OBJ-001 | REQ-004 REQ-007 REQ-010 REQ-011, DEL-11-03:1 | AC-005 | VER-001 VER-004 | Outside source: each cited source itself, opened by the reviewer, with its own licence statement |
| OUT-001 | SOW-033 OBJ-001 | REQ-005 REQ-008 | AC-006 | VER-004 | Owner ruling: PRD §20.1 lists what the public repository may not contain; the classes are DEL-01-02's policy |
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-002 REQ-003 REQ-014 | AC-007 | VER-001 VER-004 | Owner ruling: exit criterion 13 requires the statement and PRD §21.2 lists the prohibited claims |
| OUT-001 | SOW-033 OBJ-003 | REQ-006 | AC-008 | VER-003 | Outside source: dimensional consistency and the stated conventions, checked by a reviewer who did not write the notes |
