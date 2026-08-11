# ORCHESTRATION PLAN — DEL-09-04 Owner-Gates Preparation

- RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`
- PlanVersion: `1` (frozen before child dispatch)
- RequestedBy: human owner direction dated 2026-08-10
- ParentInstanceID: `HELP_HUMAN/Agent0/current-session`
- ManagerInstanceID: `WORKING_ITEMS/Agent1/working_items_del0904_owner_gates_prepare`
- SelectionAuthority: `HUMAN`
- Pattern: `TERMINAL_FAN_OUT_IN`
- RepositoryBase: `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`
- Branch: `codex/piping-del0904-owner-gates-20260810`
- PackageID: `PKG-09`
- PackagePath: `projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles`
- SelectedDeliverables: `DEL-09-04` only
- DeliverableRepresentation: `SOW_V1`
- ActivationState: `IN_PROGRESS`; exactly two live Remaining bullets at the accepted base

## Objective and authority

Prepare, but do not decide or apply, the owner gates held for DEL-09-04:

1. the DEC-046 public-benchmark release-tolerance value packet and exact 13-case blocked-mechanics census;
2. the R14 clean-checkout reproduction acceptance packet for `REPRO_DEL0904_20260720T074714Z_a5235340aae3`;
3. the MAINTAINER_REVIEWED case-page promotion-basis packet; and
4. a precise future-integration map for TM-PIP-037 and only the first two DEL-09-04 Remaining bullets.

The human direction authorizes preparation only. Adoption, acceptance, promotion, register disposition, Remaining-bullet edits, release, reliance, and lifecycle effects remain owner-gated and are not executed in this run.

## Accepted basis

- Root `AGENTS.md`, `agents/AGENT_HELP_HUMAN.md`, `agents/AGENT_WORKING_ITEMS.md`, and project `AGENTS.md` at the repository base.
- Committed `projects/chirality-piping/loop/LOOP_INIT.md` and selected committed standing plan `projects/chirality-piping/loop/WORKPLAN_2026-07-18b_piping_loop.md` (blob `61dbbca25b9be766383aa1e5a743a021ce4d63d1`).
- Live DEL-09-04 SOW_V1 context/status/references and accepted project sources at the base.
- Human owner direction dated 2026-08-10 in the supervising session.

## Read and write scope

All children may read committed repository content at the accepted base and use read-only shell/Git/search/hash commands. No child may fetch, run a writing test/tool, alter a committed evidence bundle, or delegate.

The sole repository write root for the entire manager run is this new directory:

`projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/`

Each author child owns only `PACKET.md` and `RETURN.md` in its own instance directory. The independent verifier is serialized after author fan-in and owns only its verifier return. Manager-only integration files are `MANAGER_VALIDATION.md`, `RUN_RECORD.md`, `PACKAGE_RETURN.md`, and `HANDOFF_STATE.md` at the run root plus manager status updates. No pre-existing file may be modified.

## Work graph

| Node | Agent type | Objective | Dependencies | Owned writes |
|---|---|---|---|---|
| `A2-DEC046-CENSUS` | ephemeral Agent 2 | DEC-046 value packet plus 13-case census | none | `instances/A2_DEC046_CENSUS/{PACKET.md,RETURN.md}` |
| `A2-R14` | ephemeral Agent 2 | R14 acceptance and currency packet | none | `instances/A2_R14/{PACKET.md,RETURN.md}` |
| `A2-MAINTAINER-REVIEW` | ephemeral Agent 2 | lawful promotion basis and complete page inventory | none | `instances/A2_MAINTAINER_REVIEW/{PACKET.md,RETURN.md}` |
| `A2-VERIFY` | fresh ephemeral Agent 2 | independent 100% evidence/refutation review | all three author returns accepted for verification | `instances/A2_VERIFY/RETURN.md` |
| `A1-FANIN` | WORKING_ITEMS | validate, synthesize, and return preparation package | verifier terminal return | manager-owned run-root closeout files |

Author writes are disjoint and concurrent. Verification and fan-in are serialized. No child is an integration owner for project truth; all outputs are non-authoritative derivative packets awaiting owner rulings.

## Fan-in gates

- Exact base, branch, path containment, no tracked edits outside the run root, no ignored drift, and receipt identity unchanged.
- Every material numeric/factual claim cites live committed evidence and distinguishes raw comparison observations, internal assertions, fixture constants, and proposed governed release values.
- Exact inventories and hashes for the R14 bundle and case-page candidate corpus.
- P1–P16, PRD §24 R6, evidence-tier, DEC-046/D-19/CV-B, TM-PIP-037, and DEL-09-04 Remaining claims checked directly against source.
- Options are materially distinct, recommendations are explicitly non-binding, and application mechanisms are conditional on later owner rulings.
- No GUI-workflow evidence, export/CAEPIPE scope, D-61 reliance posture, lifecycle, release, repair, case-page, benchmark, register, status, receipt, or decision-surface effect.

## Escalation and terminus

Unknown or contradictory evidence is surfaced as a blocker or caveat; values, review acts, and acceptance are never invented. The terminal result is an owner-ready preparation package. This manager stops before the owner gate and requests no Git closeout in this preparation-only stage.
