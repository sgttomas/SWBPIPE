# Specification: DEL-09-01 Mechanics benchmark suite

## Scope

This deliverable specifies setup evidence for a future mechanics benchmark suite covering cantilevers, frames, thermal growth, imposed displacement, and stiffness transforms.

This setup pass does not implement benchmark source files, add tests, create numerical hand-calculation cases, modify solver code, edit validation directories outside this deliverable, choose final tolerances, or introduce protected/proprietary benchmark data.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-09-01-RQ-001 | The benchmark suite shall remain separate from solver implementation and shall not modify production solver behavior. | DEL-09-01 context; AB-00-02; docs/SPEC.md section 9 | Module boundary review once implementation exists. |
| DEL-09-01-RQ-002 | The suite shall cover cantilever, frame, thermal-growth, imposed-displacement, and stiffness-transform benchmark families. | DEL-09-01 context; docs/VALIDATION_STRATEGY.md section 2 | Fixture inventory review against required families. |
| DEL-09-01-RQ-003 | Benchmark sources and expected-result derivations shall be original, public-domain, public-permissive, or otherwise documented with redistribution rights. | SOW-026; docs/VALIDATION_STRATEGY.md section 5; OPS-K-IP-1..3 | Protected-content/provenance review before publication. |
| DEL-09-01-RQ-004 | Benchmark inputs and expected outputs shall be unit-aware and dimensionally checked. | OPS-K-UNIT-1; docs/SPEC.md sections 4.1 and 4.2 | Unit validation tests once fixture schema exists. |
| DEL-09-01-RQ-005 | The suite shall preserve solver diagnostics, result-envelope status, solver version, assumptions, provenance, and limitations where supported by implementation contracts. | AB-00-06; docs/SPEC.md sections 4.5 and 9 | Result-envelope comparison and diagnostic review. |
| DEL-09-01-RQ-006 | The setup artifacts shall not set final numerical tolerances, release thresholds, or benchmark pass/fail authority without human approval. | OI-005; OPS-K-AGENT-1..4 | Review confirms unresolved tolerances remain `TBD`. |
| DEL-09-01-RQ-007 | The suite shall remain mechanics verification support and shall not claim certification, code compliance, professional approval, or project-specific reliance. | OPS-K-AUTH-1; docs/VALIDATION_STRATEGY.md section 1 | Review of result labels, reports, and release notes. |

## Standards

No protected standard text, proprietary benchmark suite, commercial software example, or vendor dataset is available in this deliverable-local setup context. Any future mechanics case must record source, provenance, license/redistribution status, contributor certification, and review disposition before it is accepted as a public benchmark.

Clause-level or code-specific validation requirements are `TBD` and must not be inferred from protected standards.

## Verification

| Verification area | Minimum setup expectation |
|---|---|
| Required family coverage | The future fixture inventory explicitly maps each case to one or more required benchmark families. |
| Analytical derivation | Hand-calculation notes identify assumptions, units, equations used from public/original mechanics, and expected result fields. |
| Numerical comparison | Tolerance policy is cited when approved; until then each tolerance field remains `TBD`. |
| Unit safety | Inputs, expected values, and outputs are dimensionally checked under the accepted unit system. |
| Data boundary | Fixture provenance confirms original/public/permissive status and excludes protected standards/commercial examples. |
| Diagnostics/result envelopes | Benchmark outputs preserve warnings, diagnostics, solver version, and limitations without compliance claims. |

## Documentation

Expected future artifacts, when implementation is authorized, are:

- `validation/benchmarks/mechanics` fixture set;
- hand-calculation notes for each benchmark family;
- fixture provenance and redistribution-status index;
- benchmark runner or integration notes;
- comparison-result records suitable for regression and release-gate review.

The exact module paths, fixture schema, runner command, result export format, numerical tolerances, and CI gates are `TBD` and must not be resolved by this setup pass.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A |
