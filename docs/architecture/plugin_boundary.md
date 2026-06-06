---
doc_id: OPS-PLUGIN-BOUNDARY
doc_kind: architecture.api_contract
status: draft
created: 2026-04-30
deliverable_id: DEL-10-01
package_id: PKG-10
scope_item: SOW-030
decomposition_revision: "0.7"
scope_change_authority: SCA-004
---

# Plugin Boundary

This document defines the public plugin and API boundary rules for OpenPipeStress. The machine-readable companion is `api/api_boundary_contract.yaml`, written as strict JSON syntax so it can be validated without YAML parser dependencies.

## Boundary Rules

- Public API contracts are schema-first. Transport remains `TBD`; this deliverable does not lock the project to HTTP, OpenAPI, local IPC, CLI stdin/stdout, or an embedded plugin ABI.
- External callers use command, query, job, and result-envelope boundaries. Mutating operations are commands, read-only operations are queries, long-running solve/export/report work is a job, and nontrivial outputs return result envelopes.
- Persistence is not a public plugin surface. Plugins, adapters, and external API callers must route persistence effects and reads through application-service commands, queries, or jobs; they must not receive SQL strings, raw SQLite connections/cursors, database handles, table names, or direct project-store mutation hooks.
- Validation-test execution is a governed job boundary when exposed through public API or plugin surfaces. Validation fixtures and outputs must carry provenance, diagnostics, reproducibility metadata, and result envelopes.
- Plugins and adapters translate data at the boundary. They do not own domain validation, unit validation, provenance policy, protected-content screening, rule-pack sandboxing, or report boundary controls.
- Model import/export, solver invocation, result access, rule-pack hooks, diagnostics, provenance, privacy, permissions, and checksums are explicit contract categories.
- SCA-004 export interoperability surfaces are boundary concepts here, not implemented writers or endpoint definitions. Export profiles, stable ID maps, loss reports, external-run evidence, stress-neutral export, conservative PCF export, GLB/glTF review geometry export, and the export adapter SDK must remain routed through the same command/query/job/result-envelope controls as first-party operations.
- Export profiles identify target-family metadata, validation posture, no-bypass controls, and unresolved `TBD` decisions. They do not grant runtime access, select public transport, define endpoint syntax, or implement target writer behavior.
- Stable ID maps preserve canonical model identity when a target cannot carry canonical IDs directly. Loss reports record exported, omitted, approximated, unsupported, or delegated behavior instead of hiding target limitations.
- External-run evidence is optional, user-owned, hash/provenance-bound evidence for regression or handoff review. It is not a bundled commercial solver path, license bypass, formal external validation, professional acceptance, or code-compliance result.
- Stress-neutral CSV/JSON exports must preserve result status, units, diagnostics, provenance, hashes, and professional-boundary notices without exporting protected rule content or compliance semantics.
- Conservative PCF export must use explicit target limitations, stable ID mapping, and loss reporting; hidden translator defaults are not accepted as boundary behavior.
- GLB/glTF review geometry is visual review context only. It is not solver geometry, local FEA handoff geometry, external validation evidence, or professional acceptance.
- Export adapter SDK surfaces may expose manifests, target registries, validation checklists, diagnostics, and no-bypass constraints, but they must not widen plugin permissions, bypass application-service boundaries, or imply runtime loading/signing/isolation choices.
- API outputs must preserve the distinction between `MECHANICS_SOLVED`, `USER_RULE_CHECKED`, `USER_RULE_FAILED`, `RULE_INPUTS_INCOMPLETE`, `HUMAN_REVIEW_REQUIRED`, and any external human acceptance record. Human acceptance records are hash-bound references, not software-emitted `analysis_status` values. Automatic API status must not use compliance, approval, or certification language.
- Public transport protocol, endpoint syntax, OpenAPI transport binding, API stability level, code-generation tooling, external format list, plugin runtime, plugin loading/signing/isolation, and permission grant persistence remain `TBD` until a later human-approved architecture decision records them.

## Permission Model Skeleton

Permissions are denied by default and must be requested by plugin manifest or boundary request. Initial permission names are:

