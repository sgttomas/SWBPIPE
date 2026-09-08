# Native walkthrough isolation readiness

Status: **READY_AFTER_SERIALIZED_BUILD_RELEASE** at merged-main basis `779dedb8670625b36af07b89fc5557470e47c50e`. This pass prepared and inspected configuration only. It did not build or launch the app, read or modify existing store contents, or change `HOME`. Initial discovery made one metadata-only existence probe against the normal identifier directory; it returned present. No directory enumeration, database open, content read, hash, move, or write occurred, and no further probe is required.

## Supported isolation basis

The installed Tauri CLI 2.11.1 `build --help` defines `--config <CONFIG>` as one or more JSON strings or JSON/JSON5/TOML paths merged with the default configuration in order; conflicting overlay values replace the defaults. The CLI explicitly describes this mechanism as suitable for build flavors. `--bundles app`, `--ci`, and `--no-sign` are supported by this installed command.

The installed schema describes the top-level `identifier` as the unique reverse-domain application identity used for the bundle ID and webview data directory, with alphanumeric characters, hyphens, and periods allowed. The prepared identifier conforms to that character contract. Its paired `productName` also conforms to the schema. The overlay contains only those two keys, so application source, frontend configuration, physics code, commands, window configuration, and build hooks remain inherited from `tauri.conf.json`.

Tauri 2.11.1 source implements `app_local_data_dir()` as the platform local-data directory joined to `config.identifier`. On macOS the installed `dirs` 6.0.0 crate resolves `data_local_dir()` to `Library/Application Support` under the current user. OpenPipeStress source implements every ordinary native project-store command through `app_store_path()`, which calls `app_local_data_dir()`, creates that directory, and appends the fixed filename `openpipestress-projects.sqlite3`.

Therefore a build using the prepared unique identifier uses this isolated store, expressed portably:

`{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-779dedb8-20260908/openpipestress-projects.sqlite3`

The normal application identifier remains `org.openpipestress.technical-preview`; its directory is disjoint and must not be inspected or touched further. The isolated directory and unique `.app` path were both absent during this preflight. Exact host paths and absence results are structural data in `_run_records/NATIVE_WALKTHROUGH_HOST_PATHS.json`.

## Prepared overlay and exact commands

The overlay is `{WORKING_ROOT}/execution/_Change/PHYSICS_UI_EXECUTION_20260908/_run_records/NATIVE_WALKTHROUGH_TAURI_OVERLAY.json`, where `{WORKING_ROOT}` is `projects/chirality-piping` in the execution worktree. Its native application identity is:

- product name: `OpenPipeStress Technical Preview Walkthrough 779dedb8 20260908`;
- bundle identifier: `org.openpipestress.technical-preview.walkthrough-physics-ui-779dedb8-20260908`;
- executable: `openpipestress-desktop`.

After the owning work graph releases the serialized native build slot, build from `{WORKING_ROOT}/apps/desktop` with:

```sh
CARGO_NET_OFFLINE=true npm run tauri -- build --bundles app --ci --no-sign --config "{WORKING_ROOT}/execution/_Change/PHYSICS_UI_EXECUTION_20260908/_run_records/NATIVE_WALKTHROUGH_TAURI_OVERLAY.json"
```

The expected application bundle is `{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview Walkthrough 779dedb8 20260908.app`. Before launch, inspect its `Contents/Info.plist` and require `CFBundleIdentifier` to equal the unique identifier above. Launch the packaged GUI with:

```sh
/usr/bin/open -n "{WORKING_ROOT}/apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview Walkthrough 779dedb8 20260908.app"
```

Use an invented project inside this isolated database, for example project name `Physics UI Walkthrough 779dedb8 20260908`. Native save/reopen/solve then exercises the same application and physics runtime while persisting only to the isolated SQLite path above. The model is a database record; ordinary native save does not create a separately addressable model file.

## Guarded cleanup proof

Before launch, require the unique store directory to be absent. After the walkthrough, quit this unique app, require that no process command contains the unique bundle path, capture any required evidence from the isolated SQLite store, and delete only the exact unique application-data directory. Use a task-specific variable, compare it byte-for-byte to the expected path, and require the test-only identifier prefix before deletion:

```sh
walkthrough_store_dir='{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-779dedb8-20260908'
expected_walkthrough_store_dir='{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-779dedb8-20260908'
test "$walkthrough_store_dir" = "$expected_walkthrough_store_dir"
case "$walkthrough_store_dir" in
  '{USER_APPLICATION_SUPPORT}/org.openpipestress.technical-preview.walkthrough-physics-ui-'*) ;;
  *) exit 64 ;;
esac
rm -rf -- "$walkthrough_store_dir"
test ! -e "$walkthrough_store_dir"
```

The later execution record must preserve the prelaunch absence result, merged `Info.plist` identity, launch/quit evidence, isolated store evidence before deletion, and post-cleanup absence result. It must not enumerate, hash, open, move, or delete the normal `org.openpipestress.technical-preview` directory. This configuration solution removes the need for a test-store source feature.
