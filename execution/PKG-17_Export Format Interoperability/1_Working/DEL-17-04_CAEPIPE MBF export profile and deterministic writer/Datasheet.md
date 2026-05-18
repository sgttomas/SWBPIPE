# Datasheet: DEL-17-04 CAEPIPE MBF export profile and deterministic writer

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-17-04 |
| Package | PKG-17 Export Format Interoperability |
| Type | BACKEND_FEATURE_SLICE |
| Source foundation | DEL-17-01 and DEL-17-02 |
| Lifecycle role | First CAEPIPE-specific export profile/writer design deliverable |

## Purpose

DEL-17-04 defines the document-level contract/design for a CAEPIPE MBF export profile and deterministic writer. It is grounded in admitted public/official CAEPIPE MBF references and the DEL-17-01 source basis.

This tranche does not implement an MBF writer, schema, parser, fixture, external execution harness, public API, GUI action, release claim, CAEPIPE compatibility claim, code-compliance claim, or professional-acceptance claim.

## Source and Evidence Slots

| Source slot | Required treatment |
|---|---|
| DEL-17-01 source basis | Carry CAEPIPE source findings, TBD register entries, and question-dossier gates into the target profile. |
| DEL-17-02 export contract | Carry export profile, stable-ID map, manifest, and loss-report vocabulary into the writer contract. |
| CAEPIPE public MBF import reference | Treat as evidence for MBF text input, keyword-ordered sections, and command-line behavior only; profile coverage remains source-gated. |
| CAEPIPE public MBF export reference | Treat as evidence that CAEPIPE exports MBF model data and version-flavoured MBF output; OpenPipeStress support claims remain `TBD`. |
| Implementation evidence | `TBD` until later invented fixtures, deterministic writer checks, and loss-report tests exist. |

## Profile Concepts

| Concept | Required treatment |
|---|---|
| Target family | CAEPIPE MBF profile. |
| Target version basis | `TBD` until source-confirmed. |
| Record-family subset | `TBD` until downstream source review and/or vendor clarification closes it. |
| Stable ID strategy | Direct MBF carrying versus sidecar mapping remains `TBD`. |
| Source basis trace | Each CAEPIPE-specific requirement must map to DEL-17-01, DEL-17-02, a public MBF source, or an explicit `TBD`. |
| Loss reporting | Required for exported, omitted, approximated, delegated, unsupported, and TBD behavior. |
| Pass-through options | May be target configuration only; not local code-checking logic. |

## Boundary Summary

- No CAEPIPE executable, commercial example, proprietary model, or copied vendor fixture is added.
- No reverse-engineered behavior or license-bypass workflow is introduced.
- No protected standards values, material allowables, SIF/flexibility values, or owner criteria are introduced.
- MBF export profile work remains a source-bounded design contract until a later implementation tranche.
