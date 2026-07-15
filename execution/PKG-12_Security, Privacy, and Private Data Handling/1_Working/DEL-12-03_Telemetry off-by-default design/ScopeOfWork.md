---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-12-03
package_id: PKG-12
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@4d153302c3c4cd42578936db160c2bac1270225a
project_scope_refs: [SOW-037]
package_objective_refs: [OBJ-010]
---

# Scope of Work — DEL-12-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-12-03` in service of project scope [SOW-037] and package objectives [OBJ-010].

- **OUT-001** — A telemetry-off-by-default contract for fail-closed configuration, explicit affirmative request boundaries, approved allowlists, and pre-payload privacy enforcement is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-12-03 Telemetry off-by-default design

> #### Datasheet: DEL-12-03 Telemetry off-by-default design
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-12-03-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-12-03 |
> | Deliverable name | Telemetry off-by-default design |
> | Package ID | PKG-12 |
> | Package name | Security, Privacy, and Private Data Handling |
> | Deliverable type | SECURITY_CONTROL |
> | Scope item | SOW-037 |
> | Objective | OBJ-010 |
> | Context envelope | S |
> | Current local lifecycle state | IN_PROGRESS; this evidence alignment does not change `_STATUS.md` or promote lifecycle. |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Value | Source |
> |---|---|---|
> | MVP telemetry posture | No telemetry by default; telemetry may be a no-op in MVP. | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-037 and OI-008 |
> | User consent posture | Any telemetry, if later implemented, is opt-in only and requires explicit human approval before design activation. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OI-008; `docs/CONTRACT.md` OPS-K-PRIV-2 |
> | Forbidden telemetry content | Private project data, code-specific data, private rule-pack data, private material/component data, secrets, paths, report content, and protected standards content. | `docs/CONTRACT.md` OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-PRIV-1/2 |
> | Local-first boundary | No cloud operation is included unless separately authorized. | `_CONTEXT.md`; `docs/DIRECTIVE.md` Section 4.2 |
> | Current evidence artifacts | Telemetry policy, Python metadata guard, and desktop fail-closed attempt guard/policy-review panel exist. The desktop seam always remains disabled without an authorized affirmative-action/allowlist path and exposes no payload/network/persistence operation. Other consumer interception remains PDU-043 documented absence. | `_CONTEXT.md`; `docs/security/telemetry_policy.md`; `core/security/telemetry_policy/`; `tests/security/test_telemetry_policy.py`; `apps/desktop/src/services/telemetryPolicyService.ts`; `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` |
>

### CLM-005 — Conditions

> ##### Conditions
>
> | Condition | Required handling |
> |---|---|
> | Configuration key is absent | Treat telemetry as disabled. |
> | Configuration value is malformed | Treat telemetry as disabled and emit a diagnostic or validation finding. |
> | User has not explicitly opted in | Do not initialize telemetry transport, collection, background jobs, or upload queues. |
> | Human approval for runtime telemetry collection is absent | Keep runtime telemetry as no-op/default-off and record telemetry details as TBD. |
> | A payload field could expose private or protected data | Exclude the field and record an IP/privacy boundary finding. |
>

### CLM-006 — Construction

> ##### Construction
>
> This deliverable began as a design-boundary setup kit. Current evidence also includes a metadata-only guard helper, focused tests, and a desktop policy-review panel that harden and expose the default-off boundary before any telemetry payload construction. The panel's local JSON is a review artifact, not a telemetry payload. Neither the helper nor the panel is a runtime telemetry system or authorization for endpoint, vendor, transport, queue, upload job, persistence, retention, consent UI/CLI, support-bundle workflow, or professional/code-compliance authority.
>
> | Artifact | Construction intent | Current evidence result |
> |---|---|---|
> | Telemetry policy | State the default-off, opt-in, privacy-preserving rule set. | Captured in this kit and implemented as repo policy in `docs/security/telemetry_policy.md`. |
> | Config defaults | Define default-disabled behavior for absent, empty, unknown, unsupported, malformed, or incomplete telemetry metadata. | Implemented in `core/security/telemetry_policy/` as metadata-only config resolution; exact product config schema and storage location remain `TBD`. |
> | Tests | Prove default-off, fail-closed, allowlist-gated, no-payload, and no-network-default behavior for the metadata-only guard. | Covered by `tests/security/test_telemetry_policy.py`; Worker B reported 15 passing tests and parent fan-in reported 44 passing PKG-12 privacy-guard tests. |
>

