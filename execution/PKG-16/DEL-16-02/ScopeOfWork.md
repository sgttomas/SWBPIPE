---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-02
package_id: PKG-16
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-069]
package_objective_refs: [OBJ-015]
---

# Scope of Work — DEL-16-02 Operation validation and diff preview

## Purpose and Objective Traceability

Before a change touches the model, the person deciding on it sees exactly what it would do, or is told by name why it cannot be done. This deliverable checks an operation or a batch against the operation contract, the model's own rules, its units, its references and the model state it was written for, changing nothing, and returns either a deterministic preview of every record the change would alter or the refusal an apply would give. The result it contributes is that acceptance is an informed act and that an invalid change is stopped before application.

It serves SOW-069 under OBJ-015 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-16, row DEL-16-02), with `docs/PRD.md` §5.3, §10.3 step 4, FR-AGENT-002 and the diff of FR-AGENT-003. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 17 through DEL-16-06, whose one operation route validates here.

- **OUT-001** — Operation validation and the diff preview: the checks, the refusals by name, and the preview rows.
- **OUT-002** — The validated outcome that acceptance and application are bound to: what was evaluated, on which model state, with which result, and nothing applied.

## Deliverable Definition — Ontology

**Terms the validation implements.**

- *Validate-only run.* An evaluation of one operation or one batch against a model document. It returns an outcome and never a model document or an acceptance record.
- *Check.* One of: the operation contract; the target and every reference; units and dimensions; the before values; the model basis; the model document's rules, judged on the candidate document; and constraint findings. The contract check comes first, and an operation that fails it passes no other.
- *Candidate document.* The document the operation would produce, built privately so that the same rules judge a preview as judge an apply. It is discarded by a validate-only run.
- *Refusal.* A blocking diagnostic with a code that names the rule, the record and the remedy. A refused operation has no generated preview and is reported as not applied.
- *Warning.* A finding that does not block. It is reported with the preview, the same in a validate-only run as in an apply.
- *Diff preview.* The rows that say what the operation would change: for each changed member, the record, its object type, the field path, the value before and after, the unit, the dimension and the change kind. Consequential changes are rows too, such as a deleted pipe leaving the line that listed it.
- *Stale basis.* The model hash an operation or batch claims differs from the hash of the model it is evaluated on.
- *Stale before value.* The value the author recorded as current differs from the model's.
- *Batch evaluation.* Each operation of a batch is evaluated on the document the previous one left. One refusal refuses the batch.

**Boundary.**

- The record's form and the taxonomy are DEL-16-01's. Validation trusts nothing a record says about itself: a submitted validation state, status or boundary value is checked, never believed.
- Applying a validated operation is DEL-16-06's. Validation and application are two modes of one engine; the validate-only mode is this deliverable's.
- The person's decision and the operation history are DEL-16-03's. Showing the preview to the person is DEL-07-08's.
- The model document's rules and its validator are DEL-02-01's; the unit catalog and dimension checks are DEL-02-02's; the model hash is DEL-08-02's; the diagnostic record is DEL-00-06's.
- What a valid value means for a change kind belongs to the deliverable that owns the records it changes. This deliverable runs those rules and reports their findings.

**Deferred.** Validation against design constraints (connectivity, route, clearance, support zone, slope) is DEL-13-03's engine. It is deferred to v0.2 with design knowledge. REQ-003 applies from the release in which those constraints exist; until then the constraints an operation meets are the model document's rules.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Only a structured operation in DEL-16-01's contract is validated. A request to set model state directly, and a record that declares itself already applied or accepted, are refused (SOW-069).
- **REQ-002** — An operation is checked against the operation contract before it is treated as contract-passed, and one that fails is never reported as passed (FR-AGENT-002).
- **REQ-003** — A blocking finding from constraint validation blocks the operation before application (SOW-069; FR-AGENT-002).
- **REQ-004** — A valid operation or batch yields a deterministic preview of what it would change, before application (SOW-069; PRD §10.3).
- **REQ-005** — An invalid operation is blocked before application and the model is unchanged (decomposition row DEL-16-02).
- **REQ-006** — Outcomes are structured diagnostics in DEL-00-06's record, and none states certification, sealing, approval, authentication or code compliance (PRD §21.2).
- **REQ-007** — Validation is one service that every caller reaches the same way. No GUI module, adapter, plugin or agent route validates by rules of its own or goes round it (decomposition §8).
- **REQ-009** — An operation or batch written against another model state is refused: its claimed model hash must be the hash of the model it is evaluated on, and a before value must equal the current value.
- **REQ-010** — A unit-bearing value is checked for a known unit and for the dimension of its target field. Nothing is converted or defaulted silently (PRD §5.5, §5.6).
- **REQ-011** — A target, and every record an operation names, must exist in the model or be created earlier in the same batch.
- **REQ-012** — A validate-only run gives the answer an apply would give: the same refusal, or a preview of exactly the changes the apply makes.

### Acceptance criteria

