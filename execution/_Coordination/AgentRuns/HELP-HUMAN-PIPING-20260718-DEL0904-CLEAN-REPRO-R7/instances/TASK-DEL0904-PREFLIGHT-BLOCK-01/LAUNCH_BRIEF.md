# Agent 2 Launch Brief — DEL-09-04 R7 Preflight Block Record

RequestedBy: `WORKING_ITEMS`

RunID: `HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R7`

ParentInstanceID: `WORKING-ITEMS-PKG09-DEL0904-R7`

ChildInstanceID: `TASK-DEL0904-PREFLIGHT-BLOCK-01`

PackageID: `PKG-09`

DeliverableID: `DEL-09-04`

ControlSurface: `INLINE`

TaskProfile: `NONE`

TaskSkill: `NONE`

WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`

ScopePath: `{REPO_ROOT}/projects/chirality-piping`

## Objective

Independently revalidate the registered evidence-sweep prerequisite preflight and construct the complete truthful immutable `BLOCKED` R7 derivative bundle and bounded execution records. Do not execute any reproduction/runtime/profile surface.

## Accepted basis

- Source commit `525ef0903e68b536ff5b22f985263ca737a67986` on branch `codex/piping-del0904-clean-repro-20260718-r7`, clean before this managed record began.
- The source commit is the cleanup merge and contains `946feb629b16472d45f99ef503c11c07667e97b9`.
- Candidate brief `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`, SHA-256 `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`.
- Procedure `docs/validation_manual/headless_runner_reproduction.md`, SHA-256 `5fee14dd6ed62e4b75ef833af2ad9de2e711a83889be94d10b9a0a7de230418d`.
- Profile `software-workflow.json`, SHA-256 `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`.
- Preflight implementation `tools/release/run_evidence_sweep.py`, SHA-256 `ac13dd8c6c590a6caf0b633c2ae50e47b410408feccbca4af697b2285f9309c4`.
- Frozen reproduction ID `REPRO_DEL0904_20260719T023848Z_525ef0903e68`; its target path was verified absent before the first durable write.
- R3 and its bundle are immutable terminal FAIL history. Read only the R3 `HANDOFF_STATE.md`; do not inspect, reuse, modify, overwrite, amend, or reinterpret any R3 bundle content.

## Tasks

1. Read root `AGENTS.md`, `agents/AGENT_TASK.md`, project `AGENTS.md`, the candidate brief, Cleanup R6 `HANDOFF_STATE.md`, and R3 `HANDOFF_STATE.md`. Do not delegate.
2. Verify the frozen bundle path remains absent before creating it.
3. Revalidate prerequisites by directly invoking the registered `preflight_prerequisites` function or an exactly equivalent local/offline probe. Use `PYTHONDONTWRITEBYTECODE=1` and `CARGO_NET_OFFLINE=true`. Install, download, provision, activate, update, fetch, or contact nothing.
4. Record complete tool paths and versions for `git`, `python3`, `rustc`, `cargo`, `node`, `npm`, `rustup`, `wasm-bindgen`, `uname`, and `sw_vers` when present. Absence is data, not permission to provision.
5. Because the expected prerequisite result is nonempty errors, terminate `BLOCKED` before `mktemp`, clone, fixture generator, runner, Cargo runner/test, contract test, registered checks, evidence sweep, or other reproduction/profile execution.
6. Create `validation/evidence/reproduction/REPRO_DEL0904_20260719T023848Z_525ef0903e68/` with a truthful pre-execution BLOCKED adaptation of the candidate brief's layout. Include at least `README.md`, `manifest.json`, `environment.txt`, `source_commit.txt`, `procedure.sha256`, `git-status-before.txt`, `git-status-after.txt`, `commands.jsonl`, `validation_summary.json`, `checks/prerequisite-preflight.json`, `checks/commands-not-run.json`, and `SHA256SUMS.txt`. Do not fabricate stdout, stderr, exit-code, or runner-output files for commands not run.
7. The bundle must record source commit, branch, pre-managed-run clean source state, current managed-write state, environment paths/versions, brief/procedure/profile/preflight hashes, exact preflight argv/method and complete observed errors, all commands not run, `overall_status: BLOCKED`, derivative-package status, preserved boundaries, and rerun requirements.
8. Create exactly one new DEL-09-04 record at `_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R7_BLOCKED.md`, with the TASK-compatible structured headings plus explicit BLOCKED package-return facts.
9. Update this instance's `STATUS.json` and create `RETURN.md` with the exact outputs, observed blocker, checks performed, missing/not-run surfaces, rerun requirements, and model/capability attribution.
10. Before final checksum creation, parse all new JSON, perform scoped claims/path/receipt checks as applicable, `git diff --check`, confirm DEL-09-04 `_STATUS.md`, `MEMORY.md`, and `loop/LOOP_RECEIPTS.md` are unchanged, and confirm no new sweep artifact exists. Record those check results inside the bundle before checksum finalization.
11. Generate `SHA256SUMS.txt` last, verify it, and never mutate any bundle file afterward. Return only after checksum verification.

## Allowed tools

`read`, `write`, `bash`; local commands only. Bash makes this child the sole serialized integration owner for this stage.

## Allowed write targets

- `{WORKING_ROOT}/validation/evidence/reproduction/REPRO_DEL0904_20260719T023848Z_525ef0903e68/**`
- `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R7/instances/TASK-DEL0904-PREFLIGHT-BLOCK-01/STATUS.json`
- `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R7/instances/TASK-DEL0904-PREFLIGHT-BLOCK-01/RETURN.md`
- `{WORKING_ROOT}/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R7_BLOCKED.md`

## Exclusions

- No edits to `_STATUS.md`, `MEMORY.md`, `LOOP_RECEIPTS.md`, candidate brief, code, tests, fixtures, docs, tools, profiles, governance, other deliverables, or any prior run/bundle.
- No sweep artifact.
- No temporary clone or reproduction execution.
- No Git staging, commit, reset, stash, checkout, fetch, pull, push, PR, or merge.
- No network or external effect.
- No acceptance, lifecycle, target-stage, evidence-promotion, release, publication, or professional-reliance inference.

## Expected return

Terminal `BLOCKED` with the immutable bundle, one deliverable-local run record, instance status/return, verified checksums, exact evidence pointers, and rerun requirement: provision the missing locked prerequisites outside the sealed run, then start a new clean run with a new run ID and bundle.
