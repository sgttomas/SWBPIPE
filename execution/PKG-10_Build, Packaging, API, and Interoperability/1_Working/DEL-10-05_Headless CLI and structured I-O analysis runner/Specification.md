# Specification: DEL-10-05 Headless CLI and structured I/O analysis runner

## Scope

This deliverable defines setup-stage requirements and evidence for a future headless CLI or equivalent structured I/O analysis runner. The future runner exists to support early solver verification, regression automation, and non-GUI execution paths while the full GUI matures.

`DEC-065` settles the local CLI/process policy for this deliverable. The
current bounded implementation provides the stable `openpipestress-runner`
binary with `solve`, `validate-input`, `export-results`, `run-benchmark`, and
`run-regression` verbs. It does not implement release packaging/signing,
hosted/public transport, external adapters, CI workflows, public publication,
or lifecycle issuance.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| R-01 | The future runner must execute through schema-first command/query/job/result-envelope boundaries rather than direct solver bypasses. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-03 | Confirm future implementation calls application-service contracts and preserves command/job separation. |
| R-02 | Structured input and output must remain unit-aware, deterministic, and reproducible. | `docs/CONTRACT.md` OPS-K-UNIT-1; `docs/_Registers/ScopeLedger.csv` SOW-054 and OBJ-012 | Confirm future fixtures include unit-bearing values and deterministic rerun expectations. |
| R-03 | Missing solve-required or rule-check-required data must surface as diagnostics, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/DIRECTIVE.md` section 3 | Confirm future runner exits/results include blocking diagnostics for missing required inputs. |
| R-04 | Result output must align with schema-first result exports and diagnostic/result-envelope contracts. | `docs/_Registers/ScopeLedger.csv` SOW-046 and SOW-061; `docs/_Registers/Deliverables.csv` DEL-10-05 notes | Confirm future outputs can be consumed by regression comparison and report/result export workflows without private-data leakage. |
| R-05 | Diagnostics must preserve code/class/severity/source/object/remediation/provenance semantics and no compliance/certification claim. | `execution/_Decomposition/SOFTWARE_DECOMP.md` AB-00-06; `docs/CONTRACT.md` OPS-K-AUTH-1 | Confirm future error/warning examples include professional-boundary notices where relevant. |
| R-06 | The local runner must expose stable development/test CLI verbs while keeping CI provider, release matrix, signing/publication, and hosted/public transport outside this implementation. | `docs/_Registers/ScopeLedger.csv` SOW-032; `DEC-065`; `DEC-025`; `DEC-057`; `DEC-059` | Confirm `openpipestress-runner` verbs are stable, package scripts are not release claims, and repo-level CI/release workflow files are not introduced by this tranche. |
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

TP-RUNNER-015 verification covers the bounded final local CLI surface:

- `openpipestress-runner solve` runs an invented preview-model payload and
  emits structured JSON with a completed runner result and deterministic result
  references.
- `openpipestress-runner validate-input` emits structured blocking diagnostics
  for invalid request metadata.
- `openpipestress-runner run-benchmark` proves the stable verb mapping while
  returning a structured blocking diagnostic until a downstream benchmark
  payload binding exists.
- Rust crate tests cover the library, compatibility preview binary, and final
  CLI binary.
- The schema contract test confirms the `DEC-065` settled/deferred decision
  vocabulary.

Future implementation verification remains for persisted-project input,
full result-export payload binding, benchmark/regression payload execution,
CI/public transport/release packaging, and external adapter integrations.

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

Semantic lensing identified implementation-sensitive TBDs. `DEC-065` resolves
the local CLI command names, local process policy, network prohibition,
filesystem posture, and package-script posture for TP-RUNNER-015. Public API
transport, CI provider, release matrix, external adapter formats,
persisted-project storage roots, and release automation remain visible TBDs
that must be handled by their owning decisions or later bounded tranches.
