---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
node: N2-I1
amendment: 1
status: frozen
reason: N2-R1 actionable review finding
---

# Amendment 1 — close shared-fixture root drift

The original brief, writes, exclusions, objective, and acceptance remain in force. This amendment narrows one required remediation:

1. Make the TypeScript test assert the fixed fixture schema version and compare the complete constructed root projection with the complete imported fixture so removed, changed, or unexpected root fields fail.
2. In Rust, deserialize the fixture into a typed wrapper that checks the exact supported schema version and denies unknown root fields. Prefer typed, `deny_unknown_fields` present/missing case wrappers as well; the embedded production records may remain `serde_json::Value` because they are subsequently deserialized through the actual report DTO.
3. Add/adjust a focused assertion proving an unsupported schema version or unexpected root field fails closed if needed to make the contract unambiguous.
4. Rerun the focused Vitest, report-package Cargo tests, Cargo fmt check, containment, and diff check. Do not change production behavior, the fixture content, scope, status/memory, or commit.

Return exact changed paths/checks and whether the finding is fully remediated. Update the existing TASK run only through a new immutable amendment return under your managed child folder; do not modify the completed `_run_records/TASK_RUN_2026-08-19_2347.md`.
