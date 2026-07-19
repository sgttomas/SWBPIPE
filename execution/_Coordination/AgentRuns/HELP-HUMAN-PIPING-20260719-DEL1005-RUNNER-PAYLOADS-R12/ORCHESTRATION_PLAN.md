# ORCHESTRATION PLAN — HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12

- **Plan version:** 1
- **Prepared:** 2026-07-19T21:11:09Z
- **Parent:** HELP_HUMAN (Agent 0), loop session under
  `loop/WORKPLAN_2026-07-18b_piping_loop.md` (committed-HEAD selected),
  cursor `Receipt-58`, examined-through `96563e8e0` (HEAD at plan freeze).
- **Selection authority:** AGENT_0 (per plan Step 1 principles; no explicit
  human sequence for this iteration; per-run steer: "consider the effective
  use of subagents").
- **Descriptive posture:** TERMINAL_FAN_OUT_IN (serialized chain; each child
  returns terminally to HELP_HUMAN; no sibling messaging).

## Objective

Advance the R6-path prerequisite: bind the headless runner `run-benchmark`
and `run-regression` downstream payloads (DEL-10-05 Remaining item sourced
from TP-E2-VALMANUAL-001 residuals), so per-case validation reproduction can
run through `openpipestress-runner` instead of suite tests only. The
`export-results` binding (coupled to DEL-08-01 report container) is OUT of
this tranche.

## Authority basis

- Standing plan Step 2 delegation triage: DEC-082/Shared-Block v1 class test,
  fast-reject first; D-52/DEC-085 standing-approval lane; D-54/DEC-087
  reasoned-selection refinement (agent selects among defensible alternatives
  with rationale + independent refutation before effect; borderline defaults
  to the owner).
- Preserved owner gates: brief-adoption effect only through the D-52
  standing rule after CLASSIFY_ELIGIBLE + independent COMMIT-SAFE; no
  lifecycle/stage/release/acceptance act; no threshold or tolerance
  promotion (DEC-046); owner merges; D-45 and all AWAITING_RULING rows
  untouched.

## Work graph (nodes, dependencies, ownership)

| Node | Role | Objective | Depends on | Writes |
|---|---|---|---|---|
| N1 | Agent 1 (ORCHESTRATOR-for-HELP_HUMAN) | Intake DEL-10-05 + accepted basis; author CANDIDATE brief + D-54 rationale artifact (limits screen, four-lens, alternatives) | — | `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_DEL-10-05_RUNNER_PAYLOAD_BINDINGS.md`; this run's `instances/N1/**`, `CURRENT_CANDIDATE_RATIONALE.md` |
| N2 | Agent 2 (fresh-context adversarial verifier) | Refute N1's enumerated claims; return COMMIT-SAFE or BLOCK only | N1 | `instances/N2/RETURN.md` |
| N3 | Agent 1 (WORKING_ITEMS manager, serialized non-delegating executor) | Execute the sealed brief: implement bindings, tests, evidence; DEC-025 sweep; deliverable-local state updates | N2=COMMIT-SAFE and HELP_HUMAN release | per brief write fence only |
| N4 | Agent 2 (fresh-context implementation verifier) | Refute N3's claimed results pre-commit; COMMIT-SAFE or BLOCK | N3 | `instances/N4/RETURN.md` |

HELP_HUMAN validates fan-in at each edge; a BLOCK at N2 or N4 holds the
dependent node and returns the condition (repair or slate) without
manufacturing new scope. Closeout (receipt append + validator) follows only
after N4 COMMIT-SAFE.

## Concurrency and write ownership

Children run serialized (N1 → N2 → N3 → N4); no concurrent writes. N3 is the
sole write-fence owner for project content in this run. Concurrent external
loops (app-dev, PEC, `_DomainEngines/**`) are out of scope; their dirty state,
if any, is external and untouched.

## Human decision points

1. Any fast-reject hit or verifier BLOCK → near-miss slate to owner.
2. PR merge (always the owner's).
3. Reproduction acceptance, threshold promotion, lifecycle/stage acts —
   never performed here.