### CLM-007 — June 7 Readiness Evidence

> ##### June 7 Readiness Evidence
>
> | Evidence | Current interpretation |
> |---|---|
> | `core/security/telemetry_policy/` | Provides metadata-only `TelemetryConfig`, `TelemetryEventAttempt`, diagnostics, decision/result records, config resolution, and event guard behavior before payload construction. |
> | `tests/security/test_telemetry_policy.py` | Tests default-off config handling, opt-in/allowlist gates, rejected unknown events/fields, rejected private/protected/secret/path/hash/report/professional-claim field classes, and no payload or network initialization flags. |
> | `docs/security/telemetry_policy.md` | Documents the policy and helper non-authority boundary while preserving open `TBD` decisions. |
> | `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` | Presents and locally exports the default-off policy/guard boundary with `payload_constructed=false` and network/runtime initialization false. PDU-042 adds an ephemeral distinct affirmative review request that remains disabled pending consent/allowlist; consent, config, persistence, and allowlist approval remain `TBD`. |
>
> PDU-042 evidence distinguishes the panel request from terms/install/open/solve actions and records that it mutates no product configuration, grants no opt-in/consent, constructs no payload, and initializes no network behavior.
> | `_run_records/TASK_RUN_2026-06-07_0141.md` and package fan-in | Record successful focused tests, `git diff --check`, and boundary scan evidence. |
>

### CLM-008 — References

