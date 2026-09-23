---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-02
package_id: PKG-12
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@00115c71931bcae79909602d653740d3bb72dfa1
project_scope_refs: [SOW-040]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-02

Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `00115c71931bcae79909602d653740d3bb72dfa1` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open obligations remain in this Scope of Work and owning decisions, with execution in the selected graph; lifecycle remains in `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Purpose and Objective Traceability

This Scope of Work defines `DEL-12-02` in service of project scope [SOW-040] and package objectives [OBJ-010].

- **OUT-001** — A private-data redaction and export-control contract for reports, shared models, downstream tools, public examples, and local-private outputs is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-12-02 Private data redaction and export controls

> #### Datasheet: DEL-12-02 Private data redaction and export controls
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-12-02 |
> | Deliverable Name | Private data redaction and export controls |
> | Package ID | PKG-12 |
> | Package Name | Security, Privacy, and Private Data Handling |
> | Deliverable Type | SECURITY_CONTROL |
> | Scope Item | SOW-040 |
> | Objective | OBJ-010 |
> | Setup Run Date | 2026-04-30 |
> | Lifecycle State | `_STATUS.md` currently records `IN_PROGRESS`; this evidence alignment does not authorize lifecycle promotion |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Value |
> |---|---|
> | Control surface | Report/export privacy guardrails for private rule, material, component, project, and code/design-basis values |
> | Primary output class | Redaction configuration contract and export-test expectations |
> | Product posture | Local-first; no cloud export or transmission unless separately approved |
> | Default sharing posture | Shared/public exports require explicit private-data handling and must not silently include protected/private values |
> | Private data classes | Project model values; private rule-pack values; material and allowable-like values; component/manufacturer values; owner/company design-basis fields; private report-template content |
> | Protected data classes | Standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, and proprietary commercial data |
> | Diagnostic class | `IP_BOUNDARY_WARNING` where export/report content may expose private or protected data |
> | Redaction action vocabulary | `include`, `warning_only`, `redact_value`, `redact_field`, `omit_field`, `block_export` |
> | Export context vocabulary | `public_report`, `public_example`, `shared_model`, `downstream_tool`, `local_private` |
> | Config persistence | `schemas/redaction_export_controls.schema.yaml` now records the local-first control profile, field policies, decisions, and findings contract; durable project/profile storage location and storage roots remain `TBD` |
> | Export-test status | Current guard and fixed-route tests are defined in `tests/security/test_redaction_export_controls.py`, `redactionExportControls.test.ts` and `ControlledExportLink.test.tsx`. The R15 accepted breadth scope and native witness limit remain specific; this record runs no suite or new acceptance. |
> | Implementation status | Metadata helper/schema plus actual desktop/Python/Rust final-sink bindings exist. Persist profile lifecycle, native witnesses, future public/plugin egress, quarantine, cloud and legal/security approval as separately scoped obligations. |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Constraint |
> |---|---|
> | Configuration required | Any export path that can expose private values must have an explicit redaction/export policy, even if the policy is to allow a local private export. |
> | Default redaction boundary | Public templates and public examples must not include protected standards content, private user data, copied proprietary formulas, or vendor data without documented rights. |
> | Local private export | A user may need an unredacted local export for project review, but the workflow must make the privacy status and responsibility visible. |
> | Shared/public export | Shared or public-facing exports must redact or block private values according to configuration and emit an auditable warning/manifest. |
> | Report manifest | Export records should preserve model/report hashes, solver/report versions, rule-pack identity/version/checksum, provenance summaries, warnings, and limitations without revealing redacted values. |
> | Adapter boundary | Import/export adapters and plugins cannot bypass units, provenance, diagnostics, sandboxing, report controls, or public/private data checks. |
> | Professional boundary | Redaction status, report export, or rule-pack checksum must not be framed as certification, sealing, approval, authentication, or code compliance (PRD §21.2). |
>

### CLM-005 — Redaction Configuration Contract

> ##### Redaction Configuration Contract
>
> This deliverable now has current evidence for a schema-governed redaction/export-control contract in `schemas/redaction_export_controls.schema.yaml` and a metadata-only helper under `core/security/redaction/`. Current final-sink bindings are recorded in `docs/security/redaction_export_controls.md` and desktop/Python/Rust consumers. Persisted profile lifecycle, public transport, future adapter/plugin egress, legal approval and native witness limitations remain separate obligations; existing UI/local CLI/export routes are not blanket TBDs.
>
> | Proposed Config Slot | Purpose | Default Setup Expectation |
> |---|---|---|
> | `profile_id` | Stable identifier for a redaction/export profile. | Schema slot exists; persisted profile lifecycle remains `TBD`. |
> | `export_context` | Classifies the intended export surface. | Configured context vocabulary exists for public report/example, shared model, downstream tool, and local private export. |
> | `field_classification_rules` | Maps model/report fields to public, private, protected-suspected, or unknown handling classes. | Helper uses explicit metadata only; unknown risky fields produce warning, redaction, or blocking behavior. |
> | `rule_pack_detail_policy` | Controls formula, allowable, interpretation, source-note, and checksum exposure. | Public/shared exports may include safe metadata such as identity/version/checksum/source note, not protected formula text. |
> | `material_value_policy` | Controls material properties, allowables, provenance, and redistribution-status exposure. | Apply classification at the controlled final sink using the current desktop/Python/Rust route bindings; preserve material-value provenance and source immutability. Candidate evidence remains route-specific. |
> | `component_value_policy` | Controls manufacturer/vendor component fields, geometry catalogs, stiffnesses, and private modifiers. | Apply current guarded final-sink classification to component values and private modifiers; source data must remain unchanged and blocking findings prevent exposure. |
> | `project_value_policy` | Controls project-specific loads, owner requirements, coordinates, equipment loads, paths, and design-basis values. | Shared/public sinks redact private metadata and concrete paths through current route controls. Local-private retention requires wrapper-owned explicit intent; local placement follows the governed storage boundary. |
> | `manifest_policy` | Defines what evidence remains in an export manifest after redaction. | Preserve safe identity, unit, hash/checksum and provenance metadata at final sinks. Withhold lossless exports when redaction would corrupt required semantics; maintain invented-data producer/consumer parity. |
> | `override_policy` | Defines whether unredacted private export is allowed. | Local-private retention requires explicit local/private intent and emits warnings; approval and user-confirmation workflow remain `TBD`. |
> | `diagnostic_policy` | Defines warning classes and blocking findings. | `IP_BOUNDARY_WARNING`, `PRIVATE_DATA_WARNING`, `PROVENANCE_WARNING`, `STORAGE_BOUNDARY_WARNING`, and professional-boundary findings are represented. |
> | `template_guard_policy` | Controls public report templates and examples. | Use typed protected-content checks and controlled report/template exposure. Scoped release scanning and legal review remain separate; neither guard presence nor a scan supplies legal approval. |
>

### CLM-006 — Export Test Expectations

> ##### Export Test Expectations
>
> Current deliverable obligation: implement export/report safeguards that warn or redact private rule/material/component values where configured. Implementation and record changes require an active bounded brief under `AGENTS.md`; `_STATUS.md` records lifecycle; this Scope of Work, owning decisions and the selected graph carry current work. References below to the original setup write boundary apply only to that historical run. They do not exclude later authorized delivery, waive the retained requirements, or authorize issuance.
>
>
> | Test Expectation | Risk Covered | Setup Status |
> |---|---|---|
> | Redacted public report excludes private rule/material/component values. | Private data leakage. | Focused invented-fixture test exists; runtime report/export integration remains `TBD`. |
> | Public template linter rejects protected code text, copied tables, protected formulas, and proprietary examples. | Protected content leakage. | Guardrail scans exist for changed surfaces; protected-content linter and legal-review workflow remain `TBD`. |
> | Local private export can retain private values only with explicit user intent and warning/audit record. | Accidental disclosure. | Focused invented-fixture test exists; final UI/CLI approval flow remains `TBD`. |
> | Redacted export preserves non-sensitive manifest evidence such as hashes, versions, warnings, and provenance summaries. | Reproducibility loss after redaction. | Schema/docs/helper preserve safe metadata classes; full report/export manifest integration remains `TBD`. |
> | Adapter/plugin export path cannot bypass redaction, units, provenance, sandboxing, or diagnostics. | No-bypass failure. | Metadata hardening blocks storage/privacy bypass markers; adapter/plugin runtime routing remains `TBD`. |
> | Redaction never mutates the source project model or private libraries. | Destructive export behavior. | Focused invented-fixture source non-mutation test exists; runtime source-model integration remains `TBD`. |
>
### CLM-007 — Construction

> ##### Construction
>
> The original setup artifact constructed a documentation-level redaction/export control contract. Later bounded implementation evidence added a metadata-only redaction helper, schema, focused invented-fixture tests, and security documentation. This alignment records that evidence without changing product code or lifecycle state.
>
> Current evidence includes:
>
> - redaction/export-control requirements in `ScopeOfWork.md`;
> - configuration-slot and export-test expectations in this datasheet;
> - guidance on export contexts, warning behavior, and deferred implementation decisions in `ScopeOfWork.md`;
> - setup and future integration procedure in `ScopeOfWork.md`;
> - schema contract in `schemas/redaction_export_controls.schema.yaml`;
> - metadata-only helper in `core/security/redaction/`;
> - focused tests in `tests/security/test_redaction_export_controls.py`;
> - implementation guidance in `docs/security/redaction_export_controls.md`;
> - semantic matrix/lensing and dependency setup artifacts.
>
> This run does not create product source code, tests, schemas, real project data, private rule packs, protected standards content, secrets, credentials, cloud operations, destructive quarantine movement, legal sufficiency, security certification, lifecycle acceptance, or professional/code-compliance claims (PRD §21.2).
>
### CLM-008 — References

> ##### References
>
> | Source | Use |
> |---|---|
> | `AGENTS.md` | Bootstrap boundaries: open mechanics, protected data, private data, and no certification claims. |
> | `AGENTS.md` | Type 2 scoped execution and write-scope constraints. |
> | `docs/DIRECTIVE.md` | Product principles for private code data, provenance, unit safety, report auditability, stop rules, and hidden cloud/telemetry exclusion. |
> | `docs/CONTRACT.md` | OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-REPORT, OPS-K-PRIV, OPS-K-RULE, and OPS-K-AGENT invariants. |
> | `docs/TYPES.md` | SECURITY_CONTROL type, private/user-supplied data vocabulary, report object, rule-pack reference, and analysis-status boundaries. |
> | `docs/SPEC.md` | Layered architecture, adapter no-bypass rule, report/audit content, warning classes, and validation expectations. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy, quarantine rule, and report boundary. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, DEL-12-02, SOW-040, OBJ-010, and AB-00 architecture basis. |
> | `docs/_Registers/Deliverables.csv` | Deliverable identity, anticipated artifacts, context/risk notes. |
> | `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-040. |

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-12-02 Private data redaction and export controls

