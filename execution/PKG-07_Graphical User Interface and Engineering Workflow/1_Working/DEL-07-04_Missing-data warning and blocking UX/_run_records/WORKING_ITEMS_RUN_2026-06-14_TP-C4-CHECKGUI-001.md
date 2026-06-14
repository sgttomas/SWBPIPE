---
run-id: WORKING_ITEMS_RUN_2026-06-14_TP-C4-CHECKGUI-001
timestamp: 2026-06-14T00:00:00-0600
completed: 2026-06-14T00:40:00-0600
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/apps/desktop
write-authorization: COORDINATION_LOOP_PREAPPROVED_TRANCHE
---

# WORKING_ITEMS_RUN TP-C4-CHECKGUI-001 — GUI "Run checks" action + per-check results panel (C4 GUI slice)

## Tranche and authority basis

- Tranche: completion-plan Phase C item **C4** GUI residual — "GUI 'Run checks'
  action + per-check results panel (`TP-C4-CHECKGUI-001`), makes the R3 exit
  criterion GUI-true." Selected as the earliest unblocked item on the Phase C
  dependency spine (`plans/PLAN_2026-06-10_prd_completion.md` §3 C4 row;
  `_COORDINATION.md` Application Integration And Issuance Loop step 3.1). The C4
  backend (`run_rule_checks` command + `core/rules/rule_check_runner`) landed
  2026-06-13 (`TP-C4-CHECKRUN-001`) but had **no GUI surface**: there was no
  frontend service route for `run_rule_checks` and no panel that runs checks
  (the pre-existing `RuleCheckPanel.tsx` is a *completeness review*, not a
  runner). This slice makes PRD §22.4 GUI-true: a user runs a private rule
  pack's checks from the GUI and sees per-check `USER_RULE_CHECKED` /
  `USER_RULE_FAILED` / `RULE_INPUTS_INCOMPLETE` plus the worst-of aggregate,
  with pass/fail blocked on missing inputs.
- D-02b status at execution: AWAITING_RULING. It gates only a *writable
  expression text syntax*; it does **not** gate running checks on already-
  authored declarative-AST packs, so it did not block this slice.
- No human decision was taken. No new backend logic, schema, or example-pack
  change — the slice is purely the app-side surface over the existing command.

## Changes

### Created — frontend

- `apps/desktop/src/services/ruleCheckService.ts`: typed `runRuleChecks(...)`
  route over the `run_rule_checks` Tauri command, with the honest desktop-only
  unavailable seam in browser preview (`RULE_CHECK_BACKEND_DIAGNOSTIC`; no
  synthesized fallback evaluator, mirroring `rulePackService`/
  `libraryImportService`). Result types mirror the runner crate's
  `RuleCheckRunResult`/`CheckOutcome`/`BoundInput`/`RunFinding`/
  `ComputedQuantity`. Pure, unit-tested `deriveRuleCheckBindingPlan(document)`
  partitions a pack's `required_inputs` by `source_kind`
  (solver_result → result-row binding; private_library_value → deferred/
  unsupplied note per the C3 residual; everything else → user value entry) and
  collects `value_slots`. `loadDemoRuleCheckPack()` loads the bundled invented
  demo pack.
- `apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx`: the "Run Rule
  Checks" panel. Pack source = bundled invented demo, a saved local-store pack
  (reuses `listLocalRulePacks`/`openLocalRulePack` — the author→save→run
  journey), or pasted JSON. Binding controls are derived from the loaded pack:
  a result-row `<select>` per `solver_result` input, value+unit entry per
  user-supplied input and per value slot (dimension comes from the pack), and a
  deferred-note row per `private_library_value` input. Runs via the service and
  renders the aggregate status (with a pass/fail/blocked label and
  `data-status`), per-check outcomes (status, computed/limit quantity,
  acceptability relation, supplied/MISSING bound inputs, completeness +
  evaluator findings, diagnostic codes), and the professional-boundary notice.
