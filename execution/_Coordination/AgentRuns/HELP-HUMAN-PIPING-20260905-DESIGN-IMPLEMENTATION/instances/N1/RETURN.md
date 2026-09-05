# N1 return — PASS, compiled native backend verification

Parent: HELP_HUMAN. Scope: authorized PKG-07/PKG-02 integration witness. Ephemeral Agent 2; no children. Native role/nondelegation instruction+config asserted; actual model not exposed.

## Executed evidence
- Parent released build after W7 SOURCE_V2 review PASS and W2 review PASS.
- Build: `npm run tauri -- build --bundles app`, cwd `{WORKING_ROOT}/apps/desktop`, `CARGO_NET_OFFLINE=true`; exit 0, 101.508 seconds. Existing beforeBuildCommand rebuilt operation/self-weight WASM and TypeScript/Vite production assets.
- 626 source/config/lock/fixture inputs bound before build and verified unchanged after build/self-test. W7 13-file freeze and W2 source/fixture hashes matched parent release. Base HEAD `7458e9c1eb9399ed259da464207d9a507acdea2e`, working-tree implementation evidence.
- Binary: `{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop`.
- Binary SHA-256: `c37d5f9e70f9727cf47323f44b765171ca38a14baefb577ebdb3a79136f23c68`.
- Executed bundle binary `--self-test-saved-edited-load`: exit 0 / PASS. Invented load changed 350 to 425 N, persisted in isolated file-backed SQLite, writer connection closed, reopened value 425 N, native mechanics solved with 830 result rows; edited result differs from baseline. The self-test itself cleans its unique temporary store.
- Read-only code-signature inspection: default linker ad-hoc signature, TeamIdentifier unset, no developer identity or notarization. No signing-related environment keys present; no signing/registration/configuration changes.

## Evidence and artifacts
Exact execution metadata/output and source/asset inventories reside under `_run_records/build_v1/`. `BUILD_LOG_BYTES.json` preserves original build-log bytes via base64 plus SHA-256; `SELF_TEST.stdout.json` records actual binary output. `BUNDLE_VERIFICATION.json` binds full bundle/frontend/both WASM inventories and binary hash. Preflight observed host metadata remains preserved under `_run_records/preflight/`.

The app bundle is available at `{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app`.

## Limits and handoff
No GUI opened, no user-running app quit/replaced/identified, no user model read or changed, no source/config/Git writes, no escalation needed. This verifies compiled native backend behavior; it does not prove GUI layout, usability, the complete interactive workflow, broader numerical acceptance, or release readiness. Root independent fan-in and DEC-025 sweep remain required. Build slot released; generated ignored assets are current to captured source. Report/evidence is derivative and does not amend authoritative scope or lifecycle state.
