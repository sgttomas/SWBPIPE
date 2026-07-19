# ORCHESTRATION PLAN — HELP-HUMAN-PIPING-20260719-DEL0904-VALMANUAL-REFRESH-R13

- **Plan version:** 1 · **Posture:** TERMINAL_FAN_OUT_IN (serialized N1→N2→N3→N4)
- **Parent:** HELP_HUMAN loop session; plan `loop/WORKPLAN_2026-07-18b_piping_loop.md`;
  cursor `Receipt-59`; basis HEAD `45ec0524d` (post-merge of PR #287).
- **Selection authority:** AGENT_0; principles (a)+(b): the live tree (bound
  `run-benchmark`/`run-regression`) now contradicts DEL-09-04's Remaining
  bullet and the E1 manual page's stub-expectation rows — repair of landed
  work's derivative surfaces on the R6 path.

## Objective

Bounded DEL-09-04 docs+state tranche: refresh
`docs/validation_manual/headless_runner_reproduction.md` to the post-#287
runner behavior (bound benchmark/regression verbs; stub expectation retired to
historical context; document the new `del1005_payload_binding_*` witness
examples as the per-case reproduction path; state the rerun-trigger
consequence — prior bundles stay truthful for their pinned commits, any new
reproduction needs a fresh run ID from a post-#287 commit). Update DEL-09-04
`_STATUS.md` Remaining wording to strike only the landed bindings dependency
(MAINTAINER_REVIEWED promotion, GUI-workflow evidence, and owner-gated
tolerance promotion stay open), append History/MEMORY/run record. No
reproduction run, no lifecycle change, no threshold content.

## Nodes

| Node | Role | Output |
|---|---|---|
| N1 | Agent 1 brief author | CANDIDATE brief + D-54 rationale + enumerated claims |
| N2 | Agent 2 fresh-context verifier | COMMIT-SAFE / BLOCK on N1 claims |
| N3 | Agent 1 executor (serialized, non-delegating) | docs+state edits, checks, return + claims |
| N4 | Agent 2 fresh-context verifier | COMMIT-SAFE / BLOCK pre-commit |

Same fan-in, failure-isolation, and preserved-gate rules as R12
(`../HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/ORCHESTRATION_PLAN.md`).
Owner gates preserved: merge; case-page MAINTAINER_REVIEWED promotion;
DEC-046 threshold promotion; reproduction acceptance; lifecycle/stage acts.
