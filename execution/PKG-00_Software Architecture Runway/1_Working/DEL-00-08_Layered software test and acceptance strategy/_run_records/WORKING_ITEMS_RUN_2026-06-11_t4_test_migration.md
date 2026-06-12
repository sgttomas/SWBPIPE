# WORKING_ITEMS Run Record — T4 Test Migration (TP-SEAM-SWAP-001 fan-out)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona). Fan-out summary for DEL-00-08
  (layered software test and acceptance strategy). Primary record:
  `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-11_t4_wasm_engine_swap.md`.
- Tranche: `TP-SEAM-SWAP-001` under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T4 (D-13 RULED →
  `DEC-020` / ADR-0001).

## Test-Strategy Changes

- **Engine-test supersession.** The 17 browser-engine tests in
  `operationService.test.ts` (911 lines, testing the deleted TS engine's
  semantics) are superseded by the 44-case cross-engine contract corpus
  plus the Rust crate suite; 6 thin adapter tests replace them (routing,
  Tauri `invoke` passthrough, wasm input-error envelope → explicit throw,
  engine-status reporting). Test counts: desktop Vitest 151 → 140.
- **Corpus lane continuity.** The corpus TS lane is retired with the TS
  engine. The Vitest runner now executes two lanes — the public adapter
  seam (wasm-backed) and the wasm engine directly — against the
  Rust-blessed native reference, preserving semantic and canonical-hash
  parity as the permanent native↔wasm regression surface. Comparison rules
  and case files are unchanged.
- **App-level tests on the shipped engine.** The full App edit-loop suite
  (jsdom) now applies operations through the wasm32 `operation_applier`
  build. `src/test/setup.ts` pre-warms the engine so suites start ready;
  an absent artifact fails every suite loudly with
  `WASM-ENGINE-ASSET-ABSENT` and the exact build command (the engine is a
  test prerequisite by design; no fallback, no silent skip).
- **Playwright posture.** `test:e2e` builds the wasm artifact first; the
  smoke spec waits on the new engine-ready testid
  (`operation-engine-status`) before edits and ends with a real-browser
  apply asserting the `route=local_wasm_engine` receipt and the by-design
  clearing of stale solve results.

## Evidence (2026-06-11, this host)

Cargo profile sweep exit 0 (25 manifests, zero failures); pytest 342/342;
Vitest 140/140 (no act warnings); Playwright 1/1; desktop production build
green. Full evidence table in the primary record.
