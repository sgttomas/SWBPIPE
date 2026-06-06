# Procedure: DEL-05-01 Primitive load case engine

## Purpose

Describe bounded steps for maintaining and verifying the primitive load case engine evidence without introducing code-specific load combinations, protected standards data, final result-envelope/API behavior, or compliance claims.

## Prerequisites

- Sealed deliverable scope for DEL-05-01 and SOW-013.
- Applicable architecture basis IDs AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Invariants OPS-K-MECH-1, OPS-K-MECH-2, OPS-K-UNIT-1, OPS-K-SOLVER-1, OPS-K-DATA-1, OPS-K-DATA-2, OPS-K-AGENT-1 through OPS-K-AGENT-4, and OPS-K-IP-1.
- Current implementation evidence in `core/loads/primitive_loads/README.md` and `core/loads/primitive_loads/src/lib.rs`.
- Explicit human approval before any future scope adds code-specific combinations, coefficients, allowables, default environmental factors, dynamic procedure generation, final compliance claims, or professional reliance claims.

## Steps

1. Confirm the deliverable identity, package, scope item, objective, architecture basis, and current lifecycle state from `_CONTEXT.md` and `_STATUS.md`.
2. Read the deliverable-local truth set before edits: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `MEMORY.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
3. Read the implementation evidence in `core/loads/primitive_loads/README.md` and `core/loads/primitive_loads/src/lib.rs`.
4. Confirm all SOW-013 primitive categories remain represented: weight, pressure, thermal expansion, imposed displacement, hydrotest, wind, seismic, and occasional.
5. Confirm primitive mechanics boundaries: nodal, element-uniform, imposed-displacement, equivalent-static preparation, lumped equivalent nodal conversion, straight-pipe axial effects, and solver load-vector assembly are documented only to the extent implemented.
6. Preserve the boundary between primitive load definitions/load-case records in DEL-05-01 and mixed-category load-case algebra/user combinations in DEL-05-02.
7. Preserve the boundary between mechanics solving, diagnostic record transport, rule/compliance evaluation, and human professional judgment.
8. Keep unknown coefficients, default magnitudes, conversion constants, dynamic treatment, property/default sourcing, production tolerance policy, final API integration, release thresholds, and professional reliance as `TBD` unless supplied by lawful source material and sealed scope.
9. Run focused validation after documentation or crate-affecting changes: `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` and `git diff --check` scoped to touched files where possible.
10. Record durable closeout evidence in `MEMORY.md` and `_run_records/TASK_RUN_*.md` without editing `_STATUS.md`, dependency registers, DAG artifacts, schemas, repo-level governance, or core code unless explicitly authorized by a later brief.

## Verification

| Check | Expected evidence |
|---|---|
| Scope check | Documents reference DEL-05-01, PKG-05, SOW-013, OBJ-003, and the current architecture basis consistently. |
| Category check | All eight primitive load categories appear in implementation-aligned evidence. |
| Boundary check | No code-specific load combinations, allowables, factors, coefficients, default magnitudes, dynamic procedure generators, or certification claims are introduced. |
| Unit/boundary metadata check | Documents preserve explicit unit/dimension/provenance/hash-ref boundaries and do not claim conversion constants. |
| Load-case record check | Documents describe single-category primitive load-case records and keep mixed-category algebra in DEL-05-02. |
| Diagnostic bridge check | Documents describe local diagnostic bridge records without claiming a shared enum or final result-envelope/API implementation. |
| Equivalent-static check | Documents limit wind, seismic, and occasional equivalent-static handling to explicit mechanics loads with caller-supplied basis/provenance refs and no dynamic procedure generation. |
| Lumping check | Documents limit lumping to explicit translational/global `ForcePerLength` element loads with caller-supplied spans/connectivity. |
| Axial-effect check | Documents limit axial effects to thermal and pressure mechanics helpers from caller-supplied properties without code stress/compliance claims. |
| Test check | `cargo test --manifest-path core/loads/primitive_loads/Cargo.toml` passes or any failure is recorded with boundaries. |
| Diff hygiene check | `git diff --check` passes for touched files or any failure is recorded. |
| Status check | `_STATUS.md` remains unchanged unless a later brief explicitly authorizes a lifecycle update. |

## Records

- Four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`
- Working memory: `MEMORY.md`
- Semantic evidence: `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`
- Dependency evidence: `Dependencies.csv`, `_DEPENDENCIES.md`
- Implementation evidence: `core/loads/primitive_loads/README.md`, `core/loads/primitive_loads/src/lib.rs`
- Run evidence: `_run_records/`
- Lifecycle evidence: `_STATUS.md`
