# Datasheet: DEL-17-03 Native open JSON export package

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-03 |
| Package | PKG-17 Export Format Interoperability |
| Type | BACKEND_FEATURE_SLICE |
| Source foundation | DEL-17-01 and DEL-17-02 |
| Lifecycle role | First project-owned native JSON package implementation foundation for export interoperability |

## Purpose

DEL-17-03 defines the native open JSON export package as the project-owned debug, interchange, archival, adapter-input, manifest, ID-map, and validation-report package target.

This deliverable owns the bounded native JSON package foundation: schema contract, deterministic package builder/writer helper, invented public fixture, and focused tests. It does not implement public API, GUI action, persistence runtime, project-store export flow, downstream target adapter behavior, target compatibility, release claims, code-compliance claims, or professional-reliance claims.

## Package Members

| Member | Role |
|---|---|
| `manifest` | Records source model identity, export profile, package member inventory, hashes, diagnostics, and boundary notes. |
| `model_payload` | Carries the project-owned JSON representation selected by a future exporter. |
| `stable_id_map` | Maps canonical OpenPipeStress IDs to package-local records. |
| `loss_report` | Records exported, omitted, approximated, delegated, unsupported, and TBD behavior. |
| `validation_report` | Records package-shape and source-boundary checks without claiming solver validation. |
| `diagnostics` | Carries warnings and blocking messages from export preparation. |

## Implementation Artifacts

| Artifact | Role |
|---|---|
| `schemas/native_json_export.schema.json` | JSON Schema 2020-12 contract for the native JSON package foundation. |
| `core/handoff/native_json/` | Deterministic builder and canonical writer helper for native JSON package evidence. |
| `fixtures/native_json/invented/native_json_export_package.json` | Invented public fixture for package-shape and boundary checks. |
| `tests/test_native_json_export_package.py` | Focused tests for deterministic hashes, member inventory, diagnostics, privacy/professional-boundary flags, canonical writer output, and protected/private payload screening. |

## Profile Basis

The native JSON profile shall consume the DEL-17-02 export profile contract and declare:

- profile ID and profile version;
- source model identity and hash basis;
- unit and coordinate policy;
- canonical ID families included;
- package member path policy;
- deterministic timestamp policy;
- loss-report category policy;
- boundary notes for no release, compatibility, code-compliance, or professional claim.

## Boundary Summary

- Native JSON is project-owned interchange and adapter input, not a public compatibility promise.
- The package may preserve source model semantics but does not certify solver correctness.
- Protected standards values, private model data, proprietary target examples, and owner criteria remain excluded.
- Any future integration or target-adapter work must keep unsupported or omitted behavior visible in the loss report.
