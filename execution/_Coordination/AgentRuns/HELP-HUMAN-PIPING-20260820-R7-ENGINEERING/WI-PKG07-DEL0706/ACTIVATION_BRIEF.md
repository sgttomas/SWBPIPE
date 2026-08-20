---
run_id: HELP-HUMAN-PIPING-20260820-R7-ENGINEERING
instance_id: WI-PKG07-DEL0706
package_id: PKG-07
selected_deliverable_ids: [DEL-07-06]
graph_node: N1
status: frozen
accepted_basis: 8eea5d06d3f98dd91b56b53a2c7caec2f7ed5919
branch: codex/piping-r7-engineering-20260820
dependency_authority: DAG-009
target_stage: R5
representation: SOW_V1
---

# PKG-07 / DEL-07-06 Activation

Objective: close only the exact DEL-07-06 `_STATUS.md ## Remaining` item
“Run a packaged-Tauri smoke over a saved project with edited load data” by
executing the actual packaged macOS application against invented load data.
If the run exposes an in-scope product defect, diagnose and repair it, add
durable automated evidence where feasible, and repeat the packaged proof.

## Accepted basis and dependencies

- Human steering supplied through HELP_HUMAN is the selection authority.
- Receipt-117 is the handoff cursor; predecessor PR #587 is merged.
- DAG-009 is approved active graph authority. DEL-07-06 is a Wave-1 node; its
  seven active upstream execution prerequisites are recorded `SATISFIED`.
- `_STATUS.md ## Remaining` is the executable residual surface. The two
  PDU-045/PDU-046 independent-usability/measurable-target holds are separate,
  human-owned, and excluded from this node.
- The H4 ruled evidence posture in `execution/_Coordination/_COORDINATION.md`
  makes Playwright the default for browser-visible behavior and records manual
  or native GUI execution as the exception when the packaged macOS webview is
  not expressible through Playwright. Prior TP-MAC-140 records that
  `tauri-driver` does not support macOS.

## Frozen product journey

Use only invented repository data. In the actual packaged Tauri application:

1. launch the built `.app` and confirm the Tauri backend apply route;
2. select `load:L-100` / primitive `load:L-100-Y`, edit its unit-bearing
   magnitude from `350 N` to `425 N`, and apply through the structured seam;
3. save locally, quit, relaunch, list local projects, and reopen
   `project:invented-loop-01`;
4. confirm `425 N` survives reopen with no pending edit and no stale solved
   state represented as current; and
5. run mechanics after reopen and require completed state,
   `MECHANICS_SOLVED`, native `tauri_backend_job`, matching project/model
   identity, and non-empty result rows.

The stored project is isolated app-local invented test data. No private input,
protected content, network, daemon, telemetry, release, professional,
certification, sealing, authentication, or code-compliance claim is allowed.

## Reads, tools, and writes

- Reads: governing instructions/profile; full DEL-07-06 intake and recent
  run records; DAG-009 dependency evidence; Receipt-117; desktop package,
  source, tests, fixtures, `SMOKE.md`, and existing package-build evidence.
- Tools: registered desktop checks, focused Rust/Vitest/Playwright checks,
  exact host-capability build and native GUI execution, Git read-only
  inspection, runtime telemetry, and repository validation tools.
- Product/evidence writes: `apps/desktop/**`; DEL-07-06 `_STATUS.md`,
  `MEMORY.md`, and `_run_records/**`; this managed-run subtree.
- Any additional project-local fixture/evidence path requires a versioned
  brief amendment before writing.
- Exclusions: all other deliverables, shared receipts/registers/coordination
  stage/DAG/decomposition/root governance, branch/Git publication actions.

## Acceptance and review

- The actual packaged application, not browser preview, completes the frozen
  journey and records executable SHA-256, observed route/value/state/result
  predicates, and sanitized screenshots or structured observations.
- Focused checks and registered `desktop-test`, `desktop-build`, and
  `harness-self-check` pass; the package's execution/document changes also
  trigger the registered harness check.
- If `apps/desktop/src/**` changes, a fresh non-delegating read-only Agent 2
  using `software-code-review` must review 100% of the frozen node diff and
  return PASS with no actionable finding; remediation is followed by a fresh
  re-review.
- The exact packaged-smoke bullet is removed only after accepted proof. The
  PDU holds and lifecycle state remain unchanged.
- DEC-025 final clean-commit sweep may be retained for HELP_HUMAN/CHANGE if
  clean-commit binding is required; this return must name its exact state.
