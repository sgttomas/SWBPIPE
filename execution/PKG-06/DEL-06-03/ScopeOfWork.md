---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-03
package_id: PKG-06
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-004]
package_objective_refs: [OBJ-002, OBJ-005]
---

# Scope of Work — DEL-06-03 Required-input completeness checker

## Purpose and Objective Traceability

A user-rule outcome is reported only when everything the user's rule needs is really there. For each check, on each load case it applies to, the product binds the pack's declared inputs to the solved result, to the values the user enters and to the user's private library. An input that is missing, in an incompatible unit, without provenance or suspected of being protected content makes that outcome incomplete, with a finding that names the input, and the check is not evaluated there. A missing rule value never stops or alters the mechanics solve and is never replaced by a default. The result this deliverable contributes is that code-specific data is always the user's own, visibly, and that no outcome is ever computed on data that was not supplied.

It serves SOW-004 under OBJ-002 and OBJ-005 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-06, row DEL-06-03), and `docs/PRD.md` §5.4, §5.5, §17.5, FR-RULE-003, §14.4 and §10.6 step 5, with `docs/CONTRACT.md` OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3 and OPS-K-IP-3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. Under the owner's ruling on rule packs there (OC-6), a user-rule outcome is shown as within or exceeding the user's limit, so that no outcome reads as compliance.

- **OUT-001** — The required-input completeness gate of a rule-check run: the binding of a pack's declared inputs to solved results, entered values and private library values; the finding for each input that is missing, unit-incompatible, unprovenanced or suspected protected; the outcome of each check on each basis; the pack aggregate; and the run result that carries them.

## Deliverable Definition — Ontology

**Terms the run implements.** A basis, the basis plan and the row selector are defined by the pack format (DEL-06-01).

- *Rule-check run.* The application of one validated pack to one solved result of one model. A pack with a blocking validation finding is not run: its result has no outcomes and is incomplete.
- *Binding a solver result.* On each basis, a solver-result input binds the one row its selector selects among the rows of that basis. No row, or more than one, leaves the input unsupplied on that basis. The pack's own selector is the only route: a row selection supplied by the caller is refused by name.
- *Eligibility.* A row is bound only from a result whose mechanics are solved and that the result reader finds numerically eligible, and only if the reader does not refuse that row for rule binding. A refused row leaves the input unsupplied on that basis, with the reader's reason. A row without a basis is never bound, and under a combination the row bound is the combination's result, never an operand of it.
- *Entered values.* A user-supplied rule value, and the limit a value slot names, are entered by the user for the run, each with its unit. They are the same on every basis.
- *Private library value.* Read at the run from the slot the pack's reference names, in the user's local library. The value is never in the pack. A reference that cannot be resolved leaves the input unsupplied.
- *Required load case availability.* A required load case that the model does not hold is missing. One that the model holds but the solved result has no rows for is unsolved. Either makes every check mapped to it incomplete on that basis.
- *Complete input.* An input supplied with a value, a unit of the declared dimension, accepted provenance, a redistribution status and an accepted review standing, and not suspected of being protected content. Each defect is a blocking finding that names the input.
- *Declared unit.* A bound value in another unit of the declared dimension is converted to the declared unit through the unit catalog before evaluation. A value that cannot be converted blocks.
- *Acceptability.* A formula that yields a truth value is the outcome. A formula that yields a quantity is compared with the user-supplied limit that the check's value slots name, by the check's relation, through the evaluator. An unrecognized relation blocks; it is never read as less-than-or-equal.
- *Outcome.* One for each check on each of its bases, in check order and then mapping order: within the user's limit, exceeding it, or incomplete. The run result carries these as the status tokens `USER_RULE_CHECKED`, `USER_RULE_FAILED` and `RULE_INPUTS_INCOMPLETE`. An outcome holds its category, its basis, the computed value, the limit, the relation, the bound inputs and the findings of that basis alone. Nothing is merged across bases.
- *Aggregate.* Incomplete if any outcome is incomplete; otherwise exceeding if any outcome exceeds; otherwise within. Its counts are counts of outcomes, so one check can contribute more than once. The pack never headlines within or exceeding while any outcome is incomplete, and an exceeding outcome stays visible under an incomplete aggregate.
- *Rule-check blocking.* The class of a missing rule input. It is distinct from solve blocking, the class of a missing physical input, and it never changes the mechanics status.

