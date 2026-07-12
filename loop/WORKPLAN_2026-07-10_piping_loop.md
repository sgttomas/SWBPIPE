# Piping Work Loop — standing plan (development loop instructions)

> **Epistemic status: agent-authored plan — not authority.** Written at owner adoption
> (Ryan Tufts, K-AUTH-1) on 2026-07-10; supersedes `WORKPLAN_2026-07-04_piping_loop.md`
> in this directory. This plan never authorizes work: owner-adopted briefs, decision
> rulings, and directions do. Sources govern on any disagreement. This file is the
> project's single development-loop instruction surface — PROTOCOL plus pointer
> indexes; it carries NO status, NO work history, and NO measurements. Each iteration
> re-derives state from the live tree; loop closes append a minimal receipt to
> `LOOP_RECEIPTS.md` beside this file (rules live at the top of that file).

## Owner intent

- **2026-07-04** (Receipt 0): *"All projects should have the current development loop
  structure and workflow, and therefore same type of init prompt."* — this loop is the
  session entry convention (thin launcher → `LOOP_INIT.md` → this plan → receipts).
- **2026-07-09** (Receipt 3): the physical-model mechanics program is adopted
  (`DEC-066`–`DEC-070`); the loop pursues physical-model correctness alongside PRD
  completion. Method forks and thresholds stay owner-gated as register rows.
- **2026-07-10** (Receipt 11): consolidation adopted — *deliverables themselves are the
  means of discovering work*; this plan absorbs the operative mechanics of the
  Application Integration And Issuance Loop; `execution/_Coordination/_COORDINATION.md`
  is reduced to a ruled-record surface; `plans/` is retired as a selection surface
  (open rows rehomed into deliverable-local `_STATUS.md` `## Remaining` sections).

**The loop's goal** (agent-phrased; owner may correct the wording): advance
chirality-piping toward completion per the PRD yardstick (`docs/PRD.md` §10 functional
requirements, §22 release milestones) and physical-model correctness, as far as live
authority permits — stopping at every owner gate.

## The loop protocol (every iteration)

0. **Discover (mandatory core).** Clean `git` state and branch awareness (log since
   the last receipt; concurrent loops may be live in this monorepo — keep write scopes
   disjoint from `_DomainEngines/**`, `projects/chirality-app-dev/**`,
   `projects/pec/**`, and treat unrelated dirty files as external state). Read the
   latest receipt(s) in `LOOP_RECEIPTS.md`. Check
   `execution/_Coordination/_DECISIONS/_REGISTER.md` for rulings newer than the last
   receipt — new rulings are how work unlocks, look every time. Re-derive the current
   target stage from `execution/_Coordination/_COORDINATION.md` (the ruled recording
   surface) and the newest `DEC-*` rows. Enumerate the work surface:
   `PYTHONDONTWRITEBYTECODE=1 python3 tools/coordination/list_deliverable_status.py
   --dag <current from execution/_DAG/_LATEST.md> --format table --summary`. Run the
   repo-wide harness `self-check`. Verify any derivative statement — including this
   plan and your own tasking — against the live tree before relying on it; on
   disagreement the live tree wins and the delta goes in the receipt.
1. **Select — from deliverable folders only.** A deliverable is *selectable* when all
   three hold: (i) DAG-unblocked — every `EXECUTION UPSTREAM` edge satisfied, computed
   from the approved DAG's `DependencyEdges.csv` or the deliverable-local
   `Dependencies.csv` (note: `TopologicalWaves.md` lists waves leaf-first — read it in
   descending wave order, or recompute); (ii) lifecycle not `ISSUED`; (iii) its
   `_STATUS.md` `## Remaining` section names open scope (an absent section means no
   recorded remaining work beyond the deliverable's `Specification.md`). Items marked
   `(gated: D-XX)` or `(stage-gated: ...)` are not selectable until the named gate is
   ruled or the register advances the stage. Selection principles, in order:
   (a) repair failing validation for already-landed work before new scope; (b) work
   that discharges a gate prerequisite beats work that doesn't; (c) owner-directed
   items beat agent-inferred ones; (d) the earliest DAG-unblocked current-stage
   deliverable with open `Remaining` scope; (e) residual hardening only when it blocks
   or de-risks a current-stage item. Never manufacture out-of-stage or unrelated work
   to stay busy.
2. **Brief / slate.** Deliverable work gets a CANDIDATE brief (scope, acceptance
   basis, evidence plan, write fence). Human-gated forks get a decision packet
   (options + non-binding recommendation + on-ruling mechanism) registered in the
   decision register; if a packet already awaits ruling, take the next unblocked item.
3. **Gate.** STOP; adoption/ruling/direction is the owner's act (K-AUTH-1; D-GOV-04).
   In-session directions/rulings are quoted verbatim in the receipt and recorded in
   their governed artifact as part of execution — except directions fully re-derivable
   from git/PR history (e.g. bare merge directions), per the receipt ledger's local
   rule 2. Record every gate outcome — including no-ops and their reason.
4. **Execute + check.** *Intake:* read the selected deliverable's folder (`_STATUS.md`
   incl. `Remaining`, `MEMORY.md`, `_CONTEXT.md`, `_REFERENCES.md`, dependency files,
   the four-document kit, recent `_run_records/**`) and active DAG-discovered
   dependencies only — do not trawl governance deliverable folders for project-wide
   policy. *Multi-agent execution:* derive the current work graph under
   `LOOP_INIT.md` §7. HELP_HUMAN may activate one WORKING_ITEMS instance per package;
   a directly invoked WORKING_ITEMS instance remains within one package. Each Agent 2
   brief names its deliverable scope, parent package, acceptance basis, evidence plan,
   and read/write bounds. Use terminal or supervised many-to-many coordination as the
   graph requires; validate staged and terminal fan-in before downstream use.
   *Discipline:* branch-first + PR unless the owner directs otherwise; never
   self-merge; write scopes stay inside `projects/chirality-piping/**` unless the
   owner grants wider scope. *Validation:* the selected work type's evidence
   conventions (test plans, run records); the `DEC-025` five-surface evidence sweep
   (`python3 tools/release/run_evidence_sweep.py --execute`) as the pre-push/fan-in
   merge gate for every code-touching branch; UI slices carry the H4 desktop evidence
   posture recorded in `_COORDINATION.md`'s ruled workflow records (Playwright e2e
   default, Vitest unit tests, `apps/desktop/SMOKE.md`-only evidence as the recorded
   exception);
   repo-wide `self-check` exit 0 and full practitioner-harness pytest at closeout.
   CI green; owner merges.
