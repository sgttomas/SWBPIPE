# Remaining physics evaluation protocol

## Accepted frame

- **Manager:** P0 EVALUATION Agent 1 under HELP_HUMAN.
- **Source basis:** commit `533332349a4607eee561d4ef90fb05a62d86519e` (merged PR 760); working tree subject state is read-only.
- **Accepted governed basis:** decomposition 0.12, SCA-009, DAG-010, and R5, as frozen by `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/SETUP_BASIS_V1.json`.
- **Activation:** root work graph SHA-256 `61471cfa25a3773e279c860afd1be7a7535ed036c26b33eaae9fd4efa22aef47`; P0 brief SHA-256 `f1a3d51375b3b12128fcf0ebd7bb43ed2793feb9f6ce43aa2abc0c56668f5bee`; Owner direction authorizes the bounded assessment and independent fan-out.
- **Scope:** remaining nonlinear/solver and pressure/connectors/physical-to-analytical bridge concerns across PKG-04, PKG-05, PKG-09, and PKG-13.
- **Decision criterion:** distinguish a reproducible defect in current accepted behavior that can receive a bounded repair from a question requiring a new engineering meaning, public contract, compatibility/migration choice, numerical threshold, formal dependency disposition, or Owner decision.
- **Stakes:** decision support only. This package is derivative evidence and does not alter decomposition truth, dependencies, lifecycle state, decisions, deliverables, source, receipts, or Git state.

## Method and toolbelt

The manager applies `skills/evaluation-protocol/SKILL.md`. Two independent, bounded, nondelegating Agent 2 generalists may run concurrently under the sealed work graph:

1. `NS`: read-only nonlinear/solver assessment with narrowly targeted reproductions written only below `returns/NS/`.
2. `PCB`: read-only pressure/connectors/bridge assessment with narrowly targeted reproductions written only below `returns/PCB/`.

Permitted tools are repository read/search, deterministic inspection, and focused local reproductions that do not invoke a shared heavy build. Existing accepted evaluation and candidate-reference packages may be consumed with their stated limits. Extracted equations under `domains/piping-design/` are excluded as physics authority.

The manager validates both returns against their sealed briefs, independently inspects consequential claims, and writes `FINDINGS.csv`, `EVALUATION_REPORT.md`, `HANDOFF.md`, `VALIDATION.json`, and a SHA-256 manifest. No score is requested or produced.

## Coverage and decision points

Coverage is bounded to concerns that remain material after PR 760's current-normal friction and route-authoring repairs. Historical N7 is excluded. Pressure four-case references, canonical design, and mixed references from 2026-09-08 are candidate evidence rather than adopted implementation meaning.

Each finding is routed as one of:

- bounded source/test repair under already accepted behavior;
- engineering/reference decision;
- public contract or compatibility/migration decision;
- numerical threshold/policy decision;
- formal dependency/review/Owner routing;
- no current defect reproduced / retained unknown.

The assessment closes only when evidence coverage, conflicts, unknowns, repair boundaries, decision owners, and rerun requirements are explicit. Closure of this derivative package is not physics or lifecycle closure.
