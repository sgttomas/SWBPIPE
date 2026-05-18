# Procedure: DEL-08-02 Audit manifest and model hash

## Purpose

This procedure defines how the future audit-manifest feature should be produced and verified. In this setup session it is documentation only; no hashing code, schemas, source implementation, or tests are created.

## Prerequisites

Before implementation work proceeds, the following inputs or upstream contracts should be available or explicitly marked `TBD`:

| Prerequisite | Expected source | Reason |
|---|---|---|
| Canonical model/persistence payload boundary | DEL-02-05 Project persistence and round-trip serialization | The model hash needs a defined canonical JSON payload boundary and migration/versioning posture. |
| Unit-system and unit-bearing value conventions | DEL-02-02 Unit system and dimensional-analysis core contract | Reproducible model identity must not depend on implicit unit defaults. |
| Rule-pack checksum and lifecycle fields | DEL-06-04 Private rule-pack lifecycle and checksum handling | The manifest records rule-pack identity/version/checksum/source status without copying private payloads. |
| Solver/version and deterministic run metadata | DEL-10-05 Headless CLI and structured I/O analysis runner | Automated replay/report workflows need a consistent run envelope and version stamp. |
| Report renderer integration point | DEL-08-01 Calculation report generator | Reports consume manifest fields for review and reproducibility. |
| Result export contract | DEL-08-04 Result export format | Exported results should carry or reference manifest/hash metadata for regression and review. |

If any prerequisite is unavailable during future implementation, record the missing dependency as `TBD` or a warning rather than inventing a default.

## Steps

1. Define the manifest boundary for the run: model payload reference, unit system, solver/application version, rule-pack references, external asset references, warnings, assumptions, and professional-boundary notices.
2. Canonicalize JSON payloads using the project-approved JCS-compatible basis before hashing.
3. Record non-JSON and binary assets as manifest entries with digest, provenance, media/type, and inclusion policy.
4. Capture rule-pack identity, version, checksum, source notice, and redistribution status without copying protected formulas or private values into public artifacts.
5. Capture solver version and deterministic settings relevant to replay or regression comparison.
6. Emit missing-input, missing-provenance, protected-content, or private-data warnings when manifest fields cannot be populated safely.
7. Expose manifest and hash metadata to report generation and result export surfaces without claiming professional approval or code compliance.
8. Verify deterministic behavior through hash/reproducibility tests in a later implementation brief.

## Verification

For this setup run, verification consists of:

- the four production documents exist and preserve the default section roles;
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` exist and remain lens artifacts, not engineering authorities;
- `Dependencies.csv` validates against the v3.1 schema;
- `_DEPENDENCIES.md` summarizes extracted dependency rows consistently;
- `_STATUS.md` is `SEMANTIC_READY` only after the setup sequence succeeds;
- no private/protected payloads or certification claims are introduced.

For future implementation, run the hash determinism, manifest stability, protected-content, and report reproducibility tests described in `Specification.md`.

## Records

This setup sequence records:

- document drafts in `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`;
- semantic setup outputs in `_SEMANTIC.md` and `_SEMANTIC_LENSING.md`;
- dependency outputs in `Dependencies.csv` and `_DEPENDENCIES.md`;
- execution evidence in `_run_records/`;
- lifecycle evidence in `_STATUS.md`.
