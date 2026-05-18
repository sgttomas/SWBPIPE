# Procedure: DEL-05-01 Primitive load case engine

## Purpose

Describe setup-safe steps for producing and later using the primitive load case engine evidence without implementing the engine or introducing code-specific load combinations.

## Prerequisites

- Sealed deliverable scope for DEL-05-01 and SOW-013.
- Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Invariants OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-AGENT-1 through OPS-K-AGENT-4, and OPS-K-IP-1.
- Explicit human approval before any future scope adds code-specific combinations, coefficients, allowables, default environmental factors, or certification claims.

## Steps

1. Confirm the deliverable identity, package, scope item, and objective from `_CONTEXT.md` and the decomposition/register rows.
2. Record the primitive load categories from SOW-013: weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional.
3. For each category, keep missing implementation parameters, coefficients, default magnitudes, and dynamic methods as `TBD` unless provided by lawful source material and sealed scope.
4. Preserve the boundary between primitive load definitions in DEL-05-01 and load-case algebra/user combinations in DEL-05-02.
5. Preserve the boundary between mechanics solving and rule/compliance evaluation.
6. Plan future deterministic load application tests without writing product code during setup.
7. Record diagnostics/provenance expectations for missing or invalid load inputs using AB-00-06 result-envelope concepts.

## Verification

| Check | Expected evidence |
|---|---|
| Scope check | Documents reference DEL-05-01, PKG-05, SOW-013, and OBJ-003 consistently. |
| Category check | All eight primitive load categories appear in setup evidence. |
| Boundary check | No code-specific load combinations, allowables, factors, coefficients, default magnitudes, or certification claims are introduced. |
| Unit check | Unit-aware/dimensional validation is identified as a future requirement. |
| Test check | Deterministic load application tests are identified as anticipated artifacts, not implemented. |
| Status check | `_STATUS.md` remains safe and not `ISSUED`. |

## Records

- Four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- Semantic evidence: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
- Dependency evidence: `Dependencies.csv`, `_DEPENDENCIES.md`
- Run evidence: `_run_records/`
- Lifecycle evidence: `_STATUS.md`
