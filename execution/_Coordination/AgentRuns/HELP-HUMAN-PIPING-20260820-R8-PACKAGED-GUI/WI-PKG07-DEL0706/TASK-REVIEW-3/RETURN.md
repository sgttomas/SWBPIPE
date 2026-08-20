# Software code review 3 return — FAIL

RUN_STATUS: `FAILED`

ReviewVerdict: `FAIL / INVALID_FOR_FAN_IN`

- Frozen hashes: PASS, 31/31.
- Containment: PASS, 31/31.
- Exact profile selected `desktop-test`, `desktop-build`, `harness-pytest`,
  and `harness-self-check`; recorded evidence existed for all four.
- Coverage: 100% of 31 frozen members, all six screenshots, both prior review
  histories, and manifest structure.
- Tool policy: PASS; read-only; no writes or delegation.

## Actionable findings

1. **HIGH — model/open invalidation is not synchronous with semantic model
   replacement.** Invalidation occurs only in `useLayoutEffect`, after
   apply/undo/redo/blank/open enqueue `setModel`. An older solve can pass its
   token/revision checks and publish stale result, manifest, analysis, hash,
   and proof in the pre-effect window. Invalidate and clear running/proof
   synchronously before each accepted model replacement; retain the effect as
   a defensive backstop; add deferred callback integration tests.
2. **MEDIUM — pre-start cancellation is silently lost.** Cancel is enabled
   after `running=true` but before hashing/backend start produces a current job.
   Preserve a cancellation intent on the active generation, represent it
   honestly, and dispatch it immediately when a backend job receipt arrives.
   Test DOM/native immediate cancel.
3. **MEDIUM — callback coverage is overstated.** The pure gate test never
   drives a real completion/cancellation/failure/model-open/publication/finally
   callback. Add deferred service/rendered-App/native-event coverage and narrow
   claims until it exists.

Reviewer-1 sequential stale-proof closure is substantially complete. Reviewer-2
overlap acquisition/token checks are substantially complete, but synchronous
model replacement, pre-start cancellation, and real callback coverage remain.

Residual risk: packaged sequential screenshots do not exercise adversarial
timing; persisted `425 N` remains narratively recorded rather than visible in
the final screenshot pair.

Fan-in validity: `false`. No human ruling or dependency blocker.
