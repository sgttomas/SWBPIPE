# Specification: DEL-04-05 Sparse solver performance harness

## Scope

This deliverable specifies setup evidence for a future test-suite harness that will exercise sparse solver performance, reproducibility, and numerical-conditioning behavior for practical piping-model sizes.

This setup pass does not implement a benchmark runner, add tests, choose thresholds, edit solver code, select a numerical library, or introduce proprietary/protected benchmark data.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-04-05-RQ-001 | The harness shall be separate from solver logic and shall not modify production solver behavior. | DEL-04-05 context; PKG-04 note; AB-00-02 | Module boundary review once implementation exists. |
| DEL-04-05-RQ-002 | Harness cases shall support deterministic regression comparison for the same model, unit system, solver version, and settings. | SOW-035; docs/SPEC.md section 4.5; OPS-K-SOLVER-1 | Repeat-run tests with accepted tolerance policy; exact tolerances `TBD`. |
| DEL-04-05-RQ-003 | Performance and conditioning metrics shall be recorded without inventing release thresholds. | SOW-035; OI-005 | Review that thresholds remain `TBD` until approved. |
| DEL-04-05-RQ-004 | Fixture data shall be invented, public-permissive, or otherwise lawful and shall not contain protected standards examples, protected tables, vendor data, or proprietary commercial benchmark data. | OPS-K-IP-1; docs/DIRECTIVE.md data-boundary rules | Protected-content and provenance review. |
| DEL-04-05-RQ-005 | Harness outputs shall preserve solver version, model/hash basis where available, warning/diagnostic classes, assumptions, provenance notes, and limitations. | AB-00-06; OPS-K-REPORT-1 | Result-envelope/report-facing tests once result schema exists. |
| DEL-04-05-RQ-006 | Unit-sensitive fixture inputs and outputs shall pass the accepted unit-system/dimensional checks. | OPS-K-UNIT-1 | Unit validation tests once fixture schema exists. |
| DEL-04-05-RQ-007 | The harness shall remain mechanics-only and shall not claim professional approval, certification, or code compliance. | OPS-K-MECH-1; OPS-K-AGENT-4; OPS-K-REPORT-1 | Review of result labels, generated reports, and release notes. |

## Standards

No protected standard text, proprietary benchmark suite, or vendor dataset is available in this deliverable-local setup context. Any future benchmark basis must be recorded with provenance, redistribution status, and human/IP review disposition. Clause-level or vendor-specific requirements are `TBD`.

## Verification

| Verification area | Minimum setup expectation |
|---|---|
| Determinism | Define repeat-run evidence without setting unsupported numeric thresholds. |
| Sparse performance | Capture timing/memory/scale observations once solver implementation exists; target values remain `TBD`. |
| Conditioning | Record conditioning-related diagnostics or solver-status observations when supported by the solver/result envelope. |
| Data boundary | Confirm all fixtures are invented, public-permissive, or otherwise lawful. |
| Reporting | Confirm harness output does not imply certification, code compliance, or professional acceptance. |

## Documentation

Expected future artifacts, when implementation is authorized, are:

- performance tests;
- benchmark harness;
- benchmark fixture provenance notes;
- deterministic run records or comparable result snapshots.

The exact module paths, runner command, fixture schema, metrics, and CI gates are `TBD` and must not be resolved by this setup pass.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A |
