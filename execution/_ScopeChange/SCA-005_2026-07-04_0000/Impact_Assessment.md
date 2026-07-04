---
doc_id: SCA-005-IMPACT-ASSESSMENT
doc_kind: scope_change.impact
status: prepared_not_accepted
created: 2026-07-04
---

# SCA-005 Impact Assessment

## Gate Status

This impact assessment is prepared for owner acceptance. It is not accepted yet.
No truth edits have been applied.

Active SCA pointer before this amendment remains:

- `execution/_ScopeChange/_LATEST.md` -> `SCA-004_2026-05-18_0000/`

## Impact Summary

| Action | Affected sections/files/workflows | Impact |
|---|---|---|
| SCA-005-A001 | `docs/PRD.md`; `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`; D-21 packet Annex A | Replace or amend the governing PRD reference so v0.2 is forward authority. Preserve v0.1 history and carry v0.1 R5 release-machinery residuals into the v0.2 R6-entry posture. |
| SCA-005-A002 | `docs/PLAN.md` | Remove stale "contingent until D-21 is ruled" language; point strategic roadmap to DEC-056 and the v0.2 milestone set while keeping stage-gate authority external. |
| SCA-005-A003 | `plans/PLAN_2026-06-17_prd_completion.md` | Carry D-21 Annex A as the mandatory forward FR crosswalk and update extended-horizon language from conditional to adopted-but-gated. |
| SCA-005-A004 | `execution/_Coordination/_COORDINATION.md` | Update intake/roadmap pointers that still route to v0.1 PRD sections as the only completion yardstick. Do not advance lifecycle or target-stage authority. |
| SCA-005-A005 | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Add SCA-005 to the revision/change narrative and preserve DEC-056 as immutable ruling history. Do not rewrite DEC-041, DEC-042, DEC-054, DEC-055, or DEC-056. |
| SCA-005-A006 | `execution/_ScopeChange/SCA-005_2026-07-04_0000/`; `execution/_ScopeChange/_LATEST.md` | On Gate-5 execution only, write accepted SCA artifacts and move `_LATEST.md` from SCA-004 to SCA-005. |
| SCA-005-A007 | `execution/_Coordination/_DECISIONS/_REGISTER.md`; D-29 packet | On Gate-5 closure only, move D-29 from SCA-gate-held state to `RULED` with the SCA-005 handoff pointer. |

## Package-Role Classification

| Surface | Package role | Proposed treatment | Authority basis |
|---|---|---|---|
| `docs/PRD.md` | working surface | DIRECT_EDIT or replacement pointer to v0.2 authority | DEC-056; D-21 Annex A |
| `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` | source / candidate replacement PRD | NO_CHANGE; cited as adopted source | DEC-056 |
| `docs/PLAN.md` | strategy map | DIRECT_EDIT | DEC-056; SCA-005 approval |
| `plans/PLAN_2026-06-17_prd_completion.md` | tactical plan / derivative planning surface | DIRECT_EDIT, or mark stale if successor plan is preferred | DEC-056; SCA-005 approval |
| `execution/_Coordination/_COORDINATION.md` | coordination working surface | DIRECT_EDIT limited to pointers/gate posture | DEC-056; SCA-005 approval |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | authoritative decomposition working surface | DIRECT_EDIT limited to revision/change narrative if approved | DEC-056; SCA-005 approval |
| `execution/_ScopeChange/SCA-005_2026-07-04_0000/` | snapshot / handoff artifact | WRITE_GATE_ARTIFACTS, then accepted snapshot after Gate 5 | SCOPE_CHANGE protocol |
| `execution/_ScopeChange/_LATEST.md` | active snapshot pointer | HELD until Gate 5 validates and closes | SCOPE_CHANGE protocol |
| deliverable folders under `execution/PKG-*` | downstream working surfaces | NO_CHANGE in SCOPE_CHANGE pass | SCOPE_CHANGE non-write boundary |
| source code, schemas, tests, app packages | implementation surfaces | NO_CHANGE | D-29 scope; DEC-056 non-execution boundary |
| DAG, dependency extraction, estimates, schedules | derivative packages | STALE_REBUILD_REQUIRED after accepted truth edits | SCOPE_CHANGE handoff rule |

## Derivative-Package Status After Accepted Amendment

| Package/surface | Owner | Status after amendment | Required rerun / closure action |
|---|---|---|---|
| Approved DAG package (`execution/_DAG/DAG-007/`) | ORCHESTRATOR / dependency workflow | STALE_REVIEW_REQUIRED | Decide whether v0.2 R6/R7 traceability requires DAG successor. |
| Dependency extraction registers | TASK + dependency-extract | STALE_REBUILD_REQUIRED | Re-extract after PRD/PLAN/decomposition traceability surfaces are amended. |
| Estimate / schedule snapshots | estimation / SCHEDULING | STALE_REVIEW_REQUIRED | Recompute only if owner selects schedule/estimate work. |
| Deliverable-local `_CONTEXT.md` / `_STATUS.md` files | PREPARATION / TASK owners | DEFERRED | No direct SCOPE_CHANGE write; future preparation or metadata alignment only after separate authorization. |
| Release/readiness evidence | REVIEW / release-quality workflow | NO_CHANGE | No release claim is created. |

## Invariant And Orphan-Risk Summary

- No package or deliverable is added, removed, split, merged, or reclassified in
  this SCA pass.
- No `PKG-*`, `DEL-*`, or `SOW-*` ID is reused.
- Immutable ruled history remains in v0.1 terms where it was originally ruled.
  Future-facing references must cite D-21 Annex A when translating v0.1 flat FR
  identifiers to v0.2 namespaced FR families.
- R7 execution and live binding remain blocked by independent gates: app-dev
  F3, DEC-041 automation condition as further constrained by D-30, and tier-0
  adoption posture.
- No professional, certification, sealing, authentication, code-compliance, or
  release-readiness claim is introduced.

## Baseline And Validation Notes

This prepared bundle uses direct file inspection as its pre-change basis. A
Gate-5 execution pass should record pre/post validation in `RUN_SUMMARY.md` and
should not update `_LATEST.md` until the accepted SCA-005 snapshot and handoff
state are complete.
