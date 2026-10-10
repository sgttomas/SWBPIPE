# Review: DEL-10-02 Import-export adapter framework

## Audit Identity

| Field | Value |
|---|---|
| PackageID | PKG-10 |
| DeliverableID | DEL-10-02 |
| TaskProfile | PACKAGE_AUDIT |
| Audit | DEV-001 DAG-003 downstream PKG-02 compatibility audit |
| ReviewerID | TASK-PKG10-PKG02-AUDIT |
| Date | 2026-05-16 |
| ScopePath | `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working` |
| Verdict | TECHNICALLY_ADDRESSED_PENDING_HUMAN |

## Inputs Read

Required deliverable-local inputs were present and read: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, `Datasheet.md`, `Specification.md`, `Procedure.md`, and `Guidance.md`.

Targeted implementation evidence named by `MEMORY.md` was read for compatibility signals only: `schemas/adapter_framework.schema.yaml`, `core/adapters/framework/adapter_framework.py`, `fixtures/adapters/invented/invented_adapter_framework.json`, and `tests/test_adapter_framework_contract.py`.

PKG-02 foundation inputs were read for the five compatibility checks: DEL-02-01 through DEL-02-05 context, status, dependency, memory, and four-document artifacts, plus `docs/CONTRACT.md`.

No expected input for this audit was missing.

## PKG-02 Compatibility Verdict

| PKG-02 check | Verdict | Audit note |
|---|---:|---|
| DEL-02-01 canonical model/schema and physical source-of-truth role | TECHNICALLY_ADDRESSED_PENDING_HUMAN | Active package-local dependency row `DEV-001-STAGE2-DEL-10-02-PKG02-001` now names DEL-02-01 for model import/export schema records. |
| DEL-02-02 explicit unit metadata and no silent unit defaults | TECHNICALLY_ADDRESSED_PENDING_HUMAN | Active package-local dependency row `DEV-001-STAGE2-DEL-10-02-PKG02-002` now names DEL-02-02; specification, schema, fixture, and tests require unit validation and reject silent defaults. |
| DEL-02-03 mechanics/rule/human authority separation | TECHNICALLY_ADDRESSED_PENDING_HUMAN | Active package-local dependency row `DEV-001-STAGE2-DEL-10-02-PKG02-003` now names DEL-02-03; adapter result and professional-boundary fields keep mechanics readiness, rule-check readiness, and human review distinct. |
| DEL-02-04 plugin/adapter no-bypass constraints | PASS | Active dependency row `DEP-010-02-011` names DEL-02-04. Schema and code require no-bypass controls for public API boundary, unit validation, provenance, diagnostics, rule-pack sandbox, persistence/hash controls, and human acceptance boundary. |
| DEL-02-05 persistence/hash/provenance/round-trip assumptions | TECHNICALLY_ADDRESSED_PENDING_HUMAN | Active package-local dependency row `DEV-001-STAGE2-DEL-10-02-PKG02-005` now names DEL-02-05 for checksum/audit references and persistence hash controls. |

Overall classification: TECHNICALLY_ADDRESSED_PENDING_HUMAN. No PKG-02 blocker was found; the prior traceability gap from this adapter framework to PKG-02 foundations beyond DEL-02-04 is technically addressed in package-local metadata. Human disposition, lifecycle approval, and aggregate DAG authority remain unchanged.

## Findings Summary

| FindingID | Severity | Status | Summary |
|---|---:|---:|---|
| PKG10-DEL1002-PKG02-W001 | WARNING | TECHNICALLY_ADDRESSED_PENDING_HUMAN | The adapter framework now has active package-local dependency rows for DEL-02-01, DEL-02-02, DEL-02-03, DEL-02-04, and DEL-02-05. |

See `Review_Findings.csv` for the structured finding row.

## Deferred Or Not Applicable

Concrete external import/export formats, public transport, endpoint syntax, OpenAPI binding, plugin runtime/loading/signing/isolation, permission persistence, package scripts, CI provider, release matrix, physical project container, local FEA package format, redaction workflow, GUI/report runtime behavior, and real external file parsing remain `TBD` or outside DEL-10-02.

This audit did not rerun adapter tests and did not update lifecycle state, dependency registers, source code, schemas, fixtures, tests, product documents, or coordination artifacts.

## Audit Boundary

This is an audit-only package-scoped review against PKG-02 foundation contracts. It is not a candidate promotion, lifecycle approval, release claim, certification, sealing, code-compliance claim, professional reliance statement, or product implementation review.

## Human Ruling Evidence Packet - 2026-06-07

Prepared by REVIEW for human disposition support. No `HumanDisposition`,
finding `Status`, lifecycle state, dependency register, DAG artifact, fixture,
schema, code, or test file is changed by this evidence packet.

