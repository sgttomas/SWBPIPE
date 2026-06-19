---
run-id: WORKING_ITEMS_RUN_2026-06-19_TP-R3UX-WORKSPACEREDESIGN-001
timestamp: 2026-06-19T00:36:08-06:00
run-status: SUCCESS
persona: WORKING_ITEMS
primary-deliverable: DEL-07-06
tranche-ids:
  - TP-R3UX-WORKSPACEREDESIGN-001
smoke-ids:
  - TP-MAC-272
---

# WORKING_ITEMS Run Record - C5.7R Workspace Redesign Inc 0-5

## Objective

Implement the C5.7R minimum redesign needed before the next packaged human
pass attempt: replace the three competing navigation systems with one ribbon,
demote evidence/telemetry strings into status drawers, make the canvas the
primary work surface, and expose inspector-side dual-unit/provenance/required
field context. This realizes Inc 0-5 from
`plans/PLAN_2026-06-18_workspace_redesign_c5_7.md`.

This run does not close F-4, the A3 authoring-usability finding, R3 exit
review, lifecycle issuance, release readiness, professional approval,
certification, sealing, authentication, or code-compliance.

## Authority and scope

- `plans/PLAN_2026-06-18_workspace_redesign_c5_7.md`: C5.7R Inc 0-5 minimum
  for a first C5.7 human packaged-pass reattempt.
- `DEC-035`: F-4 and A3 remain R3-exit blockers until human packaged evidence
  closes them.
- `DEC-037`: writable rule text remains out of scope; structured composer
  routes are preserved.
- `DEC-041` / `D-21`: agent panel is only a reserved seam; no agent panel was
  built.
- Boundary authorities: `docs/CONTRACT.md`,
  `docs/IP_AND_DATA_BOUNDARY.md`, and project `AGENTS.md`.

Allowed write scope used: desktop frontend source/tests/styles/e2e,
`apps/desktop/SMOKE.md`, DEL-07-06 run record, active plan/coordination
handoff text, and the DEC-025 evidence-sweep summary.

## Outputs produced

- Replaced the old topbar/project toolbar, journey rail, workspace nav, and
  guided journey stack with a slim title bar plus one workflow ribbon:
  Model, Loads, Analyze, Results, Rules, Report.
- Added a global bottom status bar with Mechanics, Rule check, and
  Professional status pills, plus Audit and Issues drawer controls.
- Moved storage/boundary/evidence surfaces into the Audit drawer by default.
- Moved diagnostics, missing-data blockers, and operation diagnostics into the
  Issues drawer by default.
- Enlarged the viewport into the central hero surface and added axis triad,
  view cube buttons, scale bar, load glyph summary, selection handles, and a
  command/selection bar.
- Refactored Operation Apply machine-string summaries into compact chips and
  human-readable applied-operation receipts.
- Extended the Property Inspector with display-only dual-unit context,
  provenance rows, and required-field flags.
- Updated App and dead-control tests to assert the new ribbon/status/drawer
  shell while preserving legacy governed packet assertions.

## Validation

- `npm ci`
  - passed before final validation; npm reported 4 audit vulnerabilities
    (2 low, 2 high), not remediated in this UX tranche.
- `npm run build:wasm --workspace apps/desktop`
  - passed; regenerated local WASM preview assets needed by the desktop build.
- `npm test --workspace apps/desktop`
  - passed: 18 test files / 399 tests.
- `npm run test:e2e --workspace apps/desktop`
  - passed: 18 Playwright checks across desktop and compact Chromium projects.
- `npm run test:e2e:dist --workspace apps/desktop`
  - passed: 1 packaged-production smoke check.
- `npm run build --workspace apps/desktop`
  - passed; Vite retained the existing chunk-size warning for large bundles.
- `python3 tools/release/run_evidence_sweep.py --execute`
  - passed overall; summary:
    `validation/evidence/sweeps/SWEEP_20260619T072613Z_5dbd406023ba-dirty.json`.
  - covered cargo crate sweep, Python pytest (361 tests), desktop Vitest
    (18 files / 399 tests), desktop Playwright e2e (18 checks),
    production-dist Playwright e2e (1 check), and desktop production build.

## Boundary review

This tranche changed frontend shell, presentation, and test coverage only. It
did not change solver mechanics, schemas, evaluator grammar, persistence
contracts, backend APIs, unit storage semantics, private-data policy,
protected-content policy, network/telemetry posture, rule-pack authoring
authority, lifecycle state, release-readiness status, or professional-boundary
semantics.

No protected standards content, private project data, repository-default
private-data write, network/telemetry feature, professional approval,
certification, sealing, authentication, code-compliance, release-readiness, or
R3 exit-review claim is created.

## Residuals and next item

- Inc 6 bulk tabular grid remains a committed C5.7R follow-through item and is
  not claimed complete by this record.
- The next packaged human pass still requires a fresh bundle and human
  execution of TP-MAC-189 before F-4/A3 can close.
- The coordination discrepancy discovered during intake remains: the active
  prompt says all deliverables are CHECKING/ISSUED, while the DAG-006 status
  tool reported `CHECKING=8`, `IN_PROGRESS=92`, `ISSUED=1`.
