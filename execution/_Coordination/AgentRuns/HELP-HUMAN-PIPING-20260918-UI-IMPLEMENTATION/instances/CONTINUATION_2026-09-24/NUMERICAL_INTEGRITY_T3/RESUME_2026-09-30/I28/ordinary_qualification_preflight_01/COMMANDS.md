# Prospective commands — none executed in I28

ROOT supplies values for the portable variables below in the granted run record.
`RUST_TOOLCHAIN_ROOT` is the already installed1.97.1-aarch64-apple-darwin directory;
`REPO_ROOT` is K6C; `QUAL_ROOT` is a new owned scratch directory; `R` is the
absolute path to the existing RESUME evidence root in K6C. The installed paths observed now are
in I28/_run_records. Never reuse a target or overwrite a prior qualification.

ROOT first confirms final freeze and existing memguard liveness/host slot. This
example uses the maintained freeze supplied to I28; substitute a later commit only
after explicitly rebinding review and source evidence. All following commands are
a proposal and require that release.

```sh
FINAL_SHA=81c03849033f3ce745668f581f446530789397b8
P=projects/chirality-piping
H=$P/core/solver/performance_harness
VR=$P/validation/benchmarks/numerical_robustness

mkdir "$QUAL_ROOT"
mkdir "$QUAL_ROOT/source" "$QUAL_ROOT/logs"
GIT_OPTIONAL_LOCKS=0 git -C "$REPO_ROOT" archive --format=tar \
  --output="$QUAL_ROOT/source.tar" "$FINAL_SHA" \
  "$P/core/loads/primitive_loads" \
  "$P/core/solver/curved_bend" "$P/core/solver/diagnostics" \
  "$P/core/solver/frame_kernel" "$P/core/solver/linear_supports" \
  "$P/core/solver/nonlinear_integration" "$P/core/solver/nonlinear_supports" \
  "$H" "$P/core/solver/sparse_direct" "$VR"
tar -xf "$QUAL_ROOT/source.tar" -C "$QUAL_ROOT/source"
shasum -a 256 "$QUAL_ROOT/source.tar"
chmod -R a-w "$QUAL_ROOT/source"
cd "$QUAL_ROOT/source"
```

Bind the archive inventory/commit tree, compiler binaries, installed sysroot/library
input hashes and inspected ancestor/Cargo-home configuration before compiling.
Keep this archive at its recorded absolute path: VR's compiled CARGO_MANIFEST_DIR
is an input/launch premise. The builds use normal crate defaults and no
`--features`, `--all-features`, `--no-default-features`, `--test` or injected
diagnostic cfg. Clear the same mutation/flag/wrapper overrides as the accepted
commands, and use absolute installed cargo and rustc. Any additional effective
Cargo or linker override must be recorded and reconciled before reliance.

Sequential H and VR builds; stop on the first failure, with no install/network or
tool replacement. Targets are initially absent and external to the source archive.
Shell pipeline failure propagation is required.

```sh
set -e
set -o pipefail
env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
  -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER -u RUSTC_BOOTSTRAP \
  RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 \
  RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true \
  RUSTC="$RUST_TOOLCHAIN_ROOT/bin/rustc" \
  CARGO_TARGET_DIR="$QUAL_ROOT/target-h" \
  "$RUST_TOOLCHAIN_ROOT/bin/cargo" build --release --offline --locked -j 4 \
  --verbose --target aarch64-apple-darwin \
  --manifest-path "$QUAL_ROOT/source/$H/Cargo.toml" --bin k6_observe \
  2>&1 | tee "$QUAL_ROOT/logs/build-h.log"

env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
  -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER -u RUSTC_BOOTSTRAP \
  RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 \
  RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true \
  RUSTC="$RUST_TOOLCHAIN_ROOT/bin/rustc" \
  CARGO_TARGET_DIR="$QUAL_ROOT/target-vr" \
  "$RUST_TOOLCHAIN_ROOT/bin/cargo" build --release --offline --locked -j 4 \
  --verbose --target aarch64-apple-darwin \
  --manifest-path "$QUAL_ROOT/source/$VR/Cargo.toml" --example vk_scale \
  2>&1 | tee "$QUAL_ROOT/logs/build-vr.log"
```

Do not infer actual build success, features or flags from these proposed commands.
Record real exit/status/time and every effective rustc argv/fingerprint. Rehash source,
compiler/sysroot and library inputs after the build; preserve the full dependency
and archive relation. Current-hash equality must never be backdated into a claim
about old link inputs. Ordinary outputs are:

