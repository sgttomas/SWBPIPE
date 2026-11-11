---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-02-03
package_id: PKG-02
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-002]
package_objective_refs: [OBJ-001, OBJ-011]
---

# Scope of Work — DEL-02-03 Code-neutral analysis boundary model

## Purpose and Objective Traceability

A reader of any SWBPIPE status can tell who is speaking. The solver says only that the mechanics were solved or that the model is incomplete. The rule evaluator says only what the user's own rule pack computed. Acceptance of the work is a person's record, made outside the software and bound to what that person reviewed. This deliverable defines those three authorities and the status vocabulary that keeps them apart, so that no result, report or interface can present a software computation as code compliance or as professional approval.

It serves SOW-002 under OBJ-001 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-02, row DEL-02-03), and PRD §5.1, §5.4, §5.5, §5.9, §17.5 and §21.1 to §21.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It carries the release target's standing statement that SWBPIPE computes mechanics and evaluates the user's rule packs and does not certify, approve or establish code compliance, and it supports criterion 13, whose documentation states the same.

- **OUT-001** — The code-neutral analysis boundary: the analysis status vocabulary, the authority model, and the boundary record that separates the mechanics solve, the user-rule check and human acceptance.

## Deliverable Definition — Ontology

**Terms the boundary implements.**

- *Analysis status vocabulary.* Seven statuses, each with one authority (`docs/TYPES.md` §4):

  | Status | Meaning | Authority |
  |---|---|---|
  | `MODEL_INCOMPLETE` | Physical data required for solving is missing | Software finding |
  | `MECHANICS_SOLVED` | The numerical mechanics solve completed | Solver result only |
  | `RULE_INPUTS_INCOMPLETE` | Mechanics may be solved, but values the rule pack requires are missing | Rule-pack finding |
  | `USER_RULE_CHECKED` | The user's rule pack evaluated the result | Software computation using user data |
  | `USER_RULE_FAILED` | The user's rule pack produced a failing result | Software computation using user data |
  | `HUMAN_REVIEW_REQUIRED` | The result needs competent engineering review before reliance | Always true for professional use |
  | `HUMAN_APPROVED_FOR_PROJECT` | A person has recorded project-specific acceptance outside the solver | Human record only |

