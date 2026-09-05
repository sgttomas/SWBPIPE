# N1 — native package verification

Parent: HELP_HUMAN `/root`; child `/root/native_verification`, ephemeral Agent 2, no delegation.
Delegation: delegated-harness-native; role/nondelegation instruction+config asserted, not mechanism-proven. Actual model: unknown (not exposed).

## Authorized bounded integration scope
Verify frozen, reviewed PKG-07 GUI / PKG-02 native-unit implementation through an actual locally built macOS Tauri binary. Source base supplied by parent: `7458e9c1eb9399ed259da464207d9a507acdea2e`, branch `codex/piping-modeling-workspace`. Parent supplies final freeze/release; working-tree content hashes bind actual build.

Read scope: root/project AGENTS, build guide, desktop/native/build source and manifests, applicable parent/W7/W2 evidence. Write scope: this instances/N1 directory, existing ignored build outputs, isolated system-temp artifacts. No source/config/authority/Git/user app/model writes. No delegation, GUI interaction, install/registration, certificate/signing changes, or use of user saved models.

## Sequence and acceptance
1. Read-only preflight now. Do not build until explicit parent source-freeze/exclusive-build release.
2. Capture exact source/lockfile/config inputs, Git state, runtime versions and existing generated assets.
3. Run `npm run tauri -- build --bundles app` in apps/desktop, offline Cargo where supported. Existing beforeBuildCommand rebuilds operation + self-weight WASM and frontend. No concurrent cargo, WASM, or sweep writers.
4. Hash resulting bundle/binary/frontend/WASM; run bundle executable with `--self-test-saved-edited-load`, capturing exact output and exit. Existing self-test enters before GUI and uses a unique isolated temp SQLite store, cleaned by implementation.
5. Verify source input hashes did not change. Return command/version/hash binding, bundle path, result and limits. Root independent review and DEC-025 sweep remain separate.

Scope of evidence: compiled native edit/save/reopen/solve behavior. Not GUI usability proof, engineering acceptance, release publication or identity proof for user-running app. Owner target 1024 × 768 applies to later layout observation only.
