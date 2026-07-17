---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-07-07
package_id: PKG-07
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@eaad463c0d481f6f1654e6adb5ee718f566176e9
project_scope_refs: [SOW-055]
package_objective_refs: [OBJ-006, OBJ-007]
---

# Scope of Work — DEL-07-07

## Purpose and Objective Traceability

This Scope of Work defines `DEL-07-07` in service of project scope [SOW-055] and package objectives [OBJ-006, OBJ-007].

- **OUT-001** — A solve-execution UX contract covering background job launch, service-reported progress, cancellation requests and terminal states, diagnostics, blocked-state separation, reproducibility signals, and professional-review boundaries is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

> #### Datasheet: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-07-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-07-07 |
> | Deliverable name | Solve execution UX: progress, cancellation, and diagnostics |
> | Package ID | PKG-07 |
> | Package name | Graphical User Interface and Engineering Workflow |
> | Type | UX_UI_SLICE |
> | Scope item | SOW-055 |
> | Objectives | OBJ-006, OBJ-007 |
> | Context envelope | M |
> | Setup state | SEMANTIC_READY candidate after setup gates pass |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Setup value |
> |---|---|
> | UX surface | Solve run panel, progress/cancel controls, diagnostic presentation |
> | Execution mode | Background solve execution through application-service command/job boundaries |
> | Progress basis | Report only progress states, phases, or measures supplied by the job contract; exact progress semantics remain TBD |
> | Cancellation basis | User cancellation is routed through the application-service job cancellation contract; direct GUI bypass of solver state is out of scope |
> | Diagnostics basis | Diagnostic/result-envelope contract from PKG-00, including code, class, severity, source, affected object, message, remediation, and provenance where available |
> | Result review basis | Mechanics solve status, missing data, assumptions, diagnostics, and reproducibility signals remain visible for professional review |
> | Implementation status | No GUI source code, tests, schemas, job code, or solver code is implemented by this setup run |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Treatment |
> |---|---|
> | Missing solve-required inputs | Presented as `SOLVE_BLOCKING` diagnostics or equivalent envelope entries; never silently defaulted |
> | Missing rule-check inputs | Presented separately from solve readiness as `RULE_CHECK_BLOCKING` or equivalent diagnostics |
> | Nonlinear or numerical uncertainty | Presented as solver diagnostics and warning classes rather than hidden state |
> | Protected standards/code data | Not introduced; any code-specific or proprietary data remains user-supplied/private |
> | Professional reliance | Human review remains required; acceptance stays with the responsible engineer |
> | Reproducibility | Run records should preserve model/version/hash/checksum inputs when provided by upstream contracts; exact fields remain TBD |
>

### CLM-006 — Construction

> ##### Construction
>
> This setup constructs only deliverable-local planning artifacts:
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
> Future implementation work must remain bounded by the GUI/application-service seam defined by PKG-00. It should consume command/job/progress/cancellation and diagnostics/result-envelope contracts rather than inventing GUI-owned solver semantics.
>

### CLM-007 — References

> ##### References
>
> | Reference | Use |
> |---|---|
> | `INIT.md` | Bootstrap boundaries: protected data, mechanics vs rule check, human authority |
> | `AGENTS.md` | Type 2 sealed deliverable execution and write-scope discipline |
> | `docs/CONTRACT.md` | Invariants for data, units, IP, privacy, authority, and agents |
> | `docs/DIRECTIVE.md` | Product stop rules and no-silent-default principles |
> | `docs/SPEC.md` | GUI warnings, result/report expectations, and layer boundaries |
> | `docs/TYPES.md` | Analysis-status and professional-boundary vocabulary |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | DEL-07-07 package, scope, objectives, and architecture-basis rows |
> | `docs/_Registers/Deliverables.csv` | Deliverable row for DEL-07-07 |
> | `docs/_Registers/ScopeLedger.csv` | SOW-055 row |
> | `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-03_Application service command-query-job model/Specification.md` | Command, job, cancellation, progress, and result-envelope boundary |
> | `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-05_GUI state and interaction architecture/Specification.md` | Durable/transient GUI state and job-progress state separation |
> | `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-06_Diagnostics, warning, and result-envelope contract/Specification.md` | Diagnostics fields, warning classes, and no-compliance-claim boundary |

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

