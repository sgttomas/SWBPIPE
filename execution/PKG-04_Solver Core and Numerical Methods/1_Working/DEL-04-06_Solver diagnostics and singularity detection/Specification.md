# Specification: DEL-04-06 Solver diagnostics and singularity detection

## Scope

This deliverable specifies the setup boundary for a backend solver diagnostic layer covering singular systems, ill-conditioned systems, nonconverged analyses, invalid restraints, and solver status reporting. It does not implement solver code, choose numerical libraries, set numerical thresholds, or claim engineering compliance.

## Requirements

| ReqID | Requirement | Source |
|---|---|---|
| REQ-04-06-001 | The deliverable shall preserve deterministic diagnostic reporting for singular, ill-conditioned, and nonconverged analysis states. | SOW-053 |
| REQ-04-06-002 | Solver status reporting shall use machine-readable diagnostics with code, class, severity, source, affected object, message, remediation, and provenance. | AB-00-06 |
| REQ-04-06-003 | Solver diagnostics shall remain inside the mechanics boundary and shall not claim certification, sealing, approval, or code compliance. | OPS-K-MECH-2; OPS-K-AUTH-1 |
| REQ-04-06-004 | Missing solve-required data and invalid restraint definitions shall be explicit findings, not silently repaired defaults. | OPS-K-DATA-2; SOW-053 |
| REQ-04-06-005 | Unit-bearing diagnostic context shall preserve unit awareness and dimensional checks. | OPS-K-UNIT-1 |
| REQ-04-06-006 | Nonlinear nonconvergence diagnostics, when applicable, shall report convergence, active-set state, and unresolved non-convergence. | OPS-K-SOLVER-2; SPEC section 4.4 |
| REQ-04-06-007 | Diagnostic tests shall include singular-model coverage before release use. | OPS-K-SOLVER-1; SOW-053 |
| REQ-04-06-008 | Conditioning warnings shall be reportable, but exact thresholds and solver-library settings remain TBD until a future implementation decision. | SOW-035; OI-005 |
| REQ-04-06-009 | Future implementation acceptance shall prove that singular, ill-conditioned, nonconverged, and invalid-restraint diagnostics cannot be bypassed by ordinary solve/report flows. | `_SEMANTIC_LENSING.md` A-001; OPS-K-SOLVER-1 |
| REQ-04-06-010 | Diagnostic provenance shall identify, at minimum as TBD fields, the solver stage, affected object, diagnostic source, and relevant solver setting when available. | `_SEMANTIC_LENSING.md` F-002; AB-00-06 |

## Standards

No external protected standard text is introduced by this setup. Governing local standards are the project invariant catalog, architecture basis rows AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08, and the decomposition/register rows listed in `_CONTEXT.md`.

## Verification

| Requirement | Verification approach |
|---|---|
| REQ-04-06-001 | Singular, ill-conditioned, and nonconverged fixture tests with deterministic expected diagnostic classes. |
| REQ-04-06-002 | Schema or result-envelope tests confirming required diagnostic fields are present. |
| REQ-04-06-003 | Report/status wording review for mechanics-only and no-certification boundaries. |
| REQ-04-06-004 | Invalid input/restraint tests proving findings are emitted rather than hidden defaults. |
| REQ-04-06-005 | Unit tests for unit-bearing diagnostic context and dimensional consistency. |
| REQ-04-06-006 | Nonlinear support regression tests when nonlinear diagnostics are in scope. |
| REQ-04-06-007 | CI or release gate requiring deterministic solver diagnostic tests. |
| REQ-04-06-008 | Future threshold decision review; current setup records threshold values as TBD. |
| REQ-04-06-009 | Integration or service-level test showing blocking/warning diagnostics remain visible through result envelopes. |
| REQ-04-06-010 | Schema/result-envelope inspection for provenance fields or explicit TBD placeholders. |

## Documentation

Required local setup artifacts are `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, `_STATUS.md`, and `_run_records/`.
