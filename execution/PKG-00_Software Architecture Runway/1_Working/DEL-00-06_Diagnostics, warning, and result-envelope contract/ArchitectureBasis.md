<!-- chirality-architecture-basis/v1 -->
# DEL-00-06 — Diagnostics, warning, and result-envelope contract — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-06
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Defines the diagnostic taxonomy, warning-class contract, result-envelope contract, failure-state mapping, and reporting/UI handoff shared by solver, rule packs, GUI, CLI, reports, storage, and adapters. Diagnostic, warning, status, and result-envelope contracts now have schemas, consumers, and verification evidence in the implemented slice (D-41 R5 T7 PDU-054 current declaration); producer coverage and broader integration exist only where explicitly recorded. This member supplies the contract those surfaces must preserve; it does not itself implement schemas, exception types, solver diagnostics, GUI rendering, or report output.

Identity: Deliverable DEL-00-06, PKG-00 Software Architecture Runway, type DATA_MODEL_CHANGE, scope item SOW-061, objective OBJ-013.

## Normative requirements

| ID | Current normative text | Basis |
|---|---|---|
| REQ-06-01 | Define machine-readable diagnostic fields for code, class, severity, source, affected object, message, remediation, and provenance where applicable. | Kit Specification; AB-00-06 row, SOFTWARE_DECOMP rev 0.9 |
| REQ-06-02 | Preserve the six SPEC warning classes — SOLVE_BLOCKING, RULE_CHECK_BLOCKING, PROVENANCE_WARNING, ASSUMPTION_WARNING, NONLINEAR_WARNING, IP_BOUNDARY_WARNING — as refined by the accepted TP-DIAG-019 ruling: per-boundary local class vocabularies are permitted with a defined mapping rule to the six SPEC classes. Diagnostics crossing a boundary preserve original `code`, `source`, `affected_object`, `message`, `remediation`, and provenance; only the class vocabulary maps to the receiving boundary where a schema requires local classes (e.g., runner `RUNNER_BLOCKING` → export `EXPORT_BLOCKING`; runner `PRIVACY_WARNING` → export `IP_BOUNDARY_WARNING`). All diagnostic fields other than class are always preserved. | Kit Specification as refined by TP-DIAG-019 (accepted ruling recorded in this member's `MEMORY.md`, 2026-05-17; run record `_run_records/TASK_RUN_2026-05-17_TP-DIAG-019.md`; boundary reconciliation TP-VERIFY-013E) |
| REQ-06-03 | Solver, rule-pack, GUI, CLI, report, storage, and adapter outputs use result envelopes for nontrivial operations. | Kit Specification; AB-00-06 row |
| REQ-06-04 | Distinguish invalid input, incomplete model, mechanics result, rule-check result, and human-review-needed states. | Kit Specification; AB-00-06 row |
| REQ-06-05 | Diagnostics must not claim certification, code compliance, approval, or professional authentication. | Kit Specification; AB-00-06 row; `OPS-K-AUTH-1` |

Sanctioned additive classes (extensions, not contradictions of REQ-06-02): the implemented warning-class enum in `schemas/analysis_run.schema.json` additionally carries `UNIT_WARNING`, `RUN_REPRODUCIBILITY_WARNING`, and `RESULT_HASH_WARNING` alongside the six SPEC classes.

Cross-cutting invariants carried unchanged from the kit: `OPS-K-IP-1` (no protected standards text, tables, formulas, allowables, or proprietary data in public artifacts), `OPS-K-DATA-2` (missing solve/rule-check-required values remain explicit findings, never silent defaults), `OPS-K-AUTH-1` (no certification/sealing/approval/code-compliance claims), `OPS-K-MECH-1` (global analysis is a 3D centerline/frame model; local FEA is a handoff path), `OPS-K-AGENT-1` (unknown facts become `TBD`), `OPS-K-AGENT-3` (Type 2 execution stays within sealed scope).

Downstream consumers use this member as AB-00-06 architecture-basis context through sealed briefs and governed dispatch surfaces (declared consumers include PKG-04 through PKG-10 and PKG-12 per the AB-00-06 row); that consumption does not make PKG-00 ISSUED, and it does not authorize PKG-01 through PKG-17 work from within this member.

## Resolved decisions (former TBD and human-ruling queue)

Register: `execution/_Coordination/_DECISIONS/_REGISTER.md`; codification in `execution/_Decomposition/SOFTWARE_DECOMP.md` §8.4 (~lines 585–640).

| Former open item | Resolution | Record |
|---|---|---|
| Uniform six-class preservation at every boundary | Refined: per-boundary local class vocabularies with the defined mapping rule to the six SPEC classes; no shared diagnostic enum was introduced in that tranche. Folded into REQ-06-02 above as current normative text. | TP-DIAG-019 (accepted, 2026-05-17; `MEMORY.md`; run record `TASK_RUN_2026-05-17_TP-DIAG-019.md`) |
| Boundary-vocabulary alignment verification | Reconciled TP-DIAG-019 mapping rules against result-export, headless-runner, and physical-to-analytical adapter vocabularies; remaining crossings classified `READY_FOR_RUNTIME_TRACE_TRANCHE`, `READY_FOR_SECTION_EVIDENCE_SCHEMA_TRANCHE`, `READY_FOR_AUDIT_CANONICALIZATION_RULING`. | TP-VERIFY-013E (2026-05-17; `MEMORY.md`; run record `TASK_RUN_2026-05-17_TP-VERIFY-013E.md`) |
| Schema syntax / machine-readable envelope format (setup-era TBD) | JSON Schema 2020-12 with schema-first command/query/job/result envelopes, per the resolved architecture baseline; realized as the implemented schemas below. | SCA-001 baseline (SOFTWARE_DECOMP rev 0.9 §8.2) |
| Setup-era "contracts not yet implemented" framing | Superseded as current declaration by the implemented schemas, consumers, and verification evidence; earlier wording is historical setup context only. | D-41 R5 T7 PDU-054 (3 cited declaration claims reconciled 2026-07-12) |

## Realized artifacts

| Anticipated by the kit | Realized state | Owner |
|---|---|---|
| `docs/architecture/diagnostics_contract.md` | Never created under that name. Status/diagnostic semantics realized as `docs/architecture/analysis_status_semantics.md`. | DEL-05-04 |
| Standalone result-envelope schema | Never created under that name. Realized as `schemas/operation_outcome.schema.json`. | DEL-10-03 |
| Warning-class contract | Realized as the warning-class enum in `schemas/analysis_run.schema.json` (six SPEC classes plus the sanctioned additive classes noted above). | Implementation surface owned by its package; this member remains the class-contract basis |

## Open holds and routed questions

- OPEN: diagnostic code namespace and severity-taxonomy details (kit TBDs never ruled; remain routed to human architecture ruling).
- OPEN: localization policy (kit TBD never ruled).
- OPEN: shared diagnostic enum, release/acceptance/professional-reliance diagnostic policy — classified `KEEP_AS_TBD` by TP-VERIFY-013E; audit checksum diagnostic policy held at `READY_FOR_AUDIT_CANONICALIZATION_RULING`.
- Deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining` (currently empty). Nothing in this document implies lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054 currentness declarations, 2026-07-12) and the accepted TP-DIAG-019 / TP-VERIFY-013E rulings. Prior wording is preserved in git history; `MEMORY.md` is retained unchanged as the dated decision/evidence trail. Current upstream authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9 with approved `execution/_DAG/DAG-007/` graph context.
