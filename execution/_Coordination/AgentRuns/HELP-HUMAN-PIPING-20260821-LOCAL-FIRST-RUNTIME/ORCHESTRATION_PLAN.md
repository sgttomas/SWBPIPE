# Orchestration Plan v1

- RunID: `HELP-HUMAN-PIPING-20260821-LOCAL-FIRST-RUNTIME`
- SelectionAuthority: `HUMAN` for the development-pressure steer; `AGENT_0` for Step-0 node selection.
- AcceptedBasis: `main@1b375af4f`; Receipt 124; DAG-010; DEL-12-01 `_STATUS.md` `## Remaining`; project `software-workflow.json`.
- Posture: `MIXED` (serial branch setup, one package production node, fresh review, integration closeout).
- Objective: materially close DEL-12-01's authorized runtime-enforcement residual without selecting owner/review/publication decisions.
- HumanDecisionPoints: only findings that touch a hard fence or need new owner authority; collect and park them without holding authorized repairs.
- SharedSurfaceOwner: HELP_HUMAN at fan-in; CHANGE for Git state, commits, push, and PR.

## Frozen graph

1. `CHANGE-001` creates `codex/piping-local-first-runtime-20260821` from clean `main@1b375af4f`.
2. `WI-PKG12-001` owns N1 for PKG-12 / DEL-12-01 and may dispatch bounded Agent 2 implementation and mandatory fresh review work.
3. N1 runs focused checks, full Piping checks, a fresh 100%-diff read-only review with hash inventory, and DEC-025 before landing.
4. HELP_HUMAN validates package fan-in; CHANGE performs ordered commit/push/PR closeout and one receipt is appended at fan-in.

N1 is the only engineering node. Discoveries inside its accepted residual amend N1 in place; discoveries outside it are registered and stopped.