> ##### References
>
> - `INIT.md` for project bootstrap and boundary rules.
> - `AGENTS.md` for bounded Type 2 dispatch rules.
> - `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, and OPS-K-AGENT invariants.
> - `docs/DIRECTIVE.md` for founding boundaries and out-of-scope hidden cloud telemetry.
> - `docs/IP_AND_DATA_BOUNDARY.md` for public/private data handling limits.
> - `docs/SPEC.md` for architecture, diagnostics, reports, and acceptance semantics.
> - `docs/security/telemetry_policy.md` for the implemented policy and metadata-only guard boundary.
> - `core/security/telemetry_policy/` for metadata-only helper evidence.
> - `tests/security/test_telemetry_policy.py` for focused default-off/helper tests.
> - `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` for the DEL-12-03 default-off policy-review surface attributed by DEC-074 O3.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for PKG-12, DEL-12-03, SOW-037, OBJ-010, OI-008, and AB-00-01/02/03/04/06/07/08.
> - `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_run_records/TASK_RUN_2026-06-07_0141.md`.
> - `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_run_records/WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md`.
> - `docs/_Registers/Deliverables.csv` row DEL-12-03.
> - `docs/_Registers/ScopeLedger.csv` row SOW-037.
> - `docs/_Registers/ContextBudgetQA.csv` row DEL-12-03.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-12-03 Telemetry off-by-default design

> #### Specification: DEL-12-03 Telemetry off-by-default design
>

### CLM-010 — Scope

> ##### Scope
>
> This deliverable specifies the privacy and configuration design for telemetry in OpenPipeStress. It covers policy, default configuration behavior, and tests needed to prove that telemetry is disabled by default and cannot transmit private engineering, code, rule-pack, component, material, report, secret, path, or protected standards data.
>
> Current evidence includes the repo policy in `docs/security/telemetry_policy.md`, a metadata-only guard helper in `core/security/telemetry_policy/`, focused tests in `tests/security/test_telemetry_policy.py`, and the default-off policy-review panel in `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`. Under DEC-074 O7-before-E5, the panel now routes its modeled desktop event attempts through `apps/desktop/src/services/telemetryPolicyService.ts`, a fail-closed pre-payload seam. The seam treats a capability request as insufficient without affirmative opt-in and allowlist evidence, constructs no payload, and exposes no endpoint, vendor, network, queue, upload, or persistence operation. Product configuration, consent surface, allowlist approval, and all transport remain unimplemented. Plugin/adapter/import/export/report/private-library interception remains a documented PDU-043 absence rather than a whole-runtime no-bypass claim.
>

### CLM-011 — Requirements

> ##### Requirements
>
> | ID | Requirement | Verification |
> |---|---|---|
> | TEL-REQ-001 | Telemetry shall be disabled by default. An absent, unset, empty, unknown, unsupported, malformed, or incomplete telemetry setting shall resolve to disabled. | Covered for metadata input by `tests/security/test_telemetry_policy.py`; product config schema/storage remains `TBD`. |
> | TEL-REQ-002 | Telemetry shall not initialize network transport, background upload jobs, queues, or persistence for telemetry unless the user has explicitly opted in. | Metadata helper exposes no network/payload initialization and tests assert no network, endpoint, vendor, upload queue/job, persistence, or external client flags; runtime startup remains `TBD`. |
> | TEL-REQ-003 | Telemetry, if later implemented, shall require explicit human/security approval of the event allowlist before any event is collected. | Helper requires allowlist approval metadata before allowing a metadata event; actual approval records and event allowlist remain `TBD`. |
> | TEL-REQ-004 | Telemetry shall not collect or transmit private project models, code-specific rule data, private rule packs, private material or component libraries, generated reports, model hashes, local file paths, user secrets, credentials, or protected standards content. | Helper rejects forbidden field classes before payload construction; protected-content/privacy scan evidence is recorded in the June 7 package fan-in. |
> | TEL-REQ-005 | Opt-in state shall be represented as a local configuration value whose default is false. The exact storage location and schema are TBD until the implementation deliverable chooses the product config surface. | Metadata resolution tests cover fail-closed behavior; product config-default fixture remains `TBD` until schema exists. |
> | TEL-REQ-006 | Turning telemetry on shall require an affirmative user action distinct from accepting general terms, installing the software, opening a project, or running a solve. | PDU-042 adds a distinct `Request telemetry enablement review` action to the existing panel. The interaction only records a local request and remains fail-closed; it does not turn telemetry on, grant consent, approve an allowlist, mutate product config, or initialize payload/network behavior. Focused App interaction evidence covers this bounded behavior. |
> | TEL-REQ-007 | Telemetry settings and diagnostics shall preserve the distinction between mechanics solved, user-rule checked, and human-approved states; telemetry shall not create professional approval or compliance claims. | Report/diagnostic text scan; status vocabulary test. |
> | TEL-REQ-008 | Telemetry-related diagnostics, if emitted, shall use the project diagnostic envelope when available and shall not include private payload content in messages. | Diagnostic schema test; privacy snapshot test. |
> | TEL-REQ-009 | Plugins, adapters, import/export paths, reports, and private-library mechanisms shall not bypass telemetry opt-in or privacy filters. | PDU-043 documented absence: the current guard covers telemetry-panel event attempts only; consumer interception remains unimplemented until separately authorized. |
> | TEL-REQ-010 | If telemetry remains unimplemented in MVP, product behavior shall remain a no-op with tests proving no outbound telemetry behavior is reachable by default. | Focused desktop service/App tests prove the selected seam remains disabled with no payload/network/persistence initialization; this is not whole-product security validation. |
>

### CLM-012 — Standards

> ##### Standards
>
> | Source | Applicable constraint |
> |---|---|
> | `docs/CONTRACT.md` OPS-K-PRIV-1 | Private project, material, component, and rule-pack data must not be transmitted or committed publicly by default. |
> | `docs/CONTRACT.md` OPS-K-PRIV-2 | Telemetry is off by default and cannot include private engineering or code data. |
> | `docs/CONTRACT.md` OPS-K-IP-1/2/3 | Protected standards text, tables, examples, copied formulas, and proprietary data must not enter public artifacts or telemetry payloads. |
> | `docs/CONTRACT.md` OPS-K-AUTH-1/2 | Software and agents must not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. |
> | `docs/DIRECTIVE.md` Section 4.2 | Hidden cloud storage or telemetry of private project/rule data is out of scope. |
> | `docs/IP_AND_DATA_BOUNDARY.md` Sections 3 and 6 | Private rule packs, owner standards, company design bases, component catalogs, and project data remain user controlled. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-037 and OI-008 | Telemetry, if implemented, is opt-in and privacy preserving; MVP default is no telemetry; any telemetry design requires explicit human approval. |
>

### CLM-013 — Verification

> ##### Verification
>
> Current metadata-guard evidence plus minimum test coverage for any later implementation:
>
> | Test ID | Test intent | Expected result |
> |---|---|---|
> | TEL-TEST-001 | Resolve absent, empty, unknown, unsupported, malformed, or incomplete telemetry metadata. | Current helper tests show telemetry resolves disabled and initializes no transport, endpoint, vendor, upload queue/job, persistence, or external client. |
> | TEL-TEST-002 | Load malformed product config once product config schema exists. | Telemetry remains disabled and a validation finding is emitted. This is still `TBD` for product config storage/schema. |
> | TEL-TEST-003 | Attempt to emit telemetry before opt-in or without approved allowlist evidence. | Current helper tests show event attempts are blocked locally before payload construction. Runtime network-denial/startup tests remain future scope. |
> | TEL-TEST-004 | Attempt to include private model, rule-pack, material, component, report, path, hash, secret, protected, or professional-claim fields. | Current helper tests reject the field before payload construction. |
> | TEL-TEST-005 | Enable opt-in using an explicit approved config path. | Only approved allowlist fields are eligible for metadata-only allowance; actual consent surface and approval record remain `TBD`. |
> | TEL-TEST-006 | Exercise plugin, adapter, report, and private-library routes. | No route bypasses telemetry opt-in or payload filtering. Runtime route coverage remains future scope until those surfaces call the helper. |
> | TEL-TEST-007 | Scan telemetry messages and docs. | No certification, approval, seal, security-certification, or automatic code-compliance claims appear. |
>

### CLM-014 — Current Evidence

> ##### Current Evidence
>
> | Evidence | Result |
> |---|---|
> | Worker B run record | `_run_records/TASK_RUN_2026-06-07_0141.md` records successful metadata-only helper implementation, 15 focused telemetry tests passing, and `git diff --check` passing for the worker write set. |
> | Package fan-in | `WORKING_ITEMS_RUN_2026-06-07_0150_TP-PKG12-LOCAL-PRIVACY-GUARDS-FANIN.md` records 44 passing PKG-12 privacy-guard tests and no runtime telemetry endpoint, vendor, transport, upload queue/job, persistence, schema, or approval artifact introduced. |
> | Policy document | `docs/security/telemetry_policy.md` documents default-off semantics, forbidden payload classes, the metadata-only helper boundary, and open decisions. |
> | Desktop policy-review panel | `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx` presents and locally exports default-off boundary state for review. It is DEL-12-03 implementation evidence under DEC-074 O3, but constructs no telemetry payload, initializes no network/runtime behavior, and keeps config/consent/allowlist decisions `TBD`. |
>

### CLM-015 — Documentation

> ##### Documentation
>
> The deliverable-local artifact set remains:
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
>
> Current external evidence artifacts are `docs/security/telemetry_policy.md`, `core/security/telemetry_policy/`, `tests/security/test_telemetry_policy.py`, and `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`. Future runtime telemetry, product config schema/storage, consent UI/CLI, endpoint/vendor/transport, retention, event allowlist/schema, and support-bundle implementation artifacts remain `TBD` and shall remain outside this setup folder unless dispatched by a separate sealed brief.
>

### CLM-016 — D-41 R5 T5 PDU-042 affirmative-request boundary

> ##### D-41 R5 T5 PDU-042 affirmative-request boundary
>
> The existing panel now demonstrates one explicit action whose identity is distinct from terms acceptance, installation, application/project open, and solve. Before the action, no request is recorded. After the action, the panel records `request_telemetry_enablement_review` and immediately reports `request_recorded_fail_closed_pending_consent_and_allowlist`. The action is not an opt-in or consent surface and cannot enable telemetry.

- **AC-001** — The contract preserves absent/malformed/default-disabled behavior, forbidden private/protected/secret/path/report fields, no implicit network/persistence/payload construction, the bounded desktop request and telemetry-panel guard seam, explicit absence of whole-runtime consumer interception, remaining consent/config/allowlist/transport TBDs, and the prohibition on professional, certification, approval, or code-compliance claims.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-12-03 Telemetry off-by-default design

> #### Procedure: DEL-12-03 Telemetry off-by-default design
>

### CLM-018 — Purpose

> ##### Purpose
>
> Use this procedure to produce or review a telemetry design for OpenPipeStress without breaching the local-first privacy boundary. The current evidence set includes design documents, `docs/security/telemetry_policy.md`, a metadata-only guard helper in `core/security/telemetry_policy/`, focused tests in `tests/security/test_telemetry_policy.py`, and the DEL-12-03 desktop policy-review panel in `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`. The panel constructs only a local review artifact and does not implement runtime telemetry transport, telemetry payload construction, endpoint, vendor, upload queue/job, telemetry persistence, product config schema/storage, consent UI/CLI, retention, support-bundle workflow, allowlist approval, or approval records.
>

### CLM-019 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status |
> |---|---|
> | Sealed deliverable context for DEL-12-03 | Present in `_CONTEXT.md`. |
> | Scope item SOW-037 and objective OBJ-010 | Present in decomposition/registers. |
> | Applicable invariants OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, OPS-K-AGENT | Present in `docs/CONTRACT.md`. |
> | Telemetry policy documentation | Present in `docs/security/telemetry_policy.md`. |
> | Metadata-only guard helper and focused tests | Present in `core/security/telemetry_policy/` and `tests/security/test_telemetry_policy.py`. |
> | Default-off desktop policy-review panel | Present in `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`; attributed to DEL-12-03 by DEC-074 O3 and non-authorizing by construction. |
> | Explicit human/security approval for runtime telemetry collection | TBD; absent for runtime telemetry implementation. |
> | Product configuration surface and schema | TBD; not selected by this deliverable. |
>

### CLM-020 — Steps

> ##### Steps
>
> 1. Confirm the current brief is scoped only to DEL-12-03 and the assigned folder.
> 2. Read `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `_STATUS.md`, `INIT.md`, `AGENTS.md`, `docs/CONTRACT.md`, the DEL-12-03 register rows, and the applicable PKG-12/architecture-basis decomposition rows.
> 3. Treat telemetry as disabled unless explicit approved opt-in configuration exists.
> 4. If no human/security approval exists for telemetry collection, keep event names, endpoint details, vendor selections, payload fields, and transport behavior as `TBD` or no-op.
> 5. Verify configuration behavior so absent, unset, empty, unknown, unsupported, malformed, or incomplete telemetry metadata resolves to disabled.
> 6. Verify payload rules so private project, code, rule-pack, material, component, report, path, hash, secret, protected standards content, and professional/code-compliance claim fields cannot be collected or transmitted.
> 7. Verify Python and desktop tests that prove metadata-only default-off behavior, no payload/network/persistence initialization, fail-closed capability requests without affirmative opt-in, and payload/runtime/forbidden-field rejection. Runtime startup and plugin/adapter/import-export/report/private-library interception remain PDU-043 documented absence until separately authorized.
> 8. For PDU-042, render the existing panel and verify `requested=false` before interaction. Click only `Request telemetry enablement review`; verify the action is distinct from terms/install/open/solve, records the request, disables itself, and leaves telemetry, opt-in, consent, allowlist, config mutation, payload construction, persistence, and network initialization false.
> 8. Record any unresolved approval, config, endpoint, or payload decision as `TBD` for human/security ruling.
> 9. Do not write product code or repo-level product artifacts unless a later sealed brief explicitly authorizes that scope.
>

