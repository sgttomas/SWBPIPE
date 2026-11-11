---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-05
package_id: PKG-09
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-026, SOW-027]
package_objective_refs: [OBJ-008]
---

# Scope of Work — DEL-09-05 Release quality gate checklist

## Purpose and Objective Traceability

This deliverable defines the release quality gates: for each kind of change, the software evidence that must exist before the change is released, and the record that shows, criterion by criterion, whether it does. A change to the solver must carry benchmark and regression evidence; a change to the rule engine, the GUI or a report template must carry its own. The gates make that routing explicit, keep a criterion with no decided threshold from being read as passed, and keep the maintainer's acceptance of a release distinct from any engineer's acceptance of a calculation.

It serves SOW-026 and SOW-027 under OBJ-008 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-09, row DEL-09-05) and the per-release automated tests of PRD §22.4. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. The exit criteria are the definition of done for v1.0; the gates route changes and record evidence on the way there and add no exit criterion.

- **OUT-001** — The release quality gates: the checklist `docs/RELEASE_QUALITY_GATES.md` that routes solver, rule-engine, GUI, report-template and mixed changes to their required evidence; the commit-bound gate records; the release-readiness check; the release-candidate evidence sweep; and coverage telemetry.

## Deliverable Definition — Ontology

**Terms the gates implement.**

- *Gate family.* One of four, by what a change touches. *Solver*: stiffness, loads, numerical solve behaviour, diagnostics, units, result envelopes or stress recovery. *Rule engine*: the rule-pack schema, expression evaluation, required-input handling, sandboxing, or checksum and provenance semantics. *GUI*: modelling, editing, solve execution, warnings, results or the report workflow. *Report template*: public report templates, examples, export manifests, notices or reproducibility records. A *mixed* change touches more than one and takes the union of their gates.
- *Required evidence.* What a family's gate demands, stated in REQ-002 to REQ-005.
- *Gate record.* For one commit and one family: each criterion with its outcome and the evidence the outcome rests on, together with the commands run and their results, benchmark source provenance, known limitations, open risks, undecided items and the governance decision or waiver.
- *Outcome.* `PASS`: the required evidence is present and the applicable checks pass. `FAIL`: the evidence exists and shows a failure. `BLOCKED_TBD`: a required threshold, authority decision or evidence source is not decided or not available. `HUMAN_REVIEW_REQUIRED`: a protected-content, private-data, professional-boundary, release-label or waiver question needs a person's disposition.
- *Waiver.* The release authority's explicit record that a gate item will not run: the item, the reason, the release scope it covers, any compensating evidence, the risk and limitation text.
- *Release authority.* The sole maintainer. The release authority's acceptance of a release is project governance. It is not professional approval of any piping calculation, model, rule-pack result or report.
- *Release-readiness check.* A local check that runs a named profile of the project's checks without any hosted service and reports each step's result. Its security profile runs the private-data and real-secret checks.
- *Evidence sweep.* One deterministic entry point that runs five surfaces at one commit, in a fixed order (the Cargo crates, the Python tests, the desktop unit tests, the desktop end-to-end tests and the desktop production build), and writes a machine-readable summary bound to that commit that records whether the tree was clean and whether every surface passed.
- *Coverage telemetry.* A commit-bound record of test coverage per lane (Rust, TypeScript, Python). It is recorded and never blocks.

**Boundary.** The gates say what evidence is required and record whether it exists. They produce none of it.

