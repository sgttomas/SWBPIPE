# Specification: DEL-17-09 Export adapter SDK and additional targets

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-17-09-DECL-001`.

## Scope

DEL-17-09 defines a contract-level export adapter SDK and target-admission model for future community or additional export targets. It covers adapter obligations, target registry admission states, source-basis requirements, boundary contracts, and validation checklist expectations.

This deliverable shall not implement source code, schemas, runtime loaders, public endpoints, package manifests, sample adapters, tests, external parser behavior, executable harnesses, or target-specific writers. It shall not make compatibility, target support, release, formal validation, code-compliance, or professional-acceptance claims. SourcePath: `_CONTEXT.md`; SectionRef: Package Exclusions. SourcePath: sealed task brief; SectionRef: Deliverable-specific emphasis and acceptance criteria.

## Requirements

| Req ID | Requirement | Source |
|---|---|---|
| DEL-17-09-REQ-001 | The adapter SDK contract shall consume DEL-17-02 as its common export package/profile/stable-ID/manifest/loss-report contract and shall not redefine those package objects in incompatible terms. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Downstream Requirements. |
| DEL-17-09-REQ-002 | Additional targets shall provide their own admitted source basis before any target-specific behavior is described as supported, compatible, validated, or ready for use. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements; SourcePath: `DEL-17-02/Guidance.md`; SectionRef: Target-Specific Carryforward. |
| DEL-17-09-REQ-003 | Unknown, unsupported, version-sensitive, or source-insufficient target behavior shall remain `TBD`, `unsupported`, `omitted`, `approximated`, or `delegated` rather than becoming a support claim. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements and Loss Report Requirements. |
| DEL-17-09-REQ-004 | Adapter participation shall require target version basis, unit policy, coordinate policy, stable-ID policy, source-basis IDs, loss-report policy, diagnostics policy, and boundary notes at the contract level. Exact field names/layout remain `TBD` until later schema work. | SourcePath: `DEL-17-02/Datasheet.md`; SectionRef: Required Export-Profile Fields; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements; SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
| DEL-17-09-REQ-005 | Adapter targets shall preserve canonical identity directly or through sidecar mapping when target artifacts cannot carry OpenPipeStress IDs. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Stable ID Map Requirements. |
| DEL-17-09-REQ-006 | Every target adapter shall require a loss report, including successful exports, so exported, omitted, approximated, delegated, unsupported, and `TBD` behavior remains visible. | SourcePath: `DEL-17-02/Specification.md`; SectionRef: Loss Report Requirements; SourcePath: `DEL-17-02/Guidance.md`; SectionRef: Loss report. |
| DEL-17-09-REQ-007 | Adapter validation shall distinguish syntactic readiness, schema validation, unit and dimensional validation, provenance completeness, redistribution review, privacy classification, protected-content screening, mechanics readiness, rule-check readiness, export review, and human review. | SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts. |
| DEL-17-09-REQ-008 | Adapter and target registry records shall not grant filesystem, network, process, private-data, report-control, solver, rule-pack, or storage access by default. Runtime grant mechanics remain `TBD`. | SourcePath: `DEL-10-01/Specification.md`; SectionRef: REQ-12 and REQ-13. |
| DEL-17-09-REQ-009 | Public adapter templates and target examples shall use invented or otherwise rights-cleared data only, and shall not include protected standards data, proprietary commercial examples, copied vendor files, or private project data. | SourcePath: `docs/IP_AND_DATA_BOUNDARY.md`; SectionRef: Public repository may contain and Public repository must not contain. |
| DEL-17-09-REQ-010 | External execution, if a later target adapter needs it, shall be optional, user-owned, license-bound, and recorded as non-authoritative external-run evidence rather than professional acceptance. | SourcePath: `execution/_Decomposition/SOFTWARE_DECOMP.md`; SectionRef: SOW-075; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |
| DEL-17-09-REQ-011 | Adapter outputs, manifests, diagnostics, documentation, and checklist language shall not claim certification, sealing, approval, authentication, endorsement, code compliance, professional acceptance, formal solver validation, release readiness, or vendor compatibility. | SourcePath: `docs/CONTRACT.md`; SectionRef: OPS-K-AUTH-1, OPS-K-GOV-3, OPS-K-AGENT-4; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Boundary Requirements. |
| DEL-17-09-REQ-012 | Future implementation work shall be bounded to later sealed deliverables and shall not be implied by this Phase A contract kit. | SourcePath: `AGENTS.md`; SectionRef: Dispatch rule; SourcePath: `skills/four-documents/SKILL.md`; SectionRef: Non-goal and Non-negotiable constraints. |
| DEL-17-09-REQ-013 | Adapter contracts shall carry DEL-17-01 and DEL-17-02 source-basis refs, and any `source_basis_admitted` target shall carry target-specific source evidence beyond package-level contract refs. | SourcePath: `DEL-17-01/Specification.md`; SectionRef: Source Authority Requirements; SourcePath: `DEL-17-02/Specification.md`; SectionRef: Export Profile Requirements. |

Runtime permission taxonomy, sandbox capability approval, and grant-record ownership are not named by the available Phase A sources. They remain `TBD` and shall not be inferred from adapter manifest requests, target registry records, or candidate target names. SourcePath: `docs/SPEC.md`; SectionRef: 4.5 Plugin and extension domain contracts.

## Standards

No protected engineering standard, vendor format standard, proprietary example, material allowable table, SIF/flexibility table, protected dimensional table, code-specific acceptance criterion, or private owner/project data is incorporated into this deliverable.

Applicable internal standards and baselines:

- `docs/CONTRACT.md`, especially OPS-K-IP-1 through OPS-K-IP-3, OPS-K-DATA-1 through OPS-K-DATA-3, OPS-K-AUTH-1, OPS-K-UNIT-1, OPS-K-REPORT-1 through OPS-K-REPORT-2, OPS-K-PRIV-1 through OPS-K-PRIV-2, and OPS-K-AGENT-1 through OPS-K-AGENT-4.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data, provenance, quarantine, contribution review, and non-professional boundary policy.
- `docs/SPEC.md` section 4.5 for plugin and adapter no-bypass controls.
- `DEL-17-01` for admitted target source evidence and unresolved CAEPIPE/PCF/glTF target TBDs.
- `DEL-17-02` for export package, export profile, stable ID map, manifest, and loss-report contract objects.

External standards and target formats:

- Concrete additional target standards, formats, vendor APIs, target versions, and source-citation requirements are `TBD` until a later source-basis intake admits them.
- This deliverable makes no statement that any additional target is supported.

## Verification

| Requirement IDs | Verification approach |
|---|---|
| REQ-001, REQ-003, REQ-005, REQ-006 | Contract review confirms alignment with DEL-17-02 package/profile/stable-ID/manifest/loss-report semantics. |
| REQ-002, REQ-004, REQ-009 | Source-basis review confirms target-specific statements are either source-grounded, explicitly `TBD`, or excluded. |
| REQ-007, REQ-008 | Architecture/security review confirms adapter admission does not grant runtime permissions, storage access, external execution, or bypass privileges. |
| REQ-010, REQ-011 | Professional/IP boundary review confirms no external-run, compatibility, validation, release, code-compliance, or professional-acceptance claim is present. |
| REQ-012 | Sealed-scope review confirms no implementation files, schemas, tests, package manifests, public API endpoints, or sibling deliverable files were modified. |

Category-level acceptance detail for REQ-007:

| Validation category | Acceptance record for this contract phase |
|---|---|
| Syntactic readiness and schema validation | Record `TBD` until concrete schema layout exists; later records should cite schema validation evidence when available. |
| Unit and dimensional validation | Record pass/fail/TBD against declared unit and coordinate policies; no silent unit default is acceptable. |
| Provenance completeness | Record source-basis IDs, source location, license or redistribution posture, and review disposition, or mark the entry `TBD`/rejected. |
| Redistribution review, privacy classification, and protected-content screening | Record public/private/protected-content disposition before public artifact use. Suspected protected content is rejected or quarantined, not normalized into an example. |
| Mechanics readiness, rule-check readiness, and export review | Record only evidence-supported readiness; do not convert target parser success into code-compliance, professional-acceptance, or solver-validation claims. |
| Human review | Record reviewer role and signoff format as `TBD` unless a later sealed workflow names them. |

## Documentation

Required documentation artifacts for this Phase A deliverable:

- `Datasheet.md` identifying contract objects, source basis, conditions, and boundaries.
- `Specification.md` defining target-admission and adapter-boundary requirements without implementation commitments.
- `Guidance.md` explaining how to evaluate additional targets conservatively and preserve unresolved behavior as `TBD`.
- `Procedure.md` defining future intake, admission, validation, and closeout steps at contract level.
- `_run_records/TASK_RUN_*.md` recording this sealed TASK execution.

Required downstream artifacts remain future work and `TBD`:

- concrete schema file layout;
- concrete adapter runtime/API surface;
- permission taxonomy and sandbox mechanism;
- sample adapter implementation;
- target-specific source-basis dossiers;
- target-specific fixtures and validation tests.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict detected in Phase A sources. | NA | NA | NA | NA | TBD |
## D-41 R5 T2A canonicalization requirement (2026-07-12)

All DEL-17-09-produced JSON checksum records SHALL use `deterministic_sorted_compact_json_payload_hash` for the existing sorted-key compact ASCII-escaped Python JSON serializer and SHALL NOT claim RFC 8785/JCS conformance.

## D-41 R5 T4 PDU-004 held taxonomy

The current admission record fields are `check_id`, `category`, `description`, `status`, `required_before_target_admission`, `evidence_refs`, `affected_refs`, `human_review_required`, and `provenance`. The exact additional category taxonomy and the reviewer-role, signoff-format, and approval-artifact fields remain owner-unselected. `human_review_required=true` is only a boundary flag; it is not reviewer identity, signoff, acceptance, or approval.
