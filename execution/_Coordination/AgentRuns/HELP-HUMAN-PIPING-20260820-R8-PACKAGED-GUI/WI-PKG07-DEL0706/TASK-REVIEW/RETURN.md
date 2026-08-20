# TASK software-code-review return — attempt 1

RUN_STATUS: `FAILED`

ReviewVerdict: `FAIL / INVALID_FOR_FAN_IN`

- Frozen hashes: PASS, 16/16.
- Containment: PASS, 16/16.
- Affected checks: desktop-test, desktop-build, harness-pytest,
  harness-self-check; recorded coverage present for all four.
- Coverage: 100% of the 16 frozen files plus manifest structure.

## Actionable findings

1. **HIGH / blocking — stale result can be combined with a different current
   solve job.** `App.tsx` rendered proof whenever `result` existed and derived
   seam from mutable `solveJob`. A completed result retained during a new
   running/cancelled/failed job could therefore display `identity=match` under
   the wrong job seam. Same-project-ID model changes were not version-bound.
   Required: bind proof to completed job and model version/hash, suppress proof
   across rerun/failure/cancellation, reject late completion after model/open
   change, and add adverse lifecycle tests.
2. **MEDIUM / control — PROFILE_PATH was not executable.** The brief named
   `docs/SOFTWARE_WORKFLOW_PROFILE.md`; the authoritative executable profile is
   `projects/chirality-piping/software-workflow.json`. Correct it in the next
   sealed brief/freeze.

Residual risk noted by reviewer: the screenshots corroborate reopened-unsolved
and final solved-proof states, while the restored `425 N` observation is
recorded narratively rather than visible in either frozen screenshot.

No writes, installs, checks, commits, delegation, lifecycle actions, or release
claims were performed by the reviewer.
