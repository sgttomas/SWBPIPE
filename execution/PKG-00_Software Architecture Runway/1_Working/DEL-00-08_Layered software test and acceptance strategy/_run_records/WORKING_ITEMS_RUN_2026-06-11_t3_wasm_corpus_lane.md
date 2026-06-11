# WORKING_ITEMS Run Record — T3 Wasm Corpus Test Lane (TP-SEAM-WASM-001 fan-out)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona). Fan-out summary for DEL-00-08
  (layered software test and acceptance strategy). Primary record:
  `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-02_Repository and module boundary architecture/_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_completion.md`.
- Tranche: `TP-SEAM-WASM-001` under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T3 (D-13 RULED →
  `DEC-020` / ADR-0001).

## Test-Strategy Relevance

The cross-engine contract corpus (TP-SEAM-CORPUS-001, 44 cases) gained its
third lane: the wasm32 build of `operation_applier`, loaded under
Vitest/jsdom exactly as browser mode loads it
(`apps/desktop/src/services/operationContractCorpus.test.ts`, wasm describe
block). The corpus now proves native↔wasm↔TS three-way parity — semantic
outcomes and corpus-harness canonical sha256 equality — inside the normal
`npm test --workspace apps/desktop` run, with no separate harness.

Lane properties:

- An absent wasm artifact is a loud named failure
  (`WASM-ENGINE-ASSET-ABSENT` with the `npm run build:wasm` remediation
  command and every probed path); the lane never skips silently and no
  fallback engine exists.
- Two wasm-specific assertions beyond the 44 cases: the honest
  `local_wasm_engine` application route, and the structured input-error
  envelope (`WASM-ENGINE-INPUT-JSON-INVALID`) for malformed input instead of
  a trap.
- The spike confirmed wasm loading is viable in Vitest/jsdom (plan §6 stop
  rule did not fire); the only fixes needed were artifact path probing under
  Vite's `import.meta.url` rewrite and a `BufferSource` typing copy.

## Evidence (2026-06-11, this host)

- `cargo test` (operation_applier): 36/36 — native suite unchanged.
- `npm run build:wasm:desktop`: OK (pinned wasm-bindgen 0.2.123).
- `npm test --workspace apps/desktop`: 151/151 (46 wasm-lane tests all green).
- `npm run build --workspace apps/desktop`: green (tsc clean; index chunk at
  the ~577 kB baseline — engine dynamically loaded).

T4 (`TP-SEAM-SWAP-001`) will retire the TS corpus lane with the TS engine;
the corpus then continues as the native↔wasm parity + regression surface.
