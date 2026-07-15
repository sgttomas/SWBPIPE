---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-09
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-09

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-09` in service of project scope [SOW-030, SOW-074, SOW-075] and package objectives [OBJ-009, OBJ-017, OBJ-018].

- **OUT-001** — A contract-level export adapter SDK and conservative target-admission model for future community and additional targets is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-09 Export adapter SDK and additional targets

> #### Datasheet: DEL-17-09 Export adapter SDK and additional targets
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-09-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-09 |
> | Package ID | PKG-17 |
> | Package name | Export Format Interoperability |
> | Name | Export adapter SDK and additional targets |
> | Type | API_CONTRACT |
> | Scope items | SOW-030, SOW-074, SOW-075 |
> | Objectives | OBJ-009, OBJ-017, OBJ-018 |
> | Lifecycle role | Contract-level adapter extensibility target sequenced after DEL-17-01 source basis and DEL-17-02 common export contracts. |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | Contract surface | Adapter SDK contract, adapter template, target registry contract, and validation checklist at planning/contract level only. | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts. |
> | Primary upstream contract | DEL-17-02 export package, profile, stable ID map, manifest, and loss-report contract. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Downstream Requirements, DEL-17-09 row. |
> | Source-basis authority | DEL-17-01 admitted source basis and TBD register gate target-specific behavior. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements and Boundary Requirements. |
> | Adapter boundary | Adapters validate units, provenance, diagnostics, redistribution, and public/private data boundaries, and cannot bypass validation, sandboxing, envelopes, or report controls. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07. |
> | No-bypass extension posture | Plugins and adapters are denied by default until a later approved permission/runtime design authorizes bounded capability. | SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
> | Target-admission posture | Additional targets are candidate targets until they provide public, official, project-owned, or rights-cleared source basis and explicit loss/TBD records. | SourcePath: `DEL-17-02/Guidance.md`; SectionRef: Target-Specific Carryforward and Boundary Guidance. |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Status | Notes |
> |---|---|---|
> | Implementation code | OUT OF SCOPE | This Phase A deliverable does not create code, schemas, package manifests, sample adapters, public endpoints, runtime loaders, or compatibility artifacts. SourcePath: sealed task brief; SectionRef: Acceptance criteria. |
> | Public API promise | OUT OF SCOPE | Public API transport, endpoint syntax, plugin runtime, permission taxonomy, and exact code-generation tooling remain `TBD`. SourcePath: `DEL-10-01/Specification.md`; SectionRef: REQ-13. |
> | Concrete additional target support | TBD | Additional targets may be evaluated only after source basis, target version basis, rights posture, loss categories, and validation obligations are recorded. SourcePath: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; SectionRef: Suggested Repository Layout and Main Risks. |
> | External tool execution | Optional/user-owned when applicable | External execution metadata must not imply bundled executables, license independence, formal validation, or professional acceptance. SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
> | Protected/private content | Prohibited for public artifacts | Public artifacts must not include protected standards text, proprietary examples, private project data, code-specific values, or target files without redistribution rights. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: Public repository must not contain. |
>

### CLM-006 — Construction

> ##### Construction
>
> DEL-17-09 constructs a contract-level adapter SDK admission model, not an SDK implementation. The construction is composed of four planning objects:
>
> | Object | Contract role | Current status |
> |---|---|---|
> | Adapter SDK contract | Defines the obligations an export adapter must satisfy before it can participate in the PKG-17 export package/profile/manifest/loss-report workflow. | Drafted at document level; concrete runtime/API surface `TBD`. |
> | Adapter template | Describes the expected evidence and checklist sections for future target adapters without providing implementation code or examples from proprietary formats. | Drafted at document level; sample adapter implementation out of scope. |
> | Target registry contract | Defines target admission states, required source basis, boundary notes, target version basis, validation evidence, and unresolved-TBD tracking. | Drafted at document level; no target is admitted by this deliverable. |
> | Validation checklist | Lists checks for source basis, units, provenance, stable IDs, manifests, loss reports, diagnostics, privacy, protected-content screening, and professional-boundary wording. | Drafted at document level; test implementation out of scope. |
>
> Expected descriptive fields for later registry, profile, and checklist records:
>
> | Record object | Expected fields | Source |
> |---|---|---|
> | Target registry/profile record | `target_name`, admission state, target version basis or `TBD`, source-basis IDs or `TBD`, rights/redistribution posture, boundary notes, unresolved-TBD list, validation evidence reference, and no-support/no-claim statement. Exact schema field names remain `TBD`. | SourcePath: `DEL-17-02/Datasheet.md`; SectionRef: Required Export-Profile Fields. SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
> | Source-basis record | Source location, license or redistribution basis, admitted use, boundary notes, affected target behavior, review disposition, and source-basis ID. Exact source-basis ID field name remains `TBD` until later schema work. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements and Documentation Requirements. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Contribution evidence fields. |
> | Validation checklist record | Checklist item, validation category, expected evidence artifact, result status (`pass`, `fail`, `TBD`, or `not applicable`), reviewer note, and source or run-record reference. Exact record format remains `TBD`. | SourcePath: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; SectionRef: Validation Strategy. SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
>

### CLM-007 — References

> ##### References
>
> | Source | Use |
> |---|---|
> | `AGENTS.md` | Type 2 sealed-task dispatch boundary. |
> | `docs/CONTRACT.md` | Binding invariants for IP, authority, unit, data, privacy, report, governance, and agent boundaries. |
> | `docs/SPEC.md` | Plugin/adapter no-bypass contract and result export constraints. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content policy. |
> | `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Human-authored export strategy, adapter architecture, target validation, and risk basis. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-17, DEL-17-09, SOW, objective, architecture-basis, and open-issue context. |
> | `DEL-17-01` four-document kit plus `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md` | Source-admission and target-claim boundary. |
> | `DEL-17-02` four-document kit | Common export package/profile/stable-ID/manifest/loss-report contract consumed by DEL-17-09. |

