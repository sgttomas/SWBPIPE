# Isolated post-G0 native witness plan V2

Status: **PREPARED; BUILD AND LAUNCH HELD**.

This successor preserves `NATIVE_WITNESS_PLAN_V1.md` as historical evidence. It uses the same source-free Tauri configuration-overlay mechanism and changes only `productName` and `identifier`; committed application source, runtime behavior, frontend configuration, commands, window configuration, and build hooks remain inherited.

## Fresh isolated identity

- Product name: `OpenPipeStress Physics UI Post-G0 Witness 20260908 R2`
- Bundle identifier: `org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2`
- Overlay: `execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_WITNESS_TAURI_OVERLAY_V2.json`
- Isolated store: `{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-post-g0-20260908-r2/openpipestress-projects.sqlite3`
- Invented project name: `Physics UI Post-G0 Default Fixture Witness 20260908 R2`

Exact host paths belong only in `_run_records/NATIVE_WITNESS_HOST_PATHS_V2.json`. Preparation does not probe any store or bundle path and does not inspect the normal application store.

## Release and source binding

Do not build until root releases CHANGE after F4's applied validation and fresh RF applied-candidate PASS. At release, seal one successor source-binding record that:

1. identifies the current Git base and truthfully distinguishes committed HEAD from any live uncommitted source;
2. lists and rehashes all 17 accepted source, test, and fixture members;
3. binds the final F4 manifest, validation, return, and the fresh RF review, inventory, validation, return, and PASS status;
4. binds the approved post-G0 Owner direction and current work graph;
5. binds this overlay by SHA-256; and
6. verifies that every live member equals the independently reviewed post-image before the build starts.

Any member mismatch, active source writer, missing review output, non-PASS review, or scope expansion keeps the build held.

## Build and prelaunch checks

After root releases the serialized native slot, build from `{WORKING_ROOT}/apps/desktop`:

```sh
CARGO_NET_OFFLINE=true npm run tauri -- build --bundles app --ci --no-sign --config "{WORKING_ROOT}/execution/_Change/PHYSICS_UI_IMPLEMENTATION_20260908/_run_records/NATIVE_WITNESS_TAURI_OVERLAY_V2.json"
```

Before launch, require all of the following:

- the build exits zero with no source or lockfile changes;
- all 17 bound members still match;
- a deterministic inventory and digest of the generated `apps/desktop/dist/**` files is recorded;
- a deterministic inventory and digest of the generated `.app` bundle files is recorded;
- the bundle's `Info.plist` reports the exact V2 product name and identifier above and executable `openpipestress-desktop`;
- the dedicated V2 store directory is absent; and
- the normal application store has not been inspected or accessed.

The expected bundle is `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Physics UI Post-G0 Witness 20260908 R2.app`.

## Native smoke and cleanup

Launch remains held for a fresh C0-sealed native brief after the build packet passes. The later Agent 2 smoke must use the default invented fixture in the isolated V2 store and verify the actual packaged UI/runtime, including:

- the default fixture presents 830 result rows;
- retained-spring load case L-100 presents friction reaction `0.411203 N` and derived normal reaction `41.120279 N`; and
- the required default-fixture solve, display, save, reopen, and cleanup evidence is bound to the exact 17-member source cut, dist digest, and bundle digest.

The previous authored R2 walkthrough remains historical and does not substitute for this default-fixture smoke.

After the witness, quit the isolated app, prove no process uses the exact bundle path, and delete only the exact dedicated V2 store directory through a literal-equality and prefix guard. Require post-cleanup absence. Never enumerate, read, hash, modify, move, or delete the normal application store.

This plan authorizes no build, launch, store access, source edit, test run, Git mutation, receipt append, or publication action.
