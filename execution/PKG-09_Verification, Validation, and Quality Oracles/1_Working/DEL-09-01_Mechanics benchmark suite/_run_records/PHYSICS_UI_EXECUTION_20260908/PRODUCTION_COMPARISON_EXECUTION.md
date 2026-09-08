# Production comparison execution record

Status: COMPLETE WITH DISCLOSED SCHEDULING DEVIATION.

## Source, command and target

Production source was read from the current working tree rooted at `/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-execution-20260908`. The bounded harness depends on `projects/chirality-piping/core/product_physics` through `production_comparison_harness/Cargo.toml` and calls its public API. The exact authored input was `projects/chirality-piping/fixtures/product_preview/invented_preview_model.json` at SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`.

The final production invocation was:

```text
RUNREC='projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/_run_records/PHYSICS_UI_EXECUTION_20260908'
FIXTURE='projects/chirality-piping/fixtures/product_preview/invented_preview_model.json'
CARGO_TARGET_DIR=/tmp/m9-cargo-target cargo run --quiet --manifest-path "$RUNREC/production_comparison_harness/Cargo.toml" -- "$FIXTURE" "$RUNREC/production_outputs"
```

The build target was `/tmp/m9-cargo-target`, outside the repository. Cargo compiled the local dependency graph into that private target. The executable wrote only the eight JSON envelopes under this run record's `production_outputs/` directory. It made no production source, fixture, status, threshold or DAG write.

The post-freeze mapping invocation was:

```text
/Users/ryan/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3 "$RUNREC/compare_frozen_to_production.py" --expected "$RUNREC/EXPECTED_RESULTS_FROZEN.json" --production-dir "$RUNREC/production_outputs" --json-output "$RUNREC/COMPARISON.json" --csv-output "$RUNREC/COMPARISON.csv"
```

## Timing and scheduling deviation

Filesystem birth/modify times provide the durable wall-clock evidence, in America/Edmonton (UTC-06):

| Event | Time | Evidence |
|---|---|---|
| frozen expected-result bytes finalized | 2026-09-07T23:11:36-0600 | `EXPECTED_RESULTS_FROZEN.json` modification time and `FREEZE_MANIFEST.json` |
| first native Rust comparison output | 2026-09-07T23:16:56-0600 | birth time of `physical_frame.dense_scrutiny.json` |
| frictionless native comparison output first added | 2026-09-07T23:24:32-0600 | birth time of `physical_frame_frictionless.dense_scrutiny.json` |
| final isolated-L-100 comparison output | 2026-09-07T23:29:13-0600 | final modification time of all eight output envelopes |

M9 learned after these invocations that F4 held the exclusive native/Rust slot until its recent release. The first compile and the later cached/incremental Rust invocations therefore occurred before M9 received the slot-release notice. This is a scheduling deviation and is not hidden or reclassified as a non-native comparison. The mechanism was an isolated private Cargo target plus read-only imports of production crates, so there was no shared build-directory or production-source write race; that containment does not erase the coordination breach. No further broad/native run was performed after the notice.

The frozen independent reference was already complete before the first Rust invocation, so this scheduling deviation does not affect oracle independence or freeze ordering. It does affect run-governance conformance and is retained for manager disposition.
