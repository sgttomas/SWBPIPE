# I22 checkpoint B — stopped at FK compilation

**Compile failure; no tests ran. Repairs used: 0 of 2.**
H k6b_export was not launched. No retry, source edit, counter dump, broader
suite or numerical job occurred. No owned build remains running.

Candidate HEAD: 3cf296e36645d97e4c657c8ad1a6322bc4163f16, initially clean.
All four numerical source hashes matched implementation_a before and after.
Grant: COORD bb6f9b75ea859560a45f2ff049a02e19098c1c2f,
SHA256 645f86c3f96723667cc131340350f5dbed0233a04b4c0014d127ce8c2fa0c002.
Same I22 TASK/parent; no delegation or Git/index mutation.

## Exact compiler errors and dependency

1. New publication_tests.rs:9: Wide::from_f64 is ambiguous between the generic
   and Wide<2> implementations. Proposed in-scope fix: Wide::<4>::from_f64.
2. New publication_tests.rs:82: Wide::ZERO is likewise ambiguous.
   Proposed in-scope fix: Wide::<4>::ZERO.
3. FK/tests/retained_k4/method_tests.rs:101: exhaustive AttemptReason match
   lacks PublicationEnclosure. **This existing protected test is outside the
   four-file write grant.** It compiles as part of FK lib tests even with a
   runtime name filter, so the focused test cannot proceed without disposition.

Minimal proposed fifth-path compiler adaptation in tokens():
```rust
AttemptReason::PublicationEnclosure { quantity, predicate, .. } =>
    format!("publication_enclosure:{}:{predicate:?}", index(quantity)),
```
It reports the new reason without accepting it, changing expected tokens or
weakening a criterion. No wildcard hides new outcomes. Nothing has been edited.
ROOT must explicitly grant/disposition that mechanical path before repair.
The manager directed preservation of this stop and zero repair count.

## Job and environment

Launched 2026-10-01T06:45:35.691Z, tool-managed PTY session 76996.
Completed exit 101. Exact sanitized argv:
```sh
/usr/bin/time -l env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<WT>/a1-target "$HOME/.cargo/bin/cargo" test --offline --locked -j 4 --manifest-path <A1_WT>/projects/chirality-piping/core/solver/frame_kernel/Cargo.toml --lib publication_tests
```
Stdout/stderr were separately redirected to owned scratch/implementation_b.
Owner HOME/CARGO_HOME/RUSTUP_HOME preserved; no relevant Cargo config files
found along A1 cwd ancestors or CARGO_HOME. Guard PID5387 was running before
and after. No new guard, installation or host-tool change.

Time reports 4.29 s real, 3.73 user, 0.23 system; 690,716,672 B maximum RSS,
7,127,664 B peak footprint, zero swaps. These are command-reported resources,
not a process-group bound, per-process cap or automatic deadline. Checkpoint
stopped well before its 30-minute limit.

FK_01.stdout is the unchanged empty stream.
FK_01.stderr.portable retains the complete diagnostics/time output with only
machine roots replaced; original raw stderr remains in owned scratch, SHA256
4037fcf9d50be71874b8e682a2c3616aea3d3bc7446bacb9a80db2d1bc7071a8.
COMMANDS.json holds all commands, tool completion and process/guard observations.
TARGET_SCRATCH_SNAPSHOT_SHA256SUMS is an end-state inventory, not a claimed
pre/post write delta for any pre-existing target files.

## Released controls and remaining work

Read the independent control packet and verified supplied manifest/truth hashes:
c6cc8d878eec3fd9efcfdc8c5839daeb278dd91d1e9ed53f8bf84435495c8673 /
aac399bfb68bbb8b1cbcce08cc2901229535e88860dc5cc812cf08c9fe3ba0ee.
Exact case IDs are EXTRA-FM-01, EXTRA-MF-01 and EXTRA-ZR-01. Their required load
source IDs are EXTRA-FM-01:load, EXTRA-MF-01:load, EXTRA-ZR-01:load.
Current unrun builders use F-M:tip-force, M-F:tip-moment, ZERO-ROT:tip-force.
List those identity changes for later grant; they are not compiler fixes and
were not applied. No truth changed, no source-control comparison ran.
The first filename guess RETURN.md was absent; CHECKPOINT_0.md was then read.

RV29's newly routed frozen ledger was not read or observed through helper counters
here. Full accounting closure is not established. After ROOT's scope ruling,
bounded compiler repair and focused FK/H tests remain first; all numerical,
independent accounting, oracle/mutant, protected availability and merge gates
remain open. A1 is still BLOCKING.

Writes: additive implementation_b evidence, two raw scratch streams and ordinary
Cargo artifacts in the specifically granted a1-target. No maintained source was
changed. Previous evidence seals remain preserved.

