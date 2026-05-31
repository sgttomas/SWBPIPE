# Coordination Record

## Active Surface

Keep this surface lean. Use canonical project instructions, approved DAG
artifacts, and deliverable-local records as the active state.

Primary pointers:

- `AGENTS.md`
- `execution/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/_Registers/Deliverables.csv`
- `execution/_DAG/_LATEST.md`
- `execution/_DAG/DAG-005/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-005/DAG_Audit.md`
- `execution/_DAG/DAG-005/DependencyEdges.csv`
- `execution/_DAG/DAG-005/DeliverableNodes.csv`
- `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md` when present
- `execution/PKG-*/1_Working/DEL-*/MEMORY.md`
- `execution/PKG-*/1_Working/DEL-*/_STATUS.md`

## Current Authority

`DAG-005` is approved graph authority as of 2026-05-18. Approval applies to the
active edge set only. Candidate rows remain non-gating unless a later explicit
human gate promotes them and the graph is revalidated.

`DAG-005` is oriented toward export-format contract fulfillment from SCA-004.
The 8 `PKG-00` deliverables remain architecture-basis context, not
implementation work.

`DEV-001` remains the current development path. The blocker queue has been
recomputed from approved `DAG-005`: 101 unblocked, 0 blocked. TP-EXPORT-CLOSEOUT-001
refreshed PKG-17 implementation evidence for `DEL-17-07`, `DEL-17-08`, and
`DEL-17-09`; lifecycle states remain unchanged.

## State Tracking Rules

Use two layers of state. Do not let handoff prose become substitute authority.

Authoritative state:

1. `execution/_Decomposition/SOFTWARE_DECOMP.md` says what must be built and why.
2. `execution/_DAG/DAG-005/` says what depends on what, using approved active
   edges only. Candidate rows remain non-gating unless explicitly promoted by a
   later human gate and graph revalidation.
3. `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv` records
   committed implementation evidence. `DEV-001_BLOCKER_QUEUE.md/.csv` are
   deterministic derivative views of blocked/unblocked implementation state.
4. Deliverable-local `_STATUS.md`, `MEMORY.md`, and `_run_records/**` carry
   lifecycle, working memory, and execution evidence inside each deliverable's
   ownership boundary.

Handoff state:

- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` contains stable entry
  instructions for the next agent instance.
- `execution/_Coordination/NEXT_INSTANCE_STATE.md` contains compact resume
  state: authority pointers, current program state, immediate next actions,
  residual blockers, and explicit do-not-change boundaries.
- `NEXT_INSTANCE_STATE.md` may summarize authoritative state but must cite the
  owning authoritative artifacts. It must not replace decomposition truth, DAG
  authority, implementation evidence, blocker queue derivatives, lifecycle
  files, release records, or professional/acceptance records.

At session closeout, update affected deliverable-local `MEMORY.md` and
`_run_records/**`, then update `NEXT_INSTANCE_STATE.md` with only current
pointers, completed work, residual gaps, validation state, and next action.
Leave `NEXT_INSTANCE_PROMPT.md` mostly stable unless the entry protocol itself
changes.

## Integrated Verification And Tranche Selection Loop

Use this loop as the default OpenPipeStress development driver until the
`SOFTWARE_DECOMP` objectives are closed. It is project-specific, but general
across packages and deliverables. If a human has already approved an
implementation tranche, continue at the execution step; otherwise use the
verification and gap-selection steps to propose exactly one next bounded
tranche.

1. **Authority intake.** Read `NEXT_INSTANCE_PROMPT.md`,
   `NEXT_INSTANCE_STATE.md` when present, this coordination record, and the
   active surface. `SOFTWARE_DECOMP` defines what must be built and why;
   `DAG-005` defines approved active dependencies; `DEV-001` records committed
   implementation evidence and derived blocked/unblocked state.
2. **State verification.** Before coordination-sensitive planning, run
   `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
   so stale derivative blocker queues are visible. Record
   `git status --short` so evidence is tied to the working-tree state. For
   release-readiness or whole-surface gap selection, run or propose a read-only
   verification tranche that runs
   `python3 tools/release/check_release_readiness.py --profile all --execute`
   plus supplemental local checks not covered by that script, currently
   `npm run test:desktop` and `npm run build:desktop` when the desktop package
   is in scope.
3. **Verification snapshot.** When the approved tranche is a read-only
   integrated verification sweep, write only derivative audit evidence under
   `execution/_Aggregation/TP-INTEGRATED-VERIFY-###_YYYY-MM-DD/`. Expected
   closeout files are `RUN_SUMMARY.md`, `Verification_Results.md`,
   `Gap_Register.csv`, and `Source_Index.csv`. The snapshot may compare results
   against accepted evaluation findings and latest package refresh evidence,
   including PKG-17 refresh evidence when relevant.
4. **Gap-to-tranche decision.** Map each observed failure or gap to the owning
   `PackageID`, `DeliverableID`, objective, active `DAG-005` dependency status,
   and evidence source. Recommend exactly one next bounded tranche with
   objective, scope, write bounds, validation commands, expected closeout, and
   any useful `TASK` fan-out. Do not implement until the human approves or
   redirects the tranche.
5. **Bounded execution.** After approval, dispatch canonical `TASK` workers
   only where the file sets are disjoint and the write scopes are explicit.
   Normally use one deliverable or one clearly owned slice per worker. Any brief
   with `DeliverablePath` uses `TASK` deliverable-local mode and must read
   `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`,
   `MEMORY.md`, and primary deliverable artifacts before acting.
6. **Fan-in and validation.** The parent agent fans in worker results, checks
   for scope drift, runs targeted validation, and reruns broader release
   readiness checks when the tranche affects release surface. Record pass,
   fail, waived, or deferred evidence without overstating acceptance.
7. **Evidence and derivatives.** Update affected deliverable-local `MEMORY.md`
   and `_run_records/**`. Edit `DEV-001_IMPLEMENTATION_EVIDENCE.csv` only when
   explicitly authorized and backed by committed evidence. After approved
   implementation-evidence edits, run
   `python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --write`
   to refresh derivative blocker queue mirrors, then rerun `--check`.
8. **Handoff.** Update `NEXT_INSTANCE_STATE.md` with current authority
   pointers, completed work, residual gaps, validation state, and the
   recommended next action. Leave `NEXT_INSTANCE_PROMPT.md` stable unless the
   entry protocol itself changes.

The coordination maintenance tool is deterministic TOOLMAKER-style support: it
validates evidence and regenerates blocker queue derivatives, but it does not
approve or edit implementation evidence rows, lifecycle states, candidate rows,
release claims, professional claims, or graph authority.

Human approval is required for lifecycle changes, candidate promotion, commits,
release claims, acceptance records, or any professional/code compliance claim.
Read-only verification snapshots and derivative gap registers are not release,
professional, code-compliance, or acceptance claims.

Completion requires accepted implementation evidence for all applicable
`SOFTWARE_DECOMP` objectives, satisfied active `DAG-005` dependencies, passing
or explicitly waived readiness checks, closed or formally deferred residual
gaps, and the required human gate for any release, acceptance, professional, or
code-compliance assertion.
