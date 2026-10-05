#!/bin/bash
# I65 U4 G5 part 2: the PP suite control, re-run after run_p2_controls.sh. The candidate copy
# (cand3: 1e323058f3's git archive of core/fixtures/schemas/validation plus part 2's files) lacked
# apps/, which PP's tests include_str!; apps is identical at 8abb5274a9 and 1e323058f3, so base's
# copy is added.
set -u
T=WT
S=$T/scratch/i65_u4_g5_01; C=$S/cand3/projects/chirality-piping/core
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
outcomes() { awk '/^ *Running /{t=$2} /^test .* \.\.\. (ok|FAILED|ignored)/{sub(/ \(.*\)/,""); print t" :: "$0}' $S/logs/$1.log | sed -E 's#target/[^ ]*/deps/##; s#-[0-9a-f]{16}##' | sort > $S/logs/$1.outcomes; }
run() { guard; echo "== $1 start $(date -u +%FT%TZ)"; ( cd $(dirname $2) && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 ${4:-} perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --no-fail-fast --manifest-path $2 --target-dir $3 ${5:-} > $S/logs/$1.log 2>&1 ); echo "== $1 exit=$? $(date -u +%FT%TZ)"; outcomes $1; }
[ -d $S/cand3/projects/chirality-piping/apps ] || cp -R $S/base/projects/chirality-piping/apps $S/cand3/projects/chirality-piping/
diff -rq $S/base/projects/chirality-piping/apps $S/cand3/projects/chirality-piping/apps && echo "== apps equals base's"
run p2_pp $C/product_physics/Cargo.toml $T/targets/i65-g5/cand3
diff <(sed -E 's/ \(.*//' $S/logs/base_pp.outcomes) <(sed -E 's/ \(.*//' $S/logs/p2_pp.outcomes)
echo "p2_pp: $(grep -c ' ok$' $S/logs/p2_pp.outcomes) ok, $(grep -c 'FAILED' $S/logs/p2_pp.outcomes) failed, $(grep -c 'ignored' $S/logs/p2_pp.outcomes) ignored; base_pp: $(grep -c ' ok$' $S/logs/base_pp.outcomes) ok, $(grep -c 'FAILED' $S/logs/base_pp.outcomes) failed, $(grep -c 'ignored' $S/logs/base_pp.outcomes) ignored"
grep -E 'generated [0-9]+ warning' $S/logs/p2_pp.log | sort | uniq