> #### Specification: DEL-12-02 Private data redaction and export controls
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — Scope

> ##### Scope
>
> Maintain private/protected-data controls for reports, shared models, downstream outputs and public templates/examples. The metadata helper itself has no I/O; actual route bindings control payload exposure before report DOM, download, native IPC/save and local CLI sinks. Current sources and the final-sink contract are identified in `docs/security/redaction_export_controls.md`.
>
> Require wrapper-owned route context and explicit local-private intent, source non-mutation, blocking before exposure and safe metadata/parity. The R15 handoff closes only its accepted REXC-REQ-012 breadth scope and retains native save/print witness limits. Preserve missing native witnesses, persisted-profile lifecycle, future public/plugin transports, destructive quarantine, cloud exceptions and legal/security review. No real protected-data handling, professional acceptance or lifecycle issuance is inferred by this record repair.

### CLM-012 — Requirements

> ##### Requirements
>
> | Requirement ID | Requirement | Source Basis | Verification |
> |---|---|---|---|
> | REXC-REQ-001 | Export and report workflows shall classify the export context before exposing project, rule-pack, material, component, owner, or code/design-basis data. | SOW-040; OPS-K-PRIV-1; `docs/SPEC.md` §§1,8 | Confirm the control contract includes explicit export context classes. |
> | REXC-REQ-002 | Shared/public exports shall not silently include private project data, private rule-pack values, private material/component values, protected standards content, copied formulas, proprietary templates, or vendor data without documented rights. | OPS-K-IP-1; OPS-K-IP-3; OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` §§3,7 | Focused redaction tests cover private/unknown metadata handling; protected-content linter and runtime export-route tests remain `TBD`. |
> | REXC-REQ-003 | Public report templates and public examples shall be protected-data-free and shall not embed protected formulas, standards tables, copied code text, proprietary report templates, or private project examples. | OPS-K-REPORT-2; `docs/SPEC.md` §8; `docs/IP_AND_DATA_BOUNDARY.md` §7 | Guardrail scans exist for changed surfaces; report-template linter/runtime integration remains `TBD`. |
> | REXC-REQ-004 | Redaction configuration shall support at least warning, value redaction, field redaction, export blocking, and explicit local-private export allowance. | SOW-040; `docs/_Registers/Deliverables.csv` row DEL-12-02 | Schema vocabulary and focused tests now check the supported policy vocabulary. |
> | REXC-REQ-005 | Unknown or insufficiently proven redistribution status shall result in warning, redaction, or block behavior rather than silent public inclusion. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-AGENT-1 | Focused redaction tests check unknown-source handling. |
> | REXC-REQ-006 | Local private exports may retain private values only when the export context is local/private and user intent is explicit; the export record shall preserve a warning or audit note. | OPS-K-PRIV-1; `docs/DIRECTIVE.md` §§4.2,6 | Focused tests check explicit local-private intent; final UI/CLI approval flow remains `TBD`. |
> | REXC-REQ-007 | Redaction shall not mutate the source project model, private libraries, or rule packs; it shall operate on an export/report representation. | AB-00-04; OPS-K-DATA-3 | Focused tests confirm the helper copies export/report representations; runtime source-model integration remains `TBD`. |
> | REXC-REQ-008 | Redacted exports shall preserve non-sensitive reproducibility evidence such as model/report hashes, solver/report versions, input-manifest identifiers, warning summaries, rule-pack identity/version/checksum, and provenance summaries where safe. | OPS-K-REPORT-1; SOW-039; AB-00-04; `docs/SPEC.md` §8 | Schema/docs/helper support safe metadata preservation; full report/export manifest integration remains `TBD`. |
> | REXC-REQ-009 | Rule-pack details in public/shared reports shall be limited to safe metadata such as ID, version, checksum, and source note unless the user has documented rights to include formula/detail content. | OPS-K-RULE-3; OPS-K-REPORT-2; `docs/IP_AND_DATA_BOUNDARY.md` §7 | Focused tests preserve safe checksum metadata while redacting private rule-pack values; runtime rule-pack/report integration remains `TBD`. |
> | REXC-REQ-010 | Materials, components, sections, SIF/flexibility-like values, allowables, manufacturer/vendor values, and code/design-basis fields shall carry provenance and privacy/redistribution status into redaction decisions. | OPS-K-DATA-3; OPS-K-IP-2; `docs/TYPES.md` §§7-8 | Future schema/adapter tests check provenance and status inputs. |
> | REXC-REQ-011 | Diagnostics and result/report envelopes shall surface redaction, protected-content, and private-data export findings using machine-readable diagnostics, including `IP_BOUNDARY_WARNING` where applicable. | AB-00-06; `docs/SPEC.md` §7 | Future diagnostics tests check code/class/severity/source/affected object/message/remediation/provenance fields. |
> | REXC-REQ-012 | Adapters, plugins, CLI exports, GUI report preview/export, and downstream-tool handoffs shall not bypass redaction, provenance, unit, sandboxing, diagnostics, or report controls. | AB-00-02; AB-00-07; OPS-K-PRIV-1 | Future adapter/plugin tests exercise no-bypass routes. |
> | REXC-REQ-013 | Export controls shall preserve the distinction among mechanics solved, user-rule checked, and human-approved states, and shall not claim certification, sealing, approval, authentication, or code compliance (PRD §21.2). | OPS-K-AUTH-1; OPS-K-AUTH-2; AB-00-03; `docs/TYPES.md` §4 | Report review checks professional-boundary notices and status vocabulary. |
> | REXC-REQ-014 | Redaction/export tests shall cover public report export, local private export, shared model export, downstream-tool export, adapter/plugin routes, manifest preservation, unknown provenance, and source-model non-mutation. | AB-00-08; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` §5 | Use current fixed-route desktop/Python/Rust guard tests and R15 scope-bound evidence. Retain native save/print limits, future public/plugin routes and scoped legal review; this record establishes no fresh execution or acceptance. |
> | REXC-REQ-015 | Any unresolved config schema, UI control, export format, public API transport, or physical project package/container choice shall remain `TBD` until resolved through an authorized implementation or architecture decision. | AB-00-04; AB-00-07; OPS-K-AGENT-1 | Review this deliverable for explicit TBD/open issue entries. |
>

