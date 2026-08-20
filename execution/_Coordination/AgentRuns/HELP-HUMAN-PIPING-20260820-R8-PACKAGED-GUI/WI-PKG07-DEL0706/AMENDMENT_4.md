# AMENDMENT 4 — overlapping-run generation remediation

Version: 4

Authority: HELP_HUMAN standing direction to amend in-scope N1 discoveries;
fresh read-only software-code-review 2.

Reason: `OVERLAPPING_PACKAGED_RUN_CALLBACKS_NOT_GENERATION_BOUND`.

Reviewer 2 verified 23/23 hashes, containment, exact executable profile, and
affected-check coverage, but found the static packaged native Analyze command
could dispatch overlapping same-model runs before React `running` state became
a sufficient barrier. Older completion/cancellation/failure/finally callbacks
could then overwrite a newer run. It also found two evidence-count/coverage
wording mismatches.

Authorized bounded correction:

- add an immediate synchronous run-generation gate used by DOM/native starts;
- bind every post-await transition, terminal state, result/proof publication,
  cancellation receipt/failure, and `finally` to the captured active token and
  job ID;
- invalidate active generation on model/open-project revision and reject all
  superseded callbacks, including same browser job IDs;
- add deterministic delayed overlap/completion/cancellation/failure/model-open/
  same-ID gate tests;
- narrow Playwright evidence to happy-path bound visibility, retain adverse
  suppression claims only for Vitest, and reconcile focused-test counts;
- rerun all affected checks, rebuild/package, repeat bounded packaged reopen/
  solve proof, update evidence, refreeze, and use a third different fresh
  read-only non-delegating reviewer.

All prior FAIL history is preserved. Allowed writes, exclusions, claim fences,
host proof surface, runtime attribution, and CHANGE ownership remain unchanged.
