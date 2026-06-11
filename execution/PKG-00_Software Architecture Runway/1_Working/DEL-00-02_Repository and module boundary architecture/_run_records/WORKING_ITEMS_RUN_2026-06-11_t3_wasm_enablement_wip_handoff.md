# WORKING_ITEMS run — TP-SEAM-WASM-001 (plan T3) WIP handoff

**Date:** 2026-06-11
**Status:** WIP_HANDOFF — tranche NOT complete; session paused by the human
project authority for context-window handoff. This record is handoff prose
only; it cites the authoritative surfaces below and replaces none of them.
**Plan:** `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T3
(accepted; D-13 RULED 2026-06-11 → `DEC-020`, ADR-0001).

## Landed before this WIP (committed and pushed)

- T1 `TP-SEAM-CORPUS-001` — commit `46e0e0718` (44-case cross-engine contract
  corpus + `LocalFeaHandoffPanel` rider).
- T2 `TP-SEAM-DECISION-001` — commit `801c2e98d` (`docs/architecture/adr/`
  ADR-0001 accepted; `DEC-020` in SOFTWARE_DECOMP §12; D-13 register row RULED).
- Plan/completion-log maintenance — commit `4029e5b3a`.

## T3 work in progress (committed as `wip(seam-T3)`, UNVERIFIED at runtime)

Done and locally verified:

- `core/model_operations/operation_applier`: feature-gated `wasm` wasm-bindgen
  exports (`Cargo.toml` + `src/lib.rs`). Native suite green after the change
  (34 lib tests + 2 contract-corpus integration tests, `cargo test`);
  `cargo build --features wasm` compiles on the host target.
- `apps/desktop/scripts/build-wasm-engine.mjs` + desktop `build:wasm` script +
  root `build:wasm:desktop` script.
- `apps/desktop/src/services/wasmEngine/loadWasmEngine.ts` loading shim
  (explicit `WASM-ENGINE-ASSET-ABSENT` failure; no silent fallback).
- `.gitignore`: `apps/desktop/src/services/wasmEngine/__generated__/`
  (generated artifacts are never committed).
- Wasm lane appended to `apps/desktop/src/services/operationContractCorpus.test.ts`
  (44 cases through the wasm engine + route assertion
  `local_wasm_engine` + structured input-error envelope case).

NOT done / NOT verified — next session's work, in order:

1. **The spike (plan §3 T3 first task, stop rule applies):** install/verify the
   pinned toolchain (`wasm32-unknown-unknown` target, wasm-bindgen tooling),
   run `npm run build:wasm --workspace apps/desktop` to produce
   `__generated__/`, then `npm test --workspace apps/desktop` — the wasm lane
   has NEVER run; no `__generated__` artifact exists yet. If wasm loading in
   Vitest/jsdom proves intractable: stop and escalate per plan §6 — do not
   hand-write any TS fallback engine.
2. `docs/BUILD_AND_RELEASE.md` §3 baseline sentence (toolchain prerequisite +
   build step; per plan write targets).
3. Evidence chain at completion: native `cargo test` unchanged; three-way
   native↔wasm↔TS corpus parity in Vitest; desktop production build green.
4. Run records to DEL-00-02, DEL-00-08, DEL-16-02; plan T3 row compression +
   `plans/PLAN_COMPLETION_LOG.md` entry; tranche commit/push closeout.

## Constraints still in force

- Plan §5 freeze rule (no new operation kinds/field rules in the TS engine)
  holds until T4 lands. T4 (`TP-SEAM-SWAP-001`) proceeds only after T3 evidence.
- Boundary prohibitions unchanged (invented fixtures; no network/daemon
  surfaces — wasm is in-process; no compliance/approval claims).
- `init/init-prompt.md` modification is pre-existing external-scope noise
  (plan §9.7; human git action) — recorded and bypassed, not committed here.
