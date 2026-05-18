# Specification: DEL-17-03 Native open JSON export package

## Normative Scope

DEL-17-03 shall define the native open JSON export package contract/design for future implementation. It shall consume DEL-17-01 source boundaries and DEL-17-02 export package/profile/stable-ID/loss-report requirements.

This deliverable shall not implement code, schemas, tests, fixtures, package writers, public APIs, GUI behavior, persistence runtime, release claims, compatibility claims, code-compliance claims, or professional-acceptance claims.

## Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-03-REQ-001 | The native JSON package shall include a manifest, model payload, stable ID map, loss report, validation report, and diagnostics concept. |
| DEL-17-03-REQ-002 | The package contract shall preserve canonical OpenPipeStress identity directly in JSON records or through a package-local ID map. |
| DEL-17-03-REQ-003 | The manifest shall identify source model reference, export profile ID, package members, deterministic hash basis, and boundary notes. |
| DEL-17-03-REQ-004 | The loss report shall use the DEL-17-02 categories: exported, omitted, approximated, delegated, unsupported, and TBD. |
| DEL-17-03-REQ-005 | Validation report content shall be package-shape and source-boundary evidence only; it shall not claim solver validation or professional acceptance. |
| DEL-17-03-REQ-006 | Future implementation shall keep target-specific or adapter-specific behavior outside the native JSON contract unless separately scoped. |

## Verification Requirements

| Check | Requirement |
|---|---|
| Four-document check | The four-document kit exists. |
| Dependency schema | `Dependencies.csv` validates as v3.1. |
| Semantic artifacts | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and are lens artifacts only. |
| Boundary review | No protected/proprietary data or external compatibility claim is added. |

## Acceptance Criteria

| Criterion | Acceptance Evidence | Source reread evidence |
|---|---|---|
| Required package members are identified. | Manifest, model payload, stable ID map, loss report, validation report, and diagnostics are listed as package concepts before implementation proceeds. | Reread Specification.md Requirements and Datasheet.md Package Members for A-001. |
| Boundary review names excluded claim classes. | Review evidence confirms no release, external compatibility, code-compliance, solver-validation, or professional-acceptance claim is introduced. | Reread Specification.md Verification Requirements and Procedure.md Future Implementation Procedure for X-001. |

## Downstream Use

The native JSON package is intended to support later adapter SDK and target exporter work as a project-owned intermediate package. Later implementation must still define concrete schemas, package writer behavior, fixtures, and tests in a separate code tranche.
