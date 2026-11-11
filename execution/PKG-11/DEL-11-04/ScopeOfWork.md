---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-04
package_id: PKG-11
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-008]
---

# Scope of Work — DEL-11-04 Invented educational example models

## Purpose and Objective Traceability

A newcomer can open a small piping model that is plainly made up, solve it, and see what the software does with it: first the mechanics alone, then a rule check run against a rule pack that is just as plainly fictional. The examples teach how the product works without teaching a single engineering value, and the project's tests use the same files, so an example that stops working is noticed. Nothing in an example comes from a standard, a vendor or a real project, and nothing in it can be taken for a design basis.

It serves SOW-033 under OBJ-001 and OBJ-008 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-11, row DEL-11-04: examples must be clearly non-code), and PRD §5.4, §6.2 goal 4 (educational and research examples using invented, public-domain or permissively licensed data), FR-RULE-007, §20.1 and §21.2, which bars any claim that public example data is suitable for project use. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches criterion 14, whose release-candidate content scan covers the public examples.

- **OUT-001** — The invented example models, `examples/models/invented/`: at least one mechanics-only example and one fake-rule-pack example, each with its description and notice.

## Deliverable Definition — Ontology

**Terms the examples implement.**

- *Invented example model.* A model document of the current format whose every value was made up for the example. It is public, small enough to audit by reading, and the same file serves teaching and the project's tests.
- *Mechanics-only example.* An example that demonstrates the mechanics solve: units, nodes and elements, supports, load cases, the solve, its diagnostics and its results. It carries no rule-pack reference, no code-specific combination and no acceptance value.
- *Fake-rule-pack example.* An example that demonstrates the user-rule-check workflow. It cites the invented rule pack of DEL-06-05 by identity, version and pack checksum, and a rule-check run on it shows that a solved model, a rule-check outcome and professional acceptance are three different things.
- *Clearly non-code.* A reader cannot mistake the example for engineering data: its names are fictional, its values are toy values that resemble no code allowable, formula, load combination or acceptance rule, and it names no standard, clause, vendor, commercial software or real project.
- *Example description.* What the example demonstrates and what a reader should observe on solving or checking it, kept in the example or beside it.
- *Non-reliance notice.* The statement each example carries: it is invented for education and testing, it is not a design basis, it is not a standards example, and it is not suitable for engineering reliance.
- *Invented provenance.* The provenance record of DEL-01-02 on every data set of an example, stating an original invented origin, a public-permissive redistribution status and the contributor's certification.

**Boundary.** This deliverable is the example models. The following belong elsewhere:

- the invented rule pack the fake-rule-pack example cites, and its notice: DEL-06-05;
- the model document format and the bundled demo model of the desktop: DEL-02-01;
- the rule-check run and its outcomes: DEL-06-03;
- the benchmark models and the reference system, whose purpose is to examine the solver: DEL-09-01;
- fixtures in the physical-to-analytical transform's reference format: DEL-13-04;
- the status vocabulary an example's solve and rule check report in: DEL-05-04;
- the guide text that walks a reader through an example: DEL-11-01.

**Not decided.**

- Whether the fake-rule-pack example must also show an outcome within or exceeding the invented limit, or need only show the incomplete outcome. It is undecided with DEL-06-05.
- The owner's disposition of two review findings on the examples: that an example exercises the physical model as the source of truth, and that the examples' validation covers format, hash and round trip.

## Completion and Reliance Basis — Epistemology

The examples are adequate when a reader can open, solve and check them and observe what their descriptions say, and when no reader could take anything in them for engineering data. An example shows that the software's mechanism works on made-up content. It is evidence about the software and about no piping system.

### Requirements

REQ-001 to REQ-006 keep the numbers of the earlier `R-DEL-11-04` series. Number 007 of that series is not used.

