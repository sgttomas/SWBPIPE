---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-04
package_id: PKG-01
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-034, SOW-064]
package_objective_refs: [OBJ-011, OBJ-018]
---

# Scope of Work — DEL-01-04 Professional responsibility and product-claims policy

## Purpose and Objective Traceability

This deliverable fixes the words the product uses about its own results. SWBPIPE has a full analytical engine, and its boundary is claim authority, not solver capability: the software may say that mechanics were solved and a user's rule was evaluated, and it may never say or imply that a design is approved, certified, sealed or code compliant. This deliverable supplies the governed vocabulary that keeps every screen, report, export, document and agent message on the right side of that line, and the notice every report carries.

It serves SOW-034 and SOW-064 under OBJ-011 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-01, row DEL-01-04: policy wording and product-claims boundaries, no code), and PRD §4.2, §4.3, §5.1, §5.9, §19.3, §19.4 and §21. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches the release target through criterion 4, whose report carries the §19.3 notice kept here, through criterion 13, whose documentation states that SWBPIPE does not determine code compliance, and through criterion 17, where the embedded agent records no acceptance, approval or reliance.

- **OUT-001** — The claims registry, `docs/claims_registry.md`: the registered boundary statements, the display forms of the status and evidence tokens, and the rules for where each may appear.
- **OUT-002** — The report notice template, `docs/report_notice_template.md`: the required report notice, the notice for authoring, comparison, handoff and export material, the disclosure slots a report fills, and the optional human acceptance block.

## Deliverable Definition — Ontology

**Terms the policy implements.**

- *Permitted claim* and *prohibited claim.* The classes of PRD §21.1 and §21.2. The software may claim mechanics results computed from recorded inputs, user-rule checks computed from user-supplied rules and data, diagnostics, provenance and validation evidence for software behaviour. It may not claim or imply that it certifies, seals, approves or authenticates engineering work, declares code compliance for professional reliance, replaces the engineer of record or project-specific review, provides standards-body approval, or makes public example data suitable for project use.
- *Authority domain.* Who a statement speaks for: the mechanics solve, the user-rule check, or human acceptance (`docs/SPEC.md` §4.3). A fourth marking, evidence status, says what evidence stands behind a result.
- *Boundary statement.* A registered sentence with an identifier, one canonical text, any listed short variants, and the surfaces it belongs on and must not appear on. The statements in use are the content boundary (`BS-IP`) and the validation posture (`BS-VALID`). The acceptance sentence (`BS-ACCEPT`) is withdrawn from every product surface, emitted notice and project document, and the maturity sentence (`BS-MATURITY`) from every surface; nothing replaces either.
- *Display form.* The registered label of a status or evidence token. A label is shown with its authority domain, with its token reachable in place, and no label exists outside the registry's table.
- *Evidence-status label.* `INTERNALLY_VERIFIED` for internal benchmark, regression or rule-check evidence; `PROVER_CORRELATED` for a recorded comparison with an external prover. `ENGINEER_ACCEPTED` is reserved for a future human acceptance record and has no display form.
- *Required notice.* The notice of PRD §19.3, carried by every generated report.
- *Human acceptance record.* A record a human makes outside the solver, rule-pack, comparison and report outputs, bound to the model, rule pack and report it accepts. The product has no acceptance workflow.
- *Design-engine framing.* SWBPIPE described as an analysis-grade piping design engine and stress-model authoring environment whose results are decision support. Acceptance and professional judgment stay with the responsible engineer.

**Boundary.** PRD §21 states the professional boundary; this deliverable keeps no separate policy document that restates it. It supplies the words. The following use them and belong elsewhere:

- the status tokens and their schema: DEL-02-03;
- the desktop's table of status labels: DEL-05-04;
- emitting the notice in a report: DEL-08-01;
- linting a report for prohibited claims: DEL-08-05;
- the user guide's statement that SWBPIPE does not determine code compliance: DEL-11-01;
- what a public release must disclose: DEL-01-01;
- the meaning of an outcome whose wording another deliverable registers here, such as the numerical-health forms: that deliverable (DEL-07-05 for numerical health).

**Not decided.**

- Jurisdiction-specific professional-practice wording. It needs a qualified legal or professional reviewer.
- How a human acceptance record would be stored and made stale by a change, should the product ever keep one.
- Whether the display form of `USER_RULE_FAILED` changes to follow the owner's ruling that user-rule outcomes are shown as within or exceeding the user's limit.
- Whether the acceptance sentence and the governance fence token keep any placement in scopes of work.
- Which deliverable examines product surfaces for retired and prohibited wording.

