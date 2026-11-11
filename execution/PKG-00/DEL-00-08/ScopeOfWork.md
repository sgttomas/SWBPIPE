---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-00-08
package_id: PKG-00
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-063]
package_objective_refs: [OBJ-013]
---

# Scope of Work — DEL-00-08 Layered software test and acceptance strategy

## Purpose and Objective Traceability

This deliverable is the test strategy of the product. It names the layers of testing, says what each layer must establish and what counts as evidence in it, and ties every quality obligation of the product to the layer that can catch its failure. The result it contributes is that a change to any part of SWBPIPE is examined by a check able to fail on it; that the solver and the rule engine do not reach a release without verification against grounds outside the product; that nothing public ships without a protected-content and provenance check; and that each test result is reported as the kind of evidence it is.

It serves SOW-063 under OBJ-013 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-00, row DEL-00-08, and architecture-basis row AB-00-08 in §8.1), with `docs/PRD.md` §22.1 and §22.4 and `docs/CONTRACT.md` OPS-K-SOLVER-1. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it. It sets the standard that criterion 2 applies (each analysis passes references frozen with an independent generator) and reaches criterion 14 (the content check) through DEL-08-05.

- **OUT-001** — The layered test strategy, a document under `docs/architecture/`: fourteen layers, each with the obligations it verifies, the kind of evidence, and the checks that realise it.
- **OUT-002** — The architecture acceptance gate: the checks of this package's architecture rules, run as the architecture layer.

## Deliverable Definition — Ontology

**Terms the strategy implements.**

- *Layer.* One of fourteen subjects of testing: architecture; schemas; units; services; solver mechanics; load and stress recovery; rule packs; GUI; command-line runner; reports; adapters; packaging; security; regressions.
- *Obligation.* An invariant of `docs/CONTRACT.md`, or a requirement of the PRD or `docs/SPEC.md`, that a test can observe.
- *Kind of evidence.* An executable check, which observes the behaviour itself; a comparison with a named reference; or a judgment by a named role. Mechanics verification, user-rule checking and professional approval are separate kinds of result, and no layer produces the third.
- *Deterministic verification.* For the solver and the rule engine: the same inputs give the same outputs on every run, and the outputs agree with a reference whose expected values come from outside the product, at the governed tolerance.
- *Reference class.* One of three, each with governed relative and absolute pairs per quantity kind: an analytic benchmark, seeded at 1e-9 relative where measured; a comparison between two of the product's own engines, exact; and a regression golden, exact. A fixture may tighten a governed tolerance and never loosen it; an unmeasured pair is stated as open. Comparisons with an external prover are screened under the release target's benchmark method and are not a reference class here.
- *Regression golden.* A comparison with the product's own earlier output. It guards against change and is never counted as verification of correctness.
- *Protected-content and provenance gate.* The check that public examples, reports, templates, fixtures and tests hold no protected standards or proprietary content and that their engineering values carry a source.
- *Architecture gate.* The architecture layer: the executable checks that DEL-00-01 to DEL-00-07 name for their rules.
- *Test baseline.* Cargo tests for Rust, Vitest for TypeScript units and Playwright for desktop journeys, with validation references and the two gates above.
- *Coverage.* Measured and recorded for each language. It blocks nothing and has no numeric floor. A floor is set only by the owner and from then on is only raised.

**Boundary.** This deliverable defines the layers, what each must establish and what counts as evidence; it writes no product test, benchmark or pipeline. The strategy document (OUT-001) and the checks in VER-001, VER-002 and VER-004, which examine the check registry, the reference fixtures and the coverage configuration, are this deliverable's. A layer that lacks a check is repaired by the deliverable that owns the code under test.

- Each deliverable owns the tests of its own code and the references its criteria name.
- DEL-09-01, DEL-09-02 and DEL-09-03 own the mechanics, stress-recovery and nonlinear benchmark suites; DEL-04-05 owns the performance harness.
- DEL-09-05 owns the release quality gates, the release-candidate sweep and the coverage record.
- DEL-10-04 owns CI: which checks run on a change, and the build.
- DEL-08-05 owns the protected-content linter.
- DEL-00-01 to DEL-00-07 own their architecture rules and name the checks; this scope owns that those checks run together as one layer.

## Completion and Reliance Basis — Epistemology

### Requirements

