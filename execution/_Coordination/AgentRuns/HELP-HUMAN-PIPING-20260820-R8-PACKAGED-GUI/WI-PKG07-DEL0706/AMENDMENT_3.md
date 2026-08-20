# AMENDMENT 3 — independent-review remediation

Version: 3

Authority: HELP_HUMAN standing direction to amend discoveries that belong to
N1 in place; first fresh read-only software-code-review return.

Reason: `STALE_SOLVE_PROOF_NOT_BOUND_TO_COMPLETED_JOB_AND_MODEL_VERSION`.

The first frozen review verified 16/16 hashes and containment but returned one
blocking product finding. The visible proof row could combine a retained old
result with a new running/cancelled/failed solve job and compare only project
IDs. The review also found the sealed brief's PROFILE_PATH must be the project
JSON profile.

Authorized bounded correction:

- bind visible proof to the completed job ID/backend seam, result run/model,
  current project, exact model SHA-256, input-manifest SHA-256, and row count;
- suppress proof unless every binding matches a `completed` current solve;
- clear proof at rerun/model change and reject late solve completion after a
  model/open-project revision;
- add happy-path plus running/cancelled/failed/job-mismatch/model-version
  adverse tests and update Playwright evidence;
- correct PROFILE_PATH in the next review brief, rerun affected checks, rebuild
  package, repeat the bounded packaged post-reopen solve observation, and run a
  different fresh read-only non-delegating reviewer over a new 100% freeze.

Allowed writes, exclusions, claim fences, runtime attribution, host proof
surface, and CHANGE ownership remain unchanged. Attempt-1 and first-review
history are preserved.
