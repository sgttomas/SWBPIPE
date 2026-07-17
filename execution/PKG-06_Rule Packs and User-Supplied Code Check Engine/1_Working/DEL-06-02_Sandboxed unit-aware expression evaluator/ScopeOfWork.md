---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-06-02
package_id: PKG-06
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-045]
package_objective_refs: [OBJ-005]
---

# Scope of Work — DEL-06-02

## Purpose and Objective Traceability

This Scope of Work defines `DEL-06-02` in service of project scope [SOW-045] and package objectives [OBJ-005].

- **OUT-001** — A sandboxed unit-aware expression-evaluator contract covering a declarative allowlisted grammar, typed variables and functions, dimensional checks, bounded evaluation, deterministic diagnostics, canonical rule-pack version binding, and schema-first result envelopes is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-06-02 Sandboxed unit-aware expression evaluator

> #### Datasheet: DEL-06-02 Sandboxed unit-aware expression evaluator
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-02-DECL-002`.
>

### CLM-003 — PDU-024 Version Integration

> ##### PDU-024 Version Integration
>
> - Project-carried evaluator inputs consume DEL-02-05 model-document version handling; accepted current family is `0.2.0`.
> - Version-check diagnostics remain the established current/stale/unsupported/newer/failed vocabulary.
> - No expression grammar, binding namespace, evaluator library, rule-pack fixture, or migration tooling is added.
>

### CLM-004 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-06-02 | `_CONTEXT.md` |
> | Package ID | PKG-06 | `_CONTEXT.md` |
> | Package | Rule Packs and User-Supplied Code Check Engine | `_CONTEXT.md` |
> | Type | BACKEND_FEATURE_SLICE | `docs/_Registers/Deliverables.csv` row DEL-06-02 |
> | Scope item | SOW-045 | `docs/_Registers/ScopeLedger.csv` row SOW-045 |
> | Objective | OBJ-005 | `execution/_Decomposition/SOFTWARE_DECOMP.md` objective map |
> | Context envelope | L | `_CONTEXT.md` |
> | Setup status | Setup documents and semantic/dependency artifacts only | Human sealed brief |
>

### CLM-005 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary purpose | Bound a future evaluator that can evaluate user-defined rule-pack expressions against solver results and user-owned design bases. | SOW-045; OBJ-005 |
> | Required posture | Sandboxed, unit-aware, deterministic, and declarative. | `docs/SPEC.md` section 6; OPS-K-RULE-2; OPS-K-UNIT-1 |
> | Arbitrary executable code | Not permitted. | OPS-K-RULE-2 |
> | Unit and dimensional checks | Required for formulas, values, imports, exports, and evaluator results. | OPS-K-UNIT-1 |
> | Missing required values | Explicit findings, never silent defaults. | OPS-K-DATA-2 |
> | Expression grammar/library | TBD. No final grammar or library is selected by this setup run. | OI-006; sealed brief |
> | Protected formulas or code-derived values | Excluded from public setup artifacts. | OPS-K-IP-1; OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` |
> | Professional approval | Outside evaluator authority. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | OPS-K-MECH-2; OPS-K-AUTH-1; `docs/TYPES.md` section 4 |
>

### CLM-006 — Conditions

> ##### Conditions
>
> | Condition | Handling |
> |---|---|
> | Public repository boundary | Public artifacts may describe schemas, sandbox behavior, invented examples, and tests. They must not embed protected standards text, tables, proprietary formulas, material allowables, SIF/flexibility tables, or owner design-basis content. |
> | User-owned rule basis | User rule packs may contain private or licensed formulas and values in user-controlled paths. Public deliverables only define the safe evaluation boundary. |
> | Determinism | Future implementation must produce repeatable results and diagnostics for the same inputs, units, evaluator version, and rule-pack content. Exact numerical tolerances remain TBD. |
> | Diagnostics | Future outputs should use result-envelope diagnostics with code, class, severity, source, affected object, message, remediation, and provenance where applicable. |
> | Plugin and adapter boundary | Adapters and plugins must not bypass sandboxing, unit validation, provenance checks, diagnostics, or public/private data controls. |
>

### CLM-007 — Construction