### CLM-008 — D-41 R5 T2A canonicalization evidence (2026-07-12)

> ##### D-41 R5 T2A canonicalization evidence (2026-07-12)
>
> Adapter SDK package checksums use `deterministic_sorted_compact_json_payload_hash`: sorted-key compact ASCII-escaped Python JSON. The label records the implemented deterministic bytes and makes no RFC 8785/JCS claim.
>

### CLM-009 — D-41 R5 T4 PDU-004 evidence state

> ##### D-41 R5 T4 PDU-004 evidence state
>
> The implemented nine-category checklist is admission metadata only. Distinct mechanics-readiness and rule-check-readiness categories are absent, and reviewer role, signoff format, and approval artifact remain `TBD` pending owner selection. No current field supplies a validation or readiness outcome.

## Completion and Reliance Basis — Epistemology

### CLM-010 — Specification: DEL-17-09 Export adapter SDK and additional targets

> #### Specification: DEL-17-09 Export adapter SDK and additional targets
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-011 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-09-DECL-001`.
>

### CLM-012 — Scope

> ##### Scope
>
> DEL-17-09 defines a contract-level export adapter SDK and target-admission model for future community or additional export targets. It covers adapter obligations, target registry admission states, source-basis requirements, boundary contracts, and validation checklist expectations.
>
> This deliverable shall not implement source code, schemas, runtime loaders, public endpoints, package manifests, sample adapters, tests, external parser behavior, executable harnesses, or target-specific writers. It shall not make compatibility, target support, release, formal validation, code-compliance, or professional-acceptance claims. SourcePath: `_CONTEXT.md`; SectionRef: Package Exclusions. SourcePath: sealed task brief; SectionRef: Deliverable-specific emphasis and acceptance criteria.
>

### CLM-013 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Source |
> |---|---|---|
> | DEL-17-09-REQ-001 | The adapter SDK contract shall consume DEL-17-02 as its common export package/profile/stable-ID/manifest/loss-report contract and shall not redefine those package objects in incompatible terms. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Downstream Requirements. |
> | DEL-17-09-REQ-002 | Additional targets shall provide their own admitted source basis before any target-specific behavior is described as supported, compatible, validated, or ready for use. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements; SourcePath: `DEL-17-02/Guidance.md`; SectionRef: Target-Specific Carryforward. |
> | DEL-17-09-REQ-003 | Unknown, unsupported, version-sensitive, or source-insufficient target behavior shall remain `TBD`, `unsupported`, `omitted`, `approximated`, or `delegated` rather than becoming a support claim. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements and Loss Report Requirements. |
> | DEL-17-09-REQ-004 | Adapter participation shall require target version basis, unit policy, coordinate policy, stable-ID policy, source-basis IDs, loss-report policy, diagnostics policy, and boundary notes at the contract level. Exact field names/layout remain `TBD` until later schema work. | SourcePath: `DEL-17-02/Datasheet.md`; SectionRef: Required Export-Profile Fields; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements; SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
> | DEL-17-09-REQ-005 | Adapter targets shall preserve canonical identity directly or through sidecar mapping when target artifacts cannot carry OpenPipeStress IDs. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Stable ID Map Requirements. |
> | DEL-17-09-REQ-006 | Every target adapter shall require a loss report, including successful exports, so exported, omitted, approximated, delegated, unsupported, and `TBD` behavior remains visible. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Loss Report Requirements; SourcePath: `DEL-17-02/Guidance.md`; SectionRef: Loss report. |
> | DEL-17-09-REQ-007 | Adapter validation shall distinguish syntactic readiness, schema validation, unit and dimensional validation, provenance completeness, redistribution review, privacy classification, protected-content screening, mechanics readiness, rule-check readiness, export review, and human review. | SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
> | DEL-17-09-REQ-008 | Adapter and target registry records shall not grant filesystem, network, process, private-data, report-control, solver, rule-pack, or storage access by default. Runtime grant mechanics remain `TBD`. | SourcePath: `DEL-10-01/Specification.md`; SectionRef: REQ-12 and REQ-13. |
> | DEL-17-09-REQ-009 | Public adapter templates and target examples shall use invented or otherwise rights-cleared data only, and shall not include protected standards data, proprietary commercial examples, copied vendor files, or private project data. | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: Public repository may contain and Public repository must not contain. |
> | DEL-17-09-REQ-010 | External execution, if a later target adapter needs it, shall be optional, user-owned, license-bound, and recorded as non-authoritative external-run evidence rather than professional acceptance. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: SOW-075; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
> | DEL-17-09-REQ-011 | Adapter outputs, manifests, diagnostics, documentation, and checklist language shall not claim certification, sealing, approval, authentication, endorsement, code compliance, professional acceptance, formal solver validation, release readiness, or vendor compatibility. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1, OPS-K-GOV-3, OPS-K-AGENT-4; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Boundary Requirements. |
> | DEL-17-09-REQ-012 | Future implementation work shall be bounded to later sealed deliverables and shall not be implied by this Phase A contract kit. | SourcePath: `AGENTS.md`; SectionRef: Dispatch rule; SourcePath: `skills/four-documents/SKILL.md`; SectionRef: Non-goal and Non-negotiable constraints. |
> | DEL-17-09-REQ-013 | Adapter contracts shall carry DEL-17-01 and DEL-17-02 source-basis refs, and any `source_basis_admitted` target shall carry target-specific source evidence beyond package-level contract refs. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
>
> Runtime permission taxonomy, sandbox capability approval, and grant-record ownership are not named by the available Phase A sources. They remain `TBD` and shall not be inferred from adapter manifest requests, target registry records, or candidate target names. SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts.
>

### CLM-014 — Standards

> ##### Standards
>
> No protected engineering standard, vendor format standard, proprietary example, material allowable table, SIF/flexibility table, protected dimensional table, code-specific acceptance criterion, or private owner/project data is incorporated into this deliverable.
>
> Applicable internal standards and baselines:
>
> - `docs/CONTRACT.md`, especially OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-1 through OPS-K-DATA-3, OPS-K-AUTH-1, OPS-K-UNIT-1, OPS-K-REPORT-1 through OPS-K-REPORT-2, OPS-K-PRIV-1 through OPS-K-PRIV-2, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
> - `docs/IP_AND_DATA_BOUNDARY.md` for public/private data, provenance, quarantine, contribution review, and non-professional boundary policy.
> - `docs/SPEC.md` section 4.5 for plugin and adapter no-bypass controls.
> - `DEL-17-01` for admitted target source evidence and unresolved CAEPIPE/PCF/glTF target TBDs.
> - `DEL-17-02` for export package, export profile, stable ID map, manifest, and loss-report contract objects.
>
> External standards and target formats:
>
> - Concrete additional target standards, formats, vendor APIs, target versions, and source-citation requirements are `TBD` until a later source-basis intake admits them.
> - This deliverable makes no statement that any additional target is supported.
>

### CLM-015 — Verification

> ##### Verification
>
> | Requirement IDs | Verification approach |
> |---|---|
> | REQ-001, REQ-003, REQ-005, REQ-006 | Contract review confirms alignment with DEL-17-02 package/profile/stable-ID/manifest/loss-report semantics. |
> | REQ-002, REQ-004, REQ-009 | Source-basis review confirms target-specific statements are either source-grounded, explicitly `TBD`, or excluded. |
> | REQ-007, REQ-008 | Architecture/security review confirms adapter admission does not grant runtime permissions, storage access, external execution, or bypass privileges. |
> | REQ-010, REQ-011 | Professional/IP boundary review confirms no external-run, compatibility, validation, release, code-compliance, or professional-acceptance claim is present. |
> | REQ-012 | Sealed-scope review confirms no implementation files, schemas, tests, package manifests, public API endpoints, or sibling deliverable files were modified. |
>
> Category-level acceptance detail for REQ-007:
>
> | Validation category | Acceptance record for this contract phase |
> |---|---|
> | Syntactic readiness and schema validation | Record `TBD` until concrete schema layout exists; later records should cite schema validation evidence when available. |
> | Unit and dimensional validation | Record pass/fail/TBD against declared unit and coordinate policies; no silent unit default is acceptable. |
> | Provenance completeness | Record source-basis IDs, source location, license or redistribution posture, and review disposition, or mark the entry `TBD`/rejected. |
> | Redistribution review, privacy classification, and protected-content screening | Record public/private/protected-content disposition before public artifact use. Suspected protected content is rejected or quarantined, not normalized into an example. |
> | Mechanics readiness, rule-check readiness, and export review | Record only evidence-supported readiness; do not convert target parser success into code-compliance, professional-acceptance, or solver-validation claims. |
> | Human review | Record reviewer role and signoff format as `TBD` unless a later sealed workflow names them. |
>

### CLM-016 — Documentation

> ##### Documentation
>
> Required documentation artifacts for this Phase A deliverable:
>
> - `Datasheet.md` identifying contract objects, source basis, conditions, and boundaries.
> - `Specification.md` defining target-admission and adapter-boundary requirements without implementation commitments.
> - `Guidance.md` explaining how to evaluate additional targets conservatively and preserve unresolved behavior as `TBD`.
> - `Procedure.md` defining future intake, admission, validation, and closeout steps at contract level.
> - `_run_records/TASK_RUN_*.md` recording this sealed TASK execution.
>
> Required downstream artifacts remain future work and `TBD`:
>
> - concrete schema file layout;
> - concrete adapter runtime/API surface;
> - permission taxonomy and sandbox mechanism;
> - sample adapter implementation;
> - target-specific source-basis dossiers;
> - target-specific fixtures and validation tests.
>

### CLM-017 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict detected in Phase A sources. | NA | NA | NA | NA | TBD |

### CLM-018 — D-41 R5 T2A canonicalization requirement (2026-07-12)

> ##### D-41 R5 T2A canonicalization requirement (2026-07-12)
>
> All DEL-17-09-produced JSON checksum records SHALL use `deterministic_sorted_compact_json_payload_hash` for the existing sorted-key compact ASCII-escaped Python JSON serializer and SHALL NOT claim RFC 8785/JCS conformance.
>

### CLM-019 — D-41 R5 T4 PDU-004 held taxonomy

> ##### D-41 R5 T4 PDU-004 held taxonomy
>
> The current admission record fields are `check_id`, `category`, `description`, `status`, `required_before_target_admission`, `evidence_refs`, `affected_refs`, `human_review_required`, and `provenance`. The exact additional category taxonomy and the reviewer-role, signoff-format, and approval-artifact fields remain owner-unselected. `human_review_required=true` is only a boundary flag; it is not reviewer identity, signoff, acceptance, or approval.

- **AC-001** — The contract preserves the common package/profile/stable-ID/manifest/loss contract, admitted target source basis, target registry states, no-bypass validation and permission boundaries, explicit behavior classifications, diagnostics, protected/private-data controls, optional user-owned external execution, and unresolved runtime, schema, target, reviewer, and approval details as TBD without implementing adapters or claiming support.

## Production and Verification Method — Praxeology

### CLM-020 — Procedure: DEL-17-09 Export adapter SDK and additional targets

> #### Procedure: DEL-17-09 Export adapter SDK and additional targets
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-021 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-09-DECL-004`.
>

