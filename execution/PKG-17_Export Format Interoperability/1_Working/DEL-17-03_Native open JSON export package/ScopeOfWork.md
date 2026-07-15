---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-03
package_id: PKG-17
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@e8f59a63372f38d9e788ac39b39995558f5aba73
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017]
---

# Scope of Work — DEL-17-03

## Purpose and Objective Traceability

This Scope of Work defines `DEL-17-03` in service of project scope [SOW-030, SOW-074] and package objectives [OBJ-009, OBJ-017].

- **OUT-001** — A deterministic native open-JSON export-package contract that materializes the common interoperability contracts without target-specific or commercial dependencies is produced.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-17-03 Native open JSON export package

> #### Datasheet: DEL-17-03 Native open JSON export package
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-17-03 |
> | Package | PKG-17 Export Format Interoperability |
> | Type | BACKEND_FEATURE_SLICE |
> | Source foundation | DEL-17-01 and DEL-17-02 |
> | Lifecycle role | First project-owned native JSON package implementation foundation for export interoperability |
>

### CLM-003 — Purpose

> ##### Purpose
>
> DEL-17-03 defines the native open JSON export package as the project-owned debug, interchange, archival, adapter-input, manifest, ID-map, and validation-report package target.
>
> This deliverable owns the bounded native JSON package foundation: schema contract, deterministic package builder/writer helper, invented public fixture, and focused tests. It does not implement public API, GUI action, persistence runtime, project-store export flow, downstream target adapter behavior, target compatibility, release claims, code-compliance claims, or professional-reliance claims.
>

### CLM-004 — Package Members

> ##### Package Members
>
> | Member | Role |
> |---|---|
> | `manifest` | Records source model identity, export profile, package member inventory, hashes, diagnostics, and boundary notes. |
> | `model_payload` | Carries the project-owned JSON representation selected by a future exporter. |
> | `stable_id_map` | Maps canonical OpenPipeStress IDs to package-local records. |
> | `loss_report` | Records exported, omitted, approximated, delegated, unsupported, and TBD behavior. |
> | `validation_report` | Records package-shape and source-boundary checks without claiming solver validation. |
> | `diagnostics` | Carries warnings and blocking messages from export preparation. |
>

### CLM-005 — Implementation Artifacts

> ##### Implementation Artifacts
>
> | Artifact | Role |
> |---|---|
> | `schemas/native_json_export.schema.json` | JSON Schema 2020-12 contract for the native JSON package foundation. |
> | `core/handoff/native_json/` | Deterministic key-sorted, compact JSON builder/writer and SHA-256 helper labelled `deterministic_sorted_compact_json_payload_hash` for hashes computed here; governed source checksum metadata is passed through without relabelling, and no RFC 8785 conformance is asserted. |
> | `fixtures/native_json/invented/native_json_export_package.json` | Invented public fixture for package-shape and boundary checks. |
> | `tests/test_native_json_export_package.py` | Focused tests for the exact computed-hash label, governed source-checksum pass-through, deterministic bytes/digests, mutation sensitivity, timestamp-free inputs, member inventory, diagnostics, privacy/professional-boundary flags, writer output, and protected/private payload screening. |
>

### CLM-006 — Profile Basis

> ##### Profile Basis
>
> The native JSON profile shall consume the DEL-17-02 export profile contract and declare:
>
> - profile ID and profile version;
> - source model identity and hash basis;
> - unit and coordinate policy;
> - canonical ID families included;
> - package member path policy;
> - deterministic timestamp policy;
> - loss-report category policy;
> - boundary notes for no release, compatibility, code-compliance, or professional claim.
>

### CLM-007 — Boundary Summary

> ##### Boundary Summary
>
> - Native JSON is project-owned interchange and adapter input, not a public compatibility promise.
> - The package may preserve source model semantics but does not certify solver correctness.
> - Protected standards values, private model data, proprietary target examples, and owner criteria remain excluded.
> - Any future integration or target-adapter work must keep unsupported or omitted behavior visible in the loss report.

## Completion and Reliance Basis — Epistemology

### CLM-008 — Specification: DEL-17-03 Native open JSON export package

> #### Specification: DEL-17-03 Native open JSON export package
>

### CLM-009 — Normative Scope

> ##### Normative Scope
>
> DEL-17-03 shall define and implement the bounded native open JSON export package foundation. It shall consume DEL-17-01 source boundaries and DEL-17-02 export package/profile/stable-ID/loss-report requirements.
>
> The bounded implementation scope includes the native JSON schema contract, deterministic package builder/writer helper, invented public fixture, and focused tests for sorted compact serialization, member hashes, manifest/member inventory, stable ID map, loss report, validation report, diagnostics, provenance, privacy, and professional-boundary flags.
>
> This deliverable shall not implement public APIs, GUI behavior, persistence runtime or project-store export flow, downstream target adapters, CAEPIPE/PCF/glTF target behavior, release claims, compatibility claims, code-compliance claims, solver-validation claims, professional-reliance claims, or professional-acceptance claims.
>