## Completion and Reliance Basis — Epistemology

The policy is wording. It is adequate when an author of any product surface can find the registered words for what they need to say, and when nothing registered here can be read as approval, certification or code compliance.

### Requirements

- **REQ-001** — The required notice states that the software does not certify, seal, approve, authenticate or determine code compliance for professional reliance, and no registered statement or display form says or implies otherwise (SOW-034; PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-002** — The vocabulary keeps a mechanics solve, a user-rule check and human professional acceptance distinct: each automatic status has one display form in one authority domain, and human acceptance is never an automatic status (PRD §5.9; `docs/TYPES.md` §4; `docs/CONTRACT.md` OPS-K-MECH-2).
- **REQ-003** — The required notice is the PRD §19.3 text. It says that code-specific and project-specific data are supplied by the user and that competent human review remains the user's and the project authority's responsibility. The template is the notice's home (PRD §19.3; `docs/CONTRACT.md` OPS-K-REPORT-1).
- **REQ-004** — A claim the product or a release makes about validation carries its evidence status and its limitations. The vocabulary supplies those words for the release disclosures DEL-01-01 requires (`docs/CONTRACT.md` OPS-K-GOV-3).
- **REQ-005** — A human acceptance record is separate from solver, rule-pack, comparison and report outputs. Where one is referenced, it binds to the specific model, rule pack and report by hash and does not survive a change to any of them without re-review. Nothing populates it automatically (PRD §21.3; `docs/CONTRACT.md` OPS-K-AUTH-2).
- **REQ-006** — The vocabulary gives generated output and agent output no acceptance standing: nothing a generator or an agent produces is labelled accepted, approved or relied on (PRD §5.3, §5.9; OBJ-018).
- **REQ-007** — A legal interpretation or a jurisdiction-specific professional obligation is stated as undecided until a qualified reviewer supplies it. The policy states no legal conclusion (`docs/CONTRACT.md` OPS-K-AGENT-1).
- **REQ-008** — Public notices, templates and examples reproduce no protected standards content, private rule-pack content, private project data or proprietary engineering data (PRD §19.4; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-REPORT-2).
- **REQ-010** — Product claims frame SWBPIPE as an analysis-grade piping design engine and stress-model authoring environment only with acceptance and professional judgment left to the responsible engineer. Design-authoring records, comparison outputs, handoff packages, export metadata and external-prover references are presented as review aids, not as validation or acceptance (SOW-064; OBJ-018; PRD §4.3, §5.1).
- **REQ-011** — The registry is the single source of boundary language. A surface that needs boundary language uses a registered statement or a listed short variant word for word, one boundary per surface, and composes no prohibition list of its own (DEC-081).
- **REQ-012** — The registry records each withdrawal with what it covers: the acceptance sentence on product surfaces, emitted notices and project documents (DEC-100, DEC-107), and the maturity sentence on every surface (DEC-105). A withdrawn statement has no replacement.
- **REQ-013** — The registry holds the display-form table for the automatic statuses and the emitted evidence-status labels, with its three rules (DEC-102).

### Acceptance criteria

- **AC-001** — The display-form table has exactly one row for each automatic status of `docs/TYPES.md` §4 and each emitted evidence-status label, each with one label and one authority domain; and it has no row for human approval, engineer acceptance, code compliance, certification, sealing or approval.
- **AC-002** — No registered statement, display form or notice text states or implies a claim PRD §21.2 prohibits. Where a prohibited term appears it is negated or named as prohibited.
- **AC-003** — The template's required notice is the PRD §19.3 notice word for word, and the template carries the second notice of REQ-010 for reports that include authoring, comparison, handoff, export or external-prover material.
- **AC-004** — The template's disclosure slots cover everything `docs/CONTRACT.md` OPS-K-REPORT-1 requires a report to disclose: model version, solver version, rule-pack checksum, source and provenance notes, warnings, assumptions and limitations.
- **AC-005** — The template's human acceptance block binds a model hash, a rule-pack checksum and a report hash, states that a change to a bound item requires re-review, and states that it is never filled from solver or rule-pack output.
- **AC-006** — Neither document contains protected standards text, tables or examples, private rule-pack content, private project data or proprietary engineering values.
- **AC-007** — Wherever either document describes the product, it describes decision support or an analysis-grade design engine with acceptance left to the responsible engineer, and nothing in either says that internal results, a comparison, a handoff package or an agent's output suffices for professional reliance.
- **AC-008** — Every registered statement has an identifier, one canonical text and its placement rules; every withdrawn statement is marked withdrawn with the surfaces the withdrawal covers and no replacement; and nothing in the registry places a withdrawn statement on a surface its withdrawal covers.
- **AC-009** — The registry holds the validation-posture statement (`BS-VALID`) and defines each evidence-status label by the evidence it stands for. The statement says that candidate designs are validated in the user's accepted professional tools, and that internal benchmarks and rule checks are development verification and screening evidence.

## Production and Verification Method — Praxeology

**Method choices that define the policy.**

- *A registry, not a policy essay.* The professional boundary is stated once, in PRD §21. This deliverable registers the words that carry it; it writes no second statement of the boundary.
- *Entries a test can read.* Each statement is an entry under its own identifier with its canonical text, short variants and placement rules, and the display forms are one table, so that product code and tests can read them as a contract input.
- *Label the result, not the product.* Evidence status attaches to an individual result, report or case. The product carries no global hedge.
- *The notice is quoted, not paraphrased.* The template holds the PRD §19.3 text exactly; a report generator may format it and may not reword it.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the table's rows): the set of tokens in the display-form table equals the automatic statuses of `docs/TYPES.md` §4 together with the two emitted evidence-status labels; each has one label and one authority domain; and no row exists for the excluded statuses.
- **VER-002** — Comparison with a named reference (the notice text of PRD §19.3): compare the template's required notice with it, ignoring line breaks.
- **VER-003** — Judgment by a named role (an independent reviewer who did not write the text): read both documents against PRD §4.2, §4.3, §5.1, §5.9, §19.4, §21.1 to §21.3 and §22.5, `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AUTH-2, OPS-K-GOV-3 and OPS-K-REPORT-1, and the registry rulings (DEC-081, DEC-100, DEC-102, DEC-105, DEC-107); report each statement that can be read as a prohibited claim, each missing disclosure slot or binding, a missing or weakened validation-posture statement or evidence-status label, each withdrawn statement still placed, and any protected or private content.

## Governing Values and Decisions — Axiology

- **AX-001** — *Precision over reach.* Better is wording that says exactly what the software computed or recorded, measured by the number of registered statements and labels a reader could take as acceptance, approval or compliance, which is to be zero. An appealing or shorter claim is never bought with a word that implies reliance.
- **AX-002** — *Fewer words, one vocabulary.* Better is fewer distinct boundary wordings across product surfaces, each surface carrying at most one registered statement. This is never bought by removing or weakening the required report notice or the prohibited claims of PRD §21.2.
- **AX-003** — *Separate the authorities.* Where a label could blur a solver result, a user-rule outcome and a human decision, the form that keeps them apart is the better one, even where it costs screen space.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-034 OBJ-011 | REQ-002 REQ-013 | AC-001 | VER-001 | Owner ruling: DEC-102 gives the labels and their rules. The token set is read from `docs/TYPES.md` §4, which is DEL-02-03's vocabulary, not from the registry |
| OUT-001 | SOW-034 OBJ-011 | REQ-001 REQ-006 REQ-007 | AC-002 | VER-003 | Owner ruling: PRD §21.2 lists the prohibited claims; the reviewer compares the wording with it |
| OUT-002 | SOW-034 OBJ-011 | REQ-003 REQ-010 | AC-003 | VER-002 | Owner ruling: the notice text of PRD §19.3, read from the PRD and not from the template |
| OUT-002 | SOW-034 OBJ-011 | REQ-003 | AC-004 | VER-003 | Owner ruling: `docs/CONTRACT.md` OPS-K-REPORT-1 lists the disclosures |
| OUT-002 | SOW-034 OBJ-011 | REQ-005 | AC-005 | VER-003 | Owner ruling: PRD §21.3 and `docs/CONTRACT.md` OPS-K-AUTH-2 |
| OUT-002 | SOW-034 OBJ-018 | REQ-008 | AC-006 | VER-003 | Owner ruling: PRD §19.4 lists what a public template may not contain |
| OUT-001 | SOW-064 OBJ-018 | REQ-010 | AC-007 | VER-003 | Owner ruling: PRD §4.2, §4.3 and §5.1 |
| OUT-001 | SOW-034 OBJ-018 | REQ-011 REQ-012 | AC-008 | VER-003 | Owner ruling: the registry rulings DEC-081, DEC-100, DEC-105 and DEC-107, read from the decision register |
| OUT-001 | SOW-034 OBJ-011 | REQ-004 | AC-009 | VER-003 | Owner ruling: PRD §22.5 gives the validation posture and `docs/CONTRACT.md` OPS-K-GOV-3 requires a release to disclose its validation status; the statement and the labels are those of DEC-081, read from the decision register |
