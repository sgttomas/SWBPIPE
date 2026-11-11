---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-04
package_id: PKG-09
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-027]
package_objective_refs: [OBJ-008, OBJ-011]
---

# Scope of Work — DEL-09-04 Validation manual skeleton

## Purpose and Objective Traceability

This deliverable is the validation manual: the one place where a maintainer, a user or a reviewer can see what SWBPIPE's software evidence is, what each piece of it establishes, and how to reproduce it. The manual keeps four questions apart that are easily run together: whether the software solves a declared mechanics problem correctly, whether its workflow is fit for its support role, whether a user's rule pack was computed as written, and whether a real calculation may be relied on. Only a competent person answers the last. The manual also carries the procedure by which anyone, starting from a clean checkout, reruns the validation examples and records what happened.

It serves SOW-027 under OBJ-008 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-09, row DEL-09-04), PRD §22.1 and §22.5, the validation manual of PRD §24 R6 and its reproduction exit criterion. In `docs/RELEASE_TARGET_v1.0.md`, exit criterion 12 names this deliverable: a fresh reproduction bundle exists at the release candidate.

- **OUT-001** — The validation manual: its ten-section structure, its case pages, and the wording that separates mechanics verification, workflow validation, user rule checks and professional reliance.
- **OUT-002** — The reproduction procedure and the bundle that records each run of it.

## Deliverable Definition — Ontology

**Terms the manual implements.**

- *Evidence class.* Each section and page states which of four it holds. *Mechanics verification*: comparison of the software with a declared mechanics problem at a declared tolerance. *Workflow validation*: evidence that the workflow keeps assumptions, warnings, reproducibility and limitations in view and is fit for its support role. *User rule check*: a deterministic computation under a rule pack and data the user supplied. *Professional reliance context*: what a competent person needs in order to judge a real calculation; the judgment itself is outside the software.
- *The ten sections.* Product scope and limitations; solver theory summary; unit and schema verification; element verification; load and stress recovery verification; nonlinear support verification; rule-pack evaluator verification; GUI workflow validation; report reproducibility validation; known limitations and open issues (`docs/VALIDATION_STRATEGY.md` §3).
- *Case page.* The record of one validated case: its purpose, input model, hand calculation or independent reference, expected result, software result, tolerance, pass or fail, and solver version. A page is generated from the suite's own sources of record and is never edited by hand.
- *Evidence state.* The state a section or page declares for its evidence: planned, draft, reviewed by the maintainer, blocked, or undecided.
- *Validation posture.* Product-level validation is correlation with an external professional solver; the internal benchmark and regression suites are development verification (PRD §22.1, §22.5). The manual states this and does not present internal results as independent validation.
- *Reproduction procedure.* The documented steps by which a maintainer or an agent, from a clean checkout at one pinned commit, builds the tools and runs the validation examples, with the exit and output each step is expected to give.
- *Reproduction bundle.* The immutable record of one run of the procedure, at `validation/evidence/reproduction/<run-id>/`: the source commit, the hash of the procedure followed, the environment and tool versions, every command with its output, exit code and output hash, the state of the working tree before and after, and checksums over the bundle.

**Boundary.** The manual organizes and presents evidence; it produces none of the results it shows.

