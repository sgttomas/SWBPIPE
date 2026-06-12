# WORKING_ITEMS Run Record — T4 Wasm Engine Swap (TP-SEAM-SWAP-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), hardening-lane tranche under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T4 and the
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop (D-13 RULED → `DEC-020` / ADR-0001).
- Tranche: `TP-SEAM-SWAP-001` — route browser mode through the wasm engine;
  delete the TypeScript engine.
- Deliverable context: DEL-16-02 (operation validation and diff preview) —
  primary record. Fan-out summaries: DEL-16-03 (acceptance/audit receipts),
  DEL-07-02 (inspector/apply-panel UI surface), DEL-00-08 (test strategy).
- Closes the operation-seam unification plan: with T4 landed, all §4 exit
  criteria are met and the §5 freeze rule is lifted.

## What Landed

- `apps/desktop/src/services/operationService.ts` — 2,280-line browser
  engine (validation/diff/apply logic with private `FIELD_RULES`,
  `CANONICAL_DIMENSIONS`, `RESTRAINT_TOKENS`, `DEFERRED_FIELDS`,
  `COLLECTIONS` tables) **deleted**; replaced by a 112-line thin routing
  adapter: Tauri present → `invoke` `validate_model_operation` /
  `apply_model_operation` (authoritative route, unchanged); otherwise →
  wasm32 `operation_applier` via `loadWasmEngine()`. The adapter also
  exposes `outcomeFromWasmEngineJson` (wasm input-error envelope →
  explicit thrown error, never a silent outcome) and honest engine-status
  reporting (`initialOperationEngineStatus` / `warmupOperationEngine`).
- `apps/desktop/src/types.ts` — `application_route` browser value is now
  `"local_wasm_engine"` (was `"browser_fixture_local_apply"`).
- `apps/desktop/src/App.tsx` — engine warmup on mount with status state
  passed to the apply panel.
- `apps/desktop/src/features/operations/OperationApplyPanel.tsx` — new
  `operation-engine-status` line (`engine_route`/`engine_state`, plus the
  named diagnostic when unavailable).
- `apps/desktop/src/services/operationService.test.ts` — 17 superseded
  engine tests (911 lines) **deleted**; 6 thin adapter tests added
  (browser→wasm routing with route receipt and input non-mutation;
  validate-only; Tauri `invoke` passthrough with exact command/args;
  envelope→throw; engine-status browser and Tauri variants). Engine
  semantics remain covered by the 44-case contract corpus + the Rust suite.
- `apps/desktop/src/services/operationContractCorpus.test.ts` — TS engine
  lane retired by the swap; the first lane now exercises the public adapter
  seam (wasm-backed) and the second the wasm engine directly, both against
  the Rust-blessed native reference (permanent native↔wasm parity +
  regression surface). Comment/title updates only; comparison rules and the
  44 case files unchanged.
- `apps/desktop/src/test/setup.ts` — pre-warms the wasm engine (loud
  `WASM-ENGINE-ASSET-ABSENT` failure with build command if absent).
- `apps/desktop/e2e/r2-smoke.spec.ts` — waits on engine-ready status after
  load; at spec end queues the prepared explicit viewport node intent,
  applies it, asserts the `route=local_wasm_engine` receipt and the
  by-design clearing of stale solve results.
- `apps/desktop/package.json` — `test:e2e` builds the wasm artifact first.
- `apps/desktop/SMOKE.md` — entry TP-MAC-110.

Net deletion across engine + superseded tests: **−3,114 / +200 lines**
(`git diff --stat`); adapter is 112 lines against the plan's ≤ ~250 target.

## Validation Evidence (all run by WORKING_ITEMS on this host, 2026-06-11)

| Surface | Command | Result |
|---|---|---|
| Cargo profile sweep | `python3 tools/release/check_release_readiness.py --profile cargo --execute` | exit 0; 25 manifests discovered; 51 test-result lines all `ok`, zero failures |
| Python suite | `python3 -m pytest -q tests` | 342/342 (unaffected, as planned) |
| Desktop Vitest | `npm test --workspace apps/desktop` | 140/140 across 7 files (was 151: −17 superseded engine tests, +6 adapter tests); no React act warnings |
| Playwright e2e | `npm run test:e2e:desktop` | 1/1 including the new engine-ready wait and engine-route receipt assertions (real Chrome) |
| Production build | `npm run build --workspace apps/desktop` | Green (`tsc -b` + Vite); index chunk at the existing ~577 kB baseline — the wasm engine loads dynamically and is not bundled |

## Semantics Preservation

This tranche changed *where* validation runs, never *what* it accepts: the
adapter contains no validation, diff, or apply logic; every outcome in
every environment is produced by `core/model_operations/operation_applier`
(native via Tauri commands, wasm32 otherwise). The contract corpus passed
unchanged through the swap — the same 44 blessed expectations, including
canonical-hash equality. Deferred fields stay deferred (blocked-stays-
blocked encoded in the corpus).

## Boundaries and Residuals

- Boundary review: wasm in-process; local-only (no cloud/daemon/network/
  telemetry); invented data only; no protected standards content; no
  release-readiness, professional approval, certification, sealing,
  authentication, or code-compliance claims.
- Plan §5 freeze rule honored through the swap (no operation kinds or field
  rules entered the TS engine between plan acceptance and its deletion) and
  lifted at this landing.
- Residuals: none for T4 scope. Post-plan deferrals remain as recorded in
  plan §9 (R2 authoring set and Phase B B2 now unblocked by design;
  packaged-Tauri smoke over a saved edited project remains a deferred item).
- External-scope noise: pre-existing uncommitted `init/init-prompt.md`
  modification (plan §9.7; human git action) — bypassed, not committed.