> #### Specification: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-009 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-07-DECL-001`.
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the setup basis for a GUI solve-execution workflow that keeps background solve execution, progress display, cancellation, diagnostic logs, solver warnings, and result-envelope status reviewable.
>
> This setup does not implement GUI source code, UI tests, schemas, application-service code, background job code, solver code, report/export code, package manifests, or protected engineering data. Future implementation must consume the command/job/progress/cancellation and diagnostics/result-envelope contracts supplied by PKG-00 rather than inventing product semantics in the GUI slice.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source |
> |---|---|---|
> | REQ-07-07-001 | The solve UX shall initiate solve execution through an application-service command or job contract and shall not directly mutate domain-core or solver state from GUI components. | `_CONTEXT.md` Architecture Basis Injection; DEL-00-03 REQ-03-01 and REQ-03-03 |
> | REQ-07-07-002 | The solve UX shall treat solve execution as a background job where cancellation, progress, and reproducibility metadata are provided by the service/job contract. Exact progress phases, percentages, and event names remain `TBD` until the job contract fixes them. | DEL-00-03 REQ-03-04; SOW-055 |
> | REQ-07-07-003 | Progress presentation shall display only service-reported states, phases, counts, or messages; it shall not synthesize misleading completion percentages when the job contract does not supply them. | OPS-K-AGENT-1/2; DEL-00-03 REQ-03-04 |
> | REQ-07-07-004 | Cancellation controls shall issue a cancellation request through the application-service boundary and shall preserve the final job/result-envelope state returned by that boundary. The GUI shall not bypass cancellation boundaries or claim successful cancellation without envelope evidence. | DEL-00-03 REQ-03-04; OPS-K-AGENT-1 |
> | REQ-07-07-005 | Diagnostic presentation shall preserve machine-readable diagnostic fields for code, class, severity, source, affected object, message, remediation, and provenance where supplied. | DEL-00-06 REQ-06-01 |
> | REQ-07-07-006 | The solve UX shall preserve warning classes for `SOLVE_BLOCKING`, `RULE_CHECK_BLOCKING`, `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `NONLINEAR_WARNING`, and `IP_BOUNDARY_WARNING` when those classes are present in diagnostics/result envelopes. | `docs/SPEC.md` section 7; DEL-00-06 REQ-06-02 |
> | REQ-07-07-007 | The solve UX shall distinguish invalid input, incomplete model, mechanics solved, rule-check result, and human-review-needed states; it shall not expose automatic code-compliance, certification, approval, sealing, or professional-authentication states (PRD §21.2). | DEL-00-06 REQ-06-04/05; `docs/TYPES.md` section 4; OPS-K-AUTH-1 |
> | REQ-07-07-008 | Solve-required missing data and rule-check-required missing data shall remain separate visible findings. Missing values shall not be supplied by the GUI as silent defaults. | OPS-K-DATA-1/2/3; `docs/DIRECTIVE.md` section 3 |
> | REQ-07-07-009 | Any solve-run review summary intended for reports or exports shall retain traceability to available reproducibility metadata such as model hash, input manifest, solver version, rule-pack version/checksum, warnings, assumptions, and limitations. | OBJ-007; `docs/SPEC.md` section 8; DEL-00-03 REQ-03-04 |
> | REQ-07-07-010 | Private project data, private rule-pack data, proprietary values, and protected standards/code content shall not be transmitted, displayed as public defaults, or copied into public setup artifacts. | OPS-K-IP-1/2/3; OPS-K-PRIV-1/2; OPS-K-DATA-1 |
> | REQ-07-07-011 | Future implementation evidence shall include UI tests or equivalent review evidence for background job state rendering, progress display, cancellation request/terminal-state handling, diagnostic filtering/detail review, and professional-boundary notices. | DEL-07-07 anticipated artifacts; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` |
>

