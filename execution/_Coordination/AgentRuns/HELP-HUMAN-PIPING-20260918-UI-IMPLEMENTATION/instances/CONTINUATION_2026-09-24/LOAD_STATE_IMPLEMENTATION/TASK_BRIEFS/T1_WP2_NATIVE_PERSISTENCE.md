# TASK brief — T1_WP2_NATIVE_PERSISTENCE (native 0.4.0 document acceptance and persistence)

Read `_T1_COMMON.md` first, including its **Wave 2** section. Return folder: `LSI/T1_WP2_NATIVE_PERSISTENCE/`.

## Assignment

**Native.** Change `apps/desktop/src-tauri/src/model_document_migration.rs` and the native project persistence and IPC path so that a 0.4.0 model document is accepted exactly as 0.3.0 is accepted today:

- retained without migration;
- no down-migration and no upgrade (D3);
- no invented coefficient, datum, fit, predecessor or state;
- no whole-model serde error for records the native side does not model: they round-trip byte-exactly.

A pre-0.4 document, and a 0.3.0 one, must open unchanged, with no 0.4.0 keys added. A pre-0.4 document that carries 0.4.0 keys is not silently normalized. It is retained as authored, and the product's `LOAD_STATE_CONTRACT_VERSION_MISMATCH` block is what the user sees on solve. If the native side must refuse instead, stop and ask the manager.

Saved results keep their raw result hash and resolved evidence bytes unchanged through save and reopen.

**Portable schemas.** If `schemas/model.schema.yaml` or `schemas/project_persistence.schema.yaml` validate model bytes, add additive 0.4.0 coverage that agrees with `schemas/load_reference_state.schema.json`. Pre-existing branches keep their bytes and meaning.

## Evidence limits (read before starting)

This Linux host has no WebKitGTK development libraries, so `apps/desktop/src-tauri` cannot be compiled or tested here, and the DEC-025 sweep does not cover it (`T1_PLAN.md` §6).

- Keep the native change small and self-contained, in pure Rust with no new Tauri API use.
- **Host evidence.** Compile and unit-test the changed module(s) in a scratch crate outside the repository that includes the module source verbatim (for example with `#[path]`) and stubs only what it cannot link. The tests you add to `src-tauri` must be exactly those you ran in the scratch crate. Record the harness in `_run_records/`.
- State plainly in your return that `src-tauri` itself was not built. The owner's Mac runs the real suite (native witness 1).

## Write boundary

- `apps/desktop/src-tauri/src/**` (the migration, persistence and IPC files and their tests only).
- `schemas/model.schema.yaml`, `schemas/project_persistence.schema.yaml` (additive), and their Python tests in `tests/`.
- Your return folder.
- Not `apps/desktop/src/**`: that belongs to T1_WP2_DESKTOP_READERS. If a TS IPC type must change, describe the change for the manager.

## Checks

- The scratch-crate tests, both new and pre-existing, for the module(s) you change.
- The pytest schema suites you touch, plus `tests/test_load_reference_schema.py`.
- Mutants: remove the 0.4.0 acceptance, the byte-exact retention and the no-injection guards. Every mutant must be killed in the scratch crate.
