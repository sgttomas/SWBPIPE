# AMENDMENT 6 — detached pre-start cancellation tombstone

Version: 6

Authority: HELP_HUMAN standing direction to amend in-scope N1 discoveries;
fresh read-only software-code-review 4.

Reason: `PRESTART_CANCEL_DROPPED_BY_MODEL_TRANSITION`.

Reviewer 4 verified 40/40 hashes, containment, exact executable profile, all
affected-check evidence, ten screenshots, and 100% of the integrated snapshot,
but found that a model transition after pre-start cancellation invalidates the
generation before the eventual backend receipt can receive the requested
cancellation.

Authorized bounded correction:

- retain a detached, generation-bound cancellation tombstone after model/open
  invalidation only until a pending start receipt resolves;
- if that receipt creates a backend job, dispatch cooperative cancellation
  exactly once using its job ID and token without publishing stale UI state;
- if no backend job is created or start fails, retire the tombstone without a
  cancellation-success claim;
- add a rendered native-event regression for Run -> immediate Cancel ->
  accepted open/model commit -> delayed start receipt, proving one backend
  cancel and no stale state/result/proof publication;
- rerun the triggered product and harness checks, refreeze the entire
  integrated diff, and use another different fresh read-only non-delegating
  reviewer.

No contract, persistence, solver, or cross-package change is authorized. The
existing sequential packaged GUI predicates are unchanged; repeat host proof
only if the built executable or visible behavior changes. All prior FAIL
history, allowed writes, exclusions, claim fences, runtime attribution, and
CHANGE ownership remain unchanged.
