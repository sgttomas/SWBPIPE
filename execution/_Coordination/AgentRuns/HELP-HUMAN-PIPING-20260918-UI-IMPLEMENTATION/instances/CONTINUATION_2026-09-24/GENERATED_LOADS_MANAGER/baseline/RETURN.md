# Baseline regression candidate

Authored `core/loads/self_weight_wasm/tests/applied_self_weight.rs` and its sole new dev dependency on `operation_applier`. All paths are relative to `projects/chirality-piping` unless qualified. Actual parentage, supplied source hashes, output hashes and independent arithmetic: `_run_records/AUTHORING.json`.

Three connected tests use real generation (direct and JSON transport parity), real structured atomic apply, a manually authored same-case tip force, real structured density or span edits, JSON serialization/reopen, and real product solve. Original inputs are checked unchanged. The third test separately protects the existing pre-apply source hash rejection for both generation and batch apply.

Fixture is invented: straight 2 m cantilever, OD 0.1 m, wall 0.01 m, density 1000 kg/m³; user gravity -7 m/s² global Y; manual tip force -11 N. Independent annular area times density times acceleration supplies the oracle, with total force and first moment from equilibrium. No production mass helper is used as oracle. Density doubling must produce fresh force and moment; the baseline intentionally asserts the correct answer and should fail. Span-only elongation to 3 m should pass with unchanged N/m and correctly increased force/moment.

No Cargo, build, npm, browser, native or test execution was performed by this child. Parent should run from repository root with its designated isolated target and at most two jobs:

```sh
CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/private/tmp/piping-generated-loads-baseline-target cargo test --offline --manifest-path projects/chirality-piping/core/loads/self_weight_wasm/Cargo.toml --test applied_self_weight -- --nocapture
```

Retain raw stdout/stderr and exit status before repair. Expected baseline: two passes and density fresh-force failure (actual about 50.584067435 N, required about 90.168134870 N). Compile or fixture failures are not yet ruled out. If repair chooses explicit stale blocking with explicit refresh, retain this original failing artifact and revise maintained tests to that precise contract; never make the stale numeric answer a passing oracle. Native save/reopen UI and full operation-history undo/redo are outside this bounded Rust baseline; JSON transport round-trip is the witness here.