> ##### Construction
>
> This setup run produces deliverable-local planning and readiness artifacts only:
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
> - `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` for semantic readiness.
> - `Dependencies.csv` and `_DEPENDENCIES.md` for dependency extraction.
> - `_run_records/` evidence for each required setup step.
>
> The anticipated implementation artifacts remain future work:
>
> - `rule evaluator module` - not produced in this setup run.
> - `evaluator tests` - not produced in this setup run.
>

### CLM-008 — References

> ##### References
>
> | Source | Use |
> |---|---|
> | `INIT.md` | Bootstrap boundaries and stop rules. |
> | `AGENTS.md` | TASK dispatch discipline and write-scope rule. |
> | `docs/CONTRACT.md` | Invariants OPS-K-RULE-2, OPS-K-UNIT-1, OPS-K-DATA-2, OPS-K-PRIV, OPS-K-IP, OPS-K-AGENT-1..4. |
> | `docs/SPEC.md` | Rule-pack evaluator section, diagnostic classes, reporting, and V&V expectations. |
> | `docs/TYPES.md` | Rule-pack check, user-rule checked status, human review boundary, and data provenance vocabulary. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private rule-pack and protected-data boundary. |
> | `docs/VALIDATION_STRATEGY.md` | Rule-pack evaluator verification families. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-06, DEL-06-02, AB-00 basis rows, OI-006. |
> | `docs/_Registers/Deliverables.csv` | Deliverable identity and risk note. |
> | `docs/_Registers/ScopeLedger.csv` | SOW-045 source row. |
> | `docs/_Registers/ContextBudgetQA.csv` | WATCH risk and split-if-expanded note. |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-06-02 Sandboxed unit-aware expression evaluator

> #### Specification: DEL-06-02 Sandboxed unit-aware expression evaluator
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-02-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> This setup specification bounds the future backend evaluator for user-defined rule-pack expressions. It covers sandboxing, unit awareness, variable binding, deterministic diagnostics, and public/private data boundaries.
>
> It does not implement an evaluator module, create evaluator tests, choose an expression grammar or library as final, invent numerical tolerances, include protected formulas, ship code-specific rule content, or claim professional code compliance.
>

### CLM-012 — PDU-024 Persistence Integration Boundary

