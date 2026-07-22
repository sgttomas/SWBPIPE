# HELP_HUMAN remediation authorization — N4G immutable attempt 7

**Decision:** `AUTHORIZED`  
**Recorded:** 2026-07-22T09:32:06Z  
**Basis:** owner-adopted candidate v6 and N5F terminal `BLOCK`.

N4G continues the same adopted N4 sole serialized implementation-owner lane.
Authorized scope is exactly:

1. Guard/validate `target_mapping_contract`, `target_fixture`, and manifest
   `units_manifest` before dereference. Missing, null, or non-Mapping values
   must emit their declared blocking diagnostics rather than exceptions.
2. Compose those diagnostics into the additive exposure gate: blocked, no
   payload/materialization/side effect, with sanitized evidence.
3. Represent/count the actual lossless exposure blocker so
   `exposure_blocking_count` is consistent with `blocked=true`, without
   weakening lossless semantics or inventing unrelated findings.
4. Add exact missing/null/non-Mapping and lossless-count regressions.

No H4, other product/test, public-authority, state, receipt, final deliverable
record, lifecycle, release, or Git changes. Attempts 1–6 remain byte-identical.
Full pre-sweep gates and containment precede exactly one attempt-7 sweep, no
rerun, then fresh read-only N5G. W3 remains held.

