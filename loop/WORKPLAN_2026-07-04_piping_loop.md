# Piping Work Loop — standing plan (PRD-completion development loop)

> **SUPERSEDED 2026-07-10** by `WORKPLAN_2026-07-10_piping_loop.md` in this directory
> (owner-adopted consolidation; Receipt 11). Retained as a dated record only — the
> newest `WORKPLAN_*.md` governs. Do not follow this file's protocol or pointers.

> **Epistemic status: agent-authored plan — not authority.** Written at owner direction
> (Ryan Tufts, K-AUTH-1) on 2026-07-04. This plan never authorizes work: owner-adopted
> plans, decision rulings, and directions do. Sources govern on any disagreement. This
> file is a PROTOCOL plus pointer indexes; it carries NO status, NO work history, and NO
> measurements — each loop iteration re-derives state from the live tree, and loop closes
> append a minimal receipt to `LOOP_RECEIPTS.md` beside this file (rules live at the top
> of that file).

## Owner intent (recorded 2026-07-04)

Owner direction of record (quoted verbatim in `LOOP_RECEIPTS.md` Receipt 0): *"All
projects should have the current development loop structure and workflow, and therefore
same type of init prompt."* — converting this project's session entry from the
status-laden `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` convention to the current
loop convention (thin launcher → `LOOP_INIT.md` → this plan → receipts).

**The loop's goal** (agent-phrased under that direction; owner may correct the wording):
advance chirality-piping toward completion per the PRD yardstick (`docs/PRD.md` §10
functional requirements, §22 release milestones), through the coordination workflow and
the current completion plan, as far as live authority permits — stopping at every owner
gate. The pre-existing workflow authority is unchanged: `_COORDINATION.md`'s Application
Integration And Issuance Loop remains how work is executed; this loop wraps it in the
current session convention, it does not replace it.

## Owner intent addendum (recorded 2026-07-09)

Owner direction of record (quoted verbatim in `LOOP_RECEIPTS.md` Receipt 3): the
physical-model mechanics program is adopted — bend/branch flexibility factors enter
the global stiffness ("introduce flexibility factors and bend flexibility together and
now properly solve those elements"; implementation chosen over disclosure), the
nonlinear-support repair, workflow-physics items, and PRD domain-naming are approved
(`DEC-066`–`DEC-069`). The loop pursues **physical-model correctness alongside PRD
completion**; tactical routing lives in the mechanics plan named in the pointer index
below. Method forks and thresholds stay owner-gated as ordinary register rows
(currently `D-34`).

## Why any staging remains (the compelling reasons)

1. **Adoption, ruling, and direction are human acts** (K-AUTH-1; D-GOV-04). The decision
   register's human-gated rows (`D-XX` series) are never resolved by agents — prepare
   packets, surface pending rulings, proceed only on unblocked work.
2. **Stage boundaries bind regardless of ambition** — the current target stage and its
   prohibitions are re-derived from the decision register (`DEC-*` records) each
   iteration, never assumed from this file.
3. **Fences below** bound every tranche.

## The loop protocol (every iteration)

0. **Discover (mandatory core).** Clean `git` state and branch awareness (log since the
   last receipt; concurrent loops may be live in this monorepo — keep write scopes
   disjoint from `_DomainEngines/**`, `projects/chirality-app-dev/**`, `projects/pec/**`,
   and treat unrelated dirty files as external state). Read the latest receipt(s) in
   `LOOP_RECEIPTS.md` beside this file. Check
   `execution/_Coordination/_DECISIONS/_REGISTER.md` for rulings newer than the last
   receipt — new rulings are how work unlocks, look every time. Re-derive: the current
   approved DAG pointer (`execution/_DAG/_LATEST.md`), deliverable lifecycle states
   (`python3 tools/coordination/list_deliverable_status.py --dag <current> --format table
   --summary`), the current target stage from the newest `DEC-*` rows, and repo-wide
   `PYTHONDONTWRITEBYTECODE=1` harness `self-check`. Verify any derivative statement —
   including this plan and your own tasking — against the live tree before relying on it;
   on disagreement the live tree wins and the delta goes in the receipt.
1. **Select.** The widest lawful tranche(s) now, re-derived. Principles, in order:
   (a) repair failing validation for already-landed work before new scope; (b) work that
   discharges a gate prerequisite beats work that doesn't; (c) owner-directed items beat
   agent-inferred ones; (d) the earliest unblocked item on the completion plan's
   dependency spine for the current target stage; (e) residual hardening only when it
   blocks or de-risks a current-stage item. Never manufacture out-of-stage or unrelated
   work to stay busy.
2. **Brief / slate.** Deliverable work follows `_COORDINATION.md`'s tranche intake and
   evidence conventions. Human-gated forks get a decision packet (options + non-binding
   recommendation + on-ruling mechanism) registered in the decision register; if a packet
   already awaits ruling, take the next unblocked item.
3. **Gate.** STOP; adoption/ruling/direction is the owner's act (K-AUTH-1; D-GOV-04).
   In-session directions/rulings are quoted verbatim in the receipt and recorded in their
   governed artifact as part of execution — except directions fully re-derivable from
   git/PR history (e.g. bare merge directions), per the receipt ledger's local rule 2.
   Record every gate outcome — including no-ops and their reason.
4. **Execute + check.** Branch-first + PR unless the owner directs otherwise; never
   self-merge; write scopes stay inside `projects/chirality-piping/**` unless the owner
   grants wider scope. Checks per `_COORDINATION.md`'s validation requirements and the
   selected work type's evidence conventions (test plans, run records, DEC-025 sweeps
   where the work class requires them), plus repo-wide `self-check` exit 0 and full
   practitioner-harness pytest at closeout. CI green; owner merges.
