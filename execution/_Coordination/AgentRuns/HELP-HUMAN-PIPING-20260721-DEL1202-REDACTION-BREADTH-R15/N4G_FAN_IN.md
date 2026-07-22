# WORKING_ITEMS Fan-in — N4G immutable remediation attempt 7

**Verdict:** `ACCEPT_FOR_FRESH_N5G_REVIEW`

- N4G terminal `SUCCESS`; all nine missing/null/non-Mapping input variants now
  emit their declared blockers without exception, block, withhold payload, and
  skip materialization with sanitized evidence.
- Lossless-only withholding now records one actual lossless/exposure blocker,
  consistent with `blocked=true`.
- Focused 17, piping 533, desktop 492/build, H4 2+1, Rust 44, harness 311 plus
  self-check, validators, and containment pass.
- Exactly one attempt-7 sweep passed: `SWEEP_20260722T094654Z_0c066652cd52-dirty.json`,
  SHA-256 `6b6a99dfe79186a3b6d25b6fa192eba24485584bd82694db40782b6c89641ade`.
  Attempts 1–6 remain byte-identical.
- Final inventory 257 paths, zero violations/staged/test-results; no state,
  receipt, lifecycle/release, branch/HEAD, or Git effect.

Fresh N5G must adversarially verify all N5F closures and the complete prior
implementation, tests/amendments, evidence selection, seven sweeps, and scope.
No edits or sweep. W3 remains held.

