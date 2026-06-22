# NEXT INSTANCE PROMPT

## Current Authority

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
- Set `WORKING_ROOT` to `{REPO_ROOT}/projects/chirality-piping`.
- Read `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md`.
- Read `{WORKING_ROOT}/AGENTS.md` for project-local agent posture and closeout discipline.
- Read `{WORKING_ROOT}/execution/_Coordination/_COORDINATION.md` and follow the Application Integration And Issuance Loop directions.
- Read `docs/PLAN.md` — the non-governing strategic roadmap — for the definition of "complete per the PRD", the current milestone position, the layer-relation map, and roadmap-level risks. It is strategic orientation that routes to the authorities; the completion plan below remains the tactical selection instrument.
- Read `plans/PLAN_2026-06-17_prd_completion.md` — the current completion plan (non-governing `PROPOSAL`, supersedes the 2026-06-10 plan) — for phase ordering toward the PRD, the dependency spine, the FR completion map, and the human decision register `D-01..D-23` (with `DEC-041/042/043` recorded 2026-06-18, `DEC-044/045/046` recorded for D-16/D-18/D-19, and `DEC-048` recording `D-23` Option O-A: R3 exit evidence accepted and current target stage advanced to R4).
- Discover current state from authoritative surfaces named in `_COORDINATION.md`: `_DAG/_LATEST.md`, approved DAG artifacts, deliverable-local `_STATUS.md`, `MEMORY.md`, `_run_records/**`, dependency/review files, current app/build/test surfaces, `execution/_Coordination/_DECISIONS/_REGISTER.md`, and current aggregation or review artifacts as needed.
- Treat `execution/_DAG/DAG-007/` as the validated canonical dependency type-system rectification successor pending human approval. Do not treat it as current authority or update `_DAG/_LATEST.md` unless the human approval record is completed.
- Treat blockers or dirty git state outside this project/write scope as external-scope noise; record and bypass, do not fix.

## Governing Imperatives

1. `SOFTWARE_DECOMP` says what must be built and why.
2. `_DAG/_LATEST.md` names the approved DAG pointer. As of 2026-06-16 it still points to `DAG-006`; `DAG-007` is canonical and validated but pending human approval.
3. Deliverable-local `_STATUS.md` files say the current lifecycle state for
   issuance and review gates. `CHECKING` means design authority is mature; it
   does not mean the application has absorbed that deliverable's scope.
4. `docs/PRD.md` §10 (functional requirements) and §22 (release milestones)
   are the product completion yardstick. The completion plan orders work
   toward that yardstick; it is selection guidance, not authority. When the
   plan and authoritative surfaces disagree, surface the discrepancy and
   correct the plan, not the authority.
5. `_COORDINATION.md` says how to execute work: app-integration tranches by
   default, local-status discovery, DAG-guided context selection, bounded
   workers, fan-in, validation, evidence records, and project-local
   `AGENTS.md` closeout routing for completed validated tranches.
6. Human decisions (`D-01..D-23` and any newly discovered human-gated `TBD`)
   are never resolved by agents. Prepare decision packets per the loop's
   decision-escalation step, surface pending rulings every session, and
   proceed only on unblocked work.
7. Current lifecycle state must be discovered from deliverable-local
   `_STATUS.md` files. The 2026-06-21 discovery run reported `CHECKING=8`,
   `IN_PROGRESS=92`, and `ISSUED=1`; `DEL-01-01` is the sole currently
   `ISSUED` deliverable and remains the accepted governance baseline unless a
   human-approved change path opens it.

## Next Action

1. Enter through the coordination workflow:
   - read this prompt;
   - read `_COORDINATION.md`;
   - perform the baseline authority intake defined in `_COORDINATION.md`,
     including the PRD yardstick sections, the completion plan, the `DAG-007`
     pending-approval note, and the decision register;
   - add application-integration, execution, review, or issuance intake
     documents according to the selected tranche type;
   - run `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary` until `_DAG/_LATEST.md` is human-approved to point elsewhere;
   - record `git status --short` before coordination-sensitive planning or
     execution.
2. Use the Application Integration And Issuance Loop in `_COORDINATION.md` as
   the governing workflow. If a human-approved implementation, app-integration,
   review, issuance, or release-readiness tranche is already active, continue
   that tranche within its write bounds.