- The benchmark and regression suites are DEL-09-01's, DEL-09-02's and DEL-09-03's; the validation manual and the reproduction bundle are DEL-09-04's.
- Hosted checks, the merge check, the build, packaging, signing and the release artifact record are DEL-10-04's. The protected-content linter and the release-candidate content scan with its owner sign-off are DEL-08-05's. No deliverable currently commits that the hosted desktop end-to-end run executes the registered specs across both registered viewport projects and records which projects and specs it executed, which REQ-014 reads. The performance harness and its limits are DEL-04-05's. Release wording draws on DEL-01-04's claims registry.
- The gates set no tolerance, convergence limit, performance limit or coverage floor. They read the governed values and record a criterion whose value is undecided as `BLOCKED_TBD`.
- Not decided: the vocabulary of release labels. Until the owner decides it, the gates mint no label.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Every release-impacting change is routed to one or more gate families: solver, rule engine, GUI, report template, or mixed. Where the classification is uncertain the change takes the broader set, and the uncertainty is recorded in the gate record.
- **REQ-002** — The solver gate requires deterministic verification and regression evidence for the mechanics, stress recovery and nonlinear behaviour the change affects, for unit-aware calculation, for numerical-quality diagnostics and for warning and result-envelope behaviour, before release use (`docs/CONTRACT.md` OPS-K-SOLVER-1; PRD §22.2 to §22.4).
- **REQ-003** — The rule-engine gate requires evidence of sandboxing with no arbitrary code execution, unit awareness, deterministic evaluation, required-input completeness, invented example data, and preserved rule-pack checksum and provenance (`docs/CONTRACT.md` OPS-K-RULE-1 to OPS-K-RULE-3).
- **REQ-004** — The GUI gate requires workflow evidence that missing solve data, missing rule-check data, weak provenance, assumptions, nonlinear uncertainty and IP-boundary warnings stay visible and are never collapsed into a generic success state, and that a mechanics solve, a user-rule check and professional approval are never presented as the same state (PRD §14.4); and, where GUI surfaces are affected, accessibility and usability evidence and regression evidence for the model editing, solve execution, result viewing, warning and report preview and export workflows the release touches.
- **REQ-005** — The report-template gate requires evidence of reproducibility, checksum stability, inclusion of warnings, assumptions and limitations, rule-pack references and provenance disclosure, the professional-boundary notice and protected-content lint (PRD §19; `docs/IP_AND_DATA_BOUNDARY.md`).
- **REQ-006** — The checklist keeps mechanics verification, workflow validation, user-rule checking and professional review distinct.
- **REQ-007** — A release candidate is not labelled engineering beta unless the release-gate evidence `docs/VALIDATION_STRATEGY.md` §4 requires is present and its open risks are listed and accepted by the release authority.
- **REQ-008** — No public gate artifact contains protected standards text, copied code formulas, protected examples, material allowables, SIF or flexibility tables, protected dimensional tables, proprietary commercial data or private user data (`docs/CONTRACT.md` OPS-K-IP-1).
- **REQ-009** — A threshold or release-authority decision that the owner has not made stays undecided. The gates select none, and a criterion that depends on one is `BLOCKED_TBD`.
- **REQ-010** — A gate record identifies the commands run and their results, the benchmark source provenance, the known limitations, the open risks, the undecided items and the governance decision the record awaits or carries.
- **REQ-011** — A mixed change runs the union of the applicable gate families unless the release authority records a waiver with its risk disposition. No waiver authorizes copying protected content, exposing private data, or a certification, code-compliance or professional-reliance claim. A release does not proceed while evidence a gate requires is missing, unless the release authority records its acceptance of that risk for the release's maturity label.
- **REQ-012** — A release-governance record lists the validation status, known limitations, open risks, undecided items and the professional-boundary notice. Release labels describe software maturity and validation evidence only, with no reliance or compliance language (PRD §21.2).
- **REQ-013** — For a release, the gates show an automated test result for each behaviour PRD §22.4 lists: unit conversion; schema round-trip; matrix assembly; coordinate transformation; boundary conditions; load application; thermal strain; force recovery; stress recovery; rule-pack evaluation; nonlinear convergence where applicable; model state hashing; analysis run reproducibility; comparison output determinism; report reproducibility.
- **REQ-014** — The evidence sweep is the commit-bound evidence a release candidate's authenticity record consumes. Its end-to-end surface may be satisfied by a hosted run only when the summary records that the surface ran in CI and binds the workflow, the run and its attempt, a head commit equal to the sweep's commit, a successful conclusion, a clean tree, and that the registered end-to-end specs ran across both registered viewport projects.
- **REQ-015** — Coverage telemetry is recorded per lane at a commit and never blocks; a lane whose tool is absent or whose run fails is recorded as not measured. No numeric coverage floor exists. Promoting observed minima to floors may be proposed only once at least five clean-head telemetry records spanning at least two commits exist for a lane, takes a new owner decision, and may only tighten afterwards.
- **REQ-016** — The release-readiness check runs the checks of the profile it is given on the tree it is given and reports a pass or the findings of each step. Its security profile is the private-data and real-secret check of a release-candidate scan.