- Benchmark and regression cases and their references are DEL-09-01's, DEL-09-02's and DEL-09-03's. Evidence of the rule-pack evaluator, the GUI workflow and report reproducibility comes from the deliverables that own those products; the manual gives each a section.
- The boundary words the manual uses are the registered statements of DEL-01-04's claims registry. The headless runner the procedure drives is DEL-10-05's. The desktop panel that displays manual evidence is DEL-07-12's.
- The release gates that read this evidence are DEL-09-05's. The user guide and theory notes are DEL-11-01's and DEL-11-03's. Records of correlation with an external prover are DEL-17-05's.
- The manual is not an engineering seal, a code-compliance ruling or a project acceptance record.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — The manual separates mechanics verification from workflow validation (SOW-027; `docs/VALIDATION_STRATEGY.md` §1).
- **REQ-002** — Mechanics verification sections describe comparison of the software with declared mechanics problems at declared tolerances, never code compliance (SOW-027).
- **REQ-003** — Validation sections evaluate fitness of the workflow for its support role and leave project-specific judgment with the professional; the manual states the validation posture (OBJ-011; PRD §22.1, §22.5).
- **REQ-004** — Rule-pack sections treat a pass or fail as the result of the user's rule computation, not professional authentication, and no automatic code-compliant status appears (PRD §21.2).
- **REQ-005** — The manual has the ten sections, each stating its evidence class.
- **REQ-006** — Benchmark and validation examples the manual references are original, public-domain or permissively licensed; protected and proprietary examples are excluded (`docs/IP_AND_DATA_BOUNDARY.md`).
- **REQ-007** — Missing evidence, open risks, limitations and undecided items are stated where they apply. No gap is filled silently, and a tolerance that is not decided is shown as not decided (`docs/CONTRACT.md` OPS-K-AGENT-1, OPS-K-AGENT-2).
- **REQ-008** — Wherever the manual speaks of a release, it describes software maturity and validation evidence only, never engineering approval (`docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-009** — Unit, schema, diagnostic and result-envelope checks have their place in the sections they bear on (`docs/CONTRACT.md` OPS-K-UNIT-1; decomposition AB-00-06).
- **REQ-010** — The manual keeps the public and private data boundary: rule packs, materials, component data, owner requirements and project models are described as supplied by the user or a lawful private source, and none is reproduced (`docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3, OPS-K-DATA-1 to OPS-K-DATA-3).
- **REQ-011** — Every case of the mechanics, stress recovery and nonlinear suites has a case page with the eight fields, generated from the suite's sources of record (SOW-027).
- **REQ-012** — Every section and case page declares its evidence state. Every case page reaches the maintainer-reviewed state, each on the maintainer's recorded review; a page is marked as reviewed by the maintainer only on that review.
- **REQ-013** — Validation examples reproduce from a clean checkout by following the documented procedure, with the environment, tool versions, commands, exit codes and output hashes recorded. The procedure can be carried out by a maintainer or by an agent (PRD §24 R6).
- **REQ-014** — Each run of the procedure is recorded as a new reproduction bundle in `validation/evidence/reproduction/`, and a completed bundle is never edited. A fresh bundle exists at the release candidate (exit criterion 12).
- **REQ-015** — Not decided: the final result-comparison values for public benchmarks, whose selection this deliverable carries (`docs/VALIDATION_STRATEGY.md` §4). Until they are selected, the tolerance field of a case page shows the governed value in force or states that it is not decided.

### Acceptance criteria

- **AC-001** — The manual contains the ten sections, each naming its evidence class and its evidence state, and unit, schema, diagnostic and result-envelope checks each appear in the sections they bear on.
- **AC-002** — No text of the manual presents mechanics verification, a rule-pack result, a passing benchmark or a release as code compliance, certification, approval or sufficiency for professional reliance. Rule-pack results are described as computations under the user's rule pack, professional reliance as a competent person's decision, and the validation posture is stated.
- **AC-003** — Every case of the three suites has a case page carrying the eight fields; each page's expected result, software result, tolerance, pass or fail and solver version equal those in the suite's hand calculation, governed tolerance record and run report; each committed page equals what the generator produces from those sources; and the tolerance field names the governed tolerance in force or states that it is not decided.
- **AC-004** — Every section without evidence says so, every limitation and undecided item is listed in the known-limitations section or on the page it affects. Every case page is marked as reviewed by the maintainer, and each mark has the maintainer's recorded review; no page is marked as reviewed without one.
- **AC-005** — The manual and its pages contain no protected standards content, proprietary value or private project data, and every example they reference is original, public-domain or permissively licensed.
- **AC-006** — A maintainer or an agent, starting from a clean checkout at one pinned commit, completes every step of the procedure as written, and each command gives the exit and output the procedure states.
- **AC-007** — A reproduction bundle exists for the release candidate's commit in `validation/evidence/reproduction/`. It holds the source commit, the procedure's hash, the environment and tool versions, every command with its exit code and output hash, and the working-tree state before and after; its checksums verify; every command it records gave the exit and output the procedure states; and no earlier bundle differs from its committed content.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- Case pages are deterministic renderings: the hand calculations, case definitions, suite results and governed records remain the sources of record, and a page is regenerated when they change.
- Each section carries one evidence class, so that a reader never has to infer which question a result answers.
- The procedure is written as commands with their expected exits and outputs, so that agreement can be checked by someone who knows nothing else about the project.
- A reproduction is run on a detached clean clone at one commit, with tools built at that commit, and its bundle is named by time and commit.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-003, executable check: observes the manual's section list, evidence classes and states, compares each page's values with the suites' hand calculations, governed tolerance records and run reports, regenerates the case pages and compares them with the committed pages, and compares the set of pages with the suites' case inventories.
- **VER-002** — AC-002, AC-004 and AC-005, judgment by a reviewer who did not write the text: examines the manual's wording against `docs/claims_registry.md` and PRD §21.2, the visibility of gaps and undecided items, and the sources of referenced examples; the claims-language lint and the protected-content scan run over the manual.
- **VER-003** — AC-006, executable check carried out by a maintainer or an agent working from the procedure alone: follows the procedure from a clean checkout and compares each command's exit and output with what the procedure states.
- **VER-004** — AC-007, executable check: observes the bundle's contents against REQ-013's list, verifies its checksums and its source commit against the release candidate, compares each recorded command's exit and output with what the procedure states, and compares every earlier bundle with its committed content.