### CLM-012 — Standards

> ##### Standards
>
> No external engineering code or protected standards text is introduced by this deliverable. The controlling references for this setup are the OpenPipeStress governance, decomposition, register, and PKG-00 architecture-basis artifacts listed in `_REFERENCES.md` and `Datasheet.md`.
>
> Any future code-specific acceptance criterion, stress limit, load-combination default, allowable, SIF/flexibility value, or proprietary component value must be supplied by a user-owned/private source with provenance and redistribution status. This GUI slice may display resulting diagnostics/statuses; it must not define or certify code compliance.
>

### CLM-013 — Verification

> ##### Verification
>
> | Verification ID | Method | Expected evidence |
> |---|---|---|
> | VER-07-07-001 | Document review | Four-document kit exists and matches DEL-07-07 scope. |
> | VER-07-07-002 | Boundary review | No GUI source, tests, schemas, job/solver code, package manifests, repo-level docs, or `ISSUED` artifacts were edited. |
> | VER-07-07-003 | Contract review | Requirements explicitly route progress, cancellation, diagnostics, and result status through PKG-00 command/job and result-envelope contracts. |
> | VER-07-07-004 | Protected-content review | Setup artifacts contain no protected standards text, proprietary engineering values, certification claims, or automatic code-compliance claims. |
> | VER-07-07-005 | Semantic setup review | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and provide complete lens coverage. |
> | VER-07-07-006 | Dependency-register validation | `Dependencies.csv` validates against the v3.1 schema and contains evidence-linked ACTIVE rows. |
> | VER-07-07-007 | Future implementation test | UI test coverage for job launch state, progress, cancellation, diagnostics, blocked states, and report/export traceability remains `TBD` until implementation work is authorized. |
>

### CLM-014 — Documentation

> ##### Documentation
>
> Required setup records for this deliverable are:
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

### CLM-015 — Acceptance Notes

> ##### Acceptance Notes
>
> `SEMANTIC_READY` means the setup artifacts are prepared for review. It does not mean product implementation is complete, UI behavior is tested, solver behavior is verified, or protected data is authorized. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

### CLM-016 — D-41 R5 T5 PDU-008 current GUI boundary

> ##### D-41 R5 T5 PDU-008 current GUI boundary
>
> The desktop diagnostic type/detail/filter path preserves class, remediation, and provenance when supplied and labels their absence as producer-unsupplied. Producer-side reduced shapes remain open; the GUI does not fabricate missing metadata.

- **AC-001** — The contract preserves service-owned progress and cancellation semantics, complete diagnostic fields and warning classes, separate solve and rule-check readiness, reproducibility traceability, and no invented progress, silent defaults, compliance status, or professional approval.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

> #### Procedure: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-07-DECL-004`.
>

### CLM-019 — Purpose