- *Automatic status.* A status the software may emit: the first six. `HUMAN_APPROVED_FOR_PROJECT` is never one, and there is no status for code compliance, certification, sealing, approval or authentication.
- *Authority model.* Three authorities, declared on every boundary record: the mechanics solve is a solver result only; the user-rule check is a software computation using user data; human acceptance is an external record bound by hash to the evidence reviewed.
- *Boundary record.* One record for one analysis subject, with a part for each authority: the mechanics solve (status, result reference, evidence hashes); the user-rule check (status, the user's rule-pack reference, evaluation reference, evidence hashes); the missing inputs; the diagnostics; references to human acceptance records; and the professional-boundary statement, whose claim fields are fixed false. `schemas/analysis_boundary.schema.yaml` describes it.
- *Rule-pack reference.* The identity of the user's rule pack: its id, version, checksum, source notice, redistribution status and provenance. Rule content is not part of it.
- *Human acceptance record.* A project record made by a person, outside the solver and the rule evaluator, naming the model, rule-pack, result or report content it reviewed by hash. The term "approved" is used only with this qualifier.
- *Evidence for a status.* What each status must point to:

  | Status | Emitted by | Evidence |
  |---|---|---|
  | `MODEL_INCOMPLETE` | Model validation or the solve's pre-check | A solve-blocking diagnostic naming the missing physical data |
  | `MECHANICS_SOLVED` | The mechanics solver | The solver's result and its hashes |
  | `RULE_INPUTS_INCOMPLETE` | The rule-pack completeness check | The rule-pack reference and a diagnostic naming the missing input |
  | `USER_RULE_CHECKED`, `USER_RULE_FAILED` | The rule evaluator | The rule-pack reference and the evaluation |
  | `HUMAN_REVIEW_REQUIRED` | Any presenting surface | The notice that reliance needs competent review |
  | `HUMAN_APPROVED_FOR_PROJECT` | A person | The human acceptance record and the hashes it binds |

**Undecided.** Where a human acceptance record is stored, and how one is marked stale when the content it binds changes, are not decided. Whether a surface carries one status per authority, as the boundary record does, or a set of statuses for the subject, as the status envelope does, is not decided.

**Boundary.** This deliverable defines the authorities and the boundary record, and the boundary record carries the vocabulary of `docs/TYPES.md` §4. The status envelope is DEL-05-04's; each emitter (DEL-04-07, DEL-06-03, PKG-06, PKG-08) decides when its own statuses are emitted; the display form and wording of a status are DEL-01-04's. DEL-05-04 also states the vocabulary and the meaning of each status; which of the two scopes owns that statement is not settled. The solver's own status and diagnostics are DEL-04-06's and DEL-04-07's; rule evaluation is PKG-06's; reports are PKG-08's. The claims a product text may make are DEL-01-04's. It is independent of the solver library, the rule expression grammar, the API transport and the project container: it names none and changes with none. It implements no solving, no rule evaluation, no views and no acceptance workflow.

## Completion and Reliance Basis — Epistemology

A status says what a named authority established. A boundary record establishes that the three are kept apart; it establishes nothing about the mechanics, the rules or the person's judgment.

### CLM-010 — Requirements

Requirement numbers are those of the earlier `DEL-02-03-Rnn` series.

- **REQ-001** — The status vocabulary is exactly the seven statuses of the table above.
- **REQ-002** — There is no automatic code-compliance status, and no status, field or text of the boundary claims certification, sealing, approval, authentication or code compliance for reliance (PRD §21.2).
- **REQ-003** — The mechanics part of a record states that the solver's output is a mechanics result only.
- **REQ-004** — A user-rule check requires the user's own rule-pack reference. The public model bundles no proprietary or protected code content.
- **REQ-005** — A missing solve-required or rule-check-required value appears as an explicit status with a diagnostic. It is never filled by a default.
- **REQ-006** — A user-rule outcome is distinct from human acceptance: no rule outcome sets, implies or stands in for a human acceptance record.
- **REQ-007** — A human acceptance record is outside the solver's and the rule evaluator's authority, is bound to the specific model, rule-pack, result or report content reviewed, and does not survive a change to that content without re-review.
- **REQ-008** — Every status carries its authority and a reference to the evidence that justifies it, and every diagnostic carries a code, class, severity, source, affected object, message, remediation and provenance.
- **REQ-009** — The boundary record's public schema is JSON Schema 2020-12.
- **REQ-010** — Every adapter, plugin, API or schema surface that can read or write an analysis status goes through validation, diagnostics and the data-boundary controls. There is no unvalidated path that writes a status (also cited as DEL-02-03:1).
- **REQ-011** — The boundary model selects no solver library, rule expression grammar, API transport or project container.
- **REQ-012** — `USER_RULE_FAILED` means an evaluated user-defined rule failed, and `USER_RULE_CHECKED` means the user-rule evaluation completed with no blocking missing input.

### Acceptance criteria

- **AC-001** — The status vocabulary of the boundary schema is the seven statuses, and the set it admits as software output is the six automatic ones. It admits no human-approval, compliance, certification, sealing, approval or passed status as software output.
- **AC-002** — A boundary record is valid only when its mechanics part holds `MODEL_INCOMPLETE` or `MECHANICS_SOLVED` under solver authority; where a rule pack has been selected for the subject, its user-rule part holds one of the three user-rule statuses under user-data authority with a complete rule-pack reference; a record for a mechanics-only solve with no rule pack is valid without a user-rule part; its human acceptance appears only as references with bound evidence hashes; and its claim fields are false. A record that puts a status under another authority is refused.
- **AC-003** — A record for a subject with a missing solve-required or rule-required input carries the incomplete status and a diagnostic with its affected object and provenance, and carries no substituted value.
- **AC-004** — Each status in a record points to the evidence of the table above, and a record without it is refused. Each diagnostic in a record carries a code, class, severity, source, affected object, message, remediation and provenance, and a record whose diagnostic lacks one is refused.
- **AC-005** — No output of the solver or the rule evaluator contains `HUMAN_APPROVED_FOR_PROJECT` or a human acceptance record.
- **AC-006** — Each surface of REQ-010 refuses a status write that fails validation, and none writes a status by another path.
- **AC-007** — The schema, its description, its fixtures and its documentation contain no protected or proprietary content and no claim of compliance, certification or approval; the fixtures are invented.
- **AC-008** — The boundary schema is valid JSON Schema 2020-12, and each fixture validates against it.

## Production and Verification Method — Praxeology

**Method choices that define the boundary.**

- The authority is stated in the record, not inferred from which code wrote it.
- One part per authority. A fact about mechanics and a fact about a rule check are held side by side, never folded into one label.
- Human acceptance enters only as a reference to a record made elsewhere, with the hashes it binds.
- The claim fields of the professional-boundary statement are constants, so no writer can set them.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002, AC-004 and AC-008, executable check: observes the status enumerations of the boundary schema, and the acceptance or refusal of constructed records, a mechanics-only record and records with an incomplete diagnostic among them, under a JSON Schema 2020-12 validator that imports no product code.
- **VER-002** — AC-003 and AC-005, executable check: observes the status, diagnostic and absence of a substituted value for constructed incomplete subjects; and the absence of any human status or human acceptance record in solver and rule-evaluator outputs.
- **VER-003** — AC-006, judgment by a reviewer who did not write the surfaces, with an executable check: examines each adapter, plugin, API and schema surface that can write a status, and observes the refusal of an invalid write at each.
- **VER-004** — AC-007, judgment by a reviewer who did not write the schema or fixtures, with the protected-content scan: examines the texts against `docs/claims_registry.md` and `docs/IP_AND_DATA_BOUNDARY.md`.

## Governing Values and Decisions — Axiology

- **AX-001** — *The status says who is speaking.* Better is fewer places where a reader could take a software computation for a professional judgment; the target is none. A shorter label is never bought by dropping the authority or the qualifier that makes "approved" a person's project record.
- **AX-002** — *Missing is said, not filled.* Better is every missing input reaching the user as a named status and diagnostic. Completeness of a result is never obtained by a default.
- **AX-003** — *A simple headline, with its evidence beside it.* A coarse status is easier to read and to exchange; the detail lives in the evidence and diagnostics. A simultaneous fact, such as mechanics solved with rule inputs incomplete, is never hidden to fit one label.
- **AX-004** — *Acceptance stays outside the software.* Showing acceptance near a result is convenient; storing or generating it in a solver or rule result is never the way to do it.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-002 OBJ-001 OBJ-011 | REQ-001 REQ-002 REQ-012 | AC-001 | VER-001 | Owner ruling: the vocabulary of `docs/TYPES.md` §4 and the prohibited claims of PRD §21.2 |
| OUT-001 | SOW-002 OBJ-011 | REQ-003 REQ-004 REQ-006 REQ-007 | AC-002 | VER-001 | The records are constructed, so the criterion states which are valid; the authorities are those of PRD §5.1, §5.4 and §21, and PRD §17.5 permits a mechanics-only solve |
| OUT-001 | SOW-002 OBJ-001 | REQ-005 | AC-003 | VER-002 | The subjects are constructed, so the criterion states the expected status and diagnostic (PRD §5.5, §17.5) |
| OUT-001 | SOW-002 OBJ-011 | REQ-008 | AC-004 | VER-001 | The criterion states the expected evidence per status, from the table in the Ontology; the diagnostic fields are those of the architecture basis AB-00-06 of the decomposition |
| OUT-001 | SOW-002 OBJ-011 | REQ-007 | AC-005 | VER-002 | Owner ruling: PRD §21.3 keeps a human acceptance record separate from solver and rule-pack outputs; the criterion states the expected result, that none is found |
| OUT-001 | SOW-002 OBJ-011 | REQ-010 REQ-011 | AC-006 | VER-003 | The criterion states the expected refusal; the reviewer examines the surfaces against the architecture basis AB-00-07 of the decomposition, which the product did not write |
| OUT-001 | SOW-002 OBJ-001 | REQ-002 REQ-004 | AC-007 | VER-004 | Owner ruling: PRD §20.1 and §21.2, `docs/IP_AND_DATA_BOUNDARY.md` and `docs/claims_registry.md` |
| OUT-001 | SOW-002 OBJ-001 | REQ-009 | AC-008 | VER-001 | Outside source: the JSON Schema 2020-12 specification, applied by a validator that imports no product code |
