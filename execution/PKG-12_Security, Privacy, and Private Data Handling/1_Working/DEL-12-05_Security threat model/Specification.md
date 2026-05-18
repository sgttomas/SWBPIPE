# Specification: DEL-12-05 Security threat model

## Scope

This specification defines the deliverable-local setup content for the planned OpenPipeStress security threat model. It covers private data handling threats for local-first workflows, report sharing, shared model/export paths, plugins, imports, rule packs, private libraries, and supply chain exposure.

This document is not the product artifact `docs/security/threat_model.md`; it is the setup-stage source content inside the sealed DEL-12-05 folder. It must not introduce protected standards content, real private project data, real secrets, legal sufficiency claims, certification claims, or professional approval claims.

Source basis: `_CONTEXT.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md` rows `PKG-12`, `SOW-040`, `OBJ-010`, architecture basis rows `AB-00-01/02/03/04/06/07/08`, `docs/CONTRACT.md`, `docs/PRD.md` section 18, and `docs/IP_AND_DATA_BOUNDARY.md`.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| STM-REQ-001 | The threat model must identify private project, code, rule-pack, material, component, report, diagnostic, plugin/import, and supply-chain assets as protected or private-sensitive where applicable. | `OBJ-010`; `docs/TYPES.md` sections 6-8; `docs/PRD.md` 17.3, 18.3 | Asset table includes each asset family or marks omissions `TBD`. |
| STM-REQ-002 | The threat model must preserve the local-first default and must not assume cloud operation unless separately approved. | `docs/PRD.md` 18.1; `docs/_Decomposition/SOFTWARE_DECOMP.md` `PKG-12` exclusion | Trust-boundary table includes local/default and cloud-out-of-scope notes. |
| STM-REQ-003 | Telemetry threats must assume telemetry is disabled by default and opt-in if implemented, with no private engineering/code data transmitted without explicit user action. | `docs/PRD.md` 18.2; `OPS-K-PRIV-2` | Telemetry threat row and control notes are present. |
| STM-REQ-004 | Export, report, shared-model, and bug-report threats must include redaction and explicit warning controls for protected/private values. | `SOW-040`; `docs/PRD.md` 18.3; `docs/PRD.md` 17.3 | Report/export threat rows include redaction, warning, and protected-content lint controls. |
| STM-REQ-005 | Public report templates and examples must not reproduce protected standards content, proprietary formulas, protected tables, or private project data by default. | `OPS-K-IP-1`; `OPS-K-REPORT-2`; `docs/SPEC.md` section 8 | Public-template threat/control rows include protected-content review. |
| STM-REQ-006 | Rule-pack threats must include sandboxing, checksums, versioning, source notices, redistribution status, and private/public marking. | `OPS-K-RULE-2`; `OPS-K-RULE-3`; `docs/SPEC.md` section 6 | Rule-pack threat rows include sandbox and provenance controls. |
| STM-REQ-007 | Plugin, adapter, import/export, and FEA handoff threats must preserve the no-bypass boundary for unit checks, provenance, diagnostics, validation, sandboxing, and report controls. | `AB-00-07`; `docs/SPEC.md` section 1; `SOW-038` context | Plugin/import threat rows include no-bypass controls and implementation `TBD` fields. |
| STM-REQ-008 | Import and public contribution threats must include provenance, redistribution status, protected-content suspicion, quarantine, and human/legal review paths. | `OPS-K-IP-2`; `OPS-K-IP-3`; `docs/IP_AND_DATA_BOUNDARY.md` sections 4-5 | Import/contribution threat rows include provenance and quarantine controls. |
| STM-REQ-009 | Diagnostics and result envelopes referenced by the threat model must use warning classes that include provenance, assumption, and IP-boundary signals where relevant. | `AB-00-06`; `docs/SPEC.md` section 7 | Control rows mention diagnostic classes and affected output paths. |
| STM-REQ-010 | The threat model must distinguish mechanics solved, user-rule checked, and human-approved states; it must not imply automatic code compliance or professional approval. | `OPS-K-AUTH-1`; `OPS-K-MECH-2`; `docs/TYPES.md` section 4 | Authority-boundary section is present and contains no certification/seal claims. |
| STM-REQ-011 | Supply-chain threats must include dependency/plugin package provenance, build/release artifact integrity, open-source license review, and protected-content review gates where applicable. | `AB-00-08`; `docs/PRD.md` technology considerations and risk table | Supply-chain threat rows include provenance/review gates and `TBD` implementation details. |
| STM-REQ-012 | Unknown implementation choices must remain `TBD` rather than being invented. | `OPS-K-AGENT-1`; `_CONTEXT.md` "Still TBD" | Open questions list marks exact permission, encryption, package, and transport decisions `TBD`. |

### Threat Inventory

