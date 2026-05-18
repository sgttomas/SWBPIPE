# Specification: DEL-10-05 Headless CLI and structured I/O analysis runner

## Scope

This deliverable defines setup-stage requirements and evidence for a future headless CLI or equivalent structured I/O analysis runner. The future runner exists to support early solver verification, regression automation, and non-GUI execution paths while the full GUI matures.

This setup run does not implement CLI/source code, fixtures, package manifests, CI workflows, external adapters, result-export schemas, or final command syntax. Those remain future implementation artifacts or separate deliverables.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| R-01 | The future runner must execute through schema-first command/query/job/result-envelope boundaries rather than direct solver bypasses. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 | Confirm future implementation calls application-service contracts and preserves command/job separation. |
| R-02 | Structured input and output must remain unit-aware, deterministic, and reproducible. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/_Registers/ScopeLedger.csv` SOW-054 and OBJ-012 | Confirm future fixtures include unit-bearing values and deterministic rerun expectations. |
| R-03 | Missing solve-required or rule-check-required data must surface as diagnostics, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` section 3 | Confirm future runner exits/results include blocking diagnostics for missing required inputs. |
| R-04 | Result output must align with schema-first result exports and diagnostic/result-envelope contracts. | `docs/_Registers/ScopeLedger.csv` SOW-046 and SOW-061; `docs/_Registers/Deliverables.csv` DEL-10-05 notes | Confirm future outputs can be consumed by regression comparison and report/result export workflows without private-data leakage. |
| R-05 | Diagnostics must preserve code/class/severity/source/object/remediation/provenance semantics and no compliance/certification claim. | `docs/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/CONTRACT.md` OPS-K-AUTH-1 | Confirm future error/warning examples include professional-boundary notices where relevant. |
| R-06 | The future runner must support build and CI automation without deciding the CI provider, package manifest, or release matrix in this setup pass. | `docs/_Registers/ScopeLedger.csv` SOW-032; `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-002 and DEC-012 | Record remaining TBDs and avoid editing package manifests or repo-level automation files. |
| R-07 | Public examples and fixtures must use original/invented or otherwise permitted data and must not embed protected standards content. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-RULE-1; `docs/DIRECTIVE.md` sections 3-5 | Future fixture review must check protected-content risk and provenance. |
| R-08 | The runner must not transmit private project, material, component, or rule-pack data by default. | `docs/CONTRACT.md` OPS-K-PRIV-1/2; `docs/DIRECTIVE.md` section 6 | Future runner configuration review must confirm local-first/private-data defaults. |
| R-09 | Runner results may indicate mechanics solve and user-rule-check statuses but must not use automatic code-compliance status. | `docs/TYPES.md` section 4; `docs/CONTRACT.md` OPS-K-MECH-2 and OPS-K-AUTH-1 | Future status tests must reject misleading compliance/certification wording. |

## Standards

| Standard or basis | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 baseline | Governs public schema/interchange posture for future structured I/O | Accepted by SCA-001; exact schema files TBD |
| Canonical JSON/JCS-compatible hashing | Applies where JSON payload hashes are used for reproducibility | Accepted by SCA-001; physical package/container TBD |
| Command/query/job/result-envelope baseline | Governs application-service separation for GUI/headless execution | Accepted by SCA-001; concrete interface language TBD |
| Professional responsibility boundary | Prohibits software or agent claims of certification, sealing, approval, or automatic code compliance | Binding invariant |

## Verification

Setup verification for this run is limited to document/setup artifacts:

- Four-document kit exists with required sections.
- `_SEMANTIC.md` contains matrices A, B, C, F, D, K, G, X, T, and E and passes local semantic-gate checks.
- `_SEMANTIC_LENSING.md` includes complete coverage for A, B, C, F, D, X, and E.
- `Dependencies.csv` includes the v3.1 dependency schema and active rows with evidence.
- `_STATUS.md` remains no higher than `SEMANTIC_READY`.
- No files are written outside the assigned deliverable folder.

Future implementation verification remains TBD and must be handled in a later sealed task. It should include schema validation, deterministic runner regression cases, diagnostic failure cases, result-export compatibility checks, protected-content/provenance gates, and CI automation checks as applicable.

## Documentation

Required setup artifacts for this run:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

## Pass 3 Lensing Addendum

Semantic lensing identified one implementation-sensitive TBD that must remain visible: exact CLI command names, exact structured I/O schema fields, public API transport, CI provider, and release matrix are not established by this setup deliverable. Future implementation work must resolve those details through sealed scope or human approval before hard-coding commands, schema fields, or automation behavior.