**Undecided.** The words a person is shown for the outcome tokens are the display forms of the claims registry (DEL-01-04), where the form that follows the owner's ruling is undecided. Whether the run's notice must be a registered statement is also undecided: AC-011 requires one, and the registry holds none for the rule-check run notice, which keeps text of its own.

**Boundary.** This deliverable decides whether a check may be evaluated and reports its outcome. The pack format and its validation are DEL-06-01's. Evaluating a formula is DEL-06-02's. The pack's lifecycle findings and checksum are DEL-06-04's. Solve-required physical inputs and their diagnostics are DEL-04-06's. The status vocabulary is DEL-02-03's and DEL-05-04's. How findings and outcomes are shown is DEL-07-04's and DEL-07-05's, how they are reported is DEL-08-01's, and the rule status an analysis run records is DEL-14-02's. No public artifact of this deliverable holds a code formula, allowable or default.

## Completion and Reliance Basis — Epistemology

A complete input is a supplied and sourced one. Completeness establishes nothing about whether the value is right, and an outcome is a computation on the user's data, never a determination of compliance.

### Requirements

Requirement numbers 001 to 007 are those of the earlier `R-DEL-06-03-0nn` series.

- **REQ-001** — A missing rule-check-required value is an explicit finding, never a silent default.
- **REQ-002** — Code-specific and project-specific values are supplied by the user or imported privately. None is bundled as a public default.
- **REQ-003** — Rule-pack values, materials, components, SIF and flexibility factors, allowables and related inputs carry provenance where they are relied upon.
- **REQ-004** — Completeness is machine-checkable from the pack's declarative input declarations, not from arbitrary executable code.
- **REQ-005** — A missing rule-pack input maps to a rule-check-blocking condition, distinct from a solve-blocking physical input.
- **REQ-006** — The run asserts no code compliance, certification, approval, sealing or professional reliance (PRD §21.2).
- **REQ-007** — Suspected protected or proprietary data is surfaced, blocks and is escalated for human review; it is not turned into public data (`docs/CONTRACT.md` OPS-K-IP-3).
- **REQ-008** — No user-rule outcome, within or exceeding the user's limit, is reported for a check on a basis whose required values are not all present, and the pack aggregate is incomplete while any outcome is incomplete (FR-RULE-003; PRD §17.5).
- **REQ-009** — A run yields one outcome for each check on each load case or combination its category maps, and realizes DEL-06-01/REQ-013: a basis that is absent from the model, unsolved or lacking a row the check needs is incomplete and not evaluated, and the check's other outcomes are reported independently.
- **REQ-010** — Where the pack declares a dimension or unit, binding is unit-aware: a bound value is in its declared unit, or is converted to it through the unit catalog, before it is used.
- **REQ-011** — Every finding of a run carries the governed diagnostic fields: code, class, severity, source, affected object, message, remediation and provenance (also cited as DEL-06-03:1 and DEL-06-02:2).
- **REQ-012** — The fixtures of this deliverable and the invented example pack are reviewed for public and default data (also cited as DEL-06-03:1).

### Acceptance criteria