### Ruling HR-DEL1002-001 - Package-Audit Warning Disposition

| Field | Evidence |
|---|---|
| Finding | `PKG10-DEL1002-PKG02-W001` in `Review_Findings.csv` |
| Current finding state | `HumanDisposition=TBD`; `Status=TECHNICALLY_ADDRESSED_PENDING_HUMAN` |
| Decision needed | Decide whether the PKG-02 compatibility warning is technically acceptable, needs revision, or should be deferred. |
| Technical evidence | Package-local dependency rows `DEV-001-STAGE2-DEL-10-02-PKG02-001/002/003/005` are active and satisfied for DEL-02-01, DEL-02-02, DEL-02-03, and DEL-02-05. Existing active row `DEP-010-02-011` names DEL-02-04. |
| Current hardening evidence | `core/adapters/framework/adapter_framework.py` now requires the SCA-003 persistence no-bypass controls, and `tests/test_adapter_framework_contract.py` contains focused regression coverage for application-service persistence routing. |
| Validation evidence | PKG-10 fan-in passed `python3 -m pytest -q tests/test_adapter_framework_contract.py tests/test_local_fea_handoff_contract.py tests/test_release_readiness_script.py tests/test_coordination_maintenance.py tests/test_headless_runner_contract.py` with 27 tests, release-readiness dry-run/execute checks, DAG dependency schema validation, headless Cargo tests, and `git diff --check`. |

Recommended human ruling for this finding: `ACCEPT_AS_IS` for technical
resolution of the PKG-02 compatibility warning. This does not itself authorize
`CHECKING`, release, candidate promotion, or professional/code-compliance
claims.

Alternative rulings:

- `REVISE` if the human wants additional PKG-02 traceability beyond the active
  package-local dependency rows and focused adapter tests.
- `DEFER` if the human wants the technical resolution accepted for now but
  aggregate DAG promotion or lifecycle decision held separately.

### Ruling HR-DEL1002-002 - Invented Adapter Fixture Refresh

| Field | Evidence |
|---|---|
| Issue | `fixtures/adapters/invented/invented_adapter_framework.json` is outside the 2026-06-07 tranche write scope and lacks four newer persistence no-bypass controls. |
| Missing controls | `must_route_persistence_through_application_services`, `must_not_expose_sql_or_raw_sqlite`, `must_not_expose_table_names`, `must_not_mutate_project_store_directly` |
| Current behavior | The hardened validator rejects the unmodified invented fixture for those missing controls. |
| Source evidence | `MEMORY.md` records the fixture gap under `2026-06-07 - PKG-10 review-readiness hardening`; the PKG-10 fan-in record lists the same item under `Needs Human Ruling`. |
| Scope relevance | `_CONTEXT.md` names `sample invented adapter` as an anticipated artifact for DEL-10-02, so a rejected invented fixture is material to review readiness. |

Recommended human ruling before advancing DEL-10-02 to `CHECKING`: authorize a
bounded fixture-refresh task to add the four controls to
`fixtures/adapters/invented/invented_adapter_framework.json`, then rerun
`python3 -m pytest -q tests/test_adapter_framework_contract.py` and update this
review evidence.

Alternative ruling:

- `DEFER` only if the human explicitly accepts that the sample invented adapter
  fixture is not required for the current `CHECKING` boundary. Record the
  rationale because this conflicts with the current anticipated-artifact signal.

### Transition Recommendation

Human-approved remediation completed. Current REVIEW recommendation is
`RECOMMEND_ADVANCE` for `IN_PROGRESS -> CHECKING`, subject to final status-gate
validation.

### Human Ruling Record - 2026-06-07

| Ruling | Human decision | Evidence update |
|---|---|---|
| HR-DEL1002-001 | `ACCEPT_AS_IS` | `Review_Findings.csv` updated to `HumanDisposition=ACCEPT_AS_IS`, `Status=RESOLVED`. |
| HR-DEL1002-002 | Fixture refresh authorized and completed | `fixtures/adapters/invented/invented_adapter_framework.json` now includes the four SCA-003 persistence no-bypass controls; targeted validation reports `accepted=True`. |

Validation after ruling:

- `python3 -m pytest -q tests/test_adapter_framework_contract.py tests/test_local_fea_handoff_contract.py tests/test_release_readiness_script.py tests/test_coordination_maintenance.py tests/test_headless_runner_contract.py`
  passed with 27 tests.
- `cargo test --manifest-path core/runner/headless/Cargo.toml` passed with 11 tests.

No release, professional/code-compliance, DAG promotion, dependency-register
mutation, or aggregate authority claim is implied by this ruling record.
