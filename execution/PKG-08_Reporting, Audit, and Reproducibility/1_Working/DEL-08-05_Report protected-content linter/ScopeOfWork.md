---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-05
package_id: PKG-08
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-043]
package_objective_refs: [OBJ-002, OBJ-007]
---

# Scope of Work — DEL-08-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-08-05` in service of project scope [SOW-043] and package objectives [OBJ-002, OBJ-007].

- **OUT-001** — A protected-content report-linting contract covering configured public report surfaces, conservative heuristic findings, safe synthetic fixtures, deterministic diagnostics, quarantine and review routing, optional private scanning, and CI integration boundaries is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-08-05 Report protected-content linter

> #### Datasheet: DEL-08-05 Report protected-content linter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-05-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-08-05 |
> | Deliverable Name | Report protected-content linter |
> | Package ID | PKG-08 |
> | Package Name | Reporting, Audit, and Reproducibility |
> | Deliverable Type | TEST_SUITE |
> | Scope Item | SOW-043 |
> | Objectives | OBJ-002, OBJ-007 |
> | Context Envelope | M |
> | Current Setup Boundary | Document/setup artifacts only; no linter source, CI guard, tests, report templates, or repo-level artifacts are implemented in this session. |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Primary purpose | Define setup expectations for checks that prevent public report templates/examples from automatically embedding protected code text, copied standards tables, proprietary formulas, or private/proprietary engineering content. |
> | Public/private boundary | Public templates and examples must stay protected-content-free. User-private report templates remain user responsibility. |
> | Review posture | Heuristic plus review; this linter cannot be the sole legal/IP control. |
> | Anticipated future artifacts | `report linter`; `CI guard` |
> | Authorized current artifacts | Four-document setup kit, semantic artifacts, dependency register, run records, status update. |
> | Explicit exclusions | No linter source, no CI workflow edits, no test fixtures embedding protected examples, no legal-sufficiency claim, no `ISSUED` movement. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | SourceRef | Disposition |
> |---|---|---|
> | Public repository must not contain protected standards text, tables, figures, examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Section 3 | Binding guardrail for future linter scope. |
> | Suspected protected content must be quarantined and escalated; agents must not paraphrase protected tables into public data. | `docs/CONTRACT.md` OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` Section 5 | Linter findings must route to review rather than automatic acceptance. |
> | Public report templates and examples must not reproduce protected standards content. | `docs/CONTRACT.md` OPS-K-REPORT-2; `docs/SPEC.md` Section 8 | Direct requirement for this deliverable. |
> | Reports must not claim certification, sealing, approval, authentication, or code compliance for reliance (PRD §21.2). | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Section 4 | Linter scope may include prohibited public-claim language, but human review remains required. |
> | Reports may reference user rule-pack IDs, versions, checksums, and source notes without embedding protected formulas or tables. | `docs/IP_AND_DATA_BOUNDARY.md` Section 7; `docs/SPEC.md` Sections 6 and 8 | Safe metadata is allowed; rule-pack content remains user/private unless rights are documented. |
> | Telemetry/private data boundaries must be preserved. | `docs/CONTRACT.md` OPS-K-PRIV-1; OPS-K-PRIV-2 | Future linter should not transmit or collect private project/rule data. |
>

### CLM-006 — Construction

> ##### Construction
>
> The future linter should be framed as a conservative protected-content and public-report guard. It should inspect only authorized public report-template/example surfaces unless a user explicitly runs it against private material. It should flag suspected protected or private content for human review, not decide legal sufficiency or professional acceptability.
>
> Expected future check categories:
>
> | Category | Intended behavior | Boundary |
> |---|---|---|
> | Protected text/table/formula indicators | Detect likely copied standards/code/vendor content in public templates/examples using heuristic patterns and allowlisted safe placeholders. | No protected examples may be embedded as fixtures. |
> | Public/private marker checks | Require explicit public/private or redistribution-status metadata where template/example content may expose rule-pack or provenance text. | Unknown status is a finding, not a silent pass. |
> | Prohibited public claims | Flag public report language that claims software certification, sealing, approval, authentication, or automatic code compliance (PRD §21.2). | The check supports OPS-K-AUTH-1 but does not replace human review. |
> | Safe metadata allowance | Permit rule-pack identity, version, checksum, source notice, redistribution status, and user-supplied notices when represented as metadata. | Do not reproduce protected rule formulas or tables in public artifacts. |
> | Review routing | Emit actionable diagnostics such as `IP_BOUNDARY_WARNING` and review-required findings. | Do not auto-quarantine outside future authorized implementation scope. |
>

### CLM-007 — References

> ##### References
>
> | Reference | Use |
> |---|---|
> | `INIT.md` | Root bootstrap and agent constraints. |
> | `AGENTS.md` | TASK dispatch and package role constraints. |
> | `docs/CONTRACT.md` | Invariants OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-RULE-1/3, OPS-K-AUTH-1, OPS-K-PRIV-1/2, OPS-K-AGENT-1..4, OPS-K-REPORT-2. |
> | `docs/SPEC.md` | Reporting/audit requirements, warning classes, rule-pack metadata boundary, V&V expectations. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy, quarantine rule, report boundary. |
> | `docs/TYPES.md` | Analysis status vocabulary, protected-content vocabulary, report object boundary. |
> | `docs/DIRECTIVE.md` | Founding stop rules and professional/IP responsibility constraints. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-08 and DEL-08-05 decomposition basis, SOW-043, OBJ-002, OBJ-007, architecture-basis IDs. |
> | `docs/_Registers/Deliverables.csv` | DEL-08-05 register row. |
> | `docs/_Registers/ScopeLedger.csv` | SOW-043 register row. |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-08-05 Report protected-content linter

> #### Specification: DEL-08-05 Report protected-content linter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-05-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies setup requirements for a protected-content linter that will guard public report templates and examples against accidental inclusion of protected standards/code text, copied standards tables, proprietary formulas, private rule-pack content, or misleading professional-authority claims.
>
> In scope for future implementation:
>
> - checks over public report templates/examples and other explicitly configured public report surfaces;
> - heuristic detection of protected-content risk categories without embedding protected examples as fixtures;
> - diagnostics that route suspected protected/private content to human review;
> - checks that public report language does not claim software certification, sealing, approval, authentication, or automatic code compliance (PRD §21.2);
> - CI guard integration when separately authorized by implementation scope.
>
> Out of scope for this setup session:
>
> - writing linter source code;
> - adding or modifying CI guards;
> - adding tests or protected-content fixtures;
> - editing report templates or report generator source;
> - making legal sufficiency, compliance, certification, or professional-reliance claims;
> - moving the deliverable to `ISSUED`.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | Requirement ID | Requirement | Source | Verification |
> |---|---|---|---|
> | DEL-08-05-REQ-001 | The future linter shall scan only authorized public report-template/example surfaces by default, not user-private templates or private project/rule data. | `docs/IP_AND_DATA_BOUNDARY.md` Sections 6-7; `docs/CONTRACT.md` OPS-K-PRIV-1 | Future unit/integration tests verify public-surface targeting and private-surface opt-in behavior. |
> | DEL-08-05-REQ-002 | The future linter shall flag suspected protected code text, copied standards tables, protected figures/examples, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data in public templates/examples. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` Section 3; `docs/SPEC.md` Section 8 | Future linter fixtures use invented/synthetic trigger placeholders and safe negative fixtures; no protected examples are embedded. |
> | DEL-08-05-REQ-003 | The future linter shall not treat a clean heuristic scan as legal clearance or professional approval. | `docs/CONTRACT.md` OPS-K-AGENT-4; `docs/_Registers/ContextBudgetQA.csv` row DEL-08-05 | Future report and CI output includes review-required wording; review checklist confirms no legal-sufficiency claim. |
> | DEL-08-05-REQ-004 | The future linter shall route suspected protected content to quarantine/review workflow signals rather than accepting, paraphrasing, or normalizing the content into public artifacts. | `docs/CONTRACT.md` OPS-K-IP-3; `docs/IP_AND_DATA_BOUNDARY.md` Section 5 | Future tests verify suspected-content diagnostics and review routing. |
> | DEL-08-05-REQ-005 | The future linter shall permit safe report metadata such as rule-pack ID, version, checksum, source notice, redistribution status, and required-input status when the metadata does not reproduce protected formulas or tables. | `docs/SPEC.md` Sections 6 and 8; `docs/IP_AND_DATA_BOUNDARY.md` Section 7; `docs/CONTRACT.md` OPS-K-RULE-3 | Future fixtures verify metadata allowance and protected rule-content rejection. |
> | DEL-08-05-REQ-006 | The future linter shall flag public report wording that claims the software certifies, seals, approves, authenticates, or declares engineering code compliance for reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Sections 4 and 8; `docs/DIRECTIVE.md` Sections 4-6 | Future text fixtures verify prohibited-claim detection while permitting decision-support wording. |
> | DEL-08-05-REQ-007 | The future linter shall surface findings using diagnostics compatible with report/result warning contracts, including `IP_BOUNDARY_WARNING` where applicable. | `docs/SPEC.md` Section 7; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06 | Future diagnostic output tests verify code/class/severity/source/remediation fields where available. |
> | DEL-08-05-REQ-008 | Public examples used to exercise the linter shall use invented non-code values or synthetic markers and shall clearly avoid standards-body/vendor copied content. | `docs/CONTRACT.md` OPS-K-RULE-1; `docs/SPEC.md` Section 9 | Future fixture review confirms invented-only content and source/provenance notes. |
> | DEL-08-05-REQ-009 | The future linter shall be deterministic for the same input files, configuration, and version so that CI and review evidence are reproducible. | `docs/SPEC.md` Section 9; `docs/CONTRACT.md` OPS-K-REPORT-1 | Future tests compare stable machine-readable finding output. |
> | DEL-08-05-REQ-010 | The future CI guard shall fail or warn according to a documented severity policy, with suspected protected content treated as review-blocking until human/legal disposition is recorded. | `docs/CONTRACT.md` OPS-K-IP-3; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` Sections 4-5 | Future CI tests verify severity mapping and review handoff behavior. |
> | DEL-08-05-REQ-011 | The future linter shall not transmit private project, material, component, or rule-pack data by default. | `docs/CONTRACT.md` OPS-K-PRIV-1 and OPS-K-PRIV-2 | Future security/privacy tests verify no telemetry or external transfer by default. |
> | DEL-08-05-REQ-012 | The future implementation shall preserve schema-first result-envelope and no-bypass adapter/plugin constraints from the architecture basis where linter output integrates with reports, APIs, or CI. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03, AB-00-06, AB-00-07, AB-00-08 | Architecture review verifies integration boundaries before implementation acceptance. |
>

### CLM-012 — Standards

> ##### Standards
>
> No protected standards text, standards tables, protected examples, or proprietary formulas are needed or authorized for this deliverable. Applicable public project standards are the OpenPipeStress governance artifacts listed in `_REFERENCES.md`, especially `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/TYPES.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md`.
>

### CLM-013 — Verification

> ##### Verification
>
> Future implementation work should provide at least these checks:
>
> | Check | Purpose |
> |---|---|
> | Public-surface scan fixture | Verifies only configured public report-template/example paths are scanned by default. |
> | Synthetic protected-risk fixture | Verifies invented markers trigger protected-content findings without copying protected content. |
> | Safe metadata fixture | Verifies rule-pack identity/version/checksum/source notice can appear without embedding protected rule content. |
> | Prohibited-claim fixture | Verifies public report language does not claim certification, sealing, approval, authentication, or automatic code compliance (PRD §21.2). |
> | Diagnostic output fixture | Verifies findings carry stable code/class/severity/source/remediation fields where available. |
> | Determinism check | Verifies stable output for the same inputs/config/version. |
> | CI policy check | Verifies severity-to-fail/warn mapping and review-blocking behavior. |
> | Privacy check | Verifies no private data transmission by default. |
> | Human review checklist | Confirms the linter is treated as heuristic support, not sole legal control. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required setup artifacts for this run:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/*`
> - `_STATUS.md`
>
> Future production artifacts anticipated by the register:
>
> - report linter;
> - CI guard.
>