### Acceptance criteria

- **AC-001** — The checklist routes every change type it names to at least one gate family, a mixed change to the union of its families, and an uncertain change to the broader set with the uncertainty recorded in its gate record. A solver stiffness change routes to the solver gate, and also to the report-template gate if export manifests change; a rule-evaluator parser change routes to the rule-engine gate; a results-viewer release routes to the GUI gate, and also to the report-template gate if export or preview changes; a public report-template wording change routes to the report-template gate.
- **AC-002** — For each of the four families the checklist requires every item of evidence that family's requirement lists, and a gate record for a family has a criterion for each of those items. For the GUI family these include accessibility and usability evidence and regression evidence for each workflow the release touches; for the report-template family, the inclusion of assumptions, limitations and rule-pack references.
- **AC-003** — A gate record is bound to one commit and validates against its schema. Each criterion has exactly one outcome and names the evidence it rests on. A criterion whose threshold, authority decision or evidence is missing is `BLOCKED_TBD`, a criterion only a person can settle is `HUMAN_REVIEW_REQUIRED`, and neither is ever recorded as `PASS`; evidence from another commit never decides a criterion.
- **AC-004** — Every gate record carries the content REQ-010 lists, and every release-governance record the content REQ-012 lists, including the statement that governance acceptance is not professional approval.
- **AC-005** — A mixed change with a family's gate omitted and no recorded waiver is not `PASS`. A waiver record names the item, reason, scope, compensating evidence and risk, and a waiver of any of the three things REQ-011 excludes is refused. A release-governance record for a candidate whose required evidence is missing names each missing item, and it presents the candidate as releasable only with the release authority's recorded acceptance of that risk for the maturity label.
- **AC-006** — No checklist text, gate record, summary or release label states or implies code compliance, certification, endorsement, sealing or professional approval, and no label is issued while the label vocabulary is undecided.
- **AC-007** — The checklist, the gate records, the sweep summaries and the telemetry records contain nothing REQ-008 excludes.
- **AC-008** — The evidence sweep runs its five surfaces at one commit in the fixed order and writes a summary bound to that commit that records whether the tree was clean and the result of every surface; a failing surface gives a failing summary; and a hosted end-to-end run is accepted only when every binding of REQ-014 holds: a head commit that differs from the sweep's commit is refused, and so is a run missing either viewport project or any registered spec.
- **AC-009** — A telemetry record is bound to its commit and reports each lane as measured or not measured. No check, gate or record fails on a coverage number, and no gate states a coverage floor.
- **AC-010** — For a release candidate, the gate records show an automated test result at the candidate's commit for each of the fifteen behaviours of REQ-013, and a behaviour with no result is not `PASS`.
- **AC-011** — Run with its security profile on a tree, the release-readiness check reports a pass when the private-data and real-secret checks find nothing and reports the findings otherwise; a step that cannot run for want of a tool is reported as such and is not a pass.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- A gate record is derived from governed sources only: sweep summaries, the governed tolerance and convergence records, release-candidate scan records and coverage telemetry. Nothing is estimated or defaulted to fill a criterion.
- A record's writing succeeds or fails on the record's own integrity (its commit binding, its schema, that it could be written). Criterion outcomes are content of the record.
- Routing is conservative: where a change might touch a family, it is routed there.
- The sweep runs its surfaces one after another, never in parallel, so that its result does not depend on scheduling.
- Evidence shared by two families of a mixed change is recorded once and referenced from both.

