# References: DEL-10-04 Build, packaging, and CI/CD pipeline

## Governing References
- INIT.md - Root bootstrap, reading order, and agent constraints.
- AGENTS.md - OpenPipeStress agent index and dispatch rules.
- docs/README.md - Governance document map and status.
- docs/DIRECTIVE.md - Founding intent, product boundaries, and stop rules.
- docs/CONTRACT.md - Invariant catalog.
- docs/TYPES.md - Identifier, lifecycle, and vocabulary definitions.
- docs/SPEC.md - Technical and agentic implementation specification.
- docs/AGENTIC_DEVELOPMENT_WORKFLOW.md - Type 1/Type 2 execution workflow.
- agents/AGENT_PREPARATION.md - PREPARATION scaffolding protocol and file schemas.

## Decomposition and Registers
- execution/_Decomposition/SOFTWARE_DECOMP.md - Accepted revision 0.7 current decomposition basis; package PKG-10 and deliverable DEL-10-04.
- docs/_Registers/Deliverables.csv - Deliverable identity, description, artifacts, scope, objectives, and context envelope.
- docs/_Registers/ScopeLedger.csv - Scope item mapping for SOW-032.
- docs/_Registers/ContextBudgetQA.csv - Context budget row for DEL-10-04.

## Package-Specific References
- `projects/chirality-piping/package.json` - existing root npm workspace
  manifest and build-script evidence attributed to DEL-10-04 under D-41
  `DEC-074` O3; SHA-256 at the R5 T1 read-only inspection:
  `7e719791e3ffdc7b57eddb2bb32d682705bf945af5b7207ffc699a2a45648656`.
- Package context remains limited to accepted decomposition/register scope;
  the manifest attribution does not expand it.

## Notes
- Historical PREPARATION used only the governing documents and registers.
- D-41 R5 T1 adds the ruled root-manifest evidence pointer without editing the
  manifest or resolving the separate D-42 build-readiness panel conflict.
