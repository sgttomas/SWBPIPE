---
doc_id: SCA-005-AMENDMENT-PREVIEW
doc_kind: scope_change.amendment_preview
status: prepared_not_accepted
created: 2026-07-04
---

# SCA-005 Amendment Preview

This preview names the intended truth edits for owner acceptance. It is not an
applied diff.

## Intended Direct Amendments

| Surface | Preview |
|---|---|
| `docs/PRD.md` | Either replace the governing PRD body with the adopted v0.2 text or add an authority-forwarding preamble that names `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` as the adopted forward PRD authority under DEC-056. The chosen execution should preserve v0.1 history and carry the v0.1 R5 release-machinery residuals as explicit R6-entry residuals. |
| `docs/PLAN.md` | Replace language saying R6/R7 are contingent on unruled D-21 with language saying DEC-056 adopted v0.2 and that R6/R7 are now forward-plannable ordinary-gated work, with live binding still gated by D-30/F3/tier-0/DEC-041 constraints. |
| `plans/PLAN_2026-06-17_prd_completion.md` | Change D-21 extended-horizon text from conditional to adopted; insert or point to the full D-21 Annex A FR crosswalk as the required forward traceability bridge. |
| `execution/_Coordination/_COORDINATION.md` | Update baseline intake / stage-selection guidance so the completion yardstick no longer points exclusively to PRD v0.1 R0-R5. Preserve the current target-stage authority mechanism and avoid lifecycle advancement. |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Add a revision note for SCA-005 after v0.7 if the amendment is accepted. Preserve DEC-056 and older DEC rows unchanged as immutable history. |
| `execution/_ScopeChange/_LATEST.md` | On successful Gate 5 only, point to `SCA-005_2026-07-04_0000/` with status accepted or open-pending-derivative-closure, according to the validated handoff state. |
| D-29 register row | On successful Gate 5 only, move D-29 to `RULED` with pointer to the accepted SCA-005 handoff state. |

## Explicit Non-Amendments

- No deliverable folder content is edited.
- No `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `_REFERENCES.md`, or
  `_SEMANTIC.md` file is edited by SCOPE_CHANGE.
- No source code, schema, test, runtime dependency, package installation,
  protected-path hook, live-binding integration, lifecycle transition, release
  claim, or professional claim is created.

## Acceptance Needed Before Execution

Gate-5 execution requires owner acceptance of:

1. `Impact_Assessment.md`
2. this amendment preview
3. `Propagation_Plan.md`
4. the non-write boundaries and downstream handoff obligations
