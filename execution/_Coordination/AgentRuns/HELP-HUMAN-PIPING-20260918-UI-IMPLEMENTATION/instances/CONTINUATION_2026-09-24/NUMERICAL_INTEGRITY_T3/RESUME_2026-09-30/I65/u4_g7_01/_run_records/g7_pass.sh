#!/bin/bash
# I65 U4 G7 Pass B (and any later re-qualification): repeat items 2-5 of the G7 brief mechanically
# on a new basis, and report byte-identical TEXT/profile outputs or the exact deltas against Pass A.
# Usage: I65_T=<WT> g7_pass.sh <basis dir holding projects/chirality-piping> <basis rev (commit or tree)> <tag>
# Reads Pass A's scripts, rules and reference outputs from REC (this folder). Writes only under
# WT/scratch/i65_u4_g7_01/pass_<tag>/ and WT/targets/i65_g7/work*-pass_<tag>. Read-only for every source
# file; Git reads only (GIT_OPTIONAL_LOCKS=0); one cargo job at a time; memguard checked first.
# Exit codes: 0 ran to the end (read the summary); 3 the build is not Registered (stop, brief item 2);
# 4 the line map could not carry a rule (stop: the deltas need reading); 9 memguard not running.
set -u
T=${I65_T:?set I65_T to the worktree root}; BASIS=$1; REV=$2; TAG=$3
REC=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g7_01/_run_records
REV87=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV87/u4_g6_02/_run_records/rv87_g6r_noncandidates.out.json
NUMREPO=$T/numerics; PASS_A_REV=ba1faa1c858ce3630a22767677310b1902a14b83
O=$T/scratch/i65_u4_g7_01/pass_$TAG; WK=$O/work; RR=$O/rr; mkdir -p $O/logs $O/tmp
export GIT_OPTIONAL_LOCKS=0 TMPDIR=$O/tmp
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
say() { echo "== $* ($(date -u +%FT%TZ))"; }
guard
# ---- 0. a fresh copy of the basis, checked blob for blob against the revision
rm -rf $WK; mkdir -p $WK; cp -R $BASIS/projects $WK/
git -C $NUMREPO ls-tree -r $REV -- projects/chirality-piping | grep -v "projects/chirality-piping/execution/" | awk '$2=="blob"{print $3"\t"$4}' > $O/tree_blobs.tsv
say "tree: $(python3 $REC/tree_check.py $O/tree_blobs.tsv $WK)"
W=$WK/projects/chirality-piping; P=$W/core/product_physics
# ---- 2. identity, reviewed inputs, layouts (the registered law tests), and statics
guard
( cd $P && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $T/targets/i65_g7/work-pass_$TAG retained_memory -- --nocapture --test-threads=1 > $O/logs/law.log 2>&1 )
python3 $REC/identity_check.py $O/logs/law.log $P/src/retained_memory.rs > $O/identity_check.out.json
say "identity: $(python3 -c "import json;d=json.load(open('$O/identity_check.out.json'));print('Registered' if d['registered'] else 'NOT REGISTERED', d['summary'])")"
python3 -c "import json,sys;sys.exit(0 if json.load(open('$O/identity_check.out.json'))['registered'] else 3)" || { say "STOP: the build is Stale on this basis"; exit 3; }
python3 $REC/statics_list.py $W $REC/crate_dirs.txt $REC/pass_a/statics_a.json > $O/statics.json
say "statics added since Pass A: $(python3 -c "import json;d=json.load(open('$O/statics.json'));print(d['added'], 'removed', d['removed'])")"
# ---- 3/4. TEXT: rules carried from Pass A's basis by the line map; the inventory regenerated
mkdir -p $RR; cp $REC/chain/* $RR/
python3 $REC/g7_linemap.py $NUMREPO $PASS_A_REV $REV $O/rules $REC/chain/callgraph_rules.g4.json $REC/chain/loop_bounds.g4.json $REC/chain/text_args.g4.json $REC/chain/sens.py > $O/linemap.out.json || { cat $O/linemap.out.json; say "STOP: a rule's line lies in a changed hunk (read the deltas)"; exit 4; }
cp $O/rules/* $RR/
python3 - $W $REC/crate_dirs.txt $RR <<'PY'
import os, sys, subprocess
W, crates, RR = sys.argv[1:4]
files = []
for c in open(crates).read().split():
    for d, _, fs in os.walk(os.path.join(W, c)):
        if "/tests" in d or "/bin" in d: continue
        files += [os.path.relpath(os.path.join(d, f), W) for f in fs if f.endswith(".rs") and not f.endswith("_tests.rs") and f != "tests.rs"]
out = subprocess.run(["python3", RR + "/template_inventory.py", "."] + sorted(files), cwd=W, capture_output=True, text=True, check=True).stdout
open(RR + "/template_inventory_head.out.json", "w").write(out)
PY
guard
( cd $W && bash $RR/run_text_part2.sh pb $O > $O/logs/text.log 2>&1 )
say "TEXT: $(cat $O/sens_pb.summary.json | head -c 300)"
A=$REC/pass_a/text_g7
for v in "" _W _X _env; do python3 $REC/text_row_diff.py $A/text_budget$v.caps.out.json $O/sens_pb/text_budget$v.caps.out.json > $O/text_row_diff$v.out.txt; say "rows$v: $(head -1 $O/text_row_diff$v.out.txt)"; done
for f in composite_text.caps.json g4_caps.caps.eps2.out.json ordinary_caps.caps.out.json producer_caps.caps.out.json profile_tree.json t07_repair.caps.out.json t08_closure.caps.log t25_g4.caps.eps2.out.json; do
  cmp -s $A/$f $O/sens_pb/$f && echo "   identical $f" || echo "   DIFFERS $f"; done
cmp -s $A/sens_g7.summary.json $O/sens_pb.summary.json && say "summary identical" || say "summary DIFFERS"
# the committed profile block against this chain's transcription
cp $P/src/retained_memory.rs $O/retained_memory.regen.rs; python3 $RR/g5_profile.py $O/sens_pb/profile_tree.json $O/retained_memory.regen.rs > /dev/null
diff -u $P/src/retained_memory.rs $O/retained_memory.regen.rs > $O/profile_block.diff; say "profile block vs transcription: $(grep -c '^[-+] ' $O/profile_block.diff) changed lines (Pass A: the T17_V4 line only, if the F5 term is not applied)"
python3 $REC/price_delta.py $O/sens_pb/profile_tree.json $O/logs/law.log > $O/price_delta.out.json
grep -E 'I65_G5_PROFILE' $O/logs/law.log
# identifier audit enforcement (inside the run's `complete`), its controls, and the section 11 sweep
( cd $W && env G4_CAPS='{"l": 128}' TB_LEXICON=$O/lexicon_pb.json TB_COMPOSITE=$O/sens_pb/composite_text.caps.json TB_D=14734 TB_NONCAND_OUT=$O/noncand.json python3 $O/sens_pb/text_budget.py $W $O/sens_pb/template_inventory_head.out.json $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $O/sens_pb/text_args.g4.json run_linear_static_preview_value_with_retained_direct caps > $O/whole_nc.out.json )
python3 $REC/noncand_compare.py $REV87 $O/noncand.json > $O/noncand_compare.out.json; say "section 11 sweep: $(python3 -c "import json;print(json.load(open('$O/noncand_compare.out.json'))['verdict'])")"
( cd $W && python3 $REC/audit_controls_g7.py $O/sens_pb $W $O/edges_pb.json $O/lexicon_pb.json $O/ctl > $O/controls.out.txt ); say "controls: $(grep -c 'pass= True' $O/controls.out.txt)/11 pass"
# ---- 5. witnesses, challenge, PP (all targets) and runner/headless, registered build of the basis
I65_T=$T bash $REC/run_cargo_a.sh pass_$TAG $WK > $O/logs/cargo.out 2>&1; cat $O/logs/cargo.out | grep -E '^== |passed|I65_G5_CHALLENGE |I65_G5_WITNESS'
G7L=$T/scratch/i65_u4_g7_01/logs
for s in pp runner; do diff <(sed -E 's/ \(.*//' $REC/pass_a/a_$s.outcomes) <(sed -E 's/ \(.*//' $G7L/pass_${TAG}_$s.outcomes) > $O/outcomes_$s.diff && say "$s outcomes identical to Pass A" || say "$s outcomes DIFFER from Pass A ($O/outcomes_$s.diff)"; done
say "done"
