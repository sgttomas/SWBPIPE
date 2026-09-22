---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-09-01
package_id: PKG-09
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-026]
package_objective_refs: [OBJ-008]
---

# Scope of Work — DEL-09-01

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open work and lifecycle remain governed by `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-09-01` in service of project scope [SOW-026] and package objectives [OBJ-008].

- **OUT-001** — A mechanics benchmark-suite contract covering cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms with original/public/permissive provenance is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-09-01 Mechanics benchmark suite

> #### Datasheet: DEL-09-01 Mechanics benchmark suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-09-01 |
> | Package ID | PKG-09 |
> | Package | Verification, Validation, and Quality Oracles |
> | Deliverable type | TEST_SUITE |
> | Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` (accepted authority; see the project decision register) |
> | Scope items | SOW-026 |
> | Objectives | OBJ-008 |
> | Context envelope | M |
> | Lifecycle state during setup Historical setup constraint only; current work follows the accepted deliverable scope and active bounded brief, with lifecycle/issuance separately governed. | Draft setup evidence only; not implementation and not ISSUED |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | Suite purpose | Provide repeatable, source-qualified mechanics benchmark evidence through the current `validation/benchmarks/mechanics/` suite; preserve original/public source and engineering-acceptance boundaries. |
> | Benchmark families | Cantilevers, frames, thermal growth, imposed displacement, and local-to-global stiffness transforms. |
> | Solver boundary Historical setup constraint only; current work follows the accepted deliverable scope and active bounded brief, with lifecycle/issuance separately governed. | The suite observes solver behavior; this setup pass does not implement solver logic, benchmark source files, tests, or final comparison tolerances. |
> | Source posture | Benchmark sources must be original, public-domain, public-permissive, or otherwise documented for redistribution before entering public artifacts. |
> | Data boundary | Protected standards examples, code text, protected tables, proprietary commercial benchmarks, and vendor/private project data are excluded. |
> | Tolerance posture | Final numerical tolerances, benchmark acceptance ranges, and release thresholds remain `TBD` pending solver prototype and human authority. |
> | Result posture | Benchmark outputs should preserve units, solver version, diagnostics/result-envelope fields, assumptions, provenance, and limitations once implementation is authorized. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> Current deliverable obligation: Create benchmark cases for cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> This setup context authorizes only document production and local setup registers. It does not authorize edits to validation benchmark source files, implementation tests, solver modules, repo-level CI, or `ISSUED` lifecycle state.
>
> The future mechanics benchmark suite must stay aligned with the architecture-basis constraints for module boundaries, diagnostics/result envelopes, layered validation gates, unit safety, and protected-content/provenance review. Exact fixture schema, runner command, solver numerical library, output comparison format, comparison tolerances, and CI/release thresholds remain `TBD`.
>
### CLM-006 — Construction

> ##### Construction
>
> | Construction item | Status |
> |---|---|
> | `validation/benchmarks/mechanics` Historical setup constraint only; current work follows the accepted deliverable scope and active bounded brief, with lifecycle/issuance separately governed. | Anticipated future artifact; not created or edited in this setup pass. |
> | Hand-calculation notes | The current fixture inventory and hand-calculation locations are recorded in `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/`. Use the recorded candidate-bound inputs, units and regression values; final source eligibility, unadopted tolerances and engineering acceptance remain separate under DEC-026/027. |
> | Cantilever cases | The current fixture inventory and hand-calculation locations are recorded in `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/`. Use the recorded candidate-bound inputs, units and regression values; final source eligibility, unadopted tolerances and engineering acceptance remain separate under DEC-026/027. |
> | Frame cases | The current fixture inventory and hand-calculation locations are recorded in `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/`. Use the recorded candidate-bound inputs, units and regression values; final source eligibility, unadopted tolerances and engineering acceptance remain separate under DEC-026/027. |
> | Thermal growth cases | The current fixture inventory and hand-calculation locations are recorded in `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/`. Use the recorded candidate-bound inputs, units and regression values; final source eligibility, unadopted tolerances and engineering acceptance remain separate under DEC-026/027. |
> | Imposed displacement cases | The current fixture inventory and hand-calculation locations are recorded in `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/`. Use the recorded candidate-bound inputs, units and regression values; final source eligibility, unadopted tolerances and engineering acceptance remain separate under DEC-026/027. |
> | Stiffness transform cases | The current fixture inventory and hand-calculation locations are recorded in `validation/benchmarks/mechanics/README.md` and `validation/hand_calcs/mechanics/`. Use the recorded candidate-bound inputs, units and regression values; final source eligibility, unadopted tolerances and engineering acceptance remain separate under DEC-026/027. |
> | Provenance index | Required before public fixture publication; exact format is `TBD`. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, objectives, anticipated artifacts, and architecture-basis injection.
> - `_REFERENCES.md` for governing local references.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` accepted current basis, rows for PKG-09, DEL-09-01, SOW-026, OBJ-008, AB-00-01, AB-00-02, AB-00-06, and AB-00-08.
> - `docs/_Registers/Deliverables.csv` row DEL-09-01.
> - `docs/_Registers/ScopeLedger.csv` row SOW-026.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-09-01.
> - `docs/SPEC.md` sections 4.1, 4.2, 4.5, and 9.
> - `docs/VALIDATION_STRATEGY.md` sections 1, 2, 4, and 5.
> - `docs/CONTRACT.md` invariants OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-DATA-3, OPS-K-UNIT-1, OPS-K-AUTH-1, OPS-K-SOLVER-1, and OPS-K-AGENT-1..4.
>

