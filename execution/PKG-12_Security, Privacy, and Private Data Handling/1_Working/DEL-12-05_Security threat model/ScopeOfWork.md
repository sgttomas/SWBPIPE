---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-05
package_id: PKG-12
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-040]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-05

## Purpose and Objective Traceability

This Scope of Work defines `DEL-12-05` in service of project scope [SOW-040] and package objectives [OBJ-010].

- **OUT-001** — A security threat-model contract covering local storage, private data, reports and sharing, telemetry, plugins and imports, rule evaluation, secrets, hashes, and supply-chain exposure is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-12-05 Security threat model

> #### Datasheet: DEL-12-05 Security threat model
>

### CLM-002 — PDU-027 Evidence Slice

> ##### PDU-027 Evidence Slice
>
> - Selected control seam: `apps/desktop/src/services/telemetryPolicyService.ts` at telemetry-panel attempt grain.
> - Referenced warning classes: provenance, assumption, IP boundary, and privacy.
> - Explicit absence: no plugin/adapter/report/export interception and no runtime security result-envelope closure.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-12-05 |
> | Deliverable name | Security threat model |
> | Package ID | PKG-12 |
> | Package name | Security, Privacy, and Private Data Handling |
> | Deliverable type | DOC_UPDATE |
> | Scope item | SOW-040 |
> | Objective | OBJ-010 |
> | Context envelope | M |
> | Anticipated product artifact | docs/security/threat_model.md |
> | Setup artifact location | This deliverable folder only |
> | Current setup state | SEMANTIC_READY |
>
> Source basis: `_CONTEXT.md` sections "Description", "Anticipated Artifacts", "Scope Coverage", "Objective Support", and "Architecture Basis Injection"; `docs/_Registers/Deliverables.csv` row `DEL-12-05`; `docs/_Registers/ScopeLedger.csv` row `SOW-040`; `docs/_Registers/ContextBudgetQA.csv` row `DEL-12-05`.
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Primary purpose | Draft and maintain a threat model for private data, report sharing, plugins, imports, and supply chain exposure. | `_CONTEXT.md` Description; `docs/_Registers/Deliverables.csv` row `DEL-12-05` |
> | Data protection objective | Protect private project, code, rule-pack, and component data in local-first workflows. | `execution/_Decomposition/SOFTWARE_DECOMP.md` objective `OBJ-010` |
> | Governing scope | Private data handling controls, including redaction/export safeguards where reports or shared models may expose protected/private values. | `execution/_Decomposition/SOFTWARE_DECOMP.md` row `SOW-040` |
> | Product posture | Local-first; no cloud service required for modeling, solving, rule checking, or reporting. | `docs/PRD.md` section 18.1 |
> | Telemetry posture | Disabled by default; opt-in only if added; must not transmit private engineering data without explicit user action. | `docs/PRD.md` section 18.2; `docs/CONTRACT.md` `OPS-K-PRIV-2` |
> | Rule-pack posture | Private or user-owned; versioned, checksummed, source-noted, and marked public/private. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` `OPS-K-RULE-3` |
> | Plugin/import posture | Adapters and plugins cannot bypass validation, unit checks, provenance, diagnostics, sandboxing, or report controls. | `_CONTEXT.md` Architecture Basis Injection `AB-00-07`; `docs/SPEC.md` section 1 |
> | Report posture | Reports disclose provenance, warnings, limitations, and rule-pack identifiers without public protected content. | `docs/SPEC.md` section 8; `docs/IP_AND_DATA_BOUNDARY.md` section 7 |
> | Professional boundary | Software and agents must not certify, seal, approve, authenticate, or declare code compliance for reliance. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/DIRECTIVE.md` section 3 |
>

### CLM-005 — Conditions

> ##### Conditions
>

### CLM-006 — In Scope

> ###### In Scope
>
> | Area | Threat-model coverage |
> |---|---|
> | Private project data | Local project files, model contents, result envelopes, report manifests, hashes, and user settings that may identify project engineering work. |
> | Private code and rule-pack data | User-supplied rule packs, code-specific inputs, formulas, allowables, load combinations, rule-pack checksums, and source notices. |
> | Private library data | Material libraries, component libraries, manufacturer/vendor values, owner standards, and project templates kept in user-controlled private paths. |
> | Report sharing | Report export, report templates, bug reports, shared model packages, result tables, screenshots, and copy/export workflows that could disclose protected or private values. |
> | Plugins and adapters | Import/export plugins, scripting/plugin APIs, FEA handoff adapters, private-library importers, and external-tool interfaces. |
> | Supply chain | Application dependencies, plugins, adapter packages, build artifacts, packaging/signing choices, and provenance of imported data. |
>