5. **Receipt.** Append a minimal receipt to `LOOP_RECEIPTS.md` per its local rules —
   pointers, verbatim owner directions, gate outcomes, check pass/fail. No narrative
   here or anywhere else. Next iteration starts at 0.

## Standing constraints — fences (all iterations)

- **F-PIP-1 (boundary prohibitions):** local-only operation — no cloud, daemon, network,
  or telemetry features; no repository-default private-data writes; user-created models
  never committed; no protected standards content or private project data; invented
  bundled fixtures only.
- **F-PIP-2 (claims):** no release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claims. Git closeout is source-control
  hygiene, not lifecycle issuance.
- **F-PIP-3 (lifecycle):** deliverable lifecycle transitions follow the register's ruled
  gates; no `CHECKING -> ISSUED` issuance without the owner's gate; the currently
  `ISSUED` baseline is opened only through a human-approved change path.
- **F-PIP-4 (scope-gated integrations):** live external SDK/harness promotion, domain-
  engine bindings, and version-scope promotions stay behind their named register rows;
  tier-0/domain-engine surfaces (`_DomainEngines/**`) belong to their own loops — this
  loop never writes them.

## Where live work is re-derived (pointer index — never a status surface)

- **Workflow authority:** `execution/_Coordination/_COORDINATION.md` (the Application
  Integration And Issuance Loop; intake, bounded workers, fan-in, validation, evidence,
  closeout routing) · project agent posture: `AGENTS.md`.
- **What must be built and why:** `SOFTWARE_DECOMP.md` · product yardstick: `docs/PRD.md`
  (§10, §22) · strategic orientation: `docs/PLAN.md` (non-governing).
- **Tactical selection:** the newest completion plan under `plans/`
  (`PLAN_*_prd_completion.md` class; non-governing PROPOSAL — routes to the
  authorities) · the physical-model mechanics program plan
  `plans/PLAN_2026-07-09_physical_model_mechanics.md` (same class; owner-adopted
  scope per `DEC-066`–`DEC-069`) · completion log: `plans/PLAN_COMPLETION_LOG.md`.
- **Decision register** (open rows are the owner-gated surface):
  `execution/_Coordination/_DECISIONS/_REGISTER.md` (`D-XX` rows; `DEC-*` records carry
  the ruled outcomes and current target stage).
- **Dependency authority:** `execution/_DAG/_LATEST.md` → the approved DAG it names.
- **Lifecycle state:** deliverable-local `_STATUS.md` files, enumerated via
  `tools/coordination/list_deliverable_status.py`.
- **Evidence:** `_run_records/**`, `validation/evidence/**`, deliverable-local files.
- **Legacy session entry (historical):** `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
  — the pre-loop, status-laden entry document. Treat as a dated map, never authority;
  the register and live tree govern on any disagreement.
- **Handoff context** (owner directions, gate outcomes, stale-map deltas):
  `LOOP_RECEIPTS.md` beside this file.