```sh
H_BIN="$QUAL_ROOT/target-h/aarch64-apple-darwin/release/k6_observe"
VR_BIN="$QUAL_ROOT/target-vr/aarch64-apple-darwin/release/examples/vk_scale"
shasum -a 256 "$H_BIN" "$VR_BIN"
/usr/bin/nm -n "$H_BIN" > "$QUAL_ROOT/logs/h-symbols.txt"
/usr/bin/nm -n "$VR_BIN" > "$QUAL_ROOT/logs/vr-symbols.txt"
/usr/bin/otool -L "$H_BIN" > "$QUAL_ROOT/logs/h-dylibs.txt"
/usr/bin/otool -L "$VR_BIN" > "$QUAL_ROOT/logs/vr-dylibs.txt"
```

From those two symbol tables identify only the SOURCE_BINDING.md target
specializations and named runtime/error constructors. Addresses are *new outputs*,
not the old layout08/vk_records addresses. For each identified exact function
range use the already successful native reader form:

```sh
/usr/bin/objdump --disassemble --demangle \
  --start-address="$START" --stop-address="$STOP" "$BOUND_BINARY" \
  > "$QUAL_ROOT/logs/$FACT_ID.stdout" 2> "$QUAL_ROOT/logs/$FACT_ID.stderr"
```

The selected bounded set is three kernel container specializations (leaf and internal
requests where directly emitted), two runtime requests, eight VR node specializations
and three error boxes plus the already selected forwarding/caller ranges necessary
for attribution. Follow existing I23/RV30 command records for symbol/type/argument
interpretation, not stale address substitution. For each result record binary hash,
symbol/type, interval, allocation call entry, argument setup/data flow, expected and
observed request, caller/source identity, raw stream hashes and verdict. Return any
missing exact specialization/constructor, inaccessible final range or incompatible
reader as a finite gap; do not dump arbitrary code, repeat the failed rlib reader,
invent a private mirror, install LLVM or change compilation to expose symbols.

The unchanged public_layout20 reporter is the finite ten-type diagnostic. After
recording the exact VR/FK rlib paths from the successful ordinary VR build, the
prospective command reuses its accepted source byte-for-byte:

```sh
env -u FK_SEEDED_FAULT -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS \
  -u RUSTC_WRAPPER -u RUSTC_WORKSPACE_WRAPPER -u RUSTC_BOOTSTRAP \
  RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 \
  "$RUST_TOOLCHAIN_ROOT/bin/rustc" --edition=2021 \
  --crate-name t3_public_layout_20 --target aarch64-apple-darwin -C opt-level=3 \
  --extern "piping_numerical_robustness=$FINAL_VR_RLIB" \
  --extern "open_pipe_stress_frame_kernel=$FINAL_FK_RLIB" \
  -L "dependency=$QUAL_ROOT/target-vr/aarch64-apple-darwin/release/deps" \
  "$R/verification/public_layout_20/_run_records/public_layout.rs" \
  -o "$QUAL_ROOT/public_layout_20" \
  > "$QUAL_ROOT/logs/public-layout-compile.stdout" \
  2> "$QUAL_ROOT/logs/public-layout-compile.stderr"
"$QUAL_ROOT/public_layout_20" > "$QUAL_ROOT/logs/public-layout.stdout" \
  2> "$QUAL_ROOT/logs/public-layout.stderr"
```

Hash source, compiler, rlibs and produced reporter; require the ten exact type/label/
size/align rows and successful exits. This reporter has no model/solver/private
mirror and does not qualify private allocations. No final rlib basename or binary
address is guessed before compilation.

Existing layout04/08 source overlays, complete type maps and accepted invocation
forms are referenced in SOURCE_BINDING.md. If final public/nominal correspondence
needs those reporters, ROOT may commission their **unchanged source** reuse in
separate diagnostic archives/targets after source-overlay review. That is a finite
fallback obligation, not an automatic extra build in this minimal step. A changed
overlay or unavailable private fact is returned instead of engineered around.

No model, counts, solve, runner tier, timing or measurement command is granted by
this record. Future normal argv/input manifests must be frozen and checked using
the existing H/VR runner binding machinery before their separate authorized runs;
see PLAN.md for fields. A counts-only number or runner metadata label never replaces
the final source/build/request evidence above.
