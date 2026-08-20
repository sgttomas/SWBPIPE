# Amendment 5 — V28 remediation

Close exactly the three V28 findings without widening product scope:

1. Make raw safe-marker observation exception-contained against hostile keys
   and equality while retaining safely observable exact quarantine markers.
2. Bound unit-evidence fallback work deterministically after snapshot failure;
   markers within the declared acceptance budget retain precedence.
3. Keep malformed-manifest fallback within manifest-specific node/text/depth/
   serialization limits and never copy over-limit/noncanonical raw identifiers
   into diagnostics.

Add direct/composed regressions for colliding hostile keys, over-width unit
lists and marker boundaries, over-limit provenance, and over-limit/hostile
plugin identifiers. Require structured reject/quarantine, canonical envelopes,
and `runtime_dispatched=false`. Run complete N1 suite, schema assertions,
five-path containment, and diff check. No manager/status/fan-in, N2/N3, schema,
Git, receipt, PR, or runtime-dispatch changes.
