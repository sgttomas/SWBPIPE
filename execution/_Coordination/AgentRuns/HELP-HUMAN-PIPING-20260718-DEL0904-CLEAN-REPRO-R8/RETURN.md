# WORKING_ITEMS Package Return — DEL-09-04 Clean Reproduction R8

## Terminal Result

- Manager result: `SUCCESSFUL_FAN_IN`
- Objective result: `FAIL`
- Package: `PKG-09`
- Selected deliverable: `DEL-09-04`
- Source commit: `89a93d7ca21d64c57cc344955d17deef709fd685`
- Reproduction run ID: `REPRO_DEL0904_20260719T033249Z_89a93d7ca21d`
- Branch: `codex/piping-del0904-clean-repro-20260718-r8`
- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986`: confirmed ancestor

The fresh detached reproduction itself passed the generator, three runner commands, every exact output predicate, both documented review checks, project piping pytest, and the single five-surface evidence sweep. The mandatory registered `harness-pytest` profile check then exited `1`; 264 tests passed and two live-baseline tests failed. The candidate therefore requires terminal `FAIL` without repair.

## Accepted Child Return

- Instance: `instances/TASK-DEL0904-CLEAN-REPRO-R8-01/`
- Construction: exactly one serialized ephemeral bounded Agent 2 generalist under the TASK base contract
- Delegation: none
- Instance status: `COMPLETED`; task-shell `RUN_STATUS: SUCCESS`; objective `FAIL`
- Accepted return: `instances/TASK-DEL0904-CLEAN-REPRO-R8-01/RETURN.md`

## Exact Failure

Evidence: `validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/checks/harness-pytest.json`.

- `tools/practitioner_harness/test_live_baseline.py::test_live_self_check_severity_totals_are_recorded_loop_anchors`: observed live self-check `REVIEW: 38`, pinned baseline `REVIEW: 30`.
- `tools/practitioner_harness/test_live_baseline.py::test_live_gen8_abs_path_24_file_baseline`: the live absolute-path baseline contained additional committed source paths, including prerequisite-provisioning P1 records.

The failure is in the mandatory live practitioner-harness baseline, not in the clean detached runner/runtime predicates. No baseline pin, P1 record, harness test, source, fixture, runner, tool, or governance repair was authorized or attempted.

`harness-self-check` was not run after this mandatory failure. Receipt 57 was not appended.

## Reproduction and Profile Evidence

- Immutable derivative bundle: `validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/`
- Exact runtime predicates: `validation_summary.json` (`runtime_review_pass: true`, overall `FAIL`)
- Profile summary: `checks/profile-summary.json`
- Exact profile failure: `checks/harness-pytest.json`
- Single sweep: `validation/evidence/sweeps/SWEEP_20260719T034442Z_89a93d7ca21d-dirty.json`
- Sweep result: exit `0`, exactly one-file delta, all five surfaces `pass`
- Checksum file: `SHA256SUMS.txt`, generated last with 82 entries
- Checksum-file SHA-256: `003db610d9bfee47ea73349c81dae79bc196af8ee56aaeb0fb1b85a54bcf7c77`
- Deliverable run record: `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R8.md`

The sweep filename and its `working_tree_dirty: true` field truthfully bind the authorized in-progress R8 closeout delta. The sweep is not represented as clean-tree evidence. Its five surfaces nevertheless completed `pass` and it supplies no reproduction acceptance or lifecycle effect.

## Independent Manager Fan-In Validation

All validation required to accept the child's terminal FAIL return passed:

- all 82 `SHA256SUMS.txt` entries verified; the bundle contains 83 files including the checksum file;
- 18 R8 managed-run/bundle JSON files parsed; 12 `commands.jsonl` rows parsed;
- exact candidate changed-path containment, including untracked paths, passed with zero violations;
- claims-language validation passed, 262 surfaces scanned;
- path-anchor validation passed, 453 live surfaces scanned;
- loop-receipt validation passed and Receipt 57 is absent;
- `git diff --check` passed;
- HEAD and branch remain the frozen source/branch;
- DEL-09-04 `_STATUS.md`, `MEMORY.md`, and `loop/LOOP_RECEIPTS.md` match their exact SOURCE_COMMIT SHA-256 values: `e1ed7b6d...`, `cee73c9c...`, and `2ad81234...` respectively;
- DEL-09-04 remains `IN_PROGRESS`; the exact clean-reproduction Remaining bullet remains open;
- exactly one new sweep artifact exists and its SHA-256 is `9994c20815dd1d5a64d90cf93f9956f61085b896ed93ec9b07017a1da0d3dd64`;
- exactly one new DEL-09-04 `WORKING_ITEMS_RUN_*` record exists;
- the temporary root `/tmp/del0904-r8.iFx8Aa` is removed;
- no ignored `node_modules`, Cargo target, or Cargo-cache artifact appears in the bundle; and
- the protected R3 run/bundle, R7 run/bundle, and P1 run aggregate hashes exactly match their pre-dispatch values.

Protected aggregate SHA-256 values: R3 run `f530f329...`, R3 bundle `81e9ee96...`, R7 run `306eed9a...`, R7 bundle `df565da8...`, P1 run `bcb1324e...`.

## Changed-Path Inventory

Ninety-three new files, all inside the candidate fence after this manager close:

- 83 immutable bundle files under the fresh R8 reproduction path;
- one profile-emitted sweep JSON;
- one DEL-09-04 terminal R8 run record; and
- eight R8 managed-run files: `ORCHESTRATION_PLAN.md`, `WORK_GRAPH.json`, `STATUS.json`, manager `RETURN.md`, manager `HANDOFF_STATE.md`, and the child instance's `LAUNCH_BRIEF.md`, `STATUS.json`, and `RETURN.md`.

There are no tracked-file modifications. `_STATUS.md`, `MEMORY.md`, and the receipt ledger are unchanged from SOURCE_COMMIT.

## Deliverable Effects and Preserved Boundaries

- DEL-09-04 lifecycle remains `IN_PROGRESS`.
- The clean-checkout reproduction Remaining item remains open.
- Receipt 56 remains the applicable cursor; Receipt 57 is absent.
- R3 remains immutable terminal FAIL; R7 remains immutable terminal BLOCKED; P1 remains provisioning audit only. None was modified, reused, copied as reproduction evidence, overwritten, amended, or reinterpreted.
- The R8 bundle is derivative terminal FAIL evidence, not authoritative deliverable truth or reproduction acceptance.
- No reproduction acceptance, lifecycle/stage advancement, evidence promotion, release, publication, prover status, professional reliance, or external effect is inferred.
- No install, download, provisioning, network, Git stage/commit/fetch/pull/push/PR/merge/rebase/stash/reset, or external action occurred in the sealed R8 run.

## Rerun Requirement and Next Owner

A future clean reproduction requires the committed practitioner-harness live-baseline condition to be resolved through a separately authorized tranche, followed by a fresh managed run ID and fresh immutable reproduction bundle. Never reuse or mutate R8. Return to `HELP_HUMAN`; any later Git closeout belongs to CHANGE.

## Model and Capability Attribution

- WORKING_ITEMS manager: inherited Codex runtime capability; exact model string is not exposed by the harness; no override or mid-task substitution.
- Agent 2 executor: same inherited Codex runtime capability as one ephemeral bounded generalist under TASK; no delegation, override, or model substitution.
- HELP_HUMAN: see the owning parent run record.

## Git Closeout Disposition — D-55 / DEC-088

The owner adopted a new, one-off R8-specific raw-evidence whitespace
exception, recorded in D-55 and codified as `DEC-088`. An independent R9
verifier recomputed the complete boundary and returned `COMMIT-SAFE`.

CHANGE may commit the exact 93 staged R8 terminal-`FAIL` files plus the bounded
D-55/`DEC-088` governance, R9 record, and these minimum manager closeout
annotations while `git diff --cached --check` remains truthfully exit `2` with
the sole bound `stdout/cargo_test.txt:42` finding. The closeout gate is
`PASS_WITH_OWNER_EXCEPTION_DEC_088`; the command is not represented as PASS.
Any D-55 predicate drift returns `HOLD`.

This Git authorization does not change the objective result: R8 remains
terminal `FAIL`, DEL-09-04 remains `IN_PROGRESS` with the clean-reproduction
Remaining item open, Receipt 56 remains latest, Receipt 57 remains absent, and
all reproduction-acceptance, evidence-promotion, lifecycle/stage/release,
push/merge/publication/prover/external-effect exclusions remain in force.
