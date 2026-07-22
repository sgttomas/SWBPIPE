# WORKING_ITEMS Fan-in — N5E fresh attempt-5 verification

**Verdict:** `BLOCK`

N5E independently reproduced two material defects:

1. `REXC-CORE-001` does not compose its own blocking source diagnostics into
   the exposure gate. `EXP-UNIT-METADATA-MISSING` can return
   `blocked=false` with a payload while diagnostic code/severity are merely
   redacted. A1 also removed the blocking/no-payload assertions that should
   detect this bypass, violating its no-weaker-coverage condition.
2. A3 added a `rendered-report-render` click before its assertions, exceeding
   the expectation-only amendment and explicit no-E2E-flow-change boundary.

N5E otherwise confirmed the safe-key removal, exact PCF structural authority,
independent CAEPIPE CSV intent, sanitized decision/finding observability, A2,
31/31 route/disposition/owner coverage, all five sweep hashes/dispositions,
portable terminal evidence, and scope/protected/state/Git no-change.

No repair, sweep, DEL-12-02 state/memory/final record, receipt, lifecycle,
release, or Git effect occurred. W3 remains held. Any remediation requires a
new bounded HELP_HUMAN authorization and fresh implementation/verifier pair.

