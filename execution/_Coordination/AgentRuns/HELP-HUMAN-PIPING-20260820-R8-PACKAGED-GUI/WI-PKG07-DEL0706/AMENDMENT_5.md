# AMENDMENT 5 — synchronous model transition and cancellation remediation

Version: 5

Authority: HELP_HUMAN standing direction to amend in-scope N1 discoveries;
fresh read-only software-code-review 3.

Reason: `MODEL_TRANSITION_AND_PRESTART_CANCEL_NOT_GENERATION_BOUND`.

Reviewer 3 verified 31/31 hashes, containment, exact executable profile, all
affected-check evidence, six screenshots, and 100% of the integrated snapshot,
but found a pre-layout-effect stale-publication window, silently lost immediate
cancellation, and unsupported callback-coverage wording.

Authorized bounded correction:

- synchronously invalidate active solve generation and clear running/proof
  immediately before every accepted model replacement; keep the layout effect
  as a defensive backstop;
- make pre-start cancellation a generation-bound intent, surface it honestly,
  and dispatch it exactly once when a backend receipt supplies job ID/token;
- bind immediate DOM/native cancellation, terminal result, receipt/failure,
  model/open transition, proof publication, and finally behavior to real
  deferred-callback tests rather than observational gate-only assertions;
- narrow evidence to exactly the callbacks those tests execute;
- rerun all affected checks, rebuild/package, repeat the bounded packaged
  reopen/solve proof, update evidence, refreeze, and use another different
  fresh read-only non-delegating reviewer.

All prior FAIL history is preserved. Allowed writes, exclusions, claim fences,
host proof surface, runtime attribution, and CHANGE ownership remain unchanged.
