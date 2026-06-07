# References: DEL-13-02 Constraint entity and provenance model

## Governing References
- INIT.md - Root bootstrap, reading order, and agent constraints.
- AGENTS.md - OpenPipeStress agent index and dispatch rules for Type 2 TASK execution.
- /Users/ryan/ai-env/projects/chirality/agents/AGENT_TASK.md - Canonical bounded TASK shell, write-scope, and run-record protocol.
- docs/README.md - Governance document map and status.
- docs/DIRECTIVE.md - Founding intent, product boundaries, and stop rules.
- docs/CONTRACT.md - Invariant catalog.
- docs/TYPES.md - Identifier, lifecycle, and vocabulary definitions.
- docs/SPEC.md - Technical and agentic implementation specification.
- docs/IP_AND_DATA_BOUNDARY.md - Protected-data and public/private data boundary policy.
- agents/AGENT_PREPARATION.md - PREPARATION scaffolding protocol and file schemas.

## Decomposition and Registers
- execution/_Decomposition/SOFTWARE_DECOMP.md - Accepted revision 0.7 current decomposition basis; package PKG-13 and deliverable DEL-13-02.
- docs/_Registers/Deliverables.csv - Deliverable identity, description, artifacts, scope, objectives, and context envelope.
- docs/_Registers/ScopeLedger.csv - Scope item mapping for SOW-068, SOW-067.
- docs/_Registers/ContextBudgetQA.csv - Context budget row for DEL-13-02.

## DAG and Coordination References
- execution/_DAG/DAG-006/APPROVAL_RECORD.md - Approved revision 0.7 active-edge coordination basis.
- execution/_DAG/DAG-006/DependencyEdges.csv - Aggregate dependency source of truth.
- execution/_Coordination/_COORDINATION.md - Current coordination record, development loop, state-discovery rules, and bounded worker discipline.
- execution/_Coordination/NEXT_INSTANCE_PROMPT.md - Stable entry protocol for the next agent instance.
- execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md - SCA-002 acceptance record.
- plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md - Downstream refresh plan.

## Implementation Evidence
- schemas/constraint.schema.json - Implemented JSON Schema 2020-12 constraint entity and provenance contract for DEL-13-02.
- tests/test_constraint_schema.py - Focused stdlib structural checks for schema identity, required definitions, enum coverage, unit dimensions, data-boundary constants, and professional-boundary constants.

## Notes
- No source material beyond the governing documents, accepted decomposition, approved DAG, and registers is introduced by PREPARATION.
- The quarantined Chirality app corpus is not a source for this control surface.
