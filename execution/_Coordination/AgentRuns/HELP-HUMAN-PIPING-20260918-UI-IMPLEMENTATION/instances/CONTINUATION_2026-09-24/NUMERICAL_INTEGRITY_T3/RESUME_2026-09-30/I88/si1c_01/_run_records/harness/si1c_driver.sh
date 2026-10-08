#!/bin/bash
# I88 T3-SI1c scratch driver (not repository content): runs I88's cargo jobs in
# sequence, each through the T3 host lock (WT/tools/t3_cargo.sh, --locked
# --offline). Usage: si1c_driver.sh STEP...
#   suites:<tree>   the three crates' committed suites and test lists (no harness files present)
#   dumps:<tree>    the I79, RV104 and SI1c harnesses (copied in first); ibase also writes side records
#   fmt:<tree>      cargo fmt --check on the evaluator crate
# Trees: WT/scratch/i88_si1c/trees/<tree> (base, ibase, cand); targets WT/targets/i88-si1c-<tree>.
set -u
WT=WT
S=$WT/scratch/i88_si1c
P=projects/chirality-piping
export TMPDIR=$S/tmp
LOG=$S/logs/driver.log
mkdir -p "$S/logs" "$S/dumps" "$TMPDIR"

run() { # label crate_dir target cargo-args...
  local label=$1 dir=$2 target=$3; shift 3
  echo "$(date -u '+%FT%TZ') begin $label" >> "$LOG"
  (cd "$dir" && env RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
      CARGO_TARGET_DIR="$WT/targets/$target" "$WT/tools/t3_cargo.sh" "$@" > "$S/logs/$label.log" 2>&1)
  local rc=$?
  echo "$(date -u '+%FT%TZ') end $label rc=$rc" >> "$LOG"
  return $rc
}

harness_in() { # tree
  local root=$S/trees/$1/$P/core/rules
  cp "$S/harness/i79_point_diff.rs" "$root/expression_evaluator/tests/"
  cp "$S/harness/i79_run_diff.rs" "$S/harness/rv104_ee_diff.rs" "$S/harness/rv104_run_diff.rs" \
     "$S/harness/si1c_family.rs" "$root/rule_check_runner/tests/"
}

harness_out() { # tree
  local root=$S/trees/$1/$P/core/rules
  rm -f "$root/expression_evaluator/tests/i79_point_diff.rs" "$root/rule_check_runner/tests/i79_run_diff.rs" \
        "$root/rule_check_runner/tests/rv104_ee_diff.rs" "$root/rule_check_runner/tests/rv104_run_diff.rs" \
        "$root/rule_check_runner/tests/si1c_family.rs"
}

for step in "$@"; do
  kind=${step%%:*}; tree=${step#*:}
  root=$S/trees/$tree/$P/core/rules
  case $kind in
    suites)
      harness_out "$tree"
      for crate in expression_evaluator rule_check_runner rule_pack_document; do
        run "suite_${tree}_${crate}" "$root/$crate" "i88-si1c-$tree" test --locked --offline
        run "list_${tree}_${crate}" "$root/$crate" "i88-si1c-$tree" test --locked --offline -- --list
      done ;;
    dumps)
      harness_in "$tree"
      D=$S/dumps/$tree; mkdir -p "$D"; rm -f "$D"/*.flags
      FL=""; [ "$tree" = ibase ] && FL=1
      ( export I73_DUMP_OUT=$D/i79_dump.tsv I79_EXTREME_OUT=$D/i79_extreme.tsv I79_TABLE_OUT=$D/i79_table.tsv
        [ -n "$FL" ] && export SI1C_FLAGS_OUT=$D/ee_point.flags
        run "dumps_${tree}_ee" "$root/expression_evaluator" "i88-si1c-$tree" \
          test --locked --offline --test i79_point_diff -- --test-threads=1 )
      ( export I73_RUN_DUMP_OUT=$D/i79_run_plain.tsv I79_RUN_EXTREME_OUT=$D/i79_run_extreme.tsv \
               RV104_EE_OUT=$D/rv104_ee.txt RV104_RUN_OUT=$D/rv104_run.txt \
               SI1C_EE_OUT=$D/si1c_ee.txt SI1C_RUN_OUT=$D/si1c_run.txt
        [ -n "$FL" ] && export SI1C_FLAGS_OUT=$D/runner_crate.flags
        run "dumps_${tree}_rcr" "$root/rule_check_runner" "i88-si1c-$tree" \
          test --locked --offline --test i79_run_diff --test rv104_ee_diff --test rv104_run_diff \
          --test si1c_family -- --test-threads=1 )
      harness_out "$tree" ;;
    full)
      D=$S/dumps/$tree; mkdir -p "$D"; rm -f "$D"/runner_full.flags
      cp "$S/harness/rv104_run_full.rs" "$root/rule_check_runner/tests/"
      ( export RV104_RUN_OUT=$D/rv104_run_full.txt
        [ "$tree" = ibase ] && export SI1C_FLAGS_OUT=$D/runner_full.flags
        run "full_${tree}_rcr" "$root/rule_check_runner" "i88-si1c-$tree" \
          test --locked --offline --test rv104_run_full -- --test-threads=1 )
      rm -f "$root/rule_check_runner/tests/rv104_run_full.rs" ;;
    bins)
      # The two binaries P/tests/conftest.py would otherwise build itself (outside the lock).
      run "bins_checked_json" "$S/trees/cand/$P/core/serialization/canonical_json" "i88-si1c-bins" \
        build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson
      run "bins_units" "$S/trees/cand/$P/core/units" "i88-si1c-bins" \
        build --locked --offline --release --features cli --bin openpipestress_units ;;
    pytest)
      echo "$(date -u '+%FT%TZ') begin pytest_$tree" >> "$LOG"
      (cd "$S/trees/$tree/$P" && /usr/bin/lockf -k "$WT/guard/cargo_job.lock" env \
          OPENPIPESTRESS_CHECKED_JSON_BIN="$WT/targets/i88-si1c-bins/release/openpipestress_jcs_ijson" \
          OPENPIPESTRESS_UNITS_BIN="$WT/targets/i88-si1c-bins/release/openpipestress_units" \
          PYTHONDONTWRITEBYTECODE=1 \
          VENV/bin/python \
          -m pytest -q -p no:cacheprovider -rA tests/test_rule_interval.py > "$S/logs/pytest_$tree.log" 2>&1)
      echo "$(date -u '+%FT%TZ') end pytest_$tree rc=$?" >> "$LOG" ;;
    fmt)
      run "fmt_${tree}_ee" "$root/expression_evaluator" "i88-si1c-$tree" fmt --check ;;
    *) echo "unknown step $step" >> "$LOG" ;;
  esac
done
echo "$(date -u '+%FT%TZ') driver done: $*" >> "$LOG"
