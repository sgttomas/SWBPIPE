---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-06
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-061]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-06 Diagnostics, warning, and result-envelope contract

## Purpose and Objective Traceability

This deliverable is the common shape of a finding and of a result. Whatever layer raises it, a diagnostic says what was found, how serious it is, which class of problem it is, what object it concerns, what the user can do about it, and where the value in question came from. Whatever layer returns it, a result envelope says what kind of outcome it is and carries its diagnostics. The result this contributes is that the solver, the rule engine, the GUI, the runner, reports and exports all report problems in one vocabulary; that missing data is shown and never defaulted; and that no message reads as approval.

It serves SOW-061 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-06, and architecture-basis row AB-00-06 in §8.1), with `docs/PRD.md` §5.5, §5.9, FR-GUI-006, §14.4 and §21.2 and `docs/SPEC.md` §8. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 3 (every remaining refusal is listed) through the deliverables that own each refusal.

- **OUT-001** — The diagnostic record and the warning-class vocabulary.
- **OUT-002** — The result-envelope contract: the outcome kinds and how an envelope carries diagnostics.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Diagnostic.* A machine-readable finding with these fields: code, class, severity, source (the component that raised it), affected object (the entity or input it concerns), message, remediation, and provenance where the finding concerns a sourced value.
- *Warning class.* One of six:
  - `SOLVE_BLOCKING`: a physical input the mechanics need is missing.
  - `RULE_CHECK_BLOCKING`: the mechanics can solve, but a rule-pack check lacks data the user must supply.
  - `PROVENANCE_WARNING`: a value exists but its source is missing or weak.
  - `ASSUMPTION_WARNING`: an assumption by the user or the model needs review.
  - `NONLINEAR_WARNING`: convergence or the active state of a support is uncertain.
  - `IP_BOUNDARY_WARNING`: a public contribution or report may contain protected or private data.
- *Local class.* A class in a boundary's own vocabulary, which a boundary may declare where its schema needs one. The runner's `RUNNER_BLOCKING` and `PRIVACY_WARNING` and the result export's `EXPORT_BLOCKING` and `UNIT_WARNING` are local classes; at the export boundary `RUNNER_BLOCKING` becomes `EXPORT_BLOCKING` and `PRIVACY_WARNING` becomes `IP_BOUNDARY_WARNING`.
- *Crossing.* The passage of a diagnostic from one boundary's record to another's. Every field other than the class arrives unchanged: code, severity, source, affected object, message, remediation and provenance. The class changes only where the receiving boundary's schema requires its own class vocabulary, and then by that boundary's stated mapping.
- *Result envelope.* The structured return of a nontrivial operation: its outcome, its diagnostics and its payload.
- *Outcome kind.* One of five that the envelope of an operation evaluating a model distinguishes: invalid input, incomplete model, mechanics result, rule-check result, and human review needed.

**Boundary.** This deliverable defines the record, the classes and the envelope's shape; it raises no diagnostic and owns no schema file. Its criteria are observed on the schemas and outputs of the deliverables below, and a shortfall is repaired there.

- DEL-00-03 owns the rule that every command and job returns an envelope. This scope's part is what each envelope contains.
- DEL-05-04 owns the analysis status vocabulary that expresses the outcome kinds for a run.
- DEL-04-06 owns the solver's diagnostics; each other deliverable owns the diagnostics it raises and their codes.
- DEL-07-04 owns how warnings and blockers are shown; DEL-08-03 owns the report section that lists them.
- DEL-08-04, DEL-10-05 and DEL-14-02 own the envelopes of the result export, the runner and the analysis-run record. DEL-16-02 and DEL-16-06 own the envelope of the operation outcome (validate-only and apply).

**Not decided.** Whether one shared diagnostic schema replaces the per-boundary class vocabularies. The namespace of diagnostic codes. The severity taxonomy. Whether and how messages are localized. Which diagnostic a failed audit checksum raises.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Diagnostics are machine-readable and carry code, class, severity, source, affected object, message and remediation, and provenance where applicable (decomposition AB-00-06).
- **REQ-002** — The six warning classes are preserved. A boundary may declare local classes where its schema needs them. A crossing preserves every field other than the class, and changes the class only where the receiving boundary's schema requires its own class vocabulary, by that boundary's stated mapping.
- **REQ-003** — Solver, rule-pack, GUI, command-line, report, storage and adapter outputs use result envelopes for nontrivial operations.
- **REQ-004** — Invalid input, incomplete model, mechanics result, rule-check result and human-review-needed are distinguished (`docs/PRD.md` §14.4; `docs/CONTRACT.md` OPS-K-MECH-2).
- **REQ-005** — Diagnostics claim no certification, code compliance, approval or professional authentication (`docs/PRD.md` §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).

