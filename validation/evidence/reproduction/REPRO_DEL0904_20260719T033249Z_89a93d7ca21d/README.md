# DEL-09-04 Clean-Checkout Reproduction R8

**Overall status:** `FAIL`

This immutable derivative package records the actor-neutral DEL-09-04 clean-checkout reproduction attempted from source commit `89a93d7ca21d64c57cc344955d17deef709fd685`. The detached clone, fixture generator, three documented runner cases, exact output predicates, headless-runner Cargo tests, Python contract test, project pytest, and the five-surface evidence sweep passed under the sealed offline/no-install posture.

The overall result is `FAIL` because the mandatory registered `harness-pytest` check exited `1`: 264 tests passed and two live-baseline tests failed. The observed self-check `REVIEW` count was 38 rather than the pinned 30, and the GEN8 absolute-path baseline had additional source paths, including committed P1 prerequisite-run records. The sealed brief forbids repair in this tranche, so `harness-self-check` and later state-changing closeout steps were not run.

DEL-09-04 remains `IN_PROGRESS`; its actor-neutral clean-checkout Remaining item remains open. No Receipt 57 was appended. The temporary PASS-only `_STATUS.md` and `MEMORY.md` edits were rolled back byte-for-byte to `SOURCE_COMMIT`; the new DEL-09-04 R8 run record truthfully preserves the late failure.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

This bundle does not record reproduction acceptance, lifecycle or target-stage advancement, evidence promotion, release, publication, prover status, professional reliance, or external effect.

## Primary evidence

- `validation_summary.json` — exact runtime/review predicates and output hashes.
- `commands.jsonl` — timestamped cwd/argv/environment/exit records.
- `checks/harness-pytest.json` — terminal mandatory-profile failure.
- `checks/evidence-sweep.json` and `checks/sweeps-delta.txt` — passing single-invocation sweep and exactly-one-file delta.
- `checks/not-run.json` — commands omitted after the mandatory profile failure.
- `manifest.json` — derivative-package provenance, statuses, hashes, and predicates.
- `SHA256SUMS.txt` — final bundle integrity index.
