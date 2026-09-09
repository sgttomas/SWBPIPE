# NS sealed brief V1 — nonlinear and solver concerns

`REQUESTED_BY`: P0 EVALUATION Agent 1 under HELP_HUMAN. Execute as a bounded ephemeral Agent 2 generalist. Requested model `gpt-5.6-sol`, reasoning `high`. You must not delegate.

## Objective

Assess remaining nonlinear-support and solver/numerical concerns in current source commit `533332349a4607eee561d4ef90fb05a62d86519e` across PKG-04 and PKG-09, with PKG-05 interfaces only where directly consumed. Determine which concerns are reproducible current defects repairable within already accepted behavior and which require a new engineering meaning, algorithm/history choice, numerical threshold/policy, compatibility contract, or formal dependency/Owner decision.

PR 760 repaired the accepted current-normal friction issue. Do not re-open historical N7. Check whether consequential nonlinear/solver defects remain after that repair. Avoid a full re-audit or blanket test run.

## Accepted basis and declared context

- Root and project `AGENTS.md`; `agents/AGENT_EVALUATION.md`.
- Root P0 controls under `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/`.
- Accepted decomposition 0.12, SCA-009, DAG-010, R5, and current source at the stated commit.
- Current PKG-04/PKG-09 deliverable context, status, dependencies, and directly relevant source/tests under `projects/chirality-piping/core/**` and `projects/chirality-piping/validation/**`.
- Selective prior evidence under `execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/**`, `execution/_Evaluation/PHYSICS_UI_PREPARATION_20260907/**`, and accepted 2026-09-08 coordination/evidence only where it bears directly on a current claim.
- Extracted `domains/piping-design/` equations are prohibited as physics authority.

## Tools and write scope

Use read/search and focused deterministic reproductions. Do not run shared heavy builds or blanket suites. All writes must be inside `projects/chirality-piping/execution/_Evaluation/PHYSICS_REMAINING_20260909/returns/NS/**`. Do not edit source, tests, deliverables, decisions, dependencies, DAGs, registers, receipts, coordination controls, or Git state.

## Required outputs

- `RETURN.md`: concise assessment with early consequential finding first; explicit defect-versus-decision routing; evidence citations with paths and line numbers where feasible.
- `FINDINGS.csv`: columns `FindingID,Concern,Classification,Severity,Scope,Claim,EvidenceRefs,Status,RecommendedOwner,RerunRequirement`.
- `COVERAGE.md`: assessed surfaces, exclusions, and unknowns.
- `VALIDATION.json`: source SHA observed, commands/reproductions run, exit results, files written, and confirmation of no subject writes.

## Acceptance and escalation

A defect claim requires a current-source trace and focused reproduction or an equally decisive deterministic proof. Preserve disagreements with prior evidence. Do not assign arbitrary severity scores; use `critical|high|medium|low|informational` only as routing urgency. Escalate any needed new physical meaning, state-history convention, supported problem class, tolerance, pivot/conditioning threshold, stabilization, compatibility behavior, dependency disposition, or engineering acceptance. Return no-finding evidence where a suspected issue is not reproducible.