### CLM-010 — Requirements

> ##### Requirements
>
> | Req ID | Requirement |
> |---|---|
> | DEL-17-03-REQ-001 | The native JSON package shall include a manifest, model payload, stable ID map, loss report, validation report, and diagnostics concept. |
> | DEL-17-03-REQ-002 | The package contract shall preserve canonical OpenPipeStress identity directly in JSON records or through a package-local ID map. |
> | DEL-17-03-REQ-003 | The manifest shall identify source model reference, export profile ID, package members, deterministic hash basis, and boundary notes. |
> | DEL-17-03-REQ-004 | The loss report shall use the DEL-17-02 categories: exported, omitted, approximated, delegated, unsupported, and TBD. |
> | DEL-17-03-REQ-005 | Validation report content shall be package-shape and source-boundary evidence only; it shall not claim solver validation or professional acceptance. |
> | DEL-17-03-REQ-006 | The implementation shall emit deterministic, key-sorted, compact JSON and label SHA-256 hashes it computes `deterministic_sorted_compact_json_payload_hash`, without hiding runtime timestamps inside hash inputs. Governed caller-supplied source checksum mappings shall preserve their declared canonicalization and digest metadata or be rejected as malformed; the project-local computed-hash label does not assert RFC 8785 conformance. |
> | DEL-17-03-REQ-007 | The implementation shall include invented public fixture and focused tests for package shape, deterministic hashes, diagnostics, privacy/professional-boundary flags, and protected/private payload screening. |
> | DEL-17-03-REQ-008 | Target-specific or adapter-specific behavior shall remain outside the native JSON package foundation unless separately scoped. |
>

### CLM-011 — Verification Requirements

> ##### Verification Requirements
>
> | Check | Requirement |
> |---|---|
> | Four-document check | The four-document kit exists. |
> | Dependency schema | `Dependencies.csv` validates as v3.1. |
> | Semantic artifacts | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and are lens artifacts only. |
> | Schema and fixture checks | Native JSON schema and invented fixture parse as JSON. |
> | Builder and test checks | Native JSON builder compiles and focused tests pass. |
> | Hash-basis checks | Focused tests verify the exact label on DEL-17-03-computed hashes, lossless pass-through of a differently labelled governed source checksum, repeatable bytes/digests, digest mutation sensitivity, timestamp-free hash inputs, and absence of an RFC 8785 conformance claim. |
> | Boundary review | No protected/proprietary data or external compatibility claim is added. |
>

### CLM-012 — Acceptance Criteria

> ##### Acceptance Criteria
>
> | Criterion | Acceptance Evidence | Source reread evidence |
> |---|---|---|
> | Required package members are identified and represented in the native JSON foundation. | Manifest, model payload, stable ID map, loss report, validation report, and diagnostics are listed as package members and covered by schema/builder/fixture/test evidence. | Reread Specification.md Requirements, Datasheet.md Package Members, and TP-EXPORT-006 run evidence for A-001. |
> | Boundary review names excluded claim classes. | Review evidence confirms no release, external compatibility, code-compliance, solver-validation, professional-reliance, or professional-acceptance claim is introduced. | Reread Specification.md Verification Requirements and Procedure.md Bounded Implementation Procedure for X-001. |
>

### CLM-013 — Downstream Use

> ##### Downstream Use
>
> The native JSON package foundation supports later adapter SDK and target exporter work as a project-owned intermediate package. Later work must still define API/CLI/GUI integration, production project-store export binding, downstream target adapter consumption, and any target-specific behavior in separately scoped tranches.

- **AC-001** — The contract preserves package, manifest, target-profile, stable-ID-map and loss-report members; canonical JSON/JCS-compatible hashing and deterministic ordering; model, unit, provenance and diagnostic metadata; complete explicit loss classification; schema validation; invented or rights-cleared fixtures; and downstream adapter boundaries without adding solver execution, target equivalence, lifecycle acceptance, or professional authority.

## Production and Verification Method — Praxeology

### CLM-014 — Procedure: DEL-17-03 Native open JSON export package

> #### Procedure: DEL-17-03 Native open JSON export package
>

### CLM-015 — Population Procedure

> ##### Population Procedure
>
> 1. Read DEL-17-01 source basis and DEL-17-02 export contract.
> 2. Maintain the native JSON package contract and bounded implementation foundation in sync.
> 3. Generate `_SEMANTIC.md` with `semantic-matrix-build`.
> 4. Generate `_SEMANTIC_LENSING.md` with `lens-register`.
> 5. Apply warranted P3 enrichment through `four-documents P3_ONLY`.
> 6. Extract dependencies into `Dependencies.csv`.
> 7. Record memory and run evidence.
>

### CLM-016 — Bounded Implementation Procedure

