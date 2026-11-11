---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-04
package_id: PKG-07
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-022]
package_objective_refs: [OBJ-006, OBJ-011]
---

# Scope of Work — DEL-07-04 Missing-data warning and blocking UX

## Purpose and Objective Traceability

This deliverable is how the desktop tells the engineer what is missing, what that stops, and what to do about it. A value the solve needs and lacks stops the solve. A value only the user's rule check needs stops or qualifies the check and leaves a valid mechanics result standing. Weak provenance, assumptions, nonlinear uncertainty and suspected protected content are shown as what they are. Nothing is filled in silently, and no finding or status reads as approval. The deliverable also holds the surface where the user runs a rule check and reads its outcome. The result it contributes is that an engineer reviewing a model or a result can tell, without reading a log, which inputs are absent, which conclusions that withholds, and where to go to supply them.

It serves SOW-022 under OBJ-006 and OBJ-011 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-07, row DEL-07-04), and PRD §5.5, §14.4, §17.5, §21.2 and FR-RULE-003. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. The release target's ruling that a user-rule outcome is shown as within or exceeding the user's limit applies to the rule-check surface held here.

- **OUT-001** — The presentation of findings: their classes, what each blocks, their fields, and the way from a finding to the object it concerns.
- **OUT-002** — The rule-check run surface: starting a run, its outcomes and its aggregate status.

## Deliverable Definition — Ontology

**Terms the presentation implements.**

- *Finding.* A diagnostic a producer returns about the model, a solve, a rule check, an import or an export. A finding has a code, a class, a severity, a source, an affected object, a message, a remediation and, where it applies, provenance. This deliverable shows findings; it produces none of them.
- *Class.* One of the six classes every producer keeps: `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING` and `IP_BOUNDARY_WARNING`.
- *What a finding blocks.* PRD §14.4 separates a finding that blocks model validity, one that blocks the solve, one that blocks the user-rule check, a warning and an informational note. The presentation states which of these a finding is, from its class and severity as supplied.
- *Solve-required and rule-check-required.* A solve-required value is one the mechanics solve cannot run without. A rule-check-required value is one only the user's rule pack asks for. The producer of the finding says which; the presentation keeps them apart.
- *Qualified result.* A result or outcome shown together with the findings that limit it, where it is shown at all.
- *Rule-check run.* One evaluation of a rule pack the user chose over a solved result. It returns one outcome for each check on each load case or combination, and one aggregate status for the pack.
- *Outcome.* For one check on one load case or combination: within the user's limit, exceeding the user's limit, or incomplete, with the computed value, the limit, their unit and the findings that explain an incomplete one.
- *Status.* One of the automatic statuses a model, a result or a rule check carries: model incomplete, mechanics solved, rule inputs incomplete, user-rule checked, user-rule failed, human review required. Each is shown in the display form the claims registry gives it.
- *Affected-object navigation.* Activating a finding selects the object it concerns in the tree, the viewport and the inspector.

**Boundary.** This deliverable presents. The following belong elsewhere:

- the fields and classes of a diagnostic: DEL-00-06;
- which values a solve requires, and the solver's and the model checks' findings: DEL-04-06 and the engine deliverables that emit them;
- nonlinear convergence and active-state findings: DEL-04-04. The presentation shows a supplied nonlinear finding as a nonlinear warning and forms no outcome of its own;
- rule-pack semantics, the completeness check and the runner: DEL-06-01, DEL-06-02 and DEL-06-03;
- what each status means: DEL-05-04. The words each status is shown in: DEL-01-04;
- unit conversion for display: DEL-02-02;
- detection of protected or private content and the review that releases a held export: DEL-12-02 for exports and DEL-08-05 for reports;
- the diagnostics panel of a solve and the gate on the Solve command: DEL-07-07, which follows the presentation rules stated here;
- values and ratios in the results tables: DEL-07-05.

