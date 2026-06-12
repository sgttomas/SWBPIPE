# WORKING_ITEMS Run Record — T4 Apply-Panel Engine Status (TP-SEAM-SWAP-001 fan-out)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona). Fan-out summary for DEL-07-02
  (model tree and property inspector / edit-loop UI surface). Primary
  record:
  `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-11_t4_wasm_engine_swap.md`.
- Tranche: `TP-SEAM-SWAP-001` under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T4 (D-13 RULED →
  `DEC-020` / ADR-0001).

## UI-Surface Relevance

The inspector → queue → validate/apply loop is visually unchanged; what
changed underneath and on the receipts:

- `App.tsx` warms the operation engine on mount and passes an honest
  status to the Apply Operations panel. New panel line
  (`data-testid="operation-engine-status"`):
  `engine_route=tauri_backend_apply|local_wasm_engine`;
  `engine_state=loading|ready|unavailable`, with the named
  `WASM-ENGINE-ASSET-ABSENT` diagnostic and exact build command when the
  wasm artifact is missing. No silent fallback exists.
- Applied receipts in the panel ledger now read `route=local_wasm_engine`
  in browser mode (`route=tauri_backend_apply` unchanged under Tauri).
- Every inspector/load-manager/viewport edit applied in jsdom App tests and
  in the Playwright spec now executes through the wasm32
  `operation_applier` build — the shipped engine — rather than a TS copy,
  which retroactively upgrades the probative value of the UI edit-loop
  evidence.

## Evidence (2026-06-11, this host)

Vitest 140/140 (the full App edit-loop suite passed through the wasm engine
with only the route-string assertion updated); Playwright 1/1 with the
engine-ready wait and the end-of-spec apply + `route=local_wasm_engine`
receipt assertion in real Chrome; production build green. Full evidence
table in the primary record.
