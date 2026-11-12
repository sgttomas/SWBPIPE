---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-16-04
package_id: PKG-16
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-070]
package_objective_refs: [OBJ-015, OBJ-018]
---

# Scope of Work — DEL-16-04 Agent rationale and professional-boundary controls

## Purpose and Objective Traceability

When an agent proposes a change, the person deciding on it and anyone reviewing the model later can see why the agent proposed it and what it assumed, and nothing the agent emits can pass for engineering acceptance. This deliverable is the control on the agent route that makes both true. It says what an agent's proposal must carry and keep, and what no agent output and no surface that shows agent output may ever state.

It serves SOW-070 under OBJ-015 and OBJ-018 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-16, row DEL-16-04), with `docs/PRD.md` §5.9, FR-AGENT-004, FR-AGENT-005, §21.2 and §21.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It reaches criterion 17, under which the agent records no acceptance, approval or reliance, through DEL-16-06, whose embedded agent is the route this control governs.

- **OUT-001** — The agent rationale record and the professional-boundary controls on every surface where agent output is shown or stored.

## Deliverable Definition — Ontology

**Terms the control implements.**

- *Agent rationale.* The agent's stated reason for an operation. Every agent-authored operation has one. It is decision support for the person and has no other standing.
- *Constraints considered.* What the agent says it took into account in proposing the change (FR-AGENT-004).
- *Unresolved assumption.* Something the agent took as true without the model or the person establishing it. Assumptions are declared as records, each with an identifier, a statement and the operations it affects. An empty declaration says there are none.
- *Agent attribution.* The reference to where in the agent's conversation a proposal came from: the conversation, the turn and the call. The host sets it; the agent cannot.
- *Agent surface.* Any place the product shows or stores what an agent produced: the conversation panel, a proposal in batch review, a receipt, the operation history and an exported ledger.
- *Prohibited claim.* A statement that software or an agent certifies, seals, approves or authenticates engineering work; declares code compliance for professional reliance; replaces the engineer of record or project-specific professional review; provides standards-body approval or official interpretation; or makes public example data suitable for project use (PRD §21.2). A statement that it accepts engineering work is one too (PRD §5.3). The registered wording of the boundary is in `docs/claims_registry.md`.
- *Copied context.* Text an agent's output carries over from an operation, a validation outcome or a history record. A prohibited claim in copied context is a prohibited claim.
- *Boundary record.* The fixed professional-boundary values every operation and outcome carries: human review is required, and the software makes no compliance, certification, sealing, approval or authentication claim.
- *Human acceptance record.* A record a person makes outside the software, bound to the hashes of what was reviewed. The product generates none and has no acceptance workflow (PRD §21.3).

**Boundary.**

- The record's form is DEL-16-01's. Keeping rationale and assumptions in the operation history is DEL-16-03's; this deliverable says what they must hold and what must never be in them.
- The embedded agent's host and its capability catalog are DEL-16-06's. That host applies this control on its surfaces.
- The registered boundary statements and labels are DEL-01-04's; the automatic status tokens are DEL-02-03's; linting a report for prohibited claims is DEL-08-05's; redaction and export control of private content are DEL-12-02's; the threat model is DEL-12-05's.
- The report workflow passes no agent rationale to downstream runtime integrations, and no report infers human or professional approval from agent rationale or from an applied agent proposal.
- Generating route or support candidates is not this deliverable's. Its ownership is unassigned and the feature is deferred to v0.2.

**Staged.** The scan of agent-authored text for prohibited claims belongs to milestone R7 of PRD §24: AC-004, and REQ-010's blocking of prohibited claims, apply from that milestone. The embedded agent of v1.0 comes before it. Until then, on agent surfaces, REQ-004 rests on what the product itself controls: the host's stamps and statuses, the fixed boundary record and the product's own strings (AC-005), and the absence of any agent route to acceptance (AC-003). Until then a prohibited claim the agent writes in its own text is not blocked, and it gains no standing.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-002** — For an accepted agent-authored operation, the operation history keeps its rationale, the constraints considered, the unresolved assumptions, the affected entities and the agent attribution, in a form that supports reproducible review of the model state (SOW-070; FR-AGENT-004).
- **REQ-003** — Agent output is a proposal until a person accepts it or it falls within autonomy the person granted. Recording a rationale accepts nothing, applies nothing and changes no model (PRD §5.3; FR-AGENT-001; FR-AGENT-003).
- **REQ-004** — Software and agents do not claim to certify, seal, approve or authenticate engineering work, or to declare code compliance for reliance (FR-AGENT-005; PRD §21.2; `docs/CONTRACT.md` OPS-K-AUTH-1).
- **REQ-005** — No automatic status on an agent surface is, or stands for, human approval, code compliance, certification, sealing or approval (`docs/TYPES.md` §4).
- **REQ-006** — A human acceptance record is external to the software, owned by the person and bound to the hashes of what was reviewed. The product generates no professional approval (PRD §21.3).
- **REQ-007** — Missing data, unresolved assumptions, warnings and limitations stay explicit findings and are never turned into defaults (PRD §5.5; `docs/CONTRACT.md` OPS-K-AGENT-1).
- **REQ-008** — Public artifacts carry no protected standards text, code-specific value, proprietary data, private project data or private rule-pack content (`docs/IP_AND_DATA_BOUNDARY.md` §§2-6).
- **REQ-009** — Every path that carries agent rationale (the operation route, the history, persistence and any export) keeps the contract check, provenance, the diagnostics, the private-data and protected-content controls, the hashes and the boundary record.
- **REQ-010** — The control works in both directions: it preserves legitimate rationale and assumptions unchanged, and it blocks prohibited claims.

### Acceptance criteria

