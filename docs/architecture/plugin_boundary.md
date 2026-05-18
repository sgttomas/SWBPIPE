---
doc_id: OPS-PLUGIN-BOUNDARY
doc_kind: architecture.api_contract
status: draft
created: 2026-04-30
deliverable_id: DEL-10-01
package_id: PKG-10
scope_item: SOW-030
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

Grant storage, user-consent workflow, signature requirements, revocation behavior, and per-platform enforcement are `TBD`.

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
- External import/export format list.
- Permission persistence, user consent UX, and revocation semantics.
- Rule expression grammar/library and sandbox enforcement implementation.
- Canonicalization edge cases for non-JSON payloads.
- CI gates and protected-content linter integration for plugin submissions.
