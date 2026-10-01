# layout_04 VR prebuild checkpoint

Status: PREPARED; NOT COMPILED. Manager fence verification and explicit slot release are mandatory before the command below.

Start 2026-10-01 08:12:46 UTC; common deadline 08:52:46 UTC.
Frozen source cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00.
Archive SHA256 b5ad18adb9577e894b962bb3609f1cd4d960a98a34df92cb09ffcd5da190156e.
Archive root <I21_LAYOUT_SCRATCH>/layout_04_cb13fcf_20261001/archive.

Exact overlay fence (archive only):
- P/core/solver/frame_kernel/src/structural/retained/adaptive.rs: appended actual-private-type report function.
- P/core/solver/frame_kernel/src/structural/retained/mod.rs: appended report dispatcher and actual crate-visible type reports.
- P/core/solver/frame_kernel/src/structural.rs: one export inside retained_api.
- P/validation/benchmarks/numerical_robustness/examples/i21_layout_04.rs: new trivial diagnostic caller.

OVERLAY.diff is the complete four-file patch. SHA256 71e685730f745cbfe816850d76d67cbcd39ddd19e25d5ac451eefa32281c9157.
LABEL_TYPE_MAP.json has 151 justified labels with actual expression, source/line/hash and allocation/component purpose. SHA256 71a969143465dc04bc27ab56a8140847e860c7c20347851702997be5acb69395.
OVERLAY_FILES.json records original/overlaid hashes. SHA256 214a41f4a9c43d96e8de91f4a78f45bea89b8df79bcd842636bbbe0e133e0876.

No copied structs, datatype edits, numerical-flow changes, manifest/lock edits, feature/cfg changes, model constructors or solver calls. Field tuple reports are the actual built-in tuple types declared by source, not reports for nominal H aggregate types. Std-private BTree leaf nodes and private I/O wrapper sizes remain gaps.

Proposed release command (one VR invocation; not yet executed):
```text
RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 RUST_TEST_THREADS=2 \\
CARGO_TARGET_DIR=<T3_HOST_ROOT>/k6c-layout-target/vr \\
cargo run --release --offline --locked -j 4 --manifest-path <ARCHIVE>/P/validation/benchmarks/numerical_robustness/Cargo.toml --example i21_layout_04
```

Use normal production cfg/features (no --features, no tests); release matches the observation-binary profile. Before starting, verify existing memguard remains running and record installed compiler/target/config/environment/features without changing them. Output is only type_name/size_of/align_of and metadata, not sampled capacity or heap.

ROOT's clarification at NUM 71bd33e89041bf71062672c688d3b20ad7b739b5 permits a second, separate H example under its original manifest/lock/features and a distinct target subdirectory within the same deadline. It will have its own complete overlay checkpoint and awaits separate release. No new dependency edge will be introduced.

Maximum two local diagnostic compiler corrections total across the two jobs. No installation/fetch if the existing locked dependency set is unavailable. No universal E_max or admission claim follows from these layouts.

