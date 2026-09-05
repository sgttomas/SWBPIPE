# N1 build V2 — PASS

Parent: HELP_HUMAN; bounded compiled-native package verification; ephemeral Agent 2, no delegation. Role/nondelegation instruction+config asserted; actual model not exposed.

Parent released a fresh build after final W7 repair review, including styles.css hash `5e239ae690ac918d5aa4577916c816e6accd21eb89023a29f5c7b9e2c9de5b5b`. Built from HEAD `e5ec1d4c6dc83315486b1df121474c6e2a75be3d` plus captured working changes. All 626 current source/fixture/config/lock inputs matched before and after actual build/self-test.

## Actual execution
- `npm run tauri -- build --bundles app` in `{WORKING_ROOT}/apps/desktop`, Cargo offline, exit 0, 18.446 seconds. Existing command rebuilt both WASM engines, production frontend, native executable, and app bundle. Vite reports existing large-chunk warnings; build passes.
- Bundle executable `--self-test-saved-edited-load`: exit 0, PASS. Invented load 350 to 425 N, isolated SQLite save, close writer, reopen 425 N, MECHANICS_SOLVED with 830 result rows and edited result differing from baseline.
- Binary SHA-256: `da744f96b3102842197861a2b4a71309c46bf28eb1527cdfe5c604813733b66c`.
- Bundle: `{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.
- Default linker ad-hoc signature, TeamIdentifier unset. No signing-related environment keys, developer identity, certificate, notarization, or registration changes.

Exact source bindings, build command/output bytes, runtime versions, binary output, signing inspection, and complete bundle/frontend/WASM inventories are preserved under `../_run_records/build_v2/`. The manifest binds those evidence records and this return. Original N1 manifest and continuity packets are unchanged; this new build supersedes earlier bundle bytes for current-source testing.

## Limits and handoff
No GUI interaction, user-running app replacement/quit, user models, source/config/Git writes, or escalation. Compiled backend proof only; no new native usability or engineering acceptance. Working-tree evidence does not replace root independent review or the commit-bound DEC-025 sweep. Build slot released. Snapshot is derivative verification evidence; no authoritative scope/lifecycle changes. No remaining blocker within N1 scope.
