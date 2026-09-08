# RF validation record

All commands used workdir `{REPO_ROOT}` unless an explicit project workdir is stated. Exact executed commands and host paths remain losslessly preserved under `_run_records/`.

- Launch brief SHA-256: expected/actual `158bb86ae243318a3f9ab32431916a6e31bffc741667122d9c2122bcc8c5c9b6`.
- All 14 `FINAL_MANIFEST.json` member hashes: PASS.
- `cmp <(git diff --no-ext-diff 55df51ac3201456e0f181823e3aefefef47a73bb -- projects/chirality-piping/core/solver/nonlinear_integration/src/lib.rs) <frozen CANDIDATE_DIFF.patch>`: exit 0.
- Frozen diff/source/product hashes: `8080a8c...`, `da4cc3f...`, `e757b8a...`; all exact. Base solver/source hash: `fb02a522...`. Product base/live hashes match.
- Scoped change validation for the frozen subject path: PASS, no violations. Unfinished U7 App files were excluded and not interpreted.
- Project-relative affected-check selection: `evidence-sweep`, `harness-self-check`, `piping-pytest`. The full evidence sweep, native/WASM builds, U7 tests, and full repository harness were not run, as prohibited/held by the RF brief.
- `CARGO_TARGET_DIR={RF_PRIVATE_CARGO_TARGET} cargo test --offline --locked --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml`: exit 0; 29 passed, 0 failed; doc tests 0.
- `CARGO_TARGET_DIR={RF_PRIVATE_CARGO_TARGET} cargo fmt --manifest-path projects/chirality-piping/core/solver/nonlinear_integration/Cargo.toml --check`: exit 0.
- Unchanged M1-N-008 harness, offline/locked in the same private target: exit 0. Both seeds and both modes returned `u=0.051851851851851864`, `N=7.407407407407408`, `Rf=-2.222222222222222`.
- Reviewer-owned external public-API witness: final exit 0. It verified simultaneous row physics/order, mixed explicit/derived affine base, prescribed displacement affine offset, singular propagation, and nonfinite rejection. Its diagnostic assertion attempt exited 101 and revealed RF-F4-001; lossless output is preserved in `_run_records/EXTERNAL_WITNESS_ATTEMPT_2_FAILURE.txt`. Final trace is `_run_records/EXTERNAL_WITNESS_FINAL.txt`.
- One discovery command exited 1 because a prior-run brief path was initially resolved under the current run and manifest paths with spaces were unquoted. Its lossless output is preserved in `_run_records/DISCOVERY_COMMAND_FAILURE.txt`; the brief was then found at its authority-stated prior-run path and all manifest paths were quoted.
- Initial manifest-bound `cargo clean` exited 101 because the pre-created `mktemp` target lacked `CACHEDIR.TAG`; preserved in `_run_records/CARGO_CLEAN_FAILURE.txt`. An exact raw-delete attempt was rejected before execution; preserved in `_run_records/RAW_DELETE_REJECTION.txt`. After adding the standard Cargo cache tag, manifest-bound `cargo clean` removed 1,804 files / 128.7 MiB. Both `{RF_PRIVATE_CARGO_TARGET}` and `{RF_PRIVATE_WITNESS_ROOT}` are absent; their exact executed paths remain in the structural run records.
- Final source SHA-256 remained `da4cc3f5d21f1e1841d375dd2525f3bb2d3f0e5d3e960bd2389d32cef3a8b320`. RF wrote no source, test, F4 author, U7, governance, DAG, dependency, or Git state.

Rust slot: `RELEASED_BY_RF`.
