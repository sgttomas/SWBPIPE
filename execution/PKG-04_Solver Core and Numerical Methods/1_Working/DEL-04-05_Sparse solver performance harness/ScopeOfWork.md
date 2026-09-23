---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-04-05
package_id: PKG-04
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-035]
package_objective_refs: [OBJ-003, OBJ-008]
---

# Scope of Work — DEL-04-05

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions, with execution in the selected graph; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-04-05` in service of project scope [SOW-035] and package objectives [OBJ-003, OBJ-008].

- **OUT-001** — A sparse-solver performance and regression harness contract covering deterministic practical-model runs, reproducibility, performance and conditioning observations, lawful fixture provenance, solver/version/hash settings, diagnostics, limitations, and reviewable records is produced for the declared scope and objectives.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-04-05 Sparse solver performance harness

> #### Datasheet: DEL-04-05 Sparse solver performance harness
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-04-05 |
> | Package ID | PKG-04 |
> | Package | Solver Core and Numerical Methods |
> | Deliverable type | TEST_SUITE |
> | Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` (accepted authority; see the project decision register) |
> | Scope items | SOW-035 |
> | Objectives | OBJ-003, OBJ-008 |
> | Context envelope | M |
> | Lifecycle state during setup | Draft setup evidence only; not implementation and not ISSUED |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | Harness purpose | Define a deterministic performance/regression harness for sparse solver behavior on practical piping-model sizes and numerical-conditioning cases. |
> | Solver boundary | The harness observes solver behavior; it does not implement solver logic, select a numerical library, or change sparse solve algorithms. |
> | Determinism posture | Repeated runs for the same model, units, solver version, and settings must be reproducible enough for regression comparison; numerical verification follows DEC-026; unmeasured release criteria remain explicit. |
> | Performance target posture | Specific runtime, memory, scale, and conditioning thresholds are `TBD` until their governing acceptance; the implemented harness records bounded DEC-050/053 observations. |
> | Data posture | Fixtures must use original, invented, public-permissive, or otherwise lawful inputs; no proprietary benchmark data or protected standards examples are introduced. |
> | Reporting posture | Results and diagnostics must preserve warnings, assumptions, provenance, solver version, and limitations without claiming certification or code compliance. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> The current performance-harness implementation records bounded observations. New release thresholds, dependency changes and use of proprietary data require their own authorized scope and governing decisions.
>
> The harness must remain compatible with the architecture-basis constraints for module boundaries, result/diagnostic envelopes, layered tests, and protected-content review. Numerical library choice, sparse-solver settings, conditioning metrics, practical model-size bands, timing methodology, hardware normalization, and CI gating thresholds remain `TBD`.
>

### CLM-006 — Construction

> ##### Construction
>
> | Construction item | Status |
> |---|---|
> | Performance tests | Current crate tests in `core/solver/performance_harness/src/lib.rs`; results remain candidate-bound. |
> | Benchmark harness | Current carrier is `core/solver/performance_harness/`; invocation and observation format belong to bound implementation evidence. |
> | Conditioning cases | Required conceptually by SOW-035; concrete matrices/models and acceptable ranges are `TBD`. |
> | Regression records | Required conceptually for OBJ-008; retention format and comparison policy are `TBD`. |
> | Diagnostics/result-envelope hooks | Required by AB-00-06; exact codes/classes for harness failures are `TBD`. |
>

