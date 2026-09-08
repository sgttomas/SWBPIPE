# F4 focused prerequisite boundary run V1

Record class: structural execution evidence; host-specific paths retained intentionally for exact provenance; not a reusable control surface
Source commit: `779dedb8670625b36af07b89fc5557470e47c50e`
Working directory: `/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-execution-20260908`
Executed target: `/tmp/f4-boundary-manager-8728`
Isolation: manager-only `CARGO_TARGET_DIR`; not shared with F4-D1 or another manager
Cleanup: target removed after all processes exited
Aggregate result: six exact filtered Cargo invocations exited `0`; six tests passed, zero failed

The exact shell invocation body was:

```sh
set -euo pipefail
F4_BOUNDARY_TARGET=/tmp/f4-boundary-manager-8728
python3 -c 'import shutil; shutil.rmtree("/tmp/f4-boundary-manager-8728", ignore_errors=True)'
export CARGO_TARGET_DIR="$F4_BOUNDARY_TARGET"
cargo test --quiet --manifest-path projects/chirality-piping/core/solver/linear_supports/Cargo.toml frame_dof_reexport_matches_frame_kernel_boundary
cargo test --quiet --manifest-path projects/chirality-piping/core/solver/diagnostics/Cargo.toml nonconvergence_after_iteration_limit_is_failure
cargo test --quiet --manifest-path projects/chirality-piping/core/units/Cargo.toml conversion_can_be_bound_to_semantic_dimensions_with_shared_units
cargo test --quiet --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml sliding_friction_support_applies_bounded_force_independent_of_seed
cargo test --quiet --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml iteration_cap_returns_nonconvergence_failure_diagnostic
cargo test --quiet --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml tbd_policy_emits_visible_tolerance_policy_diagnostic_without_defaults
```

Observed result blocks, in command order:

```text
1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out
1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out
1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out
1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out
1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out
1 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out
```

This record proves only current-head consumed-interface behavior. It does not mutate or close E006-E009.