- `fixtures/product_preview/invented_demo_rule_pack.json`: byte-parallel copy of
  the backend example fixture `examples/rule_packs/invented_demo.yaml` (the
  command tests' `invented_demo_rule_pack`), loaded into the GUI demo via the
  established `fixtures/product_preview/*.json` dynamic-import mechanism. A
  valid, runnable invented non-engineering pack — not an engineering basis.
- Tests: `apps/desktop/src/services/ruleCheckService.test.ts` (7) and
  `apps/desktop/src/features/rule-check/RuleCheckRunPanel.test.tsx` (6).

### Modified

- `apps/desktop/src/App.tsx`: import + mount `RuleCheckRunPanel` in the Solve
  workspace section (it consumes the solved `result`), adjacent to the
  completeness `RuleCheckPanel`.
- `apps/desktop/e2e/r2-smoke.spec.ts`: new browser-lane e2e — open Solve, load
  the demo pack, assert the derived binding controls (solver select + value +
  slot), and assert running routes to the desktop-only seam (the runner is
  Tauri-only; pass/fail/blocked outcomes are covered by the Rust command tests
  and the Vitest desktop-mode mocked panel suite).

## Evidence

- `npm test --workspace apps/desktop` (Vitest): **339 pass** (was 326; +13 — 7
  service + 6 panel). Includes: binding-plan partition/empty/fallback; run-route
  browser-unavailable seam; backend-present invoke with empty bindings omitted;
  non-empty bindings forwarded + solved-envelope preferred over model; demo-pack
  load shape; panel scope/no-solve seams; demo load → binding controls; JSON
  parse error; browser-preview run seam; desktop-run per-check outcome + aggregate
  rendering.
- `npm run build --workspace apps/desktop` (tsc -b + vite build): **PASS** (demo
  pack emitted as its own chunk; wasm-engine dist guard satisfied).
- `npx playwright test` (both viewports): **10/10 pass** (5 specs ×
  chromium-desktop + chromium-compact), including the new run-checks spec.
- Browser visual check (dev server, Solve section): the panel renders with
  project scope, the honest no-solve reason, the pack-source controls, and — on
  "Load demo rule pack" — the derived binding controls
  (`rule-check-solver-select-demo_actual_quantity`,
  `rule-check-value-input-demo_limit_quantity`,
  `rule-check-slot-input-demo_limit_slot`).
- Five-surface evidence sweep (DEC-025): see the committed sweep summary. As
  recorded for `TP-C4-CHECKRUN-001` and the DEC-025 sweep note, the sweep's
  overall status reads `fail` only because the Playwright worker processes do
  not exit within the 300s teardown grace in this execution environment and are
  force-killed (maintainer-confirmed 2026-06-14 environmental artifact, not a
  test/product failure — every e2e test passes). No Rust/Python surface changed
  in this slice.

## Residuals and hand-offs

- **C3 residual** — `private_library_value`-sourced input resolution (rule-pack
  ↔ library reference wiring). The panel surfaces such inputs as deferred and
  the runner treats them as unsupplied (blocks); resolution lands with that
  slice.
- **Solve-envelope wiring** — driving `aggregate_status` into the solve
  envelope's `status.rule_check` / a `result_export` `UserRuleCheck` set (carried
  from the C4 backend record; not in this GUI slice).
- **Additive schema decision-candidates** — per-check `acceptability_relation`
  (beyond `<=`) and an explicit solver-result-selector member on
  `CheckDefinition` (the GUI's caller-supplied binding stands in until then).
- **Saved-pack run journey at packaged-Tauri scale** — the saved-store pick and
  the desktop run are exercised by Rust command tests + mocked Vitest; a
  packaged-app human run rides the F-4 prepared kit (R3-exit blocking residual).

## Boundary compliance

Local-only (Tauri command / local SQLite store / bundled invented fixture; no
network, daemon, telemetry, or repository-default private-data writes). The
panel and runner emit only the three automatic rule-check statuses; the panel
text and boundary note make no compliance/certification/sealing/authentication/
approval/code-compliance or professional-acceptance claim (human review remains
required). Invented demo pack only; user packs stay in local storage and are
never committed. Git/test evidence is source-control hygiene, not a lifecycle,
release, professional, or code-compliance claim.

## Open decisions awaiting human ruling

- **D-02b** — writable rule-expression text syntax (`AWAITING_RULING`); does not
  block this slice (checks run on the authored declarative AST; the panel
  provides no expression text input).
