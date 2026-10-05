#!/bin/bash
# Protected byte control 1 (continued): runner/headless base and candidate; result_export and FK --lib candidate.
set -u
W=WT; S=$W/scratch/i61_u1_serializer_01
C=$W/f2a-serializer/projects/chirality-piping/core; B=$S/lane/projects/chirality-piping/core
run() { # label manifest target [extra args]
  pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
  echo "== $1 start $(date -u +%FT%TZ)"
  ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 1200 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${4:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"
  awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sort > $S/logs/$1.outcomes
}
run base_runner $B/runner/headless/Cargo.toml $W/targets/i61-u1/base-runner
run cand_runner $C/runner/headless/Cargo.toml $W/targets/i61-u1/runner
run cand_result_export $C/reporting/result_export/Cargo.toml $W/targets/i61-u1/result_export
run cand_fk_lib $C/solver/frame_kernel/Cargo.toml $W/targets/i61-u1/fk --lib