### CLM-007 — Out of Scope

> ###### Out of Scope
>
> | Area | Boundary |
> |---|---|
> | Legal sufficiency | This deliverable is not a legal opinion and does not determine redistribution rights. |
> | Professional approval | This deliverable does not certify, approve, seal, or authenticate engineering work. |
> | Cloud service design | Cloud operation is excluded unless separately authorized by the human project authority. |
> | Detailed encryption implementation | Optional encrypted storage is noted as a product requirement; concrete key management remains `TBD`. |
> | Exact plugin permission model | Plugin permissions, transport protocol, and external format list remain implementation-level `TBD`. |
> | Protected standards content | No protected standards text, tables, figures, examples, formulas, allowables, or copied code data are included. |
>

### CLM-008 — Construction

> ##### Construction
>
> This setup document represents the planned content for `docs/security/threat_model.md` without writing that repo-level product artifact. It is organized as a maintainable threat-model basis for later architecture and implementation work.
>

### CLM-009 — Threat Surfaces

> ###### Threat Surfaces
>
> | Surface | Representative exposure | Required model treatment |
> |---|---|---|
> | Local storage | Private project/rule/library files readable from user paths. | Record trust boundary, asset class, confidentiality risk, and storage-control questions. |
> | Export/report | Reports, shared models, bug reports, result exports, and screenshots may carry private or protected values. | Record redaction, disclosure warning, provenance, and protected-content lint controls. |
> | Telemetry | Diagnostic or usage collection could disclose private engineering data. | Record off-by-default posture and explicit opt-in/user-action requirements. |
> | Plugins/adapters | Plugin, import/export, scripting, and FEA handoff flows could bypass validation or exfiltrate data. | Record no-bypass boundary, sandbox/permission questions, validation gates, and diagnostics. |
> | Rule evaluator | Rule packs may contain private code data and expressions. | Record sandboxing, checksums, private/public metadata, and no arbitrary-code execution. |
> | Supply chain | Dependencies, plugin packages, build artifacts, and importer libraries could be compromised or unreviewed. | Record provenance, dependency review, build reproducibility, and release-gate questions. |
>

### CLM-010 — Asset Classes

> ###### Asset Classes
>
> | Asset class | Public/private posture | Notes |
> |---|---|---|
> | Public mechanics code | Public if original/permissive and protected-content clean. | Must not embed protected standards data. |
> | Project model data | Private by default. | May include owner/project information, geometry, loads, results, and assumptions. |
> | Rule packs | Private by default unless explicitly public/permissive. | Must carry source notice, redistribution status, version, and checksum. |
> | Material/component libraries | Private by default unless provenance and redistribution rights are documented. | Public contribution requires provenance and review. |
> | Reports and exports | Potentially private. | Must support warnings/redaction and avoid protected public templates. |
> | Diagnostics and telemetry | Private-sensitive. | Must avoid private engineering/code data by default. |
> | Secrets | Private. | Concrete storage and rotation controls remain `TBD`. |
>

### CLM-011 — Control Themes

