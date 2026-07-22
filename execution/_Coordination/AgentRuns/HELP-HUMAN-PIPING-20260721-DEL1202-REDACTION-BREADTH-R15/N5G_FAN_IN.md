# WORKING_ITEMS Fan-in — N5G fresh attempt-7 verification

**Verdict:** `BLOCK`

N5G reproduced one material regression: N4G treats an empty
`units_manifest={}` as present, unlike the frozen falsy-value behavior. With a
matching null source reference, the public workflow emits no
`EXP-HANDOFF-MANIFEST-FIELD-MISSING`, returns unblocked with payload, and
materializes. The 17 tests omit the empty-Mapping case.

Other mandatory probes passed: 18 invalid variants, all 15 declared
diagnostics, additive warning/blocker behavior, lossless 0/0/1/1 accounting,
31/31 route coverage, seven hashes, 261-path containment, and state/Git holds.

No repair, sweep, closeout, receipt, lifecycle/release, or Git effect. W3 held.