> ##### PDU-024 Persistence Integration Boundary
>
> When user-supplied design-basis inputs or rule-check references are carried in a project model, downstream DEL-02-05 persistence uses the accepted model-document `0.2.0` family and established current/stale/unsupported/newer/failed version checks. This does not change the evaluator's binding contract, grammar, sandbox, or result-field scope.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | ReqID | Requirement | Source |
> |---|---|---|
> | REQ-06-02-001 | The future evaluator shall be sandboxed and incapable of arbitrary code execution. | SOW-045; OPS-K-RULE-2 |
> | REQ-06-02-002 | The evaluator shall remain declarative: rule packs define expressions and checks, not executable programs. | SOW-045; OI-006 |
> | REQ-06-02-003 | All expression inputs, intermediate quantities where represented, comparisons, and outputs shall be unit-aware and dimensionally checked. | OPS-K-UNIT-1 |
> | REQ-06-02-004 | Missing variables, missing required rule-check values, unit mismatches, invalid references, and unsupported expression forms shall produce explicit findings rather than silent defaults. | OPS-K-DATA-2; `docs/SPEC.md` section 7 |
> | REQ-06-02-005 | Variable binding shall be limited to declared rule-pack variables, solver result fields, and user-supplied design-basis inputs allowed by the sealed schema/interface contract. | `docs/SPEC.md` section 6; AB-00-02; AB-00-07 |
> | REQ-06-02-006 | The exact expression grammar, parser, and evaluator library remain TBD and require a future human architecture decision before implementation. | OI-006; sealed brief |
> | REQ-06-02-007 | Public artifacts shall not include protected standards text, tables, code-derived formulas, material allowables, SIF/flexibility data, proprietary vendor data, owner standards, or private rule packs. | OPS-K-IP-1; OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` |
> | REQ-06-02-008 | Rule-pack evaluation results shall not be represented as automatic code compliance, certification, sealing, approval, or professional reliance (PRD §21.2). | OPS-K-AUTH-1; OPS-K-MECH-2; `docs/TYPES.md` section 4 |
> | REQ-06-02-009 | Diagnostics and result envelopes shall preserve mechanics solved, user-rule checked, incomplete data, and human-approved state separation. | AB-00-03; AB-00-06 |
> | REQ-06-02-010 | Adapters and plugins shall not bypass sandboxing, unit checks, provenance checks, diagnostics, or public/private data boundaries. | AB-00-07 |
> | REQ-06-02-011 | Future tests shall cover unsafe expression rejection, dimension mismatch, missing binding, deterministic diagnostic emission, protected-content boundaries for public examples, and plugin/adapter bypass attempts. | `docs/VALIDATION_STRATEGY.md`; AB-00-08 |
>

### CLM-014 — Standards

> ##### Standards
>
> No external protected standard text is introduced by this setup. Governing local standards are the project bootstrap, invariant catalog, data-boundary policy, analysis-status vocabulary, sealed architecture-basis rows in `_CONTEXT.md`, and the decomposition/register rows for DEL-06-02 and SOW-045.
>

### CLM-015 — Open Decisions

> ##### Open Decisions
>
> | Decision | Current status | Required handling |
> |---|---|---|
> | Expression grammar and parser/evaluator library | TBD | Do not implement until a future sealed brief records the human architecture decision. |
> | Quantity representation and unit algebra integration point | TBD | Coordinate with the unit-system contract before implementation. |
> | Final diagnostic code taxonomy | TBD | Use explicit finding classes in setup; define final codes in implementation scope. |
> | Numerical tolerances for comparisons | TBD | Do not invent tolerances in setup artifacts. |
> | Variable namespace and result-field binding contract | TBD | Coordinate with rule-pack schema and result-envelope contracts. |
>

### CLM-016 — Verification

> ##### Verification
>
> | Requirement | Verification approach |
> |---|---|
> | REQ-06-02-001 | Security tests demonstrating that expressions cannot invoke filesystem, network, process, reflection, imports, or host-language execution paths. |
> | REQ-06-02-002 | Schema/evaluator tests proving accepted rule definitions are declarative expressions and checks only. |
> | REQ-06-02-003 | Unit tests for compatible dimensions, incompatible dimensions, comparison dimensions, and explicit unit metadata propagation. |
> | REQ-06-02-004 | Negative tests for missing variables, invalid references, unsupported operators, and missing user rule inputs. |
> | REQ-06-02-005 | Binding tests using only declared variables and result fields from the sealed interface. |
> | REQ-06-02-006 | Review gate check confirming grammar/library selection is recorded before implementation. |
> | REQ-06-02-007 | Protected-content and private-data review for public rule-pack examples and evaluator fixtures. |
> | REQ-06-02-008 | Status/result tests confirming no automatic `CODE_COMPLIANT` or professional approval state is emitted. |
> | REQ-06-02-009 | Result-envelope inspection for diagnostics, warnings, provenance, and state separation. |
> | REQ-06-02-010 | Adapter/plugin boundary tests proving no bypass path reaches the evaluator without validation. |
> | REQ-06-02-011 | CI or release-gate inclusion of deterministic evaluator verification. |
>

### CLM-017 — Documentation

> ##### Documentation
>
> Required setup artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`.
>
> Future implementation documentation should add the selected grammar/library decision record, evaluator threat model, unit-binding contract, diagnostic taxonomy, and test evidence. Those artifacts are outside this setup session.

- **AC-001** — The contract preserves the accepted no-arbitrary-code, no-side-effect, unit/provenance, protected-content, private-data, and professional-authority boundaries together with retained grammar/library and integration TBDs, without embedding proprietary rules, formulas, allowables, silent conversions, or compliance meaning.

## Production and Verification Method — Praxeology

### CLM-018 — Procedure: DEL-06-02 Sandboxed unit-aware expression evaluator

