# Datasheet: DEL-17-09 Export adapter SDK and additional targets

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-09 |
| Package ID | PKG-17 |
| Package name | Export Format Interoperability |
| Name | Export adapter SDK and additional targets |
| Type | API_CONTRACT |
| Scope items | SOW-030, SOW-074, SOW-075 |
| Objectives | OBJ-009, OBJ-017, OBJ-018 |
| Lifecycle role | Contract-level adapter extensibility target sequenced after DEL-17-01 source basis and DEL-17-02 common export contracts. |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| Contract surface | Adapter SDK contract, adapter template, target registry contract, and validation checklist at planning/contract level only. | SourcePath: `_CONTEXT.md`; SectionRef: Anticipated Artifacts. |
| Primary upstream contract | DEL-17-02 export package, profile, stable ID map, manifest, and loss-report contract. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Downstream Requirements, DEL-17-09 row. |
| Source-basis authority | DEL-17-01 admitted source basis and TBD register gate target-specific behavior. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements and Boundary Requirements. |
| Adapter boundary | Adapters validate units, provenance, diagnostics, redistribution, and public/private data boundaries, and cannot bypass validation, sandboxing, envelopes, or report controls. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: AB-00-07. |
| No-bypass extension posture | Plugins and adapters are denied by default until a later approved permission/runtime design authorizes bounded capability. | SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
| Target-admission posture | Additional targets are candidate targets until they provide public, official, project-owned, or rights-cleared source basis and explicit loss/TBD records. | SourcePath: `DEL-17-02/Guidance.md`; SectionRef: Target-Specific Carryforward and Boundary Guidance. |

## Conditions

| Condition | Status | Notes |
|---|---|---|
| Implementation code | OUT OF SCOPE | This Phase A deliverable does not create code, schemas, package manifests, sample adapters, public endpoints, runtime loaders, or compatibility artifacts. SourcePath: sealed task brief; SectionRef: Acceptance criteria. |
| Public API promise | OUT OF SCOPE | Public API transport, endpoint syntax, plugin runtime, permission taxonomy, and exact code-generation tooling remain `TBD`. SourcePath: `DEL-10-01/Specification.md`; SectionRef: REQ-13. |
| Concrete additional target support | TBD | Additional targets may be evaluated only after source basis, target version basis, rights posture, loss categories, and validation obligations are recorded. SourcePath: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; SectionRef: Suggested Repository Layout and Main Risks. |
| External tool execution | Optional/user-owned when applicable | External execution metadata must not imply bundled executables, license independence, formal validation, or professional acceptance. SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
| Protected/private content | Prohibited for public artifacts | Public artifacts must not include protected standards text, proprietary examples, private project data, code-specific values, or target files without redistribution rights. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: Public repository must not contain. |

## Construction

DEL-17-09 constructs a contract-level adapter SDK admission model, not an SDK implementation. The construction is composed of four planning objects:

| Object | Contract role | Current status |
|---|---|---|
| Adapter SDK contract | Defines the obligations an export adapter must satisfy before it can participate in the PKG-17 export package/profile/manifest/loss-report workflow. | Drafted at document level; concrete runtime/API surface `TBD`. |
| Adapter template | Describes the expected evidence and checklist sections for future target adapters without providing implementation code or examples from proprietary formats. | Drafted at document level; sample adapter implementation out of scope. |
| Target registry contract | Defines target admission states, required source basis, boundary notes, target version basis, validation evidence, and unresolved-TBD tracking. | Drafted at document level; no target is admitted by this deliverable. |
| Validation checklist | Lists checks for source basis, units, provenance, stable IDs, manifests, loss reports, diagnostics, privacy, protected-content screening, and professional-boundary wording. | Drafted at document level; test implementation out of scope. |

Expected descriptive fields for later registry, profile, and checklist records:

| Record object | Expected fields | Source |
|---|---|---|
| Target registry/profile record | `target_name`, admission state, target version basis or `TBD`, source-basis IDs or `TBD`, rights/redistribution posture, boundary notes, unresolved-TBD list, validation evidence reference, and no-support/no-claim statement. Exact schema field names remain `TBD`. | SourcePath: `DEL-17-02/Datasheet.md`; SectionRef: Required Export-Profile Fields. SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
| Source-basis record | Source location, license or redistribution basis, admitted use, boundary notes, affected target behavior, review disposition, and source-basis ID. Exact source-basis ID field name remains `TBD` until later schema work. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements and Documentation Requirements. SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: 4. Contribution evidence fields. |
| Validation checklist record | Checklist item, validation category, expected evidence artifact, result status (`pass`, `fail`, `TBD`, or `not applicable`), reviewer note, and source or run-record reference. Exact record format remains `TBD`. | SourcePath: `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md`; SectionRef: Validation Strategy. SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |

## References

| Source | Use |
|---|---|
| `AGENTS.md` | Type 2 sealed-task dispatch boundary. |
| `docs/CONTRACT.md` | Binding invariants for IP, authority, unit, data, privacy, report, governance, and agent boundaries. |
| `docs/SPEC.md` | Plugin/adapter no-bypass contract and result export constraints. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data and protected-content policy. |
| `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` | Human-authored export strategy, adapter architecture, target validation, and risk basis. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | PKG-17, DEL-17-09, SOW, objective, architecture-basis, and open-issue context. |
| `DEL-17-01` four-document kit plus `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md` | Source-admission and target-claim boundary. |
| `DEL-17-02` four-document kit | Common export package/profile/stable-ID/manifest/loss-report contract consumed by DEL-17-09. |
## D-41 R5 T2A canonicalization evidence (2026-07-12)

Adapter SDK package checksums use `deterministic_sorted_compact_json_payload_hash`: sorted-key compact ASCII-escaped Python JSON. The label records the implemented deterministic bytes and makes no RFC 8785/JCS claim.

## D-41 R5 T4 PDU-004 evidence state

The implemented nine-category checklist is admission metadata only. Distinct mechanics-readiness and rule-check-readiness categories are absent, and reviewer role, signoff format, and approval artifact remain `TBD` pending owner selection. No current field supplies a validation or readiness outcome.
