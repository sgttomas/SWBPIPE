# Actual DEC-083 S5 Return 01 — BLOCK

**Verdict:** `BLOCK`
**Provenance:** Actual owner-mediated DEC-083 sibling-project review, relayed
by HELP_HUMAN to HELPS_HUMANS on 2026-07-18.
**Disposition:** Accepted without softening; policy semantics require no
correction, but activation and graph-integrity defects block landing.

## Verbatim Relay

<!-- BEGIN S5 BLOCK RELAY 01 -->
Actual owner-mediated DEC-083 S5 review returned BLOCK. Accept it and preserve the return durably in R5 history; do not soften it. Findings:
1 Semantic refinement coherent; no correction to policy semantics.
2 Shared-Block v1 byte-identical; app-dev has no change as expected; no paired v2.
3 Premature activation defect: LOOP_INIT selects newest WORKPLAN_*.md and the untracked loop/WORKPLAN_2026-07-18_piping_loop.md is already newest, contradicting effect-held/no-application-before-S5+durable landing. Carry-forward verifier missed this staging hazard.
4 WORK_GRAPH.json N4_DEC083_S5_SIBLING_REVIEW depends on nonexistent N1_FIRST_RETURN; actual node is N1_INTEGRATION. Semantic verifier missed referential integrity/enumeration claim.
Minimum correction required by S5: prevent selection/effect before landing, fix graph dependency, rerun fresh enumeration-derived semantic verifier covering pointer/effect state + graph referential integrity, rerun carry-forward verifier, then repeat S5.

Implement correction within existing R5 scope. Preferred correction: add a mechanically checkable fail-closed pre-landing activation guard as the first operational instruction of the new re-minted plan. It must make a fresh loop fall back to the prior tracked WORKPLAN_2026-07-17_piping_loop.md unless ALL are true: the new plan is tracked at HEAD (`git ls-files --error-unmatch`), the worktree bytes equal `git show HEAD:<path>`/blob hash, D-54 and DEC-087 are present at the same HEAD, and the R5 actual S5 return is COMMIT-SAFE at the same durable HEAD basis. Design the check without a circular future commit SHA and make it deterministic/fail-closed. The guard may use content/blob hashes and tracked-at-HEAD checks. Before durable landing it must unequivocally select/fall back to the prior plan and prohibit DEC-087 application. After landing it may activate only when all checks pass. If this cannot be made non-circular and mechanically sound, relocate the candidate plan out of loop/WORKPLAN_*.md and report a landing choreography; do not leave it active.
Fix WORK_GRAPH dependency to N1_INTEGRATION and validate all graph dependencies resolve.
Create/preserve S5_BLOCK_RETURN_01.md with exact review provenance and findings. Amend status/handoff/packet truthfully to CORRECTION_IN_PROGRESS then REVIEW_PENDING_S5 only after corrections pass.
Dispatch two FRESH independent verifier nodes with new IDs/briefs/returns: semantic verifier v2 enumeration must include every touched governed artifact, activation guard/pointer state, graph reference integrity, attribution/boundaries, Shared-Block, DEL-09-04; carry-forward verifier v2 must test the guard in current pre-landing state and simulate/verify post-landing conditions without applying effect, plus old/new plan carry-forward and Step3 bytes. Preserve v1 returns.
Rerun validators/invariants. Do not claim S5 pass, apply refined discretion, append receipt, touch DEL-09-04, or commit. Return corrected first-return packet for repeat S5.
<!-- END S5 BLOCK RELAY 01 -->

Canonical relay binding, excluding marker lines and adjacent delimiter
newlines: 2,997 UTF-8 bytes; SHA-256
`169aa2795716b5b170a6499ef825ef960848fae1e72ccb1f43ba195fb0fb839c`.

## Findings Applied Without Softening

1. Policy semantics are coherent; no policy correction is made.
2. Shared-Block v1 remains byte-identical; app-dev remains untouched.
3. The candidate plan needs a deterministic fail-closed pre-landing guard.
4. The graph dependency must resolve to `N1_INTEGRATION`.

Fresh v2 verification and repeat actual S5 are mandatory.
