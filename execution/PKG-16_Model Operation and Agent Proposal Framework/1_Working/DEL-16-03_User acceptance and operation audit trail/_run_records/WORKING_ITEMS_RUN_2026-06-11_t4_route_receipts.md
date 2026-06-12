# WORKING_ITEMS Run Record — T4 Honest Route Receipts (TP-SEAM-SWAP-001 fan-out)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona). Fan-out summary for DEL-16-03
  (user acceptance and operation audit trail). Primary record:
  `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-11_t4_wasm_engine_swap.md`.
- Tranche: `TP-SEAM-SWAP-001` under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T4 (D-13 RULED →
  `DEC-020` / ADR-0001).

## Acceptance/Audit-Trail Relevance

Applied-operation receipts stay honest through the engine unification:

- The browser-mode `application_route` receipt value is now
  `local_wasm_engine` — the receipt names the engine that actually produced
  the outcome (the wasm32 `operation_applier` build sets this itself; the
  adapter never rewrites routes). The Tauri value `tauri_backend_apply` is
  unchanged. The retired value `browser_fixture_local_apply` no longer
  exists in the type union or any receipt.
- Acceptance semantics are unchanged: `user_initiated_apply_in_local_session`
  basis, `session_state_only_not_yet_saved` persistence,
  `acceptance_is_professional_approval=false` — asserted through the UI
  apply flow in App tests and in the Playwright receipt assertion.
- The apply panel now also reports engine readiness
  (`operation-engine-status`: `engine_route`/`engine_state`); when the wasm
  artifact is absent the status carries the named
  `WASM-ENGINE-ASSET-ABSENT` diagnostic with the build command — apply
  attempts surface the same explicit error through the operation message.
  No fallback engine exists, so no receipt can ever name one.

## Evidence (2026-06-11, this host)

Vitest 140/140 (UI receipt assertion updated to `route=local_wasm_engine`);
Playwright 1/1 — real-browser apply of the explicit viewport node intent
with the `route=local_wasm_engine` receipt asserted on the applied ledger;
full evidence table in the primary record.
