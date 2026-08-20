---
run_id: HELP-HUMAN-PIPING-20260820-R6-ENGINEERING
parent_instance_id: WI-PKG08-DEL0801
child_instance_id: TASK-IMPLEMENT
node: N2-I1
amendment: 2
attempt: 3
status: SUCCESS
finding_state: FULLY_REMEDIATED_PENDING_FRESH_REVIEW
accepted_basis: 357a58b56726feba49507534159c3fbc4656b818
---

# N2-I1 amendment return 2 — exact current-session evidence basis

## Remediation

- The TypeScript proof now clones the live preview model and removes `component:C-140` provenance before mechanics, input-manifest, and analysis-run construction.
- `runPreviewMechanics`, `buildCurrentSessionInputManifest`, and `buildAnalysisRunPreview` each consume dependencies derived from that same mutated model; no result, manifest, comparison, or analysis-run object from the original model is reused.
- Before report-package construction or shared-projection comparison, the test asserts the live input manifest's `model_basis.model_ref` matches the report model, its complete `model_basis.model_payload` equals that model, and its `component:C-140` payload carries `provenance: ""`.
- The report producer therefore receives the same model state bound by the exact current-session manifest/hash evidence.
- Amendment 1 complete-root/version binding, Rust typed/version/unknown-field gates, shared fixture, present/missing/malformed behavior, private/pending classifications, and canonical HTML package-output proof remain intact.
- No production defect emerged. Production behavior and the shared fixture were unchanged.

Finding disposition: **FULLY REMEDIATED**, pending a fresh governed read-only review over the complete N2 diff from the original accepted basis.

## Amendment 2 changed paths

- `projects/chirality-piping/apps/desktop/src/features/report/reportPackageRequest.test.ts`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R6-ENGINEERING/WI-PKG08-DEL0801/TASK-IMPLEMENT/AMENDMENT_RETURN_2.md`

No earlier TASK run/return, DEL-08-01 status/memory, manager/handoff/FAN_IN, N1/N3, production, fixture, or Git surface was modified by this amendment.

## Final N2 product/test freeze

Basis: Git `357a58b56726feba49507534159c3fbc4656b818`.

- `apps/desktop/src/features/report/reportPackageRequest.test.ts` — SHA-256 `afb037581feb58e251c3a8b821b37d93ceca7f8f776b655407c4b67e253c7358` (`63` additions, `2` deletions from basis).
- `core/reporting/report_package/src/wire.rs` — SHA-256 `4a04fbde7b50515b00d55dde7438497fa7c5fe1d433bb7fc7d9ff5ee771489e1` (`136` additions from basis).
- `fixtures/reports/invented/component_provenance_cross_layer_projection.json` — SHA-256 `3b20700718c3e5ee87ddf242637f4bb997f38617115bb7ccc2cba89cbf51d110` (new from basis).

## Checks

- `npm test --workspace apps/desktop -- --run src/features/report/reportPackageRequest.test.ts` — PASS: 1 file, 6 tests.
- `cargo test --manifest-path core/reporting/report_package/Cargo.toml` — PASS: 7 unit tests, 12 container integration tests, and doc tests with 0 failures.
- `cargo fmt --manifest-path core/reporting/report_package/Cargo.toml -- --check` — PASS.
- `python3 tools/software_workflow/validate_change_scope.py ... --path <three complete N2 product/test paths>` — PASS: zero violations.
- `git diff --check 357a58b... -- <TypeScript test> <Rust wire>` — PASS.
- `python3 -m json.tool fixtures/reports/invented/component_provenance_cross_layer_projection.json` — PASS.

## Containment and blockers

- All implementation and evidence writes remain within the original sealed write targets.
- Blockers: none.
- Rerun trigger: any change to the current-session model mutation/setup, manifest model payload/binding, shared projection, TypeScript producer, Rust projection wrappers, report DTO, or assembly path requires both focused suites and fresh full-N2 review again.