**How each criterion is examined.**

- **VER-001** — AC-001 to AC-005, executable check: observes the checklist's routing table and required-evidence lists, the records emitted for a commit against their schema, the outcome given to criteria with undecided thresholds, human-only items and other-commit evidence, the required content of a record, and the handling of an omitted family, of each excluded waiver and of a candidate with missing evidence.
- **VER-002** — AC-006 and AC-007, judgment by a reviewer who did not write the text: examines the checklist, records and labels against `docs/claims_registry.md` and PRD §21.2; the claims-language lint and the protected-content scan run over the gate artifacts.
- **VER-003** — AC-008, AC-009 and AC-011, executable check: observes the readiness check's reported result for a clean tree, a tree seeded with a marker and a missing tool; the sweep's surface order, its commit binding, its summary for a failing surface and its acceptance or refusal of each hosted-run binding; and the telemetry record for measured, absent and failed lanes, with the absence of any coverage failure.
- **VER-004** — AC-010, comparison with a named reference: the list of PRD §22.4, against which the release candidate's gate records are read behaviour by behaviour.

## Governing Values and Decisions — Axiology

- **AX-001** — *Conservative routing.* Better is fewer release-impacting changes that miss a relevant gate; the target is none. Routing is never narrowed to save running checks without a recorded waiver.
- **AX-002** — *Honest outcomes.* Better is fewer criteria left `BLOCKED_TBD`. That number is never reduced by inventing a threshold, defaulting a criterion to `PASS` or taking evidence from another commit.
- **AX-003** — *Evidence over assertion.* Better is a larger share of criteria decided by a governed artifact at the commit instead of by statement. A lint or a passing gate is never presented as a legal conclusion or as professional approval.
- **AX-004** — *Governance is not approval.* Better is fewer places where the release authority's acceptance could be read as an engineer's acceptance; the target is none. Wording is never softened to make a release sound more assured.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-001 | AC-001 | VER-001 | This scope's family definitions (Ontology) and the expected routes AC-001 states; the checklist's routing table is compared with those routes, which it did not write |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-002 REQ-003 REQ-004 REQ-005 | AC-002 | VER-001 | The checklist's required-evidence lists, and the criteria of a family's record, are compared with the evidence `docs/CONTRACT.md`, PRD §14.4, §19 and §22 require of each family |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-009 REQ-010 | AC-003 | VER-001 | REQ-009 states that an undecided value is never selected; the record's outcomes are compared with the governed sources it read, which the gates did not write |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-006 REQ-010 REQ-012 | AC-004 | VER-001 | Each record's content is compared with the lists of REQ-010 and REQ-012 |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-011 | AC-005 | VER-001 | Records for an omitted family, a waiver, each excluded waiver and a candidate with missing evidence are compared with what REQ-011 allows |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-006 REQ-007 REQ-012 | AC-006 | VER-002 | PRD §21.2 and `docs/claims_registry.md`; judged by a reviewer who did not write the text |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-008 | AC-007 | VER-002 | The gate artifacts are compared with `docs/IP_AND_DATA_BOUNDARY.md` by the protected-content scan and by a reviewer who did not write them |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-014 | AC-008 | VER-003 | The sweep's surfaces, order and summary are compared with the evidence-sweep definition, and its handling of a hosted run with each binding of REQ-014 |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-015 | AC-009 | VER-003 | Records, checks and gates are read for any coverage failure or floor, which REQ-015 excludes |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-013 | AC-010 | VER-004 | PRD §22.4 lists the behaviours; the records are compared with test results the gates did not produce |
| OUT-001 | SOW-026 SOW-027 OBJ-008 | REQ-016 | AC-011 | VER-003 | A tree seeded with a known marker says what must be found, and a clean tree what must not |