### CLM-015 — Acceptance Criteria For This Setup Session

> ##### Acceptance Criteria For This Setup Session
>
> - No file outside `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/` is edited.
> - No linter source, CI guard, tests, report templates, docs outside this deliverable, or repo-level artifacts are modified.
> - Setup artifacts are grounded in local governance, decomposition, register, and context files.
> - No protected standards content, proprietary examples, private project/rule data, or certification/compliance claim is introduced.
> - `Dependencies.csv` validates against v3.1 schema and active rows contain evidence.
> - `_STATUS.md` reports `SEMANTIC_READY` only after the four-document kit, semantic artifacts, dependency register, and validation checks are complete.

- **AC-001** — The contract preserves the public/private and redistribution boundary, flags suspected protected or prohibited-authority content without asserting legal sufficiency, uses no protected fixtures, keeps severity and review ownership TBD where unresolved, and retains human/legal review as an independent control.

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-08-05 Report protected-content linter

> #### Procedure: DEL-08-05 Report protected-content linter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-017 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-05-DECL-004`.
>

### CLM-018 — Purpose

> ##### Purpose
>
> This procedure records how DEL-08-05 setup artifacts are produced and how future implementation work should approach the protected-content linter without exceeding the public IP/data boundary.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Source | Status |
> |---|---|---|
> | Sealed deliverable context for DEL-08-05 with explicit write scope. | User brief; `_CONTEXT.md` | Available for this setup run. |
> | Local governance references for IP/data, report boundary, professional boundary, and agent constraints. | `_REFERENCES.md`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/TYPES.md`; `docs/DIRECTIVE.md` | Available. |
> | Decomposition/register scope for SOW-043, OBJ-002, OBJ-007. | `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/*.csv` | Available. |
> | Future report template/example locations. | DEL-08-01 and future implementation work | TBD; not required for setup artifacts. |
> | Future CI guard location and policy. | DEL-10-04 or later authorized implementation work | TBD; not modified in this setup session. |
>

### CLM-020 — Steps

> ##### Steps
>

### CLM-021 — Setup sequence used in this session

> ###### Setup sequence used in this session
>
> 1. Read the sealed context and required governance/decomposition files.
> 2. Run `four-documents` with `RUN_PASSES=P1_P2` by drafting `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` from accessible local sources.
> 3. Run `semantic-matrix-build` by replacing the placeholder `_SEMANTIC.md` with deliverable-bound semantic matrices and recording audit PASS.
> 4. Run `lens-register` by creating `_SEMANTIC_LENSING.md` with complete matrix-cell coverage.
> 5. Run `four-documents` with `RUN_PASSES=P3_ONLY` by applying warranted lensing findings or recording that no document rewrite was warranted.
> 6. Run `dependency-extract` by creating `Dependencies.csv` v3.1 and refreshing `_DEPENDENCIES.md`.
> 7. Validate setup gates and set `_STATUS.md` to `SEMANTIC_READY` only after checks pass.
>

### CLM-022 — Future implementation procedure

> ###### Future implementation procedure
>
> 1. Confirm authorized scan surfaces.
>    - Enumerate public report template/example paths.
>    - Keep private/user template paths out of default scanning.
>    - Record opt-in behavior for local private scans if supported.
> 2. Define safe linter finding categories.
>    - Protected content risk.
>    - Private data risk.
>    - Unknown/weak redistribution status.
>    - Prohibited professional-authority claim.
>    - Review-required or quarantine-required disposition.
> 3. Build fixtures without protected content.
>    - Use invented markers, placeholders, and synthetic table/formula shapes.
>    - Do not copy standards text, standards tables, proprietary formulas, vendor tables, or commercial examples into fixtures.
> 4. Implement deterministic scan behavior.
>    - Stable inputs/config/version produce stable findings.
>    - Findings cite file and best-effort location.
>    - Unknowns become review-required findings.
> 5. Integrate diagnostics and CI policy.
>    - Emit diagnostic classes compatible with the project result-envelope/warning model.
>    - Map severity to fail/warn/review-required behavior only after policy approval.
> 6. Verify boundaries.
>    - Check no private data transmission occurs by default.
>    - Check public templates/examples remain protected-content-free.
>    - Check public report language avoids certification, sealing, approval, authentication, and automatic code-compliance claims (PRD §21.2).
> 7. Prepare review evidence.
>    - Include test results, fixture provenance, known limitations, and residual risk notes.
>    - State that heuristic linting is not sole legal control.
>

### CLM-023 — Verification

> ##### Verification
>
> Setup verification for this session:
>
> - Four production documents exist and contain the default schema sections.
> - `_SEMANTIC.md` contains canonical matrices A/B and derived matrices C/F/D/K/G/X/T/E with audit PASS.
> - `_SEMANTIC_LENSING.md` covers matrices A/B/C/F/D/X/E.
> - `Dependencies.csv` validates against the v3.1 required-column schema.
> - Dependency enum values validate against the local enum validator.
> - `_STATUS.md` reports `SEMANTIC_READY`.
> - No protected examples, proprietary formulas, private data, linter source, CI edits, report-template edits, or repo-level artifacts are introduced.
>

### CLM-024 — Records

> ##### Records
>
> | Record | Purpose |
> |---|---|
> | `Datasheet.md` | Descriptive setup facts and boundary conditions. |
> | `Specification.md` | Normative setup requirements and future verification targets. |
> | `Guidance.md` | Rationale, principles, trade-offs, examples, and open questions. |
> | `Procedure.md` | Setup and future implementation workflow. |
> | `_SEMANTIC.md` | Semantic matrix setup lens. |
> | `_SEMANTIC_LENSING.md` | Coverage-complete lensing register. |
> | `Dependencies.csv` | Machine-readable dependency register v3.1. |
> | `_DEPENDENCIES.md` | Human-readable dependency summary and run notes. |
> | `_run_records/*` | Setup run evidence for each required step. |
> | `_STATUS.md` | Lifecycle state and history. |

- **VER-001** — Validate the contract and review source parity, authorized scan surfaces, synthetic fixture safety, stable finding traceability, private opt-in behavior, fail/warn/quarantine residuals, professional-claim detection, human/legal review boundaries, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-08-05 Report protected-content linter

> #### Guidance: DEL-08-05 Report protected-content linter
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-08-05-DECL-003`.
>

### CLM-027 — Purpose

> ##### Purpose
>
> The protected-content linter exists to reduce the risk that public report templates or examples accidentally carry content the OpenPipeStress public repository must not redistribute. It supports the IP/data boundary and reproducible professional review, but it remains a heuristic guard plus review workflow, not a legal opinion or engineering acceptance decision.
>

### CLM-028 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Protect public artifacts | Default scanning should target public report templates/examples and configured public report surfaces. |
> | Preserve private responsibility | User-private report templates, licensed standards quotes, owner design bases, and private rule packs remain user responsibility unless the user explicitly opts into local scanning. |
> | Use synthetic fixtures | Future tests should use invented markers and safe placeholders rather than copied standards/code/vendor content. |
> | Flag, do not certify | Findings should route suspected content to quarantine/review. A clean scan should not be described as legally sufficient. |
> | Allow safe metadata | Rule-pack IDs, versions, checksums, source notices, redistribution status, and required-input status can be safe when they do not expose protected content. |
> | Keep professional boundary visible | Public report text should avoid claims of certification, sealing, approval, authentication, or automatic code compliance (PRD §21.2). |
> | Keep diagnostics reproducible | Finding output should be stable, traceable to files/locations, and suitable for CI and review records. |
>

### CLM-029 — Considerations

> ##### Considerations
>
> The linter cannot know whether every phrase, formula, or table is protected. It should combine conservative heuristic checks, required metadata, safe fixture policy, and human/legal review routing. False positives are acceptable when they protect the public repository boundary; false negatives are a residual risk that review must address.
>
> Public report templates should prefer placeholders, invented values, and metadata references over copied explanatory text or engineering formula bodies. User-private report templates may include licensed or proprietary content under user control, but the public project should not assume those templates are redistributable.
>
> The future implementation should avoid a fixture strategy that recreates the very material it is intended to detect. Synthetic markers, invented table structures, and non-code phrases are preferable to copied source examples.
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Topic | Conservative direction |
> |---|---|
> | Detection breadth vs false positives | Prefer review-blocking warnings for plausible protected-content risk in public artifacts. |
> | Pattern specificity vs protected fixture risk | Use safe synthetic markers, metadata checks, and structural heuristics instead of copied protected examples. |
> | CI automation vs legal judgment | CI can block or warn based on policy, but legal/provenance disposition remains a human process. |
> | Public template convenience vs IP boundary | Public templates should be sparse and metadata-driven where protected content might otherwise be copied. |
> | User-private template support vs privacy | Local opt-in scanning can help users, but default behavior must not transmit or commit private content. |
>

### CLM-031 — Examples

> ##### Examples
>
> Safe public examples may use:
>
> - invented placeholder notices;
> - synthetic rule-pack IDs and checksums;
> - fake non-code formula markers that are explicitly marked as invented;
> - empty or schematic table headings that do not reproduce protected table content;
> - professional-review notices that describe decision support without claiming approval.
>
> Unsafe public-example patterns include:
>
> - copied standards-body explanatory text;
> - copied standards or vendor tables;
> - copied proprietary formulas or formula commentary;
> - private owner/rule-pack text committed as a public template;
> - report wording that states the software certifies or declares code compliance for reliance.
>
> These examples are categorical only. They intentionally do not include protected source text or copied proprietary values.
>

### CLM-032 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> No source conflicts were identified in the local reference set for this setup run.
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | None identified | N/A | N/A | N/A | N/A | N/A |
>

### CLM-033 — Open Questions

> ##### Open Questions
>
> | ID | Question | Current disposition |
> |---|---|---|
> | OQ-08-05-001 | Exact linter implementation language/library and integration point. | TBD; future implementation-level decision. |
> | OQ-08-05-002 | Exact public template/example path list to scan by default. | TBD; should be resolved when report template locations exist. |
> | OQ-08-05-003 | Exact severity policy for fail, warn, quarantine, and review-required outcomes. | TBD; must be confirmed before CI guard implementation. |
> | OQ-08-05-004 | Exact diagnostic schema fields for linter output. | TBD; align with AB-00-06 result-envelope/diagnostic basis. |
> | OQ-08-05-005 | Human/legal review ownership for protected-content findings. | TBD; governance process must record disposition authority. |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-043 OBJ-002 OBJ-007 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