- `read_model`
- `write_model`
- `read_results`
- `run_solver`
- `read_rule_pack_metadata`
- `read_rule_pack_private_values`
- `write_rule_pack`
- `read_private_library`
- `write_private_library`
- `export_payload`
- `network_access`
- `filesystem_project_read`
- `filesystem_project_write`
- `diagnostics_emit`
- `read_export_profile`
- `write_export_profile`
- `read_stable_id_map`
- `write_stable_id_map`
- `read_loss_report`
- `write_loss_report`
- `record_external_run_evidence`
- `use_adapter_sdk`
- `export_stress_neutral`
- `export_conservative_pcf`
- `export_review_geometry`

These names are concept-level boundary tokens, not a final permission taxonomy. Grant storage, user-consent workflow, signature requirements, revocation behavior, and per-platform enforcement are `TBD`.

## No-Bypass Constraints

Plugins and adapters must not:

- bypass domain schema validation, unit checks, or dimensional checks;
- bypass provenance, redistribution-status, privacy, or protected-content checks;
- call solver internals in a way that skips solve-required input checks, diagnostics, result envelopes, model snapshot hashes, or reproducibility metadata;
- execute arbitrary code through rule packs or bypass the sandboxed rule-pack evaluator;
- bypass persistence, migration, checksum, manifest, report, diagnostics, validation, or human-acceptance boundary controls;
- open raw SQLite/database handles, issue SQL, depend on table names, or mutate the project store directly instead of using application-service commands, queries, or jobs;
- write private libraries, private rule packs, project data, or export payloads without explicit permission;
- transmit private project, material, component, or rule-pack data by default;
- bypass export profile registry checks, stable ID maps, or loss reports where target limitations affect identity, units, provenance, diagnostics, or downstream review;
- treat external-run evidence as authoritative solver validation, professional acceptance, or code-compliance evidence;
- treat GLB/glTF review geometry as solver geometry, local FEA geometry, formal validation evidence, or professional acceptance;
- rely on hidden translator defaults for conservative PCF or other target exports instead of emitting explicit unsupported, approximated, omitted, or delegated behavior records;
- allow adapter SDK templates or manifests to widen permissions, skip no-bypass controls, or depend on transport/runtime/loading details still marked `TBD`;
- embed protected standards text, protected tables, copied code formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, or private user rule packs in public artifacts;
- claim certification, sealing, approval, authentication, official endorsement, or code compliance for professional reliance.

## Private Data Handling

Private project models, private material/component libraries, owner design bases, licensed code data, and private rule packs remain user-controlled. Boundary payloads must carry privacy classification, provenance records, redistribution status, and checksum metadata where applicable.

Telemetry is off by default and must not include private engineering data, code data, rule-pack values, project models, or proprietary source material. Public contribution candidates require protected-content screening and provenance review before acceptance.

If protected or proprietary content is suspected, the boundary result must block or quarantine the payload, return an `IP_BOUNDARY_WARNING` or privacy diagnostic, and route the issue to human review.

## Checksums and Provenance

Model snapshots, request payloads, result payloads, rule packs, plugin manifests, exports, and reports should be hashable with a canonical JSON/JCS-compatible basis where JSON payloads are hashed. Exact canonicalization details beyond that basis remain `TBD`.

Rule-pack references must include name or ID, version, checksum, source note, public/private marking, and redistribution status. Reports and exported results may reference private rule-pack identity and checksums without exposing protected formulas or private values in public templates.

## Remaining TBDs

- Public transport protocol and external API surface.
- Endpoint syntax, OpenAPI transport binding, API stability level, and code-generation tooling.
- Plugin packaging, loading, signing, isolation, and update mechanism.
- External import/export format list, target field coverage, concrete writer behavior, and exact target schemas.
- Export profile, stable ID map, loss report, external-run evidence, stress-neutral CSV/JSON, conservative PCF, GLB/glTF review geometry, and adapter SDK concrete contracts.
- Permission persistence, user consent UX, and revocation semantics.
- Rule expression grammar/library and sandbox enforcement implementation.
- Canonicalization edge cases for non-JSON payloads.
- CI gates and protected-content linter integration for plugin submissions.
