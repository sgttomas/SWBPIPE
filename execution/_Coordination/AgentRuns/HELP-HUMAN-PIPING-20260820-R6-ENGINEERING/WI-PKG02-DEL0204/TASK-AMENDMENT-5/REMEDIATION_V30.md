# Amendment 5 — V30 remediation

Close exactly two findings:

1. Canonicalize and bound every preflight invalid path before any finding or
   diagnostic reference, replacing over-limit/noncanonical segments with a
   deterministic bounded identifier across adapter, manifest, catalog, and
   evidence surfaces.
2. Normalize caller-supplied plugin schema through fully exception-contained,
   bounded exact-JSON snapshotting before fingerprint/evaluation; hostile or
   over-depth inputs return `PLUGIN_MANIFEST_SCHEMA_MALFORMED` and cannot mask
   an already-observed quarantine.

Add direct/composed adversarial-key and hostile/deep-schema regressions. Require
canonical envelopes and runtime non-dispatch. Run full N1 suite, schema,
containment, diff, then fresh review. No other scope changes.
