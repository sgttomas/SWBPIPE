# WORKING_ITEMS Run Record — T3 Wasm Engine Parity (TP-SEAM-WASM-001 fan-out)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona). Fan-out summary for DEL-16-02
  (operation validation and diff preview). Primary record:
  `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-02_Repository and module boundary architecture/_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_completion.md`.
- Tranche: `TP-SEAM-WASM-001` under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T3 (D-13 RULED →
  `DEC-020` / ADR-0001).

## Seam Relevance

The operation validation/apply seam now has a second runtime of the SAME
engine: `core/model_operations/operation_applier` compiled to
wasm32-unknown-unknown with feature-gated wasm-bindgen JSON-string exports
(`validate_operation_json` / `apply_operation_json`). The 44-case contract
corpus (TP-SEAM-CORPUS-001) passes three ways — native Rust reference, wasm
build, TS engine — with identical semantic outcomes (validation states,
diff-preview rows, order-insensitive diagnostic records) and identical
corpus-harness canonical sha256 hashes for applied documents. Hash evidence
is therefore stable across the wasm boundary: the no-silent-defaults and
hash-evidence guarantees do not fork by environment.

Seam-visible properties landed with the wasm lane:

- Honest receipts: the wasm engine reports `application_route:
  "local_wasm_engine"` (asserted in the lane).
- Malformed input produces a structured blocking envelope
  (`openpipestress.desktop.wasm_engine_input_error` /
  `WASM-ENGINE-INPUT-JSON-INVALID`), not a trap.
- Engine absence is a named diagnostic (`WASM-ENGINE-ASSET-ABSENT`) with the
  build command; no fallback engine exists or may be added.

## Evidence (2026-06-11, this host)

`cargo test` 36/36 (native unchanged); `npm run build:wasm:desktop` OK
(pinned wasm-bindgen 0.2.123); `npm test --workspace apps/desktop` 151/151
(three-way corpus parity); `npm run build --workspace apps/desktop` green.

T4 (`TP-SEAM-SWAP-001`) next: route browser-mode `operationService` through
this wasm engine and delete the TS validation/apply logic; until T4 lands
the plan §5 freeze rule holds (no new operation kinds or field rules in the
TS engine).
