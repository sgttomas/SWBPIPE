# HELP_HUMAN remediation authorization — N4H immutable attempt 8

**Decision:** `AUTHORIZED`

Treat empty `units_manifest={}` as missing/invalid under frozen-base falsy
semantics, emit `EXP-HANDOFF-MANIFEST-FIELD-MISSING`, compose it into the
additive gate, block/withhold/skip materialization, and preserve sanitized
evidence. Add the exact `{}` plus matching null source-ref regression. Do not
alter non-empty valid mappings or any other behavior/surface. Attempts 1–7
remain immutable. One post-green attempt-8 sweep, no rerun, then fresh N5H.