- **AC-001** — For each check, an invented operation that fails only that check is refused with that check's named diagnostic, gets no generated preview and is reported as not applied. An operation that fails the contract is never reported as contract-passed, and a record that declares itself applied or accepted is refused. The constraint-finding case applies from the release in which REQ-003 applies.
- **AC-002** — A validate-only run returns no model document and no acceptance record, whatever its result. The model document it was given is byte for byte the same afterwards, with the same model hash. The outcome names the model hash it was evaluated on and the state of each check, so a later decision and application can be bound to it.
- **AC-003** — The same operation or batch on the same model document gives the same outcome and the same preview rows on every run, equal in canonical form, in the native build and in the web build of the engine.
- **AC-004** — For a valid operation or batch, the preview lists every record member an apply of it changes and no other, and an apply changes exactly those. For a refused one, an apply gives the same refusal. In a batch each operation is evaluated on the document the previous one left, and one refusal refuses the whole batch.
- **AC-005** — An operation or batch whose claimed model hash is not the hash of the model it is evaluated on is refused by name, as is one whose before value differs from the model's current value. A batch with no claimed hash is refused.
- **AC-006** — A unit-bearing value with a missing unit, an unknown unit or a unit of another dimension than its target field is refused by name. No stored or previewed value differs from the entered one by a conversion the author did not ask for.
- **AC-007** — An operation that names a record absent from the model and not created earlier in its batch is refused by name with the missing reference.
- **AC-008** — Every diagnostic carries the fields of DEL-00-06's record, and no diagnostic, state or outcome value states or implies certification, sealing, approval, authentication, acceptance of engineering work or code compliance.
- **AC-009** — A single edit in the desktop, a batch in review, a proposal from the embedded agent and a proposal from offline intake all validate through the same engine entry. No product route validates an operation by rules held in any other module, or reaches application without that entry.

## Production and Verification Method — Praxeology

**Method choices that define the validation.**

- One engine. Validation runs in the operation engine that applies, in its native build and its web build, and no product route uses a second implementation of the rules. A separate implementation kept for reference, which no product route calls, is not such a route.
- The preview is made from the candidate document. A validate-only run builds the document an apply would publish, has the one document validator judge it, and reads the preview from it.
- Refuse and name. A failed check is a blocking diagnostic that names the record and the rule; validation supplies no value and repairs nothing.
- Layered tests on public examples. Validation is examined by layered tests, and every refusal on a public, invented example (decomposition AB-00-08; PRD §5.4).

**How each criterion is examined.**

- **VER-001** — AC-001, AC-005, AC-006 and AC-007, executable check: observes the outcome of invented operations each built to fail one check (contract, reference, unit, dimension, before value, model basis, document rule, and constraint finding from the release in which REQ-003 applies) and of their valid twins.
- **VER-002** — AC-002, executable check: observes the bytes and model hash of the input document before and after validate-only runs that pass and that are refused, and the absence of a model document and an acceptance record in each outcome.
- **VER-003** — AC-003, comparison between runs and between the native and the web build of the engine: the outcomes and preview rows of the same invented operations, in canonical form.
- **VER-004** — AC-004, comparison with a structural difference of the model document before and after apply, computed by a differ that imports no engine code: the preview rows against that difference, and the refusal of a validate-only run against the refusal of the apply.
- **VER-005** — AC-008, executable check with judgment by a named role (a reviewer who did not write the diagnostics): validates each emitted diagnostic against DEL-00-06's record, and reads the codes, messages and state values against the prohibited claims of PRD §21.2.
- **VER-006** — AC-009, executable check: observes, for each caller, the engine entry its validation reaches, and searches the modules any product route reaches for operation validation rules outside the engine.

## Governing Values and Decisions — Axiology

- **AX-001** — *The preview is what happens.* Better is a smaller difference between what a preview shows and what the apply changes; the target is none. A consequential change is never left out to make a preview shorter.
- **AX-002** — *A refusal the author can act on.* Better is a larger share of refusals that name the record, the rule and the remedy, so that a person or an agent corrects the operation in one step. Refusals are never reduced by relaxing a check or by supplying a value.
- **AX-003** — *Cheap enough to run before every decision.* Better is a shorter time to validate an operation or a batch. Time is never bought by skipping a check or by validating against anything but the current model.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-069 OBJ-015 | REQ-001 REQ-002 REQ-003 REQ-005 | AC-001 | VER-001 | Each invented operation states the check it must fail. The engine's recorded outcomes for the same operations are its own earlier output and only guard against change |
| OUT-002 | SOW-069 OBJ-015 | REQ-005 | AC-002 | VER-002 | The criterion states the expected result: the same bytes and hash, and no document or acceptance in the outcome. The comparison is with a copy taken before the run |
| OUT-001 | SOW-069 OBJ-015 | REQ-004 | AC-003 | VER-003 | The comparison is between runs and between the native and the web build. Agreement shows determinism and build parity, not a correct preview, which AC-004 examines |
| OUT-001 | SOW-069 OBJ-015 | REQ-004 REQ-012 | AC-004 | VER-004 | An outside reference: a structural difference of the two documents by a differ that imports no engine code |
| OUT-001 | SOW-069 OBJ-015 | REQ-009 | AC-005 | VER-001 | The stale cases state what must be refused. The model hash they are built from is DEL-08-02's, reproduced by an RFC 8785 and SHA-256 implementation that shares no code with the product |
| OUT-001 | SOW-069 OBJ-015 | REQ-010 | AC-006 | VER-001 | Each invented operation states the unit or dimension that must be refused. Unit and dimension membership comes from the unit catalog, which DEL-02-02 owns |
| OUT-001 | SOW-069 OBJ-015 | REQ-011 | AC-007 | VER-001 | The criterion states the expected result; the invented operations name records that do not exist |
| OUT-001 | SOW-069 OBJ-015 | REQ-006 | AC-008 | VER-005 | The record's fields are DEL-00-06's; the prohibited claims are those PRD §21.2 lists, read by a reviewer who did not write the diagnostics |
| OUT-001 | SOW-069 OBJ-015 | REQ-007 | AC-009 | VER-006 | Each caller's route is traced to the engine entry it reaches; a module no product route reaches is outside the search |
