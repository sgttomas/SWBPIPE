---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
node: N2-I1
amendment: 1
status: SUCCESS
finding_state: FULLY_REMEDIATED_PENDING_FRESH_REVIEW
---

# N2-I1 amendment return 1 — shared-fixture root drift

## Remediation

- TypeScript now constructs the complete projection root, asserts the supported `schema_version` is exactly `1.0.0`, and compares that complete root to the complete imported fixture. Missing, changed, or unexpected root fields therefore fail the equality proof.
- Rust now deserializes the shared fixture into a typed root plus typed present/missing case wrappers, all with `deny_unknown_fields`, before injecting embedded production records into the report witness.
- Rust checks the exact supported schema version and has an explicit fail-closed test for both version `2.0.0` and an unexpected root field.
- Embedded value, provenance-note, and diagnostic records remain `serde_json::Value` only until they are consumed by the actual typed report DTO through `assemble_wire_request`, preserving the original cross-layer proof.
- The shared fixture content and production behavior were not changed.

Finding disposition: **FULLY REMEDIATED**, pending the frozen N2-R1 fresh 100% review required by the parent work graph.

## Amendment changed paths

- `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.test.ts`
- `projects/chirality-piping/core/reporting/report_package/src/wire.rs`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/TASK-IMPLEMENT/AMENDMENT_RETURN_1.md`

The completed `_run_records/TASK_RUN_2026-08-19_2347.md`, fixture, DEL-08-01 `_STATUS.md`, and DEL-08-01 `MEMORY.md` were not modified by this amendment.

## Checks

- `npm test --workspace apps/desktop -- --run src/features/report/reportPackageRequest.test.ts` — PASS: 1 file, 6 tests.
- `cargo test --manifest-path core/reporting/report_package/Cargo.toml` — PASS: 7 unit tests, 12 container integration tests, 0 doc-test failures.
- `cargo fmt --manifest-path core/reporting/report_package/Cargo.toml -- --check` — PASS.
- `python3 tools/software_workflow/validate_change_scope.py ... --path <four complete N2-I1 paths>` — PASS: zero violations.
- `git diff --check -- <TypeScript test> <Rust wire>` — PASS.
- `python3 -m json.tool fixtures/reports/invented/component_provenance_cross_layer_projection.json` — PASS; fixture remains valid and unchanged by the amendment.

## Containment and blockers

- All implementation and evidence writes remain within the original sealed write targets.
- No production function, public contract, policy, status/memory surface, Git state, or sibling path was changed.
- Blockers: none.
- Rerun trigger: any change to the shared projection root/schema, either wrapper, the TypeScript producer projection, or the Rust report DTO/assembly path requires both focused suites and fresh review again.
