# ORCHESTRATION PLAN — DEL-09-04 Clean Reproduction R11

- RunID: `HELP-HUMAN-PIPING-20260719-DEL0904-CLEAN-REPRO-R11`
- Parent: `HELP_HUMAN`
- Manager: `WORKING_ITEMS`
- Package / deliverable: `PKG-09 / DEL-09-04`
- Posture: `TERMINAL_FAN_OUT_IN`
- Selection authority: explicit HELP_HUMAN launch under the effective adopted candidate brief
- Plan version: `1`
- Frozen before dispatch: `2026-07-19T20:20:23Z`
- SOURCE_COMMIT: `23eeaabc904064e2297690e391df153dea116ff0`
- Reproduction RUN_ID: `REPRO_DEL0904_20260719T202023Z_23eeaabc9040`

## Accepted Basis

- The source worktree was clean at plan freeze and cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor.
- Receipt 57 is the latest valid cursor; R10 is committed prerequisite infrastructure and is not rerun or edited.
- The registered local/offline prerequisite preflight returned zero errors immediately before this plan was written.
- Candidate brief SHA-256: `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`.
- Procedure SHA-256: `5fee14dd6ed62e4b75ef833af2ad9de2e711a83889be94d10b9a0a7de230418d`.
- Workflow profile SHA-256: `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`.
- Root workflow contract SHA-256: `f97af1d323524f9a2be1dab8b5b33c1350c8f48c38cd3c6f8d0f8a9cd9821ea2`.

## Work Graph and Convergence

Exactly one serialized, Bash-bearing, non-delegating Agent 2 is the integration owner. It executes the adopted brief in a clean detached local clone, runs each expensive check at most once, and retains completed output if its agent session ends while a process finishes. No verifier child exists.

The experiment is the frozen source commit and detached clone. Its verdict and bundle checksums are sealed before post-result coordination records. Later R11 terminal records are checked only for schema and containment and do not invalidate or rerun the frozen experiment.

## Failure Posture

The child performs no repair. The first genuine failure/blocker produces truthful terminal evidence and stops later state-changing closeout. R3 and R8 runs/bundles remain immutable terminal FAIL history. R10 remains immutable committed prerequisite history. No stage, commit, push, merge, lifecycle transition, acceptance, promotion, release, publication, prover claim, or external action is authorized.