- **AC-001** — For each way an input can be incomplete (no value, no unit, another dimension, a unit that cannot be converted, provenance missing or unknown, redistribution status missing, review pending or rejected, protected content suspected), a run on a constructed pack gives an incomplete outcome with a blocking finding that names the input and the reason. Nothing is evaluated on that basis and no value is substituted. The finding for suspected protected content marks the input for human review.
- **AC-002** — A run on a solved model with a rule input missing leaves the mechanics status and every mechanics result unchanged, and the finding's class is rule-check blocking, never solve blocking.
- **AC-003** — On an invented model with two load cases and a combination: a check mapped to two load cases yields two outcomes, each bound to its own basis's row; a check mapped to the combination binds the combination's row; and a required load case that is absent from the model, has no result rows, has no row the selector selects or has more than one makes that outcome incomplete with its named finding while the check's other outcomes are reported. No value, input or finding appears on a basis it did not come from.
- **AC-004** — A row the result reader refuses for rule binding is never bound, a result whose mechanics are not solved or that is not numerically eligible yields no outcome that is within or exceeding, and a row selection supplied by the caller is refused by name.
- **AC-005** — A value bound in another catalog unit of the declared dimension gives the same computed value and outcome as the same quantity entered in the declared unit. For each of the four relations, a computed value below, at and above the limit gives the outcome the relation defines, and an unrecognized relation blocks.
- **AC-006** — A private-library input resolved from the user's local library gives the outcome its value implies, the value appears in no pack document, and an unresolvable reference leaves the input unsupplied and the outcome incomplete.
- **AC-007** — The aggregate is incomplete when any outcome is incomplete, otherwise exceeding when any outcome exceeds, otherwise within, and the counts equal the number of outcomes of each kind. A check that exceeds on one basis and is incomplete on another is counted once in each.
- **AC-008** — A pack with a blocking validation finding yields a run result with no outcomes and an incomplete aggregate.
- **AC-009** — Outcomes are identified by check and basis, and a rerun and a reordering of the solved result's rows give the same outcomes in the same order.
- **AC-010** — A run result validates against its schema, and every finding in it carries a code, class, severity, source, affected object, message and remediation, with provenance where it concerns a sourced value.
- **AC-011** — The statuses a run can emit are the three outcome tokens. No status, finding, message or notice of a run states or implies a pass, compliance, certification or approval, and its notice is a statement of the claims registry.
- **AC-012** — The fixtures of this deliverable and the invented example pack hold no code-specific or protected value and no default for a rule input.

## Production and Verification Method — Praxeology

**Method choices that define the run.**

- Completeness is decided before evaluation, from declarations and bindings alone. A check that is not complete on a basis is never evaluated there.
- One binding route for solver results: the selector the pack's author wrote. Nothing is bound by position or by a row identifier chosen at run time.
- Each basis is evaluated with only that basis's values, refusals and findings.
- Library values are resolved at the run from the user's local library and travel in no pack.
- The run is pure computation over the pack, the model, the solved result and the supplied values; it reads and writes nothing else.

**How each criterion is examined.**

- **VER-001** — AC-001, AC-002 and AC-008, executable check: runs constructed packs with each defect of an input and with a blocking validation finding, and observes the outcome, the finding, the absence of an evaluated value and the mechanics status before and after.
- **VER-002** — AC-003, AC-004 and AC-009, executable check: runs packs on an invented two-case model with a combination and on solved results that hold refused rows, tie rods and held operands, and observes each outcome's basis, bound row and findings, on a rerun and with the rows reordered.
- **VER-003** — AC-005 and AC-007, comparison with a named reference: outcomes and aggregates worked by hand from the four ordering relations and the units' public definitions on invented values.
- **VER-004** — AC-006, executable check: runs a pack with a private-library input against a local library that holds the record, and against one that does not, and searches the pack document for the value.
- **VER-005** — AC-010, executable check: validates run results against the run-result schema under a validator that imports no product code, and observes the fields of every finding.
- **VER-006** — AC-011 and AC-012, judgment by a reviewer who did not write the run, with the claims-language and protected-content scans: examines the status, finding and notice texts against `docs/claims_registry.md`, and the fixtures and the example pack for code-specific or default data.