### CLM-008 — Open Setup Questions

> ##### Open Setup Questions
>
> | Question | Needed from |
> |---|---|
> | Which mechanics benchmark fixtures and public/permissive sources are approved for implementation? | Validation owner / IP review owner |
> | What numerical comparison tolerance policy is acceptable for each benchmark family? | DEC-026 supplies the comparison-policy structure and protected promotion route; per-kind measurements and unadopted numeric criteria remain open. |
> | What fixture schema, runner interface, and result-envelope fields should executable benchmarks use? | Architecture / solver / validation owners |
> | Which benchmark cases gate release and which are advisory regression checks? | QA/release owner / human project authority |
>

### CLM-009 — D-41 R5 T2B PDU-013 Evidence State

> ##### D-41 R5 T2B PDU-013 Evidence State
>
> DEC-018 adopts the project unit catalog and conversion basis. The PDU fixture-local unit declaration does not itself prove that this suite is bound to or rerun through that catalog; preserve the unit-integration and conversion-witness delivery obligation.

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification: DEL-09-01 Mechanics benchmark suite

> #### Specification: DEL-09-01 Mechanics benchmark suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-012 — Scope

> ##### Scope
>
> Current deliverable obligation: Create benchmark cases for cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> This deliverable specifies setup evidence for a future mechanics benchmark suite covering cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms.
>
> The original setup pass does not implement benchmark source files, add tests, create numerical hand-calculation cases, modify solver code, edit validation directories outside this deliverable, choose final tolerances, or introduce protected/proprietary benchmark data.
>
### CLM-013 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis | Verification hook |
> |---|---|---|---|
> | DEL-09-01-RQ-001 | The benchmark suite shall remain separate from solver implementation and shall not modify production solver behavior. | DEL-09-01 context; AB-00-02; docs/SPEC.md section 9 | Module boundary review once implementation exists. |
> | DEL-09-01-RQ-002 | The suite shall cover cantilever, frame, thermal-growth, imposed-displacement, and stiffness-transform benchmark families. | DEL-09-01 context; docs/VALIDATION_STRATEGY.md section 2 | Fixture inventory review against required families. |
> | DEL-09-01-RQ-003 | Benchmark sources and expected-result derivations shall be original, public-domain, public-permissive, or otherwise documented with redistribution rights. | SOW-026; docs/VALIDATION_STRATEGY.md section 5; OPS-K-IP-1..3 | Protected-content/provenance review before publication. |
> | DEL-09-01-RQ-004 | Benchmark inputs and expected outputs shall be unit-aware and dimensionally checked. | OPS-K-UNIT-1; docs/SPEC.md sections 4.1 and 4.2 | Unit validation tests once fixture schema exists. |
> | DEL-09-01-RQ-005 | The suite shall preserve solver diagnostics, result-envelope status, solver version, assumptions, provenance, and limitations where supported by implementation contracts. | AB-00-06; docs/SPEC.md sections 4.5 and 9 | Result-envelope comparison and diagnostic review. |
> | DEL-09-01-RQ-006 | The setup artifacts shall not set final numerical tolerances, release thresholds, or benchmark pass/fail authority without human approval. | OI-005; OPS-K-AGENT-1..4 | Review confirms unresolved tolerances remain `TBD`. |
> | DEL-09-01-RQ-007 | The suite shall remain mechanics verification support and shall not claim certification, code compliance, professional approval, or project-specific reliance (PRD §21.2). | OPS-K-AUTH-1; docs/VALIDATION_STRATEGY.md section 1 | Review of result labels, reports, and release notes. |
>

### CLM-014 — Standards

> ##### Standards
>
> No protected standard text, proprietary benchmark suite, commercial software example, or vendor dataset is available in this deliverable-local setup context. Any future mechanics case must record source, provenance, license/redistribution status, contributor certification, and review disposition before it is accepted as a public benchmark.
>
> Clause-level or code-specific validation requirements are `TBD` and must not be inferred from protected standards.
>