> ##### Purpose
>
> This procedure records how to produce and later use the setup basis for the solve-execution UX deliverable. It is not a product implementation procedure.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status |
> |---|---|
> | Sealed deliverable context for DEL-07-07 | Present in `_CONTEXT.md` |
> | Governing invariants and stop rules | Present in `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `INIT.md` |
> | Command/job/cancellation/progress architecture basis | Present in DEL-00-03 specification and architecture-basis injection |
> | GUI transient job-progress state basis | Present in DEL-00-05 specification and architecture-basis injection |
> | Diagnostics/result-envelope basis | Present in DEL-00-06 specification and architecture-basis injection |
> | Protected-data and professional-boundary constraints | Present in `docs/CONTRACT.md`, `docs/TYPES.md`, and `docs/SPEC.md` |
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Confirm the deliverable scope.
>    - Verify `Deliverable ID = DEL-07-07`, `Package ID = PKG-07`, and `Scope Coverage = SOW-055` in `_CONTEXT.md`.
>    - Verify objectives `OBJ-006` and `OBJ-007`.
>
> 2. Draft the four-document setup kit.
>    - Populate `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
>    - Keep content at setup/specification level.
>    - Do not create GUI source code, tests, schemas, job/solver implementation, manifests, repo-level docs, or issued artifacts.
>
> 3. Apply architecture-basis constraints.
>    - Route solve launch through command/job boundaries.
>    - Treat progress and cancellation as job-contract behavior.
>    - Preserve diagnostic/result-envelope fields and warning classes.
>    - Separate mechanics solved, user-rule checked, and human-review-required states.
>
> 4. Preserve data and authority boundaries.
>    - Do not introduce protected standards text, code tables, proprietary values, or private project data.
>    - Mark unknown implementation details as `TBD`.
>    - Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
>
> 5. Build semantic setup artifacts.
>    - Generate `_SEMANTIC.md` as a lens, not an authority.
>    - Generate `_SEMANTIC_LENSING.md` with complete matrix coverage.
>    - Use lensing output only as a candidate worklist for consistency review.
>
> 6. Refresh dependency artifacts.
>    - Create or update `Dependencies.csv` using v3.1 columns.
>    - Keep anchors and execution/interface edges evidence-linked.
>    - Update `_DEPENDENCIES.md` with counts, run notes, lifecycle summary, and handoff notes.
>
> 7. Validate local setup gates.
>    - Run dependency schema validation.
>    - Confirm required setup files exist.
>    - Confirm no writes occurred outside the deliverable folder.
>    - Confirm `_STATUS.md` remains at `SEMANTIC_READY` only if all setup gates pass.
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Four documents exist | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist |
> | Semantic artifacts exist | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist |
> | Dependency artifacts exist | `Dependencies.csv` and `_DEPENDENCIES.md` exist |
> | Dependency schema validates | `validate_dependencies_schema.py` reports valid v3.1 schema |
> | Protected data absent | No protected standards/code text, copied tables, proprietary values, or certification claims appear |
> | Scope respected | Only DEL-07-07 deliverable-local files are written |
>

### CLM-023 — Records

> ##### Records
>
> The setup run records are stored in `_run_records/`:
>
> - `TASK_RUN_2026-04-30_1049_four-documents_P1_P2.md`
> - `TASK_RUN_2026-04-30_1050_semantic-matrix-build.md`
> - `TASK_RUN_2026-04-30_1051_lens-register.md`
> - `TASK_RUN_2026-04-30_1052_four-documents_P3_ONLY.md`
> - `TASK_RUN_2026-04-30_1053_dependency-extract.md`
>

### CLM-024 — Completion Criteria

> ##### Completion Criteria
>
> The deliverable may be left at `SEMANTIC_READY` when:
>
> - all setup artifacts listed in `Specification.md` exist,
> - dependency validation passes,
> - semantic and lensing artifacts are present,
> - no protected-data or certification boundary issue is found,
> - no out-of-scope files were edited,
> - open implementation choices remain explicitly marked as `TBD` rather than silently chosen.

- **VER-001** — Validate the contract and review source parity, job-boundary routing, progress and cancellation semantics, diagnostic fields and warning classes, missing-data separation, reproducibility metadata, protected-content limits, and every retained TBD or governed residual.

## Governing Values and Decisions — Axiology

### CLM-025 — Guidance: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

> #### Guidance: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-07-07-DECL-003`.
>

### CLM-027 — Purpose

> ##### Purpose
>
> This deliverable keeps solve execution reviewable from the GUI without moving solver authority into the GUI. The user should be able to see that a solve is running, what the service reports about its progress, whether cancellation was requested or completed, and which diagnostics qualify the result.
>
> The setup is intentionally contract-facing. It prepares a future GUI slice to consume application-service jobs and diagnostics/result envelopes while preserving missing-data, provenance, unit, privacy, and professional-responsibility boundaries.
>

