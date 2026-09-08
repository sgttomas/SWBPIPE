# RF successor backcheck validation

All commands ran from `{REPO_ROOT}`. `{RF_PRIVATE_CARGO_TARGET}` was an exclusive task-private directory outside the repository.

| Check | Exact portable command | Result |
| --- | --- | --- |
| Repaired exact-zero branch | `CARGO_TARGET_DIR={RF_PRIVATE_CARGO_TARGET} cargo test --manifest-path {REPO_ROOT}/projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml --locked --offline tests::both_nonzero_assumed_branches_accept_exact_zero_current_normal -- --exact` | exit 0; 1 passed |
| Repaired signed-zero rejection | `CARGO_TARGET_DIR={RF_PRIVATE_CARGO_TARGET} cargo test --manifest-path {REPO_ROOT}/projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml --locked --offline tests::active_signed_zero_branch_rejects_cross_coupled_nonzero_current_normal -- --exact` | exit 0; 1 passed |
| Unchanged rational witness | `CARGO_TARGET_DIR={RF_PRIVATE_CARGO_TARGET} cargo test --manifest-path {REPO_ROOT}/projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml --locked --offline tests::current_normal_affine_fixture_matches_both_signed_oracles_seeds_and_modes -- --exact` | exit 0; 1 passed |
| Full crate | `CARGO_TARGET_DIR={RF_PRIVATE_CARGO_TARGET} cargo test --manifest-path {REPO_ROOT}/projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml --locked --offline` | exit 0; 29 passed, 0 failed; doc tests 0 |
| Formatting | `cargo fmt --manifest-path {REPO_ROOT}/projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml -- --check` | exit 0 |
| Private cleanup | `cargo clean --manifest-path {REPO_ROOT}/projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml --target-dir {RF_PRIVATE_CARGO_TARGET}` followed, only if needed, by `rmdir {RF_PRIVATE_CARGO_TARGET}` and `test ! -e {RF_PRIVATE_CARGO_TARGET}` | exit 0; absent |

`_run_records/COMMANDS_EXACT_STRUCTURAL.json` preserves the resolved workdir, target, environment, and argv arrays. Per-command stdout, stderr, and exit records are lossless. `_run_records/IDENTITY_PROOF.json` records the in-memory two-patch reconstruction and immutable-region comparison. `_run_records/BINDING_VERIFICATION.json` records all sealed bindings and the 13/13 manifest check.

An initial command-runner wrapper exited after its first subprocess because it assigned zsh's read-only `status` variable. `_run_records/ORCHESTRATION_FAILURE_01.txt` preserves the exact error and diagnosis. No substantive test failure occurred. Its guarded task-private directory was cleaned with Cargo plus `rmdir`, and absence was confirmed before the corrected Bash runner performed the successful validation above.
