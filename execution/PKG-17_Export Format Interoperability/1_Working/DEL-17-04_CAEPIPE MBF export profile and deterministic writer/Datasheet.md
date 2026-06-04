# Datasheet: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-04 |
| Package | PKG-17 Export Format Interoperability |
| Type | BACKEND_FEATURE_SLICE |
| Source foundation | DEL-17-01 and DEL-17-02 |
| Lifecycle role | First CAEPIPE-specific export profile/writer foundation deliverable |

## Purpose

DEL-17-04 defines the document-level contract/design and first bounded implementation foundation for a CAEPIPE MBF export profile and deterministic writer. It is grounded in admitted public/official CAEPIPE MBF references, the DEL-17-01 source basis, and the DEL-17-02 export package/profile/loss-report contract.

The implementation foundation emits a deliberately narrow invented smoke subset, a JSON package contract, a deterministic MBF text member, sidecar stable-ID mapping, diagnostics, and loss reports. It does not implement a parser, external execution harness, public API, GUI action, release claim, CAEPIPE compatibility claim, code-compliance claim, or professional-acceptance claim.

## Source and Evidence Slots

| Source slot | Required treatment |
|---|---|
| DEL-17-01 source basis | Carry CAEPIPE source findings, TBD register entries, and question-dossier gates into the target profile. |
| DEL-17-02 export contract | Carry export profile, stable-ID map, manifest, and loss-report vocabulary into the writer contract. |
| CAEPIPE public MBF import reference | Treat as evidence for MBF text input, keyword-ordered sections, and command-line behavior only; profile coverage remains source-gated. |
| CAEPIPE public MBF export reference | Treat as evidence that CAEPIPE exports MBF model data and version-flavoured MBF output; OpenPipeStress support claims remain `TBD`. |
| Implementation evidence | `core/handoff/caepipe_mbf/`, `schemas/caepipe_mbf_export.schema.json`, `fixtures/caepipe_mbf/invented/`, and `tests/test_caepipe_mbf_export_package.py` provide bounded foundation evidence. |

## Profile Concepts

| Concept | Required treatment |
|---|---|
| Target family | CAEPIPE MBF profile. |
| Target version basis | `TBD` until source-confirmed. |
| Record-family subset | `TBD` until downstream source review and/or vendor clarification closes it. |
| Profile-basis guardrail | The implementation foundation blocks blank or unsupported target-version and record-subset basis values; until source-confirmed closure exists, the profile must carry `TBD-17-01-001`, `TBD-17-01-002`, and `TBD-17-01-003`. |
| Stable ID strategy | Direct MBF carrying remains `TBD`; the implementation foundation uses manifest-referenced sidecar mapping as the conservative default. |
| Source basis trace | Each CAEPIPE-specific requirement must map to DEL-17-01, DEL-17-02, a public MBF source, or an explicit `TBD`. |
| Loss reporting | Required for exported, omitted, approximated, delegated, unsupported, and TBD behavior. |
| Unsupported-entity severity | Missing or malformed unsupported-entity evidence blocks the package; explicit unsupported behavior is warning-level by default unless separately marked blocking; `info` severity is not accepted for unsupported behavior. |
| Pass-through options | May be target configuration only; not local code-checking logic. |

## Boundary Summary

- No CAEPIPE executable, commercial example, proprietary model, or copied vendor fixture is added.
- No reverse-engineered behavior or license-bypass workflow is introduced.
- No protected standards values, material allowables, SIF/flexibility values, or owner criteria are introduced.
- MBF export work remains a source-bounded foundation until a later tranche closes target version/profile and record-family coverage.
- Guardrail diagnostics preserve source uncertainty; they do not close CAEPIPE version/profile, record-family coverage, direct stable-ID carrying, compatibility, or professional reliance.
