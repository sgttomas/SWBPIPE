# References: DEL-13-03 Constraint validation engine

## Governing References
- INIT.md - Root bootstrap, reading order, and agent constraints.
- AGENTS.md - OpenPipeStress agent index and dispatch rules.
- /Users/ryan/ai-env/projects/chirality/agents/AGENT_TASK.md - Canonical bounded TASK shell, write authorization, and run-record protocol.
- docs/README.md - Governance document map and status.
- docs/DIRECTIVE.md - Founding intent, product boundaries, and stop rules.
- docs/CONTRACT.md - Invariant catalog.
- docs/TYPES.md - Identifier, lifecycle, and vocabulary definitions.
- docs/SPEC.md - Technical and agentic implementation specification.
- docs/IP_AND_DATA_BOUNDARY.md - Protected-data and public/private data boundary policy.
- agents/AGENT_PREPARATION.md - PREPARATION scaffolding protocol and file schemas.

## Decomposition and Registers
- execution/_Decomposition/SOFTWARE_DECOMP.md - Accepted revision 0.7 current decomposition basis; package PKG-13 and deliverable DEL-13-03.
- docs/_Registers/Deliverables.csv - Deliverable identity, description, artifacts, scope, objectives, and context envelope.
- docs/_Registers/ScopeLedger.csv - Scope item mapping for SOW-068.
- docs/_Registers/ContextBudgetQA.csv - Context budget row for DEL-13-03.

## DAG and Coordination References
- execution/_DAG/DAG-006/APPROVAL_RECORD.md - Approved revision 0.7 active-edge coordination basis.
- execution/_DAG/DAG-006/DependencyEdges.csv - Aggregate dependency source of truth.
- execution/_DAG/DAG-006/DeliverableNodes.csv - Approved deliverable node context.
- execution/_Coordination/_COORDINATION.md - Durable coordination rulings.
- execution/_Coordination/NEXT_INSTANCE_PROMPT.md - Current stable entry and discovery prompt.
- execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md - SCA-002 acceptance record.
- plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md - Downstream refresh plan.

## Implementation Evidence
- core/constraints/validation/engine.py - Implemented deterministic constraint validation engine and diagnostic record shape.
- core/constraints/validation/__init__.py - Package export surface for `Diagnostic`, `ValidationResult`, `diagnostic_dicts`, and `validate_constraint_envelope`.
- tests/test_constraint_validation.py - Focused validation diagnostics tests and invented public fixtures.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/RUN_2026-05-04_IMPLEMENTATION.md - Historical implementation run notes and verification record.

## Upstream Read-Only Context
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Datasheet.md - Current constraint schema and provenance evidence summary.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Specification.md - Current constraint entity/provenance requirements and verification evidence.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Guidance.md - Current upstream guidance and downstream `TBD` boundaries.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Procedure.md - Current upstream schema/provenance procedure.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/MEMORY.md - Current upstream memory, including the 2026-06-07 Worker A evidence refresh addendum.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/_run_records/TASK_RUN_2026-06-07_1127.md - Worker A TASK run evidence for DEL-13-02 stale evidence refresh.

## Notes
- Missing workflow and handoff-state reference entries were removed on 2026-06-07 because those files are not present in the current workspace.
- Current implementation evidence exists in `core/constraints/validation/engine.py` and `tests/test_constraint_validation.py`; this reference refresh does not make release, lifecycle, professional, or human-acceptance claims.
- The quarantined Chirality app corpus is not a source for this control surface.
