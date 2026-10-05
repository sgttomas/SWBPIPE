#!/bin/bash
# RV89: build.rs compiled standalone (rustc, scratch output) and run under crafted environments.
T=WT
S=$T/scratch/rv89_u4_g5; B=$S/bs
PP=$T/rv89/cand/projects/chirality-piping/core/product_physics
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
RUSTC_BIN=$(rustup which rustc)
TMPDIR=$S/tmp rustc --edition 2021 $PP/build.rs -o $B/build_script 2>&1 | tail -3
base_env=(PATH=/usr/bin:/bin RUSTC=$RUSTC_BIN TARGET=aarch64-apple-darwin CARGO_CFG_TARGET_ARCH=aarch64 CARGO_CFG_TARGET_POINTER_WIDTH=64
  CARGO_CFG_TARGET_ENDIAN=little CARGO_CFG_TARGET_OS=macos CARGO_CFG_TARGET_ENV= CARGO_CFG_PANIC=unwind PROFILE=debug OPT_LEVEL=0
  CARGO_CFG_DEBUG_ASSERTIONS= CARGO_ENCODED_RUSTFLAGS= CARGO_PKG_NAME=open_pipe_stress_product_physics CARGO_PKG_VERSION=0.2.0 CARGO_MANIFEST_DIR=$PP)
HOSTILE=$'-C\x1fopt-level=3;x\ny%'
NONUTF=$'\xff'
run() { local name=$1; shift; echo "### $name"; ( cd $B && env -i "$@" $B/build_script ); echo "### $name exit=$?"; }
run A_normal "${base_env[@]}"
run B_no_rustc "${base_env[@]/RUSTC=*/RUSTC=$B/does_not_exist}"
run C_false_rustc "${base_env[@]/RUSTC=*/RUSTC=/usr/bin/false}"
run D_duplicate_field "${base_env[@]/RUSTC=*/RUSTC=$B/dup_rustc.sh}"
run D2_empty_field "${base_env[@]/RUSTC=*/RUSTC=$B/empty_rustc.sh}"
run D3_odd_values "${base_env[@]/RUSTC=*/RUSTC=$B/odd_rustc.sh}"
run E_env_unset $(printf '%s\n' "${base_env[@]}" | grep -v '^CARGO_CFG_TARGET_ENV=')
run F_hostile_rustflags "${base_env[@]/CARGO_ENCODED_RUSTFLAGS=*/CARGO_ENCODED_RUSTFLAGS=$HOSTILE}"
run G_non_utf8 "${base_env[@]/CARGO_CFG_TARGET_OS=*/CARGO_CFG_TARGET_OS=$NONUTF}"
run H_no_manifest_dir $(printf '%s\n' "${base_env[@]}" | grep -v '^CARGO_MANIFEST_DIR=')
run I_manifest_elsewhere "${base_env[@]/CARGO_MANIFEST_DIR=*/CARGO_MANIFEST_DIR=$B}"
run K_no_debug_assertions $(printf '%s\n' "${base_env[@]}" | grep -v '^CARGO_CFG_DEBUG_ASSERTIONS')
echo "### J_closed_stdout"; ( cd $B && env -i "${base_env[@]}" $B/build_script >&- 2>/dev/null ); echo "### J_closed_stdout exit=$?"
