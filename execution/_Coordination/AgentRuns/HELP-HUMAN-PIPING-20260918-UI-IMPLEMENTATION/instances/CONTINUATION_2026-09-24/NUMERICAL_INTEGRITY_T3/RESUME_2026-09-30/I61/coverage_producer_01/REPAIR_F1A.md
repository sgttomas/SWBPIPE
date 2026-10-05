# I61 addendum: f1a_tests repair

**Status: DONE, uncommitted in WT/f2a-coverage for ROOT.** All six `f1a_tests` failures are fixed.
product_physics `--lib` now has a single failure, `s11g_tests::t13_committed_fallback_uz_is_byte_identical`,
which also fails on main on this Mac.

This was a small follow-on grant from ROOT, done by the same TASK. Started 2026-10-03T20:15:03Z,
ended 20:16:07Z. The memory guard (PID 5387) was running. There were no Git writes or index
operations, Git reads used `GIT_OPTIONAL_LOCKS=0`, and only one cargo job ran at a time.

## Change

`P/core/product_physics/tests/formation_check_runtime.rs` was restored to `origin/main`
(`381be775ae`) bytes with `git show origin/main:<path> > <path>`. This was a working-tree write
only. No other file was touched, including the fixture and its users.

| | sha256 | lines |
|---|---|---|
| Before (WT HEAD `c618675e84`, carrying `8104a4fedd`'s `include_str!` line) | 2e14cce9379c92e6ceec9887d181f7a21fb02babf80b67b080ed947b76c3a2f9 | 400 |
| After (= main) | faaf940d8973349501b468505c8e1e9bbdfe3e3adf4df84fe179ed791cc0a0a5 | 400 |

Three checks confirm the restore:
- `git diff origin/main -- <path>` prints nothing (0 bytes);
- `git diff --numstat` against HEAD shows `1 1` on this file only;
- the restored inline `r#"…"#` literal is byte-identical to `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json` (checked with `cmp`).

The fixture stays in place for its other users:
- PP `src/retained_memory.rs`;
- PP `src/retained_product_tests.rs`;
- PP `tests/retained_precision_admission.rs`;
- `P/core/runner/headless/tests/retained_precision_admission.rs`.

## Commands and results

All runs used product_physics with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`,
an explicit `--manifest-path` and the existing target dir `WT/targets/i61-coverage/product_physics`.
Each had a 1,200 s wall.

| Run | Result |
|---|---|
| `--lib` | 455 passed, 1 failed, 1 ignored. All seven `f1a_tests` pass. The only failure is `s11g_tests::t13_committed_fallback_uz_is_byte_identical` ("committed bytes changed"), the known Mac platform failure. |
| `--test retained_precision_admission` | 5 passed |
| `--test formation_check_runtime` (its own test target) | 5 passed |

I did not run the headless runner's `retained_precision_admission` test, which also uses the
fixture. It is outside the brief and lives in another crate.

## Logs (WT/scratch/i61_coverage_producer_01/)

| sha256 | bytes | path |
|---|---|---|
| b54c7806349676718c87d1c7c98f3816443a5b5d398c5acd4cbc36b8bf7ce605 | 47394 | 20_f1a_pp_lib.log |
| 723b425bc4052c317d10fbc45c6d02390fdbfe777515af569f91bbdd3de3c24a | 4916 | 21_f1a_retained_precision_admission.log |
| bc8a3e6e548f88eaf3de345da8d48d70174bf39d73b0b102822a5652f62a698e | 4896 | 22_f1a_formation_check_runtime.log |
| 2e14cce9379c92e6ceec9887d181f7a21fb02babf80b67b080ed947b76c3a2f9 | 41899 | formation_check_runtime.before.rs (the pre-repair bytes) |