### CLM-007 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, scope, objectives, anticipated artifacts, and architecture-basis injection.
> - `_REFERENCES.md` for governing local references.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` (accepted authority through the decision register), rows for PKG-04, DEL-04-05, SOW-035, OBJ-003, OBJ-008, AB-00-01, AB-00-02, AB-00-06, and AB-00-08.
> - `docs/_Registers/Deliverables.csv` row DEL-04-05.
> - `docs/_Registers/ScopeLedger.csv` row SOW-035.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-04-05.
> - `docs/CONTRACT.md` invariants OPS-K-SOLVER-1, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-REPORT-1, OPS-K-AGENT-1..4, and OPS-K-IP-1.
>

### CLM-008 — Open Setup Questions

> ##### Open Setup Questions
>
> | Question | Needed from |
> |---|---|
> | Which sparse strategy governs current observations? | DEC-023 strategy and DEC-050/053 live/default policy; no new release thresholds |
> | What practical model-size bands and conditioning metrics are meaningful for release gates? | Solver lead / validation owner |
> | What deterministic timing methodology is acceptable across local and CI environments? | QA/release owner |
> | Which invented or public-permissive fixtures may represent practical piping models without protected data? | Validation/IP review owner |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-04-05 Sparse solver performance harness

> #### Specification: DEL-04-05 Sparse solver performance harness
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — Scope

> ##### Scope
>
> This deliverable defines the performance-harness contract, realized in `core/solver/performance_harness/`, for sparse solver performance, reproducibility, and numerical-conditioning behavior for practical piping-model sizes.
>
> This setup pass does not implement a benchmark runner, add tests, choose thresholds, edit solver code, select a numerical library, or introduce proprietary/protected benchmark data.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source basis | Verification hook |
> |---|---|---|---|
> | DEL-04-05-RQ-001 | The harness shall be separate from solver logic and shall not modify production solver behavior. | DEL-04-05 context; PKG-04 note; AB-00-02 | Module boundary review once implementation exists. |
> | DEL-04-05-RQ-002 | Harness cases shall support deterministic regression comparison for the same model, unit system, solver version, and settings. | SOW-035; docs/SPEC.md section 4.5; OPS-K-SOLVER-1 | Repeat-run tests with accepted tolerance policy; exact tolerances `TBD`. |
> | DEL-04-05-RQ-003 | Performance and conditioning metrics shall be recorded without inventing release thresholds. | SOW-035; OI-005 | Review that thresholds remain `TBD` until approved. |
> | DEL-04-05-RQ-004 | Fixture data shall be invented, public-permissive, or otherwise lawful and shall not contain protected standards examples, protected tables, vendor data, or proprietary commercial benchmark data. | OPS-K-IP-1; docs/DIRECTIVE.md data-boundary rules | Protected-content and provenance review. |
> | DEL-04-05-RQ-005 | Harness outputs shall preserve solver version, model/hash basis where available, warning/diagnostic classes, assumptions, provenance notes, and limitations. | AB-00-06; OPS-K-REPORT-1 | Result-envelope/report-facing tests once result schema exists. |
> | DEL-04-05-RQ-006 | Unit-sensitive fixture inputs and outputs shall pass the accepted unit-system/dimensional checks. | OPS-K-UNIT-1 | Unit validation tests once fixture schema exists. |
> | DEL-04-05-RQ-007 | The harness shall remain mechanics-only and shall not claim professional approval, certification, or code compliance (PRD §21.2). | OPS-K-MECH-1; OPS-K-AGENT-4; OPS-K-REPORT-1 | Review of result labels, generated reports, and release notes. |
>

### CLM-013 — Standards

> ##### Standards
>
> No protected standard text, proprietary benchmark suite, or vendor dataset is available in this deliverable-local setup context. Any future benchmark basis must be recorded with provenance, redistribution status, and human/IP review disposition. Clause-level or vendor-specific requirements are `TBD`.
>

### CLM-014 — Verification

> ##### Verification
>
> | Verification area | Minimum setup expectation |
> |---|---|
> | Determinism | Define repeat-run evidence without setting unsupported numeric thresholds. |
> | Sparse performance | Capture timing/memory/scale observations once solver implementation exists; target values remain `TBD`. |
> | Conditioning | Record conditioning-related diagnostics or solver-status observations when supported by the solver/result envelope. |
> | Data boundary | Confirm all fixtures are invented, public-permissive, or otherwise lawful. |
> | Reporting | Confirm harness output does not imply certification, code compliance, or professional acceptance (PRD §21.2). |
>

### CLM-015 — Documentation

> ##### Documentation
>
> Maintain the current harness artifacts and their bounded evidence:
>
> - performance tests;
> - benchmark harness;
> - benchmark fixture provenance notes;
> - deterministic run records or comparable result snapshots.
>
> The exact module paths, runner command, fixture schema, metrics, and CI gates are `TBD` and must not be resolved by this setup pass.
>

### CLM-016 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Issue | Contenders | Human ruling |
> |---|---|---|---|
> | None | No source conflict identified in setup evidence. | N/A | N/A |

- **AC-001** — The contract preserves the accepted observer-only harness boundary, deterministic regression intent, explicit units and provenance, no invented timing, memory, conditioning, model-size, or release thresholds, mechanics-only reporting, and formal-review, dimensional-validation, hosted-CI, and cross-machine policy residuals.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-04-05 Sparse solver performance harness

> #### Procedure: DEL-04-05 Sparse solver performance harness
>

### CLM-018 — Purpose

> ##### Purpose
>
> Describe maintenance and use of `core/solver/performance_harness/` under the current brief; preserve DEC-023/050/053 and retain open release-threshold decisions.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> - Accepted solver/kernel interfaces from PKG-04 implementation deliverables.
> - Accepted unit-system and schema contracts for model fixtures and result envelopes.
> - Human-approved sparse solver/library and diagnostic policy, if implementation requires one.
> - Invented, public-permissive, or otherwise lawful benchmark fixtures with provenance.
> - Human-approved performance/conditioning threshold policy; current value is `TBD`.
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the sealed brief authorizes implementation and lists exact write targets.
> 2. Read the accepted solver interface, result-envelope schema, diagnostics contract, and unit-system contract.
> 3. Select or create benchmark fixtures only from invented, public-permissive, or otherwise lawful data.
> 4. Record fixture provenance, model/hash basis where available, unit basis, solver version, and harness settings.
> 5. Run repeated deterministic cases through the solver interface without changing solver logic.
> 6. Capture performance observations, conditioning diagnostics, warnings, assumptions, and limitations.
> 7. Compare repeated outputs using the approved tolerance/threshold policy; if no policy exists, record `TBD` rather than failing or passing on invented criteria.
> 8. Emit run records suitable for regression review and release-gate evidence.
> 9. Stop and escalate if protected data, proprietary benchmark content, missing engineering values, or compliance/certification wording appears.
>

### CLM-021 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Scope boundary | Harness code/tests do not modify solver logic or repo-level artifacts unless explicitly authorized. |
> | Determinism | Repeat-run comparison evidence is present; tolerance policy is cited or marked `TBD`. |
> | Fixture provenance | Each fixture records source/provenance and redistribution status. |
> | Unit safety | Fixture inputs and outputs pass accepted dimensional checks. |
> | Diagnostic/reporting boundary | Results include warnings/assumptions/limitations and avoid certification or compliance claims. |
>

### CLM-022 — Records

> ##### Records
>
> - Performance/regression run records.
> - Fixture provenance index.
> - Solver version and settings record.
> - Conditioning diagnostic summary.
> - Review notes for threshold changes and human rulings.
>

### CLM-023 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. The sparse-solver performance harness now exists. Threshold selection, CI/release gating, platform policy, and validation remain held where recorded; existence of the harness does not establish a production-performance ruling.

- **VER-001** — Validate the contract and review source parity, harness/solver separation, repeated-run determinism, performance and conditioning records, unit and fixture provenance, protected-content controls, diagnostics and limitations, no compliance claims, and all unresolved governed threshold and release items.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-04-05 Sparse solver performance harness

> #### Guidance: DEL-04-05 Sparse solver performance harness
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — Purpose

> ##### Purpose
>
> This deliverable prepares the evidence boundary for the deterministic sparse-solver performance/regression harness. It exists to keep performance and conditioning checks observable, repeatable, and reviewable without coupling benchmark code to solver implementation choices.
>

### CLM-027 — Principles

> ##### Principles
>
> - Treat the harness as an observer and regression surface, not as solver logic.
> - Prefer deterministic, schema-backed inputs and outputs so repeated runs can be compared.
> - Preserve unknowns as `TBD`; do not invent runtime, memory, model-size, or conditioning thresholds.
> - Use only invented, public-permissive, or otherwise lawful benchmark fixtures.
> - Keep result wording mechanics-focused. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>

### CLM-028 — Considerations

> ##### Considerations
>
> Sparse performance measurements are sensitive to solver library, hardware, compiler settings, platform, matrix ordering, and fixture structure. Those implementation variables are not resolved here. DEC-023 resolves solver strategy and deterministic ordering; DEC-053 names bounded size-band observations. Hardware-normalized release thresholds remain open, and each observation must retain its actual environment/candidate binding.
>
> Conditioning cases should exercise solver diagnostic behavior without embedding protected standards examples or proprietary commercial models. If a fixture source cannot be proven redistributable, it should not enter the public harness.
>

### CLM-029 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Deterministic regression vs benchmark realism | Prefer reproducible invented/public fixtures first; practical realism improves only when provenance and redistribution rights are clear. |
> | CI stability vs timing sensitivity | Timing gates are `TBD`; early harnesses may record metrics without failing builds on unsupported thresholds. |
> | Solver abstraction vs library-specific diagnostics | Keep common harness records stable; library-specific fields should be explicit and reviewable if introduced. |
> | Performance visibility vs professional claims | Publish observations and warnings, not certification, compliance, or engineering acceptance. |
>

### CLM-030 — Examples

> ##### Examples
>
> Invented chain/grid fixtures and bounded size bands are recorded in `core/solver/performance_harness/README.md` under DEC-050/053. Release timing, memory and conditioning thresholds remain `TBD`. Future examples must use invented or public-permissive data with provenance.
>

### CLM-031 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in setup evidence. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-032 — Open Enrichment Items

> ##### Open Enrichment Items
>
> | Item | Status |
> |---|---|
> | Approved performance metrics and threshold policy | TBD |
> | Bounded practical model-size observations | DEC-053 observation set; no general release-scale claim |
> | Approved sparse solver/library choice | DEC-023; bounded live/default policy DEC-050/053 |
> | Public/permissive fixture source list | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-035 OBJ-003 OBJ-008 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-04-05:1** — Complete or locate the module-boundary and fixture provenance/IP review, accepted dimensional-check evidence, and report-facing result-schema binding for the performance harness; retain PDU-035 formal disposition. (R5 continuation claim references: DEL-04-05:SOW#CLM-012/DEL-04-05-RQ-001; RQ-004; RQ-005; RQ-006; CLM-022.)
- **DEL-04-05:2** — Obtain the owning formal REVIEW disposition and accepted dimensional-check basis before PDU-035 closure; fixture unit metadata is reproducibility evidence, not conversion or engineering validation.
- **DEL-04-05:3** — Promote release/external sparse thresholds: timing, allocator/RSS memory, conditioning beyond the pivot-ratio proxy, and cross-machine hardware-normalized pass/fail gates (stage-gated: R5 release) (source: PRD plan §3 D7 row + Phase D sparse update / DEC-050/DEC-053)
- **DEL-04-05:4** — Provide hosted-CI sparse evidence once public-export CI activates (gated: D-05b conditions per DEC-059) (source: PRD plan §3 D7 row)
