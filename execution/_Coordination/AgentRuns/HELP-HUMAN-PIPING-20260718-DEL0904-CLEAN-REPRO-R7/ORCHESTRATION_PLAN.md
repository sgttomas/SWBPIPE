# Orchestration Plan — DEL-09-04 Clean Reproduction R7

- Run ID: `HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R7`
- Parent: `HELP_HUMAN`
- Manager: `WORKING_ITEMS`
- Package: `PKG-09`
- Selected deliverable: `DEL-09-04`
- Objective: execute only the adopted clean-checkout reproduction objective in `CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md` as far as its existing authority permits.
- Source commit: `525ef0903e68b536ff5b22f985263ca737a67986`
- Branch: `codex/piping-del0904-clean-repro-20260718-r7`
- Frozen reproduction ID: `REPRO_DEL0904_20260719T023848Z_525ef0903e68`
- Posture: `TERMINAL_FAN_OUT_IN`, one serialized Bash-bearing Agent 2 integration owner.
- Selection authority: HELP_HUMAN launch brief implementing the owner's effective standing-approved candidate brief.

## Accepted basis

- Candidate brief: `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`, SHA-256 `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`.
- Documented procedure: `docs/validation_manual/headless_runner_reproduction.md`, SHA-256 `5fee14dd6ed62e4b75ef833af2ad9de2e711a83889be94d10b9a0a7de230418d`.
- Software workflow profile: `software-workflow.json`, SHA-256 `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`.
- Registered evidence-sweep implementation: `tools/release/run_evidence_sweep.py`, SHA-256 `ac13dd8c6c590a6caf0b633c2ae50e47b410408feccbca4af697b2285f9309c4`.
- Cleanup merge/source baseline: `525ef0903e68b536ff5b22f985263ca737a67986`; it contains cleanup implementation commit `946feb629b16472d45f99ef503c11c07667e97b9`.
- Cleanup R6 handoff reports the clean commit-bound five-surface PASS admitted in Receipt 56.
- R3 handoff and R3 reproduction bundle remain immutable terminal FAIL history and are excluded from reads, reuse, modification, overwrite, or reinterpretation.

## Live prerequisite gate

Before the first durable write, WORKING_ITEMS invoked the registered `preflight_prerequisites` function directly with `PYTHONDONTWRITEBYTECODE=1` and `CARGO_NET_OFFLINE=true`. It returned nonzero with four absent project-local Node binaries and fifteen incomplete locked/offline Cargo manifest probes, including `core/runner/headless/Cargo.toml`. No installation, download, or provisioning occurred.

Candidate brief §§3.2, 3.6, 7 require fail-closed termination. The only child node therefore revalidates and records this prerequisite failure, constructs the truthful immutable `BLOCKED` derivative bundle and required records, and does not create a temporary clone or run the generator, runner, tests, evidence sweep, or any registered profile surface.

## Fan-in gates

WORKING_ITEMS will accept the child return only after confirming:

1. write containment and exactly one new DEL-09-04 `WORKING_ITEMS_RUN_*` record;
2. valid JSON records;
3. verified bundle `SHA256SUMS.txt`, finalized last and never mutated afterward;
4. no change to DEL-09-04 `_STATUS.md` or `MEMORY.md`;
5. no append to `loop/LOOP_RECEIPTS.md`;
6. zero new evidence-sweep artifacts;
7. exact `BLOCKED` semantics, truthful command-not-run inventory, and no fabricated runtime outputs;
8. claims/path/receipt validators as applicable, `git diff --check`, and protected-surface checks.

No lifecycle, acceptance, evidence promotion, release, publication, external action, Git staging, commit, push, PR, merge, installation, download, network access, or repair is authorized.

## Model attribution

- HELP_HUMAN parent: owning run record.
- WORKING_ITEMS manager: Codex agent based on GPT-5; exact runtime model string is not exposed to this agent.
- Agent 2: inherits the same parent runtime capability; exact runtime model string is not exposed.
