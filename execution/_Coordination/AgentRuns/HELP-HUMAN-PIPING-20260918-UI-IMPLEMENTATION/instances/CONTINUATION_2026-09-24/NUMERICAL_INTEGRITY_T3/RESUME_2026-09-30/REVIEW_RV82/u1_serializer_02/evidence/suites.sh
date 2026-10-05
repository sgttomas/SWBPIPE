#!/bin/bash
# RV82: sequential suites, candidate then base. One cargo job at a time.
set -u
W=WT
S=$W/scratch/rv82_u1_serializer_02; L=$S/logs
run() { # label manifest target [extra]
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)" >> $L/progress.txt
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 2400 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${4:-} > $L/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)" >> $L/progress.txt
  awk '/^ *Running /{t=$2; sub(/-[0-9a-f]+\)?$/,"",t)} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $L/$1.log | sort > $L/$1.outcomes
}
C=$W/rv82/cand/projects/chirality-piping/core; G=$W/rv82/g1/projects/chirality-piping/core; B=$W/rv82/base/projects/chirality-piping/core; T=$W/targets/rv82
for step in "$@"; do
  case $step in
    cand_pp) run cand_pp $C/product_physics/Cargo.toml $T/cand-pp ;;
    base_pp) run base_pp $B/product_physics/Cargo.toml $T/base-pp ;;
    cand_runner) run cand_runner $C/runner/headless/Cargo.toml $T/cand-runner ;;
    base_runner) run base_runner $B/runner/headless/Cargo.toml $T/base-runner ;;
    cand_rx) run cand_rx $C/reporting/result_export/Cargo.toml $T/cand-rx ;;
    base_rx) run base_rx $B/reporting/result_export/Cargo.toml $T/base-rx ;;
    g1_pp) run g1_pp $G/product_physics/Cargo.toml $T/g1-pp ;;
    g1_runner) run g1_runner $G/runner/headless/Cargo.toml $T/g1-runner ;;
    g1_rx) run g1_rx $G/reporting/result_export/Cargo.toml $T/g1-rx ;;
  esac
done
echo "== all done $(date -u +%FT%TZ)" >> $L/progress.txt
