# Amendment 5 — V29 remediation

Close exactly two fallback defects:

1. Never traverse arbitrary-width raw adapter capabilities after snapshot
   failure. Either omit inspection or cap it deterministically while preserving
   safely observable quarantine markers.
2. Route adapter IDs and unit diagnostic paths through bounded canonical
   fallback-reference helpers; over-limit/noncanonical values become `TBD` or
   deterministic bounded index paths.

Add direct/composed oversized-capability plus adapter-ID/unit-path byte-overflow
regressions, including protected/quarantine combinations. Require canonical
structured envelopes and runtime non-dispatch. Run full N1 suite, schema,
five-path containment, and diff check. No other scope changes.
