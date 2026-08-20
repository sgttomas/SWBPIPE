# Amendment 1 — identity fail-closed remediation and fresh review

Detection layer: fresh integrated-diff Agent 2 review attempt 1.

Failure class: false-pass risk / missing restored identity assertion.

Reason code: `RESTORED_PROJECT_IDENTITY_NOT_GATED`.

Authorized remediation within the existing node: require the restored SQLite
row ID, restored model project ID, and solved `model_ref` all equal
`project:packaged-edited-load-smoke`; assert the solved model ref in the
durable Rust test; rerun focused/full/package checks; freeze v2; dispatch a
different fresh read-only non-delegating reviewer over 100% of v2.

No write-scope, objective, lifecycle, GUI residual, or authority change.
