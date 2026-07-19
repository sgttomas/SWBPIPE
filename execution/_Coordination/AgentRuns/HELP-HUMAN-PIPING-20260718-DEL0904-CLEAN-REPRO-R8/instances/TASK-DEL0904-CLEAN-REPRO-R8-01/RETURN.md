# TASK Return — DEL-09-04 Clean Reproduction R8

## Terminal Result

- `RUN_STATUS: SUCCESS`
- Objective result: `FAIL`
- Control surface: `FILE`
- Task profile / skill: `NONE / NONE`
- Scope path: `/Users/ryan/.codex/worktrees/2faf/chirality/projects/chirality-piping`
- Write authorization: `ALLOWED_WRITE_TARGETS`
- Tool policy compliance: `PASS`

The fresh detached reproduction passed every generator, runner, exact output-predicate, and documented review predicate. `piping-pytest` passed. The single registered evidence-sweep invocation exited `0`, emitted exactly one new sweep file, and all five sweep surfaces passed. The mandatory registered `harness-pytest` check then exited `1`, so the overall objective is terminal `FAIL`.

## Exact Failure

- Evidence: `validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/checks/harness-pytest.json`.
- Summary: 264 passed, two failed.
- Failures:
  - `tools/practitioner_harness/test_live_baseline.py::test_live_self_check_severity_totals_are_recorded_loop_anchors` — observed `REVIEW: 38`, pinned `REVIEW: 30`.
  - `tools/practitioner_harness/test_live_baseline.py::test_live_gen8_abs_path_24_file_baseline` — live GEN8 absolute-path findings included additional committed source paths, including P1 prerequisite-run records.
- Repair attempted: none; prohibited by the sealed tranche.
- `harness-self-check`: `NOT_RUN` after the mandatory profile failure.

## Evidence

- Immutable derivative bundle: `validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/`.
- Source commit: `89a93d7ca21d64c57cc344955d17deef709fd685`.
- One sweep: `validation/evidence/sweeps/SWEEP_20260719T034442Z_89a93d7ca21d-dirty.json`.
- Runtime predicates: `validation_summary.json` (`runtime_review_pass: true`, overall `FAIL`).
- Profile summary: `checks/profile-summary.json`.
- Final integrity: `SHA256SUMS.txt` generated last and verified after all other bundle content was final.

## Deliverable and Receipt State

- DEL-09-04 remains `IN_PROGRESS`.
- The actor-neutral clean-checkout Remaining item remains open.
- `_STATUS.md`, `MEMORY.md`, and `loop/LOOP_RECEIPTS.md` match their exact `SOURCE_COMMIT` bytes/hashes.
- Receipt 57 was not appended.
- Exactly one new DEL-09-04 R8 `WORKING_ITEMS_RUN_*` record truthfully records this terminal failure.

## Applied Changes

- Created one unique immutable R8 reproduction bundle.
- Created exactly one profile-emitted sweep artifact.
- Created one DEL-09-04 terminal R8 run record.
- Terminalized this child `STATUS.json` and `RETURN.md`.

## Missing / Human Ruling / Dependency Notes

- Missing: no required terminal FAIL evidence.
- Needs human ruling: none inside this sealed run.
- Dependency notes: a future rerun requires the harness live-baseline condition to be resolved under separate authority and a fresh run ID/bundle; do not reuse R8.

## Preserved Boundaries

R3, R7, P1, and their bundles/records remain immutable excluded history. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No reproduction acceptance, lifecycle/stage advancement, evidence promotion, release, publication, prover status, professional reliance, external action, install, download, provisioning, network, fetch, stage, commit, push, PR, merge, or repair occurred in this sealed run.

## Model and Capability Attribution

One ephemeral bounded Agent 2 generalist executed under the TASK base contract using the inherited Codex runtime capability. No child was created, no delegation occurred, and no model substitution occurred.