### CLM-015 — Verification

> ##### Verification
>
> | Verification area | Minimum setup expectation |
> |---|---|
> | Required family coverage | The future fixture inventory explicitly maps each case to one or more required benchmark families. |
> | Analytical derivation | Hand-calculation notes identify assumptions, units, equations used from public/original mechanics, and expected result fields. |
> | Numerical comparison | Tolerance policy is cited when approved; until then each tolerance field remains `TBD`. |
> | Unit safety | Inputs, expected values, and outputs are dimensionally checked under the accepted unit system. |
> | Data boundary | Fixture provenance confirms original/public/permissive status and excludes protected standards/commercial examples. |
> | Diagnostics/result envelopes | Benchmark outputs preserve warnings, diagnostics, solver version, and limitations without compliance claims. |
>

### CLM-016 — Documentation

> ##### Documentation
>
> Maintain the implemented fixture inventory in `validation/benchmarks/mechanics/`, hand-calculation notes in `validation/hand_calcs/mechanics/`, source/provenance records, runner/integration notes and candidate-bound comparison results. Fixture-local units remain distinct from DEC-018 catalog integration. DEC-026 governs protected comparison criteria; unmeasured values, dedicated provenance-index completeness and external-validation/release acceptance remain separate obligations.

### CLM-017 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | None | No source conflict identified in setup evidence. | N/A | N/A |
>

### CLM-018 — D-41 R5 T2B PDU-013 Evidence Disposition (2026-07-12)

> ##### D-41 R5 T2B PDU-013 Evidence Disposition (2026-07-12)
>
> DEC-018 adopts the project unit catalog and conversion basis. The PDU fixture-local unit declaration does not itself prove that this suite is bound to or rerun through that catalog; preserve the unit-integration and conversion-witness delivery obligation.

- **AC-001** — The contract preserves unit-aware inputs and outputs, solver diagnostics, result-envelope fields, assumptions, provenance, limitations, fixture-local unit evidence, and DEC-018 project-unit authority with separately verified suite binding, DEC-026 tolerance authority and explicit unmeasured per-kind values. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Production and Verification Method — Praxeology

### CLM-019 — Procedure: DEL-09-01 Mechanics benchmark suite

> #### Procedure: DEL-09-01 Mechanics benchmark suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-021 — Purpose

> ##### Purpose
>
> Current deliverable obligation: Create benchmark cases for cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle and remaining work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> Describe how a future TASK worker should produce or use the mechanics benchmark suite once implementation is authorized, while preserving the current setup-only boundary.
>
### CLM-022 — Prerequisites

> ##### Prerequisites
>
> - Accepted frame-stiffness and coordinate-transform interfaces from PKG-04 implementation deliverables.
> - Accepted straight-element mechanics behavior for cantilever-style cases.
> - Accepted support, restraint, and imposed-displacement semantics for boundary-condition cases.
> - Accepted load/thermal input semantics where thermal-growth benchmarks require them.
> - Accepted unit-system, diagnostics, and result-envelope contracts for benchmark inputs and outputs.
> - Original, public-domain, public-permissive, or otherwise lawfully redistributable benchmark sources with provenance.
> - Human-approved numerical tolerance and release-gate policy; current value is `TBD`.
>

### CLM-023 — Steps

> ##### Steps
>
> 1. Confirm the sealed brief authorizes benchmark implementation and lists exact write targets.
> 2. Read the accepted solver interfaces, unit-system contract, diagnostics/result-envelope contract, and applicable validation strategy.
> 3. Select or create benchmark fixtures only from original, public-domain, public-permissive, or otherwise documented lawful sources.
> 4. For each fixture, record benchmark family, source/provenance, redistribution status, assumptions, units, model inputs, expected result fields, and derivation notes.
> 5. Keep protected standards examples, proprietary commercial benchmarks, vendor-private data, and copied code-derived formulas out of public fixtures.
> 6. Run benchmark cases through the authorized solver or headless runner without changing solver logic.
> 7. Compare outputs using the approved comparison/tolerance policy; if no approved policy exists, record `TBD` rather than inventing pass/fail criteria.
> 8. Capture solver version, result-envelope fields, warnings, diagnostics, assumptions, limitations, and comparison records.
> 9. Stop and escalate if fixture provenance is missing, protected content is suspected, numerical tolerances are unsupported, or output wording implies certification, code compliance, or professional approval (PRD §21.2).
>