### CLM-022 — Purpose

> ##### Purpose
>
> This procedure defines how future work should use the DEL-17-09 adapter SDK and target-admission contract. It is an operational contract for source intake, target admission, adapter review, validation planning, and closeout. It does not implement adapters, schemas, code, tests, public API endpoints, runtime loaders, or target-specific writers.
>

### CLM-023 — Prerequisites

> ##### Prerequisites
>
> Before a future target adapter or additional target can move beyond candidate status:
>
> 1. Read the DEL-17-01 four-document kit, `Source_Basis_Register.md`, and `CAEPIPE_Question_Dossier.md` when the target is affected by PKG-17 source-basis rules.
> 2. Read the DEL-17-02 four-document kit and use its export package, profile, stable-ID, manifest, and loss-report contract objects.
> 3. Confirm the target has public, official, project-owned, user-provided-with-rights, or otherwise rights-cleared source evidence.
> 4. Confirm protected standards data, proprietary examples, private project data, copied vendor files, and license-bypass material are not being introduced.
> 5. Confirm runtime permission, sandbox, public API transport, package layout, concrete schema layout, external execution, and target-specific validation remain `TBD` unless a later sealed brief authorizes them.
>

### CLM-024 — Steps

> ##### Steps
>

### CLM-025 — 1. Register a candidate target