> ##### Bounded Implementation Procedure
>
> Bounded native JSON foundation work must:
>
> 1. Define concrete package schema and builder/writer artifacts inside the DEL-17-03 scope.
> 2. Preserve manifest, stable ID map, loss report, validation report, and diagnostics behavior.
> 3. Use invented public fixtures only.
> 4. Test the exact `deterministic_sorted_compact_json_payload_hash` label on DEL-17-03-computed hashes, lossless pass-through of governed upstream source checksum metadata, repeatable key-sorted compact bytes and SHA-256 digests, mutation sensitivity, timestamp-free hash inputs, member inventory, diagnostics, privacy/professional-boundary flags, writer output, and protected/private payload screening; do not represent this project-local algorithm as RFC 8785 conformance.
> 5. Keep protected, proprietary, release, compatibility, target-support, code-compliance, solver-validation, professional-reliance, and professional-acceptance claims out of the package.
> 6. Leave public API, GUI behavior, persistence runtime, project-store export flow, and downstream target-adapter behavior to separately scoped work.
>

### CLM-017 — Semantic Enrichment Verification

> ##### Semantic Enrichment Verification
>
> Pass 3 checked the lensing register against DEL-17-02 manifest, stable-ID, and loss-report requirements. Later TP-EXPORT-006 evidence authorized and produced the bounded schema, builder, fixture, and test foundation now reflected in this procedure.
>
>

### CLM-018 — Pass 3 Source Reread Evidence

> ##### Pass 3 Source Reread Evidence
>
> | Register Item | Source slices reread | Disposition |
> |---|---|---|
> | A-001 | Specification.md Requirements; Datasheet.md Package Members | Incorporated as member-level acceptance criteria in Specification.md. |
> | F-001 | Guidance.md Closed Foundation Questions And Residual Boundaries | Dispositioned by TP-EXPORT-006 foundation evidence; residual integration work remains future-scoped. |
> | X-001 | Specification.md Verification Requirements; Procedure.md Bounded Implementation Procedure | Incorporated as explicit boundary-review verification in Specification.md and Procedure.md. |

- **VER-001** — Validate the contract and review source parity, all native package members and implementation artifacts, common-contract conformance, deterministic serialization and hash basis, stable-ID and loss-report closure, schema and fixture evidence, target-neutral boundaries, retained residual gates, and absence of proprietary content, hidden defaults, compatibility proof, or professional claims.

## Governing Values and Decisions — Axiology

### CLM-019 — Guidance: DEL-17-03 Native open JSON export package

> #### Guidance: DEL-17-03 Native open JSON export package
>

### CLM-020 — Design Guidance

> ##### Design Guidance
>
> Treat the native JSON package as the least target-specific export path. Its main value is traceability: it should make source identity, package contents, canonical IDs, diagnostics, and losses visible before any target adapter consumes the package.
>
> The package foundation should be deterministic. Hashes computed by DEL-17-03 use key-sorted compact JSON (`sort_keys=True`, compact separators, ASCII escaping) labelled `deterministic_sorted_compact_json_payload_hash`; this is not an RFC 8785 conformance claim. A governed caller-supplied source checksum is upstream evidence: preserve its declared algorithm/canonicalization/digest metadata after shape validation rather than relabelling it. Runtime timestamps, if retained for reportability, should be declared rather than hidden inside hashes or manifests.
>

### CLM-021 — Interpretation Guidance

> ##### Interpretation Guidance
>
> - Use DEL-17-02 as the contract foundation.
> - Use DEL-17-01 for target-boundary discipline and source authority.
> - Prefer explicit omissions over silent defaults.
> - Treat validation reports as package evidence, not solver validation.
> - Keep adapter-specific behavior in later deliverables.
> - Treat the native JSON schema, builder, invented fixture, and focused tests as DEL-17-03-owned foundation outputs.
> - Keep API/CLI/GUI integration, project-store export flow, downstream target adapters, and target-specific behavior in later deliverables or separately approved tranches.
>

### CLM-022 — Closed Foundation Questions And Residual Boundaries

> ##### Closed Foundation Questions And Residual Boundaries
>
> | Item | Question | Current disposition |
> |---|---|---|
> | TBD-17-03-001 | Which concrete JSON schemas will bind package members? | Closed for the bounded foundation by `schemas/native_json_export.schema.json`; future schema changes require governed follow-up. |
> | TBD-17-03-002 | Which hash canonicalization helper will package writer code use? | Closed for hashes computed by the bounded foundation through `core/handoff/native_json/package.py` key-sorted compact JSON and the project-local `deterministic_sorted_compact_json_payload_hash` label; governed source checksum metadata remains upstream-labelled pass-through evidence, and broader project hash unification remains an integration concern. |
> | TBD-17-03-003 | Which invented fixtures will exercise native JSON round trips? | Closed for the bounded foundation by `fixtures/native_json/invented/native_json_export_package.json` and focused tests. |
> | TBD-17-03-004 | Which concrete schema and writer binding source will authorize native JSON implementation? | Closed for the bounded foundation by TP-EXPORT-006 implementation evidence; API/CLI/GUI and project-store binding remain future scoped work. |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | CLM-008 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