### CLM-013 — Standards

> ##### Standards
>
> No external engineering code, standards clause, protected table, protected formula, material allowable, SIF/flexibility table, protected dimensional table, proprietary report template, vendor catalog, real private project, or real secret is used or reproduced by this deliverable.
>
> The controlling project sources for this deliverable are the SWBPIPE governance and decomposition artifacts listed in `ScopeOfWork.md` and `_REFERENCES.md`.
>
### CLM-014 — Verification

> ##### Verification
>
> | Verification ID | Check | Expected Result |
> |---|---|---|
> | REXC-VER-001 | Confirm `ScopeOfWork.md` exists and preserves required sections. | `ScopeOfWork.md` is present. |
> | REXC-VER-002 | Validate `Dependencies.csv` with `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Schema valid with all 29 v3.1 columns. |
> | REXC-VER-003 | Confirm `_SEMANTIC.md` has no `MatrixError` or `MATRIX_ERROR` and no algebra/operator leaks in final result tables. | Semantic setup gate passes. |
> | REXC-VER-004 | Confirm `_SEMANTIC_LENSING.md` has complete coverage for matrices A, B, C, F, D, X, and E. | 96 required lens coverage rows are present. |
> | REXC-VER-005 | Search deliverable artifacts for protected standards content, real private project values, real secrets, cloud-operation assumptions, and certification/compliance/approval/seal claims (PRD §21.2). | No disallowed content found. |
> | REXC-VER-006 | Read `_STATUS.md` without editing it. | Current lifecycle state is `IN_PROGRESS`; this evidence alignment does not promote lifecycle state or imply acceptance. |
> | REXC-VER-007 | Confirm June 7 evidence surfaces exist for `schemas/redaction_export_controls.schema.yaml`, `core/security/redaction/`, `tests/security/test_redaction_export_controls.py`, and `docs/security/redaction_export_controls.md`. | Evidence exists; lifecycle state remains separately governed. |
> | REXC-VER-008 | Confirm June 7 run evidence records focused redaction/storage privacy hardening validation. | `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` and package fan-in record passing focused tests. |
>

### CLM-015 — Documentation

> ##### Documentation
>
> Required setup artifacts for this run:
>
> - `ScopeOfWork.md`;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - `_DEPENDENCIES.md`;
> - `_run_records/*`;
> - `_STATUS.md`.
>
> Current implementation evidence:
>
> - `schemas/redaction_export_controls.schema.yaml`;
> - `core/security/redaction/`;
> - `tests/security/test_redaction_export_controls.py`;
> - `docs/security/redaction_export_controls.md`;
> - `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`;
> - package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md`.
>
> Original June 7 evidence above remains historical. Current final-sink bindings are described in `docs/security/redaction_export_controls.md`; the metadata helper itself remains free of I/O. Remaining artifacts and choices:
>
> - native save/print witness limitations and candidate-bound integration evidence;
> - persisted redaction-profile lifecycle;
> - future public API/plugin/adapter egress not represented by current guarded local routes;
> - scoped protected-content release review and legal review workflow;
> - destructive quarantine movement workflow;
> - cloud exception workflow;
> - persisted profile location and unaccepted storage/security guarantees;
> - future public transport and external export-format admission;
> - approval choices;
> - real project data, real private values, protected standards content, secrets, credentials, or cloud behavior.
- **AC-001** — The contract preserves export-context and redaction-action vocabularies, safe manifest evidence, explicit local-private intent, non-mutation, protected/private/provenance handling, metadata-only schema/helper/test evidence, no-bypass adapter boundaries, and guarded final-sink exposure and invented-data parity, with explicit native witness and legal/security approval limits; export safeguards remain separate from legal and security review. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Production and Verification Method — Praxeology

### CLM-016 — Procedure: DEL-12-02 Private data redaction and export controls

> #### Procedure: DEL-12-02 Private data redaction and export controls
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — Purpose

> ##### Purpose
>
> This procedure describes how to maintain the DEL-12-02 redaction/export-control artifact set after setup and how implementation work should wire the current metadata-only helper, schema contract, focused tests, and security documentation into product report/export behavior without crossing private-data, protected-data, or professional-authority boundaries.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Required State |
> |---|---|
> | Sealed deliverable context | DEL-12-02, PKG-12, SOW-040, OBJ-010, explicit write scope |
> | Governance sources | `AGENTS.md`, `AGENTS.md`, `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, and decomposition/register rows read |
> | Architecture basis | AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, and AB-00-08 injected as constraints, not copied wholesale |
> | Scope boundary | No edits outside this deliverable folder |
> | Protected/private data boundary | No real private values, credentials, protected standards content, proprietary report templates, or legal/compliance claims introduced |
> | Current evidence basis | June 7 evidence exists for `schemas/redaction_export_controls.schema.yaml`, `core/security/redaction/`, `tests/security/test_redaction_export_controls.py`, `docs/security/redaction_export_controls.md`, and `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` |
>

### CLM-020 — Steps

> ##### Steps
>
> | Step | Action | Output |
> |---|---|---|
> | 1 | Confirm DEL-12-02 identity, scope, objective, invariants, acceptance criteria, and write scope. | `_CONTEXT.md` remains the scope anchor. |
> | 2 | Classify export risk surfaces: report preview/export, shared model export, public templates/examples, CLI/API export, adapter/plugin export, and downstream-tool handoff. | `ScopeOfWork.md` attributes and conditions. |
> | 3 | Classify sensitive value groups: project data, rule-pack details, material/component/library values, owner/company design basis, protected standards content, proprietary/vendor data, and private report-template content. | `ScopeOfWork.md` and `ScopeOfWork.md` private/protected value sections. |
> | 4 | Define setup-level redaction mode and export context vocabulary against `schemas/redaction_export_controls.schema.yaml`. | `ScopeOfWork.md` redaction configuration contract. |
> | 5 | Translate redaction/export safeguards into requirements and verification expectations. | `ScopeOfWork.md` REXC requirements and verification table. |
> | 6 | Record implementation guidance, trade-offs, open issues, and human-ruling conflicts. | `ScopeOfWork.md` principles, trade-offs, open issues, and conflict table. |
> | 7 | Build semantic matrix and lensing artifacts against the current Scope of Work. | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`. |
> | 8 | Apply P3 lensing by surfacing warranted TBDs, verification gaps, or conflicts only when source evidence supports the edit. | Open issues and verification gaps remain visible. |
> | 9 | Extract dependency register rows for anchors and explicit execution information flow. | `Dependencies.csv` and `_DEPENDENCIES.md`. |
> | 10 | During the historical setup workflow, run validation checks before any authorized lifecycle update. | Final setup run records. |
> | 11 | During readiness-evidence alignment, replace stale setup-only language where current run evidence proves schema/helper/docs/tests exist. | `ScopeOfWork.md` reflects current evidence without lifecycle promotion. |
> | 12 | Preserve native witness limits, profile lifecycle, destructive quarantine, legal review, cloud exceptions and future public/plugin transport/export admission as explicit remaining gates; current final-sink bindings follow the security contract. | No hidden dependency closure or overclaim. |
> | 13 | Refresh dependency and review evidence only where cited upstream statuses, reviews, or run records warrant the change. | `Dependencies.csv`, `_DEPENDENCIES.md`, `_REVIEW.md`, and `Review_Findings.csv` remain evidence-first. |
>

### CLM-021 — Future Implementation Procedure

> ##### Future Implementation Procedure
>
> When a later implementation task is authorized, it should:
>
> 1. Evidence persisted redaction-profile integration with the project-local store under accepted DEC-017/019/028 and the native store implementation. Existing placement/migration policy is not an unresolved blanket prerequisite; unfulfilled profile persistence, rollback and native behavior require their own scoped implementation and witness.
> 2. Add GUI/CLI/API controls that require explicit export context selection before shareable export.
> 3. Route report generation, model export, adapter/plugin export, and downstream-tool handoff through the same redaction and diagnostic checks.
> 4. Emit `IP_BOUNDARY_WARNING` and related diagnostics with code, class, severity, source, affected object, message, remediation, and provenance.
> 5. Preserve unit awareness, provenance, privacy/redistribution status, rule-pack checksums, model/report hashes, and report limitations.
> 6. Ensure redaction affects only export/report representations and never mutates authoritative project models or private libraries.
> 7. Extend the focused helper/schema tests into runtime integration tests for public/shared redaction, local-private override, unknown provenance, protected-content linting, adapter/plugin no-bypass behavior, manifest preservation, and source non-mutation.
> 8. Define quarantine review routing without destructive movement unless a maintainer-approved quarantine workflow is authorized.
> 9. Preserve professional-boundary notices and avoid certification, sealing, approval, authentication, or code-compliance claims (PRD §21.2).
>

### CLM-022 — Verification

> ##### Verification
>
> | Check | Method | Expected Result |
> |---|---|---|
> | Scope of Work presence | Confirm `ScopeOfWork.md` exists. | Present. |
> | Default sections | Confirm Datasheet has Identification/Attributes/Conditions/Construction/References; Specification has Scope/Requirements/Standards/Verification/Documentation; Guidance has Purpose/Principles/Considerations/Trade-offs/Examples; Procedure has Purpose/Prerequisites/Steps/Verification/Records. | Present. |
> | Dependency schema | Run `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv`. | Valid v3.1 schema. |
> | Current implementation evidence | Confirm `schemas/redaction_export_controls.schema.yaml`, `core/security/redaction/`, `tests/security/test_redaction_export_controls.py`, and `docs/security/redaction_export_controls.md` exist. | Present as June 7 evidence. |
> | Focused redaction tests | Review `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md` and package fan-in validation. | Focused redaction/local-first tests passed in June 7 evidence. |
> | Semantic audit | Confirm `_SEMANTIC.md` contains no `MatrixError`/`MATRIX_ERROR` and no algebra/operator leaks in final result tables. | PASS. |
> | Lensing coverage | Count `_SEMANTIC_LENSING.md` lens rows for matrices A, B, C, F, D, X, and E. | 96 required rows. |
> | Boundary scan | Search deliverable files for real secrets, protected standards content, real private project values, cloud-operation assumptions, or certification/compliance claims. | No disallowed content found. |
> | Lifecycle status | Read `_STATUS.md` without editing it. | Current state remains `IN_PROGRESS`; this procedure does not authorize promotion. |
>

### CLM-023 — Records

> ##### Records
>
> The deliverable evidence surface includes these records in the deliverable folder:
>
> - four production documents;
> - `_SEMANTIC.md`;
> - `_SEMANTIC_LENSING.md`;
> - `Dependencies.csv`;
> - `_DEPENDENCIES.md`;
> - `_run_records/*`;
> - `_STATUS.md`.
>
> Do not move any artifact to `ISSUED` during a readiness-evidence alignment run.
>
> Readiness-evidence alignments must also leave product code, schemas, tests, DAG artifacts, coordination files, approval records, release files, package registers, and lifecycle files unchanged unless a separate authorized brief grants that write scope.
- **VER-001** — Validate the contract and review source parity, export-context and redaction-policy coverage, private/protected/provenance handling, safe manifest preservation and source non-mutation, metadata-only evidence limits, no-bypass routes, retained conflicts and TBDs, and authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-12-02 Private data redaction and export controls

> #### Guidance: DEL-12-02 Private data redaction and export controls
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-026 — Purpose

> ##### Purpose
>
> This deliverable keeps SWBPIPE report and export workflows aligned with the product boundary: public mechanics and schemas are allowed, while private project values, user-supplied code data, rule-pack details, component/vendor data, material allowables, and protected standards content remain controlled.
>
> The original guidance was setup-level. Current June 7 evidence now includes a concrete schema contract, metadata-only helper, focused invented-fixture tests, and security documentation. Current final-sink bindings define wrapper context/intent for desktop and local CLI exports. Preserve unresolved persisted profile lifecycle, future public/plugin egress, native witness limitations, cloud exceptions, legal review, destructive quarantine and approval workflow.
>

### CLM-027 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Explicit export context | Treat local private export, shared redacted export, public template/example export, and downstream-tool handoff as different privacy contexts. |
> | Warn or redact by configuration | The product should not silently include private or protected values in outputs that may be shared. |
> | Block when provenance is unsafe | Unknown, protected-suspected, or private-only redistribution status should trigger warning, redaction, or block behavior. |
> | Preserve reproducibility evidence | Redaction should keep non-sensitive hashes, versions, warnings, provenance summaries, and rule-pack identifiers where safe. |
> | Do not mutate source data | Redaction is an export/report transformation, not a change to the authoritative project model or private libraries. |
> | No-bypass exports | GUI, CLI, adapters, plugins, public APIs, and downstream handoffs must use the same unit, provenance, diagnostic, sandboxing, and data-boundary checks. |
> | Human authority | Redaction/export controls support review. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority. |
>

### CLM-028 — Considerations

> ##### Considerations
>

### CLM-029 — Private and Protected Value Classes

> ###### Private and Protected Value Classes
>
> Treat these as sensitive in export/report decisions unless documented redistribution rights and user intent say otherwise:
>
> - private project model coordinates, loads, equipment loads, owner requirements, design bases, and project identifiers;
> - private rule-pack formulas, allowables, interpretations, source excerpts, and code-specific checking logic;
> - material properties, allowable-like values, temperature-dependent data, and source/license metadata;
> - component and section library values, manufacturer/vendor data, SIF/flexibility-like values, stiffnesses, dimensions, and proprietary catalog-derived records;
> - private report templates or owner/company calculation report formats;
> - protected standards text, tables, copied formulas, figures, examples, or commentary.
>

### CLM-030 — Export Contexts

> ###### Export Contexts
>
> `local_private` can preserve private values for the user's own review only with
> explicit intent and warning/audit evidence. `public_report`, `public_example`,
> and `shared_model` assume disclosure risk and prefer redaction or blocking.
> `downstream_tool` may need values for technical continuity, but the adapter must
> still preserve privacy, provenance, unit, and diagnostic checks.
>

### CLM-031 — Report Boundary

> ###### Report Boundary
>
> Reports may safely reference rule-pack ID, version, checksum, and source note when those fields do not disclose protected/private content. Reports must not embed protected standards formulas, copied tables, proprietary report templates, or private values in public templates/examples. Users remain responsible for private report templates that quote licensed standards outside the public project.
>

### CLM-032 — Manifest Boundary

> ###### Manifest Boundary
>
> A redacted export should not become useless for audit. Runtime integration should preserve non-sensitive reproducibility evidence such as hashes, versions, redaction profile ID, warning summaries, and provenance summaries while avoiding leakage of the redacted values themselves.
>

### CLM-033 — Adapter and Plugin Boundary

> ###### Adapter and Plugin Boundary
>
> Adapters and plugins are export risk multipliers. They should never bypass redaction configuration, provenance checks, unit checks, report controls, diagnostics, or rule-pack sandboxing. A downstream-tool export is still an export boundary, not an exemption.
>

### CLM-034 — Current Evidence Boundary

> ###### Current Evidence Boundary
>
> The current helper is metadata-only. It blocks or redacts explicit storage/privacy markers for payload presence, secret material, cloud/network references, direct SQL/raw SQLite access, storage bypasses, concrete path indicators, and local/private intent. It does not read or write private paths, move quarantine material, store secrets, select storage roots, integrate report/export runtime routes, authorize cloud behavior, or decide legal/security/professional sufficiency.
>

### CLM-035 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Implication |
> |---|---|
> | Redact values vs. preserve review detail | Redaction may reduce review detail. Current final sinks preserve safe identity, units, hashes and provenance; exports requiring lossless semantic parity must withhold rather than emit a corrupted redacted representation. Native and legal acceptance remain separate. |
> | Warn-only vs. block export | Warn-only preserves user agency but can permit accidental disclosure. Current metadata hardening blocks high-risk storage/privacy markers while final approval and override workflow remains `TBD`. |
> | Field-level vs. value-level redaction | Field-level redaction is simpler and safer; value-level redaction can retain structure but may leak through labels, units, hashes, or context. |
> | Local private export vs. public template export | Local private exports may need full detail; public templates/examples must remain protected-data-free and invented-data-only. |
> | Rich downstream handoff vs. minimum disclosure | Downstream tools may require detail, but adapters must not become a bypass path for private data or protected content. |
>

### CLM-036 — Examples

> ##### Examples
>
> The following are symbolic examples only:
>
> | Scenario | Acceptable Planning Expression | Avoid |
> |---|---|---|
> | Public report example | Invented model values plus export context `public_example` | Real user project values or protected standards examples |
> | Rule-pack summary | Rule-pack ID/version/checksum/source note with private formula details redacted | Copying protected formula text or code-derived tables into a public report |
> | Material library export | Provenance summary and redistribution status; private values redacted for shared output | Public material allowable table without documented rights |
> | Local private report | Unredacted values only after explicit local-private export selection and warning/audit record | Silent unredacted export to a shareable/public location |
> | Downstream-tool handoff | Adapter route that preserves units, provenance, diagnostics, and privacy controls | Plugin export that bypasses data-boundary checks |
>

### CLM-037 — Open Issues and TBDs

> ##### Open Issues and TBDs
>
> | Issue ID | Topic | Status | Notes |
> |---|---|---|---|
> | REXC-OI-001 | Redaction config schema | PARTIAL / TBD | `schemas/redaction_export_controls.schema.yaml` now exists; persisted profile storage location, migration behavior, and storage roots remain `TBD`. |
> | REXC-OI-002 | Export context UI and override flow | TBD | Current desktop/local CLI wrappers own fixed route context and explicit local-private intent. Future public/plugin interaction, profile persistence and legal approval remain separately governed. |
> | REXC-OI-003 | Public API transport and export formats | TBD | SCA-004 and the accepted decomposition OI-004/PKG-17 allocation govern named export scope. Per-format interface/provenance guarantees and integration remain separately governed; this does not select a new public API transport or plugin runtime. |
> | REXC-OI-004 | Executable export tests | Scoped evidence remains | Current Python/desktop fixed-route tests cover route context, explicit intent, source immutability, blocking and stale completion. Retain R15 native save/print witness limits and future public/plugin route evidence; definitions are not a fresh result. |
> | REXC-OI-005 | Legal sufficiency of redaction | TBD | This deliverable does not claim that any redaction policy satisfies legal, client, security, or professional obligations. |
> | REXC-OI-006 | Physical project package/container | Choice ruled; implementation/conformance gaps retained | DEC-017 retains canonical JSON domain truth and a rebuildable SQLite projection; DEC-028 selects a multi-member zip/directory package with member inventory and per-member JCS hashes. DEC-057 names the `.opsproj` package. Storage roots, integration and unmet controls remain separately governed. |
> | REXC-OI-007 | Runtime report/export integration | Current bindings; scoped gates remain | Current final-sink bindings span controlled report/download/IPC/package and local Python/Rust routes as documented in the security contract. Preserve native witness limits, profile lifecycle and future public/plugin egress gates; the helper itself performs no I/O. |
> | REXC-OI-008 | Quarantine and legal review workflow | TBD | The helper blocks suspected/protected/private metadata but does not perform destructive quarantine movement or legal review routing. |
> | REXC-OI-009 | Cloud exception workflow | TBD | Default posture remains no cloud export or transmission unless separately approved; exception process and controls are not selected here. |
> | REXC-OI-010 | Storage roots and concrete paths | TBD | Concrete path indicators are controlled at the final sink. Use current native storage and accepted container authority for placement/package boundaries; preserve native witnesses and unaccepted private-path guarantees. |
>

### CLM-038 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | REXC-CON-001 | The former blanket runtime-absence premise is superseded by the current final-sink contract and its desktop/Python/Rust bindings. | `docs/_Registers/Deliverables.csv` row DEL-12-02 | `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`; package fan-in record | Specification Requirements; Procedure Future Implementation Procedure | Use current guarded-exposure/source-immutability/parity obligations; retain R15 native witness limits and separate legal/security, profile and future public/plugin gates. | Record premise repaired; no new ruling or lifecycle promotion |
> | REXC-CON-002 | Local private exports may need unredacted values, but shared/public outputs must avoid private/protected disclosure. | `docs/SPEC.md#8. Reporting and audit`; `docs/IP_AND_DATA_BOUNDARY.md#7. Report boundary` | SOW-040; OPS-K-PRIV-1 | Guidance Export Contexts; Specification REXC-REQ-006 | Allow explicit local-private export with warning/audit record; require redaction or block for shared/public contexts. | TBD |
> | REXC-CON-003 | The former blanket runtime-absence premise is superseded by the current final-sink contract and its desktop/Python/Rust bindings. | `TASK_RUN_2026-06-07_0935_redaction-export-hardening.md`; package fan-in `WORKING_ITEMS_RUN_2026-06-07_0957_TP-PKG12-REDACTION-SECRET-GUARD-CLOSEOUT.md` | `_STATUS.md` current `IN_PROGRESS`; Specification Documentation deferred items | Specification Verification; Procedure Future Implementation Procedure; Dependencies | Use current guarded-exposure/source-immutability/parity obligations; retain R15 native witness limits and separate legal/security, profile and future public/plugin gates. | Record premise repaired; no new ruling or lifecycle promotion |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-040 OBJ-010 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |

## Delivery commitments and evidence limits

These clauses retain open delivery duties, owner decisions, and bounded evidence limits. Their keys link to the finite source-retirement account. They do not assert completion, lift a hold, change an accepted scope boundary, or select execution work. The governing requirements and cited decisions control future implementation and acceptance.

- **DEL-12-02:1** — R5 retained delivery 2026-09-22 — DEL-12-02: retain native save/print witness limitations identified by R15 and separately governed profile, public/plugin transport, quarantine, cloud and legal/security gates. Claim refs: `DEL-12-02:SOW#CLM-011`.
