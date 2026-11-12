---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-01
package_id: PKG-11
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-011]
---

# Scope of Work — DEL-11-01 User guide skeleton

## Purpose and Objective Traceability

An engineer who has the application and this guide can install it, build a piping model, run every analysis the release requires, read the results and produce a report, and knows at each step what the software computed and what it leaves to them. The guide tells the user nothing the product does not do, shows nothing the public project may not publish, and never lets a solved model or an evaluated rule read as approval or code compliance.

It serves SOW-033 under OBJ-001 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-11, row DEL-11-01: documentation only), and PRD §5.5, §5.9, §20.1, §21 and the documentation-completeness measure of §25.2. In `docs/RELEASE_TARGET_v1.0.md` it owns exit criterion 13 together with DEL-11-03: the user guide and the theory notes cover every analysis of criterion 1, with its inputs, method, assumptions and limitations, and state that SWBPIPE does not determine code compliance. The guide's share is what the user enters, does and reads, and the limitations the user meets. The method and its assumptions are the theory notes' share.

- **OUT-001** — The user guide, `docs/user_guide/`.

## Deliverable Definition — Ontology

**Terms the guide implements.**

- *Release described.* The guide accompanies one release of the desktop application and describes that release. For v1.0 that is the macOS build.
- *Required analysis.* Each analysis of exit criterion 1: sustained, operating, expansion with its basis, hydrotest, wind, three-axis static seismic with SRSS and absolute-sum combination, seismic anchor movement, modal, response spectrum and hanger sizing.
- *Analysis entry.* The guide's account of one required analysis, in four parts: the inputs the user enters, where they are entered and in what units; how the analysis is run; the results it publishes and how to read them; and its limitations as the user meets them, which are its named refusals and what the release does not offer. Each entry links to the theory note that gives the method and its assumptions.
- *Guide sections.* Product scope and authority boundary; installation and project storage; creating a project; building the centerline model; solving; running user rule checks; reviewing results; reports; limitations; troubleshooting and warnings; glossary and status vocabulary.
- *Three authorities.* The mechanics solve, the user-rule check and human review. The guide keeps them apart wherever it describes a result.
- *Product vocabulary.* The analysis-status tokens of `docs/TYPES.md` §4 with their registered display forms, the warning classes, and the diagnostic codes the product emits. The guide quotes them and coins none.

**Boundary.** The guide is the user's document. The following belong elsewhere:

- the method, the assumptions and the method's limitations for each analysis: DEL-11-03;
- the invented example models a guide walkthrough may use: DEL-11-04, and the invented rule pack: DEL-06-05;
- documentation for developers and for contributors: DEL-11-02 and DEL-11-05;
- the registered boundary wording and the status labels: DEL-01-04;
- the release limitations list, `docs/RELEASE_LIMITATIONS_v1.0.md`: the deliverable that owns each refusal;
- the validation manual and the reproduction guide: DEL-09-04;
- the behaviour the guide describes: the deliverable that owns it. Where the guide and the product differ, the guide is the one to correct.

**Not decided.** A guide for a platform other than macOS. Windows is taken up after v1.0.

## Completion and Reliance Basis — Epistemology

The guide is adequate when a user can do what it says, what it says is true of the release it accompanies, and no passage can be read as approval, certification or code compliance.

### Requirements

REQ-001 to REQ-010 keep the numbers of the earlier `UG-REQ` series.

