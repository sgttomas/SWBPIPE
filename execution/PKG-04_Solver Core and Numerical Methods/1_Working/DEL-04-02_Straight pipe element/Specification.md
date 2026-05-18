# Specification: DEL-04-02 Straight pipe element

## Scope

This deliverable specifies setup evidence for the future straight pipe element backend slice. It covers straight pipe local stiffness, section-property integration, weight hooks, and element force recovery within the global 3D centerline/frame solver architecture.

This setup pass does not implement solver code, edit repo-level modules, create test fixtures, introduce protected formulas or tables, or make certification/compliance claims.

## Requirements

| Req ID | Requirement | Source basis | Verification hook |
|---|---|---|---|
| DEL-04-02-RQ-001 | The straight pipe element shall fit the 3D centerline/frame solver model and shall not bypass the solver kernel boundary. | SOW-006; OPS-K-MECH-1; AB-00-02 | Architecture/module-boundary review once implementation paths are selected. |
| DEL-04-02-RQ-002 | Local stiffness behavior shall be derived only from open mechanics and user/project/lawfully imported input values. | SOW-006; OPS-K-IP-1; OPS-K-DATA-1 | Protected-content review and fixture review. |
| DEL-04-02-RQ-003 | Section-property integration shall require explicit section-property inputs or validated upstream calculations; missing solve-required properties shall produce explicit findings. | SOW-006; OPS-K-DATA-2; OPS-K-UNIT-1 | Negative tests for missing properties and units; exact cases `TBD`. |
| DEL-04-02-RQ-004 | Weight hooks shall expose the information needed for load-case application without silently applying hidden load defaults. | SOW-006; OPS-K-DATA-2; AB-00-03 | Load-interface tests once the primitive load contract is accepted. |
| DEL-04-02-RQ-005 | Element force recovery shall return unit-aware mechanical result components suitable for downstream stress recovery, without encoding code stress checks. | SOW-006; OPS-K-MECH-2; OPS-K-UNIT-1 | Solver result-envelope tests and downstream interface review. |
| DEL-04-02-RQ-006 | Solver changes shall include deterministic verification tests before release use. | OPS-K-SOLVER-1; AB-00-08 | Deterministic solver tests using synthetic or cleared inputs. |
| DEL-04-02-RQ-007 | Diagnostics shall use governed result-envelope concepts and shall not claim professional approval, certification, or compliance. | AB-00-06; OPS-K-AGENT-4 | Diagnostic/result-envelope review. |

## Standards

No protected standard text, protected formulas, protected dimensional tables, material allowables, or proprietary commercial data are available in this deliverable-local setup context. Any future code or standard basis must be introduced only as a lawful private/project input or non-protected pointer with provenance. Clause-level requirements are `TBD`.

## Verification

| Verification area | Minimum setup expectation |
|---|---|
| Solver boundary | Tests and reviews must show the element is a solver component, not a rule-pack or compliance component. |
| Unit safety | Tests must cover dimensional checking for section properties, stiffness-related inputs, weight-related inputs, and recovered forces. |
| Missing inputs | Tests must show explicit findings for missing solve-required values. |
| Force recovery | Tests must show deterministic recovered mechanical result components; exact benchmark cases are `TBD`. |
| IP/data boundary | Test data must be synthetic, public-domain, or otherwise cleared for redistribution. |

## Documentation

Expected future artifacts, when implementation is authorized, are:

- straight pipe element;
- solver tests.

The exact module paths, API names, solver numerical library, tolerances, and test filenames are `TBD` and must be resolved without changing repo-level artifacts during this setup pass.

## Conflict Table (for human ruling)

| Conflict ID | Issue | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified in setup evidence. | N/A | N/A |