### CLM-021 — Verification

> ##### Verification
>
> | Check | Pass condition |
> |---|---|
> | Four-document kit exists | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` are present. |
> | Telemetry policy artifact exists | `docs/security/telemetry_policy.md` is present and documents default-off policy plus metadata-only helper boundaries. |
> | Metadata-only helper exists | `core/security/telemetry_policy/` is present and evaluates config/event metadata before payload construction. |
> | Default sections preserved | Each document retains its required default section headings. |
> | Privacy default | Specification states disabled-by-default and fail-closed config behavior. |
> | No private data transmission | Specification forbids private project/code/rule/material/component/report/path/hash/secret/protected content in telemetry payloads. |
> | No cloud assumption | Documents do not define cloud operation, endpoint, vendor, or upload behavior without human approval. |
> | Current tests | `tests/security/test_telemetry_policy.py` covers metadata-only default-off, no-payload/no-network flags, allowlist gating, and forbidden-field rejection. |
> | Desktop policy-review boundary | Panel evidence records `payload_constructed=false`, network/runtime initialization false, and consent/config/allowlist decisions as `TBD`; it is not treated as consent UI or runtime telemetry. |
> | Future test expectations | Specification and Procedure keep runtime no-network-before-opt-in, product config, and no-bypass tests explicit for later integration surfaces. |
> | Lifecycle | `_STATUS.md` remains `IN_PROGRESS`; this tranche changes only its attribution history while preserving all `## Remaining` content, including the D-41 bootstrap. It is not lifecycle promotion or acceptance. |
>