**Not decided.** Whether the registered display form of the user-rule-failed status changes to follow the ruling that an outcome is shown as within or exceeding the user's limit. The registry is DEL-01-04's. Until it is decided, REQ-016 governs the wording of an outcome and the registry governs the wording of a status.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Every finding is shown with its class, and the six classes stay distinct on every surface that shows a finding. No two are merged into one list entry, colour or count.
- **REQ-002** — A missing solve-required value blocks the solve of the affected model, or marks an existing result of it as no longer current. No surface supplies a default for it (PRD §5.5).
- **REQ-003** — A missing rule-check-required value blocks or qualifies the rule-check outcome it bears on and leaves the mechanics result shown as solved (PRD §17.5).
- **REQ-004** — A finding is shown with its code, class, severity, source, affected object, message, remediation and provenance. A field the producer did not supply is shown as not supplied and is never invented. A quantity a finding carries is shown with its unit, in the display unit the user chose without changing the stored value; a unit finding (a missing unit, a wrong dimension, a unit that cannot be converted) is shown as a finding that names the value concerned.
- **REQ-005** — Model incomplete, mechanics solved, rule inputs incomplete, user-rule checked, user-rule failed and human review required are shown as distinct statuses, each in its registered display form with its authority domain and its token reachable. No surface shows a status of code compliance or approval, and none derives one from the others.
- **REQ-006** — A value that exists with missing or weak provenance is shown as a provenance warning beside the value it concerns.
- **REQ-007** — An assumption is shown as an assumption wherever the result it affects is shown. Its presence is never presented as acceptance of it.
- **REQ-008** — Nonlinear uncertainty a producer reports is shown as a nonlinear warning on the result it qualifies. The presentation forms no convergence or support-state outcome of its own (also cited as DEL-07-04:1).
- **REQ-009** — Where an export, report or contribution flow returns a suspected protected or private content finding, it is shown as an IP-boundary warning, the flow is shown in the state its owner returned (blocked, redacted or held for review), and the surface offers no way past a block or a hold other than the owning flow's review.
- **REQ-010** — No text of these surfaces states or implies certification, sealing, approval or code compliance (PRD §21.2).
- **REQ-011** — A change that affects readiness is made only by an applied model operation. Findings follow the model: after an apply, an undo or a redo, the findings shown are those of the model as it then stands.
- **REQ-013** — A finding's class, severity, affected object, message and remediation are available as text to assistive technology, and no class, severity or status is signalled by colour alone.
- **REQ-014** — From a finding the user reaches the object it concerns, by keyboard as by pointer. A finding that names several objects offers each. A finding that names none says so.
- **REQ-015** — *Rule-check run.* The user starts a run by choosing a rule pack and a solved result. The surface shows the pack's identity, version and checksum, each outcome as the runner returned it, and the aggregate status. An incomplete outcome is shown as incomplete with the findings that explain it, never as zero, as within the limit or as absent. Values are shown in the display unit the user chose without changing a stored value.
- **REQ-016** — A user-rule outcome is worded as within or exceeding the user's limit. It is never worded as pass or fail (`docs/RELEASE_TARGET_v1.0.md`, owner calls; PRD FR-RULE-003).

### Acceptance criteria

- **AC-001** — For an invented model that lacks one solve-required value: the finding is shown as solve blocking with every supplied field, the Solve command refuses and names the finding, and the model document is byte for byte what it was. Supplying the value by an applied operation removes the finding; undo brings back the same finding.
- **AC-002** — For a solved invented model and an invented rule pack whose limits the fixture sets by hand: the run surface shows, for every check and load case, the outcome, computed value, limit and unit the runner's envelope carries, and the aggregate status it carries, with nothing added and nothing omitted.
- **AC-003** — For the same model and a pack that lacks one required input: the mechanics result is still shown as solved, the affected outcomes are shown as incomplete with a rule-check-blocking finding that names the missing input, and no ratio is shown for them.
- **AC-004** — For one invented finding of each of the six classes, supplied once with every field and once without remediation and provenance: every supplied field is shown, each absent field reads as not supplied, and the six are distinguishable without colour. An invented finding that carries a quantity shows it with its unit, and in a display unit the user chose with the stored value unchanged; an invented unit finding is shown as a finding that names the value concerned.
- **AC-005** — No text of the finding surfaces or of the run surface words an outcome as pass or fail, and none states or implies a claim PRD §21.2 prohibits or carries a phrasing the claims registry withdraws or retires. Each status is shown in its registered display form with its authority domain and its token reachable.
- **AC-006** — Every finding's class, severity, affected object, message and remediation can be read from the accessibility tree, and activating a finding by keyboard selects its affected object in the tree, the viewport and the inspector. A finding that names two objects offers both, and one that names none says so.
- **AC-007** — For an invented result supplied with a provenance finding, an assumption and a nonlinear finding: each is shown beside the value or result it concerns, and no surface shows the assumption as accepted or shows a convergence or support state the producer did not supply.
- **AC-008** — For an invented export finding and an invented report finding of the IP-boundary class: each flow is shown in the state its owner returned, and no control of these surfaces releases it.
- **AC-009** — For a solved invented model, after an applied operation that removes a solve-required value, the existing result is shown as no longer current.

## Production and Verification Method — Praxeology