> ###### Control Themes
>
> | Theme | Control intent | Source |
> |---|---|---|
> | Local-first default | Prevent hidden network dependency or hidden cloud storage for ordinary workflows. | `docs/PRD.md` 18.1; `docs/DIRECTIVE.md` section 4.2 |
> | Explicit disclosure action | Require user intent before private data leaves local control. | `docs/PRD.md` 18.2-18.3 |
> | Redaction and warning | Warn before export and redact private data from bug reports. | `docs/PRD.md` 18.3; `execution/_Decomposition/SOFTWARE_DECOMP.md` `SOW-040` |
> | Provenance and redistribution | Record source, license/redistribution status, and review disposition. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/CONTRACT.md` `OPS-K-IP-2` |
> | Sandboxed evaluation | Keep rule packs from executing arbitrary code. | `docs/SPEC.md` section 6; `docs/CONTRACT.md` `OPS-K-RULE-2` |
> | No-bypass adapters | Ensure plugins/adapters cannot skip validation, provenance, diagnostics, sandboxing, or report controls. | `_CONTEXT.md` `AB-00-07`; `docs/SPEC.md` section 1 |
> | Reproducible evidence | Bind hashes, checksums, versions, and warnings into reports and manifests. | `_CONTEXT.md` `AB-00-04`; `docs/SPEC.md` section 8 |
>

### CLM-012 — References

> ##### References
>
> | Reference | Use in this setup artifact |
> |---|---|
> | `INIT.md` | Bootstrap boundaries for open mechanics, protected data, professional responsibility, and local-first agent work. |
> | `AGENTS.md` | TASK dispatch scope and bounded execution rule. |
> | `docs/DIRECTIVE.md` | Product principles, stop rules, public/private data boundary, and professional-responsibility limits. |
> | `docs/CONTRACT.md` | Invariants for IP, data, privacy, rule packs, reports, authority, and agents. |
> | `docs/TYPES.md` | Canonical terms for protected standards data, user-supplied code data, private rule packs, and analysis statuses. |
> | `docs/SPEC.md` | Layer responsibilities, rule-pack evaluator requirements, diagnostics, reports, and acceptance semantics. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data policy, provenance fields, quarantine rule, and report boundary. |
> | `docs/PRD.md` | Security/privacy requirements, private-data handling, report prohibitions, and risk table. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-12, SOW-040, OBJ-010, and AB-00 architecture-basis constraints. |
> | `docs/_Registers/*.csv` | Machine-readable deliverable, scope, and context-budget rows. |
>

### CLM-013 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. `docs/security/threat_model.md` now exists as the current project-owned threat-model surface. It records threats, boundaries, and residual mitigations; it is not a penetration-test result, security certification, or assurance ruling.

## Completion and Reliance Basis — Epistemology

### CLM-014 — Specification: DEL-12-05 Security threat model

> #### Specification: DEL-12-05 Security threat model
>

### CLM-015 — D-41 R5 T3 PDU-027 Runtime-Control Evidence

> ##### D-41 R5 T3 PDU-027 Runtime-Control Evidence
>
> The desktop threat-model packet names the selected DEL-12-03 pre-payload telemetry guard seam and explicitly records that plugin/adapter/report/export interception is not implemented. It references the established `PROVENANCE_WARNING`, `ASSUMPTION_WARNING`, `IP_BOUNDARY_WARNING`, and `PRIVACY_WARNING` classes without claiming a product result-envelope producer or security-review closure. This is bounded threat-model evidence only.
>

### CLM-016 — Scope

> ##### Scope
>
> This specification defines the deliverable-local setup content for the planned OpenPipeStress security threat model. It covers private data handling threats for local-first workflows, report sharing, shared model/export paths, plugins, imports, rule packs, private libraries, and supply chain exposure.
>
> This document is not the product artifact `docs/security/threat_model.md`; it is the setup-stage source content inside the sealed DEL-12-05 folder. It must not introduce protected standards content, real private project data, real secrets, legal sufficiency claims, certification claims, or professional approval claims.
>
> Source basis: `_CONTEXT.md`, `execution/_Decomposition/SOFTWARE_DECOMP.md` rows `PKG-12`, `SOW-040`, `OBJ-010`, architecture basis rows `AB-00-01/02/03/04/06/07/08`, `docs/CONTRACT.md`, `docs/PRD.md` section 18, and `docs/IP_AND_DATA_BOUNDARY.md`.
>

### CLM-017 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | STM-REQ-001 | The threat model must identify private project, code, rule-pack, material, component, report, diagnostic, plugin/import, and supply-chain assets as protected or private-sensitive where applicable. | `OBJ-010`; `docs/TYPES.md` sections 6-8; `docs/PRD.md` 17.3, 18.3 | Asset table includes each asset family or marks omissions `TBD`. |
> | STM-REQ-002 | The threat model must preserve the local-first default and must not assume cloud operation unless separately approved. | `docs/PRD.md` 18.1; `execution/_Decomposition/SOFTWARE_DECOMP.md` `PKG-12` exclusion | Trust-boundary table includes local/default and cloud-out-of-scope notes. |
> | STM-REQ-003 | Telemetry threats must assume telemetry is disabled by default and opt-in if implemented, with no private engineering/code data transmitted without explicit user action. | `docs/PRD.md` 18.2; `OPS-K-PRIV-2` | Telemetry threat row and control notes are present. |
> | STM-REQ-004 | Export, report, shared-model, and bug-report threats must include redaction and explicit warning controls for protected/private values. | `SOW-040`; `docs/PRD.md` 18.3; `docs/PRD.md` 17.3 | Report/export threat rows include redaction, warning, and protected-content lint controls. |
> | STM-REQ-005 | Public report templates and examples must not reproduce protected standards content, proprietary formulas, protected tables, or private project data by default. | `OPS-K-IP-1`; `OPS-K-REPORT-2`; `docs/SPEC.md` section 8 | Public-template threat/control rows include protected-content review. |
> | STM-REQ-006 | Rule-pack threats must include sandboxing, checksums, versioning, source notices, redistribution status, and private/public marking. | `OPS-K-RULE-2`; `OPS-K-RULE-3`; `docs/SPEC.md` section 6 | Rule-pack threat rows include sandbox and provenance controls. |
> | STM-REQ-007 | Plugin, adapter, import/export, and FEA handoff threats must preserve the no-bypass boundary for unit checks, provenance, diagnostics, validation, sandboxing, and report controls. | `AB-00-07`; `docs/SPEC.md` section 1; `SOW-038` context | Plugin/import threat rows include no-bypass controls and implementation `TBD` fields. |
> | STM-REQ-008 | Import and public contribution threats must include provenance, redistribution status, protected-content suspicion, quarantine, and human/legal review paths. | `OPS-K-IP-2`; `OPS-K-IP-3`; `docs/IP_AND_DATA_BOUNDARY.md` sections 4-5 | Import/contribution threat rows include provenance and quarantine controls. |
> | STM-REQ-009 | Diagnostics and result envelopes referenced by the threat model must use warning classes that include provenance, assumption, and IP-boundary signals where relevant. | `AB-00-06`; `docs/SPEC.md` section 7 | Control rows mention diagnostic classes and affected output paths. |
> | STM-REQ-010 | The threat model must distinguish mechanics solved, user-rule checked, and human-approved states; it must not imply automatic code compliance or professional approval. | `OPS-K-AUTH-1`; `OPS-K-MECH-2`; `docs/TYPES.md` section 4 | Authority-boundary section is present and contains no certification/seal claims. |
> | STM-REQ-011 | Supply-chain threats must include dependency/plugin package provenance, build/release artifact integrity, open-source license review, and protected-content review gates where applicable. | `AB-00-08`; `docs/PRD.md` technology considerations and risk table | Supply-chain threat rows include provenance/review gates and `TBD` implementation details. |
> | STM-REQ-012 | Unknown implementation choices must remain `TBD` rather than being invented. | `OPS-K-AGENT-1`; `_CONTEXT.md` "Still TBD" | Open questions list marks exact permission, encryption, package, and transport decisions `TBD`. |
>

### CLM-018 — Threat Inventory

> ###### Threat Inventory
>
> | Threat ID | Surface | Threat | Initial risk posture | Required controls or open questions |
> |---|---|---|---|---|
> | STM-T-001 | Local storage | Private project, rule-pack, material, component, or owner data is stored in a public/example path or committed accidentally. | High | Default private paths, public/example exclusion, protected-content/provenance review, `IP_BOUNDARY_WARNING`, and contribution gates. |
> | STM-T-002 | Report/export | Report, shared model, result export, screenshot, or copy operation discloses private values or protected code-derived content. | High | Redaction workflow, clear export warnings, public-template protected-content linter, provenance summary, and user explicit action. |
> | STM-T-003 | Bug reports | Diagnostic package or bug report includes private project files, calculation results, paths, rule packs, or libraries. | High | Bug-report redaction, attachment review, telemetry exclusion, and explicit user attachment. |
> | STM-T-004 | Telemetry | Optional telemetry transmits private project/code data or calculation results. | High | Disabled by default, opt-in, no private engineering/code data, explicit user action for any payload containing private data. |
> | STM-T-005 | Plugin/adapter | Plugin, scripting API, import/export adapter, or FEA handoff bypasses validation, provenance, sandboxing, diagnostics, or report controls. | High | No-bypass architecture boundary, permission model `TBD`, schema validation, sandboxing, and diagnostics. |
> | STM-T-006 | Import | Imported private/proprietary/component/material data lacks provenance or appears copied from protected standards. | High | Provenance fields, redistribution status, protected-suspected quarantine, rejection/escalation path, and human/legal review. |
> | STM-T-007 | Rule evaluator | Rule pack expression executes arbitrary code or leaks private rule contents through diagnostics/reports. | High | Sandboxed evaluator, deterministic/unit-aware expression handling, private/public marking, checksum, and report redaction. |
> | STM-T-008 | Supply chain | Dependency, plugin package, build artifact, or release package is compromised, unlicensed, or provenance-weak. | Medium/High | Dependency review, license review, reproducible build/release gates, package checksums/signing `TBD`, and protected-content gates. |
> | STM-T-009 | Secrets | Tokens, license credentials, private-library passwords, or signing keys are stored in project files or reports. | High | Secret scanning `TBD`, local secret storage policy `TBD`, report/export exclusion, and no real secrets in examples/tests. |
> | STM-T-010 | Hash/provenance spoofing | Hashes/checksums or source notices do not bind to the actual private rule/library/report content. | Medium/High | Canonical JSON/JCS-compatible hash basis for JSON payloads, manifest hashes for non-JSON assets, and re-review after changes. |
>

### CLM-019 — Standards

> ##### Standards
>
> No external engineering code or protected standards text is incorporated by this setup deliverable.
>
> | Standard or policy basis | Status in this deliverable |
> |---|---|
> | OpenPipeStress `CONTRACT.md` invariants | Binding project constraints for this setup work. |
> | OpenPipeStress `DIRECTIVE.md` stop rules | Binding stop/escalation rules for protected data, missing values, and overclaims. |
> | OpenPipeStress `IP_AND_DATA_BOUNDARY.md` | Governing policy for public/private data, provenance, and quarantine. |
> | OpenPipeStress `SPEC.md` | Technical baseline for layers, rule-pack evaluator, diagnostics, reporting, and acceptance semantics. |
> | JSON Schema 2020-12 | Architecture baseline for future schemas/interchange; no schema is authored here. |
> | External legal/compliance standards | `TBD`; not interpreted here. |
>

### CLM-020 — Verification

> ##### Verification
>
> | Check | Acceptance signal |
> |---|---|
> | Four-document setup kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist with default sections. |
> | Scope match | Documents reference `DEL-12-05`, `PKG-12`, `SOW-040`, and `OBJ-010` without expanding to unrelated deliverables. |
> | Boundary compliance | No protected standards text/tables/data, no real private project data, no real secrets, and no certification/approval/seal claims. |
> | Local-first posture | Cloud operation remains out of scope unless separately approved; telemetry is off by default. |
> | Threat coverage | Threat inventory covers reports/sharing, plugins/adapters, imports, rule packs, private libraries, telemetry, local storage, secrets, and supply chain. |
> | Open questions | Implementation-level unknowns are marked `TBD`. |
> | Semantic outputs | `_SEMANTIC.md` has no matrix audit errors and final result tables do not contain algebra/operator leaks. |
> | Dependency register | `Dependencies.csv` validates against `tools/validation/validate_dependencies_schema.py`. |
> | Lifecycle state | `_STATUS.md` remains `SEMANTIC_READY`, not `ISSUED`. |
>

### CLM-021 — Documentation

> ##### Documentation
>
> The eventual product artifact should be `docs/security/threat_model.md`; this run intentionally writes only deliverable-local setup content. A later implementation or documentation task may promote accepted content into product documentation after human review.
>
> Required records for this setup run:
>
> - Four document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
> - Semantic artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`.
> - Dependency artifacts: `Dependencies.csv`, `_DEPENDENCIES.md`.
> - Status artifact: `_STATUS.md` with `SEMANTIC_READY`.
> - Run records under `_run_records/` for P1/P2, semantic matrix, lens register, P3, and dependency extraction.
>

### CLM-022 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. `docs/security/threat_model.md` now exists as the current project-owned threat-model surface. It records threats, boundaries, and residual mitigations; it is not a penetration-test result, security certification, or assurance ruling.

- **AC-001** — The contract preserves the source-defined assets, threat inventory, trust boundaries, local-first and explicit-disclosure controls, provenance/quarantine/sandbox/no-bypass requirements, bounded telemetry-guard witness and explicit interception absences, open implementation questions, update triggers, and the distinction from penetration testing, legal sufficiency, security certification, professional approval, or code-compliance assurance.

## Production and Verification Method — Praxeology

### CLM-023 — Procedure: DEL-12-05 Security threat model

> #### Procedure: DEL-12-05 Security threat model
>

### CLM-024 — PDU-027 Check

> ##### PDU-027 Check
>
> 1. Verify the packet identifies the actual bounded telemetry guard seam.
> 2. Verify provenance, assumption, IP-boundary, and privacy warning classes remain explicit.
> 3. Verify plugin/adapter/report/export interception is false and no result-envelope or security-review closure is claimed.
>

### CLM-025 — Purpose

> ##### Purpose
>
> This procedure defines how a bounded TASK worker drafts, refreshes, and checks the DEL-12-05 security threat model setup content. It is intended for deliverable-local setup work only and does not publish `docs/security/threat_model.md`.
>

### CLM-026 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Required handling |
> |---|---|
> | Sealed deliverable context | Confirm `DEL-12-05`, `PKG-12`, `SOW-040`, `OBJ-010`, and the allowed write scope before editing. |
> | Governance sources | Read `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, relevant `SOFTWARE_DECOMP.md` revision 0.7 rows, register rows, and deliverable-local metadata. |
> | Source boundary | Use only accessible source material; mark missing implementation details `TBD`. |
> | Write boundary | Write only inside this deliverable folder. Do not create or edit repo-level `docs/security/threat_model.md`. |
> | Protected/private data boundary | Do not include protected standards text/tables/data, real private project data, real secrets, legal sufficiency claims, certification claims, or professional approval claims. |
> | Status boundary | Keep `_STATUS.md` at `SEMANTIC_READY` after a successful setup refresh; do not mark `ISSUED`. |
>

### CLM-027 — Steps

> ##### Steps
>
> 1. Confirm the deliverable identity and scope.
>    - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, and `docs/_Registers/ContextBudgetQA.csv`.
>    - Verify the scope is limited to DEL-12-05 setup content.
>
> 2. Re-read governing sources.
>    - Use `docs/CONTRACT.md` for invariant constraints.
>    - Use `docs/PRD.md` sections 17-18 for private data, telemetry, report, and bug-report requirements.
>    - Use `docs/SPEC.md` sections 1 and 6-8 for architecture, rule-pack, diagnostics, and reporting boundaries.
>    - Use `docs/IP_AND_DATA_BOUNDARY.md` for provenance, quarantine, private data, and report boundaries.
>    - Use `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for `PKG-12`, `SOW-040`, `OBJ-010`, and architecture basis rows `AB-00-01/02/03/04/06/07/08`.
>
> 3. Draft or refresh the four documents.
>    - Keep `Datasheet.md` sections `Identification`, `Attributes`, `Conditions`, `Construction`, and `References`.
>    - Keep `Specification.md` sections `Scope`, `Requirements`, `Standards`, `Verification`, and `Documentation`.
>    - Keep `Guidance.md` sections `Purpose`, `Principles`, `Considerations`, `Trade-offs`, and `Examples`.
>    - Keep `Procedure.md` sections `Purpose`, `Prerequisites`, `Steps`, `Verification`, and `Records`.
>    - Label unknown implementation details `TBD`; label inferences `ASSUMPTION` if used.
>
> 4. Build the semantic matrix artifact.
>    - Refresh `_SEMANTIC.md` using the deliverable perspective and the four documents.
>    - Verify the final result tables for matrices `C`, `F`, `D`, `X`, and `E` have no algebra/operator leaks and no matrix errors.
>    - Set or verify `_STATUS.md` as `SEMANTIC_READY` on pass.
>
> 5. Build the semantic lensing register.
>    - Refresh `_SEMANTIC_LENSING.md` with complete `A`, `B`, `C`, `F`, `D`, `X`, and `E` lens coverage.
>    - Record only warranted gaps, conflicts, or questions; do not treat lens cells as authority.
>
> 6. Apply warranted lensing items conservatively.
>    - Re-read the target document section and relevant source slice before changing content.
>    - Incorporate only supported items.
>    - Convert unsupported items to `TBD` or add them to the conflict table.
>
> 7. Refresh dependency extraction.
>    - Refresh `Dependencies.csv` using v3.1 columns.
>    - Preserve human-owned coordination notes in `_DEPENDENCIES.md`.
>    - Capture anchor links to `SOW-040` and `OBJ-010`, plus execution dependencies on governing documents that directly constrain the threat model.
>
> 8. Create run records.
>    - Write one run record for each setup phase: four-documents P1/P2, semantic matrix, lens register, four-documents P3, and dependency extraction.
>    - Each record must include `run-status: SUCCESS` or a concrete failure.
>
> 9. Run validation.
>    - Run `python3 tools/validation/validate_dependencies_schema.py <ScopePath>/Dependencies.csv`.
>    - Check `_SEMANTIC.md` for `MatrixError`/`MATRIX_ERROR`.
>    - Check final result table rows for leaked algebra/operator tokens.
>

### CLM-028 — Verification

> ##### Verification
>
> | Verification item | Method |
> |---|---|
> | Four documents exist | File presence check for `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`. |
> | Default sections preserved | Heading check for required sections in all four documents. |
> | Semantic audit clean | Search `_SEMANTIC.md` for matrix errors and inspect final result tables for algebra/operator leaks. |
> | Lensing coverage complete | Confirm `_SEMANTIC_LENSING.md` includes matrix sections for `A`, `B`, `C`, `F`, `D`, `X`, and `E`. |
> | Dependency schema valid | Run the v3.1 dependency schema validator. |
> | Status safe | Confirm `_STATUS.md` says `Current State: SEMANTIC_READY`, not `ISSUED`. |
> | Write scope respected | Review changed paths before final report; no writes outside this deliverable folder. |
>

### CLM-029 — Records

> ##### Records
>
> Required records for this run:
>
> - `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_STATUS.md`
> - `_run_records/PHASE_2_2_four_documents_P1_P2_2026-04-30.md`
> - `_run_records/PHASE_2_3_semantic_matrix_2026-04-30.md`
> - `_run_records/PHASE_2_4_lens_register_2026-04-30.md`
> - `_run_records/PHASE_2_5_four_documents_P3_2026-04-30.md`
> - `_run_records/PHASE_2_6_dependency_extract_2026-04-30.md`

- **VER-001** — Validate the contract and review source parity, asset/threat/trust-boundary coverage, local-first disclosure and report controls, plugin/import/rule/supply-chain/secret handling, bounded control evidence and explicit absences, retained TBDs and update triggers, and non-assurance limits.

## Governing Values and Decisions — Axiology

### CLM-030 — Guidance: DEL-12-05 Security threat model

> #### Guidance: DEL-12-05 Security threat model
>

### CLM-031 — PDU-027 Guidance

> ##### PDU-027 Guidance
>
> Treat the DEL-12-03 desktop guard as one bounded control witness, not proof that all named trust boundaries are intercepted. Keep warning-class references distinct from a runtime result envelope and retain the packet's explicit false plugin/adapter/report/export-interception flag. Do not infer security validation or formal review closure.
>

### CLM-032 — Purpose

> ##### Purpose
>
> This guidance explains how to interpret and maintain the OpenPipeStress security threat model setup content for private data handling. The deliverable exists to make privacy, protected-content, report-sharing, plugin/import, and supply-chain risks visible before implementation details harden.
>
> The threat model is a development artifact. It supports architecture and review; it is not legal advice, professional engineering approval, certification, or a compliance attestation.
>

### CLM-033 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Local-first by default | Treat ordinary modeling, solving, rule checking, reporting, and private-library use as local workflows. Cloud operation requires separate approval. |
> | Explicit disclosure | Treat private data leaving local control as a user-intent event requiring warning, review, redaction, or explicit attachment/export action. |
> | Private by default | Treat project files, rule packs, private material/component libraries, owner standards, diagnostics, reports, and secrets as private unless the user intentionally contributes or exports them with documented rights. |
> | Provenance before reuse | Treat imported or contributed data without source/license/redistribution metadata as incomplete or suspect. |
> | Sandboxed extensibility | Treat plugins, adapters, importers, script APIs, and rule evaluators as untrusted boundaries until validation, sandboxing, permissions, diagnostics, and no-bypass controls are specified. |
> | Reports are decision support | Reports may disclose hashes, checksums, warnings, assumptions, and source notes, but public templates must not reproduce protected code text/tables or proprietary formulas. |
> | Human authority preserved | Software statuses and reports must not claim certification, seal, approval, authentication, or automatic code compliance. |
>
> Source basis: `docs/DIRECTIVE.md` sections 3-6, `docs/CONTRACT.md` invariant index, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/SPEC.md` sections 6-8, and `docs/PRD.md` sections 17-18.
>

### CLM-034 — Considerations

> ##### Considerations
>

### CLM-035 — Trust Boundaries

> ###### Trust Boundaries
>
> | Boundary | Why it matters | Conservative handling |
> |---|---|---|
> | User-controlled local filesystem | Private rule packs, material/component libraries, reports, and project files may coexist near public examples or source checkouts. | Separate public examples from private paths; warn on public/export destinations; scan contribution paths where applicable. |
> | Report/export boundary | Reports, result exports, screenshots, and shared model packages may expose private project data or protected code-derived values. | Redaction, export warnings, protected-content checks, and explicit user action. |
> | Bug-report boundary | Logs and diagnostics can disclose paths, project names, result values, rule-pack metadata, or private attachments. | Redacted default package; explicit attachment review; telemetry exclusion. |
> | Telemetry boundary | Even coarse diagnostics can become sensitive if tied to private engineering work. | Off by default; opt-in only; no private engineering/code data unless user explicitly chooses a payload. |
> | Plugin/import boundary | External code or file formats may bypass domain validation or introduce protected/proprietary data. | Validate through domain contracts; require provenance; sandbox where execution exists; record diagnostics. |
> | Rule-evaluator boundary | Rule expressions may include private licensed/proprietary logic and must not execute arbitrary code. | Sandboxed, deterministic, unit-aware evaluator with private/public markings and checksums. |
> | Supply-chain boundary | Dependencies, plugins, adapters, packages, and build artifacts may introduce malicious code or licensing/provenance gaps. | Dependency/license review, reproducible build gates, artifact integrity checks, and package provenance `TBD`. |
>

### CLM-036 — Update Triggers

> ###### Update Triggers
>
> Update this threat model when any of these change:
>
> - Public API transport, plugin permission model, or import/export format list is selected.
> - Physical project package/container, migration mechanism, or encryption/key-management choice is selected.
> - Rule expression grammar/library or sandbox approach is selected.
> - Telemetry is introduced, even as an opt-in feature.
> - Report/export behavior changes to include new fields, attachments, copy paths, or templates.
> - Build/package/release signing or dependency review policy changes.
> - A protected-content, private-data, or supply-chain issue is found in review.
>

### CLM-037 — Open Questions

> ###### Open Questions
>
> | Question | Current disposition |
> |---|---|
> | Should private rule packs or libraries be encrypted by default? | `TBD`; PRD notes optional encrypted storage and asks this as an open question. |
> | What is the concrete plugin permission model? | `TBD`; no-bypass architecture basis applies until resolved. |
> | What public API transport and import/export formats are supported? | `TBD`; schema-first envelopes are the current architecture basis. |
> | What secret storage and signing-key process is used? | `TBD`; no real secrets may appear in setup or public examples. |
> | What package/container format stores project data? | `TBD`; canonical JSON/JCS-compatible hash basis applies for JSON payload hashes. |
>

### CLM-038 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative position |
> |---|---|
> | Usability vs redaction friction | Prefer visible warnings and explicit user action where private/protected disclosure is plausible. |
> | Extensibility vs no-bypass controls | Prefer slower plugin/API enablement over adapters that bypass domain validation, provenance, diagnostics, or report controls. |
> | Rich reports vs protected-content risk | Prefer identifiers, checksums, source notes, warnings, and user-private templates over public templates that embed protected text or formulas. |
> | Local convenience vs secret safety | Prefer `TBD` and explicit secret-handling design over ad hoc credentials in project files, examples, logs, or reports. |
> | Open examples vs realism | Prefer invented/public/permissive examples with provenance over realistic data copied from standards, vendors, or commercial tools. |
> | Supply-chain speed vs review evidence | Prefer dependency/license/provenance review and reproducible package evidence over unreviewed package additions. |
>

### CLM-039 — Examples

> ##### Examples
>
> These examples are invented setup scenarios and contain no real project data, real secrets, or protected standards content.
>
> | Scenario | Threat-model interpretation |
> |---|---|
> | A user exports a calculation report for outside review. | The report path crosses the disclosure boundary; warnings, redaction options, provenance summaries, and professional-boundary notices must be present. |
> | A user attaches a private project file to a bug report. | Default bug-report packaging should redact private data; explicit attachment should be treated as a user disclosure action. |
> | A plugin imports a component library from an external file. | The import must pass schema, unit, provenance, redistribution, and protected-content checks before data is treated as usable. |
> | A private rule pack is referenced in a report. | The report may identify name, version, checksum, and source note; it should not expose protected or proprietary rule content unless the user intentionally uses a private template. |
> | A public example is proposed using vendor catalog values. | Accept only if redistribution rights and provenance are documented; otherwise quarantine or reject. |
>

### CLM-040 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No active source conflict was identified in this setup run. | N/A | N/A | N/A | N/A | N/A |
>

### CLM-041 — Pass 3 Semantic-Lensing Notes

> ##### Pass 3 Semantic-Lensing Notes
>
> The lensing pass emphasized three conservative enrichments that are now represented in the documents: update triggers, explicit supply-chain treatment, and explicit plugin/import no-bypass boundaries. Source rereads used `docs/PRD.md` sections 17-18, `docs/SPEC.md` sections 1 and 6-8, `docs/IP_AND_DATA_BOUNDARY.md` sections 4-7, and `_CONTEXT.md` Architecture Basis Injection.
>

### CLM-042 — D-41 R5 T7 PDU-054 current declaration

> ##### D-41 R5 T7 PDU-054 current declaration
>
> Earlier setup-era statements on this surface are retained as historical setup context where applicable; this section is the active current-state declaration. `docs/security/threat_model.md` now exists as the current project-owned threat-model surface. It records threats, boundaries, and residual mitigations; it is not a penetration-test result, security certification, or assurance ruling.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-040 OBJ-010 | CLM-014 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