> ###### 1. Register a candidate target
>
> 1. Record the target name only as a candidate.
> 2. Record source-basis status as `TBD` until source evidence is admitted.
> 3. Record that no support, compatibility, validation, release, code-compliance, or professional-acceptance claim is made.
> 4. Record any known IP, license, privacy, or redistribution concern as a blocker or `TBD`.
> 5. Record the target-admission decision artifact as `TBD` unless a later sealed brief names a registry file, run record, or decision log.
>

### CLM-026 — 2. Admit source basis

> ###### 2. Admit source basis
>
> 1. Classify each source as public official documentation, public specification, project-owned material, user-provided rights-cleared material, private-only material, or rejected/quarantined material.
> 2. Reject or quarantine suspected protected standards content, proprietary examples, copied target files, private project data, or unclear-redistribution material.
> 3. Record source location, license or redistribution basis, admitted use, boundary notes, and affected target behavior.
> 4. Mark unresolved or version-sensitive target behavior as `TBD`.
> 5. Record the source-basis admission owner and ruling record as `TBD` unless a later sealed brief assigns that responsibility.
>

### CLM-027 — 3. Draft the target profile contract

> ###### 3. Draft the target profile contract
>
> 1. Reference DEL-17-02 package/profile/stable-ID/manifest/loss-report concepts.
> 2. Declare target version basis or `TBD`.
> 3. Declare unit policy, coordinate policy, source-basis IDs, stable-ID behavior, loss-report policy, and external execution policy where applicable.
> 4. Classify each entity or behavior family as exported, omitted, approximated, delegated, unsupported, or `TBD`.
> 5. Preserve sidecar mapping requirements where target artifacts cannot carry canonical IDs.
>