### Acceptance criteria

- **AC-001** — Every schema that carries diagnostics requires the code, class, severity, source, affected object, message and remediation fields and provides for provenance, and every diagnostic the product emits validates against the schema of the boundary that emits it.
- **AC-002** — A diagnostic carried across a boundary arrives with its code, severity, source, affected object, message, remediation and provenance equal to the original. Its class is unchanged, except where the receiving boundary's schema requires its own class vocabulary; there it is the class that boundary's stated mapping gives.
- **AC-003** — The envelope returned by each of the seven kinds of output named in REQ-003 carries a list of diagnostics that meet AC-001 and a field that states its outcome.
- **AC-004** — From the envelope's fields alone, without reading message text, a program can tell which of the five outcome kinds an envelope of a validation, solve, rule check, run record or result export holds. An input that is invalid, a model that is incomplete and a rule check that lacks data each produce their own kind and never a mechanics result with an empty payload.
- **AC-005** — No diagnostic code, class, message or remediation, and no envelope value, states or implies certification, code compliance, approval or authentication. A remediation says what is missing and never supplies an engineering value.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- Schema first: each boundary's diagnostic and envelope have a schema contract, in the dialect the baseline names (DEL-00-01).
- A boundary keeps its own class vocabulary where its schema needs one, and the mapping it applies to a class arriving at it is part of the boundary's contract.
- Missing data is a diagnostic. It is never a default and never a silent omission.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-003, executable check: observes every schema with a diagnostics member for the required fields, and validates the diagnostics and envelopes emitted by each kind of output on inputs that provoke each class.
- **VER-002** — AC-002, executable check: carries diagnostics across each boundary pair (solver to result export, runner to export, run record to report) and observes every field other than the class against the original, and the class against the receiving boundary's stated mapping where that boundary's schema requires its own vocabulary.
- **VER-003** — AC-004, executable check: submits an invalid input, an incomplete model, a solvable model and a rule check with missing data, and observes the outcome kind of each envelope from its fields.
- **VER-004** — AC-005, judgment by a reviewer who did not write the diagnostics: examines the code, class and message vocabularies against `docs/claims_registry.md`, with an executable check for prohibited claim terms.

## Governing Values and Decisions — Axiology

- **AX-001** — *One vocabulary.* Better is fewer class changes at a crossing that the receiving boundary's stated mapping does not give; the target is none. This is never bought by merging two classes that call for different responses from the user.
- **AX-002** — *Findings a person can act on.* Better is a larger share of diagnostics that name the affected object and a remediation the user can carry out. A remediation never suggests an engineering value, and severity is never lowered to let a result through.
- **AX-003** — *Machine first.* The outcome is read from fields and never from the wording of a message. Wording may improve freely; a code or a class changes only as a contract change.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-061 OBJ-013 | REQ-001 | AC-001 | VER-001 | Decomposition AB-00-06 names the fields; the comparison is each schema and each emitted diagnostic |
| OUT-001 | SOW-061 OBJ-013 | REQ-002 | AC-002 | VER-002 | `docs/SPEC.md` §8 and decomposition AB-00-06 name the six classes; the comparison is each carried diagnostic against the original, and its class against the receiving boundary's stated mapping |
| OUT-002 | SOW-061 OBJ-013 | REQ-003 | AC-003 | VER-001 | Decomposition AB-00-06 and SOW-061 name the outputs; the comparison is each kind of output's envelope against AC-001's fields and an outcome field |
| OUT-002 | SOW-061 OBJ-013 | REQ-004 | AC-004 | VER-003 | `docs/PRD.md` §14.4 and decomposition AB-00-06; each scenario states the outcome kind that must result |
| OUT-001 | SOW-061 OBJ-013 | REQ-005 | AC-005 | VER-004 | `docs/PRD.md` §21.2 and §5.5 and `docs/claims_registry.md` say what may not be claimed; the comparison is the code, class and message vocabularies against them |
