# Process, store, and cleanup evidence

- Native app quit through the exact window close control.
- Exact executable: `.../OpenPipeStress Technical Preview Implementation Witness 55df51ac 20260908.app/Contents/MacOS/openpipestress-desktop`.
- Exact process matches after close: none (`POSTRUN_GUARD_CHECKS.json`).
- Exact dedicated store integrity before cleanup: `ok`; `local_projects` row count: 1.
- Pre-cleanup directory guard and exact member hashes: `STORE_PRE_CLEANUP.json`.
- Guarded cleanup used exact `Path.unlink` calls for the three allowed SQLite basenames and exact `Path.rmdir` for the dedicated identifier directory. No wildcard and no `rm -rf` were used.
- Post-cleanup exact dedicated store absence: true (`STORE_POST_CLEANUP.json`, `POSTRUN_GUARD_CHECKS.json`).
- No normal store path was accessed or enumerated.
