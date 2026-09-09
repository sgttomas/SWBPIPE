# PCB sealed brief V1 — pressure, connectors, and analytical bridge concerns

`REQUESTED_BY`: P0 EVALUATION Agent 1 under HELP_HUMAN. Execute as a bounded ephemeral Agent 2 generalist. Requested model `gpt-5.6-sol`, reasoning `high`. You must not delegate.

## Objective

Assess remaining pressure, connector, and physical-to-analytical bridge concerns in current source commit `533332349a4607eee561d4ef90fb05a62d86519e` across PKG-05, PKG-13, PKG-04 interfaces, and PKG-09 verification. Determine whether any current behavior contains a reproducible, bounded implementation defect under an already accepted meaning, separately from work that requires a new engineering/reference choice, public result contract, compatibility/migration decision, schema adoption, threshold, formal dependency disposition, or Owner decision.

Treat the 2026-09-08 four-case pressure/reference work, canonical design, and mixed-reference work as accepted candidate evidence only. It is not implementation adoption. Do not propose speculative fixes from candidate equations or inferred meanings.

## Accepted basis and declared context

- Root and project `AGENTS.md`; `agents/AGENT_EVALUATION.md`.
- Root P0 controls under `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/`.
- Accepted decomposition 0.12, SCA-009, DAG-010, R5, and current source at the stated commit.
- Current PKG-05/PKG-13/PKG-09 context, status, dependencies and directly relevant source/tests under `projects/chirality-piping/core/**`, app adapter/schema code only where it consumes the physical model, and `projects/chirality-piping/validation/**`.
- Candidate evidence in `execution/_Evaluation/PHYSICS_UI_PREPARATION_20260907/E1/children/{pressure,connector_numerics}/**`, `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/**`, and its accepted snapshot, read with stated candidate limits.
- Extracted `domains/piping-design/` equations are prohibited as physics authority.

## Tools and write scope

Use read/search and focused deterministic reproductions. Do not run shared heavy builds or blanket suites. All writes must be inside `projects/chirality-piping/execution/_Evaluation/PHYSICS_REMAINING_20260909/returns/PCB/**`. Do not edit source, tests, deliverables, decisions, dependencies, DAGs, registers, receipts, coordination controls, or Git state.

## Required outputs

- `RETURN.md`: concise assessment with early consequential finding first; explicit defect-versus-decision routing; evidence citations with paths and line numbers where feasible.
- `FINDINGS.csv`: columns `FindingID,Concern,Classification,Severity,Scope,Claim,EvidenceRefs,Status,RecommendedOwner,RerunRequirement`.
- `COVERAGE.md`: assessed surfaces, exclusions, and unknowns.
- `VALIDATION.json`: source SHA observed, commands/reproductions run, exit results, files written, and confirmation of no subject writes.

## Acceptance and escalation

A defect claim requires a current-source trace and focused reproduction or equally decisive deterministic proof against accepted behavior. Preserve disagreements with prior evidence. Do not assign arbitrary scores; use `critical|high|medium|low|informational` only as routing urgency. Escalate pressure wall/effective-force and sign/reference meanings, constitutive/Poisson choices, connector stiffness/reference-point meanings, bridge/schema/public semantics, compatibility/migration, thresholds, and dependency or engineering acceptance. Return no-finding evidence where a suspected issue is not reproducible.
