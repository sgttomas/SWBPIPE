---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-05-04
package_id: PKG-05
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-047]
package_objective_refs: [OBJ-005, OBJ-011]
---

# Scope of Work — DEL-05-04 Analysis status semantics

## Purpose and Objective Traceability

This deliverable fixes what the product may say about how far an analysis has got. A model can be incomplete; its mechanics can be solved; a user's rule pack can lack inputs, can have been evaluated, or can have reported a result beyond the user's limit; and every result needs a competent person's review before it is relied on. These are different facts, and the deliverable gives each one status, one meaning and one authority, so that a solve, a rule check and a person's acceptance can never be read as the same thing. Acceptance by a person is kept out of the software's statuses altogether: it exists only as a separate record that a person makes, tied to exactly what that person reviewed.

It serves SOW-047 under OBJ-005 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-05, row DEL-05-04: a small state model with high governance value), and PRD §5.9, §17.5, §21.1, §21.2 and §21.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 4 (a released report carries the required notice) and criterion 13 (the documentation says that the product does not determine code compliance) through the deliverables that publish a status, all of which take its meaning from here. It carries the requirement that the user's engineer issues the product's mechanics results under their own responsibility.

- **OUT-001** — The analysis status semantics: the status vocabulary and the meaning and authority of each status, the status envelope that carries them, and the human acceptance record that stands apart from them.

## Deliverable Definition — Ontology

**The statuses.**

| Status | Meaning | Authority |
|---|---|---|
| `MODEL_INCOMPLETE` | Physical inputs the solve requires are missing. Mechanics are not solved for the affected subject. | Software finding |
| `MECHANICS_SOLVED` | The numerical mechanics were computed for one model snapshot. It says nothing about rule inputs or compliance. | Solver result |
| `RULE_INPUTS_INCOMPLETE` | Values a user's rule pack requires are missing. Mechanics may be solved all the same. | Rule-pack finding |
| `USER_RULE_CHECKED` | A user's rule pack was evaluated on the result, with the user's data and rules. | Software computation on user data |
| `USER_RULE_FAILED` | A user's rule pack reported at least one check beyond the user's limit. | Software computation on user data |
| `HUMAN_REVIEW_REQUIRED` | Professional use needs competent human review. It is a notice, not a failure. | Professional-boundary notice |
| `HUMAN_APPROVED_FOR_PROJECT` | A person recorded acceptance of specific content for a project. | Human record only |

**Further terms.**

- *Automatic status.* A status software may emit: the first six. `HUMAN_APPROVED_FOR_PROJECT` is never one.
- *Positive rule outcome.* A rule check with nothing beyond the user's limit is `USER_RULE_CHECKED` with its details. There is no passed status, and no status named for compliance, certification, sealing, approval or authentication.
- *Coexistence.* Statuses are not a sequence. One subject and snapshot carry every status that is true of them.
- *Status envelope.* The record that carries statuses: the subject, a primary automatic status and the set of automatic statuses, rule-check details where a rule was checked, the source and actor, at least one hash of the content the statuses describe, references to diagnostics, the human-review notice, and professional-boundary fields that are fixed false for compliance, certification, sealing, approval and authentication.
- *Human acceptance record.* A record apart from the envelope's statuses: a human actor, a time, references to and hashes of the model, result, rule pack and report that were reviewed, an outcome that is either acceptance or an explicit non-acceptance, and a notice of what was accepted. Software, an agent and an external system cannot be its accepting actor. It applies only to content whose hashes it binds.

**Boundary.** This deliverable defines what a status means. The following belong elsewhere:

- the analysis boundary record, which states for each status of a subject who produced it and on what evidence: DEL-02-03. The status vocabulary and its meaning are this deliverable's, and so is the envelope schema; DEL-02-03's record carries that vocabulary;
- emitting a status: the product solve (DEL-04-07), the completeness checker (DEL-06-03), the rule-pack runner (PKG-06) and the reports and exports (PKG-08) each emit the statuses of their own work. The interface part of AC-001, the solve and rule-check part of AC-002 and the report part of AC-004 are observed on the surfaces of the emitting deliverables, and a shortfall there is repaired in that deliverable;
- the numerical standing of a result, which says whether its integrity checks passed and what a reader may use it for, is not an analysis status: DEL-04-07 publishes it and DEL-04-08 defines what a reader makes of it;
- the display form and authority domain shown for each status, the evidence labels, the report notice and the words a claim may not use: DEL-01-04. The desktop's label table carries that registry for the statuses defined here. A user-rule outcome is shown as within or exceeding the user's limit, not as passed or failed (`docs/RELEASE_TARGET_v1.0.md`, rule packs), and that applies to the display form of `USER_RULE_FAILED`;
- the result envelope and its outcome kinds: DEL-00-06.

