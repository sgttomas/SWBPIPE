---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-03
package_id: PKG-16
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-069,SOW-070]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-03 User acceptance and operation audit trail

## Purpose and Objective Traceability

A model changes only because a person decided it should, and afterwards anyone reviewing the model can see what was changed, by whom or by what, why, on which assumptions and against which model state, including the proposals that were turned down. This deliverable contributes the two things that make that true: the acceptance rule, which says what counts as the person's decision and what a batch must have before it can be decided on, and the operation history, the durable record of those decisions and their outcomes.

It serves SOW-069 and SOW-070 under OBJ-015 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-16, row DEL-16-03), with `docs/PRD.md` §5.3, §10.3 step 5, FR-AGENT-003 and FR-AGENT-004. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 17, under which a change is applied on the person's acceptance and the agent records no acceptance, through DEL-16-06.

- **OUT-001** — The acceptance rule and the operation history: its records of accepted, rejected and held batches and what each holds.
- **OUT-002** — The handoff between decision and application: the decision record application consumes, and the outcome record the history takes back.

## Deliverable Definition — Ontology

**Terms the deliverable implements.**

- *Acceptance.* The person's explicit act that approves a previewed batch for application to the current model. It is a decision about a model edit in the person's own working session. It is not professional acceptance of engineering work, and no record or label presents it as such (PRD §21.2).
- *Decision record.* What was decided on a batch: accepted, rejected, or held for the person's decision. A batch is held when it lacks something acceptance requires. A proposal withdrawn before a decision is recorded as withdrawn, not as rejected.
- *Outcome record.* What application did with an accepted batch: applied, refused, or unknown. Unknown means no evidence of the result was kept; it is never replaced by a guess.
- *Operation history.* The durable, ordered record, per project, of decisions and outcomes. This is the audit trail of SOW-070.
- *History record.* One batch's entry: the operations as proposed, unaltered; the entities they affect; each operation's author type and source; the rationale and the assumptions; the validation outcome and the preview it was decided on; the model hash it was decided on and, when applied, the resulting model hash; the times of proposal, decision and outcome.
- *Author stamp.* The author type a route gives an operation. The route sets it. A claim of authorship inside imported or agent content is kept only as display context.
- *Person-entered value.* The bend flexibility factor k, the SIFs, the other component modifiers and their source, as DEL-16-06 defines them. Users enter their own k and SIF. The engine's refusal of such a write by anyone else is DEL-16-06's; it relies on the author stamp.
- *Review context.* A record read back from a reopened project whose acceptance is not known. It can be read and requeued; it cannot be applied as it stands.

**Boundary.**

- The record's form is DEL-16-01's; validation and the preview are DEL-16-02's; applying a batch and returning its outcome are DEL-16-06's; what an agent's rationale and assumptions must hold are DEL-16-04's.
- The review surface is DEL-07-08's: showing the preview, the Apply and Reject controls and the ledger panel. Collecting queued changes into one batch is DEL-07-02's.
- The project store is DEL-02-05's. It keeps the operation history's records across save and reopen (DEL-02-05/REQ-037), which REQ-005 and AC-004 rely on. Saved model states are DEL-14-01's.
- A formal professional-acceptance workflow is not required (PRD §21.3) and is not this deliverable's; the words for that boundary are DEL-01-04's.

**Not decided.**