- **AC-001** — An agent-authored operation with an empty rationale or without a complete source is refused by name before it is queued; a person's own operation needs neither. A proposal declares its assumptions as records, and one whose assumption has a duplicate identifier or names an operation outside the proposal is refused.
- **AC-002** — After a person applies an agent proposal, the receipt and the history record show, for each operation, its rationale, the declared assumptions with the operations each affects, the affected entities, the agent attribution and the model basis. None is merged into free text or dropped, and all are equal after save and reopen. A proposal that was rejected or withdrawn, or whose outcome is unknown, keeps them too.
- **AC-003** — Recording a rationale creates no acceptance record, starts no application and leaves the model hash unchanged. No tool, message or record available to an agent accepts a change on the person's behalf, grants autonomy or records the person's decision; a change is applied only on the person's acceptance or within autonomy the person granted.
- **AC-004** — On every agent surface, agent-authored text that states a prohibited claim is blocked with a named diagnostic and is not shown or stored as captured rationale. This holds for the rationale, for an assumption and for copied context, and for a status-like token such as an approval or compliance status. Text that uses the same words without claiming authority, such as a reference to an approved document, is not blocked. This criterion applies from milestone R7 of PRD §24.
- **AC-005** — Every string, label and status the product itself puts on an agent surface is a registered statement of `docs/claims_registry.md` or makes no boundary statement, and none states acceptance of engineering work, approval, certification, sealing, authentication or code compliance. Each operation and outcome carries the boundary record with its fixed values, and one with an altered value is refused.
- **AC-006** — A missing rationale input, source, attribution or time appears as a visible finding. Unresolved assumptions are counted and shown to the person before Apply. None is dropped or filled with a default.
- **AC-007** — The report workflow passes no agent rationale to downstream runtime integrations, and no report infers human or professional approval from agent rationale or from an applied agent proposal. An export that carries rationale is treated as private project content under the export controls, and no public fixture, example or test holds anything but invented rationale.

## Production and Verification Method — Praxeology

**Method choices that define the control.**

- Structure over prose. Rationale, assumptions and attribution are separate members of the record and stay separate through review, application and history.
- The host, not the agent, sets everything that carries authority: the author stamp, the attribution, the boundary record and the statuses. An agent's arguments that supply any of these are refused.
- Blocking is visible. Blocked text is reported as blocked, with the reason; it is never silently removed or rewritten.
- Ambiguity resolves toward blocking: wording that can be read as a prohibited claim is blocked until a person changes it.

**How each criterion is examined.**

- **VER-001** — AC-001 and AC-003, executable check: observes the queue, the acceptance records and the model hash after agent proposals submitted with and without rationale, source and well-formed assumptions, and lists the tools and messages the agent can reach.
- **VER-002** — AC-002 and AC-006, executable check: applies, rejects and withdraws scripted agent proposals in the native application, saves and reopens, and compares the receipt and history members with the scripted rationale, assumptions and attribution; then removes each input in turn and observes the finding.
- **VER-003** — AC-004, executable check: feeds each agent surface a set of texts written from the prohibited claims of PRD §21.2 and the registry, placed in the rationale, in an assumption and in copied context, with a set of texts that use the same words without a claim, and observes what is blocked, shown and stored.
- **VER-004** — AC-005 and AC-007, judgment by a named role (a reviewer who did not write the surfaces): reads every product string, label and status on each agent surface against `docs/claims_registry.md` and PRD §21.2, each report and what the report workflow passes on for agent rationale and for approval inferred from it, and each export that carries rationale against the export controls, with an executable check that an operation or outcome with an altered boundary value is refused and that public fixtures hold invented rationale only.

## Governing Values and Decisions — Axiology

- **AX-001** — *Nothing reads as acceptance.* Better is fewer strings on agent surfaces that a reviewer can read as a prohibited claim; the target is none. That is never reached by silently deleting or rewording what the agent said: blocked text is shown as blocked.
- **AX-002** — *Rationale stays whole.* Better is a larger share of applied agent operations whose rationale, assumptions and attribution can be read back in structured form after reopening. Review is never made easier by sending rationale to a public export.
- **AX-003** — *A precise guard.* Better is fewer blocks of text that claims nothing, while every seeded prohibited claim is still blocked. Fewer false blocks are never bought by letting a claim through.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-070 OBJ-015 | REQ-002 | AC-001 | VER-001 | The scripted proposals state what each lacks and so what must be refused |
| OUT-001 | SOW-070 OBJ-015 | REQ-002 REQ-009 | AC-002 | VER-002 | The comparison is with the scripted rationale, assumptions and attribution, which the product did not write |
| OUT-001 | SOW-070 OBJ-015 OBJ-018 | REQ-003 REQ-006 | AC-003 | VER-001 | The criterion states the expected result: no acceptance record, no application and an unchanged hash, compared with the hash taken before; the list of tools and messages is read from the host's catalog |
| OUT-001 | SOW-070 OBJ-018 | REQ-004 REQ-005 REQ-010 | AC-004 | VER-003 | The seeded texts are written from PRD §21.2 and `docs/claims_registry.md`, not from the product's output |
| OUT-001 | SOW-070 OBJ-018 | REQ-004 REQ-005 REQ-006 | AC-005 | VER-004 | PRD §21.2 and `docs/claims_registry.md`, read by a reviewer who did not write the surfaces |
| OUT-001 | SOW-070 OBJ-015 | REQ-007 | AC-006 | VER-002 | The scripted proposals state which input is missing |
| OUT-001 | SOW-070 OBJ-018 | REQ-008 REQ-009 | AC-007 | VER-004 | `docs/IP_AND_DATA_BOUNDARY.md` and PRD §19.4, read by a reviewer who did not write the surfaces |