> #### Procedure: DEL-06-02 Sandboxed unit-aware expression evaluator
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-019 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-02-DECL-004`.
>

### CLM-020 — PDU-024 Integration Check

> ##### PDU-024 Integration Check
>
> 1. Validate evaluator inputs under the existing binding, unit, sandbox, and diagnostic contract.
> 2. For inputs carried in a project model, consume DEL-02-05 version-check/migration evidence rather than a local current-version literal.
> 3. Preserve stale/unsupported/newer/failed project-version diagnostics; do not alter evaluator grammar, binding namespaces, or fixtures in response to model-document version handling.
>

### CLM-021 — Purpose

> ##### Purpose
>
> Describe the setup-to-implementation procedure for a future sandboxed, unit-aware, declarative rule-pack expression evaluator. This procedure is operational guidance only; it does not implement the evaluator in this setup run.
>

### CLM-022 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status |
> |---|---|
> | Sealed deliverable context for DEL-06-02 | Present in `_CONTEXT.md`. |
> | Governing invariants and data boundary read | Completed for this setup run. |
> | Four-document setup kit | Produced by TASK+four-documents. |
> | Semantic matrix and lensing artifacts | Produced by TASK+semantic-matrix-build and TASK+lens-register. |
> | Dependency register | Produced by TASK+dependency-extract. |
> | Expression grammar/library decision | TBD; required before implementation. |
> | Unit-system integration contract | Interface dependency on DEL-02-02 or equivalent sealed unit contract. |
> | Rule-pack schema interface | Interface dependency on DEL-06-01 or equivalent sealed schema contract. |
>

### CLM-023 — Steps

> ##### Steps
>
> 1. Confirm the active sealed brief names only DEL-06-02, PKG-06, SOW-045, OBJ-005, applicable invariants, and explicit write scope.
> 2. Confirm no protected standards text, protected formulas, proprietary values, owner standards, private rule packs, or commercial examples are being introduced.
> 3. Before implementation, obtain a human architecture decision for the expression grammar/library and record any security constraints or rejected options.
> 4. Define the accepted declarative expression surface without host-language execution, imports, reflection, filesystem access, network access, process execution, or hidden side effects.
> 5. Bind variables only from declared rule-pack variables, solver result fields, and user-supplied design-basis inputs allowed by the sealed schema/interface contract.
> 6. Apply unit and dimensional checks to expression operands, comparisons, and outputs using the sealed unit-system contract.
> 7. Emit deterministic diagnostics for invalid expressions, unsupported constructs, missing variables, missing rule-check inputs, unit mismatches, and boundary violations.
> 8. Preserve state semantics: mechanics solved, rule inputs incomplete, user-rule checked, user-rule failed, and human review required are distinct outcomes.
> 9. Add tests for unsafe expression rejection, unit mismatch, missing binding, deterministic diagnostics, public/private data boundaries, and adapter/plugin bypass prevention.
> 10. Route implementation and test evidence to REVIEW before any lifecycle transition beyond development draft states.
>

### CLM-024 — Verification

> ##### Verification
>
> | Check | Evidence |
> |---|---|
> | Four-document kit exists | `tools/validation/check_four_documents.sh <deliverable path>` |
> | Semantic artifact exists and audit states PASS | `_SEMANTIC.md#Audit-Result` |
> | Lensing coverage exists for matrices A, B, C, F, D, X, and E | `_SEMANTIC_LENSING.md` |
> | Dependency register is schema-valid | `python3 tools/validation/validate_dependencies_schema.py <deliverable path>/Dependencies.csv` |
> | Status value is valid | `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY` |
> | No ISSUED transition occurred | `_STATUS.md` current state remains `SEMANTIC_READY` |
>

### CLM-025 — Records

> ##### Records
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_STATUS.md`
> - `_run_records/TASK_RUN_2026-04-30_1032_four-documents-p1-p2.md`
> - `_run_records/TASK_RUN_2026-04-30_1032_semantic-matrix-build.md`
> - `_run_records/TASK_RUN_2026-04-30_1032_lens-register.md`
> - `_run_records/TASK_RUN_2026-04-30_1032_four-documents-p3.md`
> - `_run_records/TASK_RUN_2026-04-30_1032_dependency-extract.md`

- **VER-001** — Validate the contract and review source parity, grammar and sandbox boundaries, dimensional compatibility, persistence/version binding, deterministic evaluation and diagnostics, resource constraints, protected-content controls, and every unresolved governed item.

## Governing Values and Decisions — Axiology

### CLM-026 — Guidance: DEL-06-02 Sandboxed unit-aware expression evaluator

> #### Guidance: DEL-06-02 Sandboxed unit-aware expression evaluator
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-027 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-06-02-DECL-003`.
>

### CLM-028 — PDU-024 Guidance

> ##### PDU-024 Guidance
>
> Treat project model-document compatibility as downstream DEL-02-05 authority. Evaluator-facing UI evidence must not duplicate a current schema-version literal or normalize away stale, unsupported, newer, or failed project-version diagnostics. Keep binding, sandbox, unit, and missing-data behavior unchanged.
>

### CLM-029 — Purpose