**Not decided.**

- Who stores a human acceptance record, and which surface of the product, if any, presents one with the content it was made for. PRD §21.3 requires no acceptance workflow.
- How a hash is formed for reviewed content that is not JSON.

## Completion and Reliance Basis — Epistemology

A status is a claim about what software did. The semantics are adequate when every surface that carries a status can carry only the automatic ones, each with the meaning above, and when nothing software produces can be read as a person's approval.

### Requirements

- **REQ-001** — The mechanics-solved state, the rule-pack evaluation states, the incomplete-data findings and human acceptance are distinct, each with its own status and authority (SOW-047; decomposition AB-00-03).
- **REQ-002** — `MECHANICS_SOLVED` means mechanics outputs were computed for one model snapshot. It implies neither complete rule inputs nor code compliance (PRD §5.1, §21.1; `docs/CONTRACT.md` OPS-K-MECH-2).
- **REQ-003** — `MODEL_INCOMPLETE` means missing solve-required physical input, and mechanics are not solved for the affected subject (PRD §5.5, §12.8; `docs/CONTRACT.md` OPS-K-DATA-2).
- **REQ-004** — `RULE_INPUTS_INCOMPLETE` means missing rule-pack-required user, code or project data, and it may coexist with `MECHANICS_SOLVED` (PRD §17.5).
- **REQ-005** — `USER_RULE_CHECKED` and `USER_RULE_FAILED` describe the outcome of a user's rule pack on the user's data. Neither declares professional code compliance (PRD §21.1, §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-006** — `HUMAN_REVIEW_REQUIRED` is carried by every reportable result (PRD §19.3; `docs/CONTRACT.md` OPS-K-REPORT-1).
- **REQ-007** — Software never emits `HUMAN_APPROVED_FOR_PROJECT`. That value exists only in a human acceptance record (PRD §21.2, §21.3; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-008** — A human acceptance record binds to the hashes of the specific model, result, rule pack, report and manifest reviewed, and does not apply to content whose hash has changed (`docs/CONTRACT.md` OPS-K-AUTH-2).
- **REQ-009** — A status envelope keeps the references to the diagnostics, rule-pack identity, warnings, assumptions and limitations that a report needs to audit the status (decomposition AB-00-06; `docs/CONTRACT.md` OPS-K-REPORT-1).
- **REQ-010** — Mechanics solved, rule checked, rule inputs incomplete, human review required and human approved cannot collapse into one flag: no surface that carries a status has a single compliance or approval value (`docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-MECH-2).
- **REQ-011** — The schema, its examples and the deliverable's documents hold no protected standards text, tables or formulas, no proprietary or private rule values and no approval claim (`docs/CONTRACT.md` OPS-K-IP-1; PRD §20.1).
- **REQ-012** — An undecided part of human acceptance (who stores a record, where it is shown, how non-JSON content is hashed) is stated as undecided, and nothing is invented for it (`docs/CONTRACT.md` OPS-K-AGENT-1).
- **REQ-013** — A status envelope identifies at least the software status, the diagnostic references, the human-review notice and, where one exists, a reference to a human acceptance record, without implying approval (decomposition AB-00-03, AB-00-06).
- **REQ-014** — No product surface gives automatic approval or an automatic code-compliance claim, in a status or in the wording around one, and a human acceptance record is never reused for content that changed after it was made (`docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AUTH-2).

### Acceptance criteria

- **AC-001** — The status schema admits exactly the six automatic statuses as software output. Every schema and interface that carries an analysis status (results, analysis run, headless runner, the API contract) admits only those six, and each refuses `HUMAN_APPROVED_FOR_PROJECT`, a passed status and any compliance, certification, sealing, approval or authentication token as a software status.
- **AC-002** — A model that lacks a solve-required input is reported `MODEL_INCOMPLETE` and publishes no mechanics result for the affected subject. A solved model whose rule pack lacks an input carries `MECHANICS_SOLVED` and `RULE_INPUTS_INCOMPLETE` together, keeps its mechanics result and reports no rule outcome. The status schema refuses an envelope whose status set holds `MODEL_INCOMPLETE` with `MECHANICS_SOLVED` for the same subject and snapshot, or `RULE_INPUTS_INCOMPLETE` with a user-rule outcome, and admits `MECHANICS_SOLVED` with `RULE_INPUTS_INCOMPLETE`.
- **AC-003** — An envelope with `USER_RULE_CHECKED` validates only with its rule-check details, and one with `USER_RULE_FAILED` names the rule pack, its version and checksum and the checks beyond the limit. Neither validates with a compliance claim.
- **AC-004** — Every status envelope carries the human-review notice and the five professional-boundary fields as false, and validates with neither absent nor true. Every report and export of a result carries `HUMAN_REVIEW_REQUIRED`.
- **AC-005** — A human acceptance record validates only with a human actor, a time, the hashes of the reviewed content, an outcome and a scope notice; a record naming software, an agent or an external system as accepting actor is refused; and a record of non-acceptance carries no `HUMAN_APPROVED_FOR_PROJECT`. Where the product presents a human acceptance record with content, a record whose bound hash differs from that content's hash is presented as not applying to it.
- **AC-006** — A status envelope validates only with its subject, statuses, source, actor and at least one hash, and its diagnostic and rule-pack references reach a report unchanged.
- **AC-007** — The schema's examples, the semantics document and the status wording on the desktop, in reports and in the runner's output contain no protected or private content and no word that states or implies approval or compliance. Where such a term appears, it is negated, named as prohibited, or names the human acceptance record. Each undecided part of human acceptance appears as one statement of what is undecided.

## Production and Verification Method — Praxeology

**Method choices that define the semantics.**

- The vocabulary is a closed set in one schema. A surface that carries a status takes the automatic set from it and adds no status of its own.
- Human acceptance is a separate record, not a status value, so that it can bind to hashes and cannot be produced by a solve, a rule check, an import or a report.
- Statuses are a set per subject and snapshot, not a state machine.
- The professional boundary is carried as explicit false fields, so that a reader does not have to infer it from the absence of a claim.

**How each criterion is examined.**

- **VER-001** — Executable check (observes what each schema admits and refuses, not what a solve emits): validates envelopes and acceptance records that should pass and fail against the status schema, and reads the status enumerations of the results, analysis-run, headless-runner and API schemas, for AC-001 and AC-003 to AC-006.
- **VER-002** — Executable check: observes the statuses and results a solve and a rule check publish for an incomplete model and for a solved model with a rule input missing, the status carried by a generated report and an export, and what the status schema refuses and admits among the status sets of AC-002.
- **VER-003** — Executable check: on a surface that presents a human acceptance record, presents one with content whose hash differs from the bound hash and observes that it is presented as not applying.
- **VER-004** — Judgment by a named role (an independent reviewer who did not write the text): reads the schema's examples, the semantics document and the status wording of the desktop, reports and runner against PRD §21.1 to §21.3, `docs/claims_registry.md` and PRD §20.1.

## Governing Values and Decisions — Axiology

- **AX-001** — *Computation is not judgment.* Where a status or its wording could be read as a person's approval or as compliance, the reading that claims less is the better one. Clearer or friendlier wording is never bought with a word that implies approval.
- **AX-002** — *One meaning everywhere.* Better is fewer places that define a status; the target is one, with every other surface reading it. Convenience for one surface never justifies a status of its own or a merged flag.
- **AX-003** — *Missing data is a result.* Better is a missing input reported under its own status. A status is never advanced by assuming a value or skipping a check.
- **AX-004** — *Acceptance belongs to a person and to exact content.* Better is a tighter binding between an acceptance record and what was reviewed. Ease of reuse never justifies applying a record to changed content.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-047 OBJ-011 | REQ-001 REQ-007 REQ-010 | AC-001 | VER-001 | PRD §21.2 and `docs/CONTRACT.md` OPS-K-AUTH-1 say what software may not claim; the check compares each schema with that list |
| OUT-001 | SOW-047 OBJ-005 | REQ-002 REQ-003 REQ-004 | AC-002 | VER-002 | PRD §17.5 and §5.5 say what a solve and a rule check do when data is missing; the scenario states the expected statuses |
| OUT-001 | SOW-047 OBJ-005 | REQ-005 | AC-003 | VER-001 | PRD §21.1 and §21.2; the criterion states what validates |
| OUT-001 | SOW-047 OBJ-011 | REQ-006 REQ-013 | AC-004 | VER-001 VER-002 | PRD §19.3 and §21.2; the criterion states the required fields |
| OUT-001 | SOW-047 OBJ-011 | REQ-008 REQ-014 | AC-005 | VER-001 VER-003 | `docs/CONTRACT.md` OPS-K-AUTH-2 and PRD §21.3, both conditional on a record being used; the changed hash is computed from the content presented, not written by the record. While no surface presents a record, only the schema's hash binding is examined, and nothing yet establishes that a record is not applied to changed content |
| OUT-001 | SOW-047 OBJ-005 OBJ-011 | REQ-009 REQ-013 | AC-006 | VER-001 | The criterion states the required fields; decomposition AB-00-06 lists what a diagnostic reference carries |
| OUT-001 | SOW-047 OBJ-011 | REQ-011 REQ-012 REQ-014 | AC-007 | VER-004 | PRD §20.1, §21.1 and §21.2 and the claims registry; the reviewer compares the texts with them |
