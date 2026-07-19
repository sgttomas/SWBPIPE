# Handoff State — DEL-09-04 Clean Reproduction R8

**Closure verdict:** `FAIL`

## Accepted Upstream Snapshot

- SOURCE_COMMIT: `89a93d7ca21d64c57cc344955d17deef709fd685`
- cleanup merge ancestor: `525ef0903e68b536ff5b22f985263ca737a67986`
- adopted candidate SHA-256: `72521c0ae90fc04d5d2e22ff3e3d0be5e96561fe3e2d3847b546c4fa26af1951`
- active DAG: approved `DAG-007`, all eight DEL-09-04 execution-upstream rows `SATISFIED`
- Receipt 56 prerequisite context and separately completed P1 provisioning audit; neither substitutes for the R8 result

## Derivative Package

`validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/`

The fresh bundle is truthful immutable terminal `FAIL` evidence. Its 82-entry `SHA256SUMS.txt` verifies and has SHA-256 `003db610d9bfee47ea73349c81dae79bc196af8ee56aaeb0fb1b85a54bcf7c77`. It is a derivative package, not authoritative deliverable truth or reproduction acceptance. Never mutate or reuse it.

## Failure and Successful Predicates

The detached reproduction passed the generator, fixture-byte determinism, all three documented runner exit/output predicates, the offline headless Cargo test, the Python contract test, `piping-pytest`, and one five-surface evidence sweep. The registered `harness-pytest` then failed: 264 passed, two live-baseline tests failed because the live self-check reported `REVIEW: 38` versus pinned `30` and the GEN8 baseline omitted additional committed source paths including P1 records.

`harness-self-check` and Receipt 57 were not run/written after the mandatory failure. No repair occurred.

## Deliverable and Closure State

- DEL-09-04 remains `IN_PROGRESS`.
- The clean-checkout reproduction Remaining item remains open.
- `_STATUS.md`, `MEMORY.md`, and `loop/LOOP_RECEIPTS.md` match SOURCE_COMMIT.
- Receipt 56 remains the applicable cursor; Receipt 57 is absent.
- Exactly one new sweep exists: `validation/evidence/sweeps/SWEEP_20260719T034442Z_89a93d7ca21d-dirty.json`; its five surfaces pass, while its dirty-tree field truthfully records the authorized in-progress R8 delta.
- R3/R7/P1 protected history and the R3/R7 bundles retained their pre-dispatch aggregate hashes.
- No ignored build artifact is treated as source or reproduction evidence.

No reproduction acceptance, lifecycle/stage advancement, evidence promotion, release, publication, prover status, professional reliance, or external effect is inferred.

## Rerun Requirement

Resolve the committed practitioner-harness live-baseline condition under separate authority. Then create a new managed run ID and a fresh immutable reproduction bundle. Never overwrite or reuse R8.

## Next Owner

`HELP_HUMAN` for disposition. Any Git closeout remains CHANGE work; WORKING_ITEMS performed no stage, commit, push, PR, merge, or external action.

## Git Closeout Authorization — D-55 / DEC-088

D-55/`DEC-088` now authorizes CHANGE to commit the exact 93 staged R8
terminal-`FAIL` files plus the bounded ruling/codification, R9 governance
record, and minimum R8 manager annotations. The independently verified gate is
`PASS_WITH_OWNER_EXCEPTION_DEC_088`; `git diff --cached --check` remains
truthfully exit `2` solely for the bound immutable raw-output finding. Any
second finding or size/hash/checksum/count/scope/protected-state drift returns
`HOLD`.

This authorization is Git closeout only. Closure verdict remains `FAIL`;
DEL-09-04 remains `IN_PROGRESS` with its reproduction Remaining item open;
Receipt 56 remains latest and Receipt 57 absent. No acceptance, promotion,
lifecycle/stage/release, push/merge/publication/prover, or external effect is
authorized.
