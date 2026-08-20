# Software code review 4 return — FAIL

RUN_STATUS: `FAILED`

ReviewVerdict: `FAIL / INVALID_FOR_FAN_IN`

- Frozen hashes: PASS, 40/40.
- Containment: PASS for all 40 members plus the self-excluded manifest.
- Exact profile selected `desktop-test`, `desktop-build`, `harness-pytest`,
  and `harness-self-check`; recorded evidence existed for all four.
- Coverage: 100% of the frozen integrated diff, including product source and
  tests, SMOKE, DEL records, manager records, telemetry/graph, all prior
  review histories, control metadata, and all ten screenshots.
- Tool policy: PASS; read-only; no writes or delegation.

## Actionable finding

1. **MEDIUM — pre-start cancellation is lost if a model transition occurs
   before the backend start receipt.** A cancellation requested while
   `activeSolveJob` is absent is stored only on the active generation. An
   accepted apply/undo/redo/blank/open transition invalidates that generation
   and clears the intent. When the backend start receipt later resolves, the
   stale-generation return exits without dispatching backend cancellation.
   UI/result/proof integrity remains protected, but the old backend solve can
   continue despite the explicit user request. Retain a detached,
   generation/job-bound cancellation tombstone across model invalidation;
   dispatch cancellation exactly once when the receipt arrives while keeping
   every stale UI callback inert. Add a rendered regression for
   Run -> Cancel before receipt -> model/open commit -> receipt.

Reviewer-1 stale proof/job/model binding is closed. Reviewer-2 overlapping-run
UI callback corruption is closed. Reviewer-3 synchronous pre-commit
invalidation and real terminal callback coverage are closed. Reviewer-3
pre-start cancellation is only partially closed for the transition sequence
above.

Evidence wording is otherwise consistent. Residual risk: the packaged
screenshots cover sequential reopened-unsolved and solved-proof states rather
than adversarial timing; persisted `425 N` is narratively recorded rather than
visible in the final screenshot pair.

Fan-in validity: `false`. No human ruling or dependency blocker.