- **REQ-001** — A public example uses invented or original data only. It holds no protected standards content, no standards-derived example, no commercial-software example, no proprietary vendor data, no code allowable, no stress-intensification or flexibility data and no protected dimensional table (PRD §20.1, FR-RULE-007; `docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3, OPS-K-RULE-1).
- **REQ-002** — Every example carries the non-reliance notice and makes no claim of certification, approval, sealing or code compliance (PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-003** — A mechanics-only example demonstrates open mechanics with every quantity unit-bearing, and embeds no code-specific load combination, allowable, acceptance formula or interpretation of a standard (`docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-UNIT-1).
- **REQ-004** — A fake-rule-pack example is clearly non-code: fictional labels and placeholder values that approximate no design-code formula, allowable or acceptance criterion (`docs/CONTRACT.md` OPS-K-RULE-1, OPS-K-RULE-3).
- **REQ-005** — An example leaves a value out only on purpose, to demonstrate a finding, and says so in its description. Nothing an example needs is supplied by a silent default (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-006** — An example used in a test is evidence of software behaviour only. Neither the example nor a test on it is described as validation of a piping system, as code compliance or as professional approval (OBJ-008; PRD §21.2).
- **REQ-008** — Each example is a model document of the current format, so that the product opens it, solves it and saves it like any other model (PRD §5.2; SOW-033).
- **REQ-009** — Each example has an example description, and every data set in it carries invented provenance (PRD §20.2; `docs/CONTRACT.md` OPS-K-IP-2).
- **REQ-010** — A checksum an example cites is the checksum of the cited content as it is: for the invented rule pack, the pack checksum it carries.

### Delivery commitments

- **DEL-11-04:1** — The two review findings named under "Not decided" are closed by the owner's disposition. Meeting AC-002 is evidence for that disposition and is not the disposition.

### Acceptance criteria

- **AC-001** — The example set has at least one mechanics-only example and at least one fake-rule-pack example. Each has an example description, and the mechanics-only example carries no rule-pack reference.
- **AC-002** — Each example validates against the model document format at its current version and passes the document validator with no blocking finding. It opens in the release, and saved and reopened it returns equal content and the same model hash.
- **AC-003** — Each example solves in the release with the mechanics-solved status and no blocking diagnostic, except where its description says it demonstrates a named finding, and then the product reports exactly that finding. Every result value an example or its description states is the value the release reports for that example.
- **AC-004** — The fake-rule-pack example cites the invented rule pack by its identity, version and the pack checksum it carries (DEL-06-04's definition), and the cited value equals that checksum recomputed from the pack's content as it is. A rule-check run of that pack on the example reports the outcome the example's description says it will, with every missing input named, and the mechanics status is unchanged by the run.
- **AC-005** — Every value in every example is invented. No example holds content of the classes REQ-001 names; names a standard, a clause, a vendor, a commercial software product or a real project; or carries a name, value, formula or threshold that resembles a code allowable, combination or acceptance rule. Every data set carries invented provenance with all fields of the provenance record.
- **AC-006** — Every example carries the non-reliance notice in each of its four parts, and no example carries a human-approval status or any statement of compliance, certification or approval.
- **AC-007** — Every quantity in an example carries its unit as the model document defines a quantity (node coordinates in the project's length unit), and every value an example leaves out is named in its description with the finding it demonstrates.

## Production and Verification Method — Praxeology

**Method choices that define the examples.**

- *The product's own document.* An example is a model document like any a user saves, with nothing special-cased for it. What the product does with a user's model, it does with the example.
- *Toy before realistic.* Values are chosen to be easy to check by hand and impossible to mistake for practice. Where realism and auditability pull apart, auditability wins.
- *Two families kept apart.* The mechanics-only example and the fake-rule-pack example are separate files, so that a solved model is never seen as a checked one.
- *One file for teaching and testing.* The project's tests run on the published examples themselves, not on copies.
- *Cite the pack, don't copy it.* The fake-rule-pack example refers to DEL-06-05's pack by identity, version and checksum and embeds none of its content.

**How each criterion is examined.**

- **VER-001** — Executable check (observes each example file against the model document format and the document validator, and the product's open, save and reopen of it): the families present, the validation result, the findings, and the content and model hash before and after the round trip.
- **VER-002** — Executable check (observes the release's solve of each example): the status, the diagnostics and every result value the example or its description states, against the example's description.
- **VER-003** — Executable check (observes the fake-rule-pack example's citation, the invented rule pack's pack checksum recomputed from its content, and a rule-check run of the pack on the example): the cited identity, version and checksum against the pack's own, the outcome of each check with its named inputs, and the mechanics status before and after.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the examples): read every example and its description against PRD §20.1 and §21.2 and the boundary policy of DEL-01-02, and report each value, name, formula or threshold that is not plainly invented or that resembles code content, each data set without complete invented provenance, and each statement that can be read as a prohibited claim.
- **VER-005** — Executable check (observes each example's strings, quantities and description, a proxy for the reviewer's reading): the four parts of the notice are present; no approval status and no listed protected or private marker occurs; every quantity carries its unit as the model document defines it; and every value left out is named in the description.

## Governing Values and Decisions — Axiology

- **AX-001** — *Unmistakably invented.* Better is fewer names, values and thresholds a reader could take for engineering data, which is to be none. Realism never justifies a value, a formula shape or a name that resembles code content.
- **AX-002** — *Small enough to audit.* Better is an example a reader can check by reading and by hand, measured by the share of its stated results reproduced in closed form. An example is never made simpler by leaving out a unit, a provenance record or the notice.
- **AX-003** — *The same as a user's model.* Better is fewer ways an example is handled differently from a model a user saves, which is to be none. Convenience for the tests never puts an example in a form the product does not open.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 | REQ-003 REQ-004 REQ-009 | AC-001 | VER-001 | The two families are AC-001's, from SOW-033 and the decomposition row. The check reads the example set against them |
| OUT-001 | SOW-033 OBJ-008 | REQ-008, DEL-11-04:1 | AC-002 | VER-001 | Outside source: the model document format and validator are DEL-02-01's and the hash is SHA-256 over the RFC 8785 canonical form; none is written by the examples |
| OUT-001 | SOW-033 OBJ-008 | REQ-005 REQ-006 REQ-008 | AC-003 | VER-002 | The example's description says what must happen; the comparison is with the release's solve. Agreement shows that the description is consistent with the product, not that its values are mechanically correct; the solver's correctness is examined by DEL-09-01's benchmarks |
| OUT-001 | SOW-033 OBJ-001 | REQ-004 REQ-010 | AC-004 | VER-003 | The example's description says what must happen; the pack checksum is DEL-06-04's definition, recomputed from the pack's content, and the outcomes are those DEL-06-03's run defines, neither written by the example |
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-009 | AC-005 | VER-004 | PRD §20.1 and FR-RULE-007 say what a public example may contain and DEL-01-02's record gives the provenance fields. The reviewer reads every example against them |
| OUT-001 | SOW-033 OBJ-001 | REQ-002 REQ-006 | AC-006 | VER-004 VER-005 | PRD §21.2 bars any claim that public example data is suitable for project use. The check finds the notice's parts and the reviewer reads every statement against §21.2 |
| OUT-001 | SOW-033 OBJ-001 | REQ-003 REQ-005 | AC-007 | VER-005 | PRD §5.5 and §5.6 require explicit units and no silent default. The check reads each quantity against the model document's definition and each omission against the description |