## Governing Values and Decisions — Axiology

- **AX-001** — *Incomplete before wrong.* Better is fewer outcomes reported as within or exceeding on data that was absent, ambiguous or unvetted; the target is none. Convenience never fills a value, chooses among ambiguous rows or relaxes a provenance requirement.
- **AX-002** — *Say what to supply.* Better is a larger share of incomplete outcomes whose findings name the input, the basis and what the user can supply. A finding never suggests an engineering value.
- **AX-003** — *The mechanics stand alone.* Better is fewer paths by which a rule run can read back into or hold up a solve; the target is none. A gap in rule data never blocks, delays or changes a mechanics result, however complete the rule check would otherwise look.
- **AX-004** — *An exceeding outcome is never hidden.* Better is more of each outcome's detail kept separate by check and load case. Outcomes are never collapsed across load cases, and an incomplete aggregate never removes an exceeding outcome from view.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-004 OBJ-002 OBJ-005 | REQ-001 REQ-003 REQ-004 REQ-007 REQ-008 | AC-001 | VER-001 | Owner ruling: PRD §5.5, §17.5 and FR-RULE-003. The packs are constructed, so the criterion states the expected outcome and finding |
| OUT-001 | SOW-004 OBJ-005 | REQ-005 | AC-002 | VER-001 | Owner ruling: PRD §14.4 and §17.5. The mechanics result compared is the solve's own, which this deliverable did not write |
| OUT-001 | SOW-004 OBJ-005 | REQ-008 REQ-009 | AC-003 | VER-002 | Owner ruling: one outcome for each check on each mapped load case or combination, adopted by the owner under PRD §17.2 (load-case mappings) and FR-RULE-003. DEL-06-01/REQ-013 holds the shared definition. The model and packs are constructed, and the criterion states each expected outcome |
| OUT-001 | SOW-004 OBJ-005 | REQ-001 REQ-009 | AC-004 | VER-002 | The refusal and the eligibility verdict are the result reader's, which this deliverable did not write; the criterion states that neither is bound |
| OUT-001 | SOW-004 OBJ-005 | REQ-010 | AC-005 | VER-003 | Outside source: the units' public definitions and the ordering relations, a closed form on invented values. The relation set is the owner's ruling DEC-039 |
| OUT-001 | SOW-004 OBJ-002 | REQ-002 | AC-006 | VER-004 | Owner ruling: DEC-038, a library value is referenced and never embedded. The library and pack are constructed |
| OUT-001 | SOW-004 OBJ-005 | REQ-008 | AC-007 | VER-003 | Owner ruling: the owner's reading of FR-RULE-003, stated in the aggregate's definition. The expected aggregate is fixed by that rule for each constructed set of outcomes |
| OUT-001 | SOW-004 OBJ-005 | REQ-008 | AC-008 | VER-001 | Owner ruling: PRD §10.6 steps 3 and 5, a pack is validated before it is applied, and §17.5, no outcome without the required values. The pack is constructed, so the criterion states the expected result |
| OUT-001 | SOW-004 OBJ-005 | REQ-009 | AC-009 | VER-002 | The criterion fixes the expected result: equality between runs and row orders |
| OUT-001 | SOW-004 OBJ-005 | REQ-011 | AC-010 | VER-005 | Owner ruling: the decomposition's AB-00-06 names the fields. Outside source for the schema check: the JSON Schema 2020-12 specification, applied without product code |
| OUT-001 | SOW-004 OBJ-005 | REQ-006 | AC-011 | VER-006 | Owner ruling: PRD §21.2, the rule-pack ruling OC-6 and `docs/claims_registry.md` |
| OUT-001 | SOW-004 OBJ-002 | REQ-002 REQ-012 | AC-012 | VER-006 | Owner ruling: PRD §5.4 and §20.1. The reviewer compares against sources the product did not write |
