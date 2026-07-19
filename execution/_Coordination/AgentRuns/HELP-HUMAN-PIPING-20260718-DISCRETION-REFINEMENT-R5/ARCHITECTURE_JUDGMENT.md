# Architecture Judgment — Candidate Isolation and HEAD-Only Fail-Stop

**OwnerStandingApproval:** owner architectural clarification / D-54 direction
**AgentJudgment:** `SELECT_HEAD_ONLY_NO_FALLBACK`
**OwnerCaseSelection:** `NONE`
**Effect:** `HELD`; no owner gate crossed

## Selected Outcome

Keep the reviewed successor plan at the R5 candidate path, outside the
auto-discovered `loop/WORKPLAN_*.md` namespace. `LOOP_INIT.md` selects only
the lexicographically newest matching plan present as one mode-`100644` blob
in committed `HEAD` and reads only `git show HEAD:path`. Enumeration,
selection, regular-blob validation, or committed-byte read failure stops
before Step 0. There is no silent older-plan fallback.

After two fresh independent `COMMIT-SAFE` verifier returns, CHANGE may
materialize the active 2026-07-18 plan byte-identically and land candidate,
active plan, governance records, curtailed-review history, and `LOOP_INIT.md`
in the same durable commit. The owner later clarified that no repeat sibling
review is required because Shared-Block v1 and app-dev did not change. Staging
does not activate the plan; that commit does.

## Four-Lens Rationale

- **Ontology:** candidate and active plan are distinct governed states. Their
  distinct paths make review state and operational state explicit, while the
  committed `HEAD` namespace is the sole active-plan population.
- **Epistemology:** committed `HEAD` provides stable, reproducible evidence of
  which plan and bytes govern. Worktree and index state are preparation
  evidence, not activation evidence.
- **Praxeology:** one finite HEAD-tree selection rule plus atomic promotion
  gives a small, testable action path and removes the recursive guard surface.
  Fail-stop behavior makes loader defects visible before Step 0.
- **Axiology:** fail-closed selection, preserved review history, truthful
  attribution, and atomic activation minimize surprise while respecting owner
  gates and the value of durable evidence.

## Rejected Alternatives

- **Embedded semantic guard:** rejected because successive mutation classes
  expanded the guard and its adversarial surface without improving the core
  activation boundary. Its prior BLOCK, interruption, and tool-error history
  remains preserved as evidence.
- **Fixed-date fallback:** rejected because silently selecting an older plan
  can mask corruption or staleness and makes the selected authority ambiguous.
  Explicit stop-before-Step-0 behavior is more coherent with fail-closed
  governance.

This is an agent reasoned selection under the owner's standing approval, not
an owner case selection, adoption, ruling, or direction authored by the agent.
Both fresh v7 verifiers returned `COMMIT-SAFE`; the architecture is ready for
durable landing and remains without effect until that landing.
