# References: DEL-13-04 Physical-to-analytical transformation contract

## Governing References
- INIT.md - Root bootstrap, reading order, and agent constraints.
- AGENTS.md - OpenPipeStress agent index and dispatch rules.
- /Users/ryan/ai-env/projects/chirality/agents/AGENT_TASK.md - Canonical bounded TASK shell, write-scope, and run-record requirements.
- docs/README.md - Governance document map and status.
- docs/DIRECTIVE.md - Founding intent, product boundaries, and stop rules.
- docs/CONTRACT.md - Invariant catalog.
- docs/TYPES.md - Identifier, lifecycle, and vocabulary definitions.
- docs/SPEC.md - Technical and agentic implementation specification.
- docs/IP_AND_DATA_BOUNDARY.md - Protected-data and public/private data boundary policy.
- agents/AGENT_PREPARATION.md - PREPARATION scaffolding protocol and file schemas.

## Decomposition and Registers
- execution/_Decomposition/SOFTWARE_DECOMP.md - Accepted revision 0.7 current decomposition basis; package PKG-13 and deliverable DEL-13-04.
- docs/_Registers/Deliverables.csv - Deliverable identity, description, artifacts, scope, objectives, and context envelope.
- docs/_Registers/ScopeLedger.csv - Scope item mapping for SOW-066.
- docs/_Registers/ContextBudgetQA.csv - Context budget row for DEL-13-04.

## DAG and Coordination References
- execution/_DAG/DAG-006/APPROVAL_RECORD.md - Approved revision 0.7 active-edge coordination basis.
- execution/_DAG/DAG-006/DependencyEdges.csv - Aggregate dependency source of truth.
- execution/_Coordination/_COORDINATION.md - Durable coordination rulings.
- execution/_Coordination/NEXT_INSTANCE_PROMPT.md - Stable entry protocol and state-discovery instructions.
- execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md - SCA-002 acceptance record.
- plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md - Downstream refresh plan.

## Upstream Deliverable Evidence
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Datasheet.md - Current constraint entity/provenance evidence summary.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Specification.md - Current constraint schema requirements and verification mapping.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Guidance.md - Current constraint provenance and downstream-deferral guidance.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/Procedure.md - Current constraint schema verification procedure.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/MEMORY.md - Current DEL-13-02 implementation and 2026-06-07 refresh memory.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/_run_records/TASK_RUN_2026-06-07_1127.md - Worker A stale-evidence refresh record.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/Datasheet.md - Current validation-engine evidence summary.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/Specification.md - Current validation-engine requirements and verification mapping.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/Guidance.md - Current validation-engine boundary guidance.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/Procedure.md - Current validation-engine verification procedure.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/MEMORY.md - Current DEL-13-03 implementation and 2026-06-07 refresh memory.
- execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/_run_records/TASK_RUN_2026-06-07_1133.md - Worker B stale-evidence refresh record.

## Implementation and Test Evidence
- core/model_transform/physical_to_analytical/contract.py - Implemented provider-neutral physical-to-analytical transform contract.
- core/model_transform/physical_to_analytical/_solver_boundary_adapter.py - Internal analytical solver-boundary DTO adapter.
- fixtures/domain/invented_physical_source_of_truth_model.json - Canonical invented physical source-of-truth fixture used by DEL-13-04 tests.
- tests/test_physical_to_analytical_transform.py - Focused transform contract, warning/diagnostic, traceability, source-preservation, fixture, component, load, and authority-boundary tests.
- tests/test_analytical_solver_boundary_adapter.py - Focused internal adapter DTO, property-binding, load-diagnostic, orientation, hash/source-chain, and role-boundary tests.

## Notes
- This reference list reflects the 2026-06-07 Worker C stale-evidence refresh. Historical PREPARATION references remain background context, not the full current evidence set.
- The quarantined Chirality app corpus is not a source for this control surface.
