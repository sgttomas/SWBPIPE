# Specification: DEL-10-04 Build, packaging, and CI/CD pipeline

## Scope

This specification governs the setup basis for `DEL-10-04` only. It describes requirements and verification expectations for a future build, packaging, and CI/CD implementation pass, but this setup session does not modify CI workflows, packaging scripts, manifests, release files, source code, or repository-level artifacts.

In scope for this setup run:

- define build/package/CI requirements at the deliverable-document level;
- surface unresolved authority decisions as `TBD`;
- preserve architecture-basis constraints from SCA-001;
- produce semantic and dependency setup artifacts.

Out of scope for this setup run:

- selecting a final CI provider;
- finalizing a release matrix or final thresholds;
- creating or editing actual workflow files, packaging scripts, manifests, release templates, or source code;
- making certification, endorsement, sealing, or code-compliance claims.

## Requirements

| ID | Requirement | Source basis | Verification |
|---|---|---|---|
| REQ-10-04-01 | Future implementation must provide reproducible build, packaging, and CI/CD workflows for supported platforms. | SOW-032; Deliverables.csv DEL-10-04 | Human review of future implementation artifacts. |
| REQ-10-04-02 | Build/test gates must align with the accepted Cargo, Vitest, Playwright, validation, protected-content, and provenance gate baseline. | SOFTWARE_DECOMP AB-00-08; DEC-011 | Future CI/job review and test evidence. |
| REQ-10-04-03 | Desktop packaging planning must follow the Tauri-supported macOS, Windows, and Linux baseline without finalizing the detailed platform release matrix in this setup pass. | SOW-032 notes; architecture baseline | Confirm platform matrix remains `TBD` unless human-ruling evidence is cited. |
| REQ-10-04-04 | CI provider, coverage thresholds, performance thresholds, exact dependency versions, signing, publishing, and release matrix details must remain `TBD` unless a human authority record resolves them. | DEC-012; OI-002; user brief | Check all unresolved choices are visible as `TBD`. |
| REQ-10-04-05 | Pipeline and packaging concepts must not bypass unit checks, provenance checks, diagnostics, privacy controls, or adapter/plugin governance boundaries. | AB-00-02; AB-00-06; AB-00-07; OPS-K-UNIT-1; OPS-K-PRIV | Future implementation review and security/privacy checks. |
| REQ-10-04-06 | Public release automation must not include protected standards text, protected examples, proprietary engineering values, private rule packs, private project data, or private library data. | OPS-K-IP-1/2/3; OPS-K-DATA-1/2/3; OPS-K-PRIV | Protected-content/provenance gate and review. |
| REQ-10-04-07 | Release artifacts and pipeline status must not claim certification, sealing, approval, authentication, or engineering code compliance for reliance. | OPS-K-AUTH-1; professional responsibility boundary | Product-claims and release-note review. |
| REQ-10-04-08 | Packaging and CI outputs must support reproducibility evidence such as commit/build identifiers, test results, validation status, and known limitations. | PRD 22; VALIDATION_STRATEGY release gate; SPEC reporting/audit principles | Future release checklist and reproducibility review. |

## Standards

No external engineering code or standards-body text is incorporated by this deliverable. Governing project standards for this setup pass are internal OpenPipeStress governance artifacts: `INIT.md`, `docs/DIRECTIVE.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/VALIDATION_STRATEGY.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md`.

The term "Tauri-supported targets" is used only as an architecture-baseline label from the sealed context. Exact Tauri target details, operating-system versions, signing requirements, installer formats, and publishing rules remain `TBD`.

## Verification

| Check | Method | Expected result |
|---|---|---|
| Document kit presence | File inspection | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. |
| Semantic setup | File inspection and local QA | `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist, with no matrix parse errors. |
| Dependency setup | Schema validation | `Dependencies.csv` validates against v3.1 schema. |
| Scope boundary | Diff/file inspection | No CI workflows, package scripts, manifests, release files, source code, or repo-level artifacts are modified. |
| TBD preservation | Text review | CI provider, release matrix, and thresholds remain visible as `TBD`. |
| Protected-data boundary | Text review | No protected standards content, proprietary values, or private data are introduced. |
| Authority boundary | Text review | No certification, sealing, approval, endorsement, or compliance claim is made. |

## Documentation

Required setup artifacts for this session:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/TASK_RUN_*.md`
- `_STATUS.md`

Anticipated implementation artifacts remain future work:

- CI workflows
- packaging scripts
- release notes template