- **REQ-001** — Layered tests are defined for architecture, schemas, units, services, solver mechanics, load and stress recovery, rule packs, GUI, the command-line runner, reports, adapters, packaging, security and regressions (decomposition AB-00-08).
- **REQ-002** — Solver and rule-engine deliverables include deterministic verification before release use (`docs/CONTRACT.md` OPS-K-SOLVER-1).
- **REQ-003** — Public examples, reports, templates and tests pass protected-content and provenance checks where applicable (`docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-REPORT-2).
- **REQ-004** — Architecture acceptance gates are defined for this package's rules, and they claim no product release quality.
- **REQ-005** — A CI or tooling choice is part of the strategy only where an owner ruling selects it. A choice no one has made is stated as undecided.

### Acceptance criteria

- **AC-001** — The strategy maps each of the fourteen layers to the obligations it verifies, the kind of evidence and the checks that realise it. Every invariant of `docs/CONTRACT.md` whose enforcement is a test, a validator or CI is assigned to at least one layer, and every layer has at least one check that runs with the project's checks.
- **AC-002** — Every solver and rule-engine crate has at least one check that compares its output with expected values that come from outside the product, such as a closed-form or hand calculation or a frozen reference produced without product code, at the governed tolerance of its reference class. Repeated runs of those checks give identical results, and no fixture carries a tolerance looser than the governed one.
- **AC-003** — Every public example, report template, fixture and test file is covered by the protected-content scan, and each one that carries engineering values has a provenance record. A seeded protected-content marker fails the scan.
- **AC-004** — Every executable check that DEL-00-01 to DEL-00-07 name for an architecture rule runs in the architecture layer. The layer's result is reported as architecture conformance, and no test result, gate record or summary of any layer states release readiness, validation of the product or professional approval.
- **AC-005** — The strategy names no CI provider, tool or threshold that an owner ruling has not selected, and no check fails on a coverage number.

## Production and Verification Method — Praxeology

**Method choices that define the strategy.**

- An obligation is assigned to the lowest layer that can observe it, and a higher layer repeats it only where the layers' interaction is what is being tested.
- For a numerical result, where the expected value comes from is part of the test: an outside source, an owner ruling, or the product's own earlier output, which only guards against change.
- A gate reports what it measured. Acceptance, waivers and release are a person's acts and are not outputs of a test.

**How each criterion is examined.**

- **VER-001** — AC-001, judgment by a reviewer who did not write the strategy, with an executable check: the reviewer examines the map against the enforcement column of `docs/CONTRACT.md` and `docs/SPEC.md` §10, and the check observes that each layer has at least one registered check that runs.
- **VER-002** — AC-002, executable check with judgment by a reviewer who did not write the check: the check observes for each solver and rule-engine crate a comparison with expected values whose source is outside the product, the tolerance each fixture uses against the governed one, and repeat-run identity; the reviewer judges the source of any expected value the check cannot trace.
- **VER-003** — AC-003, executable check: runs the protected-content scan and the provenance check over the public examples, templates, fixtures and tests, with a seeded marker that must fail.
- **VER-004** — AC-004 and AC-005, judgment by the same reviewer, with an executable check: the reviewer examines the architecture layer against the checks DEL-00-01 to DEL-00-07 name, the wording of test and gate outputs against `docs/claims_registry.md`, and each tooling choice against its ruling; the check observes that no coverage threshold is configured to fail.

## Governing Values and Decisions — Axiology

- **AX-001** — *Grounds outside the product.* Better is a larger share of numerical criteria whose expected values come from an outside source. A reference is never altered and a tolerance never loosened to make a test pass.
- **AX-002** — *Fast, and still able to fail.* Better is a shorter time from a change to the result of the checks that can fail on it. Speed never removes the only check that observes an obligation.
- **AX-003** — *Evidence says what it is.* Better is a larger share of test and gate outputs whose label names the kind of evidence observed: verification, regression guard or validation evidence. A label never claims more than the check observed.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-063 OBJ-013 | REQ-001 | AC-001 | VER-001 | Decomposition AB-00-08 names the layers and `docs/CONTRACT.md` the obligations; a reviewer who did not write the strategy compares the map against them |
| OUT-001 | SOW-063 OBJ-013 | REQ-002 | AC-002 | VER-002 | `docs/CONTRACT.md` OPS-K-SOLVER-1 and the governed tolerance of each reference class; each reference's own values come from outside the product. The comparison shows agreement with those values; a regression golden shows only consistency with earlier output |
| OUT-001 | SOW-063 OBJ-013 | REQ-003 | AC-003 | VER-003 | `docs/IP_AND_DATA_BOUNDARY.md` and `docs/CONTRACT.md` OPS-K-IP-1; the seeded marker states what must fail |
| OUT-002 | SOW-063 OBJ-013 | REQ-004 | AC-004 | VER-004 | Decomposition AB-00-08 and `docs/PRD.md` §21.2; the comparison is the checks named in the scopes of DEL-00-01 to DEL-00-07 |
| OUT-001 | SOW-063 OBJ-013 | REQ-005 | AC-005 | VER-004 | Each tooling choice is compared with the decision that selects it in the decision log (decomposition §12); the coverage configuration is examined for a threshold that can fail |
