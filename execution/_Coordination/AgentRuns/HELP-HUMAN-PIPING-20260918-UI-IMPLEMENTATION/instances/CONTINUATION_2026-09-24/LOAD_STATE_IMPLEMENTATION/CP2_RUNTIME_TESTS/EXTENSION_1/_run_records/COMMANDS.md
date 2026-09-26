# EXTENSION_1 commands (machine-specific record)

- Checkout: /home/user/wt/loadstate, branch codex/piping-load-states-20260925.
- HEAD: d92ca3b7cbbd20a9adcfda50f10202f42b0d997e, plus the uncommitted implementer working tree.
- Crate: /home/user/wt/loadstate/projects/chirality-piping/core/product_physics.
- Toolchain: cargo 1.97.1 (c980f4866 2026-06-30) and rustc 1.97.1 (8bab26f4f 2026-07-14).
  Formatting used stable rustfmt 1.8.0; the 1.97.1 toolchain has no rustfmt.
- Target dir (as ROOT gave it): /home/user/cargo-targets/loadstate/product_physics_tests.

1. **Input hash checks.**
   - EXTENSION_1/BRIEF.md: f453bcec39a5535acccabfc52833741da6f7868939b976da6dcd18a4af6c6a1d.
   - CP2_WIRE_ADDENDUM_2.md: ec66628ef8db1ac70500b80abbcc754b82133fb8d97d4cf0f41ecd4a32ab9133.
   - CP2_WIRE_ADDENDUM_1.md: c389f5e3878c6a32d8c72b80d1374ea6f30c4350c2870ab7ac653972df3dc760.
   - CP2_WIRE.md: 81a7adba…6996.
   - reference_cases.json: 4d7b777708806c211cda0e126d7cd4eb7acba9b658b1da63cca36fd0523beeab.
   - eigen_motion.request.json: 58ce5b95d185d56964a30962b355c14595cdf831505603c6e5d455425c4ea8e9.
     The test also checks this hash at run time.
   - load_reference_state_runtime.rs: d5be0bd8…960b, unchanged.

2. **Authoring checks, before any Cargo run.**
   - rustfmt --edition 2021 tests/load_reference_state_runtime_extension.rs
   - rustfmt --edition 2021 --check (clean)

3. **ext_run_1.log**, after the manager said the tree compiles. Test file 7ff9a5c5…3c83.
   - Command: CARGO_TARGET_DIR=... cargo +1.97.1 test --locked --offline -j 1 --test load_reference_state_runtime_extension -- --nocapture
   - Result: 8 passed, 1 failed (dilation_datum_zero_is_consulted_not_consumed). Exit 101.

4. **Edit, with no change to any failing expectation.**
   - The dilation test prints each variant's law evidence before asserting.
   - The secant test prints its law evidence.
   - Consulted-segment labels only need to be one of the two labels ADDENDUM_1 defines. The consumed-segment assertion is unchanged.
   - The file was rustfmt-clean afterwards.

5. **ext_run_2.log**, same command. Test file 6fc7b27a26ccb91908c74c640d9c46ae2de4f62c229c1c8d61d3c7872ac853d9 (final).
   - Result: 8 passed, 1 failed (same test, same original failure text). Exit 101.

6. **cp2_baseline_run.log**, the checkpoint-2 binary once.
   - Command: cargo +1.97.1 test --locked --offline -j 1 --test load_reference_state_runtime -- --nocapture
   - Test file d5be0bd8…960b. Result: 20 passed. Exit 0.

src_tree_sha256 (HEAD + sorted sha256sum of src/**/*.rs) was
8d197045b3a950f056c5ea539b821c8809193040e6783269878c8f477a94b221 before and after
every run. Per-file hashes are in ext_run_1.log and ext_run_2.log.

No other Cargo invocation, no Git, and no browser, native, UI or npm run.
The target directory is deleted at the end, as the brief directs.
# 2026-09-25T19:26:35Z deleted /home/user/cargo-targets/loadstate/product_physics_tests (brief: delete target when done)

## Final step (ROOT resume after the CP3 wire adoption)

7. **Input hash check.** CP3_WIRE_ADDENDUM.md sha256 f69043b682d027cc7f15e2173b932f016cd76f3a64d76b76d974cd0f0105f7d4 (verified).

8. **Test 7 revision.** Only the consumed/consulted segment label assertions changed.
   - Test file before: 6fc7b27a26ccb91908c74c640d9c46ae2de4f62c229c1c8d61d3c7872ac853d9.
   - Test file after: 5febacba957bc9aed01c8010ea3496616d05021290c2b15d0e08cbabf4558a58.
   - Unified diff: test7_revision.diff (sha256 4ff8f12cb3ed4a7c0867cf1036c53ded5ed7358661700769a45a6973bdfa2eac).
   - rustfmt --check: clean.

9. **Rebuild** in /home/user/cargo-targets/loadstate/product_physics_tests (fresh; it had been deleted). Each binary ran once:
   - final_load_reference_state_runtime_extension.log (sha256 e8db5b20…661f): 9 passed, exit 0.
   - final_load_reference_state_runtime.log (sha256 06d7d35e…b518): 20 passed, exit 0.

   Both used `cargo +1.97.1 test --locked --offline -j 1 --test <bin> -- --nocapture`.
   src_tree_sha256 was 8d197045…b221, identical before and after both runs.

10. **Target deleted** again after the runs.