### CLM-024 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Scope boundary | Benchmark work stays in authorized validation/test locations and does not modify solver logic unless separately authorized. |
> | Family coverage | Fixture inventory covers cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms or records an explicit `TBD`. |
> | Fixture provenance | Every public fixture records source, license/redistribution status, and review disposition. |
> | Unit safety | Inputs, expected values, and solver outputs pass accepted dimensional checks. |
> | Tolerance authority | Comparison tolerances cite an approved source or remain `TBD`. |
> | Diagnostics/reporting boundary | Results preserve warnings/assumptions/limitations and avoid certification or compliance claims. |
>

### CLM-025 — Records

> ##### Records
>
> - Benchmark fixture inventory and provenance index.
> - Hand-calculation or derivation notes for each case.
> - Solver version and benchmark runner settings.
> - Unit-check and result-envelope comparison records.
> - Review notes for tolerance policy, fixture approval, and human rulings.
>

### CLM-026 — D-41 R5 T2B PDU-013 Check

> ##### D-41 R5 T2B PDU-013 Check
>
> Record fixture-local unit identifiers and canonical dimensions exactly. Do not describe those checks as project-unit-system acceptance until the upstream catalog and conversions are accepted and the cases are rerun through that basis. Keep comparison tolerances and pass/fail authority unchanged.

- **VER-001** — Validate the contract and review source parity, required benchmark-family coverage, source and redistribution posture, unit and diagnostic visibility, setup versus implementation boundaries, retained tolerance and project-unit TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-027 — Guidance: DEL-09-01 Mechanics benchmark suite

> #### Guidance: DEL-09-01 Mechanics benchmark suite
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-029 — Purpose

> ##### Purpose
>
> Provide repeatable and reviewable open-mechanics benchmark evidence using the current fixture and hand-calculation inventories. Preserve the boundary between public/original/permissive mechanics cases and protected standards or proprietary examples; test success does not supply professional validation.

### CLM-030 — Principles

> ##### Principles
>
> - Treat benchmarks as verification evidence, not as solver implementation.
> - Use public/original mechanics cases with clear provenance and redistribution status.
> - Keep final comparison tolerances and release thresholds as `TBD` until approved by the proper human authority.
> - Make units, assumptions, expected result fields, diagnostics, and limitations explicit.
> - Preserve the distinction between mechanics verification, user-rule checks, and professional approval.
>

### CLM-031 — Considerations

> ##### Considerations
>
> Cantilever, frame, thermal-growth, imposed-displacement, and stiffness-transform cases exercise different solver interfaces. Future implementation should identify which solver/load/support contracts each case depends on before marking a benchmark executable.
>
> Thermal-growth and imposed-displacement cases can look similar to code or owner-standard examples if copied from protected sources. Future workers should create original cases or use permissively licensed/public-domain mechanics references, then record source and license status before publishing fixtures.
>
> Stiffness-transform cases should isolate local-to-global coordinate behavior and sign conventions. Current fixture matrices, geometry and regression values are recorded in `validation/benchmarks/mechanics/` and their original hand calculations. Apply DEC-026 to protected comparison criteria and preserve unmeasured values; the original setup brief did not itself authorize numeric acceptance.
>

### CLM-032 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Simple hand checks vs representative piping behavior | Start with analytically transparent cases; broaden only when provenance and expected-result derivations remain reviewable. |
> | Strict tolerances vs solver maturity | Record exact and approximate expected values separately when authorized; final acceptance tolerances remain `TBD`. |
> | Public benchmark usefulness vs IP risk | Prefer original/public mechanics cases even when protected examples are familiar. |
> | Release gating vs exploratory regression | Distinguish required release benchmarks from advisory regression checks after human QA policy is approved. |
>

### CLM-033 — Examples

> ##### Examples
>
> Current geometries, material values, loads, boundary conditions and regression observations are recorded in the fixture inventory and hand-calculation notes under `validation/benchmarks/mechanics/` and `validation/hand_calcs/mechanics/`. Additions must be original, public-domain or permissively licensed with provenance; numeric comparison acceptance follows DEC-026 and remains distinct from recorded regression values.

### CLM-034 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in setup evidence. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-035 — Open Enrichment Items

> ##### Open Enrichment Items
>
> | Item | Status |
> |---|---|
> | Approved benchmark fixture list and source basis | TBD |
> | Approved numerical tolerance/comparison policy | DEC-026 governs comparison-policy structure; record each measured/accepted scope separately and keep unadopted values TBD. |
> | Approved fixture schema and result-envelope comparison format | TBD |
> | Release-gating vs advisory benchmark classification | TBD |
>

### CLM-036 — D-41 R5 T2B PDU-013 Boundary

> ##### D-41 R5 T2B PDU-013 Boundary
>
> DEC-018 adopts the project unit catalog and conversion basis. The PDU fixture-local unit declaration does not itself prove that this suite is bound to or rerun through that catalog; preserve the unit-integration and conversion-witness delivery obligation.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-026 OBJ-008 | CLM-010 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