> ##### Purpose
>
> This deliverable exists to make user-defined rule-pack checks evaluable without turning rule packs into arbitrary executable code. The evaluator boundary should let users apply their own design bases to solver results while preserving unit safety, explicit missing-data findings, protected-data boundaries, and professional-responsibility limits.
>

### CLM-030 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Declarative before executable | Treat rule packs as data plus expressions. Do not permit host-language code, imports, reflection, filesystem access, network access, process execution, or hidden side effects. |
> | Unit safety is structural | Unit compatibility is not a UI nicety. It is part of evaluator correctness and should fail visibly when incompatible or missing. |
> | User-owned code data stays user-owned | Public setup and examples can define structure and invented checks only. Licensed formulas, protected interpretations, and owner design bases belong in private user rule packs. |
> | Findings over defaults | Missing variables, missing rule inputs, incompatible dimensions, and unsupported constructs should produce diagnostics. They should not be guessed or defaulted. |
> | Rule checked is not code compliant | `USER_RULE_CHECKED` and `USER_RULE_FAILED` are software computations using user data. Professional reliance still requires competent human review. |
>

### CLM-031 — Considerations

> ##### Considerations
>
> - The evaluator is security-sensitive and numerically important, so the large context envelope is justified, but expansion beyond sandboxing/unit-awareness should be split or escalated.
> - The exact expression grammar/library is deliberately unresolved. A future implementation brief should record the selected option, rejected options, threat model, and validation evidence.
> - Unit algebra should coordinate with the unit-system contract rather than duplicate a second unit model.
> - Rule-pack schema, required-input completeness, private lifecycle/checksum handling, and invented examples are sibling deliverables. This deliverable should not absorb those scopes.
> - Diagnostics should be deterministic and suitable for result envelopes, reports, and review without exposing protected formulas in public examples.
> - Public test fixtures should use invented or original examples only.
>

### CLM-032 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Setup guidance |
> |---|---|
> | Expressiveness vs sandbox strength | Prefer a small declarative surface that can be validated and audited. Do not trade sandbox guarantees for convenience. |
> | Unit richness vs implementation scope | Require dimensional checks, but leave final quantity representation and tolerance choices to the future sealed implementation brief. |
> | Detailed diagnostics vs protected-content leakage | Diagnostics should identify missing inputs and rule-check findings without reproducing licensed formulas or protected standards language in public artifacts. |
> | Plugin flexibility vs governance | Plugins and adapters may provide data, but they cannot bypass validation, sandboxing, provenance, diagnostics, or public/private boundaries. |
>

### CLM-033 — Examples

> ##### Examples
>
> No protected formulas or code-derived examples are provided. Acceptable future public examples should use invented non-engineering values and clear notices. Conceptual examples for test planning include:
>
> - A malformed expression is rejected with a deterministic diagnostic.
> - A comparison between incompatible dimensions is rejected with a unit-mismatch finding.
> - A missing user-supplied variable produces a `RULE_CHECK_BLOCKING` style finding.
> - A successful invented check is reported as user-rule checked, not professionally approved.
>

### CLM-034 — Semantic Enrichment Notes

> ##### Semantic Enrichment Notes
>
> Pass 3 reviewed `_SEMANTIC_LENSING.md` and incorporated the warranted setup improvements as bounded notes:
>
> | Lensing item | Disposition |
> |---|---|
> | Unsafe expression and bypass test coverage | Added to `Specification.md` verification and `Procedure.md` checks. |
> | Diagnostic-code taxonomy | Retained as `TBD` in `Specification.md` open decisions. |
> | Grammar/library decision | Retained as `TBD` and future human architecture decision. |
> | Vocabulary normalization | Standardized on evaluator, expression, variable binding, rule-pack check, user-rule checked, and human review boundary. |
> | Future interface handoff | Recorded as an implementation-stage dependency/proposal rather than a setup implementation step. |
>

### CLM-035 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | OI-006 | Expression grammar/library is required before implementation but remains unresolved. | `execution/_Decomposition/SOFTWARE_DECOMP.md` open issue OI-006 | `docs/SPEC.md` section 6 requires sandboxed unit-aware evaluator | `Specification.md#Open-Decisions`; `Procedure.md#Prerequisites` | Future human architecture decision should seal grammar/library before implementation. | TBD |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-045 OBJ-005 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
