# Procedure: DEL-06-04 Private rule-pack lifecycle and checksum handling

## Purpose

This procedure describes how a future bounded implementation task should produce private rule-pack lifecycle and checksum handling while preserving the public/private data boundary. It also records how this setup run should be verified.

## Prerequisites

- Read and apply INIT.md, AGENTS.md, docs/CONTRACT.md, docs/SPEC.md, docs/IP_AND_DATA_BOUNDARY.md, docs/PRD.md, and the DEL-06-04 sealed context.
- Confirm the implementation task has an explicit write scope before editing code, schemas, registry modules, or tests.
- Treat DEL-06-01 rule-pack schema as an upstream schema source for final field names and enum ownership.
- Treat DEL-08-02 audit manifest/model hash as a downstream consumer of rule-pack checksum metadata.
- Treat PKG-12 security/privacy deliverables as the owner for private storage locations, encryption/access defaults, telemetry, redaction, and secret handling.
- Do not use or create real private rule-pack payloads in public repository artifacts.

## Steps

1. Establish the rule-pack lifecycle metadata boundary.
   - Include identity, version, source notice, redistribution/private status, checksum metadata, diagnostics, and review/quarantine disposition.
   - Do not embed protected code text, copied formulas, private values, or proprietary tables.

2. Define checksum payload boundaries.
   - For JSON payloads, use canonical JSON with JCS-compatible canonicalization before hashing.
   - Record algorithm, canonicalization, payload reference, and checksum value.
   - For non-JSON or binary assets, create a manifest-level hash entry and mark unresolved partition details as `TBD`.

3. Define stale-check and lifecycle diagnostics.
   - Missing source note, missing redistribution status, missing checksum, stale checksum, suspected protected content, and attempted public export of private content must be explicit diagnostics.
   - Missing rule-check-required values or units remain rule/check findings, not defaulted values.

4. Connect audit/report hooks.
   - Provide report-facing references to rule-pack ID/name, version, checksum, and source note.
   - Public templates must not render private formulas, protected tables, or copied standards text.

5. Preserve professional-boundary wording.
   - Do not emit `CODE_COMPLIANT`, certification, approval, sealing, endorsement, or equivalent professional reliance language.
   - Keep human acceptance external and hash-bound if it is introduced by later work.

6. Defer private storage and access decisions.
   - Record storage location, encryption defaults, access-control policy, permission persistence, and credential handling as `TBD` or PKG-12-owned unless the sealed brief explicitly authorizes them.

7. Verify setup artifacts for this run.
   - Confirm the four documents exist.
   - Confirm semantic matrix and lensing artifacts exist.
   - Validate `Dependencies.csv` with the v3.1 schema validator.
   - Check dependency enum fields with `validate_enum.py`.
   - Confirm `_STATUS.md` remains `SEMANTIC_READY` only after all setup gates pass.

## Verification

| Check | Expected result |
|---|---|
| Four-document kit | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. |
| Data boundary scan | No private rule-pack payloads, protected standards data, proprietary formulas, or code-compliance claims are present. |
| Checksum basis | Specification and guidance state JCS-compatible canonical JSON for JSON payload hashes. |
| Deferred decisions | Storage, encryption, access-control defaults, and physical container decisions remain deferred. |
| Dependency schema | `Dependencies.csv` validates as v3.1. |
| Lifecycle state | `_STATUS.md` states `SEMANTIC_READY` after setup artifacts validate. |

## Records

This setup session records:

- four-document drafting and P3 enrichment in the production documents;
- semantic matrix output in `_SEMANTIC.md`;
- semantic lensing register in `_SEMANTIC_LENSING.md`;
- dependency extraction results in `Dependencies.csv` and `_DEPENDENCIES.md`;
- run records in `_run_records/`;
- final lifecycle state in `_STATUS.md`.
