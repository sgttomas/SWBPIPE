# Exact reproduction
From REPO_ROOT, set S=projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/M1/M1-N.

`CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$PWD/$S/harness/target" cargo run --manifest-path "$S/harness/Cargo.toml" --bin m1_n_audit`

`CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$PWD/$S/harness/target" cargo run --manifest-path "$S/harness/Cargo.toml" --bin robustness`

The original main run before adding a second binary omitted --bin, otherwise command identical. Supplement command explicitly used --bin robustness. Exit0 both; caught panic is expected audit evidence, not test failure hidden. Main74cases,37configurations, raw every-state traces in witness_stdout.log.json. Exact raw stdout/stderr UTF8 text and decodedSHA256 are preserved in JSON; no log bytes normalized. No source edits or fullsuite reruns. Initial compile1.01s; supplement timing retained in stderr. Expected files were hashed BEFORE each execution. Treat result numbers as observations unless independent criteria justify comparison; TBD convergence status deliberately emits a warning.
