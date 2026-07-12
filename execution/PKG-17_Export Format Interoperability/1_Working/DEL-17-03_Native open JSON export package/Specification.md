# Specification: DEL-17-03 Native open JSON export package

## Normative Scope

DEL-17-03 shall define and implement the bounded native open JSON export package foundation. It shall consume DEL-17-01 source boundaries and DEL-17-02 export package/profile/stable-ID/loss-report requirements.

The bounded implementation scope includes the native JSON schema contract, deterministic package builder/writer helper, invented public fixture, and focused tests for sorted compact serialization, member hashes, manifest/member inventory, stable ID map, loss report, validation report, diagnostics, provenance, privacy, and professional-boundary flags.

This deliverable shall not implement public APIs, GUI behavior, persistence runtime or project-store export flow, downstream target adapters, CAEPIPE/PCF/glTF target behavior, release claims, compatibility claims, code-compliance claims, solver-validation claims, professional-reliance claims, or professional-acceptance claims.

## Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-03-REQ-001 | The native JSON package shall include a manifest, model payload, stable ID map, loss report, validation report, and diagnostics concept. |
| DEL-17-03-REQ-002 | The package contract shall preserve canonical OpenPipeStress identity directly in JSON records or through a package-local ID map. |
| DEL-17-03-REQ-003 | The manifest shall identify source model reference, export profile ID, package members, deterministic hash basis, and boundary notes. |
| DEL-17-03-REQ-004 | The loss report shall use the DEL-17-02 categories: exported, omitted, approximated, delegated, unsupported, and TBD. |
| DEL-17-03-REQ-005 | Validation report content shall be package-shape and source-boundary evidence only; it shall not claim solver validation or professional acceptance. |
| DEL-17-03-REQ-006 | The implementation shall emit deterministic, key-sorted, compact JSON and label SHA-256 hashes it computes `deterministic_sorted_compact_json_payload_hash`, without hiding runtime timestamps inside hash inputs. Governed caller-supplied source checksum mappings shall preserve their declared canonicalization and digest metadata or be rejected as malformed; the project-local computed-hash label does not assert RFC 8785 conformance. |
| DEL-17-03-REQ-007 | The implementation shall include invented public fixture and focused tests for package shape, deterministic hashes, diagnostics, privacy/professional-boundary flags, and protected/private payload screening. |
| DEL-17-03-REQ-008 | Target-specific or adapter-specific behavior shall remain outside the native JSON package foundation unless separately scoped. |

## Verification Requirements

| Check | Requirement |
|---|---|
| Four-document check | The four-document kit exists. |
| Dependency schema | `Dependencies.csv` validates as v3.1. |
| Semantic artifacts | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and are lens artifacts only. |
| Schema and fixture checks | Native JSON schema and invented fixture parse as JSON. |
| Builder and test checks | Native JSON builder compiles and focused tests pass. |
| Hash-basis checks | Focused tests verify the exact label on DEL-17-03-computed hashes, lossless pass-through of a differently labelled governed source checksum, repeatable bytes/digests, digest mutation sensitivity, timestamp-free hash inputs, and absence of an RFC 8785 conformance claim. |
| Boundary review | No protected/proprietary data or external compatibility claim is added. |

## Acceptance Criteria

| Criterion | Acceptance Evidence | Source reread evidence |
|---|---|---|
| Required package members are identified and represented in the native JSON foundation. | Manifest, model payload, stable ID map, loss report, validation report, and diagnostics are listed as package members and covered by schema/builder/fixture/test evidence. | Reread Specification.md Requirements, Datasheet.md Package Members, and TP-EXPORT-006 run evidence for A-001. |
| Boundary review names excluded claim classes. | Review evidence confirms no release, external compatibility, code-compliance, solver-validation, professional-reliance, or professional-acceptance claim is introduced. | Reread Specification.md Verification Requirements and Procedure.md Bounded Implementation Procedure for X-001. |

## Downstream Use

The native JSON package foundation supports later adapter SDK and target exporter work as a project-owned intermediate package. Later work must still define API/CLI/GUI integration, production project-store export binding, downstream target adapter consumption, and any target-specific behavior in separately scoped tranches.
