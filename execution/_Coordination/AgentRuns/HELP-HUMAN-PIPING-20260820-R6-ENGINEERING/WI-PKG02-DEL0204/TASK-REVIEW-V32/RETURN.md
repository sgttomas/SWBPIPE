# TASK-PKG02-DEL0204-REVIEW-V32 Return

Verdict: `FAIL` — one blocking finding.

Normalized schema-valid `metadata.plugin_id` is copied directly into diagnostic
`source.ref_id`; the canonical schema has a pattern but no maximum length, so a
near-1-MiB canonical ID bypasses the existing 256-byte safe manifest-reference
fence. Evidence: `plugin_verification.py:1666`.

Use the existing safe plugin-reference helper for normalized manifests and add
direct/composed huge and adversarial plugin-ID regressions. Amendment 6 path and
message remediation otherwise passed. All 21 hashes/8,326 lines, scope, and
diff integrity passed. Fresh review required.