3. If no active human-approved tranche exists, propose exactly one next bounded
   tranche:
   - select the earliest unblocked item on the completion plan's dependency
     spine for the current target stage (R4/Phase D per `DEC-048`). C1-C4,
     C5, and the R3 exit evidence packet are complete; `D-23` Option O-A is
     ruled and Phase D/R4 work is now ordinary in-stage work. D1 bend-object
     app absorption landed for the invented preview path through
     `TP-R4-D1-BENDVIS-001` and `TP-R4-D1-BENDSTRESS-001`; D2 branch-object
     app absorption landed through `TP-R4-D2-BRANCHSTRESS-001`; D3 rigid /
     semi-rigid component app absorption landed through
     `TP-R4-D3-RIGIDVIS-001` with `mechanics_geometry_only`
     data/provenance/diagnostic evidence and no frame-stiffness behavior
     change. D4 expansion-joint app absorption landed through
     `TP-R4-D4-EJSTIFF-001`, `TP-R4-D4-EJMACRO-001`, and
     `TP-R4-D4-EJTHRUST-001`: user-stiffness review rows, the dedicated
     `DEC-045` EJ solver macro-element, and explicit load-side
     pressure-thrust evidence from user effective area plus named pressure
     loads are now present for invented mechanics.
     D5 spring-hanger user-data model evidence landed through
     `TP-R4-D5-HANGERDATA-001` under `DEC-049`, with invented variable spring
     and constant-effort support records and no catalog sizing, protected
     defaults, hidden support values, or professional/code-compliance claim.
     D6/D9 residuals after `TP-R4-D6-LOOPCORE-001`,
     `TP-R4-D6-PHYSINTEG-001`, `TP-R4-D9-ASSEMBLEDSEED-001`,
     `TP-R4-D9-FRICTIONSEED-001`, `TP-R4-D9-FRICTIONSLIDE-001`,
     `TP-R4-D6-FRICTIONNORMAL-001`, and
     `TP-R4-D9-BRANCHASSEMBLY-001`, `TP-R4-D9-CONVOBS-001`,
     `TP-R4-D9-CONVPOLICY-001`, `TP-R4-D6-LIVEBUNDLE-001`, and
     `TP-R4-D9-FREEDOFRESIDPOLICY-001`: the loop-core crate
     `core/solver/nonlinear_integration` exists under `DEC-044`/`DEC-046`, the
     first invented product/app/result-envelope sidecar is landed, explicit
     friction normal-reaction input evidence is visible without being combined
     as a load effect for sticking and sliding fixtures, and derived normal
     evidence is now sourced from named restrained support DOFs. The PRD §16.2
     branch-assembly benchmark is landed as public-original mechanics evidence.
     Structured observed convergence values are recorded for the current
     assembled fixtures, and the governed active-set-count policy is promoted
     for that assembled validation seed only. Free-DOF force/moment residual
     threshold policies are promoted for the current assembled validation seed
     and invented product-preview surface only. Mixed one-way/gap/friction
     product live-loop coverage is landed for one invented dense solve.
     Accepted multi-support validation companion coverage now includes
     one-way/gap, lift-off/gap, and friction/gap public-original fixtures under
     the narrow `DEC-046` multisupport active-set, free-DOF force/moment, and
     free-DOF work policies.
     `TP-R4-D7-SPARSELIVE-001` landed the `DEC-050` live sparse evidence lane
     while dense remains default. `TP-R4-D7-SPARSEPROFILE-001` added direct
     reduced profile-entry assembly for product-preview sparse evidence rows
     while dense remains the product solve path and parity oracle.
     `TP-R4-D9-DISPREACTIONOBS-001` records displacement/reaction-delta
     observation axes for the current assembled validation seed, accepted
     multi-support fixture set, and product-preview metadata; accepted delta
     thresholds remain `TBD`.
     `TP-R4-D9-EXITGAP-001` records the current R4
     exit verdict as not ready: non-seed force/displacement/energy threshold
     promotion, accepted displacement/reaction-delta thresholds, multi-DOF /
     multi-support nonlinear fixture depth, default sparse promotion and
     nonlinear/core profile-direct sparse promotion, deeper spring-hanger
     behavior, and the remaining D9 validation
     package remain open. `D-20` is held for
     Phase E; `D-21` remains held and does not authorize
     v0.2/R6/R7 scope. Continue to honor the
     R3/R4 boundaries: no protected standards content, no private-data default
     writes, no network/telemetry feature, no live external SDK/harness or
     `D-21` promotion, and no release-readiness/professional/code-compliance
     claim;
   - if that item is blocked by a human decision, run the loop's
     decision-escalation step: prepare the decision packet if none exists;
     if a packet already awaits ruling, take the next unblocked
     implementation item;
   - if no current-stage plan item remains unblocked, stop: prepare any
     missing decision packets, finish any validated work in hand, route it
     through autonomous `CHANGE` closeout, end the session with the
     pending-rulings summary, and await the human ruling — do not substitute
     out-of-stage scope or unrelated hardening work;
   - prefer regression repair (failed or insufficient tests, build breaks,
     smoke-evidence gaps) ahead of new scope, and residual hardening only
     when it blocks or de-risks a current-stage plan item;
   - use `CHECKING` deliverables as mature design and review context;
   - select from `CHECKING` only when the human asks for lifecycle closeout or
     issuance review; do not select `ISSUED` deliverables for ordinary work.
4. For app-integration work, inspect current app/build surfaces first:
   `package.json`, `apps/desktop/package.json`, `apps/desktop/src/**`,
   `apps/desktop/src-tauri/**`, `apps/desktop/SMOKE.md`,
   `core/product_physics/**`, and `fixtures/product_preview/**` as needed.
   Use the current approved DAG pointer only to discover upstream/downstream
   deliverable context, then inspect the relevant deliverable-local files
   directly.
5. Inspect in detail any app, core implementation, fixture, test, or evidence
   files needed to decide what to do next.
6. A working desktop application tranche preserves the boundary prohibitions:
   local-only operation with no cloud, daemon, network, telemetry, or
   repository-default private-data writes; invented bundled fixtures;
   user-created models stay in local project storage and are never committed
   to the repository; no protected standards content or private project data;
   and no release-readiness, professional approval, certification, sealing,
   authentication, or code-compliance claims. Interactive authoring of
   user-created local models remains in scope for the current R4 target stage
   and does not violate these prohibitions.
7. End every session summary with the decision packets awaiting human ruling
   (from `execution/_Coordination/_DECISIONS/_REGISTER.md`) and the next
   unblocked plan item.
8. At completion of a validated tranche, record evidence, update the
   completion-plan and decision-register rows the tranche affects —
   compressing landed items per the plan-maintenance rule (one line plus
   run-record and `plans/PLAN_COMPLETION_LOG.md` pointers) — then
   autonomously hand off to a `CHANGE` agent/subagent for final Git/file-state
   review under `{WORKING_ROOT}/AGENTS.md` closeout discipline. Git closeout
   is source control hygiene only; it is not lifecycle issuance, release
   readiness, professional approval, certification, sealing, authentication,
   or code-compliance acceptance.
