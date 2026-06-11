# WORKING_ITEMS Run Record — T3 Wasm Enablement Completion (TP-SEAM-WASM-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), hardening-lane tranche under
  `plans/PLAN_2026-06-11_operation_seam_unification.md` §3 T3 and the
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop (selection rationale recorded in that plan §8; D-13 RULED →
  `DEC-020` / ADR-0001).
- Tranche: `TP-SEAM-WASM-001` (wasm build enablement + three-way corpus
  parity proof).
- Deliverable context: DEL-00-02 (repository and module boundary
  architecture) — primary record; the wasm engine build seam is a
  module-boundary change (crate → browser asset). Fan-out summaries:
  DEL-00-08 (test strategy: the wasm Vitest lane), DEL-16-02 (operation
  validation seam).
- Predecessor: `WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_wip_handoff.md`
  (this folder) — scaffolding committed as `wip(seam-T3)` (`12ca2946f`),
  spike not yet run. This record completes that tranche.

## Spike Outcome (plan §3 T3 first task — §6 stop rule did NOT fire)

The corpus-through-wasm spike succeeded in the Vitest/jsdom environment.
No TS fallback engine was written or needed; the stop-and-escalate rule was
not triggered. Two environment-fit fixes were required in the loading shim
(`apps/desktop/src/services/wasmEngine/loadWasmEngine.ts`), both artifact
location/typing only — engine semantics untouched:

1. **Wasm-bytes path probing under Node/Vitest.** Vite's test transform
   rewrites `import.meta.url` to a root-relative file URL (`file:///src/...`),
   so module-relative resolution of the `_bg.wasm` bytes fails (`ENOENT` on
   `/src/services/wasmEngine/__generated__/...`). The Node branch now probes
   deterministic candidates (module-relative when the URL is a real file
   path; desktop-workspace cwd; repo-root cwd) and fails loudly with
   `WASM-ENGINE-ASSET-ABSENT` listing every probed path when the artifact is
   genuinely absent. Probing locations of one artifact is not an engine
   fallback; the no-silent-fallback posture is unchanged.
2. **`BufferSource` typing.** Node's `readFile` returns `Buffer`
   (`Uint8Array<ArrayBufferLike>`), which TypeScript 5.9 rejects for
   wasm-bindgen's `BufferSource` init parameter; bytes are copied into a
   fresh `Uint8Array<ArrayBuffer>`. (This also fixed the previously latent
   `tsc -b` failure in the committed WIP shim — caught by the production
   build step of this evidence chain.)

## Toolchain (verified on this host, per the pinned prerequisites)

- `wasm32-unknown-unknown` target installed (`rustup target list --installed`).
- `wasm-bindgen` CLI 0.2.123 — exactly the crate-pinned version.
- cargo 1.92.0. Build script `apps/desktop/scripts/build-wasm-engine.mjs`
  verified all prerequisites and produced
  `apps/desktop/src/services/wasmEngine/__generated__/` (gitignored, never
  committed) via `npm run build:wasm:desktop` on first attempt.

## What Landed (beyond the committed WIP scaffolding)

- `apps/desktop/src/services/wasmEngine/loadWasmEngine.ts` — the two
  environment-fit fixes above.
- `docs/BUILD_AND_RELEASE.md` §3 — repository-baseline paragraph: wasm32
  engine identity (`DEC-020` / ADR-0001), toolchain prerequisites (target +
  pinned wasm-bindgen CLI), build command, generated-artifact non-commit
  rule, explicit-failure posture.
- This record plus fan-out records and plan/completion-log maintenance.

## Validation Evidence (all run by WORKING_ITEMS on this host, 2026-06-11)

| Surface | Command | Result |
|---|---|---|
| Native crate suite (unchanged) | `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml` | 36/36 green (34 lib + 2 corpus integration) |
| Wasm artifact build | `npm run build:wasm:desktop` | OK (wasm-bindgen 0.2.123; artifacts not committed) |
| Three-way corpus parity | `npm test --workspace apps/desktop` | 151/151 green — 105 prior (incl. 44-case TS corpus lane) + 46 wasm-lane (44 corpus cases + `local_wasm_engine` route assertion + structured input-error envelope) |
| Desktop production build | `npm run build --workspace apps/desktop` | Green; `tsc -b` clean; index chunk 577.65 kB (baseline ~577 kB — wasm engine is dynamically loaded, not bundled) |

Parity is three-way per plan §3: the same 44 corpus case files execute
against (a) the native Rust reference (blessed expectations, `cargo test`),
(b) the wasm32 build of the same crate loaded as browser mode loads it, and
(c) the still-present TS engine — identical semantic outcomes and identical
corpus-harness canonical sha256 hashes across all three, including the
hash-stability proof across the wasm boundary.

## Boundaries and Residuals

- Boundary review: wasm runs in-process (no network/daemon/cloud surface);
  invented corpus data only; no compliance/certification/approval language;
  plan §5 freeze rule remains in force until T4 lands.
- Residual: none for T3 scope. T4 (`TP-SEAM-SWAP-001`) — routing browser
  mode through the wasm engine and deleting the TS engine logic — proceeds
  next per the plan.
- External-scope noise: pre-existing uncommitted `init/init-prompt.md`
  modification (plan §9.7; human git action) — bypassed, not committed.