- Whether a person can grant an agent standing permission to apply changes without a decision on each batch (decomposition OI-016; FR-AGENT-003 allows a configured workflow, and exit criterion 17 allows autonomy the person granted). Until that is decided every batch needs the person's own act.
- How long rejected and withdrawn records are kept, and when one may be removed.
- Whether an author is ever a verified identity. Until that is decided author type and source are asserted metadata from the trusted desktop, and the history says so.
- The precision and clock of the recorded times.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Only structured operations in DEL-16-01's contract are decided on and recorded (SOW-069).
- **REQ-002** — A batch can be accepted only when its validation passed with no blocking finding, a preview was generated for it, and it has not been applied (SOW-069).
- **REQ-003** — The history records accepted, rejected and held batches (decomposition row DEL-16-03).
- **REQ-004** — A history record holds the affected entities, each operation's author type and source, the times, the assumptions, the validation outcome and the preview reference. A missing input is recorded as missing with a diagnostic.
- **REQ-005** — For accepted operations the history preserves the operations, their rationale and assumptions, the affected entities, the audit metadata and the model hashes needed to review a model state reproducibly, and it survives save and reopen (SOW-070; FR-AGENT-004).
- **REQ-006** — Acceptance requires the person's explicit act. A signal from an agent, an import or any other source is never acceptance (FR-AGENT-003; decomposition OI-016).
- **REQ-007** — Acceptance of a model change is never represented as professional approval, certification, sealing or code compliance (PRD §21.2).
- **REQ-008** — Public examples and sample records hold invented values only: no protected standards text, code-specific value, proprietary project data or private engineering data (`docs/IP_AND_DATA_BOUNDARY.md`).
- **REQ-009** — Missing data and assumptions are visible in the record and are never filled with a default (PRD §5.5).
- **REQ-010** — Acceptance is bound to the preview it was given on: the preview's model hash must be the current model's hash when the person decides.
- **REQ-011** — Recording a decision changes no model and applies nothing.
- **REQ-012** — A rejected batch is recorded as rejected and changes nothing.
- **REQ-013** — The author stamp is set by the route. A desktop editor stamps the person. The embedded agent's host and offline proposal intake stamp `agent`, whatever the content claims, and keep a claimed author only as display context. A retained record requeued after reopening is stamped `agent` when its origin is recognisably an agent or an import; one that claims a person and writes a person-entered value is refused; one that names no recognisable author is refused. A retained record that claims a person but whose human authorship cannot be established, such as one saved before routes stamped authors, which cannot be told from an import that claimed a person, never receives person-authored treatment on requeue. It stays recoverable: a refusal or a requeue without person-authored treatment leaves it in the history as review context. An operation not authored by a person that carries a person-entered member (a component's modifiers, a bend realization or a curved bend interface), at any depth of its value, is refused at intake and at requeue. That no review row shows a member that will not be written, for any member and any author, is DEL-16-02's commitment, not yet met for a member a creator does not carry.
- **REQ-014** — The decision and the outcome are two records. Application consumes the decision made before it; the history takes the actual outcome from application afterwards and never writes an outcome of its own.

### Acceptance criteria