5. **Update deliverable state + receipt.** Closeout writes deliverable-local state:
   `_STATUS.md` (lifecycle history per ruled gates, and `Remaining` updated to reflect
   what landed — landed scope is removed, newly named residuals are added),
   `MEMORY.md`, and `_run_records/**`. Lifecycle transitions follow the register's
   ruled gates (F-PIP-3); `CHECKING → ISSUED` routing follows the `DEC-062`
   issuance-wave structure (W1–W7, five-point evidence bar, owner-paced). Append a
   minimal receipt to `LOOP_RECEIPTS.md` per its local rules. Next iteration starts
   at 0.

## Standing constraints — fences (all iterations)

- **F-PIP-1 (boundary prohibitions):** local-only operation — no cloud, daemon,
  network, or telemetry features; no repository-default private-data writes;
  user-created models never committed; no protected standards content or private
  project data; invented bundled fixtures only.
- **F-PIP-2 (claims):** no release-readiness, professional approval, certification,
  sealing, authentication, or code-compliance claims. Git closeout is source-control
  hygiene, not lifecycle issuance.
- **F-PIP-3 (lifecycle):** deliverable lifecycle transitions follow the register's
  ruled gates; no `CHECKING -> ISSUED` issuance without the owner's gate; the
  currently `ISSUED` baseline is opened only through a human-approved change path.
- **F-PIP-4 (scope-gated integrations):** live external SDK/harness promotion,
  domain-engine bindings, and version-scope promotions stay behind their named
  register rows; tier-0/domain-engine surfaces (`_DomainEngines/**`) belong to their
  own loops — this loop never writes them.
- **F-PIP-5 (single-surface rule; owner-adopted 2026-07-10):** work items live in
  deliverable folders (`_STATUS.md` `## Remaining`); owner decisions live in
  `execution/_Coordination/_DECISIONS/_REGISTER.md`; no new standing plan, register,
  or status surface is created without an owner ruling. `plans/` is a historical
  archive — never select work from it.

## Where live work is re-derived (pointer index — never a status surface)

- **Work surface (discovery + selection):** `execution/PKG-*/1_Working/DEL-*/` —
  `_STATUS.md` (lifecycle + `## Remaining` open scope), `MEMORY.md`, `_CONTEXT.md`,
  `Specification.md`, `_run_records/**` — enumerated via
  `tools/coordination/list_deliverable_status.py`.
- **Ruled records:** current target stage in
  `execution/_Coordination/_COORDINATION.md` (ruled-record stub; also carries the
  `DEC-040` state-tracking augmentation) · decision register
  `execution/_Coordination/_DECISIONS/_REGISTER.md` (`D-XX` rows; packets beside it) ·
  codified rulings `execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
- **Dependency authority:** `execution/_DAG/_LATEST.md` → the approved DAG it names.
- **What must be built and why:** `execution/_Decomposition/SOFTWARE_DECOMP.md` ·
  product yardstick: `docs/PRD.md` (§10, §22) · strategic orientation: `docs/PLAN.md`
  (non-governing).
- **Agent posture:** `AGENTS.md` (dispatch contract, closeout discipline).
- **Evidence:** `_run_records/**`, `validation/evidence/**`, deliverable-local files.
- **Historical archive (never selection surfaces):** `plans/**` (open rows rehomed to
  deliverable `_STATUS.md` on 2026-07-10; see `plans/PLAN_COMPLETION_LOG.md`) ·
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` (pre-loop session entry — dated
  historical map, never authority).
- **Handoff context** (owner directions, gate outcomes, stale-map deltas):
  `LOOP_RECEIPTS.md` beside this file.