**Method choices.** The finding presentation is one set of rules (the fields shown, the reading of an absent field, the non-colour class marker and the way to the affected object) that every surface showing a finding follows; this scope states them once. The run surface calls the rule-check service and shows its envelope; it computes no outcome. Status words come from the label table that carries the claims registry.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the desktop session with an invented model): opens a model that lacks a solve-required value, reads the finding surface and the Solve command, applies the operation that supplies the value, then undoes it, and compares the model document's bytes and the findings at each step; then solves, applies an operation that removes the value and reads how the existing result is marked.
- **VER-002** — Executable check (observes the run surface against the runner's envelope): runs the invented pack over the solved invented model, with and without the required input, and compares every displayed outcome, value, limit, unit and status with the envelope the runner returned.
- **VER-003** — Executable check (observes the rendered finding surfaces): renders one finding of each class with and without the optional fields, a finding that carries a quantity and a unit finding, and reads the displayed fields, units and the non-colour marker of each class.
- **VER-004** — Comparison with a named reference (the prohibited claims of PRD §21.2, the claims registry's display-form table and its withdrawn and retired phrasings, and the release target's wording of a user-rule outcome): reads every text the finding surfaces and the run surface can show and compares it with them.
- **VER-005** — Executable check (observes the accessibility tree and keyboard input): reads each finding's fields from the tree and activates a finding by keyboard, then reads the session's selection; repeats with a finding that names two objects and one that names none.
- **VER-006** — Executable check (observes the result and flow surfaces with supplied findings): supplies a provenance finding, an assumption and a nonlinear finding with an invented result, and an IP-boundary finding with an export and with a report, and reads where each is shown, what state each flow shows and which controls are offered.

## Governing Values and Decisions — Axiology

- **AX-001** — An absent value stays absent. Better is a blocked action with a named reason, measured by the model document being unchanged after every refusal, never a result produced from a value the user did not enter.
- **AX-002** — A finding says no more than its producer said. Better is a field shown as not supplied, measured by the displayed fields equalling the supplied ones, never a remediation, cause or outcome composed by the presentation.
- **AX-003** — A status is not a judgment of the design. Better is a mechanics result, a rule-check outcome and the need for human review each shown as its own fact, never one combined verdict.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-022 OBJ-006 | REQ-001 REQ-002 REQ-011 | AC-001 | VER-001 | Owner ruling: PRD §5.5 says a missing solve-required value blocks and is never defaulted. The expected finding is the one the model checks return for the value the fixture omits; the expected document is the fixture file itself. |
| OUT-002 | SOW-022 OBJ-006 OBJ-011 | REQ-005 REQ-015 | AC-002 | VER-002 | The runner's envelope is the reference and the limits are set by hand in the fixture. This shows the surface alters nothing; whether the outcomes are right is DEL-06-03's to show. |
| OUT-002 | SOW-022 OBJ-011 | REQ-003 REQ-015 | AC-003 | VER-002 | Owner ruling: PRD §17.5 says missing rule inputs leave the mechanics result valid and the check incomplete. The missing input is the one the fixture removes. |
| OUT-001 | SOW-022 OBJ-006 | REQ-001 REQ-004 REQ-015 | AC-004 | VER-003 | Owner ruling: the decomposition's architecture basis AB-00-06, stated in DEL-00-06, fixes the fields and the six classes, and PRD §14.4 the distinctions. The expected display is the finding as the check supplies it; the units of a dimension are the unit catalog's (DEL-02-02). |
| OUT-001 | SOW-022 OBJ-011 | REQ-005 REQ-010 REQ-016 | AC-005 | VER-004 | Owner ruling: the release target's wording of a user-rule outcome and PRD §21.2. The display forms, authority domains and withdrawn phrasings are DEL-01-04's registry, which this deliverable does not write. |
| OUT-001 | SOW-022 OBJ-006 | REQ-013 REQ-014 | AC-006 | VER-005 | Owner ruling: the desktop's accessibility baseline (DEL-07-06) requires a keyboard alternative and a signal other than colour. The expected selection is the finding's own affected object. |
| OUT-001 | SOW-022 OBJ-006 | REQ-006 REQ-007 REQ-008 | AC-007 | VER-006 | The supplied findings say what must be shown and where; they are written by hand for the check, not by the presentation. PRD §14.4 is the owner ruling that each stays visible. |
| OUT-001 | SOW-022 OBJ-011 | REQ-009 | AC-008 | VER-006 | The flow's state is its owner's (DEL-12-02 for exports, DEL-08-05 for reports), supplied by hand to the check; the expected count of releasing controls is zero. |
| OUT-001 | SOW-022 OBJ-006 | REQ-002 | AC-009 | VER-001 | Owner ruling: PRD §5.5. The removed value is the one the check removes, and the expected marking is the requirement's. |
