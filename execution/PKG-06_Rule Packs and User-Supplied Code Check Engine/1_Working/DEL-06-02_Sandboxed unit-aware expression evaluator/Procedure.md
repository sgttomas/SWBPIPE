# Procedure: DEL-06-02 Sandboxed unit-aware expression evaluator

## Purpose

Describe the setup-to-implementation procedure for a future sandboxed, unit-aware, declarative rule-pack expression evaluator. This procedure is operational guidance only; it does not implement the evaluator in this setup run.

## Prerequisites

| Prerequisite | Status |
|---|---|
| Sealed deliverable context for DEL-06-02 | Present in `_CONTEXT.md`. |
| Governing invariants and data boundary read | Completed for this setup run. |
| Four-document setup kit | Produced by TASK+four-documents. |
| Semantic matrix and lensing artifacts | Produced by TASK+semantic-matrix-build and TASK+lens-register. |
| Dependency register | Produced by TASK+dependency-extract. |
| Expression grammar/library decision | TBD; required before implementation. |
| Unit-system integration contract | Interface dependency on DEL-02-02 or equivalent sealed unit contract. |
| Rule-pack schema interface | Interface dependency on DEL-06-01 or equivalent sealed schema contract. |

## Steps

1. Confirm the active sealed brief names only DEL-06-02, PKG-06, SOW-045, OBJ-005, applicable invariants, and explicit write scope.
2. Confirm no protected standards text, protected formulas, proprietary values, owner standards, private rule packs, or commercial examples are being introduced.
3. Before implementation, obtain a human architecture decision for the expression grammar/library and record any security constraints or rejected options.
4. Define the accepted declarative expression surface without host-language execution, imports, reflection, filesystem access, network access, process execution, or hidden side effects.
5. Bind variables only from declared rule-pack variables, solver result fields, and user-supplied design-basis inputs allowed by the sealed schema/interface contract.
6. Apply unit and dimensional checks to expression operands, comparisons, and outputs using the sealed unit-system contract.
7. Emit deterministic diagnostics for invalid expressions, unsupported constructs, missing variables, missing rule-check inputs, unit mismatches, and boundary violations.
8. Preserve state semantics: mechanics solved, rule inputs incomplete, user-rule checked, user-rule failed, and human review required are distinct outcomes.
9. Add tests for unsafe expression rejection, unit mismatch, missing binding, deterministic diagnostics, public/private data boundaries, and adapter/plugin bypass prevention.
10. Route implementation and test evidence to REVIEW before any lifecycle transition beyond development draft states.

## Verification

| Check | Evidence |
|---|---|
| Four-document kit exists | `tools/validation/check_four_documents.sh <deliverable path>` |
| Semantic artifact exists and audit states PASS | `_SEMANTIC.md#Audit-Result` |
| Lensing coverage exists for matrices A, B, C, F, D, X, and E | `_SEMANTIC_LENSING.md` |
| Dependency register is schema-valid | `python3 tools/validation/validate_dependencies_schema.py <deliverable path>/Dependencies.csv` |
| Status value is valid | `python3 tools/validation/validate_enum.py LIFECYCLE_STATE SEMANTIC_READY` |
| No ISSUED transition occurred | `_STATUS.md` current state remains `SEMANTIC_READY` |

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/TASK_RUN_2026-04-30_1032_four-documents-p1-p2.md`
- `_run_records/TASK_RUN_2026-04-30_1032_semantic-matrix-build.md`
- `_run_records/TASK_RUN_2026-04-30_1032_lens-register.md`
- `_run_records/TASK_RUN_2026-04-30_1032_four-documents-p3.md`
- `_run_records/TASK_RUN_2026-04-30_1032_dependency-extract.md`
