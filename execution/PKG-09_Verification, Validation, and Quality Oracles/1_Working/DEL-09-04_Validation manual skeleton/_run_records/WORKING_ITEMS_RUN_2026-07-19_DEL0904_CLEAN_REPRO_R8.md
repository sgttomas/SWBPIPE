---
run-id: WORKING_ITEMS_RUN_2026-07-19_DEL0904_CLEAN_REPRO_R8
timestamp: 2026-07-19T03:45:00Z
run-status: SUCCESS
objective-status: FAIL
control-surface: FILE
scope-path: /Users/ryan/.codex/worktrees/2faf/chirality/projects/chirality-piping
task-profile: NONE
task-skill: NONE
resolved-skill-path: NONE
resolved-skill-version: UNKNOWN
resolved-task-profile-requirement: NONE
companion-files: []
allowed-tools:
  - read
  - write
  - bash
write-authorization: ALLOWED_WRITE_TARGETS
runtime-overrides:
  PYTHONDONTWRITEBYTECODE: "1"
  CARGO_NET_OFFLINE: "true"
package-id: PKG-09
deliverable-id: DEL-09-04
managed-run-id: HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R8
child-instance-id: TASK-DEL0904-CLEAN-REPRO-R8-01
reproduction-run-id: REPRO_DEL0904_20260719T033249Z_89a93d7ca21d
source-commit: 89a93d7ca21d64c57cc344955d17deef709fd685
---

# DEL-09-04 R8 Clean-Reproduction Record

## Requested Tasks

- Execute the adopted actor-neutral DEL-09-04 clean-checkout reproduction from the frozen source commit and local prerequisites without installation, download, network, or provisioning.
- Preserve R3, R7, and P1 as immutable excluded history and create a fresh R8 bundle.
- Perform the narrow PASS-only deliverable closeout only if every runtime, review, profile, and validation predicate passes.

## Expected Outputs

- One immutable derivative reproduction bundle at `validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/`.
- One profile-emitted evidence sweep, exact DEL-09-04 state/MEMORY/run-record closeout, and Receipt 57 only on full PASS.
- Managed child terminal `STATUS.json` and `RETURN.md`.

## Tools Used

- `bash /usr/bin/git` — local clone, source identity, cleanliness, and containment evidence.
- `bash /Library/Frameworks/Python.framework/Versions/3.13/bin/python3` — fixture generator, contract test, registered checks, JSON predicates, and validators.
- `bash /Users/ryan/.cargo/bin/cargo` — offline runner commands and headless-runner review tests.
- `bash /sbin/sha256sum` — input/output/procedure and bundle checksums.
- `bash local-posix-utilities` — bounded local file, path, version, and inventory operations.

## Tool Policy Compliance

`PASS`; no install, download, provisioning, network, fetch, or external action occurred during the sealed reproduction run. Every Cargo execution set `CARGO_NET_OFFLINE=true` and passed `--offline`.

## Write Authorization

`ALLOWED_WRITE_TARGETS`; only the sealed R8 bundle, one profile-emitted sweep, exact DEL-09-04 closeout surfaces, this run record, the R8 child status/return, and Receipt 57 are writable.

## Outputs Produced

- Fresh immutable terminal `FAIL` derivative bundle: `validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/`.
- Exact runtime/review evidence: all generator, runner, predicate, and two documented review checks passed.
- One profile-emitted sweep: `validation/evidence/sweeps/SWEEP_20260719T034442Z_89a93d7ca21d-dirty.json`; exactly one-file delta and all five surfaces passed.
- Terminal profile failure: `checks/harness-pytest.json`, exit `1`, 264 passed and two live-baseline tests failed.

## Missing

- `harness-self-check` was not run after the mandatory `harness-pytest` failure, as required by the candidate halting rule.
- Receipt 57 is absent because it is PASS-only.

## Needs Human Ruling

none

## Dependency Notes

- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor of the frozen source.
- The exact registered prerequisite revalidation returned zero errors before execution.

## Applied Changes

- Created the fresh R8 derivative reproduction bundle.
- Created exactly one evidence-sweep artifact from the single registered invocation.
- Created this one DEL-09-04 R8 run record.
- PASS-only `_STATUS.md` and `MEMORY.md` edits were made before the late profile failure, then restored byte-for-byte to `SOURCE_COMMIT` under the fail-closed disposition. Their final source hashes match; the clean-reproduction Remaining item remains open.

## Terminal Result

`FAIL`. The detached reproduction itself passed every runtime and documented review predicate, `piping-pytest` passed, and the five-surface evidence sweep passed. The mandatory registered `harness-pytest` check failed because the live self-check `REVIEW` total was 38 rather than the pinned 30 and the GEN8 absolute-path baseline contained additional committed source paths, including P1 prerequisite-run records. No repair was authorized or attempted. `harness-self-check` and Receipt 57 were not run/written.

## Preserved Boundaries

R3, R7, P1, and their bundles/records remain immutable excluded history. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No reproduction acceptance, lifecycle/stage advancement, evidence promotion, release, publication, prover status, professional reliance, or external effect is inferred.

## Model and Capability Attribution

Agent 2 executed as one ephemeral bounded generalist governed by the TASK base contract, using the inherited Codex runtime capability. No delegation or model substitution occurred.