### CLM-022 — Records

> ##### Records
>
> Deliverable-local records:
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
>
> Current external evidence records include `docs/security/telemetry_policy.md`, `core/security/telemetry_policy/`, `tests/security/test_telemetry_policy.py`, `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`, `_run_records/TASK_RUN_2026-06-07_0141.md`, and the PKG-12 package fan-in run record. Future implementation records, if authorized, should include human approval evidence, config schema/default fixtures, payload allowlist, transport-disabled runtime tests, opt-in tests, payload privacy tests, and plugin/adapter bypass tests.
>
> The PDU-042 request interaction is not one of those future consent/opt-in implementations. Treat it as bounded GUI evidence that an affirmative request can be distinct while the O7 guard remains authoritative and fail-closed.

- **VER-001** — Validate the contract and review source parity, default-off and fail-closed cases, forbidden-field and no-network/no-payload boundaries, distinct affirmative-request behavior, bounded seam versus documented interception absences, retained TBDs, and professional-authority limits.

## Governing Values and Decisions — Axiology

### CLM-023 — Guidance: DEL-12-03 Telemetry off-by-default design

> #### Guidance: DEL-12-03 Telemetry off-by-default design
>

### CLM-024 — Purpose

> ##### Purpose
>
> This guidance explains how to interpret the telemetry boundary for OpenPipeStress. The product is local-first, public-repository content must avoid protected standards data, and private project/rule/component/material data stays user controlled. Telemetry is therefore not a default feature; it is either absent/no-op or explicitly approved, opt-in, and payload-limited.
>
> DEL-12-03 has repo policy documentation, a Python metadata-only guard, focused tests, and a desktop fail-closed event-attempt seam used by the default-off policy-review panel. Under DEC-074 O7-before-E5, this seam is the selected private-by-default enforcement grain: it can only drop or reject attempts before payload construction and cannot initialize network or persistence. It is not consent UI, transport, or whole-runtime interception; PDU-043 remains absent outside the telemetry panel.
>

