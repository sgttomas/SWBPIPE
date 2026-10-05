#!/bin/bash
# RV85: sequential suites, candidate then base. One cargo job at a time.
set -u
W=WT
S=$W/scratch/rv85_u3_facade_02; L=$S/logs
run() { # label manifest target [extra...]
  local label=$1 man=$2 tgt=$3; shift 3
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING" >> $L/progress.txt; exit 9; }
  echo "== $label start $(date -u +%FT%TZ)" >> $L/progress.txt
  ( cd $(dirname $man) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --no-fail-fast --manifest-path $man --target-dir $tgt "$@" > $L/$label.log 2>&1 ); echo "== $label exit=$? $(date -u +%FT%TZ)" >> $L/progress.txt
  awk '/^ *Running /{t=$2; sub(/-[0-9a-f]+\)?$/,"",t)} /Doc-tests/{t="doctests"} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $L/$label.log | sort > $L/$label.outcomes
}
C=$W/rv85/cand/projects/chirality-piping/core; B=$W/rv85/base/projects/chirality-piping/core; T=$W/targets/rv85/g1b
for step in "$@"; do
  case $step in
    cand_pp) run cand_pp $C/product_physics/Cargo.toml $T/cand-pp ;;
    base_pp) run base_pp $B/product_physics/Cargo.toml $T/base-pp ;;
    cand_runner) run cand_runner $C/runner/headless/Cargo.toml $T/cand-runner ;;
    base_runner) run base_runner $B/runner/headless/Cargo.toml $T/base-runner ;;
    cand_rx) run cand_rx $C/reporting/result_export/Cargo.toml $T/cand-rx ;;
    base_rx) run base_rx $B/reporting/result_export/Cargo.toml $T/base-rx ;;
  esac
done
echo "== all done $(date -u +%FT%TZ)" >> $L/progress.txt
