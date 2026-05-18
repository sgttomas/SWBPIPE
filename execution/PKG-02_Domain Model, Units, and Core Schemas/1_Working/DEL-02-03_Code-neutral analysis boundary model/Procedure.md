# Procedure: DEL-02-03 Code-neutral Analysis Boundary Model

## Purpose

Define a repeatable procedure for producing and checking the `DEL-02-03` boundary model. The procedure is for deliverable drafting and later implementation planning; it does not perform mechanics solving, rule-pack evaluation, GUI development, or human professional acceptance.

## Prerequisites

- Read `_CONTEXT.md` revision 0.4 for deliverable identity, scope, artifacts, and SCA-001 architecture-basis injection.
- Confirm `_STATUS.md` allows editing under the active task brief. Reread the current state for each run; do not carry forward stale prior-run state claims.
- Read `_REFERENCES.md` and use accessible local sources.
- Read `docs/_Registers/Deliverables.csv` row `DEL-02-03`, `docs/_Registers/ScopeLedger.csv` row `SOW-002`, and `docs/_Registers/ContextBudgetQA.csv` row `DEL-02-03`.
- Read `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 sections for objectives, `PKG-02`, `DEL-02-03`, SCA-001 basis, and `SOW-002`.
- Read governing source slices: `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/DIRECTIVE.md`, `docs/SPEC.md`, `docs/PRD.md` sections 6.1/6.2/12/17.4, `docs/INTENT.md` rule-pack/mechanics boundary, and `docs/IP_AND_DATA_BOUNDARY.md`.
- Confirm no human-owned upstream dependency list blocks the work. `_DEPENDENCIES.md` states dependencies are coordinated externally and not tracked in this folder.

## Steps

1. Confirm scope.
   - Use `DEL-02-03`, `PKG-02`, `SOW-002`, and anticipated artifacts from `_CONTEXT.md` and the registers.
   - Do not expand into solver, GUI, report, or rule-pack implementation deliverables.

2. Establish the boundary actors.
   - Mechanics solver: computes mechanical results only.
   - Rule-pack evaluator: evaluates user-supplied acceptability checks.
   - Human reviewer: records project-specific acceptance outside solver authority.

3. Establish the status vocabulary.
   - Start from `docs/TYPES.md` section 4.
   - Include only the listed statuses unless the human project authority later amends the vocabulary.
   - Do not introduce `CODE_COMPLIANT` as an automatic status.

4. Define status evidence.
   - For `MODEL_INCOMPLETE`, require solve-blocking missing-data evidence.
   - For `MECHANICS_SOLVED`, require a solver result envelope.
   - For `RULE_INPUTS_INCOMPLETE`, require rule-pack missing-input evidence.
   - For `USER_RULE_CHECKED` or `USER_RULE_FAILED`, require rule-pack identity, version/checksum, and evaluation evidence.
   - For `HUMAN_APPROVED_FOR_PROJECT`, require a human acceptance record bound to specific model/rule/report evidence.

5. Apply data-boundary rules.
   - Do not include protected standards/code text, tables, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data, or commercial software examples.
   - Mark unknowns as `TBD`.
   - Label inferences as `ASSUMPTION`.

6. Apply architecture-basis constraints.
   - Preserve layer responsibilities and inward dependency direction from `AB-00-02`.
   - Preserve command/query/job/result-envelope separation from `AB-00-03`.
   - Use JSON Schema 2020-12 where a public schema artifact is later produced, per `AB-00-04` and `_CONTEXT.md`.
   - Carry diagnostics/provenance per `AB-00-06`.
   - Preserve no-bypass adapter/plugin boundaries per `AB-00-07`.
   - Plan verification hooks per `AB-00-08`.

7. Draft or update artifacts.
   - Draft `analysis_status` enum semantics.
   - Draft the `docs/SPEC.md` state-model text in deliverable-local form only unless a later task authorizes editing `docs/SPEC.md`.
   - Keep implementation-level choices marked `TBD`: exact schema file path, exact field layout, expression grammar/library, public API transport, and physical persistence container.

8. Run cross-document consistency checks.
   - Confirm Datasheet attributes are reflected in Specification requirements.
   - Confirm Specification requirements have Guidance rationale and Procedure verification hooks.
   - Confirm terminology is consistent: mechanics solve, user-rule check, human professional acceptance, user-supplied rule pack, protected standards data, and `analysis_status`.
   - Confirm no numeric engineering values or protected source content were introduced.

## Verification

| Check | Expected result |
|---|---|
| Scope check | All content remains about `DEL-02-03` and does not implement adjacent deliverables. |
| Status vocabulary check | Status names match `docs/TYPES.md` section 4; `CODE_COMPLIANT` is absent as an automatic status. |
| Authority check | Solver, rule-pack, and human acceptance authority are separately described. |
| Data-boundary check | No protected standards/code data, code tables, copied formulas, proprietary examples, or certification/compliance claims are introduced. |
| TBD/ASSUMPTION check | Unknown implementation details remain `TBD`; inferred design choices are labeled `ASSUMPTION`. |
| Architecture-basis check | Applicable SCA-001 basis IDs are referenced as constraints without copying full PKG-00 prose. |
| Cross-document check | Datasheet, Specification, Guidance, and Procedure use consistent terms and status meanings. |

## Records

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_STATUS.md` safe transition record, when state permits.
- `_run_records/TASK_RUN_*.md`
- Future record target: authorized update to `docs/SPEC.md` state model, if authorized by a separate task or change process.
