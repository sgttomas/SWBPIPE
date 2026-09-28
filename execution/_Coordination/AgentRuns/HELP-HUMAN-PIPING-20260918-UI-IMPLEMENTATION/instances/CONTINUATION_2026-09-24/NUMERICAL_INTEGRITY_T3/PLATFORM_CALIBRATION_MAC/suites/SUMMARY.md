# Suites on Mac main (the baseline for Mac-run T3 slices)

## Method

- CI's numerical cargo profile: every manifest `tools/release/check_release_readiness.py` discovers, 39 in all.
- Each runs `cargo test --offline --locked`.
- The tree is a `git archive` of main `649162522`, without `execution/`, on the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1).
- Memory caps: `CARGO_BUILD_JOBS=8`, `RUST_TEST_THREADS=4` (`run_suites.sh.txt`).
- The two crates with a failure were re-run with `--no-fail-fast`, so that no test target after a failing one goes unrun. Cargo stops at the first failing target otherwise, as CI's command does.

## Result

- **37 of 39 manifests pass** (`per_manifest_first_pass.txt`).
- **Three tests fail on Mac main,** all in the no-fail-fast re-runs:

| Crate | Test | Result on the re-run | What it compares |
|---|---|---|---|
| product_physics | `s11g_tests::t13_committed_fallback_uz_is_byte_identical` | 522 passed, 1 failed, 17 targets | Committed raw bytes of `load_reference_fallback_uz`, sparse ("committed bytes changed") |
| headless | `load_reference_route_tests::load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes` | 82 passed, 2 failed, 8 targets | Committed raw bytes of `load_reference/connected`, sparse ("actual producer changed") |
| headless | `cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route` (tests/load_reference_cli.rs) | (same re-run) | The same committed `connected` sparse bytes, through the CLI |

The failure excerpts are in `nofailfast_*.excerpt.txt`.

## Attribution

Each failing test compares against a committed raw output generated on Linux. These are exactly the committed outputs whose Mac bytes differ from Linux in T9 (`../t9/platform_differences.txt`):
- the `fallback_uz` sparse output, by one `support_reaction_moment_magnitude_v2` at 1 ulp;
- the `connected` sparse output, by two `support_reaction_force_magnitude_v2` values at 1 ulp.

Both are macOS `hypot` roundings.
- `fallback_uz`'s dense output is byte-identical.
- `connected`'s dense output also differs, by one force magnitude. The tests assert the sparse mode first and stop there, so their dense halves are not reached.

This is consistent with T9's attribution. It was not re-proved by building the test binaries with the libm shim.

## Use

- **On the Mac, these three tests are platform failures,** not defects. A Mac-run slice must show the same three failing identically on its base, and no other failure.
- **CI on Linux stays the authority** for the suites.
