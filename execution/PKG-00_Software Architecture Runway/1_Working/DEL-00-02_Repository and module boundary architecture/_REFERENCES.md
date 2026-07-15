# References: DEL-00-02 Repository and module boundary architecture

## Governing References
- INIT.md - Root bootstrap, reading order, and agent constraints.
- AGENTS.md - OpenPipeStress agent index and dispatch rules.
- docs/README.md - Governance document map and status.
- docs/DIRECTIVE.md - Founding intent, product boundaries, and stop rules.
- docs/CONTRACT.md - Invariant catalog.
- docs/TYPES.md - Identifier, lifecycle, and vocabulary definitions.
- docs/SPEC.md - Technical and agentic implementation specification.
- docs/PRD.md - Product requirements and architecture questions.
- docs/AGENTIC_DEVELOPMENT_WORKFLOW.md - Type 1/Type 2 execution workflow.
- agents/AGENT_ORCHESTRATOR.md - ORCHESTRATOR workflow and coordination rules.
- agents/AGENT_PREPARATION.md - PREPARATION scaffolding protocol and file schemas.

## Decomposition and Registers
- execution/_Decomposition/SOFTWARE_DECOMP.md - Accepted revision 0.9 current decomposition basis; package PKG-00 and deliverable DEL-00-02.
- docs/_Registers/Deliverables.csv - Deliverable identity, description, artifacts, scope, objectives, and context envelope.
- docs/_Registers/ScopeLedger.csv - Scope item mapping for SOW-057,SOW-062.
- docs/_Registers/ContextBudgetQA.csv - Context budget row for DEL-00-02.

## Coordination References
- execution/_Coordination/_COORDINATION.md - approved DAG-007 graph authority, architecture-basis context rules, and lifecycle-state discovery.
- execution/_Coordination/NEXT_INSTANCE_PROMPT.md - Stable ORCHESTRATOR control-loop instructions.
- Current state is discovered from deliverable-local `_STATUS.md`, `MEMORY.md`, `_run_records/**`, and approved DAG-007 surfaces; there is no separate mutable handoff-state authority.

## Notes
- No source material beyond governing documents, registers, and coordination records is introduced by PREPARATION.