## Governing Values and Decisions — Axiology

- **AX-001** — *Narrow claims.* Better is fewer statements a reader could take for compliance, approval or independent validation; the target is none. The manual is never made more persuasive by blurring which of the four questions a result answers.
- **AX-002** — *Visible gaps.* Better is a larger share of missing evidence, limitations and undecided values that the manual names where they apply. Completeness is never implied by leaving a gap unmentioned or by a page that shows coverage that does not exist.
- **AX-003** — *Reproducible by anyone.* Better is fewer steps that an independent actor cannot complete from the procedure alone; the target is none. A reproduction is never made to pass by editing an earlier bundle, omitting a failing command or recording an exit that did not occur.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-027 OBJ-008 OBJ-011 | REQ-005 REQ-009 REQ-012 | AC-001 | VER-001 | The manual's sections are compared with the ten of `docs/VALIDATION_STRATEGY.md` §3 and with the distinctions SOW-027 requires |
| OUT-001 | SOW-027 OBJ-008 OBJ-011 | REQ-001 REQ-002 REQ-003 REQ-004 REQ-008 | AC-002 | VER-002 | PRD §21.2, §22.1 and §22.5 and `docs/claims_registry.md`; judged by a reviewer who did not write the text |
| OUT-001 | SOW-027 OBJ-008 OBJ-011 | REQ-011 REQ-015 | AC-003 | VER-001 | SOW-027's source, PRD §16.5 in the numbering the decomposition cites, lists the eight fields. Page values are compared with the hand calculations, governed tolerance records and run reports of DEL-09-01 to DEL-09-03, which the manual did not write. That pages equal what the generator regenerates is the product's own output and only guards against hand edits |
| OUT-001 | SOW-027 OBJ-008 OBJ-011 | REQ-007 REQ-012 | AC-004 | VER-002 | `docs/CONTRACT.md` OPS-K-AGENT-1 and OPS-K-AGENT-2 state that gaps stay visible; each reviewed mark is compared with the maintainer's recorded review, which the manual did not write; judged by a reviewer who did not write the text |
| OUT-001 | SOW-027 OBJ-008 OBJ-011 | REQ-006 REQ-010 | AC-005 | VER-002 | The manual is compared with `docs/IP_AND_DATA_BOUNDARY.md` by the protected-content scan and by a reviewer who did not write it |
| OUT-002 | SOW-027 OBJ-008 OBJ-011 | REQ-013 | AC-006 | VER-003 | PRD §24 R6 states what reproduction means; each command's result is compared with the exit and output the procedure states and with the suites' own outcomes |
| OUT-002 | SOW-027 OBJ-008 OBJ-011 | REQ-013 REQ-014 | AC-007 | VER-004 | REQ-013, REQ-014 and exit criterion 12 state what a bundle records and where; the bundle is compared with the release candidate's commit, its own checksums and the exits and outputs the procedure states |