### CLM-028 — 4. Review adapter boundary obligations

> ###### 4. Review adapter boundary obligations
>
> 1. Confirm adapter operations cannot bypass schema validation, unit checks, provenance checks, diagnostics/result envelopes, protected-content screening, private-data controls, report controls, solver boundaries, rule-pack sandboxing, or professional-boundary wording.
> 2. Confirm any filesystem, network, process, storage, private-data, or rule-pack access remains denied unless a later approved runtime/sandbox design grants a bounded capability.
> 3. Confirm target options are treated as pass-through configuration and not as OpenPipeStress code-checking logic.
>

### CLM-029 — 5. Prepare validation checklist

> ###### 5. Prepare validation checklist
>
> The checklist for any later target implementation should cover:
>
> | Checklist topic | Expected evidence artifact |
> |---|---|
> | Source-basis completeness | Source-basis record or dossier entry with source location, source-basis ID, license/redistribution posture, admitted use, and review disposition. |
> | Target version basis | Target profile record naming the target version basis or marking it `TBD`. |
> | Units and coordinate conventions | Profile field, validation record, or review note confirming unit and coordinate policy; unresolved cases remain `TBD`. |
> | Stable ID mapping and sidecars | ID-map artifact, sidecar reference, or manifest entry; omitted mappings require reasons. |
> | Export package member inventory | Manifest or package-member list with emitted files, diagnostics, reports, hashes where applicable, and declared omissions. |
> | Manifest and hash policy | Manifest record identifying deterministic text/JSON hashes or explaining why a hash is unavailable. |
> | Loss-report categories | Loss report with affected canonical IDs, category, severity, target artifact if any, reason, source-basis reference, and downstream implication. |
> | Diagnostics and severity routing | Validation report or diagnostics envelope preserving warnings, blockers, and target limitations. |
> | Privacy and protected-content screening | Review note or contribution disposition showing public/private/protected-content status and quarantine/rejection when needed. |
> | External execution ownership when applicable | Run manifest or review note showing user-owned executable/service, target version/profile version, preserved run artifact, and non-authoritative status. |
> | No compatibility, code-compliance, release, formal-validation, or professional-acceptance wording | Boundary review note confirming prohibited claims were not introduced. |
>

