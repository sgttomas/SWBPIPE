# Sealed brief — B3 CI tooltip/Close repair

Parent ROOT HELP_HUMAN; WORKING_ITEMS Type1, existing /root/b3_manager,
gpt-6-astra/high. This is the bounded continuation of B3 required for PR825.
The wider UI undertaking remains paused by the owner. No B3A/B3B/I1 or Runtime.

Resolve REPO_ROOT in swbpipe-wt3 with git rev-parse; WORKING_ROOT is
projects/chirality-piping; RUN is the existing HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION.
Begin by verifying clean c613922f1725a9b86d81c6c120b05b7df2e0558d and preserve all
work. Integrate reviewed PR head002dff0f244976f98b36517d920b3761f6f88704 at a clean
point. Do not change ROOT's wt2 (which owns CI edits).

Read the latest Root/Piping AGENTS, WORKING_ITEMS role, loop, approved strategy
and OWNER_MODEL_DIRECTION. Read ROOT's new OWNER_CI_REPAIR_DIRECTION.md and
_run_records/PR825_CI_FAILURE/MANIFEST.json plus compressed raw failed log.
Their exact supplied revision/brief hash are in the launch message.
Diagnosis and code-review skills are selected as applicable; retain their hashes.

Observed failure: full hosted Linux/Chromium run35496887667 on002dff0f:
408 passed,20 existing skips,2 failed. Both profiles time out at
apps/desktop/e2e/b3-accessibility.spec.ts:79 while Agent reason tooltip
intercepts the real page Close pointer click after keyboard traversal.
The local full source/dist/sweep passed; do not substitute that pass for this failure.

Seal and launch a fresh TASK Astra/low to reproduce deterministically, explain
focus/hover/pointer-event cause, and repair the actual interaction. A passive
reason tooltip must not trap a user away from Close or other adjacent controls.
Keep useful accessible reason text, keyboard reachability, visibility and dismissal;
consider hover/focus persistence rather than blindly hiding the tooltip.
No force-click, test-only cursor workaround, assertion weakening or new skip.
Add one meaningful regression that recreates the failing focus/pointer state
and fails before the fix, passes after. Preserve all existing assertions.

Write scope: styles.css, shell reason-tooltip component(s) only if needed, the
existing b3-accessibility.spec.ts and narrowly necessary maintained tests. Notify
ROOT of any additional path before writing it. No controller/model/schema/core,
benchmark instrument, native Rust, package or workflow change. New evidence only
RUN/instances/B3-CODEX/ci-tooltip-repair/. Historical seals remain immutable.
You own lane integration/checkpoints; children do not commit. No push/PR/merge.

Run focused source checks for complete B3 accessibility and relevant shell
interaction/layout/page-close journeys in both profiles; affected dist coverage,
TypeScript/build/unit as affected. Report exact named selection to ROOT for CI.
ROOT will run the required clean sweep; do not repeat full source/dist suites
outside it unless a new concern requires them. Obtain an actual Tauri witness
of the affected focus/tooltip/Close pointer case on the repaired binary, with
source/binary identity and isolated/disposable project; no unrelated native sweep.
Reserve browser/native slots with ROOT; serialise via existing run lock, one worker.
No resource measurements or performance qualification.

Return bounded diagnosis, failing/passing regression evidence, exact diff/head,
semantic and test changes, native witness/limits, useful raw logs, and released
resources. ROOT owns independent backcheck, CI scope amendment, clean sweep,
PR updates and merge. After this repair, pause again; launch no next UI slice.