### CLM-025 — Principles

> ##### Principles
>
> | Principle | Guidance |
> |---|---|
> | Default-off | Design the product so no telemetry path is active until an explicit opt-in is recorded. |
> | No hidden cloud behavior | Do not introduce endpoints, upload queues, background jobs, or remote storage as implicit behavior. |
> | Private data never leaves by telemetry | Treat project models, rule packs, code-specific values, reports, hashes, paths, secrets, private libraries, and protected standards content as forbidden telemetry content. |
> | Allowlist over redaction-only | If telemetry is later approved, use a small approved event allowlist. Redaction can be a second guard, not the primary permission model. |
> | Local evidence | Keep approval records, config state, and test evidence local and reviewable. |
> | Human authority | Telemetry status must never imply professional engineering review, certification, sealing, or code compliance. |
>

### CLM-026 — Considerations

> ##### Considerations
>
> - A no-op runtime telemetry path is acceptable for MVP if tests prove that no default outbound behavior exists.
> - The current `core/security/telemetry_policy/` helper is an acceptable metadata-only guardrail for default-off behavior. It should stay pre-payload and local unless a later sealed brief authorizes runtime integration.
> - The current `TelemetryBoundaryPanel` is a policy-review surface, not a consent control. Its display/export of disabled-state evidence does not enable telemetry, approve an allowlist, or authorize collection or transport.
> - PDU-042's `Request telemetry enablement review` button is a distinct affirmative request demonstration inside that existing surface. It records request intent only, once per mounted panel, and then disables itself while telemetry remains off. Do not label it consent, opt-in, enablement, or configuration persistence.
> - A future opt-in design needs a human-approved allowlist before implementation. Without that approval, event names, endpoint details, vendor choices, and payload fields remain `TBD`.
> - Configuration defaults should fail closed: missing, unset, unknown, or malformed values disable telemetry.
> - Product diagnostics may report that telemetry is disabled or misconfigured, but diagnostic text should not echo private payload content.
> - Plugins and adapters must not create their own telemetry side channel or bypass core privacy filters.
>