### CLM-030 — 6. Dispatch later implementation separately

> ###### 6. Dispatch later implementation separately
>
> 1. Do not implement adapter code from this Phase A contract.
> 2. Dispatch future implementation only under a sealed deliverable with explicit write scope.
> 3. Keep each target-specific writer, parser, harness, fixture, or schema change inside its approved deliverable scope.
> 4. Preserve all unresolved target behavior as `TBD` until the later deliverable has source and validation evidence.
>
> Candidate downstream dispatch notes, not dependency declarations:
>
> | Future work item | Candidate owner surface from current sources | Gate before dispatch |
> |---|---|---|
> | Concrete schema layout | DEL-17 target implementation deliverables or DEL-10 adapter framework work, as later scoped. | JSON Schema 2020-12 and package/profile contract alignment confirmed. |
> | Runtime/API surface and permission taxonomy | DEL-10-01 public API/plugin boundary and DEL-10-02 import/export adapter framework, as later scoped. | Runtime grants, sandbox capability approval, filesystem/network/process access, and package layout remain `TBD` until named by later authority. |
> | Sample adapter and adapter test harness | Later adapter SDK implementation tranche under PKG-17 or PKG-10. | Invented or rights-cleared fixtures only; no implementation from this Phase A contract. |
> | Target-specific source dossiers | DEL-17-01 source-basis workflow or a later target-specific source-intake deliverable. | Public, official, project-owned, user-provided-with-rights, or otherwise rights-cleared source basis admitted. |
> | Target-specific fixtures and validation tests | Later target implementation deliverables such as native JSON, CAEPIPE MBF, PCF, GLB/glTF, or additional adapters. | Fixture rights, target version basis, loss-report obligations, and validation evidence path recorded. |
>

### CLM-031 — Verification

> ##### Verification
>
> For this Phase A deliverable, verification consists of:
>
> - four-document structure check for `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`;
> - manual source-grounding review against local `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, DEL-17-01, DEL-17-02, project governance docs, and the export interoperability plan;
> - write-scope review confirming only authorized DEL-17-09 files were edited;
> - boundary review confirming no implementation code, schemas, public API promise, compatibility claim, target support claim, protected standards data, proprietary examples, release claim, code-compliance claim, formal-validation claim, or professional claim was introduced.
> - acceptance signoff record for these reviews remains `TBD`: reviewer role, signoff format, and required approval artifact are not named by the available sources.
>

### CLM-032 — Records

> ##### Records
>
> This Phase A run should leave:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_STATUS.md` safe state update from `OPEN` to `INITIALIZED` when applicable
> - `_run_records/TASK_RUN_*.md`
>
> Future target-admission runs should add records only under their own sealed write scope. Expected decision artifacts remain `TBD` unless a later brief names them; candidates include a target registry record, source-basis admission record, validation checklist record, and run record. `MEMORY.md` is intentionally not updated in this phase.