- **REQ-001** — The guide has each of the guide sections (SOW-033; decomposition row DEL-11-01).
- **REQ-002** — Installation and storage content gives only the requirements, steps, file names and storage locations that the release has. It invents no setup command and no product behaviour, and it says so where a platform or a packaging choice is undecided (PRD §5.5; `docs/CONTRACT.md` OPS-K-AGENT-1).
- **REQ-003** — Modelling content presents the global model as a unit-aware 3D centerline/frame model with explicit nodes, elements, components, supports and loads, whose values carry units and provenance (PRD §5.2, §5.6; `docs/SPEC.md` §3 to §5).
- **REQ-004** — Solving content distinguishes the mechanics-solve status from the user-rule-check status and from human professional acceptance, and describes no automatic status of approval or code compliance (`docs/TYPES.md` §4; `docs/CONTRACT.md` OPS-K-MECH-2, OPS-K-AUTH-1).
- **REQ-005** — Rule-check content states that code-specific values, allowables, combinations, formulas, stress-intensification and flexibility factors, owner requirements and proprietary data are supplied by the user through lawful private data or rule packs (PRD §5.4; `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-RULE-1, OPS-K-RULE-3).
- **REQ-006** — Report content tells the reader where a report shows the software and solver version, the model hash, the input manifest, the units, the warnings, the assumptions, the source and provenance notes, the rule-pack checksum, the results and the limitations (PRD §19.2; `docs/CONTRACT.md` OPS-K-REPORT-1). It also tells the reader that a project remembers the report packages saved from it, and that a report saved as HTML alone or printed is not listed with the project (DEL-08-01/REQ-023).
- **REQ-007** — Limitation content covers professional responsibility, validation status, the boundary between public and private data, the handoff to local FEA, the behaviour on missing data, and what the release does not offer. It agrees with the release limitations list (PRD §21; exit criterion 3).
- **REQ-008** — The guide reproduces no protected standards text, table, figure or example, no protected dimensional data, no proprietary commercial data and no private user or project data. Every example value is invented (PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3).
- **REQ-009** — Where the guide refers to a status, a warning class or a diagnostic, it uses the product vocabulary exactly (`docs/TYPES.md` §4 to §7).
- **REQ-010** — Where something the guide must describe is undecided or not offered, the guide says so in the place it affects. It fills no gap with a guessed value, default or behaviour, and it tells the user that the product reports missing data and never defaults it (PRD §5.5; `docs/CONTRACT.md` OPS-K-DATA-2, OPS-K-AGENT-2).
- **REQ-011** — The guide has an analysis entry for every required analysis (exit criterion 13).
- **REQ-012** — The guide states, in the release target's words, that SWBPIPE does not determine code compliance, and that establishing it is the user's act, outside this software (exit criterion 13; PRD §24, V1).
- **REQ-013** — An instruction names each control by its visible label and gives the keyboard route where the desktop has one. The desktop's accessibility baseline is DEL-07-06's.
- **REQ-014** — The string "CAEPIPE" appears nowhere in the guide. The guide describes the export only as the export of a `.mbf` model batch file.

### Acceptance criteria

- **AC-001** — Every guide section is present under a heading a reader can find from the guide's table of contents, and every link in the guide resolves.
- **AC-002** — For each of the ten required analyses there is an analysis entry with all four parts, and its link reaches the part of a theory note that covers that analysis.
- **AC-003** — A reader who follows the guide on the release it accompanies, from installation through each required analysis on the invented reference system, obtains what each instruction says they will. Every command, menu path, file name, default and code the guide names exists in that release, nothing the guide presents as available is refused, and each thing the release does not offer is stated as not offered.
- **AC-004** — The scope and authority section states that SWBPIPE does not determine code compliance. No passage states or implies a claim PRD §21.2 prohibits, and wherever a result is described the reader can tell which of the three authorities it belongs to.
- **AC-005** — Every status token, display label, warning class and diagnostic code in the guide is one the product has, spelled as the product spells it.
- **AC-006** — The guide contains nothing of the classes PRD §20.1 keeps out of the public repository, every example value in it is invented, and the rule-check content says that code-specific and proprietary values are the user's to supply.
- **AC-007** — The report content names every item of REQ-006, and the limitation content covers every subject of REQ-007 and contradicts no entry of the release limitations list.
- **AC-008** — Each instruction names its controls by the labels the desktop shows, and each step the desktop offers from the keyboard is given with its command, menu path or shortcut.
- **AC-009** — The string "CAEPIPE" occurs nowhere in the guide, in any letter case, and every passage about the `.mbf` export describes it as the export of a model batch file and attributes the file to no vendor or program.

## Production and Verification Method — Praxeology

**Method choices that define the guide.**

- *Organised by the user's work.* The guide follows the order in which an engineer installs, models, loads, solves, reviews and reports. It is not organised by module or by deliverable.
- *Describe, never define.* The guide quotes behaviour, vocabulary and limits from the product and the owning scopes. It adds no value, default, rule or status of its own.
- *One entry per analysis, the same four parts.* A reader who has used one analysis entry knows where to look in the next.
- *Link the method.* The entry sends the reader to the theory note for how the result is computed and under what assumptions, and does not restate it.
- *Invented data only.* Every walkthrough uses the invented example models, the invented rule pack or the invented reference system.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the guide's headings, entries and links): the guide sections exist; each required analysis has an entry with the four parts; every internal link and every link to a theory note resolves; and the code-compliance statement is present in the scope and authority section.
- **VER-002** — Judgment by a named role (an independent reviewer who did not write the guide, working on the release-candidate build): follow the guide from installation through each required analysis on the invented reference system, and record each instruction that cannot be followed, each stated result the application does not give, each capability presented as available that the application refuses, each control named otherwise than by its label, and each keyboard route the desktop has and the guide omits.
- **VER-003** — Executable check (observes the tokens the guide quotes, a proxy for correct use): every status token, display label, warning class and diagnostic code in the guide is found in the product's vocabulary sources.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the guide): read the guide against PRD §19.2, §20.1 and §21.2, the boundary policy of DEL-01-02, the claims registry of DEL-01-04 and the release limitations list, and report each protected or private item, each example value that is not invented, each statement that can be read as a prohibited claim, each result whose authority is unclear, each missing report item or limitation subject, and each contradiction of the limitations list.
- **VER-005** — Executable check (observes the guide's text): the string "CAEPIPE" does not occur in any letter case, and every passage that mentions `.mbf` calls it a model batch file and names no vendor or program for it.

## Governing Values and Decisions — Axiology

- **AX-001** — *True before complete.* Better is fewer statements a user finds false on the release, measured by the walkthrough's count of instructions that fail and results that differ, which is to be zero. Coverage is never bought with a description of something the release does not do; a gap is written as a gap.
- **AX-002** — *Shorter path to a solved model.* Better is less time for a new user to go from installation to a solved, reviewed and reported model by following the guide (PRD §25.2). That time is never reduced by leaving out a limitation, a warning, a missing-data finding or the authority boundary.
- **AX-003** — *Claim less.* Where a passage could be read as acceptance, approval or compliance, the wording that claims less is the better one, even where it is longer.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 | REQ-001 | AC-001 | VER-001 | The section list is REQ-001's, from SOW-033 and the decomposition row. The check reads the guide against the list |
| OUT-001 | SOW-033 OBJ-001 | REQ-011 | AC-002 | VER-001 | Exit criterion 1 lists the analyses and exit criterion 13 lists what is covered for each. The list is read from the release target, not from the guide |
| OUT-001 | SOW-033 OBJ-001 | REQ-002 REQ-003 REQ-010 | AC-003 | VER-002 | The guide says what must happen at each step; the comparison is with what the application does on the invented reference system, which the guide did not write, observed by a reviewer who did not write the guide |
| OUT-001 | SOW-033 OBJ-011 | REQ-004 REQ-012 | AC-004 | VER-001 VER-004 | Exit criterion 13 requires the statement and PRD §21.2 lists the prohibited claims. The check finds the statement; the reviewer reads every passage against the §21.2 list |
| OUT-001 | SOW-033 OBJ-011 | REQ-009 | AC-005 | VER-003 | Outside source: the tokens are read from `docs/TYPES.md` §4, the claims registry and the product's diagnostic vocabulary, none of which the guide writes |
| OUT-001 | SOW-033 OBJ-001 | REQ-005 REQ-008 | AC-006 | VER-004 | PRD §20.1 lists what the public repository may not contain and DEL-01-02's policy states the classes. The reviewer reads the guide's text and examples against them |
| OUT-001 | SOW-033 OBJ-011 | REQ-006 REQ-007 | AC-007 | VER-004 | PRD §19.2 lists the report content and `docs/RELEASE_LIMITATIONS_v1.0.md` is the release limitations list of exit criterion 3. The reviewer reads the guide's report and limitation content against both |
| OUT-001 | SOW-033 OBJ-001 | REQ-013 | AC-008 | VER-002 | The labels and keyboard routes are the desktop's, read from the application by a reviewer who did not write the guide |
| OUT-001 | SOW-033 OBJ-011 | REQ-014 | AC-009 | VER-005 | The string scanned for and the required description of the `.mbf` export are fixed by REQ-014, not by the guide. The check reads the guide's text against them |
