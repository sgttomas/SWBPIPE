# Isolated native witness plan

Status: **READY_FOR_FUTURE_SOURCE_BINDING; BUILD_AND_LAUNCH_HELD**.

This plan reuses the established source-free Tauri build-flavor mechanism recorded in `execution/_Change/PHYSICS_UI_EXECUTION_20260908/NATIVE_WALKTHROUGH_ISOLATION.md`. Tauri CLI 2.11.1 merges the two-key overlay with the committed `apps/desktop/src-tauri/tauri.conf.json`. The overlay changes only `productName` and `identifier`; all application source, physics runtime, frontend configuration, commands, window configuration, and build hooks remain inherited.

## Isolated identity

- Product name: `OpenPipeStress Technical Preview Implementation Witness 55df51ac 20260908`
- Bundle identifier: `org.openpipestress.technical-preview.walkthrough-physics-ui-implementation-55df51ac-20260908`
- Overlay: `execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_WITNESS_TAURI_OVERLAY.json`
- Overlay SHA-256: `c97845a67e0a7019074868c2fa29e9ddeabfaea6b19e6dcd982396ef8141a4cc`
- Isolated store: `{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-implementation-55df51ac-20260908/openpipestress-projects.sqlite3`
- Invented project name: `Physics UI Implementation Witness 55df51ac 20260908`

The dedicated store directory and expected application bundle were absent at preparation. Exact structural paths and absence results are in `_run_records/NATIVE_WITNESS_HOST_PATHS.json`. This preparation did not inspect the normal application store or build, launch, or modify the app.

## Release gate and commands

Do not build until root records all of the following in a successor binding record: exact source `HEAD`, frozen F4 and U7 source-slice manifest hashes, independent review artifact hashes and PASS verdicts for both slices, and confirmation that those hashes still match the working tree. The later record must also bind this overlay hash. F4 owns the compile slot until root explicitly releases serialized native verification.

After that release, build from `{WORKING_ROOT}/apps/desktop`:

```sh
CARGO_NET_OFFLINE=true npm run tauri -- build --bundles app --ci --no-sign --config "{WORKING_ROOT}/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_WITNESS_TAURI_OVERLAY.json"
```

Require the resulting bundle at `{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview Implementation Witness 55df51ac 20260908.app`. Before launch, inspect only that bundle's `Contents/Info.plist` and require `CFBundleIdentifier` to equal the isolated identifier. Recheck that the dedicated store directory is absent, then launch:

```sh
/usr/bin/open -n "{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview Implementation Witness 55df51ac 20260908.app"
```

Use only the invented project in the isolated store. Exercise the reviewed native GUI workflow, including save, reopen, solve, and the F4/U7 acceptance interactions specified by their frozen review records. A browser fixture, headless job test, or packaged backend self-test cannot substitute for this GUI witness.

## Completion and cleanup

After capturing the required witness evidence, quit the isolated app and require that no process command contains its exact bundle path. Capture evidence from the isolated store only, then delete exactly the dedicated directory with the guarded procedure below. Never enumerate, open, hash, move, modify, or delete the normal `org.openpipestress.technical-preview` directory.

```sh
native_witness_store_dir='{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-implementation-55df51ac-20260908'
expected_native_witness_store_dir='{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-implementation-55df51ac-20260908'
test "$native_witness_store_dir" = "$expected_native_witness_store_dir"
case "$native_witness_store_dir" in
  '{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-implementation-'*) ;;
  *) exit 64 ;;
esac
rm -rf -- "$native_witness_store_dir"
test ! -e "$native_witness_store_dir"
```

The later verification record must bind the frozen source and review hashes; prelaunch absence; merged `Info.plist` identity; launch, workflow, and quit evidence; isolated-store evidence; and post-cleanup absence. Only after those checks may the owning workflow assess native acceptance. This preparation itself makes no source-acceptance or receipt claim.
