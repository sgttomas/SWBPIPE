# I62 review repair 07c: D21 widened by a third indicator

**Basis:** the ruling paragraph "D21 widened by a third indicator" (NUM `d232f0859f`).

**Starting state:** READER at `0c81e09b09`.

**Run window:** 2026-10-04T01:03:06Z to the 01:05:01Z freeze of SHARED_SNAPSHOT_07C, inside the 30-minute box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo. Writes are within the fence. No stop condition was reached.

## Changed READER files

| File | Before (07b) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | f6ec97fb09… | 59e5b1cc985be4a915e530a57f27d9bfb05672db4afe4ce77d21de7d8712f465 |
| P/tests/test_retained_precision_contract.py | 57257094da… | bd356816669f15c1e715e7d8c824acced5b1ddc95cedafffefcc917ff6a5fa2b |
| P/fixtures/results/retained_precision_cases.json | 729c12574a… | d33667719e777cd6d6881e207bf359b6f897bdb2f24094781943d44b68ea4b38 |

These are unchanged: the schema (`07951edacf`), the schema test, the definition, the semantic fixture and the results yaml.

## Changes

**Python (D5b/D21 check):** on a verification record that failed with an escalating stop, any one of three things is evidence that the verification pass ran, and fails G5 ATTEMPT:
- `verification_lme` > 0;
- a verification shared build (adaptive.rs:4286);
- a non-null `verification` summary (set only by `verify_precision`, adaptive.rs:4333).

**Corpus 07c:** one new mutation, `verification_summary_on_escalating_failed_verification`, on `verification_failure_skip_synthetic`.
- Record 1, the escalating Condition failure, has `verification_lme` 0 and no shared build. It gets the base's record 3 verification summary.
- It expects G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`.
- The 07b Python reader accepted it.
- Everything else is byte-identical (asserted).

**Tests:** the snapshot count test now expects 15 / 254 / 19. A mutant that drops the summary indicator fails the new pin.

## Results

- **Full suite:** `pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py` gave **348 passed, 0 failed** (`pytest_07c.log`).
- **Every entry, installed 07c** (`PYTHON_OUTCOMES_07C.json`): 15 of 15 bases with expected classifications; 254 of 254 mutations at their expected first failure; 19 of 19 must-pass entries.
- **Staged, then installed:** 07c was staged and validated in memory before install, and the installed corpus is byte-identical to the stage.

**Counts:** 15 cases, 254 mutations, 19 must-pass entries.

**For I64:** SHARED_SNAPSHOT_07C.json is here, and READER's corpus hash is `d33667719e`.
