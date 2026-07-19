# Sealed Agent 2 Launch Brief — DEL-09-04 Clean Reproduction R11

RequestedBy: `WORKING_ITEMS`

RunID: `HELP-HUMAN-PIPING-20260719-DEL0904-CLEAN-REPRO-R11`

ParentInstanceID: `WORKING_ITEMS-DEL0904-R11`

ChildInstanceID: `TASK-DEL0904-CLEAN-REPRO-R11-01`

AgentType: `2`

Construction: ephemeral bounded generalist governed by the Agent 2/TASK base contract

PackageID / DeliverableID: `PKG-09 / DEL-09-04`

WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`

SOURCE_COMMIT: `23eeaabc904064e2297690e391df153dea116ff0`

REPRO_RUN_ID: `REPRO_DEL0904_20260719T202023Z_23eeaabc9040`

## Objective

Execute the already-adopted DEL-09-04 clean-reproduction candidate brief once from the frozen source in a clean detached local clone. Produce and seal one fresh immutable derivative bundle. Do not delegate and do not repair any failure.

## Required Reads and Basis

Read root and project `AGENTS.md`, `agents/AGENT_TASK.md`, the committed active workplan selected by `loop/LOOP_INIT.md`, the complete adopted candidate brief, Receipt 57, DEL-09-04 status/memory/context/references/dependencies/ScopeOfWork, DAG-007 authority, procedure and workflow profile, R8 terminal handoff, and R10 terminal handoff. Live committed source governs.

The manager confirmed a clean source worktree, cleanup-merge ancestry, and zero-error registered local/offline prerequisite preflight before dispatch. Revalidate without provisioning.

## Exact Execution Contract

1. Use one task-created `mktemp -d` root. Clone only the local `{REPO_ROOT}`, detach at SOURCE_COMMIT, and prove initial HEAD/status/tracked/cached cleanliness. Put Cargo target and scratch outside the clone.
2. Use no network, install, download, fetch, update, browser provisioning, remote, cloud, daemon, telemetry, or external action. Every Cargo command uses `CARGO_NET_OFFLINE=true` and `--offline`.
3. Execute the adopted candidate commands and predicates exactly. Capture argv, cwd, environment, UTC, stdout, stderr, exit codes, outputs, hashes, and fixture determinism.
4. Run the two documented review checks once. Then run the registered checks in order and at most once: `piping-pytest`, exactly one `evidence-sweep`, `harness-pytest`, `harness-self-check`. Do not rerun an expensive command. If a process outlives the agent session, inspect its completed output/artifact rather than launching another.
5. On the first genuine failure/blocker, stop later state-changing closeout and terminalize truthful FAIL/BLOCKED evidence. Do not repair.
6. On runtime/profile PASS only, close only the named clean-reproduction Remaining bullet while preserving lifecycle `IN_PROGRESS`; append status history and memory; create one DEL-09-04 run record; append Receipt 58 with Parent Receipt 57; run the remaining validators and containment gate.
7. Determine the frozen-source reproduction verdict and finalize the bundle before writing terminal coordination records. Compute `SHA256SUMS.txt` only after bundle content is stable and verify it. Post-result R11 coordination records do not cause the experiment or expensive checks to rerun.
8. Remove only the validated task-created temporary root after all needed evidence is copied.

## Durable Write Fence

- `{WORKING_ROOT}/validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/**`.
- Exactly one new `{WORKING_ROOT}/validation/evidence/sweeps/SWEEP_*.json` from the single registered sweep.
- DEL-09-04 `_STATUS.md` and `MEMORY.md` on PASS only.
- Exactly one new DEL-09-04 `_run_records/WORKING_ITEMS_RUN_*.md`.
- This R11 managed-run tree, limited to terminal child records; do not edit parent plan, graph, or brief.
- `{WORKING_ROOT}/loop/LOOP_RECEIPTS.md` for Receipt 58 on PASS only.
- One validated ephemeral temporary root.

## Immutable Exclusions

Never edit, reuse, reinterpret, or copy R3/R8 run or bundle evidence. Do not edit or rerun R10. No code, tool, fixture, test, documentation, governance, sibling-project, or prior-run repair is authorized. No Git stage/commit/push/merge or lifecycle/acceptance/promotion/release/publication/external effect is authorized.

## Terminal Return

Return exact source, bundle and checksum hash/count, sweep path/hash, every command/check exit and predicate summary, protected-history hashes, changed-path inventory and containment, receipt/status effects, blockers/rerun requirement, and `COMMIT_READY_NOT_STAGED` only if every applicable closeout gate passes.
