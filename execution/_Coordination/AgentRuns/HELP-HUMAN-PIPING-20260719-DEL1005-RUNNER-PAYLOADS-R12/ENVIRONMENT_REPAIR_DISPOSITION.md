# Environment-Repair Disposition — R12 evidence-sweep prerequisite block

- **Disposition ID:** R12-ENVREPAIR-01
- **Dispositioned:** 2026-07-19 (UTC), by HELP_HUMAN (Agent 0, R12 parent) at
  N3 fan-in
- **Class:** disposition-class exercise under DEC-082 / Shared-Block v1 with
  DEC-087 error-asymmetry review; recorded here as the rationale artifact and
  cited from the loop receipt

## Condition

Node N3 executed CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001 to a truthful
`BLOCKED`: all implementation work and 16/17 checks passed, but the mandatory
DEC-025 `evidence-sweep` preflight failed in this fresh git worktree because
ignored, rebuildable local build state is absent here: `node_modules/**`
(playwright/tsc/vite/vitest) and per-crate gitignored `Cargo.lock` files. The
sealed brief correctly forbade the executor from provisioning.

## Fast-reject screen (limits first)

No D-49/D-52 limit class is touched by the selected repair:

- **No network / no acquisition (classes 6, 10).** The repair copies already
  present local state from the primary checkout
  (`~/ai-env/projects/chirality/projects/chirality-piping`; path form made
  home-relative by N3 for `validate_path_anchors.py` conformance, meaning
  unchanged) into
  this worktree. No registry, download, fetch, install, or external service.
  The network acquisition of this exact state was performed and durably
  recorded earlier under explicit owner direction (run
  `HELP-HUMAN-PIPING-20260718-DEL0904-PREREQ-PROVISION-P1`, landed at commit
  `89a93d7ca`).
- **Identity of state.** `package-lock.json` is byte-identical between the
  two checkouts (SHA-256
  `0dd1616e1ef3c596d5cdaa8e56994f26127c20943bfde066f26c9f924c65d732`, equal to
  the P1 frozen baseline), so `node_modules` materialized from it is the same
  provisioned tree the owner already recorded. The copied `Cargo.lock` files
  are the P1-created ignored lock state for the same committed manifests.
- **No project-content write (classes 4, 5, 7).** Every copied path is
  gitignored ephemeral build state; `git status` is unchanged by the repair.
  No durable project file, no lifecycle/stage/acceptance effect, no history
  mutation; fully reversible by deleting the copied ignored paths.
- **No new criteria or authority (class 4).** The repair changes nothing
  about what the sweep checks; it only lets the already-registered gate run.

Judgment: not borderline — the sole reason P1 required an owner override was
network acquisition, which this repair does not perform. Over-referral of a
no-network, ignored-state, precedent-identical materialization would spend
owner attention with no authority at stake.

## Action

1. Copy per-crate `Cargo.lock` files that exist in the primary checkout and
   are absent here (gitignored paths only).
2. Copy `node_modules/**` from the primary checkout (APFS clone copy).
3. Re-run the registered `evidence-sweep` check exactly as N3 did; require
   exactly one new `validation/evidence/sweeps/SWEEP_*.json` delta.
4. Return the result to the N3 executor for its withheld §4.4 closeout, then
   proceed to N4 independent verification.

## Boundaries

This disposition performs no owner act: no adoption, ruling, acceptance,
lifecycle, stage, release, promotion, merge, push, publication, or external
effect. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
