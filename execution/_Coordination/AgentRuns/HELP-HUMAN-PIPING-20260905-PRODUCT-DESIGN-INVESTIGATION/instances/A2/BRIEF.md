# Sealed launch brief — A2

Parent: HELP_HUMAN / Agent 0.
Role: Ephemeral specialist / Agent 2.

## Objective

Investigate physical model, engineering intent, analytical idealization, analysis/run/result architecture for a robust 3D piping modeler feeding stress analysis. Inspect pertinent PKG-00/01/02/03/04/05/06/08/09/10/11/12/13/14/15/16/17 source and deliverables as relevant; bind exact selected IDs to claims. Address identity/topology, ports/fittings/bends/supports, units/provenance, model-to-analysis derivation, mesh/segmentation, scenarios/load cases, boundary conditions, validation, solver separation, version/hash lineage, results mapped back to geometry, and exchange limitations. Clearly distinguish current architecture and proposed evolution. Supply concrete scenarios and staged feasible recommendations.

## Declared context and common constraints

Resolve REPO_ROOT with `git rev-parse --show-toplevel`; WORKING_ROOT={REPO_ROOT}/projects/chirality-piping; RUN_ROOT={WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PRODUCT-DESIGN-INVESTIGATION.
Read root and project AGENTS.md; frozen ORCHESTRATION_PLAN.md; project docs/CONTRACT.md and docs/_Registers/Deliverables.csv; accepted pointers and handoffs identified in the plan. Source HEAD must match f4b223dd115c4234e0182dcd752a885c3de175ce or report drift before relying on it. User authorization is investigation/report only.
Permission: read relevant repository source/specs/coordination and public primary external documentation via available read-only tools. No product/model/Git/source-of-truth/authority/lifecycle changes, UI mutations, private data publication, or external messages. Write only named targets. Follow IP/data/provenance/unit/engineering-claim and Agent 2 scope invariants cited in the plan. Never ground physics equations in unreviewed piping-design equation extractions.
Output contract: concise evidence-backed FINDINGS.md and RETURN.md under own instance path; record role, actual parent, scope, start/completion status, source HEAD, brief/output SHA-256, actual model or unavailable, inspected paths/URLs, findings, uncertainty, blockers, and next owner. Findings are derivative recommendations; distinguish implemented code, RUN verification, external documentation, inference, and proposals. Parent-observed app visuals are inherited evidence only. No invented tool execution. Native role constraints are instruction+config asserted, not mechanism-proven.

## Allowed write targets

{RUN_ROOT}/instances/A2/

## Execution gate and acceptance

No dependency; parallel with U1/H3. Do not delegate. Return to HELP_HUMAN.
Complete only when the objective has concrete evidence-backed outputs, limitations and unresolved gaps are explicit, scope is respected, and RETURN.md records actual execution and hashes.
