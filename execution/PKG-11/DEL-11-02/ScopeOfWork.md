---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-11-02
package_id: PKG-11
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-033]
package_objective_refs: [OBJ-001, OBJ-002]
---

# Scope of Work — DEL-11-02 Developer guide for solver and rule packs

## Purpose and Objective Traceability

A developer who wants to extend the solver or the rule-pack machinery learns from one document where their code goes, what every change must pass through, what evidence a change needs before it is used, and what may never enter the public repository. The result this deliverable contributes is that the open mechanics can be inspected and extended by someone who did not build them, without the extension weakening the unit, provenance, diagnostic and data boundaries or reading as engineering approval.

It serves SOW-033 under OBJ-001 and OBJ-002 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-11, row DEL-11-02: documentation only), and PRD §9.5, whose open-source contributor needs a clear architecture, tests, a contribution policy, IP review gates and public examples that use lawful data only, with PRD §5.4, §5.5, §20.1 and §21.2. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable.

- **OUT-001** — The developer guide, `docs/developer_guide/`.

## Deliverable Definition — Ontology

**Terms the guide implements.**

- *Content groups.* The architecture map; the solver architecture; the rule-pack architecture; data, privacy and provenance; diagnostics and result envelopes; test discipline; contribution boundaries with the checklist a developer runs before proposing a change; and review and acceptance: how a change moves from draft evidence through review to acceptance, without the guide, the evidence or the review claiming engineering reliance.
- *Governing source.* The document or scope that defines what the guide describes: the layer map and dependency rule (DEL-00-02), the architecture decisions (DEL-00-01), the diagnostic record and result envelope (DEL-00-06), the adapter and plugin rule (DEL-00-07), the test strategy (DEL-00-08), the boundary policy (DEL-01-02), the model document (DEL-02-01), the unit system (DEL-02-02), the frame kernel (DEL-04-01), the rule-pack format (DEL-06-01), the expression evaluator (DEL-06-02) and the public API and plugin contract (DEL-10-01). The guide describes these and links them. Where the guide and a governing source differ, the governing source stands and the guide is the one to correct.
- *No-bypass rule.* No plugin, adapter, public API, runner or export path gets around schema validation, unit checks, provenance checks, diagnostics, the rule sandbox, report controls or the data boundary.
- *Stop rule.* What a developer does on meeting content that may be protected or private: stop, mark the concern, quarantine it by the boundary policy's route and ask for human review. It is never paraphrased into public content.
- *Evidence family.* The kind of deterministic evidence a change to a given surface needs, as the test strategy assigns it.

**Boundary.** The guide is for people changing the software. The following belong elsewhere:

- how to work in this repository, find a deliverable and hand a change over: DEL-11-05;
- what a user does with the application: DEL-11-01;
- the theory of the analyses: DEL-11-03;
- the contributor workflow, the certification and the issue forms: DEL-01-03;
- every rule, format and decision the guide describes: its governing source.

**Not decided.** The contributor legal instrument and the opening of external contribution intake. Both are the owner's, and the guide presents both as undecided.

## Completion and Reliance Basis — Epistemology

The guide is adequate when a developer new to the code can place a change, name what it must pass and what evidence it needs, and recognise what must not be contributed, and when nothing in the guide disagrees with a governing source.

### Requirements

REQ-001 to REQ-013 keep the numbers of the earlier `REQ-11-02` series.

