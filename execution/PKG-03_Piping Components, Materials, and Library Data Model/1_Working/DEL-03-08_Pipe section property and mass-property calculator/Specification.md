# Specification: DEL-03-08 Pipe section property and mass-property calculator

## Scope

This deliverable specifies setup evidence for a backend feature slice that will calculate pipe section and mass properties from user-entered dimensions and material data with unit checks. It also records the private-library and provenance constraints that apply when those inputs are sourced from pipe section, component, or material libraries.

This setup pass does not implement code, edit repo-level schemas, add test fixtures, or introduce any protected dimensional, material, contents, insulation, corrosion, code, or vendor data.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-03-08-RQ-001 | The calculator shall accept only explicit user-entered or lawfully imported private/project input values for dimensions, material density, contents, insulation, and corrosion basis. | SOW-051; OPS-K-DATA-1; OPS-K-IP-1 | Unit/input validation tests with synthetic or user-provided data only. |
| DEL-03-08-RQ-002 | The calculator shall reject or flag missing solve-required values instead of applying silent defaults. | OPS-K-DATA-2; SOW-051 | Negative tests for missing OD, wall, density, unit, and applicable mass contributors; exact cases `TBD`. |
| DEL-03-08-RQ-003 | All input quantities, intermediate calculations, and outputs shall be unit-aware and dimensionally checked. | SOW-051; OPS-K-UNIT-1; OBJ-012 | Unit tests against the accepted unit-system contract; exact dimensions and tolerances `TBD`. |
| DEL-03-08-RQ-004 | Library-sourced inputs shall carry provenance and redistribution status through calculator schema hooks. | SOW-018; OPS-K-DATA-3 | Schema validation tests once schema hooks are defined. |
| DEL-03-08-RQ-005 | Public repository fixtures shall not encode protected pipe dimensional tables, protected material data, proprietary commercial data, or paraphrased protected tables. | OPS-K-IP-1; OPS-K-IP-3 | Protected-content review and fixture review. |
| DEL-03-08-RQ-006 | Calculator outputs intended for solver consumption shall remain code-neutral and shall not claim certification, code compliance, or professional approval. | OPS-K-AGENT-4; AB-00-06 | Review of diagnostics/result envelopes and report-facing text. |
| DEL-03-08-RQ-007 | The calculator shall be isolated from global solver implementation and rule-pack compliance logic. | PKG-03 exclusion; OPS-K-SOLVER-1 | Module boundary review and dependency tests once implementation exists. |

## Standards

No protected standard text or table is available in this deliverable-local setup context. Any future standard, code, or vendor basis must be introduced only as a licensed/private input or as a non-protected pointer with provenance. Clause-level requirements are `TBD`.

## Verification

| Verification area | Minimum setup expectation |
|---|---|
| Unit safety | Tests must demonstrate deterministic unit conversion/dimensional rejection through the accepted unit core. |
| Missing input behavior | Tests must show explicit findings for missing required values; no silent defaults. |
| Provenance | Tests must preserve source/provenance/redistribution metadata when inputs originate from libraries/imports. |
| IP/data boundary | Test fixtures must be synthetic or otherwise cleared for redistribution. |
| Solver boundary | Tests must assert calculator outputs are data/service outputs, not solver certification or code-compliance claims. |

## Documentation

Expected future artifacts, when implementation is authorized, are:

- section property calculator;
- mass property tests;
- schema hooks.

The exact module paths, schema paths, and test filenames are `TBD` and must be resolved without editing repo-level artifacts during this setup pass.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A |
