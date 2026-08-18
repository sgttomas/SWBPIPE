# Sealed Brief — TEST-EXECUTION-01

- RequestedBy: `WORKING-ITEMS-PKG09-DEL0904-GUI-01`
- RunID: `HELP-HUMAN-PIPING-20260817-DEL0904-GUI-VALIDATION`
- ChildInstanceID: `TEST-EXECUTION-01`
- PackageID / DeliverableID: `PKG-09` / `DEL-09-04`
- Objective: independently execute the frozen focused Playwright GUI workflow candidate in both registered viewport projects and report exact outcomes.
- AcceptedBasis: work graph v2; human 2026-08-17 steer; frozen working-tree candidate at dispatch.
- Dependencies: manager WASM build passed; dependencies and a pinned Playwright browser are provisioned locally.
- DeclaredReads: repository instructions; `apps/desktop/e2e/gui-workflow-validation.spec.ts`; Playwright config; runtime product surfaces needed to diagnose execution only.
- AllowedTools: read-only shell commands and test runners.
- AllowedWriteTargets: none. Transient ignored test/build outputs are allowed; no tracked file may change.
- ExpectedOutputs: commands, actual test-body execution status for both viewports, pass/fail counts, exact failure classification, and `git status --short` confirmation.
- AcceptanceCriteria: actual assertions run in `chromium-desktop` and `chromium-compact`; 2/2 pass; no tracked write; no network/cloud/telemetry/private-data action by the case.
- Escalation: report launch/substrate failure or any assertion failure without repair.
- EXCLUSIONS: editing, delegation, commits, PRs, source changes, evidence/status changes, lifecycle/release/professional acts.