- **REQ-001** — The guide explains the layered architecture: the layers, the responsibility of each, the direction of dependencies and the no-bypass rule for adapters (decomposition AB-00-02, AB-00-07; `docs/SPEC.md` §1).
- **REQ-002** — The guide describes the solver as a mechanics engine for a 3D centerline/frame model with six degrees of freedom per node and deterministic results. It presents no solid-model default and no code-compliance role (SOW-005; `docs/SPEC.md` §5; `docs/CONTRACT.md` OPS-K-MECH-1).
- **REQ-003** — The guide distinguishes solver mechanics from user-rule checking and from professional approval (`docs/CONTRACT.md` OPS-K-MECH-2, OPS-K-AUTH-1; `docs/TYPES.md` §4).
- **REQ-004** — The guide states that code-specific load combinations, code formulas, allowables, stress-intensification and flexibility factors and protected dimensional values are user-supplied, private or lawfully imported, and are never public defaults (`docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1).
- **REQ-005** — The guide describes what a rule pack carries: identity, version, source notice, redistribution status, checksum, required inputs, variables, checks and report notice (`docs/SPEC.md` §7; `docs/CONTRACT.md` OPS-K-RULE-3).
- **REQ-006** — The guide describes the evaluator boundary: declarative, sandboxed, unit-aware, deterministic and unable to execute arbitrary code (`docs/CONTRACT.md` OPS-K-RULE-2, OPS-K-UNIT-1).
- **REQ-007** — The guide requires unit-aware data flow through solver inputs, rule-pack variables, imports, exports, reports and tests (`docs/CONTRACT.md` OPS-K-UNIT-1).
- **REQ-008** — The guide explains structured diagnostics and result envelopes, covering findings that block a solve, findings that block a rule check, and provenance, assumption, nonlinear and IP-boundary warnings (decomposition AB-00-06; `docs/SPEC.md` §8).
- **REQ-009** — The guide states the test discipline: the evidence family for a change to the solver, to loads and stress recovery, to nonlinear supports, to rule packs and to reports, and the protected-content and provenance gates on public content (decomposition AB-00-08; `docs/VALIDATION_STRATEGY.md`; `docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-010** — The guide states the contribution boundaries: no protected standards text, tables, figures or examples; no private project data; provenance required for public data; and the stop rule for suspected protected content (`docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3, OPS-K-PRIV-1).
- **REQ-011** — The guide itself holds no protected code example or formula, no proprietary commercial example, and no invented value that could be taken for design guidance (SOW-033; `docs/CONTRACT.md` OPS-K-RULE-1).
- **REQ-012** — The guide chooses nothing. It states an architecture or tooling choice only as its governing source records it, and states an undecided one as undecided (`docs/CONTRACT.md` OPS-K-AGENT-1).
- **REQ-013** — The guide states that agent output, generated artifacts and software results are drafts or decision support until a person accepts them (`docs/CONTRACT.md` OPS-K-AGENT-4, OPS-K-AUTH-1).

### Delivery commitments

- **DEL-11-02:1** — The guide states that external contribution intake is closed and names no contributor legal instrument as adopted. Adopting one is the owner's act on legal advice (DEC-027, DEC-079), and the project licence does not settle it.

### Acceptance criteria

- **AC-001** — Every content group is present under a heading a reader can find from the guide's table of contents, and every link, module path, schema path and document path the guide names exists in the repository.
- **AC-002** — The layers the guide names, their responsibilities and the direction of dependencies are those of the layer map, and the guide's statement of the no-bypass rule omits none of the controls the adapter and plugin rule names.
- **AC-003** — The solver content describes a 3D centerline/frame model with six degrees of freedom per node and deterministic results, presents mechanics results, user-rule outcomes and human acceptance as three separate things, and nowhere describes the solver or a status as establishing code compliance.
- **AC-004** — The rule-pack content names every item of REQ-005 and agrees with the rule-pack format, and its description of the evaluator has every property of REQ-006 and agrees with the evaluator's sandbox boundary.
- **AC-005** — For each change area of REQ-009 the guide names the evidence a change needs, and each is an evidence kind the test strategy assigns to that layer.
- **AC-006** — The guide states each boundary of REQ-004 and REQ-010 and the stop rule with the boundary policy's quarantine route, and the guide's own text and examples hold nothing of the classes PRD §20.1 keeps out of the public repository and no value that resembles a design value.
- **AC-007** — Every architecture, format or tooling choice the guide states is recorded in its governing source in the same terms, every undecided one is written as undecided, contribution intake is stated as closed with no legal instrument named as adopted, and the guide says that agent and software output is draft until a person accepts it.
- **AC-008** — A developer who did not write the guide and has not worked in the code, given an invented change to the solver and one to the rule-pack machinery, names from the guide alone the layer the code belongs in, the controls it must pass and the evidence it needs, and each answer agrees with the governing sources.

## Production and Verification Method — Praxeology

**Method choices that define the guide.**

- *Describe and link.* The guide gives the developer the shape of each rule and sends them to its governing source. It does not restate a rule in full, so that it cannot drift into a second definition.
- *Organised by what a developer changes.* The solver and the rule-pack machinery each have their own part; the cross-cutting controls are stated once and referred to from both.
- *Paths that exist.* A module, schema or document is named by its repository path, so that a stale reference fails a check.
- *Process examples only.* An example in the guide shows a field map, a checklist or a quarantine record, with placeholder names and no engineering value.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the guide's headings, links and paths): every content group has its heading, every link resolves, and every repository path the guide names exists.
- **VER-002** — Comparison with a named reference (the governing sources): compare the guide's statements of the layers, the dependency direction, the no-bypass rule, the diagnostic record and result envelope, the rule-pack contents, the evaluator boundary, the evidence families and each stated choice with the governing source of each, and report every statement that differs or that the source does not make.
- **VER-003** — Judgment by a named role (an independent reviewer who did not write the guide): read the guide against PRD §20.1 and §21.2, the boundary policy of DEL-01-02, and DEC-027 and DEC-079, and report each protected or private item, each example value that resembles a design value, each missing boundary or stop rule, each statement that can be read as approval or compliance, and any invitation to contribute or instrument presented as adopted.
- **VER-004** — Judgment by a named role (a developer who did not write the guide and has not worked in the code): answer, from the guide alone, where two invented changes go, what each must pass and what evidence each needs. The reviewer of VER-002 compares the answers with the governing sources.

## Governing Values and Decisions — Axiology

- **AX-001** — *A newcomer places a change correctly.* Better is a larger share of the trial's questions answered correctly from the guide alone. Clarity is never bought by simplifying away a control: every item of the no-bypass rule and every stop rule stays.
- **AX-002** — *Point, don't restate.* Better is fewer statements in the guide that a governing source also makes in full, measured by the count of statements that disagree with their source, which is to be zero. Brevity never removes the path to the source.
- **AX-003** — *Mechanics open, values private.* Where an example would be more helpful with realistic engineering values, the invented placeholder is the better one. Helpfulness never justifies a protected formula, table or value, or a value that looks like one.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-002 REQ-005 REQ-008 REQ-009 REQ-010 REQ-013 | AC-001 | VER-001 | The content groups are SOW-033's developer documentation as this scope's requirements divide it. The headings are read from the guide, and every path is checked against the repository tree, which the guide does not write |
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-007 REQ-008 | AC-002 | VER-002 | Outside source: the layer map of DEL-00-02, the diagnostic and envelope contract of DEL-00-06 and the adapter and plugin rule of DEL-00-07, read from those documents and not from the guide |
| OUT-001 | SOW-033 OBJ-001 | REQ-002 REQ-003 | AC-003 | VER-002 VER-003 | Owner ruling: `docs/CONTRACT.md` OPS-K-MECH-1 and OPS-K-MECH-2 and PRD §21.2 |
| OUT-001 | SOW-033 OBJ-002 | REQ-005 REQ-006 | AC-004 | VER-002 | Outside source: the rule-pack format of DEL-06-01 and the evaluator boundary of DEL-06-02, read from their documents |
| OUT-001 | SOW-033 OBJ-001 | REQ-009 | AC-005 | VER-002 | Outside source: the test strategy of DEL-00-08 assigns the evidence kinds |
| OUT-001 | SOW-033 OBJ-002 | REQ-004 REQ-010 REQ-011 | AC-006 | VER-003 | Owner ruling: PRD §20.1 lists what the public repository may not contain; the quarantine route is DEL-01-02's policy |
| OUT-001 | SOW-033 OBJ-002 | REQ-012 REQ-013, DEL-11-02:1 | AC-007 | VER-002 VER-003 | Owner ruling: the decisions the governing sources record, and DEC-027 and DEC-079 for contribution intake |
| OUT-001 | SOW-033 OBJ-001 | REQ-001 REQ-009 | AC-008 | VER-004 | The invented changes say what must be answered; the expected answers are the governing sources', and the answers are given by someone who did not write the guide |
