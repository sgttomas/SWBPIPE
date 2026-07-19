# Sealed Agent 2 Launch Brief — DEL-09-04 Clean Reproduction R8

RequestedBy: `WORKING_ITEMS`

RunID: `HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R8`

ParentInstanceID: `WORKING_ITEMS-DEL0904-R8`

ChildInstanceID: `TASK-DEL0904-CLEAN-REPRO-R8-01`

AgentType: `2`

Construction: ephemeral bounded generalist governed by the Agent 2/TASK base contract

PackageID: `PKG-09`

DeliverableID: `DEL-09-04`

WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`

ScopePath: `{WORKING_ROOT}`

TaskProfile: `NONE`

TaskSkill: `NONE`

ApplyEdits: `true`

SOURCE_COMMIT: `89a93d7ca21d64c57cc344955d17deef709fd685`

REPRO_RUN_ID: `REPRO_DEL0904_20260719T033249Z_89a93d7ca21d`

Branch: `codex/piping-del0904-clean-repro-20260718-r8`

## Objective

Execute the full accepted DEL-09-04 actor-neutral clean-checkout reproduction candidate brief exactly, from the current sealed source and local prerequisites, and terminalize a fresh immutable derivative reproduction bundle. If and only if every runtime and documented review predicate passes, perform the candidate's narrow PASS-only state/profile/receipt closeout. Do not delegate.

## Required Instruction and Context Reads

Read root `AGENTS.md`, `agents/AGENT_TASK.md`, project `AGENTS.md`, committed active plan bytes selected by `loop/LOOP_INIT.md`, the full accepted candidate brief, Cleanup R6 handoff, R7 RETURN/HANDOFF, P1 RETURN/HANDOFF, DEL-09-04 status/memory/context/references/dependencies/ScopeOfWork and relevant local run records, DAG-007 active rows, procedure, workflow profile, claims taxonomy as needed, and Receipt 56. Use live source truth on disagreement.

## Accepted Basis

- SOURCE_COMMIT and branch above; cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor.
- Candidate SHA-256 `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`.
- Procedure SHA-256 `5fee14dd6ed62e4b75ef833af2ad9de2e711a83889be94d10b9a0a7de230418d`.
- Project workflow profile SHA-256 `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f`.
- Root workflow contract SHA-256 `f97af1d323524f9a2be1dab8b5b33c1350c8f48c38cd3c6f8d0f8a9cd9821ea2`.
- DAG-007 and all eight active DEL-09-04 execution-upstream rows satisfied.
- DEL-09-04 `IN_PROGRESS` and exact clean-reproduction Remaining bullet open.
- Registered prerequisite function independently returned zero errors immediately before plan freeze.

## Immutable Exclusions

- R3 and its bundle are terminal FAIL; R7 and its bundle are terminal BLOCKED; P1 is provisioning audit only. Do not modify, reuse, copy as reproduction evidence, overwrite, amend, or reinterpret them.
- Do not install, download, provision, update, access the network, fetch, pull, push, create a PR, merge, rebase, stash, reset, stage, commit, or perform external action.
- Do not repair code, tests, fixtures, docs, tools, governance, workflows, or sibling projects.
- Do not infer reproduction acceptance, lifecycle/stage advancement, evidence promotion, release, publication, prover status, professional reliance, or external effect.
- Agent 2 must not delegate or spawn any child.

## Allowed Tools

Existing local read/write/shell utilities and already-present repo tools needed by the accepted candidate. Every Cargo execution must set `CARGO_NET_OFFLINE=true` and pass `--offline`. No provisioning command is allowed.

## Allowed Durable Write Targets

1. `{WORKING_ROOT}/validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/**` — exactly one new immutable bundle.
2. `{WORKING_ROOT}/validation/evidence/sweeps/<exactly-one-new-SWEEP_*.json>` — only the single tool-emitted evidence-sweep file on PASS closeout.
3. DEL-09-04 `_STATUS.md` — PASS only, exact bullet/history update.
4. DEL-09-04 `MEMORY.md` — PASS only, append evidence/boundaries.
5. DEL-09-04 `_run_records/<exactly-one-new-WORKING_ITEMS_RUN_*.md>` — one terminal execution record.
6. This R8 managed-run directory, limited to child `STATUS.json`, `RETURN.md`, and any necessary typed local validation record; parent-owned plan/graph/launch files must not be edited.
7. `{WORKING_ROOT}/loop/LOOP_RECEIPTS.md` — PASS only, append exactly Receipt 57.

Ephemeral writes are authorized only inside one task-created `mktemp -d` root after validating its absolute path. The child is the serialized Bash-bearing integration owner for the project-root stage.

## Execution Contract

1. Revalidate the sealed authority, exact identities, clean branch/source, and registered prerequisite zero-error result without provisioning. If drift exists, fail closed before reproduction.
2. Snapshot the sweep directory before any execution that could create a sweep.
3. Allocate exactly one `mktemp -d` local root. Clone only from resolved local REPO_ROOT using `git clone --no-hardlinks --no-checkout`, detach at SOURCE_COMMIT, and prove initial HEAD/status/tracked/cached cleanliness. Keep `CARGO_TARGET_DIR` outside the clone under the temp root.
4. From the detached reproduction project, preflight existing tool paths/versions; run the generator and exact documented solve, validate-input, and run-benchmark argv. Capture cwd, argv, environment overrides, UTC times, stdout, stderr, and actual exit separately. Prove generator byte determinism against all three fixtures.
5. Run `cargo test --offline --manifest-path core/runner/headless/Cargo.toml` with offline env and the Python contract test as separate commands. Assert every exact predicate from candidate §6 and prove final tracked/cached cleanliness.
6. Build the complete fresh derivative bundle with candidate layout, raw outputs, manifest provenance/hashes/timestamps/commands/predicates, and truthful `PASS|FAIL|BLOCKED`. `INTERNALLY_VERIFIED` is permitted only on full PASS. Use the standard F-PIP-2 sentence; never use PROVER_CORRELATED or ENGINEER_ACCEPTED.
7. On runtime/review PASS only, update exactly the target status bullet/history, append MEMORY, create exactly one DEL-09-04 WORKING_ITEMS run record, and run each of the four registered checks separately through root `tools/software_workflow/run_registered_checks.py`. The evidence sweep is invoked exactly once; require exactly one new sweep JSON from the before/after snapshot.
8. Run claims-language, path-anchor, receipt, JSON/JSONL, whitespace, and candidate change-scope containment checks. Append exactly Receipt 57 with Parent Receipt 56, Examined-Through SOURCE_COMMIT, exact R8/bundle/sweep pointers, claim-calibrated gate outcome, and model attribution; then validate the receipt. Preserve `IN_PROGRESS` and the other Remaining bullets.
9. Copy and hash all needed temporary evidence before cleanup. Validate the exact task-created temp root, remove only that root, and record cleanup. Finalize `SHA256SUMS.txt` last, verify it, and never mutate the bundle afterward.
10. Terminalize child `STATUS.json` and `RETURN.md` with exact evidence pointers, changed inventory, rerun/blocker status, and model attribution.

If any runtime, documented review, profile, sweep-delta, validator, containment, JSON, whitespace, cleanup, hash, or checksum predicate fails or blocks: preserve a truthful immutable FAIL/BLOCKED bundle and child/run record, leave DEL-09-04 `_STATUS.md`, `MEMORY.md`, and Receipt 56 unchanged from source, do not repair, and return exact evidence. If PASS-only closeout writes were made before a late failure, fail closed and report the discrepancy; do not rewrite source history or alter protected artifacts.

## Expected Outputs

- Fresh immutable reproduction bundle at the exact RUN_ID path.
- Child terminal `STATUS.json` and structured `RETURN.md`.
- PASS-only exact DEL-09-04 state/MEMORY/run-record closeout, exactly one new sweep, and exactly Receipt 57; otherwise preservation of source state and Receipt 56.
- Complete command/provenance/predicate/profile/validation/checksum evidence and exact changed-path inventory.

## Acceptance and Escalation

Return `SUCCESS` at the task-shell level only when the objective result is truthfully terminalized as `PASS`, `FAIL`, or `BLOCKED` with complete evidence. Any missing evidence, write-scope violation, protected-history drift, or unclear authority is a task failure. Return to WORKING_ITEMS; do not repair or delegate.
