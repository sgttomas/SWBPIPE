# NEXT INSTANCE PROMPT

## Current Authority

- Read `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/_COORDINATION.md` and follow the Application Integration And Issuance Loop directions.
- Read `plans/PLAN_2026-06-10_prd_completion.md` — the current completion plan (non-governing `PROPOSAL`) — for phase ordering toward the PRD, the dependency spine, the FR completion map, and the human decision register `D-01..D-12`.
- Discover current state from authoritative surfaces named in `_COORDINATION.md`: `_DAG/_LATEST.md`, approved DAG artifacts, deliverable-local `_STATUS.md`, `MEMORY.md`, `_run_records/**`, dependency/review files, current app/build/test surfaces, `execution/_Coordination/_DECISIONS/_REGISTER.md`, and current aggregation or review artifacts as needed.
- Treat blockers or dirty git state outside this project/write scope as external-scope noise; record and bypass, do not fix.

## Governing Imperatives

1. `SOFTWARE_DECOMP` says what must be built and why.
2. `DAG-006` says what depends on what, using approved active edges.
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
   workers, fan-in, validation, evidence records, and git commit/push closeout
   for completed validated tranches.
6. Human decisions (`D-01..D-12` and any newly discovered human-gated `TBD`)
   are never resolved by agents. Prepare decision packets per the loop's
   decision-escalation step, surface pending rulings every session, and
   proceed only on unblocked work.
7. Current lifecycle state is application-integration ready: all deliverables
   are `CHECKING` or `ISSUED`; `DEL-01-01` is the sole currently `ISSUED`
   deliverable and remains the accepted governance baseline unless a
   human-approved change path opens it.

## Next Action

1. Enter through the coordination workflow:
   - read this prompt;
   - read `_COORDINATION.md`;
   - perform the baseline authority intake defined in `_COORDINATION.md`,
     including the PRD yardstick sections, the completion plan, and the
     decision register;
   - add application-integration, execution, review, or issuance intake
     documents according to the selected tranche type;
   - run `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary`;
   - record `git status --short` before coordination-sensitive planning or
     execution.
2. Use the Application Integration And Issuance Loop in `_COORDINATION.md` as
   the governing workflow. If a human-approved implementation, app-integration,
   review, issuance, or release-readiness tranche is already active, continue
   that tranche within its write bounds.
3. If no active human-approved tranche exists, propose exactly one next bounded
   tranche:
   - select the earliest unblocked item on the completion plan's dependency
     spine for the current target stage (R3/Phase C per `DEC-035`: C2
     rule-pack editor GUI — AST-based per `DEC-022`, D-02b packet at the C2
     lead-up — then C3 and C4; the A3 usability lane and B2/B3 units
     remainder run alongside, A3 feeding the R3-exit blocking residual);
   - if that item is blocked by a human decision, run the loop's
     decision-escalation step: prepare the decision packet if none exists;
     if a packet already awaits ruling, take the next unblocked
     implementation item;
   - if no current-stage plan item remains unblocked, stop: prepare any
     missing decision packets, finish and commit validated work in hand,
     end the session with the pending-rulings summary, and await the human
     ruling — do not substitute out-of-stage scope or unrelated hardening
     work;
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
   Use `DAG-006` only to discover upstream/downstream deliverable context, then
   inspect the relevant deliverable-local files directly.
5. Inspect in detail any app, core implementation, fixture, test, or evidence
   files needed to decide what to do next.
6. A working desktop application tranche preserves the boundary prohibitions:
   local-only operation with no cloud, daemon, network, telemetry, or
   repository-default private-data writes; invented bundled fixtures;
   user-created models stay in local project storage and are never committed
   to the repository; no protected standards content or private project data;
   and no release-readiness, professional approval, certification, sealing,
   authentication, or code-compliance claims. Interactive authoring of
   user-created local models is in scope for the current R3 target stage and
   does not violate these prohibitions.
7. End every session summary with the decision packets awaiting human ruling
   (from `execution/_Coordination/_DECISIONS/_REGISTER.md`) and the next
   unblocked plan item.
8. At completion of a validated tranche, record evidence, update the
   completion-plan and decision-register rows the tranche affects —
   compressing landed items per the plan-maintenance rule (one line plus
   run-record and `plans/PLAN_COMPLETION_LOG.md` pointers) — then stage,
   commit, and push the tranche to the tracked remote branch unless the human
   pauses git closeout or a real validation/git blocker prevents it. Git
   closeout is source control hygiene only; it is not lifecycle issuance,
   release readiness, professional approval, certification, sealing,
   authentication, or code-compliance acceptance.