### CLM-033 — D-41 R5 T2A canonicalization check (2026-07-12)

> ##### D-41 R5 T2A canonicalization check (2026-07-12)
>
> Before accepting adapter package hashes, execute the exact-byte/fixed-hash vector, stable-order test, mutation test, schema/fixture validation, and explicit no-JCS assertion.
>

### CLM-034 — D-41 R5 T4 PDU-004 hold check

> ##### D-41 R5 T4 PDU-004 hold check
>
> Inventory only fields and categories emitted by the current builder. Record distinct mechanics/rule-check categories and reviewer/signoff/approval fields as absent and owner-unselected. Do not add placeholder schema fields or normalize existing evidence into a readiness or acceptance record.

- **VER-001** — Validate the contract and review source parity, target-admission and source-basis gates, adapter obligations, stable identity and loss visibility, no-bypass and default-deny permission boundaries, optional external-run limits, retained runtime/schema/reviewer TBDs, protected-content exclusions, and absence of implementation, target support, compatibility, release, compliance, validation, or professional-acceptance claims.

## Governing Values and Decisions — Axiology

### CLM-035 — Guidance: DEL-17-09 Export adapter SDK and additional targets

> #### Guidance: DEL-17-09 Export adapter SDK and additional targets
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-036 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-17-09-DECL-003`.
>

### CLM-037 — Purpose

> ##### Purpose
>
> Use DEL-17-09 as the boundary guide for future export adapters and additional targets. Its job is to keep extension work possible without allowing adapter code, target names, or community contribution pressure to weaken the PKG-17 source-basis, stable-ID, loss-report, protected-content, privacy, diagnostics, or professional-boundary controls.
>
> This guidance is contract-level only. It is not an implementation guide, public API promise, compatibility statement, or target support announcement.
>

### CLM-038 — Principles

> ##### Principles
>
> - Start from DEL-17-02 package/profile/stable-ID/manifest/loss-report contracts before considering any adapter-specific behavior.
> - Admit target behavior only from public, official, project-owned, user-provided-with-rights, or otherwise rights-cleared sources.
> - Keep candidate targets in a non-support state until source basis, target version basis, redistribution posture, validation expectations, and unresolved TBDs are recorded.
> - Treat every export as potentially lossy; require manifest, ID-map, diagnostics, and loss-report evidence even when the target file is produced.
> - Keep adapter work behind the same no-bypass controls as first-party export code: schema validation, unit checks, provenance, diagnostics, private-data controls, protected-content screening, report boundaries, and professional-boundary wording. SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts.
> - Prefer explicit `TBD`, `unsupported`, `omitted`, `approximated`, or `delegated` classifications over broad compatibility or support language.
>

### CLM-039 — Considerations

> ##### Considerations
>

### CLM-040 — Target admission

> ###### Target admission
>
> Additional targets should move through conservative admission states:
>
> | State | Meaning |
> |---|---|
> | Candidate | A target has been named for possible future review, but no support is claimed. |
> | Source-basis pending | Public or rights-cleared source evidence has not yet been admitted. Target behavior remains `TBD`. |
> | Source-basis admitted | Source evidence has been recorded with boundary notes, but support still depends on a later sealed implementation/validation deliverable. |
> | Contract-ready | The target profile can reference source-basis IDs, required fields, loss categories, stable-ID behavior, and validation obligations. |
> | Implementation-gated | Future code/test work may be dispatched under a separate sealed deliverable. DEL-17-09 does not perform that work. |
>
> Do not use these states as release, compatibility, vendor-certification, or professional acceptance states.
>

### CLM-041 — Adapter SDK surface

> ###### Adapter SDK surface
>
> The SDK surface should be described as obligations and review checkpoints until a later implementation brief authorizes code. Useful obligations include source basis, profile metadata, stable identity, package member declaration, loss reporting, diagnostics, protected-content screening, privacy classification, and professional-boundary wording. Exact method names, schemas, endpoints, loader mechanisms, and package structure remain `TBD`.
>
> Vocabulary note:
>
> | Term | Use in this deliverable |
> |---|---|
> | Adapter SDK contract | The normative obligation set for future exporters: source basis, target profile intake, validation, stable identity, loss reporting, diagnostics, privacy, protected-content screening, and professional-boundary controls. |
> | SDK surface | The future runtime/API methods, schemas, loader mechanics, and package structure. This remains `TBD` until later sealed implementation work. |
> | Target profile contract | The per-target declaration of target family, target version basis, source-basis IDs, unit and coordinate policy, supported/unsupported behavior, loss policy, and validation policy. |
> | Target registry contract | The admission record for candidate/source-basis/contract-ready/implementation-gated target states. It is not a release, compatibility, or professional-acceptance registry. |
>

### CLM-042 — Additional targets

> ###### Additional targets
>
> Additional targets may include plant-design exchanges, solver-neutral packages, geometry exports, or user-owned APIs only when the target brings a source basis and rights posture. A target name alone is not evidence. Common industry practice is not enough to close target behavior gaps.
>

### CLM-043 — External execution

> ###### External execution
>
> If a future adapter involves a downstream executable or hosted service, that execution must remain optional and user-owned unless a later human authority explicitly approves a different boundary. A successful external run may be recorded as regression or handoff evidence; it must not be described as professional acceptance, formal validation, code compliance, or vendor certification.
>
> External-run evidence is admissible only as bounded technical evidence when the user controls the executable/service, has the legal right to run it, preserves run artifacts, and records the target version, profile version, manifest or run record, and limitations. Exclude or quarantine external-run evidence when licensing, redistribution, private-data, protected-content, provenance, or target-version basis is unclear. Treat external-run evidence as non-authoritative context when it supports regression or handoff review but does not close source-basis, code-compliance, professional-acceptance, or formal-validation questions. SourcePath: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; SectionRef: CAEPIPE Scripted Validation Harness and Validation Strategy. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 6. Private/user-supplied data boundary.
>

### CLM-044 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Guidance |
> |---|---|
> | Community extensibility vs. governance control | Favor a narrow admission contract with required evidence and loss reporting. A target can remain useful while still carrying explicit limitations. |
> | Broad target list vs. source quality | Keep unsupported targets as candidates. Admit fewer targets with stronger source basis instead of naming many targets as supported. |
> | Adapter convenience vs. stable identity | Require stable ID maps and sidecars where target artifacts cannot carry canonical IDs directly. |
> | Fast file writing vs. auditability | Validation, manifests, diagnostics, and loss reports are part of the export outcome, not optional extras. |
> | Target-specific options vs. local code-checking | Pass-through target options may be recorded as target configuration, but shall not become OpenPipeStress code-checking logic. |
>

### CLM-045 — Examples

> ##### Examples
>
> Acceptable contract-level example:
>
> - A future target registry entry names a target as `Candidate`, records source-basis status as `TBD`, states that no support is claimed, and lists required evidence before implementation can be dispatched.
>
> Acceptable public template example:
>
> - An invented adapter checklist uses synthetic target names and invented model references to show where source basis, stable ID behavior, loss categories, and boundary notes would be recorded.
>
> Not acceptable:
>
> - A public fixture copied from a vendor sample, standards table, protected format manual, private project file, or licensed report without documented redistribution rights.
> - A statement that a named target is compatible, supported, validated, code-compliant, accepted for professional use, or release-ready before source basis and later sealed validation work exist.
>

### CLM-046 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No current conflict detected. | NA | NA | NA | NA | TBD |

### CLM-047 — D-41 R5 T2A canonicalization guidance (2026-07-12)

> ##### D-41 R5 T2A canonicalization guidance (2026-07-12)
>
> Keep the narrow sorted-compact JSON label coupled to exact-byte and mutation tests. Do not upgrade the label to JCS without a governed RFC 8785 implementation and conformance evidence.
>

### CLM-048 — D-41 R5 T4 PDU-004 guidance

> ##### D-41 R5 T4 PDU-004 guidance
>
> Do not infer missing categories from nearby evidence or convert the current `export_review`/`human_review` records into mechanics, rule-check, reviewer, signoff, or approval semantics. Preserve the exact missing taxonomy as an owner hold.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 SOW-075 OBJ-009 OBJ-017 OBJ-018 | CLM-010 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