| Threat ID | Surface | Threat | Initial risk posture | Required controls or open questions |
|---|---|---|---|---|
| STM-T-001 | Local storage | Private project, rule-pack, material, component, or owner data is stored in a public/example path or committed accidentally. | High | Default private paths, public/example exclusion, protected-content/provenance review, `IP_BOUNDARY_WARNING`, and contribution gates. |
| STM-T-002 | Report/export | Report, shared model, result export, screenshot, or copy operation discloses private values or protected code-derived content. | High | Redaction workflow, clear export warnings, public-template protected-content linter, provenance summary, and user explicit action. |
| STM-T-003 | Bug reports | Diagnostic package or bug report includes private project files, calculation results, paths, rule packs, or libraries. | High | Bug-report redaction, attachment review, telemetry exclusion, and explicit user attachment. |
| STM-T-004 | Telemetry | Optional telemetry transmits private project/code data or calculation results. | High | Disabled by default, opt-in, no private engineering/code data, explicit user action for any payload containing private data. |
| STM-T-005 | Plugin/adapter | Plugin, scripting API, import/export adapter, or FEA handoff bypasses validation, provenance, sandboxing, diagnostics, or report controls. | High | No-bypass architecture boundary, permission model `TBD`, schema validation, sandboxing, and diagnostics. |
| STM-T-006 | Import | Imported private/proprietary/component/material data lacks provenance or appears copied from protected standards. | High | Provenance fields, redistribution status, protected-suspected quarantine, rejection/escalation path, and human/legal review. |
| STM-T-007 | Rule evaluator | Rule pack expression executes arbitrary code or leaks private rule contents through diagnostics/reports. | High | Sandboxed evaluator, deterministic/unit-aware expression handling, private/public marking, checksum, and report redaction. |
| STM-T-008 | Supply chain | Dependency, plugin package, build artifact, or release package is compromised, unlicensed, or provenance-weak. | Medium/High | Dependency review, license review, reproducible build/release gates, package checksums/signing `TBD`, and protected-content gates. |
| STM-T-009 | Secrets | Tokens, license credentials, private-library passwords, or signing keys are stored in project files or reports. | High | Secret scanning `TBD`, local secret storage policy `TBD`, report/export exclusion, and no real secrets in examples/tests. |
| STM-T-010 | Hash/provenance spoofing | Hashes/checksums or source notices do not bind to the actual private rule/library/report content. | Medium/High | Canonical JSON/JCS-compatible hash basis for JSON payloads, manifest hashes for non-JSON assets, and re-review after changes. |

## Standards

No external engineering code or protected standards text is incorporated by this setup deliverable.

| Standard or policy basis | Status in this deliverable |
|---|---|
| OpenPipeStress `CONTRACT.md` invariants | Binding project constraints for this setup work. |
| OpenPipeStress `DIRECTIVE.md` stop rules | Binding stop/escalation rules for protected data, missing values, and overclaims. |
| OpenPipeStress `IP_AND_DATA_BOUNDARY.md` | Governing policy for public/private data, provenance, and quarantine. |
| OpenPipeStress `SPEC.md` | Technical baseline for layers, rule-pack evaluator, diagnostics, reporting, and acceptance semantics. |
| JSON Schema 2020-12 | Architecture baseline for future schemas/interchange; no schema is authored here. |
| External legal/compliance standards | `TBD`; not interpreted here. |

## Verification

| Check | Acceptance signal |
|---|---|
| Four-document setup kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist with default sections. |
| Scope match | Documents reference `DEL-12-05`, `PKG-12`, `SOW-040`, and `OBJ-010` without expanding to unrelated deliverables. |
| Boundary compliance | No protected standards text/tables/data, no real private project data, no real secrets, and no certification/approval/seal claims. |
| Local-first posture | Cloud operation remains out of scope unless separately approved; telemetry is off by default. |
| Threat coverage | Threat inventory covers reports/sharing, plugins/adapters, imports, rule packs, private libraries, telemetry, local storage, secrets, and supply chain. |
| Open questions | Implementation-level unknowns are marked `TBD`. |
| Semantic outputs | `_SEMANTIC.md` has no matrix audit errors and final result tables do not contain algebra/operator leaks. |
| Dependency register | `Dependencies.csv` validates against `tools/validation/validate_dependencies_schema.py`. |
| Lifecycle state | `_STATUS.md` remains `SEMANTIC_READY`, not `ISSUED`. |

## Documentation

The eventual product artifact should be `docs/security/threat_model.md`; this run intentionally writes only deliverable-local setup content. A later implementation or documentation task may promote accepted content into product documentation after human review.

Required records for this setup run:

- Four document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`.
- Semantic artifacts: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`.
- Dependency artifacts: `Dependencies.csv`, `_DEPENDENCIES.md`.
- Status artifact: `_STATUS.md` with `SEMANTIC_READY`.
- Run records under `_run_records/` for P1/P2, semantic matrix, lens register, P3, and dependency extraction.

