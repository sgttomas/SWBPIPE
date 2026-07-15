---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-04
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-074, SOW-075]
package_objective_refs: [OBJ-009, OBJ-017, OBJ-018]
---

# Scope of Work — DEL-17-04

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-04` in service of project scope [SOW-030, SOW-074, SOW-075] and package objectives [OBJ-009, OBJ-017, OBJ-018].

- **OUT-001** — A versioned CAEPIPE MBF export-profile and deterministic writer contract with explicit supported, omitted, approximated, delegated, sidecar-mapped, unsupported, and TBD behavior is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Datasheet: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-04 |
> | Package | PKG-17 Export Format Interoperability |
> | Type | BACKEND_FEATURE_SLICE |
> | Source foundation | DEL-17-01 and DEL-17-02 |
> | Lifecycle role | First CAEPIPE-specific export profile/writer foundation deliverable |
>

### CLM-003 — Purpose

> ##### Purpose
>
> DEL-17-04 defines the document-level contract/design and first bounded implementation foundation for a CAEPIPE MBF export profile and deterministic writer. It is grounded in admitted public/official CAEPIPE MBF references, the DEL-17-01 source basis, and the DEL-17-02 export package/profile/loss-report contract.
>
> The implementation foundation emits a deliberately narrow invented smoke subset, a JSON package contract, a deterministic MBF text member, sidecar stable-ID mapping, diagnostics, and loss reports. It does not implement a parser, external execution harness, public API, GUI action, release claim, CAEPIPE compatibility claim, code-compliance claim, or professional-acceptance claim.
>

### CLM-004 — Source and Evidence Slots

> ##### Source and Evidence Slots
>
> | Source slot | Required treatment |
> |---|---|
> | DEL-17-01 source basis | Carry CAEPIPE source findings, TBD register entries, and question-dossier gates into the target profile. |
> | DEL-17-02 export contract | Carry export profile, stable-ID map, manifest, and loss-report vocabulary into the writer contract. |
> | CAEPIPE public MBF import reference | Treat as evidence for MBF text input, keyword-ordered sections, and command-line behavior only; profile coverage remains source-gated. |
> | CAEPIPE public MBF export reference | Treat as evidence that CAEPIPE exports MBF model data and version-flavoured MBF output; OpenPipeStress support claims remain `TBD`. |
> | Implementation evidence | `core/handoff/caepipe_mbf/`, `schemas/caepipe_mbf_export.schema.json`, `fixtures/caepipe_mbf/invented/`, and `tests/test_caepipe_mbf_export_package.py` provide bounded foundation evidence. |
>

### CLM-005 — Profile Concepts

> ##### Profile Concepts
>
> | Concept | Required treatment |
> |---|---|
> | Target family | CAEPIPE MBF profile. |
> | Target version basis | `TBD` until source-confirmed. |
> | Record-family subset | `TBD` until downstream source review and/or vendor clarification closes it. |
> | Profile-basis guardrail | The implementation foundation blocks blank or unsupported target-version and record-subset basis values; until source-confirmed closure exists, the profile must carry `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`. |
> | Stable ID strategy | Direct MBF carrying remains `TBD`; the implementation foundation uses manifest-referenced sidecar mapping as the conservative default. |
> | Source basis trace | Each CAEPIPE-specific requirement must map to DEL-17-01, DEL-17-02, a public MBF source, or an explicit `TBD`. |
> | Source basis guardrail | The implementation foundation blocks caller-supplied profile source-basis refs that omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, and blocks `DEL-17-03` as CAEPIPE target/source authority. |
> | Loss reporting | Required for exported, omitted, approximated, delegated, unsupported, and TBD behavior. |
> | Unsupported-entity severity | Missing or malformed unsupported-entity evidence blocks the package; explicit unsupported behavior is warning-level by default unless separately marked blocking; `info` severity is not accepted for unsupported behavior. |
> | Pass-through options | May be target configuration only; not local code-checking logic. |
>

### CLM-006 — Boundary Summary

> ##### Boundary Summary
>
> - No CAEPIPE executable, commercial example, proprietary model, or copied vendor fixture is added.
> - No reverse-engineered behavior or license-bypass workflow is introduced.
> - No protected standards values, material allowables, SIF/flexibility values, or owner criteria are introduced.
> - MBF export work remains a source-bounded foundation until a later tranche closes target version/profile and record-family coverage.
> - Guardrail diagnostics preserve source uncertainty; they do not close CAEPIPE version/profile, record-family coverage, direct stable-ID carrying, compatibility, or professional reliance.
> - DEL-17-03 may remain historical implementation-pattern evidence, but it is not CAEPIPE MBF source-basis authority.

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Specification: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-008 — Normative Scope

> ##### Normative Scope
>
> DEL-17-04 shall define a source-bounded CAEPIPE MBF export profile and deterministic writer contract/design, and may include a first bounded implementation foundation. It shall consume DEL-17-01 source authority and DEL-17-02 package/profile/stable-ID/loss-report requirements.
>
> This deliverable's implementation foundation shall remain limited to deterministic project-owned package building, invented fixtures, schema validation, sidecar stable-ID mapping, diagnostics, and loss reports. It shall not implement a CAEPIPE parser, external run harness, public API, GUI behavior, release claims, CAEPIPE compatibility claims, code-compliance claims, professional-acceptance claims, or full MBF coverage claims.
>

### CLM-009 — Requirements

> ##### Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-04-REQ-001 | The CAEPIPE MBF profile shall declare target version basis as `TBD` until source-confirmed. |
> | DEL-17-04-REQ-002 | The first MBF record-family subset shall remain `TBD` until source evidence or human-approved clarification closes it. |
> | DEL-17-04-REQ-003 | The writer contract shall not silently approximate unsupported physical or analytical model entities. |
> | DEL-17-04-REQ-004 | The writer contract shall require a loss report for omitted, approximated, delegated, unsupported, and TBD behavior. |
> | DEL-17-04-REQ-005 | Stable canonical ID carrying inside MBF shall remain `TBD`; sidecar mapping shall be required when direct carrying is not source-confirmed. |
> | DEL-17-04-REQ-006 | Pass-through target options shall remain target configuration metadata and shall not become OpenPipeStress local code-checking logic. |
> | DEL-17-04-REQ-007 | Later implementation shall use invented fixtures only. |
> | DEL-17-04-REQ-008 | The first implementation foundation shall use sidecar stable-ID mapping while `TBD-17-01-003` remains open. |
> | DEL-17-04-REQ-009 | The first implementation foundation shall expose blocking diagnostics when the invented smoke subset, sidecar stable-ID map, or loss report is missing. |
> | DEL-17-04-REQ-010 | The first implementation foundation shall expose blocking diagnostics when target-version basis, record-subset basis, or carried profile TBD references are missing, blank, or weakened without admitted source-confirmed closure. |
> | DEL-17-04-REQ-011 | The first implementation foundation shall expose blocking diagnostics when profile source-basis refs omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, or when they include DEL-17-03 as CAEPIPE target/source authority. |
>

### CLM-010 — Verification Requirements

> ##### Verification Requirements
>
> | Check | Requirement |
> |---|---|
> | Source trace | CAEPIPE-specific statements cite DEL-17-01 source IDs or remain `TBD`. |
> | Boundary trace | No proprietary examples or protected data are introduced. |
> | Loss trace | Unsupported and TBD behavior is visible in the loss-report contract. |
> | Profile-basis trace | Target-version, record-subset, and direct stable-ID uncertainty remain visible through carried TBD refs until source-confirmed closure exists. |
> | Source-basis guardrail trace | Profile and manifest source-basis refs carry the required authority set and do not promote implementation-pattern evidence to CAEPIPE source authority. |
> | Dependency trace | `Dependencies.csv` validates as v3.1. |
>

### CLM-011 — Acceptance Criteria

> ##### Acceptance Criteria
>
> | Criterion | Acceptance treatment |
> |---|---|
> | Source basis mapping | Each CAEPIPE-specific requirement is traced to DEL-17-01, DEL-17-02, an admitted public MBF reference, or an explicit `TBD` closure path. |
> | Target version closure | REQ-001 remains unresolved until the first CAEPIPE version/profile is source-confirmed or human-approved as a bounded profile decision. |
> | Record subset closure | REQ-002 remains unresolved until MBF record families and required fields are enumerated from admitted evidence. |
> | Stable ID preservation | REQ-005 is accepted only when direct MBF carrying is source-confirmed or the writer uses manifest-referenced sidecar mapping. |
> | Loss category coverage | REQ-004 is accepted only when exported, omitted, approximated, delegated, unsupported, and `TBD` behavior are all visible in the loss report. |
> | Diagnostic classification | `TBD-17-04-004` is addressed for this foundation: malformed unsupported refs, missing unsupported loss coverage, and `info`-severity unsupported losses block; explicit `warning` or `blocking` unsupported losses are accepted as classification evidence. |
> | Profile-basis guardrail | REQ-010 is accepted when diagnostics block blank or unsupported `target_version_basis`, `record_subset_basis`, or carried-TBD evidence while the foundation remains source-bounded. |
> | Source-basis guardrail | REQ-011 is accepted when diagnostics and schema validation block weakened CAEPIPE source-basis refs while preserving DEL-17-03 only as historical implementation-pattern evidence. |
> | Fixture provenance | REQ-007 is accepted only with invented fixtures and no vendor, proprietary, protected, or owner/project examples. |
> | Foundation implementation | REQ-008 and REQ-009 are accepted when schema, builder, fixture, and tests demonstrate sidecar-first IDs, deterministic MBF text, and blocking diagnostics for incomplete packages. |
>

### CLM-012 — Downstream Use

> ##### Downstream Use
>
> DEL-17-05 may consume this deliverable later for CAEPIPE external run harness and CSV parser work. DEL-17-05 remains blocked until DEL-17-04 has committed evidence.

- **AC-001** — The contract preserves DEL-17-01 source authority and DEL-17-02 common package/profile/ID-map/loss contracts, a selected-profile gate, deterministic text generation, stable canonical-ID mapping, units and coordinates, pass-through target options without local code logic, invented/rights-cleared fixtures, diagnostics, loss reporting, and unresolved MBF record, field, version, line-ending, encoding, and external-validation questions without claiming broad compatibility or acceptance.

## Production and Verification Method — Praxeology

### CLM-013 — Procedure: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Procedure: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-014 — Population Procedure

> ##### Population Procedure
>
> 1. Read DEL-17-01 source-basis dossier and CAEPIPE question dossier.
> 2. Read DEL-17-02 export package/profile/stable-ID/loss-report contract.
> 3. Draft the CAEPIPE MBF profile/writer four-document kit at contract/design level only.
> 4. Generate `_SEMANTIC.md` with `semantic-matrix-build`.
> 5. Generate `_SEMANTIC_LENSING.md` with `lens-register`.
> 6. Apply warranted P3 enrichment through `four-documents P3_ONLY`.
> 7. Extract dependencies into `Dependencies.csv`.
> 8. Record memory and run evidence.
>

### CLM-015 — Foundation Implementation Procedure

> ##### Foundation Implementation Procedure
>
> Bounded foundation implementation work must:
>
> 1. Close or explicitly carry target version and MBF record-family TBDs.
> 2. Define the deterministic writer subset from admitted evidence.
> 3. Emit a loss report for exported, unsupported, omitted, approximated, delegated, and TBD behavior.
> 4. Preserve stable IDs through manifest-referenced sidecars unless direct MBF carrying is source-confirmed.
> 5. Block blank or unsupported profile-basis values until target version, record subset, and direct stable-ID carrying are closed by admitted source evidence.
> 6. Block missing or unsafe profile source-basis refs so DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, and `CAEPIPE-EXPORT-MBF` remain visible, and DEL-17-03 is not treated as CAEPIPE target/source authority.
> 7. Use invented fixtures only.
> 8. Avoid CAEPIPE compatibility, release, code-compliance, and professional-acceptance claims.
> 9. Keep external execution, CSV parsing, target result interpretation, public API, and GUI integration out of scope unless separately authorized.
>

### CLM-016 — Implementation Readiness Checks

> ##### Implementation Readiness Checks
>
> Before a later writer tranche begins, re-read DEL-17-01, DEL-17-02, the admitted public MBF references, and this four-document kit. The tranche must record:
>
> 1. Target version/profile closure or explicit carried `TBD`.
> 2. MBF record-family and required-field subset closure or explicit carried `TBD`.
> 3. Stable-ID direct-carry evidence or manifest-referenced sidecar policy.
> 4. Loss-report coverage for exported, omitted, approximated, delegated, unsupported, and `TBD` behavior.
> 5. Blocking versus non-blocking diagnostic classification: unsupported behavior defaults to warning-level evidence unless explicitly blocking, while missing coverage, malformed refs, or `info` severity block.
> 6. Profile-basis diagnostics proving target-version, record-subset, and carried-TBD evidence cannot be silently omitted or weakened.
> 7. Source-basis diagnostics proving required CAEPIPE source authority cannot be silently omitted or replaced by implementation-pattern evidence.
> 8. Invented-fixture provenance.
> 9. Validation results for the deliverable-local document kit, semantic artifacts, dependency schema, and diff hygiene.
>

### CLM-017 — Semantic Enrichment Verification

> ##### Semantic Enrichment Verification
>
> Pass 3 checked `_SEMANTIC_LENSING.md` against DEL-17-01 CAEPIPE TBDs, DEL-17-02 loss-report/stable-ID requirements, and the public MBF source boundary. Applied TP-EXPORT-004R items added source-basis slots, acceptance criteria, classification guidance, pass-through-option rationale, and implementation readiness checks. No P3 item authorized implementation in this tranche.
>
> | Register Item | Disposition |
> |---|---|
> | A-001 | Incorporated through implementation readiness checks for source-confirmed MBF target basis before writer enablement. |
> | A-002 | Incorporated through the future implementation validation checklist for writer determinism, loss reports, and boundary claims. |
> | B-001 | Incorporated as a carried `TBD` for first CAEPIPE target version/profile and public citation target. |
> | B-002 | Incorporated through source and evidence slots for DEL-17-01, DEL-17-02, and public MBF references. |
> | C-001 | Incorporated through acceptance criteria requiring CAEPIPE-specific requirements to map to admitted evidence or explicit `TBD` closure. |
> | C-002 | Incorporated as diagnostic classification guidance and an open question for unsupported-entity severity. |
> | F-001 | Incorporated through profile acceptance criteria for target version, record subset, stable-ID policy, loss categories, and invented fixtures. |
> | F-002 | Incorporated through future implementation loss-report coverage checks. |
> | D-001 | Incorporated through fixture provenance acceptance criteria. |
> | D-002 | Incorporated as an open diagnostic policy question for blocker versus warning behavior. |
> | X-001 | Incorporated through implementation readiness checks with required rereads and validation evidence. |
> | E-001 | Incorporated through pass-through-option rationale in guidance. |

- **VER-001** — Validate the contract and review source parity, source/profile gates, deterministic writer and byte-output requirements, complete behavior classification and loss reporting, stable-ID carrier/sidecar treatment, unit/coordinate and encoding boundaries, fixture provenance, every retained MBF TBD, and absence of bundled/proprietary material, local code-checking logic, compatibility overclaim, or professional authority.

## Governing Values and Decisions — Axiology

### CLM-018 — Guidance: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

> #### Guidance: DEL-17-04 CAEPIPE MBF export profile and deterministic writer
>

### CLM-019 — Design Guidance

> ##### Design Guidance
>
> Treat CAEPIPE MBF as a narrow, source-confirmed target profile. The purpose is not to claim compatibility; it is to define and test what a bounded deterministic writer foundation may safely emit, what it must omit, and what it must report.
>
> The safe posture is conservative: if a record family, required field, stable ID carrier, option, or target behavior is not confirmed by admitted source evidence, mark it `TBD` and require a loss-report or profile diagnostic. The first implementation foundation therefore uses sidecar stable-ID mapping and carries direct MBF ID carrying as `TBD-17-01-003`.
>
> Profile-basis fields are guardrails, not target-support claims. Until later source-confirmed closure exists, blank values or informal target-version/record-subset labels should block the package because they would make unresolved CAEPIPE scope look settled.
>
> Source-basis refs are also guardrails. A caller-provided CAEPIPE MBF profile should not be able to omit DEL-17-01, DEL-17-02, `CAEPIPE-IMPORT-MBF`, or `CAEPIPE-EXPORT-MBF`, and should not promote DEL-17-03 implementation-pattern evidence into CAEPIPE target/source authority.
>

### CLM-020 — Interpretation Guidance

> ##### Interpretation Guidance
>
> - Use DEL-17-01 for CAEPIPE source facts and unanswered questions.
> - Use DEL-17-02 for manifest, profile, ID-map, and loss-report structure.
> - Do not treat MBF text output as equivalent to professional acceptance.
> - Do not turn pass-through target options into local code-checking logic.
> - Do not infer undocumented MBF behavior from the target name or common practice.
> - Treat generated MBF text as a deterministic invented smoke artifact, not as target acceptance evidence.
>

### CLM-021 — Classification Guidance

> ##### Classification Guidance
>
> Unsupported target behavior must be classified before implementation as blocking, diagnostic-only, delegated, omitted, approximated, or `TBD`. Until that classification exists, the writer contract should preserve the uncertainty rather than enabling a broad MBF support claim.
>
> For this foundation, explicit unsupported behavior defaults to warning-level loss-report evidence unless the loss entry marks it blocking. Missing unsupported loss coverage, malformed unsupported references, or `info` severity for unsupported behavior are blocking because they would hide material target-scope limits.
>
> Pass-through CAEPIPE options are metadata for the target profile because they may name target-side analysis settings. They must not become OpenPipeStress local code-checking logic, local professional acceptance logic, or a substitute for user-supplied rule packs and human review.
>

### CLM-022 — Open Questions

> ##### Open Questions
>
> | TBD | Question | Later closure path |
> |---|---|---|
> | TBD-17-04-001 | Which CAEPIPE version/profile is the first target? | Public documentation review or CAEPIPE developer/support clarification. |
> | TBD-17-04-002 | Which MBF record families and required fields are in the first subset? | Source review and bounded implementation design. |
> | TBD-17-04-003 | Can MBF carry stable canonical IDs directly? | Source review; otherwise sidecar-only mapping. |
> | TBD-17-04-004 | Which unsupported entities block export versus produce non-blocking diagnostics? | This foundation uses warning by default with explicit blocking allowed; later source-confirmed profiles may specialize entity-specific severity. |
>

### CLM-023 — Foundation Guidance

> ##### Foundation Guidance
>
> The first implementation foundation should remain sidecar-first and loss-report-first:
>
> - deterministic text output is acceptable only for the invented smoke subset;
> - every canonical identity must appear in a sidecar mapping or an explicit loss entry;
> - every unsupported entity must be a stable reference and must have an explicit unsupported loss entry;
> - target-version, record-subset, and direct stable-ID uncertainty must remain carried as `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`;
> - missing smoke-subset inputs, sidecar IDs, or loss reports should block the package;
> - missing, blank, or weakened profile-basis evidence should block the package;
> - missing or unsafe CAEPIPE source-basis refs should block the package;
> - external execution, CSV parsing, target result interpretation, and target code/check logic remain out of scope.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 SOW-075 OBJ-009 OBJ-017 OBJ-018 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
