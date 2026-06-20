# NEXT INSTANCE PROMPT

## Current Authority

- Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`.
- Set `WORKING_ROOT` to `{REPO_ROOT}/projects/chirality-piping`.
- Read `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md`.
- Read `{WORKING_ROOT}/AGENTS.md` for project-local agent posture and closeout discipline.
- Read `{WORKING_ROOT}/execution/_Coordination/_COORDINATION.md` and follow the Application Integration And Issuance Loop directions.
- Read `docs/PLAN.md` — the non-governing strategic roadmap — for the definition of "complete per the PRD", the current milestone position, the layer-relation map, and roadmap-level risks. It is strategic orientation that routes to the authorities; the completion plan below remains the tactical selection instrument.
- Read `plans/PLAN_2026-06-17_prd_completion.md` — the current completion plan (non-governing `PROPOSAL`, supersedes the 2026-06-10 plan) — for phase ordering toward the PRD, the dependency spine, the FR completion map, and the human decision register `D-01..D-22` (with `DEC-041/042/043` recorded 2026-06-18: the embedded-agent substrate ruling, the D-21 held-with-prep disposition, and the piping-design equation-reliability constraint).
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
6. Human decisions (`D-01..D-22` and any newly discovered human-gated `TBD`)
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
     spine for the current target stage (R3/Phase C per `DEC-035`). C1-C4
     are landed; continue **Phase C5 — R3 exit readiness and
     authoring-usability closure** before any Phase D work unless the human
     explicitly overrides. C5.1-C5.6 are landed. **The C5.7 human packaged
     pass was run and FAILED on usability (2026-06-18): "a wall of
     evidence/telemetry strings, three competing navigation systems, the model
     a postage stamp" — F-4 and the A3 authoring-usability finding stay open.**
     The human-approved repair tranche **C5.7R** has Inc 0-7 landed: Inc 0-5
     are recorded under `TP-R3UX-WORKSPACEREDESIGN-001` / SMOKE TP-MAC-272 /
     DEL-07-06 `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-WORKSPACEREDESIGN-001.md`;
     Inc 6 Grid mode is recorded under `TP-R3UX-GRIDMODE-001` / SMOKE
     TP-MAC-273 / DEL-07-06
     `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-GRIDMODE-001.md`; Inc 7 packaged
     kit is recorded under `TP-R3UX-PACKAGEKIT-002` / SMOKE TP-MAC-274 /
     DEL-07-06
     `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-PACKAGEKIT-002.md`.
     **Further human-directed "professional grade" CAD-shell increments
     landed on top of Inc 0-7** (frontend + native-menu only, per
     `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md` §10): the CAD-grade
     3D viewport (`TP-R3UX-VIEWPORT-001` / DEL-07-06
     `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-VIEWPORT-001.md`); Phase 2 tranche
     2a, a native + in-DOM menu-bar IA with a collapsible viewport-dominant
     dock (`TP-R3UX-CADSHELL-001` /
     `WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-CADSHELL-001.md`); and Phase 2
     tranche 2b, collapsible tree/inspector rails with inspector auto-open on
     selection (`TP-R3UX-CADSHELL-002` /
     `WORKING_ITEMS_RUN_2026-06-20_TP-R3UX-CADSHELL-002.md`). Each is
     gates-green (tsc; vitest 406/406; e2e 18/18; cargo check; `.app`
     rebuilt/boots clean) and committed to `main`.
     **The remaining builder increment is tranche 2c — an object-creation
     toolbar that makes Insert commands arm a creation tool rather than only
     navigate.** After 2c, rebuild the `.app`; the packaged C5.7 human re-pass
     (target SMOKE TP-MAC-189) remains the gate and must be performed by the
     human and recorded before F-4/A3 can close or C5.8 can begin. Honor the
     spec's scope boundaries:
     agent-panel seam reserved-not-built (`D-21`); `DEC-037` (structured rule
     composer, no writable text); dual units display-only; no new engine
     contracts (Grid mode fans existing `EditorOperationIntent`s). The Phase B
     unit-aware-I/O remainder is allowed alongside only when it does not block
     or displace C5;
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
   user-created local models is in scope for the current R3 target stage and
   does not violate these prohibitions.
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
