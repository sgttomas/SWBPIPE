# CP2_RUNTIME_TESTS commands (machine-specific record)

Checkout: /home/user/wt/loadstate, branch codex/piping-load-states-20260925.
Crate directory: /home/user/wt/loadstate/projects/chirality-piping/core/product_physics.
Toolchain: cargo 1.97.1 (c980f4866 2026-06-30), rustc 1.97.1 (8bab26f4f 2026-07-14);
syntax/format check used stable rustfmt 1.8.0-stable (e408947bfd 2026-03-25),
because the 1.97.1 toolchain has no rustfmt component installed.

1. Input hash checks (before starting):
   sha256sum CP2_RUNTIME_TESTS/BRIEF.md CP2_WIRE.md
   -> 4cf1ae3d8f2dceffefaefde5ee489cda9bf53423d0bb4b47c49ae514669c9b77 BRIEF.md
   -> 81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996 CP2_WIRE.md
   git show 9e8a55daecdeb9669131fd3e53c0e0303ee550d6:<...>/CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/{VERIFICATION,INTERFACE}.md | sha256sum
   -> 061f1f8f... (VERIFICATION.md) and 18587811... (INTERFACE.md), matching the fixture's `sources`.

2. Syntax check before the facade compiled (no Cargo):
   rustfmt --edition 2021 --check tests/load_reference_state_runtime.rs   (formatting diffs only; parsed)
   rustfmt --edition 2021 tests/load_reference_state_runtime.rs           (own file only)

3. Run 1, after the manager reported "facade compiles" (raw log: run_1.log):
   CARGO_TARGET_DIR=/home/user/cargo-targets/loadstate/product_physics_tests \
     cargo +1.97.1 test --locked --offline -j 1 --test load_reference_state_runtime -- --nocapture
   Test file sha256 734f59a3fccfb175d41ae0cd286fce294143c949bd12dcb1168e122495dee1af; HEAD 5511596af + uncommitted
   implementer working tree. Result: 20 passed, 0 failed, exit 0.

4. Edit: observed-block prints gained each blocking diagnostic's message (no assertion or expectation change).
   rustfmt --edition 2021 tests/load_reference_state_runtime.rs; rustfmt --check -> clean.

5. Run 2, same command, wrapped in `timeout 580` (raw log: run_2.log):
   Test file sha256 eef0ddd53d03301e6d9db004b2ba683dd18e49c44c7e66eacd0758036af12402 (final).
   Result: 20 passed, 0 failed, exit 0.
   src tree hash before/after differed: only src/case_state/resolve.rs changed during the run
   (2a85c490... -> e47ca49a...), from concurrent implementer edits. The exact compiled source
   state is therefore not pinned; rerun on the frozen candidate.

6. Edit after the manager's SF2/SF3 notice (added variants only; no existing expectation
   or tolerance changed):
   - the thermal datum test adds a degF variant and the reviewed control-4 two-point table
     (50/150 degC, datum 20 degC outside the table), in degC and in K;
   - the signed-fit fixed test adds a variant with installation and operating temperatures
     authored in K, while points and datum stay in degC.
   rustfmt --edition 2021 tests/load_reference_state_runtime.rs; rustfmt --check -> clean.

7. Run 3, same command, wrapped in `timeout 580` (raw log: run_3.log):
   Test file sha256 d5be0bd812a2af51c1a32fc5c237a42b76b460ae75d0a610837f01b676f6960b (final).
   HEAD 5511596af66085ec1d70fab1277431b9023744ff + uncommitted implementer working tree;
   src_tree_sha256 58dd2feb9edfc80670cb9c2893f372c3150616bfa6b28d82366d06ae6c6fb3e9, identical
   before and after the run (per-file hashes are in the log). Result: 20 passed, 0 failed, exit 0.

No other Cargo invocation, no browser/native/UI run, no Git write.