### CLM-028 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Service boundary first | GUI components initiate commands/jobs and observe job/envelope state. They do not own solver lifecycle semantics. |
> | Report what is supplied | Progress should reflect service-provided phases, counts, statuses, or messages. If only indeterminate progress is available, the UI should show indeterminate progress rather than fabricate precision. |
> | Cancellation is a request | Cancellation UX should distinguish requested, accepted, completed, failed, and not-supported states only when the service contract supplies those states. Exact enum names remain `TBD`. |
> | Diagnostics stay attached | Warnings and errors should remain linked to the run, affected object, source, severity, and remediation fields where available. |
> | Missing data is visible | Solve-blocking, rule-check-blocking, provenance, assumption, nonlinear, and IP-boundary warnings should remain visible instead of becoming generic failure messages. |
> | Professional boundary remains visible | A completed mechanics solve is decision-support information for review by the responsible engineer. Human review remains required; acceptance stays with the responsible engineer. |
>

### CLM-029 — Considerations

> ##### Considerations
>
> Background solves often have phases that are meaningful to users even when exact completion percentages are not reliable. A future UI can show service-reported states such as queued, validating, solving, recovering results, checking user rules, writing report/export artifacts, canceled, failed, or complete only if those states are supplied by the job contract. These labels are examples of UX categories, not fixed contract enums.
>
> Cancellation should preserve auditability. A canceled run may still produce diagnostics, partial records, or a terminal envelope explaining what was stopped and what was not computed. The GUI should present that terminal envelope rather than removing the run from view.
>
> Diagnostic presentation should support both quick triage and detailed review. A user needs to know which diagnostics block solving, which block only rule checks, which are provenance or assumption warnings, and which reflect nonlinear/numerical uncertainty. The future UI should keep diagnostic class and severity visible without converting warnings into professional judgments.
>
> Result/export readiness should be traceable. Where upstream contracts supply hashes, solver versions, model versions, rule-pack checksums, warning sets, and limitations, the solve UX should preserve those signals for reports and result exports. Where those fields are missing, the setup should mark them `TBD` rather than inventing them.
>

### CLM-030 — Trade-offs

> ##### Trade-offs
>
> | Decision area | Recommended posture | Risk if ignored |
> |---|---|---|
> | Percent progress | Prefer contract-supplied percentages; otherwise use indeterminate or phase-based progress | False precision can mislead users and reviewers |
> | Cancellation feedback | Show request and final job state separately | Users may believe a solve stopped cleanly when the service did not confirm it |
> | Diagnostic density | Provide summary and detail surfaces | Hiding details weakens reviewability; flooding the main surface weakens usability |
> | Result status wording | Use mechanics/rule/human-review status vocabulary | Users may misread software output as a determination reserved for the responsible engineer (PRD §21.2) |
> | Error recovery | Preserve terminal envelopes and run records | Failed/canceled solves become unreviewable |
>

### CLM-031 — Examples

> ##### Examples
>
> The following are non-normative UX examples only:
>
> - If the job contract supplies no percentage, show an indeterminate progress indicator plus the current service-reported phase.
> - If cancellation is requested, disable duplicate cancel requests while continuing to display diagnostics and the final terminal envelope when received.
> - If a solve completes but rule-pack inputs are missing, present the mechanics result separately from the rule-check-blocking diagnostics.
> - If a nonlinear warning is returned, keep it visible with the affected object and remediation text supplied by the diagnostic record.
>

### CLM-032 — Open Issues

> ##### Open Issues
>
> | ID | Issue | Owner |
> |---|---|---|
> | OI-07-07-001 | Exact job state enum, progress payload, and cancellation terminal states remain implementation-level `TBD` until the application-service contract is materialized. | Future implementation TASK |
> | OI-07-07-002 | Exact diagnostic filtering, grouping, and detail layout remain `TBD` until GUI design work is authorized. | Future GUI TASK |
> | OI-07-07-003 | Exact report/export handoff fields remain `TBD` until result export and report artifacts are implemented. | PKG-08 / future interface TASK |
>

### CLM-033 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflicts were identified during setup. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-055 OBJ-006 OBJ-007 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