- **AC-001** — A batch reaches application only after the person's explicit Apply on a batch whose validation passed, whose preview was generated, and whose preview hash is the current model's hash. A batch with a blocking finding, with no preview, on a stale basis, or carrying only an agent's or an import's signal is held with a diagnostic that says what is missing, and nothing is applied.
- **AC-002** — An applied batch has exactly one decision record and one outcome record, whichever surface produced the batch. The outcome record holds every operation with its author and is bound to the decision by the batch's identity, the model hash decided on and the resulting model hash. An accepted batch that application refuses is recorded as refused, and a batch with no outcome evidence is recorded as unknown, never as applied or rejected.
- **AC-003** — Every history record holds each member the history record is defined to hold, with the operations exactly as proposed. An input that is absent appears as missing with its diagnostic, and no placeholder reads as a value.
- **AC-004** — After save, close and reopen, the project's history lists every accepted and every rejected batch decided before the save, in order, with every member equal. A record read back whose acceptance is not known is review context: it must be requeued and validated afresh before it can be applied.
- **AC-005** — Recording an acceptance, a rejection, a hold or a withdrawal leaves the model document and its hash unchanged. A rejected batch is in the history as rejected and no operation of it is applied.
- **AC-006** — Each history record carries a hash that is the SHA-256 of its RFC 8785 canonical form and is reproduced from the stored record; the same decision on the same inputs gives the same record.
- **AC-007** — An operation from a desktop editor is stamped as the person's. An operation from the embedded agent or from offline intake is stamped `agent` with a host-set source whatever its content claims, and the claimed author appears only as display context marked unverified. On requeue after reopening, a record of agent or imported origin is stamped `agent`, a record claiming a person that writes a person-entered value is refused, and a record with no recognisable author is refused. A record that claims a person but whose human authorship cannot be established is never requeued as the person's, and after a refusal or a requeue it is still in the history, readable as review context. An imported operation or a requeued record not authored by a person that carries a person-entered member (a component's modifiers, a bend realization or a curved bend interface) at any depth of its value is refused before it is queued, naming the operation and where the member is, and the stored record is kept.
- **AC-008** — No history record, decision, outcome, label or exported ledger states or implies professional approval, certification, sealing, authentication or code compliance. An acceptance record says that it is the person's acceptance of a model change in a local session and is not professional approval.
- **AC-009** — Every public example and sample record holds invented values and carries a provenance record that marks it invented.

## Production and Verification Method — Praxeology

**Method choices that define the deliverable.**

- The person's Apply in the review surface is the only acceptance signal. No call, message or tool available to an agent or an import carries one.
- The history stores what was proposed, not what was applied in its place: operations are kept as submitted, and the outcome is a separate record.
- Trust is placed in the route, not in the content. The bundled desktop is trusted to say which route an operation came by; content is never trusted to say who wrote it.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-005, executable check: observes application calls, the model hash and the history after each of an accepted batch, a rejected one, and batches lacking in turn a passed validation, a preview, a current basis and the person's act.
- **VER-002** — AC-002, executable check: observes the decision and outcome records after an applied batch from each surface (a desktop editor, batch review, the embedded agent, offline intake), after an accepted batch that application refuses, and after an application whose result is withheld from the history.
- **VER-003** — AC-003 and AC-006, executable check: observes each record's members against the scripted decision that produced it, removes each input in turn to observe the missing marker, and recomputes each record's hash with an RFC 8785 and SHA-256 implementation that shares no code with the product.
- **VER-004** — AC-004, executable check: scripts a sequence of accepted and rejected batches, saves, closes and reopens the project in the native application, and compares the history read back with the scripted sequence; then attempts to apply a record read back without requeueing it.
- **VER-005** — AC-007, executable check: observes the stamp and source on operations sent by each route with content that claims each author type, and the result of requeueing retained records of each origin, including a record that claims a person and was saved without a route stamp, and the history after each refusal and requeue; and an imported batch and a retained record of each origin carrying each person-entered member (a component's modifiers, a bend realization, a curved bend interface) at the top level of its value and nested in an object and an array, each refused before the review queue.
- **VER-006** — AC-008 and AC-009, judgment by a named role (a reviewer who did not write the records): reads the history, the acceptance record, the labels and the exported ledger against the prohibited claims of PRD §21.2 and `docs/claims_registry.md`, and the examples for protected or private content, with an executable check of each example's provenance record.

## Governing Values and Decisions — Axiology

- **AX-001** — *A complete history.* Better is a larger share of the decisions made on a project's operations that can be read back after reopening with every member; the target is all of them. Completeness never comes from recording as accepted or applied something not known to be: unknown stays unknown.
- **AX-002** — *The person decides.* Better is fewer ways for a batch to reach application without the person's own act; the target is none. Convenience for an agent, an import or a bulk edit is never a reason for another way.
- **AX-003** — *Truthful attribution.* Better is a larger share of records whose author stamp matches the route that produced them. A person's own edit is never relabelled as an agent's to be safe, and a claim of authorship inside content is never taken as the person's.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-069 OBJ-015 | REQ-001 REQ-002 REQ-006 REQ-010 | AC-001 | VER-001 | Each scripted batch states what it lacks and so what must be held |
| OUT-002 | SOW-069 SOW-070 OBJ-015 | REQ-014 | AC-002 | VER-002 | The scripted scenario states what was accepted and what application returned |
| OUT-001 | SOW-070 OBJ-015 | REQ-003 REQ-004 REQ-009 | AC-003 | VER-003 | The comparison is with the scripted decision, which the product did not write |
| OUT-001 | SOW-070 OBJ-015 | REQ-005 | AC-004 | VER-004 | The comparison is with the scripted sequence of decisions, which the product did not write |
| OUT-001 | SOW-069 OBJ-015 | REQ-011 REQ-012 | AC-005 | VER-001 | The criterion states the expected result: an unchanged hash, compared with the hash taken before |
| OUT-001 | SOW-070 OBJ-015 | REQ-005 | AC-006 | VER-003 | Outside source: RFC 8785 and SHA-256, computed by an implementation that shares no code with the product |
| OUT-001 | SOW-069 SOW-070 OBJ-015 | REQ-013 | AC-007 | VER-005 | The routes and the claims in the content state what each stamp must be. The check shows the stamp each route sets; it cannot show that a stamp from the trusted desktop is true, since the author is asserted metadata |
| OUT-001 | SOW-070 OBJ-015 | REQ-007 REQ-008 | AC-008, AC-009 | VER-006 | PRD §21.2, §21.3 and `docs/claims_registry.md`, read by a reviewer who did not write the records |
