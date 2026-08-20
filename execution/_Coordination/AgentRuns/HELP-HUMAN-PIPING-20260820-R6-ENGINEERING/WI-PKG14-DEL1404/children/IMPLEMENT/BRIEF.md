# Sealed TASK brief — N3.IMPLEMENT

RequestedBy: `WI-PKG14-DEL1404`
RunID: `HELP-HUMAN-PIPING-20260820-R6-ENGINEERING`
ParentInstanceID: `WI-PKG14-DEL1404`
ChildInstanceID: `TASK-PKG14-DEL1404-CATEGORIES`
PackageID: `PKG-14`
DeliverableID: `DEL-14-04`
TaskSkill: `software-bounded-implementation`
ApplyEdits: `true`

Objective: implement the minimum coherent product/test change that makes every currently supported analysis-run comparison result family explicit, exercised, deterministic, and independently bound without inventing a canonical comparison-result/export schema.

AcceptedBasis: HEAD `357a58b56726feba49507534159c3fbc4656b818`; DAG-009; target R5; frozen parent activation and graph; current engine and DEL-14-04 `SOW_V1` kit.

Dependencies: DAG-unblocked; independent of parent run N1/N2.

DeclaredReads: root/project instructions; `docs/SOFTWARE_WORKFLOW_PROFILE.md`; `projects/chirality-piping/software-workflow.json`; frozen activation/graph; DEL-14-04 ScopeOfWork, `_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`; `core/comparison/analysis_run/engine.py`; `tests/test_analysis_run_comparison.py`; only directly relevant imported comparison contracts if needed.

AllowedTools: read, rg, apply_patch, focused pytest, registered software workflow selection/check/scope tools.

AllowedWriteTargets:

- `projects/chirality-piping/core/comparison/analysis_run/**`
- `projects/chirality-piping/tests/test_analysis_run_comparison.py`
- this child evidence folder

AcceptanceCriteria:

- Every family in `SUPPORTED_RESULT_FAMILIES` (`displacement`, `rotation`, `force`, `moment`, `reaction`, `stress`, `ratio`) has focused comparison evidence.
- Result bindings are independently addressable by family and deterministic, while preserving aggregate `result_deltas` compatibility.
- Unsupported/mismatched and missing categories fail or report explicitly under accepted diagnostics.
- No tolerance/default/policy, result/export schema, validation/suitability, lifecycle, professional-claim, register, decision, DAG, decomposition, or PRD change.
- Focused `python3 -m pytest -q tests/test_analysis_run_comparison.py` passes.
- Return exact paths, check results, write containment, residual risks, blockers, and review handoff notes.

Escalation: stop and return if the objective requires a new canonical output schema, tolerance or validation policy, cross-package changes, desktop/UI writes, or any write outside the targets above.