### CLM-027 — Trade-offs

> ##### Trade-offs
>
> | Option | Benefit | Risk | Current disposition |
> |---|---|---|---|
> | No telemetry in MVP | Small privacy surface and simple verification. | Less automatic usage insight. | Preferred by SOW-037 notes and OI-008 until human approval changes it. |
> | Opt-in telemetry later | Can support voluntary product-improvement metrics. | Requires strong payload governance and user trust. | Allowed only after explicit human/security approval. |
> | Redaction-only telemetry | Easier to retrofit around broad events. | Higher leakage risk if payloads are assembled before filtering. | Not sufficient as the primary design. |
> | Local diagnostics instead of telemetry | Preserves reviewability without transmission. | Users must share diagnostics intentionally for support. | Compatible with local-first policy. |
>

### CLM-028 — Examples

> ##### Examples
>
> Concrete product config syntax and runtime event schemas are `TBD` because the implementation surface is not selected in this deliverable. The permitted current example behavior is:
>
> - absent telemetry config means disabled;
> - empty, unknown, unsupported, malformed, or incomplete telemetry metadata means disabled;
> - user has not opted in means disabled;
> - MVP may contain no telemetry implementation;
> - allowlisted low-sensitivity metadata can be evaluated by the helper only after explicit opt-in, approved consent surface metadata, and human-approved allowlist evidence are present;
> - unknown events, unknown fields, private/protected/secret/path/hash/report/professional-claim field classes, and payload-shaped attempts are rejected before payload construction;
> - any future telemetry payload is approved by allowlist and excludes private/protected engineering data.
>
> Focused interaction evidence must prove both sides of the boundary: ordinary render/open state records no request, while clicking the dedicated request action records only the fail-closed request and leaves opt-in, consent, allowlist, configuration mutation, payload, persistence, and network behavior false.
>

### CLM-029 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in this evidence alignment. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-037 OBJ-010 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
